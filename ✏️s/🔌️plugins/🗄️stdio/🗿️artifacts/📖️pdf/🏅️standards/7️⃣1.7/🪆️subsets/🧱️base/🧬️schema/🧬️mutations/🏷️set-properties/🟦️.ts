/** 🏷️ Direct set-properties TypeScript payload. */
import type { ObjRef, PdfCcittParameters, PdfDecimal, PdfDictEntry, PdfNamedProperties, PdfObject, PdfPredictor, PdfStreamFilter } from '../../📸️snapshot/🟦️.ts';
export interface SetPropertiesMutation {
  mutation: 'setProperties';
  properties: PdfNamedProperties;
}
