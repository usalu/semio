/** 📖️ Direct set-page-layout TypeScript payload. */
import type { PdfPageLayout } from '../../📸️snapshot/📇️metadata/🟦️.ts';
export interface SetPageLayoutMutation {
  mutation: 'setPageLayout';
  layout?: PdfPageLayout | null;
}
