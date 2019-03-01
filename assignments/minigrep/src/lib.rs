use std::fs;
use std::error::Error;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let result = search(&config.query, &contents);

    result.iter().for_each(
        |line| println!("{}", line)
    );

    Ok(())
}

fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    content.lines()
        .filter(|line| line.contains(query))
        .collect()
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
    use crate::run;
    use crate::search;

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

    #[test]
    fn test_run() {
        let config = Config {
            query: String::from("long"),
            filename: String::from("poem.txt"),
        };
        let result = run(config);
        assert!(result.is_ok());

        let fail_result = run(Config{
            query: String::from("abc"),
            filename: String::from("abc.txt"),
        });
        assert!(fail_result.is_err());
    }

    #[test]
    fn test_search() {
        let content = String::from("\
Hello World
or friend
and fellow
        ");
        let query = String::from("or");

        let lines = search(&query, &content);
        assert_eq!(
            vec![
                "Hello World",
                "or friend",
            ],
            lines
        )
    }
}
