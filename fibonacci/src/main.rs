fn main() {
    let n = 10;

    println!("The {}th fibonacci number is {}", n, fib(n));
}

fn fib(num: i32) -> i32 {
    if num == 1 || num == 0 {
        num
    } else {
        fib(num - 2) + fib(num - 1)
    }
}
