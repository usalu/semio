/** 📦️ Validates the release boundary through the same plugin that loads every authored asset module.
 * @see ./🥒️.feature */
import { mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join, resolve } from "node:path";
import { expect, it } from "vitest";
import example from "../../../🧫️fixtures/👤️sample-figure/🔣️.json";
import { STAFFAGE_ASSET_QUERY, staffageAssetVitePlugin } from "../../../🎯️targets/⚛️react/🌐️vite/🟦️.ts";

const fixture = resolve(import.meta.dirname, "../../../🧫️fixtures/👤️sample-figure/🔣️.json");

async function load(file: string, release: boolean) {
  const plugin = staffageAssetVitePlugin(release);
  const module = await plugin.resolveId?.(file + STAFFAGE_ASSET_QUERY);
  if (!module) throw new Error("Expected an asset module for " + file);
  const loadAsset = plugin.load;
  return loadAsset?.(module);
}

it("rejects invalid assets only in release builds", async () => {
  const directory = mkdtempSync(join(tmpdir(), "staffage-release-"));
  const file = join(directory, "invalid.json");
  try {
    const asset = { ...example.asset, dimensions: { ...example.asset.dimensions, heightM: 0 } };
    writeFileSync(file, JSON.stringify({ ...example, asset }));
    await expect(load(file, false)).resolves.toContain("dimension-invalid");
    await expect(load(file, true)).rejects.toThrow(/invalid\.json[\s\S]*dimension-invalid/u);
  } finally {
    rmSync(directory, { recursive: true, force: true });
  }
});

it("ships valid assets with their revision and no findings", async () => {
  const source = await load(fixture, true);
  expect(source).toMatch(/"revision":"sha256:[0-9a-f]{64}"/u);
  expect(source).toContain('"issues":[]');
});
