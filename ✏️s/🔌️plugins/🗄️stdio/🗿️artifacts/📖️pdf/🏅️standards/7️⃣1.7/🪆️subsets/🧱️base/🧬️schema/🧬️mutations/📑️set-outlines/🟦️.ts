/** 📑️ Direct set-outlines TypeScript payload. */
import type { ObjRef, PdfAction, PdfActionKind, PdfCcittParameters, PdfDecimal, PdfDestination, PdfDestinationFit, PdfDictEntry, PdfFileSpecification, PdfObject, PdfOutlineItem, PdfPredictor, PdfStreamFilter } from '../../📸️snapshot/🟦️.ts';
export interface SetOutlinesMutation {
  mutation: 'setOutlines';
  outlines: PdfOutlineItem[];
}
