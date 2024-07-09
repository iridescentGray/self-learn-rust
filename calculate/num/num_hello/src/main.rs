use num::rational::Ratio;

fn main() {
    let a = Ratio::new(1, 10); //0.1
    let b = Ratio::new(2, 10); //0.2
    let expected = Ratio::new(3, 10); //0.3
    println!("{}", a); //1/10
    println!("{}", b); //1/5
    println!("{}", (a + b == expected)); //true
}
