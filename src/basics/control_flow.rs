// Rust 控制流示例
pub fn run() {
    // if-else 条件语句
    let number = 6;
    
    if number % 4 == 0 {
        println!("数字可以被4整除");
    } else if number % 3 == 0 {
        println!("数字可以被3整除");
    } else if number % 2 == 0 {
        println!("数字可以被2整除");
    } else {
        println!("数字不能被2、3、4整除");
    }
    
    // 在 let 语句中使用 if
    let condition = true;
    let result = if condition { 5 } else { 6 };
    println!("result 的值为: {}", result);
    
    // 循环: loop
    let mut counter = 0;
    let result = loop {
        counter += 1;
        
        if counter == 10 {
            // 使用 break 返回值
            break counter * 2;
        }
    };
    println!("loop 结果: {}", result);
    
    // 循环标签：处理嵌套循环
    let mut count = 0;
    'outer: loop {
        println!("外层循环计数: {}", count);
        
        let mut remaining = 10;
        
        loop {
            println!("内层循环剩余: {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'outer;
            }
            remaining -= 1;
        }
        
        count += 1;
    }
    
    // while 循环
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("发射!");
    
    // for 循环
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("值为: {}", element);
    }
    
    // 使用 range
    for number in 1..4 {
        println!("{}!", number);
    }
    println!("发射!");
    
    // match 表达式
    let dice_roll = 3;
    match dice_roll {
        1 => println!("掷出了1"),
        2 => println!("掷出了2"),
        3 => println!("掷出了3"),
        4 => println!("掷出了4"),
        5 => println!("掷出了5"),
        6 => println!("掷出了6"),
        _ => println!("无效的骰子值"),
    }
    
    // if let 简单模式匹配
    let some_value = Some(3);
    if let Some(3) = some_value {
        println!("是3!");
    }
} 