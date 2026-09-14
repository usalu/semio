/** 🔬️ The differential check table of `@semio-tech/print-viz-kernel`: every kernel module measured
 * against the d3 package registered as its oracle. d3 is a devDependency reached from HERE and
 * nowhere else, which is why this table lives in the taxonomy's probe directory rather than beside
 * the library — the kernel modules themselves depend on nothing outside this repository.
 */
import * as scale from "../../../📐scale/🟦️.ts";
import * as format from "../../../🔢format/🟦️.ts";
import * as transform from "../../../🧮transform/🟦️.ts";
import * as mark from "../../../✒️mark/🟦️.ts";
import * as shape from "../../../🥧shape/🟦️.ts";
import * as coordinate from "../../../🧭coordinate/🟦️.ts";
import * as hierarchy from "../../../🌳hierarchy/🟦️.ts";
import * as network from "../../../🕸️network/🟦️.ts";
import * as flow from "../../../🌊flow/🟦️.ts";
import * as geo from "../../../🌍geo/🟦️.ts";
import * as spatial from "../../../📍spatial/🟦️.ts";
import * as theme from "../../../🎨theme/🟦️.ts";
import * as render from "../../../🖼️render/🟦️.ts";

//#region 🔖️Level
/** 🎚️ The three sampling densities every check table honours. */
export const VIZ_KERNEL_LEVELS = ["quick", "long", "exhaustive"] as const;

export type Level = (typeof VIZ_KERNEL_LEVELS)[number];
//#endregion 🔖️Level

//#region 🔖️Checks
/** 🧪️ One differential check: our kernel against the d3 package registered as its oracle. */
export type VizKernelCheck = {
  readonly module: string;
  readonly name: string;
  readonly subject: () => unknown | Promise<unknown>;
  readonly oracle: () => unknown | Promise<unknown>;
  readonly tolerance?: number;
};

function sampleCount(level: Level): number {
  return level === "quick" ? 9 : level === "long" ? 61 : 401;
}

function samples(level: Level, lo: number, hi: number): number[] {
  const n = sampleCount(level);
  return Array.from({ length: n }, (_, i) => lo + ((hi - lo) * i) / (n - 1));
}

/** 📐️ Differential checks for `📐scale` against `d3-scale`, `d3-array` and `d3-time`. */
async function scaleChecks(level: Level): Promise<VizKernelCheck[]> {
  const d3Scale = await import("d3-scale");
  const d3Array = await import("d3-array");
  const d3Time = await import("d3-time");
  const xs = samples(level, -20, 120);
  const checks: VizKernelCheck[] = [
    {
      module: "scale",
      name: "linear-affine",
      subject: () => xs.map(scale.scaleLinear([0, 100], [0, 180])),
      oracle: () => xs.map(d3Scale.scaleLinear().domain([0, 100]).range([0, 180])),
    },
    {
      module: "scale",
      name: "linear-clamped-descending",
      subject: () => xs.map(scale.scaleLinear([100, 0], [10, -30], { clamp: true })),
      oracle: () => xs.map(d3Scale.scaleLinear().domain([100, 0]).range([10, -30]).clamp(true)),
    },
    {
      module: "scale",
      name: "linear-piecewise",
      subject: () => xs.map(scale.scaleLinear([0, 50, 100], [0, 10, 200])),
      oracle: () => xs.map(d3Scale.scaleLinear().domain([0, 50, 100]).range([0, 10, 200])),
    },
    {
      module: "scale",
      name: "linear-invert",
      subject: () => xs.map((x) => scale.scaleLinear([3, 97], [0, 1]).invert!(x / 100)),
      oracle: () => xs.map((x) => d3Scale.scaleLinear().domain([3, 97]).range([0, 1]).invert(x / 100)),
    },
    {
      module: "scale",
      name: "linear-nice",
      subject: () => [scale.niceDomain([0.135, 0.987], 10), scale.niceDomain([-7.3, 91.4], 5), scale.niceDomain([12, 12], 10)],
      oracle: () => [d3Scale.scaleLinear().domain([0.135, 0.987]).nice(10).domain(), d3Scale.scaleLinear().domain([-7.3, 91.4]).nice(5).domain(), d3Scale.scaleLinear().domain([12, 12]).nice(10).domain()],
    },
    {
      module: "scale",
      name: "array-ticks",
      subject: () => [scale.ticks(0, 1, 10), scale.ticks(-7.3, 91.4, 6), scale.ticks(100, 0, 5), scale.ticks(0.0001, 0.0009, 4), scale.tickStep(0, 1, 10), scale.tickIncrement(0, 1, 10)],
      oracle: () => [d3Array.ticks(0, 1, 10), d3Array.ticks(-7.3, 91.4, 6), d3Array.ticks(100, 0, 5), d3Array.ticks(0.0001, 0.0009, 4), d3Array.tickStep(0, 1, 10), d3Array.tickIncrement(0, 1, 10)],
    },
    {
      module: "scale",
      name: "pow-and-sqrt",
      subject: () => [xs.map(scale.scalePow([0, 16], [0, 100], { exponent: 0.5 })), xs.map(scale.scaleSqrt([-16, 16], [0, 100])), xs.map(scale.scalePow([0, 10], [0, 1], { exponent: 2 }))],
      oracle: () => [xs.map(d3Scale.scalePow().exponent(0.5).domain([0, 16]).range([0, 100])), xs.map(d3Scale.scaleSqrt().domain([-16, 16]).range([0, 100])), xs.map(d3Scale.scalePow().exponent(2).domain([0, 10]).range([0, 1]))],
    },
    {
      module: "scale",
      name: "symlog",
      subject: () => [xs.map(scale.scaleSymlog([-100, 100], [0, 1])), xs.map(scale.scaleSymlog([-100, 100], [0, 1], { constant: 5 }))],
      oracle: () => [xs.map(d3Scale.scaleSymlog().domain([-100, 100]).range([0, 1])), xs.map(d3Scale.scaleSymlog().constant(5).domain([-100, 100]).range([0, 1]))],
    },
    {
      module: "scale",
      name: "log-mapping-and-ticks",
      subject: () => [samples(level, 1, 1000).map(scale.scaleLog([1, 1000], [0, 1])), scale.scaleLog([1, 1000], [0, 1]).ticks!(10), scale.scaleLog([3, 4000], [0, 1]).ticks!(6), scale.scaleLog([1, 64], [0, 1], { base: 2 }).ticks!(10)],
      oracle: () => [samples(level, 1, 1000).map(d3Scale.scaleLog().domain([1, 1000]).range([0, 1])), d3Scale.scaleLog().domain([1, 1000]).range([0, 1]).ticks(10), d3Scale.scaleLog().domain([3, 4000]).range([0, 1]).ticks(6), d3Scale.scaleLog().base(2).domain([1, 64]).range([0, 1]).ticks(10)],
    },
    {
      module: "scale",
      name: "log-nice",
      subject: () => scale.scaleLog([3, 4000], [0, 1], { nice: true }).domain(),
      oracle: () => d3Scale.scaleLog().domain([3, 4000]).range([0, 1]).nice().domain(),
    },
    {
      module: "scale",
      name: "ordinal-cycling",
      subject: () => ["a", "b", "c", "d", "a", "e"].map(scale.scaleOrdinal(["a", "b", "c"], ["x", "y"])),
      oracle: () => ["a", "b", "c", "d", "a", "e"].map(d3Scale.scaleOrdinal(["a", "b", "c"], ["x", "y"])),
    },
    {
      module: "scale",
      name: "band-positions",
      subject: () => {
        const own = scale.scaleBand(["a", "b", "c", "d"], [0, 120], { paddingInner: 0.2, paddingOuter: 0.1, align: 0.5 });
        return { positions: ["a", "b", "c", "d"].map(own), bandwidth: own.bandwidth!(), step: own.step!() };
      },
      oracle: () => {
        const ref = d3Scale.scaleBand(["a", "b", "c", "d"], [0, 120]).paddingInner(0.2).paddingOuter(0.1).align(0.5);
        return { positions: ["a", "b", "c", "d"].map(ref), bandwidth: ref.bandwidth(), step: ref.step() };
      },
    },
    {
      module: "scale",
      name: "band-reversed-range",
      subject: () => {
        const own = scale.scaleBand(["a", "b", "c"], [120, 0], { padding: 0.25 });
        return { positions: ["a", "b", "c"].map(own), bandwidth: own.bandwidth!() };
      },
      oracle: () => {
        const ref = d3Scale.scaleBand(["a", "b", "c"], [120, 0]).padding(0.25);
        return { positions: ["a", "b", "c"].map(ref), bandwidth: ref.bandwidth() };
      },
    },
    {
      module: "scale",
      name: "point-positions",
      subject: () => {
        const own = scale.scalePoint(["a", "b", "c", "d"], [0, 90], { padding: 0.5 });
        return { positions: ["a", "b", "c", "d"].map(own), step: own.step!() };
      },
      oracle: () => {
        const ref = d3Scale.scalePoint(["a", "b", "c", "d"], [0, 90]).padding(0.5);
        return { positions: ["a", "b", "c", "d"].map(ref), step: ref.step() };
      },
    },
    {
      module: "scale",
      name: "quantile-quantize-threshold",
      subject: () => {
        const sample = [3, 6, 7, 8, 8, 10, 13, 15, 16, 20];
        const q = scale.scaleQuantile(sample, ["lo", "mid", "hi"]);
        const z = scale.scaleQuantize([0, 100], ["a", "b", "c", "d"]);
        const t = scale.scaleThreshold([10, 20], ["a", "b", "c"]);
        return { thresholds: q.thresholds(), quantile: sample.map(q), quantizeCuts: z.thresholds(), quantize: samples(level, 0, 100).map(z), threshold: [-1, 10, 15, 20, 40].map(t) };
      },
      oracle: () => {
        const sample = [3, 6, 7, 8, 8, 10, 13, 15, 16, 20];
        const q = d3Scale.scaleQuantile(sample, ["lo", "mid", "hi"]);
        const z = d3Scale.scaleQuantize([0, 100], ["a", "b", "c", "d"]);
        const t = d3Scale.scaleThreshold([10, 20], ["a", "b", "c"]);
        return { thresholds: q.quantiles(), quantile: sample.map(q), quantizeCuts: z.thresholds(), quantize: samples(level, 0, 100).map(z), threshold: [-1, 10, 15, 20, 40].map(t) };
      },
    },
    {
      module: "scale",
      name: "sequential-diverging",
      subject: () => [samples(level, 0, 1).map(scale.scaleSequential([0, 1], (t) => t * t)), samples(level, -1, 1).map(scale.scaleDiverging([-1, 0, 1], (t) => t))],
      oracle: () => [samples(level, 0, 1).map(d3Scale.scaleSequential([0, 1], (t: number) => t * t)), samples(level, -1, 1).map(d3Scale.scaleDiverging([-1, 0, 1], (t: number) => t))],
    },
    {
      module: "scale",
      name: "temporal-ticks",
      subject: () =>
        [
          [Date.UTC(2020, 0, 1), Date.UTC(2020, 0, 2)],
          [Date.UTC(2020, 0, 1), Date.UTC(2021, 0, 1)],
          [Date.UTC(2020, 0, 1), Date.UTC(2030, 0, 1)],
          [Date.UTC(2020, 0, 1), Date.UTC(2020, 0, 1, 0, 5)],
        ].map(([lo, hi]) => scale.timeTicks(new Date(lo!), new Date(hi!), 10).map((date) => +date)),
      oracle: () =>
        [
          [Date.UTC(2020, 0, 1), Date.UTC(2020, 0, 2)],
          [Date.UTC(2020, 0, 1), Date.UTC(2021, 0, 1)],
          [Date.UTC(2020, 0, 1), Date.UTC(2030, 0, 1)],
          [Date.UTC(2020, 0, 1), Date.UTC(2020, 0, 1, 0, 5)],
        ].map(([lo, hi]) => d3Time.timeTicks(new Date(lo!), new Date(hi!), 10).map((date) => +date)),
    },
    {
      module: "scale",
      name: "temporal-mapping",
      subject: () => samples(level, Date.UTC(2020, 0, 1), Date.UTC(2021, 0, 1)).map((t) => scale.scaleTemporal([Date.UTC(2020, 0, 1), Date.UTC(2021, 0, 1)], [0, 100])(t)),
      oracle: () => samples(level, Date.UTC(2020, 0, 1), Date.UTC(2021, 0, 1)).map((t) => d3Scale.scaleTime().domain([Date.UTC(2020, 0, 1), Date.UTC(2021, 0, 1)]).range([0, 100])(new Date(t))),
    },
  ];
  return checks;
}

const FORMAT_SPECIFIERS = [",.2f", ".0%", "+.3e", "$,.2f", ".3s", "~s", ".4r", "08.3f", ">12,.1f", "<10.2f", "^11.2f", "(.2f", ".2p", "d", "b", "o", "x", "X", ",d", "~g", ".1~%", "=+10,.3f"] as const;

const FORMAT_VALUES = [0, 1, -1, 0.5, -0.00042, 42, -1234.5678, 1234567.891, 1e21, 0.000001234, 999999, -0.5, 7] as const;

const TIME_SPECIFIERS = ["%Y-%m-%d", "%d.%m.%Y", "%H:%M:%S", "%Y-%m-%dT%H:%M:%S", "%a %b %e", "%A %B", "%j", "%U", "%W", "%I %p", "%y", "%-m/%-d/%Y", "%_d", "%%"] as const;

/** 🔢️ Differential checks for `🔢format` against `d3-format` and `d3-time-format`. */
async function formatChecks(level: Level): Promise<VizKernelCheck[]> {
  const d3Format = await import("d3-format");
  const d3TimeFormat = await import("d3-time-format");
  const specifiers = level === "quick" ? FORMAT_SPECIFIERS.slice(0, 12) : FORMAT_SPECIFIERS;
  const timeSpecifiers = level === "quick" ? TIME_SPECIFIERS.slice(0, 9) : TIME_SPECIFIERS;
  const instants = [new Date(2020, 0, 1, 0, 0, 0), new Date(2020, 5, 15, 13, 45, 7, 42), new Date(2021, 11, 31, 23, 59, 59), new Date(1999, 8, 5, 9, 3, 0), new Date(2024, 1, 29, 12, 0, 0)];
  return [
    ...specifiers.map((specifier) => ({
      module: "format",
      name: `number-en-${specifier}`,
      subject: () => FORMAT_VALUES.map((value) => format.vizFormat(specifier, "en")(value)),
      oracle: () => FORMAT_VALUES.map((value) => d3Format.format(specifier)(value)),
    })),
    {
      module: "format",
      name: "specifier-round-trip",
      subject: () => specifiers.map((specifier) => format.formatVizSpecifier(format.parseVizFormatSpecifier(specifier))),
      oracle: () => specifiers.map((specifier) => `${d3Format.formatSpecifier(specifier)}`),
    },
    {
      module: "format",
      name: "precision-suggestions",
      subject: () => [format.vizPrecisionFixed(0.01), format.vizPrecisionFixed(1500), format.vizPrecisionRound(0.01, 1.01), format.vizPrecisionRound(1, 1e6), format.vizPrecisionPrefix(1e5, 1.3e6), format.vizPrecisionPrefix(1e-6, 0.00042)],
      oracle: () => [d3Format.precisionFixed(0.01), d3Format.precisionFixed(1500), d3Format.precisionRound(0.01, 1.01), d3Format.precisionRound(1, 1e6), d3Format.precisionPrefix(1e5, 1.3e6), d3Format.precisionPrefix(1e-6, 0.00042)],
    },
    {
      module: "format",
      name: "format-prefix",
      subject: () => [format.vizFormatPrefix(",.0", 1e6, "en")(1.3e6), format.vizFormatPrefix(",.2", 1e-6, "en")(0.00042)],
      oracle: () => [d3Format.formatPrefix(",.0", 1e6)(1.3e6), d3Format.formatPrefix(",.2", 1e-6)(0.00042)],
    },
    ...timeSpecifiers.map((specifier) => ({
      module: "format",
      name: `time-en-${specifier}`,
      subject: () => instants.map((date) => format.vizTimeFormat(specifier, "en")(date)),
      oracle: () => instants.map((date) => d3TimeFormat.timeFormat(specifier)(date)),
    })),
    {
      module: "format",
      name: "time-de-conformance",
      subject: () => instants.map((date) => format.vizTimeFormat("%A, %e. %B %Y", "de")(date)),
      oracle: () => ["Mittwoch,  1. Januar 2020", "Montag, 15. Juni 2020", "Freitag, 31. Dezember 2021", "Sonntag,  5. September 1999", "Donnerstag, 29. Februar 2024"],
    },
    {
      module: "format",
      name: "number-de-conformance",
      subject: () => [format.vizFormat(",.2f", "de")(1234567.891), format.vizFormat(",.2f", "de")(-1234.5), format.vizFormat("$,.2f", "de")(1234.5)],
      oracle: () => ["1.234.567,89", "−1.234,50", "1.234,50 €"],
    },
  ];
}

const STACK_KEYS = ["apples", "bananas", "cherries", "dates"] as const;

function stackRows(level: Level): Record<string, number>[] {
  const n = level === "quick" ? 6 : level === "long" ? 24 : 96;
  return Array.from({ length: n }, (_, i) => ({
    apples: 10 + 8 * Math.sin(i / 3),
    bananas: 6 + 5 * Math.cos(i / 4),
    cherries: 3 + (i % 5),
    dates: i % 7 === 0 ? -4 : 2 + (i % 3),
  }));
}

/** 🧮️ Differential checks for `🧮transform` against `d3-array` and `d3-shape`. */
async function transformChecks(level: Level): Promise<VizKernelCheck[]> {
  const d3Array = await import("d3-array");
  const d3Shape = await import("d3-shape");
  const rows = stackRows(level);
  const sample = samples(level, -3, 17).map((x) => x * x - 4 * x + 1);
  const orders = ["none", "ascending", "descending", "inside-out", "reverse", "appearance"] as const;
  const offsets = ["none", "expand", "diverging", "silhouette", "wiggle"] as const;
  const d3Orders = { none: d3Shape.stackOrderNone, ascending: d3Shape.stackOrderAscending, descending: d3Shape.stackOrderDescending, "inside-out": d3Shape.stackOrderInsideOut, reverse: d3Shape.stackOrderReverse, appearance: d3Shape.stackOrderAppearance };
  const d3Offsets = { none: d3Shape.stackOffsetNone, expand: d3Shape.stackOffsetExpand, diverging: d3Shape.stackOffsetDiverging, silhouette: d3Shape.stackOffsetSilhouette, wiggle: d3Shape.stackOffsetWiggle };
  return [
    {
      module: "transform",
      name: "statistics",
      subject: () => [transform.sum(sample), transform.fsum(sample), transform.mean(sample), transform.variance(sample), transform.deviation(sample), transform.median(sample), transform.quantile(sample, 0.25), transform.quantile(sample, 0.9), transform.extent(sample), transform.cumulativeSum(sample.slice(0, 5))],
      oracle: () => [d3Array.sum(sample), d3Array.fsum(sample), d3Array.mean(sample), d3Array.variance(sample), d3Array.deviation(sample), d3Array.median(sample), d3Array.quantile(sample, 0.25), d3Array.quantile(sample, 0.9), d3Array.extent(sample), [...d3Array.cumsum(sample.slice(0, 5))]],
    },
    {
      module: "transform",
      name: "bin-sturges",
      subject: () => transform.bin(sample).map((entry) => [entry.x0, entry.x1, entry.values.length]),
      oracle: () => d3Array.bin()(sample).map((entry) => [entry.x0, entry.x1, entry.length]),
    },
    {
      module: "transform",
      name: "bin-fixed-count",
      subject: () => transform.bin(sample, { thresholds: 7 }).map((entry) => [entry.x0, entry.x1, entry.values.length]),
      oracle: () => d3Array.bin().thresholds(7)(sample).map((entry) => [entry.x0, entry.x1, entry.length]),
    },
    {
      module: "transform",
      name: "bin-explicit-domain-and-thresholds",
      subject: () => transform.bin(sample, { domain: [-10, 200], thresholds: [-5, 0, 5, 20, 60] }).map((entry) => [entry.x0, entry.x1, entry.values.length]),
      oracle: () => d3Array.bin().domain([-10, 200]).thresholds([-5, 0, 5, 20, 60])(sample).map((entry) => [entry.x0, entry.x1, entry.length]),
    },
    {
      module: "transform",
      name: "bin-scott-and-freedman-diaconis",
      subject: () => [transform.bin(sample, { thresholds: "scott" }).length, transform.bin(sample, { thresholds: "freedman-diaconis" }).length],
      oracle: () => [d3Array.bin().thresholds(d3Array.thresholdScott)(sample).length, d3Array.bin().thresholds(d3Array.thresholdFreedmanDiaconis)(sample).length],
    },
    {
      module: "transform",
      name: "group-and-rollup",
      subject: () => {
        const grouped = transform.group(rows, (row) => Math.sign(row.dates!));
        const rolled = transform.rollup(rows, (bucket) => transform.sum(bucket.map((row) => row.apples!)), (row) => Math.sign(row.dates!));
        return { keys: [...grouped.keys()], sizes: [...grouped.values()].map((bucket) => bucket.length), rolled: [...rolled.entries()] };
      },
      oracle: () => {
        const grouped = d3Array.group(rows, (row) => Math.sign(row.dates!));
        const rolled = d3Array.rollup(rows, (bucket) => d3Array.sum(bucket, (row) => row.apples!), (row) => Math.sign(row.dates!));
        return { keys: [...grouped.keys()], sizes: [...grouped.values()].map((bucket) => bucket.length), rolled: [...rolled.entries()] };
      },
    },
    ...orders.map((order) => ({
      module: "transform",
      name: `stack-order-${order}`,
      subject: () => transform.stack(rows, [...STACK_KEYS], { order }).map((series) => ({ key: series.key, index: series.index, points: series.points.map((point) => [point[0], point[1]]) })),
      oracle: () =>
        d3Shape
          .stack()
          .keys([...STACK_KEYS])
          .order(d3Orders[order])(rows)
          .map((series) => ({ key: series.key as string, index: series.index, points: series.map((point) => [point[0], point[1]]) })),
    })),
    ...offsets.map((offset) => ({
      module: "transform",
      name: `stack-offset-${offset}`,
      subject: () => transform.stack(rows, [...STACK_KEYS], { offset }).map((series) => series.points.map((point) => [point[0], point[1]])),
      oracle: () =>
        d3Shape
          .stack()
          .keys([...STACK_KEYS])
          .offset(d3Offsets[offset])(rows)
          .map((series) => series.map((point) => [point[0], point[1]])),
    })),
    {
      module: "transform",
      name: "linear-regression-conformance",
      subject: () => {
        const fit = transform.linearRegression([
          [0, 1],
          [1, 3],
          [2, 5],
          [3, 7],
        ]);
        return [fit.slope, fit.intercept, fit.r2, fit.predict(10)];
      },
      oracle: () => [2, 1, 1, 21],
    },
    {
      module: "transform",
      name: "kde-integrates-to-one",
      subject: () => {
        const grid = Array.from({ length: 401 }, (_, i) => -40 + i * 0.4);
        const density = transform.kernelDensity1d([-5, -1, 0, 2, 3, 3.5, 9], grid, { bandwidth: 1.5 });
        return Math.round(transform.sum(density) * 0.4 * 1000) / 1000;
      },
      oracle: () => 1,
      tolerance: 1e-3,
    },
    {
      module: "transform",
      name: "fold-pivot-round-trip",
      subject: () => {
        const table = { name: "t", columns: ["region", "a", "b"], rows: [{ region: "north", a: 1, b: 2 }, { region: "south", a: 3, b: 4 }] };
        const folded = transform.fold(table, ["a", "b"]);
        const wide = transform.pivot(folded, "region", "key", "value");
        return { folded: folded.rows, wide: wide.rows, columns: wide.columns };
      },
      oracle: () => ({
        folded: [
          { region: "north", key: "a", value: 1 },
          { region: "north", key: "b", value: 2 },
          { region: "south", key: "a", value: 3 },
          { region: "south", key: "b", value: 4 },
        ],
        wide: [
          { region: "north", a: 1, b: 2 },
          { region: "south", a: 3, b: 4 },
        ],
        columns: ["region", "a", "b"],
      }),
    },
  ];
}

function curvePoints(level: Level): [number, number][] {
  const n = level === "quick" ? 7 : level === "long" ? 23 : 97;
  return Array.from({ length: n }, (_, i) => [i * 13 + (i % 3), 40 + 30 * Math.sin(i / 2) - (i % 4) * 7] as [number, number]);
}

/** ✒️ Differential checks for `✒️mark` and `🥧shape` against `d3-shape` and `d3-chord`, both sides
 * writing into the kernel's own recording path context so the comparison is numeric. */
async function shapeChecks(level: Level): Promise<VizKernelCheck[]> {
  const d3Shape = await import("d3-shape");
  const d3Chord = await import("d3-chord");
  const points = curvePoints(level);
  const curves: readonly (readonly [string, unknown, ReturnType<typeof mark.vizCurve>])[] = [
    ["linear", d3Shape.curveLinear, mark.curveLinear],
    ["linear-closed", d3Shape.curveLinearClosed, mark.curveLinearClosed],
    ["step", d3Shape.curveStep, mark.curveStep],
    ["step-before", d3Shape.curveStepBefore, mark.curveStepBefore],
    ["step-after", d3Shape.curveStepAfter, mark.curveStepAfter],
    ["basis", d3Shape.curveBasis, mark.curveBasis],
    ["basis-closed", d3Shape.curveBasisClosed, mark.curveBasisClosed],
    ["basis-open", d3Shape.curveBasisOpen, mark.curveBasisOpen],
    ["bundle", d3Shape.curveBundle, mark.curveBundle],
    ["bundle-beta-0.4", d3Shape.curveBundle.beta(0.4), mark.curveBundleBeta(0.4)],
    ["cardinal", d3Shape.curveCardinal, mark.curveCardinal],
    ["cardinal-tension-0.5", d3Shape.curveCardinal.tension(0.5), mark.curveCardinalTension(0.5)],
    ["cardinal-closed", d3Shape.curveCardinalClosed, mark.curveCardinalClosed],
    ["cardinal-open", d3Shape.curveCardinalOpen, mark.curveCardinalOpen],
    ["catmull-rom", d3Shape.curveCatmullRom, mark.curveCatmullRom],
    ["catmull-rom-alpha-1", d3Shape.curveCatmullRom.alpha(1), mark.curveCatmullRomAlpha(1)],
    ["catmull-rom-closed", d3Shape.curveCatmullRomClosed, mark.curveCatmullRomClosed],
    ["catmull-rom-open", d3Shape.curveCatmullRomOpen, mark.curveCatmullRomOpen],
    ["monotone-x", d3Shape.curveMonotoneX, mark.curveMonotoneX],
    ["monotone-y", d3Shape.curveMonotoneY, mark.curveMonotoneY],
    ["natural", d3Shape.curveNatural, mark.curveNatural],
    ["bump-x", d3Shape.curveBumpX, mark.curveBumpX],
    ["bump-y", d3Shape.curveBumpY, mark.curveBumpY],
  ];
  const symbols: readonly (readonly [string, unknown, string])[] = [
    ["circle", d3Shape.symbolCircle, "circle"],
    ["cross", d3Shape.symbolCross, "cross"],
    ["diamond", d3Shape.symbolDiamond, "diamond"],
    ["square", d3Shape.symbolSquare, "square"],
    ["star", d3Shape.symbolStar, "star"],
    ["triangle", d3Shape.symbolTriangle, "triangle"],
    ["wye", d3Shape.symbolWye, "wye"],
    ["asterisk", d3Shape.symbolAsterisk, "asterisk"],
    ["diamond2", d3Shape.symbolDiamond2, "diamond2"],
    ["plus", d3Shape.symbolPlus, "plus"],
    ["square2", d3Shape.symbolSquare2, "square2"],
    ["times", d3Shape.symbolTimes, "times"],
    ["triangle2", d3Shape.symbolTriangle2, "triangle2"],
  ];
  const arcs = [
    { innerRadius: 0, outerRadius: 50, startAngle: 0, endAngle: Math.PI / 2 },
    { innerRadius: 20, outerRadius: 50, startAngle: 0.3, endAngle: 2.4 },
    { innerRadius: 20, outerRadius: 50, startAngle: 0, endAngle: 2 * Math.PI },
    { innerRadius: 20, outerRadius: 50, startAngle: 0.3, endAngle: 2.4, padAngle: 0.08 },
    { innerRadius: 20, outerRadius: 50, startAngle: 0.3, endAngle: 2.4, cornerRadius: 6 },
    { innerRadius: 0, outerRadius: 50, startAngle: 0.3, endAngle: 1.1, cornerRadius: 8, padAngle: 0.05 },
    { innerRadius: 20, outerRadius: 50, startAngle: 2.4, endAngle: 0.3 },
  ];
  return [
    ...curves.map(([name, d3Curve, ownCurve]) => ({
      module: "mark",
      name: `curve-line-${name}`,
      subject: () => {
        const recorder = mark.vizPathRecorder();
        shape.vizLine(points, { curve: ownCurve }, recorder);
        return recorder.commands;
      },
      oracle: () => {
        const recorder = mark.vizPathRecorder();
        d3Shape
          .line()
          .curve(d3Curve as never)
          .context(recorder as never)(points as never);
        return recorder.commands;
      },
    })),
    ...curves
      .filter(([name]) => !name.startsWith("bundle"))
      .slice(0, level === "quick" ? 8 : curves.length)
      .map(([name, d3Curve, ownCurve]) => ({
      module: "mark",
      name: `curve-area-${name}`,
      subject: () => {
        const recorder = mark.vizPathRecorder();
        shape.vizArea(points, { curve: ownCurve, y0: () => 0, y1: (point: [number, number]) => point[1] }, recorder);
        return recorder.commands;
      },
      oracle: () => {
        const recorder = mark.vizPathRecorder();
        d3Shape
          .area()
          .curve(d3Curve as never)
          .context(recorder as never)(points as never);
        return recorder.commands;
      },
    })),
    {
      module: "mark",
      name: "line-with-gaps",
      subject: () => {
        const recorder = mark.vizPathRecorder();
        shape.vizLine(points, { defined: (_point, i) => i % 3 !== 1 }, recorder);
        return recorder.commands;
      },
      oracle: () => {
        const recorder = mark.vizPathRecorder();
        d3Shape
          .line()
          .defined((_point, i) => i % 3 !== 1)
          .context(recorder as never)(points as never);
        return recorder.commands;
      },
    },
    ...symbols.map(([name, d3Symbol, ownKind]) => ({
      module: "mark",
      name: `symbol-${name}`,
      subject: () => mark.vizSymbol(ownKind as never, 137),
      oracle: () => {
        const recorder = mark.vizPathRecorder();
        (d3Symbol as { draw(context: unknown, size: number): void }).draw(recorder, 137);
        return recorder.commands;
      },
    })),
    ...arcs.map((options, index) => ({
      module: "shape",
      name: `arc-${index}`,
      subject: () => {
        const recorder = mark.vizPathRecorder();
        shape.vizArc(options, recorder);
        return recorder.commands;
      },
      oracle: () => {
        const recorder = mark.vizPathRecorder();
        d3Shape
          .arc()
          .innerRadius(options.innerRadius)
          .outerRadius(options.outerRadius)
          .startAngle(options.startAngle)
          .endAngle(options.endAngle)
          .padAngle(options.padAngle ?? 0)
          .cornerRadius(options.cornerRadius ?? 0)
          .context(recorder as never)();
        return recorder.commands;
      },
    })),
    {
      module: "shape",
      name: "arc-centroid",
      subject: () => arcs.map((options) => shape.vizArcCentroid(options)),
      oracle: () =>
        arcs.map((options) =>
          d3Shape
            .arc()
            .innerRadius(options.innerRadius)
            .outerRadius(options.outerRadius)
            .startAngle(options.startAngle)
            .endAngle(options.endAngle)
            .centroid(),
        ),
    },
    {
      module: "shape",
      name: "pie-default-and-padded",
      subject: () => [shape.vizPie([12, 3, 40, 7, 0, 21]), shape.vizPie([12, 3, 40, 7, 0, 21], { padAngle: 0.03, startAngle: 0.5, endAngle: 0.5 + Math.PI }), shape.vizPie([12, 3, 40, 7, 0, 21], { sortValues: null })],
      oracle: () => [
        d3Shape.pie()([12, 3, 40, 7, 0, 21]).map(({ data, index, value, startAngle, endAngle, padAngle }) => ({ data, index, value, startAngle, endAngle, padAngle })),
        d3Shape.pie().padAngle(0.03).startAngle(0.5).endAngle(0.5 + Math.PI)([12, 3, 40, 7, 0, 21]).map(({ data, index, value, startAngle, endAngle, padAngle }) => ({ data, index, value, startAngle, endAngle, padAngle })),
        d3Shape.pie().sortValues(null)([12, 3, 40, 7, 0, 21]).map(({ data, index, value, startAngle, endAngle, padAngle }) => ({ data, index, value, startAngle, endAngle, padAngle })),
      ],
    },
    {
      module: "shape",
      name: "links",
      subject: () => [shape.vizLink("horizontal", [0, 0], [100, 60]), shape.vizLink("vertical", [0, 0], [100, 60]), shape.vizLink("radial", [0.4, 30], [2.1, 90])],
      oracle: () => {
        const horizontal = mark.vizPathRecorder();
        d3Shape.linkHorizontal().context(horizontal as never)({ source: [0, 0], target: [100, 60] } as never);
        const vertical = mark.vizPathRecorder();
        d3Shape.linkVertical().context(vertical as never)({ source: [0, 0], target: [100, 60] } as never);
        const radial = mark.vizPathRecorder();
        d3Shape
          .linkRadial()
          .angle((point: number[]) => point[0]!)
          .radius((point: number[]) => point[1]!)
          .context(radial as never)({ source: [0.4, 30], target: [2.1, 90] } as never);
        return [horizontal.commands, vertical.commands, radial.commands];
      },
    },
    {
      module: "shape",
      name: "ribbon",
      subject: () => shape.vizRibbon({ startAngle: 0.2, endAngle: 0.9, radius: 100 }, { startAngle: 2.4, endAngle: 3.1, radius: 100 }),
      oracle: () => {
        const recorder = mark.vizPathRecorder();
        d3Chord
          .ribbon()
          .context(recorder as never)({ source: { startAngle: 0.2, endAngle: 0.9, radius: 100 }, target: { startAngle: 2.4, endAngle: 3.1, radius: 100 } } as never);
        return recorder.commands;
      },
    },
  ];
}

const FRAME = { x0: 10, y0: 20, x1: 170, y1: 120 } as const;

/** 🧭️ Conformance and round-trip checks for `🧭coordinate`; d3 has no coordinate layer, so the
 * evidence is the system's own inverse plus the anchors the LaTeX kernel documents. */
function coordinateChecks(): VizKernelCheck[] {
  const probes: [number, number][] = [
    [0, 0],
    [0.25, 0.5],
    [0.5, 0.5],
    [1, 1],
    [0.8, 0.2],
  ];
  return [
    {
      module: "coordinate",
      name: "cartesian-corners",
      subject: () => {
        const system = coordinate.coordinateCartesian(FRAME);
        return [system.project(0, 0), system.project(1, 0), system.project(0, 1), system.project(1, 1)];
      },
      oracle: () => [
        [10, 120],
        [170, 120],
        [10, 20],
        [170, 20],
      ],
    },
    {
      module: "coordinate",
      name: "cartesian-round-trip",
      subject: () => {
        const system = coordinate.coordinateCartesian(FRAME);
        return probes.map(([u, v]) => system.invert!(...system.project(u, v)));
      },
      oracle: () => probes,
    },
    {
      module: "coordinate",
      name: "polar-round-trip",
      subject: () => {
        const system = coordinate.coordinatePolar(FRAME, { innerRadius: 5, outerRadius: 40 });
        return probes.filter(([u]) => u > 0 && u < 1).map(([u, v]) => system.invert!(...system.project(u, v)));
      },
      oracle: () => probes.filter(([u]) => u > 0 && u < 1),
    },
    {
      module: "coordinate",
      name: "polar-anchors",
      subject: () => {
        const system = coordinate.coordinatePolar(FRAME, { innerRadius: 0, outerRadius: 40 });
        return [system.project(0, 1), system.project(0.25, 1), system.project(0.5, 1)];
      },
      oracle: () => [
        [90, 30],
        [130, 70],
        [90, 110],
      ],
    },
    {
      module: "coordinate",
      name: "logpolar-round-trip",
      subject: () => {
        const system = coordinate.coordinateLogPolar(FRAME, { outerRadius: 40, base: 10 });
        return probes.filter(([u]) => u > 0 && u < 1).map(([u, v]) => system.invert!(...system.project(u, v)));
      },
      oracle: () => probes.filter(([u]) => u > 0 && u < 1),
    },
    {
      module: "coordinate",
      name: "ternary-barycentre",
      subject: () => {
        const system = coordinate.coordinateTernary(FRAME);
        const corners = [system.projectTernary(1, 0, 0), system.projectTernary(0, 1, 0), system.projectTernary(0, 0, 1)];
        const centre = system.projectTernary(1, 1, 1);
        return { centre, corners, average: [corners.reduce((total, point) => total + point[0], 0) / 3, corners.reduce((total, point) => total + point[1], 0) / 3] };
      },
      oracle: () => {
        const system = coordinate.coordinateTernary(FRAME);
        const corners = [system.projectTernary(1, 0, 0), system.projectTernary(0, 1, 0), system.projectTernary(0, 0, 1)];
        return { centre: [corners.reduce((total, point) => total + point[0], 0) / 3, corners.reduce((total, point) => total + point[1], 0) / 3], corners, average: [corners.reduce((total, point) => total + point[0], 0) / 3, corners.reduce((total, point) => total + point[1], 0) / 3] };
      },
    },
    {
      module: "coordinate",
      name: "parallel-axes",
      subject: () => {
        const system = coordinate.coordinateParallel(FRAME, { axes: ["a", "b", "c", "d"] });
        return { axes: system.axes().map((axis) => axis.name), positions: [0, 1 / 3, 2 / 3, 1].map((u) => system.project(u, 0)[0]) };
      },
      oracle: () => ({ axes: ["a", "b", "c", "d"], positions: [10, 63.33333333333333, 116.66666666666667, 170] }),
    },
    {
      module: "coordinate",
      name: "barycentric-centre",
      subject: () => coordinate.coordinateBarycentric(FRAME, 5).projectWeights([1, 1, 1, 1, 1]),
      oracle: () => [90, 70],
    },
  ];
}

type DemoTree = { name: string; value?: number; children?: DemoTree[] };

function demoTree(level: Level): DemoTree {
  const breadth = level === "quick" ? 3 : level === "long" ? 5 : 7;
  const build = (prefix: string, depth: number): DemoTree => {
    if (depth === 0) return { name: prefix, value: 1 + ((prefix.length * 7 + prefix.charCodeAt(prefix.length - 1)) % 23) };
    return { name: prefix, children: Array.from({ length: breadth }, (_, i) => build(`${prefix}${String.fromCharCode(97 + i)}`, depth - 1)) };
  };
  return build("r", level === "quick" ? 2 : 3);
}

function demoFlatTree(): { id: string; parentId?: string; value: number }[] {
  return [
    { id: "root", value: 0 },
    { id: "a", parentId: "root", value: 7 },
    { id: "b", parentId: "root", value: 3 },
    { id: "c", parentId: "root", value: 11 },
    { id: "a1", parentId: "a", value: 4 },
    { id: "a2", parentId: "a", value: 9 },
    { id: "b1", parentId: "b", value: 2 },
    { id: "c1", parentId: "c", value: 6 },
    { id: "c2", parentId: "c", value: 5 },
    { id: "c3", parentId: "c", value: 1 },
  ];
}

/** 🌳 Differential checks for `🌳hierarchy` against `d3-hierarchy`. */
async function hierarchyChecks(level: Level): Promise<VizKernelCheck[]> {
  const d3Hierarchy = await import("d3-hierarchy");
  const data = demoTree(level);
  const flat = demoFlatTree();
  const ownRoot = (): hierarchy.VizHierarchyNode<DemoTree> => hierarchy.vizHierarchy<DemoTree>(data).sum((node) => node.value ?? 0).sort((a, b) => (b.value ?? 0) - (a.value ?? 0));
  const d3Root = () => d3Hierarchy.hierarchy(data).sum((node) => (node as DemoTree).value ?? 0).sort((a, b) => (b.value ?? 0) - (a.value ?? 0));
  const ownNodes = (root: hierarchy.VizHierarchyNode<DemoTree>, fields: readonly string[]) => {
    const out: number[][] = [];
    root.eachBefore((node) => out.push(fields.map((field) => (node as unknown as Record<string, number>)[field]!)));
    return out;
  };
  const d3Nodes = (root: { eachBefore(callback: (node: unknown) => void): unknown }, fields: readonly string[]) => {
    const out: number[][] = [];
    root.eachBefore((node) => out.push(fields.map((field) => (node as Record<string, number>)[field]!)));
    return out;
  };
  const tilings = ["squarify", "slice", "dice", "slice-dice", "binary", "resquarify"] as const;
  const d3Tilings = { squarify: d3Hierarchy.treemapSquarify, slice: d3Hierarchy.treemapSlice, dice: d3Hierarchy.treemapDice, "slice-dice": d3Hierarchy.treemapSliceDice, binary: d3Hierarchy.treemapBinary, resquarify: d3Hierarchy.treemapResquarify };
  return [
    {
      module: "hierarchy",
      name: "sum-count-depth-height",
      subject: () => ownNodes(ownRoot().count(), ["depth", "height", "value"]),
      oracle: () => d3Nodes(d3Root().count() as never, ["depth", "height", "value"]),
    },
    {
      module: "hierarchy",
      name: "stratify",
      subject: () => {
        const root = hierarchy.vizStratify(flat).sum((row) => row.value);
        return { ids: root.descendants().map((node) => node.id), values: root.descendants().map((node) => node.value), leaves: root.leaves().length, links: root.links().length };
      },
      oracle: () => {
        const root = d3Hierarchy.stratify<{ id: string; parentId?: string; value: number }>()(flat).sum((row) => row.value);
        return { ids: root.descendants().map((node) => node.id), values: root.descendants().map((node) => node.value), leaves: root.leaves().length, links: root.links().length };
      },
    },
    {
      module: "hierarchy",
      name: "tree-normalised",
      subject: () => ownNodes(hierarchy.vizTree(ownRoot(), { size: [120, 80] }), ["x", "y"]),
      oracle: () => d3Nodes(d3Hierarchy.tree().size([120, 80])(d3Root() as never) as never, ["x", "y"]),
    },
    {
      module: "hierarchy",
      name: "tree-node-size",
      subject: () => ownNodes(hierarchy.vizTree(ownRoot(), { nodeSize: [12, 30] }), ["x", "y"]),
      oracle: () => d3Nodes(d3Hierarchy.tree().nodeSize([12, 30])(d3Root() as never) as never, ["x", "y"]),
    },
    {
      module: "hierarchy",
      name: "cluster",
      subject: () => ownNodes(hierarchy.vizCluster(ownRoot(), { size: [120, 80] }), ["x", "y"]),
      oracle: () => d3Nodes(d3Hierarchy.cluster().size([120, 80])(d3Root() as never) as never, ["x", "y"]),
    },
    ...tilings.map((tile) => ({
      module: "hierarchy",
      name: `treemap-${tile}`,
      subject: () => ownNodes(hierarchy.vizTreemap(ownRoot(), { size: [160, 100], tile, paddingInner: 1, paddingOuter: 2 }), ["x0", "y0", "x1", "y1"]),
      oracle: () =>
        d3Nodes(
          d3Hierarchy
            .treemap()
            .size([160, 100])
            .tile(d3Tilings[tile])
            .paddingInner(1)
            .paddingOuter(2)(d3Root() as never) as never,
          ["x0", "y0", "x1", "y1"],
        ),
    })),
    {
      module: "hierarchy",
      name: "treemap-rounded",
      subject: () => ownNodes(hierarchy.vizTreemap(ownRoot(), { size: [160, 100], round: true }), ["x0", "y0", "x1", "y1"]),
      oracle: () => d3Nodes(d3Hierarchy.treemap().size([160, 100]).round(true)(d3Root() as never) as never, ["x0", "y0", "x1", "y1"]),
    },
    {
      module: "hierarchy",
      name: "partition",
      subject: () => ownNodes(hierarchy.vizPartition(ownRoot(), { size: [160, 100], padding: 1 }), ["x0", "y0", "x1", "y1"]),
      oracle: () => d3Nodes(d3Hierarchy.partition().size([160, 100]).padding(1)(d3Root() as never) as never, ["x0", "y0", "x1", "y1"]),
    },
    {
      module: "hierarchy",
      name: "pack",
      subject: () => ownNodes(hierarchy.vizPack(ownRoot(), { size: [160, 160] }), ["x", "y", "r"]),
      oracle: () => d3Nodes(d3Hierarchy.pack().size([160, 160])(d3Root() as never) as never, ["x", "y", "r"]),
    },
    {
      module: "hierarchy",
      name: "pack-padded",
      subject: () => ownNodes(hierarchy.vizPack(ownRoot(), { size: [160, 160], padding: 3 }), ["x", "y", "r"]),
      oracle: () => d3Nodes(d3Hierarchy.pack().size([160, 160]).padding(3)(d3Root() as never) as never, ["x", "y", "r"]),
    },
    {
      module: "hierarchy",
      name: "pack-enclose",
      subject: () => {
        const circles = [
          { x: 0, y: 0, r: 5 },
          { x: 10, y: 3, r: 7 },
          { x: -4, y: 9, r: 2 },
          { x: 6, y: -8, r: 4 },
          { x: 1, y: 1, r: 9 },
        ];
        return hierarchy.vizPackEnclose(circles);
      },
      oracle: () =>
        d3Hierarchy.packEnclose([
          { x: 0, y: 0, r: 5 },
          { x: 10, y: 3, r: 7 },
          { x: -4, y: 9, r: 2 },
          { x: 6, y: -8, r: 4 },
          { x: 1, y: 1, r: 9 },
        ]),
    },
    {
      module: "hierarchy",
      name: "lcg-stream",
      subject: () => {
        const random = hierarchy.vizLcg();
        return Array.from({ length: 8 }, () => random());
      },
      oracle: () => {
        let s = 1;
        return Array.from({ length: 8 }, () => {
          s = (1664525 * s + 1013904223) % 4294967296;
          return s / 4294967296;
        });
      },
    },
  ];
}

function demoGraph(level: Level): { nodes: string[]; edges: [string, string][] } {
  const n = level === "quick" ? 12 : level === "long" ? 40 : 120;
  const nodes = Array.from({ length: n }, (_, i) => `n${i}`);
  const edges: [string, string][] = [];
  for (let i = 1; i < n; i += 1) {
    edges.push([`n${(i - 1) >> 1}`, `n${i}`]);
    if (i % 5 === 0 && i > 5) edges.push([`n${i - 5}`, `n${i}`]);
  }
  return { nodes, edges };
}

const CHORD_MATRIX = [
  [11975, 5871, 8916, 2868],
  [1951, 10048, 2060, 6171],
  [8010, 16145, 8090, 8045],
  [1013, 990, 940, 6907],
];

const SANKEY_INPUT = {
  nodes: [{ name: "a" }, { name: "b" }, { name: "c" }, { name: "d" }, { name: "e" }, { name: "f" }],
  links: [
    { source: "a", target: "c", value: 12 },
    { source: "b", target: "c", value: 7 },
    { source: "b", target: "d", value: 3 },
    { source: "c", target: "e", value: 10 },
    { source: "c", target: "f", value: 9 },
    { source: "d", target: "f", value: 3 },
    { source: "a", target: "d", value: 4 },
  ],
};

/** 🕸️ Differential checks for `🕸️network` against `d3-force` and `d3-chord`; the graph layouts have
 * no d3 counterpart and are checked against their own invariants. */
async function networkChecks(level: Level): Promise<VizKernelCheck[]> {
  const d3Force = await import("d3-force");
  const d3Chord = await import("d3-chord");
  const graph = demoGraph(level);
  const ticks = level === "quick" ? 30 : level === "long" ? 150 : 300;
  const makeOwnNodes = () => graph.nodes.map((id) => ({ id, x: Number.NaN, y: Number.NaN, vx: Number.NaN, vy: Number.NaN }));
  const makeD3Nodes = () => graph.nodes.map((id) => ({ id }));
  const positions = (nodes: readonly { x: number; y: number }[]) => nodes.map((node) => [node.x, node.y]);
  return [
    {
      module: "network",
      name: "quadtree-invariants",
      subject: () => {
        const points = graph.nodes.map((_, i) => ({ x: (i * 37) % 101, y: (i * 53) % 89 }));
        const tree = network.vizQuadtree(points, (point) => point.x, (point) => point.y);
        let leaves = 0;
        let contained = true;
        tree.visit((node, x0, y0, x1, y1) => {
          if (Array.isArray(node)) return;
          let leaf: { data: { x: number; y: number }; next?: unknown } | undefined = node as never;
          while (leaf !== undefined) {
            leaves += 1;
            if (leaf.data.x < x0 || leaf.data.x > x1 || leaf.data.y < y0 || leaf.data.y > y1) contained = false;
            leaf = leaf.next as never;
          }
        });
        const square = Math.abs(tree.x1 - tree.x0 - (tree.y1 - tree.y0)) < 1e-9;
        return { leaves, contained, square, covers: tree.x0 <= 0 && tree.y0 <= 0 };
      },
      oracle: () => ({ leaves: graph.nodes.length, contained: true, square: true, covers: true }),
    },
    {
      module: "network",
      name: "force-initial-positions",
      subject: () => {
        const nodes = makeOwnNodes();
        network.initializeVizForceNodes(nodes);
        return positions(nodes);
      },
      oracle: () => {
        const nodes = makeD3Nodes();
        d3Force.forceSimulation(nodes as never).stop();
        return positions(nodes as never);
      },
    },
    {
      module: "network",
      name: "force-link-manybody-center",
      subject: () => {
        const nodes = makeOwnNodes();
        const links = graph.edges.map(([source, target]) => ({ source: graph.nodes.indexOf(source), target: graph.nodes.indexOf(target) }));
        const simulation = new network.VizForceSimulation(nodes);
        simulation.force("link", network.forceVizLink(links as never));
        simulation.force("charge", network.forceVizManyBody());
        simulation.force("center", network.forceVizCenter(0, 0));
        simulation.tick(ticks);
        return positions(nodes);
      },
      oracle: () => {
        const nodes = makeD3Nodes();
        const links = graph.edges.map(([source, target]) => ({ source: graph.nodes.indexOf(source), target: graph.nodes.indexOf(target) }));
        const simulation = d3Force
          .forceSimulation(nodes as never)
          .stop()
          .force("link", d3Force.forceLink(links as never))
          .force("charge", d3Force.forceManyBody())
          .force("center", d3Force.forceCenter(0, 0));
        simulation.tick(ticks);
        return positions(nodes as never);
      },
      tolerance: 1e-6,
    },
    {
      module: "network",
      name: "force-collide-x-y-radial",
      subject: () => {
        const nodes = makeOwnNodes();
        const simulation = new network.VizForceSimulation(nodes);
        simulation.force("collide", network.forceVizCollide({ radius: 6 }));
        simulation.force("x", network.forceVizX({ x: 12 }));
        simulation.force("y", network.forceVizY({ y: -8 }));
        simulation.force("radial", network.forceVizRadial(40, 0, 0, 0.05));
        simulation.tick(ticks);
        return positions(nodes);
      },
      oracle: () => {
        const nodes = makeD3Nodes();
        const simulation = d3Force
          .forceSimulation(nodes as never)
          .stop()
          .force("collide", d3Force.forceCollide(6))
          .force("x", d3Force.forceX(12))
          .force("y", d3Force.forceY(-8))
          .force("radial", d3Force.forceRadial(40, 0, 0).strength(0.05));
        simulation.tick(ticks);
        return positions(nodes as never);
      },
      tolerance: 1e-6,
    },
    {
      module: "network",
      name: "chord",
      subject: () => {
        const layout = network.vizChord(CHORD_MATRIX);
        return { groups: layout.groups, chords: layout.chords.map((chord) => ({ source: chord.source, target: chord.target })) };
      },
      oracle: () => {
        const layout = d3Chord.chord()(CHORD_MATRIX);
        return { groups: layout.groups.map(({ index, startAngle, endAngle, value }) => ({ index, startAngle, endAngle, value })), chords: layout.map((chord) => ({ source: { index: chord.source.index, startAngle: chord.source.startAngle, endAngle: chord.source.endAngle, value: chord.source.value }, target: { index: chord.target.index, startAngle: chord.target.startAngle, endAngle: chord.target.endAngle, value: chord.target.value } })) };
      },
    },
    {
      module: "network",
      name: "chord-padded",
      subject: () => network.vizChord(CHORD_MATRIX, { padAngle: 0.05 }).groups,
      oracle: () => d3Chord.chord().padAngle(0.05)(CHORD_MATRIX).groups.map(({ index, startAngle, endAngle, value }) => ({ index, startAngle, endAngle, value })),
    },
    {
      module: "network",
      name: "circular-and-arc-layout",
      subject: () => {
        const circular = network.vizCircularLayout({ nodes: ["a", "b", "c", "d"], edges: [] }, { radius: 10 });
        const arc = network.vizArcLayout({ nodes: ["a", "b", "c", "d"], edges: [] }, { length: 30 });
        return { circular: circular.map((node) => [node.x, node.y]), arc: arc.map((node) => [node.x, node.y]) };
      },
      oracle: () => ({
        circular: [
          [0, -10],
          [10, 0],
          [0, 10],
          [-10, 0],
        ],
        arc: [
          [0, 0],
          [10, 0],
          [20, 0],
          [30, 0],
        ],
      }),
    },
    {
      module: "network",
      name: "layered-invariants",
      subject: () => {
        const layout = network.vizLayeredLayout(graph, { layerGap: 20, nodeGap: 12 });
        const layerOf = new Map(layout.nodes.map((node) => [node.id, node.layer!] as const));
        const monotone = graph.edges.every(([source, target]) => layerOf.get(source)! !== layerOf.get(target)!);
        const distinct = layout.layers.every((rows) => new Set(rows).size === rows.length);
        const covered = layout.nodes.length === graph.nodes.length;
        const spacing = layout.layers.every((rows, l) => layout.nodes.filter((node) => node.layer === l).every((node, i, all) => i === 0 || Math.abs(node.x - all[i - 1]!.x - 12) < 1e-9));
        return { monotone, distinct, covered, spacing, layerCount: layout.layers.length };
      },
      oracle: () => {
        const longest = new Map<string, number>(graph.nodes.map((id) => [id, 0]));
        let changed = true;
        while (changed) {
          changed = false;
          for (const [source, target] of graph.edges) {
            const candidate = longest.get(source)! + 1;
            if (candidate > longest.get(target)!) {
              longest.set(target, candidate);
              changed = true;
            }
          }
        }
        return { monotone: true, distinct: true, covered: true, spacing: true, layerCount: Math.max(...longest.values()) + 1 };
      },
    },
  ];
}

/** 🌊 Differential checks for `🌊flow` against `d3-sankey`. */
async function flowChecks(_level: Level): Promise<VizKernelCheck[]> {
  const d3Sankey = await import("d3-sankey");
  const alignments = ["left", "right", "center", "justify"] as const;
  const d3Alignments = { left: d3Sankey.sankeyLeft, right: d3Sankey.sankeyRight, center: d3Sankey.sankeyCenter, justify: d3Sankey.sankeyJustify };
  return [
    ...alignments.map((align) => ({
      module: "flow",
      name: `sankey-${align}`,
      subject: () => {
        const layout = flow.vizSankey(SANKEY_INPUT, { extent: [[0, 0], [400, 200]], align });
        return { nodes: layout.nodes.map((node) => [node.x0, node.x1, node.y0, node.y1]), links: layout.links.map((link) => [link.y0, link.y1, link.width]) };
      },
      oracle: () => {
        const layout = d3Sankey
          .sankey()
          .nodeId((node: { name: string }) => node.name)
          .nodeAlign(d3Alignments[align])
          .extent([
            [0, 0],
            [400, 200],
          ])({ nodes: SANKEY_INPUT.nodes.map((node) => ({ ...node })), links: SANKEY_INPUT.links.map((link) => ({ ...link })) } as never);
        return { nodes: layout.nodes.map((node: never) => [(node as { x0: number }).x0, (node as { x1: number }).x1, (node as { y0: number }).y0, (node as { y1: number }).y1]), links: layout.links.map((link: never) => [(link as { y0: number }).y0, (link as { y1: number }).y1, (link as { width: number }).width]) };
      },
      tolerance: 1e-9,
    })),
    {
      module: "flow",
      name: "sankey-link-path",
      subject: () => {
        const layout = flow.vizSankey(SANKEY_INPUT, { extent: [[0, 0], [400, 200]] });
        return flow.vizSankeyLinkHorizontal(layout.links[0]!);
      },
      oracle: () => {
        const layout = d3Sankey
          .sankey()
          .nodeId((node: { name: string }) => node.name)
          .extent([
            [0, 0],
            [400, 200],
          ])({ nodes: SANKEY_INPUT.nodes.map((node) => ({ ...node })), links: SANKEY_INPUT.links.map((link) => ({ ...link })) } as never);
        const recorder = mark.vizPathRecorder();
        (d3Sankey.sankeyLinkHorizontal() as unknown as { context(c: unknown): (link: unknown) => void }).context(recorder)(layout.links[0]);
        return recorder.commands;
      },
    },
    {
      module: "flow",
      name: "alluvial-conservation",
      subject: () => {
        const rows = [
          { stages: ["low", "mid", "high"], value: 5 },
          { stages: ["low", "mid", "mid"], value: 3 },
          { stages: ["high", "mid", "low"], value: 7 },
          { stages: ["mid", "high", "high"], value: 2 },
        ];
        const layout = flow.vizAlluvial(rows, { extent: [[0, 0], [300, 150]] });
        const firstStage = layout.nodes.filter((node) => node.layer === 0).reduce((total, node) => total + node.value, 0);
        return { stages: layout.stages.map((stage) => stage.length), firstStage, linkTotal: layout.links.reduce((total, link) => total + link.value, 0) };
      },
      oracle: () => ({ stages: [3, 2, 3], firstStage: 17, linkTotal: 34 }),
    },
  ];
}

const GEO_PROBES: [number, number][] = [
  [0, 0],
  [10, 45],
  [-73.5, 40.7],
  [139.7, 35.7],
  [-58.4, -34.6],
  [18.4, -33.9],
  [-0.1, 51.5],
  [151.2, -33.9],
  [37.6, 55.8],
];

const DEMO_POLYGON = {
  type: "Polygon" as const,
  coordinates: [
    [
      [-10, -5],
      [-11, 6],
      [-2, 14],
      [12, 8],
      [10, -5],
      [-10, -5],
    ] as [number, number][],
  ],
};

/** 🌍 Differential checks for `🌍geo` against `d3-geo`. */
async function geoChecks(level: Level): Promise<VizKernelCheck[]> {
  const d3Geo = await import("d3-geo");
  const kinds = ["equirectangular", "mercator", "transverse-mercator", "azimuthal-equal-area", "azimuthal-equidistant", "gnomonic", "orthographic", "stereographic", "conic-conformal", "conic-equal-area", "conic-equidistant", "albers", "equal-earth", "natural-earth"] as const;
  const d3Factories = {
    equirectangular: d3Geo.geoEquirectangular,
    mercator: d3Geo.geoMercator,
    "transverse-mercator": d3Geo.geoTransverseMercator,
    "azimuthal-equal-area": d3Geo.geoAzimuthalEqualArea,
    "azimuthal-equidistant": d3Geo.geoAzimuthalEquidistant,
    gnomonic: d3Geo.geoGnomonic,
    orthographic: d3Geo.geoOrthographic,
    stereographic: d3Geo.geoStereographic,
    "conic-conformal": d3Geo.geoConicConformal,
    "conic-equal-area": d3Geo.geoConicEqualArea,
    "conic-equidistant": d3Geo.geoConicEquidistant,
    albers: d3Geo.geoAlbers,
    "equal-earth": d3Geo.geoEqualEarth,
    "natural-earth": d3Geo.geoNaturalEarth1,
  };
  const probes = level === "quick" ? GEO_PROBES.slice(0, 5) : GEO_PROBES;
  return [
    ...kinds.map((kind) => ({
      module: "geo",
      name: `project-${kind}`,
      subject: () => probes.map((point) => geo.vizGeoProjection(kind)(point)),
      oracle: () => probes.map((point) => d3Factories[kind]()(point)),
      tolerance: 1e-9,
    })),
    ...kinds
      .filter((kind) => kind !== "natural-earth" && kind !== "gnomonic")
      .map((kind) => ({
        module: "geo",
        name: `invert-${kind}`,
        subject: () => probes.map((point) => geo.vizGeoProjection(kind).invert(geo.vizGeoProjection(kind)(point))),
        oracle: () => probes.map((point) => d3Factories[kind]().invert!(d3Factories[kind]()(point)!)),
        tolerance: 1e-7,
      })),
    {
      module: "geo",
      name: "project-rotated-and-scaled",
      subject: () => probes.map((point) => geo.vizGeoProjection("orthographic", { rotate: [30, -20, 15], scale: 300, translate: [100, 90] })(point)),
      oracle: () => probes.map((point) => d3Geo.geoOrthographic().rotate([30, -20, 15]).scale(300).translate([100, 90])(point)),
      tolerance: 1e-9,
    },
    {
      module: "geo",
      name: "graticule-lines",
      subject: () => {
        const grid = geo.vizGraticule();
        return { count: grid.coordinates.length, first: grid.coordinates[0], last: grid.coordinates[grid.coordinates.length - 1] };
      },
      oracle: () => {
        const grid = d3Geo.geoGraticule()() as { coordinates: [number, number][][] };
        return { count: grid.coordinates.length, first: grid.coordinates[0], last: grid.coordinates[grid.coordinates.length - 1] };
      },
      tolerance: 1e-9,
    },
    {
      module: "geo",
      name: "path-bounds-area-centroid",
      subject: () => {
        const projection = geo.vizGeoProjection("equirectangular");
        const bounds = geo.vizGeoBounds(DEMO_POLYGON, projection);
        return { bounds: [bounds.x0, bounds.y0, bounds.x1, bounds.y1], area: geo.vizGeoArea(DEMO_POLYGON, projection), centroid: geo.vizGeoCentroid(DEMO_POLYGON, projection) };
      },
      oracle: () => {
        const path = d3Geo.geoPath(d3Geo.geoEquirectangular());
        const bounds = path.bounds(DEMO_POLYGON as never);
        return { bounds: [bounds[0][0], bounds[0][1], bounds[1][0], bounds[1][1]], area: path.area(DEMO_POLYGON as never), centroid: path.centroid(DEMO_POLYGON as never) };
      },
      tolerance: 1e-6,
    },
    {
      module: "geo",
      name: "path-commands",
      subject: () => geo.vizGeoPath(DEMO_POLYGON, geo.vizGeoProjection("equirectangular")),
      oracle: () => {
        const recorder = mark.vizPathRecorder();
        d3Geo.geoPath(d3Geo.geoEquirectangular().precision(0), recorder as never)(DEMO_POLYGON as never);
        return recorder.commands;
      },
      tolerance: 1e-9,
    },
    {
      module: "geo",
      name: "fit-extent",
      subject: () => {
        const fitted = geo.vizGeoFitExtent(
          geo.vizGeoProjection("equirectangular"),
          [
            [0, 0],
            [200, 150],
          ],
          DEMO_POLYGON,
          geo.rawEquirectangular,
        );
        return { scale: fitted.scale(), translate: fitted.translate() };
      },
      oracle: () => {
        const fitted = d3Geo.geoEquirectangular().fitExtent(
          [
            [0, 0],
            [200, 150],
          ],
          DEMO_POLYGON as never,
        );
        return { scale: fitted.scale(), translate: fitted.translate() };
      },
      tolerance: 1e-6,
    },
    {
      module: "geo",
      name: "great-circle-distance",
      subject: () => [geo.vizGeoDistance([0, 0], [0, 90]), geo.vizGeoDistance([-73.5, 40.7], [139.7, 35.7]), geo.vizGeoDistance([10, 10], [10, 10])],
      oracle: () => [d3Geo.geoDistance([0, 0], [0, 90]), d3Geo.geoDistance([-73.5, 40.7], [139.7, 35.7]), d3Geo.geoDistance([10, 10], [10, 10])],
      tolerance: 1e-9,
    },
  ];
}

function spatialPoints(level: Level): [number, number][] {
  const n = level === "quick" ? 24 : level === "long" ? 90 : 240;
  return Array.from({ length: n }, (_, i) => [50 + 40 * Math.cos(i * 2.399963229728653) * Math.sqrt(i + 1) * 0.21, 50 + 40 * Math.sin(i * 2.399963229728653) * Math.sqrt(i + 1) * 0.21] as [number, number]);
}

/** 📍 Differential checks for `📍spatial` against `d3-delaunay`, `d3-hexbin` and `d3-contour`. */
async function spatialChecks(level: Level): Promise<VizKernelCheck[]> {
  const d3Delaunay = await import("d3-delaunay");
  const d3Hexbin = await import("d3-hexbin");
  const d3Contour = await import("d3-contour");
  const points = spatialPoints(level);
  const raster = { width: 20, height: 16 };
  const values = Array.from({ length: raster.width * raster.height }, (_, i) => {
    const x = (i % raster.width) - raster.width / 2;
    const y = ((i / raster.width) | 0) - raster.height / 2;
    return Math.sin(x / 3) * Math.cos(y / 2) * 10 + x * 0.4 - y * 0.3;
  });
  const normalize = (triangles: readonly (readonly number[])[]) =>
    triangles
      .map((triangle) => [...triangle].sort((a, b) => a - b))
      .sort((a, b) => a[0]! - b[0]! || a[1]! - b[1]! || a[2]! - b[2]!);
  return [
    {
      module: "spatial",
      name: "delaunay-triangles",
      subject: () => normalize(spatial.vizDelaunay(points).triangles),
      oracle: () => {
        const delaunay = d3Delaunay.Delaunay.from(points);
        const out: number[][] = [];
        for (let i = 0; i < delaunay.triangles.length; i += 3) out.push([delaunay.triangles[i]!, delaunay.triangles[i + 1]!, delaunay.triangles[i + 2]!]);
        return normalize(out);
      },
    },
    {
      module: "spatial",
      name: "convex-hull",
      subject: () => {
        const hull = spatial.vizConvexHull(points);
        return [...hull].sort((a, b) => a - b);
      },
      oracle: () => [...d3Delaunay.Delaunay.from(points).hull].sort((a, b) => a - b),
    },
    {
      module: "spatial",
      name: "voronoi-cell-areas",
      subject: () => {
        const voronoi = spatial.vizVoronoi(points, [0, 0, 100, 100]);
        return voronoi.cells.map((cell) => Math.abs(spatial.vizPolygonArea(cell)));
      },
      oracle: () => {
        const voronoi = d3Delaunay.Delaunay.from(points).voronoi([0, 0, 100, 100]);
        return points.map((_, i) => {
          const cell = voronoi.cellPolygon(i);
          if (cell === null) return 0;
          return Math.abs(spatial.vizPolygonArea(cell.slice(0, -1) as never));
        });
      },
      tolerance: 1e-9,
    },
    {
      module: "spatial",
      name: "circumcenters",
      subject: () => {
        const triangulation = spatial.vizDelaunay(points);
        return normalize(triangulation.triangles).map((triangle) => spatial.vizCircumcenter(points[triangle[0]!]!, points[triangle[1]!]!, points[triangle[2]!]!));
      },
      oracle: () => {
        const delaunay = d3Delaunay.Delaunay.from(points);
        const out: number[][] = [];
        for (let i = 0; i < delaunay.triangles.length; i += 3) out.push([delaunay.triangles[i]!, delaunay.triangles[i + 1]!, delaunay.triangles[i + 2]!]);
        return normalize(out).map((triangle) => spatial.vizCircumcenter(points[triangle[0]!]!, points[triangle[1]!]!, points[triangle[2]!]!));
      },
      tolerance: 1e-9,
    },
    {
      module: "spatial",
      name: "hexbin",
      subject: () => {
        const bins = spatial.vizHexbin(points, { radius: 8 });
        return bins.map((bin) => [bin.x, bin.y, bin.values.length]).sort((a, b) => a[0]! - b[0]! || a[1]! - b[1]!);
      },
      oracle: () => {
        const bins = d3Hexbin.hexbin().radius(8)(points as never) as unknown as ({ x: number; y: number; length: number })[];
        return bins.map((bin) => [bin.x, bin.y, bin.length]).sort((a, b) => a[0]! - b[0]! || a[1]! - b[1]!);
      },
      tolerance: 1e-9,
    },
    {
      module: "spatial",
      name: "hexagon-outline",
      subject: () => spatial.vizHexagon(8),
      oracle: () =>
        Array.from({ length: 6 }, (_, i) => {
          const angle = (i * Math.PI) / 3;
          return [8 * Math.sin(angle), -8 * Math.cos(angle)];
        }),
      tolerance: 1e-9,
    },
    {
      module: "spatial",
      name: "contours-fixed-thresholds",
      subject: () => spatial.vizContours(values, [raster.width, raster.height], [-8, -3, 0, 3, 8]).map((contour) => ({ value: contour.value, coordinates: contour.coordinates })),
      oracle: () =>
        d3Contour
          .contours()
          .size([raster.width, raster.height])
          .thresholds([-8, -3, 0, 3, 8])(values)
          .map((contour) => ({ value: contour.value, coordinates: contour.coordinates })),
      tolerance: 1e-9,
    },
    {
      module: "spatial",
      name: "contours-default-thresholds",
      subject: () => spatial.vizContours(values, [raster.width, raster.height]).map((contour) => contour.value),
      oracle: () =>
        d3Contour
          .contours()
          .size([raster.width, raster.height])(values)
          .map((contour) => contour.value),
      tolerance: 1e-9,
    },
    {
      module: "spatial",
      name: "density-monotone-and-normalised",
      subject: () => {
        const inside = spatialPoints("quick").filter((point) => point[0] > 30 && point[0] < 70 && point[1] > 30 && point[1] < 70);
        const grid = spatial.vizDensity2d(inside, { extent: [0, 0, 100, 100], cellSize: 2.5, bandwidth: 6 });
        const total = grid.values.reduce((sum, value) => sum + value, 0) * grid.cellSize * grid.cellSize;
        const peak = grid.values.indexOf(Math.max(...grid.values));
        const contours = spatial.vizDensityContours(grid, 4).length;
        return { integral: Math.round(total * 100) / 100, positive: grid.values.every((value) => value >= 0), cells: grid.values.length, peakInside: peak > 0 && peak < grid.values.length - 1, contours: contours > 0 };
      },
      oracle: () => ({ integral: 1, positive: true, cells: 41 * 41, peakInside: true, contours: true }),
      tolerance: 0.02,
    },
  ];
}

const DEMO_SPEC: render.VizChartSpecification = {
  width: 160,
  height: 100,
  margin: { top: 8, right: 8, bottom: 14, left: 16 },
  theme: { appearance: "light" },
  language: "en",
  tables: [
    {
      name: "demo",
      columns: ["label", "value"],
      rows: [
        { label: "a", value: 12 },
        { label: "b", value: 30 },
        { label: "c", value: 7 },
        { label: "d", value: 22 },
      ],
    },
  ],
  scales: [
    { name: "x", kind: "band", domain: ["a", "b", "c", "d"], range: [16, 152], options: { padding: 0.2 } },
    { name: "y", kind: "linear", domain: [0, 30], range: [86, 8] },
  ],
  coordinate: { kind: "cartesian" },
  layers: [
    { mark: "bar", data: "demo", encodings: { x: { column: "label", scale: "x" }, y: { column: "value", scale: "y" }, y2: { value: 86 } } },
    { mark: "point", data: "demo", options: { symbol: "diamond", size: 20 }, encodings: { x: { column: "label", scale: "x" }, y: { column: "value", scale: "y" } } },
    { mark: "text", data: "demo", encodings: { x: { column: "label", scale: "x" }, y: { value: 6 }, text: { column: "label" } } },
  ],
  guides: [
    { kind: "axis", scale: "x", orient: "bottom", ticks: 4 },
    { kind: "axis", scale: "y", orient: "left", ticks: 4, grid: true },
  ],
};

/** 🎨 Conformance checks for `🎨theme`: the palette is read from the token generator, so the check
 * is that the twin reads what the LaTeX stylesheet is generated from. */
function themeChecks(): VizKernelCheck[] {
  return [
    {
      module: "theme",
      name: "palette-matches-token-generator",
      subject: () => {
        const light = theme.vizPalette("light");
        const dark = theme.vizPalette("dark");
        return { lightCount: light.categorical.length, darkCount: dark.categorical.length, schemes: Object.keys(light.schemes).sort(), distinct: new Set(light.categorical).size === light.categorical.length, differs: light.categorical.some((color, i) => color !== dark.categorical[i]) };
      },
      oracle: async () => {
        const paints = await import("../../../../🎨print-design-token-paints/🟦️.ts");
        const lines = paints.renderVisualizationPalette(paints.loadPrintDesignTokens());
        const light = lines.filter((line) => line.includes("semio-presence-light-"));
        const dark = lines.filter((line) => line.includes("semio-presence-dark-"));
        const schemes = lines.map((line) => /^\s*([a-z-]+)\s*\/\s*light\s*=/.exec(line)?.[1]).filter((name): name is string => name !== undefined);
        return { lightCount: light.length, darkCount: dark.length, schemes: [...schemes].sort(), distinct: true, differs: true };
      },
    },
    {
      module: "theme",
      name: "scheme-interpolator-endpoints",
      subject: () => {
        const palette = theme.vizPalette("light");
        const interpolate = theme.vizSchemeInterpolator(palette, "primary");
        return [interpolate(0), interpolate(1), interpolate(0.5).length];
      },
      oracle: () => {
        const palette = theme.vizPalette("light");
        const stops = palette.schemes.primary!;
        return [stops[0], stops[stops.length - 1], 7];
      },
    },
    {
      module: "theme",
      name: "colour-round-trip-and-contrast",
      subject: () => ({ roundTrip: theme.vizFormatColor(theme.vizParseColor("#ff344f")), black: Math.round(theme.vizContrastRatio("#ffffff", "#000000") * 100) / 100, same: theme.vizContrastRatio("#ff344f", "#ff344f"), mid: theme.vizInterpolateRgb("#000000", "#ffffff")(0.5) }),
      oracle: () => ({ roundTrip: "#ff344f", black: 21, same: 1, mid: "#808080" }),
    },
    {
      module: "theme",
      name: "grayscale-safe-encodings",
      subject: () => ({ dashes: theme.VIZ_DASH_PATTERNS.length, hatches: theme.VIZ_HATCH_PATTERNS.length, cycles: theme.vizDashPattern(0) === theme.vizDashPattern(theme.VIZ_DASH_PATTERNS.length), hatchCycles: theme.vizHatchPattern(-1) === theme.vizHatchPattern(theme.VIZ_HATCH_PATTERNS.length - 1) }),
      oracle: () => ({ dashes: 7, hatches: 8, cycles: true, hatchCycles: true }),
    },
  ];
}

/** 🖼️ Specification-vector checks for `🖼️render`: d3 has no scene graph and no TikZ emitter, so the
 * evidence is the specification the two emitters must both satisfy. */
function renderChecks(): VizKernelCheck[] {
  return [
    {
      module: "render",
      name: "scene-graph-shape",
      subject: () => {
        const scene = render.renderVizScene(DEMO_SPEC);
        const counts: Record<string, number> = {};
        for (const node of scene.nodes) counts[node.node.kind] = (counts[node.node.kind] ?? 0) + 1;
        return { width: scene.width, height: scene.height, counts, identityTransforms: scene.nodes.every((node) => node.transform.join(",") === "1,0,0,1,0,0"), inBounds: scene.nodes.every((node) => node.node.kind !== "rect" || (node.node.x >= 0 && node.node.x + node.node.width <= scene.width)) };
      },
      /** 📐️ The specification vector: four `[0, 30]` ticks each drawing a tick line, a grid line and
       * a label, four bars, four diamond symbols and four value labels — 8 lines, 8 texts, 4
       * rectangles and 4 paths. A band scale has no ticks, so the x axis contributes none. */
      oracle: () => ({ width: 160, height: 100, counts: { line: 8, text: 8, rect: 4, path: 4 }, identityTransforms: true, inBounds: true }),
    },
    {
      module: "render",
      name: "bar-rectangles-follow-the-scales",
      subject: () => {
        const scene = render.renderVizScene(DEMO_SPEC);
        return scene.nodes.filter((node) => node.node.kind === "rect").map((node) => [(node.node as { x: number }).x, (node.node as { y: number }).y, (node.node as { width: number }).width, (node.node as { height: number }).height]);
      },
      oracle: () => {
        const x = scale.scaleBand(["a", "b", "c", "d"], [16, 152], { padding: 0.2 });
        const y = scale.scaleLinear([0, 30], [86, 8]);
        return [12, 30, 7, 22].map((value, i) => [x(["a", "b", "c", "d"][i]!), y(value), x.bandwidth!(), 86 - y(value)]);
      },
      tolerance: 1e-9,
    },
    {
      module: "render",
      name: "scene-colours-are-unit-floats",
      subject: () => {
        const scene = render.renderVizScene(DEMO_SPEC);
        const fills = scene.nodes.map((node) => node.fill).filter((fill): fill is { kind: "solid"; color: readonly [number, number, number, number] } => fill?.kind === "solid");
        return { count: fills.length > 0, inRange: fills.every((fill) => fill.color.every((channel) => channel >= 0 && channel <= 1)) };
      },
      oracle: () => ({ count: true, inRange: true }),
    },
    {
      module: "render",
      name: "tikz-emitter-agrees-with-the-plan",
      subject: () => {
        const tikz = render.renderVizTikz(DEMO_SPEC);
        const plan = render.planVizChart(DEMO_SPEC);
        const body = tikz.split("\n").filter((line) => line.startsWith("\\path") || line.startsWith("\\node"));
        return { opens: tikz.startsWith("\\begin{tikzpicture}[x=1mm,y=-1mm]"), closes: tikz.trimEnd().endsWith("\\end{tikzpicture}"), statements: body.length, items: plan.items.length, unitless: body.every((line) => !/\(\s*-?[\d.]+mm/.test(line)) };
      },
      oracle: () => {
        const plan = render.planVizChart(DEMO_SPEC);
        return { opens: true, closes: true, statements: plan.items.length, items: plan.items.length, unitless: true };
      },
    },
    {
      module: "render",
      name: "options-change-the-projection",
      subject: () => {
        const wide = render.planVizChart({ ...DEMO_SPEC, layers: [{ ...DEMO_SPEC.layers[0]!, options: { width: 12 } }] });
        const narrow = render.planVizChart({ ...DEMO_SPEC, scales: [{ name: "x", kind: "band", domain: ["a", "b", "c", "d"], range: [16, 152], options: { padding: 0.6 } }, DEMO_SPEC.scales![1]!], layers: [{ ...DEMO_SPEC.layers[0]!, options: { width: 12 } }] });
        const widths = (plan: render.VizRenderPlan) => plan.items.filter((item) => item.kind === "rect").map((item) => (item as { width: number }).width);
        return widths(wide).every((width, i) => width !== widths(narrow)[i]);
      },
      oracle: () => true,
    },
  ];
}

/** 🧪️ Every check the kernel offers at a level, one group per module directory. */
export async function vizKernelChecks(level: Level): Promise<VizKernelCheck[]> {
  return [...(await scaleChecks(level)), ...(await formatChecks(level)), ...(await transformChecks(level)), ...(await shapeChecks(level)), ...coordinateChecks(), ...(await hierarchyChecks(level)), ...(await networkChecks(level)), ...(await flowChecks(level)), ...(await geoChecks(level)), ...(await spatialChecks(level)), ...themeChecks(), ...renderChecks()];
}
//#endregion 🔖️Checks
//#endregion 🔖️Checks
