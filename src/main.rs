use std::{path::Path, process};

use clap::{Args, Parser, Subcommand};

mod resize;

#[derive(Parser)]
#[command(name = "rimp")]
#[command(author = "Vail Gold")]
#[command(version = "0.0.1")]
#[command(about = "Manipulates images via command line", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Resizes images
    Resize(ResizeArgs),
}

#[derive(Args)]
struct ResizeArgs {
    /// Path to image to resize
    input: String,
    /// Path to which to write resized image. If empty, "-out" will be appended
    /// to the basename of the input file.
    #[arg(long, default_value = "")]
    output: String,
    /// Target width of image. Do not use with percent. Negative value is subtracted from input width.
    #[arg(long, default_value_t = 0)]
    width: i32,
    /// Target height of image. Do not use with percent. Negative value is subtracted from input width.
    #[arg(long, default_value_t = 0)]
    height: i32,
    /// Percent of original dimensions to resize. Do not use with width/height. Must be greater than zero.
    /// Ex: 10 will be interpreted as 10% or 1/10 of original size. 0.1 will be interepreted as 0.1% or 1/1000
    /// of original size.
    #[arg(long, default_value_t = 0.0)]
    percent: f64,
    /// Filter to use for resizing.
    #[arg(long, default_value = "lanczos3")]
    filter: String,
}

fn check_file(file: &String) {
    let f = Path::new(file);
    if !(f.exists() && f.is_file()) {
        eprintln!("File does not exist: {}", file);
        process::exit(1);
    }
}

fn main() {
    let cli = Cli::parse();

    let exit_code = match &cli.command {
        Commands::Resize(args) => {
            check_file(&args.input);
            resize::resize(
                &args.input,
                &args.output,
                args.width,
                args.height,
                args.percent,
                &args.filter,
            )
        }
    };
    process::exit(exit_code)
}
