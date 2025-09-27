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

## Lecture 2 Memory Safety



Rust如何解决这些问题？

1. Ownership：所有权

   例如a = 10，其中a是一个变量，10是一个值，在Rust中我们称变量a拥有值10的所有权。

   所有权的特点有：

   - 每个值都属于某个变量
   - 一个值同一时刻只能属于一个变量

   ```rust
   fn main() {
       let s: String = "i am a lil string".to_string();  // 创建一个变量，拥有一个字符串值
       let u = s;				// s将该字符串的所有权转移给u
       println!("{}", s);		// 错误！！s已经没有任何值的所有权了，无法使用
   }
   
   minghan@Minghan:~/projs/cs110L/hello$ cargo build
    --> src/main.rs:3:9
     |
   3 |     let u = s;
     |         ^ help: if this is intentional, prefix it with an underscore: `_u`
     |
     = note: `#[warn(unused_variables)]` on by default
   
   error[E0382]: borrow of moved value: `s`
    --> src/main.rs:4:20
     |
   2 |     let s: String = "i am a lil string".to_string();
     |         - move occurs because `s` has type `String`, which does not implement the `Copy` trait
   3 |     let u = s;
     |             - value moved here
   4 |     println!("{}", s);
     |                    ^ value borrowed here after move
     |
     = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
   help: consider cloning the value if the performance cost is acceptable
     |
   3 |     let u = s.clone();
     |              ++++++++
   ```

   运行该程序，可以看到Rust编译器报错了：`borrow of moved value: s`，意思是在所有权转移后仍然使用该变量s

2. Borrowing：借用

   借用指获取某个变量的指针，并不进行所有权的转移，我们可以通过该指针来访问值，但是该指针变量并不拥有值的所有权。

   ```rust
   fn main() {
       let s: String = "i am a lil string".to_string();
       let u = &s;		// 借用，不发生所有权的转移
       
       println!("{}", s);	// 使用s的值，没问题
       println!("{}", u);	// 通过指针u使用s的值
   }
   
   minghan@Minghan:~/projs/cs110L/hello$ cargo run
      Compiling hello v0.1.0 (/home/minghan/projs/cs110L/hello)
       Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.18s
        Running `target/debug/hello`
   i am a lil string
   i am a lil string
   ```

   注意，在Rust中定义的变量默认为不可变的，如果想要定义可以修改值的变量，需要加上`mut`，例如`let mut a: int = 100;`

   一个变量可以同时存在多个引用，引用又分可变引用和不可变引用，通过可变引用我们可以修改原变量的值（原变量）。

3. Lifetime：生命周期

   变量都有其作用域，当程序离开拥有该值的变量所在的作用域，该值将被回收，Rust不允许使用一个超过原变量生命周期的借用（引用）。生命周期限制了像返回局部变量引用这种错误的出现。

Rust对于这些概念的检查都是在编译时进行的，这也是为什么Rust程序安全性高的原因，即尽可能在程序开发时杜绝可能的安全性问题，而不是在运行时才发现错误。

# Exercises

## Week1 Hello world 

### Part1 Getting oriented

先安装配置好开发环境：安装Rust、选择顺手的编辑器（RustRover、VScode）。

拉取start code：

````shell
git clone https://github.com/reberhardt7/cs110l-spr-2020-starter-code.git
````

进入week1的part1-hello-world目录，运行该项目，检查环境安装是否完成：

```shell
minghan@Minghan:~/projs/cs110L/cs110l-spr-2020-starter-code/week1/part-1-hello-world$ cargo run
warning: both `/home/minghan/.cargo/config` and `/home/minghan/.cargo/config.toml` exist. Using `/home/minghan/.cargo/config`
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/hello-world`
Hello, world!
```

修改main.rs，让其输出其它内容：

```rust
fn main() {
    println!("You rock!");
}
```

```shell
minghan@Minghan:~/projs/cs110L/cs110l-spr-2020-starter-code/week1/part-1-hello-world$ cargo run
warning: both `/home/minghan/.cargo/config` and `/home/minghan/.cargo/config.toml` exist. Using `/home/minghan/.cargo/config`
   Compiling hello-world v0.1.0 (/home/minghan/projs/cs110L/cs110l-spr-2020-starter-code/week1/part-1-hello-world)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/hello-world`
You rock!
```

### Part2 Rust warmup

完成三个简单任务，练习下Rust的基础语法：

1. `add_n`：传入一个Vec v和一个整数n，返回一个新的Vec，元素为v中元素+n

   ```rust
   fn add_n(v: Vec<i32>, n: i32) -> Vec<i32> {
       let mut ans: Vec<i32> = Vec::new();
       for i in v.iter() {
           ans.push(*i + n);
       }
       ans
   }
   ```

   通过实现这个函数要学会使用Vec，包括创建Vec、使用迭代器遍历Vec和向Vec中添加元素。

   更多Vec的使用请参考文档：https://doc.rust-lang.org/std/vec/struct.Vec.html#

2. `add_n_inplace`：同上，不过改为在原数组上+n

   ```rust
   fn add_n_inplace(v: &mut Vec<i32>, n: i32) {
       for i in v.iter_mut() {
           *i += n;
       }
   }
   ```

   这里使用了mut迭代器，可以通过它修改Vec中的值。

3. `dedup`：删除Vec中的重复元素

   ```rust
   fn dedup(v: &mut Vec<i32>) {
       let mut seen = HashSet::new();
       let mut i = 0;
       while i < v.len() {
           if !seen.insert(v[i]) {  // insert在*i存在时返回false，否则返回true
               v.remove(i);    // 删除Vec中下标为i的元素
           } else {
               i += 1;
           }
       }
   }
   ```

   学习HashSet的使用，包括创建HashSet、向HashSet中添加元素，使用remove方法删除Vec中指定下标的元素，index + while遍历Vec。

测试结果：

```shell
minghan@Minghan:~/projs/cs110L/cs110l-spr-2020-starter-code/week1/part-2-warmup$ cargo test
   Compiling warmup v0.1.0 (/home/minghan/projs/cs110L/cs110l-spr-2020-starter-code/week1/part-2-warmup)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.36s
     Running unittests src/main.rs (target/debug/deps/warmup-9bc4934a53489a7d)

running 3 tests
test test::test_add_n ... ok
test test::test_add_n_inplace ... ok
test test::test_dedup ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00
```

	### Part3 Hangman

按照实验指导的要求写一个猜单词的小游戏：

```rust
// Simple Hangman Program
// User gets five incorrect guesses
// Word chosen randomly from words.txt
// Inspiration from: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
// This assignment will introduce you to some fundamental syntax in Rust:
// - variable declaration
// - string manipulation
// - conditional statements
// - loops
// - vectors
// - files
// - user input
// We've tried to limit/hide Rust's quirks since we'll discuss those details
// more in depth in the coming lectures.
extern crate rand;
use rand::Rng;
use std::fs;
use std::io;
use std::io::Write;

const NUM_INCORRECT_GUESSES: u32 = 5;
const WORDS_PATH: &str = "words.txt";

fn pick_a_random_word() -> String {
    let file_string = fs::read_to_string(WORDS_PATH).expect("Unable to read file.");
    let words: Vec<&str> = file_string.split('\n').collect();
    String::from(words[rand::thread_rng().gen_range(0, words.len())].trim())
}

fn main() {
    let secret_word = pick_a_random_word();
    // Note: given what you know about Rust so far, it's easier to pull characters out of a
    // vector than it is to pull them out of a string. You can get the ith character of
    // secret_word by doing secret_word_chars[i].
    let secret_word_chars: Vec<char> = secret_word.chars().collect();
    // Uncomment for debugging:
    println!("random word: {}", secret_word);

    // Your code here! :)
    println!("Welcome to CS110L Hangman!");

    // idx_right_yet，记录了当前secret_word中已经被猜出来的字符下标
    let mut  idx_right_yet: Vec<bool> = vec![false; secret_word_chars.len()];

    // letter_yet记录历史输入
    let mut letter_yet: String = String::from("");

    // 两个counter分别记录剩余的猜测机会和猜对的字符数
    let mut chance_guess = NUM_INCORRECT_GUESSES;
    let mut count_right = 0;
    while chance_guess > 0 && count_right < secret_word_chars.len() {
        println!("The word so far is {}", word_with_mask(&secret_word_chars, &idx_right_yet));
        println!("You have guessed the following letters: {}", letter_yet);
        println!("You have {} guesses left", chance_guess);
        print!("Please guess a letter: ");

        // 字符串输入
        io::stdout()
            .flush()
            .expect("Error flushing stdout.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Error reading line.");

        // 更新输入历史
        let ch = guess.chars().nth(0).unwrap();
        letter_yet.push(ch);

        // 查看该guess是否在secret_word中
        let mut i = 0;
        while i < secret_word_chars.len() {
            if !idx_right_yet[i] && secret_word_chars[i] == ch {
                break;
            }
            i += 1;
        }
        
        if i == secret_word_chars.len() {
            println!("Sorry, that letter is not in the word");
            chance_guess -= 1;
        } else {
            idx_right_yet[i] = true;
            count_right += 1;
        }
        println!();
    }

    if count_right == secret_word_chars.len() {
        println!("Congratulations you guessed the secret word: {}!", secret_word);
    } else {
        println!("Sorry, you ran out of guesses!")
    }
}

// 构造带掩码的字符串
fn word_with_mask(secret_word_chars: &Vec<char>, idx: &Vec<bool>) -> String {
    let mut ans: String = String::from("");
    for i in 0..secret_word_chars.len() {
        if idx[i] {
            ans.push(secret_word_chars[i]);
        } else {
            ans.push_str("-");
        }
    }
    ans
}
```

## Week2 Ownership and structs







# Projects

