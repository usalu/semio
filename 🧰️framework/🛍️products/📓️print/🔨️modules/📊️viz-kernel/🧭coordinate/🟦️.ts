/** 🧭 Coordinate systems: the TypeScript twin of `semio-viz-coordinate`. Each system maps a pair of
 * scaled data values onto figure millimetres inside the plot rectangle, and every invertible one
 * reports its inverse so a hit test or an annotation can travel back.
 * @see ../../../🖋️latex/semio-viz-coordinate.sty
 */
import type { VizCoordinateKind, VizCoordinateOptions, VizExtent, VizPoint } from "../🧬️schema/🟦️.ts";

//#region 🔖️Contract
/** 🧭️ A coordinate system: a forward map, an optional inverse, and the frame it works in. */
export type VizCoordinateSystem = {
  readonly kind: VizCoordinateKind;
  readonly frame: VizExtent;
  project(u: number, v: number): VizPoint;
  invert?(x: number, y: number): VizPoint;
  /** 🧭️ The axes this system draws, as unit-interval anchors in its own space. */
  axes(): readonly { readonly name: string; readonly from: VizPoint; readonly to: VizPoint }[];
};

const TAU = 2 * Math.PI;

function span(frame: VizExtent): { width: number; height: number } {
  return { width: frame.x1 - frame.x0, height: frame.y1 - frame.y0 };
}
//#endregion 🔖️Contract

//#region 🔖️Systems
/** 🧭️ `cartesian`: the unit square onto the frame, `v` growing upward as a chart expects. */
export function coordinateCartesian(frame: VizExtent, options: VizCoordinateOptions = {}): VizCoordinateSystem {
  const { width, height } = span(frame);
  const transpose = options.transpose === true;
  return {
    kind: "cartesian",
    frame,
    project: (u, v) => (transpose ? [frame.x0 + v * width, frame.y1 - u * height] : [frame.x0 + u * width, frame.y1 - v * height]),
    invert: (x, y) => (transpose ? [(frame.y1 - y) / height, (x - frame.x0) / width] : [(x - frame.x0) / width, (frame.y1 - y) / height]),
    axes: () => [
      { name: "x", from: [0, 0], to: [1, 0] },
      { name: "y", from: [0, 0], to: [0, 1] },
    ],
  };
}

/** 🧭️ `polar`: `u` is the angular fraction, `v` the radial fraction between the two radii. */
export function coordinatePolar(frame: VizExtent, options: VizCoordinateOptions = {}): VizCoordinateSystem {
  const { width, height } = span(frame);
  const cx = frame.x0 + width / 2;
  const cy = frame.y0 + height / 2;
  const startAngle = options.startAngle ?? 0;
  const endAngle = options.endAngle ?? TAU;
  const outer = options.outerRadius ?? Math.min(width, height) / 2;
  const inner = options.innerRadius ?? 0;
  return {
    kind: "polar",
    frame,
    project: (u, v) => {
      const angle = startAngle + u * (endAngle - startAngle) - Math.PI / 2;
      const radius = inner + v * (outer - inner);
      return [cx + radius * Math.cos(angle), cy + radius * Math.sin(angle)];
    },
    invert: (x, y) => {
      const dx = x - cx;
      const dy = y - cy;
      const radius = Math.hypot(dx, dy);
      let angle = Math.atan2(dy, dx) + Math.PI / 2 - startAngle;
      const sweep = endAngle - startAngle;
      angle = ((angle % TAU) + TAU) % TAU;
      if (sweep < 0) angle -= TAU;
      return [sweep === 0 ? 0 : angle / sweep, outer === inner ? 0 : (radius - inner) / (outer - inner)];
    },
    axes: () => [
      { name: "angle", from: [0, 1], to: [1, 1] },
      { name: "radius", from: [0, 0], to: [0, 1] },
    ],
  };
}

/** 🧭️ `logpolar`: the polar system with a logarithmic radius, for spiral and decade plots. */
export function coordinateLogPolar(frame: VizExtent, options: VizCoordinateOptions = {}): VizCoordinateSystem {
  const polar = coordinatePolar(frame, options);
  const base = options.base ?? 10;
  const toLog = (v: number) => Math.log(1 + v * (base - 1)) / Math.log(base);
  const fromLog = (v: number) => (base ** v - 1) / (base - 1);
  return {
    kind: "logpolar",
    frame,
    project: (u, v) => polar.project(u, toLog(v)),
    invert: (x, y) => {
      const [u, v] = polar.invert!(x, y);
      return [u, fromLog(v)];
    },
    axes: polar.axes,
  };
}

/** 🧭️ `ternary`: three barycentric weights onto the equilateral triangle inscribed in the frame. */
export function coordinateTernary(frame: VizExtent): VizCoordinateSystem & { projectTernary(a: number, b: number, c: number): VizPoint } {
  const { width, height } = span(frame);
  const side = Math.min(width, height / (Math.sqrt(3) / 2));
  const originX = frame.x0 + (width - side) / 2;
  const originY = frame.y1 - (height - (side * Math.sqrt(3)) / 2) / 2;
  const apexX = originX + side / 2;
  const apexY = originY - (side * Math.sqrt(3)) / 2;
  const projectTernary = (a: number, b: number, c: number): VizPoint => {
    const total = a + b + c;
    const [wa, wb, wc] = total === 0 ? [1 / 3, 1 / 3, 1 / 3] : [a / total, b / total, c / total];
    return [originX * wa + (originX + side) * wb + apexX * wc, originY * wa + originY * wb + apexY * wc];
  };
  return {
    kind: "ternary",
    frame,
    projectTernary,
    project: (u, v) => projectTernary(1 - u - v, u, v),
    axes: () => [
      { name: "a", from: [0, 0], to: [1, 0] },
      { name: "b", from: [1, 0], to: [0, 1] },
      { name: "c", from: [0, 1], to: [0, 0] },
    ],
  };
}

/** 🧭️ `barycentric`: weights over an arbitrary polygon of anchors, the generalised ternary plot. */
export function coordinateBarycentric(frame: VizExtent, anchorCount: number): VizCoordinateSystem & { projectWeights(weights: readonly number[]): VizPoint } {
  const { width, height } = span(frame);
  const cx = frame.x0 + width / 2;
  const cy = frame.y0 + height / 2;
  const radius = Math.min(width, height) / 2;
  const anchors: VizPoint[] = Array.from({ length: anchorCount }, (_, i) => {
    const angle = (TAU * i) / anchorCount - Math.PI / 2;
    return [cx + radius * Math.cos(angle), cy + radius * Math.sin(angle)];
  });
  const projectWeights = (weights: readonly number[]): VizPoint => {
    let total = 0;
    for (const weight of weights) total += weight;
    if (total === 0) return [cx, cy];
    let x = 0;
    let y = 0;
    weights.forEach((weight, i) => {
      x += (weight / total) * anchors[i % anchorCount]![0];
      y += (weight / total) * anchors[i % anchorCount]![1];
    });
    return [x, y];
  };
  return {
    kind: "barycentric",
    frame,
    projectWeights,
    project: (u, v) => projectWeights([1 - u, u * (1 - v), u * v]),
    axes: () => anchors.map((_, i) => ({ name: `w${i}`, from: [0, 0] as VizPoint, to: [1, i / Math.max(1, anchorCount - 1)] as VizPoint })),
  };
}

/** 🧭️ `parallel`: one vertical axis per named dimension, `u` selecting the axis. */
export function coordinateParallel(frame: VizExtent, options: VizCoordinateOptions = {}): VizCoordinateSystem {
  const { width, height } = span(frame);
  const names = options.axes ?? [];
  const count = Math.max(1, names.length);
  const step = count > 1 ? width / (count - 1) : 0;
  return {
    kind: "parallel",
    frame,
    project: (u, v) => [frame.x0 + u * (count - 1) * step, frame.y1 - v * height],
    invert: (x, y) => [step === 0 ? 0 : (x - frame.x0) / (step * (count - 1)), (frame.y1 - y) / height],
    axes: () => names.map((name, i) => ({ name, from: [count > 1 ? i / (count - 1) : 0, 0] as VizPoint, to: [count > 1 ? i / (count - 1) : 0, 1] as VizPoint })),
  };
}

/** 🧭️ `geographic`: the frame handed to a `🌍geo` projection, which owns the spherical mathematics. */
export function coordinateGeographic(frame: VizExtent, project: (lon: number, lat: number) => VizPoint, invert?: (x: number, y: number) => VizPoint): VizCoordinateSystem {
  return {
    kind: "geographic",
    frame,
    project,
    invert,
    axes: () => [
      { name: "longitude", from: [-180, 0], to: [180, 0] },
      { name: "latitude", from: [0, -90], to: [0, 90] },
    ],
  };
}
//#endregion 🔖️Systems

//#region 🔖️Dispatch
/** 🧭️ `\SemioVizCoordinate`: builds the named coordinate system for a plot rectangle. */
export function buildVizCoordinate(kind: VizCoordinateKind, frame: VizExtent, options: VizCoordinateOptions = {}): VizCoordinateSystem {
  switch (kind) {
    case "polar":
      return coordinatePolar(frame, options);
    case "logpolar":
      return coordinateLogPolar(frame, options);
    case "ternary":
      return coordinateTernary(frame);
    case "barycentric":
      return coordinateBarycentric(frame, Math.max(3, options.axes?.length ?? 3));
    case "parallel":
      return coordinateParallel(frame, options);
    case "geographic":
      throw new Error("the geographic coordinate system is built with coordinateGeographic and a projection from 🌍geo");
    default:
      return coordinateCartesian(frame, options);
  }
}
//#endregion 🔖️Dispatch
