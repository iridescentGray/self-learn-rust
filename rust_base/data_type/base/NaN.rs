fn main() {
    let x = (-42.0_f32).sqrt();
    println!("{}", x); // NaN
    println!("{}", x == x); // false
}
