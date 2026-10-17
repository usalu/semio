/** 💠️ The curve registry (`vizCurve`) that names every interpolator and the area-sized symbol glyphs of `\SemioVizSymbol`. */
import type { VizCurveKind, VizPoint, VizSymbolKind } from "@semio-tech/print/schema/snapshot/chart";
import { type VizCurveFactory, type VizPathCommand, type VizPathContext, vizPathRecorder } from "../🟦️.ts";
import { curveLinear, curveLinearClosed, curveStep, curveStepAfter, curveStepBefore } from "../🪜️curves-linear-and-step/🟦️.ts";
import { curveBasis, curveBasisClosed, curveBasisOpen, curveBundleBeta } from "../🌀️curves-basis-and-bundle/🟦️.ts";
import { curveCardinalClosedTension, curveCardinalOpenTension, curveCardinalTension, curveCatmullRomAlpha, curveCatmullRomClosedAlpha, curveCatmullRomOpenAlpha } from "../🎢️curves-cardinal-and-catmull-rom/🟦️.ts";
import { curveBumpX, curveMonotoneX, curveMonotoneY, curveNatural } from "../🏔️curves-monotone-natural-and-bump/🟦️.ts";

//#region 🔖️CurveRegistry
/** ✒️ Resolves a curve name from the grammar onto its factory, with the documented parameters. */
export function vizCurve(kind: VizCurveKind, options: { tension?: number; alpha?: number; beta?: number } = {}): VizCurveFactory {
  switch (kind) {
    case "linear-closed":
      return curveLinearClosed;
    case "step":
      return curveStep;
    case "step-before":
      return curveStepBefore;
    case "step-after":
      return curveStepAfter;
    case "basis":
      return curveBasis;
    case "basis-closed":
      return curveBasisClosed;
    case "basis-open":
      return curveBasisOpen;
    case "bundle":
      return curveBundleBeta(options.beta ?? 0.85);
    case "cardinal":
      return curveCardinalTension(options.tension ?? 0);
    case "cardinal-closed":
      return curveCardinalClosedTension(options.tension ?? 0);
    case "cardinal-open":
      return curveCardinalOpenTension(options.tension ?? 0);
    case "catmull-rom":
      return curveCatmullRomAlpha(options.alpha ?? 0.5);
    case "catmull-rom-closed":
      return curveCatmullRomClosedAlpha(options.alpha ?? 0.5);
    case "catmull-rom-open":
      return curveCatmullRomOpenAlpha(options.alpha ?? 0.5);
    case "monotone-x":
      return curveMonotoneX;
    case "monotone-y":
      return curveMonotoneY;
    case "natural":
      return curveNatural;
    case "bezier":
      return curveBumpX;
    default:
      return curveLinear;
  }
}

//#endregion 🔖️CurveRegistry
//#region 🔖️Symbols
const TAU = 2 * Math.PI;
const SQRT3 = Math.sqrt(3);
const TAN30 = Math.sqrt(1 / 3);
const STAR_KA = 0.8908130915292852;
const STAR_KR = Math.sin(Math.PI / 10) / Math.sin((7 * Math.PI) / 10);
const STAR_KX = Math.sin(TAU / 10) * STAR_KR;
const STAR_KY = -Math.cos(TAU / 10) * STAR_KR;

/** ✒️ Draws one symbol of the given area into a path context, centred on the origin. */
export function drawVizSymbol(kind: VizSymbolKind, context: VizPathContext, size: number): void {
  switch (kind) {
    case "cross": {
      const r = Math.sqrt(size / 5) / 2;
      context.moveTo(-3 * r, -r);
      context.lineTo(-r, -r);
      context.lineTo(-r, -3 * r);
      context.lineTo(r, -3 * r);
      context.lineTo(r, -r);
      context.lineTo(3 * r, -r);
      context.lineTo(3 * r, r);
      context.lineTo(r, r);
      context.lineTo(r, 3 * r);
      context.lineTo(-r, 3 * r);
      context.lineTo(-r, r);
      context.lineTo(-3 * r, r);
      context.closePath();
      return;
    }
    case "diamond": {
      const y = Math.sqrt(size / (2 * TAN30));
      const x = y * TAN30;
      context.moveTo(0, -y);
      context.lineTo(x, 0);
      context.lineTo(0, y);
      context.lineTo(-x, 0);
      context.closePath();
      return;
    }
    case "square": {
      const w = Math.sqrt(size);
      context.rect(-w / 2, -w / 2, w, w);
      return;
    }
    case "star": {
      const r = Math.sqrt(size * STAR_KA);
      const x = STAR_KX * r;
      const y = STAR_KY * r;
      context.moveTo(0, -r);
      context.lineTo(x, y);
      for (let i = 1; i < 5; i += 1) {
        const a = (TAU * i) / 5;
        const c = Math.cos(a);
        const s = Math.sin(a);
        context.lineTo(s * r, -c * r);
        context.lineTo(c * x - s * y, s * x + c * y);
      }
      context.closePath();
      return;
    }
    case "triangle": {
      const y = -Math.sqrt(size / (SQRT3 * 3));
      context.moveTo(0, y * 2);
      context.lineTo(-SQRT3 * y, -y);
      context.lineTo(SQRT3 * y, -y);
      context.closePath();
      return;
    }
    case "wye": {
      const c = -0.5;
      const s = SQRT3 / 2;
      const k = 1 / Math.sqrt(12);
      const a = (k / 2 + 1) * 3;
      const r = Math.sqrt(size / a);
      const x0 = r / 2;
      const y0 = r * k;
      const x1 = x0;
      const y1 = r * k + r;
      const x2 = -x1;
      const y2 = y1;
      context.moveTo(x0, y0);
      context.lineTo(x1, y1);
      context.lineTo(x2, y2);
      context.lineTo(c * x0 - s * y0, s * x0 + c * y0);
      context.lineTo(c * x1 - s * y1, s * x1 + c * y1);
      context.lineTo(c * x2 - s * y2, s * x2 + c * y2);
      context.lineTo(c * x0 + s * y0, c * y0 - s * x0);
      context.lineTo(c * x1 + s * y1, c * y1 - s * x1);
      context.lineTo(c * x2 + s * y2, c * y2 - s * x2);
      context.closePath();
      return;
    }
    case "asterisk": {
      const r = Math.sqrt(size + Math.min(size / 28, 0.75)) * 0.59436;
      const t = r / 2;
      const u = t * SQRT3;
      context.moveTo(0, r);
      context.lineTo(0, -r);
      context.moveTo(-u, -t);
      context.lineTo(u, t);
      context.moveTo(-u, t);
      context.lineTo(u, -t);
      return;
    }
    case "diamond2": {
      const r = Math.sqrt(size) * 0.62625;
      context.moveTo(0, -r);
      context.lineTo(r, 0);
      context.lineTo(0, r);
      context.lineTo(-r, 0);
      context.closePath();
      return;
    }
    case "plus": {
      const r = Math.sqrt(size - Math.min(size / 7, 2)) * 0.87559;
      context.moveTo(-r, 0);
      context.lineTo(r, 0);
      context.moveTo(0, r);
      context.lineTo(0, -r);
      return;
    }
    case "square2": {
      const r = Math.sqrt(size) * 0.4431;
      context.moveTo(r, r);
      context.lineTo(r, -r);
      context.lineTo(-r, -r);
      context.lineTo(-r, r);
      context.closePath();
      return;
    }
    case "times": {
      const r = Math.sqrt(size - Math.min(size / 6, 1.7)) * 0.6189;
      context.moveTo(-r, -r);
      context.lineTo(r, r);
      context.moveTo(-r, r);
      context.lineTo(r, -r);
      return;
    }
    case "triangle2": {
      const s = Math.sqrt(size) * 0.6824;
      const t = s / 2;
      const u = (s * SQRT3) / 2;
      context.moveTo(0, -s);
      context.lineTo(u, t);
      context.lineTo(-u, t);
      context.closePath();
      return;
    }
    default: {
      const r = Math.sqrt(size / Math.PI);
      context.moveTo(r, 0);
      context.arc(0, 0, r, 0, TAU);
    }
  }
}

/** ✒️ `\SemioVizSymbol`: one glyph of the given area recorded as path commands. */
export function vizSymbol(kind: VizSymbolKind, size = 64, at: VizPoint = [0, 0]): VizPathCommand[] {
  const recorder = vizPathRecorder();
  const shifted: VizPathContext = {
    moveTo: (x, y) => recorder.moveTo(x + at[0], y + at[1]),
    lineTo: (x, y) => recorder.lineTo(x + at[0], y + at[1]),
    quadraticCurveTo: (cx, cy, x, y) => recorder.quadraticCurveTo(cx + at[0], cy + at[1], x + at[0], y + at[1]),
    bezierCurveTo: (cx1, cy1, cx2, cy2, x, y) => recorder.bezierCurveTo(cx1 + at[0], cy1 + at[1], cx2 + at[0], cy2 + at[1], x + at[0], y + at[1]),
    arc: (x, y, r, a0, a1, ccw) => recorder.arc(x + at[0], y + at[1], r, a0, a1, ccw),
    arcTo: (x1, y1, x2, y2, r) => recorder.arcTo(x1 + at[0], y1 + at[1], x2 + at[0], y2 + at[1], r),
    rect: (x, y, w, h) => recorder.rect(x + at[0], y + at[1], w, h),
    closePath: () => recorder.closePath(),
  };
  drawVizSymbol(kind, shifted, size);
  return recorder.commands;
}
//#endregion 🔖️Symbols
