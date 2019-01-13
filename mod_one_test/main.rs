mod hello; // importing in the other file {hello.rs}

fn main() {
    println!("Hello from Main");
    hello::print_hello();
}