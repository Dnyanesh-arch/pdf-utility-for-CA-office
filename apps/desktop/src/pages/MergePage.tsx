import { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { save } from '@tauri-apps/plugin-dialog';
import { FileDropzone } from '../components/FileDropzone';
import { IndexEditor } from '../components/IndexEditor';
import { ProgressModal } from '../components/ProgressModal';
import { IndexOptions, InputPdf, MergeRequest } from '../types/contracts';

const defaultIndex: IndexOptions = { enabled: true, bundleTitle: 'Paper Book' };

export function MergePage() {
  const [files, setFiles] = useState<InputPdf[]>([]);
  const [indexOptions, setIndexOptions] = useState<IndexOptions>(defaultIndex);
  const [busy, setBusy] = useState(false);
  const [status, setStatus] = useState('');

  const merge = async () => {
    if (!files.length) return;
    const outputPath = await save({ defaultPath: 'merged.pdf', filters: [{ name: 'PDF', extensions: ['pdf'] }] });
    if (!outputPath) return;

    const request: MergeRequest = { files, outputPath, createBookmarks: true, indexOptions };
    try {
      setBusy(true);
      setStatus('Merging files...');
      await invoke('merge_pdfs', { request });
      setStatus(`Saved: ${outputPath}`);
    } catch (error) {
      setStatus(`Merge failed: ${String(error)}`);
    } finally {
      setBusy(false);
    }
  };

  return (
    <section>
      <h2>Merge + Auto Index</h2>
      <FileDropzone onFilesSelected={(newFiles) => setFiles((old) => [...old, ...newFiles])} />
      <section className="card">
        <h3>Selected Files</h3>
        {files.map((file, idx) => (
          <div className="list-row" key={file.id}>
            <span>{idx + 1}. {file.title}</span>
            <button onClick={() => setFiles(files.filter((item) => item.id !== file.id))}>Remove</button>
          </div>
        ))}
      </section>
      <IndexEditor value={indexOptions} onChange={setIndexOptions} />
      <button onClick={merge} disabled={!files.length || busy}>Merge PDFs</button>
      <p>{status}</p>
      <ProgressModal open={busy} title="Processing" message="Please wait while documents are merged." />
    </section>
  );
}
