mod my_hello;

fn main() {
    let message = my_hello::build_message();
    println!("{}", message);
}
