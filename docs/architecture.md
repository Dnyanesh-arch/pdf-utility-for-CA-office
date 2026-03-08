# Architecture

## Desktop app layers
1. React UI pages/components gather user intent.
2. Tauri command bridge validates and dispatches requests.
3. Rust services execute PDF workflows:
   - PdfMergeService
   - PdfSplitService
   - PdfCompressService
   - IndexGenerationService
   - FileNamingService
   - JobHistoryService
   - ValidationService
4. qpdf/lopdf/printpdf adapters perform document operations.

## Compression model
Compression profiles separate:
- Structural optimization (safe)
- Content recompression (lossy office use)

Strategies are exposed as explicit presets, not vague low/medium/high labels.

## Data persistence
- Job history stored as JSON in app data dir.
- Designed for future SQLite replacement without UI contract changes.
