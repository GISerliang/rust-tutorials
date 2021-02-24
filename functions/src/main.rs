fn main() {
  println!("Hello, world!");

  another_function();

  another_function_2(5);

  another_function_3(5, 6);

  let x = 5;
  let y = {
    let x = 3;
    x + 1 // 表达式x + 1没有添加分号，表示该表达式是一个返回值
  };
  println!("The value of y is: {}", y);

  let x = five();
  println!("The value of x is: {}", x);

  let x = plus_one(5);
  println!("The value of x is: {}", x);
}

fn another_function() {
  println!("Another function");
}

// 在函数签名中，必须显式地声明每个参数的类型。
fn another_function_2(x: i32) {
  println!("The value of x is: {}", x);
}

fn another_function_3(x: i32, y: i32) {
  println!("The value of x is: {}", x);
  println!("The value of y is: {}", y);
}

fn five() -> i32 {
  5
}

fn plus_one(x: i32) -> i32 {
  x + 1
}
