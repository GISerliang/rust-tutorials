fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // 索引值会被绑定到变量上
    s.clear(); // 使用clear方法会清空当前字符串，使之为“”

    // 虽然word依然拥有值，但是由于原字符串改变了，因此这个索引也没有任何意义了，word便失去了有效性

    // 范围语法..有一个小小的语法糖：当你希望范围从第一个元素（也就是索引值为0的元素）开始时，则可以省略两个点号之前的值
    // 切片想要包含String中的最后一个字节，可以省略双点号之后的值
    let s= String::from("hello world");
    let hello = &s[..5];
    let world = &s[6..];

    let s = String::from("hello world");
    let word = first_word2(&s);
    // s.clear(); // error: mutable borrow occurs here
    println!("the first word is : {}", word);

    let my_string = String::from("hello world");
    // first_word2 可以接收String对象的切片作为参数
    let word = first_word2(&my_string[..]);

    let my_string_literal = "hello world";
    // first_word2可以接收字符串字面量的切片作为参数
    let word = first_word2(&my_string_literal[..]);

    // 由于字符串字面量本身就是切片，所以可以在这里直接将它传入函数，而不需要使用额外的切片语法
    let word = first_word2(my_string_literal);

    // 其他切片
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word2(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
