fn main() {
    let mut v = vec![1, 2, 3, 4, 5];

    for item in &mut v {
        *item += 1;
        if *item == 1 {
            do_something(item);
        }
    }
    println!("{:?}", v);
}

fn do_something(param: &mut i32) {
    println!("dropping {}", param);
}
