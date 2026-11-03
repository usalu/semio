/** 📑️ The aligned multiview sheet and every export path that names its view: the plan, front and side elevation of ONE evaluated state, written as SVG, PDF and DXF at the physical size `mm = m * 1000 / denominator`, read back by independent parsers (lxml, PyMuPDF, ezdxf), byte-deterministic, and addressed by view in every output name, metadata note and CAD content.
 *
 * @see ./🥒️.feature — the scenarios
 * @see ./🐍️.py — the oracle
 * @see ../../🔨️modules/📤️export/📑️sheet/🟦️.ts — the sheet
 */
import { copyFileSync, readFileSync, readdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { beforeAll, describe, expect, it } from "vitest";
import type { Asset, Viewpoint } from "../../🧬️schema/🟦️.ts";
import type { EvaluationQuery, View } from "../../🔨️modules/🔗️contracts/🟦️.ts";
import { defaultDepictionOptions, modelScene } from "../../🔨️modules/🎨️depiction/🟦️.ts";
import { configureDwg } from "../../🔨️modules/📤️export/🖊️dwg/🟦️.ts";
import { parseArguments, runExport, type ToolPort } from "../../🔨️modules/📤️export/🎬️cli/🟦️.ts";
import { writeDxf } from "../../🔨️modules/📤️export/📐️dxf/🟦️.ts";
import { PROJECTIONS, SheetUnavailableError, modelSheet, projectionSymbolGeometry, resolveSheet, withNote, type Projection } from "../../🔨️modules/📤️export/📑️sheet/🟦️.ts";
import { writePdf } from "../../🔨️modules/📤️export/📕️pdf/🟦️.ts";
import { frameName, frameSchedule, writePngSequenceZip } from "../../🔨️modules/📤️export/🎞️frames/🟦️.ts";
import { encodeClip, videoFileName, type VideoFrame } from "../../🔨️modules/📤️export/🎥️video/🟦️.ts";
import { writeSvg } from "../../🔨️modules/📤️export/🖼️svg/🟦️.ts";
import { DEFAULT_CATALOG_ROOT, loadCatalogFromFiles } from "../../🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { evaluate } from "../../🔨️modules/🧮️evaluation/🟦️.ts";
import { type DrawOptions, resolveViews } from "../../🔨️modules/🔭️multiview/🟦️.ts";
import { solveStructure } from "../../🔨️modules/🏗️structure/🟦️.ts";
import { authoredScales, extentOf } from "../🧊️three-views/🧰️.ts";
import { STAFFAGE_ROOT, python, withScratch } from "../🧰️support/🧰️.ts";

type Box = [number, number, number, number] | null;
type Boxes = Record<Viewpoint, Box>;
type Report = {
  svg: { widthMm: number; heightMm: number; viewBox: number[]; boxes: Boxes; scale: string; mmPerMetre: number; projection: string | null; title: string; desc: string; attributes: Record<string, string>; origins: Record<Viewpoint, [number, number]> };
  pdf: { pages: number; widthMm: number; heightMm: number; boxes: Boxes; metadata: Record<string, string> };
  dxf: { errors: string[]; insunits: number; projectname: string; boxes: Boxes; layers: string[]; texts: { layer: string; text: string }[] } | null;
};

type Case = { readonly name: string; readonly asset: string; readonly query: EvaluationQuery | "default"; readonly view: View; readonly scaleIndex: number };
const CASES: readonly Case[] = [
  { name: "a pointing adult in line", asset: "standing-adult", query: { kind: "pose", pose: "pointing" }, view: "light-line", scaleIndex: 0 },
  { name: "a walking commuter at 0.3 s of the walk cycle, solid", asset: "walking-commuter", query: { kind: "clip", clip: "walk-cycle", timeS: 0.3 }, view: "dark-solid", scaleIndex: 1 },
  { name: "a tilted window under the thermal mode", asset: "tilt-window", query: { kind: "pose", pose: "tilted-open" }, view: "thermal", scaleIndex: 0 },
  { name: "a forklift in the moments mode", asset: "forklift", query: "default", view: "moments", scaleIndex: 0 },
];

let assets: ReadonlyMap<string, Asset>;
beforeAll(async () => {
  assets = new Map((await loadCatalogFromFiles(DEFAULT_CATALOG_ROOT)).assets.map((asset) => [asset.id, asset]));
  for (const entry of CASES)
    for (const projection of PROJECTIONS) {
      const b = built(entry, projection, "en");
      jobs.set(`${entry.name}|${projection}`, { svg: writeSvg(b.sheet), pdf: writePdf([b.sheet]), dxf: cad(b, projection, "en", entry.view) });
    }
  const asset = assets.get("jogger")!;
  const symbol = asset.symbols.find((candidate) => candidate.id === "runner-direction-mark")!;
  const conventional = resolveSheet(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "symbol", symbol: symbol.id, denominator: symbol.denominators[0]! }, "light-line", optionsOf("light-line", symbol.denominators[0]!, "en"), "third", "en").sheet;
  jobs.set("conventional", { svg: writeSvg(conventional), pdf: writePdf([conventional]), dxf: null });
  for (const entry of CASES) {
    const { sheet } = built({ ...entry, query: "default" }, "third", "en");
    jobs.set(`d18|${entry.asset}`, { svg: writeSvg(sheet), pdf: writePdf([sheet]), dxf: null });
  }
  await withScratch((directory) => {
    const manifest: Record<string, { svg: string; pdf: string; dxf: string | null }> = {};
    [...jobs.entries()].forEach(([name, job], index) => {
      const stem = join(directory, `job-${index}`);
      writeFileSync(`${stem}.svg`, job.svg);
      writeFileSync(`${stem}.pdf`, job.pdf);
      if (job.dxf !== null) writeFileSync(`${stem}.dxf`, job.dxf);
      manifest[name] = { svg: `${stem}.svg`, pdf: `${stem}.pdf`, dxf: job.dxf === null ? null : `${stem}.dxf` };
    });
    writeFileSync(join(directory, "jobs.json"), JSON.stringify(manifest));
    reports = JSON.parse(python(ORACLE, ["lxml", "pymupdf", "ezdxf"], [join(directory, "jobs.json")])) as Record<string, Report>;
  });
}, 600_000);

const ORACLE = join(STAFFAGE_ROOT, "🧪️tests/📑️multiview-sheet/🐍️.py");
const queryOf = (asset: Asset, query: EvaluationQuery | "default"): EvaluationQuery => (query === "default" ? { kind: "pose", pose: asset.presentation.defaultPose } : query);
const scaleOf = (asset: Asset, index: number): number => {
  const scales = authoredScales(asset);
  return scales[Math.min(index, scales.length - 1)]!;
};
const mm = (metres: number, denominator: number): number => (metres * 1000) / denominator;
const near = (actual: number, expected: number, label: string, tolerance = 0.02): void => expect(Math.abs(actual - expected), `${label}: ${actual} against ${expected}`).toBeLessThanOrEqual(tolerance);
const extentMm = (box: NonNullable<Box>): { x: number; up: number } => ({ x: box[2] - box[0], up: box[3] - box[1] });
const optionsOf = (view: View, denominator: number, language: "en" | "de"): DrawOptions => {
  const { view: _view, viewpoint: _viewpoint, scale: _scale, representation: _representation, ...rest } = defaultDepictionOptions(view, language, denominator);
  return rest as DrawOptions;
};
const diagramsOf = (view: View) => (view === "forces" || view === "moments" ? (["moment"] as const) : []);

function built(entry: Case, projection: Projection, language: "en" | "de") {
  const asset = assets.get(entry.asset)!;
  const denominator = scaleOf(asset, entry.scaleIndex);
  const resolved = resolveSheet(asset, queryOf(asset, entry.query), { kind: "scale", denominator }, entry.view, optionsOf(entry.view, denominator, language), projection, language);
  return { asset, denominator, ...resolved };
}

const cad = (b: ReturnType<typeof built>, projection: Projection, language: "en" | "de", view: View): string => writeDxf(withNote(modelSheet(b.state, { projection, denominator: b.denominator, language, analysis: b.analysis, diagrams: diagramsOf(view) }), "note", b.denominator), "note");

type Job = { readonly svg: string; readonly pdf: Uint8Array; readonly dxf: string | null };
const jobs = new Map<string, Job>();
let reports: Record<string, Report> = {};
const oracle = (name: string): Report => reports[name] ?? (() => { throw new Error(`no oracle report for ${name}`); })();

describe("📑️ multiview sheet", () => {
  for (const entry of CASES)
    for (const projection of PROJECTIONS)
      it(`writes ${entry.name} as ${projection}-angle SVG, PDF and DXF at the physical size`, async () => {
        const b = built(entry, projection, "en");
        const { denominator, state, sheet } = b;
        const report = oracle(`${entry.name}|${projection}`);
        const dxf = report.dxf!;
        const views = ["elevation", "front", "plan"] as const;
        expect(report.svg.widthMm).toBeCloseTo(sheet.widthMm, 3);
        expect(report.svg.heightMm).toBeCloseTo(sheet.heightMm, 3);
        expect(report.svg.viewBox.slice(2)).toEqual([sheet.widthMm, sheet.heightMm].map((v) => Number(v.toFixed(4))));
        expect(report.svg.scale).toBe(`1:${denominator}`);
        expect(report.svg.mmPerMetre).toBeCloseTo(1000 / denominator, 9);
        expect(report.svg.projection).toBe(projection);
        expect(report.pdf.pages).toBe(1);
        expect(report.pdf.widthMm).toBeCloseTo(sheet.widthMm, 2);
        expect(report.pdf.heightMm).toBeCloseTo(sheet.heightMm, 2);
        expect(dxf.errors).toEqual([]);
        expect(dxf.insunits).toBe(6);
        expect(dxf.projectname).toBe("note");
        expect(dxf.layers).toEqual(expect.arrayContaining(["ELEVATION-STAFFAGE-BODY", "FRONT-STAFFAGE-BODY", "PLAN-STAFFAGE-BODY", "SHEET-LABELS", "STAFFAGE-NOTE"]));
        for (const view of views) {
          const metres = extentOf(state.shapes[view])!;
          const expected = { x: mm(metres.maxX - metres.minX, denominator), up: mm(metres.maxY - metres.minY, denominator) };
          const label = `${entry.asset} ${view} 1:${denominator}`;
          const svg = extentMm(report.svg.boxes[view]!);
          near(svg.x, expected.x, `${label} svg paper x`);
          near(svg.up, expected.up, `${label} svg paper up`);
          const pdf = extentMm(report.pdf.boxes[view]!);
          near(pdf.x, expected.x, `${label} pdf paper x`, 0.03);
          near(pdf.up, expected.up, `${label} pdf paper up`, 0.03);
          const model = extentMm(dxf.boxes[view]!);
          near(model.x, metres.maxX - metres.minX, `${label} dxf x (metres)`);
          near(model.up, metres.maxY - metres.minY, `${label} dxf up (metres)`);
        }
        const { elevation, front, plan } = report.svg.origins;
        expect(plan[0], "plan shares x with the elevation").toBeCloseTo(elevation[0], 3);
        expect(front[1], "front shares y with the elevation").toBeCloseTo(elevation[1], 3);
        if (projection === "third") {
          expect(report.svg.boxes.plan![3]).toBeLessThan(report.svg.boxes.elevation![1]);
          expect(report.svg.boxes.front![0]).toBeGreaterThan(report.svg.boxes.elevation![2]);
        } else {
          expect(report.svg.boxes.plan![1]).toBeGreaterThan(report.svg.boxes.elevation![3]);
          expect(report.svg.boxes.front![2]).toBeLessThan(report.svg.boxes.elevation![0]);
        }
        for (const text of [report.svg.title, report.svg.desc, report.pdf.metadata.subject ?? "", report.pdf.metadata.keywords ?? ""]) expect(text, "no raw mode id in prose").not.toContain(entry.view);
        expect(report.svg.attributes["view"]).toBe(entry.view);
        expect(report.svg.attributes["view-title-de"]).toBeTruthy();
        expect(report.svg.attributes["fidelity"]).toBe("physical");
        expect(report.svg.desc).toContain("Physical drawing, to scale");
        expect(report.pdf.metadata.subject).toContain("Physical drawing, to scale");
        expect(report.pdf.metadata.subject).toContain("plan, front elevation and side elevation");
        expect(dxf.texts.find((t) => t.layer === "STAFFAGE-NOTE")!.text).toBe("note");
      }, 120_000);

  it("marks a conventional symbol in the metadata and as a visible localized note on the sheet", async () => {
    const asset = assets.get("jogger")!;
    const query = { kind: "pose", pose: asset.presentation.defaultPose } as const;
    const symbol = asset.symbols.find((candidate) => candidate.id === "runner-direction-mark")!;
    const denominator = symbol.denominators[0]!;
    const make = (language: "en" | "de") => resolveSheet(asset, query, { kind: "symbol", symbol: symbol.id, denominator }, "light-line", optionsOf("light-line", denominator, language), "third", language).sheet;
    const sheet = make("en");
    expect(sheet.metadata.fidelity).toBe("conventional-symbol");
    expect(Object.values(sheet.sheet.fidelity)).toEqual(["conventional-symbol", "conventional-symbol", "conventional-symbol"]);
    const svg = writeSvg(sheet);
    expect(svg).toContain(`fidelity="conventional-symbol"`);
    expect(svg).toContain(">Conventional symbol, not to scale<");
    expect(writeSvg(make("de"))).toContain(">Konventionelles Symbol, nicht maßstäblich<");
    const report = oracle("conventional");
    expect(report.svg.attributes["fidelity-title-de"]).toBe("Konventionelles Symbol, nicht maßstäblich");
    expect(report.pdf.metadata.subject).toContain("Conventional symbol, not to scale / Konventionelles Symbol, nicht maßstäblich");
  }, 120_000);

  it("proves the catalog dimensions on paper for the four cases at the default pose within max(2 %, 15 mm)", async () => {
    const rows: Record<string, number | string>[] = [];
    for (const entry of CASES) {
      const asset = assets.get(entry.asset)!;
      const denominator = scaleOf(asset, entry.scaleIndex);
      const report = oracle(`d18|${entry.asset}`);
      const paper = (view: Viewpoint) => extentMm(report.svg.boxes[view]!);
      const measured = { "elevation widthM": paper("elevation").x, "elevation heightM": paper("elevation").up, "front depthM": paper("front").x, "front heightM": paper("front").up, "plan widthM": paper("plan").x, "plan depthM": paper("plan").up };
      const wanted = { "elevation widthM": asset.dimensions.widthM, "elevation heightM": asset.dimensions.heightM, "front depthM": asset.dimensions.depthM, "front heightM": asset.dimensions.heightM, "plan widthM": asset.dimensions.widthM, "plan depthM": asset.dimensions.depthM };
      for (const [name, value] of Object.entries(measured)) {
        const metres = (value * denominator) / 1000;
        const dimension = wanted[name as keyof typeof wanted];
        rows.push({ asset: asset.id, scale: denominator, extent: name, catalogM: dimension, paperMm: Number(value.toFixed(3)), expectedPaperMm: Number(mm(dimension, denominator).toFixed(3)), deltaM: Number((metres - dimension).toFixed(4)), toleranceM: Math.max(0.02 * dimension, 0.015) });
      }
    }
    const outside = rows.filter((row) => Math.abs(row.deltaM as number) > (row.toleranceM as number));
    if (process.env["STAFFAGE_DELTAS"]) writeFileSync(process.env["STAFFAGE_DELTAS"], JSON.stringify(rows, null, 2));
    expect(outside, `outside the D18 tolerance: ${JSON.stringify(outside)}`).toEqual([]);
  }, 300_000);

  it("draws the projection-method symbol of ISO 5456-2 as the Wikimedia reference symbols do: first-angle circles left of the frustum, third-angle circles right, the frustum's shortest side always on the right", () => {
    const reference = {
      first: { frustum: [[110, 10], [190, 30], [190, 70], [110, 90]], circles: [100, 50, 40, 20] },
      third: { frustum: [[10, 10], [90, 30], [90, 70], [10, 90]], circles: [150, 50, 40, 20] },
    } as const;
    for (const projection of PROJECTIONS) {
      const geometry = projectionSymbolGeometry(projection);
      const ref = reference[projection];
      const scale = 8 / 80;
      const minX = Math.min(...ref.frustum.map((p) => p[0]!));
      const circleLeft = ref.circles[0] - ref.circles[2];
      const originX = Math.min(minX, circleLeft);
      const mine = geometry.frustum.map((p) => [p.x, p.y]);
      const expected = ref.frustum.map((p) => [(p[0]! - originX) * scale, (p[1]! - 10) * scale]);
      const shift = Math.min(...mine.map((p) => p[0]!)) - Math.min(...expected.map((p) => p[0]!));
      mine.forEach((p, i) => {
        near(p[0]! - shift, expected[i]![0]!, `${projection} frustum corner ${i} x`, 1e-9);
        near(p[1]!, expected[i]![1]!, `${projection} frustum corner ${i} y`, 1e-9);
      });
      near(geometry.circles.radii[0], 4, `${projection} outer circle`, 1e-9);
      near(geometry.circles.radii[1], 2, `${projection} inner circle`, 1e-9);
      near(geometry.circles.cx - Math.min(...geometry.frustum.map((p) => p.x)), (ref.circles[0] - minX) * scale, `${projection} circle centre against the frustum`, 1e-9);
    }
    const circlesLeftOfFrustum = (g: ReturnType<typeof projectionSymbolGeometry>): boolean => g.circles.cx + g.circles.radii[0] <= Math.min(...g.frustum.map((p) => p.x));
    expect(circlesLeftOfFrustum(projectionSymbolGeometry("first")), "first-angle: circles left").toBe(true);
    expect(circlesLeftOfFrustum(projectionSymbolGeometry("third")), "third-angle: circles not left").toBe(false);
    const { sheet } = built(CASES[0]!, "third", "en");
    const frustum = sheet.layers.flatMap((layer) => layer.items).find((item) => item.id === "projection-symbol:frustum");
    const circle = sheet.layers.flatMap((layer) => layer.items).find((item) => item.id === "projection-symbol:outer");
    expect(frustum && circle, "the sheet draws the symbol").toBeTruthy();
    const corner = (item: typeof frustum): number => ((item as unknown as { commands: { x: number }[] }).commands[0]!.x);
    expect(corner(circle) > corner(frustum), "third-angle sheet: circles right of the frustum").toBe(true);
  });

  it("extends the front ground line symmetrically from the span of the front shapes, and the one-sided stretch is the height dimension's extension line", () => {
    const { sheet, state } = built({ name: "", asset: "skateboarder", query: "default", view: "dark-line", scaleIndex: 1 }, "third", "en");
    const k = sheet.metadata.mmPerMetre;
    const xs = (pick: (id: string, layer: string) => boolean): number[] => sheet.layers.flatMap((layer) => layer.items.filter((item) => item.kind === "path" && pick(item.id, layer.id)).flatMap((item) => (item as unknown as { commands: ({ x?: number })[] }).commands.flatMap((c) => (c.x === undefined ? [] : [c.x]))));
    const ground = xs((id) => id === "front:ground:line");
    const figure = xs((id, layer) => id.startsWith("front:") && layer !== "ground" && layer !== "annotations");
    const span = extentOf(state.shapes.front)!;
    near(Math.min(...figure) - Math.min(...ground), 0.3 * k, "left overhang");
    near(Math.max(...ground) - Math.max(...figure), 0.3 * k, "right overhang");
    near(Math.max(...figure) - Math.min(...figure), (span.maxX - span.minX) * k, "figure span against the front shapes");
    const extension = sheet.layers.flatMap((layer) => layer.items).find((item) => item.id === "dimension:height:extension-1");
    expect(extension, "the height dimension has a bottom extension line at ground level").toBeTruthy();
  });

  it("surfaces the typed unavailability of every view instead of a substituted sheet", () => {
    const asset = assets.get("standing-adult")!;
    const attempt = (): unknown => resolveSheet(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", denominator: 5 }, "light-line", optionsOf("light-line", 5, "en"), "third", "en");
    expect(attempt).toThrow(SheetUnavailableError);
    try {
      attempt();
    } catch (error) {
      const unavailable = (error as SheetUnavailableError).unavailable;
      expect(unavailable.map((u) => u.viewpoint).sort()).toEqual(["elevation", "front", "plan"]);
      for (const u of unavailable) expect(u.code).toMatch(/^scale-/u);
    }
  });

  it("writes the same bytes on a rerun, in SVG, PDF and DXF", () => {
    const first = built(CASES[1]!, "third", "en");
    const second = built(CASES[1]!, "third", "en");
    expect(writeSvg(second.sheet)).toBe(writeSvg(first.sheet));
    expect(Buffer.from(writePdf([second.sheet])).equals(Buffer.from(writePdf([first.sheet])))).toBe(true);
    expect(cad(second, "third", "en", "dark-solid")).toBe(cad(first, "third", "en", "dark-solid"));
  });

  it("labels the views, the projection and the dimensions in the language asked for", () => {
    const en = writeSvg(built(CASES[0]!, "first", "en").sheet);
    const de = writeSvg(built(CASES[0]!, "first", "de").sheet);
    const texts = (svg: string): string => [...svg.matchAll(/<text [^>]*>([^<]*)<\/text>/gu)].map((match) => match[1]).join("|");
    expect(texts(en)).toContain("First-angle projection");
    expect(en).toContain("Overall width / Gesamtbreite");
    expect(texts(de)).toContain("Projektionsmethode 1");
    for (const caption of ["Draufsicht", "Vorderansicht", "Seitenansicht"]) {
      expect(texts(de)).toContain(caption);
      expect(texts(en)).not.toContain(caption);
    }
    expect(texts(de)).not.toContain("First-angle");
  });

  it("composes only projections of one state: one evaluate, one query, one mode", () => {
    const { sheet } = built(CASES[0]!, "third", "en");
    expect(sheet.metadata.query).toEqual({ kind: "pose", pose: "pointing" });
    const asset = assets.get("standing-adult")!;
    const multi = resolveViews(asset, { kind: "pose", pose: "pointing" }, { kind: "scale", denominator: 50 }, "light-line", optionsOf("light-line", 50, "en"));
    for (const view of Object.values(multi.views)) expect(view.kind === "drawing" && view.state === multi.state).toBe(true);
  });
});

describe("📑️ export paths name their view", () => {
  const base = (extra: readonly string[]): readonly string[] => ["--asset", "standing-adult", "--pose", "relaxed", "--view", "light-line", "--scale", "100", "--lang", "en", "--out", "scratch", ...extra];
  const quiet: ToolPort = { run: async () => "" };

  it("requires a viewpoint or a projection and refuses a mix", () => {
    expect(() => parseArguments([...base(["--format", "svg"])])).toThrow(/missing --viewpoint/);
    expect(() => parseArguments([...base(["--format", "sheet-svg"])])).toThrow(/missing --projection/);
    expect(() => parseArguments([...base(["--format", "sheet-svg", "--viewpoint", "plan", "--projection", "third"])])).toThrow(/does not apply/);
    expect(() => parseArguments([...base(["--format", "svg", "--viewpoint", "plan", "--projection", "third"])])).toThrow(/sheet formats only/);
    expect(() => parseArguments([...base(["--format", "svg", "--viewpoint", "plan", "--time", "1"])])).toThrow(/needs --clip/);
  });

  it("writes every viewpoint and the sheet to its own file name, none overwriting another", async () => {
    await withScratch(async (directory) => {
      const names: string[] = [];
      const run = async (extra: readonly string[]): Promise<void> => {
        const options = parseArguments(base(extra).map((value) => (value === "scratch" ? directory : value)));
        names.push(...(await runExport(options)).map((path) => path.slice(directory.length + 1)));
      };
      for (const viewpoint of ["elevation", "front", "plan"]) for (const format of ["svg", "dxf"]) await run(["--format", format, "--viewpoint", viewpoint]);
      for (const projection of PROJECTIONS) for (const format of ["sheet-svg", "sheet-dxf"]) await run(["--format", format, "--projection", projection]);
      expect(new Set(names).size).toBe(names.length);
      expect(names.length).toBe(10);
      expect(readdirSync(directory).sort()).toEqual([...names].sort());
      for (const viewpoint of ["elevation", "front", "plan"]) expect(names.filter((name) => name.includes(`-${viewpoint}-`)).length).toBe(2);
      const dxf = readFileSync(join(directory, names.find((name) => name.endsWith(".dxf") && name.includes("-plan-"))!), "utf8");
      expect(dxf).toContain("staffage standing-adult relaxed light-line plan 1:100 physical");
      expect(dxf).toContain("STAFFAGE-NOTE");
    });
  }, 120_000);

  it("exports one instant of a clip with --time and keeps the instant in the name", async () => {
    await withScratch(async (directory) => {
      const options = parseArguments(["--asset", "walking-commuter", "--clip", "walk-cycle", "--time", "0.3", "--view", "light-line", "--viewpoint", "front", "--scale", "100", "--lang", "en", "--format", "svg", "--out", directory]);
      const written = await runExport(options);
      expect(written).toHaveLength(1);
      expect(written[0]).toContain("-t0.3-light-line-front-1to100-");
      expect(readFileSync(written[0]!, "utf8")).toContain(`viewpoint="front"`);
    });
  }, 60_000);

  for (const view of ["forces", "moments"] as const)
    it(`writes the same DXF layers and entities as the browser export in the ${view} mode`, async () => {
      await withScratch(async (directory) => {
        const asset = assets.get("forklift")!;
        const denominator = scaleOf(asset, 0);
        const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator } });
        const browser = writeDxf(modelScene(state, { viewpoint: "elevation", analysis: { structure: solveStructure(state) }, diagrams: ["moment"] }));
        const options = parseArguments(["--asset", "forklift", "--pose", asset.presentation.defaultPose, "--view", view, "--viewpoint", "elevation", "--scale", String(denominator), "--lang", "en", "--format", "dxf", "--out", directory]);
        const cli = readFileSync((await runExport(options))[0]!, "utf8");
        const layers = (text: string): string[] => [...new Set([...text.matchAll(/\n {2}8\n([^\n]+)\n/gu)].map((match) => match[1]!))].filter((name) => name !== "STAFFAGE-NOTE").sort();
        const count = (text: string): number => [...text.matchAll(/\n {2}0\nLWPOLYLINE\n/gu)].length;
        expect(layers(cli)).toEqual(layers(browser));
        expect(count(cli), `layers ${layers(cli).join(",")} against ${layers(browser).join(",")}`).toBe(count(browser));
        expect(layers(cli)).toContain(view === "moments" ? "ANALYSIS-DIAGRAM" : "ANALYSIS-ARROWS");
      });
    }, 120_000);

  it("hands the DWG encoder the viewpoint in the content and the same bytes on a rerun", async () => {
    configureDwg({});
    const previous = process.env["SEMIO_STAFFAGE_DWG_CLI"];
    process.env["SEMIO_STAFFAGE_DWG_CLI"] = "fake-encoder";
    const copy: ToolPort = { run: async (_tool, args) => (copyFileSync(args[0]!, args[1]!), "") };
    try {
      await withScratch(async (directory) => {
        const run = async (name: string): Promise<string> => {
          const options = parseArguments(["--asset", "standing-adult", "--pose", "relaxed", "--view", "light-line", "--viewpoint", "front", "--scale", "100", "--lang", "en", "--format", "dwg", "--out", join(directory, name)]);
          return readFileSync((await runExport(options, copy))[0]!, "utf8");
        };
        const first = await run("a");
        expect(first).toBe(await run("b"));
        const scene = JSON.parse(first) as { layers: { name: string; texts: { text: string }[] }[] };
        expect(scene.layers.find((layer) => layer.name === "STAFFAGE-NOTE")!.texts[0]!.text).toBe("staffage standing-adult relaxed light-line front 1:100 physical");
      });
    } finally {
      if (previous === undefined) delete process.env["SEMIO_STAFFAGE_DWG_CLI"];
      else process.env["SEMIO_STAFFAGE_DWG_CLI"] = previous;
      configureDwg({});
    }
  }, 60_000);

  it("names the viewpoint of an MP4 in its file name and its container comment", async () => {
    await withScratch(async (directory) => {
      const calls: { tool: string; args: readonly string[] }[] = [];
      const record: ToolPort = { run: async (tool, args) => (calls.push({ tool, args }), "") };
      const options = parseArguments(["--asset", "standing-adult", "--pose", "relaxed", "--view", "light-line", "--viewpoint", "plan", "--scale", "100", "--lang", "en", "--format", "mp4", "--out", directory]);
      const written = await runExport(options, record);
      expect(written[0]).toContain("-light-line-plan-1to100.mp4");
      const ffmpeg = calls.find((call) => call.tool === "ffmpeg")!;
      expect(ffmpeg.args.join(" ")).toContain("comment=staffage standing-adult relaxed light-line plan 1:100 physical");
    });
  }, 60_000);

  it("names the viewpoint of a browser clip and keeps the first-party MP4 and the PNG ZIP byte-deterministic", async () => {
    expect(videoFileName("standing-adult", "walk", "light-line", "plan", 100, "mp4")).toBe("standing-adult-walk-light-line-plan-1to100.mp4");
    const frame = (value: number): VideoFrame => ({ data: new Uint8ClampedArray(Array.from({ length: 16 * 16 * 4 }, (_, at) => (at % 4 === 3 ? 255 : (value + at) % 256))), width: 16, height: 16 });
    const clip = async () => encodeClip((async function* () { for (const value of [1, 1, 9]) yield frame(value); })(), "mp4", { viewpoint: "front", fps: 10, frameCount: 3 });
    const [a, b] = [await clip(), await clip()];
    expect(a.viewpoint).toBe("front");
    expect(Buffer.from(a.bytes).equals(Buffer.from(b.bytes))).toBe(true);
    const schedule = frameSchedule({ durationS: 0.3, loop: false }, 10);
    const input = { asset: "standing-adult", clip: "relaxed", view: "light-line", viewpoint: "plan", scaleDenominator: 100, widthPx: 1, heightPx: 1, transparent: true } as const;
    const zip = async () => writePngSequenceZip(schedule, input, (async function* () { for (let i = 0; i < schedule.count; i++) yield new Uint8Array([137, 80, 78, 71, i]); })());
    const [x, y] = [await zip(), await zip()];
    expect(Buffer.from(x).equals(Buffer.from(y))).toBe(true);
    expect(new TextDecoder().decode(x)).toContain(`"viewpoint": "plan"`);
    expect(new TextDecoder().decode(x)).toContain(frameName(0));
  });

  it("draws the model space of each viewpoint at metres with the viewpoint's own coordinates", () => {
    const asset = assets.get("standing-adult")!;
    const state = evaluate(asset, { kind: "pose", pose: "relaxed" }, { kind: "scale", scale: { denominator: 100 } });
    const extent = (viewpoint: Viewpoint) => {
      const points = modelScene(state, { viewpoint }).layers.filter((layer) => layer.name !== "GROUND").flatMap((layer) => layer.polylines.flatMap((line) => line.points));
      return { x: Math.max(...points.map((p) => p.x)) - Math.min(...points.map((p) => p.x)), y: Math.max(...points.map((p) => p.y)) - Math.min(...points.map((p) => p.y)) };
    };
    near(extent("elevation").y, extent("front").y, "elevation height against front height");
    near(extent("front").x, extent("plan").y, "front depth against plan depth");
    near(extent("elevation").x, extent("plan").x, "elevation width against plan width");
  });
});
