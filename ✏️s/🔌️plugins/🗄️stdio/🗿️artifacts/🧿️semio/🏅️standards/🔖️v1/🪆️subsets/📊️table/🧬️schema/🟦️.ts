/** 🧬️ SemioTableArtifact schema — real facet mirror of the Rust `🦀️.rs` sibling. */
export type SemioTableCellKind = "null" | "bool" | "int" | "float" | "str" | "bytes";
export interface SemioTableColumn {
  name: string;
  kind: SemioTableCellKind;
}
export type SemioValue =
  | { kind: "null" }
  | { kind: "bool"; value: boolean }
  | { kind: "int"; value: string }
  | { kind: "float"; lexeme: string }
  | { kind: "str"; value: string }
  | { kind: "bytes"; value: string };
export interface SemioTableRow {
  cells: SemioValue[];
}
export interface SemioTableArtifact {
  /** @state artifact */ schema: string;
  /** @state artifact */ columns: SemioTableColumn[];
  /** @state artifact */ rows: SemioTableRow[];
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioSemioV1TableArtifactGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioSemioV1TableArtifactGuardReject = (at: string, why: string): never => {
  throw new stdioSemioV1TableArtifactGuardRefusal(at, why);
};

type stdioSemioV1TableArtifactGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioSemioV1TableArtifactGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioSemioV1TableArtifactGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioSemioV1TableArtifactGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioSemioV1TableArtifactGuardReject(at, "value is not an object");
export const stdioSemioV1TableArtifactGuardArray = (value: unknown, at: string, bounds: stdioSemioV1TableArtifactGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioSemioV1TableArtifactGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioSemioV1TableArtifactGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioSemioV1TableArtifactGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioSemioV1TableArtifactGuardString = (value: unknown, at: string, bounds: stdioSemioV1TableArtifactGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioSemioV1TableArtifactGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioSemioV1TableArtifactGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioSemioV1TableArtifactGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioSemioV1TableArtifactGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioSemioV1TableArtifactGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioSemioV1TableArtifactGuardReject(at, "value is not a boolean"));
export const stdioSemioV1TableArtifactGuardNumber = (value: unknown, at: string, bounds: stdioSemioV1TableArtifactGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioSemioV1TableArtifactGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioSemioV1TableArtifactGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioSemioV1TableArtifactGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioSemioV1TableArtifactGuardInteger = (value: unknown, at: string, bounds: stdioSemioV1TableArtifactGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioSemioV1TableArtifactGuardNumber(value, at, bounds) : stdioSemioV1TableArtifactGuardReject(at, "value is not an integer");
export const stdioSemioV1TableArtifactGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioSemioV1TableArtifactGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioSemioV1TableArtifactGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioSemioV1TableArtifactGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseSemioTableArtifact(value: unknown, at = "$"): SemioTableArtifact {
  const row = stdioSemioV1TableArtifactGuardObject(value, at);
  return {
    schema: stdioSemioV1TableArtifactGuardString(row["schema"], `${at}.schema`),
    columns: stdioSemioV1TableArtifactGuardArray(row["columns"], `${at}.columns`).map((item, index) => stdioSemioV1TableArtifactGuardObject(item, `${at}.columns[${index}]`)),
    rows: stdioSemioV1TableArtifactGuardArray(row["rows"], `${at}.rows`).map((item, index) => stdioSemioV1TableArtifactGuardObject(item, `${at}.rows[${index}]`)),
  };
}
