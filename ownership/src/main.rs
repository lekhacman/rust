
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

    let mut another_string = String::from("Hello Man");
    // get_length does not own another_string
    let another_string_len = get_length(&another_string);
    println!("length of '{}' is {}", another_string, another_string_len);

    append_sth(&mut another_string);
    println!("length of '{}' is {}", another_string, another_string_len);

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

fn get_length(str: &String) -> usize {
    str.len()
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn append_sth(s: &mut String) {
    s.push_str(" Le");
}
