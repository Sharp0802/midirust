use std::env::args;
use std::fs::read_to_string;
use std::io::{BufRead, Write, stdin, stdout};
use std::process::ExitCode;

mod cx;
mod error;
mod expr;

macro_rules! log {
    (@lv error) => { "\x1b[31merror" };
    (@lv warning) => { "\x1b[33mwarning" };

    ($scope:expr, $lv:ident, $fmt:literal $(, $tt:expr)* $(,)?) => {
        eprintln!(
            concat!("\x1b[1m{}: ", $crate::log!(@lv $lv), ":\x1b[0m ", $fmt),
            $scope
            $(, $tt)*
        )
    };
    ($lv:ident, $fmt:literal $(, $tt:expr)* $(,)?) => {
        $crate::log!(env!("CARGO_PKG_NAME"), $lv, $fmt $(, $tt)*)
    };
}

use cx::Context;
use error::Error;
use expr::Expr;
use log;

fn print_help() {
    println!(concat!(
        "Usage: ",
        env!("CARGO_PKG_NAME"),
        " [options]\n",
        "Options:\n",
        "  -h, --help     Display this information.\n",
        "  -v, --version  Display version information.",
    ));
}

fn print_version() {
    println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
}

fn main() -> ExitCode {
    #[expect(clippy::never_loop)]
    for arg in args().skip(1) {
        match arg.as_str() {
            "-h" | "--help" => {
                print_help();
                return ExitCode::SUCCESS;
            }
            "-v" | "--version" => {
                print_version();
                return ExitCode::SUCCESS;
            }
            arg => {
                log!(error, "unrecognized option '{}'", arg);
                return ExitCode::FAILURE;
            }
        }
    }

    let mut cx = Context::default();

    let mut line = String::new();
    let mut stdin = stdin().lock();
    loop {
        print!("> ");
        if let Err(e) = stdout().flush() {
            log!(warning, "failed to flush: {}", e);
        }

        line.clear();
        if let Err(e) = stdin.read_line(&mut line) {
            log!(error, "failed to read line: {}", e);
            break;
        }

        if let Some(cmd_line) = line.trim().strip_prefix(":") {
            let (cmd, arg) = cmd_line.split_once(' ').unwrap_or((cmd_line, ""));

            match cmd {
                "q" | "quit" => break,
                "l" | "load" => {
                    let input = match read_to_string(arg) {
                        Ok(input) => input,
                        Err(e) => {
                            log!(arg, error, "{}", e);
                            continue;
                        }
                    };

                    let raw = match syn::parse_file(&input) {
                        Ok(src) => src,
                        Err(e) => {
                            log!(arg, error, "{}", e);
                            continue;
                        }
                    };

                    let old_cx = cx.clone();
                    for item in raw.items {
                        if let Err(e) = cx.push(item) {
                            log!(arg, error, "{}", e);
                            cx = old_cx;
                            break;
                        }
                    }
                }
                "d" | "dump" => {
                    println!("{cx:#?}");
                }
                _ => {
                    log!(error, "unrecognized command '{}'", cmd);
                }
            }

            continue;
        }

        let raw_expr: syn::Expr = match syn::parse_str(&line) {
            Ok(expr) => expr,
            Err(e) => {
                log!(error, "{}", e);
                continue;
            }
        };

        let expr: Expr = match raw_expr.try_into() {
            Ok(expr) => expr,
            Err(e) => {
                log!(error, "{}", e);
                continue;
            }
        };

        match expr.eval(&mut cx) {
            Ok(value) => println!("{value}"),
            Err(e) => log!(error, "{}", e),
        }
    }
    drop(stdin);

    ExitCode::SUCCESS
}
