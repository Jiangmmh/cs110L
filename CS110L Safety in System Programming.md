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

## Lecture 3 Error Handling

在介绍Rust的错误处理前，要来介绍一下Enum，即枚举类型。

```rust
enum TrafficLightColor {
    Red,
    Yellow,
    Green,
}

let current_state: TrafficLightColor = TrafficLightColor::Green;
```

我们可以通过enum关键字定义枚举类型，其中有很多字段，一个枚举类型的值可以是其中某一个字段。

```rust
fn drive(light_state: TrafficLightColor) {
    match light_state {
        TrafficLightColor::Green => println!("zoom zoom!"),
        TrafficLightColor::Yellow => println!("slowing down..."),
        TrafficLightColor::Red => println!("sitting like a boulder!"),
    }
}
```

可以使用match来根据枚举类型变量的字段来做出不同的反应。

```rust
match light_state {
    TrafficLightColor::Green => println!("zoom zoom!"),
    _ => println!("do not pass go"), // default binding
}
```

由于match必须是完备的（必须涵盖所有情况），因此可以使用`_`来代替剩余的情况。

```rust
enum Location {
    Coordinates(f32, f32),
    Address(String),
    Unknown,
}

let location = Location::Address("353 Jane Stanford Way".to_string());

fn print_location(loc: Location) {
    match loc {
        Location::Coordinates(lat, long) => {
            println!("Person is at ({}, {})", lat, long);
        },
      
        Location::Address(addr) => {
            println!("Person is at {}", addr);
        },
      
        Location::Unknown => println!("Location unknown!"),
    }
}
```

在Rust中，枚举类型的字段还可以携带值，并且能够在match中获取枚举类型变量中存放的值。

Ok，有了枚举类型的基础，就可以来介绍Rust的错误处理方法了，即Result类型：

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

可以看到Result是一个使用了泛型（后续课程中会学习）的枚举类型，有两个字段Ok和Err，分别携带了类型为T和类型为E的值。

```rust
fn gen_num_sometimes() -> Result<u32, &'static str> {
    if get_random_num() > 10 {
        Ok(get_random_num())
    } else {
        Err("Spontaneous failure!")
    }
}

fn main() {
    match gen_num_sometimes() {
        Ok(num) => println!("Got number: {}", num),
        Err(message) => println!("Operation failed: {}", message),
    }
}
```

我们可以使Result作为函数的返回值，当程序正确执行时返回携带正确值的`Ok`，否则就返回携带错误信息的`Err`。

```rust
fn read_file(filename: &str) -> Result<String, io::Error> {
    let mut s = String::new();

    let result: Result<File, io::Error> = File::open(filename);
  
    let mut f: File = match result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    match f.read_to_string(&mut s) {
        Ok(_) => (),
        Err(e) => return Err(e),
    };

    return Ok(s);
}
```

这是一个更实际的例子，open和read_to_string返回的都是Result，因此需要在调用后使用match来判断是否出错。

```rust
fn read_file(filename: &str) -> Result<String, io::Error> {
    let mut s = String::new();

    let mut f = File::open(filename)?;

    f.read_to_string(&mut s)?;

    return Ok(s);
}

fn read_file(filename: &str) -> String {
    let mut contents = String::new();
    File::open(filename)?.read_to_string(&mut contents)?;
    return contents;
}
```

`?`用于简化错误判断，在遇到错误时直接返回Error，否则取出其中的数据

```rust
let mut file: File = File::open(filename).unwrap();
let mut file: File = File::open(filename).expect("Failed to open file");
```

`unwrap`和`expect`和`?`类似，都是用于简化错误判断的，但是区别在于`unwrap`和`expect`在遇到错误时会直接panic，一般用于出现了不可恢复的错误。

由于NULL很容易导致安全问题（ [“billion-dollar mistake”](https://en.wikipedia.org/wiki/Tony_Hoare) ），因此Rust选择不提供NULL，而是通过Option枚举类型来提供相似的功能：

```rust
enum Option<T> {
    None,
    Some(T),
}
```

当数据存在时使用Some字段，否则使用None字段，表示NULL。

```rust
fn feeling_lucky() -> Option<String> {
    if get_random_num() > 10 {
        Some(String::from("I'm feeling lucky!"))
    } else {
        None
    }
}

// 使用match判断是否为空
match feeling_lucky() {
    Some(message) => {
        println!("Got message: {}", message);
    },
    None => {
        println!("No message returned :-/");
    },
}

// is_none和is_some方法判断空和非空
if feeling_lucky().is_none() {
    println!("Not feeling lucky :(");
}

// 和Result的处理一样，可以使用unwrap和expect
let message: String = feeling_lucky().unwrap();
let message: String = feeling_lucky().expect("feeling_lucky failed us!");
```

这里介绍了Option的一些使用方法，其实Option还提供了一些方法，如`unwrap_or`等，需要时查看Doc即可。



## Lecture 4 Object Oriented Rust

这节课以一个简单单链表的实现为例子，介绍Rust中实现面向对象的方法、进一步运用前面所学的关于Rust的概念。

要实现链表，我们需要一个节点的定义：

```rust
// Node节点
struct Node {
    val: i32,
    next: Option<Box<Node>>,  // Box用于使用指针，Option在Rust中作为代替NULL的机制
}

impl Node {
  	// 在使用next时，我们不希望将next的所有权交出去，而只是借用
    fn next(&self) -> Option<&Box<Node>> {
        self.next.as_ref()
    }
}
```

next为什么这么复杂？

- next不能为Node，因为会触发递归引用
- next不能为&Node，因为借用的对象必须有其所有者，一个独立的借用无法通过编译

- OK，那我们使用智能指针Box来装Node就可以了，为什么还要套一层Option？
  - 因为Rust没有NULL或nullptr这类空对象，因此需要通过Option来作为空指针的标记

这里要实现一个next函数的原因是，我们不希望直接使用next而造成所有权转移。

```rust
struct LinkedList {
    head: Option<Box<Node>>,
    length: usize,
}
```

链表结构体有一个头节点指针和一个长度字段，下面介绍一下LinkedList需要实现的方法：

```rust
impl LinkedList {
    fn new() -> LinkedList {
        LinkedList{
            head: None,
            length: 0
        }
    }
    
    fn len(&self) -> usize {
        self.length
    }
    
    // 这里不能返回Optioin<Box<Node>>，因为我们不希望在调用front后，链表节点出现move
    fn front(&self) -> Option<&Box<Node>> {
        // 先获取&Option<Box<Node>>，然后使用as_ref转换为Option<&Box<Node>>
        // 如果head为None，as_ref也会返回None
        // 这里不需要再使用&了，因为self本身就是一个借用
        self.head.as_ref()
    }
    
    fn back(&self) -> Option<&Box<Node>> {
        let mut curr = self.front();
        while curr.is_some() {
            let node = curr.unwrap();   // 从Option中取出Box<Node>
            if node.next.is_none() {    // 如果下一个节点为None，说明当前节点为尾节点
                return Some(node);
            }
            curr = node.next.as_ref();
        }
        None
    }
    
    fn front_mut(&mut self) -> Option<&mut Box<Node>> {
        // 将 &mut Option<Box<Node>>转换为Option<&mut Box<Node>>
        self.head.as_mut()
    }
    
    fn push_front(&mut self, val: i32) {
        let mut node = Some(Box::new(Node{ val: val, next: None}));
        // 将node移动到&mut head中去，返回旧head
        let old_head = std::mem::replace(&mut self.head, node);
        self.head.as_mut().unwrap().next = old_head;
        self.length += 1;
    }
    
    fn display(&self) {
        // 将 &Option<Box<Node>> 转换为 Option<&Box<Node>>
        let mut curr = self.front();
        while curr.is_some() {
          	let node = curr.unwrap();
            print!("{} ", node.val);
            curr = (&node.next).as_ref();
        }
    }
}
```

- new：创建一个新Node
- len：返回链表长度
- front：返回头节点，如果链表为空就返回None
  - 这里返回值类型为Option<&Box<Node>>，返回引用是为了避免所有权转移
  - `self.head`的类型为`&Option<Box<Node>>`，使用as_ref将其转换为`Option<&Box<Node>>`
- back：返回尾节点，如果链表为空就返回None
  - 注意这里遍历链表的方法
- front_mut：返回头节点的可变引用
  - `self.head`的类型为`&mut Option<Box<Node>>`，使用as_mut将其转换为`Option<&mut Box<Node>>`
- push_front：头插节点
  - 首先创建一个新节点，将值写入
  - 将head指向新节点，方法是使用`std::mem::replace`将src（第二个参数）写入desc（第一个参数）中，desc必须为&mut，返回旧dest
  - 将新head的next指向旧head，链表长度+1

测试一下：

```rust
fn main() { 
    let mut l1 = LinkedList::new();
    println!("{}", l1.len());
    l1.push_front(1);
    println!("Front: {}", l1.front().unwrap().val);
    println!("Back: {}", l1.back().unwrap().val);

    l1.push_front(2);
    println!("After adding 2:");
    println!("Front: {}", l1.front().unwrap().val);
    println!("Back: {}", l1.back().unwrap().val);

    let node_mut = l1.front_mut();
    node_mut.unwrap().val = 3;
    println!("After changing to 3:");
    println!("Front: {}", l1.front().unwrap().val);
    println!("Back: {}", l1.back().unwrap().val);

    let node_using_next = l1.front().unwrap().next().unwrap();
    println!("Testing the `next` function on Node! Second element: {}", node_using_next.val);

    println!("Length after adding: {}", l1.len());
}
```

## Lecture 5 Traits and Generics

要设计一个优秀的软件，必须要能够区分出系统中可变与不可变的部分，将其中不可变的部分抽取出来复用，而将可变部分留给开发者后续扩展，即满足开闭原则。

这节课介绍了Rust中的两个重要概念Trait和泛型，合理使用它们可以编写出更优秀的软件。

trait用于定义一组函数接口，结构体可以实现这些函数接口，我们可以使用trait作为函数的参数类型，只要传入的实际类型实现了该trait即可（duck type），一个类型实现了一个trait，就告诉我们这个类型提供了哪些接口。

```rust
// 定义一个 Sizable trait，要求实现者提供一个 size_in_bytes 方法
trait Sizable {
    fn size_in_bytes(&self) -> usize;
}

struct Book {
    title: String,
    pages: u32,
}

// 为 Book 实现 Sizable trait
impl Sizable for Book {
    fn size_in_bytes(&self) -> usize {
        // String 在堆上分配，但这里我们只计算栈上 Book 结构体的大小
        // 实际应用中，你可能需要加上堆内存的大小
        std::mem::size_of::<Self>()
    }
}
```

这里定义了一个traits `Sizeable`，它包含了一个函数声明`size_in_bytes`，然后Book结构体实现了该traits，因此所有的Book对象都拥有了`size_in_bytes`方法。

系统定义的一些traits：

- Copy: Will create a new copy of an instance, instead of moving ownership when using assignment (=) 
- Clone: Will return a new copy of an instance when calling the .clone() function on the method. 
- Drop: Will define a way to free the memory of an instance - called when the instance reaches the end of the scope. 
- Display: Defines a way to format a type, and show it (used by println!) 
- Debug: Similar to Display, though not meant to be user facing (Meant for you to debug your types!) 
- Eq: Defines a way to determine equality (defined by an equivalence relation) for two objects of the same type. 
- PartialOrd: Defines a way to compare instances (less than, greater than, less than or equal to, etc.)

我们可以通过编译器宏（macro）`#[derive]` **自动**为类型实现一些通用的 **trait**。

```rust
#[derive(Clone, Copy, Debug)]
struct Coordinate {
    x: i32,
    y: i32,
}

fn main() {
    let c1 = Coordinate { x: 10, y: 20 };
    let c2 = c1; // 因为实现了 Copy，这里是复制而不是移动
    println!("{:?}", c1); // c1 仍然有效
    
    let c3 = c1.clone(); // 调用 clone 方法进行深拷贝
}
```



泛型是对同相同逻辑的不同数据类型实现的抽象，通过抽象可以屏蔽数据类型的差异来编写模版代码，在编译时根据实际数据类型生成对应的特化版本，可用于结构体和函数定义中。链表的泛型版本：

```rust
struct Node<T> {
    val: T,
    next: Option<Box<Node<T>>>,  // Option：Some/None
}

impl<T> Node<T> {
    fn next(&self) -> Option<&Box<Node<T>>> {
        self.next.as_ref()
    }
}

struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    length: usize,
}

impl<T: std::fmt::Display> LinkedList<T> {
    fn new() -> LinkedList<T> {
        LinkedList{
            head: None,
            length: 0
        }
    }
    
    fn len(&self) -> usize {
        self.length
    }
    
    // 这里不能返回Optioin<Box<Node>>，因为我们不希望在调用front后，链表节点出现move
    fn front(&self) -> Option<&Box<Node<T>>> {
        // 先获取&Option<Box<Node>>，然后使用as_ref转换为Option<&Box<Node>>
        // 如果head为None，as_ref也会返回None
        // 这里不需要再使用&了，因为self本身就是一个借用
        self.head.as_ref()
    }
    
    fn back(&self) -> Option<&Box<Node<T>>> {
        let mut curr = self.front();
        while curr.is_some() {
            let node = curr.unwrap();   // 从Option中取出Box<Node>
            if node.next.is_none() {    // 如果下一个节点为None，说明当前节点为尾节点
                return Some(node);
            }
            curr = node.next.as_ref();
        }
        None
    }
    
    fn front_mut(&mut self) -> Option<&mut Box<Node<T>>> {
        // 将 &mut Option<Box<Node>>转换为Option<&mut Box<Node>>
        self.head.as_mut()
    }
    
    fn push_front(&mut self, val: T) {
        let mut node = Some(Box::new(Node{ val: val, next: None}));
        // 将node移动到&mut head中去，返回旧head
        let old_head = std::mem::replace(&mut self.head, node);
        self.head.as_mut().unwrap().next = old_head;
        self.length += 1;
    }
    
    fn display(&self) {
        // 将 &Option<Box<Node>> 转换为 Option<&Box<Node>>
        let mut curr = (&self.head).as_ref();
        while curr.is_some() {
            print!("{} ", curr.unwrap().val);
            curr = (&curr.unwrap().next).as_ref();
        }
    }
}

fn main() { 
    let mut l1 = LinkedList::new();
    println!("{}", l1.len());
    l1.push_front(1);
    println!("Front: {}", l1.front().unwrap().val);
    println!("Back: {}", l1.back().unwrap().val);

    l1.push_front(2);
    println!("After adding 2:");
    println!("Front: {}", l1.front().unwrap().val);
    println!("Back: {}", l1.back().unwrap().val);

    let node_mut = l1.front_mut();
    node_mut.unwrap().val = 3;
    println!("After changing to 3:");
    println!("Front: {}", l1.front().unwrap().val);
    println!("Back: {}", l1.back().unwrap().val);

    let node_using_next = l1.front().unwrap().next().unwrap();
    println!("Testing the `next` function on Node! Second element: {}", node_using_next.val);

    println!("Length after adding: {}", l1.len());
}
```

注意在定义LinkedList时，为泛型的类型T加入了trait限定，即只有实现了该trait的类型才能够使用该泛型模版：

```rust
impl<T: std::fmt::Display> LinkedList<T>
```

这里的`std::fmt::Display`是一个trait，提供了面向用户的输出接口，如果要使用fmt通过`"{}"`进行输出就需要类型T实现该接口。





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

### Part1 Ownership short-answer exercises

给出了几个简单的代码片段，让我们判断这些代码能否通过编译，如果不能的话解释原因。

1. 代码片段一：

   ```rust
   fn main() {
       let mut s = String::from("hello");
       let ref1 = &s;
       let ref2 = &ref1;
       let ref3 = &ref2;
       s = String::from("goodbye");
       println!("{}", ref3.to_uppercase());
   }
   ```

   在这段代码中创建了一个String（分配在堆上），然后创建了三个引用（这里是对引用进行引用，形成了多层引用，Rust会在引用被使用时自动进行解引用），然后希望将s的值改为`String("goodbye")，这是无法已通过编译器的，因为在执行完这条语句后，String("hello")将不再被s所有，因此它需要被释放。但是，在后面的代码中还需要使用对于该值的借用ref3，这就导致了在内存释放后访问它，是不允许的。来看看编译器的解释：

   ```shell
   ~/minghan/rust/cs110l/cs110l-spr-2020-starter-code/warm % cargo build
      Compiling warm v0.1.0 (/Users/minghan/minghan/rust/cs110l/cs110l-spr-2020-starter-code/warm)
   warning: value assigned to `s` is never read
    --> src/main.rs:6:5
     |
   6 |     s = String::from("goodbye");
     |     ^
     |
     = help: maybe it is overwritten before being read?
     = note: `#[warn(unused_assignments)]` on by default
   
   warning: variable does not need to be mutable
    --> src/main.rs:2:9
     |
   2 |     let mut s = String::from("hello");
     |         ----^
     |         |
     |         help: remove this `mut`
     |
     = note: `#[warn(unused_mut)]` on by default
   
   error[E0506]: cannot assign to `s` because it is borrowed
    --> src/main.rs:6:5
     |
   3 |     let ref1 = &s;
     |                -- `s` is borrowed here
   ...
   6 |     s = String::from("goodbye");
     |     ^ `s` is assigned to here but it was already borrowed
   7 |     println!("{}", ref3.to_uppercase());
     |                    ---- borrow later used here
   
   For more information about this error, try `rustc --explain E0506`.
   warning: `warm` (bin "warm") generated 2 warnings
   error: could not compile `warm` (bin "warm") due to 1 previous error; 2 warnings emitted
   ```

   和我们的想法一致，“cannot assign to `s` because it is borrowed”，即不能在变量的值被借用的情况下为其赋新值。

2. 代码片段二：

   ```rust
   fn drip_drop() -> &String {
       let s = String::from("hello world!");
       return &s;
   }
   ```

   这段代码不能通过编译，其在函数中创建一个String s，然后将该String的借用作为返回值返回给调用者。由于s是在函数中创建的，在函数结束后会释放掉String在堆上的内存，然而这里却返回了一个String的借用，可能导致使用已经被释放了的值或多次释放的问题。

   这里应该直接返回String，将所有权从函数的局部作用域转移给调用者的作用域。

   ```rust
   fn drip_drop() -> String {
       let s = String::from("hello world!");
       return s;
   }
   ```

3. 代码片段三：

   ```rust
   fn main() {
       let s1 = String::from("hello");
       let mut v = Vec::new();
       v.push(s1);
       let s2: String = v[0];
       println!("{}", s2);
   }
   ```

   在定义s2时会出现问题，因为Rust 不允许直接移动 `Vec` 中的元素，避免Vector出现空洞，确保**内存安全**。

### Part2 rdiff

要求我们编写一个简单的命令行工具diff，用于找到两个文件中不同的行。

1. 实现一个根据文件名，打开并读取文件内容，返回每行字符串组成的Vector的函数

   ```rust
   fn read_file_lines(filename: &String) -> Result<Vec<String>, io::Error> {
       let mut f = File::open(filename)?;
       let mut data = vec![];
   
       // 创建一个BufReader，然后调用lines返回行迭代器
       // 每次访问得到一个 io::Result<String>
       for line in io::BufReader::new(f).lines() {
           // 使用?将String从Result中拿出来
           let line_str = line?;
           data.push(line_str);
       }
   
       Ok(data)
   }
   ```

   - 使用`File::open(filename)`打开文件
   - 通过`io::BufReader::new(f).lines()`获取一个文件中每行字符串的迭代器
   - 遍历所有行并将其push到data中

2. 完善Grid结构体方法的实现

   ```rust
   // Grid implemented as flat vector
   pub struct Grid {
       num_rows: usize,
       num_cols: usize,
       elems: Vec<usize>,
   }
   
   impl Grid {
       /// Returns a Grid of the specified size, with all elements pre-initialized to zero.
       pub fn new(num_rows: usize, num_cols: usize) -> Grid {
           Grid {
               num_rows: num_rows,
               num_cols: num_cols,
               // This syntax uses the vec! macro to create a vector of zeros, initialized to a
               // specific length
               // https://stackoverflow.com/a/29530932
               elems: vec![0; num_rows * num_cols],
           }
       }
   
       pub fn size(&self) -> (usize, usize) {
           (self.num_rows, self.num_cols)
       }
   
       /// Returns the element at the specified location. If the location is out of bounds, returns
       /// None.
       // #[allow(unused)] // TODO: delete this line when you implement this function
       pub fn get(&self, row: usize, col: usize) -> Option<usize> {
           // unimplemented!();
           // Be sure to delete the #[allow(unused)] line above
           if row >= self.num_rows && col >= self.num_cols {
               return None;
           }
           Some(self.elems[row * self.num_cols + col])
       }
   
       /// Sets the element at the specified location to the specified value. If the location is out
       /// of bounds, returns Err with an error message.
       // #[allow(unused)] // TODO: delete this line when you implement this function
       pub fn set(&mut self, row: usize, col: usize, val: usize) -> Result<(), &'static str> {
           // unimplemented!();
           // Be sure to delete the #[allow(unused)] line above
           if row >= self.num_rows && col >= self.num_cols {
               return Err("Row out of bounds");
           }
   
           self.elems[row * self.num_cols + col] = val;
           Ok(())
       }
   
       /// Prints a visual representation of the grid. You can use this for debugging.
       pub fn display(&self) {
           for row in 0..self.num_rows {
               let mut line = String::new();
               for col in 0..self.num_cols {
                   line.push_str(&format!("{}, ", self.get(row, col).unwrap()));
               }
               println!("{}", line);
           }
       }
   
       /// Resets all the elements to zero.
       pub fn clear(&mut self) {
           for i in self.elems.iter_mut() {
               *i = 0;
           }
       }
   }
   ```

   主要就是get和set方法的实现，没啥困难的地方。

3. 根据伪代码实现LCS

   ```rust
   fn lcs(seq1: &Vec<String>, seq2: &Vec<String>) -> Grid {
       // unimplemented!();
       // Be sure to delete the #[allow(unused)] line above
       let len1 = seq1.len();
       let len2 = seq2.len();
     	
     	// 创建Grid并初始化
       let mut grid = Grid::new(len1+1, len2+1);
       for i in 0..=len1 {
           grid.set(i, 0, 0).unwrap();
       }
       for i in 0..=len2 {
           grid.set(0, i, 0).unwrap();
       }
   	
     	// DP解决LCS问题
       for i in 0..len1 {
           for j in 0..len2 {
               if seq1[i] == seq2[j] {
                   grid.set(i + 1,
                            j + 1,
                            grid.get(i, j).unwrap()+1).unwrap();
               } else {
                   grid.set(i + 1,
                            j + 1,
                            cmp::max(
                                grid.get(i+1, j).unwrap(),
                                grid.get(i, j+1).unwrap()
                            )).unwrap();
               }
           }
       }
       grid
   }
   ```

​	LCS问题介绍：

4. 利用LCS来实现diff

   实现的思路：

   ```rust
   fn print_diff(lcs_table: &Grid, lines1: &Vec<String>, lines2: &Vec<String>, i: usize, j: usize) {
       // unimplemented!();
       // Be sure to delete the #[allow(unused)] line above
       if i > 0 && j > 0 && lines1[i - 1] == lines2[j - 1] {
           print_diff(lcs_table, lines1, lines2, i-1, j-1);
           println!(" {}", lines1[i - 1]);
       } else if j > 0 &&
           (i == 0 || lcs_table.get(i, j-1).unwrap() >= lcs_table.get(i-1, j).unwrap()) {
           print_diff(lcs_table, lines1, lines2, i, j-1);
           println!("> {}", lines2[j - 1]);
       } else if i > 0 &&
           (j == 0 || lcs_table.get(i, j-1).unwrap() < lcs_table.get(i-1, j).unwrap()) {
           print_diff(lcs_table, lines1, lines2, i-1, j);
           println!("< {}", lines1[i - 1]);
       } else {
           println!();
       }
   }
   
   fn main() {
       let args: Vec<String> = env::args().collect();
       if args.len() < 3 {
           println!("Too few arguments.");
           process::exit(1);
       }
       let filename1 = &args[1];
       let filename2 = &args[2];
   
       // unimplemented!();
       // Be sure to delete the #[allow(unused)] line above
       let lines1 = read_file_lines(filename1).unwrap();
       let lines2 = read_file_lines(filename2).unwrap();
   
       let grid = lcs(&lines1, &lines2);
   
       print_diff(&grid, &lines1, &lines2, lines1.len(), lines2.len());
   }
   ```

### Option rwc

```rust
use std::{env, io};
use std::fs::File;
use std::io::{BufRead, Read};
use std::process;

fn main() {
  	// 读取参数
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Too few arguments.");
        process::exit(1);
    }
    let filename = &args[1];

    // 打开文件
    let file = File::open(filename).unwrap();
    let mut reader = io::BufReader::new(file);

    // 初始化计数器
    let mut cnt_lines = 0;
    let mut cnt_words = 0;
    let mut cnt_chs = 0;

    // 读出文件的全部内容到contents中
    let mut contents = String::new();
    reader.read_to_string(&mut contents).unwrap();

    cnt_lines = contents.lines().count();

    // split_whitespace按空格、换行分割字符串并返回一个迭代器
    for word in contents.split_whitespace() {
        cnt_words += 1;
        cnt_chs += word.chars().count(); // 这里要使用chars().count()，而不是len()
    }
		
  	// 如果contents为空，行数应当设置为0
    if contents.is_empty() {
        cnt_lines = 0;
    }
	
    println!("lines: {}", cnt_lines);
    println!("words: {}", cnt_words);
    println!("characters: {}", cnt_chs);
}
```



# Projects

