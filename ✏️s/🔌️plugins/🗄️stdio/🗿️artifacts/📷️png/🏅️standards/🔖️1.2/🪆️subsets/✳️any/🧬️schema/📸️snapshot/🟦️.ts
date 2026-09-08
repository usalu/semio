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

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioPng12AnySnapshotGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioPng12AnySnapshotGuardReject = (at: string, why: string): never => {
  throw new stdioPng12AnySnapshotGuardRefusal(at, why);
};

type stdioPng12AnySnapshotGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioPng12AnySnapshotGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioPng12AnySnapshotGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioPng12AnySnapshotGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioPng12AnySnapshotGuardReject(at, "value is not an object");
export const stdioPng12AnySnapshotGuardArray = (value: unknown, at: string, bounds: stdioPng12AnySnapshotGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioPng12AnySnapshotGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioPng12AnySnapshotGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioPng12AnySnapshotGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioPng12AnySnapshotGuardString = (value: unknown, at: string, bounds: stdioPng12AnySnapshotGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioPng12AnySnapshotGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioPng12AnySnapshotGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioPng12AnySnapshotGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioPng12AnySnapshotGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioPng12AnySnapshotGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioPng12AnySnapshotGuardReject(at, "value is not a boolean"));
export const stdioPng12AnySnapshotGuardNumber = (value: unknown, at: string, bounds: stdioPng12AnySnapshotGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioPng12AnySnapshotGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioPng12AnySnapshotGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioPng12AnySnapshotGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioPng12AnySnapshotGuardInteger = (value: unknown, at: string, bounds: stdioPng12AnySnapshotGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioPng12AnySnapshotGuardNumber(value, at, bounds) : stdioPng12AnySnapshotGuardReject(at, "value is not an integer");
export const stdioPng12AnySnapshotGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioPng12AnySnapshotGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioPng12AnySnapshotGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioPng12AnySnapshotGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parsePngSnapshot(value: unknown, at = "$"): PngSnapshot {
  const row = stdioPng12AnySnapshotGuardObject(value, at);
  return {
    schema: stdioPng12AnySnapshotGuardString(row["schema"], `${at}.schema`),
    width: stdioPng12AnySnapshotGuardInteger(row["width"], `${at}.width`, {"minimum": 0}),
    height: stdioPng12AnySnapshotGuardInteger(row["height"], `${at}.height`, {"minimum": 0}),
    bitDepth: stdioPng12AnySnapshotGuardInteger(row["bitDepth"], `${at}.bitDepth`, {"minimum": 0, "maximum": 16}),
    colorType: parsePngColorType(row["colorType"], `${at}.colorType`),
    interlace: stdioPng12AnySnapshotGuardBoolean(row["interlace"], `${at}.interlace`),
    plte: row["plte"] === undefined ? undefined : stdioPng12AnySnapshotGuardArray(row["plte"], `${at}.plte`).map((item, index) => parsePngRgb(item, `${at}.plte[${index}]`)),
    trns: row["trns"] === undefined ? undefined : parsePngTransparency(row["trns"], `${at}.trns`),
    gama: row["gama"] === undefined ? undefined : stdioPng12AnySnapshotGuardInteger(row["gama"], `${at}.gama`, {"minimum": 0}),
    chrm: row["chrm"] === undefined ? undefined : parsePngChromaticities(row["chrm"], `${at}.chrm`),
    srgb: row["srgb"] === undefined ? undefined : parsePngSrgbIntent(row["srgb"], `${at}.srgb`),
    phys: row["phys"] === undefined ? undefined : parsePngPhysicalDims(row["phys"], `${at}.phys`),
    time: row["time"] === undefined ? undefined : parsePngTimestamp(row["time"], `${at}.time`),
    bkgd: row["bkgd"] === undefined ? undefined : parsePngBackground(row["bkgd"], `${at}.bkgd`),
    textChunks: stdioPng12AnySnapshotGuardArray(row["textChunks"], `${at}.textChunks`).map((item, index) => parsePngTextChunk(item, `${at}.textChunks[${index}]`)),
    pixels: stdioPng12AnySnapshotGuardArray(row["pixels"], `${at}.pixels`).map((item, index) => stdioPng12AnySnapshotGuardInteger(item, `${at}.pixels[${index}]`, {"minimum": 0, "maximum": 255})),
    chunkOrder: stdioPng12AnySnapshotGuardArray(row["chunkOrder"], `${at}.chunkOrder`).map((item, index) => parsePngChunkMarker(item, `${at}.chunkOrder[${index}]`)),
    unknownChunks: stdioPng12AnySnapshotGuardArray(row["unknownChunks"], `${at}.unknownChunks`).map((item, index) => parsePngChunk(item, `${at}.unknownChunks[${index}]`)),
  };
}

export function parsePngColorType(value: unknown, at = "$"): PngColorType {
  return stdioPng12AnySnapshotGuardMember(value, `${at}`, ["grayscale", "rgb", "palette", "grayscaleAlpha", "rgba"] as const);
}

export function parsePngRgb(value: unknown, at = "$"): PngRgb {
  const row = stdioPng12AnySnapshotGuardObject(value, at);
  return {
    r: stdioPng12AnySnapshotGuardInteger(row["r"], `${at}.r`, {"minimum": 0, "maximum": 255}),
    g: stdioPng12AnySnapshotGuardInteger(row["g"], `${at}.g`, {"minimum": 0, "maximum": 255}),
    b: stdioPng12AnySnapshotGuardInteger(row["b"], `${at}.b`, {"minimum": 0, "maximum": 255}),
  };
}

export function parsePngTransparency(value: unknown, at = "$"): PngTransparency {
  const row = stdioPng12AnySnapshotGuardObject(value, at);
  return {
    colorType: stdioPng12AnySnapshotGuardMember(row["colorType"], `${at}.colorType`, ["indexed", "grayscale", "rgb"] as const),
    alpha: row["alpha"] === undefined ? undefined : stdioPng12AnySnapshotGuardArray(row["alpha"], `${at}.alpha`).map((item, index) => stdioPng12AnySnapshotGuardInteger(item, `${at}.alpha[${index}]`, {"minimum": 0, "maximum": 255})),
    gray: row["gray"] === undefined ? undefined : stdioPng12AnySnapshotGuardInteger(row["gray"], `${at}.gray`, {"minimum": 0, "maximum": 65535}),
    r: row["r"] === undefined ? undefined : stdioPng12AnySnapshotGuardInteger(row["r"], `${at}.r`, {"minimum": 0, "maximum": 65535}),
    g: row["g"] === undefined ? undefined : stdioPng12AnySnapshotGuardInteger(row["g"], `${at}.g`, {"minimum": 0, "maximum": 65535}),
    b: row["b"] === undefined ? undefined : stdioPng12AnySnapshotGuardInteger(row["b"], `${at}.b`, {"minimum": 0, "maximum": 65535}),
  };
}

export function parsePngChromaticities(value: unknown, at = "$"): PngChromaticities {
  const row = stdioPng12AnySnapshotGuardObject(value, at);
  return {
    whiteX: stdioPng12AnySnapshotGuardInteger(row["whiteX"], `${at}.whiteX`),
    whiteY: stdioPng12AnySnapshotGuardInteger(row["whiteY"], `${at}.whiteY`),
    redX: stdioPng12AnySnapshotGuardInteger(row["redX"], `${at}.redX`),
    redY: stdioPng12AnySnapshotGuardInteger(row["redY"], `${at}.redY`),
    greenX: stdioPng12AnySnapshotGuardInteger(row["greenX"], `${at}.greenX`),
    greenY: stdioPng12AnySnapshotGuardInteger(row["greenY"], `${at}.greenY`),
    blueX: stdioPng12AnySnapshotGuardInteger(row["blueX"], `${at}.blueX`),
    blueY: stdioPng12AnySnapshotGuardInteger(row["blueY"], `${at}.blueY`),
  };
}

export function parsePngSrgbIntent(value: unknown, at = "$"): PngSrgbIntent {
  return stdioPng12AnySnapshotGuardMember(value, `${at}`, ["perceptual", "relativeColorimetric", "saturation", "absoluteColorimetric"] as const);
}

export function parsePngPhysicalDims(value: unknown, at = "$"): PngPhysicalDims {
  const row = stdioPng12AnySnapshotGuardObject(value, at);
  return {
    ppuX: stdioPng12AnySnapshotGuardInteger(row["ppuX"], `${at}.ppuX`, {"minimum": 0}),
    ppuY: stdioPng12AnySnapshotGuardInteger(row["ppuY"], `${at}.ppuY`, {"minimum": 0}),
    unitIsMeter: stdioPng12AnySnapshotGuardBoolean(row["unitIsMeter"], `${at}.unitIsMeter`),
  };
}

export function parsePngTimestamp(value: unknown, at = "$"): PngTimestamp {
  const row = stdioPng12AnySnapshotGuardObject(value, at);
  return {
    year: stdioPng12AnySnapshotGuardInteger(row["year"], `${at}.year`),
    month: stdioPng12AnySnapshotGuardInteger(row["month"], `${at}.month`),
    day: stdioPng12AnySnapshotGuardInteger(row["day"], `${at}.day`),
    hour: stdioPng12AnySnapshotGuardInteger(row["hour"], `${at}.hour`),
    minute: stdioPng12AnySnapshotGuardInteger(row["minute"], `${at}.minute`),
    second: stdioPng12AnySnapshotGuardInteger(row["second"], `${at}.second`),
  };
}

export function parsePngBackground(value: unknown, at = "$"): PngBackground {
  const row = stdioPng12AnySnapshotGuardObject(value, at);
  return {
    colorType: stdioPng12AnySnapshotGuardMember(row["colorType"], `${at}.colorType`, ["grayscale", "rgb", "indexed"] as const),
    gray: row["gray"] === undefined ? undefined : stdioPng12AnySnapshotGuardInteger(row["gray"], `${at}.gray`),
    r: row["r"] === undefined ? undefined : stdioPng12AnySnapshotGuardInteger(row["r"], `${at}.r`),
    g: row["g"] === undefined ? undefined : stdioPng12AnySnapshotGuardInteger(row["g"], `${at}.g`),
    b: row["b"] === undefined ? undefined : stdioPng12AnySnapshotGuardInteger(row["b"], `${at}.b`),
    index: row["index"] === undefined ? undefined : stdioPng12AnySnapshotGuardInteger(row["index"], `${at}.index`),
  };
}

export function parsePngTextKind(value: unknown, at = "$"): PngTextKind {
  return stdioPng12AnySnapshotGuardMember(value, `${at}`, ["text", "zText", "iText"] as const);
}

export function parsePngTextChunk(value: unknown, at = "$"): PngTextChunk {
  const row = stdioPng12AnySnapshotGuardObject(value, at);
  return {
    keyword: stdioPng12AnySnapshotGuardString(row["keyword"], `${at}.keyword`),
    value: stdioPng12AnySnapshotGuardString(row["value"], `${at}.value`),
    compressed: stdioPng12AnySnapshotGuardBoolean(row["compressed"], `${at}.compressed`),
    kind: parsePngTextKind(row["kind"], `${at}.kind`),
    languageTag: stdioPng12AnySnapshotGuardString(row["languageTag"], `${at}.languageTag`),
    translatedKeyword: stdioPng12AnySnapshotGuardString(row["translatedKeyword"], `${at}.translatedKeyword`),
  };
}

export function parsePngChunk(value: unknown, at = "$"): PngChunk {
  const row = stdioPng12AnySnapshotGuardObject(value, at);
  return {
    kind: stdioPng12AnySnapshotGuardArray(row["kind"], `${at}.kind`, {"minItems": 4, "maxItems": 4}).map((item, index) => stdioPng12AnySnapshotGuardInteger(item, `${at}.kind[${index}]`, {"minimum": 0, "maximum": 255})),
    data: stdioPng12AnySnapshotGuardArray(row["data"], `${at}.data`).map((item, index) => stdioPng12AnySnapshotGuardInteger(item, `${at}.data[${index}]`, {"minimum": 0, "maximum": 255})),
  };
}

export function parsePngChunkMarker(value: unknown, at = "$"): PngChunkMarker {
  const row = stdioPng12AnySnapshotGuardObject(value, at);
  return {
    chunk: stdioPng12AnySnapshotGuardMember(row["chunk"], `${at}.chunk`, ["ihdr", "plte", "trns", "gama", "chrm", "srgb", "phys", "time", "bkgd", "idat", "iend", "text", "unknown"] as const),
    index: row["index"] === undefined ? undefined : stdioPng12AnySnapshotGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
  };
}
