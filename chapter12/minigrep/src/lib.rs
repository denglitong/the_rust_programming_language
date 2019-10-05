use std::env;
use std::error::Error;
use std::fs;

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            // panic! 更趋向于程序上的问题而不是使用上的问题，这里可以换成 Result
            // panic!("not enough arguments")
            // 再次复习 Result<T, E> 枚举：
            // Ok(T), Err(E)
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        // read var from env
        // 如果 CASE_INSENSITIVE 被设置为任何值，is_err 会返回 false 并将进行大小写不敏感搜索
        // 这里我们只关心 CASE_INSENSITIVE 是否被设置了而不关心所设置的值，所以使用了 is_err 而不是 unwrap/expect
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

// Box<dyn Error> 意味着函数会返回实现了 Error trait 的类型，不过无需指定具体返回的值的类型
// dyn 表示 动态的（dynamic）
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    // println!("With text:\n{}", contents);

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if !line.contains(query) {
            continue;
        }
        results.push(line);
    }

    results
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if !line.to_lowercase().contains(&query) {
            continue;
        }
        results.push(line);
    }

    results
}

#[cfg(test)]
mod tests {
    use crate::{search, search_case_insensitive, Config};

    #[test]
    fn not_enough_args() {
        let args = vec![String::from("/path/to/script")];
        let config = Config::new(&args);
        match config {
            Ok(c) => panic!("config should not be Ok"),
            Err(e) => {
                println!("err: {}", e);
                assert_eq!(e, "not enough arguments")
            }
        }
    }

    #[test]
    fn enough_args() {
        let args = vec![
            String::from("/path/to/script"),
            String::from("the"),
            String::from("poem.txt"),
        ];
        let config = Config::new(&args);
        match config {
            Ok(c) => (),
            Err(e) => {
                println!("err: {}", e);
                panic!("config should be Ok");
            }
        }
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_incensitive() {
        let query = "rUST";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
