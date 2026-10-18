/** 🔺️ Sparse GIF87a document delta over the owned snapshot: whole-value screen scalars, tri-state palette slots and index-keyed row triples (`removed` base indices, `modified` row deltas, `added` rows with their final index). */
import type { GifColorTable, GifImage } from "../📸️snapshot/🟦️.ts";

export interface GifImageDiff { left?: number; top?: number; width?: number; height?: number; interlace?: boolean; lct?: GifColorTable | null; indices?: number[] }
export interface GifImageModified { index: number; diff: GifImageDiff }
export interface GifImageAdded { index: number; image: GifImage }
export interface GifImagesDiff { removed?: number[]; modified?: GifImageModified[]; added?: GifImageAdded[] }
export interface GifDiff { width?: number; height?: number; gct?: GifColorTable | null; backgroundColorIndex?: number; pixelAspectRatio?: number; images?: GifImagesDiff }

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioGif87aAnyDiffGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioGif87aAnyDiffGuardReject = (at: string, why: string): never => {
  throw new stdioGif87aAnyDiffGuardRefusal(at, why);
};

type stdioGif87aAnyDiffGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioGif87aAnyDiffGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioGif87aAnyDiffGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioGif87aAnyDiffGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioGif87aAnyDiffGuardReject(at, "value is not an object");
export const stdioGif87aAnyDiffGuardClosed = (row: Readonly<Record<string, unknown>>, at: string, keys: readonly string[]): Readonly<Record<string, unknown>> => {
  for (const key of Object.keys(row)) if (!keys.includes(key)) stdioGif87aAnyDiffGuardReject(`${at}.${key}`, "unknown property");
  return row;
};
export const stdioGif87aAnyDiffGuardArray = (value: unknown, at: string, bounds: stdioGif87aAnyDiffGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioGif87aAnyDiffGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioGif87aAnyDiffGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioGif87aAnyDiffGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioGif87aAnyDiffGuardString = (value: unknown, at: string, bounds: stdioGif87aAnyDiffGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioGif87aAnyDiffGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioGif87aAnyDiffGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioGif87aAnyDiffGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioGif87aAnyDiffGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioGif87aAnyDiffGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioGif87aAnyDiffGuardReject(at, "value is not a boolean"));
export const stdioGif87aAnyDiffGuardNumber = (value: unknown, at: string, bounds: stdioGif87aAnyDiffGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioGif87aAnyDiffGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioGif87aAnyDiffGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioGif87aAnyDiffGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioGif87aAnyDiffGuardInteger = (value: unknown, at: string, bounds: stdioGif87aAnyDiffGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioGif87aAnyDiffGuardNumber(value, at, bounds) : stdioGif87aAnyDiffGuardReject(at, "value is not an integer");
export const stdioGif87aAnyDiffGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioGif87aAnyDiffGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioGif87aAnyDiffGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioGif87aAnyDiffGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

const known = (row: Readonly<Record<string, unknown>>, at: string, keys: readonly string[]): void => {
  for (const key of Object.keys(row)) if (!keys.includes(key)) throw new stdioGif87aAnyDiffGuardRefusal(`${at}.${key}`, "unknown property");
};
const present = <T>(row: Readonly<Record<string, unknown>>, key: string, at: string, parse: (value: unknown, at: string) => T): T | undefined => (row[key] === undefined ? undefined : parse(row[key], `${at}.${key}`));
const nullable = <T>(parse: (value: unknown, at: string) => T) => (value: unknown, at: string): T | null => (value === null ? null : parse(value, at));
const byte = (value: unknown, at: string): number => stdioGif87aAnyDiffGuardInteger(value, at, { minimum: 0, maximum: 255 });
const word = (value: unknown, at: string): number => stdioGif87aAnyDiffGuardInteger(value, at, { minimum: 0, maximum: 65535 });
const dword = (value: unknown, at: string): number => stdioGif87aAnyDiffGuardInteger(value, at, { minimum: 0, maximum: 4294967295 });
const index = (value: unknown, at: string): number => stdioGif87aAnyDiffGuardInteger(value, at, { minimum: 0 });
const list = <T>(value: unknown, at: string, parse: (value: unknown, at: string) => T): T[] => stdioGif87aAnyDiffGuardArray(value, at).map((item, position) => parse(item, `${at}[${position}]`));
const octets = (value: unknown, at: string): number[] => list(value, at, byte);
const exactOctets = (count: number) => (value: unknown, at: string): number[] => { const found = stdioGif87aAnyDiffGuardArray(value, at, { minItems: count, maxItems: count }); return found.map((item, position) => byte(item, `${at}[${position}]`)); };


const rgb = (value: unknown, at: string): { r: number; g: number; b: number } => { const row = stdioGif87aAnyDiffGuardObject(value, at); known(row, at, ["r", "g", "b"]); return { r: byte(row["r"], `${at}.r`), g: byte(row["g"], `${at}.g`), b: byte(row["b"], `${at}.b`) }; };
const colorTable = (value: unknown, at: string): GifColorTable => { const row = stdioGif87aAnyDiffGuardObject(value, at); known(row, at, ["sorted", "colors"]); return { sorted: stdioGif87aAnyDiffGuardBoolean(row["sorted"], `${at}.sorted`), colors: list(row["colors"], `${at}.colors`, rgb) }; };


const image = (value: unknown, at: string): GifImage => { const row = stdioGif87aAnyDiffGuardObject(value, at); known(row, at, ["left", "top", "width", "height", "interlace", "lct", "indices"]); return { left: dword(row["left"], `${at}.left`), top: dword(row["top"], `${at}.top`), width: dword(row["width"], `${at}.width`), height: dword(row["height"], `${at}.height`), interlace: stdioGif87aAnyDiffGuardBoolean(row["interlace"], `${at}.interlace`), lct: nullable(colorTable)(row["lct"], `${at}.lct`), indices: octets(row["indices"], `${at}.indices`) }; };

export function parseGifImageDiff(value: unknown, at = "$"): GifImageDiff {
  const row = stdioGif87aAnyDiffGuardObject(value, at);
  known(row, at, ["left", "top", "width", "height", "interlace", "lct", "indices"]);
  return {
    left: present(row, "left", at, dword), top: present(row, "top", at, dword), width: present(row, "width", at, dword), height: present(row, "height", at, dword),
    interlace: present(row, "interlace", at, stdioGif87aAnyDiffGuardBoolean), lct: present(row, "lct", at, nullable(colorTable)), indices: present(row, "indices", at, octets),
  };
}
export function parseGifImageModified(value: unknown, at = "$"): GifImageModified {
  const row = stdioGif87aAnyDiffGuardClosed(stdioGif87aAnyDiffGuardObject(value, at), at, ["index", "diff"]);
  return {
    index: stdioGif87aAnyDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    diff: parseGifImageDiff(row["diff"], `${at}.diff`),
  };
}
export function parseGifImageAdded(value: unknown, at = "$"): GifImageAdded {
  const row = stdioGif87aAnyDiffGuardObject(value, at);
  known(row, at, ["index", "image"]);
  return { index: index(row["index"], `${at}.index`), image: image(row["image"], `${at}.image`) };
}
export function parseGifImagesDiff(value: unknown, at = "$"): GifImagesDiff {
  const row = stdioGif87aAnyDiffGuardClosed(stdioGif87aAnyDiffGuardObject(value, at), at, ["removed", "modified", "added"]);
  return {
    removed: row["removed"] === undefined ? undefined : stdioGif87aAnyDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => stdioGif87aAnyDiffGuardInteger(item, `${at}.removed[${index}]`, {"minimum": 0})),
    modified: row["modified"] === undefined ? undefined : stdioGif87aAnyDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseGifImageModified(item, `${at}.modified[${index}]`)),
    added: row["added"] === undefined ? undefined : stdioGif87aAnyDiffGuardArray(row["added"], `${at}.added`).map((item, index) => parseGifImageAdded(item, `${at}.added[${index}]`)),
  };
}
export function parseGifDiff(value: unknown, at = "$"): GifDiff {
  const row = stdioGif87aAnyDiffGuardObject(value, at);
  known(row, at, ["width", "height", "gct", "backgroundColorIndex", "pixelAspectRatio", "images"]);
  return {
    width: present(row, "width", at, dword), height: present(row, "height", at, dword), gct: present(row, "gct", at, nullable(colorTable)),
    backgroundColorIndex: present(row, "backgroundColorIndex", at, byte), pixelAspectRatio: present(row, "pixelAspectRatio", at, byte), images: present(row, "images", at, parseGifImagesDiff),
  };
}
