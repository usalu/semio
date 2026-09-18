/** 🚪️ Direct set-open-action TypeScript payload. */
import type { ObjRef, PdfAction, PdfActionKind, PdfCcittParameters, PdfDecimal, PdfDestination, PdfDestinationFit, PdfDictEntry, PdfFileSpecification, PdfObject, PdfOpenAction, PdfPredictor, PdfStreamFilter } from '../../📸️snapshot/🟦️.ts';
export interface SetOpenActionMutation {
  mutation: 'setOpenAction';
  action?: PdfOpenAction | null;
}
