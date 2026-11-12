/** 📐️ Dimension clearance: the overall dimension never touches the drawn geometry, in the side, front and top view at every authored scale.
 *
 * THE REFERENCE is shapely (🐍️.py): the distance of every dimension line to the bounding box of the drawn geometry.
 *
 * @see ./🥒️.feature — the scenarios
 */
import { mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { beforeAll, describe, expect, it, vi } from "vitest";
import { PLANAR_VIEWPOINTS, type Asset, type Viewpoint } from "../../🧬️schema/🟦️.ts";
import type { DrawingScene } from "../../🔨️modules/🔗️contracts/🟦️.ts";
import { DEFAULT_CATALOG_ROOT, loadCatalogFromFiles } from "../../🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { defaultDepictionOptions, depict } from "../../🔨️modules/🎨️depiction/🟦️.ts";
import { DIMENSION_GAP_MM } from "../../🔨️modules/🎨️depiction/🏷️annotations/🟦️.ts";
import { evaluate } from "../../🔨️modules/🧮️evaluation/🟦️.ts";
import { STAFFAGE_ROOT } from "../🧰️support/🧰️.ts";
import { python } from "../🧰️support/🐍️.ts";

type Case = { name: string; box: [number, number, number, number]; items: [number, number][][] };
type Verdict = Record<string, { intersects: boolean; distance: number }>;
const GEOMETRY = ["fill", "body", "detail", "texture", "glazing"];
vi.setConfig({ testTimeout: 900_000 });
const ORACLE = join(STAFFAGE_ROOT, "🧪️tests/📐️dimension-clearance/🐍️.py");
let assets: ReadonlyMap<string, Asset>;

beforeAll(async () => {
  assets = new Map((await loadCatalogFromFiles(DEFAULT_CATALOG_ROOT)).assets.map((asset) => [asset.id, asset]));
});

function caseOf(asset: Asset, denominator: number, viewpoint: Viewpoint): Case {
  const selector = { kind: "scale", scale: { denominator } } as const;
  const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, selector);
  const scene: DrawingScene = depict(state, { ...defaultDepictionOptions("dark-line", "en", denominator, viewpoint), representation: selector });
  const points = (commands: readonly { op: string; x?: number; y?: number }[]): [number, number][] => commands.filter((c) => c.op !== "Z").map((c) => [c.x!, c.y!]);
  const geometry = scene.layers.filter((layer) => GEOMETRY.includes(layer.id)).flatMap((layer) => layer.items).flatMap((item) => (item.kind === "path" ? points(item.commands) : []));
  const items = scene.layers.filter((layer) => layer.id === "annotations").flatMap((layer) => layer.items).flatMap((item) => (item.kind === "path" && item.id.startsWith("dimension:") ? [points(item.commands)] : []));
  const xs = geometry.map((p) => p[0]);
  const ys = geometry.map((p) => p[1]);
  return { name: `${asset.id} 1:${denominator} ${viewpoint}`, box: [Math.min(...xs), Math.min(...ys), Math.max(...xs), Math.max(...ys)], items };
}

function measure(cases: Case[]): Verdict {
  const directory = mkdtempSync(join(tmpdir(), "staffage-"));
  try {
    const file = join(directory, "cases.json");
    writeFileSync(file, JSON.stringify(cases));
    return JSON.parse(python(ORACLE, "geometry", [file])) as Verdict;
  } finally {
    rmSync(directory, { recursive: true, force: true });
  }
}

const authored = (asset: Asset): number[] => Object.values(asset.representations).flatMap((rep) => (rep.status === "authored" ? [rep.denominator] : []));

describe("dimension clearance", () => {
  it("keeps the dimension of the retractable bollard at 1:500 and the street lamp at 1:200 clear in all views", () => {
    const cases = PLANAR_VIEWPOINTS.flatMap((viewpoint) => [caseOf(assets.get("retractable-bollard")!, 500, viewpoint), caseOf(assets.get("street-lamp")!, 200, viewpoint)]);
    const verdict = measure(cases);
    for (const { name, items } of cases) {
      expect(items.length, name).toBeGreaterThan(0);
      expect(verdict[name]!.intersects, name).toBe(false);
      expect(verdict[name]!.distance, name).toBeGreaterThanOrEqual(DIMENSION_GAP_MM - 1e-6);
    }
  });

  it("keeps it clear at every authored scale of both assets", () => {
    const cases = ["retractable-bollard", "street-lamp"].flatMap((id) => authored(assets.get(id)!).flatMap((denominator) => PLANAR_VIEWPOINTS.map((viewpoint) => caseOf(assets.get(id)!, denominator, viewpoint))));
    const verdict = measure(cases);
    for (const { name } of cases) {
      expect(verdict[name]!.intersects, name).toBe(false);
      expect(verdict[name]!.distance, name).toBeGreaterThanOrEqual(DIMENSION_GAP_MM - 1e-6);
    }
  });
});
