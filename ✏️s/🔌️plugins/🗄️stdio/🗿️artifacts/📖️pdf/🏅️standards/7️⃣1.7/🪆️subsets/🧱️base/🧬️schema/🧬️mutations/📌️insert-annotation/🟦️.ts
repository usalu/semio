/** 📌️ Direct insert-annotation TypeScript payload. */
import type { ObjRef, PdfCcittParameters, PdfDecimal, PdfDictEntry, PdfObject, PdfPredictor, PdfStreamFilter } from '../../📸️snapshot/🧩️cos/🟦️.ts';
import type { PdfAction, PdfActionKind, PdfDestination, PdfDestinationFit, PdfFileSpecification } from '../../📸️snapshot/🎯️navigation/🟦️.ts';
import type { PdfAnnotation, PdfAppearance, PdfAppearanceEntry, PdfAppearanceState, PdfBorderStyle, PdfMarkupAnnotation } from '../../📸️snapshot/📌️annotation/🟦️.ts';
import type { PdfAnnotationKind } from '../../📸️snapshot/📌️annotation/🏷️kind/🟦️.ts';
import type { PdfDate } from "../../📸️snapshot/📆️date/🟦️.ts";
export interface InsertAnnotationMutation {
  mutation: 'insertAnnotation';
  index: number;
  at: number;
  annotation: PdfAnnotation;
}
