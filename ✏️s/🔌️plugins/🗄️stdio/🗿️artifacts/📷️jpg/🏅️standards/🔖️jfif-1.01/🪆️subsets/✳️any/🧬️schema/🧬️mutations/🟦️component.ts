/** 🧬️ JpgMutation union — mirrors 🦀️component.rs's `#[serde(tag = "mutation")]` enum. */
import type {
  JfifDensityUnits, JfifThumbnail, JpgHuffmanTable, JpgQuantTable, JpgSegment, JpgSnapshot,
} from '../📸️snapshot/🟦️component.ts';
import type { JpgHuffmanTableKey } from '../🔺️diff/🟦️component.ts';

export type JpgMutation =
  | { mutation: 'noMutation' }
  | { mutation: 'setSnapshot'; snapshot: JpgSnapshot }
  | { mutation: 'setJfifHeader'; version: [number, number]; densityUnits: JfifDensityUnits; xDensity: number; yDensity: number; thumbnail?: JfifThumbnail }
  | { mutation: 'setQuantTable'; table: JpgQuantTable }
  | { mutation: 'removeQuantTable'; id: number }
  | { mutation: 'setHuffmanTable'; table: JpgHuffmanTable }
  | { mutation: 'removeHuffmanTable'; key: JpgHuffmanTableKey }
  | { mutation: 'setRestartInterval'; restartInterval?: number }
  | { mutation: 'insertOtherSegment'; index: number; segment: JpgSegment }
  | { mutation: 'removeOtherSegment'; index: number }
  | { mutation: 'setPixels'; pixels: number[] }
  | { mutation: 'setReEncodeQuality'; quality?: number };
