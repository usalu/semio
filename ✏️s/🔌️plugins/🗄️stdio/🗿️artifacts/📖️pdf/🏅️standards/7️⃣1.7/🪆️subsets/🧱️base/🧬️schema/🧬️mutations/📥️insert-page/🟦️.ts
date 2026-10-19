/** 📥️ Direct insert-page TypeScript payload. */
import type { ObjRef, PdfCcittParameters, PdfDecimal, PdfDictEntry, PdfObject, PdfPredictor, PdfStreamFilter } from '../../📸️snapshot/🧩️cos/🟦️.ts';
import type { PdfAction, PdfActionKind, PdfDestination, PdfDestinationFit, PdfFileSpecification } from '../../📸️snapshot/🎯️navigation/🟦️.ts';
import type { PdfAnnotation, PdfAppearance, PdfAppearanceEntry, PdfAppearanceState, PdfBorderStyle, PdfMarkupAnnotation } from '../../📸️snapshot/📌️annotation/🟦️.ts';
import type { PdfAnnotationKind } from '../../📸️snapshot/📌️annotation/🏷️kind/🟦️.ts';
import type { PdfColorSpace, PdfFunction } from '../../📸️snapshot/🌈️color/🟦️.ts';
import type { PdfDate } from "../../📸️snapshot/📆️date/🟦️.ts";
import type { PdfInlineImage, PdfOp, PdfPropertyList, PdfTextArrayItem, PdfTextString } from '../../📸️snapshot/🖋️content/🟦️.ts';
import type { PdfTransparencyGroup } from "../../📸️snapshot/🖼️resource/🎚️state/🟦️.ts";
import type { PdfLineCap, PdfLineJoin } from "../../📸️snapshot/🖊️line-style/🟦️.ts";
import type { PdfPage } from '../../📸️snapshot/📄️document/🟦️.ts';
export interface InsertPageMutation {
  mutation: 'insertPage';
  index: number;
  page: PdfPage;
}
