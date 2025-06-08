use clap::{Args, Parser, Subcommand};
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

impl CliParams {
    fn canonize(&mut self) -> Result<()> {
        if let Command::SyncWith(args) = &mut self.command {
            args.alignmen_point.sort_by_key(|fp| fp.src_frame);
            args.alignmen_point.dedup();
            if !args
                .alignmen_point
                .is_sorted_by_key(|fp| fp.reference_frame)
            {
                return Err(eyre::eyre!(
                    "Bigger src_frame fields must correspond to bigger reference_frame"
                ));
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct FramePair {
    src_frame: u32,
    reference_frame: u32,
}

fn parse_frame_pair(value: &str) -> Result<FramePair> {
    if let Some(parts) = value.split_once(':') {
        let src_frame: u32 = parts.0.parse()?;
        let reference_frame: u32 = parts.1.parse()?;
        Ok(FramePair {
            src_frame,
            reference_frame,
        })
    } else {
        Err(eyre::eyre!(
            "Frame pair must be in form src_frame_number:reference_frame_number"
        ))
    }
}

#[derive(Debug, Args)]
struct SyncWithArgs {
    /// Path to .srt file with reference timings.
    reference_path: path::PathBuf,
    /// Pairs of algnment points - timings among corresponding frames
    /// will be aligned. Format is "src_frame_number:reference_frame_number"
    #[arg(action=clap::ArgAction::Append, value_parser=parse_frame_pair, short,long)]
    alignmen_point: Vec<FramePair>,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Offset all items in file to specific duration.
    Offset {
        offset_ms: i32,
    },
    SyncWith(SyncWithArgs),
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

fn handle_command(
    items: Vec<subtitles::SubItem>,
    command: Command,
) -> Result<Vec<subtitles::SubItem>> {
    match command {
        Command::Offset { offset_ms } => Ok(operations::perform_offset(items, offset_ms)),
        Command::SyncWith(sync_args) => {
            let reference_items = load_subtitles(&sync_args.reference_path)?;
            operations::perform_sync(items, sync_args.alignmen_point, &reference_items)
        }
    }
}

fn do_main() -> Result<()> {
    let mut params = CliParams::parse();
    params.canonize()?;
    println!("Running with params {:?}", params);
    let original_items = load_subtitles(&params.input)?;
    let new_items = handle_command(original_items, params.command)?;
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
