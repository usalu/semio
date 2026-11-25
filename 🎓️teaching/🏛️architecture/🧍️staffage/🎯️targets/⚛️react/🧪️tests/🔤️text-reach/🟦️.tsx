/** 🔤️ The window around a drawing takes in its texts as the page sets them: the overhang of the widest text past its reserved room, steady while a caption counts time, and none where nothing can be measured.
 * @see ./🥒️.feature — the scenarios
 * @see ../../🔨️modules/🧮️pipeline/🔤️text/🟦️.ts — the measure
 * @see ../../../../🌐️site/🧪️tests/🏷️label-fit/🟦️.ts — the same in Chromium, against the boxes the browser lays out */
import { describe, expect, it } from "vitest";
import { TEXT_SLACK } from "../../../../🔨️modules/🎨️depiction/🧱️canvas/🟦️.ts";
import { textWidthMm } from "../../../../🔨️modules/📐️units/🟦️.ts";
import type { DrawingScene } from "@teaching/staffage";
import { figureFrame, sceneExtent, sceneWindow } from "../../🔨️modules/🧮️pipeline/🟦️.ts";
import { textOverhang, type TextMeasure } from "../../🔨️modules/🧮️pipeline/🔤️text/🟦️.ts";
import { walker } from "../🧰️support/🧰️.ts";

const request = { view: "dark-line", scale: { denominator: 100 }, language: "en", viewpoint: "elevation" } as const;
const clip = walker.presentation.defaultClip;
const at = (timeS: number): DrawingScene => figureFrame(walker, { kind: "clip", clip, timeS }, request).scene;
const texts = (scene: DrawingScene) => scene.layers.flatMap((layer) => layer.items.flatMap((item) => (item.kind === "text" ? [item] : [])));
const reserved = (item: ReturnType<typeof texts>[number]): number => textWidthMm(item.text, item.sizeMm, item.weight === "bold") * TEXT_SLACK;
const em: TextMeasure = (text) => [...text].length;
const lean: TextMeasure = (text) => [...text].reduce((sum, char) => sum + (char === "1" ? 0.3 : 0.6), 0);

describe("🔤️ text reach", () => {
  it("needs no window while every text fits its room", () => {
    const scene = at(0);
    expect(texts(scene).length).toBeGreaterThan(1);
    expect(textOverhang(scene, () => 0)).toEqual({ left: 0, right: 0 });
  });

  it("states how far the widest text reaches past its room on each side", () => {
    const scene = at(0);
    const more = (anchor: string): number => Math.max(0, ...texts(scene).filter((item) => item.anchor === anchor).map((item) => [...item.text].length * item.sizeMm - reserved(item)));
    const overhang = textOverhang(scene, em);
    expect(overhang.right).toBeCloseTo(Math.max(more("start"), more("middle") / 2), 9);
    expect(overhang.left).toBeCloseTo(Math.max(more("end"), more("middle") / 2), 9);
    expect(overhang.right).toBeGreaterThan(0);
  });

  it("keeps the overhang of a caption steady while its time runs", () => {
    const first = at(1.11);
    const second = at(0.8);
    const caption = (scene: DrawingScene): string => texts(scene).find((item) => item.text.includes(" @ "))!.text;
    expect(caption(first)).toMatch(/@ 1\.1\d s$/u);
    expect(caption(second)).toMatch(/@ 0\.80 s$/u);
    const steady = (scene: DrawingScene): number => {
      const item = texts(scene).find((candidate) => candidate.text.includes(" @ "))!;
      return [...item.text].length * 0.6 * item.sizeMm;
    };
    expect(steady(first)).toBeCloseTo(steady(second), 9);
    expect(textOverhang(first, lean).right).toBeGreaterThan(0);
    expect(textOverhang(first, lean).right - textOverhang(second, lean).right).toBeCloseTo(reserved(texts(second).find((item) => item.text.includes(" @ "))!) - reserved(texts(first).find((item) => item.text.includes(" @ "))!), 9);
  });

  it("is the paper of the scene where nothing can be measured", () => {
    const scene = at(0);
    expect(typeof OffscreenCanvas).toBe("undefined");
    expect(sceneWindow(scene)).toBeUndefined();
    expect(sceneExtent(scene)).toEqual({ minX: -scene.metadata.origin.xMm, minY: -scene.metadata.origin.yMm, maxX: scene.widthMm - scene.metadata.origin.xMm, maxY: scene.heightMm - scene.metadata.origin.yMm });
  });
});
