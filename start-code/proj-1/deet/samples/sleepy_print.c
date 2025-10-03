#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

int main(int argc, char *argv[]) {
    // 检查并绑定参数到num_seconds
    unsigned long num_seconds;
    if (argc != 2 || (num_seconds = strtoul(argv[1], NULL, 10)) == 0) {
        fprintf(stderr, "Usage: %s <seconds to sleep>\n", argv[0]);
        exit(1);
    }

    // 每秒打印一个数字，直到达到num_seconds
    for (unsigned long i = 0; i < num_seconds; i++) {
        printf("%lu\n", i);
        sleep(1);
    }

    return 0;
}
