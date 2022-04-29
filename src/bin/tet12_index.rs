use clap::Parser;

use tet12::*;

/// 12平均律のインデックスを周波数に変換する。付随情報も表示する。
#[derive(Debug, Parser)]
struct Cli {
    #[clap(required = true)]
    idxs: Vec<i32>,
}

fn main() {
    let cli = Cli::parse();

    println!("# idx\tfreq\tname");

    for idx in cli.idxs {
        let freq = index_to_freq(idx);
        let name = index_to_name(idx);

        println!("{idx}\t{freq:.2}\t{name}");
    }
}
