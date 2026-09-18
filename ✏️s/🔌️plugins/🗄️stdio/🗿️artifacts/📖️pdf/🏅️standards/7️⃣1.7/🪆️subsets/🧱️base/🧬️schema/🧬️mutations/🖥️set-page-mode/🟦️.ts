/** 🖥️ Direct set-page-mode TypeScript payload. */
import type { PdfPageMode } from '../../📸️snapshot/🟦️.ts';
export interface SetPageModeMutation {
  mutation: 'setPageMode';
  mode?: PdfPageMode | null;
}
