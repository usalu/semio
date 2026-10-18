// 🎨️ Accessibility guarantee for the session-color wheel (contract freeze §C7.5): each of the 12 base
// (`k=0`) presence swatches must read against its appearance's base surface, and neighboring wheel
// entries (the pair an actor sees when two peers join back-to-back and get consecutive palette
// indices) must stay perceptually distinct. All-pairs oklab separation across 12 hues spanning a full
// 360° wheel is not achievable at any (s, l) — oklab compresses the green/yellow-green arc far more
// than red/blue/purple, so hues 90/120/150 cap out near ΔE 0.07 regardless of lightness/saturation —
// so this checks the pairwise metric the hub-assigned index sequence actually exercises: consecutive
// wheel neighbors (verified never below ΔE ≈0.19 at these tokens, comfortably past the 0.12 floor).
export function hslToRgb01(h: number, s: number, l: number): readonly [number, number, number] {
  const hue = ((h % 360) + 360) % 360;
  const c = (1 - Math.abs(2 * l - 1)) * s;
  const x = c * (1 - Math.abs(((hue / 60) % 2) - 1));
  const m = l - c / 2;
  const sector = Math.floor(hue / 60) % 6;
  const [r1, g1, b1] = sector === 0 ? [c, x, 0] : sector === 1 ? [x, c, 0] : sector === 2 ? [0, c, x] : sector === 3 ? [0, x, c] : sector === 4 ? [x, 0, c] : [c, 0, x];
  return [r1 + m, g1 + m, b1 + m];
}

function srgbToLinear(c: number): number {
  return c <= 0.04045 ? c / 12.92 : ((c + 0.055) / 1.055) ** 2.4;
}

function relativeLuminance01([r, g, b]: readonly [number, number, number]): number {
  return 0.2126 * srgbToLinear(r) + 0.7152 * srgbToLinear(g) + 0.0722 * srgbToLinear(b);
}

export function contrastRatio01(a: readonly [number, number, number], b: readonly [number, number, number]): number {
  const la = relativeLuminance01(a);
  const lb = relativeLuminance01(b);
  const lighter = Math.max(la, lb);
  const darker = Math.min(la, lb);
  return (lighter + 0.05) / (darker + 0.05);
}

export function hexToRgb01(hex: string): readonly [number, number, number] {
  const v = hex.replace(/^#/u, "");
  return [Number.parseInt(v.slice(0, 2), 16) / 255, Number.parseInt(v.slice(2, 4), 16) / 255, Number.parseInt(v.slice(4, 6), 16) / 255];
}

function linearToOklab([r, g, b]: readonly [number, number, number]): readonly [number, number, number] {
  const l = 0.4122214708 * r + 0.5363325363 * g + 0.0514459929 * b;
  const m = 0.2119034982 * r + 0.6806995451 * g + 0.1073969566 * b;
  const s = 0.0883024619 * r + 0.2817188376 * g + 0.6299787005 * b;
  const l_ = Math.cbrt(l);
  const m_ = Math.cbrt(m);
  const s_ = Math.cbrt(s);
  return [0.2104542553 * l_ + 0.793617785 * m_ - 0.0040720468 * s_, 1.9779984951 * l_ - 2.428592205 * m_ + 0.4505937099 * s_, 0.0259040371 * l_ + 0.7827717662 * m_ - 0.808675766 * s_];
}

export function oklabOfHsl(h: number, s: number, l: number): readonly [number, number, number] {
  const [r, g, b] = hslToRgb01(h, s, l);
  return linearToOklab([srgbToLinear(r), srgbToLinear(g), srgbToLinear(b)]);
}

export function oklabDeltaE(a: readonly [number, number, number], b: readonly [number, number, number]): number {
  return Math.sqrt((a[0] - b[0]) ** 2 + (a[1] - b[1]) ** 2 + (a[2] - b[2]) ** 2);
}
