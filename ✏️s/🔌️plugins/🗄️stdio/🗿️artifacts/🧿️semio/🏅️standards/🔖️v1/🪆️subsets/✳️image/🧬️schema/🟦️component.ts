/** 🧬️ SemioImageArtifact schema — real facet mirror of the Rust `🦀️component.rs` sibling; full
 * artifact state, mirrors `SemioImageSnapshot` field for field. */
import type { SemioColorspace, SemioImageFrame, SemioImageMetadataEntry } from "./📸️snapshot/🟦️component.ts";

export interface SemioImageArtifact {
  /** @state persistent */ schema: string;
  /** @state persistent */ width: number;
  /** @state persistent */ height: number;
  /** @state persistent */ colorspace: SemioColorspace;
  /** @state persistent */ bitDepth: number;
  /** @state persistent */ frames: SemioImageFrame[];
  /** @state persistent */ icc: string | null;
  /** @state persistent */ metadata: SemioImageMetadataEntry[];
}
