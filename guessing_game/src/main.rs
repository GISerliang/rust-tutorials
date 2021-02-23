use std::io;
// 引入标准库(std)的io模块
use std::cmp::Ordering;
// Ordering用于比较大小
use rand::Rng; // 引入随机数生成器的方法集合

fn main() {
  println!("Guess the number!");

  let secret_number = rand::thread_rng().gen_range(1, 101); // 生成[1,101)的随机数

  // println!("The secret number is: {}", secret_number);

  loop { // 循环游戏
    println!("Please input your guess.");

    // 用let创建存储输入数据的变量, mut关键字指定其可变
    // String::new() 返回一个String实例，String是字符串类型，使用UTF-8编码
    let mut guess = String::new();

    // 使用标准库io模块的read_line方法获取用户输入，并把输入放入到guess参数，返回io::Result值。
    // Result是一个枚举类型。枚举类型由一系列固定的值组合而成，这些值被称作枚举的变体。Result拥有Ok和Err两个变体。
    // 其中的Ok变体表明当前的操作执行成功，并附带代码产生的结果值。相应地，Err变体则表明当前的操作执行失败，并附带引发失败的具体原因。
    // 如果返回值是Err，程序则会中断，如果返回Ok，则将用户输入返回给参数。如果不增加except，cargo编译时将会有警告信息提示存在潜在的错误Result值未被处理。
    // 参数前面的&意味着当前的参数是一个引用。代码可以通过引用在不同的地方访问同一份数据，而无须付出多余的拷贝开销。
    io::stdin().read_line(&mut guess)
               .expect("Failed to read line."); // 处理异常

    // 使用相同的变量名来shadow旧的变量
    // 此时let声明的guess随与上面的guess名字一样，但不是同一个
    // 将上面获取的用户输入去除空格(trim)并转换为u32类型(parse)
    // parse根据声明时显示指定的u32类型将输入值转换为u32类型
    let guess: u32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(_) => continue, // 输入非数字则忽略本次猜测，继续游戏
    };

    println!("You guessed: {}", guess); // 打印用户输入，{}为占位符，用于将后面的参数插入到预留位置。

    match guess.cmp(&secret_number) {
      Ordering::Less => println!("Too small!"),
      Ordering::Greater => println!("Too big!"),
      Ordering::Equal => {
        println!("You win!");
        break; //猜对之后跳出循环
      },
    }
  }
}
