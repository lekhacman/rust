fn main() {
    let mut other_v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3, 4];

    other_v.push(5);
    other_v.push(6);
    other_v.push(7);
    other_v.push(8);

    let third = &v[2];
    println!("Third: {}", third);

    match v.get(2) {
        Some(third) => println!("third {}", third),
        None => println!("No third element"),
    };

    for i in &v {
        println!("{}", i);
    }

    let mut vec = vec![100, 32, 57];
    for i in &mut vec {
        *i += 50;
    }

    for i in vec {
        println!("{}", i);
    }
}
