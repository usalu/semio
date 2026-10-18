/** 📑️ Direct set-outlines TypeScript payload. */
import type { ObjRef, PdfCcittParameters, PdfDecimal, PdfDictEntry, PdfObject, PdfPredictor, PdfStreamFilter } from '../../📸️snapshot/🧩️cos/🟦️.ts';
import type { PdfAction, PdfActionKind, PdfDestination, PdfDestinationFit, PdfFileSpecification, PdfOutlineItem } from '../../📸️snapshot/🎯️navigation/🟦️.ts';
export interface SetOutlinesMutation {
  mutation: 'setOutlines';
  outlines: PdfOutlineItem[];
}
