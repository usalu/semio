/** 🧊️ Cast levels of detail: schedule vectors, the schema, the coverage of every course cast at every level and the exactness of a sampled frame.
 *
 * THE REFERENCES are ajv (the levels file against its schema), the repository's own `frameSchedule`/`evaluate`/`prepare` recomputed per cast, and a full-detail export whose frames at the shared canonical times must carry the same thermal fills as the preview frames.
 * @see ../../🧫️fixtures/🧊️cast-lod/🔣️.json — the language-agnostic vectors
 * @see ../../🔨️modules/📤️export/🎭️cast/🧊️lod/🟦️.ts — the levels */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import Ajv from "ajv";
import { JSDOM } from "jsdom";
import { beforeAll, describe, expect, it } from "vitest";
import type { Asset } from "../../🧬️schema/🟦️.ts";
import { evaluate, poseOf } from "../../🔨️modules/🧮️evaluation/🟦️.ts";
import { prepare, regionsOf, selectRoles, sourceOf } from "../../🔨️modules/🎨️depiction/🧩️shapes/🟦️.ts";
import { DEFAULT_CATALOG_ROOT, loadCatalogFromFiles } from "../../🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { canonicalSchedule, exportCast, thinned } from "../../🔨️modules/📤️export/🎭️cast/🟦️.ts";
import { type CastIndex, type CastManifest } from "../../🔨️modules/📤️export/🎭️cast/🧬️schema/🟦️.ts";
import { LOD_IDS, lodSchedule, resolveLod } from "../../🔨️modules/📤️export/🎭️cast/🧊️lod/🟦️.ts";
import { STAFFAGE_ROOT, fixture, withScratch } from "../🧰️support/🧰️.ts";

type Vectors = {
  schedules: { name: string; canonical: { fps: number; durationS: number; loop: boolean; count: number }; lod: string; outputFps: number; expected: { count: number; fps: number; first: number; last: number } }[];
  invalid: { name: string; lod: string; outputFps: number; message: string }[];
  resolved: { lod: string; outputFps: number; fpsCap: number | null; revision: string }[];
  roles: Record<string, string[]>;
};

const vectors = fixture<Vectors>("🧊️cast-lod");
const lodRoot = join(STAFFAGE_ROOT, "🔨️modules/📤️export/🎭️cast/🧊️lod");
const read = (name: string): unknown => JSON.parse(readFileSync(join(lodRoot, name), "utf8"));
const manifest = JSON.parse(readFileSync(join(STAFFAGE_ROOT, "../⚡️energy/🏗️authoring/🎨️visuals/🧍️staffage/🔣️.json"), "utf8")) as CastManifest;
const ids = Object.keys(manifest.casts);

const canonicalOf = (c: Vectors["schedules"][number]["canonical"]) => ({ ...c, times: Array.from({ length: c.count }, (_, frame) => frame / c.fps) });

describe("🧊️ the levels file", () => {
  it("is valid in ajv and offers exactly the level ids of the twin", () => {
    const validate = new Ajv({ strict: true, allErrors: true }).compile(read("🔣️.schema.json") as object);
    expect(validate(read("🔣️.json")), JSON.stringify(validate.errors?.slice(0, 3))).toBe(true);
    expect(Object.keys((read("🔣️.json") as { levels: object }).levels)).toEqual([...LOD_IDS]);
  });

  for (const { lod, outputFps, fpsCap, revision } of vectors.resolved)
    it(`resolves ${lod} at ${outputFps} fps with a sample cap of ${fpsCap}`, () => {
      const resolved = resolveLod(lod, outputFps);
      expect(resolved.fpsCap).toBe(fpsCap);
      expect(resolved.level.roles).toEqual(vectors.roles[lod]);
      expect(resolved.revision).toBe(revision);
      expect(resolveLod(lod, outputFps).revision).toBe(resolved.revision);
    });

  it("gives different parameters different revisions", () => {
    const revisions = [resolveLod("preview", 15), resolveLod("preview", 30), resolveLod("full", 15), resolveLod("poster", 15)].map((lod) => lod.revision);
    expect(new Set(revisions).size).toBe(revisions.length);
  });

  for (const { name, lod, outputFps, message } of vectors.invalid) it(`refuses ${name}`, () => expect(() => resolveLod(lod, outputFps)).toThrow(new RegExp(message, "u")));
});

describe("🧊️ the schedule of a level", () => {
  for (const { name, canonical, lod, outputFps, expected } of vectors.schedules)
    it(name, () => {
      const schedule = lodSchedule(canonicalOf(canonical), resolveLod(lod, outputFps));
      expect(schedule.count).toBe(expected.count);
      expect(schedule.times).toHaveLength(expected.count);
      expect(schedule.fps).toBeCloseTo(expected.fps, 9);
      expect(schedule.times[0]).toBeCloseTo(expected.first, 9);
      expect(schedule.times.at(-1)).toBeCloseTo(expected.last, 9);
      expect(schedule.durationS).toBe(canonical.durationS);
      expect(schedule.loop).toBe(canonical.loop);
    });
});

describe("🧊️ every course cast at every level", { timeout: 300_000 }, () => {
  let assets: Map<string, Asset>;

  beforeAll(async () => {
    const catalog = await loadCatalogFromFiles(DEFAULT_CATALOG_ROOT, undefined, new Set(Object.values(manifest.casts).map((cast) => cast.asset)));
    assets = new Map(catalog.assets.map((asset) => [asset.id, asset]));
  }, 120_000);

  it("samples a monotone, in-range, terminal-exact schedule of the canonical clock for every cast and level", () => {
    for (const id of ids) {
      const cast = manifest.casts[id]!;
      const canonical = canonicalSchedule(cast, assets.get(cast.asset)!);
      expect(canonical.times.length, id).toBe(canonical.count);
      for (const level of LOD_IDS) {
        const lod = resolveLod(level, 15);
        const schedule = lodSchedule(canonical, lod);
        const label = `${id} at ${level}`;
        expect(schedule.count, label).toBeLessThanOrEqual(Math.max(1, Math.min(canonical.count, lod.level.frames.maxFrames)));
        expect(schedule.times, label).toHaveLength(schedule.count);
        schedule.times.forEach((time, index) => {
          expect(time, label).toBeGreaterThanOrEqual(0);
          expect(time, label).toBeLessThanOrEqual(Math.max(canonical.times.at(-1)!, canonical.durationS) + 1e-9);
          if (index > 0) expect(time, label).toBeGreaterThan(schedule.times[index - 1]!);
        });
        if (level === "full") expect(schedule, label).toEqual(canonical);
        if (!canonical.loop && canonical.count > 1 && schedule.count > 1) expect(schedule.times.at(-1)!, label).toBeCloseTo(canonical.durationS, 9);
        if (level === "preview" && lod.fpsCap !== null && schedule.count > 1) expect(schedule.fps, label).toBeLessThanOrEqual(Math.max(lod.fpsCap, canonical.fps) + 1e-9);
      }
    }
  });

  it("keeps every thermal region painted, every anchor shape and the figure bounds when only the preview roles are drawn", () => {
    const preview = resolveLod("preview", 15).level.roles;
    let drawnFull = 0;
    let drawnPreview = 0;
    for (const id of ids) {
      const cast = manifest.casts[id]!;
      const asset = assets.get(cast.asset)!;
      const query = cast.clip === undefined ? ({ kind: "pose", pose: poseOf(asset, cast.pose!).id, timeS: 0 } as const) : ({ kind: "clip", clip: cast.clip, timeS: cast.transportAgeS ?? 0 } as const);
      const state = evaluate(asset, query, { kind: "scale", scale: { denominator: cast.scale } });
      if (cast.viewpoint === "axonometric") continue;
      const all = prepare(state, cast.viewpoint, sourceOf(state), 1000 / cast.scale);
      const kept = selectRoles(all, preview);
      expect(kept.length, `${id}: something is drawn`).toBeGreaterThan(0);
      drawnFull += all.length;
      drawnPreview += kept.length;
      if (cast.view === "thermal") {
        const painted = new Set(kept.flatMap((shape) => regionsOf(shape.shape)));
        const missing = [...new Set(all.flatMap((shape) => regionsOf(shape.shape)))].filter((region) => !painted.has(region));
        expect(missing, `${id}: thermal regions only painted by dropped roles`).toEqual([]);
      }
      expect(kept.filter((shape) => shape.shape.role === "ground").length, `${id}: ground shapes`).toBe(all.filter((shape) => shape.shape.role === "ground").length);
    }
    expect(drawnPreview, "the preview draws fewer shapes than the full level").toBeLessThan(drawnFull);
  });
});

describe("🧊️ a sampled frame equals the full frame at the same canonical time", { timeout: 180_000 }, () => {
  const fills = (svg: string): Map<string, string> => {
    const document = new JSDOM(svg, { contentType: "image/svg+xml" }).window.document;
    return new Map([...document.querySelectorAll('g[id^="shape-"]')].flatMap((group) => {
      const path = group.querySelector("path");
      return path === null ? [] : [[group.id, `${path.getAttribute("fill")}|${path.getAttribute("fill-opacity") ?? ""}`] as const];
    }));
  };
  const frame = (root: string, index: CastIndex, position: number): string => readFileSync(join(root, index.cast, index.frames[position]!), "utf8");

  it("writes the same thermal fills at coinciding canonical times, the same logical truth and a profile-scoped index", async () => {
    await withScratch(async (directory) => {
      const cast = { version: 1, casts: { walker: { asset: "walking-commuter", viewpoint: "elevation", clip: "walk-cycle", view: "thermal", theme: "dark", scale: 50, fps: 15, environment: "winter-overcast" } } } as CastManifest;
      const [full] = await exportCast(cast, ["walker"], join(directory, "full"), { lod: resolveLod("full", 15) });
      const [preview] = await exportCast(cast, ["walker"], join(directory, "preview"), { lod: resolveLod("preview", 15) });
      const [poster] = await exportCast(cast, ["walker"], join(directory, "poster"), { lod: resolveLod("poster", 15) });
      expect(full!.frames).toHaveLength(18);
      expect(preview!.frames).toHaveLength(9);
      expect(poster!.frames).toHaveLength(1);
      expect(preview!.fps).toBeCloseTo(7.5, 9);
      expect([full!.lod.id, preview!.lod.id, poster!.lod.id]).toEqual(["full", "preview", "poster"]);
      expect(preview!.lod).toMatchObject({ canonicalFrames: 18, canonicalFps: 15 });
      expect(preview!.provenance.lod).toEqual({ id: "preview", revision: preview!.lod.revision });
      expect(preview!.provenance.lod.revision).not.toBe(full!.provenance.lod.revision);
      expect(preview!.logical).toEqual({ ...full!.logical, maxPaths: preview!.logical.maxPaths });
      expect(poster!.logical).toEqual({ ...full!.logical, maxPaths: poster!.logical.maxPaths });
      expect(preview!.logical.maxPaths).toBeLessThan(full!.logical.maxPaths);
      expect(preview!.mmPerMetre).toBe(full!.mmPerMetre);
      expect(preview!.durationS).toBe(full!.durationS);
      expect(preview!.loop).toBe(full!.loop);
      for (let k = 0; k < preview!.frames.length; k++) {
        const mine = fills(frame(join(directory, "preview"), preview!, k));
        const theirs = fills(frame(join(directory, "full"), full!, 2 * k));
        expect(mine.size).toBeGreaterThan(0);
        for (const [shape, fill] of mine) expect(theirs.get(shape), `frame ${k} shape ${shape}`).toBe(fill);
      }
    });
  });
});

describe("🧊️ canonical clock", () => {
  it("is the thinned frame schedule of the clip", () => {
    const schedule = thinned({ fps: 15, durationS: 100, loop: false, count: 1501, times: Array.from({ length: 1501 }, (_, frame) => frame / 15) });
    expect(schedule.count).toBe(600);
    expect(lodSchedule(schedule, resolveLod("full", 30))).toBe(schedule);
  });
});
