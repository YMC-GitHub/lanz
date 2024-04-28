#[allow(clippy::single_component_path_imports)]
use {add_one, add_two};

fn main() {
    let num = 10;
    println!("Hello, world! {} + 1 = {}", num, add_one::add(num));
    println!("Hello, world! {} + 2 = {}", num, add_two::add(num));
}
