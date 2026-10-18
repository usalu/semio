/** 📎️ Direct set-embedded-file TypeScript payload. */
import type { PdfDate, PdfEmbeddedFile } from '../../📸️snapshot/📇️metadata/🟦️.ts';
export interface SetEmbeddedFileMutation {
  mutation: 'setEmbeddedFile';
  file: PdfEmbeddedFile;
  index?: number | null;
}
