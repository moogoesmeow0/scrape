use csv::{ReaderBuilder, WriterBuilder};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs::{File, OpenOptions};
use std::io::{Seek, SeekFrom};

#[derive(Clone, Serialize, Eq, PartialEq, Hash, Debug, Deserialize)]
#[allow(dead_code)]
struct Record {
    text: String,
    num: i8,
}

pub fn append_to_csv(path: &str, entry: (&String, i8)) -> Result<(), Box<dyn std::error::Error>> {
    let mut wtr = WriterBuilder::new()
        .has_headers(false)
        .from_writer(OpenOptions::new().append(true).create(true).open(path)?);

    let entry_struct = Record {
        text: entry.0.clone(),
        num: entry.1,
    };
    wtr.serialize(entry_struct)?;
    wtr.flush()?;
    Ok(())
}

pub fn deduplicate_csv(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .from_reader(File::open(path)?);

    let mut seen: HashSet<String> = HashSet::new();
    let mut unique_entries: Vec<Record> = Vec::new();

    for result in rdr.deserialize::<Record>() {
        let entry: Record = result?;
        if seen.insert(entry.text.clone()) {
            unique_entries.push(entry);
        }
    }

    // Overwrite file with unique entries
    let mut file = OpenOptions::new().write(true).truncate(true).open(path)?;
    file.seek(SeekFrom::Start(0))?;
    let mut wtr = WriterBuilder::new().has_headers(false).from_writer(file);
    for entry in unique_entries {
        wtr.serialize(entry)?;
    }
    wtr.flush()?;
    Ok(())
}
