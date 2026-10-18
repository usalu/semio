/** 🚫️ Direct remove-dict-entry TypeScript payload. */
import type { ObjRef } from '../../📸️snapshot/🧩️cos/🟦️.ts';
import type { PdfPathSegment } from '../../🔺️diff/🟦️.ts';
export interface RemoveDictEntryMutation {
  mutation: 'removeDictEntry';
  id: ObjRef;
  path: PdfPathSegment[];
  key: string;
}
