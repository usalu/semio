/** 🧬️ SemioModelArtifact schema — full artifact state, mirrors `SemioModelSnapshot` field for field. */
import type { SpatialNode, SemioModelElement, ModelRelation } from "./📸️snapshot/🟦️component.ts";

export interface SemioModelArtifact {
  /** @state persistent */ schema: string;
  /** @state persistent */ spatial: SpatialNode[];
  /** @state persistent */ elements: SemioModelElement[];
  /** @state persistent */ relations: ModelRelation[];
}
