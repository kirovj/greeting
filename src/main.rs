fn main() {
    // gives_ownership 将返回值移给 s1
    let s1 = gives_ownership();

    // s2 进入作用域
    let s2 = String::from("hello");

    // s2 被移动到takes_and_gives_back 中,它也将返回值移给 s3
    let s3 = takes_and_gives_back(s2);

    // println!("{}, {}, {}", s1, s2, s3); borrow of moved value: `s2`
} // 这里, s3 移出作用域并被丢弃。s2 也移出作用域，但已被移走，
  // 所以什么也不会发生。s1 移出作用域并被丢弃

fn gives_ownership() -> String {
    // gives_ownership 将返回值移动给调用它的函数

    // some_string 进入作用域.
    let some_string = String::from("hello");
    some_string // 返回 some_string 并移出给调用的函数
}

// takes_and_gives_back 将传入字符串并返回该值
fn takes_and_gives_back(a_string: String) -> String {
    // a_string 进入作用域
    a_string // 返回 a_string 并移出给调用的函数
}
