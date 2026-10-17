/** 🌀️ The basis, closed basis, open basis and bundle curve interpolators of `\SemioVizPath`, numerically identical to d3-shape. */
import { type VizCurveFactory, type VizPathContext, vizCurveNoop } from "../🟦️.ts";
import { curveLinear } from "../🪜️curves-linear-and-step/🟦️.ts";

//#region 🔖️CurvesBasis
type BasisState = { x0: number; x1: number; y0: number; y1: number; point: number; line: number };

function basisPoint(context: VizPathContext, s: BasisState, x: number, y: number): void {
  context.bezierCurveTo((2 * s.x0 + s.x1) / 3, (2 * s.y0 + s.y1) / 3, (s.x0 + 2 * s.x1) / 3, (s.y0 + 2 * s.y1) / 3, (s.x0 + 4 * s.x1 + x) / 6, (s.y0 + 4 * s.y1 + y) / 6);
}

/** ✒️ `curveBasis`: a cubic B-spline through the control polygon. */
export const curveBasis: VizCurveFactory = (context) => {
  const s: BasisState = { x0: Number.NaN, x1: Number.NaN, y0: Number.NaN, y1: Number.NaN, point: 0, line: Number.NaN };
  return {
    areaStart: () => void (s.line = 0),
    areaEnd: () => void (s.line = Number.NaN),
    lineStart: () => {
      s.x0 = Number.NaN;
      s.x1 = Number.NaN;
      s.y0 = Number.NaN;
      s.y1 = Number.NaN;
      s.point = 0;
    },
    lineEnd: () => {
      if (s.point === 3) {
        basisPoint(context, s, s.x1, s.y1);
        context.lineTo(s.x1, s.y1);
      } else if (s.point === 2) context.lineTo(s.x1, s.y1);
      if (s.line || (s.line !== 0 && s.point === 1)) context.closePath();
      s.line = 1 - s.line;
    },
    point: (x, y) => {
      if (s.point === 0) {
        s.point = 1;
        if (s.line) context.lineTo(x, y);
        else context.moveTo(x, y);
      } else if (s.point === 1) s.point = 2;
      else {
        if (s.point === 2) {
          s.point = 3;
          context.lineTo((5 * s.x0 + s.x1) / 6, (5 * s.y0 + s.y1) / 6);
        }
        basisPoint(context, s, x, y);
      }
      s.x0 = s.x1;
      s.x1 = x;
      s.y0 = s.y1;
      s.y1 = y;
    },
  };
};

/** ✒️ `curveBasisClosed`: the periodic B-spline. */
export const curveBasisClosed: VizCurveFactory = (context) => {
  const s: BasisState = { x0: Number.NaN, x1: Number.NaN, y0: Number.NaN, y1: Number.NaN, point: 0, line: Number.NaN };
  let x2 = Number.NaN;
  let y2 = Number.NaN;
  let x3 = Number.NaN;
  let y3 = Number.NaN;
  let x4 = Number.NaN;
  let y4 = Number.NaN;
  const push = (x: number, y: number): void => {
    if (s.point === 0) {
      s.point = 1;
      x2 = x;
      y2 = y;
    } else if (s.point === 1) {
      s.point = 2;
      x3 = x;
      y3 = y;
    } else if (s.point === 2) {
      s.point = 3;
      x4 = x;
      y4 = y;
      context.moveTo((s.x0 + 4 * s.x1 + x) / 6, (s.y0 + 4 * s.y1 + y) / 6);
    } else basisPoint(context, s, x, y);
    s.x0 = s.x1;
    s.x1 = x;
    s.y0 = s.y1;
    s.y1 = y;
  };
  return {
    areaStart: vizCurveNoop,
    areaEnd: vizCurveNoop,
    lineStart: () => {
      s.x0 = Number.NaN;
      s.x1 = Number.NaN;
      s.y0 = Number.NaN;
      s.y1 = Number.NaN;
      x2 = Number.NaN;
      y2 = Number.NaN;
      x3 = Number.NaN;
      y3 = Number.NaN;
      x4 = Number.NaN;
      y4 = Number.NaN;
      s.point = 0;
    },
    lineEnd: () => {
      if (s.point === 1) {
        context.moveTo(x2, y2);
        context.closePath();
      } else if (s.point === 2) {
        context.moveTo((x2 + 2 * x3) / 3, (y2 + 2 * y3) / 3);
        context.lineTo((x3 + 2 * x2) / 3, (y3 + 2 * y2) / 3);
        context.closePath();
      } else if (s.point === 3) {
        push(x2, y2);
        push(x3, y3);
        push(x4, y4);
      }
    },
    point: push,
  };
};

/** ✒️ `curveBasisOpen`: the B-spline without the endpoint duplication. */
export const curveBasisOpen: VizCurveFactory = (context) => {
  const s: BasisState = { x0: Number.NaN, x1: Number.NaN, y0: Number.NaN, y1: Number.NaN, point: 0, line: Number.NaN };
  return {
    areaStart: () => void (s.line = 0),
    areaEnd: () => void (s.line = Number.NaN),
    lineStart: () => {
      s.x0 = Number.NaN;
      s.x1 = Number.NaN;
      s.y0 = Number.NaN;
      s.y1 = Number.NaN;
      s.point = 0;
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
        const x0 = (s.x0 + 4 * s.x1 + x) / 6;
        const y0 = (s.y0 + 4 * s.y1 + y) / 6;
        if (s.line) context.lineTo(x0, y0);
        else context.moveTo(x0, y0);
      } else {
        if (s.point === 3) s.point = 4;
        basisPoint(context, s, x, y);
      }
      s.x0 = s.x1;
      s.x1 = x;
      s.y0 = s.y1;
      s.y1 = y;
    },
  };
};

/** ✒️ `curveBundle`: the basis spline pulled toward the straight chord by `beta`. */
export function curveBundleBeta(beta: number): VizCurveFactory {
  if (!(beta > 0)) return curveLinear;
  return (context) => {
    const basis = curveBasis(context);
    let xs: number[] = [];
    let ys: number[] = [];
    return {
      areaStart: vizCurveNoop,
      areaEnd: vizCurveNoop,
      lineStart: () => {
        xs = [];
        ys = [];
        basis.lineStart();
      },
      lineEnd: () => {
        const j = xs.length - 1;
        if (j > 0) {
          const x0 = xs[0]!;
          const y0 = ys[0]!;
          const dx = xs[j]! - x0;
          const dy = ys[j]! - y0;
          for (let i = 0; i <= j; i += 1) {
            const t = i / j;
            basis.point(beta * xs[i]! + (1 - beta) * (x0 + t * dx), beta * ys[i]! + (1 - beta) * (y0 + t * dy));
          }
        }
        xs = [];
        ys = [];
        basis.lineEnd();
      },
      point: (x, y) => {
        xs.push(x);
        ys.push(y);
      },
    };
  };
}

/** ✒️ `curveBundle` at d3's default β of 0.85. */
export const curveBundle = curveBundleBeta(0.85);
//#endregion 🔖️CurvesBasis
