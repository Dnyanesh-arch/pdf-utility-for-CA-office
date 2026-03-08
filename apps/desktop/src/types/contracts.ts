export type CompressionProfile = 'archive-safe' | 'office-upload' | 'aggressive-upload' | 'custom';

export interface InputPdf {
  id: string;
  path: string;
  title: string;
  pages?: number;
  sizeBytes?: number;
}

export interface IndexOptions {
  enabled: boolean;
  bundleTitle: string;
  clientName?: string;
  matterRef?: string;
  assessmentYear?: string;
  financialYear?: string;
  preparedBy?: string;
  date?: string;
  footerText?: string;
}

export interface MergeRequest {
  files: InputPdf[];
  outputPath: string;
  createBookmarks: boolean;
  indexOptions: IndexOptions;
}

export interface CompressionOptions {
  profile: CompressionProfile;
  outputFolder: string;
  custom?: {
    imageDpi: number;
    jpegQuality: number;
    grayscale: boolean;
    removeMetadata: boolean;
    linearize: boolean;
    flattenAnnotations: boolean;
  };
}

export interface SplitBySizeRequest {
  inputPath: string;
  outputFolder: string;
  maxSizeMb: number;
}

export interface JobHistoryItem {
  id: string;
  timestamp: string;
  operation: 'merge' | 'compress' | 'split';
  sources: string[];
  outputPath: string;
  status: 'success' | 'failed';
  outputSizes: string[];
}
