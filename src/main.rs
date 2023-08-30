// Program calculating sha256 hashes and returning result
//
//! Expected arguments -N [zeros_need] and -F [matches_need], where
//!
//! `zeros_count` is a count of `0` from end of generated hash.
//!
//! `matches_need` is a count of these hashes need to print to stdout.
use clap::Parser;
use sha2::{Digest, Sha256};
use std::time::Instant;
use tokio::spawn;
use tokio::sync::mpsc;

/// Need for restrict count of `tokio` tasks
const MAX_TASKS: usize = 10;

/// Struct for `clap` crate, need to parse CLI args
#[derive(Parser, Debug)]
struct AppOptions {
    /// number of zeros to find in hash
    #[arg(name("N"), short)]
    zeros_need: usize,

    /// number of hashes to print
    #[arg(name("F"), short)]
    matches_need: usize,
}

/// Enum for controlling hash results
#[derive(Debug)]
enum HashResult {
    /// if hash is validated
    Result((u64, String)),
    /// if hash is not validated
    Error,
}

/// Main function
///
/// Generate tokio tasks for generating and validating sha256 hashes
///
#[tokio::main]
async fn main() {
    let options: AppOptions = AppOptions::parse();
    let now = Instant::now();
    let mut tasks = Vec::with_capacity(MAX_TASKS);
    let mut max_count = options.matches_need;

    // declare channel mpsc
    let (tx, mut rx) = mpsc::channel(options.matches_need);

    // while loop until finded max_count
    let mut i: u64 = 1;
    while i <= u64::MAX && max_count != 0 {
        // clone tx for spawn task
        let tx2 = tx.clone();
        // controlling maximum tasks
        if tasks.len() < MAX_TASKS {
            tasks.push(spawn(async move {
                // spawn(async move {
                let res = get_hash_with_zeros(i, options.zeros_need).await;
                let _ = tx2.send(res).await;
                // });
            }));
            // next num
            i += 1;
        }
        // recive channel messages and match
        while let Ok(message) = rx.try_recv() {
            match message {
                // print result and fix founded hash
                HashResult::Result(res) => {
                    println!("{}, \"{}\"", res.0, res.1);
                    max_count -= 1;
                }
                // no actions if error
                HashResult::Error => {}
            }
            // controlling maximum tasks
            tasks.pop();
        }
    }

    drop(rx);
    drop(tx);

    let elapsed = now.elapsed();
    println!("Done in {:?}, last asked = {:?}", elapsed, i);
}

/// Generate sha256 hash, and return as `String`
///
/// Generate sha256 hash for given `num`, and return hash as `String`
///
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// let res = generate_sha256_hash(3);
/// ```
/// ```rust
/// let res = generate_sha256_hash(4163);
/// ```
async fn generate_sha256_hash(num: u64) -> String {
    // create a Sha256 object
    let mut hasher = Sha256::new();
    // put number to hasher
    hasher.update(num.to_string());
    // read hash digest and consume hasher
    let result = hasher.finalize();
    // return string
    format!("{:x}", result)
}

/// Validate sha256 hash.
///
/// Compare given hash with given count of zeros
///
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// let res = validate_sha256_hash("dfd000", 3); // return true
/// ```
/// ```rust
/// let res = validate_sha256_hash("dfd000", 4); // return false
/// ```
async fn validate_sha256_hash(hash: &str, count: usize) -> bool {
    let sl = format!("{:0<1$}", "", count);
    let sl_cmp = &hash[hash.len() - count..];
    // compare slices and return result
    sl == sl_cmp
}

/// Return enum `HashResult`
///
/// If generated hash is validated with `validate_sha256_hash`
/// return it with `HashResult::Result`, if not then return HashResult::Error
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// let res = get_hash_with_zeros(4163, 3); // return HashResult::Result
/// ```
/// ```rust
/// let res = get_hash_with_zeros(4163, 4); // return HashResult::Error
/// ```
async fn get_hash_with_zeros(num: u64, zeros: usize) -> HashResult {
    let hash = generate_sha256_hash(num).await;
    // validate sha256 hash
    if validate_sha256_hash(&hash, zeros).await {
        HashResult::Result((num, hash))
    } else {
        HashResult::Error
    }
}

#[cfg(test)]
mod test;
