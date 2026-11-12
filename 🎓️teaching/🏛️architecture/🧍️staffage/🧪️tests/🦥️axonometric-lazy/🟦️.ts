/** 🦥️ The axonometric depiction is a chunk of its own: before it is loaded the synchronous `depict` and `modelScene` refuse the viewpoint with a named, bilingual error, `loadAxonometric` and the asynchronous `depictAsync` and `modelSceneAsync` load it once, and the planar viewpoints never need it. Oracle: the planar depiction of the same state (never loaded) and the synchronous axonometric depiction after the load.
 *
 * @see ./🥒️.feature — the scenarios */
import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";
import type { AssetDocument } from "../../🧬️schema/🟦️.ts";
import { AxonometricNotLoadedError, defaultDepictionOptions, depict, depictAsync, loadAxonometric, modelScene, modelSceneAsync } from "../../🔨️modules/🎨️depiction/🟦️.ts";
import { writeSvg } from "../../🔨️modules/📤️export/🖼️svg/🟦️.ts";
import { evaluate } from "../../🔨️modules/🧮️evaluation/🟦️.ts";

const asset = (JSON.parse(readFileSync(new URL("../../🧫️fixtures/👤️sample-figure/🔣️.json", import.meta.url), "utf8")) as AssetDocument).asset;
const state = evaluate(asset, { kind: "pose", pose: "stand" }, { kind: "scale", scale: { denominator: 100 } });
const options = defaultDepictionOptions("light-line", "en", 100, "axonometric");

describe("🦥️ the axonometric depiction loads on demand", () => {
  it("draws the planar viewpoints before anything is loaded", () => {
    expect(depict(state, defaultDepictionOptions("light-line", "en", 100, "elevation")).metadata.viewpoint).toBe("elevation");
    expect(modelScene(state, { viewpoint: "plan" }).layers.length).toBeGreaterThan(0);
  });

  it("refuses the axonometric viewpoint before it is loaded, in both languages, in the synchronous calls", () => {
    expect(() => depict(state, options)).toThrow(AxonometricNotLoadedError);
    expect(() => modelScene(state, { viewpoint: "axonometric" })).toThrow(AxonometricNotLoadedError);
    try {
      depict(state, options);
    } catch (error) {
      const reason = (error as AxonometricNotLoadedError).reason;
      expect(reason.en).toContain("loadAxonometric");
      expect(reason.de).toContain("loadAxonometric");
      expect(reason.de).not.toBe(reason.en);
    }
  });

  it("loads once through depictAsync and then equals the synchronous depiction byte for byte", async () => {
    const asynchronous = await depictAsync(state, options);
    expect(asynchronous.metadata.viewpoint).toBe("axonometric");
    expect(writeSvg(depict(state, options))).toBe(writeSvg(asynchronous));
  });

  it("loads idempotently and gives the CAD scene through modelSceneAsync", async () => {
    await Promise.all([loadAxonometric(), loadAxonometric()]);
    const cad = await modelSceneAsync(state, { viewpoint: "axonometric" });
    expect(cad.layers.length).toBeGreaterThan(0);
    expect(JSON.stringify(modelScene(state, { viewpoint: "axonometric" }))).toBe(JSON.stringify(cad));
  });
});
