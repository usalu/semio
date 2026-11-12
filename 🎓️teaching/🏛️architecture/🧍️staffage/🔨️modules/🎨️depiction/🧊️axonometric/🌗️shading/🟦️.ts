/** 🌗️ The face shading of the solid views of the axonometric viewpoint: four bands of the tone colour by the angle of a face to a fixed light.
 *
 * The light comes from the upper left of the picture and from the viewer: `(-0.45, 0.78, 0.5)` normalised in view space (`x` right, `y` up, `z` towards the viewer). A face is lit by `max(0, n · light)` for its unit normal `n`; the bands are `< 0.25`, `< 0.5`, `< 0.75` and above, the brightest band keeps the tone colour of the theme exactly and the darker ones mix it with the ink-dark paper `#001117` by 0.12, 0.24 and 0.36. Four flat bands keep the vector output compact (one path per solid and band) and the drawing legible in print;
 * `shading: "flat"` paints every face with its tone colour like the planar views.
 */
import { mix } from "../../🖍️palette/🟦️.ts";
import * as portable from "../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🧮️math/🎯️portable/🟦️.ts";

/** 🦄 The number of bands. */
export const BANDS = 4;

/** 💡️ The unit light direction in view space. */
export const LIGHT: readonly [number, number, number] = (() => {
  const l = portable.hypot(-0.45, 0.78, 0.5);
  return [-0.45 / l, 0.78 / l, 0.5 / l];
})();

/** 🌑️ The share of the shadow colour mixed into each band, darkest first. */
export const SHADE_AMOUNTS: readonly number[] = [0.36, 0.24, 0.12, 0];

/** 🐙 The shadow colour the darker bands mix towards. */
export const SHADOW = "#001117";

/** 🦋 The band `0..3` of a face with the (not necessarily unit) normal `(x, y, z)` in view space: 3 is the brightest. */
export function bandOf(x: number, y: number, z: number): number {
  const length = portable.hypot(x, y, z);
  if (length === 0) return BANDS - 1;
  const lit = Math.max(0, (x * LIGHT[0] + y * LIGHT[1] + z * LIGHT[2]) / length);
  return lit >= 0.75 ? 3 : lit >= 0.5 ? 2 : lit >= 0.25 ? 1 : 0;
}

/** 🎨️ The colour of a band of a `#rrggbb` tone colour. */
export const shaded = (hex: string, band: number): string => (band >= BANDS - 1 ? hex : mix(hex, SHADOW, SHADE_AMOUNTS[band]!));
