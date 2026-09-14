/** ✒️ Marks: the TypeScript twin of `semio-viz-mark`. A recording path context, the nineteen true
 * curve interpolators of `\SemioVizPath`, and the area-sized symbol glyphs of `\SemioVizSymbol` —
 * every one of them numerically identical to its d3-shape counterpart.
 * @see ../../../🖋️latex/semio-viz-mark.sty
 */
import type { VizCurveKind, VizPoint, VizSymbolKind } from "../🧬️schema/🟦️.ts";

//#region 🔖️PathContext
/** 🖊️ One recorded path command; the argument order is the canvas order d3 emits. */
export type VizPathCommand =
  | { readonly op: "moveTo"; readonly args: readonly [number, number] }
  | { readonly op: "lineTo"; readonly args: readonly [number, number] }
  | { readonly op: "quadraticCurveTo"; readonly args: readonly [number, number, number, number] }
  | { readonly op: "bezierCurveTo"; readonly args: readonly [number, number, number, number, number, number] }
  | { readonly op: "arc"; readonly args: readonly [number, number, number, number, number, number] }
  | { readonly op: "arcTo"; readonly args: readonly [number, number, number, number, number] }
  | { readonly op: "rect"; readonly args: readonly [number, number, number, number] }
  | { readonly op: "closePath"; readonly args: readonly [] };

/** 🖊️ The canvas-shaped sink every curve and shape generator writes into. */
export type VizPathContext = {
  moveTo(x: number, y: number): void;
  lineTo(x: number, y: number): void;
  quadraticCurveTo(cx: number, cy: number, x: number, y: number): void;
  bezierCurveTo(cx1: number, cy1: number, cx2: number, cy2: number, x: number, y: number): void;
  arc(x: number, y: number, r: number, a0: number, a1: number, counterclockwise?: boolean): void;
  arcTo(x1: number, y1: number, x2: number, y2: number, r: number): void;
  rect(x: number, y: number, w: number, h: number): void;
  closePath(): void;
};

/** 🖊️ A path context that records every command, so geometry is comparable without rasterising. */
export type VizPathRecorder = VizPathContext & { readonly commands: VizPathCommand[]; toString(): string };

function formatNumber(value: number): string {
  return Number.isInteger(value) ? String(value) : String(Math.round(value * 1e6) / 1e6);
}

/** 🖊️ Creates a recording path context. */
export function vizPathRecorder(): VizPathRecorder {
  const commands: VizPathCommand[] = [];
  const recorder: VizPathRecorder = {
    commands,
    moveTo: (x, y) => void commands.push({ op: "moveTo", args: [x, y] }),
    lineTo: (x, y) => void commands.push({ op: "lineTo", args: [x, y] }),
    quadraticCurveTo: (cx, cy, x, y) => void commands.push({ op: "quadraticCurveTo", args: [cx, cy, x, y] }),
    bezierCurveTo: (cx1, cy1, cx2, cy2, x, y) => void commands.push({ op: "bezierCurveTo", args: [cx1, cy1, cx2, cy2, x, y] }),
    arc: (x, y, r, a0, a1, counterclockwise = false) => void commands.push({ op: "arc", args: [x, y, r, a0, a1, counterclockwise ? 1 : 0] }),
    arcTo: (x1, y1, x2, y2, r) => void commands.push({ op: "arcTo", args: [x1, y1, x2, y2, r] }),
    rect: (x, y, w, h) => void commands.push({ op: "rect", args: [x, y, w, h] }),
    closePath: () => void commands.push({ op: "closePath", args: [] }),
    toString: () => commands.map((command) => `${command.op}(${command.args.map(formatNumber).join(",")})`).join(" "),
  };
  return recorder;
}
//#endregion 🔖️PathContext

//#region 🔖️CurveContract
/** ✒️ A curve interpolator: the state machine `line` and `area` drive point by point. */
export type VizCurve = {
  areaStart(): void;
  areaEnd(): void;
  lineStart(): void;
  lineEnd(): void;
  point(x: number, y: number): void;
};

/** ✒️ A curve factory bound to a path context, exactly d3's `curve(context)` shape. */
export type VizCurveFactory = (context: VizPathContext) => VizCurve;

const EPSILON = 1e-12;

type CurveState = { line: number; point: number };

function noop(): void {}
//#endregion 🔖️CurveContract

//#region 🔖️CurvesLinear
/** ✒️ `curveLinear`: straight segments between consecutive points. */
export const curveLinear: VizCurveFactory = (context) => {
  const state: CurveState = { line: Number.NaN, point: 0 };
  return {
    areaStart: () => void (state.line = 0),
    areaEnd: () => void (state.line = Number.NaN),
    lineStart: () => void (state.point = 0),
    lineEnd: () => {
      if (state.line || (state.line !== 0 && state.point === 1)) context.closePath();
      state.line = 1 - state.line;
    },
    point: (x, y) => {
      if (state.point === 0) {
        state.point = 1;
        if (state.line) context.lineTo(x, y);
        else context.moveTo(x, y);
        return;
      }
      if (state.point === 1) state.point = 2;
      context.lineTo(x, y);
    },
  };
};

/** ✒️ `curveLinearClosed`: a closed polygon through the points. */
export const curveLinearClosed: VizCurveFactory = (context) => {
  let point = 0;
  return {
    areaStart: noop,
    areaEnd: noop,
    lineStart: () => void (point = 0),
    lineEnd: () => {
      if (point) context.closePath();
    },
    point: (x, y) => {
      if (point) context.lineTo(x, y);
      else {
        point = 1;
        context.moveTo(x, y);
      }
    },
  };
};

/** ✒️ `curveStep` at a given transition fraction; 0 is step-before, 1 is step-after. */
export function curveStepAt(t: number): VizCurveFactory {
  return (context) => {
    const state: CurveState = { line: Number.NaN, point: 0 };
    let x = Number.NaN;
    let y = Number.NaN;
    let transition = t;
    return {
      areaStart: () => void (state.line = 0),
      areaEnd: () => void (state.line = Number.NaN),
      lineStart: () => {
        x = Number.NaN;
        y = Number.NaN;
        state.point = 0;
      },
      lineEnd: () => {
        if (transition > 0 && transition < 1 && state.point === 2) context.lineTo(x, y);
        if (state.line || (state.line !== 0 && state.point === 1)) context.closePath();
        if (state.line >= 0) {
          transition = 1 - transition;
          state.line = 1 - state.line;
        }
      },
      point: (px, py) => {
        if (state.point === 0) {
          state.point = 1;
          if (state.line) context.lineTo(px, py);
          else context.moveTo(px, py);
        } else {
          if (state.point === 1) state.point = 2;
          if (transition <= 0) {
            context.lineTo(x, py);
            context.lineTo(px, py);
          } else {
            const midpoint = x * (1 - transition) + px * transition;
            context.lineTo(midpoint, y);
            context.lineTo(midpoint, py);
          }
        }
        x = px;
        y = py;
      },
    };
  };
}

/** ✒️ `curveStep`, the midpoint transition. */
export const curveStep = curveStepAt(0.5);

/** ✒️ `curveStepBefore`. */
export const curveStepBefore = curveStepAt(0);

/** ✒️ `curveStepAfter`. */
export const curveStepAfter = curveStepAt(1);
//#endregion 🔖️CurvesLinear

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
    areaStart: noop,
    areaEnd: noop,
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
      areaStart: noop,
      areaEnd: noop,
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

/** ✒️ `curveCardinalClosed` at a given tension. */
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
      areaStart: noop,
      areaEnd: noop,
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

/** ✒️ `curveCardinalOpen` at a given tension. */
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

/** ✒️ `curveCardinal` at d3's default tension of 0. */
export const curveCardinal = curveCardinalTension(0);

/** ✒️ `curveCardinalClosed` at d3's default tension of 0. */
export const curveCardinalClosed = curveCardinalClosedTension(0);

/** ✒️ `curveCardinalOpen` at d3's default tension of 0. */
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

/** ✒️ `curveCatmullRom` at a given α; α = 0 collapses onto the uniform cardinal spline. */
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

/** ✒️ `curveCatmullRomClosed` at a given α. */
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
      areaStart: noop,
      areaEnd: noop,
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

/** ✒️ `curveCatmullRomOpen` at a given α. */
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

/** ✒️ `curveCatmullRom` at d3's default α of ½. */
export const curveCatmullRom = curveCatmullRomAlpha(0.5);

/** ✒️ `curveCatmullRomClosed` at d3's default α of ½. */
export const curveCatmullRomClosed = curveCatmullRomClosedAlpha(0.5);

/** ✒️ `curveCatmullRomOpen` at d3's default α of ½. */
export const curveCatmullRomOpen = curveCatmullRomOpenAlpha(0.5);
//#endregion 🔖️CurvesCatmullRom

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

/** ✒️ `curveMonotoneY`: the same interpolator with the axes exchanged. */
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

/** ✒️ `curveNatural`: the natural cubic spline through every point. */
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

/** ✒️ `curveBumpX` — the horizontal bump `linkHorizontal` draws with. */
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

/** ✒️ `curveBumpY` — the vertical bump `linkVertical` draws with. */
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
