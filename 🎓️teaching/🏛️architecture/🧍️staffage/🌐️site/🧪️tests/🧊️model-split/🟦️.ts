/** 🧊️ The 3D code is split from the 2D path: the release build puts the viewer, the stage, the volume module, the kernel (schema, tessellation, validation), the axonometric depiction and the GLB, FBX and bundle encoders into chunks of their own that the entry loads only on demand, and the entry carries none of them.
 * @see ./🥒️.feature — the scenarios */
import { mkdtempSync, readFileSync, readdirSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { join, resolve } from "node:path";
import { build } from "vite";
import { expect, it } from "vitest";
import { repositoryCachePartitionRootDataV1 } from "../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📂️paths/🟦️.ts";
import { staffageModelPipeline } from "../../🏗️builder/🌐️vite/🟦️.ts";

const config = resolve(import.meta.dirname, "../../🏗️builder/🌐️vite/🟦️.ts");

it("classifies the model pipeline modules and nothing else as free of import-time side effects", () => {
  const root = "/repo/🎓️teaching/🏛️architecture/🧍️staffage";
  for (const path of ["🔨️modules/🧊️volume/🟦️.ts", "🔨️modules/📤️export/🧊️glb/🟦️.ts", "🔨️modules/📤️export/🎬️fbx/🟦️.ts", "🔨️modules/📤️export/🗿️model/🟦️.ts", "🔨️modules/📤️export/🗜️zip/📦️models/🟦️.ts"]) expect(staffageModelPipeline(`${root}/${path}`)).toBe(true);
  expect(staffageModelPipeline("/repo/🧰️framework/🔨️modules/🧊️3d/🗿️model/🖥️viewer/🟦️.ts")).toBe(true);
  for (const path of ["🔨️modules/📤️export/🖼️svg/🟦️.ts", "🔨️modules/🎨️depiction/🟦️.ts", "🎯️targets/⚛️react/🔨️modules/🧍️figure/🟦️.tsx"]) expect(staffageModelPipeline(`${root}/${path}`)).toBe(false);
});

it("builds the viewer and the encoders into lazy chunks and keeps them out of the entry", async () => {
  const out = mkdtempSync(join(tmpdir(), "staffage-split-"));
  process.env.SEMIO_VITE_CACHE_DIRECTORY = join(repositoryCachePartitionRootDataV1(resolve(import.meta.dirname, "../../../../../.."), "products"), "vite-staffage-split");
  try {
    await build({ configFile: config, mode: "release", logLevel: "silent", build: { outDir: out, emptyOutDir: true, write: true } });
    const html = readFileSync(join(out, "index.html"), "utf8");
    const entry = /src="\/assets\/([^"]+\.js)"/u.exec(html)![1]!;
    const chunks = Object.fromEntries(readdirSync(join(out, "assets")).filter((name) => name.endsWith(".js") && !name.startsWith("🔣️")).map((name) => [name, readFileSync(join(out, "assets", name), "utf8")]));
    const has = (needle: string): string[] => Object.entries(chunks).filter(([, text]) => text.includes(needle)).map(([name]) => name);
    for (const marker of ["#version 300 es", "KHR_materials_variants", "FBXHeaderExtension", "semio-stage", "staffage volume/v1", "Removing hidden surfaces", "semio.model/v1"]) {
      expect(has(marker), marker).not.toContain(entry);
      expect(has(marker).length, marker).toBeGreaterThan(0);
    }
  } finally {
    rmSync(out, { recursive: true, force: true });
  }
}, 180_000);
