use std::fs::File;
use std::fs::read_to_string;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

fn main() {
//    let f = File::open("hello.txt");

//    let f = match f {
//        Ok(file) => file,
//        Err(error) => match error.kind() {
//            ErrorKind::NotFound => match File::create("hello.txt") {
//                Ok(fc) => fc,
//                Err(e) => panic!("Tried to create file but failed {:?}", e)
//            },
//            other_error => panic!("Problem opening the file {:?}", other_error)
//        }
//    };
//    let f = File::open("hello.txt")
//        .map_err(|err| {
//            if err.kind() == ErrorKind::NotFound {
//                File::create("hello.txt").unwrap_or_else(|err| {
//                    panic!("Tried to create file but failed {:?}", err);
//                })
//            } else {
//                panic!("Open file failed");
//            }
//        });
    match read_username_from_file() {
        Ok(s) => println!("Content {}", s),
        Err(e) => panic!("{:?}", e),
    }

}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
