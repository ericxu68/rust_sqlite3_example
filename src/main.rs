mod my_hello;
mod my_sqlite;
mod my_time;

fn main() {
    let message = my_hello::build_message();
    println!("{}", message);

    my_time::print_time();
    my_sqlite::run_sql().unwrap();
}
