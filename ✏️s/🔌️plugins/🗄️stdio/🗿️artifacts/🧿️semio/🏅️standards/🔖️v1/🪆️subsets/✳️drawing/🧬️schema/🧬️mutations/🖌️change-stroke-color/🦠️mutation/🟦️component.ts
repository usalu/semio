/** mutation payload — mirrors `ChangeStrokeColor`. Addressed by `styleName` (the real name-keyed
 * style collection), not a node — `DrawStyle` is referenced BY NAME from `DrawNode.style`. */
export interface ChangeStrokeColor {
  styleName: string;
  newColor?: { r: number; g: number; b: number; a: number };
}
