use std::{fs::OpenOptions, io::Error, io::Write, path::PathBuf};

use clap::Parser;
use clap_verbosity_flag::Verbosity;

#[derive(Parser, Debug)]
#[command(about)]
struct Args {
    /// The doi you're looking up
    doi: String,

    #[clap(long, short, value_name = "BIB FILE")]
    /// the absolute filepath to your .bib file
    bib_file: Option<PathBuf>,

    #[clap(flatten)]
    verbosity: Verbosity,
}

fn write_to_stdout(args: &Args) {
    println!("{}", resolve_doi(&args.doi).expect("Unable to resolve doi"));
}

fn write_to_bib(args: &Args) {
    log::debug!("main::write_to_bib");
    let mut file = OpenOptions::new()
        .append(true)
        .open(args.bib_file.as_ref().unwrap())
        .expect("unable to read file");

    let bib_entry = format!(
        "\n{}",
        resolve_doi(&args.doi).expect("Unable to resolve doi")
    );
    log::debug!("bib_entry: {:#?}", bib_entry);
    file.write_all(bib_entry.as_bytes()).expect("write failed");
}

fn resolve_doi(doi: &str) -> Result<String, Error> {
    log::debug!("main::resolve");
    let client = reqwest::blocking::Client::new();
    let res = client
        .get(format!("https://dx.doi.org/{}", doi))
        .header("Accept", "application/x-bibtex")
        .send()
        .expect("Something went wrong with getting the page")
        .text()
        .expect("Something went wrong with parsing the page");
    Ok(res)
}

fn main() {
    let args = Args::parse();
    env_logger::Builder::new()
        .filter_level(args.verbosity.log_level_filter())
        .init();
    log::debug!("Debugging mode turned on");
    match args.bib_file {
        Some(..) => write_to_bib(&args),
        None => write_to_stdout(&args),
    }
}
