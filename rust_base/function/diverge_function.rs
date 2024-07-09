use std::{thread, time};

fn forever() -> ! {
    loop {
        println!("{}", "hello");
        thread::sleep(time::Duration::from_millis(500));
    }
}

fn dead_end() -> ! {
    panic!("你已经到了穷途末路，崩溃吧！");
}

fn main() {
    forever()
}
