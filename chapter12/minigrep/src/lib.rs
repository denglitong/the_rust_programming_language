use std::borrow::{Borrow, BorrowMut};
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
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

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
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // 关于迭代器的性能：迭代器作为一个高级的抽象，被编译成了与手写的底层代码大体一致性能的代码
    // 迭代器是 Rust 的 零成本抽象（zero-cost abstractions） 之一，它意味着抽象并不会引入运行时开销
    contents
        .lines()
        .filter(|line| {
            line.to_lowercase()
                .as_str()
                .contains(query.to_lowercase().as_str())
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::{search, search_case_insensitive, Config};

    //    #[test]
    //    #[ignore]
    //    fn not_enough_args() {
    //        let args = vec![String::from("/path/to/script")];
    //        let config = Config::new(&args);
    //        match config {
    //            Ok(c) => panic!("config should not be Ok"),
    //            Err(e) => {
    //                println!("err: {}", e);
    //                assert_eq!(e, "not enough arguments")
    //            }
    //        }
    //    }
    //
    //    #[test]
    //    #[ignore]
    //    fn enough_args() {
    //        let args = vec![
    //            String::from("/path/to/script"),
    //            String::from("the"),
    //            String::from("poem.txt"),
    //        ];
    //        let config = Config::new(&args);
    //        match config {
    //            Ok(c) => (),
    //            Err(e) => {
    //                println!("err: {}", e);
    //                panic!("config should be Ok");
    //            }
    //        }
    //    }

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
