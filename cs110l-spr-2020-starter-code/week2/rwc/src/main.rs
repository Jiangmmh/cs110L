use std::{env, io};
use std::fs::File;
use std::io::{BufRead, Read};
use std::process;

fn main() {
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

    if contents.is_empty() {
        cnt_lines = 0;
    }

    println!("lines: {}", cnt_lines);
    println!("words: {}", cnt_words);
    println!("characters: {}", cnt_chs);
}
