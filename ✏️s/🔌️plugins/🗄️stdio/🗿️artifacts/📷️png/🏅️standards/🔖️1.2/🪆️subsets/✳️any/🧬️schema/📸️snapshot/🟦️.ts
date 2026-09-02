/** 🧬️ PngSnapshot schema facet — mirrors 🦀️.rs field-for-field. Complete PNG 1.2
 * semantic model: typed IHDR/PLTE/tRNS, the full typed ancillary set, index-keyed text chunks,
 * decoded pixels, and chunk-order + unknown-chunk verbatim retention. */

/** 🎨️ PNG §11.2.2 IHDR color type. `Palette` requires a `PLTE` chunk. */
export type PngColorType = 'grayscale' | 'rgb' | 'palette' | 'grayscaleAlpha' | 'rgba';

/** 🎨️ One `PLTE` entry — a weak value (whole-value replaced in diffs). */
export interface PngRgb {
  r: number;
  g: number;
  b: number;
}

/** 👁️ Typed `tRNS` payload — shape depends on `colorType` (§11.3.3). */
export type PngTransparency =
  | { colorType: 'indexed'; alpha: number[] }
  | { colorType: 'grayscale'; gray: number }
  | { colorType: 'rgb'; r: number; g: number; b: number };

/** 📐️ `cHRM` — CIE xy chromaticity coordinates, each `value * 100000` (§11.3.5.2). */
export interface PngChromaticities {
  whiteX: number;
  whiteY: number;
  redX: number;
  redY: number;
  greenX: number;
  greenY: number;
  blueX: number;
  blueY: number;
}

/** 🖌️ `sRGB` rendering intent (§11.3.5.3). */
export type PngSrgbIntent = 'perceptual' | 'relativeColorimetric' | 'saturation' | 'absoluteColorimetric';

/** 📏️ `pHYs` — pixel-per-unit density (§11.3.5.4). */
export interface PngPhysicalDims {
  ppuX: number;
  ppuY: number;
  unitIsMeter: boolean;
}

/** 🕰️ `tIME` — last modification time, UTC (§11.3.6.1). */
export interface PngTimestamp {
  year: number;
  month: number;
  day: number;
  hour: number;
  minute: number;
  second: number;
}

/** 🖼️ `bKGD` — default background color; shape depends on `colorType` (§11.3.5.1). */
export type PngBackground =
  | { colorType: 'grayscale'; gray: number }
  | { colorType: 'rgb'; r: number; g: number; b: number }
  | { colorType: 'indexed'; index: number };

/** 🔤 Which of the three PNG text chunk types (§11.3.4) a `PngTextChunk` came from. */
export type PngTextKind = 'text' | 'zText' | 'iText';

/** 💬️ One `tEXt`/`zTXt`/`iTXt` chunk. Index-keyed within `PngSnapshot.textChunks` — PNG
 * explicitly permits duplicate keywords, so keyword identity is unsound as a diff key.
 * `languageTag`/`translatedKeyword` are iTXt-only, empty string for `text`/`zText`. */
export interface PngTextChunk {
  keyword: string;
  value: string;
  compressed: boolean;
  kind: PngTextKind;
  languageTag: string;
  translatedKeyword: string;
}

/** 🗃️ A chunk the codec doesn't specifically model, retained verbatim. */
export interface PngChunk {
  kind: number[]; // 4-byte chunk type, e.g. [0x70,0x72,0x49,0x56] for "prIV"
  data: number[];
}

/** 🧭️ One slot in the file's real chunk sequence. `idat` coalesces every physical IDAT
 * chunk of the source file into one logical position. `text`/`unknown` carry the index into
 * `textChunks`/`unknownChunks` occupying this position. */
export type PngChunkMarker =
  | { chunk: 'ihdr' }
  | { chunk: 'plte' }
  | { chunk: 'trns' }
  | { chunk: 'gama' }
  | { chunk: 'chrm' }
  | { chunk: 'srgb' }
  | { chunk: 'phys' }
  | { chunk: 'time' }
  | { chunk: 'bkgd' }
  | { chunk: 'idat' }
  | { chunk: 'iend' }
  | { chunk: 'text'; index: number }
  | { chunk: 'unknown'; index: number };

/** 📸️ Complete `stdio.png` 1.2 semantic snapshot. `schema` is an identity field, never
 * diffed. IHDR compression method / filter method are always 0, validated on decode, never
 * modeled as mutable fields. `pixels` is always canonical 8-bit-per-channel RGBA, non-
 * interlaced regardless of the source file's own encoding. */
export interface PngSnapshot {
  schema: string;
  width: number;
  height: number;
  bitDepth: number;
  colorType: PngColorType;
  interlace: boolean;
  plte?: PngRgb[];
  trns?: PngTransparency;
  gama?: number;
  chrm?: PngChromaticities;
  srgb?: PngSrgbIntent;
  phys?: PngPhysicalDims;
  time?: PngTimestamp;
  bkgd?: PngBackground;
  textChunks: PngTextChunk[];
  pixels: number[];
  chunkOrder: PngChunkMarker[];
  unknownChunks: PngChunk[];
}
