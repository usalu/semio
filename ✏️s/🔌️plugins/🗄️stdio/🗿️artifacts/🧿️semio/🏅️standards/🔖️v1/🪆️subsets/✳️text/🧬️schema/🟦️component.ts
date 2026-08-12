/** 🧬️ SemioTextArtifact schema — real facet mirror of the Rust `🦀️component.rs` sibling. */
export type SemioTextMarkKind = "bold" | "italic" | "code" | "link";
export interface SemioTextMark {
  kind: SemioTextMarkKind;
  href: string;
}
export interface SemioTextRun {
  language: string;
  content: string;
  marks: SemioTextMark[];
}
export interface SemioTextArtifact {
  /** @state persistent */ schema: string;
  /** @state persistent */ runs: SemioTextRun[];
}
