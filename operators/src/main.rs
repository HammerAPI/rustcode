fn main() {
    println!("1u32 + 2 = {}", 1u32 + 2);
    println!("1i32 - 2 = {}", 1i32 - 2);
    // Throws a compile error because u32 cannot be negative
    //println!("1u32 - 2 = {}", 1u32 - 2);

    println!("true && false == {}", true && false);
    println!("true || false == {}", true || false);

    println!("One million is {}", 1_000_000u32);
}
