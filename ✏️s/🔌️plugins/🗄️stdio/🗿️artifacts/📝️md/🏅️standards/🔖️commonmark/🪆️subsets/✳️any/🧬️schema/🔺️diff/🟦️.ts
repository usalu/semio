import type { MdBlock, MdInline } from '../📸️snapshot/🟦️.ts';

/** 🔺️ Diff for `stdio.md`. `blocks` is an index-keyed recursive triple over the top-level block
 * sequence -- no `snapshot` full-replace slot. */
export interface MdDiff {
  blocks?: MdBlocksDiff;
}

/** 🌳 Index-keyed, recursive block-sequence triple. Reused verbatim for `list`'s item content
 * and `blockQuote`'s content -- both are `MdBlock[]`. */
export interface MdBlocksDiff {
  removed: number[];
  modified: MdBlockModified[];
  added: MdBlockAdded[];
}

export interface MdBlockModified {
  index: number;
  diff: MdBlockDiff;
}

export interface MdBlockAdded {
  index: number;
  item: MdBlock;
}

/** 🌳 Per-block diff, shaped like the `MdBlock` it targets. `replace` is the kind-change
 * fallback. `MdInline` fields are always whole-value replaced (weak entity), never sub-diffed. */
export type MdBlockDiff =
  | { kind: 'heading'; level?: number; inlines?: MdInline[] }
  | { kind: 'paragraph'; inlines?: MdInline[] }
  | { kind: 'list'; ordered?: boolean; start?: number | null; tight?: boolean; items?: MdListItemsDiff }
  | { kind: 'codeBlock'; info?: string | null; literal?: string }
  | { kind: 'blockQuote'; blocks?: MdBlocksDiff }
  | { kind: 'thematicBreak' }
  | { kind: 'htmlBlock'; raw?: string }
  | { kind: 'replace'; block: MdBlock };

/** 🌳 Index-keyed triple over a `list`'s `items: MdBlock[][]` -- each item's content is diffed
 * with the same recursive `MdBlocksDiff` used everywhere else. */
export interface MdListItemsDiff {
  removed: number[];
  modified: MdListItemModified[];
  added: MdListItemAdded[];
}

export interface MdListItemModified {
  index: number;
  diff: MdBlocksDiff;
}

export interface MdListItemAdded {
  index: number;
  item: MdBlock[];
}

/** 🧭️ One descent step from a block container down into a nested one -- mirrors the Rust
 * `MdPathStep` used by path-carrying mutations (`../🧬️mutations/🟦️.ts`). */
export type MdPathStep =
  | { step: 'blockQuote'; index: number }
  | { step: 'listItem'; index: number; item: number };

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioMdCommonmarkAnyDiffGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioMdCommonmarkAnyDiffGuardReject = (at: string, why: string): never => {
  throw new stdioMdCommonmarkAnyDiffGuardRefusal(at, why);
};

type stdioMdCommonmarkAnyDiffGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioMdCommonmarkAnyDiffGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioMdCommonmarkAnyDiffGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioMdCommonmarkAnyDiffGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioMdCommonmarkAnyDiffGuardReject(at, "value is not an object");
export const stdioMdCommonmarkAnyDiffGuardArray = (value: unknown, at: string, bounds: stdioMdCommonmarkAnyDiffGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioMdCommonmarkAnyDiffGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioMdCommonmarkAnyDiffGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioMdCommonmarkAnyDiffGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioMdCommonmarkAnyDiffGuardString = (value: unknown, at: string, bounds: stdioMdCommonmarkAnyDiffGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioMdCommonmarkAnyDiffGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioMdCommonmarkAnyDiffGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioMdCommonmarkAnyDiffGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioMdCommonmarkAnyDiffGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioMdCommonmarkAnyDiffGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioMdCommonmarkAnyDiffGuardReject(at, "value is not a boolean"));
export const stdioMdCommonmarkAnyDiffGuardNumber = (value: unknown, at: string, bounds: stdioMdCommonmarkAnyDiffGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioMdCommonmarkAnyDiffGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioMdCommonmarkAnyDiffGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioMdCommonmarkAnyDiffGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioMdCommonmarkAnyDiffGuardInteger = (value: unknown, at: string, bounds: stdioMdCommonmarkAnyDiffGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioMdCommonmarkAnyDiffGuardNumber(value, at, bounds) : stdioMdCommonmarkAnyDiffGuardReject(at, "value is not an integer");
export const stdioMdCommonmarkAnyDiffGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioMdCommonmarkAnyDiffGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioMdCommonmarkAnyDiffGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioMdCommonmarkAnyDiffGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseMdDiff(value: unknown, at = "$"): MdDiff {
  const row = stdioMdCommonmarkAnyDiffGuardObject(value, at);
  return {
    blocks: row["blocks"] === undefined ? undefined : parseMdBlocksDiff(row["blocks"], `${at}.blocks`),
  };
}

export function parseMdBlocksDiff(value: unknown, at = "$"): MdBlocksDiff {
  const row = stdioMdCommonmarkAnyDiffGuardObject(value, at);
  return {
    removed: stdioMdCommonmarkAnyDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => stdioMdCommonmarkAnyDiffGuardInteger(item, `${at}.removed[${index}]`, {"minimum": 0})),
    modified: stdioMdCommonmarkAnyDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseMdBlockModified(item, `${at}.modified[${index}]`)),
    added: stdioMdCommonmarkAnyDiffGuardArray(row["added"], `${at}.added`).map((item, index) => parseMdBlockAdded(item, `${at}.added[${index}]`)),
  };
}

export function parseMdBlockModified(value: unknown, at = "$"): MdBlockModified {
  const row = stdioMdCommonmarkAnyDiffGuardObject(value, at);
  return {
    index: stdioMdCommonmarkAnyDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    diff: parseMdBlockDiff(row["diff"], `${at}.diff`),
  };
}

export function parseMdBlockAdded(value: unknown, at = "$"): MdBlockAdded {
  const row = stdioMdCommonmarkAnyDiffGuardObject(value, at);
  return {
    index: stdioMdCommonmarkAnyDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    item: stdioMdCommonmarkAnyDiffGuardObject(row["item"], `${at}.item`),
  };
}

export function parseMdListItemsDiff(value: unknown, at = "$"): MdListItemsDiff {
  const row = stdioMdCommonmarkAnyDiffGuardObject(value, at);
  return {
    removed: stdioMdCommonmarkAnyDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => stdioMdCommonmarkAnyDiffGuardInteger(item, `${at}.removed[${index}]`, {"minimum": 0})),
    modified: stdioMdCommonmarkAnyDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseMdListItemModified(item, `${at}.modified[${index}]`)),
    added: stdioMdCommonmarkAnyDiffGuardArray(row["added"], `${at}.added`).map((item, index) => parseMdListItemAdded(item, `${at}.added[${index}]`)),
  };
}

export function parseMdListItemModified(value: unknown, at = "$"): MdListItemModified {
  const row = stdioMdCommonmarkAnyDiffGuardObject(value, at);
  return {
    index: stdioMdCommonmarkAnyDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    diff: parseMdBlocksDiff(row["diff"], `${at}.diff`),
  };
}

export function parseMdListItemAdded(value: unknown, at = "$"): MdListItemAdded {
  const row = stdioMdCommonmarkAnyDiffGuardObject(value, at);
  return {
    index: stdioMdCommonmarkAnyDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    item: stdioMdCommonmarkAnyDiffGuardArray(row["item"], `${at}.item`).map((item, index) => stdioMdCommonmarkAnyDiffGuardObject(item, `${at}.item[${index}]`)),
  };
}
