# CS110L Safety in System Programming

> Our focus is on **safety** and **robustness** in **systems programming**: Where do things often go wrong in computer systems? How can we avoid common pitfalls? We will introduce and use the **Rust programming language** as a vehicle to teach **mental models** and **paradigms** that have been shown to be helpful in **preventing errors**, and we will examine how these features have made their way back into C++. 

这门课程以Rust语言为载体，教授如何在系统编程中编写安全和鲁棒的程序。为什么是Rust？因为Rust语言编译器对很多不安全的行为进行了限制，如果我们能够学会Rust编程的思维，并自然地编写出符合Rust编译器要求的程序，那么在编写C/C++程序时也能过一眼洞穿可能出现的错误，并提前避免安全问题，提高代码的鲁棒性。

课程网址：

- 2020sp（有视频）：https://reberhardt.com/cs110l/spring-2020/

- 2022win：https://web.stanford.edu/class/cs110l/

我学习该课程是为了提高编写Rust语言的能力，以更好地完成2025秋冬季开源操作系统训练营的任务。我主要会学习20sp的内容，并参考22win的slides，下面是我的学习笔记，仅供参考。

# Lectures

## Lecture 1 Welcome to CS 110L

先介绍了课程的基本情况和导师的基本信息，然后举了一个栈溢出的例子：

```c++
#include <stdio.h>
#include <string.h>
int main() {
     char s[100];
     int i;
     printf("\nEnter a string : ");
    //==============================
     gets(s);
    //==============================
     for (i = 0; s[i]!='\0'; i++) {
         if(s[i] >= 'a' && s[i] <= 'z') {
           s[i] = s[i] -32;
         }
     }
     printf("\nString in Upper Case = %s", s);
     return 0;
}
```

这段代码的目的是将输入的字符串中的小写字母全部转换为大写字母。其中s是main函数内的局部变量，因此会被分配在栈上，**gets**函数在输入的字符串长度大于100时会出现栈溢出，有可能导致将保存在栈上的返回地址给覆盖掉（了解下函数调用时栈的作用），让用户输入的恶意代码控制系统。

再来看一个例子：

```c++
char buffer[128];
int bytesToCopy = packet.length;
if (bytesToCopy < 128) {	// OK，check num of bytes
   strncpy(buffer, packet.data, bytesToCopy); // OK，using strncpy
}	
```

这段代码用于将通过网络传来的packet拷贝到buffer中，看似没有问题，在拷贝前对要拷贝的字节数进行判断，让其小于128，并且使用了strncpy来控制拷贝数量。想要找到问题，我们需要看一看该函数的声明：

```c++
char *strncpy(char *destination, const char *source, size_t n);
```

第三个参数n的类型为size_t，它是一个无符号整型，这就是出问题的关键，因为bytesToCopy是带符号的，并且`bytesToCopy < 128`也是带符号运算，如果`packet.length`的值为`-1`，那么它可以顺利通过if的检查，然后去执行strncpy，但是在将`bytesToCopy`从带符号整型转换为无符号整型后值会变得非常大（补码表示），造成栈溢出。

即使是经验丰富的程序员仍有可能写出有安全问题的代码，发现问题，解决问题，我们应该如何避免这些错误？这就是Rust要解决的主要问题。

> **Write code differently**：Create **habits** and **frameworks** that make it **harder to produce these kinds of mistakes**.

## Lecture 2 





# Exercises





# Projects

