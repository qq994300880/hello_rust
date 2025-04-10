// Rust 变量和基础语法示例
pub fn run() {
    // 使用 let 声明变量，默认不可变
    let a = 10;
    println!("a = {}", a);

    // 使用 mut 声明可变变量
    let mut b = 20;
    println!("b = {}", b);
    b = 30;
    println!("b = {}", b);

    // 显式指定类型
    let c: i32 = 40;
    println!("c = {}", c);

    // 变量遮蔽(shadowing)
    let d = 50;
    let d = d + 5; // 这里是一个新的变量d，而不是修改原来的d
    println!("d = {}", d);

    // 常量
    const MAX_VALUE: u32 = 100_000;
    println!("常量 MAX_VALUE = {}", MAX_VALUE);

    // 元组解构
    let tup = (500, 6.4, "元组");
    let (x, y, z) = tup;
    println!("解构: x = {}, y = {}, z = {}", x, y, z);

    // 函数调用
    let sum = add(a, b);
    println!("{} + {} = {}", a, b, sum);

    // 字符串示例
    let hello = "你好，Rust！";
    println!("{}", hello);
}

// 定义一个加法函数
fn add(x: i32, y: i32) -> i32 {
    x + y // 注意这里没有分号，表示返回这个表达式的值
} 