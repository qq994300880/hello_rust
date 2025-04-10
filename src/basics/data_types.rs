// Rust 基本数据类型示例
pub fn run() {
    // 整数类型
    let integer: i32 = 42;
    let unsigned: u32 = 100;
    let small: i8 = 127;
    let big: i64 = 1_000_000_000;
    
    println!("整数: {}, {}, {}, {}", integer, unsigned, small, big);
    
    // 浮点数类型
    let float: f32 = 3.14;
    let double: f64 = 3.141592653589793;
    
    println!("浮点数: {}, {}", float, double);
    
    // 布尔类型
    let is_true: bool = true;
    let is_false: bool = false;
    
    println!("布尔值: {}, {}", is_true, is_false);
    
    // 字符类型 - 使用单引号
    let letter: char = 'A';
    let emoji: char = '😊';
    let chinese: char = '你';
    
    println!("字符: {}, {}, {}", letter, emoji, chinese);
    
    // 元组类型
    let tuple: (i32, f64, &str) = (500, 6.4, "元组");
    
    // 可以通过索引访问元组元素
    println!("元组: {}, {}, {}", tuple.0, tuple.1, tuple.2);
    
    // 可以通过解构获取元组中的值
    let (x, y, z) = tuple;
    println!("解构: {}, {}, {}", x, y, z);
    
    // 数组类型 - 固定长度
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    
    println!("数组: {}, {}, {}", array[0], array[2], array[4]);
    
    // 字符串类型
    let string_literal: &str = "这是一个字符串字面量";
    let string: String = String::from("这是一个String类型");
    
    println!("字符串: {}, {}", string_literal, string);
} 