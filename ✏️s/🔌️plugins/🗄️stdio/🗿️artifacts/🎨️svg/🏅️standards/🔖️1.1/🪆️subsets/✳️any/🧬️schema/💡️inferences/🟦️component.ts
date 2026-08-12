/** 💡️ Svg inference schema — root `<svg>` intrinsic size. */

export interface SvgDimensions {
  width: number;
  height: number;
}

export interface SvgInference {
  /** @state inferred */
  dimensions: SvgDimensions;
}
