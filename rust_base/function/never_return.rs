fn return_empty() {}

fn statement_last(x: u32, y: u32) {
    let _ = x + y;
}

fn expression_last(x: u32, y: u32) {
    x + y;
}

fn main() {
    println!("{}", return_empty() == ()); // true
    println!("{}", statement_last(1, 2) == ()); // true
    println!("{}", expression_last(1, 2) == ()); // true  ,but warning
}
