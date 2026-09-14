/** 🧮️ Data transforms: the TypeScript twin of `semio-viz-transform`. Filtering, sorting, grouping,
 * rollups, folding, pivoting, windows, binning, summary statistics, kernel density estimation,
 * regression and the stack layout with every d3 order and offset.
 * @see ../../../🖋️latex/semio-viz-transform.sty
 */
import { bisectRight, quantileSorted, tickIncrement, ticks } from "../📐scale/🟦️.ts";
import type { VizRow, VizStackOffset, VizStackOrder, VizTable } from "../🧬️schema/🟦️.ts";

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
