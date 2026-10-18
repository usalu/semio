/** 🏷️ Direct set-properties TypeScript payload. */
import type { ObjRef, PdfCcittParameters, PdfDecimal, PdfDictEntry, PdfObject, PdfPredictor, PdfStreamFilter } from '../../📸️snapshot/🧩️cos/🟦️.ts';
import type { PdfNamedProperties } from '../../📸️snapshot/📄️document/🟦️.ts';
export interface SetPropertiesMutation {
  mutation: 'setProperties';
  properties: PdfNamedProperties;
  index?: number | null;
}
