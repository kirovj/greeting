#[derive(Debug)]
struct Coordinate {
    left: u32,
    top: u32,
    right: u32,
    bottom: u32,
}

impl Coordinate {
    fn build(left: u32, top: u32, right: u32, bottom: u32) -> Coordinate {
        assert!(right > left && bottom > top);
        Coordinate {
            left,
            top,
            right,
            bottom,
        }
    }
    fn area(&self) -> u32 {
        (self.right - self.left) * (self.bottom - self.top)
    }

    fn contain(&self, other: &Coordinate) -> bool {
        self.left < other.left
            && self.right > other.right
            && self.top < other.top
            && self.bottom > other.bottom
    }
}

fn main() {
    let coordinate = Coordinate::build(4, 3, 11, 17);
    let size = coordinate.area();
    let c2 = Coordinate::build(5, 4, 7, 9);
    println!("coordinate: {:#?}, size: {}", coordinate, size);
    println!("contains ? {}", coordinate.contain(&c2));
}
