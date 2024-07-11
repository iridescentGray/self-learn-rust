/*
当 s1 被赋予 s2(把所有权从 s1 转移给了 s2)，Rust 会认为 s1 不再有效，使用s1会报错
*/
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{}, world!", s1); //error[E0382]: borrow of moved value: `s1`
}
