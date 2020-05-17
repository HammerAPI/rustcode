use add_one;
use rand::Rng;

fn main() {
    let num = rand::thread_rng().gen_range(1, 101);
    println!("{} plus one is {}", num, add_one::add_one(num));
}
