/** 🔺️ Sparse GIF89a document delta over the owned snapshot: whole-value screen scalars, tri-state palette slots and index-keyed row triples (`removed` base indices, `modified` row deltas, `added` rows with their final index). */
import type { GifAppExtension, GifColorTable, GifDisposal, GifFrame, GifPlainText } from "../📸️snapshot/🟦️.ts";

export interface GifFrameDiff { left?: number; top?: number; width?: number; height?: number; interlace?: boolean; lct?: GifColorTable | null; indices?: number[]; delayCs?: number; disposal?: GifDisposal; transparentIndex?: number | null; userInput?: boolean; plainText?: GifPlainText | null }
export interface GifFrameModified { index: number; diff: GifFrameDiff }
export interface GifFrameAdded { index: number; frame: GifFrame }
export interface GifFramesDiff { removed?: number[]; modified?: GifFrameModified[]; added?: GifFrameAdded[] }
export interface GifCommentModified { index: number; text: string }
export interface GifCommentAdded { index: number; text: string }
export interface GifCommentsDiff { removed?: number[]; modified?: GifCommentModified[]; added?: GifCommentAdded[] }
export interface GifAppExtensionModified { index: number; extension: GifAppExtension }
export interface GifAppExtensionAdded { index: number; extension: GifAppExtension }
export interface GifAppExtensionsDiff { removed?: number[]; modified?: GifAppExtensionModified[]; added?: GifAppExtensionAdded[] }
export interface GifDiff { width?: number; height?: number; gct?: GifColorTable | null; backgroundColorIndex?: number; pixelAspectRatio?: number; loopCount?: number | null; frames?: GifFramesDiff; comments?: GifCommentsDiff; appExtensions?: GifAppExtensionsDiff }

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioGif89aBaseDiffGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioGif89aBaseDiffGuardReject = (at: string, why: string): never => {
  throw new stdioGif89aBaseDiffGuardRefusal(at, why);
};

type stdioGif89aBaseDiffGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioGif89aBaseDiffGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioGif89aBaseDiffGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioGif89aBaseDiffGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioGif89aBaseDiffGuardReject(at, "value is not an object");
export const stdioGif89aBaseDiffGuardClosed = (row: Readonly<Record<string, unknown>>, at: string, keys: readonly string[]): Readonly<Record<string, unknown>> => {
  for (const key of Object.keys(row)) if (!keys.includes(key)) stdioGif89aBaseDiffGuardReject(`${at}.${key}`, "unknown property");
  return row;
};
export const stdioGif89aBaseDiffGuardArray = (value: unknown, at: string, bounds: stdioGif89aBaseDiffGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioGif89aBaseDiffGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioGif89aBaseDiffGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioGif89aBaseDiffGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioGif89aBaseDiffGuardString = (value: unknown, at: string, bounds: stdioGif89aBaseDiffGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioGif89aBaseDiffGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioGif89aBaseDiffGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioGif89aBaseDiffGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioGif89aBaseDiffGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioGif89aBaseDiffGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioGif89aBaseDiffGuardReject(at, "value is not a boolean"));
export const stdioGif89aBaseDiffGuardNumber = (value: unknown, at: string, bounds: stdioGif89aBaseDiffGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioGif89aBaseDiffGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioGif89aBaseDiffGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioGif89aBaseDiffGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioGif89aBaseDiffGuardInteger = (value: unknown, at: string, bounds: stdioGif89aBaseDiffGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioGif89aBaseDiffGuardNumber(value, at, bounds) : stdioGif89aBaseDiffGuardReject(at, "value is not an integer");
export const stdioGif89aBaseDiffGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioGif89aBaseDiffGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioGif89aBaseDiffGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioGif89aBaseDiffGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

const known = (row: Readonly<Record<string, unknown>>, at: string, keys: readonly string[]): void => {
  for (const key of Object.keys(row)) if (!keys.includes(key)) throw new stdioGif89aBaseDiffGuardRefusal(`${at}.${key}`, "unknown property");
};
const present = <T>(row: Readonly<Record<string, unknown>>, key: string, at: string, parse: (value: unknown, at: string) => T): T | undefined => (row[key] === undefined ? undefined : parse(row[key], `${at}.${key}`));
const nullable = <T>(parse: (value: unknown, at: string) => T) => (value: unknown, at: string): T | null => (value === null ? null : parse(value, at));
const byte = (value: unknown, at: string): number => stdioGif89aBaseDiffGuardInteger(value, at, { minimum: 0, maximum: 255 });
const word = (value: unknown, at: string): number => stdioGif89aBaseDiffGuardInteger(value, at, { minimum: 0, maximum: 65535 });
const dword = (value: unknown, at: string): number => stdioGif89aBaseDiffGuardInteger(value, at, { minimum: 0, maximum: 4294967295 });
const index = (value: unknown, at: string): number => stdioGif89aBaseDiffGuardInteger(value, at, { minimum: 0 });
const list = <T>(value: unknown, at: string, parse: (value: unknown, at: string) => T): T[] => stdioGif89aBaseDiffGuardArray(value, at).map((item, position) => parse(item, `${at}[${position}]`));
const octets = (value: unknown, at: string): number[] => list(value, at, byte);
const exactOctets = (count: number) => (value: unknown, at: string): number[] => { const found = stdioGif89aBaseDiffGuardArray(value, at, { minItems: count, maxItems: count }); return found.map((item, position) => byte(item, `${at}[${position}]`)); };


const rgb = (value: unknown, at: string): { r: number; g: number; b: number } => { const row = stdioGif89aBaseDiffGuardObject(value, at); known(row, at, ["r", "g", "b"]); return { r: byte(row["r"], `${at}.r`), g: byte(row["g"], `${at}.g`), b: byte(row["b"], `${at}.b`) }; };
const colorTable = (value: unknown, at: string): GifColorTable => { const row = stdioGif89aBaseDiffGuardObject(value, at); known(row, at, ["sorted", "colors"]); return { sorted: stdioGif89aBaseDiffGuardBoolean(row["sorted"], `${at}.sorted`), colors: list(row["colors"], `${at}.colors`, rgb) }; };


const disposal = (value: unknown, at: string): GifDisposal => stdioGif89aBaseDiffGuardMember(value, at, ["unspecified", "doNotDispose", "restoreToBackground", "restoreToPrevious"] as const);
const plainText = (value: unknown, at: string): GifPlainText => { const row = stdioGif89aBaseDiffGuardObject(value, at); known(row, at, ["left", "top", "width", "height", "cellWidth", "cellHeight", "fgColorIndex", "bgColorIndex", "text"]); return { left: dword(row["left"], `${at}.left`), top: dword(row["top"], `${at}.top`), width: dword(row["width"], `${at}.width`), height: dword(row["height"], `${at}.height`), cellWidth: byte(row["cellWidth"], `${at}.cellWidth`), cellHeight: byte(row["cellHeight"], `${at}.cellHeight`), fgColorIndex: byte(row["fgColorIndex"], `${at}.fgColorIndex`), bgColorIndex: byte(row["bgColorIndex"], `${at}.bgColorIndex`), text: stdioGif89aBaseDiffGuardString(row["text"], `${at}.text`) }; };
const application = (value: unknown, at: string): GifAppExtension => { const row = stdioGif89aBaseDiffGuardObject(value, at); known(row, at, ["identifier", "authCode", "data"]); return { identifier: identifier(row["identifier"], `${at}.identifier`), authCode: authCode(row["authCode"], `${at}.authCode`), data: octets(row["data"], `${at}.data`) }; };
const frame = (value: unknown, at: string): GifFrame => { const row = stdioGif89aBaseDiffGuardObject(value, at); known(row, at, ["left", "top", "width", "height", "interlace", "lct", "indices", "delayCs", "disposal", "transparentIndex", "userInput", "plainText"]); return { left: dword(row["left"], `${at}.left`), top: dword(row["top"], `${at}.top`), width: dword(row["width"], `${at}.width`), height: dword(row["height"], `${at}.height`), interlace: stdioGif89aBaseDiffGuardBoolean(row["interlace"], `${at}.interlace`), lct: nullable(colorTable)(row["lct"], `${at}.lct`), indices: octets(row["indices"], `${at}.indices`), delayCs: word(row["delayCs"], `${at}.delayCs`), disposal: disposal(row["disposal"], `${at}.disposal`), transparentIndex: nullable(byte)(row["transparentIndex"], `${at}.transparentIndex`), userInput: stdioGif89aBaseDiffGuardBoolean(row["userInput"], `${at}.userInput`), plainText: nullable(plainText)(row["plainText"], `${at}.plainText`) }; };
const identifier = (value: unknown, at: string): GifAppExtension["identifier"] => { const o = exactOctets(8)(value, at); return [o[0]!, o[1]!, o[2]!, o[3]!, o[4]!, o[5]!, o[6]!, o[7]!]; };
const authCode = (value: unknown, at: string): GifAppExtension["authCode"] => { const o = exactOctets(3)(value, at); return [o[0]!, o[1]!, o[2]!]; };

export function parseGifFrameDiff(value: unknown, at = "$"): GifFrameDiff {
  const row = stdioGif89aBaseDiffGuardObject(value, at);
  known(row, at, ["left", "top", "width", "height", "interlace", "lct", "indices", "delayCs", "disposal", "transparentIndex", "userInput", "plainText"]);
  return {
    left: present(row, "left", at, dword), top: present(row, "top", at, dword), width: present(row, "width", at, dword), height: present(row, "height", at, dword),
    interlace: present(row, "interlace", at, stdioGif89aBaseDiffGuardBoolean), lct: present(row, "lct", at, nullable(colorTable)), indices: present(row, "indices", at, octets), delayCs: present(row, "delayCs", at, word),
    disposal: present(row, "disposal", at, disposal), transparentIndex: present(row, "transparentIndex", at, nullable(byte)), userInput: present(row, "userInput", at, stdioGif89aBaseDiffGuardBoolean), plainText: present(row, "plainText", at, nullable(plainText)),
  };
}
export function parseGifFrameModified(value: unknown, at = "$"): GifFrameModified {
  const row = stdioGif89aBaseDiffGuardClosed(stdioGif89aBaseDiffGuardObject(value, at), at, ["index", "diff"]);
  return {
    index: stdioGif89aBaseDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    diff: parseGifFrameDiff(row["diff"], `${at}.diff`),
  };
}
export function parseGifFrameAdded(value: unknown, at = "$"): GifFrameAdded {
  const row = stdioGif89aBaseDiffGuardObject(value, at);
  known(row, at, ["index", "frame"]);
  return { index: index(row["index"], `${at}.index`), frame: frame(row["frame"], `${at}.frame`) };
}
export function parseGifFramesDiff(value: unknown, at = "$"): GifFramesDiff {
  const row = stdioGif89aBaseDiffGuardClosed(stdioGif89aBaseDiffGuardObject(value, at), at, ["removed", "modified", "added"]);
  return {
    removed: row["removed"] === undefined ? undefined : stdioGif89aBaseDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => stdioGif89aBaseDiffGuardInteger(item, `${at}.removed[${index}]`, {"minimum": 0})),
    modified: row["modified"] === undefined ? undefined : stdioGif89aBaseDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseGifFrameModified(item, `${at}.modified[${index}]`)),
    added: row["added"] === undefined ? undefined : stdioGif89aBaseDiffGuardArray(row["added"], `${at}.added`).map((item, index) => parseGifFrameAdded(item, `${at}.added[${index}]`)),
  };
}
export function parseGifCommentModified(value: unknown, at = "$"): GifCommentModified {
  const row = stdioGif89aBaseDiffGuardClosed(stdioGif89aBaseDiffGuardObject(value, at), at, ["index", "text"]);
  return {
    index: stdioGif89aBaseDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    text: stdioGif89aBaseDiffGuardString(row["text"], `${at}.text`),
  };
}
export function parseGifCommentAdded(value: unknown, at = "$"): GifCommentAdded {
  const row = stdioGif89aBaseDiffGuardClosed(stdioGif89aBaseDiffGuardObject(value, at), at, ["index", "text"]);
  return {
    index: stdioGif89aBaseDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    text: stdioGif89aBaseDiffGuardString(row["text"], `${at}.text`),
  };
}
export function parseGifCommentsDiff(value: unknown, at = "$"): GifCommentsDiff {
  const row = stdioGif89aBaseDiffGuardClosed(stdioGif89aBaseDiffGuardObject(value, at), at, ["removed", "modified", "added"]);
  return {
    removed: row["removed"] === undefined ? undefined : stdioGif89aBaseDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => stdioGif89aBaseDiffGuardInteger(item, `${at}.removed[${index}]`, {"minimum": 0})),
    modified: row["modified"] === undefined ? undefined : stdioGif89aBaseDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseGifCommentModified(item, `${at}.modified[${index}]`)),
    added: row["added"] === undefined ? undefined : stdioGif89aBaseDiffGuardArray(row["added"], `${at}.added`).map((item, index) => parseGifCommentAdded(item, `${at}.added[${index}]`)),
  };
}
export function parseGifAppExtensionModified(value: unknown, at = "$"): GifAppExtensionModified {
  const row = stdioGif89aBaseDiffGuardObject(value, at);
  known(row, at, ["index", "extension"]);
  return { index: index(row["index"], `${at}.index`), extension: application(row["extension"], `${at}.extension`) };
}
export function parseGifAppExtensionAdded(value: unknown, at = "$"): GifAppExtensionAdded {
  const row = stdioGif89aBaseDiffGuardObject(value, at);
  known(row, at, ["index", "extension"]);
  return { index: index(row["index"], `${at}.index`), extension: application(row["extension"], `${at}.extension`) };
}
export function parseGifAppExtensionsDiff(value: unknown, at = "$"): GifAppExtensionsDiff {
  const row = stdioGif89aBaseDiffGuardClosed(stdioGif89aBaseDiffGuardObject(value, at), at, ["removed", "modified", "added"]);
  return {
    removed: row["removed"] === undefined ? undefined : stdioGif89aBaseDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => stdioGif89aBaseDiffGuardInteger(item, `${at}.removed[${index}]`, {"minimum": 0})),
    modified: row["modified"] === undefined ? undefined : stdioGif89aBaseDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseGifAppExtensionModified(item, `${at}.modified[${index}]`)),
    added: row["added"] === undefined ? undefined : stdioGif89aBaseDiffGuardArray(row["added"], `${at}.added`).map((item, index) => parseGifAppExtensionAdded(item, `${at}.added[${index}]`)),
  };
}
export function parseGifDiff(value: unknown, at = "$"): GifDiff {
  const row = stdioGif89aBaseDiffGuardObject(value, at);
  known(row, at, ["width", "height", "gct", "backgroundColorIndex", "pixelAspectRatio", "loopCount", "frames", "comments", "appExtensions"]);
  return {
    width: present(row, "width", at, dword), height: present(row, "height", at, dword), gct: present(row, "gct", at, nullable(colorTable)),
    backgroundColorIndex: present(row, "backgroundColorIndex", at, byte), pixelAspectRatio: present(row, "pixelAspectRatio", at, byte), loopCount: present(row, "loopCount", at, nullable(word)),
    frames: present(row, "frames", at, parseGifFramesDiff), comments: present(row, "comments", at, parseGifCommentsDiff), appExtensions: present(row, "appExtensions", at, parseGifAppExtensionsDiff),
  };
}
