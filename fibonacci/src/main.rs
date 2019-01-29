fn main() {
    assert_eq!(fibonacci(0), 0);
    assert_eq!(fibonacci(1), 1);
    assert_eq!(fibonacci(2), 1);
    assert_eq!(fibonacci(3), 2);
    assert_eq!(fibonacci(4), 3);
    assert_eq!(fibonacci(5), 5);
    assert_eq!(fibonacci(6), 8);
    assert_eq!(fibonacci(9), 34);
}

fn fibonacci(n: i64) -> i64 {
    if n <= 1 {
        n
    } else {
        let mut x = 0;
        let mut y = 1;
        let mut current = x + y;
        for _ in 1..n {
            current = x + y;
            x = y;
            y = current;
        }
        current
    }
}
