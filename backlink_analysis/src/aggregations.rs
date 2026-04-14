use crate::parser::read_all_csv_files;
use crate::parser::BacklinkRecord;

pub fn clean_files() -> Vec<BacklinkRecord> {
    let mut bl_records = read_all_csv_files();

    bl_records.retain(|record| !record.page_title.to_lowercase().contains("youtube"));

    bl_records.sort_by(|a, b| {
        b.domain_rating
            .partial_cmp(&a.domain_rating)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    bl_records
}
