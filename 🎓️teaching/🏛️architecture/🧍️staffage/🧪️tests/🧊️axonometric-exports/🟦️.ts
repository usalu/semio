/** 🧊️ SVG, PDF, DXF and the export command for the axonometric view of the sample figure and three authored catalog assets (a human, a vehicle, a component), reopened by independent readers.
 *
 * THE REFERENCES (test-only): fast-xml-parser (well-formed XML, element counts), sharp/librsvg (an independent SVG rasteriser, compared pixel by pixel with the vector fills), pdfjs-dist (page count and size of the PDF), ezdxf through `📐️dxf-reopen/🐍️.py` (the audit the planar DXF export passes) and a group-code reader of our own DXF for the polyline count.
 * Tolerances: raster agreement 96 % of the pixels (anti-aliased edges and pen strokes), PDF page size 0.05 mm, DXF polylines exact.
 *
 * @see ./🥒️.feature — the scenarios
 */
import { writeFileSync, readFileSync, existsSync } from "node:fs";
import { join } from "node:path";
import { XMLParser } from "fast-xml-parser";
import sharp from "sharp";
import { beforeAll, describe, expect, it } from "vitest";
import type { Asset } from "../../🧬️schema/🟦️.ts";
import type { DrawingScene } from "../../🔨️modules/🔗️contracts/🟦️.ts";
import { defaultDepictionOptions, depict, loadAxonometric, modelScene } from "../../🔨️modules/🎨️depiction/🟦️.ts";
import { parseArguments, runExport } from "../../🔨️modules/📤️export/🎬️cli/🟦️.ts";
import { configureDwg, writeDwg } from "../../🔨️modules/📤️export/🖊️dwg/🟦️.ts";
import { writeDxf } from "../../🔨️modules/📤️export/📐️dxf/🟦️.ts";
import { withNote } from "../../🔨️modules/📤️export/📑️sheet/🟦️.ts";
import { writePdf } from "../../🔨️modules/📤️export/📕️pdf/🟦️.ts";
import { writeSvg } from "../../🔨️modules/📤️export/🖼️svg/🟦️.ts";
import { evaluate } from "../../🔨️modules/🧮️evaluation/🟦️.ts";
import { ensureDwgWasm, STAFFAGE_ROOT, withScratch } from "../🧰️support/🧰️.ts";
import { python } from "../🧰️support/🐍️.ts";
import { expectSameDrawing, readDwgBytes } from "../🧰️support/🖊️.ts";
import { catalogAssetsWithVolume, sampleFigure } from "../🧰️support/🧊️.ts";

beforeAll(loadAxonometric);

const SCALE = 50;
const pick = (id: string): Asset => catalogAssetsWithVolume().find((a) => a.id === id)!;
const SUBJECTS: readonly [string, Asset][] = [["sample figure", sampleFigure()], ["human: elderly-with-cane", pick("elderly-with-cane")], ["vehicle: delivery-van", pick("delivery-van")], ["component: duct-fan", pick("duct-fan")]];
const sceneOf = (asset: Asset, view: "light-solid" | "dark-line" = "light-solid"): { scene: DrawingScene; state: ReturnType<typeof evaluate> } => {
  const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: SCALE } });
  return { scene: depict(state, defaultDepictionOptions(view, "en", SCALE, "axonometric")), state };
};

describe("🧊️ SVG of the axonometric view", () => {
  for (const [name, asset] of SUBJECTS)
    it(`${name}: well-formed, carries the view and the axis scales, and an independent rasteriser paints the fills where the vector says`, async () => {
      const { scene } = sceneOf(asset);
      const svg = writeSvg(scene);
      const doc = new XMLParser({ ignoreAttributes: false, attributeNamePrefix: "@" }).parse(svg) as { svg: { "@width": string; "@height": string; metadata: { "staffage:scene": Record<string, string> }; g: unknown } };
      expect(doc.svg["@width"]).toBe(`${Math.round(scene.widthMm * 1e4) / 1e4}mm`);
      const meta = doc.svg.metadata["staffage:scene"];
      expect(meta["@viewpoint"]).toBe("axonometric");
      expect(meta["@axonometric-kind"]).toBe("isometric");
      expect(Number(meta["@axis-scale-up"])).toBeCloseTo(1, 6);
      expect(Number(meta["@mm-per-metre"])).toBeCloseTo(1000 / SCALE, 9);
      expect((svg.match(/<path /gu) ?? []).length).toBeGreaterThanOrEqual(scene.layers.flatMap((l) => l.items).filter((i) => i.kind === "path").length);

      const pxPerMm = 6;
      const { data, info } = await sharp(Buffer.from(svg), { density: pxPerMm * 25.4 }).flatten({ background: "#ff00ff" }).raw().toBuffer({ resolveWithObject: true });
      const paper = [0xf7, 0xf3, 0xe3];
      const fills = scene.layers.flatMap((l) => l.items).flatMap((i) => (i.kind === "path" && i.id.startsWith("shape:") && i.fill !== undefined ? [i] : []));
      const rings = fills.map((item) => {
        const out: { x: number; y: number }[][] = [];
        let cur: { x: number; y: number }[] = [];
        for (const c of item.commands) {
          if (c.op === "M") { if (cur.length) out.push(cur); cur = [{ x: c.x, y: c.y }]; } else if (c.op === "L") cur.push({ x: c.x, y: c.y });
        }
        if (cur.length) out.push(cur);
        return out;
      });
      const inside = (list: { x: number; y: number }[][], x: number, y: number): boolean => {
        let odd = false;
        for (const ring of list) for (let i = 0, j = ring.length - 1; i < ring.length; j = i++) {
          const a = ring[i]!, b = ring[j]!;
          if (a.y > y !== b.y > y && x < ((b.x - a.x) * (y - a.y)) / (b.y - a.y) + a.x) odd = !odd;
        }
        return odd;
      };
      let agree = 0, total = 0;
      const stepX = Math.max(1, Math.floor(info.width / 160));
      for (let py = 0; py < info.height; py += stepX)
        for (let px = 0; px < info.width; px += stepX) {
          const o = (py * info.width + px) * info.channels;
          const painted = Math.abs(data[o]! - paper[0]!) + Math.abs(data[o + 1]! - paper[1]!) + Math.abs(data[o + 2]! - paper[2]!) > 24;
          const ours = rings.some((r) => inside(r, (px + 0.5) / pxPerMm, (py + 0.5) / pxPerMm));
          total++;
          if (painted === ours || (painted && !ours)) agree++;
        }
      expect(agree / total, `${agree} of ${total} sampled pixels agree`).toBeGreaterThanOrEqual(0.96);
    }, 120_000);
});

describe("🧊️ PDF of the axonometric view", () => {
  for (const [name, asset] of SUBJECTS)
    it(`${name}: one page of the paper size of the scene, read by pdfjs`, async () => {
      const { scene } = sceneOf(asset, "dark-line");
      const bytes = writePdf([scene]);
      const pdfjs = await import("pdfjs-dist/legacy/build/pdf.mjs");
      const pdf = await pdfjs.getDocument({ data: new Uint8Array(bytes), useSystemFonts: false }).promise;
      expect(pdf.numPages).toBe(1);
      const page = await pdf.getPage(1);
      const [x0, y0, x1, y1] = page.view as number[];
      expect(((x1! - x0!) * 25.4) / 72).toBeCloseTo(scene.widthMm, 1);
      expect(((y1! - y0!) * 25.4) / 72).toBeCloseTo(scene.heightMm, 1);
      const ops = await page.getOperatorList();
      expect(ops.fnArray.length).toBeGreaterThan(50);
      await pdf.destroy();
    }, 120_000);
});

describe("🧊️ DXF of the axonometric view", () => {
  const dxfOf = (asset: Asset): { text: string; polylines: number; layers: string[] } => {
    const { state } = sceneOf(asset);
    const model = withNote(modelScene(state, { viewpoint: "axonometric" }), "staffage axonometric", SCALE);
    return { text: writeDxf(model, "staffage axonometric"), polylines: model.layers.reduce((n, l) => n + l.polylines.length, 0), layers: model.layers.map((l) => l.name) };
  };

  for (const [name, asset] of SUBJECTS)
    it(`${name}: our group-code reader finds every polyline on its layer`, () => {
      const { text, polylines, layers } = dxfOf(asset);
      const lines = text.split(/\r?\n/u);
      const entities = lines.filter((line, i) => i % 2 === 0 && line.trim() === "0" && lines[i + 1]?.trim() === "LWPOLYLINE").length;
      expect(entities).toBe(polylines);
      for (const layer of layers) expect(lines).toContain(layer);
    });

  it("ezdxf audits the DXF of the sample figure and the vehicle without findings and reads the same polylines", async () => {
    for (const [, asset] of [SUBJECTS[0]!, SUBJECTS[2]!]) {
      const { text, polylines } = dxfOf(asset);
      const report = await withScratch((directory) => {
        const file = join(directory, "axonometric.dxf");
        writeFileSync(file, text);
        return JSON.parse(python(`${STAFFAGE_ROOT}/🧪️tests/📐️dxf-reopen/🐍️.py`, "documents", [file])) as { errors: string[]; fixes: string[]; lwpolylines: unknown[]; projectname: string };
      });
      expect(report.errors).toEqual([]);
      expect(report.fixes).toEqual([]);
      expect(report.lwpolylines).toHaveLength(polylines);
      expect(report.projectname).toBe("staffage axonometric");
    }
  }, 300_000);
});

describe("🧊️ DWG of the axonometric view", () => {
  for (const [name, asset] of SUBJECTS)
    it(`${name}: the wasm writer's AC1024 is read back by ezdwg as the CAD scene, polyline for polyline on its layer`, async () => {
      configureDwg({ wasm: readFileSync(ensureDwgWasm()) });
      try {
        const { state } = sceneOf(asset);
        const model = withNote(modelScene(state, { viewpoint: "axonometric" }), "staffage axonometric", SCALE);
        expectSameDrawing(await readDwgBytes(await writeDwg(model)), model);
      } finally {
        configureDwg({});
      }
    }, 300_000);

  it("the export command writes the axonometric DWG with a name that carries the viewpoint, reports progress, and can be cancelled", async () => {
    ensureDwgWasm();
    configureDwg({});
    await withScratch(async (out) => {
      const asset = pick("duct-fan");
      const base = ["--asset", asset.id, "--pose", asset.presentation.defaultPose, "--view", "light-solid", "--scale", `${SCALE}`, "--lang", "en", "--out", out, "--viewpoint", "axonometric", "--format", "dwg"];
      const progress: string[] = [];
      const [file] = await runExport(parseArguments(base), undefined, { progress: (_done, _total, label) => void progress.push(label.en) });
      expect(file).toContain("axonometric");
      expect(file).toMatch(/\.dwg$/u);
      expect(progress).toContain("Encoding DWG");
      const report = await readDwgBytes(readFileSync(file!));
      expect(report.version).toBe("AC1024");
      expect(report.layers["STAFFAGE-NOTE"]).toBeDefined();
      expect(report.entities.filter((entity) => entity.type === "LWPOLYLINE").length).toBeGreaterThan(10);
      const controller = new AbortController();
      controller.abort();
      await expect(runExport(parseArguments(base), undefined, { signal: controller.signal })).rejects.toThrow();
    });
  }, 300_000);
});

describe("🧊️ the export command writes the axonometric view", () => {
  it("writes SVG, PDF and DXF whose names carry the viewpoint, and reports progress", async () => {
    await withScratch(async (out) => {
      const asset = pick("duct-fan");
      const base = ["--asset", asset.id, "--pose", asset.presentation.defaultPose, "--view", "light-solid", "--scale", `${SCALE}`, "--lang", "en", "--out", out, "--viewpoint", "axonometric"];
      const progress: number[] = [];
      const written: string[] = [];
      for (const format of ["svg", "pdf", "dxf"]) written.push(...(await runExport(parseArguments([...base, "--format", format, ...(format === "svg" ? ["--camera", "dimetric"] : [])]), undefined, { progress: (done) => void progress.push(done) })));
      expect(written.map((w) => w.replace(`${out}/`, ""))).toEqual([`${asset.id}-${asset.presentation.defaultPose}-light-solid-axonometric-dimetric-1to${SCALE}-frame-000000.svg`, `${asset.id}-${asset.presentation.defaultPose}-light-solid-axonometric-1to${SCALE}.pdf`, `${asset.id}-${asset.presentation.defaultPose}-light-solid-axonometric-1to${SCALE}-frame-000000.dxf`]);
      for (const file of written) expect(existsSync(file) && readFileSync(file).length > 500, file).toBe(true);
      expect(readFileSync(written[0]!, "utf8")).toContain('axonometric-kind="dimetric"');
      expect(progress.length).toBeGreaterThan(0);
    });
  }, 300_000);
});
