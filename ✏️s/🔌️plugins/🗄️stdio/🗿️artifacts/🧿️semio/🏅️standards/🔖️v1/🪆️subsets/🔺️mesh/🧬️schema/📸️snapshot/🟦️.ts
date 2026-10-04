/** 🧬️ SemioMeshSnapshot schema — real mirror of `🦀️.rs` (the source of truth). */
export type SemioTopology = "points" | "lines" | "lineStrip" | "triangles" | "triangleStrip" | "triangleFan";

import type {SemioPoint3,SemioUv,SemioRgba} from "../../../✉️base/🧬️schema/🧮️geometry/🟦️.ts";
import type {Binary32} from "../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔢️ieee754/🟦️.ts";
export type {SemioPoint3,SemioUv,SemioRgba} from "../../../✉️base/🧬️schema/🧮️geometry/🟦️.ts";

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
  metallic: Binary32;
  roughness: Binary32;
  baseColorTexture?: string | null;
  metallicRoughnessTexture?: string | null;
  normalTexture?: string | null;
  occlusionTexture?: string | null;
  emissiveTexture?: string | null;
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
