use clap::Parser;
use std::fs::{BufReader, File};
use subprocess::Command;

fn aggregate_sorted_file(
    sorted_file: &str,
    output_file: &str,
) -> Result<(), Box<std::error::Error>> {
    let mut fout = File::create(output_file)?;

    writeln!(fout, "bin1\tbin2\tscore")?;

    let mut prev_b1 = None;
    let mut prev_b2 = None;
    let mut total = 0;

    for result in BufReader::new(File::open(sorted_file)?).lines() {
        let line = result?.trim_end();

        if (line == "") || (line == "\n") || line.starts_with("bin1") {
            continue;
        }

        let [b1, b2, score] = line.split('\t').collect::<Vec<&str>>()[0..3] else {
            continue;
        };

        let mut b1 = b1.parse::<usize>().unwrap();
        let mut b2 = b2.parse::<usize>().unwrap();
        let score = score.parse::<usize>().unwrap();

        if prev_b1.is_none() {
            prev_b1 = Some(b1);
            prev_b2 = Some(b2);
            total = score;
        } else if (prev_b1.unwrap() == b1) && (prev_b2.unwrap() == b2) {
            total += score;
        } else {
            writeln!(
                fout,
                "{}\t{}\t{}",
                prev_b1.unwrap(),
                prev_b2.unwrap(),
                total
            )?;
            prev_b1 = Some(b1);
            prev_b2 = Some(b2);
            total = score;
        }

        if !prev_b1.is_none() {
            writeln!(
                fout,
                "{}\t{}\t{}",
                prev_b1.unwrap(),
                prev_b2.unwrap(),
                total
            )?;
        }
    }
}

pub fn dump_to_sort(tmppairs: &str, output: &str, sorted: &str, sortmemory: &str, tmpdir: &str) {
    let sort_cmd = format!(
        "sort -S {} -k1,1n -k2,2n {} -o {}",
        sortmemory, tmppairs, sorted
    );
    Command::new("sh").arg("-c").arg(sort_cmd).status().unwrap();

    aggregate_sorted_file(sorted, output).unwrap();
}

#[derive(Debug, Parser)]
#[clap(
    name = env!("CARGO_PKG_NAME"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    arg_required_else_help = true
)]
struct Cli {
    #[clap(short, long)]
    tmppairs: String,
    #[clap(short, long)]
    output: String,
    #[clap(short, long)]
    sorted: String,
    #[clap(short, long)]
    sortmemory: String,
    #[clap(short, long)]
    tmpdir: String,
}

fn main() {
    let cli = Cli::parse();

    dump_to_sort(
        &cli.tmppairs,
        &cli.output,
        &cli.sorted,
        &cli.sortmemory,
        &cli.tmpdir,
    );
}
