/** 🔏️ Direct set-mark-info TypeScript payload. */
import type { PdfMarkInfo } from '../../📸️snapshot/📇️metadata/🟦️.ts';
export interface SetMarkInfoMutation {
  mutation: 'setMarkInfo';
  info?: PdfMarkInfo | null;
}
