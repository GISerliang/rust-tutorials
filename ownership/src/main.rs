fn main() {
    let s = String::from("hello"); // 变量s进入作用域
    takes_ownership(s); // s的值被移动进入了函数 s从这里开始不再有效

    // println!("The value of s is: {}", s); // error: value borrowed here after move

    let x = 5; // 变量x进入作用域
    makes_copy(x); // 变量x同样被传递进了函数，但由于i32具有Copy trait特性，因此在此之后依然可以使用x

    let s1 = gives_ownership(); // gives_ownership将它的返回值移动到s1中

    let s2 = String::from("hello"); // s2 进入作用域

    let s3 = takes_and_gives_back(s2); // s2被移动到函数takes_and_gives_back中，而这个函数的返回值又被移动到了变量s3上
} // s3在这里离开作用域并被销毁，由于s2已经移动，所以它不会在离开作用域时发生任何事情。s1最后离开作用域并被销毁
// 作用域的范围就是在两个{}之间，即便变量所有权转移了（即变量不可用，如第五行的变量s），该变量依然在作用域内
// 如第二行声明的变量s，在第三行转移了所有权无法访问，但是变量s的作用域也是在第15行之后才离开作用域，但不触发销毁操作，因为在离开takes_ownership之后就已经被销毁

fn takes_ownership(some_string: String) { // some_string进入作用域
    println!("{}", some_string);
} // some_string在这里离开作用域，drop函数被自动调用，some_string所占用的内存也被释放

fn makes_copy(some_integer: i32) { // some_integer进入作用域
    println!("{}", some_integer);
} // some_integer在这里离开作用域，没什么特别的发生

fn gives_ownership() -> String { // gives_ownership会将它的返回值移动至调用它的函数
    let some_string = String::from("hello"); // some_string进入作用域
    some_string // some_string作为返回值移动到调用函数
}

// takes_and_gives_back将取得一个String的所有权并将它作为结果返回
fn takes_and_gives_back(a_string:String) -> String {
    // a_string进入作用域
    a_string // a_string作为返回值移动至调用函数
}
