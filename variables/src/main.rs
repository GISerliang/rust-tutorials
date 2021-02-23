fn main() {
  let mut x = 5;
  println!("The value of x is: {}", x);

  x = 6; // error: cannot assign twice to immutable variable
  println!("The value of x is: {}", x);

  // 不能用mut关键字来修饰一个常量。
  // 常量不仅是默认不可变的，它还总是不可变的。
  // 使用const关键字而不是let关键字来声明一个常量，必须显式地标注值的类型
  const M_PI: f64 = 3.14159;

  // 浮点型
  let x = 2.0; // 默认f64
  let y: f32 = 3.0; // 指定f32

  // 数值运算
  let sum = 5 + 10; // 加法
  let difference = 95.5 - 4.3; // 减法
  let product = 4 * 30; // 乘法
  let quotient = 56.7 / 32.2; // 除法
  let remainder = 43 % 5; // 取余

  // 布尔值
  let t = true;
  let f: bool = false; // 显示声明

  // 字符
  // char类型占4字节，是一个Unicode标量值
  // 拼音字母、中文、日文、韩文、零长度空白字符，甚至是emoji表情都可以作为一个有效的char类型值。
  let c = 'z';
  // let z = '';
  // let heart_eyed_cat = '';

  // 复合类型

  // 元组类型
  // 无法在声明结束后增加或减少其中的元素数量。
  let tup: (i32, f64, u8) = (500, 6.4, 1);

  // 解构
  let (x, y, z) = tup;
  println!("The value of x is: {}, y is: {}, z is: {}", x, y, z);

  let x: (i32, f64, u8) = (500, 6.4, 1);
  let five_hundred = x.0;
  let six_point_four = x.1;
  let one = x.2;

  // 数组
  // 一旦声明就再也不能随意更改大小
  // 想在栈上而不是堆上为数据分配空间时，或者想要确保总有固定数量的元素时，数组是一个非常有用的工具。
  let a = [1, 2, 3, 4, 5];
  // i32便是数组内所有元素的类型，而分号之后的5则表明当前的数组包含5个元素
  let a: [i32; 5] = [1, 2, 3, 4, 5];
  // 组将会拥有5个元素，而这些元素全部拥有相同的初始值3
  // 等价于let a = [3, 3,3, 3, 3];
  let a = [3; 5];

  let a = [1, 2, 3, 4, 5];
  let first = a[0];
  let second = a[1];

  let index = 10;
  let element = a[index];
  println!("The value of element is: {}", element); // index out of bounds: the length is 5 but the index is 10
}
