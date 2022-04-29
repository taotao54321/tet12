//! 12平均律を扱うライブラリ。
//!
//! 本ライブラリにおける「インデックス」は、12平均律において周波数 440Hz を 0 とする。

const FREQ_BASE: f64 = 440.0;

/// 実数インデックスを周波数に変換する。
pub fn indexf_to_freq(idxf: f64) -> f64 {
    FREQ_BASE * 2.0_f64.powf(idxf / 12.0)
}

/// 整数インデックスを周波数に変換する。
pub fn index_to_freq(idx: i32) -> f64 {
    indexf_to_freq(f64::from(idx))
}

/// 整数インデックスを名前に変換する。
///
/// シャープのみを使い、フラットは使わない。
pub fn index_to_name(idx: i32) -> String {
    // 周波数 440Hz の名前を "A4" とする。
    // このとき "C4" はインデックス -9 となる。

    const TABLE: [&str; 12] = [
        "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
    ];

    let name_idx = idx + 9;

    let q = name_idx.div_euclid(12);
    let r = name_idx.rem_euclid(12);

    let octave = 4 + q;
    let name_base = TABLE[usize::try_from(r).expect("r should be within 0..12")];

    format!("{name_base}{octave}")
}

/// 周波数を実数インデックスに変換する。
///
/// `freq` は正でなければならない。
pub fn freq_to_indexf(freq: f64) -> f64 {
    assert!(freq > 0.0);

    12.0 * (freq / FREQ_BASE).log2()
}

/// 周波数を最も近い整数インデックスに丸める。
///
/// `freq` は正でなければならない。
pub fn freq_to_index(freq: f64) -> i32 {
    freq_to_indexf(freq).round() as i32
}
