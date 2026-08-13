/** 🧬️ SemioMeshSnapshot schema — real mirror of `🦀️component.rs` (the source of truth). */
export type SemioTopology = "points" | "lines" | "lineStrip" | "triangles" | "triangleStrip" | "triangleFan";

export interface SemioPoint3 { x: number; y: number; z: number; }
export interface SemioUv { u: number; v: number; }
export interface SemioRgba { r: number; g: number; b: number; a: number; }

export interface SemioPrimitive {
  id: string;
  topology: SemioTopology;
  positions: SemioPoint3[];
  normals: SemioPoint3[];
  uvs: SemioUv[];
  colors: SemioRgba[];
  indices: number[];
  materialId: string | null;
}

export interface SemioMesh {
  id: string;
  primitives: SemioPrimitive[];
}

export interface SemioMaterial {
  id: string;
  baseColor: SemioRgba;
  metallic: number;
  roughness: number;
}

export interface SemioTexture {
  id: string;
  mime: string;
  bytes: number[];
}

export interface SemioMeshSnapshot {
  /** @state artifact */ schema: string;
  /** @state artifact */ meshes: SemioMesh[];
  /** @state artifact */ materials: SemioMaterial[];
  /** @state artifact */ textures: SemioTexture[];
}
