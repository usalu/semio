/** 🧬️ ObjSnapshot schema facet — mirrors 🦀️component.rs field-for-field. Complete per the
 * Wavefront OBJ 3.0 spec's real, commonly-implemented grammar. */

/** 📍 A `v` position line: x y z [w] (w default 1.0 when omitted, undefined = source omitted it). */
export interface ObjVertex { x: number; y: number; z: number; w?: number; }
/** 🧵 A `vt` texture-coordinate line: u [v] [w]. */
export interface ObjTexCoord { u: number; v: number; w?: number; }
/** 📐 A `vn` normal line: always 3 components. */
export interface ObjNormal { x: number; y: number; z: number; }
/** 🔗 One `v[/vt][/vn]` reference inside an `f` line (0-based). */
export interface ObjFaceVertex { vertex: number; texcoord?: number; normal?: number; }
/** 🧩 A `f` line, kept as its original n-gon. */
export interface ObjFace { vertices: ObjFaceVertex[]; }
/** 🏷️ A named `g` group — face-index membership list (a face may be in several groups at once). */
export interface ObjGroup { name: string; faces: number[]; }
/** 🏷️ A named `o` object — face-index membership list (exactly one object active at a time). */
export interface ObjObject { name: string; faces: number[]; }
/** 🎨 One `usemtl` transition: material active from faceIndexFrom onward. */
export interface ObjUsemtlRange { faceIndexFrom: number; material: string; }
/** 🧵 One `s` transition: smoothing group active from faceIndexFrom onward (undefined group = `s off`). */
export interface ObjSmoothingRange { faceIndexFrom: number; group?: number; }
/** 🕳️ A real source line the codec doesn't otherwise model (comments + unrecognized keywords),
 * retained verbatim in original relative order. */
export interface ObjUnknownStatement { lineIndex: number; raw: string; }

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
