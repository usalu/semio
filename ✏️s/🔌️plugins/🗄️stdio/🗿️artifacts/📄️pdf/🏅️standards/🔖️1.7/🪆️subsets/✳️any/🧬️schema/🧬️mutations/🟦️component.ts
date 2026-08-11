/** 🧬️ PdfMutation (1.7) union — mirrors the Rust `PdfMutation` enum's `#[serde(tag = "mutation")]`
 *  shape 1:1. `path` on `setDictEntry`/`removeDictEntry` addresses nesting inside ONE object's
 *  `PdfObject` tree via `PdfPathSegment` steps (`../🔺️diff/🟦️component.ts`). */

import type { ObjRef, PdfInfo, PdfObject, PdfPage, PdfSnapshot } from '../📸️snapshot/🟦️component.ts';
import type { PdfPathSegment } from '../🔺️diff/🟦️component.ts';

export type PdfMutation =
  | { mutation: 'noMutation' }
  | { mutation: 'setSnapshot'; snapshot: PdfSnapshot }
  | { mutation: 'insertPage'; index: number; page: PdfPage }
  | { mutation: 'removePage'; index: number }
  | { mutation: 'setPageMediaBox'; index: number; mediaBox: [number, number, number, number] }
  | { mutation: 'setPageCropBox'; index: number; cropBox: [number, number, number, number] | null }
  | { mutation: 'appendPageContent'; index: number; text: string }
  | { mutation: 'setInfo'; info: PdfInfo }
  | { mutation: 'insertObject'; id: ObjRef; value: PdfObject }
  | { mutation: 'removeObject'; id: ObjRef }
  | { mutation: 'setObjectValue'; id: ObjRef; value: PdfObject }
  | { mutation: 'setDictEntry'; id: ObjRef; path: PdfPathSegment[]; key: string; value: PdfObject }
  | { mutation: 'removeDictEntry'; id: ObjRef; path: PdfPathSegment[]; key: string }
  | { mutation: 'setTrailerEntry'; key: string; value: PdfObject }
  | { mutation: 'removeTrailerEntry'; key: string };
