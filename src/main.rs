use structopt::StructOpt;
//use std::env;
//use std::io::BufReader;
//use std::fs::File;
use failure::ResultExt;
use exitfailure::ExitFailure;
/// Search for a pattern in a file and display the lines that contain it.
#[derive(Debug, StructOpt)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<(), ExitFailure> {
    let args = Cli::from_args();
    println!("Looking for the pattern: {:?}", &args.pattern);
    println!("Path: {:?}", &args.path);

    //let f = File::open(&args.path)?;
    //let content = BufReader::new(f)
    //    .with_context(|_| format!("could not read file `{:?}`", &args.path))?;

    let content = std::fs::read_to_string(&args.path)
        .with_context(|_| format!("could not read file `{:?}`", &args.path))?;
    minigrep::find_matches(&content, &args.pattern, &mut std::io::stdout());
    Ok(())
}

