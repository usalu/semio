/** 🖼️ Direct set-page-box TypeScript payload. */
import type { PdfPageBox } from '../../🔺️diff/🟦️.ts';
export interface SetPageBoxMutation {
  mutation: 'setPageBox';
  index: number;
  kind: PdfPageBox;
  rect?: [number, number, number, number] | null;
}
