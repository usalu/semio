/** 📸️ Mirrors Rust `DrawingSnapshot` (persisted drawing document snapshot — artifact-lane fields only;
 * sibling `🦀️.rs`, `#[serde(rename_all = "camelCase")]`). Nested types re-import the
 * artifact's own root schema (`../🟦️.ts`) rather than re-declaring stubs, so every facet
 * of the drawing artifact agrees on the same `DrawingLayerNode`/`DrawingImageAsset`/`DrawingArtboard`. */
import {
  parseDrawingArtboard,
  parseDrawingImageAsset,
  parseDrawingLayerNode,
  type DrawingArtboard,
  type DrawingImageAsset,
  type DrawingLayerNode,
} from "../🟦️.ts";

export interface DrawingSnapshot {
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

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class drawingDrawingSnapshotGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const drawingDrawingSnapshotGuardReject = (at: string, why: string): never => {
  throw new drawingDrawingSnapshotGuardRefusal(at, why);
};

type drawingDrawingSnapshotGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type drawingDrawingSnapshotGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type drawingDrawingSnapshotGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const drawingDrawingSnapshotGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : drawingDrawingSnapshotGuardReject(at, "value is not an object");
export const drawingDrawingSnapshotGuardArray = (value: unknown, at: string, bounds: drawingDrawingSnapshotGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return drawingDrawingSnapshotGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) drawingDrawingSnapshotGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) drawingDrawingSnapshotGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const drawingDrawingSnapshotGuardString = (value: unknown, at: string, bounds: drawingDrawingSnapshotGuardTextBounds = {}): string => {
  if (typeof value !== "string") return drawingDrawingSnapshotGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) drawingDrawingSnapshotGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) drawingDrawingSnapshotGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) drawingDrawingSnapshotGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const drawingDrawingSnapshotGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : drawingDrawingSnapshotGuardReject(at, "value is not a boolean"));
export const drawingDrawingSnapshotGuardNumber = (value: unknown, at: string, bounds: drawingDrawingSnapshotGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return drawingDrawingSnapshotGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) drawingDrawingSnapshotGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) drawingDrawingSnapshotGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const drawingDrawingSnapshotGuardInteger = (value: unknown, at: string, bounds: drawingDrawingSnapshotGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? drawingDrawingSnapshotGuardNumber(value, at, bounds) : drawingDrawingSnapshotGuardReject(at, "value is not an integer");
export const drawingDrawingSnapshotGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : drawingDrawingSnapshotGuardReject(at, `value is not one of ${members.join(", ")}`);
export const drawingDrawingSnapshotGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : drawingDrawingSnapshotGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseDrawingSnapshot(value: unknown, at = "$"): DrawingSnapshot {
  const row = drawingDrawingSnapshotGuardObject(value, at);
  return {
    schema: drawingDrawingSnapshotGuardString(row["schema"], `${at}.schema`),
    id: drawingDrawingSnapshotGuardString(row["id"], `${at}.id`),
    title: row["title"] === undefined ? undefined : drawingDrawingSnapshotGuardString(row["title"], `${at}.title`),
    layers: drawingDrawingSnapshotGuardArray(row["layers"], `${at}.layers`).map((item, index) => parseDrawingLayerNode(item, `${at}.layers[${index}]`)),
    assets: row["assets"] === undefined ? {} : Object.fromEntries(
      Object.entries(drawingDrawingSnapshotGuardObject(row["assets"], `${at}.assets`))
        .map(([key, item]) => [key, parseDrawingImageAsset(item, `${at}.assets.${key}`)]),
    ),
    artboard: row["artboard"] === undefined ? undefined : parseDrawingArtboard(row["artboard"], `${at}.artboard`),
  };
}
