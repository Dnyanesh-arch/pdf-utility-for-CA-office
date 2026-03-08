use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InputPdf {
    pub id: String,
    pub path: String,
    pub title: String,
    pub pages: Option<u32>,
    pub size_bytes: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexOptions {
    pub enabled: bool,
    pub bundle_title: String,
    pub client_name: Option<String>,
    pub matter_ref: Option<String>,
    pub assessment_year: Option<String>,
    pub financial_year: Option<String>,
    pub prepared_by: Option<String>,
    pub date: Option<String>,
    pub footer_text: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MergeRequest {
    pub files: Vec<InputPdf>,
    pub output_path: String,
    pub create_bookmarks: bool,
    pub index_options: IndexOptions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompressionCustom {
    pub image_dpi: u32,
    pub jpeg_quality: u8,
    pub grayscale: bool,
    pub remove_metadata: bool,
    pub linearize: bool,
    pub flatten_annotations: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompressionRequest {
    pub input_path: String,
    pub output_folder: String,
    pub profile: String,
    pub custom: Option<CompressionCustom>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SplitBySizeRequest {
    pub input_path: String,
    pub output_folder: String,
    pub max_size_mb: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JobHistoryItem {
    pub id: String,
    pub timestamp: String,
    pub operation: String,
    pub sources: Vec<String>,
    pub output_path: String,
    pub status: String,
    pub output_sizes: Vec<String>,
}
