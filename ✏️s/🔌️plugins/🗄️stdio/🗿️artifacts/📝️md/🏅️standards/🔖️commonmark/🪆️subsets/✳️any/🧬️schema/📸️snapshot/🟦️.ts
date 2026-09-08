/** 🧩 A real CommonMark inline node. Weak entity (recipe): whole-value replaced in diffs. */
export type MdInline =
  | { kind: 'text'; text: string }
  | { kind: 'emphasis'; inlines: MdInline[] }
  | { kind: 'strong'; inlines: MdInline[] }
  | { kind: 'code'; literal: string }
  | { kind: 'link'; text: MdInline[]; url: string; title?: string }
  | { kind: 'image'; alt: string; url: string; title?: string }
  | { kind: 'softBreak' }
  | { kind: 'hardBreak' }
  | { kind: 'htmlInline'; raw: string };

/** 🧱 A real CommonMark block. Strong-like entity: block collections are index-keyed and
 * per-field diffed (see `../🔺️diff/🟦️.ts`). */
export type MdBlock =
  | { kind: 'heading'; level: number; inlines: MdInline[] }
  | { kind: 'paragraph'; inlines: MdInline[] }
  | { kind: 'list'; ordered: boolean; start?: number; tight: boolean; items: MdBlock[][] }
  | { kind: 'codeBlock'; info?: string; literal: string }
  | { kind: 'blockQuote'; blocks: MdBlock[] }
  | { kind: 'thematicBreak' }
  | { kind: 'htmlBlock'; raw: string };

/** 📸️ Persisted `stdio.md` snapshot: the complete top-level block sequence. */
export interface MdSnapshot {
  schema: string;
  blocks: MdBlock[];
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioMdCommonmarkAnySnapshotGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioMdCommonmarkAnySnapshotGuardReject = (at: string, why: string): never => {
  throw new stdioMdCommonmarkAnySnapshotGuardRefusal(at, why);
};

type stdioMdCommonmarkAnySnapshotGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioMdCommonmarkAnySnapshotGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioMdCommonmarkAnySnapshotGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioMdCommonmarkAnySnapshotGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioMdCommonmarkAnySnapshotGuardReject(at, "value is not an object");
export const stdioMdCommonmarkAnySnapshotGuardArray = (value: unknown, at: string, bounds: stdioMdCommonmarkAnySnapshotGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioMdCommonmarkAnySnapshotGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioMdCommonmarkAnySnapshotGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioMdCommonmarkAnySnapshotGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioMdCommonmarkAnySnapshotGuardString = (value: unknown, at: string, bounds: stdioMdCommonmarkAnySnapshotGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioMdCommonmarkAnySnapshotGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioMdCommonmarkAnySnapshotGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioMdCommonmarkAnySnapshotGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioMdCommonmarkAnySnapshotGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioMdCommonmarkAnySnapshotGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioMdCommonmarkAnySnapshotGuardReject(at, "value is not a boolean"));
export const stdioMdCommonmarkAnySnapshotGuardNumber = (value: unknown, at: string, bounds: stdioMdCommonmarkAnySnapshotGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioMdCommonmarkAnySnapshotGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioMdCommonmarkAnySnapshotGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioMdCommonmarkAnySnapshotGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioMdCommonmarkAnySnapshotGuardInteger = (value: unknown, at: string, bounds: stdioMdCommonmarkAnySnapshotGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioMdCommonmarkAnySnapshotGuardNumber(value, at, bounds) : stdioMdCommonmarkAnySnapshotGuardReject(at, "value is not an integer");
export const stdioMdCommonmarkAnySnapshotGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioMdCommonmarkAnySnapshotGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioMdCommonmarkAnySnapshotGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioMdCommonmarkAnySnapshotGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseMdSnapshot(value: unknown, at = "$"): MdSnapshot {
  const row = stdioMdCommonmarkAnySnapshotGuardObject(value, at);
  return {
    schema: stdioMdCommonmarkAnySnapshotGuardString(row["schema"], `${at}.schema`),
    blocks: stdioMdCommonmarkAnySnapshotGuardArray(row["blocks"], `${at}.blocks`).map((item, index) => parseMdBlock(item, `${at}.blocks[${index}]`)),
  };
}
