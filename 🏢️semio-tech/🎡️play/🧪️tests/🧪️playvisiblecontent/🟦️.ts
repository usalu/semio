import { describe, expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { PNG } from "pngjs";
import { CODE_ASSET_PATH, FRAMEWORK_FAULT_SELECTOR, MEDIA_ASSET_PATH, distinctInteriorColors, spaFallbackAnswer } from "../🎭️acceptance/🖼️visible-content/🟦️.ts";

const fixture = JSON.parse(readFileSync(join(import.meta.dir, "🔣️.json"), "utf8")) as {
  fallback: { url: string; type: string; status: number; release: boolean; answer: string | null }[];
  canvases: { name: string; width: number; height: number; pixels: "flat" | "stripes" | "border"; colors: number }[];
};

describe("play visible-content helpers", () => {
  for (const row of fixture.fallback) test(`classifies ${row.url} as ${row.type} (release ${row.release})`, () => {
    expect(spaFallbackAnswer(row.url, row.type, row.status, row.release ? [MEDIA_ASSET_PATH, CODE_ASSET_PATH] : [MEDIA_ASSET_PATH])).toBe(row.answer ?? undefined);
  });
  for (const row of fixture.canvases) test(`census of ${row.name} canvas`, () => {
    const png = new PNG({ width: row.width, height: row.height });
    for (let y = 0; y < row.height; y++) for (let x = 0; x < row.width; x++) {
      const edge = x === 0 || y === 0 || x === row.width - 1 || y === row.height - 1;
      const value = row.pixels === "stripes" ? (x % 4 < 2 ? 0 : 255) : row.pixels === "border" ? (edge ? 255 : 0) : 7;
      png.data.set([value, value, value, 255], (y * row.width + x) * 4);
    }
    const encoded = PNG.sync.write(png);
    expect(distinctInteriorColors(encoded)).toBe(row.colors);
    const oracle = new Set<number>();
    const decoded = PNG.sync.read(encoded), inset = Math.max(1, Math.floor(Math.min(row.width, row.height) / 4));
    for (let y = inset; y < row.height - inset; y += 2) for (let x = inset; x < row.width - inset; x += 2) oracle.add(decoded.data[(y * row.width + x) * 4]!);
    expect(oracle.size).toBe(row.colors);
  });
  test("lists the framework fault markers in the UI sources", () => {
    const root = join(import.meta.dir, "..", "..", "..", "..", "🧰️framework");
    const sources: Record<string, string> = {
      "data-semio-window-fault": "🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🎚️shell-model/🟦️.tsx",
      "data-shell-fault-boundary": "🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧯️fault-boundary/🟦️.tsx",
      "data-layered-pane-error": "🔨️modules/🖱️ui/🧱️elements/🥞️LayeredOverview/🧵️pane/🟦️.tsx",
      "data-slot=\"error-view\"": "🔨️modules/🖱️ui/🎯️targets/⚛️react/🚦️status-surface/🟦️.tsx",
    };
    for (const [marker, file] of Object.entries(sources)) {
      expect(readFileSync(join(root, file), "utf8")).toContain(marker);
      expect(FRAMEWORK_FAULT_SELECTOR).toContain(marker);
    }
  });
});
