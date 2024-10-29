fn main() {
    let my_str: &str = "Hello, world!";

    println!("{}", my_str);

    print_message(my_str);
}

fn print_message(message: &str) {
    println!("Message: {}", message);
}
