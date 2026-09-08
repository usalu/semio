/** 📝️ Text representation for `stdio.html` (snapshot) -- the WHATWG HTML5 well-formed subset
 * body `parse_html_document`/`write_html_document` operate on. See the sibling
 * `📖️.grammar.semio` for the real grammar. */
export type HtmlSnapshotText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioHtml5AnySnapshotTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioHtml5AnySnapshotTextGuardReject = (at: string, why: string): never => {
  throw new stdioHtml5AnySnapshotTextGuardRefusal(at, why);
};

type stdioHtml5AnySnapshotTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioHtml5AnySnapshotTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioHtml5AnySnapshotTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioHtml5AnySnapshotTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioHtml5AnySnapshotTextGuardReject(at, "value is not an object");
export const stdioHtml5AnySnapshotTextGuardArray = (value: unknown, at: string, bounds: stdioHtml5AnySnapshotTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioHtml5AnySnapshotTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioHtml5AnySnapshotTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioHtml5AnySnapshotTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioHtml5AnySnapshotTextGuardString = (value: unknown, at: string, bounds: stdioHtml5AnySnapshotTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioHtml5AnySnapshotTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioHtml5AnySnapshotTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioHtml5AnySnapshotTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioHtml5AnySnapshotTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioHtml5AnySnapshotTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioHtml5AnySnapshotTextGuardReject(at, "value is not a boolean"));
export const stdioHtml5AnySnapshotTextGuardNumber = (value: unknown, at: string, bounds: stdioHtml5AnySnapshotTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioHtml5AnySnapshotTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioHtml5AnySnapshotTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioHtml5AnySnapshotTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioHtml5AnySnapshotTextGuardInteger = (value: unknown, at: string, bounds: stdioHtml5AnySnapshotTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioHtml5AnySnapshotTextGuardNumber(value, at, bounds) : stdioHtml5AnySnapshotTextGuardReject(at, "value is not an integer");
export const stdioHtml5AnySnapshotTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioHtml5AnySnapshotTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioHtml5AnySnapshotTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioHtml5AnySnapshotTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseHtmlSnapshotText(value: unknown, at = "$"): HtmlSnapshotText {
  return stdioHtml5AnySnapshotTextGuardObject(value, `${at}`);
}
