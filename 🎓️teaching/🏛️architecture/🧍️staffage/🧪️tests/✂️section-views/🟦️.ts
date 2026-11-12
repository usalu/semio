/** ✂️ Section views: captions, the cut-line marker in the side view, the plane validation and the cast index.
 *
 * THE REFERENCE is shapely (🐍️.py) and the hand-computed numbers of the fixture.
 *
 * @see ./🥒️.feature — the scenarios
 * @see ../../🧫️fixtures/✂️section-views/🔣️.json — the vectors
 */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { beforeAll, describe, expect, it, vi } from "vitest";
import type { Asset, AuthoredRepresentation, ViewSection, Viewpoint } from "../../🧬️schema/🟦️.ts";
import type { DrawingItem, DrawingScene } from "../../🔨️modules/🔗️contracts/🟦️.ts";
import { DEFAULT_CATALOG_ROOT, loadCatalogFromFiles } from "../../🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { validateAsset } from "../../🔨️modules/🗂️catalog/🟦️.ts";
import { defaultDepictionOptions, depict, textlessDepictionOptions } from "../../🔨️modules/🎨️depiction/🟦️.ts";
import { sectionOf, viewpointCaption } from "../../🔨️modules/🎨️depiction/✂️section/🟦️.ts";
import { evaluate } from "../../🔨️modules/🧮️evaluation/🟦️.ts";
import { exportCast } from "../../🔨️modules/📤️export/🎭️cast/🟦️.ts";
import type { CastIndex } from "../../🔨️modules/📤️export/🎭️cast/🧬️schema/🟦️.ts";
import { STAFFAGE_ROOT, fixture, withScratch } from "../🧰️support/🧰️.ts";
import { python } from "../🧰️support/🐍️.ts";

type Pair = readonly [number, number];
type PathItem = Extract<DrawingItem, { kind: "path" }>;
type Placed = { line: Pair[]; ends: Pair[][]; tips: Pair[] };
type Vectors = {
  toleranceMm: number;
  bounds: { minX: number; maxX: number; minY: number; maxY: number };
  sections: { front: ViewSection; plan: ViewSection };
  markers: ({ id: string; viewpoint: "front" | "plan"; plane: number; k: number } & Placed)[];
  captions: { viewpoint: Viewpoint; label: string | null; en: string; de: string }[];
  catalog: Record<string, Partial<Record<"front" | "plan", number>>>;
  elevationCatalog: Record<string, Record<string, number>>;
};

const vectors = fixture<Vectors>("✂️section-views");
vi.setConfig({ testTimeout: 900_000 });
const ORACLE = join(STAFFAGE_ROOT, "🧪️tests/✂️section-views/🐍️.py");
const FIXTURE = join(STAFFAGE_ROOT, "🧫️fixtures/✂️section-views/🔣️.json");
let assets: ReadonlyMap<string, Asset>;

beforeAll(async () => {
  assets = new Map((await loadCatalogFromFiles(DEFAULT_CATALOG_ROOT)).assets.map((asset) => [asset.id, asset]));
});

const near = (actual: readonly number[], expected: readonly number[], tolerance: number, label: string): void => {
  expect(actual.length, label).toBe(expected.length);
  actual.forEach((value, i) => expect(Math.abs(value - expected[i]!), `${label}[${i}]`).toBeLessThanOrEqual(tolerance));
};

function sceneOf(asset: Asset, denominator: number, viewpoint: Viewpoint, textless = false): DrawingScene {
  const selector = { kind: "scale", scale: { denominator } } as const;
  const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, selector);
  const options = textless ? textlessDepictionOptions("dark-line", denominator, viewpoint) : defaultDepictionOptions("dark-line", "en", denominator, viewpoint);
  return depict(state, { ...options, representation: selector });
}

const itemsOf = (scene: DrawingScene, prefix: string): PathItem[] => scene.layers.flatMap((layer) => layer.items).filter((item): item is PathItem => item.kind === "path" && item.id.startsWith(prefix));

function pointsOf(item: PathItem, scene: DrawingScene): Pair[] {
  const { xMm, yMm } = scene.metadata.origin;
  return item.commands.filter((command) => command.op !== "Z").map((command) => [command.x - xMm, command.y - yMm] as Pair);
}

describe("section marker", () => {
  it("is reproduced by shapely from the hand-computed vectors", () => {
    const placed = JSON.parse(python(ORACLE, "geometry", [FIXTURE])) as Record<string, Placed>;
    for (const marker of vectors.markers) {
      const got = placed[marker.id]!;
      near(got.line.flat(), marker.line.flat(), vectors.toleranceMm, `${marker.id} line`);
      near(got.ends.flat(2), marker.ends.flat(2), vectors.toleranceMm, `${marker.id} ends`);
      near(got.tips.flat(), marker.tips.flat(), vectors.toleranceMm, `${marker.id} tips`);
    }
  });

  it("is drawn by the side view at the plane of the section at 1:100 and 1:50", () => {
    const asset = assets.get("curtain-wall-bay")!;
    const scenes = [100, 50].map((denominator) => ({ denominator, scene: sceneOf(asset, denominator, "elevation") }));
    const placedAt = (denominator: number): Record<string, Placed> => {
      const bounds = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator } }).bounds;
      return JSON.parse(python(ORACLE, "geometry", [FIXTURE, JSON.stringify(bounds)])) as Record<string, Placed>;
    };
    const first = placedAt(100);
    const second = placedAt(50);
    const placed: Record<string, Placed> = { "front-100": first["front-100"]!, "plan-100": first["plan-100"]!, "front-50": second["front-50"]!, "plan-50": second["plan-50"]! };
    for (const { denominator, scene } of scenes)
      for (const viewpoint of ["front", "plan"] as const) {
        const want = placed[`${viewpoint}-${denominator}`]!;
        const [line] = itemsOf(scene, `section:${viewpoint}:line`);
        near(pointsOf(line!, scene).flat(), want.line.flat(), 1e-3, `${viewpoint} ${denominator} line`);
        want.ends.forEach((end, i) => near(pointsOf(itemsOf(scene, `section:${viewpoint}:end-${i}`)[0]!, scene).flat(), end.flat(), 1e-3, `${viewpoint} ${denominator} end ${i}`));
        want.tips.forEach((tip, i) => {
          const outline = pointsOf(itemsOf(scene, `section:${viewpoint}:arrow-${i}`)[0]!, scene);
          const reach = viewpoint === "front" ? Math.min(...outline.map((p) => p[0])) : Math.max(...outline.map((p) => p[1]));
          expect(Math.abs(reach - (viewpoint === "front" ? tip[0] : tip[1])), `${viewpoint} ${denominator} tip ${i}`).toBeLessThanOrEqual(1e-3);
        });
        const letters = scene.layers.flatMap((layer) => layer.items).filter((item) => item.kind === "text" && item.id.startsWith(`section:${viewpoint}:letter`));
        expect(letters.map((letter) => (letter.kind === "text" ? letter.text : ""))).toEqual(viewpoint === "front" ? ["A", "A"] : ["B", "B"]);
      }
  });

  it("is drawn by the plan at the lateral plane of an elevation section at 1:100 and 1:50, the arrows pointing to the far side", () => {
    const asset = assets.get("cooking-pot")!;
    for (const denominator of [100, 50]) {
      const scene = sceneOf(asset, denominator, "plan");
      const bounds = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator } }).bounds;
      const want = (JSON.parse(python(ORACLE, "geometry", [FIXTURE, JSON.stringify(bounds)])) as Record<string, Placed>)[`elevation-${denominator}`]!;
      const [line] = itemsOf(scene, "section:elevation:line");
      near(pointsOf(line!, scene).flat(), want.line.flat(), 1e-3, `elevation ${denominator} line`);
      want.ends.forEach((end, i) => near(pointsOf(itemsOf(scene, `section:elevation:end-${i}`)[0]!, scene).flat(), end.flat(), 1e-3, `elevation ${denominator} end ${i}`));
      want.tips.forEach((tip, i) => expect(Math.abs(Math.min(...pointsOf(itemsOf(scene, `section:elevation:arrow-${i}`)[0]!, scene).map((p) => p[1])) - tip[1]), `elevation ${denominator} tip ${i}`).toBeLessThanOrEqual(1e-3));
      expect(scene.layers.flatMap((layer) => layer.items).filter((item) => item.kind === "text" && item.id.startsWith("section:elevation:letter")).map((letter) => (letter.kind === "text" ? letter.text : ""))).toEqual(["A", "A"]);
    }
  });

  it("is absent at 1:200, in a textless depiction and in the front and plan views", () => {
    const asset = assets.get("curtain-wall-bay")!;
    expect(itemsOf(sceneOf(asset, 200, "elevation"), "section:")).toHaveLength(0);
    expect(itemsOf(sceneOf(asset, 100, "elevation", true), "section:")).toHaveLength(0);
    expect(itemsOf(sceneOf(asset, 100, "front"), "section:")).toHaveLength(0);
    expect(itemsOf(sceneOf(asset, 100, "plan"), "section:")).toHaveLength(0);
  });

  it("is absent for an asset without sections", () => {
    expect(itemsOf(sceneOf(assets.get("solar-thermal-collector")!, 100, "elevation"), "section:")).toHaveLength(0);
  });
});

describe("section captions", () => {
  for (const { viewpoint, label, en, de } of vectors.captions)
    it(`reads ${en} / ${de}`, () => {
      const section = label === null ? undefined : { plane: 0, label: { en: label, de: label } };
      expect(viewpointCaption(viewpoint, section, "en")).toBe(en);
      expect(viewpointCaption(viewpoint, section, "de")).toBe(de);
    });

  it("titles the front and plan scenes of a sectioned asset", () => {
    const asset = assets.get("curtain-wall-bay")!;
    const texts = (scene: DrawingScene): string => scene.layers.flatMap((layer) => layer.items).flatMap((item) => (item.kind === "text" ? [item.text] : [])).join("|");
    expect(texts(sceneOf(asset, 100, "front"))).toContain("Section A-A");
    expect(texts(sceneOf(asset, 100, "plan"))).toContain("Plan section B-B");
    expect(texts(sceneOf(assets.get("solar-thermal-collector")!, 100, "front"))).toContain("Front elevation");
  });
});

describe("section catalog", () => {
  for (const [id, planes] of Object.entries(vectors.catalog))
    it(`${id} declares its sections at every authored scale`, () => {
      const asset = assets.get(id)!;
      const reps = Object.values(asset.representations).filter((rep): rep is AuthoredRepresentation => rep.status === "authored");
      expect(reps.length).toBeGreaterThan(0);
      for (const rep of reps) {
        expect(Object.keys(rep.sections ?? {}).sort()).toEqual(Object.keys(planes).sort());
        for (const [viewpoint, plane] of Object.entries(planes)) expect(sectionOf(rep, viewpoint as Viewpoint)?.plane).toBe(plane);
      }
    });
});

describe("elevation section catalog", () => {
  for (const [id, scales] of Object.entries(vectors.elevationCatalog))
    it(`${id} declares its elevation section at 1:${Object.keys(scales).join(", 1:")} and at no other scale`, () => {
      const asset = assets.get(id)!;
      for (const rep of Object.values(asset.representations).filter((candidate): candidate is AuthoredRepresentation => candidate.status === "authored")) {
        const plane = scales[String(rep.denominator)];
        expect(sectionOf(rep, "elevation")?.plane, `1:${rep.denominator}`).toBe(plane);
        expect(rep.sections?.front === undefined || id === "retractable-bollard", `1:${rep.denominator} front`).toBe(true);
      }
    });
});

describe("section validation", () => {
  it("reports a plane outside the asset in English and German", () => {
    const asset = assets.get("sliding-door")!;
    const representations = Object.fromEntries(Object.entries(asset.representations).map(([key, rep]) => [key, rep.status === "authored" ? { ...rep, sections: { plan: { plane: 9, label: { en: "A-A", de: "A-A" } } } } : rep]));
    const issue = validateAsset({ ...asset, representations } as Asset).find((candidate) => candidate.code === "section-plane-outside");
    expect(issue).toBeDefined();
    expect(issue!.message.en).toContain("outside the extent");
    expect(issue!.message.de).toContain("außerhalb der Ausdehnung");
  });

  it("reports a lateral plane outside the asset for an elevation section", () => {
    const asset = assets.get("cooking-pot")!;
    const representations = Object.fromEntries(Object.entries(asset.representations).map(([key, rep]) => [key, rep.status === "authored" ? { ...rep, sections: { elevation: { plane: 9, label: { en: "A-A", de: "A-A" } } } } : rep]));
    const issue = validateAsset({ ...asset, representations } as Asset).find((candidate) => candidate.code === "section-plane-outside");
    expect(issue).toBeDefined();
    expect(issue!.message.en).toContain("side section");
    expect(issue!.message.de).toContain("Seitenansicht");
  });

  it("accepts the declared planes", () => {
    for (const id of [...Object.keys(vectors.catalog), ...Object.keys(vectors.elevationCatalog)]) expect(validateAsset(assets.get(id)!).filter((issue) => issue.code === "section-plane-outside"), id).toEqual([]);
  });
});

describe("section cast index", () => {
  it("records the section of the viewpoint", async () => {
    const manifest = { version: 1, casts: { door: { asset: "sliding-door", pose: "closed", view: "dark-line", theme: "dark", viewpoint: "plan", scale: 100 }, side: { asset: "sliding-door", pose: "closed", view: "dark-line", theme: "dark", viewpoint: "elevation", scale: 100 } } };
    await withScratch(async (directory) => {
      const [door, side] = (await exportCast(manifest, ["door", "side"], directory, { catalog: DEFAULT_CATALOG_ROOT })) as readonly CastIndex[];
      expect(door!.section).toEqual({ plane: 1, label: { en: "A-A", de: "A-A" } });
      expect(side!.section).toBeUndefined();
      expect(JSON.parse(readFileSync(join(directory, "door", "🔣️.json"), "utf8")).section.plane).toBe(1);
    });
  });
});
