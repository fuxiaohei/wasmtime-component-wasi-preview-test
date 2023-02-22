fn print() {
    let utc = time::OffsetDateTime::now_utc();
    println!("Hello, world!, {:?}", utc);
}

fn main() {
    print();
}
