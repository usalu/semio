/** ↩️ inverse for `ChangeStrokeColor` — always `ChangeStrokeColor` with the captured old color. */
export interface ChangeStrokeColorInverseChangeStrokeColor {
  styleName: string;
  newColor?: { r: number; g: number; b: number; a: number };
}
