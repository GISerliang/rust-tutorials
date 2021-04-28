fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}", s2, len);

    let s1 = String::from("hello");

    let len = calculate_length_2(&s1); // 不获取s1所有权使用s1

    println!("The length of '{}' is {}", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len会返回当前字符串长度

    (s, length)
}

fn calculate_length_2(s: &String) -> usize {
    s.len()
}

// // expected named lifetime parameter
// fn dangle() -> &String {
//     let s = String::from("hello");
//
//     &s
// }
fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
