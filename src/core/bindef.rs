use clap::{Parser, Subcommand, ArgEnum};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug, Parser)]
#[clap(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    arg_required_else_help = true,
)]
struct Cli {
    /// Size of each bin in base pairs
    #[clap(short, long)]
    bin_size: usize,

    /// Comma-separated list of target chromosomes (e.g., "chr1,chr2,chr3")
    #[clap(short, long)]
    chroms: String,

    /// Path to the site definition file
    #[clap(short, long)]
    site_file: String,

    /// Path to the output file
    #[clap(short, long)]
    output_file: String,
}


fn read_chromosome_length(site_file: &str, target_chroms: &str) -> Result<Map<String, usize>, Box<std::error::Error>> {
    let mut chrom_lengths: Map<String, usize> = HashMap::new();

    // read site def file
    for result in BufReader::new(File::open(site_file)?).lines() {
        let line = result?.trim_end();

        if (line == "") || (line == "\n") {
            continue;
        }

        if line.starts_with("#") {
            continue;
        }

        let fields: Vec<&str> = line.split('\t').collect();

        if fields.len() < 5 {
            continue;
        }

        let [_, chrom, position, _, length_after] = fields.as_slice()[:5] else {
            panic!("Invalid line in site file: {}", line)
        };

        let position = position.parse::<usize>().unwrap();
        let length_after = length_after.parse::<usize>().unwrap();

        let chrom_end = position + length_after;

        if target_chroms.contains(chrom) {
            chrom_lengths.insert(chrom.to_string(), max(chrom_lengths.get(chrom).copied().unwrap_or(0), chrom_end));
        } else {
            chrom_lengths.insert(chrom.to_string(), chrom_end);
        }

    }
    Ok(chrom_lengths)
}

pub fn bindef(bin_size: usize, chroms: &str, site_file: &str, output_file: &str) -> Result<(), Box<std::error::Error>> {
    let target_chroms = chroms.split(',').collect::<Vec<&str>>();

    let chrom_lengths = read_chromosome_length(
        site_file,
        &target_chroms.parse::<Set<String>>()?,
    )?;

    let missing = target_chroms.iter().filter(|chrom| !chrom_lengths.contains_key(*chrom)).collect::<Vec<&&str>>().join(", ");

    if !missing.is_empty() {
        eprintln!("Warning: The following target chromosomes were not found in the site file: {}", missing);
    }

    let mut bin_id = 1;

    let mut output = File::create(output_file)?;

    writeln!(output, "bin\tchr\tstart\tend")?;

    for chrom in target_chroms {
        if let Some(&chrom_length) = chrom_lengths.get(chrom) {
            let mut start = 0;

            while start < chrom_length {
                let end = min(start + bin_size, chrom_length);
                writeln!(output, "{}\t{}\t{}\t{}", bin_id, chrom, start, end)?;
                bin_id += 1;
                start += bin_size;
            }
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<std::error::Error>> {
    let cli = Cli::parse();
    bindef(cli.bin_size, &cli.chroms, &cli.site_file, &cli.output_file);

    Ok(())
}