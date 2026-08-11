/** 🧬️ SemioImageSnapshot schema — real facet mirror of the Rust `🦀️component.rs` sibling. */
export type SemioColorspace = "rgb" | "rgba" | "grayscale" | "grayscaleAlpha" | "indexed";

export interface SemioImageFrame {
  delayMs: number;
  rgba8: string; // base64/hex-encoded RGBA8 bytes, row-major, width*height*4 long
}

export interface SemioImageMetadataEntry {
  key: string;
  value: string;
}

export interface SemioImageSnapshot {
  /** @state persistent */ schema: string;
  /** @state persistent */ width: number;
  /** @state persistent */ height: number;
  /** @state persistent */ colorspace: SemioColorspace;
  /** @state persistent */ bitDepth: number;
  /** @state persistent */ frames: SemioImageFrame[];
  /** @state persistent */ icc: string | null;
  /** @state persistent */ metadata: SemioImageMetadataEntry[];
}
