fn main() {
    let x: i32 = add_five(3);

    println!("The value of 3 + 5 is {}", x);
}

fn add_five(a: i32) -> i32{
    5 + a
}
