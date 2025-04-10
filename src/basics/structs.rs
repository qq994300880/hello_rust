// Rust 结构体示例
pub fn run() {
    println!("结构体是一种自定义数据类型，允许命名和包装多个相关的值");
    
    // 定义结构体
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }
    
    // 创建结构体实例
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    
    // 修改结构体字段（需要整个实例是可变的）
    user1.email = String::from("anotheremail@example.com");
    println!("用户邮箱: {}", user1.email);
    
    // 使用函数创建结构体实例
    let user2 = build_user(
        String::from("user2@example.com"),
        String::from("user2"),
    );
    println!("用户名: {}", user2.username);
    
    // 使用结构体更新语法，从其他实例创建实例
    let user3 = User {
        email: String::from("user3@example.com"),
        username: String::from("user3"),
        ..user1 // 其余字段从 user1 获取
    };
    println!("用户激活状态: {}", user3.active);
    
    // 元组结构体
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    
    println!("黑色RGB: ({}, {}, {})", black.0, black.1, black.2);
    println!("原点坐标: ({}, {}, {})", origin.0, origin.1, origin.2);
    
    // 使用结构体方法
    #[derive(Debug)] // 添加调试功能
    struct Rectangle {
        width: u32,
        height: u32,
    }
    
    impl Rectangle {
        // 方法，第一个参数是 self
        fn area(&self) -> u32 {
            self.width * self.height
        }
        
        // 带参数的方法
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
        
        // 关联函数，没有 self 参数（类似于静态方法）
        fn square(size: u32) -> Rectangle {
            Rectangle {
                width: size,
                height: size,
            }
        }
    }
    
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    
    let square = Rectangle::square(25); // 调用关联函数
    
    println!("rect1的面积: {}", rect1.area());
    println!("rect1能否容纳rect2: {}", rect1.can_hold(&rect2));
    println!("正方形: {:?}", square); // 使用调试格式打印
}

fn build_user(email: String, username: String) -> User {
    User {
        email,      // 简写，等同于 email: email
        username,   // 简写，等同于 username: username
        active: true,
        sign_in_count: 1,
    }
}

// 为了能在函数外部使用
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
} 