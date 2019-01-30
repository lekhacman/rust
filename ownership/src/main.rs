fn main() {
    let s1 = gives_ownership();

    takes_ownership(s1);

    let s2 = String::from("World");

    let s2 = takes_n_gives_back(s2);

    println!("{}", s2);

    let x = 5;
    makes_copy(x);

    println!("{}", x);

    let text = String::from("Hello World!");
    let (s4, len) = calculate_length(text);
    println!("length of '{}' is {}", s4, len);
}

fn takes_ownership(str: String) {
    println!("{}", str);
}

fn makes_copy(int: i32) {
    println!("{}", int);
}

fn gives_ownership() -> String {
    let str = String::from("Hello");
    str
}

fn takes_n_gives_back(str: String) -> String {
    str
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
