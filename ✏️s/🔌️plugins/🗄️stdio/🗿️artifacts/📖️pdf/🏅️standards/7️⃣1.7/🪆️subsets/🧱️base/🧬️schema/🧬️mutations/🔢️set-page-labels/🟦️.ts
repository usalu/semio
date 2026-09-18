/** 🔢️ Direct set-page-labels TypeScript payload. */
import type { PdfPageLabelRange, PdfPageLabelStyle } from '../../📸️snapshot/🟦️.ts';
export interface SetPageLabelsMutation {
  mutation: 'setPageLabels';
  labels: PdfPageLabelRange[];
}
