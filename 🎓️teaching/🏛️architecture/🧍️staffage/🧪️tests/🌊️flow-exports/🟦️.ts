/** 🌊️ Flow exports: every 2D export of the command line carries the thermodynamic air flow, proven on the written files reopened by third-party readers — svgelements and the XML parser of Python (SVG shapes, layer groups, paint), numpy (travel time along a stream), PDFium and pikepdf (PDF pixels, text, structure), ezdxf (DXF layers, polylines, audit, ACI colours), ezdwg (DWG), PyAV and the pinned ffmpeg (video frames).
 * `rsvg-convert` is not on this kind of host by rule (a system binary with no pinned source): the raster formats run through the MuPDF double of the oracle tool port, `ffmpeg` is the pinned real one.
 * @see ../../🧫️fixtures/🌊️flow-exports/🔣️.json — the scene, the CAD layers, the tolerances and the refusals
 * @see ./🐍️.py — the SVG and travel-time oracles
 * @see ../../🔨️modules/📤️export/🎬️cli/🟦️.ts — the command line */
import { mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { afterAll, beforeAll, describe, expect, it } from "vitest";
import type { Asset } from "../../🧬️schema/🟦️.ts";
import { DEFAULT_FLOW_LAYERS, FLOW_LAYERS, type DepictionOptions, type EvaluationQuery, type FlowField, type Point } from "../../🔨️modules/🔗️contracts/🟦️.ts";
import { evaluate } from "../../🔨️modules/🧮️evaluation/🟦️.ts";
import { FLOW_DEVELOP_S, transportOf } from "../../🔨️modules/🎨️depiction/🟦️.ts";
import { FLOW_STYLE, flowPaint } from "../../🔨️modules/🎨️depiction/🌊️flow-view/🟦️.ts";
import { FLOW_CAD, FLOW_CIRCLE_SEGMENTS, flowClassOf } from "../../🔨️modules/🎨️depiction/🌊️flow-view/📐️model/🟦️.ts";
import { DEFAULT_TURBULENCE } from "../../🔨️modules/💨️transport/🟦️.ts";
import { flowDetailOf, flowMarks } from "../../🔨️modules/💨️transport/🌊️flow/🟦️.ts";
import { CLI_POSE_PLAY_S, DEFAULT_BOUNDARY_LAYER, drawingOf, parseArguments, runExport, type CliOptions } from "../../🔨️modules/📤️export/🎬️cli/🟦️.ts";
import { configureDwg } from "../../🔨️modules/📤️export/🖊️dwg/🟦️.ts";
import { ensureDwgWasm, fixture, pixel, requireTool, STAFFAGE_ROOT } from "../🧰️support/🧰️.ts";
import { ffmpegBytes, ffmpegExecutable, inspectPdf, oracleToolPort, pdfText, probeVideo, python, readDwg, renderPdf } from "../🧰️support/🐍️.ts";

type Vectors = {
  still: { asset: string; pose: string; scale: number; language: "en" | "de"; conditions: string[]; environment: Record<string, number>; marks: string[]; svgLayer: string; legends: string[]; play: { durationS: number; fps: number; times: number[]; developS: number }; raster: { durationS: number; fps: number; frames: number; pxPerMm: number } };
  loop: { asset: string; clip: string; scale: number; durationS: number; midS: number };
  cad: { circleSegments: number; layers: Record<string, { aci: number; lineweight: number }>; present: string[]; sheetPrefixes: string[] };
  pdf: { dpi: number; warm: { redOverBlue: number; minPixels: number } };
  tolerances: { paperMm: number; modelM: number; domainMm: number; travelS: number; onStreamMm: number; loopMm: number };
  refusals: { flags: string[]; says: string }[];
};
type Box = readonly [number, number, number, number];
type SvgItem = { id: string | null; layer: string | null; box: Box | null; stroke: string | null; fill: string | null; strokeWidth: string | null; strokeOpacity: string | null; fillOpacity: string | null; dash: string | null };
type SvgReport = { width: string; height: string; items: SvgItem[]; texts: { id: string | null; layer: string | null; text: string }[]; scene: Record<string, string>; views: Record<string, string>[] };
type DxfReport = { projectname: string; errors: string[]; fixes: string[]; layers: Record<string, { aci: number; lineweight: number }>; lwpolylines: { layer: string; closed: boolean; points: number[][] }[] };

const vectors = fixture<Vectors>("🌊️flow-exports");
const { still, loop, cad, tolerances } = vectors;
const ORACLE = join(STAFFAGE_ROOT, "🧪️tests/🌊️flow-exports/🐍️.py");
const DXF_ORACLE = join(STAFFAGE_ROOT, "🧪️tests/📐️dxf-reopen/🐍️.py");
const readSvg = (file: string): SvgReport => JSON.parse(python(ORACLE, "geometry", ["svg", file])) as SvgReport;
const readDxf = (file: string): DxfReport => JSON.parse(python(DXF_ORACLE, "documents", [file])) as DxfReport;

requireTool("uv");

let out = "";
let counter = 0;
beforeAll(() => void (out = mkdtempSync(join(tmpdir(), "staffage-flow-exports-"))));
afterAll(() => rmSync(out, { recursive: true, force: true }));

const base = (asset: string, what: readonly string[], scale: number): string[] => ["--asset", asset, ...what, "--view", "thermal", "--scale", String(scale), "--lang", still.language];
const kettle = (extra: readonly string[]): string[] => [...base(still.asset, ["--pose", still.pose], still.scale), ...still.conditions, "--flow", "all", ...extra];
const parsed = (argv: readonly string[]): CliOptions => parseArguments([...argv, "--out", join(out, String(counter++))]);
const exported = (argv: readonly string[]): Promise<readonly string[]> => runExport(parsed(argv), oracleToolPort);

const isFlow = (item: { id: string | null }): boolean => item.id !== null && item.id.startsWith("flow-");
const isFlowLegend = (item: { id: string | null }): boolean => item.id !== null && /^legend-\d+-flow-/u.test(item.id);
const flowOf = (svg: SvgReport): SvgItem[] => svg.items.filter(isFlow);
const origin = (svg: SvgReport): Point => ({ x: Number(svg.scene["origin-x-mm"]), y: Number(svg.scene["origin-y-mm"]) });
const relative = (svg: SvgReport, item: SvgItem): Box => { const o = origin(svg); const [a, b, c, d] = item.box!; return [a - o.x, b - o.y, c - o.x, d - o.y]; };
const centre = (box: Box): Point => ({ x: (box[0] + box[2]) / 2, y: (box[1] + box[3]) / 2 });
const squeezed = (text: string): string => text.replace(/\s+/gu, " ");
const wording = (svg: SvgReport): string => squeezed(svg.texts.map((text) => text.text).join(" "));
const closeBoxes = (a: readonly (readonly number[])[], b: readonly (readonly number[])[], tolerance: number): boolean => a.length === b.length && a.every((box, i) => box.every((value, j) => Math.abs(value - b[i]![j]!) <= tolerance));
const rgb = (hex: string): readonly [number, number, number] => [1, 3, 5].map((at) => Number.parseInt(hex.slice(at, at + 2), 16)) as unknown as readonly [number, number, number];

/** 🧮️ The library's own truth of one command at one query: the asset, the depiction options, the state and the developed flow field. */
async function truth(argv: readonly string[], query?: EvaluationQuery): Promise<{ asset: Asset; drawing: Awaited<ReturnType<typeof drawingOf>>["drawing"]; field: FlowField; state: ReturnType<typeof evaluate>; scale: number }> {
  const options = parsed(argv);
  const { asset, drawing } = await drawingOf(options);
  const at = query ?? ({ kind: "pose", pose: options.pose! } as const);
  const state = evaluate(asset, at.kind === "pose" ? { kind: "pose", pose: at.pose } : at, { kind: "scale", scale: { denominator: options.scale } });
  return { asset, drawing, field: transportOf({ asset, query: at }, drawing).flow!, state, scale: options.scale };
}

const view = ["--viewpoint", "elevation"] as const;

describe("🎬️ the command line", () => {
  const argv = (flags: readonly string[]): string[] => ["--asset", still.asset, "--pose", still.pose, "--scale", String(still.scale), "--lang", "en", "--format", "svg", "--viewpoint", "elevation", "--out", "unused", ...flags];

  it("draws the default flow layers in a thermal export, every layer with --flow all, none with --flow none and none in the other views", () => {
    expect(parseArguments(argv(["--view", "thermal"])).flow).toEqual(DEFAULT_FLOW_LAYERS);
    expect(parseArguments(argv(["--view", "thermal", "--flow", "all"])).flow).toEqual(FLOW_LAYERS);
    expect(parseArguments(argv(["--view", "thermal", "--flow", "none"])).flow).toEqual([]);
    expect(parseArguments(argv(["--view", "thermal", "--flow", "fog,streams"])).flow).toEqual(["streams", "fog"]);
    expect(parseArguments(argv(["--view", "dark-line"])).flow).toBeUndefined();
    for (const format of ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm"]) expect(parseArguments([...argv(["--view", "thermal"]).filter((value, index, all) => value !== "--format" && all[index - 1] !== "--format"), "--format", format]).flow, format).toEqual(DEFAULT_FLOW_LAYERS);
    for (const format of ["sheet-svg", "sheet-pdf", "sheet-dxf", "sheet-dwg"]) expect(parseArguments(["--asset", still.asset, "--pose", still.pose, "--scale", "20", "--lang", "en", "--format", format, "--projection", "third", "--view", "thermal", "--out", "unused"]).flow, format).toEqual(DEFAULT_FLOW_LAYERS);
  });

  it("lays the environment flags over the resolved environment and turns the flow flags into conditions and mark detail", async () => {
    const options = parseArguments(argv(["--view", "thermal", ...still.conditions, "--sky-temp", "-10", "--ground-temp", "12", "--sun", "300", "--diffuse", "0.4", "--pressure", "95000", "--turbulence", "25", "--turbulence-time", "2", "--wind-profile", "logarithmic", "--roughness", "0.1", "--flow-life", "10", "--flow-seed", "7", "--flow-develop", "9", "--flow-spacing", "3", "--flow-streak", "1.2", "--flow-period", "2", "--flow-exposure", "0.5", "--flow-trail", "4", "--flow-stroke", "0.5"]));
    expect(options.environmentOverrides).toEqual({ ...still.environment, skyTempC: -10, groundTempC: 12, solarWM2: 300, diffuseFraction: 0.4, pressurePa: 95000 });
    expect(options.flowConditions).toEqual({ turbulence: { ...DEFAULT_TURBULENCE, intensity: 0.25, lagrangianTimeS: 2 }, boundaryLayer: { ...DEFAULT_BOUNDARY_LAYER, z0M: 0.1 }, maxAgeS: 10, seed: 7 });
    expect(options.flowDetail).toEqual({ spacingMm: 3, minStreakMm: 1.2, periodS: 2, exposureS: 0.5, trailS: 4, strokeMm: 0.5 });
    expect(options.flowDevelopS).toBe(9);
    const { asset, drawing } = await drawingOf(options);
    expect(drawing.environment).toEqual({ ...asset.thermal.environment, ...options.environmentOverrides });
    expect(drawing).toMatchObject({ flow: DEFAULT_FLOW_LAYERS, flowDevelopS: 9, flowConditions: options.flowConditions, flowDetail: options.flowDetail });
    const plain = await drawingOf(parseArguments(argv(["--view", "thermal"])));
    expect(plain.drawing).toMatchObject({ flow: DEFAULT_FLOW_LAYERS, flowDevelopS: FLOW_DEVELOP_S });
    expect(plain.drawing.environment).toBeUndefined();
    expect((await drawingOf(parseArguments(argv(["--view", "thermal", "--flow", "none"])))).drawing.flow).toBeUndefined();
  });

  it("plays a pose for --duration as a frame sequence and refuses --duration where nothing plays", () => {
    const sequence = (format: string, extra: readonly string[] = []): CliOptions => parseArguments([...argv(["--view", "thermal", ...extra]).map((value, index, all) => (all[index - 1] === "--format" ? format : value))]);
    expect(sequence("mp4").durationS).toBe(CLI_POSE_PLAY_S);
    expect(sequence("png-sequence", ["--duration", "2"]).durationS).toBe(2);
    expect(sequence("svg").durationS).toBeUndefined();
    expect(sequence("svg", ["--duration", "1"]).durationS).toBe(1);
    expect(() => sequence("mp4", ["--flow", "none", "--duration", "2"])).toThrow(/--duration is how long a pose plays its air flow/u);
    expect(() => parseArguments(["--asset", still.asset, "--clip", "rattle", "--view", "thermal", "--scale", "20", "--lang", "en", "--format", "mp4", "--viewpoint", "elevation", "--out", "unused", "--duration", "2"])).toThrow(/a clip has its own duration/u);
  });

  for (const refusal of vectors.refusals)
    it(`refuses ${refusal.flags.join(" ")} by name in English and German`, () => {
      let message = "";
      try {
        parseArguments(argv(refusal.flags));
      } catch (error) {
        message = (error as Error).message;
        expect((error as { reason?: { en: string; de: string } }).reason?.de.split("\n")[0]).not.toBe((error as { reason?: { en: string; de: string } }).reason?.en.split("\n")[0]);
      }
      expect(message).toContain(refusal.says);
    });

  it("refuses the flow and environment flags for the 3D model formats", () => {
    for (const flag of [["--flow", "all"], ["--wind", "1"], ["--turbulence", "10"]]) expect(() => parseArguments(["--asset", still.asset, "--pose", still.pose, "--lang", "en", "--format", "glb", "--out", "unused", ...flag])).toThrow(new RegExp(`${flag[0]} applies to the thermal view of the drawing formats`, "u"));
  });
});

describe("🖼️ SVG and PDF", { timeout: 600_000 }, () => {
  let svg: SvgReport;
  let bare: SvgReport;
  let known: Awaited<ReturnType<typeof truth>>;

  beforeAll(async () => {
    svg = readSvg((await exported(kettle(["--format", "svg", ...view])))[0]!);
    bare = readSvg((await exported(kettle(["--format", "svg", ...view, "--flow", "none"])))[0]!);
    known = await truth(kettle(["--format", "svg", ...view]));
  }, 600_000);

  it("solves the flow in the environment of the flags", () => {
    expect(known.field.ambient).toMatchObject({ airTempC: still.environment.airTempC, relativeHumidity: still.environment.relativeHumidity, windMS: still.environment.windMS, windDirectionDeg: still.environment.windDirectionDeg });
    expect(known.field.timeS).toBe(FLOW_DEVELOP_S);
  });

  it("holds every kind of flow mark in the layer group of the analysis field", () => {
    const flow = flowOf(svg);
    for (const prefix of still.marks) expect(flow.filter((item) => item.id!.startsWith(prefix)).length, prefix).toBeGreaterThan(0);
    expect(new Set(flow.map((item) => item.layer))).toEqual(new Set([still.svgLayer]));
    expect(svg.items.filter((item) => item.layer === still.svgLayer && !isFlow(item) && !item.id?.startsWith("halo"))).toEqual([]);
  });

  it("keeps every mark inside the projected flow domain", () => {
    const k = Number(svg.scene["mm-per-metre"]);
    expect(k).toBe(1000 / still.scale);
    const domain = known.field.domain;
    const slack = known.field.model.stepM * k + tolerances.domainMm;
    for (const item of flowOf(svg)) {
      const [minX, minY, maxX, maxY] = relative(svg, item);
      const reach = item.id!.startsWith("flow-fog") || /^flow-vapour-\d+-\d+$/u.test(item.id!) ? FLOW_STYLE.puffMaxMm : slack;
      expect(minX, item.id!).toBeGreaterThanOrEqual(domain.min.x * k - reach);
      expect(maxX, item.id!).toBeLessThanOrEqual(domain.max.x * k + reach);
      expect(minY, item.id!).toBeGreaterThanOrEqual(-domain.max.y * k - reach);
      expect(maxY, item.id!).toBeLessThanOrEqual(reach);
    }
  });

  it("paints streams, tracers and fog as the flow paint says, with warm air among them", () => {
    const paint = flowPaint("light", known.field, known.drawing.range);
    const bins = Array.from({ length: FLOW_STYLE.bins }, (_, i) => paint.excessColour(paint.excess.min + ((paint.excess.max - paint.excess.min) * i) / (FLOW_STYLE.bins - 1)));
    const flow = flowOf(svg);
    const streams = flow.filter((item) => item.id!.startsWith("flow-stream-"));
    for (const item of streams) expect(bins, item.id!).toContain(item.stroke);
    for (const item of flow.filter((entry) => entry.id!.startsWith("flow-arrow-"))) expect(bins, item.id!).toContain(item.fill);
    expect(streams.some((item) => { const [r, , b] = rgb(item.stroke!); return r > b + vectors.pdf.warm.redOverBlue; })).toBe(true);
    for (const item of flow.filter((entry) => /^flow-fog-\d+$/u.test(entry.id!))) expect(item.fill).toBe(paint.fogColour);
    for (const item of flow.filter((entry) => entry.id!.startsWith("flow-fog-rim-"))) expect(item.stroke).toBe(paint.fogRim);
    const marks = flowMarks(known.field, known.field.timeS, flowDetailOf(known.scale, known.drawing.flowDetail));
    const tracers = flow.filter((item) => /^flow-tracer-\d+$/u.test(item.id!));
    const bright = marks.streaks.filter((streak) => streak.opacity > 0.02).length;
    expect(tracers.length).toBeLessThanOrEqual(bright);
    expect(tracers.length).toBeGreaterThan(0.6 * bright);
    for (const item of tracers) {
      const streak = marks.streaks[Number(item.id!.slice("flow-tracer-".length))]!;
      expect(streak.opacity, item.id!).toBeGreaterThan(0.02);
      expect(item.stroke, item.id!).toBe(paint.excessColour(streak.excessK));
    }
    for (const legend of still.legends) expect(svg.texts.some((text) => text.id?.includes(`-${legend}-title`)), legend).toBe(true);
  });

  it("adds the flow to the figure and changes nothing of it: --flow none has no mark and the same figure", () => {
    expect(flowOf(bare)).toEqual([]);
    expect(bare.texts.filter(isFlowLegend)).toEqual([]);
    const figure = (report: SvgReport) => report.items.filter((item) => !isFlow(item) && !isFlowLegend(item) && item.layer !== "staffage-legend" && item.layer !== "staffage-annotations");
    const paint = (report: SvgReport) => figure(report).map((item) => ({ id: item.id, layer: item.layer, fill: item.fill, stroke: item.stroke }));
    expect(figure(bare).length).toBeGreaterThan(10);
    expect(paint(svg)).toEqual(paint(bare));
    expect(closeBoxes(figure(svg).map((item) => relative(svg, item)), figure(bare).map((item) => relative(bare, item)), tolerances.paperMm)).toBe(true);
  });

  it("shows warm-coloured pixels above the warm body in the PDF that PDFium renders, and none without the flow", async () => {
    const withFlow = (await exported(kettle(["--format", "pdf", ...view])))[0]!;
    const without = (await exported(kettle(["--format", "pdf", ...view, "--flow", "none"])))[0]!;
    for (const file of [withFlow, without]) {
      const report = inspectPdf(file);
      expect(report.problems).toEqual([]);
      expect(report.pages).toBe(1);
    }
    const k = 1000 / still.scale;
    const pxPerMm = vectors.pdf.dpi / 25.4;
    const domain = known.field.domain;
    const warm = (file: string, report: SvgReport): number => {
      const image = renderPdf(file, vectors.pdf.dpi);
      const o = origin(report);
      const px = (mm: number): number => Math.round(mm * pxPerMm);
      let count = 0;
      for (let y = Math.max(0, px(o.y - domain.max.y * k)); y < Math.min(image.height, px(o.y - known.state.bounds.maxY * k) - 2); y++)
        for (let x = Math.max(0, px(o.x + domain.min.x * k)); x < Math.min(image.width, px(o.x + domain.max.x * k)); x++) {
          const [r, g, b] = pixel(image, x, y);
          if (r - b > vectors.pdf.warm.redOverBlue && r > g) count++;
        }
      return count;
    };
    expect(warm(withFlow, svg)).toBeGreaterThan(vectors.pdf.warm.minPixels);
    expect(warm(without, bare)).toBe(0);
    const text = squeezed(pdfText(withFlow));
    expect(text).toContain("Air temperature excess of the flow");
    expect(text).toContain(`field at ${FLOW_DEVELOP_S.toFixed(2)} s`);
    expect(squeezed(pdfText(without))).not.toContain("field at");
  });
});

describe("🎞️ the frames of a pose", { timeout: 600_000 }, () => {
  const play = ["--duration", String(still.play.durationS), "--fps", String(still.play.fps)];
  let frames: SvgReport[];

  beforeAll(async () => {
    const files = await exported(kettle(["--format", "svg", ...view, ...play]));
    expect(files.length).toBe(still.play.times.length);
    frames = files.map(readSvg);
  }, 600_000);

  it("states in every frame the time its field has: developed first, then advancing with the frame times", () => {
    frames.forEach((frame, n) => expect(new Set(wording(frame).match(/field at [\d.]+ s/gu)), `frame ${n}`).toEqual(new Set([`field at ${(still.play.developS + still.play.times[n]!).toFixed(2)} s`])));
  });

  it("changes nothing but the flow marks and the flow legends between two frames", () => {
    const rest = (report: SvgReport) => report.items.filter((item) => !isFlow(item) && !isFlowLegend(item));
    const paint = (report: SvgReport) => rest(report).map((item) => ({ id: item.id, layer: item.layer, fill: item.fill, stroke: item.stroke }));
    const marks = (report: SvgReport) => JSON.stringify(flowOf(report).map((item) => relative(report, item).map((value) => value.toFixed(3))));
    expect(rest(frames[0]!).length).toBeGreaterThan(20);
    for (const frame of frames.slice(1)) {
      expect(paint(frame)).toEqual(paint(frames[0]!));
      expect(closeBoxes(rest(frame).map((item) => relative(frame, item)), rest(frames[0]!).map((item) => relative(frames[0]!, item)), tolerances.paperMm)).toBe(true);
      expect(marks(frame)).not.toBe(marks(frames[0]!));
    }
  });

  it("moves every tracer downstream by the frame time, as numpy measures the travel time of its head on its stream", async () => {
    const k = 1000 / still.scale;
    const dt = 1 / still.play.fps;
    for (const [n, frame] of frames.entries()) {
      const known = await truth(kettle(["--format", "svg", ...view, ...play]), { kind: "pose", pose: still.pose, timeS: still.play.times[n]! });
      expect(known.field.timeS).toBeCloseTo(still.play.developS + still.play.times[n]!, 9);
      const detail = flowDetailOf(known.scale, known.drawing.flowDetail);
      const now = flowMarks(known.field, known.field.timeS, detail);
      const before = flowMarks(known.field, known.field.timeS - dt, detail);
      const heads = new Map(frame.items.filter((item) => item.id?.startsWith("flow-tracer-head-")).map((item) => [Number(item.id!.slice("flow-tracer-head-".length)), centre(relative(frame, item))]));
      const planar = now.ribbons.map((ribbon) => ribbon.stream);
      const lines = planar.map((stream) => {
        const drawn = now.streaks.flatMap((streak, index) => (streak.stream === stream && heads.has(index) ? [heads.get(index)!] : []));
        const earlier = before.streaks.filter((streak) => streak.stream === stream).map((streak) => streak.points[streak.points.length - 1]!).map((p) => ({ x: p.x * k, y: -p.y * k }));
        return { stream, drawn: drawn.length, points: known.field.streams[stream]!.points.map((p) => [p.x * k, -p.y * k]), travelS: known.field.streams[stream]!.travelS, at: [...drawn, ...earlier].map((p) => [p.x, p.y]) };
      });
      const task = join(out, `travel-${n}.json`);
      writeFileSync(task, JSON.stringify({ lines }));
      const found = JSON.parse(python(ORACLE, "geometry", ["travel", task])) as { travelS: number; distance: number }[][];
      let moved = 0;
      lines.forEach((line, index) => {
        const total = line.travelS[line.travelS.length - 1]!;
        const seen = found[index]!.slice(0, line.drawn);
        for (const head of seen) expect(head.distance, `frame ${n} stream ${line.stream}: a head lies on its stream`).toBeLessThan(tolerances.onStreamMm);
        for (const start of found[index]!.slice(line.drawn)) {
          if (start.travelS < 0.02 * total || start.travelS + dt > 0.97 * total) continue;
          expect(Math.min(...seen.map((head) => Math.abs(head.travelS - start.travelS - dt))), `frame ${n} stream ${line.stream}: the tracer at ${start.travelS.toFixed(3)} s is ${dt} s further`).toBeLessThan(tolerances.travelS);
          moved++;
        }
      });
      expect(moved, `frame ${n}: tracers followed`).toBeGreaterThan(5);
    }
  });

  it("writes one PDF page per frame, each with its field time", async () => {
    const file = (await exported(kettle(["--format", "pdf", ...view, ...play])))[0]!;
    expect(inspectPdf(file).pages).toBe(still.play.times.length);
    const text = squeezed(pdfText(file));
    for (const time of still.play.times) expect(text).toContain(`field at ${(still.play.developS + time).toFixed(2)} s`);
  });
});

describe("🔁️ a looping clip", { timeout: 600_000 }, () => {
  it("draws one duration after its start the flow marks of its start, and others in between", async () => {
    const at = async (timeS: number): Promise<{ kind: string; box: Box; stroke: string | null; fill: string | null }[]> => {
      const report = readSvg((await exported([...base(loop.asset, ["--clip", loop.clip, "--time", String(timeS)], loop.scale), "--format", "svg", ...view]))[0]!);
      return flowOf(report).map((item) => ({ kind: item.id!.replace(/(-\d+)+$/u, ""), box: relative(report, item), stroke: item.stroke, fill: item.fill }));
    };
    const [start, middle, again] = [await at(0), await at(loop.midS), await at(loop.durationS)];
    const unmatched = (a: typeof start, b: typeof start): number => {
      const free = [...b];
      let missing = 0;
      for (const item of a) {
        const index = free.findIndex((other) => other.kind === item.kind && other.stroke === item.stroke && other.fill === item.fill && other.box.every((value, i) => Math.abs(value - item.box[i]!) < tolerances.loopMm));
        if (index < 0) missing++;
        else free.splice(index, 1);
      }
      return missing + free.length;
    };
    expect(start!.length).toBeGreaterThan(50);
    expect(start!.some((item) => item.kind === "flow-tracer")).toBe(true);
    expect(again!.length).toBe(start!.length);
    expect(unmatched(start!, again!)).toBe(0);
    expect(unmatched(start!, middle!)).toBeGreaterThan(start!.length / 4);
  });
});

describe("📐️ DXF and DWG", { timeout: 600_000 }, () => {
  let dxf: DxfReport;
  let file = "";
  let known: Awaited<ReturnType<typeof truth>>;
  const flowLines = (report: DxfReport, layer: (name: string) => boolean) => report.lwpolylines.filter((line) => layer(line.layer));

  beforeAll(async () => {
    file = (await exported(kettle(["--format", "dxf", ...view])))[0]!;
    dxf = readDxf(file);
    known = await truth(kettle(["--format", "dxf", ...view]));
  }, 600_000);

  it("has the flow layers with their colours and lineweights, audits clean and says so in its note", () => {
    expect(dxf.errors).toEqual([]);
    expect(dxf.fixes).toEqual([]);
    expect(dxf.projectname).toContain(`flow=${FLOW_LAYERS.join("+")}`);
    const names = Object.keys(dxf.layers).filter((name) => name.startsWith("FLOW-"));
    for (const name of cad.present) expect(names, name).toContain(name);
    for (const name of names) expect(dxf.layers[name], name).toEqual(cad.layers[name]);
    expect(cad.layers["FLOW-STREAMS-WARM"]!.aci).toBe(FLOW_CAD.aci.WARM);
    expect(cad.circleSegments).toBe(FLOW_CIRCLE_SEGMENTS);
  });

  it("gives every flow layer a colour of its class, as ezdxf resolves the ACI index", () => {
    const colours = JSON.parse(python(ORACLE, "documents", ["aci", ...new Set(Object.values(cad.layers).map((layer) => String(layer.aci)))])) as Record<string, [number, number, number]>;
    for (const [name, layer] of Object.entries(cad.layers)) {
      const [r, g, b] = colours[String(layer.aci)]!;
      if (name.endsWith("-WARM")) expect(r > b && r > g, name).toBe(true);
      else if (name.endsWith("-COOL")) expect(b > r && b > g, name).toBe(true);
      else if (name.endsWith("-NEUTRAL")) expect(r === g && g === b, name).toBe(true);
      else expect(g > r && b > r, name).toBe(true);
    }
    const paint = flowPaint("light", known.field, known.drawing.range);
    const [warmR, , warmB] = rgb(paint.excessColour(paint.excess.max));
    const [coolR, , coolB] = rgb(paint.excessColour(paint.excess.min));
    expect(warmR > warmB && coolB > coolR).toBe(true);
  });

  it("draws the streams as the projected stream points in metres, each run in the layer of its class", () => {
    const paint = flowPaint("light", known.field, known.drawing.range);
    const marks = flowMarks(known.field, known.field.timeS, flowDetailOf(known.scale, known.drawing.flowDetail));
    const runs = flowLines(dxf, (name) => name.startsWith("FLOW-STREAMS-"));
    expect(runs.length).toBeGreaterThanOrEqual(marks.ribbons.length);
    const near = (a: readonly number[], b: { x: number; y: number }): boolean => Math.abs(a[0]! - b.x) < tolerances.modelM && Math.abs(a[1]! - b.y) < tolerances.modelM;
    let covered = 0;
    for (const run of runs) {
      expect(run.closed).toBe(false);
      const home = marks.ribbons.find((ribbon) => { const start = ribbon.points.findIndex((p) => near(run.points[0]!, p)); return start >= 0 && run.points.every((q, i) => ribbon.points[start + i] !== undefined && near(q, ribbon.points[start + i]!)); });
      expect(home, `a run of ${run.points.length} points on ${run.layer}`).toBeDefined();
      const start = home!.points.findIndex((p) => near(run.points[0]!, p));
      expect(run.layer).toBe(`FLOW-STREAMS-${flowClassOf(home!.excessK[start]!, paint.excess)}`);
      covered += run.points.length - 1;
    }
    expect(covered).toBe(marks.ribbons.reduce((sum, ribbon) => sum + ribbon.points.length - 1, 0));
  });

  it("draws vapour and fog as circles inside the flow domain, arrow heads as triangles and tracers as streaks with round heads", () => {
    const domain = known.field.domain;
    const reach = (FLOW_STYLE.puffMaxMm * still.scale) / 1000;
    const circles = [...flowLines(dxf, (name) => name === "FLOW-FOG"), ...flowLines(dxf, (name) => name === "FLOW-VAPOUR").filter((line) => line.closed)];
    expect(circles.length).toBeGreaterThan(5);
    for (const circle of circles) {
      expect(circle.points.length).toBe(cad.circleSegments);
      const cx = circle.points.reduce((sum, p) => sum + p[0]!, 0) / circle.points.length;
      const cy = circle.points.reduce((sum, p) => sum + p[1]!, 0) / circle.points.length;
      const radii = circle.points.map((p) => Math.hypot(p[0]! - cx, p[1]! - cy));
      expect(Math.max(...radii) - Math.min(...radii)).toBeLessThan(tolerances.modelM);
      expect(radii[0]!).toBeLessThanOrEqual(reach + tolerances.modelM);
      expect(cx >= domain.min.x && cx <= domain.max.x && cy >= 0 && cy <= domain.max.y, `centre ${cx}, ${cy}`).toBe(true);
    }
    const marks = flowMarks(known.field, known.field.timeS, flowDetailOf(known.scale, known.drawing.flowDetail));
    expect(flowLines(dxf, (name) => name === "FLOW-FOG").length).toBe(marks.puffs.filter((puff) => puff.condensateKgPerKg > 0).length);
    const arrows = flowLines(dxf, (name) => name.startsWith("FLOW-ARROWS-"));
    expect(arrows.length).toBeGreaterThan(5);
    for (const arrow of arrows) expect(arrow.closed && arrow.points.length === 3).toBe(true);
    const tracers = flowLines(dxf, (name) => name.startsWith("FLOW-TRACERS-"));
    const drawn = marks.streaks.filter((streak) => streak.opacity > 0.02).length;
    expect(tracers.filter((line) => line.closed).length).toBe(drawn);
    expect(tracers.filter((line) => !line.closed).length).toBeLessThanOrEqual(drawn);
    const slack = known.field.model.stepM + reach;
    for (const line of flowLines(dxf, (name) => name.startsWith("FLOW-"))) for (const [x, y] of line.points) expect(x! >= domain.min.x - slack && x! <= domain.max.x + slack && y! >= -slack && y! <= domain.max.y + slack, `${line.layer} ${x}, ${y}`).toBe(true);
  });

  it("has no flow layer with --flow none", async () => {
    const bare = readDxf((await exported(kettle(["--format", "dxf", ...view, "--flow", "none"])))[0]!);
    expect(Object.keys(bare.layers).filter((name) => name.startsWith("FLOW-"))).toEqual([]);
    expect(bare.projectname).not.toContain("flow=");
    const figure = (report: DxfReport) => report.lwpolylines.filter((line) => !line.layer.startsWith("FLOW-"));
    expect(figure(bare)).toEqual(figure(dxf));
  });

  it("writes the same layers and polylines into the DWG that ezdwg reads", async () => {
    configureDwg({ wasm: readFileSync(ensureDwgWasm()) });
    const report = readDwg((await exported(kettle(["--format", "dwg", ...view])))[0]!);
    expect(report.version).toBe("AC1024");
    for (const [name, layer] of Object.entries(dxf.layers).filter(([name]) => name.startsWith("FLOW-"))) expect(report.layers[name], name).toMatchObject({ colorIndex: layer.aci, lineweight: layer.lineweight });
    const theirs = report.entities.filter((entity) => entity.type === "LWPOLYLINE" && entity.layer?.startsWith("FLOW-")).map((entity) => ({ layer: entity.layer!, closed: entity.closed!, points: entity.points! }));
    const ours = flowLines(dxf, (name) => name.startsWith("FLOW-"));
    expect(ours.length).toBeGreaterThan(50);
    expect(theirs.length).toBe(ours.length);
    const free = [...theirs];
    for (const line of ours) {
      const index = free.findIndex((other) => other.layer === line.layer && other.closed === line.closed && other.points.length === line.points.length && other.points.every((p, i) => Math.abs(p[0]! - line.points[i]![0]!) < tolerances.modelM && Math.abs(p[1]! - line.points[i]![1]!) < tolerances.modelM));
      expect(index, `${line.layer} polyline of ${line.points.length} points`).toBeGreaterThanOrEqual(0);
      free.splice(index, 1);
    }
  });
});

describe("📑️ the multiview sheets", { timeout: 600_000 }, () => {
  const sheet = ["--projection", "third"] as const;

  it("draws the tracers of one field at one instant in every cell, each aligned between the cells that show it, with the flow legends once", async () => {
    const report = readSvg((await exported(kettle(["--format", "sheet-svg", ...sheet])))[0]!);
    const origins = Object.fromEntries(report.views.map((entry) => [entry.viewpoint!, { x: Number(entry["origin-x-mm"]), y: Number(entry["origin-y-mm"]) }])) as Record<"elevation" | "front" | "plan", Point>;
    const heads = (cell: string): Map<string, Point> => new Map(report.items.filter((item) => item.id?.startsWith(`${cell}-flow-tracer-head-`)).map((item) => [item.id!.slice(`${cell}-flow-tracer-head-`.length), centre(item.box!)]));
    const [elevation, front, plan] = [heads("elevation"), heads("front"), heads("plan")];
    expect(elevation.size).toBeGreaterThan(20);
    let shared = 0;
    for (const [id, at] of elevation) {
      if (plan.has(id)) expect(plan.get(id)!.x - origins.plan.x, `tracer ${id}: plan and elevation share the paper x`).toBeCloseTo(at.x - origins.elevation.x, 3);
      if (front.has(id)) expect(front.get(id)!.y - origins.front.y, `tracer ${id}: front and elevation share the paper y`).toBeCloseTo(at.y - origins.elevation.y, 3);
      if (plan.has(id) && front.has(id)) shared++;
    }
    expect(shared, "tracers that all three cells show (the body hides some from one side)").toBeGreaterThan(0.6 * elevation.size);
    for (const cell of ["elevation", "front", "plan"]) for (const prefix of still.marks) expect(report.items.some((item) => item.id?.startsWith(`${cell}-${prefix}`)), `${cell} ${prefix}`).toBe(true);
    for (const legend of still.legends) expect(report.texts.filter((text) => text.id?.endsWith(`-${legend}-title`)).map((text) => text.id!.split("-")[0]), legend).toEqual(["elevation"]);
    expect(new Set(wording(report).match(/field at [\d.]+ s/gu))).toEqual(new Set([`field at ${FLOW_DEVELOP_S.toFixed(2)} s`]));
  });

  it("carries the flow layers of every view in the CAD sheet, which audits clean", async () => {
    const report = readDxf((await exported(kettle(["--format", "sheet-dxf", ...sheet])))[0]!);
    expect(report.errors).toEqual([]);
    expect(report.fixes).toEqual([]);
    for (const prefix of cad.sheetPrefixes)
      for (const name of cad.present) {
        expect(report.layers[`${prefix}-${name}`], `${prefix}-${name}`).toEqual(cad.layers[name]);
        expect(report.lwpolylines.some((line) => line.layer === `${prefix}-${name}`), `${prefix}-${name} has geometry`).toBe(true);
      }
    const single = readDxf((await exported(kettle(["--format", "dxf", ...view])))[0]!);
    const count = (source: DxfReport, layer: string): number => source.lwpolylines.filter((line) => line.layer === layer).length;
    for (const name of ["FLOW-FOG", "FLOW-STREAMS-WARM", "FLOW-TRACERS-NEUTRAL"]) expect(count(report, `ELEVATION-${name}`), name).toBe(count(single, name));
  });
});

describe("🎥️ frame sequences and videos of a pose", { timeout: 600_000 }, () => {
  const raster = ["--duration", String(still.raster.durationS), "--fps", String(still.raster.fps), "--px-per-mm", String(still.raster.pxPerMm)];

  it("packs the scheduled frames of the playing flow into the PNG sequence, each frame another picture on one canvas", async () => {
    const file = (await exported(kettle(["--format", "png-sequence", ...view, ...raster])))[0]!;
    const listed = JSON.parse(python(ORACLE, null, ["frames", file])) as { manifest: { frameCount: number; fps: number; frames: { timeS: number }[]; size: { widthPx: number; heightPx: number } }; frames: { name: string; width: number; height: number; digest: string }[] };
    expect(listed.manifest.frameCount).toBe(still.raster.frames);
    expect(listed.manifest.fps).toBe(still.raster.fps);
    expect(listed.manifest.frames.map((frame) => frame.timeS)).toEqual(Array.from({ length: still.raster.frames }, (_, n) => n / still.raster.fps));
    expect(listed.frames.length).toBe(still.raster.frames);
    for (const frame of listed.frames) expect([frame.width, frame.height]).toEqual([listed.manifest.size.widthPx, listed.manifest.size.heightPx]);
    expect(new Set(listed.frames.map((frame) => frame.digest)).size).toBe(still.raster.frames);
  });

  it("encodes the playing flow as MP4 at the frame rate, and the decoded frames differ", async () => {
    ffmpegExecutable();
    const file = (await exported(kettle(["--format", "mp4", ...view, ...raster])))[0]!;
    const video = probeVideo(file);
    expect(video.codec).toBe("h264");
    expect(video.rFrameRate).toBe(`${still.raster.fps}/1`);
    expect(video.framesRead).toBe(still.raster.frames);
    const raw = ffmpegBytes(["-v", "error", "-i", file, "-f", "rawvideo", "-pix_fmt", "gray", "pipe:1"]);
    const size = video.width * video.height;
    expect(raw.length).toBe(size * still.raster.frames);
    const changed = (a: number, b: number): number => { let count = 0; for (let i = 0; i < size; i++) if (Math.abs(raw[a * size + i]! - raw[b * size + i]!) > 40) count++; return count; };
    expect(changed(0, 1)).toBeGreaterThan(20);
    expect(changed(0, still.raster.frames - 1)).toBeGreaterThan(20);
  });
});
