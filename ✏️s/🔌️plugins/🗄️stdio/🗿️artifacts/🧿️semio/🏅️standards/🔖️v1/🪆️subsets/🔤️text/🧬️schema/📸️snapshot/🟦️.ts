/** 🧬️ SemioTextSnapshot schema — real facet mirror of the Rust `🦀️.rs` sibling. */
export type SemioTextMarkKind = "bold" | "italic" | "code" | "link";

export interface SemioTextMark {
  kind: SemioTextMarkKind;
  /** populated only when kind === "link"; empty string otherwise */
  href: string;
}

export interface SemioTextRun {
  /** BCP-47 language tag; "" = unspecified, inherits from context */
  language: string;
  content: string;
  marks: SemioTextMark[];
}

export interface SemioTextSnapshot {
  /** @state artifact */ schema: string;
  /** @state artifact */ runs: SemioTextRun[];
}
