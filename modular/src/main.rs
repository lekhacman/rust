use std::collections::HashMap;

fn main() {
    println!("Hello, world!");

    crate::sound::instrument::clarinet();

    sound::instrument::clarinet();

    let mut map = HashMap::new();
    map.insert(1, 2);

}

mod sound {
    pub mod instrument {
        pub fn clarinet() {
            super::super::breathe_in();
        }
    }

    mod voice {

    }

    fn guitar() {}
}

fn breathe_in() {}
