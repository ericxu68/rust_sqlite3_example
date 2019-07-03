extern crate chrono;
use chrono::prelude::*;

pub fn print_time() {
    let utc: DateTime<Utc> = Utc::now();
    dbg!(utc);
    println!("{}", utc.format("%Y-%m-%d %H:%M:%S"))
}
