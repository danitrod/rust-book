fn main() {
    println!("Hello, world!");
    println!("{}", fib(5));
}

fn fib(n: u16) -> u16 {
    if n == 0 || n == 1 {
        return 1;
    } else {
        return fib(n - 1) + fib(n - 2);
    }
}
