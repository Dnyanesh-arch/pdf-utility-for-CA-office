mod models;
mod services;

use chrono::Local;
use models::{CompressionRequest, JobHistoryItem, MergeRequest, SplitBySizeRequest};
use services::{
    compress_service::PdfCompressService, history_service::JobHistoryService, merge_service::PdfMergeService,
    split_service::PdfSplitService, validation_service::ValidationService,
};
use tauri::Manager;
use uuid::Uuid;

#[tauri::command]
fn merge_pdfs(app: tauri::AppHandle, request: MergeRequest) -> Result<(), String> {
    for file in &request.files {
        ValidationService::validate_pdf(&file.path).map_err(|e| e.to_string())?;
    }
    PdfMergeService::merge(&request)?;
    log_job(&app, "merge", request.files.iter().map(|f| f.path.clone()).collect(), request.output_path, "success", vec![]);
    Ok(())
}

#[tauri::command]
fn compress_pdf(app: tauri::AppHandle, request: CompressionRequest) -> Result<String, String> {
    ValidationService::validate_pdf(&request.input_path).map_err(|e| e.to_string())?;
    let out = PdfCompressService::compress(&request)?;
    log_job(&app, "compress", vec![request.input_path], request.output_folder, "success", vec![]);
    Ok(out)
}

#[tauri::command]
fn split_pdf_by_size(app: tauri::AppHandle, request: SplitBySizeRequest) -> Result<String, String> {
    ValidationService::validate_pdf(&request.input_path).map_err(|e| e.to_string())?;
    let out = PdfSplitService::split_by_size(&request)?;
    log_job(&app, "split", vec![request.input_path], request.output_folder, "success", vec![]);
    Ok(out)
}

#[tauri::command]
fn list_jobs(app: tauri::AppHandle) -> Result<Vec<JobHistoryItem>, String> {
    let dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    Ok(JobHistoryService::new(&dir).list())
}

fn log_job(app: &tauri::AppHandle, operation: &str, sources: Vec<String>, output_path: String, status: &str, output_sizes: Vec<String>) {
    if let Ok(dir) = app.path().app_data_dir() {
        let _ = std::fs::create_dir_all(&dir);
        let item = JobHistoryItem {
            id: Uuid::new_v4().to_string(),
            timestamp: Local::now().to_rfc3339(),
            operation: operation.to_string(),
            sources,
            output_path,
            status: status.to_string(),
            output_sizes,
        };
        JobHistoryService::new(&dir).append(item);
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![merge_pdfs, compress_pdf, split_pdf_by_size, list_jobs])
        .setup(|app| {
            if let Ok(dir) = app.path().app_data_dir() {
                let _ = std::fs::create_dir_all(&dir);
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
