/** mutation payload — mirrors `ChangeStrokeWidth`. Addressed by `styleName`. */
export interface ChangeStrokeWidth {
  styleName: string;
  newWidth?: number;
}
