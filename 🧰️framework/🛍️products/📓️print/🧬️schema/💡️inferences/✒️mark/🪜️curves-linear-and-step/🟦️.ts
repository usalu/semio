/** 🪜️ The linear, closed linear and step curve interpolators of `\SemioVizPath`, numerically identical to d3-shape. */
import { type VizCurveFactory, vizCurveNoop } from "../🟦️.ts";

//#region 🔖️CurveContract
type CurveState = { line: number; point: number };
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

/** 📕️ `curveLinearClosed`: a closed polygon through the points. */
export const curveLinearClosed: VizCurveFactory = (context) => {
  let point = 0;
  return {
    areaStart: vizCurveNoop,
    areaEnd: vizCurveNoop,
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

/** 👣️ `curveStep` at a given transition fraction; 0 is step-before, 1 is step-after. */
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

/** 🚶️ `curveStep`, the midpoint transition. */
export const curveStep = curveStepAt(0.5);

/** ⏮️ `curveStepBefore`. */
export const curveStepBefore = curveStepAt(0);

/** ⏭️ `curveStepAfter`. */
export const curveStepAfter = curveStepAt(1);
//#endregion 🔖️CurvesLinear
