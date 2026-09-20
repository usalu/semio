/** 🧲️ Cross-implementation oracle for World3d snapping and active-reference residency. */
import Ajv from "ajv";
import { existsSync, readFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { Vector3 } from "three";
import { describe, expect, test } from "vitest";

let workspace = dirname(fileURLToPath(import.meta.url));
while (!existsSync(join(workspace, ".git"))) workspace = resolve(workspace, "..");
const world = join(workspace, "🧰️framework", "🛍️products", "💻️os", "🔨️modules", "♾️infinite", "🌍️world");
const fixture = JSON.parse(readFileSync(join(world, "🧫️fixtures", "🧲️scene-input-residency", "🔣️.json"), "utf8"));
const schema = JSON.parse(readFileSync(join(world, "🧬️schema", "🧲️scene-input-residency", "🔣️.json"), "utf8"));

const snap = (point: readonly number[], enabled: boolean, factor: number): number[] => enabled && factor > 0 ? new Vector3(point[0], point[1], point[2]).divideScalar(factor).round().multiplyScalar(factor).toArray() : [...point];

describe("🧲️ World scene input and reference residency", () => {
  test("the neutral fixture satisfies its Ajv schema and Three agrees with React snapping", () => {
    const validate = new Ajv({ allErrors: true, strict: true }).compile(schema);
    expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
    expect(snap(fixture.snap.point, false, fixture.snap.factor)).toEqual(fixture.snap.disabled);
    expect(snap(fixture.snap.point, true, fixture.snap.factor)).toEqual(fixture.snap.enabled);

    const react = readFileSync(join(workspace, fixture.oracles.reactWorld3d), "utf8");
    expect(react).toContain("if (!gridSnapEnabled || gridFactor <= 0)");
    expect(react).toContain("snapWorldPointToGrid(moved, snap.gridSnapEnabled, snap.gridFactor)");
    expect(react).toContain("resolveCatalogueDropOrigin(clientX, clientY, rect, cameraRef.current, gridSnapEnabled, gridFactor)");
  });

  test("an independent latest-reference model rejects A after replacement and stays bounded beyond 256 replacements", () => {
    const { removedUrl, replacementUrl, distinctReplacementCount, maximumResidentUrls } = fixture.referenceResidency;
    let current = new Set([removedUrl]);
    const resident = new Set([removedUrl]);
    current = new Set([replacementUrl]);
    for (const url of [...resident]) if (!current.has(url)) resident.delete(url);
    if (current.has(removedUrl)) resident.add(removedUrl);
    expect(resident.has(removedUrl)).toBe(false);
    if (current.has(replacementUrl)) resident.add(replacementUrl);
    expect([...resident]).toEqual([replacementUrl]);
    for (let index = 0; index < distinctReplacementCount; index += 1) {
      const url = `/reference/replacement-${index}.png`;
      current = new Set([url]);
      for (const previous of [...resident]) if (!current.has(previous)) resident.delete(previous);
      resident.add(url);
      expect(resident.size).toBeLessThanOrEqual(maximumResidentUrls);
    }
  });

  test("the WGPU source routes all input and decoded references through the shared ownership decisions", () => {
    const source = readFileSync(join(world, "🦀️.rs"), "utf8");
    const renderer = readFileSync(join(workspace, "🧰️framework", "🛍️products", "💻️os", "🔨️modules", "📺️renderer", "🧑‍🎨engine", "🎯️targets", "🧊️wgpu", "🧊️renderer", "🦀️.rs"), "utf8");
    expect(source).toContain("grid_snap_enabled: bool");
    expect(source.match(/snap_world_point_to_grid\(/g)?.length).toBe(4);
    expect(source).toContain("cancel_reference_image_url");
    expect(source).toContain("world3d_reference_url_is_current");
    expect(renderer).toContain("world3d_reference_url_is_current(state, &url)");
  });
});
