extern "C" {
    fn abs(input: i32) -> i32;
}
fn main() {
    let mut num = 5;

    // Raw pointer as immut reference
    let r1 = &num as *const i32;

    // Raw pointer as mut ref
    let r2 = &mut num as *mut i32;

    let address = 0x012345usize;

    // Raw pointer to an arbitrary address
    let _r = address as *const i32;

    // This will probably segmentation fault
    // unsafe {
    //     println!("{}", *_r);
    // }

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    // Static mut: unsafe
    static mut COUNTER: u32 = 0;

    fn add_to_count(inc: u32) {
        unsafe {
            COUNTER += inc;
        }
    }

    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}
