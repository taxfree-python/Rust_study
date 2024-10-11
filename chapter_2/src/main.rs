fn func_ex_div_some(x: i32, y: i32) -> Option<i32> {
    // Option<T> は T 型の値を Some() で包んだものか None のどちらか
    if y == 0 {
        // if 分岐は式なので";"不要
        None
    } else {
        Some(x / y)
    }
}

fn func_ex_div_result(x: i32, y: i32) -> Result<i32, &'static str> {
    // エラーハンドリングには Result<T, E> がよく用いられるらしい
    if y == 0 {
        Err("div by zero")
    } else {
        Ok(x / y)
    }
}

// fn func_ex_print_some<T: std::fmt::Display>(ans: Option<T>) -> () {  // std::fmt::Display を方パラメータで指定するのは println! で使えるようにするため
fn func_ex_print_some(ans: Option<i32>) {
    // 今回の場合は func_ex_div_some の返り値を考えるとこれでいける
    if let Some(x) = ans {
        println!("{}", x)
    } else {
        // ans が None の場合
        println!("None")
    }
}

fn func_ex_print_some_match<T: std::fmt::Display>(ans: Option<T>) {
    match ans {
        Some(x) => println!("{}", x),
        None => println!("None"),
    }
}

fn func_ex_print_result<T: std::fmt::Display, E: std::fmt::Display>(ans: Result<T, E>) {
    match ans {
        Ok(res) => println!("{}", res),
        Err(str) => println!("{}", str),
    }
}

fn main() {
    func_ex_print_some(func_ex_div_some(10, 5));
    func_ex_print_some(func_ex_div_some(10, 0));
    func_ex_print_some_match(func_ex_div_some(10, 5));
    func_ex_print_some_match(func_ex_div_some(10, 0));
    func_ex_print_result(func_ex_div_result(10, 5));
    func_ex_print_result(func_ex_div_result(10, 0));
}
