#[derive(Debug)]
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

impl Coordinate {
    fn area(&self) -> u32 {
        (self.right - self.left) * (self.bottom - self.top)
    }
}

fn main() {
    let coordinate = build_coordinate(4, 3, 5, 7);
    let size = coordinate.area();
    println!("coordinate: {:#?}, size: {}", coordinate, size);
}
