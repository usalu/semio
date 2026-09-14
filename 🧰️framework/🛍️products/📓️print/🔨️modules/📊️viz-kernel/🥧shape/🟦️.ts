/** 🥧 Shape generators: the TypeScript twin of `semio-viz-shape`. Lines, areas, arcs with padding
 * and rounded corners, pies, links, ribbons and the stack layout, each writing into the same
 * recording path context the marks use.
 * @see ../../../🖋️latex/semio-viz-shape.sty
 */
import { curveBumpX, curveBumpY, curveLinear, vizPathRecorder, type VizCurveFactory, type VizPathCommand, type VizPathContext } from "../✒️mark/🟦️.ts";
import { stack, type VizStackSeries } from "../🧮transform/🟦️.ts";
import type { VizPoint } from "../🧬️schema/🟦️.ts";

//#region 🔖️LineArea
/** 📈️ Options of `\SemioVizLine`: the accessors, the curve and the definedness predicate. */
export type VizLineOptions<T> = {
  readonly x?: (row: T, index: number) => number;
  readonly y?: (row: T, index: number) => number;
  readonly defined?: (row: T, index: number) => boolean;
  readonly curve?: VizCurveFactory;
};

/** 📈️ `\SemioVizLine`: one polyline through the data, interpolated by the chosen curve. */
export function vizLine<T>(data: readonly T[], options: VizLineOptions<T> = {}, context?: VizPathContext): VizPathCommand[] {
  const recorder = context === undefined ? vizPathRecorder() : undefined;
  const sink = context ?? recorder!;
  const x = options.x ?? ((row: T) => (row as unknown as VizPoint)[0]);
  const y = options.y ?? ((row: T) => (row as unknown as VizPoint)[1]);
  const defined = options.defined ?? (() => true);
  const output = (options.curve ?? curveLinear)(sink);
  let defined0 = false;
  for (let i = 0; i <= data.length; i += 1) {
    const inside = i < data.length && defined(data[i]!, i);
    if (!inside === defined0) {
      defined0 = !defined0;
      if (defined0) output.lineStart();
      else output.lineEnd();
    }
    if (defined0) output.point(+x(data[i]!, i), +y(data[i]!, i));
  }
  return recorder?.commands ?? [];
}

/** 🏞️ Options of `\SemioVizArea`: the two boundaries, the curve and the definedness predicate. */
export type VizAreaOptions<T> = {
  readonly x0?: (row: T, index: number) => number;
  readonly y0?: (row: T, index: number) => number;
  readonly x1?: (row: T, index: number) => number;
  readonly y1?: (row: T, index: number) => number;
  readonly defined?: (row: T, index: number) => boolean;
  readonly curve?: VizCurveFactory;
};

/** 🏞️ `\SemioVizArea`: the band between two boundaries, closed at both ends. */
export function vizArea<T>(data: readonly T[], options: VizAreaOptions<T> = {}, context?: VizPathContext): VizPathCommand[] {
  const recorder = context === undefined ? vizPathRecorder() : undefined;
  const sink = context ?? recorder!;
  const x0 = options.x0 ?? ((row: T) => (row as unknown as VizPoint)[0]);
  const y0 = options.y0 ?? (() => 0);
  const x1 = options.x1;
  const y1 = options.y1 ?? ((row: T) => (row as unknown as VizPoint)[1]);
  const defined = options.defined ?? (() => true);
  const output = (options.curve ?? curveLinear)(sink);
  const n = data.length;
  const x0z = new Array<number>(n);
  const y0z = new Array<number>(n);
  let defined0 = false;
  let j = 0;
  for (let i = 0; i <= n; i += 1) {
    const inside = i < n && defined(data[i]!, i);
    if (!inside === defined0) {
      defined0 = !defined0;
      if (defined0) {
        j = i;
        output.areaStart();
        output.lineStart();
      } else {
        output.lineEnd();
        output.lineStart();
        for (let k = i - 1; k >= j; k -= 1) output.point(x0z[k]!, y0z[k]!);
        output.lineEnd();
        output.areaEnd();
      }
    }
    if (defined0) {
      x0z[i] = +x0(data[i]!, i);
      y0z[i] = +y0(data[i]!, i);
      output.point(x1 ? +x1(data[i]!, i) : x0z[i]!, y1 ? +y1(data[i]!, i) : y0z[i]!);
    }
  }
  return recorder?.commands ?? [];
}
//#endregion 🔖️LineArea

//#region 🔖️Arc
const EPSILON = 1e-12;
const HALF_PI = Math.PI / 2;
const TAU = 2 * Math.PI;

/** 🍩️ Options of `\SemioVizArc`; angles are radians measured clockwise from twelve o'clock. */
export type VizArcOptions = {
  readonly innerRadius: number;
  readonly outerRadius: number;
  readonly startAngle: number;
  readonly endAngle: number;
  readonly padAngle?: number;
  readonly padRadius?: number;
  readonly cornerRadius?: number;
};

function intersect(x0: number, y0: number, x1: number, y1: number, x2: number, y2: number, x3: number, y3: number): [number, number] | null {
  const x10 = x1 - x0;
  const y10 = y1 - y0;
  const x32 = x3 - x2;
  const y32 = y3 - y2;
  let t = y32 * x10 - x32 * y10;
  if (t * t < EPSILON) return null;
  t = (x32 * (y0 - y2) - y32 * (x0 - x2)) / t;
  return [x0 + t * x10, y0 + t * y10];
}

type CornerTangent = { cx: number; cy: number; x01: number; y01: number; x11: number; y11: number };

function cornerTangents(x0: number, y0: number, x1: number, y1: number, r1: number, rc: number, cw: boolean): CornerTangent {
  const x01 = x0 - x1;
  const y01 = y0 - y1;
  const lo = (cw ? rc : -rc) / Math.sqrt(x01 * x01 + y01 * y01);
  const ox = lo * y01;
  const oy = -lo * x01;
  const x11 = x0 + ox;
  const y11 = y0 + oy;
  const x10 = x1 + ox;
  const y10 = y1 + oy;
  const x00 = (x11 + x10) / 2;
  const y00 = (y11 + y10) / 2;
  const dx = x10 - x11;
  const dy = y10 - y11;
  const d2 = dx * dx + dy * dy;
  const r = r1 - rc;
  const bigD = x11 * y10 - x10 * y11;
  const d = (dy < 0 ? -1 : 1) * Math.sqrt(Math.max(0, r * r * d2 - bigD * bigD));
  let cx0 = (bigD * dy - dx * d) / d2;
  let cy0 = (-bigD * dx - dy * d) / d2;
  const cx1 = (bigD * dy + dx * d) / d2;
  const cy1 = (-bigD * dx + dy * d) / d2;
  const dx0 = cx0 - x00;
  const dy0 = cy0 - y00;
  const dx1 = cx1 - x00;
  const dy1 = cy1 - y00;
  if (dx0 * dx0 + dy0 * dy0 > dx1 * dx1 + dy1 * dy1) {
    cx0 = cx1;
    cy0 = cy1;
  }
  return { cx: cx0, cy: cy0, x01: -ox, y01: -oy, x11: cx0 * (r1 / r - 1), y11: cy0 * (r1 / r - 1) };
}

/** 🍩️ `\SemioVizArc`: a circular or annular sector with optional padding and rounded corners. */
export function vizArc(options: VizArcOptions, context?: VizPathContext): VizPathCommand[] {
  const recorder = context === undefined ? vizPathRecorder() : undefined;
  const sink = context ?? recorder!;
  let r0 = +options.innerRadius;
  let r1 = +options.outerRadius;
  const a0 = options.startAngle - HALF_PI;
  const a1 = options.endAngle - HALF_PI;
  const da = Math.abs(a1 - a0);
  const cw = a1 > a0;
  if (r1 < r0) {
    const r = r1;
    r1 = r0;
    r0 = r;
  }
  if (!(r1 > EPSILON)) sink.moveTo(0, 0);
  else if (da > TAU - EPSILON) {
    sink.moveTo(r1 * Math.cos(a0), r1 * Math.sin(a0));
    sink.arc(0, 0, r1, a0, a1, !cw);
    if (r0 > EPSILON) {
      sink.moveTo(r0 * Math.cos(a1), r0 * Math.sin(a1));
      sink.arc(0, 0, r0, a1, a0, cw);
    }
  } else {
    let a01 = a0;
    let a11 = a1;
    let a00 = a0;
    let a10 = a1;
    let da0 = da;
    let da1 = da;
    const ap = (options.padAngle ?? 0) / 2;
    const rp = ap > EPSILON ? (options.padRadius !== undefined ? +options.padRadius : Math.sqrt(r0 * r0 + r1 * r1)) : 0;
    const rc = Math.min(Math.abs(r1 - r0) / 2, options.cornerRadius ?? 0);
    let rc0 = rc;
    let rc1 = rc;
    if (rp > EPSILON) {
      let p0 = Math.asin(((rp / r0) * Math.sin(ap)) || 0);
      let p1 = Math.asin((rp / r1) * Math.sin(ap));
      da0 -= p0 * 2;
      if (da0 > EPSILON) {
        p0 *= cw ? 1 : -1;
        a00 += p0;
        a10 -= p0;
      } else {
        da0 = 0;
        a00 = (a0 + a1) / 2;
        a10 = a00;
      }
      da1 -= p1 * 2;
      if (da1 > EPSILON) {
        p1 *= cw ? 1 : -1;
        a01 += p1;
        a11 -= p1;
      } else {
        da1 = 0;
        a01 = (a0 + a1) / 2;
        a11 = a01;
      }
    }
    const x01 = r1 * Math.cos(a01);
    const y01 = r1 * Math.sin(a01);
    const x10 = r0 * Math.cos(a10);
    const y10 = r0 * Math.sin(a10);
    if (rc > EPSILON) {
      const x11 = r1 * Math.cos(a11);
      const y11 = r1 * Math.sin(a11);
      const x00 = r0 * Math.cos(a00);
      const y00 = r0 * Math.sin(a00);
      if (da < Math.PI) {
        const oc = intersect(x01, y01, x00, y00, x11, y11, x10, y10);
        if (oc !== null) {
          const ax = x01 - oc[0];
          const ay = y01 - oc[1];
          const bx = x11 - oc[0];
          const by = y11 - oc[1];
          const kc = 1 / Math.sin(Math.acos((ax * bx + ay * by) / (Math.sqrt(ax * ax + ay * ay) * Math.sqrt(bx * bx + by * by))) / 2);
          const lc = Math.sqrt(oc[0] * oc[0] + oc[1] * oc[1]);
          rc0 = Math.min(rc, (r0 - lc) / (kc - 1));
          rc1 = Math.min(rc, (r1 - lc) / (kc + 1));
        } else {
          rc0 = 0;
          rc1 = 0;
        }
      }
    }
    if (!(da1 > EPSILON)) sink.moveTo(x01, y01);
    else if (rc1 > EPSILON) {
      const x11 = r1 * Math.cos(a11);
      const y11 = r1 * Math.sin(a11);
      const x00 = r0 * Math.cos(a00);
      const y00 = r0 * Math.sin(a00);
      const t0 = cornerTangents(x00, y00, x01, y01, r1, rc1, cw);
      const t1 = cornerTangents(x11, y11, x10, y10, r1, rc1, cw);
      sink.moveTo(t0.cx + t0.x01, t0.cy + t0.y01);
      if (rc1 < rc) sink.arc(t0.cx, t0.cy, rc1, Math.atan2(t0.y01, t0.x01), Math.atan2(t1.y01, t1.x01), !cw);
      else {
        sink.arc(t0.cx, t0.cy, rc1, Math.atan2(t0.y01, t0.x01), Math.atan2(t0.y11, t0.x11), !cw);
        sink.arc(0, 0, r1, Math.atan2(t0.cy + t0.y11, t0.cx + t0.x11), Math.atan2(t1.cy + t1.y11, t1.cx + t1.x11), !cw);
        sink.arc(t1.cx, t1.cy, rc1, Math.atan2(t1.y11, t1.x11), Math.atan2(t1.y01, t1.x01), !cw);
      }
    } else {
      sink.moveTo(x01, y01);
      sink.arc(0, 0, r1, a01, a11, !cw);
    }
    if (!(r0 > EPSILON) || !(da0 > EPSILON)) sink.lineTo(x10, y10);
    else if (rc0 > EPSILON) {
      const x11 = r1 * Math.cos(a11);
      const y11 = r1 * Math.sin(a11);
      const x00 = r0 * Math.cos(a00);
      const y00 = r0 * Math.sin(a00);
      const t0 = cornerTangents(x10, y10, x11, y11, r0, -rc0, cw);
      const t1 = cornerTangents(x01, y01, x00, y00, r0, -rc0, cw);
      sink.lineTo(t0.cx + t0.x01, t0.cy + t0.y01);
      if (rc0 < rc) sink.arc(t0.cx, t0.cy, rc0, Math.atan2(t0.y01, t0.x01), Math.atan2(t1.y01, t1.x01), !cw);
      else {
        sink.arc(t0.cx, t0.cy, rc0, Math.atan2(t0.y01, t0.x01), Math.atan2(t0.y11, t0.x11), !cw);
        sink.arc(0, 0, r0, Math.atan2(t0.cy + t0.y11, t0.cx + t0.x11), Math.atan2(t1.cy + t1.y11, t1.cx + t1.x11), cw);
        sink.arc(t1.cx, t1.cy, rc0, Math.atan2(t1.y11, t1.x11), Math.atan2(t1.y01, t1.x01), !cw);
      }
    } else sink.arc(0, 0, r0, a10, a00, cw);
  }
  sink.closePath();
  return recorder?.commands ?? [];
}

/** 🍩️ The centroid of an arc, where a slice label is anchored. */
export function vizArcCentroid(options: VizArcOptions): VizPoint {
  const r = (+options.innerRadius + +options.outerRadius) / 2;
  const a = (options.startAngle + options.endAngle) / 2 - HALF_PI;
  return [Math.cos(a) * r, Math.sin(a) * r];
}
//#endregion 🔖️Arc

//#region 🔖️Pie
/** 🥧 One slice produced by `\SemioVizPie`. */
export type VizPieSlice<T> = { readonly data: T; readonly index: number; readonly value: number; readonly startAngle: number; readonly endAngle: number; readonly padAngle: number };

/** 🥧 Options of `\SemioVizPie`; the default sort is by descending value, exactly like d3. */
export type VizPieOptions<T> = {
  readonly value?: (row: T, index: number) => number;
  readonly sort?: ((a: T, b: T) => number) | null;
  readonly sortValues?: ((a: number, b: number) => number) | null;
  readonly startAngle?: number;
  readonly endAngle?: number;
  readonly padAngle?: number;
};

/** 🥧 `\SemioVizPie`: turns values into angular slices of the circle. */
export function vizPie<T>(data: readonly T[], options: VizPieOptions<T> = {}): VizPieSlice<T>[] {
  const value = options.value ?? ((row: T) => Number(row));
  const n = data.length;
  const index = Array.from({ length: n }, (_, i) => i);
  const values = data.map((row, i) => +value(row, i));
  let total = 0;
  for (const v of values) if (v > 0) total += v;
  const a0Start = options.startAngle ?? 0;
  const da = Math.min(TAU, Math.max(-TAU, (options.endAngle ?? TAU) - a0Start));
  const padAngle = options.padAngle ?? 0;
  const p = Math.min(Math.abs(da) / n, padAngle);
  const pa = p * (da < 0 ? -1 : 1);
  const sortValues = options.sortValues === undefined ? (a: number, b: number) => b - a : options.sortValues;
  if (sortValues !== null && options.sort === undefined) index.sort((i, j) => sortValues(values[i]!, values[j]!));
  else if (options.sort !== undefined && options.sort !== null) index.sort((i, j) => options.sort!(data[i]!, data[j]!));
  const k = total ? (da - n * pa) / total : 0;
  const slices = new Array<VizPieSlice<T>>(n);
  let a0 = a0Start;
  for (let i = 0; i < n; i += 1) {
    const j = index[i]!;
    const v = values[j]!;
    const a1 = a0 + (v > 0 ? v * k : 0) + pa;
    slices[j] = { data: data[j]!, index: i, value: v, startAngle: a0, endAngle: a1, padAngle: p };
    a0 = a1;
  }
  return slices;
}
//#endregion 🔖️Pie

//#region 🔖️Link
/** 🔗️ The three link orientations `\SemioVizLink` draws. */
export type VizLinkKind = "horizontal" | "vertical" | "radial";

/** 🔗️ Polar to cartesian, the convention `linkRadial` and the radial curves share. */
export function vizPointRadial(angle: number, radius: number): VizPoint {
  const a = angle - HALF_PI;
  return [radius * Math.cos(a), radius * Math.sin(a)];
}

/** 🔗️ `\SemioVizLink`: a smooth connector between a source and a target point. For the radial kind
 * both points are `(angle, radius)` pairs; for the others they are cartesian. */
export function vizLink(kind: VizLinkKind, source: VizPoint, target: VizPoint, context?: VizPathContext): VizPathCommand[] {
  const recorder = context === undefined ? vizPathRecorder() : undefined;
  const sink = context ?? recorder!;
  if (kind === "radial") {
    const mid = (source[1] + target[1]) / 2;
    const p0 = vizPointRadial(source[0], source[1]);
    const p1 = vizPointRadial(source[0], mid);
    const p2 = vizPointRadial(target[0], mid);
    const p3 = vizPointRadial(target[0], target[1]);
    sink.moveTo(p0[0], p0[1]);
    sink.bezierCurveTo(p1[0], p1[1], p2[0], p2[1], p3[0], p3[1]);
    return recorder?.commands ?? [];
  }
  const output = (kind === "vertical" ? curveBumpY : curveBumpX)(sink);
  output.lineStart();
  output.point(source[0], source[1]);
  output.point(target[0], target[1]);
  output.lineEnd();
  return recorder?.commands ?? [];
}
//#endregion 🔖️Link

//#region 🔖️Ribbon
/** 🎀️ One end of a ribbon: an angular span at a radius. */
export type VizRibbonEnd = { readonly startAngle: number; readonly endAngle: number; readonly radius: number };

/** 🎀️ `\SemioVizRibbon`: the two-ended band a chord diagram is drawn from. */
export function vizRibbon(source: VizRibbonEnd, target: VizRibbonEnd, options: { headRadius?: number } = {}, context?: VizPathContext): VizPathCommand[] {
  const recorder = context === undefined ? vizPathRecorder() : undefined;
  const sink = context ?? recorder!;
  const sa0 = source.startAngle - HALF_PI;
  const sa1 = source.endAngle - HALF_PI;
  const sr = source.radius;
  const sx0 = sr * Math.cos(sa0);
  const sy0 = sr * Math.sin(sa0);
  const ta0 = target.startAngle - HALF_PI;
  const ta1 = target.endAngle - HALF_PI;
  const tr = target.radius;
  const headRadius = options.headRadius ?? 0;
  sink.moveTo(sx0, sy0);
  sink.arc(0, 0, sr, sa0, sa1);
  if (sa0 !== ta0 || sa1 !== ta1) {
    if (headRadius > 0) {
      const sa2 = (sa0 + sa1) / 2;
      const ta2 = (ta0 + ta1) / 2;
      const hr = headRadius;
      sink.quadraticCurveTo(0, 0, hr * Math.cos(ta0), hr * Math.sin(ta0));
      sink.lineTo(tr * Math.cos(ta2), tr * Math.sin(ta2));
      sink.lineTo(hr * Math.cos(ta1), hr * Math.sin(ta1));
      sink.quadraticCurveTo(0, 0, sr * Math.cos(sa2), sr * Math.sin(sa2));
    } else {
      sink.quadraticCurveTo(0, 0, tr * Math.cos(ta0), tr * Math.sin(ta0));
      sink.arc(0, 0, tr, ta0, ta1);
    }
  }
  sink.quadraticCurveTo(0, 0, sx0, sy0);
  sink.closePath();
  return recorder?.commands ?? [];
}
//#endregion 🔖️Ribbon

//#region 🔖️Stack
export { stack as vizStack };

export type { VizStackSeries };
//#endregion 🔖️Stack
