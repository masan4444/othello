extern crate clap;
extern crate reversi;

use clap::{App, Arg};
use std::process;

fn main() {
    let app = App::new("reversi")
        .version("0.1.0") // バージョン情報
        .author("masan4444") // 作者情報
        .about("TUI Reversi") // このアプリについて
        .arg(
            Arg::with_name("pvp") // フラグを定義
                .help("PvP") // ヘルプメッセージ
                .long("pvp"), // ロングコマンド
        );

    let matches = app.get_matches();
    let opt = reversi::Opt {
        is_pvp: matches.is_present("pvp"),
    };

    if opt.is_pvp {
        println!("PvP mode");
    }

    if let Err(e) = reversi::run(opt) {
        eprintln!("Application Error: {}", e);
        process::exit(1)
    };
}
