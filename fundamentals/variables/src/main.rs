fn main() {
    let x = plus_one(2);
    println!("x: {}", x);
    greeting("Man");

    if x > 2 {
        println!("x is greater than 2");
    } else {
        println!("x is not greater than 2");
    }
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn greeting(name: &str) {
    println!("Hello {}", name);
}
