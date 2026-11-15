/** 🪤️ The per-solid trap findings of the volume validator (`volumeTrapIssues`): an open revolve profile closed through the axis (`volume-revolve-implied-closure`), a `taperTo` read as [scale u, scale v] (`volume-taper-uniform`, `volume-taper-degenerate`) and overlapping loft sections (`volume-loft-fold`), one vector each, on the sample figure with the solid added to its 1:50 body.
 *
 * THE REFERENCE for the revolve closure is shapely (profile validity and area, `🐍️.py`); the taper and loft vectors are hand-computed (the loft fold also in the kernel suite `🧪️tests/🪤️traps`).
 *
 * @see ../../🧫️fixtures/🪤️volume-traps/🔣️.json — the vectors */
import { mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { describe, expect, it } from "vitest";
import type { Asset, Vector2 } from "../../🧬️schema/🟦️.ts";
import { volumeTrapIssues } from "../../🔨️modules/🗂️catalog/🧊️volume/🟦️.ts";
import { STAFFAGE_ROOT, fixture, requireTool } from "../🧰️support/🧰️.ts";
import { python } from "../🧰️support/🐍️.ts";

type Vector = { name: string; codes: string[]; shell?: boolean; solid: Record<string, unknown> };
const vectors = fixture<{ scale: "50"; solids: Vector[] }>("🪤️volume-traps");
const sample: Asset = fixture<{ asset: Asset }>("👤️sample-figure").asset;

const withSolid = (v: Vector): Asset => {
  const asset = structuredClone(sample) as any;
  const volume = asset.volumes[vectors.scale];
  const { bone, material, role, tone } = volume.body[0];
  volume.body.push({ id: "trap", bone, material, role, tone, ...structuredClone(v.solid) });
  return asset;
};

describe("🪤️ volumeTrapIssues", () => {
  it("the sample figure carries no trap", () => {
    expect(volumeTrapIssues(sample).map((i) => i.code)).toEqual([]);
  });
  for (const v of vectors.solids) {
    it(`${v.name}: ${v.codes.length === 0 ? "no finding" : v.codes.join(", ")}`, () => {
      const found = volumeTrapIssues(withSolid(v));
      expect(found.map((i) => i.code)).toEqual(v.codes);
      for (const issue of found) {
        expect(issue.path).toMatch(new RegExp(`^volumes\\.${vectors.scale}\\.body\\[\\d+\\]`, "u"));
        expect(issue.message.en.length).toBeGreaterThan(20);
        expect(issue.message.de.length).toBeGreaterThan(20);
      }
    });
  }
  it("the revolve closure agrees with shapely", { timeout: 300_000 }, () => {
    requireTool("uv");
    const revolves = vectors.solids.filter((v) => v.solid.kind === "revolve");
    const directory = mkdtempSync(join(tmpdir(), "staffage-traps-"));
    try {
      const input = join(directory, "input.json");
      writeFileSync(input, JSON.stringify({ cases: revolves.map((v) => ({ name: v.name, profile: v.solid.profile as Vector2[] })) }));
      const reference = JSON.parse(python(join(STAFFAGE_ROOT, "🧪️tests/🪤️volume-traps/🐍️.py"), "geometry", [input])) as Record<string, { shell: boolean }>;
      for (const v of revolves) expect(reference[v.name]!.shell, v.name).toBe(v.shell === true);
    } finally {
      rmSync(directory, { recursive: true, force: true });
    }
  });
});
