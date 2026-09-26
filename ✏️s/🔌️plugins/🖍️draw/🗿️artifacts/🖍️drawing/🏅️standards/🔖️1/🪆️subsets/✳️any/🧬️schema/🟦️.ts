/** 🧬️ Drawing artifact schema — every field with its state class. */

export interface DrawingArtifact {
  /** @state artifact */
  schema: string;
  /** @state artifact */
  id: string;
  /** @state artifact */
  title?: string;
  /** @state artifact */
  layers: DrawingLayerNode[];
  /** @state artifact */
  assets: Record<string, DrawingImageAsset>;
  /** @state artifact */
  artboard?: DrawingArtboard;
}

export interface DrawingLayerNode {
  kind: string;
  [key: string]: unknown;
}

export interface DrawingImageAsset {
  mime: string;
  data: string;
  width?: number;
  height?: number;
}

export interface DrawingArtboard {
  width: number;
  height: number;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class drawingDrawingArtifactGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const drawingDrawingArtifactGuardReject = (at: string, why: string): never => {
  throw new drawingDrawingArtifactGuardRefusal(at, why);
};

type drawingDrawingArtifactGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type drawingDrawingArtifactGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type drawingDrawingArtifactGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const drawingDrawingArtifactGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : drawingDrawingArtifactGuardReject(at, "value is not an object");
export const drawingDrawingArtifactGuardArray = (value: unknown, at: string, bounds: drawingDrawingArtifactGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return drawingDrawingArtifactGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) drawingDrawingArtifactGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) drawingDrawingArtifactGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const drawingDrawingArtifactGuardString = (value: unknown, at: string, bounds: drawingDrawingArtifactGuardTextBounds = {}): string => {
  if (typeof value !== "string") return drawingDrawingArtifactGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) drawingDrawingArtifactGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) drawingDrawingArtifactGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) drawingDrawingArtifactGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const drawingDrawingArtifactGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : drawingDrawingArtifactGuardReject(at, "value is not a boolean"));
export const drawingDrawingArtifactGuardNumber = (value: unknown, at: string, bounds: drawingDrawingArtifactGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return drawingDrawingArtifactGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) drawingDrawingArtifactGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) drawingDrawingArtifactGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const drawingDrawingArtifactGuardInteger = (value: unknown, at: string, bounds: drawingDrawingArtifactGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? drawingDrawingArtifactGuardNumber(value, at, bounds) : drawingDrawingArtifactGuardReject(at, "value is not an integer");
export const drawingDrawingArtifactGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : drawingDrawingArtifactGuardReject(at, `value is not one of ${members.join(", ")}`);
export const drawingDrawingArtifactGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : drawingDrawingArtifactGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseDrawingArtifact(value: unknown, at = "$"): DrawingArtifact {
  const row = drawingDrawingArtifactGuardObject(value, at);
  return {
    schema: drawingDrawingArtifactGuardString(row["schema"], `${at}.schema`),
    id: drawingDrawingArtifactGuardString(row["id"], `${at}.id`),
    title: row["title"] === undefined ? undefined : drawingDrawingArtifactGuardString(row["title"], `${at}.title`),
    layers: drawingDrawingArtifactGuardArray(row["layers"], `${at}.layers`).map((item, index) => parseDrawingLayerNode(item, `${at}.layers[${index}]`)),
    assets: Object.fromEntries(
      Object.entries(drawingDrawingArtifactGuardObject(row["assets"], `${at}.assets`))
        .map(([key, item]) => [key, parseDrawingImageAsset(item, `${at}.assets.${key}`)]),
    ),
    artboard: row["artboard"] === undefined ? undefined : parseDrawingArtboard(row["artboard"], `${at}.artboard`),
  };
}

export function parseDrawingLayerNode(value: unknown, at = "$"): DrawingLayerNode {
  const row = drawingDrawingArtifactGuardObject(value, at);
  return {
    ...structuredClone(row),
    kind: drawingDrawingArtifactGuardString(row["kind"], `${at}.kind`),
  };
}

export function parseDrawingImageAsset(value: unknown, at = "$"): DrawingImageAsset {
  const row = drawingDrawingArtifactGuardObject(value, at);
  return {
    mime: drawingDrawingArtifactGuardString(row["mime"], `${at}.mime`),
    data: drawingDrawingArtifactGuardString(row["data"], `${at}.data`),
    width: row["width"] === undefined ? undefined : drawingDrawingArtifactGuardInteger(row["width"], `${at}.width`, {"minimum": 0}),
    height: row["height"] === undefined ? undefined : drawingDrawingArtifactGuardInteger(row["height"], `${at}.height`, {"minimum": 0}),
  };
}

export function parseDrawingArtboard(value: unknown, at = "$"): DrawingArtboard {
  const row = drawingDrawingArtifactGuardObject(value, at);
  return {
    width: drawingDrawingArtifactGuardNumber(row["width"], `${at}.width`),
    height: drawingDrawingArtifactGuardNumber(row["height"], `${at}.height`),
  };
}

/** ✏️ Owned path geometry vocabulary shared by editing and persistence. */
export type PathSegment =
  | { kind: "move"; to: [number,number] }
  | { kind: "line"; to: [number,number] }
  | { kind: "quad"; ctrl: [number,number]; to: [number,number] }
  | { kind: "cubic"; ctrl1: [number,number]; ctrl2: [number,number]; to: [number,number] }
  | { kind: "arc"; rx: number; ry: number; rotation: number; largeArc: boolean; sweep: boolean; to: [number,number] }
  | { kind: "close" };

/** 📍 Validates owned geometry without accepting malformed or unknown segment fields. */
export function parsePathSegment(value: unknown, at = "$"): PathSegment {
  const row = drawingDrawingArtifactGuardObject(value, at);
  const kind = drawingDrawingArtifactGuardMember(row.kind, `${at}.kind`, ["move", "line", "quad", "cubic", "arc", "close"] as const);
  const fields = { move: ["to"], line: ["to"], quad: ["ctrl", "to"], cubic: ["ctrl1", "ctrl2", "to"], arc: ["rx", "ry", "rotation", "largeArc", "sweep", "to"], close: [] }[kind];
  for (const key of Object.keys(row)) if (key !== "kind" && !fields.includes(key)) drawingDrawingArtifactGuardReject(`${at}.${key}`, "unknown segment field");
  const point = (key: string): [number, number] => {
    const values = drawingDrawingArtifactGuardArray(row[key], `${at}.${key}`, { minItems: 2, maxItems: 2 });
    return [drawingDrawingArtifactGuardNumber(values[0], `${at}.${key}[0]`), drawingDrawingArtifactGuardNumber(values[1], `${at}.${key}[1]`)];
  };
  switch (kind) {
    case "move": case "line": return { kind, to: point("to") };
    case "quad": return { kind, ctrl: point("ctrl"), to: point("to") };
    case "cubic": return { kind, ctrl1: point("ctrl1"), ctrl2: point("ctrl2"), to: point("to") };
    case "arc": return { kind, rx: drawingDrawingArtifactGuardNumber(row.rx, `${at}.rx`, { minimum: 0 }), ry: drawingDrawingArtifactGuardNumber(row.ry, `${at}.ry`, { minimum: 0 }), rotation: drawingDrawingArtifactGuardNumber(row.rotation, `${at}.rotation`), largeArc: drawingDrawingArtifactGuardBoolean(row.largeArc, `${at}.largeArc`), sweep: drawingDrawingArtifactGuardBoolean(row.sweep, `${at}.sweep`), to: point("to") };
    case "close": return { kind };
  }
}
