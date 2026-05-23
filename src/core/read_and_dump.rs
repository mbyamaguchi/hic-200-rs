use flate2::read::GzDecoder;
use clap::Parser;
use std::fs::{File, BufReader};
use std::collections::HashMap;
use std::io::{BufRead, Write};

fn load_bin_metadata(bin_file: &str) -> HashMap<String, HashMap<String, usize>> {

    let mut chr_meta = HashMap<String, HashMap<String, usize>>::new();

    for result in BufReader::new(File::open(bin_file)?).lines().skip(1) {
        let line = result?.trim_end();

        if (line == "") || (line == "\n") {
            continue;
        }

        let [bin_id, chrom, start, end] = line.split('\t').collect();

        let bin_id = bin_id.parse::<usize>().unwrap();
        let chrom = chrom.to_string();
        let start = start.parse::<usize>().unwrap();
        let end = end.parse::<usize>().unwrap();

        if !chr_meta.contains_key(&chrom) {
            chr_meta.insert(chrom.clone(), HashMap::new("first_bin", bin_id));
            chr_meta.insert(chrom.clone(), HashMap::new("last_end", end));
        } else {
            if end > *chr_meta.get(&chrom).unwrap().get("last_end").unwrap() {
                chr_meta.get_mut(&chrom).unwrap().insert("last_end".to_string(), end);
            }
        }
    }

    chr_meta
}

fn midpoint(start: usize, end: usize) -> usize {
    ((start + end) / 2).floor() as usize
}

fn midpoint_to_bin(chrom: &str, mid: usize, chr_meta: &HashMap<String, HashMap<String, usize>>) -> Option<usize> {
    let meta = chr_meta.get(chrom).unwrap();

    if mid > *meta.get("last_end").unwrap() {
        None
    }

    let local_index = (mid / 200).floor() as usize;

    Some(*meta.get("first_bin").unwrap() + local_index)
}

fn read_and_dump(bin_file: &str, max_distance: usize, input_gz: &str, output_file: &str) -> Result<(), Box<std::error::Error>> {
    let chr_meta = load_bin_metadata(bin_file);

    let fin = GzDecoder::new(File::open(input_gz)?);
    let fout = File::create(output_file)?;

    writeln!(fout, "bin1\tbin2\tscore")?;

    for (line_num, line) in BufReader::new(fin).lines().enumerate(2).skip(1) {
        let line = line?.trim_end();

        if (line == "") || (line == "\n") {
            continue;
        }

        let fields = line.split('\t').collect();

        if fields.len() < 9 {
            continue;
        }

        let chrom1 = fields[0];
        let start1 = fields[1].parse::<usize>().unwrap();
        let end1 = fields[2].parse::<usize>().unwrap();

        let chrom2 = fields[3];
        let start2 = fields[4].parse::<usize>().unwrap();
        let end2 = fields[5].parse::<usize>().unwrap();

        let score = fields[8].parse::<usize>().unwrap();

        if chrom1 != chrom2 {
            continue;
        }

        let mid1 = midpoint(start1, end1);
        let mid2 = midpoint(start2, end2);

        if abs_diff(mid1, mid2) > max_distance {
            continue;
        }

        bin1 = midpoint_to_bin(chrom1, mid1, &chr_meta);
        bin2 = midpoint_to_bin(chrom2, mid2, &chr_meta);

        if bin1.is_none() || bin2.is_none() {
            bin1, bin2 = bin2.clone(), bin1.clone();
        }

        writeln!(fout, "{}\t{}\t{}", bin1.unwrap(), bin2.unwrap(), score)?;


        
    }
}

#[derive(Parser)]
#[clap(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    arg_required_else_help = true,]
struct Cli {
    #[clap(short, long)]
    bin_file: String,
    max_distance: usize,
    input_gz: String,
    output_file: String,
}

fn main() -> Result<(), Box<std::error::Error>> {
    let cli = Cli::parse();

    read_and_dump(&cli.bin_file, cli.max_distance, &cli.input_gz, &cli.output_file)
}