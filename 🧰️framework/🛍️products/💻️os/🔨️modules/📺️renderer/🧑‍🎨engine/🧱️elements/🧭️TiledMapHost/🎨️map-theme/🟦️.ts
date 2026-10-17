/** 🎨️ TiledMapHost theme: the CSS color parsing and the canvas theme JSON the vello map renderer reads. */
import { MAP_VELLO_THEME_FALLBACK_RGBA } from "../📐️map-parsing/🟦️.ts";

//#region 🔖️TiledMapHost
//#region MapTheme
function mapParseCssColorToRgba8888(css: string, fallback: [number, number, number, number]): [number, number, number, number] {
  const m = css.match(/rgba?\(\s*([\d.]+)\s*,\s*([\d.]+)\s*,\s*([\d.]+)(?:\s*,\s*([\d.]+%?)\s*)?\)/u);
  if (!m) return fallback;
  const r = Math.min(255, Math.max(0, Math.round(Number(m[1]))));
  const g = Math.min(255, Math.max(0, Math.round(Number(m[2]))));
  const b = Math.min(255, Math.max(0, Math.round(Number(m[3]))));
  let a = 255;
  if (m[4] !== undefined && m[4] !== "") {
    const raw = m[4];
    if (raw.endsWith("%")) {
      a = Math.min(255, Math.max(0, Math.round((Number(raw.slice(0, -1)) / 100) * 255)));
    } else {
      const n = Number(raw);
      a = Math.min(255, Math.max(0, Math.round(n <= 1 ? n * 255 : n)));
    }
  }
  return [r, g, b, a];
}

function mapProbeCssComputed(property: "color" | "backgroundColor", value: string): string {
  if (typeof document === "undefined") return "";
  const el = document.createElement("span");
  const key = property === "color" ? "color" : "background-color";
  el.setAttribute("style", `${key}:${value};position:absolute;left:0;top:0;visibility:hidden;pointer-events:none`);
  if (document.documentElement.classList.contains("dark")) el.classList.add("dark");
  document.documentElement.appendChild(el);
  const out = getComputedStyle(el)[property];
  el.remove();
  return out;
}

export function serializeMapCanvasThemeJson(): string {
  const fb = MAP_VELLO_THEME_FALLBACK_RGBA;
  const pc = (prop: "color" | "backgroundColor", expr: string, fall: [number, number, number, number]): number[] => {
    const raw = mapProbeCssComputed(prop, expr);
    return [...mapParseCssColorToRgba8888(raw, fall)];
  };
  return JSON.stringify({
    surfaceClear: pc("backgroundColor", "var(--canvas)", fb.surfaceClear),
    landFill: pc("backgroundColor", "color-mix(in oklab, var(--color-muted-foreground) 32%, var(--color-canvas))", fb.landFill),
    landStroke: pc("color", "color-mix(in oklab, var(--color-muted-foreground) 42%, transparent)", fb.landStroke),
    labelFill: pc("color", "var(--foreground)", fb.labelFill),
    labelHalo: pc("backgroundColor", "var(--canvas)", fb.labelHalo),
    regionFill: pc("backgroundColor", "color-mix(in oklab, var(--color-secondary) 22%, transparent)", fb.regionFill),
    regionStroke: pc("color", "var(--color-secondary)", fb.regionStroke),
    routeStroke: pc("color", "var(--color-tertiary)", fb.routeStroke),
    positionFill: pc("backgroundColor", "var(--color-active-base)", fb.positionFill),
    positionStroke: pc("color", "var(--color-active-foreground)", fb.positionStroke),
    selectionStroke: pc("color", "var(--color-active-base)", fb.selectionStroke),
    hoverStroke: pc("color", "var(--color-secondary)", fb.hoverStroke),
  });
}
//#endregion MapTheme
//#endregion 🔖️TiledMapHost
