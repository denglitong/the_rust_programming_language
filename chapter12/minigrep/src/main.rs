use minigrep::{run, Config};
use std::{env, process};

fn main() {
    // 注意 std::env::args 在其任何参数包含无效 Unicode 字符时会 panic。
    // 如果你需要接受包含无效 Unicode 字符的参数，使用 std::env::args_os 代替。
    // 这个函数返回 OsString 值而不是 String 值
    // collect 是一个经常需要注明类型的函数
    let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);

    // unwrap_or_else 在 Err 时会调用一个 closure
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        // 将错误信息输出到 stderr
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
