unsafe extern "C" {
    safe fn abs(input: i32) -> i32;
}

static mut ABS_ORIGINAL: i32 = 0;

unsafe fn add_to_count(inc: i32) {
    unsafe {
        ABS_ORIGINAL += inc;
    }
}

fn main() {
    println!("Hello, world!");
    println!("abs(-3) = {}", abs(-3));

    unsafe {
        add_to_count(100);
        println!("ABS_ORIGINAL = {}", *(&raw const ABS_ORIGINAL));
    }
}
