/** 🧬️ TiffMutation union — mirrors 🦀️component.rs's `#[serde(tag = "mutation")]` enum. */
import type { TiffByteOrder, TiffFieldType, TiffIfd, TiffSnapshot, TiffValues } from '../📸️snapshot/🟦️component.ts';

export type TiffMutation =
  | { mutation: 'noMutation' }
  | { mutation: 'setSnapshot'; snapshot: TiffSnapshot }
  | { mutation: 'setByteOrder'; byteOrder: TiffByteOrder }
  | { mutation: 'insertIfd'; index: number; ifd: TiffIfd }
  | { mutation: 'removeIfd'; index: number }
  | { mutation: 'setTag'; ifdIndex: number; tag: number; kind: TiffFieldType; values: TiffValues }
  | { mutation: 'removeTag'; ifdIndex: number; tag: number }
  | { mutation: 'setPixels'; pixels: number[] };
