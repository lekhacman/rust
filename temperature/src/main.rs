fn main() {
    let c = ftoc(59.0);
    assert_eq!(c, 15.0);

    let f = ctof(15.0);
    assert_eq!(f, 59.0);
}

fn ftoc(f: f64) -> f64 {
    (f - 32.0) * (5.0/9.0)
}

fn ctof(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}
