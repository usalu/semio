/** 🤐️ A depiction names a language only when it renders text: textless options carry none, every view draws without a text item, and text asked of them fails instead of falling back on a language.
 * @see ./🥒️.feature */
import { describe, expect, it } from "vitest";
import { VIEWS, type DepictionOptions, type DrawingScene } from "../../🔨️modules/🔗️contracts/🟦️.ts";
import { defaultDepictionOptions, depict, depictAll, textlessDepictionOptions } from "../../🔨️modules/🎨️depiction/🟦️.ts";
import { sampleState, unchecked } from "../../🧫️fixtures/🧱️synthetic-figure/🟦️.ts";

const texts = (scene: DrawingScene): string[] => scene.layers.flatMap((layer) => layer.items.flatMap((item) => (item.kind === "text" ? [item.text] : [])));

describe("textless depiction", () => {
  const state = sampleState("stand", 50);

  it("carries no language and no text annotation", () => {
    const options = textlessDepictionOptions("forces", 50);
    expect("language" in options).toBe(false);
    expect(options.annotations).toEqual({ dimensions: false, scaleBar: false, legend: false, labels: false, ground: true });
    expect(options.diagrams.freeBody).toBe(false);
  });

  for (const view of VIEWS) {
    it(`draws the ${view} view without a text item`, () => {
      const scene = depict(state, unchecked(textlessDepictionOptions(view, 50)));
      expect(texts(scene)).toEqual([]);
      expect(scene.layers.some((layer) => layer.items.length > 0)).toBe(true);
    });
  }

  it("draws all seven views at once", () => {
    const { view: _view, ...options } = unchecked(textlessDepictionOptions("dark-line", 50));
    const scenes = depictAll(state, options);
    for (const view of VIEWS) expect(texts(scenes[view])).toEqual([]);
  });

  it("fails when text is forced on without a language", () => {
    const forced = { ...textlessDepictionOptions("dark-line", 50), annotations: { dimensions: false, scaleBar: false, legend: false, labels: true, ground: true } } as unknown as DepictionOptions;
    expect(() => depict(state, unchecked(forced))).toThrow(/textless depiction has no language/u);
  });

  it("refuses text annotations and the free-body inset by type", () => {
    // @ts-expect-error a textless depiction cannot switch labels on
    const labelled: DepictionOptions = { ...textlessDepictionOptions("dark-line", 50), annotations: { dimensions: false, scaleBar: false, legend: false, labels: true, ground: true } };
    // @ts-expect-error a textless depiction cannot draw the free-body inset
    const inset: DepictionOptions = { ...textlessDepictionOptions("forces", 50), diagrams: { axial: false, shear: false, moment: true, freeBody: true } };
    expect([labelled, inset]).toHaveLength(2);
  });
});

describe("textual depiction", () => {
  const state = sampleState("stand", 50);

  it("requires a language and draws text in it", () => {
    const en = texts(depict(state, unchecked(defaultDepictionOptions("dark-line", "en", 50))));
    const de = texts(depict(state, unchecked(defaultDepictionOptions("dark-line", "de", 50))));
    expect(en.length).toBeGreaterThan(0);
    expect(en.some((t) => /^\d+\.\d+ m$/u.test(t))).toBe(true);
    expect(de.some((t) => /^\d+,\d+ m$/u.test(t))).toBe(true);
  });

  it("carries the named language", () => {
    expect(defaultDepictionOptions("thermal", "de", 50).language).toBe("de");
  });
});
