/** 🧬️ SemioMeshArtifact schema — full artifact state, mirrors `SemioMeshSnapshot` field for
 * field (see `📸️snapshot/🟦️component.ts` for the nested `SemioMesh`/`SemioMaterial`/`SemioTexture`
 * shapes). */
import type { SemioMesh, SemioMaterial, SemioTexture } from "./📸️snapshot/🟦️component";

export interface SemioMeshArtifact {
  /** @state persistent */ schema: string;
  /** @state persistent */ meshes: SemioMesh[];
  /** @state persistent */ materials: SemioMaterial[];
  /** @state persistent */ textures: SemioTexture[];
}
