mod core;
use clap::Parser;

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
    tmpdir: String,
    #[clap(short, long)]
    site_file: String,
    #[clap(short, long)]
    inputpath: String,
    #[clap(short, long)]
    outputpath: String,
}

fn main() {
    let cli = Cli::parse();

    if !std::path::Path::new(&cli.tmpdir).exists() {
        println!("directory {} does not exist.", &cli.tmpdir);
        print!("proceed to make {}? [y/N]: ", &cli.tmpdir);
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        if input.trim().to_lowercase() != "y" {
            std::fs::create_dir_all(&cli.tmpdir)?;
        } else {
            println!("aborting.");
            std::process::exit(1);
        }
    }

    // default use
    let bindefpath = format!("{}/tmp_bindef.txt", cli.tmpdir);
    core::bindef::make_bindef(&cli.site_file, &bindefpath).unwrap();

    let tmppairpath = format!("{}/tmp_pairs.txt", cli.tmpdir);
    core::read_and_dump::read_and_dump(
        &bindefpath,
        max_distance = 1_000_000,
        input_gz = &cli.inputpath,
        output_file = &tmppairpath,
    )
    .unwrap();

    let sortedpath = format!("{}/tmp_sorted.txt", cli.tmpdir);
    core::dump_to_sort::dump_to_sort(
        &tmppairpath,
        &cli.outputpath,
        &sortedpath,
        sortmemory = "1G",
        tmpdir = &cli.tmpdir,
    );
}
