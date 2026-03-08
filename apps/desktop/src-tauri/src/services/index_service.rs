use std::{fs::File, io::BufWriter, path::Path};

use printpdf::{BuiltinFont, Mm, PdfDocument};

use crate::models::{IndexOptions, InputPdf};

pub struct IndexGenerationService;

impl IndexGenerationService {
    pub fn generate_index_pdf(
        files: &[InputPdf],
        options: &IndexOptions,
        path: &Path,
    ) -> Result<(), String> {
        let (doc, page1, layer1) = PdfDocument::new("Index", Mm(210.0), Mm(297.0), "Layer 1");
        let layer = doc.get_page(page1).get_layer(layer1);
        let font = doc.add_builtin_font(BuiltinFont::Helvetica).map_err(|e| e.to_string())?;

        layer.use_text(options.bundle_title.clone(), 18.0, Mm(15.0), Mm(280.0), &font);
        layer.use_text(
            "Sr. | Document Title | File Name | Page From | Page To",
            11.0,
            Mm(15.0),
            Mm(265.0),
            &font,
        );

        let mut y = 255.0;
        let mut page_from = 1;
        for (i, file) in files.iter().enumerate() {
            let pages = file.pages.unwrap_or(1);
            let page_to = page_from + pages - 1;
            let line = format!(
                "{} | {} | {} | {} | {}",
                i + 1,
                file.title,
                Path::new(&file.path)
                    .file_name()
                    .map(|x| x.to_string_lossy().to_string())
                    .unwrap_or_default(),
                page_from,
                page_to
            );
            layer.use_text(line, 9.0, Mm(15.0), Mm(y), &font);
            page_from = page_to + 1;
            y -= 8.0;
        }

        let mut writer = BufWriter::new(File::create(path).map_err(|e| e.to_string())?);
        doc.save(&mut writer).map_err(|e| e.to_string())
    }
}
