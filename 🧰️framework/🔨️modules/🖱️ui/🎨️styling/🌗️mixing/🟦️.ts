export type Rgba8 = [number, number, number, number];

function srgbByteToLinear(c: number): number {
  const x = c / 255;
  return x <= 0.04045 ? x / 12.92 : ((x + 0.055) / 1.055) ** 2.4;
}

/** 🔆️ Converts byte RGBA into linear sRGB and normalized alpha. */
export function rgba8ToLinear(rgba: Rgba8): [number, number, number, number] {
  return [srgbByteToLinear(rgba[0]), srgbByteToLinear(rgba[1]), srgbByteToLinear(rgba[2]), rgba[3] / 255];
}

function linearToSrgbByte(x: number): number {
  const c = x <= 0.0031308 ? x * 12.92 : 1.055 * x ** (1 / 2.4) - 0.055;
  return Math.round(Math.min(1, Math.max(0, c)) * 255);
}

/** @emoji 🌓️ sRGB(linear) → Oklab, per Björn Ottosson's reference matrices (https://bottosson.github.io/posts/oklab/). */
export function linearToOklab(r: number, g: number, b: number): [number, number, number] {
  const l = 0.4122214708 * r + 0.5363325363 * g + 0.0514459929 * b;
  const m = 0.2119034982 * r + 0.6806995451 * g + 0.1073969566 * b;
  const s = 0.0883024619 * r + 0.2817188376 * g + 0.6299787005 * b;
  const l_ = Math.cbrt(l);
  const m_ = Math.cbrt(m);
  const s_ = Math.cbrt(s);
  return [0.2104542553 * l_ + 0.793617785 * m_ - 0.0040720468 * s_, 1.9779984951 * l_ - 2.428592205 * m_ + 0.4505937099 * s_, 0.0259040371 * l_ + 0.7827717662 * m_ - 0.808675766 * s_];
}

/** @emoji 🌓️ Oklab → sRGB(linear), inverse of {@link linearToOklab}. */
function oklabToLinear(L: number, a: number, b: number): [number, number, number] {
  const l_ = L + 0.3963377774 * a + 0.2158037573 * b;
  const m_ = L - 0.1055613458 * a - 0.0638541728 * b;
  const s_ = L - 0.0894841775 * a - 1.291485548 * b;
  const l = l_ ** 3;
  const m = m_ ** 3;
  const s = s_ ** 3;
  return [4.0767416621 * l - 3.3077115913 * m + 0.2309699292 * s, -1.2684380046 * l + 2.6097574011 * m - 0.3413193965 * s, -0.0041960863 * l - 0.7034186147 * m + 1.707614701 * s];
}

/** @emoji 🌓️ Mixes two resolved paints in Oklab space (srgb → linear → oklab → lerp → back), `t=0` returns `a`, `t=1` returns `b`. Alpha lerps linearly. Powers the formula-derived level/element paint ladders (see contract's CSS MECHANISM section). */
export function oklabMix(a: Rgba8, b: Rgba8, t: number): Rgba8 {
  const la = rgba8ToLinear(a);
  const lb = rgba8ToLinear(b);
  const oa = linearToOklab(la[0], la[1], la[2]);
  const ob = linearToOklab(lb[0], lb[1], lb[2]);
  const mixed: [number, number, number] = [oa[0] * (1 - t) + ob[0] * t, oa[1] * (1 - t) + ob[1] * t, oa[2] * (1 - t) + ob[2] * t];
  const [lr, lg, lbl] = oklabToLinear(mixed[0], mixed[1], mixed[2]);
  const alpha = la[3] * (1 - t) + lb[3] * t;
  return [linearToSrgbByte(lr), linearToSrgbByte(lg), linearToSrgbByte(lbl), Math.round(alpha * 255)];
}
