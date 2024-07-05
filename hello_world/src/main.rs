fn main() {
    let southern_germany: &str = "Grüß Gott!";
    let chinese: &str = "世界，你好";
    let english: &str = "World, hello";
    let regions: [&str; 3] = [southern_germany, chinese, english];
    for region in regions.iter() {
        println!("{}", &region);
    }
}
