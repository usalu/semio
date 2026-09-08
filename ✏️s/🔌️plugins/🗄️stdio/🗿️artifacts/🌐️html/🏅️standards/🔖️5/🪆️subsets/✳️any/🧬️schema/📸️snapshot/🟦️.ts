/** 🧬️ HtmlSnapshot schema — own HtmlNode recursive tree model (own types, HTML is not XML). */
export interface HtmlAttr {
  name: string;
  /** `undefined` = valueless boolean attribute (e.g. `disabled`). */
  value?: string;
}

export type RawTextKind = 'script' | 'style';

export type HtmlNode =
  | { kind: 'element'; name: string; attributes: HtmlAttr[]; children: HtmlNode[] }
  | { kind: 'text'; text: string }
  | { kind: 'comment'; text: string }
  | { kind: 'rawText'; parentKind: RawTextKind; text: string };

export interface HtmlSnapshot {
  /** @state artifact */ schema: string;
  /** @state artifact */ doctype?: string;
  /** @state artifact */ root: HtmlNode;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioHtml5AnySnapshotGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioHtml5AnySnapshotGuardReject = (at: string, why: string): never => {
  throw new stdioHtml5AnySnapshotGuardRefusal(at, why);
};

type stdioHtml5AnySnapshotGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioHtml5AnySnapshotGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioHtml5AnySnapshotGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioHtml5AnySnapshotGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioHtml5AnySnapshotGuardReject(at, "value is not an object");
export const stdioHtml5AnySnapshotGuardArray = (value: unknown, at: string, bounds: stdioHtml5AnySnapshotGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioHtml5AnySnapshotGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioHtml5AnySnapshotGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioHtml5AnySnapshotGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioHtml5AnySnapshotGuardString = (value: unknown, at: string, bounds: stdioHtml5AnySnapshotGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioHtml5AnySnapshotGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioHtml5AnySnapshotGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioHtml5AnySnapshotGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioHtml5AnySnapshotGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioHtml5AnySnapshotGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioHtml5AnySnapshotGuardReject(at, "value is not a boolean"));
export const stdioHtml5AnySnapshotGuardNumber = (value: unknown, at: string, bounds: stdioHtml5AnySnapshotGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioHtml5AnySnapshotGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioHtml5AnySnapshotGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioHtml5AnySnapshotGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioHtml5AnySnapshotGuardInteger = (value: unknown, at: string, bounds: stdioHtml5AnySnapshotGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioHtml5AnySnapshotGuardNumber(value, at, bounds) : stdioHtml5AnySnapshotGuardReject(at, "value is not an integer");
export const stdioHtml5AnySnapshotGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioHtml5AnySnapshotGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioHtml5AnySnapshotGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioHtml5AnySnapshotGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseHtmlSnapshot(value: unknown, at = "$"): HtmlSnapshot {
  const row = stdioHtml5AnySnapshotGuardObject(value, at);
  return {
    schema: stdioHtml5AnySnapshotGuardString(row["schema"], `${at}.schema`),
    doctype: row["doctype"] === undefined ? undefined : stdioHtml5AnySnapshotGuardString(row["doctype"], `${at}.doctype`),
    root: parseHtmlNode(row["root"], `${at}.root`),
  };
}

export function parseHtmlAttr(value: unknown, at = "$"): HtmlAttr {
  const row = stdioHtml5AnySnapshotGuardObject(value, at);
  return {
    name: stdioHtml5AnySnapshotGuardString(row["name"], `${at}.name`),
    value: row["value"] === undefined ? undefined : stdioHtml5AnySnapshotGuardString(row["value"], `${at}.value`),
  };
}

export function parseRawTextKind(value: unknown, at = "$"): RawTextKind {
  return stdioHtml5AnySnapshotGuardMember(value, `${at}`, ["script", "style"] as const);
}
