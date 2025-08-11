use anyhow::{Result, anyhow};
use std::{
    cmp::Ordering,
    fs::{self, DirEntry, FileType},
};

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let root_dir = fs::read_dir(if args.len() > 1 { &args[1] } else { "." })?;

    let mut entries: Vec<(String, FileType, u64)> = root_dir
        .into_iter()
        .filter_map(|e| extract_metadata(e).ok())
        .collect();

    if entries.is_empty() {
        println!("Directory is empty.");
        return Ok(());
    }

    entries.sort_unstable_by(|l, r| {
        let lt = l.1.is_dir();
        let rt = r.1.is_dir();
        let ln = l.0.to_owned().to_lowercase();
        let rn = r.0.to_owned().to_lowercase();
        if lt && !rt {
            Ordering::Less
        } else if !lt && rt {
            Ordering::Greater
        } else {
            ln.cmp(&rn)
        }
    });

    let count = entries.len();
    let longest_name: usize = entries.iter().max_by_key(|e| e.0.len()).unwrap().2 as usize;

    let mut acc = String::with_capacity((21 + longest_name) * count);

    for entry in entries.into_iter().map(display) {
        acc.push_str(&entry);
    }

    println!("Total count: {count}");
    println!("{acc}");

    Ok(())
}

fn extract_metadata(de: Result<DirEntry, std::io::Error>) -> Result<(String, FileType, u64)> {
    let entry = de?;
    let meta = entry.metadata()?;
    Ok((
        entry
            .file_name()
            .into_string()
            .map_err(|_| anyhow!("String could not be converted."))?,
        meta.file_type(),
        meta.len(),
    ))
}

fn normalize_size(size: u64) -> (f64, usize) {
    let mut size = size as f64;
    let mut counter: usize = 0;
    while size >= 10.0 {
        size /= 1024.0;
        counter += 1;
    }
    (size, if counter < 6 { counter } else { 5 })
}

static SIZE_UNITS: [&str; 6] = ["B  ", "kiB", "MiB", "GiB", "TiB", "BIG"];

fn convert_file_type(ft: FileType) -> usize {
    // if ft.is_dir() {0} else if ft.is_file() {1} else {2}
    if ft.is_dir() {
        0
    } else if ft.is_symlink() {
        1
    } else {
        3
    }
}

//static ICONS: [&str; 3] = ["\u{ea83}", "\u{ea7b}", "\u{eb15}"];
static ICONS: [&str; 2] = ["\u{ea83}", "\u{eb15}"];

fn display(entry: (String, FileType, u64)) -> String {
    let fti = convert_file_type(entry.1);
    let icon = if fti < 3 { ICONS[fti] } else { " " };

    let (size, si) = normalize_size(entry.2);

    if entry.1.is_dir() {
        format!(
            " {0}  {1:>3.1} {2}    \x1b[34;4;1m{3}\x1b[0m/\n",
            icon, size, SIZE_UNITS[si], entry.0
        )
    } else {
        format!(
            " {0}  {1:>3.1} {2}    {3}\n",
            icon, size, SIZE_UNITS[si], entry.0
        )
    }
}
