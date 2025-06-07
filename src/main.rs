use eyre::Result;
use std::fs;
use std::io;
use std::io::BufWriter;
use std::path;
mod subtitles;

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

fn main() {
    let items = match load_subtitles(path::Path::new("/tmp/example.srt")) {
        Ok(res) => res,
        Err(e) => {
            println!("Error {}", e);
            return;
        }
    };
    let file = fs::File::create("/tmp/result.srt").unwrap();
    let mut writer = subtitles::Writer::new(BufWriter::new(file));
    for item in items {
        writer.write_item(item).unwrap();
    }
}
