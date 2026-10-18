/** 🚪️ Direct set-open-action TypeScript payload. */
import type { ObjRef, PdfCcittParameters, PdfDecimal, PdfDictEntry, PdfObject, PdfPredictor, PdfStreamFilter } from '../../📸️snapshot/🧩️cos/🟦️.ts';
import type { PdfAction, PdfActionKind, PdfDestination, PdfDestinationFit, PdfFileSpecification, PdfOpenAction } from '../../📸️snapshot/🎯️navigation/🟦️.ts';
export interface SetOpenActionMutation {
  mutation: 'setOpenAction';
  action?: PdfOpenAction | null;
}
