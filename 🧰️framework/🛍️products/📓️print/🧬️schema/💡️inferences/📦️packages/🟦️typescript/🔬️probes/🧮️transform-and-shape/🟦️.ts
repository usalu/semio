/** 🧮️ Differential checks of the transform and shape kernels against the d3 packages registered as their oracles.
 *
 * @see ../🧰️probe-kit/🟦️.ts — the check shape and the sampling densities
 * @see ../🟦️.ts — the differential check table that assembles the topics
 */
import * as transform from "../../../../🧮transform/🟦️.ts";
import * as mark from "../../../../✒️mark/🟦️.ts";
import { curveLinear, curveLinearClosed, curveStep, curveStepAfter, curveStepBefore } from "../../../../✒️mark/🪜️curves-linear-and-step/🟦️.ts";
import { curveBasis, curveBasisClosed, curveBasisOpen, curveBundle, curveBundleBeta } from "../../../../✒️mark/🌀️curves-basis-and-bundle/🟦️.ts";
import { curveCardinal, curveCardinalClosed, curveCardinalOpen, curveCardinalTension, curveCatmullRom, curveCatmullRomAlpha, curveCatmullRomClosed, curveCatmullRomOpen } from "../../../../✒️mark/🎢️curves-cardinal-and-catmull-rom/🟦️.ts";
import { curveBumpX, curveBumpY, curveMonotoneX, curveMonotoneY, curveNatural } from "../../../../✒️mark/🏔️curves-monotone-natural-and-bump/🟦️.ts";
import { vizCurve, vizSymbol } from "../../../../✒️mark/💠️curve-registry-and-symbols/🟦️.ts";
import * as shape from "../../../../🥧shape/🟦️.ts";
import { type Level, type VizKernelCheck, samples } from "../🧰️probe-kit/🟦️.ts";

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
export async function transformChecks(level: Level): Promise<VizKernelCheck[]> {
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
export async function shapeChecks(level: Level): Promise<VizKernelCheck[]> {
  const d3Shape = await import("d3-shape");
  const d3Chord = await import("d3-chord");
  const points = curvePoints(level);
  const curves: readonly (readonly [string, unknown, ReturnType<typeof vizCurve>])[] = [
    ["linear", d3Shape.curveLinear, curveLinear],
    ["linear-closed", d3Shape.curveLinearClosed, curveLinearClosed],
    ["step", d3Shape.curveStep, curveStep],
    ["step-before", d3Shape.curveStepBefore, curveStepBefore],
    ["step-after", d3Shape.curveStepAfter, curveStepAfter],
    ["basis", d3Shape.curveBasis, curveBasis],
    ["basis-closed", d3Shape.curveBasisClosed, curveBasisClosed],
    ["basis-open", d3Shape.curveBasisOpen, curveBasisOpen],
    ["bundle", d3Shape.curveBundle, curveBundle],
    ["bundle-beta-0.4", d3Shape.curveBundle.beta(0.4), curveBundleBeta(0.4)],
    ["cardinal", d3Shape.curveCardinal, curveCardinal],
    ["cardinal-tension-0.5", d3Shape.curveCardinal.tension(0.5), curveCardinalTension(0.5)],
    ["cardinal-closed", d3Shape.curveCardinalClosed, curveCardinalClosed],
    ["cardinal-open", d3Shape.curveCardinalOpen, curveCardinalOpen],
    ["catmull-rom", d3Shape.curveCatmullRom, curveCatmullRom],
    ["catmull-rom-alpha-1", d3Shape.curveCatmullRom.alpha(1), curveCatmullRomAlpha(1)],
    ["catmull-rom-closed", d3Shape.curveCatmullRomClosed, curveCatmullRomClosed],
    ["catmull-rom-open", d3Shape.curveCatmullRomOpen, curveCatmullRomOpen],
    ["monotone-x", d3Shape.curveMonotoneX, curveMonotoneX],
    ["monotone-y", d3Shape.curveMonotoneY, curveMonotoneY],
    ["natural", d3Shape.curveNatural, curveNatural],
    ["bump-x", d3Shape.curveBumpX, curveBumpX],
    ["bump-y", d3Shape.curveBumpY, curveBumpY],
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
      subject: () => vizSymbol(ownKind as never, 137),
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
          .context(recorder as never)(options);
  
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
            .centroid(options),
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
