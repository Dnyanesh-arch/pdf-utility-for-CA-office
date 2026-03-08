export interface CompressionStrategy {
  id: string;
  label: string;
  description: string;
}

export const builtInStrategies: CompressionStrategy[] = [
  { id: 'archive-safe', label: 'Archive Safe', description: 'Structural optimization, non-lossy intent.' },
  { id: 'office-upload', label: 'Office Upload', description: 'Moderate recompression for office submissions.' },
  { id: 'aggressive-upload', label: 'Aggressive Upload', description: 'Maximum reduction with possible visual degradation.' }
];
