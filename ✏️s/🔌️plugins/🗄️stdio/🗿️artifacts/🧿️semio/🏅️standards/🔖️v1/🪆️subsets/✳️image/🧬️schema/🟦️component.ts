/** 🧬️ SemioImageArtifact schema — real facet mirror of the Rust `🦀️component.rs` sibling; full
 * artifact state, mirrors `SemioImageSnapshot` field for field. */
import type { SemioColorspace, SemioImageFrame, SemioImageMetadataEntry } from "./📸️snapshot/🟦️component.ts";

export interface SemioImageArtifact {
  /** @state artifact */ schema: string;
  /** @state artifact */ width: number;
  /** @state artifact */ height: number;
  /** @state artifact */ colorspace: SemioColorspace;
  /** @state artifact */ bitDepth: number;
  /** @state artifact */ frames: SemioImageFrame[];
  /** @state artifact */ icc: string | null;
  /** @state artifact */ metadata: SemioImageMetadataEntry[];
}
