/** 🎨️ The seven modes from the selected representation, representation pens and the transport overlay hook; oracle: pens and shape ids are read back from the authored fixture figure.
 *
 * @see ./🥒️.feature — the scenarios
 * @see ../../🧫️fixtures/👤️sample-figure/🔣️.json — the figure
 */
import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";
import type { AssetDocument } from "../../🧬️schema/🟦️.ts";
import { VIEWS, type DrawingScene } from "../../🔨️modules/🔗️contracts/🟦️.ts";
import { defaultDepictionOptions, depict, depictAll } from "../../🔨️modules/🎨️depiction/🟦️.ts";
import { evaluate } from "../../🔨️modules/🧮️evaluation/🟦️.ts";
import { unchecked } from "../../🧫️fixtures/🧱️synthetic-figure/🟦️.ts";

const asset = (JSON.parse(readFileSync(new URL("../../🧫️fixtures/👤️sample-figure/🔣️.json", import.meta.url), "utf8")) as AssetDocument).asset;
const stateAt = (denominator: number) => evaluate(asset, { kind: "pose", pose: "stand" }, { kind: "scale", scale: { denominator } });
const fills = (scene: DrawingScene): string[] => scene.layers.flatMap((layer) => layer.items.flatMap((item) => (item.kind === "path" && item.id.startsWith("shape:") ? [item.id] : []))).sort();
const geometry = (scene: DrawingScene): string => JSON.stringify(scene.layers.flatMap((layer) => layer.items.flatMap((item) => (item.kind === "path" && item.id.startsWith("shape:") ? [[item.id, item.commands]] : []))));
const widths = (scene: DrawingScene, layer: string): number[] => scene.layers.filter((l) => l.id === layer).flatMap((l) => l.items.flatMap((i) => (i.kind === "path" && i.stroke ? [i.stroke.widthMm] : [])));

describe("🎨️ seven modes", () => {
  it("draws the same fill shapes in every mode and different shapes per scale", () => {
    const sets = new Map<number, string>();
    for (const denominator of [200, 100, 50] as const) {
      const options = unchecked(defaultDepictionOptions("light-line", "en", denominator));
      const scenes = depictAll(stateAt(denominator), options);
      const reference = fills(scenes["light-line"]);
      expect(reference.length).toBeGreaterThan(0);
      for (const view of VIEWS) if (view !== "forces" && view !== "moments") expect(fills(scenes[view]).length).toBeGreaterThan(0);
      for (const view of ["dark-line", "dark-solid", "light-solid"] as const) expect(fills(scenes[view])).toEqual(reference);
      const rep = asset.representations[`${denominator}`];
      if (rep.status !== "authored") throw new Error(`1:${denominator} is not authored`);
      expect(reference.every((id) => rep.geometry.elevation.some((shape) => `shape:${shape.id}` === id))).toBe(true);
      sets.set(denominator, geometry(scenes["light-line"]));
    }
    expect(new Set(sets.values()).size).toBe(3);
  });

  it("draws outlines with the body pen of the selected representation", () => {
    for (const denominator of [200, 50] as const) {
      const rep = asset.representations[`${denominator}`];
      if (rep.status !== "authored") throw new Error(`1:${denominator} is not authored`);
      const scene = depict(stateAt(denominator), unchecked(defaultDepictionOptions("light-line", "en", denominator)));
      const body = widths(scene, "body");
      expect(body.length).toBeGreaterThan(0);
      expect(new Set(body)).toEqual(new Set(rep.geometry.elevation.filter((s) => s.role === "body").map((s) => s.penMm ?? rep.pens.body)));
    }
  });

  it("refuses a state evaluated with another representation", () => {
    expect(() => depict(stateAt(100), unchecked(defaultDepictionOptions("light-line", "en", 50)))).toThrow(/different representation/u);
  });

  it("adds the transport overlay and its legend to any view", () => {
    const state = stateAt(100);
    for (const view of ["light-line", "thermal"] as const) {
      const plain = depict(state, unchecked(defaultDepictionOptions(view, "en", 100)));
      const withTransport = depict(state, { ...unchecked(defaultDepictionOptions(view, "en", 100)), transport: ["parcels", "streamlines"] });
      const count = (scene: DrawingScene): number => scene.layers.reduce((n, l) => n + l.items.length, 0);
      expect(count(withTransport)).toBeGreaterThan(count(plain));
      expect(withTransport.legends.length).toBeGreaterThan(plain.legends.length);
    }
  });
});
