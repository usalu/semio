/** 🧬️ Logical PresentationML mutation union. */
import type { PptxParagraph, PptxShape, PptxSlide, PptxSnapshot, PptxTransform } from '../📸️snapshot/🟦️.ts';
export type PptxMutation =
  | { mutation: 'setSnapshot'; snapshot: PptxSnapshot }
  | { mutation: 'insertSlide'; index: number; slide: PptxSlide }
  | { mutation: 'removeSlide'; index: number }
  | { mutation: 'moveSlide'; from: number; to: number }
  | { mutation: 'insertShape'; slideIndex: number; shapeIndex: number; shape: PptxShape }
  | { mutation: 'removeShape'; slideIndex: number; shapeIndex: number }
  | { mutation: 'setShapeText'; slideIndex: number; shapeIndex: number; textFrame: PptxParagraph[] }
  | { mutation: 'setShapePosition'; slideIndex: number; shapeIndex: number; position: PptxTransform };
