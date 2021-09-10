struct Vec2 {
    x: f64, // 64-bit floating point, aka "double precision"
    y: f64,
}

fn main() {
    // let v1 = Vec2 { x: 1.0, y: 3.0 };
    let v2 = Vec2 { y: 2.0, x: 4.0 };
    let v3 = Vec2 {
        x: 14.0,
        ..v2
    };
    let v4 = Vec2 { ..v3 };
    println!("v3  {}: {}", v3.x, v3.y);
    println!("v4  {}: {}", v4.x, v4.y);
}