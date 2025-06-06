use eyre::Result;
use std::fs;
use std::io;
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
    match load_subtitles(path::Path::new("/tmp/example.srt")) {
        Ok(res) => {
            println!("Got {} items", res.len());
        }
        Err(e) => {
            println!("Error {}", e);
        }
    }
}
