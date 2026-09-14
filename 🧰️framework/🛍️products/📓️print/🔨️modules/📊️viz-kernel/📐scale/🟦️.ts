/** 📐️ Scales: the TypeScript twin of `semio-viz-scale`. Domain-to-range maps with the exact
 * arithmetic of d3-scale — continuous, log, power, symlog, identity, ordinal, band, point,
 * quantile, quantize, threshold, sequential, diverging and temporal — plus the d3-array tick
 * algorithm and the d3-time calendar intervals they all rest on.
 * @see ../../../🖋️latex/semio-viz-scale.sty
 */
import type { VizScaleKind, VizScaleOptions, VizScaleSpec } from "../🧬️schema/🟦️.ts";

//#region 🔖️Ticks
const E10 = Math.sqrt(50);
const E5 = Math.sqrt(10);
const E2 = Math.sqrt(2);

function tickSpec(start: number, stop: number, count: number): [number, number, number] {
  const step = (stop - start) / Math.max(0, count);
  const power = Math.floor(Math.log10(step));
  const error = step / 10 ** power;
  const factor = error >= E10 ? 10 : error >= E5 ? 5 : error >= E2 ? 2 : 1;
  let i1: number;
  let i2: number;
  let inc: number;
  if (power < 0) {
    inc = 10 ** -power / factor;
    i1 = Math.round(start * inc);
    i2 = Math.round(stop * inc);
    if (i1 / inc < start) i1 += 1;
    if (i2 / inc > stop) i2 -= 1;
    inc = -inc;
  } else {
    inc = 10 ** power * factor;
    i1 = Math.round(start / inc);
    i2 = Math.round(stop / inc);
    if (i1 * inc < start) i1 += 1;
    if (i2 * inc > stop) i2 -= 1;
  }
  if (i2 < i1 && count >= 0.5 && count < 2) return tickSpec(start, stop, count * 2);
  return [i1, i2, inc];
}

/** 🔢️ The d3-array tick sequence: human-readable, inclusive, and stable under reversal. */
export function ticks(start: number, stop: number, count: number): number[] {
  if (!(count > 0)) return [];
  if (start === stop) return [start];
  const reverse = stop < start;
  const [i1, i2, inc] = reverse ? tickSpec(stop, start, count) : tickSpec(start, stop, count);
  if (!(i2 >= i1)) return [];
  const n = i2 - i1 + 1;
  const out = new Array<number>(n);
  for (let i = 0; i < n; i += 1) {
    const index = reverse ? i2 - i : i1 + i;
    out[i] = inc < 0 ? index / -inc : index * inc;
  }
  return out;
}

/** 🔢️ The signed tick increment for a span, negative when it is the reciprocal of a step. */
export function tickIncrement(start: number, stop: number, count: number): number {
  return tickSpec(start, stop, count)[2];
}

/** 🔢️ The tick step as a plain multiplier. */
export function tickStep(start: number, stop: number, count: number): number {
  const reverse = stop < start;
  const inc = reverse ? tickIncrement(stop, start, count) : tickIncrement(start, stop, count);
  return (reverse ? -1 : 1) * (inc < 0 ? 1 / -inc : inc);
}

/** 🎯️ Insertion index of `x` in a sorted array, right-biased exactly like `d3.bisect`. */
export function bisectRight(values: readonly number[], x: number, lo = 0, hi = values.length): number {
  let low = lo;
  let high = hi;
  while (low < high) {
    const mid = (low + high) >>> 1;
    if (x < values[mid]!) high = mid;
    else low = mid + 1;
  }
  return low;
}

/** 🎯️ Insertion index of `x` in a sorted array, left-biased exactly like `d3.bisectLeft`. */
export function bisectLeft(values: readonly number[], x: number, lo = 0, hi = values.length): number {
  let low = lo;
  let high = hi;
  while (low < high) {
    const mid = (low + high) >>> 1;
    if (values[mid]! < x) low = mid + 1;
    else high = mid;
  }
  return low;
}
//#endregion 🔖️Ticks

//#region 🔖️Interpolation
/** 🔗️ Linear number interpolation, the default continuous-scale interpolator. */
export function interpolateNumber(a: number, b: number): (t: number) => number {
  const from = +a;
  const span = +b - from;
  return (t) => from + span * t;
}

/** 🔗️ Rounded number interpolation, the `round` variant of the range interpolator. */
export function interpolateRound(a: number, b: number): (t: number) => number {
  const from = +a;
  const span = +b - from;
  return (t) => Math.round(from + span * t);
}

function normalize(a: number, b: number): (x: number) => number {
  const span = b - a;
  return span ? (x) => (x - a) / span : () => (Number.isNaN(span) ? Number.NaN : 0.5);
}

function clamper(a: number, b: number): (x: number) => number {
  const lo = a < b ? a : b;
  const hi = a < b ? b : a;
  return (x) => Math.max(lo, Math.min(hi, x));
}

function bimap<T>(domain: readonly number[], range: readonly T[], interpolate: (a: T, b: T) => (t: number) => T): (x: number) => T {
  const d0 = domain[0]!;
  const d1 = domain[1]!;
  const r0 = range[0]!;
  const r1 = range[1]!;
  const from = d1 < d0 ? normalize(d1, d0) : normalize(d0, d1);
  const to = d1 < d0 ? interpolate(r1, r0) : interpolate(r0, r1);
  return (x) => to(from(x));
}

function polymap<T>(domain: readonly number[], range: readonly T[], interpolate: (a: T, b: T) => (t: number) => T): (x: number) => T {
  const j = Math.min(domain.length, range.length) - 1;
  const descending = domain[j]! < domain[0]!;
  const dom = descending ? [...domain].reverse() : [...domain];
  const ran = descending ? [...range].reverse() : [...range];
  const d = new Array<(x: number) => number>(j);
  const r = new Array<(t: number) => T>(j);
  for (let i = 0; i < j; i += 1) {
    d[i] = normalize(dom[i]!, dom[i + 1]!);
    r[i] = interpolate(ran[i]!, ran[i + 1]!);
  }
  return (x) => {
    const i = bisectRight(dom, x, 1, j) - 1;
    return r[i]!(d[i]!(x));
  };
}
//#endregion 🔖️Interpolation

//#region 🔖️Continuous
/** 📐️ A scale that maps values of `I` onto values of `O` and reports its own vocabulary. */
export type VizScale<I, O> = {
  (value: I): O;
  kind: VizScaleKind;
  domain(): readonly I[];
  range(): readonly O[];
  invert?(value: number): number;
  ticks?(count?: number): number[];
  tickFormat?(count?: number, specifier?: string): (value: number) => string;
  nice?(count?: number): VizScale<I, O>;
  bandwidth?(): number;
  step?(): number;
};

type ContinuousConfig = {
  readonly domain: readonly number[];
  readonly range: readonly number[];
  readonly clamp?: boolean;
  readonly round?: boolean;
  readonly transform?: (x: number) => number;
  readonly untransform?: (x: number) => number;
  readonly kind?: VizScaleKind;
};

/** 📐️ Continuous numeric scale with an optional monotone transform, exactly d3's `continuous()`. */
export function scaleContinuous(config: ContinuousConfig): VizScale<number, number> {
  const transform = config.transform ?? ((x: number) => x);
  const untransform = config.untransform ?? ((x: number) => x);
  const domain = config.domain.map((value) => +value);
  const range = config.range.map((value) => +value);
  const interpolate = config.round === true ? interpolateRound : interpolateNumber;
  const transformed = domain.map(transform);
  const piecewise = transformed.length > 2 || range.length > 2 ? polymap : bimap;
  const forward = piecewise(transformed, range, interpolate);
  const clampIn = config.clamp === true ? clamper(domain[0]!, domain[domain.length - 1]!) : (x: number) => x;
  const backward = piecewise(range, transformed, interpolateNumber);
  const scale = ((value: number) => {
    const x = +value;
    return Number.isNaN(x) ? Number.NaN : forward(transform(clampIn(x)));
  }) as VizScale<number, number>;
  scale.kind = config.kind ?? "linear";
  scale.domain = () => domain;
  scale.range = () => range;
  scale.invert = (value: number) => {
    const x = untransform(backward(+value));
    return config.clamp === true ? clamper(domain[0]!, domain[domain.length - 1]!)(x) : x;
  };
  scale.ticks = (count = 10) => ticks(domain[0]!, domain[domain.length - 1]!, count);
  scale.tickFormat = (count = 10, specifier?: string) => continuousTickFormat(domain[0]!, domain[domain.length - 1]!, count, specifier);
  scale.nice = (count = 10) => scaleContinuous({ ...config, domain: niceDomain(domain, count) });
  return scale;
}

/** 📐️ Extends a domain outward onto round tick values, iterating exactly like d3's `nice`. */
export function niceDomain(domain: readonly number[], count = 10): number[] {
  const d = [...domain];
  const i1 = d.length - 1;
  let start = d[0]!;
  let stop = d[i1]!;
  let swapped = false;
  if (stop < start) {
    const tmp = start;
    start = stop;
    stop = tmp;
    swapped = true;
  }
  let prestep: number | undefined;
  let maxIter = 10;
  while (maxIter > 0) {
    maxIter -= 1;
    const step = tickIncrement(start, stop, count);
    if (step === prestep) {
      if (swapped) {
        d[0] = stop;
        d[i1] = start;
      } else {
        d[0] = start;
        d[i1] = stop;
      }
      return d;
    }
    if (step > 0) {
      start = Math.floor(start / step) * step;
      stop = Math.ceil(stop / step) * step;
    } else if (step < 0) {
      start = Math.ceil(start * step) / step;
      stop = Math.floor(stop * step) / step;
    } else break;
    prestep = step;
  }
  return d;
}

function applyNice(domain: readonly number[], options: VizScaleOptions): readonly number[] {
  if (options.nice === undefined || options.nice === false) return domain;
  return niceDomain(domain, options.nice === true ? 10 : options.nice);
}

/** 📐️ `scaleLinear`. */
export function scaleLinear(domain: readonly number[] = [0, 1], range: readonly number[] = [0, 1], options: VizScaleOptions = {}): VizScale<number, number> {
  return scaleContinuous({ domain: applyNice(domain, options), range, clamp: options.clamp, round: options.round, kind: "linear" });
}

/** 📐️ `scaleIdentity`: domain and range are the same numbers, while ticks and formats still work. */
export function scaleIdentity(domain: readonly number[] = [0, 1]): VizScale<number, number> {
  return scaleContinuous({ domain, range: domain, kind: "identity" });
}

/** 📐️ `scalePow` with a signed power transform, so a negative domain keeps its sign. */
export function scalePow(domain: readonly number[] = [0, 1], range: readonly number[] = [0, 1], options: VizScaleOptions = {}): VizScale<number, number> {
  const exponent = options.exponent ?? 1;
  const raise = (x: number) => (x < 0 ? -((-x) ** exponent) : x ** exponent);
  const lower = (x: number) => (x < 0 ? -((-x) ** (1 / exponent)) : x ** (1 / exponent));
  return scaleContinuous({ domain: applyNice(domain, options), range, clamp: options.clamp, round: options.round, transform: raise, untransform: lower, kind: "pow" });
}

/** 📐️ `scaleSqrt`, the power scale at exponent ½. */
export function scaleSqrt(domain: readonly number[] = [0, 1], range: readonly number[] = [0, 1], options: VizScaleOptions = {}): VizScale<number, number> {
  const scale = scalePow(domain, range, { ...options, exponent: 0.5 });
  scale.kind = "sqrt";
  return scale;
}

/** 📐️ `scaleSymlog`: logarithmic away from zero and linear across it. */
export function scaleSymlog(domain: readonly number[] = [0, 1], range: readonly number[] = [0, 1], options: VizScaleOptions = {}): VizScale<number, number> {
  const constant = options.constant ?? 1;
  const forward = (x: number) => Math.sign(x) * Math.log1p(Math.abs(x / constant));
  const backward = (x: number) => Math.sign(x) * Math.expm1(Math.abs(x)) * constant;
  return scaleContinuous({ domain: applyNice(domain, options), range, clamp: options.clamp, round: options.round, transform: forward, untransform: backward, kind: "symlog" });
}

function logBase(x: number, base: number): number {
  if (base === Math.E) return Math.log(x);
  if (base === 10) return Math.log10(x);
  if (base === 2) return Math.log2(x);
  return Math.log(x) / Math.log(base);
}

function logNice(domain: readonly number[], logs: (x: number) => number, pows: (x: number) => number): number[] {
  const d = [...domain];
  d[0] = pows(Math.floor(logs(d[0]!)));
  d[d.length - 1] = pows(Math.ceil(logs(d[d.length - 1]!)));
  return d;
}

function logTicks(domain: readonly number[], base: number, count: number): number[] {
  let u = domain[0]!;
  let v = domain[domain.length - 1]!;
  const reverse = v < u;
  if (reverse) {
    const tmp = u;
    u = v;
    v = tmp;
  }
  const negative = u < 0;
  const logs = negative ? (x: number) => -logBase(-x, base) : (x: number) => logBase(x, base);
  const pows = negative ? (x: number) => -(base ** -x) : (x: number) => base ** x;
  let i = logs(u);
  let j = logs(v);
  let z: number[] = [];
  if (!(base % 1) && j - i < count) {
    i = Math.floor(i);
    j = Math.ceil(j);
    if (u > 0) {
      for (; i <= j; i += 1) {
        for (let k = 1; k < base; k += 1) {
          const t = i < 0 ? k / base ** -i : k * base ** i;
          if (t < u) continue;
          if (t > v) break;
          z.push(t);
        }
      }
    } else {
      for (; i <= j; i += 1) {
        for (let k = base - 1; k >= 1; k -= 1) {
          const t = i > 0 ? k / base ** -i : k * base ** i;
          if (t < u) continue;
          if (t > v) break;
          z.push(t);
        }
      }
    }
    if (z.length * 2 < count) z = ticks(u, v, count);
  } else {
    z = ticks(i, j, Math.min(j - i, count)).map(pows);
  }
  return reverse ? z.reverse() : z;
}

/** 📐️ `scaleLog` with the base-aware tick, nice and tick-format behaviour of d3. */
export function scaleLog(domain: readonly number[] = [1, 10], range: readonly number[] = [0, 1], options: VizScaleOptions = {}): VizScale<number, number> {
  const base = options.base ?? 10;
  const negative = (domain[0] ?? 1) < 0;
  const logs = negative ? (x: number) => -logBase(-x, base) : (x: number) => logBase(x, base);
  const pows = negative ? (x: number) => -(base ** -x) : (x: number) => base ** x;
  const nicedDomain = options.nice === undefined || options.nice === false ? [...domain] : logNice(domain, logs, pows);
  const scale = scaleContinuous({ domain: nicedDomain, range, clamp: options.clamp, round: options.round, transform: logs, untransform: pows, kind: "log" });
  scale.ticks = (count = 10) => logTicks(nicedDomain, base, count);
  scale.tickFormat = (count = 10, specifier?: string) => logTickFormat(nicedDomain, base, count, specifier);
  scale.nice = () => scaleLog(logNice(nicedDomain, logs, pows), range, options);
  return scale;
}

function logTickFormat(domain: readonly number[], base: number, count: number, specifier?: string): (value: number) => string {
  const format = specifier === undefined ? formatNumberSpec(base === 10 ? ".0e" : ",") : formatNumberSpec(specifier);
  if (count === Number.POSITIVE_INFINITY) return format;
  const limit = Math.max(1, (base * count) / logTicks(domain, base, 10).length);
  return (value: number) => {
    let i = value / base ** Math.round(logBase(value, base));
    if (i * base < base - 0.5) i *= base;
    return i <= limit ? format(value) : "";
  };
}
//#endregion 🔖️Continuous

//#region 🔖️TickFormat
type FormatParts = {
  readonly fill: string;
  readonly align: string;
  readonly sign: string;
  readonly symbol: string;
  readonly zero: boolean;
  readonly width?: number;
  readonly comma: boolean;
  readonly precision?: number;
  readonly trim: boolean;
  readonly type: string;
};

const SI_PREFIXES = ["y", "z", "a", "f", "p", "n", "µ", "m", "", "k", "M", "G", "T", "P", "E", "Z", "Y"];

function makeFormatter(parts: FormatParts): (value: number) => string {
  const type = parts.type === "" ? "g" : parts.type;
  const precision = parts.precision ?? (type === "d" ? 0 : type === "e" || type === "f" || type === "%" ? 6 : 12);
  return (value: number) => {
    let body: string;
    let suffix = "";
    const negative = value < 0;
    const absolute = Math.abs(value);
    if (type === "d") body = Math.round(absolute).toString();
    else if (type === "f") body = absolute.toFixed(precision);
    else if (type === "%") body = (absolute * 100).toFixed(precision);
    else if (type === "e") body = absolute.toExponential(precision);
    else if (type === "r") body = absolute.toPrecision(Math.max(1, precision));
    else if (type === "s") {
      const exponent = absolute === 0 ? 0 : Math.floor(Math.log10(absolute) / 3);
      const clamped = Math.max(-8, Math.min(8, exponent));
      body = (absolute / 1000 ** clamped).toPrecision(Math.max(1, precision));
      suffix = SI_PREFIXES[clamped + 8]!;
    } else body = absolute.toPrecision(Math.max(1, precision));
    if (parts.trim || type === "g" || type === "r" || type === "s") body = trimZeros(body);
    if (parts.comma) body = groupThousands(body);
    if (type === "%") suffix = `%${suffix}`;
    const signText = negative ? "-" : parts.sign === "+" ? "+" : parts.sign === " " ? " " : "";
    const rendered = `${signText}${parts.symbol === "$" ? "$" : ""}${body}${suffix}`;
    if (parts.width === undefined || rendered.length >= parts.width) return rendered;
    const pad = (parts.zero ? "0" : parts.fill).repeat(parts.width - rendered.length);
    if (parts.zero) return `${signText}${pad}${rendered.slice(signText.length)}`;
    if (parts.align === "<") return `${rendered}${pad}`;
    if (parts.align === "^") return `${pad.slice(0, pad.length >> 1)}${rendered}${pad.slice(pad.length >> 1)}`;
    return `${pad}${rendered}`;
  };
}

function trimZeros(text: string): string {
  if (!text.includes(".")) return text;
  const [mantissa, exponent] = text.split("e");
  const trimmed = mantissa!.replace(/\.?0+$/, "");
  return exponent === undefined ? trimmed : `${trimmed}e${exponent}`;
}

function groupThousands(text: string): string {
  const [integer, fraction] = text.split(".");
  const grouped = integer!.replace(/\B(?=(\d{3})+(?!\d))/g, ",");
  return fraction === undefined ? grouped : `${grouped}.${fraction}`;
}

/** 🔢️ The compact format engine the scale ticks need; the full grammar lives in `🔢format`. */
export function formatNumberSpec(specifier: string): (value: number) => string {
  const match = /^(?:(.)?([<>=^]))?([+\-( ])?([$#])?(0)?(\d+)?(,)?(\.\d+)?(~)?([a-z%])?$/i.exec(specifier);
  if (!match) throw new Error(`invalid format specifier: ${specifier}`);
  const [, fill, align, sign, symbol, zero, width, comma, precision, trim, type] = match;
  return makeFormatter({
    fill: fill ?? " ",
    align: align ?? ">",
    sign: sign ?? "-",
    symbol: symbol ?? "",
    zero: zero === "0",
    width: width === undefined ? undefined : Number(width),
    comma: comma === ",",
    precision: precision === undefined ? undefined : Number(precision.slice(1)),
    trim: trim === "~",
    type: type ?? "",
  });
}

/** 🔢️ The tick formatter of a continuous scale: precision derived from the tick step. */
export function continuousTickFormat(start: number, stop: number, count: number, specifier?: string): (value: number) => string {
  if (specifier !== undefined) return formatNumberSpec(specifier);
  const step = tickStep(start, stop, count);
  const decimals = Math.max(0, -Math.floor(Math.log10(step) + 1e-12));
  return (value: number) => {
    const text = value.toFixed(Number.isFinite(decimals) ? Math.min(20, decimals) : 0);
    return text === "-0" ? "0" : text;
  };
}
//#endregion 🔖️TickFormat

//#region 🔖️Discrete
/** 📐️ `scaleOrdinal`: an explicit domain mapped onto a cycling range. */
export function scaleOrdinal<O>(domain: readonly string[] = [], range: readonly O[] = [], unknown?: O): VizScale<string, O> {
  const index = new Map<string, number>();
  const known: string[] = [];
  for (const value of domain) {
    if (index.has(value)) continue;
    index.set(value, known.length);
    known.push(value);
  }
  const scale = ((value: string) => {
    let i = index.get(value);
    if (i === undefined) {
      if (unknown !== undefined) return unknown;
      i = known.length;
      index.set(value, i);
      known.push(value);
    }
    return range[i % range.length]!;
  }) as VizScale<string, O>;
  scale.kind = "ordinal";
  scale.domain = () => known;
  scale.range = () => range;
  return scale;
}

/** 📐️ `scaleBand`: a discrete domain onto contiguous, padded bands of the range. */
export function scaleBand(domain: readonly string[] = [], range: readonly number[] = [0, 1], options: VizScaleOptions = {}): VizScale<string, number> {
  const paddingInner = options.paddingInner ?? options.padding ?? 0;
  const paddingOuter = options.paddingOuter ?? options.padding ?? 0;
  const align = options.align ?? 0.5;
  const round = options.round === true;
  const r0 = range[0]!;
  const r1 = range[1]!;
  const known = [...new Set(domain)];
  const n = known.length;
  const reversed = r1 < r0;
  let start = reversed ? r1 : r0;
  const stop = reversed ? r0 : r1;
  let step = (stop - start) / Math.max(1, n - paddingInner + paddingOuter * 2);
  if (round) step = Math.floor(step);
  start += (stop - start - step * (n - paddingInner)) * align;
  let bandwidth = step * (1 - paddingInner);
  if (round) {
    start = Math.round(start);
    bandwidth = Math.round(bandwidth);
  }
  const positions = known.map((_, i) => start + step * i);
  const ordered = reversed ? [...positions].reverse() : positions;
  const index = new Map(known.map((value, i) => [value, i] as const));
  const scale = ((value: string) => {
    const i = index.get(value);
    return i === undefined ? Number.NaN : ordered[i]!;
  }) as VizScale<string, number>;
  scale.kind = "band";
  scale.domain = () => known;
  scale.range = () => [r0, r1];
  scale.bandwidth = () => bandwidth;
  scale.step = () => step;
  return scale;
}

/** 📐️ `scalePoint`: the band scale at zero bandwidth, positioning discrete points. */
export function scalePoint(domain: readonly string[] = [], range: readonly number[] = [0, 1], options: VizScaleOptions = {}): VizScale<string, number> {
  const scale = scaleBand(domain, range, { ...options, paddingInner: 1, paddingOuter: options.padding ?? 0 });
  scale.kind = "point";
  return scale;
}

/** 📊️ The R-7 quantile of a sorted numeric sample, the estimator d3 uses. */
export function quantileSorted(values: readonly number[], p: number): number {
  const n = values.length;
  if (n === 0) return Number.NaN;
  if (p <= 0 || n < 2) return values[0]!;
  if (p >= 1) return values[n - 1]!;
  const i = (n - 1) * p;
  const i0 = Math.floor(i);
  const value0 = values[i0]!;
  const value1 = values[i0 + 1]!;
  return value0 + (value1 - value0) * (i - i0);
}

/** 📐️ `scaleQuantile`: a sample split into as many equal-count classes as the range is long. */
export function scaleQuantile<O>(sample: readonly number[], range: readonly O[]): VizScale<number, O> & { thresholds(): number[] } {
  const sorted = [...sample].filter((value) => value !== null && !Number.isNaN(value)).sort((a, b) => a - b);
  const n = Math.max(1, range.length);
  const cuts = Array.from({ length: n - 1 }, (_, i) => quantileSorted(sorted, (i + 1) / n));
  const scale = ((value: number) => (Number.isNaN(value) ? range[0]! : range[bisectRight(cuts, value)]!)) as VizScale<number, O> & { thresholds(): number[] };
  scale.kind = "quantile";
  scale.domain = () => sorted;
  scale.range = () => range;
  scale.thresholds = () => cuts;
  return scale;
}

/** 📐️ `scaleQuantize`: a continuous domain cut into uniform slices, one per range value. */
export function scaleQuantize<O>(domain: readonly number[], range: readonly O[]): VizScale<number, O> & { thresholds(): number[] } {
  const d0 = domain[0]!;
  const d1 = domain[domain.length - 1]!;
  const n = range.length;
  const cuts = Array.from({ length: n - 1 }, (_, i) => (d1 * (i + 1) - d0 * (i + 1 - n)) / n);
  const scale = ((value: number) => (Number.isNaN(value) ? range[0]! : range[bisectRight(cuts, value)]!)) as VizScale<number, O> & { thresholds(): number[] };
  scale.kind = "quantize";
  scale.domain = () => [d0, d1];
  scale.range = () => range;
  scale.thresholds = () => cuts;
  scale.ticks = (count = 10) => ticks(d0, d1, count);
  return scale;
}

/** 📐️ `scaleThreshold`: explicit cut points, one more range value than there are thresholds. */
export function scaleThreshold<O>(thresholds: readonly number[], range: readonly O[]): VizScale<number, O> {
  const scale = ((value: number) => (Number.isNaN(value) ? range[0]! : range[bisectRight(thresholds, value)]!)) as VizScale<number, O>;
  scale.kind = "threshold";
  scale.domain = () => thresholds;
  scale.range = () => range;
  return scale;
}
//#endregion 🔖️Discrete

//#region 🔖️SequentialDiverging
/** 📐️ `scaleSequential`: a continuous domain onto `[0,1]` and through an interpolator. */
export function scaleSequential<O>(domain: readonly number[], interpolator: (t: number) => O, options: VizScaleOptions = {}): VizScale<number, O> {
  const d0 = domain[0]!;
  const d1 = domain[domain.length - 1]!;
  const to = normalize(d0, d1);
  const clampIn = options.clamp === true ? clamper(d0, d1) : (x: number) => x;
  const scale = ((value: number) => interpolator(to(clampIn(+value)))) as VizScale<number, O>;
  scale.kind = "sequential";
  scale.domain = () => [d0, d1];
  scale.range = () => [];
  scale.ticks = (count = 10) => ticks(d0, d1, count);
  return scale;
}

/** 📐️ `scaleDiverging`: a three-point domain folded symmetrically around its midpoint. */
export function scaleDiverging<O>(domain: readonly number[], interpolator: (t: number) => O, options: VizScaleOptions = {}): VizScale<number, O> {
  const d0 = domain[0]!;
  const d1 = domain[1]!;
  const d2 = domain[2]!;
  const low = normalize(d0, d1);
  const high = normalize(d1, d2);
  const clampIn = options.clamp === true ? clamper(Math.min(d0, d2), Math.max(d0, d2)) : (x: number) => x;
  const scale = ((value: number) => {
    const x = clampIn(+value);
    return interpolator(x < d1 ? 0.5 * low(x) : 0.5 + 0.5 * high(x));
  }) as VizScale<number, O>;
  scale.kind = "diverging";
  scale.domain = () => [d0, d1, d2];
  scale.range = () => [];
  scale.ticks = (count = 10) => ticks(d0, d2, count);
  return scale;
}
//#endregion 🔖️SequentialDiverging

//#region 🔖️Temporal
const DURATION_SECOND = 1000;
const DURATION_MINUTE = 60000;
const DURATION_HOUR = 3600000;
const DURATION_DAY = 86400000;
const DURATION_WEEK = 604800000;
const DURATION_MONTH = 2592000000;
const DURATION_YEAR = 31536000000;

/** 🕰️ A calendar interval: floor, ceil, offset and a bounded range, in d3-time's shape. */
export type VizTimeInterval = {
  floor(date: Date): Date;
  ceil(date: Date): Date;
  offset(date: Date, step: number): Date;
  range(start: Date, stop: Date, step?: number): Date[];
  every(step: number): VizTimeInterval;
};

function timeInterval(floori: (date: Date) => void, offseti: (date: Date, step: number) => void, count?: (start: Date, end: Date) => number, field?: (date: Date) => number): VizTimeInterval {
  const interval: VizTimeInterval = {
    floor(date) {
      const d = new Date(+date);
      floori(d);
      return d;
    },
    ceil(date) {
      const d = new Date(+date - 1);
      floori(d);
      offseti(d, 1);
      floori(d);
      return d;
    },
    offset(date, step) {
      const d = new Date(+date);
      offseti(d, Math.floor(step));
      return d;
    },
    range(start, stop, step = 1) {
      const out: Date[] = [];
      if (!(step > 0)) return out;
      let current = interval.ceil(start);
      while (+current < +stop) {
        out.push(new Date(+current));
        const next = new Date(+current);
        offseti(next, step);
        floori(next);
        if (+next <= +current) break;
        current = next;
      }
      return out;
    },
    every(step) {
      const every = Math.floor(step);
      if (!Number.isFinite(every) || !(every > 0)) return interval;
      if (every === 1) return interval;
      if (field === undefined) {
        return timeInterval(
          (date) => {
            floori(date);
            const base = count === undefined ? 0 : count(new Date(0), new Date(+date));
            offseti(date, -(base % every));
            floori(date);
          },
          (date, s) => offseti(date, s * every),
          count,
        );
      }
      return timeInterval(
        (date) => {
          floori(date);
          while (field(date) % every !== 0) {
            offseti(date, -1);
            floori(date);
          }
        },
        (date, s) => {
          let remaining = s;
          while (remaining > 0) {
            offseti(date, 1);
            floori(date);
            if (field(date) % every === 0) remaining -= 1;
          }
        },
        count,
        field,
      );
    },
  };
  return interval;
}

/** 🕰️ Local millisecond ticks. */
export const timeMillisecond = timeInterval(
  () => {},
  (date, step) => date.setTime(+date + step),
);

/** 🕰️ Local second boundaries. */
export const timeSecond = timeInterval(
  (date) => date.setTime(Math.floor(+date / DURATION_SECOND) * DURATION_SECOND),
  (date, step) => date.setTime(+date + step * DURATION_SECOND),
  (start, end) => (+end - +start) / DURATION_SECOND,
  (date) => date.getUTCSeconds(),
);

/** 🕰️ Local minute boundaries. */
export const timeMinute = timeInterval(
  (date) => date.setTime(+date - date.getMilliseconds() - date.getSeconds() * DURATION_SECOND),
  (date, step) => date.setTime(+date + step * DURATION_MINUTE),
  (start, end) => (+end - +start) / DURATION_MINUTE,
  (date) => date.getMinutes(),
);

/** 🕰️ Local hour boundaries. */
export const timeHour = timeInterval(
  (date) => date.setTime(+date - date.getMilliseconds() - date.getSeconds() * DURATION_SECOND - date.getMinutes() * DURATION_MINUTE),
  (date, step) => date.setTime(+date + step * DURATION_HOUR),
  (start, end) => (+end - +start) / DURATION_HOUR,
  (date) => date.getHours(),
);

/** 🕰️ Local midnights. */
export const timeDay = timeInterval(
  (date) => date.setHours(0, 0, 0, 0),
  (date, step) => date.setDate(date.getDate() + step),
  (start, end) => (+end - +start - (end.getTimezoneOffset() - start.getTimezoneOffset()) * DURATION_MINUTE) / DURATION_DAY,
  (date) => date.getDate() - 1,
);

/** 🕰️ Local Sundays. */
export const timeWeek = timeInterval(
  (date) => {
    date.setDate(date.getDate() - (date.getDay() % 7));
    date.setHours(0, 0, 0, 0);
  },
  (date, step) => date.setDate(date.getDate() + step * 7),
  (start, end) => (+end - +start - (end.getTimezoneOffset() - start.getTimezoneOffset()) * DURATION_MINUTE) / DURATION_WEEK,
);

/** 🕰️ Local month starts. */
export const timeMonth = timeInterval(
  (date) => {
    date.setDate(1);
    date.setHours(0, 0, 0, 0);
  },
  (date, step) => date.setMonth(date.getMonth() + step),
  (start, end) => end.getMonth() - start.getMonth() + (end.getFullYear() - start.getFullYear()) * 12,
  (date) => date.getMonth(),
);

/** 🕰️ Local year starts. */
export const timeYear = timeInterval(
  (date) => {
    date.setMonth(0, 1);
    date.setHours(0, 0, 0, 0);
  },
  (date, step) => date.setFullYear(date.getFullYear() + step),
  (start, end) => end.getFullYear() - start.getFullYear(),
  (date) => date.getFullYear(),
);

const TICK_INTERVALS: readonly (readonly [VizTimeInterval, number, number])[] = [
  [timeSecond, 1, DURATION_SECOND],
  [timeSecond, 5, 5 * DURATION_SECOND],
  [timeSecond, 15, 15 * DURATION_SECOND],
  [timeSecond, 30, 30 * DURATION_SECOND],
  [timeMinute, 1, DURATION_MINUTE],
  [timeMinute, 5, 5 * DURATION_MINUTE],
  [timeMinute, 15, 15 * DURATION_MINUTE],
  [timeMinute, 30, 30 * DURATION_MINUTE],
  [timeHour, 1, DURATION_HOUR],
  [timeHour, 3, 3 * DURATION_HOUR],
  [timeHour, 6, 6 * DURATION_HOUR],
  [timeHour, 12, 12 * DURATION_HOUR],
  [timeDay, 1, DURATION_DAY],
  [timeDay, 2, 2 * DURATION_DAY],
  [timeWeek, 1, DURATION_WEEK],
  [timeMonth, 1, DURATION_MONTH],
  [timeMonth, 3, 3 * DURATION_MONTH],
  [timeYear, 1, DURATION_YEAR],
];

/** 🕰️ The calendar interval d3-time would pick for a span and a target tick count. */
export function timeTickInterval(start: Date, stop: Date, count: number): VizTimeInterval {
  const target = Math.abs(+stop - +start) / count;
  const index = bisectRight(
    TICK_INTERVALS.map((entry) => entry[2]),
    target,
  );
  if (index === TICK_INTERVALS.length) return timeYear.every(Math.max(tickStep(+start / DURATION_YEAR, +stop / DURATION_YEAR, count), 1));
  if (index === 0) return timeMillisecond.every(Math.max(tickStep(+start, +stop, count), 1));
  const chosen = target / TICK_INTERVALS[index - 1]![2] < TICK_INTERVALS[index]![2] / target ? TICK_INTERVALS[index - 1]! : TICK_INTERVALS[index]!;
  return chosen[0].every(chosen[1]);
}

/** 🕰️ The d3-time tick sequence for a date span. */
export function timeTicks(start: Date, stop: Date, count = 10): Date[] {
  const reverse = +stop < +start;
  const lo = reverse ? stop : start;
  const hi = reverse ? start : stop;
  const out = timeTickInterval(lo, hi, count).range(lo, new Date(+hi + 1));
  return reverse ? out.reverse() : out;
}

/** 📐️ `scaleTime`: a millisecond-linear scale that ticks on calendar boundaries. */
export function scaleTemporal(domain: readonly (Date | number)[], range: readonly number[], options: VizScaleOptions = {}): VizScale<Date | number, number> & { tickDates(count?: number): Date[] } {
  const numeric = domain.map((value) => +value);
  const base = scaleContinuous({ domain: numeric, range, clamp: options.clamp, round: options.round, kind: "temporal" });
  const scale = ((value: Date | number) => base(+value)) as VizScale<Date | number, number> & { tickDates(count?: number): Date[] };
  scale.kind = "temporal";
  scale.domain = () => domain;
  scale.range = () => range;
  scale.invert = (value: number) => base.invert!(value);
  scale.tickDates = (count = 10) => timeTicks(new Date(numeric[0]!), new Date(numeric[numeric.length - 1]!), count);
  scale.ticks = (count = 10) => scale.tickDates(count).map((date) => +date);
  return scale;
}
//#endregion 🔖️Temporal

//#region 🔖️Dispatch
/** 📐️ Builds any scale from its declarative `VizScaleSpec`, the way `\SemioVizScale` dispatches. */
export function buildVizScale(spec: VizScaleSpec): VizScale<never, never> {
  const options = spec.options ?? {};
  const numericDomain = spec.domain.map((value) => Number(value));
  const numericRange = spec.range.map((value) => Number(value));
  const stringDomain = spec.domain.map((value) => String(value));
  switch (spec.kind) {
    case "linear":
      return scaleLinear(numericDomain, numericRange, options) as unknown as VizScale<never, never>;
    case "log":
      return scaleLog(numericDomain, numericRange, options) as unknown as VizScale<never, never>;
    case "pow":
      return scalePow(numericDomain, numericRange, options) as unknown as VizScale<never, never>;
    case "sqrt":
      return scaleSqrt(numericDomain, numericRange, options) as unknown as VizScale<never, never>;
    case "symlog":
      return scaleSymlog(numericDomain, numericRange, options) as unknown as VizScale<never, never>;
    case "identity":
      return scaleIdentity(numericDomain) as unknown as VizScale<never, never>;
    case "ordinal":
      return scaleOrdinal(stringDomain, spec.range as readonly string[], options.unknown as string | undefined) as unknown as VizScale<never, never>;
    case "band":
      return scaleBand(stringDomain, numericRange, options) as unknown as VizScale<never, never>;
    case "point":
      return scalePoint(stringDomain, numericRange, options) as unknown as VizScale<never, never>;
    case "quantile":
      return scaleQuantile(numericDomain, spec.range) as unknown as VizScale<never, never>;
    case "quantize":
      return scaleQuantize(numericDomain, spec.range) as unknown as VizScale<never, never>;
    case "threshold":
      return scaleThreshold(numericDomain, spec.range) as unknown as VizScale<never, never>;
    case "temporal":
      return scaleTemporal(numericDomain, numericRange, options) as unknown as VizScale<never, never>;
    default:
      throw new Error(`scale kind ${spec.kind} needs an interpolator and is built with scaleSequential or scaleDiverging`);
  }
}

/** 📐️ The scale kinds this module builds from a declarative specification. */
export const VIZ_BUILDABLE_SCALE_KINDS: readonly VizScaleKind[] = ["linear", "log", "pow", "sqrt", "symlog", "identity", "ordinal", "band", "point", "quantile", "quantize", "threshold", "temporal"];
//#endregion 🔖️Dispatch
