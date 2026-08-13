/** 🧬️ ObjArtifact schema facet — mirrors 🦀️component.rs field-for-field (same shape as
 * ObjSnapshot; see 📸️snapshot/🟦️component.ts for the per-field doc comments). */
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
