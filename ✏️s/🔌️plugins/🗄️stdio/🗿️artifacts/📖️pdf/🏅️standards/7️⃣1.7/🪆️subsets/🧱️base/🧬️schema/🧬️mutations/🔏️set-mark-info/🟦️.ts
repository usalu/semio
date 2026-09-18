/** 🔏️ Direct set-mark-info TypeScript payload. */
import type { PdfMarkInfo } from '../../📸️snapshot/🟦️.ts';
export interface SetMarkInfoMutation {
  mutation: 'setMarkInfo';
  info?: PdfMarkInfo | null;
}
