/** 🔺️ SemioDrawingDiff — mirrors the real Rust 🔺️diff/🦀️.rs (handcrafted sparse diff,
 * source of truth). Collection triples reuse the shared engine's own TS shape (see
 * `⚙️engine/🧰️triples/🦀️.rs`'s facet mirrors) — `removed`/`modified`/`added`.
 * `between(base, other)` computes the `schema` delta against the base snapshot from scratch (no
 * snapshot-replace slot); `Transform` carries `translation`/`rotation`/`scale`; the hand-rolled
 * `DiffCodec`'s `line` is built from space-separated `tokens`; `NodePath{layer, path}` addresses
 * a node. */
import type { DrawLayer, DrawNode, DrawStyle, PathSegment, Rgba, SemioPoint2, Transform } from "../📸️snapshot/🟦️";

export interface IndexedTripleDiff<D, T> {
  removed: number[];
  modified: { index: number; diff: D }[];
  added: { index: number; item: T }[];
}
export interface NamedTripleDiff<K, D, T> {
  removed: K[];
  modified: { key: K; diff: D }[];
  added: T[];
}

export interface DrawCanvasDiff {
  width?: number;
  height?: number;
  /** tri-state: absent = unchanged, null = cleared, value = set */
  background?: Rgba | null;
}

export interface DrawStyleDiff {
  fill?: Rgba | null;
  stroke?: Rgba | null;
  strokeWidth?: number | null;
  opacity?: number | null;
}

export type DrawNodeDiff =
  | { kind: "path"; segments?: PathSegment[]; style?: string | null }
  | { kind: "text"; value?: string; at?: SemioPoint2; style?: string | null }
  | { kind: "group-nodes"; transform?: Transform; children?: IndexedTripleDiff<DrawNodeDiff, DrawNode> }
  | { kind: "image"; at?: SemioPoint2; width?: number; height?: number; mime?: string; bytes?: Uint8Array }
  | { kind: "replace"; node: DrawNode };

export interface DrawLayerDiff {
  id?: string;
  name?: string;
  visible?: boolean;
  root?: DrawNodeDiff;
}

export interface SemioDrawingDiff {
  canvas?: DrawCanvasDiff;
  styles?: NamedTripleDiff<string, DrawStyleDiff, DrawStyle>;
  layers?: IndexedTripleDiff<DrawLayerDiff, DrawLayer>;
}

// 🧭️ Mutation-level node addressing (`NodePath`) lives in ../🧬️mutations/🟦️.ts.

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioSemioV1DrawingDiffGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioSemioV1DrawingDiffGuardReject = (at: string, why: string): never => {
  throw new stdioSemioV1DrawingDiffGuardRefusal(at, why);
};

type stdioSemioV1DrawingDiffGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioSemioV1DrawingDiffGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioSemioV1DrawingDiffGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioSemioV1DrawingDiffGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioSemioV1DrawingDiffGuardReject(at, "value is not an object");
export const stdioSemioV1DrawingDiffGuardArray = (value: unknown, at: string, bounds: stdioSemioV1DrawingDiffGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioSemioV1DrawingDiffGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioSemioV1DrawingDiffGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioSemioV1DrawingDiffGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioSemioV1DrawingDiffGuardString = (value: unknown, at: string, bounds: stdioSemioV1DrawingDiffGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioSemioV1DrawingDiffGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioSemioV1DrawingDiffGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioSemioV1DrawingDiffGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioSemioV1DrawingDiffGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioSemioV1DrawingDiffGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioSemioV1DrawingDiffGuardReject(at, "value is not a boolean"));
export const stdioSemioV1DrawingDiffGuardNumber = (value: unknown, at: string, bounds: stdioSemioV1DrawingDiffGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioSemioV1DrawingDiffGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioSemioV1DrawingDiffGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioSemioV1DrawingDiffGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioSemioV1DrawingDiffGuardInteger = (value: unknown, at: string, bounds: stdioSemioV1DrawingDiffGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioSemioV1DrawingDiffGuardNumber(value, at, bounds) : stdioSemioV1DrawingDiffGuardReject(at, "value is not an integer");
export const stdioSemioV1DrawingDiffGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioSemioV1DrawingDiffGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioSemioV1DrawingDiffGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioSemioV1DrawingDiffGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseSemioDrawingDiff(value: unknown, at = "$"): SemioDrawingDiff {
  const row = stdioSemioV1DrawingDiffGuardObject(value, at);
  return {
    canvas: row["canvas"] === undefined ? undefined : parseDrawCanvasDiff(row["canvas"], `${at}.canvas`),
    styles: row["styles"] === undefined ? undefined : parseNamedTripleDiff(row["styles"], `${at}.styles`),
    layers: row["layers"] === undefined ? undefined : parseIndexedTripleDiff(row["layers"], `${at}.layers`),
  };
}

export function parseDrawLayerDiff(value: unknown, at = "$"): DrawLayerDiff {
  const row = stdioSemioV1DrawingDiffGuardObject(value, at);
  return {
    id: row["id"] === undefined ? undefined : stdioSemioV1DrawingDiffGuardString(row["id"], `${at}.id`),
    name: row["name"] === undefined ? undefined : stdioSemioV1DrawingDiffGuardString(row["name"], `${at}.name`),
    visible: row["visible"] === undefined ? undefined : stdioSemioV1DrawingDiffGuardBoolean(row["visible"], `${at}.visible`),
    root: row["root"] === undefined ? undefined : parseDrawNodeDiff(row["root"], `${at}.root`),
  };
}
