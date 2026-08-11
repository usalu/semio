/** 🧬️ PngMutation union — mirrors 🦀️component.rs's `#[serde(tag = "mutation")]` enum. */
import type {
  PngBackground, PngChromaticities, PngChunk, PngColorType, PngPhysicalDims, PngRgb,
  PngSnapshot, PngSrgbIntent, PngTextChunk, PngTimestamp, PngTransparency,
} from '../📸️snapshot/🟦️component.ts';

export type PngMutation =
  | { mutation: 'noMutation' }
  | { mutation: 'setSnapshot'; snapshot: PngSnapshot }
  | { mutation: 'setHeader'; width: number; height: number; bitDepth: number; colorType: PngColorType; interlace: boolean }
  | { mutation: 'setPalette'; plte?: PngRgb[] }
  | { mutation: 'setTransparency'; trns?: PngTransparency }
  | { mutation: 'setGamma'; gama?: number }
  | { mutation: 'setChromaticities'; chrm?: PngChromaticities }
  | { mutation: 'setSrgbIntent'; srgb?: PngSrgbIntent }
  | { mutation: 'setPhysicalDims'; phys?: PngPhysicalDims }
  | { mutation: 'setTimestamp'; time?: PngTimestamp }
  | { mutation: 'setBackground'; bkgd?: PngBackground }
  | { mutation: 'insertTextChunk'; index: number; chunk: PngTextChunk }
  | { mutation: 'removeTextChunk'; index: number }
  | { mutation: 'setTextChunk'; index: number; chunk: PngTextChunk }
  | { mutation: 'setPixels'; pixels: number[] }
  | { mutation: 'insertUnknownChunk'; index: number; chunk: PngChunk }
  | { mutation: 'removeUnknownChunk'; index: number };
