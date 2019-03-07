use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("spawned"),
            String::from("thread"),
        ];

        vals.iter().for_each(  |val| {
            tx.send(val.to_string()).unwrap();
            thread::sleep(Duration::from_secs(1));
        })

//        for val in vals {
//            tx.send(val).unwrap();
//            thread::sleep(Duration::from_secs(1));
//
//        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
