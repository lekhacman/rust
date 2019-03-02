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
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("No query found"),
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("No file name"),
        };

        Ok(Config { query, filename })
    }
}

#[cfg(test)]
mod tests {
    use crate::Config;
    use crate::run;
    use crate::search;
    use std::env;

    #[test]
    fn new_config() {
        let args = env::args();
        let config = Config::new(args);
        assert!(config.is_err());
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
