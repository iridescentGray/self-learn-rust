fn main() {
    println!("汉字 占用了{}字节", std::mem::size_of_val(&'中'));
    println!("a 占用了{}字节", std::mem::size_of_val(&'a'));
    println!("😻 占用了{}字节", std::mem::size_of_val(&'😻'));
}
