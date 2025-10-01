#include <unistd.h>
#include <sys/wait.h>

int main() {
    int fds[2];
    pipe(fds);
    pid_t pid = fork();
    if (pid == 0) {
        return 0;       // 未释放fds
    }
    close(fds[0]);      // 仅释放了一个fd
    sleep(2);
    waitpid(pid, NULL, 0);
    return 0;
}
