fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);

    let a = [10, 20, 30, 40, 50];

    for e in a.iter() {
        println!("Value: {}", e);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
}
