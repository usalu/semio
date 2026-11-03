/** 🌡️ Thermal span range: a symmetric ±spanK range around the air temperature makes moderate sources readable, the legend states it, other quantities keep the shared range, and the cast manifest carries it as `thermalSpanK`.
 * Oracle: the fixture vectors and d3-scale (piecewise linear) for the signed fraction; ajv for the cast schema.
 *
 * @see ./🥒️.feature — the scenarios
 * @see ../../🧫️fixtures/🌡️thermal-span/🔣️.json — the vectors
 */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import Ajv from "ajv";
import { scaleLinear } from "d3-scale";
import { describe, expect, it } from "vitest";
import environments from "../../🗂️catalog/🌤️environments/🔣️.json" with { type: "json" };
import type { Environment } from "../../🧬️schema/🟦️.ts";
import type { DepictionOptions, DisplayRange, DrawingItem } from "../../🔨️modules/🔗️contracts/🟦️.ts";
import { defaultDepictionOptions, depict } from "../../🔨️modules/🎨️depiction/🟦️.ts";
import { divergingColour, scaleOf } from "../../🔨️modules/🎨️depiction/🖍️palette/🟦️.ts";
import { evaluate } from "../../🔨️modules/🧮️evaluation/🟦️.ts";
import { castManifestIssues } from "../../🔨️modules/📤️export/🎭️cast/🧬️schema/🟦️.ts";
import { solveThermal } from "../../🔨️modules/🌡️thermal/🟦️.ts";
import { catalogAsset } from "../../🧫️fixtures/🌡️thermal/🟦️.ts";
import { UNCHECKED_PROFILE } from "../../🧫️fixtures/🧱️synthetic-figure/🟦️.ts";
import { STAFFAGE_ROOT, fixture } from "../🧰️support/🧰️.ts";

type Vectors = {
  sharedC: { min: number; max: number };
  scales: { name: string; airC: number; spanK: number; min: number; max: number }[];
  fractions: { airC: number; spanK: number; valueC: number; span: number; shared: number }[];
  invalid: { name: string; spanK: unknown }[];
  sceneRule: {
    scenes: { name: string; casts: (number | null)[]; mixed: boolean }[];
    clips: { name: string; scenes: Record<string, (number | null)[]>; mixedScenes: string[] }[];
  };
};

const mixesSpans = (spans: readonly (number | null | undefined)[]): boolean => new Set(spans.map((span) => span ?? null)).size > 1;
const mixedScenes = (scenes: Record<string, (number | null)[]>): string[] => Object.entries(scenes).filter(([, spans]) => mixesSpans(spans)).map(([name]) => name);

const vectors = fixture<Vectors>("🌡️thermal-span");
const laptop = catalogAsset("laptop");
const room = environments.presets.find((preset) => preset.id === "indoor-heated-winter")!.environment as Environment;
const schema = JSON.parse(readFileSync(join(STAFFAGE_ROOT, "🔨️modules/📤️export/🎭️cast/🧬️schema/🔣️.schema.json"), "utf8"));
const validate = new Ajv({ strict: true, allErrors: true }).compile(schema);

const fractionOf = (scale: { min: number; centre: number; max: number }, value: number): number => scaleLinear().domain([scale.min, scale.centre, scale.max]).range([-1, 0, 1]).clamp(true)(value);
const drawn = (quantity: DepictionOptions["thermalQuantity"], range: DisplayRange) => {
  const state = evaluate(laptop, { kind: "pose", pose: "open" }, { kind: "scale", scale: { denominator: 50 } });
  const field = solveThermal(state, room);
  const options: DepictionOptions = { ...defaultDepictionOptions("thermal", "en", 50), profile: UNCHECKED_PROFILE, thermalQuantity: quantity, range, environment: room };
  return depict(state, options, { thermal: field });
};
const items = (scene: ReturnType<typeof drawn>): DrawingItem[] => scene.layers.flatMap((layer) => layer.items);
const paths = (scene: ReturnType<typeof drawn>) => items(scene).filter((item): item is Extract<DrawingItem, { kind: "path" }> => item.kind === "path" && item.id.startsWith("shape:") && item.value !== undefined && item.fill !== undefined);
const chroma = (hex: string): number => {
  const [r, g, b] = [1, 3, 5].map((at) => Number.parseInt(hex.slice(at, at + 2), 16));
  return Math.max(r!, g!, b!) - Math.min(r!, g!, b!);
};

describe("🌡️ thermal span range", () => {
  it.each(vectors.scales)("spans the air temperature ± spanK: $name", ({ airC, spanK, min, max }) => {
    const scale = scaleOf([], airC, { mode: "span", spanK }, 1, vectors.sharedC);
    expect(scale).toEqual({ min, centre: airC, max });
  });

  it.each(vectors.fractions)("reads $valueC °C at air $airC °C with ±$spanK K as the vector says and as d3 does", ({ airC, spanK, valueC, span, shared }) => {
    const spanScale = scaleOf([], airC, { mode: "span", spanK }, 1, vectors.sharedC);
    const sharedScale = scaleOf([], airC, { mode: "shared" }, 1, vectors.sharedC);
    expect(fractionOf(spanScale, valueC)).toBeCloseTo(span, 6);
    expect(fractionOf(sharedScale, valueC)).toBeCloseTo(shared, 6);
    const same = airC + span * (span >= 0 ? sharedScale.max - airC : airC - sharedScale.min);
    expect(divergingColour(valueC, spanScale, true)).toBe(divergingColour(same, sharedScale, true));
  });

  it("makes a moderate source more saturated than the shared range does", () => {
    const shared = paths(drawn("surface-temperature", { mode: "shared" }));
    const span = paths(drawn("surface-temperature", { mode: "span", spanK: 25 }));
    const hottest = (list: typeof span) => list.reduce((best, item) => (item.value!.value > best.value!.value ? item : best));
    const warm = hottest(shared);
    const same = hottest(span);
    expect(warm.value!.value).toBeGreaterThan(room.airTempC + 8);
    expect(same.value!.value).toBeCloseTo(warm.value!.value, 9);
    expect(chroma(same.fill!.color)).toBeGreaterThan(chroma(warm.fill!.color));
  });

  it("states the span in the legend, in both languages, centred on the air temperature", () => {
    const legend = drawn("surface-temperature", { mode: "span", spanK: 25 }).legends[0]!;
    expect(legend.range).toBe("span");
    expect(legend.min).toBe(room.airTempC - 25);
    expect(legend.max).toBe(room.airTempC + 25);
    expect(legend.convention.en).toContain("Span ±25 K around the air temperature");
    expect(legend.convention.de).toContain("Spanne ±25 K um die Lufttemperatur");
    expect(legend.convention.en).not.toContain("Shared comparison range");
  });

  it("keeps the shared range for every quantity other than the surface temperature", () => {
    for (const quantity of ["skin-temperature", "emitted-radiant-flux", "net-radiative-flux", "sensible-heat-flux"] as const) {
      const span = drawn(quantity, { mode: "span", spanK: 25 }).legends[0]!;
      const shared = drawn(quantity, { mode: "shared" }).legends[0]!;
      expect(span.range, quantity).toBe("shared");
      expect([span.min, span.max], quantity).toEqual([shared.min, shared.max]);
    }
  });
});

describe("🌡️ thermalSpanK in the cast manifest", () => {
  const cast = (extra: Record<string, unknown>) => ({ version: 1, casts: { span: { asset: "laptop", pose: "open", view: "thermal", environment: "indoor-heated-winter", theme: "dark", viewpoint: "elevation", ...extra } } });

  it("accepts a positive span in ajv and in the twin", () => {
    const manifest = cast({ thermalSpanK: 25 });
    expect(validate(manifest), JSON.stringify(validate.errors?.slice(0, 3))).toBe(true);
    expect(castManifestIssues(manifest)).toEqual([]);
  });

  it.each(vectors.invalid)("rejects $name in ajv and in the twin", ({ spanK }) => {
    const manifest = cast({ thermalSpanK: spanK });
    expect(validate(manifest)).toBe(false);
    expect(castManifestIssues(manifest).join("\n")).toContain("thermalSpanK: must be a positive number of kelvin");
  });

  it("rejects a span outside the thermal view in ajv and in the twin", () => {
    const manifest = cast({ view: "dark-line", environment: undefined, thermalSpanK: 25 });
    expect(validate(JSON.parse(JSON.stringify(manifest)))).toBe(false);
    expect(castManifestIssues(JSON.parse(JSON.stringify(manifest))).join("\n")).toContain("thermalSpanK exists for the thermal view only");
  });

  it("gives every thermal cast of the course manifest a deliberate span; one scene shares one (guarded by the visuals staffage_only test)", () => {
    const course = JSON.parse(readFileSync(join(STAFFAGE_ROOT, "../⚡️energy/🏗️authoring/🎨️visuals/🧍️staffage/🔣️.json"), "utf8")) as { casts: Record<string, { view: string; thermalSpanK?: number }> };
    const spans = Object.values(course.casts).filter((entry) => entry.view === "thermal").map((entry) => entry.thermalSpanK);
    expect(spans.length).toBeGreaterThan(0);
    for (const span of spans) expect(span).toBeGreaterThan(0);
  });

  it.each(vectors.sceneRule.scenes)("the per-scene rule says $name mixes spans: $mixed", ({ casts, mixed }) => {
    expect(mixesSpans(casts)).toBe(mixed);
  });

  it.each(vectors.sceneRule.clips)("lets scenes differ and fails a scene that mixes: $name", ({ scenes, mixedScenes: expected }) => {
    expect(mixedScenes(scenes)).toEqual(expected);
  });

  it("holds the course manifest to a set of spans that one scene can choose from, each cast on a positive span", () => {
    const course = JSON.parse(readFileSync(join(STAFFAGE_ROOT, "../⚡️energy/🏗️authoring/🎨️visuals/🧍️staffage/🔣️.json"), "utf8")) as { casts: Record<string, { view: string; thermalSpanK?: number }> };
    const thermal = Object.entries(course.casts).filter(([, entry]) => entry.view === "thermal");
    const missing = thermal.filter(([, entry]) => entry.thermalSpanK === undefined).map(([name]) => name);
    expect(missing).toEqual([]);
    expect(new Set(thermal.map(([, entry]) => entry.thermalSpanK)).size).toBeGreaterThan(1);
  });
});
