/** 🧬️ Logical ECMA-376 PresentationML diff schema. */
import type { PptxXmlPart } from '../📸️snapshot/🟦️.ts';
export interface PptxDiff {
  opc?: unknown;
  xmlParts?: PptxXmlPart[];
  presentation?: unknown;
}
