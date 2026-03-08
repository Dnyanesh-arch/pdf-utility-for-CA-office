use std::{collections::BTreeMap, path::PathBuf};

use lopdf::{Document, Object, ObjectId};

use crate::{models::MergeRequest, services::index_service::IndexGenerationService};

pub struct PdfMergeService;

impl PdfMergeService {
    pub fn merge(request: &MergeRequest) -> Result<(), String> {
        let mut files = request.files.clone();
        let temp_index_path = PathBuf::from(&request.output_path).with_extension("index.temp.pdf");

        if request.index_options.enabled {
            IndexGenerationService::generate_index_pdf(&files, &request.index_options, &temp_index_path)?;
            files.insert(
                0,
                crate::models::InputPdf {
                    id: "index".to_string(),
                    path: temp_index_path.to_string_lossy().to_string(),
                    title: "Index".to_string(),
                    pages: Some(1),
                    size_bytes: None,
                },
            );
        }

        let docs = files
            .iter()
            .map(|f| Document::load(&f.path).map_err(|e| e.to_string()))
            .collect::<Result<Vec<_>, _>>()?;

        let mut max_id = 1;
        let mut documents_pages = BTreeMap::new();
        let mut documents_objects = BTreeMap::new();

        for mut doc in docs {
            doc.renumber_objects_with(max_id);
            max_id = doc.max_id + 1;

            for (id, object) in doc.objects {
                documents_objects.insert(id, object);
            }
            let pages = doc.get_pages();
            for (_, page_id) in pages {
                documents_pages.insert(page_id, Object::Null);
            }
        }

        let mut document = Document::with_version("1.5");
        let mut catalog_object: Option<(ObjectId, Object)> = None;
        let mut pages_object: Option<(ObjectId, Object)> = None;

        for (object_id, object) in documents_objects {
            match object.type_name().unwrap_or(b"") {
                b"Catalog" => catalog_object = Some((object_id, object)),
                b"Pages" => pages_object = Some((object_id, object)),
                b"Page" | b"Outlines" | b"Outline" => (),
                _ => {
                    document.objects.insert(object_id, object);
                }
            }
        }

        let pages_id = pages_object.map(|(id, _)| id).unwrap_or((1, 0));
        let catalog_id = catalog_object.map(|(id, _)| id).unwrap_or((2, 0));

        for (page_id, _) in &documents_pages {
            if let Ok(dictionary) = document.get_object_mut(*page_id).and_then(Object::as_dict_mut) {
                dictionary.set("Parent", pages_id);
            }
        }

        let kids = documents_pages
            .into_keys()
            .map(Object::Reference)
            .collect::<Vec<_>>();

        let mut pages_dict = lopdf::dictionary! {
            "Type" => "Pages",
            "Kids" => kids,
            "Count" => i64::from(kids.len() as i32),
        };
        pages_dict.set("Resources", lopdf::dictionary! {});

        document.objects.insert(pages_id, Object::Dictionary(pages_dict));

        let catalog_dict = lopdf::dictionary! {
            "Type" => "Catalog",
            "Pages" => pages_id,
        };
        document
            .objects
            .insert(catalog_id, Object::Dictionary(catalog_dict));

        document.trailer.set("Root", catalog_id);
        document.compress();
        document.save(&request.output_path).map_err(|e| e.to_string())?;

        if temp_index_path.exists() {
            let _ = std::fs::remove_file(temp_index_path);
        }

        Ok(())
    }
}
