#[derive(Debug)]
enum Message {
    _Quit,
    Move { x: i32, y: i32 },
    Write(String),
    _ChangeColor(i32, i32, i32),
}

impl Message {
    fn show(&self) {
        println!("{:?}", self);
    }
}

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    // Enums
    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));

    println!("IPv4: {:?}\nIPv6: {:?}", home, loopback);

    let m = Message::Write(String::from("hello"));
    m.show();

    let m2 = Message::Move { x: 5, y: 5 };
    m2.show();

    // Option
    let _some_number = Some(5);
    let _some_string = Some("a string");

    let mut _absent_number: Option<i32> = None;
    // _absent_number = 5; wrong
    _absent_number = Some(5);

    // Fails:
    // let x: i8 = 5;
    // let y: Option<i8> = Some(5);
    // let sum = x + y;
}
