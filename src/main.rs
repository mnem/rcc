extern crate walkdir;
extern crate memmap;

use walkdir::WalkDir;
use std::fs::File;
use std::io;
use walkdir::DirEntry;
use memmap::Mmap;

fn main() -> Result<(), io::Error> {
    for entry in WalkDir::new("./").into_iter().filter_map(|e| e.ok()) {
        if !entry.file_type().is_file() {
            continue
        }

        let mut file_job = load_file(&entry)?;
        let _ = process_file(&mut file_job);

        println!("{} lines={} bytes={}", file_job.name, file_job.lines, file_job.bytes);
    }
    Ok(())
}

const NUL: u8 = 0;
const NEWLINE: u8 = 10;

fn load_file(entry: &DirEntry) -> Result<(FileJob), io::Error> {
    let path = entry.path();
    let file = File::open(path)?;

    let mmap: Option<Mmap> = if file.metadata().unwrap().len() > 0 {
        Some(unsafe { Mmap::map(&file) }?)
    } else {
        None
    };

    return Ok(FileJob {
        name: entry.path().display().to_string(),
        bytes: 0,
        _blank: 0,
        _code: 0,
        _comment: 0,
        lines: 0,
        content: mmap,
    })
}

fn process_file(file_job: &mut FileJob) -> Result<(), io::Error> {

    if let Some(ref content) = file_job.content {
        for i in content.iter() {
            if i == &NUL {
                return Ok(())
            }

            if i == &NEWLINE {
                file_job.lines += 1
            }

            file_job.bytes += 1;
        }
    }

    Ok(())
}

struct FileJob {
    name: String,
    lines: u32,
    _code: u32,
    _comment: u32,
    _blank: u32,
    bytes: u32,
    content: Option<Mmap>,
}
