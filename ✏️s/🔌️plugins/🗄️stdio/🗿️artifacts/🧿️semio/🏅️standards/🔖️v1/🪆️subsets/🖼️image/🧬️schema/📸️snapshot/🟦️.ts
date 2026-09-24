/** 🧬️ SemioImageSnapshot schema — real facet mirror of the Rust `🦀️.rs` sibling. */
export type SemioColorspace = "rgb" | "rgba" | "grayscale" | "grayscaleAlpha" | "indexed";

export interface SemioImageFrame {
  delayMs: number;
  rgba8: number[];
}

export interface SemioImageMetadataEntry {
  key: string;
  value: string;
}

export interface SemioImageSnapshot {
  /** @state artifact */ schema: string;
  /** @state artifact */ width: number;
  /** @state artifact */ height: number;
  /** @state artifact */ colorspace: SemioColorspace;
  /** @state artifact */ bitDepth: number;
  /** @state artifact */ frames: SemioImageFrame[];
  /** @state artifact */ icc: number[] | null;
  /** @state artifact */ metadata: SemioImageMetadataEntry[];
}
