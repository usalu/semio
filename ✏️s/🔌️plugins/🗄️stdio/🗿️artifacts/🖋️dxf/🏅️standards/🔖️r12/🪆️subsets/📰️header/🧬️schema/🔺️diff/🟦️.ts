/** 🔺️ DxfDiff schema facet — handcrafted sparse diff. Mirrors `🔺️diff/🦀️.rs`
 * field-for-field (camelCase). Name-keyed removed/modified/added triples for
 * headerVars/layers/styles/linetypes; index-keyed triples for blocks/entities. No
 * full-replace `snapshot` slot anywhere. */
import { parseDxfOtherTable } from "../📸️snapshot/🟦️.ts";
import { parseDxfBlock, parseDxfEntity, parseDxfGroupCode, parseDxfHeaderVar, parseDxfLayer, parseDxfLinetype, parseDxfStyle, parseDxfValue } from "../📸️snapshot/🟦️.ts";

import type {
  DxfBlock, DxfEntity, DxfGroupCode, DxfHeaderVar, DxfLayer, DxfLinetype, DxfOtherTable, DxfStyle, DxfVertex,
} from '../📸️snapshot/🟦️.ts';

//#region HeaderVarDiff
export interface DxfHeaderVarDiff {
  groupCode?: number;
  value?: import('../📸️snapshot/🟦️.ts').DxfValue;
  extraGroupCodes?: DxfGroupCode[];
}
export interface DxfHeaderVarModified { name: string; diff: DxfHeaderVarDiff; }
export interface DxfHeaderVarAdded { index: number; headerVar: DxfHeaderVar; }
export interface DxfHeaderVarsDiff {
  removed?: string[];
  modified?: DxfHeaderVarModified[];
  added?: DxfHeaderVarAdded[];
}
//#endregion

//#region TableDiffs
export interface DxfLayerDiff { color?: number; linetype?: string; flags?: number; unknownGroupCodes?: DxfGroupCode[]; }
export interface DxfLayerModified { name: string; diff: DxfLayerDiff; }
export interface DxfLayerAdded { index: number; layer: DxfLayer; }
export interface DxfLayersDiff { removed?: string[]; modified?: DxfLayerModified[]; added?: DxfLayerAdded[]; }

export interface DxfStyleDiff { flags?: number; fontName?: string; unknownGroupCodes?: DxfGroupCode[]; }
export interface DxfStyleModified { name: string; diff: DxfStyleDiff; }
export interface DxfStyleAdded { index: number; style: DxfStyle; }
export interface DxfStylesDiff { removed?: string[]; modified?: DxfStyleModified[]; added?: DxfStyleAdded[]; }

export interface DxfLinetypeDiff { flags?: number; description?: string; unknownGroupCodes?: DxfGroupCode[]; }
export interface DxfLinetypeModified { name: string; diff: DxfLinetypeDiff; }
export interface DxfLinetypeAdded { index: number; linetype: DxfLinetype; }
export interface DxfLinetypesDiff { removed?: string[]; modified?: DxfLinetypeModified[]; added?: DxfLinetypeAdded[]; }

export interface DxfTablesDiff {
  layers?: DxfLayersDiff;
  styles?: DxfStylesDiff;
  linetypes?: DxfLinetypesDiff;
}
//#endregion

//#region EntityDiff
export interface DxfLineDiff { start?: [number, number, number]; end?: [number, number, number]; layer?: string; unknownGroupCodes?: DxfGroupCode[]; }
export interface DxfCircleDiff { center?: [number, number, number]; radius?: number; layer?: string; unknownGroupCodes?: DxfGroupCode[]; }
export interface DxfArcDiff { center?: [number, number, number]; radius?: number; startAngle?: number; endAngle?: number; layer?: string; unknownGroupCodes?: DxfGroupCode[]; }
export interface DxfPolylineDiff { vertices?: DxfVertex[]; closed?: boolean; layer?: string; unknownGroupCodes?: DxfGroupCode[]; }
export interface DxfTextDiff { position?: [number, number, number]; height?: number; value?: string; layer?: string; unknownGroupCodes?: DxfGroupCode[]; }
export interface DxfSolidDiff { points?: [[number, number, number], [number, number, number], [number, number, number], [number, number, number]]; layer?: string; unknownGroupCodes?: DxfGroupCode[]; }
export interface DxfInsertDiff { blockName?: string; position?: [number, number, number]; scale?: [number, number, number]; rotation?: number; layer?: string; unknownGroupCodes?: DxfGroupCode[]; }
export interface DxfOtherDiff { groupCodes?: DxfGroupCode[]; }

/** 🔺️ `replace` fires when the entity KIND changes at this index; otherwise a kind-specific
 * sparse field diff (the plan's json/xml "Replace on kind change" rule). */
export type DxfEntityDiff =
  | { replace: { entity: DxfEntity } }
  | { line: DxfLineDiff }
  | { circle: DxfCircleDiff }
  | { arc: DxfArcDiff }
  | { polyline: DxfPolylineDiff }
  | { text: DxfTextDiff }
  | { solid: DxfSolidDiff }
  | { insert: DxfInsertDiff }
  | { other: DxfOtherDiff };

export interface DxfEntityModified { index: number; diff: DxfEntityDiff; }
export interface DxfEntityAdded { index: number; entity: DxfEntity; }
/** 🔺️ Index-keyed triple — reused for both `DxfSnapshot.entities` and each `DxfBlock.entities`. */
export interface DxfEntitiesDiff { removed?: number[]; modified?: DxfEntityModified[]; added?: DxfEntityAdded[]; }
//#endregion

//#region BlockDiff
export interface DxfBlockDiff {
  name?: string;
  basePoint?: [number, number, number];
  entities?: DxfEntitiesDiff;
  unknownGroupCodes?: DxfGroupCode[];
}
export interface DxfBlockModified { index: number; diff: DxfBlockDiff; }
export interface DxfBlockAdded { index: number; block: DxfBlock; }
export interface DxfBlocksDiff { removed?: number[]; modified?: DxfBlockModified[]; added?: DxfBlockAdded[]; }
//#endregion

/** 🔺️ Diff for `stdio.dxf`. `schema` is an identity field and never appears here. */
export interface DxfDiff {
  headerVars?: DxfHeaderVarsDiff;
  tables?: DxfTablesDiff;
  blocks?: DxfBlocksDiff;
  entities?: DxfEntitiesDiff;
  otherTables?: DxfOtherTable[];
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioDxfR12HeaderDiffGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioDxfR12HeaderDiffGuardReject = (at: string, why: string): never => {
  throw new stdioDxfR12HeaderDiffGuardRefusal(at, why);
};

type stdioDxfR12HeaderDiffGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioDxfR12HeaderDiffGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioDxfR12HeaderDiffGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioDxfR12HeaderDiffGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioDxfR12HeaderDiffGuardReject(at, "value is not an object");
export const stdioDxfR12HeaderDiffGuardArray = (value: unknown, at: string, bounds: stdioDxfR12HeaderDiffGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioDxfR12HeaderDiffGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioDxfR12HeaderDiffGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioDxfR12HeaderDiffGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioDxfR12HeaderDiffGuardString = (value: unknown, at: string, bounds: stdioDxfR12HeaderDiffGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioDxfR12HeaderDiffGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioDxfR12HeaderDiffGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioDxfR12HeaderDiffGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioDxfR12HeaderDiffGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioDxfR12HeaderDiffGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioDxfR12HeaderDiffGuardReject(at, "value is not a boolean"));
export const stdioDxfR12HeaderDiffGuardNumber = (value: unknown, at: string, bounds: stdioDxfR12HeaderDiffGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioDxfR12HeaderDiffGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioDxfR12HeaderDiffGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioDxfR12HeaderDiffGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioDxfR12HeaderDiffGuardInteger = (value: unknown, at: string, bounds: stdioDxfR12HeaderDiffGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioDxfR12HeaderDiffGuardNumber(value, at, bounds) : stdioDxfR12HeaderDiffGuardReject(at, "value is not an integer");
export const stdioDxfR12HeaderDiffGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioDxfR12HeaderDiffGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioDxfR12HeaderDiffGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioDxfR12HeaderDiffGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseDxfDiff(value: unknown, at = "$"): DxfDiff {
  const row = stdioDxfR12HeaderDiffGuardObject(value, at);
  return {
    headerVars: row["headerVars"] === undefined ? undefined : parseDxfHeaderVarsDiff(row["headerVars"], `${at}.headerVars`),
    tables: row["tables"] === undefined ? undefined : parseDxfTablesDiff(row["tables"], `${at}.tables`),
    blocks: row["blocks"] === undefined ? undefined : parseDxfBlocksDiff(row["blocks"], `${at}.blocks`),
    entities: row["entities"] === undefined ? undefined : parseDxfEntitiesDiff(row["entities"], `${at}.entities`),
    otherTables: row["otherTables"] === undefined ? undefined : stdioDxfR12HeaderDiffGuardArray(row["otherTables"], `${at}.otherTables`).map((item, index) => parseDxfOtherTable(item, `${at}.otherTables[${index}]`)),
  };
}

export function parseDxfTablesDiff(value: unknown, at = "$"): DxfTablesDiff {
  const row = stdioDxfR12HeaderDiffGuardObject(value, at);
  return {
    layers: row["layers"] === undefined ? undefined : parseDxfLayersDiff(row["layers"], `${at}.layers`),
    styles: row["styles"] === undefined ? undefined : parseDxfStylesDiff(row["styles"], `${at}.styles`),
    linetypes: row["linetypes"] === undefined ? undefined : parseDxfLinetypesDiff(row["linetypes"], `${at}.linetypes`),
  };
}

export function parseDxfBlockDiff(value: unknown, at = "$"): DxfBlockDiff {
  const row = stdioDxfR12HeaderDiffGuardObject(value, at);
  return {
    name: row["name"] === undefined ? undefined : stdioDxfR12HeaderDiffGuardString(row["name"], `${at}.name`),
    basePoint: row["basePoint"] === undefined ? undefined : ((items0) => [stdioDxfR12HeaderDiffGuardNumber(items0[0], `${at}.basePoint[0]`), stdioDxfR12HeaderDiffGuardNumber(items0[1], `${at}.basePoint[1]`), stdioDxfR12HeaderDiffGuardNumber(items0[2], `${at}.basePoint[2]`)])(stdioDxfR12HeaderDiffGuardArray(row["basePoint"], `${at}.basePoint`, {"minItems": 3, "maxItems": 3})),
    entities: row["entities"] === undefined ? undefined : parseDxfEntitiesDiff(row["entities"], `${at}.entities`),
    unknownGroupCodes: row["unknownGroupCodes"] === undefined ? undefined : stdioDxfR12HeaderDiffGuardArray(row["unknownGroupCodes"], `${at}.unknownGroupCodes`).map((item, index) => parseDxfGroupCode(item, `${at}.unknownGroupCodes[${index}]`)),
  };
}

export function parseDxfBlocksDiff(value: unknown, at = "$"): DxfBlocksDiff {
  const row = stdioDxfR12HeaderDiffGuardObject(value, at);
  return {
    removed: row["removed"] === undefined ? undefined : stdioDxfR12HeaderDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => stdioDxfR12HeaderDiffGuardInteger(item, `${at}.removed[${index}]`)),
    modified: row["modified"] === undefined ? undefined : stdioDxfR12HeaderDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => ((row1) => ({ index: stdioDxfR12HeaderDiffGuardInteger(row1["index"], `${at}.modified[${index}].index`), diff: parseDxfBlockDiff(row1["diff"], `${at}.modified[${index}].diff`) }))(stdioDxfR12HeaderDiffGuardObject(item, `${at}.modified[${index}]`))),
    added: row["added"] === undefined ? undefined : stdioDxfR12HeaderDiffGuardArray(row["added"], `${at}.added`).map((item, index) => ((row1) => ({ index: stdioDxfR12HeaderDiffGuardInteger(row1["index"], `${at}.added[${index}].index`), block: parseDxfBlock(row1["block"], `${at}.added[${index}].block`) }))(stdioDxfR12HeaderDiffGuardObject(item, `${at}.added[${index}]`))),
  };
}

export function parseDxfEntitiesDiff(value: unknown, at = "$"): DxfEntitiesDiff {
  const row = stdioDxfR12HeaderDiffGuardObject(value, at);
  return {
    removed: row["removed"] === undefined ? undefined : stdioDxfR12HeaderDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => stdioDxfR12HeaderDiffGuardInteger(item, `${at}.removed[${index}]`)),
    modified: row["modified"] === undefined ? undefined : stdioDxfR12HeaderDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => ((row1) => ({ index: stdioDxfR12HeaderDiffGuardInteger(row1["index"], `${at}.modified[${index}].index`), diff: parseDxfEntityDiff(row1["diff"], `${at}.modified[${index}].diff`) }))(stdioDxfR12HeaderDiffGuardObject(item, `${at}.modified[${index}]`))),
    added: row["added"] === undefined ? undefined : stdioDxfR12HeaderDiffGuardArray(row["added"], `${at}.added`).map((item, index) => ((row1) => ({ index: stdioDxfR12HeaderDiffGuardInteger(row1["index"], `${at}.added[${index}].index`), entity: parseDxfEntity(row1["entity"], `${at}.added[${index}].entity`) }))(stdioDxfR12HeaderDiffGuardObject(item, `${at}.added[${index}]`))),
  };
}

export function parseDxfEntityDiff(value: unknown, at = "$"): DxfEntityDiff {
  return ((row0) => { if (row0["replace"] !== undefined) return { replace: ((row1) => ({ entity: parseDxfEntity(row1["entity"], `${at}.replace.entity`) }))(stdioDxfR12HeaderDiffGuardObject(row0["replace"], `${at}.replace`)) }; if (row0["line"] !== undefined) return { line: stdioDxfR12HeaderDiffGuardObject(row0["line"], `${at}.line`) }; if (row0["circle"] !== undefined) return { circle: stdioDxfR12HeaderDiffGuardObject(row0["circle"], `${at}.circle`) }; if (row0["arc"] !== undefined) return { arc: stdioDxfR12HeaderDiffGuardObject(row0["arc"], `${at}.arc`) }; if (row0["polyline"] !== undefined) return { polyline: stdioDxfR12HeaderDiffGuardObject(row0["polyline"], `${at}.polyline`) }; if (row0["text"] !== undefined) return { text: stdioDxfR12HeaderDiffGuardObject(row0["text"], `${at}.text`) }; if (row0["solid"] !== undefined) return { solid: stdioDxfR12HeaderDiffGuardObject(row0["solid"], `${at}.solid`) }; if (row0["insert"] !== undefined) return { insert: stdioDxfR12HeaderDiffGuardObject(row0["insert"], `${at}.insert`) }; if (row0["other"] !== undefined) return { other: stdioDxfR12HeaderDiffGuardObject(row0["other"], `${at}.other`) }; return stdioDxfR12HeaderDiffGuardReject(`${at}`, "value matches no variant"); })(stdioDxfR12HeaderDiffGuardObject(value, `${at}`));
}

export function parseDxfHeaderVarDiff(value: unknown, at = "$"): DxfHeaderVarDiff {
  const row = stdioDxfR12HeaderDiffGuardObject(value, at);
  return {
    groupCode: row["groupCode"] === undefined ? undefined : stdioDxfR12HeaderDiffGuardInteger(row["groupCode"], `${at}.groupCode`),
    value: row["value"] === undefined ? undefined : parseDxfValue(row["value"], `${at}.value`),
    extraGroupCodes: row["extraGroupCodes"] === undefined ? undefined : stdioDxfR12HeaderDiffGuardArray(row["extraGroupCodes"], `${at}.extraGroupCodes`).map((item, index) => parseDxfGroupCode(item, `${at}.extraGroupCodes[${index}]`)),
  };
}

export function parseDxfHeaderVarsDiff(value: unknown, at = "$"): DxfHeaderVarsDiff {
  const row = stdioDxfR12HeaderDiffGuardObject(value, at);
  return {
    removed: row["removed"] === undefined ? undefined : stdioDxfR12HeaderDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => stdioDxfR12HeaderDiffGuardString(item, `${at}.removed[${index}]`)),
    modified: row["modified"] === undefined ? undefined : stdioDxfR12HeaderDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => ((row1) => ({ name: stdioDxfR12HeaderDiffGuardString(row1["name"], `${at}.modified[${index}].name`), diff: parseDxfHeaderVarDiff(row1["diff"], `${at}.modified[${index}].diff`) }))(stdioDxfR12HeaderDiffGuardObject(item, `${at}.modified[${index}]`))),
    added: row["added"] === undefined ? undefined : stdioDxfR12HeaderDiffGuardArray(row["added"], `${at}.added`).map((item, index) => ((row1) => ({ index: stdioDxfR12HeaderDiffGuardInteger(row1["index"], `${at}.added[${index}].index`), headerVar: parseDxfHeaderVar(row1["headerVar"], `${at}.added[${index}].headerVar`) }))(stdioDxfR12HeaderDiffGuardObject(item, `${at}.added[${index}]`))),
  };
}

export function parseDxfLayerDiff(value: unknown, at = "$"): DxfLayerDiff {
  const row = stdioDxfR12HeaderDiffGuardObject(value, at);
  return {
    color: row["color"] === undefined ? undefined : stdioDxfR12HeaderDiffGuardInteger(row["color"], `${at}.color`),
    linetype: row["linetype"] === undefined ? undefined : stdioDxfR12HeaderDiffGuardString(row["linetype"], `${at}.linetype`),
    flags: row["flags"] === undefined ? undefined : stdioDxfR12HeaderDiffGuardInteger(row["flags"], `${at}.flags`),
    unknownGroupCodes: row["unknownGroupCodes"] === undefined ? undefined : stdioDxfR12HeaderDiffGuardArray(row["unknownGroupCodes"], `${at}.unknownGroupCodes`).map((item, index) => parseDxfGroupCode(item, `${at}.unknownGroupCodes[${index}]`)),
  };
}

export function parseDxfLayersDiff(value: unknown, at = "$"): DxfLayersDiff {
  const row = stdioDxfR12HeaderDiffGuardObject(value, at);
  return {
    removed: row["removed"] === undefined ? undefined : stdioDxfR12HeaderDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => stdioDxfR12HeaderDiffGuardString(item, `${at}.removed[${index}]`)),
    modified: row["modified"] === undefined ? undefined : stdioDxfR12HeaderDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => ((row1) => ({ name: stdioDxfR12HeaderDiffGuardString(row1["name"], `${at}.modified[${index}].name`), diff: parseDxfLayerDiff(row1["diff"], `${at}.modified[${index}].diff`) }))(stdioDxfR12HeaderDiffGuardObject(item, `${at}.modified[${index}]`))),
    added: row["added"] === undefined ? undefined : stdioDxfR12HeaderDiffGuardArray(row["added"], `${at}.added`).map((item, index) => ((row1) => ({ index: stdioDxfR12HeaderDiffGuardInteger(row1["index"], `${at}.added[${index}].index`), layer: parseDxfLayer(row1["layer"], `${at}.added[${index}].layer`) }))(stdioDxfR12HeaderDiffGuardObject(item, `${at}.added[${index}]`))),
  };
}

export function parseDxfLinetypeDiff(value: unknown, at = "$"): DxfLinetypeDiff {
  const row = stdioDxfR12HeaderDiffGuardObject(value, at);
  return {
    flags: row["flags"] === undefined ? undefined : stdioDxfR12HeaderDiffGuardInteger(row["flags"], `${at}.flags`),
    description: row["description"] === undefined ? undefined : stdioDxfR12HeaderDiffGuardString(row["description"], `${at}.description`),
    unknownGroupCodes: row["unknownGroupCodes"] === undefined ? undefined : stdioDxfR12HeaderDiffGuardArray(row["unknownGroupCodes"], `${at}.unknownGroupCodes`).map((item, index) => parseDxfGroupCode(item, `${at}.unknownGroupCodes[${index}]`)),
  };
}

export function parseDxfLinetypesDiff(value: unknown, at = "$"): DxfLinetypesDiff {
  const row = stdioDxfR12HeaderDiffGuardObject(value, at);
  return {
    removed: row["removed"] === undefined ? undefined : stdioDxfR12HeaderDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => stdioDxfR12HeaderDiffGuardString(item, `${at}.removed[${index}]`)),
    modified: row["modified"] === undefined ? undefined : stdioDxfR12HeaderDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => ((row1) => ({ name: stdioDxfR12HeaderDiffGuardString(row1["name"], `${at}.modified[${index}].name`), diff: parseDxfLinetypeDiff(row1["diff"], `${at}.modified[${index}].diff`) }))(stdioDxfR12HeaderDiffGuardObject(item, `${at}.modified[${index}]`))),
    added: row["added"] === undefined ? undefined : stdioDxfR12HeaderDiffGuardArray(row["added"], `${at}.added`).map((item, index) => ((row1) => ({ index: stdioDxfR12HeaderDiffGuardInteger(row1["index"], `${at}.added[${index}].index`), linetype: parseDxfLinetype(row1["linetype"], `${at}.added[${index}].linetype`) }))(stdioDxfR12HeaderDiffGuardObject(item, `${at}.added[${index}]`))),
  };
}

export function parseDxfStyleDiff(value: unknown, at = "$"): DxfStyleDiff {
  const row = stdioDxfR12HeaderDiffGuardObject(value, at);
  return {
    flags: row["flags"] === undefined ? undefined : stdioDxfR12HeaderDiffGuardInteger(row["flags"], `${at}.flags`),
    fontName: row["fontName"] === undefined ? undefined : stdioDxfR12HeaderDiffGuardString(row["fontName"], `${at}.fontName`),
    unknownGroupCodes: row["unknownGroupCodes"] === undefined ? undefined : stdioDxfR12HeaderDiffGuardArray(row["unknownGroupCodes"], `${at}.unknownGroupCodes`).map((item, index) => parseDxfGroupCode(item, `${at}.unknownGroupCodes[${index}]`)),
  };
}

export function parseDxfStylesDiff(value: unknown, at = "$"): DxfStylesDiff {
  const row = stdioDxfR12HeaderDiffGuardObject(value, at);
  return {
    removed: row["removed"] === undefined ? undefined : stdioDxfR12HeaderDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => stdioDxfR12HeaderDiffGuardString(item, `${at}.removed[${index}]`)),
    modified: row["modified"] === undefined ? undefined : stdioDxfR12HeaderDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => ((row1) => ({ name: stdioDxfR12HeaderDiffGuardString(row1["name"], `${at}.modified[${index}].name`), diff: parseDxfStyleDiff(row1["diff"], `${at}.modified[${index}].diff`) }))(stdioDxfR12HeaderDiffGuardObject(item, `${at}.modified[${index}]`))),
    added: row["added"] === undefined ? undefined : stdioDxfR12HeaderDiffGuardArray(row["added"], `${at}.added`).map((item, index) => ((row1) => ({ index: stdioDxfR12HeaderDiffGuardInteger(row1["index"], `${at}.added[${index}].index`), style: parseDxfStyle(row1["style"], `${at}.added[${index}].style`) }))(stdioDxfR12HeaderDiffGuardObject(item, `${at}.added[${index}]`))),
  };
}
