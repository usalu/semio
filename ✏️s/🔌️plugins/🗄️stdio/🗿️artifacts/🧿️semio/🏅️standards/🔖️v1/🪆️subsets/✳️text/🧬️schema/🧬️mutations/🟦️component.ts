/** 🧬️ SemioTextMutation schema — real facet mirror of the Rust `🦀️component.rs` sibling. Closed,
 * seven-variant dispatch: one interface per triad payload, tagged by `mutation`. */
export type SemioTextMutation =
  | { mutation: "insertRun"; payload: { index: number; run: { language: string; content: string; marks: { kind: string; href: string }[] } } }
  | { mutation: "removeRun"; payload: { index: number } }
  | { mutation: "editRun"; payload: { index: number; newContent: string } }
  | { mutation: "changeRunLanguage"; payload: { index: number; newLanguage: string } }
  | { mutation: "reorderRuns"; payload: { from: number; to: number } }
  | { mutation: "addMark"; payload: { runIndex: number; index: number; mark: { kind: string; href: string } } }
  | { mutation: "removeMark"; payload: { runIndex: number; index: number } };
