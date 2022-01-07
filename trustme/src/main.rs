fn main() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1: {}", *r1);
        *r2 = 10;
        println!("r1: {}", *r2);
    }

    let address = 0x012345usize;
    let r = address as *const i32;
    unsafe {
        println!("r: {}", *r); // panic
    }
}
