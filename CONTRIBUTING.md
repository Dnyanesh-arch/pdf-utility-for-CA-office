# Contributing to CA PDF Utility

## Getting started
1. Fork and clone repository.
2. Install prerequisites from README.
3. Run `npm install` and `npm run tauri:dev -w apps/desktop`.

## Development principles
- Keep features local-first.
- Avoid cloud/off-device workflows.
- Preserve modular architecture.
- Add tests for business logic.

## Pull requests
- Keep PRs focused.
- Include test evidence.
- Update docs/TASKS.md when behavior changes.

## Coding standards
- TypeScript strict mode.
- Rust error handling using `Result` and clear error messages.
- No hardcoded user-specific paths.
