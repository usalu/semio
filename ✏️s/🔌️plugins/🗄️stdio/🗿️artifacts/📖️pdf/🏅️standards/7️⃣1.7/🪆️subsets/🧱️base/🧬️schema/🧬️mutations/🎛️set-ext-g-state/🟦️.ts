/** 🎛️ Direct set-ext-g-state TypeScript payload. */
import type { ObjRef, PdfCcittParameters, PdfDecimal, PdfDictEntry, PdfObject, PdfPredictor, PdfStreamFilter } from '../../📸️snapshot/🧩️cos/🟦️.ts';
import type { PdfExtGState, PdfLineCap, PdfLineJoin, PdfSoftMask } from '../../📸️snapshot/🖼️resource/🎚️state/🟦️.ts';
import type { PdfFunction } from '../../📸️snapshot/🌈️color/🟦️.ts';
export interface SetExtGStateMutation {
  mutation: 'setExtGState';
  state: PdfExtGState;
  index?: number | null;
}
