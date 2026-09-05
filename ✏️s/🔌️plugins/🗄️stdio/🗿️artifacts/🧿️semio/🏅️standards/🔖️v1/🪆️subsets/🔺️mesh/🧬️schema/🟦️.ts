/** 🧬️ SemioMeshArtifact schema — full artifact state, mirrors `SemioMeshSnapshot` field for
 * field (see `📸️snapshot/🟦️.ts` for the nested `SemioMesh`/`SemioMaterial`/`SemioTexture`
 * shapes). */
import type { SemioMesh, SemioMaterial, SemioTexture } from "./📸️snapshot/🟦️";

export interface SemioMeshArtifact {
  /** @state artifact */ schema: string;
  /** @state artifact */ meshes: SemioMesh[];
  /** @state artifact */ materials: SemioMaterial[];
  /** @state artifact */ textures: SemioTexture[];
}
