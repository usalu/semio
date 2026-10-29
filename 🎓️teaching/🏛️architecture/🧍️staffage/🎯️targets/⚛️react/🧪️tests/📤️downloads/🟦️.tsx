/** 📤️ Browser export actions: signatures, metadata, filenames, sheets, honest availability, progress and cancellation; ZIP read back with fflate, PDF with pdfjs-dist, PNG with pngjs.
 * @see ./🥒️.feature — the scenarios */
import { unzipSync } from "fflate";
import { PNG } from "pngjs";
import { describe, expect, it } from "vitest";
import { exportCapabilities, exportFigure, exportFilename } from "@teaching/staffage-react";
import type { ExportHost } from "@teaching/staffage-react";
import { walker } from "../🧰️support/🧰️.ts";

const options = { view: "dark-line", scale: { denominator: 100 }, language: "en" } as const;
const stand = { kind: "pose", pose: "stand" } as const;
const text = (bytes: Uint8Array) => new TextDecoder().decode(bytes);
const ascii = (bytes: Uint8Array, n: number) => String.fromCharCode(...bytes.subarray(0, n));

const onePixel = (() => {
  const png = new PNG({ width: 2, height: 2 });
  png.data.fill(0);
  return new Uint8Array(PNG.sync.write(png));
})();

const host: ExportHost = { rasterise: async () => onePixel };

describe("📤️ downloads", () => {
  it("writes SVG with metadata and the stated filename", async () => {
    const result = await exportFigure(walker, stand, options, "svg");
    expect(result.status).toBe("done");
    if (result.status !== "done") return;
    const markup = text(result.bytes);
    expect(markup).toMatch(/^(<\?xml[^>]*>\s*)?<svg\b/u);
    expect(markup).toContain("reference-walker");
    expect(markup).toContain("<metadata");
    expect(result.filename).toBe("reference-walker_stand_dark-line_1-100.svg");
    expect(result.mime).toBe("image/svg+xml");
  });

  it("writes a one-page PDF and an all-views sheet of seven pages with info metadata", async () => {
    const single = await exportFigure(walker, stand, options, "pdf");
    const sheet = await exportFigure(walker, stand, { ...options, sheet: "all-views" }, "pdf");
    if (single.status !== "done" || sheet.status !== "done") throw new Error("pdf not written");
    expect(ascii(single.bytes, 8)).toBe("%PDF-1.7");
    expect(single.details.pages).toBe(1);
    expect(sheet.details.pages).toBe(7);
    expect(sheet.filename).toBe("reference-walker_stand_all-views_1-100.pdf");
    expect(text(sheet.bytes)).toContain("reference-walker");
    const pdfjs = await import("pdfjs-dist/legacy/build/pdf.mjs");
    const document = await pdfjs.getDocument({ data: sheet.bytes.slice() }).promise;
    expect(document.numPages).toBe(7);
  });

  it("writes DXF with comment metadata and an AC1015 header", async () => {
    const result = await exportFigure(walker, stand, options, "dxf");
    if (result.status !== "done") throw new Error("dxf not written");
    const dxf = text(result.bytes);
    expect(dxf.startsWith("999\n")).toBe(true);
    expect(dxf).toContain("reference-walker");
    expect(dxf).toContain("AC1015");
    expect(dxf.trimEnd().endsWith("EOF")).toBe(true);
  });

  it("reports DWG genuinely or as unavailable, never as a mislabelled file", async () => {
    const result = await exportFigure(walker, stand, options, "dwg");
    if (result.status === "done") expect(ascii(result.bytes, 6)).toBe("AC1024");
    else {
      expect(result.status).toBe("unavailable");
      if (result.status === "unavailable") expect(result.reason.en.length * result.reason.de.length).toBeGreaterThan(0);
    }
    const fake = await exportFigure(walker, stand, options, "dwg", undefined, { dwg: async () => Uint8Array.from([0x41, 0x43, 0x31, 0x30, 0x32, 0x34]) });
    expect(fake.status).toBe("done");
  });

  it("zips a PNG sequence with one frame per tick, text metadata and metadata.json", async () => {
    const progress: number[] = [];
    const result = await exportFigure(walker, { kind: "clip", clip: "walk", timeS: 0 }, { ...options, fps: 10 }, "png-sequence", { progress: (done) => progress.push(done) }, host);
    if (result.status !== "done") throw new Error("zip not written");
    expect(ascii(result.bytes, 4)).toBe("PK\u0003\u0004");
    const files = unzipSync(result.bytes);
    const names = Object.keys(files).sort();
    expect(names.filter((name) => name.endsWith(".png"))).toHaveLength(10);
    expect(names).toContain("metadata.json");
    expect(JSON.parse(text(files["metadata.json"]!)).fps).toBe(10);
    const first = files[names.find((name) => name.endsWith(".png"))!]!;
    expect(PNG.sync.read(Buffer.from(first)).width).toBe(2);
    expect(text(first)).toContain("reference-walker");
    expect(progress.at(-1)).toBe(10);
    expect(result.filename).toBe("reference-walker_walk_dark-line_1-100.zip");
  });

  it("rejects poses for frame sequences and honours cancellation", async () => {
    const pose = await exportFigure(walker, stand, options, "png-sequence", undefined, host);
    expect(pose.status).toBe("unavailable");
    const controller = new AbortController();
    controller.abort();
    expect((await exportFigure(walker, stand, options, "svg", { signal: controller.signal })).status).toBe("cancelled");
  });

  it("names files like asset_pose_view_1-denominator", () => {
    expect(exportFilename(walker, { kind: "clip", clip: "walk", timeS: 0.5 }, { ...options, scale: { denominator: 50 } }, "svg")).toBe("reference-walker_walk-t0p50_dark-line_1-50.svg");
  });

  it("probes real support: no canvas or recorder in jsdom, so videos are unavailable with reasons", async () => {
    const capabilities = await exportCapabilities();
    expect(capabilities.svg.available && capabilities.pdf.available && capabilities.dxf.available).toBe(true);
    for (const format of ["png-sequence", "mp4", "webm"] as const) {
      expect(capabilities[format].available).toBe(false);
      expect(capabilities[format].reason?.de).toBeTruthy();
    }
  });
});
