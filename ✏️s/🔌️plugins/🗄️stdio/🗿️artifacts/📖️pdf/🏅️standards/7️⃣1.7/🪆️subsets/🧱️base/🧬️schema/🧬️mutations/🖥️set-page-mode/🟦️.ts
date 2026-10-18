/** 🖥️ Direct set-page-mode TypeScript payload. */
import type { PdfPageMode } from '../../📸️snapshot/📇️metadata/🟦️.ts';
export interface SetPageModeMutation {
  mutation: 'setPageMode';
  mode?: PdfPageMode | null;
}
