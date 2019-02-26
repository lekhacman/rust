use std::fs;
use std::error::Error;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("With tetx: \n{}", contents);

    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

#[cfg(test)]
mod tests {
    use crate::Config;

    #[test]
    fn new_config() {
        let first_arg = String::from("first");
        let second_arg = String::from("second");
        let third_arg = String::from("third");
        let args = [
            first_arg.clone(),
            second_arg.clone(),
            third_arg.clone(),
        ];
        let config = Config::new(&args)
            .unwrap_or_else(|_| {panic!("failed")});
        assert_eq!(third_arg, config.filename);
        assert_eq!(second_arg, config.query);

        let failed_config = Config::new(&[
            String::from("Hello world!"),
        ]);
        assert!(failed_config.is_err());
    }
}
