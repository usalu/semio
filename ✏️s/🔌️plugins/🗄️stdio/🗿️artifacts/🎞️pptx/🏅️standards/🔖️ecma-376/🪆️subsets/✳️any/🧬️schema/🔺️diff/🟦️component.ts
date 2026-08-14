/** 🧬️ Logical ECMA-376 PresentationML diff schema. */
import type { PptxPhysicalState, PptxXmlPart } from '../📸️snapshot/🟦️component.ts';
export interface PptxDiff {
  opc?: unknown;
  xmlParts?: PptxXmlPart[];
  presentation?: unknown;
  physical?: PptxPhysicalState | null;
}
