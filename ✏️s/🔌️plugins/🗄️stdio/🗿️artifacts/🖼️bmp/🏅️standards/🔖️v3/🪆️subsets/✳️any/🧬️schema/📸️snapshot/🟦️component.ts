/** 🧬️ BmpSnapshot schema facet — mirrors 🦀️component.rs field-for-field. */

/** 📐 BITMAPINFOHEADER's signed `height` field: `bottomUp` (positive, the common case) or
 * `topDown` (negative). Decoded `pixels` are always canonicalized to row 0 = image top
 * regardless of this value. */
export type BmpRowOrder = 'bottomUp' | 'topDown';

/** 🎨 One BITMAPINFOHEADER color-table entry, on-disk field order (present when
 * `bitsPerPixel <= 8`) — a weak/value entity, whole-value replaced in diffs. */
export interface BmpPaletteEntry {
  b: number;
  g: number;
  r: number;
  reserved: number;
}

/** 📸️ Persisted `stdio.bmp` snapshot: full BITMAPINFOHEADER (11 real fields) + palette +
 * decoded canonical 8-bit RGBA `pixels` (`width * height * 4` bytes, row 0 = image top). */
export interface BmpSnapshot {
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
