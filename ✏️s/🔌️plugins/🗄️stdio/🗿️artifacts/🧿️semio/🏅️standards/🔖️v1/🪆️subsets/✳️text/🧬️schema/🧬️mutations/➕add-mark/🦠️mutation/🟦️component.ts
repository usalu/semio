/** mutation payload — mirrors `AddMark`. */
export interface AddMark {
  runIndex: number;
  index: number;
  mark: { kind: "bold" | "italic" | "code" | "link"; href: string };
}
