use std::io::Read;
use std::path::PathBuf;

use log::{debug, error};
use rayon::prelude::*;

use clap::Parser;

use glob::glob_with;
use glob::MatchOptions;

use anyhow::Result;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "./**/*.*")]
    path: String,

    #[arg(short, long, default_value_t = 4)]
    threshold: u32,
}

const READ_SIZE: usize = 4 * 1024; // 4k

fn check_file(path: &PathBuf, check_threshold: u32) {
    debug!("read file: {:?}", path);

    let f = std::fs::File::open(path);
    if f.is_err() {
        error!("read file failed: {:?}", f.unwrap_err());
        return;
    }

    let fd = f.unwrap();
    let mut read = std::io::BufReader::with_capacity(4 * READ_SIZE, fd);

    let mut buf = vec![0u8; READ_SIZE];
    let mut zero_blk_count = 0;
    loop {
        let result = read.read(&mut buf);
        match result {
            Ok(0) => break,
            Ok(n) => {
                if is_zero(&buf[0..n]) {
                    zero_blk_count += 1;
                } else {
                    zero_blk_count = 0
                }

                if zero_blk_count > check_threshold {
                    error!(
                        "file {:?} exist 4k zero block, greater than threshold",
                        path
                    );
                    break;
                }
            }
            Err(e) => {
                error!("read file error: {:?}", e);
                break;
            }
        }
    }
}

fn is_zero(buf: &[u8]) -> bool {
    buf.into_par_iter().all(|&b| b == 0)
}

fn main() -> Result<()> {
    env_logger::init();

    let args = Args::parse();
    let options = MatchOptions {
        case_sensitive: false,
        require_literal_separator: false,
        require_literal_leading_dot: true,
    };

    let check_threshold = args.threshold;
    let paths = glob_with(&args.path, options)?;

    paths
        .filter(|p| p.is_ok())
        .map(|path| path.unwrap())
        .filter(|p| p.is_file())
        .par_bridge()
        .for_each(|p| check_file(&p, check_threshold));

    Ok(())
}
