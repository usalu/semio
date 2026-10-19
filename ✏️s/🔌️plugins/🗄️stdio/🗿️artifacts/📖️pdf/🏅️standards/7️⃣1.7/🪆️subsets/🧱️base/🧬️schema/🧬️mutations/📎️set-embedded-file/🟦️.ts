/** 📎️ Direct set-embedded-file TypeScript payload. */
import type { PdfEmbeddedFile } from "../../📸️snapshot/📇️metadata/🟦️.ts";
import type { PdfDate } from "../../📸️snapshot/📆️date/🟦️.ts";
export interface SetEmbeddedFileMutation {
  mutation: 'setEmbeddedFile';
  file: PdfEmbeddedFile;
  index?: number | null;
}
