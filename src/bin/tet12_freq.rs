use clap::Parser;

use tet12::*;

/// 周波数を12平均律のインデックスに変換する。付随情報も表示する。
#[derive(Debug, Parser)]
struct Cli {
    #[clap(required = true)]
    freqs: Vec<f64>,
}

fn main() {
    let cli = Cli::parse();

    println!("# freq\tidxf\tidx\tname");

    for freq in cli.freqs {
        let idxf = freq_to_indexf(freq);
        let idx = freq_to_index(freq);
        let name = index_to_name(idx);

        println!("{freq:.2}\t{idxf:.2}\t{idx}\t{name}");
    }
}
