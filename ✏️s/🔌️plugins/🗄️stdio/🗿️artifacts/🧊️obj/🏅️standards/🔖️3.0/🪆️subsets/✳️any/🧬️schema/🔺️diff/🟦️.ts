/** 🔺️ ObjDiff schema facet — mirrors 🦀️.rs field-for-field. Handcrafted sparse
 * diff: four index-keyed recursive triples (vertices/texcoords/normals/faces), two name-keyed
 * triples (groups/objects), a tri-state scalar (mtllib), and three whole-vec-replace scalars
 * (usemtl/smoothingGroups/unknownStatements). No full-replace `snapshot` slot anywhere. */
export interface ObjFaceVertex { vertex: number; texcoord?: number; normal?: number; }
export interface ObjGroup { name: string; faces: number[]; }
export interface ObjObject { name: string; faces: number[]; }
export interface ObjUsemtlRange { faceIndexFrom: number; material: string; }
export interface ObjSmoothingRange { faceIndexFrom: number; group?: number; }
export interface ObjUnknownStatement { lineIndex: number; raw: string; }

export interface ObjVertexDiff { x?: number; y?: number; z?: number; w?: number | null; }
export interface ObjVertexModified { index: number; diff: ObjVertexDiff; }
export interface ObjVertexAdded { index: number; vertex: { x: number; y: number; z: number; w?: number }; }
export interface ObjVerticesDiff { removed: number[]; modified: ObjVertexModified[]; added: ObjVertexAdded[]; }

export interface ObjTexCoordDiff { u?: number; v?: number; w?: number | null; }
export interface ObjTexCoordModified { index: number; diff: ObjTexCoordDiff; }
export interface ObjTexCoordAdded { index: number; texcoord: { u: number; v: number; w?: number }; }
export interface ObjTexCoordsDiff { removed: number[]; modified: ObjTexCoordModified[]; added: ObjTexCoordAdded[]; }

export interface ObjNormalDiff { x?: number; y?: number; z?: number; }
export interface ObjNormalModified { index: number; diff: ObjNormalDiff; }
export interface ObjNormalAdded { index: number; normal: { x: number; y: number; z: number }; }
export interface ObjNormalsDiff { removed: number[]; modified: ObjNormalModified[]; added: ObjNormalAdded[]; }

/** `vertices` is a whole-vec-replace weak leaf (a face's own v/vt/vn reference list). */
export interface ObjFaceDiff { vertices?: ObjFaceVertex[]; }
export interface ObjFaceModified { index: number; diff: ObjFaceDiff; }
export interface ObjFaceAdded { index: number; face: { vertices: ObjFaceVertex[] }; }
export interface ObjFacesDiff { removed: number[]; modified: ObjFaceModified[]; added: ObjFaceAdded[]; }

/** `faces` is a whole-list-replace weak value (membership set) on both groups and objects. */
export interface ObjGroupDiff { faces?: number[]; }
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
