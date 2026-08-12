/** ↩️ inverse for `RemoveRun`. */
export interface RemoveRunInverseInsertRun {
  index: number;
  run: { language: string; content: string; marks: { kind: "bold" | "italic" | "code" | "link"; href: string }[] };
}
