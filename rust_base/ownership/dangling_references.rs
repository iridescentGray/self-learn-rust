/*
悬垂指针
指针指向某个值后，值被释放掉了
 */
fn main() {
    let reference_to_nothing = dangle();
}

//错误: 产生悬垂指针
fn dangle() -> &String {
    let s = String::from("hello");

    &s //error[E0106]: missing lifetime specifier
}
//正确:所有权转移
fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
