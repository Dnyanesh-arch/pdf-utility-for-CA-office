import { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';

export function SplitPage() {
  const [inputPath, setInputPath] = useState('');
  const [outputFolder, setOutputFolder] = useState('');
  const [maxSizeMb, setMaxSizeMb] = useState(5);
  const [result, setResult] = useState('');

  const splitBySize = async () => {
    try {
      const response = await invoke<string>('split_pdf_by_size', {
        request: { inputPath, outputFolder, maxSizeMb }
      });
      setResult(response);
    } catch (error) {
      setResult(`Split failed: ${String(error)}`);
    }
  };

  return (
    <section>
      <h2>Split</h2>
      <div className="card">
        <button onClick={async () => setInputPath((await open({ filters: [{ name: 'PDF', extensions: ['pdf'] }] })) as string)}>Select Input PDF</button>
        <button onClick={async () => setOutputFolder((await open({ directory: true })) as string)}>Select Output Folder</button>
        <label>
          Max size (MB)
          <input type="number" value={maxSizeMb} min={1} onChange={(e) => setMaxSizeMb(Number(e.target.value))} />
        </label>
        <button disabled={!inputPath || !outputFolder} onClick={splitBySize}>Split by max size</button>
      </div>
      <p>{result}</p>
    </section>
  );
}
