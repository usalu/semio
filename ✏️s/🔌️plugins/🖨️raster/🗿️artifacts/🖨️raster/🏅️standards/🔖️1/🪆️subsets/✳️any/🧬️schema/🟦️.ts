/** 🧬️ Raster artifact schema — every field with its state class. */
import { parseDslValue, type DslValue, type IntrinsicValue } from "../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🧬️schema/🟦️.ts";
import { parseArtifactChild, type ArtifactChild } from "../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🟦️.ts";

import {type Binary64,type Binary32,parseBinary64Transport,parseBinary32Transport,binary64,binary64Value,binary32Value} from "../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔢️ieee754/🟦️.ts";

export interface RasterArtifact {
  schema: string;
  id: string;
  title?: string;
  layers: RasterLayerNode[];
  assets: Record<string, ArtifactChild>;
}

export type RasterLayerNode = RasterLayerPixel | RasterLayerGroup | RasterLayerAdjustment;

export interface RasterLayerPixel {
  kind: "pixel";
  id: string;
  name: string;
  visible: boolean;
  locked: boolean;
  opacity: Binary32;
  blendMode: string;
  transform: RasterTransform;
  mask?: RasterLayerMask;
  width?: number;
  height?: number;
  imageKey?: string;
}

export interface RasterLayerGroup {
  kind: "group";
  id: string;
  name: string;
  visible: boolean;
  locked: boolean;
  opacity: Binary32;
  blendMode: string;
  transform: RasterTransform;
  mask?: RasterLayerMask;
  children: RasterLayerNode[];
}

export interface RasterLayerAdjustment {
  kind: "adjustment";
  id: string;
  name: string;
  visible: boolean;
  locked: boolean;
  opacity: Binary32;
  blendMode: string;
  transform: RasterTransform;
  adjustmentKind: string;
  params: Record<string, IntrinsicValue>;
}

export interface RasterTransform {
  x: Binary64;
  y: Binary64;
  a: Binary64;
  b: Binary64;
  c: Binary64;
  d: Binary64;
}

export interface RasterLayerMask {
  enabled: boolean;
  linked: boolean;
  invert: boolean;
  width?: number;
  height?: number;
  imageKey?: string;
  transform: RasterTransform;
}

export interface RasterImageAsset {
  mime: string;
  data: string;
}

export interface RasterViewportSize {
  width: number;
  height: number;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class rasterRasterArtifactGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const rasterRasterArtifactGuardReject = (at: string, why: string): never => {
  throw new rasterRasterArtifactGuardRefusal(at, why);
};

type rasterRasterArtifactGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type rasterRasterArtifactGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type rasterRasterArtifactGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const rasterRasterArtifactGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : rasterRasterArtifactGuardReject(at, "value is not an object");
export const rasterRasterArtifactGuardArray = (value: unknown, at: string, bounds: rasterRasterArtifactGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return rasterRasterArtifactGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) rasterRasterArtifactGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) rasterRasterArtifactGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const rasterRasterArtifactGuardString = (value: unknown, at: string, bounds: rasterRasterArtifactGuardTextBounds = {}): string => {
  if (typeof value !== "string") return rasterRasterArtifactGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) rasterRasterArtifactGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) rasterRasterArtifactGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) rasterRasterArtifactGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const rasterRasterArtifactGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : rasterRasterArtifactGuardReject(at, "value is not a boolean"));
export const rasterRasterArtifactGuardNumber = (value: unknown, at: string, bounds: rasterRasterArtifactGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return rasterRasterArtifactGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) rasterRasterArtifactGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) rasterRasterArtifactGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const rasterRasterArtifactGuardInteger = (value: unknown, at: string, bounds: rasterRasterArtifactGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? rasterRasterArtifactGuardNumber(value, at, bounds) : rasterRasterArtifactGuardReject(at, "value is not an integer");
export const rasterRasterArtifactGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : rasterRasterArtifactGuardReject(at, `value is not one of ${members.join(", ")}`);
export const rasterRasterArtifactGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : rasterRasterArtifactGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseRasterArtifact(value: unknown, at = "$"): RasterArtifact {
  const row = rasterRasterArtifactGuardObject(value, at);
  return {
    schema: rasterRasterArtifactGuardString(row["schema"], `${at}.schema`),
    id: rasterRasterArtifactGuardString(row["id"], `${at}.id`),
    title: row["title"] === undefined ? undefined : rasterRasterArtifactGuardString(row["title"], `${at}.title`),
    layers: rasterRasterArtifactGuardArray(row["layers"], `${at}.layers`).map((item, index) => parseRasterLayerNode(item, `${at}.layers[${index}]`)),
    assets: Object.fromEntries(
      Object.entries(rasterRasterArtifactGuardObject(row["assets"], `${at}.assets`))
        .map(([key, item]) => [key, parseArtifactChild(item)]),
    ),
  };
}

export function parseRasterImageAsset(value: unknown, at = "$"): RasterImageAsset {
  const row = rasterRasterArtifactGuardObject(value, at);
  return {
    mime: rasterRasterArtifactGuardString(row["mime"], `${at}.mime`),
    data: rasterRasterArtifactGuardString(row["data"], `${at}.data`),
  };
}

export function parseRasterLayerNode(value: unknown, at = "$"): RasterLayerNode {
  const row = rasterRasterArtifactGuardObject(value, at);
  const kind = rasterRasterArtifactGuardMember(row["kind"], `${at}.kind`, ["pixel", "group", "adjustment"] as const);
  const common = {
    id: rasterRasterArtifactGuardString(row["id"], `${at}.id`),
    name: rasterRasterArtifactGuardString(row["name"], `${at}.name`),
    visible: rasterRasterArtifactGuardBoolean(row["visible"], `${at}.visible`),
    locked: rasterRasterArtifactGuardBoolean(row["locked"], `${at}.locked`),
    opacity: parseBinary32Transport(row["opacity"]),
    blendMode: rasterRasterArtifactGuardString(row["blendMode"], `${at}.blendMode`),
    transform: parseRasterTransform(row["transform"], `${at}.transform`),
  };
  if (kind === "pixel") {
    return {
      kind,
      ...common,
      mask: row["mask"] == null ? undefined : parseRasterLayerMask(row["mask"], `${at}.mask`),
      width: row["width"] == null ? undefined : rasterRasterArtifactGuardInteger(row["width"], `${at}.width`, { minimum: 0, maximum: 4294967295 }),
      height: row["height"] == null ? undefined : rasterRasterArtifactGuardInteger(row["height"], `${at}.height`, { minimum: 0, maximum: 4294967295 }),
      imageKey: row["imageKey"] == null ? undefined : rasterRasterArtifactGuardString(row["imageKey"], `${at}.imageKey`),
    };
  }
  if (kind === "group") {
    return {
      kind,
      ...common,
      mask: row["mask"] == null ? undefined : parseRasterLayerMask(row["mask"], `${at}.mask`),
      children: rasterRasterArtifactGuardArray(row["children"], `${at}.children`).map((item, index) => parseRasterLayerNode(item, `${at}.children[${index}]`)),
    };
  }
  return {
    kind,
    ...common,
    adjustmentKind: rasterRasterArtifactGuardString(row["adjustmentKind"], `${at}.adjustmentKind`),
    params: Object.fromEntries(
      Object.entries(rasterRasterArtifactGuardObject(row["params"], `${at}.params`))
        .map(([key, item]) => [key, parseRasterParameter(item)]),
    ),
  };
}

export function parseRasterTransform(value: unknown, at = "$"): RasterTransform {
  const row = rasterRasterArtifactGuardObject(value, at);
  return {
    x: parseBinary64Transport(row["x"]),
    y: parseBinary64Transport(row["y"]),
    a: parseBinary64Transport(row["a"]),
    b: parseBinary64Transport(row["b"]),
    c: parseBinary64Transport(row["c"]),
    d: parseBinary64Transport(row["d"]),
  };
}

export function parseRasterLayerMask(value: unknown, at = "$"): RasterLayerMask {
  const row = rasterRasterArtifactGuardObject(value, at);
  return {
    enabled: rasterRasterArtifactGuardBoolean(row["enabled"], `${at}.enabled`),
    linked: rasterRasterArtifactGuardBoolean(row["linked"], `${at}.linked`),
    invert: rasterRasterArtifactGuardBoolean(row["invert"], `${at}.invert`),
    width: row["width"] == null ? undefined : rasterRasterArtifactGuardInteger(row["width"], `${at}.width`, { minimum: 1, maximum: 16384 }),
    height: row["height"] == null ? undefined : rasterRasterArtifactGuardInteger(row["height"], `${at}.height`, { minimum: 1, maximum: 16384 }),
    imageKey: row["imageKey"] == null ? undefined : rasterRasterArtifactGuardString(row["imageKey"], `${at}.imageKey`, { minLength: 1 }),
    transform: parseRasterTransform(row["transform"], `${at}.transform`),
  };
}

//#region 🔖️JsonProjection
/** 🔢️ A binary32 word as the JSON number the native printer writes: the shortest decimal that rounds back to the same binary32. */
export function rasterBinary32Number(word: Binary32): number {
  const value = binary32Value(word);
  if (!Number.isFinite(value)) return value;
  for (let digits = 1; digits <= 9; digits++) {
    const candidate = Number(value.toPrecision(digits));
    if (Math.fround(candidate) === value) return candidate;
  }
  return value;
}

/** 🔢️ The transform's derived numeric values — what compositing math and the JSON wire read; the exact identity stays the words. */
export function rasterTransformNumbers(transform: RasterTransform): { x: number; y: number; a: number; b: number; c: number; d: number } {
  return { x: binary64Value(transform.x), y: binary64Value(transform.y), a: binary64Value(transform.a), b: binary64Value(transform.b), c: binary64Value(transform.c), d: binary64Value(transform.d) };
}

/** 🌱️ Reads one adjustment parameter from its JSON wire (`DslValue` projection) into the owned intrinsic tree: integers become
 * `unsigned`/`signed`, every other number a binary64 `float`, without recursive calls. */
export function parseRasterParameter(value: unknown): IntrinsicValue {
  const root = parseDslValue(value);
  let result: IntrinsicValue | undefined;
  const pending: { source: DslValue; put: (value: IntrinsicValue) => void }[] = [{ source: root, put: (value) => { result = value; } }];
  while (pending.length) {
    const { source, put } = pending.pop()!;
    if (source === null) put({ kind: "null" });
    else if (typeof source === "boolean") put({ kind: "boolean", value: source });
    else if (typeof source === "number") put(Number.isSafeInteger(source) ? { kind: source < 0 ? "signed" : "unsigned", value: BigInt(source) } : { kind: "float", value: binary64(source) });
    else if (typeof source === "string") put({ kind: "text", value: source });
    else if (Array.isArray(source)) {
      const items: IntrinsicValue[] = new Array(source.length);
      put({ kind: "array", items });
      source.forEach((item, index) => pending.push({ source: item, put: (value) => { items[index] = value; } }));
    } else {
      const entries = Object.entries(source), members: { name: string; value: IntrinsicValue }[] = entries.map(([name]) => ({ name, value: { kind: "null" } }));
      put({ kind: "object", members });
      entries.forEach(([, item], index) => pending.push({ source: item, put: (value) => { members[index]!.value = value; } }));
    }
  }
  return result!;
}

/** 🌱️ Prints one owned intrinsic parameter as its JSON wire value (the inverse of `parseRasterParameter`); octets have no JSON projection. */
export function printRasterParameter(value: IntrinsicValue): DslValue {
  let result: DslValue = null;
  const pending: { source: IntrinsicValue; put: (value: DslValue) => void }[] = [{ source: value, put: (value) => { result = value; } }];
  while (pending.length) {
    const { source, put } = pending.pop()!;
    switch (source.kind) {
      case "null": put(null); break;
      case "boolean": case "text": put(source.value); break;
      case "unsigned": case "signed": put(Number(source.value)); break;
      case "float": put(binary64Value(source.value)); break;
      case "bytes": throw new rasterRasterArtifactGuardRefusal("$", "octet parameters have no JSON projection");
      case "array": { const items: DslValue[] = new Array(source.items.length); put(items); source.items.forEach((item, index) => pending.push({ source: item, put: (value) => { items[index] = value; } })); break; }
      case "object": { const members: { [key: string]: DslValue } = {}; put(members); for (const member of source.members) pending.push({ source: member.value, put: (value) => { members[member.name] = value; } }); break; }
    }
  }
  return result;
}

/** 🎭️ Prints a mask as its JSON wire object (absent extents and key omitted). */
export function printRasterLayerMask(mask: RasterLayerMask): Record<string, unknown> {
  return { enabled: mask.enabled, linked: mask.linked, invert: mask.invert, width: mask.width, height: mask.height, imageKey: mask.imageKey, transform: rasterTransformNumbers(mask.transform) };
}

/** 🧾️ Prints a layer node as its JSON wire object — the inverse of `parseRasterLayerNode`. */
export function printRasterLayerNode(node: RasterLayerNode): Record<string, unknown> {
  const common = { kind: node.kind, id: node.id, name: node.name, visible: node.visible, locked: node.locked, opacity: rasterBinary32Number(node.opacity), blendMode: node.blendMode, transform: rasterTransformNumbers(node.transform) };
  if (node.kind === "pixel") return { ...common, mask: node.mask && printRasterLayerMask(node.mask), width: node.width, height: node.height, imageKey: node.imageKey };
  if (node.kind === "group") return { ...common, mask: node.mask && printRasterLayerMask(node.mask), children: node.children.map(printRasterLayerNode) };
  return { ...common, adjustmentKind: node.adjustmentKind, params: Object.fromEntries(Object.entries(node.params).map(([key, value]) => [key, printRasterParameter(value)])) };
}
//#endregion 🔖️JsonProjection

export function parseRasterViewportSize(value: unknown, at = "$"): RasterViewportSize {
  const row = rasterRasterArtifactGuardObject(value, at);
  return {
    width: rasterRasterArtifactGuardNumber(row["width"], `${at}.width`),
    height: rasterRasterArtifactGuardNumber(row["height"], `${at}.height`),
  };
}

export type LayerProtectionNode={id:string;locked:boolean;children?:readonly LayerProtectionNode[]};
export type LayerProtection={locked:boolean;inherited:boolean;descendant:boolean;editable:boolean;structural:boolean;canChangeLock:boolean};
export function layerProtection(layers:readonly LayerProtectionNode[],id:string):LayerProtection|null {
  const lockedBelow=(nodes:readonly LayerProtectionNode[]):boolean=>nodes.some(node=>node.locked||lockedBelow(node.children??[]));
  const visit=(nodes:readonly LayerProtectionNode[],inherited:boolean):LayerProtection|null=>{
    for(const node of nodes) {
      if(node.id===id) {const descendant=lockedBelow(node.children??[]),editable=!node.locked&&!inherited;return {locked:node.locked,inherited,descendant,editable,structural:editable&&!descendant,canChangeLock:!inherited};}
      const found=visit(node.children??[],inherited||node.locked);if(found)return found;
    }
    return null;
  };
  return visit(layers,false);
}
