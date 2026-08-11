/** 🔺️ SemioMeshDiff schema — real mirror of `🦀️component.rs`. Collections are id-keyed
 * removed/modified/added triples (`engine::triples::NamedTripleDiff<K,D,T>`). */
import type { SemioMesh, SemioMaterial, SemioTexture, SemioPrimitive, SemioTopology, SemioPoint3, SemioUv, SemioRgba } from "../📸️snapshot/🟦️component";

export interface NamedModified<K, D> { key: K; diff: D; }
export interface NamedTripleDiff<K, D, T> { removed: K[]; modified: NamedModified<K, D>[]; added: T[]; }

export interface SemioPrimitiveDiff {
  topology?: SemioTopology;
  positions?: SemioPoint3[];
  normals?: SemioPoint3[];
  uvs?: SemioUv[];
  colors?: SemioRgba[];
  indices?: number[];
  /** tri-state: absent = unchanged, null = cleared, string = set */
  materialId?: string | null;
}

export interface SemioMeshItemDiff {
  primitives?: NamedTripleDiff<string, SemioPrimitiveDiff, SemioPrimitive>;
}

export interface SemioMaterialDiff {
  baseColor?: SemioRgba;
  metallic?: number;
  roughness?: number;
}

export interface SemioTextureDiff {
  mime?: string;
  bytes?: number[];
}

export interface SemioMeshDiff {
  meshes?: NamedTripleDiff<string, SemioMeshItemDiff, SemioMesh>;
  materials?: NamedTripleDiff<string, SemioMaterialDiff, SemioMaterial>;
  textures?: NamedTripleDiff<string, SemioTextureDiff, SemioTexture>;
}
