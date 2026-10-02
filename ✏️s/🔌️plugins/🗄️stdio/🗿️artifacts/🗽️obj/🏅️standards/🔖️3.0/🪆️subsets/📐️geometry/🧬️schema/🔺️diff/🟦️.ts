import { parseBinary64, type Binary64 } from "../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔢️ieee754/🟦️.ts";
/** 🔺️ ObjDiff schema facet — mirrors 🦀️.rs field-for-field. Handcrafted sparse
 * diff: four index-keyed recursive triples (vertices/texcoords/normals/faces), two name-keyed
 * triples (groups/objects), a tri-state scalar (mtllib), and three whole-vec-replace scalars
 * (usemtl/smoothingGroups/unknownStatements). No full-replace `snapshot` slot anywhere. */
import { parseObjSourceIndex,parseObjVertex, parseObjTexCoord, parseObjNormal, parseObjFace, parseObjGroup, parseObjObject } from "../🟦️.ts";

export interface ObjFaceVertex { vertex: number; texcoord?: number; normal?: number; }
export interface ObjGroup { name: string; faces: bigint[]; }
export interface ObjObject { name: string; faces: bigint[]; }
export interface ObjUsemtlRange { faceIndexFrom: bigint; material: string; }
export interface ObjSmoothingRange { faceIndexFrom: bigint; group?: number; }
export interface ObjUnknownStatement { lineIndex: bigint; raw: string; }

export interface ObjVertexDiff { x?: Binary64; y?: Binary64; z?: Binary64; w?: Binary64 | null; }
export interface ObjVertexModified { index: number; diff: ObjVertexDiff; }
export interface ObjVertexAdded { index: number; vertex: { x: Binary64; y: Binary64; z: Binary64; w?: Binary64 }; }
export interface ObjVerticesDiff { removed: number[]; modified: ObjVertexModified[]; added: ObjVertexAdded[]; }

export interface ObjTexCoordDiff { u?: Binary64; v?: Binary64; w?: Binary64 | null; }
export interface ObjTexCoordModified { index: number; diff: ObjTexCoordDiff; }
export interface ObjTexCoordAdded { index: number; texcoord: { u: Binary64; v: Binary64; w?: Binary64 }; }
export interface ObjTexCoordsDiff { removed: number[]; modified: ObjTexCoordModified[]; added: ObjTexCoordAdded[]; }

export interface ObjNormalDiff { x?: Binary64; y?: Binary64; z?: Binary64; }
export interface ObjNormalModified { index: number; diff: ObjNormalDiff; }
export interface ObjNormalAdded { index: number; normal: { x: Binary64; y: Binary64; z: Binary64 }; }
export interface ObjNormalsDiff { removed: number[]; modified: ObjNormalModified[]; added: ObjNormalAdded[]; }

/** `vertices` is a whole-vec-replace weak leaf (a face's own v/vt/vn reference list). */
export interface ObjFaceDiff { vertices?: ObjFaceVertex[]; }
export interface ObjFaceModified { index: number; diff: ObjFaceDiff; }
export interface ObjFaceAdded { index: number; face: { vertices: ObjFaceVertex[] }; }
export interface ObjFacesDiff { removed: number[]; modified: ObjFaceModified[]; added: ObjFaceAdded[]; }

/** `faces` is a whole-list-replace weak value (membership set) on both groups and objects. */
export interface ObjGroupDiff { faces?: bigint[]; }
export interface ObjGroupModified { name: string; diff: ObjGroupDiff; }
export interface ObjGroupAdded { index: number; group: ObjGroup; }
export interface ObjGroupsDiff { removed: string[]; modified: ObjGroupModified[]; added: ObjGroupAdded[]; }

export interface ObjObjectAdded { index: number; object: ObjObject; }
export interface ObjObjectsDiff { removed: string[]; modified: ObjGroupModified[]; added: ObjObjectAdded[]; }

/** 🔺️ Diff for stdio.obj. `schema` is an identity field and never appears here. */
export interface ObjDiff {
  vertices?: ObjVerticesDiff;
  texcoords?: ObjTexCoordsDiff;
  normals?: ObjNormalsDiff;
  faces?: ObjFacesDiff;
  groups?: ObjGroupsDiff;
  objects?: ObjObjectsDiff;
  /** tri-state: absent = unchanged, null = cleared, string = set */
  mtllib?: string | null;
  usemtl?: ObjUsemtlRange[];
  smoothingGroups?: ObjSmoothingRange[];
  unknownStatements?: ObjUnknownStatement[];
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioObj30GeometryDiffGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioObj30GeometryDiffGuardReject = (at: string, why: string): never => {
  throw new stdioObj30GeometryDiffGuardRefusal(at, why);
};

type stdioObj30GeometryDiffGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioObj30GeometryDiffGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioObj30GeometryDiffGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioObj30GeometryDiffGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioObj30GeometryDiffGuardReject(at, "value is not an object");
export const stdioObj30GeometryDiffGuardArray = (value: unknown, at: string, bounds: stdioObj30GeometryDiffGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioObj30GeometryDiffGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioObj30GeometryDiffGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioObj30GeometryDiffGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioObj30GeometryDiffGuardString = (value: unknown, at: string, bounds: stdioObj30GeometryDiffGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioObj30GeometryDiffGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioObj30GeometryDiffGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioObj30GeometryDiffGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioObj30GeometryDiffGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioObj30GeometryDiffGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioObj30GeometryDiffGuardReject(at, "value is not a boolean"));
export const stdioObj30GeometryDiffGuardNumber = (value: unknown, at: string, bounds: stdioObj30GeometryDiffGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioObj30GeometryDiffGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioObj30GeometryDiffGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioObj30GeometryDiffGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioObj30GeometryDiffGuardInteger = (value: unknown, at: string, bounds: stdioObj30GeometryDiffGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioObj30GeometryDiffGuardNumber(value, at, bounds) : stdioObj30GeometryDiffGuardReject(at, "value is not an integer");
export const stdioObj30GeometryDiffGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioObj30GeometryDiffGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioObj30GeometryDiffGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioObj30GeometryDiffGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseObjFaceVertex(value: unknown, at = "$"): ObjFaceVertex {
  const row = stdioObj30GeometryDiffGuardObject(value, at);
  return {
    vertex: stdioObj30GeometryDiffGuardInteger(row["vertex"], `${at}.vertex`, {minimum:0,maximum:4294967295}),
    texcoord: row["texcoord"] === undefined ? undefined : stdioObj30GeometryDiffGuardInteger(row["texcoord"], `${at}.texcoord`, {minimum:0,maximum:4294967295}),
    normal: row["normal"] === undefined ? undefined : stdioObj30GeometryDiffGuardInteger(row["normal"], `${at}.normal`, {minimum:0,maximum:4294967295}),
  };
}

export function parseObjUsemtlRange(value: unknown, at = "$"): ObjUsemtlRange {
  const row = stdioObj30GeometryDiffGuardObject(value, at);
  return {
    faceIndexFrom: parseObjSourceIndex(row["faceIndexFrom"], `${at}.faceIndexFrom`),
    material: stdioObj30GeometryDiffGuardString(row["material"], `${at}.material`),
  };
}

export function parseObjSmoothingRange(value: unknown, at = "$"): ObjSmoothingRange {
  const row = stdioObj30GeometryDiffGuardObject(value, at);
  return {
    faceIndexFrom: parseObjSourceIndex(row["faceIndexFrom"], `${at}.faceIndexFrom`),
    group: row["group"] === undefined ? undefined : stdioObj30GeometryDiffGuardInteger(row["group"], `${at}.group`, {minimum:0,maximum:4294967295}),
  };
}

export function parseObjUnknownStatement(value: unknown, at = "$"): ObjUnknownStatement {
  const row = stdioObj30GeometryDiffGuardObject(value, at);
  const lineIndex = row["lineIndex"];
  if (typeof lineIndex !== "bigint" || lineIndex < 0n || lineIndex > 18446744073709551615n) stdioObj30GeometryDiffGuardReject(`${at}.lineIndex`, "value is not an unsigned 64-bit integer");
  return {
    lineIndex: lineIndex as bigint,
    raw: stdioObj30GeometryDiffGuardString(row["raw"], `${at}.raw`),
  };
}

export function parseObjVertexModified(value: unknown, at = "$"): ObjVertexModified {
  const row = stdioObj30GeometryDiffGuardObject(value, at);
  return {
    index: stdioObj30GeometryDiffGuardInteger(row["index"], `${at}.index`),
    diff: parseObjVertexDiff(row["diff"], `${at}.diff`),
  };
}

export function parseObjVertexAdded(value: unknown, at = "$"): ObjVertexAdded {
  const row = stdioObj30GeometryDiffGuardObject(value, at);
  return {
    index: stdioObj30GeometryDiffGuardInteger(row["index"], `${at}.index`),
    vertex: parseObjVertex(row["vertex"], `${at}.vertex`),
  };
}

export function parseObjVerticesDiff(value: unknown, at = "$"): ObjVerticesDiff {
  const row = stdioObj30GeometryDiffGuardObject(value, at);
  return {
    removed: row["removed"] === undefined ? [] : stdioObj30GeometryDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => stdioObj30GeometryDiffGuardInteger(item, `${at}.removed[${index}]`)),
    modified: row["modified"] === undefined ? [] : stdioObj30GeometryDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseObjVertexModified(item, `${at}.modified[${index}]`)),
    added: row["added"] === undefined ? [] : stdioObj30GeometryDiffGuardArray(row["added"], `${at}.added`).map((item, index) => parseObjVertexAdded(item, `${at}.added[${index}]`)),
  };
}

export function parseObjTexCoordModified(value: unknown, at = "$"): ObjTexCoordModified {
  const row = stdioObj30GeometryDiffGuardObject(value, at);
  return {
    index: stdioObj30GeometryDiffGuardInteger(row["index"], `${at}.index`),
    diff: parseObjTexCoordDiff(row["diff"], `${at}.diff`),
  };
}

export function parseObjTexCoordAdded(value: unknown, at = "$"): ObjTexCoordAdded {
  const row = stdioObj30GeometryDiffGuardObject(value, at);
  return {
    index: stdioObj30GeometryDiffGuardInteger(row["index"], `${at}.index`),
    texcoord: parseObjTexCoord(row["texcoord"], `${at}.texcoord`),
  };
}

export function parseObjTexCoordsDiff(value: unknown, at = "$"): ObjTexCoordsDiff {
  const row = stdioObj30GeometryDiffGuardObject(value, at);
  return {
    removed: row["removed"] === undefined ? [] : stdioObj30GeometryDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => stdioObj30GeometryDiffGuardInteger(item, `${at}.removed[${index}]`)),
    modified: row["modified"] === undefined ? [] : stdioObj30GeometryDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseObjTexCoordModified(item, `${at}.modified[${index}]`)),
    added: row["added"] === undefined ? [] : stdioObj30GeometryDiffGuardArray(row["added"], `${at}.added`).map((item, index) => parseObjTexCoordAdded(item, `${at}.added[${index}]`)),
  };
}

export function parseObjNormalDiff(value: unknown, at = "$"): ObjNormalDiff {
  const row = stdioObj30GeometryDiffGuardObject(value, at);
  return {
    x: row["x"] === undefined ? undefined : parseBinary64(row["x"]),
    y: row["y"] === undefined ? undefined : parseBinary64(row["y"]),
    z: row["z"] === undefined ? undefined : parseBinary64(row["z"]),
  };
}

export function parseObjNormalModified(value: unknown, at = "$"): ObjNormalModified {
  const row = stdioObj30GeometryDiffGuardObject(value, at);
  return {
    index: stdioObj30GeometryDiffGuardInteger(row["index"], `${at}.index`),
    diff: parseObjNormalDiff(row["diff"], `${at}.diff`),
  };
}

export function parseObjNormalAdded(value: unknown, at = "$"): ObjNormalAdded {
  const row = stdioObj30GeometryDiffGuardObject(value, at);
  return {
    index: stdioObj30GeometryDiffGuardInteger(row["index"], `${at}.index`),
    normal: parseObjNormal(row["normal"], `${at}.normal`),
  };
}

export function parseObjNormalsDiff(value: unknown, at = "$"): ObjNormalsDiff {
  const row = stdioObj30GeometryDiffGuardObject(value, at);
  return {
    removed: row["removed"] === undefined ? [] : stdioObj30GeometryDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => stdioObj30GeometryDiffGuardInteger(item, `${at}.removed[${index}]`)),
    modified: row["modified"] === undefined ? [] : stdioObj30GeometryDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseObjNormalModified(item, `${at}.modified[${index}]`)),
    added: row["added"] === undefined ? [] : stdioObj30GeometryDiffGuardArray(row["added"], `${at}.added`).map((item, index) => parseObjNormalAdded(item, `${at}.added[${index}]`)),
  };
}

export function parseObjFaceDiff(value: unknown, at = "$"): ObjFaceDiff {
  const row = stdioObj30GeometryDiffGuardObject(value, at);
  return {
    vertices: row["vertices"] === undefined ? undefined : stdioObj30GeometryDiffGuardArray(row["vertices"], `${at}.vertices`).map((item, index) => parseObjFaceVertex(item, `${at}.vertices[${index}]`)),
  };
}

export function parseObjFaceModified(value: unknown, at = "$"): ObjFaceModified {
  const row = stdioObj30GeometryDiffGuardObject(value, at);
  return {
    index: stdioObj30GeometryDiffGuardInteger(row["index"], `${at}.index`),
    diff: parseObjFaceDiff(row["diff"], `${at}.diff`),
  };
}

export function parseObjFaceAdded(value: unknown, at = "$"): ObjFaceAdded {
  const row = stdioObj30GeometryDiffGuardObject(value, at);
  return {
    index: stdioObj30GeometryDiffGuardInteger(row["index"], `${at}.index`),
    face: parseObjFace(row["face"], `${at}.face`),
  };
}

export function parseObjFacesDiff(value: unknown, at = "$"): ObjFacesDiff {
  const row = stdioObj30GeometryDiffGuardObject(value, at);
  return {
    removed: row["removed"] === undefined ? [] : stdioObj30GeometryDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => stdioObj30GeometryDiffGuardInteger(item, `${at}.removed[${index}]`)),
    modified: row["modified"] === undefined ? [] : stdioObj30GeometryDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseObjFaceModified(item, `${at}.modified[${index}]`)),
    added: row["added"] === undefined ? [] : stdioObj30GeometryDiffGuardArray(row["added"], `${at}.added`).map((item, index) => parseObjFaceAdded(item, `${at}.added[${index}]`)),
  };
}

export function parseObjGroupDiff(value: unknown, at = "$"): ObjGroupDiff {
  const row = stdioObj30GeometryDiffGuardObject(value, at);
  return {
    faces: row["faces"] === undefined ? undefined : stdioObj30GeometryDiffGuardArray(row["faces"], `${at}.faces`).map((item, index) => parseObjSourceIndex(item, `${at}.faces[${index}]`)),
  };
}

export function parseObjGroupModified(value: unknown, at = "$"): ObjGroupModified {
  const row = stdioObj30GeometryDiffGuardObject(value, at);
  return {
    name: stdioObj30GeometryDiffGuardString(row["name"], `${at}.name`),
    diff: parseObjGroupDiff(row["diff"], `${at}.diff`),
  };
}

export function parseObjGroupAdded(value: unknown, at = "$"): ObjGroupAdded {
  const row = stdioObj30GeometryDiffGuardObject(value, at);
  return {
    index: stdioObj30GeometryDiffGuardInteger(row["index"], `${at}.index`),
    group: parseObjGroup(row["group"], `${at}.group`),
  };
}

export function parseObjGroupsDiff(value: unknown, at = "$"): ObjGroupsDiff {
  const row = stdioObj30GeometryDiffGuardObject(value, at);
  return {
    removed: row["removed"] === undefined ? [] : stdioObj30GeometryDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => stdioObj30GeometryDiffGuardString(item, `${at}.removed[${index}]`)),
    modified: row["modified"] === undefined ? [] : stdioObj30GeometryDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseObjGroupModified(item, `${at}.modified[${index}]`)),
    added: row["added"] === undefined ? [] : stdioObj30GeometryDiffGuardArray(row["added"], `${at}.added`).map((item, index) => parseObjGroupAdded(item, `${at}.added[${index}]`)),
  };
}

export function parseObjObjectAdded(value: unknown, at = "$"): ObjObjectAdded {
  const row = stdioObj30GeometryDiffGuardObject(value, at);
  return {
    index: stdioObj30GeometryDiffGuardInteger(row["index"], `${at}.index`),
    object: parseObjObject(row["object"], `${at}.object`),
  };
}

export function parseObjObjectsDiff(value: unknown, at = "$"): ObjObjectsDiff {
  const row = stdioObj30GeometryDiffGuardObject(value, at);
  return {
    removed: row["removed"] === undefined ? [] : stdioObj30GeometryDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => stdioObj30GeometryDiffGuardString(item, `${at}.removed[${index}]`)),
    modified: row["modified"] === undefined ? [] : stdioObj30GeometryDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseObjGroupModified(item, `${at}.modified[${index}]`)),
    added: row["added"] === undefined ? [] : stdioObj30GeometryDiffGuardArray(row["added"], `${at}.added`).map((item, index) => parseObjObjectAdded(item, `${at}.added[${index}]`)),
  };
}

/** 🧮️ Parses one sparse vertex patch while retaining explicit weight clearing. */
export function parseObjVertexDiff(value: unknown, at = "$"): ObjVertexDiff {
  const row = stdioObj30GeometryDiffGuardObject(value, at);
  return {
    x: row["x"] === undefined ? undefined : parseBinary64(row["x"]),
    y: row["y"] === undefined ? undefined : parseBinary64(row["y"]),
    z: row["z"] === undefined ? undefined : parseBinary64(row["z"]),
    w: row["w"] === undefined || row["w"] === null ? row["w"] : parseBinary64(row["w"]),
  };
}

/** 🧵️ Parses one sparse texture-coordinate patch with tri-state weight presence. */
export function parseObjTexCoordDiff(value: unknown, at = "$"): ObjTexCoordDiff {
  const row = stdioObj30GeometryDiffGuardObject(value, at);
  return {
    u: row["u"] === undefined ? undefined : parseBinary64(row["u"]),
    v: row["v"] === undefined ? undefined : parseBinary64(row["v"]),
    w: row["w"] === undefined || row["w"] === null ? row["w"] : parseBinary64(row["w"]),
  };
}

/** 🔺️ Parses the owned sparse diff and exact unsigned source-position domain. */
export function parseObjDiff(value: unknown, at = "$"): ObjDiff {
  const row = stdioObj30GeometryDiffGuardObject(value, at);
  return {
    vertices: row["vertices"] === undefined ? undefined : parseObjVerticesDiff(row["vertices"], `${at}.vertices`),
    texcoords: row["texcoords"] === undefined ? undefined : parseObjTexCoordsDiff(row["texcoords"], `${at}.texcoords`),
    normals: row["normals"] === undefined ? undefined : parseObjNormalsDiff(row["normals"], `${at}.normals`),
    faces: row["faces"] === undefined ? undefined : parseObjFacesDiff(row["faces"], `${at}.faces`),
    groups: row["groups"] === undefined ? undefined : parseObjGroupsDiff(row["groups"], `${at}.groups`),
    objects: row["objects"] === undefined ? undefined : parseObjObjectsDiff(row["objects"], `${at}.objects`),
    mtllib: row["mtllib"] === undefined || row["mtllib"] === null ? row["mtllib"] : stdioObj30GeometryDiffGuardString(row["mtllib"], `${at}.mtllib`),
    usemtl: row["usemtl"] === undefined ? undefined : stdioObj30GeometryDiffGuardArray(row["usemtl"], `${at}.usemtl`).map((item, index) => parseObjUsemtlRange(item, `${at}.usemtl[${index}]`)),
    smoothingGroups: row["smoothingGroups"] === undefined ? undefined : stdioObj30GeometryDiffGuardArray(row["smoothingGroups"], `${at}.smoothingGroups`).map((item, index) => parseObjSmoothingRange(item, `${at}.smoothingGroups[${index}]`)),
    unknownStatements: row["unknownStatements"] === undefined ? undefined : stdioObj30GeometryDiffGuardArray(row["unknownStatements"], `${at}.unknownStatements`).map((item, index) => parseObjUnknownStatement(item, `${at}.unknownStatements[${index}]`)),
  };
}
