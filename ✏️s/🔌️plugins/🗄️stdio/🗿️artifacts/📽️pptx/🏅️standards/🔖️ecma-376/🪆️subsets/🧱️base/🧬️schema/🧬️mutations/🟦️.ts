/** 🧬️ Logical PresentationML mutation union. */
import { parsePptxParagraph, parsePptxShape, parsePptxSlide, parsePptxSnapshot, parsePptxTransform } from '../📸️snapshot/🟦️.ts';
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

/** 🚪️ A precise position and reason for refusing a malformed PPTX mutation. */
export class stdioPptxEcma376BaseMutationGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) { super(`${at}: ${why}`); }
}
const reject = (at: string, why: string): never => { throw new stdioPptxEcma376BaseMutationGuardRefusal(at, why); };
const object = (value: unknown, at: string): Readonly<Record<string, unknown>> => value !== null && typeof value === 'object' && !Array.isArray(value) ? value as Record<string, unknown> : reject(at, 'value is not an object');
const array = (value: unknown, at: string): readonly unknown[] => Array.isArray(value) ? value : reject(at, 'value is not an array');
const text = (value: unknown, at: string): string => typeof value === 'string' ? value : reject(at, 'value is not a string');
const integer = (value: unknown, at: string): number => Number.isSafeInteger(value) && (value as number) >= 0 ? value as number : reject(at, 'value is not a nonnegative integer');

/** 🚪️ Parses every PPTX mutation, including a boundary-validated setSnapshot payload. */
export function parsePptxMutation(value: unknown, at = '$'): PptxMutation {
  const row = object(value, at), mutation = text(row.mutation, `${at}.mutation`);
  if (mutation === 'setSnapshot') return { mutation, snapshot: parsePptxSnapshot(row.snapshot, `${at}.snapshot`) };
  if (mutation === 'insertSlide') return { mutation, index: integer(row.index, `${at}.index`), slide: parsePptxSlide(row.slide, `${at}.slide`) };
  if (mutation === 'removeSlide') return { mutation, index: integer(row.index, `${at}.index`) };
  if (mutation === 'moveSlide') return { mutation, from: integer(row.from, `${at}.from`), to: integer(row.to, `${at}.to`) };
  if (mutation === 'insertShape') return { mutation, slideIndex: integer(row.slideIndex, `${at}.slideIndex`), shapeIndex: integer(row.shapeIndex, `${at}.shapeIndex`), shape: parsePptxShape(row.shape, `${at}.shape`) };
  if (mutation === 'removeShape') return { mutation, slideIndex: integer(row.slideIndex, `${at}.slideIndex`), shapeIndex: integer(row.shapeIndex, `${at}.shapeIndex`) };
  if (mutation === 'setShapeText') return { mutation, slideIndex: integer(row.slideIndex, `${at}.slideIndex`), shapeIndex: integer(row.shapeIndex, `${at}.shapeIndex`), textFrame: array(row.textFrame, `${at}.textFrame`).map((item, index) => parsePptxParagraph(item, `${at}.textFrame[${index}]`)) };
  if (mutation === 'setShapePosition') return { mutation, slideIndex: integer(row.slideIndex, `${at}.slideIndex`), shapeIndex: integer(row.shapeIndex, `${at}.shapeIndex`), position: parsePptxTransform(row.position, `${at}.position`) };
  return reject(`${at}.mutation`, `unknown PPTX mutation ${mutation}`);
}
