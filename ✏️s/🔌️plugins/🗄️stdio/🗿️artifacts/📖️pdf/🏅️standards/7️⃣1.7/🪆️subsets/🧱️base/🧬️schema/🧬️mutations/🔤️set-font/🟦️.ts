/** 🔤️ Direct set-font TypeScript payload. */
import type { ObjRef, PdfCcittParameters, PdfDecimal, PdfDictEntry, PdfObject, PdfPredictor, PdfStreamFilter } from '../../📸️snapshot/🧩️cos/🟦️.ts';
import type { PdfBaseEncoding, PdfCMap, PdfCharProc, PdfCidFont, PdfCidMapping, PdfCidSystemInfo, PdfCidToGid, PdfCidVerticalRun, PdfCidWidthRun, PdfCodespaceRange, PdfEmbeddedCMap, PdfEncodingDifference, PdfFont, PdfFontDescriptor, PdfFontKind, PdfFontProgram, PdfSimpleEncoding, PdfToUnicode, PdfToUnicodeMapping } from '../../📸️snapshot/🔤️font/🟦️.ts';
import type { PdfColorSpace, PdfFunction } from '../../📸️snapshot/🌈️color/🟦️.ts';
import type { PdfInlineImage, PdfOp, PdfPropertyList, PdfTextArrayItem, PdfTextString } from '../../📸️snapshot/🖋️content/🟦️.ts';
import type { PdfLineCap, PdfLineJoin } from '../../📸️snapshot/🖼️resource/🎚️state/🟦️.ts';
export interface SetFontMutation {
  mutation: 'setFont';
  font: PdfFont;
  index?: number | null;
}
