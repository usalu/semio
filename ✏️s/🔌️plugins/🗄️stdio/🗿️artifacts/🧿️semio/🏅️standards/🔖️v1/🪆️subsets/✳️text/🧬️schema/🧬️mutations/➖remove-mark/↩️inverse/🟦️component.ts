/** ↩️ inverse for `RemoveMark`. */
export interface RemoveMarkInverseAddMark {
  runIndex: number;
  index: number;
  mark: { kind: "bold" | "italic" | "code" | "link"; href: string };
}
