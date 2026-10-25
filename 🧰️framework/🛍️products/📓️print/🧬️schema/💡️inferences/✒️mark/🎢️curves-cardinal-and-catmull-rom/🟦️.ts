/** 🎢️ The cardinal and Catmull–Rom curve interpolators (open, closed and alpha forms) of `\SemioVizPath`, numerically identical to d3-shape. */
import { type VizCurveFactory, type VizPathContext, vizCurveNoop } from "../🟦️.ts";

//#region 🔖️CurveContract
const EPSILON = 1e-12;
//#endregion 🔖️CurveContract

//#region 🔖️CurvesCardinal
type CardinalState = { x0: number; x1: number; x2: number; y0: number; y1: number; y2: number; point: number; line: number };

function cardinalPoint(context: VizPathContext, s: CardinalState, k: number, x: number, y: number): void {
  context.bezierCurveTo(s.x1 + k * (s.x2 - s.x0), s.y1 + k * (s.y2 - s.y0), s.x2 + k * (s.x1 - x), s.y2 + k * (s.y1 - y), s.x2, s.y2);
}

function newCardinalState(): CardinalState {
  return { x0: Number.NaN, x1: Number.NaN, x2: Number.NaN, y0: Number.NaN, y1: Number.NaN, y2: Number.NaN, point: 0, line: Number.NaN };
}

/** ✒️ `curveCardinal` at a given tension. */
export function curveCardinalTension(tension: number): VizCurveFactory {
  const k = (1 - tension) / 6;
  return (context) => {
    const s = newCardinalState();
    return {
      areaStart: () => void (s.line = 0),
      areaEnd: () => void (s.line = Number.NaN),
      lineStart: () => {
        Object.assign(s, newCardinalState(), { line: s.line });
      },
      lineEnd: () => {
        if (s.point === 2) context.lineTo(s.x2, s.y2);
        else if (s.point === 3) cardinalPoint(context, s, k, s.x1, s.y1);
        if (s.line || (s.line !== 0 && s.point === 1)) context.closePath();
        s.line = 1 - s.line;
      },
      point: (x, y) => {
        if (s.point === 0) {
          s.point = 1;
          if (s.line) context.lineTo(x, y);
          else context.moveTo(x, y);
        } else if (s.point === 1) {
          s.point = 2;
          s.x1 = x;
          s.y1 = y;
        } else {
          if (s.point === 2) s.point = 3;
          cardinalPoint(context, s, k, x, y);
        }
        s.x0 = s.x1;
        s.x1 = s.x2;
        s.x2 = x;
        s.y0 = s.y1;
        s.y1 = s.y2;
        s.y2 = y;
      },
    };
  };
}

/** 📕️ `curveCardinalClosed` at a given tension. */
export function curveCardinalClosedTension(tension: number): VizCurveFactory {
  const k = (1 - tension) / 6;
  return (context) => {
    const s = newCardinalState();
    let x3 = Number.NaN;
    let y3 = Number.NaN;
    let x4 = Number.NaN;
    let y4 = Number.NaN;
    let x5 = Number.NaN;
    let y5 = Number.NaN;
    const push = (x: number, y: number): void => {
      if (s.point === 0) {
        s.point = 1;
        x3 = x;
        y3 = y;
      } else if (s.point === 1) {
        s.point = 2;
        x4 = x;
        y4 = y;
        context.moveTo(x4, y4);
      } else if (s.point === 2) {
        s.point = 3;
        x5 = x;
        y5 = y;
      } else cardinalPoint(context, s, k, x, y);
      s.x0 = s.x1;
      s.x1 = s.x2;
      s.x2 = x;
      s.y0 = s.y1;
      s.y1 = s.y2;
      s.y2 = y;
    };
    return {
      areaStart: vizCurveNoop,
      areaEnd: vizCurveNoop,
      lineStart: () => {
        Object.assign(s, newCardinalState());
        x3 = Number.NaN;
        y3 = Number.NaN;
        x4 = Number.NaN;
        y4 = Number.NaN;
        x5 = Number.NaN;
        y5 = Number.NaN;
      },
      lineEnd: () => {
        if (s.point === 1) {
          context.moveTo(x3, y3);
          context.closePath();
        } else if (s.point === 2) {
          context.lineTo(x3, y3);
          context.closePath();
        } else if (s.point === 3) {
          push(x3, y3);
          push(x4, y4);
          push(x5, y5);
        }
      },
      point: push,
    };
  };
}

/** 🔓️ `curveCardinalOpen` at a given tension. */
export function curveCardinalOpenTension(tension: number): VizCurveFactory {
  const k = (1 - tension) / 6;
  return (context) => {
    const s = newCardinalState();
    return {
      areaStart: () => void (s.line = 0),
      areaEnd: () => void (s.line = Number.NaN),
      lineStart: () => {
        Object.assign(s, newCardinalState(), { line: s.line });
      },
      lineEnd: () => {
        if (s.line || (s.line !== 0 && s.point === 3)) context.closePath();
        s.line = 1 - s.line;
      },
      point: (x, y) => {
        if (s.point === 0) s.point = 1;
        else if (s.point === 1) s.point = 2;
        else if (s.point === 2) {
          s.point = 3;
          if (s.line) context.lineTo(s.x2, s.y2);
          else context.moveTo(s.x2, s.y2);
        } else {
          if (s.point === 3) s.point = 4;
          cardinalPoint(context, s, k, x, y);
        }
        s.x0 = s.x1;
        s.x1 = s.x2;
        s.x2 = x;
        s.y0 = s.y1;
        s.y1 = s.y2;
        s.y2 = y;
      },
    };
  };
}

/** 🧭️ `curveCardinal` at d3's default tension of 0. */
export const curveCardinal = curveCardinalTension(0);

/** 📪️ `curveCardinalClosed` at d3's default tension of 0. */
export const curveCardinalClosed = curveCardinalClosedTension(0);

/** 📂️ `curveCardinalOpen` at d3's default tension of 0. */
export const curveCardinalOpen = curveCardinalOpenTension(0);

//#endregion 🔖️CurvesCardinal
//#region 🔖️CurvesCatmullRom
type CatmullState = CardinalState & { l01a: number; l12a: number; l23a: number; l012a: number; l122a: number; l232a: number };

function newCatmullState(): CatmullState {
  return { ...newCardinalState(), l01a: 0, l12a: 0, l23a: 0, l012a: 0, l122a: 0, l232a: 0 };
}

function catmullPoint(context: VizPathContext, s: CatmullState, x: number, y: number): void {
  let x1 = s.x1;
  let y1 = s.y1;
  let x2 = s.x2;
  let y2 = s.y2;
  if (s.l01a > EPSILON) {
    const a = 2 * s.l012a + 3 * s.l01a * s.l12a + s.l122a;
    const n = 3 * s.l01a * (s.l01a + s.l12a);
    x1 = (x1 * a - s.x0 * s.l122a + s.x2 * s.l012a) / n;
    y1 = (y1 * a - s.y0 * s.l122a + s.y2 * s.l012a) / n;
  }
  if (s.l23a > EPSILON) {
    const b = 2 * s.l232a + 3 * s.l23a * s.l12a + s.l122a;
    const m = 3 * s.l23a * (s.l23a + s.l12a);
    x2 = (x2 * b + s.x1 * s.l232a - x * s.l122a) / m;
    y2 = (y2 * b + s.y1 * s.l232a - y * s.l122a) / m;
  }
  context.bezierCurveTo(x1, y1, x2, y2, s.x2, s.y2);
}

function advanceCatmull(s: CatmullState, x: number, y: number): void {
  s.l01a = s.l12a;
  s.l12a = s.l23a;
  s.l012a = s.l122a;
  s.l122a = s.l232a;
  s.x0 = s.x1;
  s.x1 = s.x2;
  s.x2 = x;
  s.y0 = s.y1;
  s.y1 = s.y2;
  s.y2 = y;
}

function measureCatmull(s: CatmullState, alpha: number, x: number, y: number): void {
  if (s.point) {
    const x23 = s.x2 - x;
    const y23 = s.y2 - y;
    s.l232a = (x23 * x23 + y23 * y23) ** alpha;
    s.l23a = Math.sqrt(s.l232a);
  }
}

/** 🧶️ `curveCatmullRom` at a given α; α = 0 collapses onto the uniform cardinal spline. */
export function curveCatmullRomAlpha(alpha: number): VizCurveFactory {
  if (!(alpha > 0)) return curveCardinalTension(0);
  return (context) => {
    const s = newCatmullState();
    return {
      areaStart: () => void (s.line = 0),
      areaEnd: () => void (s.line = Number.NaN),
      lineStart: () => {
        Object.assign(s, newCatmullState(), { line: s.line });
      },
      lineEnd: () => {
        if (s.point === 2) context.lineTo(s.x2, s.y2);
        else if (s.point === 3) {
          measureCatmull(s, alpha, s.x2, s.y2);
          catmullPoint(context, s, s.x2, s.y2);
          advanceCatmull(s, s.x2, s.y2);
        }
        if (s.line || (s.line !== 0 && s.point === 1)) context.closePath();
        s.line = 1 - s.line;
      },
      point: (x, y) => {
        measureCatmull(s, alpha, x, y);
        if (s.point === 0) {
          s.point = 1;
          if (s.line) context.lineTo(x, y);
          else context.moveTo(x, y);
        } else if (s.point === 1) s.point = 2;
        else {
          if (s.point === 2) s.point = 3;
          catmullPoint(context, s, x, y);
        }
        advanceCatmull(s, x, y);
      },
    };
  };
}

/** 🔒️ `curveCatmullRomClosed` at a given α. */
export function curveCatmullRomClosedAlpha(alpha: number): VizCurveFactory {
  if (!(alpha > 0)) return curveCardinalClosedTension(0);
  return (context) => {
    const s = newCatmullState();
    let x3 = Number.NaN;
    let y3 = Number.NaN;
    let x4 = Number.NaN;
    let y4 = Number.NaN;
    let x5 = Number.NaN;
    let y5 = Number.NaN;
    const push = (x: number, y: number): void => {
      measureCatmull(s, alpha, x, y);
      if (s.point === 0) {
        s.point = 1;
        x3 = x;
        y3 = y;
      } else if (s.point === 1) {
        s.point = 2;
        x4 = x;
        y4 = y;
        context.moveTo(x4, y4);
      } else if (s.point === 2) {
        s.point = 3;
        x5 = x;
        y5 = y;
      } else catmullPoint(context, s, x, y);
      advanceCatmull(s, x, y);
    };
    return {
      areaStart: vizCurveNoop,
      areaEnd: vizCurveNoop,
      lineStart: () => {
        Object.assign(s, newCatmullState());
        x3 = Number.NaN;
        y3 = Number.NaN;
        x4 = Number.NaN;
        y4 = Number.NaN;
        x5 = Number.NaN;
        y5 = Number.NaN;
      },
      lineEnd: () => {
        if (s.point === 1) {
          context.moveTo(x3, y3);
          context.closePath();
        } else if (s.point === 2) {
          context.lineTo(x3, y3);
          context.closePath();
        } else if (s.point === 3) {
          push(x3, y3);
          push(x4, y4);
          push(x5, y5);
        }
      },
      point: push,
    };
  };
}

/** 🗝️ `curveCatmullRomOpen` at a given α. */
export function curveCatmullRomOpenAlpha(alpha: number): VizCurveFactory {
  if (!(alpha > 0)) return curveCardinalOpenTension(0);
  return (context) => {
    const s = newCatmullState();
    return {
      areaStart: () => void (s.line = 0),
      areaEnd: () => void (s.line = Number.NaN),
      lineStart: () => {
        Object.assign(s, newCatmullState(), { line: s.line });
      },
      lineEnd: () => {
        if (s.line || (s.line !== 0 && s.point === 3)) context.closePath();
        s.line = 1 - s.line;
      },
      point: (x, y) => {
        measureCatmull(s, alpha, x, y);
        if (s.point === 0) s.point = 1;
        else if (s.point === 1) s.point = 2;
        else if (s.point === 2) {
          s.point = 3;
          if (s.line) context.lineTo(s.x2, s.y2);
          else context.moveTo(s.x2, s.y2);
        } else {
          if (s.point === 3) s.point = 4;
          catmullPoint(context, s, x, y);
        }
        advanceCatmull(s, x, y);
      },
    };
  };
}

/** 🪢️ `curveCatmullRom` at d3's default α of ½. */
export const curveCatmullRom = curveCatmullRomAlpha(0.5);

/** 🔐️ `curveCatmullRomClosed` at d3's default α of ½. */
export const curveCatmullRomClosed = curveCatmullRomClosedAlpha(0.5);

/** 📬️ `curveCatmullRomOpen` at d3's default α of ½. */
export const curveCatmullRomOpen = curveCatmullRomOpenAlpha(0.5);
//#endregion 🔖️CurvesCatmullRom
