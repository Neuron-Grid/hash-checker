use dialoguer::{Input, Select};
use std::fs::File;
use std::io::{self, Read, Result};
mod hasher;
use hasher::{calculate_hash, HasherError};

fn main() -> Result<()> {
    let file_path: String = request_file_path()?;
    let input_hash: String = request_input_hash()?;
    let algorithm: usize = select_hash_algorithm()?;
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

fn select_hash_algorithm() -> Result<usize> {
    let selections = vec![
        "md5", "sha2-256", "sha2-384", "sha2-512", "sha3-256", "sha3-384", "sha3-512",
    ];

    let selection = Select::new()
        .with_prompt("使用するハッシュアルゴリズムを選択してください")
        .default(0)
        .items(&selections)
        .interact()?;

    Ok(selection)
}

fn calculate_file_hash(file_path: &str, algorithm: usize) -> Result<String> {
    let selections = vec![
        "md5", "sha2-256", "sha2-384", "sha2-512", "sha3-256", "sha3-384", "sha3-512",
    ];

    let algorithm_str = match selections.get(algorithm) {
        Some(alg) => alg,
        None => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "不正なアルゴリズムが選択されました。",
            ))
        }
    };

    let mut file: File = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    match calculate_hash(&buffer, algorithm_str) {
        Ok(hash) => Ok(hash),
        Err(HasherError::UnsupportedAlgorithm) => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "サポートされていないハッシュアルゴリズムが選択されました。",
        )),
        Err(_) => Err(io::Error::new(
            io::ErrorKind::Other,
            "ハッシュ値の計算中に不明なエラーが発生しました。",
        )),
    }
}

fn compare_hashes(file_hash: &str, input_hash: &str) {
    if file_hash == input_hash {
        println!("ハッシュ値が一致しました。");
    } else {
        println!("ハッシュ値が一致しません。");
    }
}
