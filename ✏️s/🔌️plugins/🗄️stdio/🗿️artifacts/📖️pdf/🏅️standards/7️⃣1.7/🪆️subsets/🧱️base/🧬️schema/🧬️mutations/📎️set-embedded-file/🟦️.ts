/** 📎️ Direct set-embedded-file TypeScript payload. */
import type { PdfDate, PdfEmbeddedFile } from '../../📸️snapshot/🟦️.ts';
export interface SetEmbeddedFileMutation {
  mutation: 'setEmbeddedFile';
  file: PdfEmbeddedFile;
}
