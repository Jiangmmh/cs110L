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
        let child = command
            .args(args)                    // 配置参数
            .pre_exec(|| {          // 配置pre_exec勾子
                match child_traceme() {
                    Ok(_) => Ok(()),
                    Err(e) => return Err(e),
                }
            })
            .spawn()  // 启动子进程，相当于fork+exec，中间插入了pre_exec的勾子
            .ok()?;

        // 3. 等待子进程停
        let inferior = Inferior{ child };
        match inferior.wait(None) {
            // SIGTRAP 是调试器与被调试程序之间进行控制权交接和同步的关键信号
            // 子进程应该是因为SIGTRAP停下的
            Ok(Status::Stopped(signal::Signal::SIGTRAP, _)) => {
                Some(Inferior { child })
            }

            Ok(status) => {
                eprintln!("Inferior did not stop with SIGTRAP, instead got: {:?}", status);
                // 尝试杀死子进程并返回 None 进行清理
                let _ = child.kill(); 
                None
            }

            Err(e) => {
                eprintln!("Waitpid failed: {}", e);
                // 尝试杀死子进程并返回 None 进行清理
                let _ = child.kill(); 
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
}
