/** 🌊️ Flow view: every layer is drawn on its own in all four viewpoints and nothing else is (no text, no emitter marker), legends carry units, the model note and the explanatory declaration in both languages, tracers keep their hand-derived schedule, speed and fade, parcel paths are the projected parcel histories with hand-derived trails (d3-scale as oracle), widths and colours follow the d3 scales, and coarser scales draw the same field with fewer marks that keep their paper spacing (d3-quadtree). */
import { quadtree } from "d3-quadtree";
import { interpolateRgb } from "d3-interpolate";
import { scaleLinear, scaleSqrt } from "d3-scale";
import { describe, expect, it } from "vitest";
import fixture from "../../🧫️fixtures/🌊️flow-view/🔣️.json" with { type: "json" };
import { SCENARIOS, scenario } from "../../🧫️fixtures/💨️transport/🟦️.ts";
import type { Viewpoint } from "../../🧬️schema/🟦️.ts";
import { FLOW_LAYERS, type DisplayRange, type FlowField, type FlowLayer, type FlowMarkDetail, type FlowPath, type FlowStream } from "../../🔨️modules/🔗️contracts/🟦️.ts";
import { solveTransport } from "../../🔨️modules/💨️transport/🟦️.ts";
import { PUFF_MIN_SPACING, PUFF_OWN_RADII, TRACER_EXPOSURE_S, TRACER_PERIOD_S, flowDetailOf, flowMarks, flowTrails, trailOf } from "../../🔨️modules/💨️transport/🌊️flow/🟦️.ts";
import { Canvas, Labels, type Ctx } from "../../🔨️modules/🎨️depiction/🧱️canvas/🟦️.ts";
import { THEMES, mix } from "../../🔨️modules/🎨️depiction/🖍️palette/🟦️.ts";
import { axonometricCamera, lookingOf } from "../../🔨️modules/🎨️depiction/🔭️views/🟦️.ts";
import { FLOW_STYLE, NEUTRAL_BAND, SHARED_FLOW_EXCESS_K, flowPaint, flowView } from "../../🔨️modules/🎨️depiction/🌊️flow-view/🟦️.ts";

const s = scenario(fixture.scene.scenario as "human");
const environment = SCENARIOS.environments[fixture.scene.environment as "calm-cold"];
const transport = solveTransport(s.asset, { kind: "pose", pose: fixture.scene.pose, timeS: fixture.scene.timeS }, environment, { ...s.options, maxAgeS: fixture.scene.timeS, flow: { domain: fixture.scene.domain } });
const flow = transport.flow!;

function draw(language: "en" | "de", layers: readonly FlowLayer[], viewpoint: Viewpoint = "elevation", denominator = 50, range: DisplayRange = { mode: "shared" }, theme: "light" | "dark" = "light", flowDetail: FlowMarkDetail = {}, field: FlowField = flow) {
  const k = 1000 / denominator;
  const ctx = { state: { asset: s.asset }, options: { viewpoint, scale: { denominator }, patterns: true, annotations: { labels: true }, flowDetail }, language, k, theme, colours: THEMES[theme], paper: (p: { x: number; y: number }) => ({ x: p.x * k, y: -p.y * k }), labels: new Labels(), ...(viewpoint === "axonometric" ? { camera: axonometricCamera() } : {}) } as unknown as Ctx;
  const canvas = new Canvas();
  const view = flowView(ctx, field, layers, range);
  view.overlay(canvas);
  return { items: canvas.finish(1).layers.flatMap((l) => l.items), legends: view.legends };
}
const count = (items: readonly { id: string }[], prefix: string): number => items.filter((i) => i.id.startsWith(prefix)).length;
const uniform = (lengthM: number, speedMS: number, points: number): FlowField => {
  const at = Array.from({ length: points }, (_, i) => ({ x: 0, y: 0.5 + (lengthM * i) / (points - 1), z: 0 }));
  const stream: FlowStream = { seed: at[0]!, role: "source", emitter: null, points: at, travelS: at.map((_, i) => (lengthM * i) / (points - 1) / speedMS), speedMS: at.map(() => speedMS), excessK: at.map(() => 2), vapourExcessKgPerKg: at.map(() => 0), condensateKgPerKg: at.map(() => 0) };
  return { ...flow, streams: [stream], puffs: [], paths: [] };
};

describe("🌊️ flow view", () => {
  it("is a developed field with streams, vapour and fog in cold air", () => {
    expect(flow.streams.length).toBeGreaterThan(5);
    expect(flow.puffs.some((p) => p.vapourExcessKgPerKg * 1000 > FLOW_STYLE.vapourThresholdGKg)).toBe(true);
    expect(flow.puffs.some((p) => p.condensateKgPerKg > 0)).toBe(true);
  });

  for (const viewpoint of ["elevation", "front", "plan", "axonometric"] as const)
    it(`draws every layer in the ${viewpoint} and each layer on its own`, () => {
      const all = draw("en", FLOW_LAYERS, viewpoint).items;
      for (const prefix of ["flow-stream:", "flow-arrow:", "flow-tracer:", "flow-tracer-head:", "flow-vapour:", "flow-fog:", "flow-fog-rim:", "flow-path:"]) expect(count(all, prefix), `${viewpoint} ${prefix}`).toBeGreaterThan(0);
      for (const item of all) if (item.kind === "path") for (const c of item.commands) if (c.op !== "Z") expect(Number.isFinite(c.x) && Number.isFinite(c.y)).toBe(true);
      const streams = draw("en", ["streams"], viewpoint).items;
      expect(count(streams, "flow-stream:")).toBe(count(all, "flow-stream:"));
      expect(count(streams, "flow-tracer") + count(streams, "flow-vapour") + count(streams, "flow-fog") + count(streams, "flow-path")).toBe(0);
      const tracers = draw("en", ["tracers"], viewpoint).items;
      expect(tracers.every((i) => i.id.startsWith("flow-tracer"))).toBe(true);
      expect(draw("en", ["moisture"], viewpoint).items.every((i) => i.id.startsWith("flow-vapour"))).toBe(true);
      expect(draw("en", ["fog"], viewpoint).items.every((i) => i.id.startsWith("flow-fog"))).toBe(true);
      expect(draw("en", ["pathlines"], viewpoint).items.every((i) => i.id.startsWith("flow-path:"))).toBe(true);
      expect(draw("en", [], viewpoint).items).toEqual([]);
      expect(all.every((i) => i.kind === "path" && i.id.startsWith("flow-"))).toBe(true);
      expect(all.some((i) => i.id.includes("emitter") || i.id.startsWith("label:"))).toBe(false);
    });

  it("explains every mark with a unit in English and German, whatever the viewpoint", () => {
    const en = draw("en", FLOW_LAYERS).legends;
    const de = draw("de", FLOW_LAYERS).legends;
    expect(en.map((l) => ({ quantity: l.quantity, unit: l.unit }))).toEqual(fixture.legends);
    expect(JSON.stringify(draw("en", FLOW_LAYERS, "plan").legends)).toBe(JSON.stringify(en));
    for (const [i, legend] of en.entries()) {
      for (const text of [legend.title, legend.convention, legend.interpretation, legend.conditions!]) {
        expect(text.en.length).toBeGreaterThan(10);
        expect(text.de.length).toBeGreaterThan(10);
        expect(text.de).not.toBe(text.en);
      }
      expect(de[i]!.title.de).toBe(legend.title.de);
      expect(legend.max).toBeGreaterThan(legend.min);
      expect(legend.conditions!.en).toContain("dew point");
      expect(legend.conditions!.de).toContain("Taupunkt");
    }
    expect(en[0]!.min).toBe(-SHARED_FLOW_EXCESS_K);
    expect(en[0]!.max).toBe(SHARED_FLOW_EXCESS_K);
    expect(draw("en", ["streams"]).legends.map((l) => l.quantity)).toEqual(["flow-excess-temperature", "flow-speed"]);
    expect(draw("en", ["fog"]).legends.map((l) => l.quantity)).toEqual(["flow-condensate"]);
    expect(draw("en", ["pathlines"]).legends.map((l) => l.quantity)).toEqual(["flow-excess-temperature", "flow-path-age"]);
  });

  it("states the model, declares invisible air an explanatory visualization and keeps visible water to the fog, in both languages with the decimal comma in German", () => {
    const legends = draw("de", FLOW_LAYERS).legends;
    const of = (quantity: string) => legends.find((l) => l.quantity === quantity)!;
    for (const legend of legends) {
      expect(legend.conditions!.en).toMatch(/not CFD/u);
      expect(legend.conditions!.de).toMatch(/keine CFD/u);
      expect(legend.conditions!.de).toMatch(/\d,\d/u);
      expect(legend.conditions!.de).not.toMatch(/\d\.\d/u);
      expect(legend.stops.length).toBeGreaterThanOrEqual(2);
    }
    for (const quantity of ["flow-excess-temperature", "flow-vapour-excess"]) {
      expect(of(quantity).assumptions!.some((a) => /Explanatory visualization/u.test(a.en) && /invisible/u.test(a.en) && /Only the fog marks/u.test(a.en))).toBe(true);
      expect(of(quantity).assumptions!.some((a) => /Erklärende Visualisierung/u.test(a.de) && /unsichtbar/u.test(a.de))).toBe(true);
    }
    expect(of("flow-condensate").convention.en).toMatch(/droplets/u);
    expect(of("flow-condensate").interpretation.en).toMatch(/saturation adjustment/u);
    expect(of("flow-path-age").title).toEqual({ en: "Parcel paths", de: "Teilchenbahnen" });
    expect(of("flow-path-age").interpretation.en).toMatch(/differ from the stream lines of the instant/u);
    expect(of("flow-path-age").interpretation.de).toMatch(/unterscheiden sich die Bahnen von den Stromlinien/u);
    expect(of("flow-speed").convention.en).toMatch(/velocity of this instant/u);
  });

  it("uses the shared range for the temperature excess and a normalised auto range on request", () => {
    const shared = draw("en", ["pathlines"]).legends[0]!;
    expect([shared.min, shared.max, shared.range]).toEqual([-SHARED_FLOW_EXCESS_K, SHARED_FLOW_EXCESS_K, "shared"]);
    expect(shared.interpretation.en).not.toMatch(/normalised/u);
    const auto = draw("en", ["streams"], "elevation", 50, { mode: "auto" }).legends[0]!;
    expect(auto.range).toBe("auto");
    expect(auto.max).toBeLessThanOrEqual(SHARED_FLOW_EXCESS_K);
    expect(auto.interpretation.en).toMatch(/normalised/u);
    expect(auto.interpretation.de).toMatch(/normiert/u);
  });

  it("gives moist air a second, colour-independent reading: dotted vapour lines and hollow tracer heads", () => {
    const items = draw("en", ["streams", "tracers", "moisture"]).items;
    const lines = items.filter((i) => i.id.startsWith("flow-vapour-line:"));
    expect(lines.length).toBeGreaterThan(0);
    for (const line of lines) expect(line.kind === "path" ? line.stroke?.dashMm : undefined).toEqual([...FLOW_STYLE.vapourDashMm]);
    expect(items.filter((i) => i.id.startsWith("flow-stream:")).every((i) => i.kind === "path" && i.stroke?.dashMm === undefined)).toBe(true);
    const heads = items.filter((i) => i.id.startsWith("flow-tracer-head:"));
    expect(heads.some((i) => i.kind === "path" && i.fill === undefined && i.stroke !== undefined)).toBe(true);
  });

  for (const viewpoint of ["elevation", "front", "plan"] as const)
    it(`draws the path every parcel travelled as its history projected into the ${viewpoint}, in the colour of what the parcel carries now`, () => {
      const k = 20;
      const paint = flowPaint("light", flow, { mode: "shared" });
      const flat = (p: { x: number; y: number; z: number }) => (viewpoint === "elevation" ? { x: p.x, y: p.y } : viewpoint === "front" ? { x: p.z, y: p.y } : { x: p.x, y: p.z });
      const items = draw("en", ["pathlines"], viewpoint).items;
      const trails = flowMarks(flow, flow.timeS, flowDetailOf(50)).trails;
      expect(trails.length).toBeGreaterThan(3);
      expect(items.length).toBe(trails.length);
      expect(flow.paths.length).toBe(transport.pathlines.filter((line) => line.points.length > 1 && flow.puffs.some((puff) => puff.at === transport.parcels.find((parcel) => parcel.id === line.parcel)!.at)).length);
      const first = trails[0]!;
      const anchor = items.find((i) => i.id === `flow-path:${first.parcel}:0`)!;
      const origin = anchor.kind === "path" && anchor.commands[0]!.op !== "Z" ? { x: anchor.commands[0]!.x / k - flat(first.points[0]!).x, y: -anchor.commands[0]!.y / k - flat(first.points[0]!).y } : { x: Number.NaN, y: Number.NaN };
      for (const trail of trails) {
        const history = transport.pathlines.find((line) => line.parcel === trail.parcel)!;
        const parcel = transport.parcels.find((candidate) => candidate.id === trail.parcel)!;
        expect(trail.points).toBe(history.points);
        trail.agoS.forEach((ago, i) => expect(parcel.ageS - ago).toBeCloseTo(history.ages[i]!, 12));
        expect(trail.fade.every((fade) => fade === 1)).toBe(true);
        const item = items.find((i) => i.id === `flow-path:${trail.parcel}:0`)!;
        if (item.kind !== "path") throw new Error("a path is drawn as a path");
        expect(item.commands.length).toBe(history.points.length);
        item.commands.forEach((c, i) => {
          if (c.op === "Z") throw new Error("a path is open");
          expect(c.x / k - origin.x).toBeCloseTo(flat(history.points[i]!).x, 9);
          expect(-c.y / k - origin.y).toBeCloseTo(flat(history.points[i]!).y, 9);
        });
        expect(item.fill).toBeUndefined();
        expect(item.stroke).toMatchObject({ color: paint.excessColour(parcel.excessK), widthMm: fixture.pathlines.widthMm.default, opacity: fixture.pathlines.opacity });
        expect(item.stroke!.dashMm).toBeUndefined();
        expect(item.value).toMatchObject({ quantity: "flow-path-age", unit: "s", value: parcel.ageS });
      }
      const red = (hex: string) => Number.parseInt(hex.slice(1, 3), 16) - Number.parseInt(hex.slice(5, 7), 16);
      const byExcess = [...trails].sort((a, b) => b.excessK - a.excessK);
      expect(red(paint.excessColour(byExcess[0]!.excessK))).toBeGreaterThan(red(paint.excessColour(byExcess[byExcess.length - 1]!.excessK)));
    });

  it("keeps the recent trail of a path with the hand-derived tail, ages, fade and opacity steps", () => {
    const vector = fixture.pathlines;
    const path: FlowPath = { parcel: "nose@0#0", emitter: "nose", points: vector.path.points.map(([x, y, z]) => ({ x: x!, y: y!, z: z! })), ageS: vector.path.ageS, excessK: vector.path.excessK, vapourExcessKgPerKg: 0, condensateKgPerKg: 0 };
    const now = path.ageS[path.ageS.length - 1]!;
    const where = (axis: "x" | "y") => scaleLinear().domain(path.ageS).range(path.points.map((p) => p[axis]));
    const fade = scaleLinear().domain([0, vector.trail.trailS]).range([1, 0]).clamp(true);
    const trail = trailOf(path, 7, vector.trail.trailS);
    expect(trail).toMatchObject({ path: 7, parcel: path.parcel, emitter: path.emitter, excessK: path.excessK });
    expect(trail.points.length).toBe(vector.trail.points);
    expect([trail.points[0]!.x, trail.points[0]!.y, trail.points[0]!.z].map((v, i) => Math.abs(v - vector.trail.tail[i]!) < 1e-12)).toEqual([true, true, true]);
    expect(trail.points[0]!.x).toBeCloseTo(where("x")(now - vector.trail.trailS), 12);
    expect(trail.points[0]!.y).toBeCloseTo(where("y")(now - vector.trail.trailS), 12);
    expect(trail.points.slice(1)).toEqual(path.points.slice(-vector.trail.points + 1));
    trail.agoS.forEach((ago, i) => {
      expect(ago).toBeCloseTo(vector.trail.agoS[i]!, 12);
      expect(trail.fade[i]).toBeCloseTo(vector.trail.fade[i]!, 11);
      expect(trail.fade[i]).toBeCloseTo(fade(ago), 12);
      expect(trail.points[i]!.x).toBeCloseTo(where("x")(now - ago), 12);
    });
    const whole = trailOf(path, 0);
    expect(whole.points).toBe(path.points);
    expect(whole.agoS[0]).toBeCloseTo(vector.whole.agoS0, 12);
    expect(new Set(whole.fade)).toEqual(new Set([1]));
    const long = trailOf(path, 0, vector.long.trailS);
    expect(long.points.length).toBe(vector.long.points);
    expect(long.fade[0]).toBeCloseTo(vector.long.firstFade, 12);
    expect(long.fade[long.fade.length - 1]).toBe(1);
    const field: FlowField = { ...flow, streams: [], puffs: [], paths: [path] };
    const runs = (flowDetail: FlowMarkDetail) => draw("en", ["pathlines"], "elevation", 50, { mode: "shared" }, "light", flowDetail, field).items.map((item) => (item.kind === "path" ? { start: Number(item.id.split(":").pop()), points: item.commands.length, opacity: item.stroke!.opacity, widthMm: item.stroke!.widthMm } : null));
    expect(FLOW_STYLE.trailBands).toBe(vector.bands);
    const faded = runs({ trailS: vector.trail.trailS });
    expect(faded.length).toBe(vector.trail.runs.length);
    faded.forEach((run, i) => {
      expect(run).toMatchObject({ start: vector.trail.runs[i]!.start, points: vector.trail.runs[i]!.points, widthMm: vector.widthMm.default });
      expect(run!.opacity).toBeCloseTo(vector.trail.runs[i]!.opacity, 12);
    });
    expect(runs({})).toEqual(vector.whole.runs.map((run) => ({ start: run.start, points: run.points, opacity: run.opacity, widthMm: vector.widthMm.default })));
    expect(runs({ strokeMm: vector.strokeMm })[0]!.widthMm).toBeCloseTo(vector.widthMm.stroked, 12);
    const legend = draw("de", ["pathlines"], "elevation", 50, { mode: "shared" }, "light", { trailS: vector.trail.trailS }, field).legends.find((l) => l.quantity === "flow-path-age")!;
    expect(legend.max).toBeCloseTo(vector.trail.trailS, 12);
    expect(legend.convention.de).toContain("nur die letzten 0,3 s jeder Bahn");
    expect(legend.convention.en).toContain("only the last 0.3 s of every path");
    expect(legend.stops[0]!.color).not.toBe(legend.stops[1]!.color);
  });

  it("leaves out the part of a path that lies behind a solid body in the viewing direction and draws it whole from where the body is not in the way", () => {
    const vector = fixture.pathlines;
    const path: FlowPath = { parcel: "nose@0#0", emitter: "nose", points: vector.path.points.map(([x, y, z]) => ({ x: x!, y: y!, z: z! })), ageS: vector.path.ageS, excessK: vector.path.excessK, vapourExcessKgPerKg: 0, condensateKgPerKg: 0 };
    const looking = lookingOf("elevation");
    const behind = vector.occlusion.behind;
    const ball = { id: "ball", kind: "solid" as const, centre: { x: vector.occlusion.centre[0]! - looking.x * vector.occlusion.towardEyeM, y: vector.occlusion.centre[1]! - looking.y * vector.occlusion.towardEyeM, z: vector.occlusion.centre[2]! - looking.z * vector.occlusion.towardEyeM }, axes: [{ x: 1, y: 0, z: 0 }, { x: 0, y: 1, z: 0 }, { x: 0, y: 0, z: 1 }] as const, radiiM: [vector.occlusion.radiusM, vector.occlusion.radiusM, vector.occlusion.radiusM] as const, exponent: 2 as const, blockage: 1 };
    const field: FlowField = { ...flow, streams: [], puffs: [], paths: [path], obstacles: [ball] };
    const seen = flowMarks(field, field.timeS, { ...flowDetailOf(50), eye: { toward: { x: -looking.x, y: -looking.y, z: -looking.z } } }).trails[0]!.visible;
    for (const index of behind) expect(seen[index], `point ${index}`).toBe(0);
    for (const index of vector.occlusion.open) expect(seen[index], `point ${index}`).toBe(1);
    expect(flowMarks(field, field.timeS, flowDetailOf(50)).trails[0]!.visible.every((value) => value === 1)).toBe(true);
    const drawn = (viewpoint: Viewpoint) => draw("en", ["pathlines"], viewpoint, 50, { mode: "shared" }, "light", {}, field).items.map((item) => ({ start: Number(item.id.split(":").pop()), points: item.kind === "path" ? item.commands.length : 0 }));
    const side = drawn("elevation");
    expect(side.length).toBe(2);
    expect(side[0]!.start).toBe(0);
    expect(side[0]!.start + side[0]!.points - 1).toBeLessThan(behind[0]!);
    expect(side[1]!.start).toBeGreaterThan(behind[behind.length - 1]!);
    expect(side[1]!.start + side[1]!.points).toBe(path.points.length);
    expect(drawn("plan")).toEqual([{ start: 0, points: path.points.length }]);
  });

  it("widens every line of the flow with the stroke of the widest stream", () => {
    const stroked = flowPaint("light", flow, { mode: "shared" }, fixture.pathlines.strokeMm);
    const plain = flowPaint("light", flow, { mode: "shared" });
    const width = scaleSqrt().domain([0, plain.speedMaxMS]).range([(FLOW_STYLE.streamMinMm * fixture.pathlines.strokeMm) / FLOW_STYLE.streamMaxMm, fixture.pathlines.strokeMm]).clamp(true);
    for (const speed of [0, 0.02, plain.speedMaxMS / 3, plain.speedMaxMS]) expect(Math.abs(stroked.streamWidthMm(speed) - width(speed))).toBeLessThan(fixture.tolerances.widthMm);
    expect(plain.strokeScale).toBe(1);
    const widths = (flowDetail: FlowMarkDetail, prefix: string) => draw("en", FLOW_LAYERS, "elevation", 50, { mode: "shared" }, "light", flowDetail).items.filter((i) => i.id.startsWith(prefix)).map((i) => (i.kind === "path" ? i.stroke!.widthMm : 0));
    for (const prefix of ["flow-stream:", "flow-tracer:", "flow-vapour-line:", "flow-path:"]) {
      const thin = widths({}, prefix);
      const bold = widths({ strokeMm: fixture.pathlines.strokeMm }, prefix);
      expect(thin.length).toBeGreaterThan(0);
      bold.forEach((value, i) => expect(value / thin[i]!).toBeCloseTo(fixture.pathlines.strokeMm / FLOW_STYLE.streamMaxMm, 9));
    }
  });

  it.each(fixture.streams.map((row) => row.id))("releases, moves and fades the tracers of the %s stream as derived by hand", (id) => {
    const row = fixture.streams.find((r) => r.id === id)!;
    const field = uniform(row.lengthM, row.speedMS, row.points);
    const detail = flowDetailOf(row.denominator);
    expect(detail.spacingM * 1000 / row.denominator).toBeCloseTo(fixture.detail.spacingMmOnPaper, 12);
    expect(detail.minStreakM * 1000 / row.denominator).toBeCloseTo(fixture.detail.minStreakMmOnPaper, 12);
    expect(field.model.periodS).toBe(TRACER_PERIOD_S);
    expect(field.model.exposureS).toBe(TRACER_EXPOSURE_S);
    const cycle = row.expected.releaseEveryS * row.expected.tracers;
    const seen = new Set<number>();
    for (const t of [0, 0.37, 1.1, 4.9]) {
      const now = flowMarks(field, t, detail).streaks;
      const later = flowMarks(field, t + 0.05, detail).streaks;
      seen.add(now.length);
      const heads = now.map((streak) => streak.points[streak.points.length - 1]!.y).sort((a, b) => a - b);
      for (let i = 1; i < heads.length; i++) expect((heads[i]! - heads[i - 1]!) / row.expected.headSpacingM).toBeCloseTo(Math.round((heads[i]! - heads[i - 1]!) / row.expected.headSpacingM), 6);
      for (const streak of now) {
        const head = streak.points[streak.points.length - 1]!.y;
        const tail = streak.points[0]!.y;
        const travelled = (head - 0.5) / row.speedMS;
        expect(head).toBeGreaterThan(tail);
        expect(Math.abs(head - tail - Math.min(row.expected.streakM, head - 0.5))).toBeLessThan(1e-9);
        expect(streak.speedMS).toBeCloseTo(row.speedMS, 12);
        expect(streak.opacity).toBeCloseTo(Math.max(0, Math.min(1, travelled / row.expected.fadeInS, (row.expected.travelS - travelled) / row.expected.fadeOutS)), 6);
        const moved = later.map((other) => other.points[other.points.length - 1]!.y - head).filter((d) => d > 0);
        if (travelled + 0.05 < row.expected.travelS) expect(Math.min(...moved)).toBeCloseTo(0.05 * row.speedMS, 9);
      }
    }
    expect(Math.max(...seen)).toBeLessThanOrEqual(row.expected.tracers);
    expect(Math.max(...seen)).toBeGreaterThanOrEqual(row.expected.tracers - 1);
    const heads = (t: number): number[] => flowMarks(field, t, detail).streaks.map((streak) => streak.points[streak.points.length - 1]!.y).sort((a, b) => a - b);
    expect(heads(3.3 + cycle).length).toBe(heads(3.3).length);
    heads(3.3 + cycle).forEach((y, i) => expect(y).toBeCloseTo(heads(3.3)[i]!, 9));
    expect(JSON.stringify(flowMarks(field, 3.3, detail))).toBe(JSON.stringify(flowMarks(field, 3.3, detail)));
  });

  it("sizes and colours the marks like the d3 scales", () => {
    const paint = flowPaint("light", flow, { mode: "shared" });
    const width = scaleSqrt().domain([0, paint.speedMaxMS]).range([FLOW_STYLE.streamMinMm, FLOW_STYLE.streamMaxMm]).clamp(true);
    for (const speed of [0, 0.01, 0.1, paint.speedMaxMS / 2, paint.speedMaxMS, 3 * paint.speedMaxMS]) expect(Math.abs(paint.streamWidthMm(speed) - width(speed))).toBeLessThan(fixture.tolerances.widthMm);
    const neutral = paint.excessColour(0);
    const warm = paint.excessColour(NEUTRAL_BAND * SHARED_FLOW_EXCESS_K);
    const blend = interpolateRgb(neutral, warm);
    const hex = (rgb: string): string => `#${rgb.match(/\d+/gu)!.map((v) => Number(v).toString(16).padStart(2, "0")).join("")}`;
    for (const f of [0.25, 0.5, 0.75]) expect(paint.excessColour(f * NEUTRAL_BAND * SHARED_FLOW_EXCESS_K)).toBe(hex(blend(f)));
    expect(paint.excessColour(-3)).not.toBe(paint.excessColour(3));
    expect(paint.excessColour(99)).toBe(paint.excessColour(SHARED_FLOW_EXCESS_K));
    expect(mix(neutral, neutral, 0.5)).toBe(neutral);
    expect(flowPaint("dark", flow, { mode: "shared" }).fogColour).not.toBe(paint.fogColour);
    const auto = flowPaint("light", flow, { mode: "auto" });
    expect(auto.excess.max).toBeLessThanOrEqual(SHARED_FLOW_EXCESS_K);
    expect(auto.excess.max).toBeGreaterThanOrEqual(0.5);
  });

  it("generalises coarser scales: fewer marks of the same field, never closer than the paper spacing", () => {
    const counts = fixture.scales.map((denominator) => {
      const detail = flowDetailOf(denominator);
      const marks = flowMarks(flow, flow.timeS, detail);
      const tree = quadtree<{ x: number; y: number; z: number; radiusM: number }>().x((p) => p.x).y((p) => p.y);
      for (const puff of marks.puffs) {
        const near = tree.find(puff.at.x, puff.at.y);
        if (near !== undefined) {
          const closest = Math.min(...tree.data().map((other) => Math.hypot(other.x - puff.at.x, other.y - puff.at.y, other.z - puff.at.z)));
          expect(closest).toBeGreaterThanOrEqual(Math.min(detail.spacingM, Math.max(PUFF_MIN_SPACING * detail.spacingM, PUFF_OWN_RADII * puff.radiusM)) - 1e-12);
        }
        tree.add({ ...puff.at, radiusM: puff.radiusM });
      }
      for (const ribbon of marks.ribbons) expect(flow.streams[ribbon.stream]!.points).toBe(ribbon.points);
      for (const streak of marks.streaks) {
        const length = streak.points.slice(1).reduce((sum, p, i) => sum + Math.hypot(p.x - streak.points[i]!.x, p.y - streak.points[i]!.y, p.z - streak.points[i]!.z), 0);
        const head = streak.points[streak.points.length - 1]!;
        const start = flow.streams[streak.stream]!.points[0]!;
        if (Math.hypot(head.x - start.x, head.y - start.y, head.z - start.z) > 2 * detail.minStreakM) expect(length).toBeGreaterThan(0.5 * detail.minStreakM);
      }
      expect(marks.trails).toEqual(flowTrails(flow, detail.spacingM));
      for (const trail of marks.trails) expect(flow.paths[trail.path]!.points).toBe(trail.points);
      return { ribbons: marks.ribbons.length, streaks: marks.streaks.length, puffs: marks.puffs.length, trails: marks.trails.length, drawn: count(draw("en", FLOW_LAYERS, "elevation", denominator).items, "flow-") };
    });
    for (let i = 1; i < counts.length; i++) {
      expect(counts[i]!.ribbons).toBeLessThanOrEqual(counts[i - 1]!.ribbons);
      expect(counts[i]!.puffs).toBeLessThanOrEqual(counts[i - 1]!.puffs);
      expect(counts[i]!.streaks).toBeLessThanOrEqual(counts[i - 1]!.streaks);
      expect(counts[i]!.trails).toBeLessThanOrEqual(counts[i - 1]!.trails);
    }
    expect(counts[counts.length - 1]!.ribbons).toBeGreaterThan(0);
    expect(counts[counts.length - 1]!.trails).toBeGreaterThan(0);
    expect(counts[counts.length - 1]!.trails).toBeLessThan(counts[0]!.trails);
    expect(counts[0]!.trails).toBeLessThanOrEqual(flow.paths.length);
    expect(counts[counts.length - 1]!.drawn).toBeLessThan(counts[0]!.drawn);
  });
});
