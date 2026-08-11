/** 🔺️ BmpDiff schema facet — mirrors 🦀️component.rs field-for-field. No full-replace slot:
 * every BITMAPINFOHEADER field is a sparse patch scalar, `palette` is an index-keyed
 * removed/modified/added triple, `pixels` is a whole-buffer replace. */

export interface BmpPaletteModified {
  index: number;
  entry: import('../📸️snapshot/🟦️component.ts').BmpPaletteEntry;
}

export interface BmpPaletteAdded {
  index: number;
  entry: import('../📸️snapshot/🟦️component.ts').BmpPaletteEntry;
}

export interface BmpPaletteDiff {
  removed?: number[];
  modified?: BmpPaletteModified[];
  added?: BmpPaletteAdded[];
}

export interface BmpDiff {
  headerSize?: number;
  width?: number;
  height?: number;
  rowOrder?: import('../📸️snapshot/🟦️component.ts').BmpRowOrder;
  planes?: number;
  bitsPerPixel?: number;
  compression?: number;
  imageSize?: number;
  xPixelsPerMeter?: number;
  yPixelsPerMeter?: number;
  colorsUsed?: number;
  colorsImportant?: number;
  palette?: BmpPaletteDiff;
  pixels?: number[];
}
