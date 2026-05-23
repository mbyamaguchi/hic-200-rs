use clap::Parser;
use std::fs::{BufReader, File};
use subprocess::Command;

fn aggregate_sorted_file(
    sorted_file: &str,
    output_file: &str,
) -> Result<(), Box<std::error::Error>> {
    let mut fout = File::create(output_file)?;

    writeln!(fout, "bin1\tbin2\tscore")?;

    let mut prev_b1;
    let mut prev_b2;
    let mut total = 0;

    for result in BufReader::new(File::open(sorted_file)?).lines() {
        let line = result?.trim_end();

        if (line == "") || (line == "\n") || line.starts_with("bin1") {
            continue;
        }

        let [b1, b2, score] = line.split('\t').collect();
    }
}
