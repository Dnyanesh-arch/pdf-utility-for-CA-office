use std::{fs, path::PathBuf};

use crate::models::JobHistoryItem;

pub struct JobHistoryService {
    path: PathBuf,
}

impl JobHistoryService {
    pub fn new(base_dir: &PathBuf) -> Self {
        Self {
            path: base_dir.join("job-history.json"),
        }
    }

    pub fn list(&self) -> Vec<JobHistoryItem> {
        fs::read_to_string(&self.path)
            .ok()
            .and_then(|raw| serde_json::from_str::<Vec<JobHistoryItem>>(&raw).ok())
            .unwrap_or_default()
    }

    pub fn append(&self, item: JobHistoryItem) {
        let mut existing = self.list();
        existing.insert(0, item);
        if let Ok(json) = serde_json::to_string_pretty(&existing) {
            let _ = fs::write(&self.path, json);
        }
    }
}
