/** 🎨️ CSS colour text to straight sRGB rgba in [0, 1]: `#rgb`, `#rgba`, `#rrggbb`, `#rrggbbaa`, `rgb()`/`rgba()` in comma or space syntax with percentages and `/ alpha`, and `transparent`. Anything else gives null.
 * @see ../🟦️.ts */
import type { V4 } from "../../🧬️contract/🟦️.ts";

const channel = (text: string): number | null => {
  const t = text.trim();
  const v = t.endsWith("%") ? Number(t.slice(0, -1)) / 100 : Number(t) / 255;
  return Number.isFinite(v) ? Math.min(1, Math.max(0, v)) : null;
};

/** 🖍️ Parses a CSS colour; null when unsupported. */
export function parseCssColor(text: string): V4 | null {
  const s = text.trim().toLowerCase();
  if (s === "transparent") return [0, 0, 0, 0];
  const hex = /^#([0-9a-f]{3,4}|[0-9a-f]{6}|[0-9a-f]{8})$/.exec(s);
  if (hex) {
    const h = hex[1]!.length <= 4 ? [...hex[1]!].map((c) => c + c).join("") : hex[1]!;
    const n = (i: number) => parseInt(h.slice(i, i + 2), 16) / 255;
    return [n(0), n(2), n(4), h.length === 8 ? n(6) : 1];
  }
  const fn = /^rgba?\(\s*([^)]*)\)$/.exec(s);
  if (!fn) return null;
  const [colour, slash] = fn[1]!.split("/");
  const parts = colour!.trim().split(/[\s,]+/).filter(Boolean);
  if (slash !== undefined) parts.push(slash.trim());
  if (parts.length < 3 || parts.length > 4) return null;
  const rgb = parts.slice(0, 3).map(channel);
  if (rgb.some((v) => v === null)) return null;
  let a = 1;
  if (parts[3] !== undefined) {
    const t = parts[3];
    a = t.endsWith("%") ? Number(t.slice(0, -1)) / 100 : Number(t);
    if (!Number.isFinite(a)) return null;
    a = Math.min(1, Math.max(0, a));
  }
  return [rgb[0]!, rgb[1]!, rgb[2]!, a];
}
