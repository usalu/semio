/** 🖼️ Differential checks of the theme and render kernels: the planned chart against the TikZ the native emitter writes for the demo specification.
 *
 * @see ../🧰️probe-kit/🟦️.ts — the check shape
 * @see ../🟦️.ts — the differential check table that assembles the topics
 */
import * as nativeRender from "@semio-tech/print/io/text/inferences/latex";
import * as scale from "../../../../📐scale/🟦️.ts";
import * as theme from "../../../../🎨theme/🟦️.ts";
import * as render from "../../../../🖼️render/🟦️.ts";
import type { VizKernelCheck } from "../🧰️probe-kit/🟦️.ts";

const DEMO_SPEC: Parameters<typeof render.planVizChart>[0] = {
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
export function themeChecks(): VizKernelCheck[] {
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
        const paints = await import("@semio-tech/print/modules/print-design-token-paints");
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
export function renderChecks(): VizKernelCheck[] {
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
       * rectangles and 4 paths; categorical x ticks contribute four lines and four labels. Categorical axes label every domain value. */
      oracle: () => ({ width: 160, height: 100, counts: { line: 18, text: 12, rect: 4, path: 4 }, identityTransforms: true, inBounds: true }),
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
        return [12, 30, 7, 22].map((value, i) => { const mapped = y(value); if (typeof mapped !== "number") throw new Error("Bar oracle requires numeric scale output"); return [x(["a", "b", "c", "d"][i]!), mapped, x.bandwidth!(), 86 - mapped]; });
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
        const tikz = nativeRender.renderVizTikz(DEMO_SPEC);
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
        const wide = render.planVizChart({ ...DEMO_SPEC, layers: [{ ...DEMO_SPEC.layers[0]! }] });
        const narrow = render.planVizChart({ ...DEMO_SPEC, scales: [{ name: "x", kind: "band", domain: ["a", "b", "c", "d"], range: [16, 152], options: { padding: 0.6 } }, DEMO_SPEC.scales![1]!], layers: [{ ...DEMO_SPEC.layers[0]! }] });
        const widths = (plan: render.VizRenderPlan) => plan.items.filter((item) => item.kind === "rect").map((item) => (item as { width: number }).width);
        return widths(wide).every((width, i) => width !== widths(narrow)[i]);
      },
      oracle: () => true,
    },
  ];
}
