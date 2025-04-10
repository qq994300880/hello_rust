// Rust 所有权系统示例
pub fn run() {
    println!("所有权(Ownership)是Rust最独特的特性，它让Rust无需垃圾回收就能保证内存安全");
    
    // 所有权规则：
    // 1. Rust 中每个值都有一个变量，称为其所有者
    // 2. 一个值只能有一个所有者
    // 3. 当所有者超出作用域时，该值将被删除

    ownership_basics();
    ownership_move();
    ownership_clone();
    references_and_borrowing();
    mutable_references();
    slices();
}

fn ownership_basics() {
    println!("\n--- 所有权基础 ---");
    
    // 变量作用域
    {
        let s = "hello"; // s 从这里开始有效
        println!("s = {}", s);
    } // 此作用域结束，s 不再有效
    
    // String 类型的所有权
    let mut s = String::from("hello"); // s 是可变的
    s.push_str(", world!"); // 向字符串添加内容
    println!("{}", s);
} // 这里 s 离开作用域并调用 `drop` 方法，内存被释放

fn ownership_move() {
    println!("\n--- 所有权移动 ---");
    
    let s1 = String::from("hello");
    let s2 = s1; // s1 的值移动到 s2，s1 不再有效
    
    // println!("{}", s1); // 这会导致编译错误，因为 s1 的值已被移动
    println!("s2 = {}", s2);
    
    // 整数等简单类型是复制而不是移动
    let x = 5;
    let y = x; // x 的值被复制到 y，x 仍然有效
    println!("x = {}, y = {}", x, y);
}

fn ownership_clone() {
    println!("\n--- 深度克隆 ---");
    
    let s1 = String::from("hello");
    let s2 = s1.clone(); // 深度复制 s1 的数据到 s2
    
    println!("s1 = {}, s2 = {}", s1, s2); // 两者都有效
}

fn references_and_borrowing() {
    println!("\n--- 引用和借用 ---");
    
    let s1 = String::from("hello");
    
    // &s1 创建一个指向 s1 的引用，但不获取所有权
    let len = calculate_length(&s1);
    
    println!("字符串 '{}' 的长度是 {}", s1, len); // s1 仍然有效
}

// 这个函数接受一个字符串引用作为参数
fn calculate_length(s: &String) -> usize {
    s.len()
} // 这里 s 离开作用域，但它只是一个引用，所以什么都不会发生

fn mutable_references() {
    println!("\n--- 可变引用 ---");
    
    let mut s = String::from("hello");
    
    change(&mut s); // 传递可变引用
    println!("修改后: {}", s);
    
    // 可变引用的限制：在同一时间只能有一个可变引用
    {
        let r1 = &mut s;
        // let r2 = &mut s; // 这会导致编译错误
        println!("{}", r1);
    } // r1 在这里离开作用域，所以我们可以创建一个新的引用
    
    // 在 r1 的作用域结束后，可以创建新的可变引用
    let r2 = &mut s;
    println!("{}", r2);
    
    // 不能同时拥有可变引用和不可变引用
    let mut s2 = String::from("hello");
    
    {
        let r1 = &s2; // 不可变引用1
        let r2 = &s2; // 不可变引用2
        println!("{} and {}", r1, r2);
    } // r1 和 r2 在这里离开作用域
    
    let r3 = &mut s2; // 可变引用有效
    println!("{}", r3);
}

fn change(s: &mut String) {
    s.push_str(", world");
}

fn slices() {
    println!("\n--- 切片类型 ---");
    
    let s = String::from("hello world");
    
    let hello = &s[0..5]; // 或者写成 &s[..5]
    let world = &s[6..11]; // 或者写成 &s[6..]
    
    println!("first = {}, second = {}", hello, world);
    
    // 字符串字面量就是切片
    let s2: &str = "Hello, world!";
    println!("{}", s2);
    
    // 其他切片
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    println!("数组切片: {:?}", slice);
} 