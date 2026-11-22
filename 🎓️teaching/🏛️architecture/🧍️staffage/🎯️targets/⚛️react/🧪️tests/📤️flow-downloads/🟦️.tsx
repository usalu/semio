/** 📤️ Browser downloads with the air flow: the request (layers, environment, conditions, mark detail) decides what SVG, PDF, DXF, DWG, PNG sequence and video carry; a pose plays its flow on one canvas; the development reports progress and can be cancelled. ZIP read back with fflate, PDF with pdfjs-dist.
 * @see ./🥒️.feature — the scenarios */
import { unzipSync } from "fflate";
import { PNG } from "pngjs";
import { describe, expect, it } from "vitest";
import { exportFigure } from "@teaching/staffage-react";
import type { ExportHost } from "@teaching/staffage-react";
import { FLOW_LAYERS, VIEWS, flowDetailOf, flowMarks, transportOf, type ModelScene, type Text } from "@teaching/staffage";
import { POSE_PLAY_S } from "../../🔨️modules/📤️downloads/🟦️.ts";
import { depictionOptions } from "../../🔨️modules/🧮️pipeline/🟦️.ts";
import { walker } from "../🧰️support/🧰️.ts";

const thermal = { view: "thermal", viewpoint: "elevation", scale: { denominator: 100 }, language: "en", flow: FLOW_LAYERS } as const;
const stand = { kind: "pose", pose: "stand" } as const;
const text = (bytes: Uint8Array): string => new TextDecoder().decode(bytes);
const ids = (markup: string, prefix: string): string[] => [...markup.matchAll(/\bid="([^"]+)"/gu)].map((match) => match[1]!).filter((id) => id.startsWith(prefix));
const wording = (markup: string): string => markup.replace(/<[^>]+>/gu, " ").replace(/\s+/gu, " ");
const rootOf = (markup: string): string => /<svg\b[^>]*>/u.exec(markup)![0];
const KINDS = { streams: ["flow-stream-", "flow-arrow-"], tracers: ["flow-tracer-"], moisture: ["flow-vapour-"], fog: ["flow-fog-"], pathlines: ["flow-path-"] } as const;

const onePixel = (() => {
  const png = new PNG({ width: 2, height: 2 });
  png.data.fill(0);
  return new Uint8Array(PNG.sync.write(png));
})();

async function svgOf(request: Parameters<typeof exportFigure>[2]): Promise<string> {
  const result = await exportFigure(walker, stand, request, "svg");
  if (result.status !== "done") throw new Error(`svg ${result.status}`);
  return text(result.bytes);
}

describe("📤️ downloads with air flow", { timeout: 600_000 }, () => {
  it("draws in the SVG exactly the layers the request names, and none outside the thermal view", async () => {
    const all = await svgOf(thermal);
    for (const prefix of [...KINDS.streams, ...KINDS.tracers, ...KINDS.moisture, ...KINDS.pathlines]) expect(ids(all, prefix).length, prefix).toBeGreaterThan(0);
    const streams = await svgOf({ ...thermal, flow: ["streams"] });
    expect(ids(streams, "flow-stream-").length).toBe(ids(all, "flow-stream-").length);
    expect(ids(streams, "flow-").every((id) => KINDS.streams.some((prefix) => id.startsWith(prefix)))).toBe(true);
    expect(ids(await svgOf({ ...thermal, flow: [] }), "flow-")).toEqual([]);
    expect(ids(await svgOf({ view: thermal.view, viewpoint: thermal.viewpoint, scale: thermal.scale, language: thermal.language }), "flow-")).toEqual([]);
    expect(ids(await svgOf({ ...thermal, view: "dark-line" }), "flow-")).toEqual([]);
  });

  it("solves and draws the flow under the environment, conditions and mark detail of the request, as the library does for the screen", async () => {
    const request = { ...thermal, environment: { ...walker.thermal.environment, airTempC: 5, meanRadiantTempC: 5, windMS: 1.5 }, flowConditions: { maxAgeS: 8, seed: 3 }, flowDetail: { spacingMm: 6 } } as const;
    const markup = await svgOf(request);
    expect(wording(markup)).toContain("Ambient air 5.0 °C");
    expect(wording(markup)).toContain("air speed 1.50 m/s");
    expect(wording(markup)).toContain("field at 8.00 s");
    const field = transportOf({ asset: walker, query: stand }, depictionOptions(request)).flow!;
    expect(field.ambient).toMatchObject({ airTempC: 5, windMS: 1.5 });
    const marks = flowMarks(field, field.timeS, flowDetailOf(request.scale.denominator, request.flowDetail));
    const bright = marks.streaks.filter((streak) => streak.opacity > 0.02).length;
    const streams = (svg: string): Set<string> => new Set(ids(svg, "flow-stream-").map((id) => id.split("-")[2]!));
    expect(ids(markup, "flow-tracer-head-").length).toBeLessThanOrEqual(bright);
    expect(ids(markup, "flow-tracer-head-").length).toBeGreaterThan(0.6 * bright);
    expect([...streams(markup)].every((index) => marks.ribbons.some((ribbon) => String(ribbon.stream) === index))).toBe(true);
    expect(streams(markup).size).toBeGreaterThan(0.6 * marks.ribbons.length);
    expect(streams(markup).size).toBeLessThan(streams(await svgOf({ ...request, flowDetail: {} })).size);
  });

  it("puts the flow on the thermal page of the all-views PDF and on no other", async () => {
    const result = await exportFigure(walker, stand, { ...thermal, sheet: "all-views" }, "pdf");
    if (result.status !== "done") throw new Error("pdf not written");
    const pdfjs = await import("pdfjs-dist/legacy/build/pdf.mjs");
    const document = await pdfjs.getDocument({ data: result.bytes.slice() }).promise;
    expect(document.numPages).toBe(VIEWS.length);
    const pages: number[] = [];
    for (let number = 1; number <= document.numPages; number++) {
      const content = await (await document.getPage(number)).getTextContent();
      if (content.items.map((item) => ("str" in item ? item.str : "")).join(" ").includes("Air temperature excess of the flow")) pages.push(number);
    }
    expect(pages).toEqual([VIEWS.indexOf("thermal") + 1]);
  });

  it("names the flow layers in the DXF and hands them to the DWG encoder as polylines in metres", async () => {
    const dxf = await exportFigure(walker, stand, thermal, "dxf");
    if (dxf.status !== "done") throw new Error("dxf not written");
    const layers = new Set([...text(dxf.bytes).matchAll(/\n  8\n(FLOW-[A-Z-]+)\n/gu)].map((match) => match[1]!));
    for (const name of ["FLOW-STREAMS-NEUTRAL", "FLOW-ARROWS-NEUTRAL", "FLOW-TRACERS-NEUTRAL", "FLOW-VAPOUR"]) expect(layers, name).toContain(name);
    let scene: ModelScene | undefined;
    const host: ExportHost = { dwg: async (model) => { scene = model; return Uint8Array.from([0x41, 0x43, 0x31, 0x30, 0x32, 0x34]); } };
    expect((await exportFigure(walker, stand, thermal, "dwg", undefined, host)).status).toBe("done");
    const flow = scene!.layers.filter((layer) => layer.name.startsWith("FLOW-"));
    expect(new Set(flow.map((layer) => layer.name))).toEqual(layers);
    const height = walker.dimensions.heightM;
    for (const layer of flow) for (const line of layer.polylines) for (const point of line.points) expect(Math.abs(point.x) < 10 * height && point.y > -0.1 && point.y < 10 * height, `${layer.name} ${point.x}, ${point.y}`).toBe(true);
    const bare = await exportFigure(walker, stand, { ...thermal, flow: [] }, "dxf");
    if (bare.status !== "done") throw new Error("dxf not written");
    expect(text(bare.bytes)).not.toContain("FLOW-");
    expect((await exportFigure(walker, stand, { ...thermal, flow: [] }, "dwg", undefined, host)).status).toBe("done");
    expect(scene!.layers.some((layer) => layer.name.startsWith("FLOW-"))).toBe(false);
  });

  it("plays the flow of a pose as frames on one canvas: the field ages by the frame time, the figure stands", async () => {
    const frames: string[] = [];
    const host: ExportHost = { rasterise: async (svg) => { frames.push(svg); return onePixel; } };
    const result = await exportFigure(walker, stand, { ...thermal, fps: 1 }, "png-sequence", undefined, host);
    if (result.status !== "done") throw new Error(`sequence ${result.status}`);
    expect(result.details.frames).toBe(POSE_PLAY_S);
    expect(Object.keys(unzipSync(result.bytes)).filter((name) => name.endsWith(".png"))).toHaveLength(POSE_PLAY_S);
    expect(frames).toHaveLength(POSE_PLAY_S);
    const size = (svg: string): string => `${/\swidth="([^"]+)"/u.exec(rootOf(svg))![1]} ${/\sheight="([^"]+)"/u.exec(rootOf(svg))![1]}`;
    expect(new Set(frames.map(size)).size).toBe(1);
    const window = (svg: string): number[] => /\sviewBox="([^"]+)"/u.exec(rootOf(svg))![1]!.split(" ").map(Number);
    for (const svg of frames) expect(window(svg).slice(2).map((value) => value.toFixed(6))).toEqual(window(frames[0]!).slice(2).map((value) => value.toFixed(6)));
    frames.forEach((svg, n) => expect(wording(svg), `frame ${n}`).toContain(`field at ${(14 + n).toFixed(2)} s`));
    const figure = (svg: string): string[] => [...svg.matchAll(/\sid="((?:shape|outline)[^"]*)"/gu)].map((match) => match[1]!);
    const tracers = (svg: string): string => [...svg.matchAll(/<path id="flow-tracer-\d+"[^>]*\sd="([^"]+)"/gu)].map((match) => match[1]).join("|");
    expect(figure(frames[0]!).length).toBeGreaterThan(3);
    for (const svg of frames.slice(1)) {
      expect(figure(svg)).toEqual(figure(frames[0]!));
      expect(tracers(svg)).not.toBe(tracers(frames[0]!));
    }
    const still = await exportFigure(walker, stand, { ...thermal, flow: [] }, "png-sequence", undefined, host);
    expect(still.status).toBe("unavailable");
    let recorded = 0;
    const video = await exportFigure(walker, stand, { ...thermal, fps: 2, durationS: 1 }, "mp4", undefined, { ...host, record: async (source) => { for await (const _frame of source) recorded++; return { bytes: Uint8Array.from([0, 0, 0, 24]), mime: "video/mp4" }; } });
    expect(video.status).toBe("done");
    expect(recorded).toBe(2);
  });

  it("reports the development of the flow in both languages and ends cancelled when aborted during it", async () => {
    const labels: Text[] = [];
    const request = { ...thermal, flowConditions: { seed: 11 } } as const;
    expect((await exportFigure(walker, stand, request, "svg", { progress: (_done, _total, label) => labels.push(label) })).status).toBe("done");
    expect(labels.some((label) => label.en === "Developing the air flow" && label.de === "Luftströmung wird entwickelt")).toBe(true);
    const controller = new AbortController();
    let ticks = 0;
    const cancelled = await exportFigure(walker, stand, { ...thermal, flowConditions: { seed: 12 } }, "svg", { signal: controller.signal, progress: () => { if (++ticks === 3) controller.abort(); } });
    expect(cancelled.status).toBe("cancelled");
    expect(ticks).toBeLessThan(20);
  });
});
