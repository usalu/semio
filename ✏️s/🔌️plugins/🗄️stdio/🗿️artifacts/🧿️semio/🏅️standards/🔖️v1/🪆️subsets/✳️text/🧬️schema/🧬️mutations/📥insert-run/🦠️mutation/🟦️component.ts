/** mutation payload — mirrors `InsertRun`. */
export interface InsertRun {
  index: number;
  run: { language: string; content: string; marks: { kind: "bold" | "italic" | "code" | "link"; href: string }[] };
}
