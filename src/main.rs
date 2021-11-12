struct Color(i32, i32, i32);

struct Point(i32, i32, i32);

struct AlwaysEqual;

fn main() {
    let black = Color(0, 0, 0);

    let point = Point(1, 2, 4);

    let subject = AlwaysEqual;
}
