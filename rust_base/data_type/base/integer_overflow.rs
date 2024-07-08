/*
* wrapping_* 方法在所有模式下都按照补码循环溢出规则处理
* checked_* 方法时发生溢出，则返回 None 值
* overflowing_* 方法返回该值和一个指示是否存在溢出的布尔值
* saturating_* 方法，可以限定计算后的结果不超过目标类型的最大值或低于最小值，例如:
*/
fn main() {
    let a: u8 = 255;
    let b = a.overflowing_add(20);
    println!("{}", a.wrapping_add(20)); // 19
    println!("{:?}", b); // (19, true)
    println!("{}", a.saturating_add(3)); // 255
}
