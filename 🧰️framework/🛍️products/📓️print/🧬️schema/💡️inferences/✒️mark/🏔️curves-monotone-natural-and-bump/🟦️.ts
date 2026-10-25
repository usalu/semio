/** 🏔️ The monotone, natural and bump curve interpolators of `\SemioVizPath`, numerically identical to d3-shape. */
import type { VizCurveFactory, VizPathContext } from "../🟦️.ts";

//#region 🔖️CurvesMonotone
type MonotoneState = { x0: number; x1: number; y0: number; y1: number; t0: number; point: number; line: number };

function slope3(s: MonotoneState, x2: number, y2: number): number {
  const h0 = s.x1 - s.x0;
  const h1 = x2 - s.x1;
  const s0 = (s.y1 - s.y0) / (h0 || (h1 < 0 ? -0 : 0));
  const s1 = (y2 - s.y1) / (h1 || (h0 < 0 ? -0 : 0));
  const p = (s0 * h1 + s1 * h0) / (h0 + h1);
  return (Math.sign(s0) + Math.sign(s1)) * Math.min(Math.abs(s0), Math.abs(s1), 0.5 * Math.abs(p)) || 0;
}

function slope2(s: MonotoneState, t: number): number {
  const h = s.x1 - s.x0;
  return h ? ((3 * (s.y1 - s.y0)) / h - t) / 2 : t;
}

function monotonePoint(context: VizPathContext, s: MonotoneState, t0: number, t1: number): void {
  const dx = (s.x1 - s.x0) / 3;
  context.bezierCurveTo(s.x0 + dx, s.y0 + dx * t0, s.x1 - dx, s.y1 - dx * t1, s.x1, s.y1);
}

function monotoneFactory(reflect: boolean): VizCurveFactory {
  return (rawContext) => {
    const context: VizPathContext = reflect
      ? {
          moveTo: (x, y) => rawContext.moveTo(y, x),
          lineTo: (x, y) => rawContext.lineTo(y, x),
          quadraticCurveTo: (cx, cy, x, y) => rawContext.quadraticCurveTo(cy, cx, y, x),
          bezierCurveTo: (cx1, cy1, cx2, cy2, x, y) => rawContext.bezierCurveTo(cy1, cx1, cy2, cx2, y, x),
          arc: (x, y, r, a0, a1, ccw) => rawContext.arc(y, x, r, a0, a1, ccw),
          arcTo: (x1, y1, x2, y2, r) => rawContext.arcTo(y1, x1, y2, x2, r),
          rect: (x, y, w, h) => rawContext.rect(y, x, h, w),
          closePath: () => rawContext.closePath(),
        }
      : rawContext;
    const s: MonotoneState = { x0: Number.NaN, x1: Number.NaN, y0: Number.NaN, y1: Number.NaN, t0: Number.NaN, point: 0, line: Number.NaN };
    const feed = (rawX: number, rawY: number): void => {
      const x = reflect ? rawY : rawX;
      const y = reflect ? rawX : rawY;
      if (x === s.x1 && y === s.y1) return;
      let t1 = Number.NaN;
      if (s.point === 0) {
        s.point = 1;
        if (s.line) context.lineTo(x, y);
        else context.moveTo(x, y);
      } else if (s.point === 1) s.point = 2;
      else if (s.point === 2) {
        s.point = 3;
        t1 = slope3(s, x, y);
        monotonePoint(context, s, slope2(s, t1), t1);
      } else {
        t1 = slope3(s, x, y);
        monotonePoint(context, s, s.t0, t1);
      }
      s.x0 = s.x1;
      s.x1 = x;
      s.y0 = s.y1;
      s.y1 = y;
      s.t0 = t1;
    };
    return {
      areaStart: () => void (s.line = 0),
      areaEnd: () => void (s.line = Number.NaN),
      lineStart: () => {
        s.x0 = Number.NaN;
        s.x1 = Number.NaN;
        s.y0 = Number.NaN;
        s.y1 = Number.NaN;
        s.t0 = Number.NaN;
        s.point = 0;
      },
      lineEnd: () => {
        if (s.point === 2) context.lineTo(s.x1, s.y1);
        else if (s.point === 3) monotonePoint(context, s, s.t0, slope2(s, s.t0));
        if (s.line || (s.line !== 0 && s.point === 1)) context.closePath();
        s.line = 1 - s.line;
      },
      point: feed,
    };
  };
}

/** ✒️ `curveMonotoneX`: monotone in x, never overshooting between samples. */
export const curveMonotoneX = monotoneFactory(false);

/** ↕️ `curveMonotoneY`: the same interpolator with the axes exchanged. */
export const curveMonotoneY = monotoneFactory(true);

//#endregion 🔖️CurvesMonotone
//#region 🔖️CurvesNatural
function naturalControlPoints(x: readonly number[]): [number[], number[]] {
  const n = x.length - 1;
  const a = new Array<number>(n);
  const b = new Array<number>(n);
  const r = new Array<number>(n);
  a[0] = 0;
  b[0] = 2;
  r[0] = x[0]! + 2 * x[1]!;
  for (let i = 1; i < n - 1; i += 1) {
    a[i] = 1;
    b[i] = 4;
    r[i] = 4 * x[i]! + 2 * x[i + 1]!;
  }
  a[n - 1] = 2;
  b[n - 1] = 7;
  r[n - 1] = 8 * x[n - 1]! + x[n]!;
  for (let i = 1; i < n; i += 1) {
    const m = a[i]! / b[i - 1]!;
    b[i]! -= m;
    r[i]! -= m * r[i - 1]!;
  }
  a[n - 1] = r[n - 1]! / b[n - 1]!;
  for (let i = n - 2; i >= 0; i -= 1) a[i] = (r[i]! - a[i + 1]!) / b[i]!;
  b[n - 1] = (x[n]! + a[n - 1]!) / 2;
  for (let i = 0; i < n - 1; i += 1) b[i] = 2 * x[i + 1]! - a[i + 1]!;
  return [a, b];
}

/** 🌿️ `curveNatural`: the natural cubic spline through every point. */
export const curveNatural: VizCurveFactory = (context) => {
  let xs: number[] = [];
  let ys: number[] = [];
  let line = Number.NaN;
  return {
    areaStart: () => void (line = 0),
    areaEnd: () => void (line = Number.NaN),
    lineStart: () => {
      xs = [];
      ys = [];
    },
    lineEnd: () => {
      const n = xs.length;
      if (n > 0) {
        if (line) context.lineTo(xs[0]!, ys[0]!);
        else context.moveTo(xs[0]!, ys[0]!);
        if (n === 2) context.lineTo(xs[1]!, ys[1]!);
        else if (n > 2) {
          const px = naturalControlPoints(xs);
          const py = naturalControlPoints(ys);
          for (let i0 = 0, i1 = 1; i1 < n; i0 += 1, i1 += 1) context.bezierCurveTo(px[0][i0]!, py[0][i0]!, px[1][i0]!, py[1][i0]!, xs[i1]!, ys[i1]!);
        }
      }
      if (line || (line !== 0 && n === 1)) context.closePath();
      line = 1 - line;
      xs = [];
      ys = [];
    },
    point: (x, y) => {
      xs.push(x);
      ys.push(y);
    },
  };
};

/** ↔️ `curveBumpX` — the horizontal bump `linkHorizontal` draws with. */
export const curveBumpX: VizCurveFactory = (context) => {
  const s: MonotoneState = { x0: Number.NaN, x1: Number.NaN, y0: Number.NaN, y1: Number.NaN, t0: Number.NaN, point: 0, line: Number.NaN };
  return {
    areaStart: () => void (s.line = 0),
    areaEnd: () => void (s.line = Number.NaN),
    lineStart: () => void (s.point = 0),
    lineEnd: () => {
      if (s.line || (s.line !== 0 && s.point === 1)) context.closePath();
      s.line = 1 - s.line;
    },
    point: (x, y) => {
      if (s.point === 0) {
        s.point = 1;
        if (s.line) context.lineTo(x, y);
        else context.moveTo(x, y);
      } else {
        if (s.point === 1) s.point = 2;
        context.bezierCurveTo((s.x0 = (s.x0 + x) / 2), s.y0, s.x0, y, x, y);
      }
      s.x0 = x;
      s.y0 = y;
    },
  };
};

/** ⛰️ `curveBumpY` — the vertical bump `linkVertical` draws with. */
export const curveBumpY: VizCurveFactory = (context) => {
  const s: MonotoneState = { x0: Number.NaN, x1: Number.NaN, y0: Number.NaN, y1: Number.NaN, t0: Number.NaN, point: 0, line: Number.NaN };
  return {
    areaStart: () => void (s.line = 0),
    areaEnd: () => void (s.line = Number.NaN),
    lineStart: () => void (s.point = 0),
    lineEnd: () => {
      if (s.line || (s.line !== 0 && s.point === 1)) context.closePath();
      s.line = 1 - s.line;
    },
    point: (x, y) => {
      if (s.point === 0) {
        s.point = 1;
        if (s.line) context.lineTo(x, y);
        else context.moveTo(x, y);
      } else {
        if (s.point === 1) s.point = 2;
        context.bezierCurveTo(s.x0, (s.y0 = (s.y0 + y) / 2), x, s.y0, x, y);
      }
      s.x0 = x;
      s.y0 = y;
    },
  };
};
//#endregion 🔖️CurvesNatural
