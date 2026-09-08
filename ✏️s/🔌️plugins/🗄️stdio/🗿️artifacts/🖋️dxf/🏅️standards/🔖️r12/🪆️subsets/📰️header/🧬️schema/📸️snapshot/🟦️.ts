/** 🧬️ DxfSnapshot schema facet — complete per DXF R12 ASCII spec. Mirrors
 * `📸️snapshot/🦀️.rs` field-for-field (camelCase). */

/** 🧮 Typed union over DXF group-code value kinds (string/integer/double/point-component). */
export type DxfValue =
  | { kind: 'str'; value: string }
  | { kind: 'int'; value: number }
  | { kind: 'double'; value: number }
  | { kind: 'point'; value: [number, number, number] };

export type DxfGroupCode = [number, DxfValue];

/** 🏷️ One raw DXF group-code/value pair — raw-retention unit only. */
export interface DxfTag {
  code: number;
  value: string;
}

/** 🏷️ One `$VAR` header entry. */
export interface DxfHeaderVar {
  name: string;
  groupCode: number;
  value: DxfValue;
  extraGroupCodes?: DxfGroupCode[];
}

/** 🗂️ `LAYER` table entry. */
export interface DxfLayer {
  name: string;
  color: number;
  linetype: string;
  flags: number;
  unknownGroupCodes?: DxfGroupCode[];
}

/** 🗂️ `STYLE` table entry. */
export interface DxfStyle {
  name: string;
  flags: number;
  fontName: string;
  unknownGroupCodes?: DxfGroupCode[];
}

/** 🗂️ `LTYPE` table entry. */
export interface DxfLinetype {
  name: string;
  flags: number;
  description: string;
  unknownGroupCodes?: DxfGroupCode[];
}

/** 🗂️ The three name-keyed table kinds this codec typed-models. */
export interface DxfTables {
  layers: DxfLayer[];
  styles: DxfStyle[];
  linetypes: DxfLinetype[];
}

/** 🕳️ Raw retention for any table kind other than LAYER/STYLE/LTYPE. */
export interface DxfOtherTable {
  name: string;
  tags: DxfTag[];
}

/** 📍 One `POLYLINE` vertex record. */
export interface DxfVertex {
  x: number;
  y: number;
  z: number;
  bulge: number;
  unknownGroupCodes?: DxfGroupCode[];
}

type Vec3 = [number, number, number];

/** 📐️ The R12 entity set this codec types directly; `other` retains any unmodeled kind. */
export type DxfEntity =
  | { line: { start: Vec3; end: Vec3; layer: string; unknownGroupCodes?: DxfGroupCode[] } }
  | { circle: { center: Vec3; radius: number; layer: string; unknownGroupCodes?: DxfGroupCode[] } }
  | { arc: { center: Vec3; radius: number; startAngle: number; endAngle: number; layer: string; unknownGroupCodes?: DxfGroupCode[] } }
  | { polyline: { vertices: DxfVertex[]; closed: boolean; layer: string; unknownGroupCodes?: DxfGroupCode[] } }
  | { text: { position: Vec3; height: number; value: string; layer: string; unknownGroupCodes?: DxfGroupCode[] } }
  | { solid: { points: [Vec3, Vec3, Vec3, Vec3]; layer: string; unknownGroupCodes?: DxfGroupCode[] } }
  | { insert: { blockName: string; position: Vec3; scale: Vec3; rotation: number; layer: string; unknownGroupCodes?: DxfGroupCode[] } }
  | { other: { kind: string; groupCodes?: DxfGroupCode[] } };

/** 🧱 One `BLOCK` — name, base point, its own nested entity list. */
export interface DxfBlock {
  name: string;
  basePoint: Vec3;
  entities: DxfEntity[];
  unknownGroupCodes?: DxfGroupCode[];
}

/** 🧬️ Complete `stdio.dxf` (r12) document snapshot. */
export interface DxfSnapshot {
  schema: string;
  headerVars: DxfHeaderVar[];
  tables: DxfTables;
  otherTables: DxfOtherTable[];
  blocks: DxfBlock[];
  entities: DxfEntity[];
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioDxfR12HeaderSnapshotGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioDxfR12HeaderSnapshotGuardReject = (at: string, why: string): never => {
  throw new stdioDxfR12HeaderSnapshotGuardRefusal(at, why);
};

type stdioDxfR12HeaderSnapshotGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioDxfR12HeaderSnapshotGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioDxfR12HeaderSnapshotGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioDxfR12HeaderSnapshotGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioDxfR12HeaderSnapshotGuardReject(at, "value is not an object");
export const stdioDxfR12HeaderSnapshotGuardArray = (value: unknown, at: string, bounds: stdioDxfR12HeaderSnapshotGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioDxfR12HeaderSnapshotGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioDxfR12HeaderSnapshotGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioDxfR12HeaderSnapshotGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioDxfR12HeaderSnapshotGuardString = (value: unknown, at: string, bounds: stdioDxfR12HeaderSnapshotGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioDxfR12HeaderSnapshotGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioDxfR12HeaderSnapshotGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioDxfR12HeaderSnapshotGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioDxfR12HeaderSnapshotGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioDxfR12HeaderSnapshotGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioDxfR12HeaderSnapshotGuardReject(at, "value is not a boolean"));
export const stdioDxfR12HeaderSnapshotGuardNumber = (value: unknown, at: string, bounds: stdioDxfR12HeaderSnapshotGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioDxfR12HeaderSnapshotGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioDxfR12HeaderSnapshotGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioDxfR12HeaderSnapshotGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioDxfR12HeaderSnapshotGuardInteger = (value: unknown, at: string, bounds: stdioDxfR12HeaderSnapshotGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioDxfR12HeaderSnapshotGuardNumber(value, at, bounds) : stdioDxfR12HeaderSnapshotGuardReject(at, "value is not an integer");
export const stdioDxfR12HeaderSnapshotGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioDxfR12HeaderSnapshotGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioDxfR12HeaderSnapshotGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioDxfR12HeaderSnapshotGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseDxfSnapshot(value: unknown, at = "$"): DxfSnapshot {
  const row = stdioDxfR12HeaderSnapshotGuardObject(value, at);
  return {
    schema: stdioDxfR12HeaderSnapshotGuardString(row["schema"], `${at}.schema`),
    headerVars: stdioDxfR12HeaderSnapshotGuardArray(row["headerVars"], `${at}.headerVars`).map((item, index) => parseDxfHeaderVar(item, `${at}.headerVars[${index}]`)),
    tables: parseDxfTables(row["tables"], `${at}.tables`),
    otherTables: stdioDxfR12HeaderSnapshotGuardArray(row["otherTables"], `${at}.otherTables`).map((item, index) => parseDxfOtherTable(item, `${at}.otherTables[${index}]`)),
    blocks: stdioDxfR12HeaderSnapshotGuardArray(row["blocks"], `${at}.blocks`).map((item, index) => parseDxfBlock(item, `${at}.blocks[${index}]`)),
    entities: stdioDxfR12HeaderSnapshotGuardArray(row["entities"], `${at}.entities`).map((item, index) => parseDxfEntity(item, `${at}.entities[${index}]`)),
  };
}

export function parseDxfTag(value: unknown, at = "$"): DxfTag {
  const row = stdioDxfR12HeaderSnapshotGuardObject(value, at);
  return {
    code: stdioDxfR12HeaderSnapshotGuardInteger(row["code"], `${at}.code`),
    value: stdioDxfR12HeaderSnapshotGuardString(row["value"], `${at}.value`),
  };
}

export function parseDxfHeaderVar(value: unknown, at = "$"): DxfHeaderVar {
  const row = stdioDxfR12HeaderSnapshotGuardObject(value, at);
  return {
    name: stdioDxfR12HeaderSnapshotGuardString(row["name"], `${at}.name`),
    groupCode: stdioDxfR12HeaderSnapshotGuardInteger(row["groupCode"], `${at}.groupCode`),
    value: parseDxfValue(row["value"], `${at}.value`),
    extraGroupCodes: row["extraGroupCodes"] === undefined ? undefined : stdioDxfR12HeaderSnapshotGuardArray(row["extraGroupCodes"], `${at}.extraGroupCodes`).map((item, index) => parseDxfGroupCode(item, `${at}.extraGroupCodes[${index}]`)),
  };
}

export type Vec3 = readonly number[];

export function parseVec3(value: unknown, at = "$"): Vec3 {
  return stdioDxfR12HeaderSnapshotGuardArray(value, `${at}`, {"minItems": 3, "maxItems": 3}).map((item, index) => stdioDxfR12HeaderSnapshotGuardNumber(item, `${at}[${index}]`));
}

export function parseDxfLayer(value: unknown, at = "$"): DxfLayer {
  const row = stdioDxfR12HeaderSnapshotGuardObject(value, at);
  return {
    name: stdioDxfR12HeaderSnapshotGuardString(row["name"], `${at}.name`),
    color: stdioDxfR12HeaderSnapshotGuardInteger(row["color"], `${at}.color`),
    linetype: stdioDxfR12HeaderSnapshotGuardString(row["linetype"], `${at}.linetype`),
    flags: stdioDxfR12HeaderSnapshotGuardInteger(row["flags"], `${at}.flags`),
    unknownGroupCodes: row["unknownGroupCodes"] === undefined ? undefined : stdioDxfR12HeaderSnapshotGuardArray(row["unknownGroupCodes"], `${at}.unknownGroupCodes`).map((item, index) => parseDxfGroupCode(item, `${at}.unknownGroupCodes[${index}]`)),
  };
}

export function parseDxfStyle(value: unknown, at = "$"): DxfStyle {
  const row = stdioDxfR12HeaderSnapshotGuardObject(value, at);
  return {
    name: stdioDxfR12HeaderSnapshotGuardString(row["name"], `${at}.name`),
    flags: stdioDxfR12HeaderSnapshotGuardInteger(row["flags"], `${at}.flags`),
    fontName: stdioDxfR12HeaderSnapshotGuardString(row["fontName"], `${at}.fontName`),
    unknownGroupCodes: row["unknownGroupCodes"] === undefined ? undefined : stdioDxfR12HeaderSnapshotGuardArray(row["unknownGroupCodes"], `${at}.unknownGroupCodes`).map((item, index) => parseDxfGroupCode(item, `${at}.unknownGroupCodes[${index}]`)),
  };
}

export function parseDxfLinetype(value: unknown, at = "$"): DxfLinetype {
  const row = stdioDxfR12HeaderSnapshotGuardObject(value, at);
  return {
    name: stdioDxfR12HeaderSnapshotGuardString(row["name"], `${at}.name`),
    flags: stdioDxfR12HeaderSnapshotGuardInteger(row["flags"], `${at}.flags`),
    description: stdioDxfR12HeaderSnapshotGuardString(row["description"], `${at}.description`),
    unknownGroupCodes: row["unknownGroupCodes"] === undefined ? undefined : stdioDxfR12HeaderSnapshotGuardArray(row["unknownGroupCodes"], `${at}.unknownGroupCodes`).map((item, index) => parseDxfGroupCode(item, `${at}.unknownGroupCodes[${index}]`)),
  };
}

export function parseDxfTables(value: unknown, at = "$"): DxfTables {
  const row = stdioDxfR12HeaderSnapshotGuardObject(value, at);
  return {
    layers: stdioDxfR12HeaderSnapshotGuardArray(row["layers"], `${at}.layers`).map((item, index) => parseDxfLayer(item, `${at}.layers[${index}]`)),
    styles: stdioDxfR12HeaderSnapshotGuardArray(row["styles"], `${at}.styles`).map((item, index) => parseDxfStyle(item, `${at}.styles[${index}]`)),
    linetypes: stdioDxfR12HeaderSnapshotGuardArray(row["linetypes"], `${at}.linetypes`).map((item, index) => parseDxfLinetype(item, `${at}.linetypes[${index}]`)),
  };
}

export function parseDxfOtherTable(value: unknown, at = "$"): DxfOtherTable {
  const row = stdioDxfR12HeaderSnapshotGuardObject(value, at);
  return {
    name: stdioDxfR12HeaderSnapshotGuardString(row["name"], `${at}.name`),
    tags: stdioDxfR12HeaderSnapshotGuardArray(row["tags"], `${at}.tags`).map((item, index) => parseDxfTag(item, `${at}.tags[${index}]`)),
  };
}

export function parseDxfVertex(value: unknown, at = "$"): DxfVertex {
  const row = stdioDxfR12HeaderSnapshotGuardObject(value, at);
  return {
    x: stdioDxfR12HeaderSnapshotGuardNumber(row["x"], `${at}.x`),
    y: stdioDxfR12HeaderSnapshotGuardNumber(row["y"], `${at}.y`),
    z: stdioDxfR12HeaderSnapshotGuardNumber(row["z"], `${at}.z`),
    bulge: stdioDxfR12HeaderSnapshotGuardNumber(row["bulge"], `${at}.bulge`),
    unknownGroupCodes: row["unknownGroupCodes"] === undefined ? undefined : stdioDxfR12HeaderSnapshotGuardArray(row["unknownGroupCodes"], `${at}.unknownGroupCodes`).map((item, index) => parseDxfGroupCode(item, `${at}.unknownGroupCodes[${index}]`)),
  };
}

export function parseDxfBlock(value: unknown, at = "$"): DxfBlock {
  const row = stdioDxfR12HeaderSnapshotGuardObject(value, at);
  return {
    name: stdioDxfR12HeaderSnapshotGuardString(row["name"], `${at}.name`),
    basePoint: parseVec3(row["basePoint"], `${at}.basePoint`),
    entities: stdioDxfR12HeaderSnapshotGuardArray(row["entities"], `${at}.entities`).map((item, index) => parseDxfEntity(item, `${at}.entities[${index}]`)),
    unknownGroupCodes: row["unknownGroupCodes"] === undefined ? undefined : stdioDxfR12HeaderSnapshotGuardArray(row["unknownGroupCodes"], `${at}.unknownGroupCodes`).map((item, index) => parseDxfGroupCode(item, `${at}.unknownGroupCodes[${index}]`)),
  };
}
