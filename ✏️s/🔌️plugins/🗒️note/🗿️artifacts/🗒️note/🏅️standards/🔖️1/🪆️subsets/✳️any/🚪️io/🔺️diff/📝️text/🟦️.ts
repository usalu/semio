import type { NoteDiff } from "../../../🧬️schema/🔺️diff/🟦️.ts";

/** 🔺️ note.diff text facade. `NoteDiff` never implements `store::ArtifactDsl` anywhere in the note
 * plugin — the sibling `🦀️.rs` documents this explicitly: this facet exists only to
 * register `note.diff`'s handcrafted grammar for LSP/tooling (design.md §2's `LanguagePair`: "a
 * registered grammar with no literal runtime parser backing it" is legal). There is nothing on the
 * Rust side to wire this to — parse/print for note diff text do not exist, by design, not by
 * omission. */
export function parseDsl(text: string): NoteDiff {
  throw new Error("note.diff has no text codec: NoteDiff never implements ArtifactDsl (grammar is registered for tooling only, never parsed)");
}
export function printDsl(value: NoteDiff): string {
  throw new Error("note.diff has no text codec: NoteDiff never implements ArtifactDsl (grammar is registered for tooling only, never parsed)");
}
