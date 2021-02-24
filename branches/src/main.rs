fn main() {
    // if 语句
    let number = 3;

    // 与Ruby或JavaScript等语言不同，Rust不会自动尝试将非布尔类型的值转换为布尔类型。
    // 必须显式地在if表达式中提供一个布尔类型作为条件。
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let number = 7;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // let number = 3;
    // expected `bool`, found integer
    // if number {
    //     println!("number was three");
    // }

    let number = 3;
    if number != 0 {
        println!("number was something more than zero");
    }

    let number = 6;
    // if 会且仅会执行第一个条件为真的代码块
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3")
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3 or 2");
    }

    let condition = true;
    // Rust 没有三目运算符， 此方式可替代
    // 此时需要保证if 和 else 返回值类型一致
    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is: {}", number);
}
