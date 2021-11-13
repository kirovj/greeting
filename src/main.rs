struct Coordinate {
    left: u32,
    top: u32,
    right: u32,
    bottom: u32,
}

fn build_coordinate(left: u32, top: u32, right: u32, bottom: u32) -> Coordinate {
    assert!(right > left && bottom > top);
    Coordinate {
        left,
        top,
        right,
        bottom,
    }
}

fn area(coordinate: &Coordinate) -> u32 {
    (coordinate.right - coordinate.left) * (coordinate.bottom - coordinate.top)
}

fn main() {
    let coordinate = build_coordinate(4, 3, 5, 7);
    let size = area(&coordinate);
    println!("size: {}", size);
}
