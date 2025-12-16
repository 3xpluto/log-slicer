use anyhow::{Context, Result};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::PathBuf;

use crate::cli::{Args, Input};
use crate::engine::record::Record;

/// Returns an iterator (boxed) over Records from stdin or files.
pub fn iter_records(args: &Args) -> Result<Box<dyn Iterator<Item = Record>>> {
    let input = args.input;

    if args.paths.is_empty() {
        // Own stdin handle inside the iterator (no lock lifetime issues).
        let reader = io::BufReader::new(io::stdin());
        let it = reader.lines().filter_map(move |line| Record::from_line(line, input).ok());
        return Ok(Box::new(it));
    }

    let mut its: Vec<Box<dyn Iterator<Item = Record>>> = Vec::new();
    for p in &args.paths {
        its.push(Box::new(iter_file(p.clone(), input)?));
    }

    Ok(Box::new(its.into_iter().flatten()))
}

fn iter_file(path: PathBuf, input: Input) -> Result<impl Iterator<Item = Record>> {
    let file = File::open(&path).with_context(|| format!("failed to open {}", path.display()))?;
    let reader = io::BufReader::new(file);
    Ok(reader.lines().filter_map(move |line| Record::from_line(line, input).ok()))
}
