/** 📥️ Direct insert-page TypeScript payload. */
import type { ObjRef, PdfAction, PdfActionKind, PdfAnnotation, PdfAnnotationKind, PdfAppearance, PdfAppearanceEntry, PdfAppearanceState, PdfBorderStyle, PdfCcittParameters, PdfColorSpace, PdfDate, PdfDecimal, PdfDestination, PdfDestinationFit, PdfDictEntry, PdfFileSpecification, PdfFunction, PdfInlineImage, PdfLineCap, PdfLineJoin, PdfMarkupAnnotation, PdfObject, PdfOp, PdfPage, PdfPredictor, PdfPropertyList, PdfStreamFilter, PdfTextArrayItem, PdfTextString, PdfTransparencyGroup } from '../../📸️snapshot/🟦️.ts';
export interface InsertPageMutation {
  mutation: 'insertPage';
  index: number;
  page: PdfPage;
}
