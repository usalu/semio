import { parseBinary64, type Binary64 } from "../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔢️ieee754/🟦️.ts";
/** 🧬️ ObjSnapshot schema facet — mirrors 🦀️.rs field-for-field. Complete per the
 * Wavefront OBJ 3.0 spec's real, commonly-implemented grammar. */

/** 📍 A `v` position line: x y z [w] (w default 1.0 when omitted, undefined = source omitted it). */
export interface ObjVertex { x: Binary64; y: Binary64; z: Binary64; w?: Binary64; }
/** 🧵 A `vt` texture-coordinate line: u [v] [w]. */
export interface ObjTexCoord { u: Binary64; v: Binary64; w?: Binary64; }
/** 📐 A `vn` normal line: always 3 components. */
export interface ObjNormal { x: Binary64; y: Binary64; z: Binary64; }
/** 🔗 One `v[/vt][/vn]` reference inside an `f` line (0-based). */
export interface ObjFaceVertex { vertex: number; texcoord?: number; normal?: number; }
/** 🧩 A `f` line, kept as its original n-gon. */
export interface ObjFace { vertices: ObjFaceVertex[]; }
/** 🏷️ A named `g` group — face-index membership list (a face may be in several groups at once). */
export interface ObjGroup { name: string; faces: bigint[]; }
/** 🏷️ A named `o` object — face-index membership list (exactly one object active at a time). */
export interface ObjObject { name: string; faces: bigint[]; }
/** 🎨 One `usemtl` transition: material active from faceIndexFrom onward. */
export interface ObjUsemtlRange { faceIndexFrom: bigint; material: string; }
/** 🧵 One `s` transition: smoothing group active from faceIndexFrom onward (undefined group = `s off`). */
export interface ObjSmoothingRange { faceIndexFrom: bigint; group?: number; }
/** 🕳️ A real source line the codec doesn't otherwise model (comments + unrecognized keywords),
 * retained verbatim in original relative order. */
export interface ObjUnknownStatement { lineIndex: bigint; raw: string; }

/** 📸️ Persisted `stdio.obj` snapshot. */
export interface ObjSnapshot {
  schema: string;
  vertices: ObjVertex[];
  texcoords: ObjTexCoord[];
  normals: ObjNormal[];
  faces: ObjFace[];
  groups: ObjGroup[];
  objects: ObjObject[];
  mtllib?: string;
  usemtl: ObjUsemtlRange[];
  smoothingGroups: ObjSmoothingRange[];
  unknownStatements: ObjUnknownStatement[];
}
