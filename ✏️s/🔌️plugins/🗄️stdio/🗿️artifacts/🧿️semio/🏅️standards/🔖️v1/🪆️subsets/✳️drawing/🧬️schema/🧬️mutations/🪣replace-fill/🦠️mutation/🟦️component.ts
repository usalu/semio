/** mutation payload — mirrors `ReplaceFill`. Addressed by `styleName` (not a node — `DrawStyle`
 * is referenced BY NAME from `DrawNode.style`). `DrawStyle.fill` is a flat `Rgba` in this
 * snapshot's current real shape (no gradient support yet — see the Rust sibling's doc comment). */
export interface ReplaceFill {
  styleName: string;
  newFill?: { r: number; g: number; b: number; a: number };
}
