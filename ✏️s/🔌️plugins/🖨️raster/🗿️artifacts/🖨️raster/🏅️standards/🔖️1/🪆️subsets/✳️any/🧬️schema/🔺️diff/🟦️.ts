/** 🧬️ Raster diff schema — sparse field delta over the artifact. */

export interface RasterDiff {
  /** @state artifact */
  artifact?: RasterArtifact;
  /** @state artifact */
  schema?: string;
  /** @state artifact */
  id?: string;
  /** @state artifact */
  title?: string | null;
  /** @state artifact */
  layers?: RasterLayersDelta;
  /** @state artifact */
  assets?: RasterAssetsDelta;
  /** @state presence */
  selectedIds?: RasterStringList;
  /** @state presence */
  activeUtilityId?: string;
  /** @state config */
  brushSize?: number;
  /** @state config */
  brushOpacity?: number;
  /** @state config */
  compositeViewport?: RasterViewportSize | null;
  /** @state config */
  cameraX?: number;
  /** @state config */
  cameraY?: number;
  /** @state config */
  cameraZoom?: number;
  /** @state config */
  /** @state artifact */
  hoveredId?: string | null;
}

export interface RasterArtifact { [key: string]: unknown; }

export interface RasterAssetsDelta {
  entries: Record<string, RasterImageAsset | null>;
}

export interface RasterImageAsset {
  mime: string;
  data: string;
}

export interface RasterStringList {
  values: string[];
}

export interface RasterLayersDelta {
  added: RasterLayerInsertion[];
  removed: string[];
  patched: RasterLayerPatchEntry[];
  moved: RasterLayerMove[];
}

export interface RasterLayerInsertion {
  parentId?: string;
  index: number;
  layer: RasterLayerNode;
}

export interface RasterLayerMove {
  id: string;
  parentId?: string;
  index: number;
}

export interface RasterLayerPatchEntry {
  id: string;
  patch: RasterLayerPatch;
}

export interface RasterLayerPatch {
  name?: string;
  visible?: boolean;
  opacity?: number;
  blendMode?: string;
  transformX?: number;
  transformY?: number;
  width?: number;
  height?: number;
  adjustmentKind?: string;
}

export interface RasterLayerNode { [key: string]: unknown; }

export interface RasterViewportSize {
  width: number;
  height: number;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class rasterRasterDiffGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const rasterRasterDiffGuardReject = (at: string, why: string): never => {
  throw new rasterRasterDiffGuardRefusal(at, why);
};

type rasterRasterDiffGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type rasterRasterDiffGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type rasterRasterDiffGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const rasterRasterDiffGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : rasterRasterDiffGuardReject(at, "value is not an object");
export const rasterRasterDiffGuardArray = (value: unknown, at: string, bounds: rasterRasterDiffGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return rasterRasterDiffGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) rasterRasterDiffGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) rasterRasterDiffGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const rasterRasterDiffGuardString = (value: unknown, at: string, bounds: rasterRasterDiffGuardTextBounds = {}): string => {
  if (typeof value !== "string") return rasterRasterDiffGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) rasterRasterDiffGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) rasterRasterDiffGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) rasterRasterDiffGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const rasterRasterDiffGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : rasterRasterDiffGuardReject(at, "value is not a boolean"));
export const rasterRasterDiffGuardNumber = (value: unknown, at: string, bounds: rasterRasterDiffGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return rasterRasterDiffGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) rasterRasterDiffGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) rasterRasterDiffGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const rasterRasterDiffGuardInteger = (value: unknown, at: string, bounds: rasterRasterDiffGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? rasterRasterDiffGuardNumber(value, at, bounds) : rasterRasterDiffGuardReject(at, "value is not an integer");
export const rasterRasterDiffGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : rasterRasterDiffGuardReject(at, `value is not one of ${members.join(", ")}`);
export const rasterRasterDiffGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : rasterRasterDiffGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseRasterDiff(value: unknown, at = "$"): RasterDiff {
  const row = rasterRasterDiffGuardObject(value, at);
  return {
    schema: row["schema"] === undefined ? undefined : rasterRasterDiffGuardString(row["schema"], `${at}.schema`),
    value: row["value"] === undefined ? undefined : rasterRasterDiffGuardString(row["value"], `${at}.value`),
  };
}
