import { open } from '@tauri-apps/plugin-dialog';
import { InputPdf } from '../types/contracts';

interface Props {
  onFilesSelected: (files: InputPdf[]) => void;
  multiple?: boolean;
}

export function FileDropzone({ onFilesSelected, multiple = true }: Props) {
  const pickFiles = async () => {
    const selected = await open({
      multiple,
      directory: false,
      filters: [{ name: 'PDF', extensions: ['pdf'] }]
    });

    if (!selected) return;
    const paths = Array.isArray(selected) ? selected : [selected];
    const files = paths.map((path, index) => ({
      id: `${Date.now()}-${index}`,
      path,
      title: path.split(/[/\\]/).pop()?.replace(/\.pdf$/i, '') || `Document ${index + 1}`
    }));
    onFilesSelected(files);
  };

  return (
    <div className="card dropzone">
      <p>Drop PDFs here (or click to select).</p>
      <button onClick={pickFiles}>Select PDF files</button>
    </div>
  );
}
