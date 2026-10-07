import { parseRasterLayerNode, parseRasterLayerMask, parseRasterTransform, rasterTransformNumbers, rasterRasterArtifactGuardRefusal, type RasterLayerNode, type RasterLayerMask, type RasterTransform } from "./../../../🧬️schema/🟦️.ts";
import { parseDslValue, type DslValue, type IntrinsicValue } from "./../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🧬️schema/🟦️.ts";
import { binary64, binary32, binary64Value, binary32Value, type Binary32 } from "./../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";
/** 📝️ Text representation for `raster.raster.snapshot`. */
export type RasterSnapshotText = Readonly<Record<string, unknown>>;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class rasterRasterSnapshotTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const rasterRasterSnapshotTextGuardReject = (at: string, why: string): never => {
  throw new rasterRasterSnapshotTextGuardRefusal(at, why);
};

type rasterRasterSnapshotTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type rasterRasterSnapshotTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type rasterRasterSnapshotTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const rasterRasterSnapshotTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : rasterRasterSnapshotTextGuardReject(at, "value is not an object");
export const rasterRasterSnapshotTextGuardArray = (value: unknown, at: string, bounds: rasterRasterSnapshotTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return rasterRasterSnapshotTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) rasterRasterSnapshotTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) rasterRasterSnapshotTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const rasterRasterSnapshotTextGuardString = (value: unknown, at: string, bounds: rasterRasterSnapshotTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return rasterRasterSnapshotTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) rasterRasterSnapshotTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) rasterRasterSnapshotTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) rasterRasterSnapshotTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const rasterRasterSnapshotTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : rasterRasterSnapshotTextGuardReject(at, "value is not a boolean"));
export const rasterRasterSnapshotTextGuardNumber = (value: unknown, at: string, bounds: rasterRasterSnapshotTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return rasterRasterSnapshotTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) rasterRasterSnapshotTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) rasterRasterSnapshotTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const rasterRasterSnapshotTextGuardInteger = (value: unknown, at: string, bounds: rasterRasterSnapshotTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? rasterRasterSnapshotTextGuardNumber(value, at, bounds) : rasterRasterSnapshotTextGuardReject(at, "value is not an integer");
export const rasterRasterSnapshotTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : rasterRasterSnapshotTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const rasterRasterSnapshotTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : rasterRasterSnapshotTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseRasterSnapshotText(value: unknown, at = "$"): RasterSnapshotText {
  return rasterRasterSnapshotTextGuardObject(value, `${at}`);
}

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

/** 🔢️ Binds finite JSON transform numbers to owned IEEE words. */
export function rasterTransformFromJson(value: unknown, at = "$"): RasterTransform {
  const row = rasterRasterSnapshotTextGuardObject(value, at);
  return parseRasterTransform(Object.fromEntries(["x", "y", "a", "b", "c", "d"].map(key => [key, binary64(rasterRasterSnapshotTextGuardNumber(row[key], `${at}.${key}`))])), at);
}
/** 🎭️ Reads mask state from the physical JSON representation. */
export function rasterLayerMaskFromJson(value: unknown, at = "$"): RasterLayerMask {
  const row = rasterRasterSnapshotTextGuardObject(value, at);
  return parseRasterLayerMask({...row, transform: rasterTransformFromJson(row.transform, `${at}.transform`)}, at);
}
/** 🖼️ Reads a layer tree from physical JSON numbers and parameter values. */
export function rasterLayerNodeFromJson(value: unknown, at = "$"): RasterLayerNode {
  const row = rasterRasterSnapshotTextGuardObject(value, at);
  const bound: Record<string, unknown> = {...row, opacity: binary32(rasterRasterSnapshotTextGuardNumber(row.opacity, `${at}.opacity`)), transform: rasterTransformFromJson(row.transform, `${at}.transform`)};
  if (row.mask != null) bound.mask = rasterLayerMaskFromJson(row.mask, `${at}.mask`);
  if (row.kind === "group") bound.children = rasterRasterSnapshotTextGuardArray(row.children, `${at}.children`).map((child, index) => rasterLayerNodeFromJson(child, `${at}.children[${index}]`));
  if (row.kind === "adjustment") bound.params = Object.fromEntries(Object.entries(rasterRasterSnapshotTextGuardObject(row.params, `${at}.params`)).map(([key, item]) => [key, parseRasterParameter(item)]));
  return parseRasterLayerNode(bound, at);
}
