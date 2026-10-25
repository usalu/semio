/** ⚖️ Differential checks of the scale and format kernels against the d3 packages registered as their oracles.
 *
 * @see ../🧰️probe-kit/🟦️.ts — the check shape and the sampling densities
 * @see ../🟦️.ts — the differential check table that assembles the topics
 */
import * as scale from "../../📐scale/🟦️.ts";
import * as format from "../../🔢format/🟦️.ts";
import { type Level, type VizKernelCheck, samples } from "../🧰️probe-kit/🟦️.ts";

/** 📐️ Differential checks for `📐scale` against `d3-scale`, `d3-array` and `d3-time`. */
export async function scaleChecks(level: Level): Promise<VizKernelCheck[]> {
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
        const t = d3Scale.scaleThreshold<number, string>([10, 20], ["a", "b", "c"]);
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
export async function formatChecks(level: Level): Promise<VizKernelCheck[]> {
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
