enum WebEvent {
    // An enum variant can be like a unit struct without fields or data types
    WELoad,
    // An enum variant can be like a tuple struct with data types but no named fields
    WEKeys(String, char),
    // An enum variant can be like a classic struct with named fields and their data types
    WEClick { x: i64, y: i64 },
}

// WELoad 没有关联的数据类型或数据。
// WEKeys 具有两个数据类型分别为 String 和 char 的字段。
// WEMClick 包含命名字段为 x 和 y 以及字段的数据类型为 i64 的匿名结构。

fn main() {
    let formal = true;
    let greeting = if formal {
        // "if" keyword used here as an expression
        "Good day to you." // Returns the string "Good day to you."
    } else {
        "Hey!" // Returns the string "Hey!"
    };
    println!("{}", greeting) // Prints "Good day to you."
}
