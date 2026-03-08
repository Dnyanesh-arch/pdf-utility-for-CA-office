use std::{path::Path, process::Command};

use crate::{models::CompressionRequest, services::naming_service::FileNamingService};

pub struct PdfCompressService;

impl PdfCompressService {
    pub fn compress(request: &CompressionRequest) -> Result<String, String> {
        let in_path = Path::new(&request.input_path);
        let stem = in_path
            .file_stem()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| "output".to_string());
        let output = Path::new(&request.output_folder)
            .join(FileNamingService::with_suffix(&stem, "compressed"));

        let mut args = vec!["--object-streams=generate", "--compress-streams=y"];
        match request.profile.as_str() {
            "archive-safe" => args.extend(["--decode-level=specialized"]),
            "office-upload" => args.extend(["--decode-level=generalized"]),
            "aggressive-upload" => args.extend(["--decode-level=all"]),
            "custom" => {
                if let Some(custom) = &request.custom {
                    if custom.linearize {
                        args.push("--linearize");
                    }
                    if custom.remove_metadata {
                        args.push("--remove-unreferenced-resources=yes");
                    }
                }
            }
            _ => {}
        }

        args.push(&request.input_path);
        args.push(output.to_str().ok_or("Invalid output path")?);

        let status = Command::new("qpdf")
            .args(args)
            .status()
            .map_err(|_| "qpdf command not found. Install qpdf and add it to PATH.".to_string())?;

        if !status.success() {
            return Err("Compression process failed with qpdf.".to_string());
        }

        Ok(format!("Compressed file saved at {}", output.display()))
    }
}
