use ::minigrep::Config;
use std::{env, ffi::OsString, process};

fn main() {
    // 关于args_os 和 args 的区别。前者对于非Unicode字符有更好的处理，本着用户输入不可信原则使用这个更加安全。后者如果接收了非Unicode字符会导致程序崩溃。
    // env::args 读取到的第一个参数是程序的可执行路径名
    let args: Vec<OsString> = env::args_os().collect();
    // let args: Vec<String> = env::args().collect();
    // dbg!(&args);
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    eprintln!("query is {:?}", config.query);
    eprintln!("file path is {:#?}", config.file_path);
    if let Err(e) = config.run() {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
