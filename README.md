# CA PDF Utility

CA PDF Utility is a **local-first desktop application** for CA offices and professional legal/tax/audit document workflows. It is built as a Tauri 2 + React + TypeScript app with Rust backend commands and qpdf-powered document processing.

## Why this project
- No cloud upload, no telemetry by default.
- Deterministic output naming for office workflows.
- Practical modular architecture for community contributors.
- Lightweight desktop footprint for Windows-focused teams.

## Features (MVP + foundation)
- Merge multiple PDFs with manual ordering.
- Optional auto-generated Index page inserted as first page.
- Compress PDFs using explicit profiles:
  - Archive Safe
  - Office Upload
  - Aggressive Upload
  - Custom
- Split PDFs by max output file size (iterative algorithm).
- Job history persisted in local JSON storage.

## Screenshots
> Placeholder: add screenshots after running the app UI.

## Detailed Installation Process
### 1) Prerequisites
1. Install **Node.js 20+** and npm.
2. Install **Rust stable** using rustup.
3. Install **qpdf** and ensure it is available in PATH.
4. Install Tauri system prerequisites:
   - Windows: WebView2 runtime + MSVC build tools.
   - Linux/macOS: follow Tauri v2 prereq guide.

### 2) Clone and install dependencies
```bash
git clone <your-fork-or-this-repo-url>
cd pdf-utility-for-CA-office
npm install
```

### 3) Run in development mode
```bash
npm run dev
```
For desktop shell:
```bash
npm run tauri:dev -w apps/desktop
```

### 4) Build production bundles
```bash
npm run build
npm run tauri:build -w apps/desktop
```

## Application Structure Chart
```text
pdf-utility-for-CA-office/
├── apps/
│   └── desktop/
│       ├── src/                    # React UI pages, components, utilities
│       ├── src-tauri/              # Rust backend services + Tauri commands
│       ├── package.json
│       └── vite.config.ts
├── packages/
│   ├── shared-types/               # Shared TS contracts
│   ├── ui/                         # Reusable UI package (expansion-ready)
│   └── pdf-core/                   # Compression strategy abstractions
├── docs/
│   ├── architecture.md
│   └── roadmap.md
├── scripts/                        # Utility scripts (reserved)
├── samples/                        # Sample PDFs for local testing
├── .github/ISSUE_TEMPLATE/         # Community issue templates
├── TASKS.md                        # Running implementation tracker
├── CONTRIBUTING.md
└── LICENSE
```

## Development setup
- Frontend: React + TypeScript + Vite.
- Desktop shell and backend: Tauri 2 + Rust.
- PDF operations:
  - Merge/split structural ops by qpdf/lopdf approach.
  - Index page generation via printpdf.

## Build instructions
1. Ensure qpdf is installed on build machine.
2. Run `npm install`.
3. Run `npm run tauri:build -w apps/desktop`.
4. Find installers in `apps/desktop/src-tauri/target/release/bundle`.

## Privacy philosophy
- All PDF processing runs locally on user machine.
- No default telemetry.
- No cloud storage dependency.
- No auto-upload behavior.

## Roadmap (summary)
- Improve split-by-size precision and performance.
- Add bookmark-driven split and merge bookmark options.
- Add robust page numbering/footer engine.
- Add SQLite job history backend option.

## Contribution guide
See [CONTRIBUTING.md](./CONTRIBUTING.md).
