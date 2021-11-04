use std::thread;
use std::time;
fn main() {
    another_function(8, 3 * five());
    loop {
        println!("Halo");
        thread::sleep(time::Duration::new(5, 0));
    }
}

fn another_function(x: i64, y: i64) {
    println!("Another function {} {}", x, y);
}

fn five() -> i64 {
    5
}
