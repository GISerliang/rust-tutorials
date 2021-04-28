#[derive(Debug)] // 添加注解来派生Debug trait
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    let width1 = 30;
    let height1 = 50;

    println!("The area of the rectangle is {} square pixels.", area(width1, height1));

    let rect1 = (30, 50);
    println!("The area of the rectangle is {} square pixels.", area1(rect1));

    let rect1 = Rectangle { width: 30, height: 50 };
    println!("The area of the rectangle is {} square pixels.", area2(&rect1));

    let rect1 = Rectangle { width: 30, height: 50 };
    // 使用名为Display的格式化方法：这类输出可以被展示给直接的终端用户。
    println!("rect1 is {:?}", rect1);
    println!("rect1 is {:#?}", rect1);

    let rect1 = Rectangle { width: 30, height: 50 };
    println!("The area of the rectangle is {} square pixels.", rect1.area());

    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let square = Rectangle::square(20);
    println!("square is {:#?}", square);
}

// 这两个参数是相互关联的，但程序中却没有任何地方可以表现出这一点。
fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area1 (dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

// 在函数签名和调用过程中使用&进行引用是因为希望借用结构体，而不是获取它的所有权
fn area2 (rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
