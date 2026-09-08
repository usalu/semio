/** 🧬️ ObjArtifact schema facet — mirrors 🦀️.rs field-for-field (same shape as
 * ObjSnapshot; see 📸️snapshot/🟦️.ts for the per-field doc comments). */
export interface ObjVertex { x: number; y: number; z: number; w?: number; }
export interface ObjTexCoord { u: number; v: number; w?: number; }
export interface ObjNormal { x: number; y: number; z: number; }
export interface ObjFaceVertex { vertex: number; texcoord?: number; normal?: number; }
export interface ObjFace { vertices: ObjFaceVertex[]; }
export interface ObjGroup { name: string; faces: number[]; }
export interface ObjObject { name: string; faces: number[]; }
export interface ObjUsemtlRange { faceIndexFrom: number; material: string; }
export interface ObjSmoothingRange { faceIndexFrom: number; group?: number; }
export interface ObjUnknownStatement { lineIndex: number; raw: string; }

export interface ObjArtifact {
  /** @state artifact */ schema: string;
  /** @state artifact */ vertices: ObjVertex[];
  /** @state artifact */ texcoords: ObjTexCoord[];
  /** @state artifact */ normals: ObjNormal[];
  /** @state artifact */ faces: ObjFace[];
  /** @state artifact */ groups: ObjGroup[];
  /** @state artifact */ objects: ObjObject[];
  /** @state artifact */ mtllib?: string;
  /** @state artifact */ usemtl: ObjUsemtlRange[];
  /** @state artifact */ smoothingGroups: ObjSmoothingRange[];
  /** @state artifact */ unknownStatements: ObjUnknownStatement[];
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioObj30GeometryArtifactGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioObj30GeometryArtifactGuardReject = (at: string, why: string): never => {
  throw new stdioObj30GeometryArtifactGuardRefusal(at, why);
};

type stdioObj30GeometryArtifactGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioObj30GeometryArtifactGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioObj30GeometryArtifactGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioObj30GeometryArtifactGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioObj30GeometryArtifactGuardReject(at, "value is not an object");
export const stdioObj30GeometryArtifactGuardArray = (value: unknown, at: string, bounds: stdioObj30GeometryArtifactGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioObj30GeometryArtifactGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioObj30GeometryArtifactGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioObj30GeometryArtifactGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioObj30GeometryArtifactGuardString = (value: unknown, at: string, bounds: stdioObj30GeometryArtifactGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioObj30GeometryArtifactGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioObj30GeometryArtifactGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioObj30GeometryArtifactGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioObj30GeometryArtifactGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioObj30GeometryArtifactGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioObj30GeometryArtifactGuardReject(at, "value is not a boolean"));
export const stdioObj30GeometryArtifactGuardNumber = (value: unknown, at: string, bounds: stdioObj30GeometryArtifactGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioObj30GeometryArtifactGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioObj30GeometryArtifactGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioObj30GeometryArtifactGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioObj30GeometryArtifactGuardInteger = (value: unknown, at: string, bounds: stdioObj30GeometryArtifactGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioObj30GeometryArtifactGuardNumber(value, at, bounds) : stdioObj30GeometryArtifactGuardReject(at, "value is not an integer");
export const stdioObj30GeometryArtifactGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioObj30GeometryArtifactGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioObj30GeometryArtifactGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioObj30GeometryArtifactGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseObjArtifact(value: unknown, at = "$"): ObjArtifact {
  const row = stdioObj30GeometryArtifactGuardObject(value, at);
  return {
    schema: stdioObj30GeometryArtifactGuardString(row["schema"], `${at}.schema`),
    vertices: stdioObj30GeometryArtifactGuardArray(row["vertices"], `${at}.vertices`).map((item, index) => parseObjVertex(item, `${at}.vertices[${index}]`)),
    texcoords: stdioObj30GeometryArtifactGuardArray(row["texcoords"], `${at}.texcoords`).map((item, index) => parseObjTexCoord(item, `${at}.texcoords[${index}]`)),
    normals: stdioObj30GeometryArtifactGuardArray(row["normals"], `${at}.normals`).map((item, index) => parseObjNormal(item, `${at}.normals[${index}]`)),
    faces: stdioObj30GeometryArtifactGuardArray(row["faces"], `${at}.faces`).map((item, index) => parseObjFace(item, `${at}.faces[${index}]`)),
    groups: stdioObj30GeometryArtifactGuardArray(row["groups"], `${at}.groups`).map((item, index) => parseObjGroup(item, `${at}.groups[${index}]`)),
    objects: stdioObj30GeometryArtifactGuardArray(row["objects"], `${at}.objects`).map((item, index) => parseObjObject(item, `${at}.objects[${index}]`)),
    mtllib: row["mtllib"] === undefined ? undefined : stdioObj30GeometryArtifactGuardString(row["mtllib"], `${at}.mtllib`),
    usemtl: stdioObj30GeometryArtifactGuardArray(row["usemtl"], `${at}.usemtl`).map((item, index) => parseObjUsemtlRange(item, `${at}.usemtl[${index}]`)),
    smoothingGroups: stdioObj30GeometryArtifactGuardArray(row["smoothingGroups"], `${at}.smoothingGroups`).map((item, index) => parseObjSmoothingRange(item, `${at}.smoothingGroups[${index}]`)),
    unknownStatements: stdioObj30GeometryArtifactGuardArray(row["unknownStatements"], `${at}.unknownStatements`).map((item, index) => parseObjUnknownStatement(item, `${at}.unknownStatements[${index}]`)),
  };
}

export function parseObjVertex(value: unknown, at = "$"): ObjVertex {
  const row = stdioObj30GeometryArtifactGuardObject(value, at);
  return {
    x: stdioObj30GeometryArtifactGuardNumber(row["x"], `${at}.x`),
    y: stdioObj30GeometryArtifactGuardNumber(row["y"], `${at}.y`),
    z: stdioObj30GeometryArtifactGuardNumber(row["z"], `${at}.z`),
    w: row["w"] === undefined ? undefined : stdioObj30GeometryArtifactGuardNumber(row["w"], `${at}.w`),
  };
}

export function parseObjTexCoord(value: unknown, at = "$"): ObjTexCoord {
  const row = stdioObj30GeometryArtifactGuardObject(value, at);
  return {
    u: stdioObj30GeometryArtifactGuardNumber(row["u"], `${at}.u`),
    v: stdioObj30GeometryArtifactGuardNumber(row["v"], `${at}.v`),
    w: row["w"] === undefined ? undefined : stdioObj30GeometryArtifactGuardNumber(row["w"], `${at}.w`),
  };
}

export function parseObjNormal(value: unknown, at = "$"): ObjNormal {
  const row = stdioObj30GeometryArtifactGuardObject(value, at);
  return {
    x: stdioObj30GeometryArtifactGuardNumber(row["x"], `${at}.x`),
    y: stdioObj30GeometryArtifactGuardNumber(row["y"], `${at}.y`),
    z: stdioObj30GeometryArtifactGuardNumber(row["z"], `${at}.z`),
  };
}

export function parseObjFaceVertex(value: unknown, at = "$"): ObjFaceVertex {
  const row = stdioObj30GeometryArtifactGuardObject(value, at);
  return {
    vertex: stdioObj30GeometryArtifactGuardInteger(row["vertex"], `${at}.vertex`, {"minimum": 0}),
    texcoord: row["texcoord"] === undefined ? undefined : stdioObj30GeometryArtifactGuardInteger(row["texcoord"], `${at}.texcoord`, {"minimum": 0}),
    normal: row["normal"] === undefined ? undefined : stdioObj30GeometryArtifactGuardInteger(row["normal"], `${at}.normal`, {"minimum": 0}),
  };
}

export function parseObjFace(value: unknown, at = "$"): ObjFace {
  const row = stdioObj30GeometryArtifactGuardObject(value, at);
  return {
    vertices: stdioObj30GeometryArtifactGuardArray(row["vertices"], `${at}.vertices`).map((item, index) => parseObjFaceVertex(item, `${at}.vertices[${index}]`)),
  };
}

export function parseObjGroup(value: unknown, at = "$"): ObjGroup {
  const row = stdioObj30GeometryArtifactGuardObject(value, at);
  return {
    name: stdioObj30GeometryArtifactGuardString(row["name"], `${at}.name`),
    faces: stdioObj30GeometryArtifactGuardArray(row["faces"], `${at}.faces`).map((item, index) => stdioObj30GeometryArtifactGuardInteger(item, `${at}.faces[${index}]`, {"minimum": 0})),
  };
}

export function parseObjObject(value: unknown, at = "$"): ObjObject {
  const row = stdioObj30GeometryArtifactGuardObject(value, at);
  return {
    name: stdioObj30GeometryArtifactGuardString(row["name"], `${at}.name`),
    faces: stdioObj30GeometryArtifactGuardArray(row["faces"], `${at}.faces`).map((item, index) => stdioObj30GeometryArtifactGuardInteger(item, `${at}.faces[${index}]`, {"minimum": 0})),
  };
}

export function parseObjUsemtlRange(value: unknown, at = "$"): ObjUsemtlRange {
  const row = stdioObj30GeometryArtifactGuardObject(value, at);
  return {
    faceIndexFrom: stdioObj30GeometryArtifactGuardInteger(row["faceIndexFrom"], `${at}.faceIndexFrom`, {"minimum": 0}),
    material: stdioObj30GeometryArtifactGuardString(row["material"], `${at}.material`),
  };
}

export function parseObjSmoothingRange(value: unknown, at = "$"): ObjSmoothingRange {
  const row = stdioObj30GeometryArtifactGuardObject(value, at);
  return {
    faceIndexFrom: stdioObj30GeometryArtifactGuardInteger(row["faceIndexFrom"], `${at}.faceIndexFrom`, {"minimum": 0}),
    group: row["group"] === undefined ? undefined : stdioObj30GeometryArtifactGuardInteger(row["group"], `${at}.group`, {"minimum": 0}),
  };
}

export function parseObjUnknownStatement(value: unknown, at = "$"): ObjUnknownStatement {
  const row = stdioObj30GeometryArtifactGuardObject(value, at);
  return {
    lineIndex: stdioObj30GeometryArtifactGuardInteger(row["lineIndex"], `${at}.lineIndex`, {"minimum": 0}),
    raw: stdioObj30GeometryArtifactGuardString(row["raw"], `${at}.raw`),
  };
}
