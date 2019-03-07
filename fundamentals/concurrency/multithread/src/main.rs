use std::time::Duration;
use std::thread;

fn main() {
    println!("Num of thread: {}", thread::);
    thread::spawn(||  {
        for i in 1..10 {
            println!("from spawned thread {}", i);
            thread::sleep(Duration::from_millis(1))
        }
    });

    for i in 1..5 {
        println!("from main thread {}", i);
        thread::sleep(Duration::from_millis(1))
    }
}
