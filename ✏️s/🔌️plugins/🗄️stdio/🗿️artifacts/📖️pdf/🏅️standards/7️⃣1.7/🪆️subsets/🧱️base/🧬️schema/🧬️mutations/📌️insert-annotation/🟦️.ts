/** 📌️ Direct insert-annotation TypeScript payload. */
import type { ObjRef, PdfAction, PdfActionKind, PdfAnnotation, PdfAnnotationKind, PdfAppearance, PdfAppearanceEntry, PdfAppearanceState, PdfBorderStyle, PdfCcittParameters, PdfDate, PdfDecimal, PdfDestination, PdfDestinationFit, PdfDictEntry, PdfFileSpecification, PdfMarkupAnnotation, PdfObject, PdfPredictor, PdfStreamFilter } from '../../📸️snapshot/🟦️.ts';
export interface InsertAnnotationMutation {
  mutation: 'insertAnnotation';
  index: number;
  at: number;
  annotation: PdfAnnotation;
}
