use nix::sys::ptrace;
use nix::sys::signal;
use nix::sys::wait::{waitpid, WaitPidFlag, WaitStatus};
use nix::unistd::Pid;
use std::os::unix::process::CommandExt;
use std::process::Child;

#[derive(Debug)]
pub enum Status {
    /// Indicates inferior stopped. Contains the signal that stopped the process, as well as the
    /// current instruction pointer that it is stopped at.
    Stopped(signal::Signal, usize),

    /// Indicates inferior exited normally. Contains the exit status code.
    Exited(i32),

    /// Indicates the inferior exited due to a signal. Contains the signal that killed the
    /// process.
    Signaled(signal::Signal),
}

/// This function calls ptrace with PTRACE_TRACEME to enable debugging on a process. You should use
/// pre_exec with Command to call this in the child process.
fn child_traceme() -> Result<(), std::io::Error> {
    ptrace::traceme().or(Err(std::io::Error::new(
        std::io::ErrorKind::Other,
        "ptrace TRACEME failed",
    )))
}

pub struct Inferior {
    child: Child,
}


impl Inferior {
    /// Attempts to start a new inferior process. Returns Some(Inferior) if successful, or None if
    /// an error is encountered.
    pub fn new(target: &str, args: &Vec<String>) -> Option<Inferior> {
        // TODO: implement me!

        // 1. 创建Command实例
        let mut command = std::process::Command::new(target);

        // 2. 配置参数和pre_exec勾子
        let child = unsafe {
            command
                .args(args)                    // 配置参数
                .pre_exec(|| {                 // 配置pre_exec勾子
                    match child_traceme() {
                        Ok(_) => Ok(()),
                        Err(e) => return Err(e),
                    }
                })
                .spawn()  // 启动子进程，相当于fork+exec，中间插入了pre_exec的勾子
                .ok()?
        };

        // 3. 子程序在调用exec切换为新程序后，执行入口的第一条指令之前，
        //      内核会将其停住，并发送SIGTRAP给父进程
        let child_pid = nix::unistd::Pid::from_raw(child.id() as i32);
        let inferior = Inferior { child };

        match waitpid(child_pid, None) {
            // 传统的 SIGTRAP 停止 (对于 exec 停止时可能是这个)
            Ok(WaitStatus::Stopped(_, signal::Signal::SIGTRAP)) |
            // 更标准的 exec 停止事件 (PTRACE_EVENT_EXEC)
            Ok(WaitStatus::PtraceEvent(_, signal::Signal::SIGTRAP, _)) => {
                // 成功：将 inferior 的所有权转移作为返回值
                Some(inferior)
            }

            // 失败或意外停止
            Ok(status) => {
                eprintln!("Inferior did not stop with SIGTRAP, instead got: {:?}", status);
                // inferior 在这里超出作用域，其内部的 child 句柄被 drop，资源句柄关闭。
                // 进程应该已经停止，不需要手动 kill。
                None
            }

            Err(e) => {
                eprintln!("Waitpid failed: {}", e);
                // 同样，inferior 句柄被 drop
                None
            }
        }
    }

    /// Returns the pid of this inferior.
    pub fn pid(&self) -> Pid {
        nix::unistd::Pid::from_raw(self.child.id() as i32)
    }

    /// Calls waitpid on this inferior and returns a Status to indicate the state of the process
    /// after the waitpid call.
    pub fn wait(&self, options: Option<WaitPidFlag>) -> Result<Status, nix::Error> {
        Ok(match waitpid(self.pid(), options)? {
            WaitStatus::Exited(_pid, exit_code) => Status::Exited(exit_code),
            WaitStatus::Signaled(_pid, signal, _core_dumped) => Status::Signaled(signal),
            WaitStatus::Stopped(_pid, signal) => {
                let regs = ptrace::getregs(self.pid())?;
                Status::Stopped(signal, regs.rip as usize)
            }
            other => panic!("waitpid returned unexpected status: {:?}", other),
        })
    }

    pub fn cont(&self) -> Result<Status, nix::Error> {
        // 1. 调用 ptrace::cont，告诉内核让子进程继续执行。
        //    通常我们传递 None 作为信号参数，告诉内核不要发送额外的信号。
        //    如果子进程是因断点（SIGTRAP）而停止，ptrace::cont 恢复执行，
        //    并允许 SIGTRAP 信号传递给进程（但内核通常在恢复时会忽略它）。
        ptrace::cont(self.pid(), None)?;

        // 2. 阻塞等待子进程的下一个状态变化。
        //    程序将一直运行，直到遇到下一个断点、单步停止、或正常退出。
        self.wait(None)
    }
}
