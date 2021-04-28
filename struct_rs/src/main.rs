struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// struct User1 {
//     username: &str, // error: missing lifetime specifier
//     email: &str,
//     sign_in_count: u64,
//     active: bool,
// }

fn main() {
    let user1 = User {
        email: QString::from("someone@example.com"),
        username: QString::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let mut user1 = User {
        email: QString::from("someone@example.com"),
        username: QString::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com");

    let user2 = User {
        email: QString::from("another@example.com"),
        username: QString::from("anotherusername123"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };
    // 使用结构体更新语法来为一个User实例设置新的email和username字段的值，并从user1实例中获取剩余字段的值。
    let user2 = User {
        email: QString::from("another@example.com"),
        username: QString::from("anotherusername123"),
        ..user1
    };

    // 元组结构体
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    // 这里的black和origin是不同的类型，因为它们两个分别是不同元组结构体的实例。
    // 定义的每一个结构体都拥有自己的类型，即便结构体中的字段拥有完全相同的类型。
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // Rust允许我们创建没有任何字段的结构体！
    // 当你想要在某些类型上实现一个trait，却不需要在该类型中存储任何数据时，空结构体就可以发挥相应的作用。

    // 结构体所有权
    // User结构体定义中，我们使用了自持所有权的String类型而不是&str字符串切片类型。这是一个有意为之的选择，因为我们希望这个结构体的实例拥有自身全部数据的所有权。
    // 在这种情形下，只要结构体是有效的，那么它携带的全部数据也就是有效的。当然，我们也可以在结构体中存储指向其他数据的引用，不过这需要用到Rust中独有的生命周期功能
    // 生命周期保证了结构体实例中引用数据的有效期不短于实例本身。

    // let user1 = User1 {
    //     email: QString::from("someone@example.com"),
    //     username: QString::from("someusername123"),
    //     active: true,
    //     sign_in_count: 1,
    // };
}

// build_user函数中使用了相同的参数名与字段名，并采用了字段初始化简写语法进行编写
fn build_user(email: String, username: String) -> User {
    // User {
    //     email: email,
    //     username: username,
    //     active: true,
    //     sign_in_count: 1,
    // }
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
