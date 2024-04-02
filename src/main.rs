use dialoguer::{Input, Select};
use std::fs::File;
use std::io::{self, BufReader, Read, Result};
mod hasher;
use hasher::calculate_hash;
mod algorithms;
use algorithms::Algorithm;

fn main() -> Result<()> {
    let file_path: String = request_file_path()?;
    let input_hash: String = request_input_hash()?;
    let algorithm: Algorithm = select_hash_algorithm()?;
    let file_hash: String = calculate_file_hash(&file_path, algorithm)?;
    compare_hashes(&file_hash, &input_hash);
    Ok(())
}

fn request_file_path() -> Result<String> {
    Input::new()
        .with_prompt("ファイルのパスを入力してください\n")
        .interact_text()
}

fn request_input_hash() -> Result<String> {
    Input::new()
        .with_prompt("ハッシュ値を入力してください\n")
        .interact_text()
}

fn select_hash_algorithm() -> Result<Algorithm> {
    let algorithms = [
        Algorithm::Md5,
        Algorithm::Sha2_256,
        Algorithm::Sha2_384,
        Algorithm::Sha2_512,
        Algorithm::Sha3_256,
        Algorithm::Sha3_384,
        Algorithm::Sha3_512,
    ];
    let selection = Select::new()
        .with_prompt("使用するハッシュアルゴリズムを選択してください")
        .default(0)
        .items(
            &algorithms
                .iter()
                .map(|alg| alg.as_str())
                .collect::<Vec<_>>(),
        )
        .interact()?;
    Ok(algorithms[selection])
}

fn calculate_file_hash(file_path: &str, algorithm: Algorithm) -> Result<String> {
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;

    calculate_hash(&buffer, algorithm).map_err(|err| match err {
        _ => io::Error::new(
            io::ErrorKind::Other,
            "ハッシュ値の計算中に不明なエラーが発生しました。",
        ),
    })
}

fn compare_hashes(file_hash: &str, input_hash: &str) {
    if file_hash == input_hash {
        println!("ハッシュ値が一致しました。");
    } else {
        println!("ハッシュ値が一致しません。");
    }
}
