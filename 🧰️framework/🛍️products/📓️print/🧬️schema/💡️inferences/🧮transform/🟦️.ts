/** 🧮️ Data transforms: the TypeScript twin of `semio-viz-transform`. Filtering, sorting, grouping,
 * rollups, folding, pivoting, windows, binning, summary statistics, kernel density estimation,
 * regression and the stack layout with every d3 order and offset.
 * @see ../../../🖋️latex/semio-viz-transform.sty
 */
import { bisectRight, quantileSorted, tickIncrement, ticks } from "../📐scale/🟦️.ts";
import { VIZ_LAYOUT_ALGORITHMS, VIZ_STACK_OFFSETS, VIZ_STACK_ORDERS, VIZ_PROJECTIONS, VIZ_TREEMAP_TILINGS, type VizChartSpecification, type VizLayerSpec, type VizOptionValue, type VizPoint, type VizRow, type VizStackOffset, type VizStackOrder, type VizTable } from "../../📸️snapshot/📊️chart/🟦️.ts";
import * as hierarchy from "../🌳hierarchy/🟦️.ts";
import * as network from "../🕸️network/🟦️.ts";
import * as flow from "../🌊flow/🟦️.ts";
import * as spatial from "../📍spatial/🟦️.ts";
import { vizPie } from "../🥧shape/🟦️.ts";
import { vizGeoProjection } from "../🌍geo/🟦️.ts";

//#region 🔖️Statistics
/** ➕️ Neumaier compensated summation, the full-precision accumulator `d3.fsum` uses. */
export function fsum(values: Iterable<number>): number {
  let hi = 0;
  let lo = 0;
  for (const value of values) {
    const x = +value;
    const sum = hi + x;
    lo += Math.abs(hi) >= Math.abs(x) ? hi - sum + x : x - sum + hi;
    hi = sum;
  }
  return hi + lo;
}

/** ➕️ Plain sum, skipping null and non-finite entries the way `d3.sum` does. */
export function sum(values: Iterable<number>): number {
  let total = 0;
  for (const value of values) {
    const x = +value;
    if (x) total += x;
  }
  return total;
}

/** 📊️ Arithmetic mean of the defined values, `undefined` when none are defined. */
export function mean(values: Iterable<number>): number | undefined {
  let total = 0;
  let count = 0;
  for (const value of values) {
    const x = +value;
    if (value !== null && !Number.isNaN(x)) {
      total += x;
      count += 1;
    }
  }
  return count > 0 ? total / count : undefined;
}

/** 📊️ Unbiased sample variance by Welford's recurrence, exactly `d3.variance`. */
export function variance(values: Iterable<number>): number | undefined {
  let count = 0;
  let delta: number;
  let currentMean = 0;
  let total = 0;
  for (const value of values) {
    const x = +value;
    if (value !== null && !Number.isNaN(x)) {
      count += 1;
      delta = x - currentMean;
      currentMean += delta / count;
      total += delta * (x - currentMean);
    }
  }
  return count > 1 ? total / (count - 1) : undefined;
}

/** 📊️ Sample standard deviation. */
export function deviation(values: Iterable<number>): number | undefined {
  const v = variance(values);
  return v === undefined ? undefined : Math.sqrt(v);
}

/** 📊️ The smallest and largest defined value, `undefined` when there is none. */
export function extent(values: Iterable<number>): [number, number] | [undefined, undefined] {
  let min: number | undefined;
  let max: number | undefined;
  for (const value of values) {
    const x = +value;
    if (value === null || Number.isNaN(x)) continue;
    if (min === undefined || x < min) min = x;
    if (max === undefined || x > max) max = x;
  }
  return min === undefined || max === undefined ? [undefined, undefined] : [min, max];
}

/** 📊️ The R-7 quantile of an unsorted sample. */
export function quantile(values: Iterable<number>, p: number): number {
  const sorted = [...values].filter((value) => value !== null && !Number.isNaN(+value)).sort((a, b) => a - b);
  return quantileSorted(sorted, p);
}

/** 📊️ The median. */
export function median(values: Iterable<number>): number {
  return quantile(values, 0.5);
}

/** 📊️ The running sums of a sequence. */
export function cumulativeSum(values: Iterable<number>): number[] {
  let total = 0;
  const out: number[] = [];
  for (const value of values) {
    total += +value;
    out.push(total);
  }
  return out;
}
//#endregion 🔖️Statistics

//#region 🔖️Binning
/** 📦️ One histogram bin: its half-open span and the rows that fell into it. */
export type VizBin<T> = { readonly x0: number; readonly x1: number; readonly values: readonly T[] };

/** 📦️ The bin-count rules `\SemioVizTransform{bin}` accepts. */
export type VizThresholdRule = "sturges" | "scott" | "freedman-diaconis";

/** 📦️ Sturges' rule for a sample size. */
export function thresholdSturges(count: number): number {
  return Math.max(1, Math.ceil(Math.log(count) / Math.LN2) + 1);
}

/** 📦️ Scott's normal reference rule. */
export function thresholdScott(values: readonly number[], min: number, max: number): number {
  const count = values.length;
  const d = deviation(values);
  return count > 0 && d !== undefined && d > 0 ? Math.ceil((max - min) * count ** (1 / 3) / (3.49 * d)) : 1;
}

/** 📦️ The Freedman–Diaconis rule. */
export function thresholdFreedmanDiaconis(values: readonly number[], min: number, max: number): number {
  const count = values.length;
  const spread = quantile(values, 0.75) - quantile(values, 0.25);
  return count > 0 && spread > 0 ? Math.ceil((max - min) / (2 * spread * count ** (-1 / 3))) : 1;
}

function niceBinDomain(start: number, stop: number, count: number): [number, number] {
  let lo = start;
  let hi = stop;
  let prestep: number | undefined;
  for (;;) {
    const step = tickIncrement(lo, hi, count);
    if (step === prestep || step === 0 || !Number.isFinite(step)) return [lo, hi];
    if (step > 0) {
      lo = Math.floor(lo / step) * step;
      hi = Math.ceil(hi / step) * step;
    } else {
      lo = Math.ceil(lo * step) / step;
      hi = Math.floor(hi * step) / step;
    }
    prestep = step;
  }
}

/** 📦️ `d3.bin`: histogram bins over a sample, with the domain niced exactly as d3 nices it. */
export function bin<T>(data: readonly T[], options: { value?: (row: T) => number; domain?: readonly [number, number]; thresholds?: number | readonly number[] | VizThresholdRule } = {}): VizBin<T>[] {
  const value = options.value ?? ((row: T) => Number(row));
  const values = data.map(value);
  const explicitDomain = options.domain !== undefined;
  const [rawMin, rawMax] = explicitDomain ? options.domain! : extent(values);
  let x0 = Number(rawMin ?? 0);
  let x1 = Number(rawMax ?? 1);
  const rule = options.thresholds ?? "sturges";
  let tz: number[];
  if (Array.isArray(rule)) tz = [...(rule as readonly number[])];
  else {
    const defined = values.filter((entry) => entry !== null && !Number.isNaN(entry));
    const count = typeof rule === "number" ? rule : rule === "scott" ? thresholdScott(defined, x0, x1) : rule === "freedman-diaconis" ? thresholdFreedmanDiaconis(defined, x0, x1) : thresholdSturges(defined.length);
    const max = x1;
    if (!explicitDomain) [x0, x1] = niceBinDomain(x0, x1, count);
    tz = ticks(x0, x1, count);
    if (tz.length > 0 && tz[tz.length - 1]! >= x1) {
      if (max >= x1 && !explicitDomain) {
        const step = tickIncrement(x0, x1, count);
        if (Number.isFinite(step)) {
          if (step > 0) x1 = (Math.floor(x1 / step) + 1) * step;
          else if (step < 0) x1 = (Math.ceil(x1 * -step) + 1) / -step;
        }
      } else tz.pop();
    }
  }
  let m = tz.length;
  let a = 0;
  let b = m;
  while (tz[a] !== undefined && tz[a]! <= x0) a += 1;
  while (b > 0 && tz[b - 1]! > x1) b -= 1;
  if (a > 0 || b < m) {
    tz = tz.slice(a, b);
    m = b - a;
  }
  const buckets: T[][] = Array.from({ length: m + 1 }, () => []);
  for (let i = 0; i < data.length; i += 1) {
    const x = values[i]!;
    if (x !== null && !Number.isNaN(x) && x0 <= x && x <= x1) buckets[bisectRight(tz, x, 0, m)]!.push(data[i]!);
  }
  return buckets.map((rows, i) => ({ x0: i > 0 ? tz[i - 1]! : x0, x1: i < m ? tz[i]! : x1, values: rows }));
}
//#endregion 🔖️Binning

//#region 🔖️Grouping
/** 🗂️ `d3.group`: rows keyed by one accessor, in first-appearance order. */
export function group<T, K>(data: Iterable<T>, key: (row: T) => K): Map<K, T[]> {
  const out = new Map<K, T[]>();
  for (const row of data) {
    const k = key(row);
    const bucket = out.get(k);
    if (bucket === undefined) out.set(k, [row]);
    else bucket.push(row);
  }
  return out;
}

/** 🗂️ `d3.rollup`: rows grouped by one accessor and reduced to one value each. */
export function rollup<T, K, R>(data: Iterable<T>, reduce: (rows: T[]) => R, key: (row: T) => K): Map<K, R> {
  const out = new Map<K, R>();
  for (const [k, rows] of group(data, key)) out.set(k, reduce(rows));
  return out;
}

/** 🗂️ Long-to-wide: one row per `index` value, one column per `key` value. */
export function pivot(table: VizTable, index: string, key: string, value: string): VizTable {
  const keys = [...new Set(table.rows.map((row) => String(row[key])))];
  const rows: VizRow[] = [];
  for (const [indexValue, bucket] of group(table.rows, (row) => String(row[index]))) {
    const row: Record<string, number | string | boolean | null> = { [index]: indexValue };
    for (const k of keys) row[k] = bucket.find((candidate) => String(candidate[key]) === k)?.[value] ?? null;
    rows.push(row);
  }
  return { name: `${table.name}-pivot`, columns: [index, ...keys], rows };
}

/** 🗂️ Wide-to-long: the named columns folded into a key column and a value column. */
export function fold(table: VizTable, columns: readonly string[], keyName = "key", valueName = "value"): VizTable {
  const kept = table.columns.filter((column) => !columns.includes(column));
  const rows: VizRow[] = [];
  for (const row of table.rows) {
    for (const column of columns) {
      const folded: Record<string, number | string | boolean | null> = {};
      for (const keep of kept) folded[keep] = row[keep] ?? null;
      folded[keyName] = column;
      folded[valueName] = row[column] ?? null;
      rows.push(folded);
    }
  }
  return { name: `${table.name}-fold`, columns: [...kept, keyName, valueName], rows };
}

/** 🗂️ A centred or trailing moving aggregate over one column. */
export function window(values: readonly number[], size: number, reduce: (frame: readonly number[]) => number, centred = false): number[] {
  const half = centred ? Math.floor(size / 2) : 0;
  return values.map((_, i) => {
    const lo = Math.max(0, i - (size - 1) + half);
    const hi = Math.min(values.length, i + half + 1);
    return reduce(values.slice(lo, hi));
  });
}

/** 🗂️ Rescales a column onto `[0, 1]` by its own extent; a degenerate column becomes all zeros. */
export function normalize(values: readonly number[]): number[] {
  const [lo, hi] = extent(values);
  if (lo === undefined || hi === undefined || hi === lo) return values.map(() => 0);
  return values.map((value) => (value - lo) / (hi - lo));
}
//#endregion 🔖️Grouping

//#region 🔖️Density
/** 🔔️ The smoothing kernels `\SemioVizTransform{kde}` offers. */
export const VIZ_KERNELS = ["gaussian", "epanechnikov", "triangular", "uniform"] as const;

export type VizKernel = (typeof VIZ_KERNELS)[number];

/** 🔔️ One smoothing kernel evaluated at a standardised distance. */
export function vizKernel(kind: VizKernel): (u: number) => number {
  switch (kind) {
    case "epanechnikov":
      return (u) => (Math.abs(u) <= 1 ? 0.75 * (1 - u * u) : 0);
    case "triangular":
      return (u) => (Math.abs(u) <= 1 ? 1 - Math.abs(u) : 0);
    case "uniform":
      return (u) => (Math.abs(u) <= 1 ? 0.5 : 0);
    default:
      return (u) => Math.exp(-0.5 * u * u) / Math.sqrt(2 * Math.PI);
  }
}

/** 🔔️ Silverman's rule-of-thumb bandwidth. */
export function silvermanBandwidth(values: readonly number[]): number {
  const n = values.length;
  const d = deviation(values) ?? 0;
  const iqr = quantile(values, 0.75) - quantile(values, 0.25);
  const spread = iqr > 0 ? Math.min(d, iqr / 1.349) : d;
  return spread > 0 ? 0.9 * spread * n ** (-1 / 5) : 1;
}

/** 🔔️ One-dimensional kernel density estimate evaluated on a grid. */
export function kernelDensity1d(values: readonly number[], grid: readonly number[], options: { bandwidth?: number; kernel?: VizKernel } = {}): number[] {
  const bandwidth = options.bandwidth ?? silvermanBandwidth(values);
  const k = vizKernel(options.kernel ?? "gaussian");
  return grid.map((x) => values.reduce((total, value) => total + k((x - value) / bandwidth), 0) / (values.length * bandwidth));
}
//#endregion 🔖️Density

//#region 🔖️Regression
/** 📈️ An ordinary-least-squares line with its coefficient of determination. */
export type VizLinearFit = { readonly slope: number; readonly intercept: number; readonly r2: number; readonly predict: (x: number) => number };

/** 📈️ Least-squares straight-line fit through the given points. */
export function linearRegression(points: readonly (readonly [number, number])[]): VizLinearFit {
  const n = points.length;
  const meanX = points.reduce((total, point) => total + point[0], 0) / n;
  const meanY = points.reduce((total, point) => total + point[1], 0) / n;
  let sxy = 0;
  let sxx = 0;
  let syy = 0;
  for (const [x, y] of points) {
    sxy += (x - meanX) * (y - meanY);
    sxx += (x - meanX) ** 2;
    syy += (y - meanY) ** 2;
  }
  const slope = sxx === 0 ? 0 : sxy / sxx;
  const intercept = meanY - slope * meanX;
  const r2 = sxx === 0 || syy === 0 ? 0 : (sxy * sxy) / (sxx * syy);
  return { slope, intercept, r2, predict: (x: number) => intercept + slope * x };
}

/** 📈️ Least-squares polynomial fit of the requested order, by normal equations. */
export function polynomialRegression(points: readonly (readonly [number, number])[], order: number): number[] {
  const size = order + 1;
  const matrix: number[][] = Array.from({ length: size }, () => new Array<number>(size + 1).fill(0));
  for (let i = 0; i < size; i += 1) {
    for (let j = 0; j < size; j += 1) matrix[i]![j] = points.reduce((total, [x]) => total + x ** (i + j), 0);
    matrix[i]![size] = points.reduce((total, [x, y]) => total + y * x ** i, 0);
  }
  for (let i = 0; i < size; i += 1) {
    let pivot = i;
    for (let r = i + 1; r < size; r += 1) if (Math.abs(matrix[r]![i]!) > Math.abs(matrix[pivot]![i]!)) pivot = r;
    const tmp = matrix[i]!;
    matrix[i] = matrix[pivot]!;
    matrix[pivot] = tmp;
    const head = matrix[i]![i]!;
    if (head === 0) continue;
    for (let c = i; c <= size; c += 1) matrix[i]![c]! /= head;
    for (let r = 0; r < size; r += 1) {
      if (r === i) continue;
      const factor = matrix[r]![i]!;
      for (let c = i; c <= size; c += 1) matrix[r]![c]! -= factor * matrix[i]![c]!;
    }
  }
  return matrix.map((row) => row[size]!);
}
//#endregion 🔖️Regression

//#region 🔖️Stack
/** 🧱️ One stacked point: the baseline, the top, and the row it came from. */
export type VizStackPoint = { 0: number; 1: number; readonly index: number };

/** 🧱️ One stacked series: the key it encodes and its points in row order. */
export type VizStackSeries = { readonly key: string; index: number; readonly points: VizStackPoint[] };

function seriesSum(series: VizStackSeries): number {
  let total = 0;
  for (const point of series.points) {
    const value = +point[1];
    if (!Number.isNaN(value)) total += value;
  }
  return total;
}

/** 🧱️ The series order for one of the six stack orders. */
export function stackOrder(series: readonly VizStackSeries[], order: VizStackOrder): number[] {
  const n = series.length;
  const none = Array.from({ length: n }, (_, i) => i);
  switch (order) {
    case "ascending":
      return none.sort((a, b) => seriesSum(series[a]!) - seriesSum(series[b]!));
    case "descending":
      return stackOrder(series, "ascending").reverse();
    case "reverse":
      return none.reverse();
    case "appearance": {
      const peaks = series.map((entry) => {
        let index = 0;
        let best = Number.NEGATIVE_INFINITY;
        entry.points.forEach((point, i) => {
          const value = +point[1];
          if (value > best) {
            best = value;
            index = i;
          }
        });
        return index;
      });
      return none.sort((a, b) => peaks[a]! - peaks[b]!);
    }
    case "inside-out": {
      const appearance = stackOrder(series, "appearance");
      const sums = series.map(seriesSum);
      const top: number[] = [];
      const bottom: number[] = [];
      let topSum = 0;
      let bottomSum = 0;
      for (const i of appearance) {
        if (topSum < bottomSum) {
          topSum += sums[i]!;
          top.push(i);
        } else {
          bottomSum += sums[i]!;
          bottom.push(i);
        }
      }
      return [...bottom.reverse(), ...top];
    }
    default:
      return none;
  }
}

function offsetNone(series: readonly VizStackSeries[], order: readonly number[]): void {
  const n = series.length;
  if (!(n > 1)) return;
  let s1 = series[order[0]!]!;
  const m = s1.points.length;
  for (let i = 1; i < n; i += 1) {
    const s0 = s1;
    s1 = series[order[i]!]!;
    for (let j = 0; j < m; j += 1) {
      s1.points[j]![0] = Number.isNaN(s0.points[j]![1]) ? s0.points[j]![0] : s0.points[j]![1];
      s1.points[j]![1] += s1.points[j]![0];
    }
  }
}

/** 🧱️ Applies one of the five stack offsets in place. */
export function stackOffset(series: readonly VizStackSeries[], order: readonly number[], offset: VizStackOffset): void {
  const n = series.length;
  if (n === 0) return;
  const m = series[0]!.points.length;
  if (offset === "expand") {
    for (let j = 0; j < m; j += 1) {
      let total = 0;
      for (const entry of series) total += entry.points[j]![1] || 0;
      if (total !== 0) for (const entry of series) entry.points[j]![1] /= total;
    }
    offsetNone(series, order);
    return;
  }
  if (offset === "diverging") {
    for (let j = 0; j < m; j += 1) {
      let yp = 0;
      let yn = 0;
      for (const index of order) {
        const point = series[index]!.points[j]!;
        const d = point[1] - point[0];
        if (d > 0) {
          point[0] = yp;
          yp += d;
          point[1] = yp;
        } else if (d < 0) {
          point[1] = yn;
          yn += d;
          point[0] = yn;
        } else {
          point[0] = 0;
          point[1] = d;
        }
      }
    }
    return;
  }
  if (offset === "silhouette") {
    const first = series[order[0]!]!;
    for (let j = 0; j < m; j += 1) {
      let y = 0;
      for (const entry of series) y += entry.points[j]![1] || 0;
      first.points[j]![0] = -y / 2;
      first.points[j]![1] += first.points[j]![0];
    }
    offsetNone(series, order);
    return;
  }
  if (offset === "wiggle") {
    const first = series[order[0]!]!;
    if (m === 0) return;
    let y = 0;
    let j = 1;
    for (; j < m; j += 1) {
      let s1 = 0;
      let s2 = 0;
      for (let i = 0; i < n; i += 1) {
        const si = series[order[i]!]!;
        const sij0 = si.points[j]![1] || 0;
        const sij1 = si.points[j - 1]![1] || 0;
        let s3 = (sij0 - sij1) / 2;
        for (let k = 0; k < i; k += 1) {
          const sk = series[order[k]!]!;
          s3 += (sk.points[j]![1] || 0) - (sk.points[j - 1]![1] || 0);
        }
        s1 += sij0;
        s2 += s3 * sij0;
      }
      first.points[j - 1]![0] = y;
      first.points[j - 1]![1] += y;
      if (s1) y -= s2 / s1;
    }
    first.points[j - 1]![0] = y;
    first.points[j - 1]![1] += y;
    offsetNone(series, order);
    return;
  }
  offsetNone(series, order);
}

/** 🧱️ `d3.stack`: one series per key, stacked with the requested order and offset. */
export function stack<T>(data: readonly T[], keys: readonly string[], options: { value?: (row: T, key: string) => number; order?: VizStackOrder; offset?: VizStackOffset } = {}): VizStackSeries[] {
  const value = options.value ?? ((row: T, key: string) => Number((row as Record<string, unknown>)[key]));
  const series: VizStackSeries[] = keys.map((key) => ({ key, index: 0, points: data.map((row, index) => ({ 0: 0, 1: value(row, key), index })) }));
  const order = stackOrder(series, options.order ?? "none");
  order.forEach((seriesIndex, position) => {
    series[seriesIndex]!.index = position;
  });
  stackOffset(series, order, options.offset ?? "none");
  return series;
}
//#endregion 🔖️Stack

//#region 🔖️LayerInference
type LayerOptions = Readonly<Record<string, VizOptionValue | null | readonly (VizOptionValue | null)[]>>;
/** ⏳️ Nonpersisted progress and cancellation controls for expensive inferred layouts. */
export type VizLayerInferenceControl = { readonly signal?: AbortSignal; readonly onProgress?: (completed: number, total: number) => void };

const INFERRED_TRANSFORM_OPTIONS: Readonly<Record<string, string>> = { filter: "column operator value", sort: "column columns descending order", group: "column columns as", aggregate: "column groupby group operation as", rollup: "column groupby group operation as", summary: "column groupby group operation as", fold: "columns key value", pivot: "index key value operation", join: "table left right prefix", window: "column as groupby size operation centred", normalize: "column as groupby mode", cumulative: "column as groupby", bin: "column thresholds min max", stack: "keys order offset", quantile: "column probabilities", kde: "column min max samples bandwidth kernel", regression: "x y method order" };
const INFERRED_LAYOUT_OPTIONS: Readonly<Record<string, string>> = { pie: "value sort startAngle endAngle padAngle innerRadius outerRadius cx cy", tree: "id parent value sort leaves separation cousinSeparation", cluster: "id parent value sort leaves separation cousinSeparation", treemap: "id parent value sort leaves tile round paddingInner paddingOuter paddingTop paddingRight paddingBottom paddingLeft", partition: "id parent value sort leaves padding round", pack: "id parent value sort leaves padding", bundling: "id parent value sort leaves separation cousinSeparation", arc: "id source target origin vertical output", dag: "id source target layerGap nodeGap sweeps output", force: "id source target seed alpha velocityDecay charge distance radius iterations output", sankey: "source target value nodeWidth nodePadding iterations align output", alluvial: "stages value nodeWidth nodePadding iterations align output", chord: "source target value padAngle transpose output radius", projection: "x y projection scale translateX translateY rotateLongitude rotateLatitude rotateGamma centerLongitude centerLatitude reflectX reflectY", hexbin: "x y radius", jitter: "x y seed amount", beeswarm: "x y radius axis", voronoi: "x y x0 y0 x1 y1", delaunay: "x y", hull: "x y", contour: "value columns rows thresholds", density: "x y x0 y0 x1 y1 cellSize bandwidth kernel thresholds" };

function validateOptions(options: LayerOptions, keys: string | undefined, name: string): void {
  if (keys === undefined) throw new Error(`Unknown computation ${name}`);
  const allowed = new Set(keys.split(" "));
  for (const key of Object.keys(options)) if (!allowed.has(key)) throw new Error(`Unknown ${name} option ${key}`);
}

function inferredTable(name: string, rows: readonly VizRow[], columns: readonly string[] = []): VizTable {
  return { name, columns: [...new Set([...columns, ...rows.flatMap(Object.keys)])], rows };
}

function optionNumber(options: LayerOptions, key: string, fallback: number): number {
  const value = options[key] ?? fallback;
  if (typeof value !== "number" || !Number.isFinite(value)) throw new Error(`Invalid numeric option ${key}`);
  return value;
}

function optionList(value: LayerOptions[string] | undefined): string[] {
  return value === undefined ? [] : Array.isArray(value) ? value.map(String) : String(value).split(/[,;]/).map((part) => part.trim()).filter(Boolean);
}

function optionEnum<T extends string>(options: LayerOptions, key: string, values: readonly T[], fallback: T): T {
  const value = String(options[key] ?? fallback);
  if (!values.includes(value as T)) throw new Error(`Invalid ${key}: ${value}`);
  return value as T;
}

function columnName(table: VizTable, options: LayerOptions, key = "column", fallback = "value"): string {
  const column = String(options[key] ?? fallback);
  if (!table.columns.includes(column)) throw new Error(`Unknown column ${column} in table ${table.name}`);
  return column;
}

function compareValues(a: VizOptionValue | null | undefined, b: VizOptionValue | null | undefined): number {
  if (a === b) return 0;
  if (a === null || a === undefined) return -1;
  if (b === null || b === undefined) return 1;
  if (typeof a === "number" && typeof b === "number") return a - b;
  return String(a) < String(b) ? -1 : 1;
}

function numericValue(row: VizRow, column: string): number {
  const value = row[column];
  if (value === null || value === undefined || value === "" || !Number.isFinite(Number(value))) throw new Error(`Non-finite value in column ${column}`);
  return Number(value);
}

function aggregateValues(rows: readonly VizRow[], column: string, operation: string): number | null {
  if (operation === "count") return rows.length;
  const values = rows.filter((row) => row[column] !== null).map((row) => numericValue(row, column));
  switch (operation) {
    case "sum": return sum(values);
    case "mean": return mean(values) ?? null;
    case "min": return extent(values)[0] ?? null;
    case "max": return extent(values)[1] ?? null;
    case "median": return values.length ? median(values) : null;
    case "variance": return variance(values) ?? null;
    case "deviation": return deviation(values) ?? null;
    default: throw new Error(`Unknown aggregate ${operation}`);
  }
}

function inferHierarchyPath<T>(source: hierarchy.VizHierarchyNode<T>, target: hierarchy.VizHierarchyNode<T>): hierarchy.VizHierarchyNode<T>[] {
  const a = source.ancestors(), b = target.ancestors(), targetAncestors = new Set(b);
  const common = a.find((node) => targetAncestors.has(node));
  if (!common) throw new Error("Cannot bundle nodes of different hierarchies");
  return [...a.slice(0, a.indexOf(common) + 1), ...b.slice(0, b.indexOf(common)).reverse()];
}

function groupRows(table: VizTable, columns: readonly string[]): VizRow[][] {
  for (const column of columns) columnName(table, { column });
  if (!columns.length) return [[...table.rows]];
  return [...group(table.rows, (row) => JSON.stringify(columns.map((column) => row[column]))).values()];
}

function inferTransform(spec: VizChartSpecification, table: VizTable, kind: string, options: LayerOptions): VizTable {
  if (INFERRED_TRANSFORM_OPTIONS[kind] === undefined) throw new Error(`Unknown transform ${kind}`);
  validateOptions(options, INFERRED_TRANSFORM_OPTIONS[kind], kind);
  const result = (rows: readonly VizRow[], columns: readonly string[] = table.columns) => inferredTable(table.name, rows, columns);
  const output = String(options.as ?? options.column ?? "value");
  if (kind === "filter") {
    const column = columnName(table, options);
    const operator = optionEnum(options, "operator", ["eq", "ne", "lt", "lte", "gt", "gte", "in", "valid"] as const, "eq");
    const value = options.value;
    return result(table.rows.filter((row) => {
      const current = row[column];
      switch (operator) {
        case "eq": return current === value;
        case "ne": return current !== value;
        case "in": return Array.isArray(value) && value.includes(current as VizOptionValue);
        case "valid": return current !== null && current !== undefined && !(typeof current === "number" && !Number.isFinite(current));
        case "lt": return current !== null && current !== undefined && compareValues(current, value as VizOptionValue) < 0;
        case "lte": return current !== null && current !== undefined && compareValues(current, value as VizOptionValue) <= 0;
        case "gt": return current !== null && current !== undefined && compareValues(current, value as VizOptionValue) > 0;
        case "gte": return current !== null && current !== undefined && compareValues(current, value as VizOptionValue) >= 0;
      }
    }));
  }
  if (kind === "sort") {
    const columns = optionList(options.columns ?? options.column);
    if (!columns.length) throw new Error("Sort requires columns");
    for (const column of columns) columnName(table, { column });
    const descending = options.descending === true || optionEnum(options, "order", ["ascending", "descending"] as const, "ascending") === "descending";
    return result([...table.rows].sort((a, b) => {
      for (const column of columns) {
        const av = a[column], bv = b[column];
        const delta = compareValues(av, bv);
        if (delta) return descending ? -delta : delta;
      }
      return 0;
    }));
  }
  if (kind === "group") {
    const columns = optionList(options.columns ?? options.column);
    if (!columns.length) throw new Error("Group requires columns");
    return result(groupRows(table, columns).flatMap((rows, index) => rows.map((row) => ({ ...row, [String(options.as ?? "group")]: index }))));
  }
  if (kind === "aggregate" || kind === "rollup" || kind === "summary") {
    const columns = optionList(options.groupby ?? options.group);
    const column = columnName(table, options);
    const operation = String(options.operation ?? "sum");
    return result(groupRows(table, columns).map((rows) => ({ ...Object.fromEntries(columns.map((key) => [key, rows[0]?.[key] ?? null])), [output]: aggregateValues(rows, column, operation) })), [...columns, output]);
  }
  if (kind === "fold") {
    const columns = optionList(options.columns);
    for (const column of columns) columnName(table, { column });
    if (!columns.length) throw new Error("Fold requires columns");
    const folded = fold(table, columns, String(options.key ?? "key"), String(options.value ?? "value"));
    return { ...folded, name: table.name };
  }
  if (kind === "pivot") {
    const index = columnName(table, options, "index", "id"), key = columnName(table, options, "key", "key"), value = columnName(table, options, "value");
    const keys = [...new Set(table.rows.map((row) => String(row[key])))];
    const rows = groupRows(table, [index]).map((bucket) => ({ [index]: bucket[0]![index]!, ...Object.fromEntries(keys.map((k) => [k, aggregateValues(bucket.filter((row) => String(row[key]) === k), value, String(options.operation ?? "sum"))])) }));
    return result(rows, [index, ...keys]);
  }
  if (kind === "join") {
    const right = spec.tables?.find((entry) => entry.name === options.table);
    if (!right) throw new Error(`Unknown join table ${options.table}`);
    const leftKey = columnName(table, options, "left", "id"), rightKey = columnName(right, options, "right", leftKey);
    const prefix = String(options.prefix ?? "");
    const rows = table.rows.flatMap((row) => right.rows.filter((other) => other[rightKey] === row[leftKey]).map((other) => ({ ...row, ...Object.fromEntries(Object.entries(other).filter(([key]) => key !== rightKey).map(([key, value]) => [prefix + key, value])) })));
    return result(rows);
  }
  if (kind === "window" || kind === "normalize" || kind === "cumulative") {
    const column = columnName(table, options), rows: VizRow[] = [];
    for (const bucket of groupRows(table, optionList(options.groupby))) {
      const values = bucket.map((row) => numericValue(row, column));
      let computed: readonly (number | null)[];
      if (kind === "cumulative") computed = cumulativeSum(values);
      else if (kind === "normalize") {
        const mode = optionEnum(options, "mode", ["sum", "extent"] as const, "sum");
        const total = sum(values);
        computed = mode === "extent" ? normalize(values) : values.map((value) => total ? value / total : 0);
      } else {
        const size = optionNumber(options, "size", 3);
        if (!Number.isInteger(size) || size < 1) throw new Error("Window size must be a positive integer");
        computed = window(values, size, (frame) => aggregateValues(frame.map((value) => ({ value })), "value", String(options.operation ?? "mean")) ?? 0, options.centred === true);
      }
      bucket.forEach((row, index) => rows.push({ ...row, [output]: computed[index]! }));
    }
    const positions = new Map(table.rows.map((row, index) => [row, index]));
    const ordered = groupRows(table, optionList(options.groupby)).flat();
    return result(rows.map((row, index) => ({ row, index: positions.get(ordered[index]!)! })).sort((a, b) => a.index - b.index).map(({ row }) => row));
  }
  if (kind === "bin") {
    const column = columnName(table, options);
    const thresholds = Array.isArray(options.thresholds) ? options.thresholds.map(Number) : options.thresholds as number | VizThresholdRule | undefined;
    const domain = options.min !== undefined || options.max !== undefined ? [optionNumber(options, "min", 0), optionNumber(options, "max", 1)] as const : undefined;
    if (domain && domain[1] < domain[0]) throw new Error("Bin domain is reversed");
    return result(bin(table.rows, { value: (row) => numericValue(row, column), domain, thresholds }).map((bucket) => ({ x: bucket.x0, x2: bucket.x1, y: bucket.values.length, x0: bucket.x0, x1: bucket.x1, count: bucket.values.length })), ["x", "x2", "y", "x0", "x1", "count"]);
  }
  if (kind === "stack") {
    const keys = optionList(options.keys);
    if (!keys.length) throw new Error("Stack requires keys");
    for (const column of keys) columnName(table, { column });
    return result(stack(table.rows, keys, { value: (row, key) => numericValue(row, key), order: optionEnum(options, "order", VIZ_STACK_ORDERS, "none"), offset: optionEnum(options, "offset", VIZ_STACK_OFFSETS, "none") }).flatMap((series) => series.points.map((point) => ({ ...table.rows[point.index], key: series.key, index: series.index, y: point[1], y2: point[0], y0: point[0], y1: point[1], value: numericValue(table.rows[point.index]!, series.key) }))));
  }
  if (kind === "quantile") {
    const column = columnName(table, options), values = table.rows.map((row) => numericValue(row, column));
    const probabilities = options.probabilities === undefined ? [0, 0.25, 0.5, 0.75, 1] : optionList(options.probabilities).map(Number);
    if (probabilities.some((p) => !Number.isFinite(p) || p < 0 || p > 1)) throw new Error("Invalid quantile probabilities");
    return result(probabilities.map((p) => ({ p, value: values.length ? quantile(values, p) : null })), ["p", "value"]);
  }
  if (kind === "kde") {
    const column = columnName(table, options), values = table.rows.map((row) => numericValue(row, column));
    if (!values.length) return result([], ["x", "y", "density"]);
    const [lo, hi] = extent(values), min = optionNumber(options, "min", lo!), max = optionNumber(options, "max", hi!);
    const count = optionNumber(options, "samples", 100), bandwidth = optionNumber(options, "bandwidth", silvermanBandwidth(values));
    if (!Number.isInteger(count) || count < 2 || bandwidth <= 0 || max < min) throw new Error("Invalid density grid or bandwidth");
    const grid = Array.from({ length: count }, (_, i) => min + (max - min) * i / (count - 1));
    const densities = kernelDensity1d(values, grid, { bandwidth, kernel: optionEnum(options, "kernel", VIZ_KERNELS, "gaussian") });
    return result(grid.map((x, i) => ({ x, y: densities[i]!, density: densities[i]! })), ["x", "y", "density"]);
  }
  if (kind === "regression") {
    const x = columnName(table, options, "x", "x"), y = columnName(table, options, "y", "y");
    const points = table.rows.map((row) => [numericValue(row, x), numericValue(row, y)] as const);
    if (!points.length) return result([], ["x", "y"]);
    const method = optionEnum(options, "method", ["linear", "poly", "exp", "log", "pow"] as const, "linear");
    if (points.some(([xv, yv]) => ((method === "log" || method === "pow") && xv <= 0) || ((method === "exp" || method === "pow") && yv <= 0))) throw new Error(`Invalid ${method} regression domain`);
    const order = optionNumber(options, "order", 2);
    if (method === "poly" && (!Number.isInteger(order) || order < 0 || order >= points.length)) throw new Error("Invalid regression order");
    const transformed = points.map(([xv, yv]) => [method === "log" || method === "pow" ? Math.log(xv) : xv, method === "exp" || method === "pow" ? Math.log(yv) : yv] as const);
    const fit = linearRegression(transformed), coefficients = method === "poly" ? polynomialRegression(points, order) : [];
    return result(points.map(([xv]) => {
      const prediction = method === "poly" ? coefficients.reduce((total, c, i) => total + c * xv ** i, 0) : fit.predict(method === "log" || method === "pow" ? Math.log(xv) : xv);
      return { x: xv, y: method === "exp" || method === "pow" ? Math.exp(prediction) : prediction, slope: fit.slope, intercept: fit.intercept, r2: fit.r2 };
    }), ["x", "y", "slope", "intercept", "r2"]);
  }
  throw new Error(`Unknown transform ${kind}`);
}

/** 🧮️ Resolves authored transforms and layout geometry as the chart's pure layer inference. */
export function inferVizLayerTable(spec: VizChartSpecification, layer: VizLayerSpec, control: VizLayerInferenceControl = {}): VizTable {
  control.signal?.throwIfAborted();
  const source = layer.data === undefined ? spec.tables?.[0] : spec.tables?.find((table) => table.name === layer.data);
  if (!source) {
    if (layer.data !== undefined || layer.transform?.length || layer.layout) throw new Error(`Unknown layer table ${layer.data ?? "<first>"}`);
    return { name: "", columns: [], rows: [] };
  }
  const forceUnits = layer.layout?.algorithm === "force" ? optionNumber(layer.layout.options ?? {}, "iterations", 300) : 0;
  const total = source.rows.length + (layer.transform?.length ?? 0) + 1 + forceUnits;
  let completed = 0;
  const checkpoint = () => { control.signal?.throwIfAborted(); control.onProgress?.(completed, total); control.signal?.throwIfAborted(); };
  let table: VizTable = { ...source, columns: [...source.columns], rows: source.rows.map((row, index) => { completed += 1; if (index % 256 === 0) checkpoint(); return { ...row }; }) };
  for (const transform of layer.transform ?? []) { checkpoint(); table = inferTransform(spec, table, transform.kind, transform.options ?? {}); completed += 1; checkpoint(); }
  checkpoint();
  if (!layer.layout) { completed = total; checkpoint(); return table; }
  const { algorithm } = layer.layout, options: LayerOptions = layer.layout.options ?? {};
  if (!VIZ_LAYOUT_ALGORITHMS.includes(algorithm)) throw new Error(`Unknown layout ${algorithm}`);
  if (algorithm === "bin" || algorithm === "stack") { const derived = inferTransform(spec, table, algorithm, options); completed = total; checkpoint(); return derived; }
  validateOptions(options, "width height " + INFERRED_LAYOUT_OPTIONS[algorithm], algorithm);
  const margin = spec.margin ?? { top: 8, right: 8, bottom: 14, left: 16 };
  const width = optionNumber(options, "width", spec.width - margin.left - margin.right), height = optionNumber(options, "height", spec.height - margin.top - margin.bottom);
  if (width <= 0 || height <= 0) throw new Error("Layout extent must be positive");
  const result = (rows: readonly VizRow[], columns: readonly string[] = []) => { completed = total; checkpoint(); return inferredTable(table.name, rows, columns); };
  if (algorithm === "pie") {
    const column = columnName(table, options, "value", layer.encodings?.angle?.column ?? "value");
    const sort = optionEnum(options, "sort", ["ascending", "descending", "none"] as const, "descending");
    return result(vizPie(table.rows, { value: (row) => numericValue(row, column), sortValues: sort === "none" ? null : sort === "ascending" ? (a, b) => a - b : (a, b) => b - a, startAngle: optionNumber(options, "startAngle", 0), endAngle: optionNumber(options, "endAngle", 2 * Math.PI), padAngle: optionNumber(options, "padAngle", 0) }).map((slice) => ({ ...slice.data, index: slice.index, value: slice.value, startAngle: slice.startAngle, endAngle: slice.endAngle, padAngle: slice.padAngle, innerRadius: optionNumber(options, "innerRadius", 0), outerRadius: optionNumber(options, "outerRadius", Math.min(width, height) / 2), x: optionNumber(options, "cx", width / 2), y: optionNumber(options, "cy", height / 2) })));
  }
  if (["tree", "cluster", "treemap", "partition", "pack", "bundling"].includes(algorithm)) {
    if (!table.rows.length) return result([]);
    const id = columnName(table, options, "id", "id"), parent = columnName(table, options, "parent", "parent");
    const root = hierarchy.vizStratify(table.rows, { id: (row) => String(row[id]), parentId: (row) => row[parent] === null || row[parent] === "" ? null : String(row[parent]) });
    const value = String(options.value ?? "value");
    if (table.columns.includes(value)) root.sum((row) => numericValue(row, value)); else root.count();
    const sort = optionEnum(options, "sort", ["none", "ascending", "descending"] as const, "none");
    if (sort !== "none") root.sort((a, b) => (sort === "ascending" ? 1 : -1) * ((a.value ?? 0) - (b.value ?? 0)));
    const size = [width, height] as const;
    if (algorithm === "treemap") hierarchy.vizTreemap(root, { size, tile: optionEnum(options, "tile", VIZ_TREEMAP_TILINGS, "squarify"), round: options.round === true, paddingInner: optionNumber(options, "paddingInner", 0), paddingOuter: optionNumber(options, "paddingOuter", 0), paddingTop: optionNumber(options, "paddingTop", optionNumber(options, "paddingOuter", 0)), paddingRight: optionNumber(options, "paddingRight", optionNumber(options, "paddingOuter", 0)), paddingBottom: optionNumber(options, "paddingBottom", optionNumber(options, "paddingOuter", 0)), paddingLeft: optionNumber(options, "paddingLeft", optionNumber(options, "paddingOuter", 0)) });
    else if (algorithm === "partition") hierarchy.vizPartition(root, { size, padding: optionNumber(options, "padding", 0), round: options.round === true });
    else if (algorithm === "pack") hierarchy.vizPack(root, { size, padding: optionNumber(options, "padding", 0) });
    else (algorithm === "tree" ? hierarchy.vizTree : hierarchy.vizCluster)(root, { size, separation: (a, b) => (a.parent === b.parent ? optionNumber(options, "separation", 1) : optionNumber(options, "cousinSeparation", 2)) });
    const nodes = options.leaves === true ? root.leaves() : root.descendants();
    if (algorithm === "bundling") return result(root.links().map(({ source, target }, i) => ({ detail: i, points: JSON.stringify(inferHierarchyPath(target, source).map((node) => [node.x, node.y])), x: source.x, y: source.y, x2: target.x, y2: target.y })));
    return result(nodes.map((node) => ({ ...node.data, value: node.value ?? 0, depth: node.depth, x: algorithm === "treemap" || algorithm === "partition" ? node.x0 : node.x, y: algorithm === "treemap" || algorithm === "partition" ? node.y0 : node.y, x2: node.x1, y2: node.y1, x0: node.x0, y0: node.y0, x1: node.x1, y1: node.y1, radius: node.r, size: Math.PI * node.r ** 2, parentX: node.parent?.x ?? node.x, parentY: node.parent?.y ?? node.y })));
  }
  if (["arc", "dag", "force", "sankey", "alluvial", "chord"].includes(algorithm)) {
    const sourceColumn = String(options.source ?? "source"), targetColumn = String(options.target ?? "target"), valueColumn = String(options.value ?? "value");
    const hasLinks = table.columns.includes(sourceColumn) && table.columns.includes(targetColumn);
    const names = hasLinks ? [...new Set(table.rows.flatMap((row) => [String(row[sourceColumn]), String(row[targetColumn])]))] : table.rows.map((row, index) => String(row[String(options.id ?? "id")] ?? index));
    const edges = hasLinks ? table.rows.map((row) => [String(row[sourceColumn]), String(row[targetColumn])] as const) : [];
    if (["sankey", "alluvial", "chord"].includes(algorithm) && !hasLinks && algorithm !== "alluvial") throw new Error(`${algorithm} requires source and target columns`);
    const sankeyOptions = { extent: [[0, 0], [width, height]] as const, nodeWidth: optionNumber(options, "nodeWidth", 4), nodePadding: optionNumber(options, "nodePadding", 2), iterations: optionNumber(options, "iterations", 6), align: optionEnum(options, "align", flow.VIZ_SANKEY_ALIGNMENTS, "justify") };
    if (algorithm === "sankey" || algorithm === "alluvial") {
      const stages = optionList(options.stages);
      if (algorithm === "alluvial" && stages.length < 2) throw new Error("Alluvial requires at least two stages");
      for (const column of stages) columnName(table, { column });
      const layout = algorithm === "sankey" ? flow.vizSankey({ nodes: names.map((name) => ({ name })), links: table.rows.map((row) => ({ source: String(row[sourceColumn]), target: String(row[targetColumn]), value: numericValue(row, valueColumn) })) }, sankeyOptions) : flow.vizAlluvial(table.rows.map((row) => ({ stages: stages.map((key) => String(row[key])), value: numericValue(row, valueColumn) })), { ...sankeyOptions, stageNames: stages });
      return options.output === "nodes" ? result(layout.nodes.map((node) => ({ id: node.name, value: node.value, x: node.x0, y: node.y0, x2: node.x1, y2: node.y1, depth: node.depth }))) : result(layout.links.map((link) => ({ source: link.source.name, target: link.target.name, value: link.value, x: link.source.x1, y: link.y0, x2: link.target.x0, y2: link.y1, width: link.width })));
    }
    if (algorithm === "chord") {
      const matrix = names.map(() => names.map(() => 0));
      for (const row of table.rows) matrix[names.indexOf(String(row[sourceColumn]))]![names.indexOf(String(row[targetColumn]))]! += numericValue(row, valueColumn);
      const layout = network.vizChord(matrix, { padAngle: optionNumber(options, "padAngle", 0), transpose: options.transpose === true });
      return options.output === "groups" ? result(layout.groups.map((span) => ({ id: names[span.index]!, value: span.value, startAngle: span.startAngle, endAngle: span.endAngle }))) : result(layout.chords.map((chord) => ({ source: names[chord.source.index]!, target: names[chord.target.index]!, value: chord.source.value, startAngle: chord.source.startAngle, endAngle: chord.source.endAngle, targetStartAngle: chord.target.startAngle, targetEndAngle: chord.target.endAngle, radius: optionNumber(options, "radius", Math.min(width, height) / 2) })));
    }
    let nodes: readonly { id: string; x: number; y: number }[];
    if (algorithm === "arc") nodes = network.vizArcLayout({ nodes: names, edges }, { length: width, origin: optionNumber(options, "origin", 0), vertical: options.vertical === true });
    else if (algorithm === "dag") nodes = network.vizLayeredLayout({ nodes: names, edges }, { layerGap: optionNumber(options, "layerGap", 20), nodeGap: optionNumber(options, "nodeGap", 12), sweeps: optionNumber(options, "sweeps", 4) }).nodes;
    else {
      const placed = names.map((id, index) => ({ id, x: hasLinks ? NaN : Number(table.rows[index]?.x ?? NaN), y: hasLinks ? NaN : Number(table.rows[index]?.y ?? NaN), vx: 0, vy: 0 }));
      const links = edges.map(([a, b]) => ({ source: names.indexOf(a), target: names.indexOf(b) }));
      const simulation = new network.VizForceSimulation(placed, { seed: optionNumber(options, "seed", 1), alpha: optionNumber(options, "alpha", 1), velocityDecay: optionNumber(options, "velocityDecay", 0.4) });
      simulation.force("charge", network.forceVizManyBody({ strength: optionNumber(options, "charge", -30) })).force("center", network.forceVizCenter(width / 2, height / 2));
      if (links.length) simulation.force("link", network.forceVizLink(links, { distance: optionNumber(options, "distance", 30) }));
      if (options.radius !== undefined) simulation.force("collide", network.forceVizCollide({ radius: optionNumber(options, "radius", 1) }));
      const iterations = optionNumber(options, "iterations", 300);
      if (!Number.isInteger(iterations) || iterations < 0) throw new Error("Force iterations must be nonnegative integer");
      for (let iteration = 0; iteration < iterations; iteration += 1) { control.signal?.throwIfAborted(); simulation.tick(); completed += 1; checkpoint(); }
      nodes = placed;
    }
    if (options.output === "links") return result(edges.map(([source, target]) => { const a = nodes.find((node) => node.id === source)!, b = nodes.find((node) => node.id === target)!; return { source, target, x: a.x, y: a.y, x2: b.x, y2: b.y }; }));
    return result(nodes.map((node, index) => ({ ...(hasLinks ? {} : table.rows[index]), ...node })));
  }
  const xColumn = algorithm === "contour" ? "x" : columnName(table, options, "x", layer.encodings?.x?.column ?? "x"), yColumn = algorithm === "contour" ? "y" : columnName(table, options, "y", layer.encodings?.y?.column ?? "y");
  const points: VizPoint[] = algorithm === "contour" ? [] : table.rows.map((row) => [numericValue(row, xColumn), numericValue(row, yColumn)]);
  const bounds = [optionNumber(options, "x0", 0), optionNumber(options, "y0", 0), optionNumber(options, "x1", width), optionNumber(options, "y1", height)] as const;
  if (algorithm === "projection") {
    const projection = vizGeoProjection(optionEnum(options, "projection", VIZ_PROJECTIONS, "equirectangular"), { scale: optionNumber(options, "scale", width / (2 * Math.PI)), translate: [optionNumber(options, "translateX", width / 2), optionNumber(options, "translateY", height / 2)], rotate: [optionNumber(options, "rotateLongitude", 0), optionNumber(options, "rotateLatitude", 0), optionNumber(options, "rotateGamma", 0)], center: [optionNumber(options, "centerLongitude", 0), optionNumber(options, "centerLatitude", 0)], reflectX: options.reflectX === true, reflectY: options.reflectY === true });
    return result(table.rows.map((row, i) => { const [x, y] = projection(points[i]!); return { ...row, x, y }; }));
  }
  if (algorithm === "hexbin") return result(spatial.vizHexbin(table.rows, { radius: optionNumber(options, "radius", 1), x: (row) => numericValue(row, xColumn), y: (row) => numericValue(row, yColumn) }).map((bucket) => ({ x: bucket.x, y: bucket.y, count: bucket.values.length, value: bucket.values.length, radius: optionNumber(options, "radius", 1) })));
  if (algorithm === "jitter") {
    const random = hierarchy.vizLcg(optionNumber(options, "seed", 1)), amount = optionNumber(options, "amount", 1);
    return result(table.rows.map((row, i) => ({ ...row, x: points[i]![0] + (random() - 0.5) * amount, y: points[i]![1] + (random() - 0.5) * amount })));
  }
  if (algorithm === "beeswarm") {
    const radius = optionNumber(options, "radius", 1), placed: VizPoint[] = [], axis = optionEnum(options, "axis", ["x", "y"] as const, "x");
    if (radius <= 0) throw new Error("Beeswarm radius must be positive");
    const rows = table.rows.map((row, i) => {
      const point = points[i]!, value = axis === "x" ? point[0] : point[1], baseline = axis === "x" ? point[1] : point[0];
      const candidates = [baseline];
      for (const prior of placed) {
        const delta = Math.abs(value - prior[0]);
        if (delta < 2 * radius) { const offset = Math.sqrt(4 * radius * radius - delta * delta); candidates.push(prior[1] - offset, prior[1] + offset); }
      }
      candidates.sort((a, b) => Math.abs(a - baseline) - Math.abs(b - baseline) || a - b);
      const offset = candidates.find((candidate) => placed.every((prior) => (value - prior[0]) ** 2 + (candidate - prior[1]) ** 2 >= 4 * radius * radius - 1e-9))!;
      placed.push([value, offset]);
      return { ...row, x: axis === "x" ? value : offset, y: axis === "x" ? offset : value };
    });
    return result(rows);
  }
  const polygons = (shapes: readonly (readonly VizPoint[])[], values?: readonly number[]) => result(shapes.flatMap((polygon, detail) => { const serialized = JSON.stringify(polygon); return polygon.map(([x, y], order) => ({ x, y, detail, order, value: values?.[detail] ?? detail, points: serialized })); }));
  if (algorithm === "voronoi") return polygons(spatial.vizVoronoi(points, bounds).cells);
  if (algorithm === "delaunay") return polygons(spatial.vizDelaunay(points).triangles.map((triangle) => triangle.map((i) => points[i]!)));
  if (algorithm === "hull") return polygons([spatial.vizConvexHull(points).map((i) => points[i]!)]);
  if (algorithm === "contour" || algorithm === "density") {
    const thresholds = optionNumber(options, "thresholds", 10);
    if (!Number.isInteger(thresholds) || thresholds < 1) throw new Error("Contour thresholds must be a positive integer");
    let contours: spatial.VizContour[];
    if (algorithm === "density") {
      const cellSize = optionNumber(options, "cellSize", 1), bandwidth = optionNumber(options, "bandwidth", 10);
      if (cellSize <= 0 || bandwidth <= 0) throw new Error("Density cellSize and bandwidth must be positive");
      if (!points.length) return result([]);
      const grid = spatial.vizDensity2d(points, { extent: bounds, cellSize, bandwidth, kernel: optionEnum(options, "kernel", VIZ_KERNELS, "gaussian") });
      contours = spatial.vizDensityContours(grid, thresholds).map((contour) => ({ ...contour, coordinates: contour.coordinates.map((polygon) => polygon.map((ring) => ring.map(([x, y]) => [grid.x0 + x * grid.cellSize, grid.y0 + y * grid.cellSize] as VizPoint))) }));
    } else {
      const columns = optionNumber(options, "columns", width), rows = optionNumber(options, "rows", height);
      if (!Number.isInteger(columns) || !Number.isInteger(rows) || columns <= 0 || rows <= 0 || table.rows.length !== columns * rows) throw new Error("Contour requires a complete rectangular grid");
      contours = spatial.vizContours(table.rows.map((row) => numericValue(row, String(options.value ?? "value"))), [columns, rows], thresholds);
    }
    return polygons(contours.flatMap((contour) => contour.coordinates.flatMap((polygon) => polygon)), contours.flatMap((contour) => contour.coordinates.flatMap((polygon) => polygon.map(() => contour.value))));
  }
  throw new Error(`Unknown layout ${algorithm}`);
}
//#endregion 🔖️LayerInference
