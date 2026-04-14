mod aggregations;
mod parser;

use aggregations::clean_files;
use parser::BacklinkRecord;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

fn extract_base_domain(url: &str) -> String {
    let url_lower = url.to_lowercase();
    let without_protocol = url_lower
        .trim_start_matches("https://")
        .trim_start_matches("http://")
        .trim_start_matches("www.");

    without_protocol
        .split('/')
        .next()
        .unwrap_or(without_protocol)
        .to_string()
}

fn group_by_domain(records: Vec<BacklinkRecord>) -> HashMap<String, Vec<BacklinkRecord>> {
    let mut grouped: HashMap<String, Vec<BacklinkRecord>> = HashMap::new();

    for record in records {
        let domain = extract_base_domain(&record.target_url);
        grouped.entry(domain).or_default().push(record);
    }

    grouped
}

fn write_csv(records: &[BacklinkRecord], filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create(filename)?;
    writeln!(
        file,
        "Page Title,Page URL,Domain Rating,URL Rating,Domain Traffic,Page Traffic,Target URL"
    )?;

    for record in records {
        writeln!(
            file,
            "\"{}\",\"{}\",{},{},{},{},\"{}\"",
            record.page_title.replace('"', "\"\""),
            record.page_url.replace('"', "\"\""),
            record.domain_rating,
            record.url_rating,
            record.domain_traffic,
            record.page_traffic,
            record.target_url.replace('"', "\"\"\"")
        )?;
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let clean_records = clean_files();
    let grouped = group_by_domain(clean_records);

    let mut total_count = 0;

    for (domain, domain_records) in &grouped {
        let filename = format!("{}.csv", domain.replace('/', "_").replace(':', "_"));
        write_csv(&domain_records, &filename)?;
        total_count += domain_records.len();
        println!("Created {} with {} records", filename, domain_records.len());
    }

    println!(
        "\nTotal: {} records across {} domains",
        total_count,
        grouped.len()
    );
    Ok(())
}
