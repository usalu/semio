/** 🧬️ BmpArtifact schema facet — full artifact state, mirrors BmpSnapshot field-for-field
 * (see ./📸️snapshot/🟦️.ts for BmpPaletteEntry/BmpRowOrder). */
import type { BmpPaletteEntry, BmpRowOrder } from './📸️snapshot/🟦️.ts';

export interface BmpArtifact {
  schema: string;
  headerSize: number;
  width: number;
  height: number;
  rowOrder: BmpRowOrder;
  planes: number;
  bitsPerPixel: number;
  compression: number;
  imageSize: number;
  xPixelsPerMeter: number;
  yPixelsPerMeter: number;
  colorsUsed: number;
  colorsImportant: number;
  palette: BmpPaletteEntry[];
  pixels: number[];
}
