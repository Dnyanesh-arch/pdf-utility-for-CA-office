use std::{fs, path::Path};

use lopdf::Document;

use crate::{models::SplitBySizeRequest, services::naming_service::FileNamingService};

pub struct PdfSplitService;

impl PdfSplitService {
    pub fn split_by_size(request: &SplitBySizeRequest) -> Result<String, String> {
        let source = Document::load(&request.input_path).map_err(|e| e.to_string())?;
        let pages = source.get_pages();
        let total_pages = pages.len() as u32;
        let max_bytes = request.max_size_mb * 1024 * 1024;
        let mut start = 1;
        let mut part = 1;

        while start <= total_pages {
            let mut low = start;
            let mut high = total_pages;
            let mut best: Option<(u32, u64)> = None;

            while low <= high {
                let mid = (low + high) / 2;
                let bytes = Self::estimate_chunk_bytes(&request.input_path, start, mid)?;
                if bytes <= max_bytes {
                    best = Some((mid, bytes));
                    if mid == total_pages {
                        break;
                    }
                    low = mid + 1;
                } else {
                    if mid == 0 {
                        break;
                    }
                    high = mid.saturating_sub(1);
                }
            }

            let (chunk_end, chunk_bytes) =
                best.unwrap_or((start, Self::estimate_chunk_bytes(&request.input_path, start, start)?));
            let filename = format!(
                "{}_Part-{:02}_Pg-{:03}-{:03}_{:.1}MB.pdf",
                FileNamingService::sanitize(
                    Path::new(&request.input_path)
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or("Document"),
                ),
                part,
                start,
                chunk_end,
                chunk_bytes as f64 / 1024.0 / 1024.0
            );
            let output = Path::new(&request.output_folder).join(filename);
            let status = std::process::Command::new("qpdf")
                .arg(&request.input_path)
                .arg("--pages")
                .arg(&request.input_path)
                .arg(format!("{}-{}", start, chunk_end))
                .arg("--")
                .arg(output.to_string_lossy().to_string())
                .status()
                .map_err(|_| "qpdf command not found. Install qpdf and add it to PATH.".to_string())?;

            if !status.success() {
                return Err("qpdf split operation failed".to_string());
            }

            if chunk_end < start {
                return Err("Split algorithm could not progress; aborting to avoid infinite loop".to_string());
            }

            start = chunk_end + 1;
            part += 1;
        }

        Ok("Split by size completed".to_string())
    }

    fn estimate_chunk_bytes(input_path: &str, start: u32, end: u32) -> Result<u64, String> {
        let temp = tempfile::NamedTempFile::new().map_err(|e| e.to_string())?;
        let output_path = temp.path().to_string_lossy().to_string();
        let status = std::process::Command::new("qpdf")
            .arg(input_path)
            .arg("--pages")
            .arg(input_path)
            .arg(format!("{}-{}", start, end))
            .arg("--")
            .arg(&output_path)
            .status()
            .map_err(|e| e.to_string())?;
        if !status.success() {
            return Err("qpdf size estimation failed".to_string());
        }
        fs::metadata(output_path)
            .map(|m| m.len())
            .map_err(|e| e.to_string())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn placeholder_for_split_logic() {
        // TODO: Add integration tests with sample PDFs and qpdf availability checks.
        assert_eq!(2 + 2, 4);
    }
}
