/** 🧬️ SemioModelArtifact schema — full artifact state, mirrors `SemioModelSnapshot` field for field. */
import type { SpatialNode, SemioModelElement, ModelRelation } from "./📸️snapshot/🟦️component.ts";

export interface SemioModelArtifact {
  /** @state artifact */ schema: string;
  /** @state artifact */ spatial: SpatialNode[];
  /** @state artifact */ elements: SemioModelElement[];
  /** @state artifact */ relations: ModelRelation[];
}
