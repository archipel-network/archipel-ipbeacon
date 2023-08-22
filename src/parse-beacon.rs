use std::io::Read;

use beacon::Beacon;
mod beacon;

fn main() {
    let mut buf = Vec::new();
    std::io::stdin().read_to_end(&mut buf).expect("pipe error");

    let b = Beacon::parse(&buf).expect("Parse error");

    println!("{:#?}", b);
}