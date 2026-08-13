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
  /** @state artifact */ schema: string;
  /** @state artifact */ runs: SemioTextRun[];
}
