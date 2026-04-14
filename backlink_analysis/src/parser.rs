use encoding_rs::UTF_16LE;
use std::fs;

#[derive(Debug)]
pub struct BacklinkRecord {
    pub page_title: String,
    pub page_url: String,
    pub domain_rating: f32,
    pub url_rating: f32,
    pub domain_traffic: u64,
    pub page_traffic: u32,
    pub target_url: String,
}

pub fn read_all_csv_files() -> Vec<BacklinkRecord> {
    let csv_files = [
        "weathershieldroofers-backlinks.csv",
        "buccosroofing-backlinks.csv",
        "mdroofing-backlinks.csv",
        "linta-backlinks.csv",
    ];

    let mut all_records = Vec::new();

    for file_name in csv_files {
        match read_csv_file(file_name) {
            Ok(records) => all_records.extend(records),
            Err(e) => eprintln!("Error reading {}: {}", file_name, e),
        }
    }

    all_records
}

fn read_csv_file(file_path: &str) -> Result<Vec<BacklinkRecord>, Box<dyn std::error::Error>> {
    let bytes = fs::read(file_path)?;
    let (decoded, _, _) = UTF_16LE.decode(&bytes);
    let content = decoded.replace('\0', "");

    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .has_headers(true)
        .from_reader(content.as_bytes());

    let mut records = Vec::new();

    for result in reader.records() {
        let record = result?;
        let record = BacklinkRecord {
            page_title: record.get(0).unwrap_or("").to_string(),
            page_url: record.get(1).unwrap_or("").to_string(),
            domain_rating: record.get(5).unwrap_or("0").parse().unwrap_or(0.0),
            url_rating: record.get(6).unwrap_or("0").parse().unwrap_or(0.0),
            domain_traffic: record.get(7).unwrap_or("0").parse().unwrap_or(0),
            page_traffic: record.get(11).unwrap_or("0").parse().unwrap_or(0),
            target_url: record.get(13).unwrap_or("").to_string(),
        };
        records.push(record);
    }

    Ok(records)
}
