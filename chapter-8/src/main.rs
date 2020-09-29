fn main() {
    let mut _test = [1, 2, 3];
    // test[5] = 5; Can't assign
    let mut vtest = _test.to_vec();
    vtest[5] = 5; // Lets assign but panics
    let mut v: Vec<i8> = vec![1, 2, 3];
    v.push(-1);
    println!("{:?} {:?}", v, vtest);
}

fn _strings() {
    // push_str
    let mut s = String::from("foo");
    s.push_str("bar");

    // push char
    let mut s = String::from("lo");
    s.push('l');

    // concat strings with +
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let _s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    // String slicing
    let hello = "Здравствуйте";
    // UTF-8 characters may have 1 byte or more than 1. That can cause issues when slicing strings.
    // String indexing is not allowed in Rust.
    let _s = &hello[0..4]; // OK: the first two characters are 2 bytes each, there fore s is Зд
    let _s = &hello[0..1]; // Will panic: as the character З is 2 bytes long, only getting 1 char will panic

    // Solution for unknown char size lengths: iterating
    for c in "नमस्ते".chars() {
        // iterating on chars
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() {
        // iterating on bytes
        println!("{}", b);
    }
}
