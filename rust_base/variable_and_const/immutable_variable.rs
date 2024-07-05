fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    x = 6; // error cannot assign twice to immutable
    println!("The value of x is: {}", x);
}
