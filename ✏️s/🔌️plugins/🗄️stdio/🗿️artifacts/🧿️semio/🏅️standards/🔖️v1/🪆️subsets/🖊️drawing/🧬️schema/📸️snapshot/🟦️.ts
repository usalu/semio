/** 🧬️ SemioDrawingSnapshot — mirrors the real Rust `📸️snapshot/🦀️.rs` (source of truth).
 * `DrawNode` is the recursive scene-graph node (`Path`/`Text`/`Group`/`Image`), matching svg's
 * `SvgNodeDiff` recursive-diff template per the master plan. The `ArtifactDsl`/`ArtifactPack`
 * codec (see the Rust sibling) hex-encodes the JSON `body`, honoring `options` where the pack
 * encoder accepts them, and wraps `text` in the `semio_format` envelope. */

export interface SemioPoint2 { x: number; y: number; }
export interface Rgba { r: number; g: number; b: number; a: number; }
export interface Transform {
  translation: { x: number; y: number; z: number };
  rotation: { x: number; y: number; z: number; w: number };
  scale: { x: number; y: number; z: number };
}

export type PathSegment =
  | { kind: "moveTo"; to: SemioPoint2 }
  | { kind: "lineTo"; to: SemioPoint2 }
  | { kind: "cubicTo"; c1: SemioPoint2; c2: SemioPoint2; to: SemioPoint2 }
  | { kind: "quadTo"; c: SemioPoint2; to: SemioPoint2 }
  | { kind: "arcTo"; rx: number; ry: number; xRotation: number; largeArc: boolean; sweep: boolean; to: SemioPoint2 }
  | { kind: "close" };

export type DrawNode =
  | { kind: "path"; segments: PathSegment[]; style?: string }
  | { kind: "text"; value: string; at: SemioPoint2; style?: string }
  | { kind: "group-nodes"; transform: Transform; children: DrawNode[] }
  | { kind: "image"; at: SemioPoint2; width: number; height: number; mime: string; bytes: Uint8Array };

export interface DrawStyle {
  name: string;
  fill?: Rgba;
  stroke?: Rgba;
  strokeWidth?: number;
  opacity?: number;
}

export interface DrawLayer {
  id: string;
  name: string;
  visible: boolean;
  root: DrawNode;
}

export interface DrawCanvas {
  width: number;
  height: number;
  background?: Rgba;
}

export interface SemioDrawingSnapshot {
  /** @state artifact */ schema: string;
  /** @state artifact */ canvas: DrawCanvas;
  /** @state artifact */ styles: DrawStyle[];
  /** @state artifact */ layers: DrawLayer[];
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioSemioV1DrawingSnapshotGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioSemioV1DrawingSnapshotGuardReject = (at: string, why: string): never => {
  throw new stdioSemioV1DrawingSnapshotGuardRefusal(at, why);
};

type stdioSemioV1DrawingSnapshotGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioSemioV1DrawingSnapshotGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioSemioV1DrawingSnapshotGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioSemioV1DrawingSnapshotGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioSemioV1DrawingSnapshotGuardReject(at, "value is not an object");
export const stdioSemioV1DrawingSnapshotGuardArray = (value: unknown, at: string, bounds: stdioSemioV1DrawingSnapshotGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioSemioV1DrawingSnapshotGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioSemioV1DrawingSnapshotGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioSemioV1DrawingSnapshotGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioSemioV1DrawingSnapshotGuardString = (value: unknown, at: string, bounds: stdioSemioV1DrawingSnapshotGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioSemioV1DrawingSnapshotGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioSemioV1DrawingSnapshotGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioSemioV1DrawingSnapshotGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioSemioV1DrawingSnapshotGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioSemioV1DrawingSnapshotGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioSemioV1DrawingSnapshotGuardReject(at, "value is not a boolean"));
export const stdioSemioV1DrawingSnapshotGuardNumber = (value: unknown, at: string, bounds: stdioSemioV1DrawingSnapshotGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioSemioV1DrawingSnapshotGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioSemioV1DrawingSnapshotGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioSemioV1DrawingSnapshotGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioSemioV1DrawingSnapshotGuardInteger = (value: unknown, at: string, bounds: stdioSemioV1DrawingSnapshotGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioSemioV1DrawingSnapshotGuardNumber(value, at, bounds) : stdioSemioV1DrawingSnapshotGuardReject(at, "value is not an integer");
export const stdioSemioV1DrawingSnapshotGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioSemioV1DrawingSnapshotGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioSemioV1DrawingSnapshotGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioSemioV1DrawingSnapshotGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseSemioDrawingSnapshot(value: unknown, at = "$"): SemioDrawingSnapshot {
  const row = stdioSemioV1DrawingSnapshotGuardObject(value, at);
  return {
    schema: stdioSemioV1DrawingSnapshotGuardString(row["schema"], `${at}.schema`),
    canvas: parseDrawCanvas(row["canvas"], `${at}.canvas`),
    styles: stdioSemioV1DrawingSnapshotGuardArray(row["styles"], `${at}.styles`).map((item, index) => parseDrawStyle(item, `${at}.styles[${index}]`)),
    layers: stdioSemioV1DrawingSnapshotGuardArray(row["layers"], `${at}.layers`).map((item, index) => parseDrawLayer(item, `${at}.layers[${index}]`)),
  };
}

export function parseSemioPoint2(value: unknown, at = "$"): SemioPoint2 {
  const row = stdioSemioV1DrawingSnapshotGuardObject(value, at);
  return {
    x: stdioSemioV1DrawingSnapshotGuardNumber(row["x"], `${at}.x`),
    y: stdioSemioV1DrawingSnapshotGuardNumber(row["y"], `${at}.y`),
  };
}

export function parseRgba(value: unknown, at = "$"): Rgba {
  const row = stdioSemioV1DrawingSnapshotGuardObject(value, at);
  return {
    r: stdioSemioV1DrawingSnapshotGuardNumber(row["r"], `${at}.r`),
    g: stdioSemioV1DrawingSnapshotGuardNumber(row["g"], `${at}.g`),
    b: stdioSemioV1DrawingSnapshotGuardNumber(row["b"], `${at}.b`),
    a: stdioSemioV1DrawingSnapshotGuardNumber(row["a"], `${at}.a`),
  };
}

export interface Point3 {
  readonly x: number;
  readonly y: number;
  readonly z: number;
}

export function parsePoint3(value: unknown, at = "$"): Point3 {
  const row = stdioSemioV1DrawingSnapshotGuardObject(value, at);
  return {
    x: stdioSemioV1DrawingSnapshotGuardNumber(row["x"], `${at}.x`),
    y: stdioSemioV1DrawingSnapshotGuardNumber(row["y"], `${at}.y`),
    z: stdioSemioV1DrawingSnapshotGuardNumber(row["z"], `${at}.z`),
  };
}

export interface Quaternion {
  readonly x: number;
  readonly y: number;
  readonly z: number;
  readonly w: number;
}

export function parseQuaternion(value: unknown, at = "$"): Quaternion {
  const row = stdioSemioV1DrawingSnapshotGuardObject(value, at);
  return {
    x: stdioSemioV1DrawingSnapshotGuardNumber(row["x"], `${at}.x`),
    y: stdioSemioV1DrawingSnapshotGuardNumber(row["y"], `${at}.y`),
    z: stdioSemioV1DrawingSnapshotGuardNumber(row["z"], `${at}.z`),
    w: stdioSemioV1DrawingSnapshotGuardNumber(row["w"], `${at}.w`),
  };
}

export function parseTransform(value: unknown, at = "$"): Transform {
  const row = stdioSemioV1DrawingSnapshotGuardObject(value, at);
  return {
    translation: parsePoint3(row["translation"], `${at}.translation`),
    rotation: parseQuaternion(row["rotation"], `${at}.rotation`),
    scale: parsePoint3(row["scale"], `${at}.scale`),
  };
}

export function parsePathSegment(value: unknown, at = "$"): PathSegment {
  const row = stdioSemioV1DrawingSnapshotGuardObject(value, at);
  return {
    kind: stdioSemioV1DrawingSnapshotGuardMember(row["kind"], `${at}.kind`, ["moveTo", "lineTo", "cubicTo", "quadTo", "arcTo", "close"] as const),
    to: row["to"] === undefined ? undefined : parseSemioPoint2(row["to"], `${at}.to`),
    c1: row["c1"] === undefined ? undefined : parseSemioPoint2(row["c1"], `${at}.c1`),
    c2: row["c2"] === undefined ? undefined : parseSemioPoint2(row["c2"], `${at}.c2`),
    c: row["c"] === undefined ? undefined : parseSemioPoint2(row["c"], `${at}.c`),
    rx: row["rx"] === undefined ? undefined : stdioSemioV1DrawingSnapshotGuardNumber(row["rx"], `${at}.rx`),
    ry: row["ry"] === undefined ? undefined : stdioSemioV1DrawingSnapshotGuardNumber(row["ry"], `${at}.ry`),
    xRotation: row["xRotation"] === undefined ? undefined : stdioSemioV1DrawingSnapshotGuardNumber(row["xRotation"], `${at}.xRotation`),
    largeArc: row["largeArc"] === undefined ? undefined : stdioSemioV1DrawingSnapshotGuardBoolean(row["largeArc"], `${at}.largeArc`),
    sweep: row["sweep"] === undefined ? undefined : stdioSemioV1DrawingSnapshotGuardBoolean(row["sweep"], `${at}.sweep`),
  };
}

export function parseDrawNode(value: unknown, at = "$"): DrawNode {
  const row = stdioSemioV1DrawingSnapshotGuardObject(value, at);
  return {
    kind: stdioSemioV1DrawingSnapshotGuardMember(row["kind"], `${at}.kind`, ["path", "text", "group", "image"] as const),
    segments: row["segments"] === undefined ? undefined : stdioSemioV1DrawingSnapshotGuardArray(row["segments"], `${at}.segments`).map((item, index) => parsePathSegment(item, `${at}.segments[${index}]`)),
    style: row["style"] === undefined ? undefined : stdioSemioV1DrawingSnapshotGuardString(row["style"], `${at}.style`),
    value: row["value"] === undefined ? undefined : stdioSemioV1DrawingSnapshotGuardString(row["value"], `${at}.value`),
    at: row["at"] === undefined ? undefined : parseSemioPoint2(row["at"], `${at}.at`),
    transform: row["transform"] === undefined ? undefined : parseTransform(row["transform"], `${at}.transform`),
    children: row["children"] === undefined ? undefined : stdioSemioV1DrawingSnapshotGuardArray(row["children"], `${at}.children`).map((item, index) => parseDrawNode(item, `${at}.children[${index}]`)),
    width: row["width"] === undefined ? undefined : stdioSemioV1DrawingSnapshotGuardNumber(row["width"], `${at}.width`),
    height: row["height"] === undefined ? undefined : stdioSemioV1DrawingSnapshotGuardNumber(row["height"], `${at}.height`),
    mime: row["mime"] === undefined ? undefined : stdioSemioV1DrawingSnapshotGuardString(row["mime"], `${at}.mime`),
    bytes: row["bytes"] === undefined ? undefined : stdioSemioV1DrawingSnapshotGuardString(row["bytes"], `${at}.bytes`),
  };
}

export function parseDrawStyle(value: unknown, at = "$"): DrawStyle {
  const row = stdioSemioV1DrawingSnapshotGuardObject(value, at);
  return {
    name: stdioSemioV1DrawingSnapshotGuardString(row["name"], `${at}.name`),
    fill: row["fill"] === undefined ? undefined : parseRgba(row["fill"], `${at}.fill`),
    stroke: row["stroke"] === undefined ? undefined : parseRgba(row["stroke"], `${at}.stroke`),
    strokeWidth: row["strokeWidth"] === undefined ? undefined : stdioSemioV1DrawingSnapshotGuardNumber(row["strokeWidth"], `${at}.strokeWidth`),
    opacity: row["opacity"] === undefined ? undefined : stdioSemioV1DrawingSnapshotGuardNumber(row["opacity"], `${at}.opacity`),
  };
}

export function parseDrawLayer(value: unknown, at = "$"): DrawLayer {
  const row = stdioSemioV1DrawingSnapshotGuardObject(value, at);
  return {
    id: stdioSemioV1DrawingSnapshotGuardString(row["id"], `${at}.id`),
    name: stdioSemioV1DrawingSnapshotGuardString(row["name"], `${at}.name`),
    visible: stdioSemioV1DrawingSnapshotGuardBoolean(row["visible"], `${at}.visible`),
    root: parseDrawNode(row["root"], `${at}.root`),
  };
}

export function parseDrawCanvas(value: unknown, at = "$"): DrawCanvas {
  const row = stdioSemioV1DrawingSnapshotGuardObject(value, at);
  return {
    width: stdioSemioV1DrawingSnapshotGuardNumber(row["width"], `${at}.width`),
    height: stdioSemioV1DrawingSnapshotGuardNumber(row["height"], `${at}.height`),
    background: row["background"] === undefined ? undefined : parseRgba(row["background"], `${at}.background`),
  };
}
