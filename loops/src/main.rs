fn main() {
    // loop

    // 使用键盘快捷键Ctrl+C来终止无限循环
    // loop {
    //     println!("again!");
    // }

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2; //amazing!!! 使用break关键字返回counter * 2
        }
    };

    println!("The result is: {}", result);

    // while

    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number = number - 1;
    }

    print!("LIFTOFF!!!");

    // for

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);

        index = index + 1;
    }

    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
