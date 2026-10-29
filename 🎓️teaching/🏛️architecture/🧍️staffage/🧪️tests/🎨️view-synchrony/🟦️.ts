/** 🎨️ All seven views of one state share identical shape geometry (relative to the model origin) and keep value labels clear of each other and of the drawing.
 * @see ./🥒️.feature */
import { describe, expect, it } from "vitest";
import { VIEWS, type DrawingItem, type DrawingScene } from "../../🔨️modules/🔗️contracts/🟦️.ts";
import { defaultDepictionOptions, depictAll } from "../../🔨️modules/🎨️depiction/🟦️.ts";
import { readFileSync } from "node:fs";
import type { Asset } from "../../🧬️schema/🟦️.ts";
import { evaluate } from "../../🔨️modules/🧮️evaluation/🟦️.ts";
import { toneColour } from "../../🔨️modules/🎨️depiction/🖍️palette/🟦️.ts";
import { labelBudget } from "../../🔨️modules/🎨️depiction/🧱️canvas/🟦️.ts";
import { walkerState } from "../../🧫️fixtures/🧱️synthetic-figure/🟦️.ts";

const items = (scene: DrawingScene): DrawingItem[] => scene.layers.flatMap((l) => l.items);
const modelGeometry = (scene: DrawingScene): Map<string, number[]> => {
  const { origin, mmPerMetre: k } = scene.metadata;
  const out = new Map<string, number[]>();
  for (const i of items(scene)) {
    if (i.kind !== "path" || !i.id.startsWith("shape:")) continue;
    out.set(i.id, i.commands.flatMap((c) => (c.op === "Z" ? [] : c.op === "C" ? [(c.x - origin.xMm) / k, (origin.yMm - c.y) / k] : [(c.x - origin.xMm) / k, (origin.yMm - c.y) / k])));
  }
  return out;
};

describe("view synchrony", () => {
  for (const denominator of [100, 50]) {
    const scenes = depictAll(walkerState("carry-bag"), { ...defaultDepictionOptions("light-line", "en", denominator) });
    it(`shares fill geometry across all seven views at 1:${denominator}`, () => {
      const reference = modelGeometry(scenes["light-solid"]);
      expect(reference.size).toBeGreaterThan(5);
      for (const view of VIEWS) {
        const g = modelGeometry(scenes[view]);
        for (const [id, coords] of g) {
          const ref = reference.get(id);
          if (ref === undefined) continue;
          expect(coords.length).toBe(ref.length);
          coords.forEach((v, i) => expect(v).toBeCloseTo(ref[i]!, 6));
        }
        expect([...g.keys()].filter((id) => !reference.has(id))).toEqual([]);
      }
    });
  }
  it("keeps label text from overlapping and from reading raw floats", () => {
    const scenes = depictAll(walkerState("carry-bag"), { ...defaultDepictionOptions("forces", "en", 50) });
    for (const view of ["forces", "moments", "thermal"] as const) {
      const texts = items(scenes[view]).flatMap((i) => (i.kind === "text" ? [i] : []));
      for (const t of texts) expect(t.text).not.toMatch(/\d{9,}|e-\d/u);
      const boxes = texts.filter((t) => t.id.includes(":label")).map((t) => ({ x0: t.x, x1: t.x + t.text.length * t.sizeMm * 0.56, y0: t.y - t.sizeMm * 0.8, y1: t.y + t.sizeMm * 0.2 }));
      boxes.forEach((a, i) => boxes.slice(i + 1).forEach((b) => expect(a.x0 < b.x1 && b.x0 < a.x1 && a.y0 < b.y1 && b.y0 < a.y1).toBe(false)));
    }
  });

  describe("label budget on a complex rig", () => {
    const oak = (JSON.parse(readFileSync(new URL("../../🗂️catalog/🌳️vegetation/🔣️.json", import.meta.url), "utf8")) as { assets: Asset[] }).assets.find((a) => a.id === "english-oak")!;
    for (const denominator of [50, 100, 200, 500])
      it(`prints at most ${labelBudget(denominator)} unique-overlap-free labels per analytic view at 1:${denominator}`, () => {
        const state = evaluate(oak, { kind: "pose", pose: "windy" });
        const scenes = depictAll(state, { ...defaultDepictionOptions("forces", "en", denominator) });
        for (const view of ["forces", "moments"] as const) {
          const labels = items(scenes[view]).flatMap((i) => (i.kind === "text" && i.id.endsWith(":label") && !i.id.startsWith("freebody") ? [i] : []));
          const required = labels.filter((l) => /reaction|Reaction/u.test(l.id)).length;
          expect(labels.length).toBeLessThanOrEqual(Math.max(labelBudget(denominator), required));
          const boxes = labels.map((t) => ({ x0: t.x, x1: t.x + t.text.length * t.sizeMm * 0.56, y0: t.y - t.sizeMm * 0.8, y1: t.y + t.sizeMm * 0.2 }));
          boxes.forEach((a, i) => boxes.slice(i + 1).forEach((b) => expect(a.x0 < b.x1 && b.x0 < a.x1 && a.y0 < b.y1 && b.y0 < a.y1).toBe(false)));
        }
      });
    it("draws foliage regions neutral in the forces and moments views", () => {
      const state = evaluate(oak, { kind: "pose", pose: "windy" });
      const scenes = depictAll(state, { ...defaultDepictionOptions("forces", "en", 100) });
      const foliage = state.shapes.elevation.filter((e) => e.closed && e.shape.filled && oak.thermal.regions.find((r) => r.id === e.shape.region)?.kind === "foliage");
      expect(foliage.length).toBeGreaterThan(0);
      for (const view of ["forces", "moments"] as const) {
        const fills = new Map(items(scenes[view]).flatMap((i) => (i.kind === "path" && i.fill ? [[i.id, i.fill] as const] : [])));
        for (const e of foliage) {
          const fill = fills.get(`shape:${e.shape.id}`);
          if (fill) expect(fill.color).toBe(toneColour("light", e.shape.tone));
        }
      }
    });
    it("labels a distributed load once, with its resultant", () => {
      const scenes = depictAll(evaluate(oak, { kind: "pose", pose: "windy" }), { ...defaultDepictionOptions("forces", "en", 50) });
      const texts = items(scenes.forces).flatMap((i) => (i.kind === "text" ? [i.text] : []));
      expect(texts.filter((t) => /distributed/u.test(t)).length).toBeLessThanOrEqual(2);
      expect(texts.filter((t) => /#\d/u.test(t))).toEqual([]);
    });
  });
});
