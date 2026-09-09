/** 🧬️ Raster snapshot schema — artifact-lane fields only. */
import { parseArtifactChild, type ArtifactChild } from "../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🟦️.ts";
import { parseRasterLayerNode, type RasterLayerNode } from "../🟦️.ts";

export interface RasterSnapshot {
  /** @state artifact */
  schema: string;
  /** @state artifact */
  id: string;
  /** @state artifact */
  title?: string;
  /** @state artifact */
  layers: RasterLayerNode[];
  /** @state artifact */
  assets: Record<string, ArtifactChild>;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class rasterRasterSnapshotGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const rasterRasterSnapshotGuardReject = (at: string, why: string): never => {
  throw new rasterRasterSnapshotGuardRefusal(at, why);
};

type rasterRasterSnapshotGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type rasterRasterSnapshotGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type rasterRasterSnapshotGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const rasterRasterSnapshotGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : rasterRasterSnapshotGuardReject(at, "value is not an object");
export const rasterRasterSnapshotGuardArray = (value: unknown, at: string, bounds: rasterRasterSnapshotGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return rasterRasterSnapshotGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) rasterRasterSnapshotGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) rasterRasterSnapshotGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const rasterRasterSnapshotGuardString = (value: unknown, at: string, bounds: rasterRasterSnapshotGuardTextBounds = {}): string => {
  if (typeof value !== "string") return rasterRasterSnapshotGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) rasterRasterSnapshotGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) rasterRasterSnapshotGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) rasterRasterSnapshotGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const rasterRasterSnapshotGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : rasterRasterSnapshotGuardReject(at, "value is not a boolean"));
export const rasterRasterSnapshotGuardNumber = (value: unknown, at: string, bounds: rasterRasterSnapshotGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return rasterRasterSnapshotGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) rasterRasterSnapshotGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) rasterRasterSnapshotGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const rasterRasterSnapshotGuardInteger = (value: unknown, at: string, bounds: rasterRasterSnapshotGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? rasterRasterSnapshotGuardNumber(value, at, bounds) : rasterRasterSnapshotGuardReject(at, "value is not an integer");
export const rasterRasterSnapshotGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : rasterRasterSnapshotGuardReject(at, `value is not one of ${members.join(", ")}`);
export const rasterRasterSnapshotGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : rasterRasterSnapshotGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseRasterSnapshot(value: unknown, at = "$"): RasterSnapshot {
  const row = rasterRasterSnapshotGuardObject(value, at);
  return {
    schema: rasterRasterSnapshotGuardString(row["schema"], `${at}.schema`),
    id: rasterRasterSnapshotGuardString(row["id"], `${at}.id`),
    title: row["title"] === undefined ? undefined : rasterRasterSnapshotGuardString(row["title"], `${at}.title`),
    layers: rasterRasterSnapshotGuardArray(row["layers"], `${at}.layers`).map((item, index) => parseRasterLayerNode(item, `${at}.layers[${index}]`)),
    assets: row["assets"] === undefined ? {} : Object.fromEntries(
      Object.entries(rasterRasterSnapshotGuardObject(row["assets"], `${at}.assets`))
        .map(([key, item]) => [key, parseArtifactChild(item)]),
    ),
  };
}
