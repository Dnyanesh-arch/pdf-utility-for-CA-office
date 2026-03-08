import { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';
import { CompressionOptions, CompressionProfile } from '../types/contracts';

const profiles: { label: string; value: CompressionProfile }[] = [
  { label: 'Archive Safe', value: 'archive-safe' },
  { label: 'Office Upload', value: 'office-upload' },
  { label: 'Aggressive Upload', value: 'aggressive-upload' },
  { label: 'Custom', value: 'custom' }
];

export function CompressPage() {
  const [inputPath, setInputPath] = useState('');
  const [outputFolder, setOutputFolder] = useState('');
  const [profile, setProfile] = useState<CompressionProfile>('archive-safe');
  const [message, setMessage] = useState('');

  const run = async () => {
    const request: CompressionOptions & { inputPath: string } = {
      inputPath,
      profile,
      outputFolder,
      custom: {
        imageDpi: 150,
        jpegQuality: 75,
        grayscale: false,
        removeMetadata: true,
        linearize: true,
        flattenAnnotations: false
      }
    };

    try {
      const result = await invoke<string>('compress_pdf', { request });
      setMessage(result);
    } catch (e) {
      setMessage(`Compression failed: ${String(e)}`);
    }
  };

  return (
    <section>
      <h2>Compress</h2>
      <div className="card">
        <button onClick={async () => setInputPath((await open({ filters: [{ name: 'PDF', extensions: ['pdf'] }] })) as string)}>Select PDF</button>
        <button onClick={async () => setOutputFolder((await open({ directory: true })) as string)}>Select Output Folder</button>
        <select value={profile} onChange={(e) => setProfile(e.target.value as CompressionProfile)}>
          {profiles.map((item) => <option key={item.value} value={item.value}>{item.label}</option>)}
        </select>
        <button disabled={!inputPath || !outputFolder} onClick={run}>Run Compression</button>
      </div>
      <p>{message}</p>
    </section>
  );
}
