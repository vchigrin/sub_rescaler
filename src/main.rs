use clap::{Parser, Subcommand};
use eyre::Result;
use std::fs;
use std::io;
use std::io::BufWriter;
use std::path;
mod operations;
mod subtitles;

#[derive(Debug, Parser)]
#[command(version, about = "Tool for editing subtitles .srt files.")]
struct CliParams {
    #[command(subcommand)]
    command: Command,
    /// Input file name
    #[arg(short, long)]
    input: path::PathBuf,
    /// Output file name
    #[arg(short, long)]
    output: path::PathBuf,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Offset all items in file to specific duration.
    Offset { offset_ms: i32 },
}

fn load_subtitles(file_path: &path::Path) -> Result<Vec<subtitles::SubItem>> {
    let file = fs::File::open(file_path)?;
    let mut loader = subtitles::Loader::new(io::BufReader::new(file));
    let mut result = Vec::<subtitles::SubItem>::new();
    loop {
        let maybe_item = loader.read_next()?;
        if let Some(item) = maybe_item {
            result.push(item);
        } else {
            break;
        }
    }
    Ok(result)
}

fn write_subtitles(items: Vec<subtitles::SubItem>, dst_path: &path::Path) -> Result<()> {
    let file = fs::File::create(dst_path)?;
    let mut writer = subtitles::Writer::new(BufWriter::new(file));
    for item in items {
        writer.write_item(item).unwrap();
    }
    Ok(())
}

fn handle_command(items: Vec<subtitles::SubItem>, command: Command) -> Vec<subtitles::SubItem> {
    match command {
        Command::Offset { offset_ms } => operations::perform_offset(items, offset_ms),
    }
}

fn do_main() -> Result<()> {
    let params = CliParams::parse();
    println!("Running with params {:?}", params);
    let original_items = load_subtitles(&params.input)?;
    let new_items = handle_command(original_items, params.command);
    write_subtitles(new_items, &params.output)?;
    Ok(())
}

fn main() {
    match do_main() {
        Ok(_) => {
            println!("Success")
        }
        Err(e) => {
            println!("Error {}", e);
            std::process::exit(1);
        }
    }
}
