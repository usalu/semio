/** 📐️ Overall dimension of a viewpoint: the elevation labels the rig height, the plan labels the drawn depth along the paper's vertical axis (the 0.52 m gas-hob plate), never the elevation height.
 *
 * THE REFERENCE is jsdom reading the written SVG (extension-line distance in paper millimetres over the scale, the label text) against the catalog `depthM` and `heightM`.
 *
 * @see ./🥒️.feature — the scenarios
 */
import { JSDOM } from "jsdom";
import { beforeAll, describe, expect, it } from "vitest";
import type { Asset, Viewpoint } from "../../🧬️schema/🟦️.ts";
import { DEFAULT_CATALOG_ROOT, loadCatalogFromFiles } from "../../🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { defaultDepictionOptions, depict } from "../../🔨️modules/🎨️depiction/🟦️.ts";
import { evaluate } from "../../🔨️modules/🧮️evaluation/🟦️.ts";
import { writeSvg } from "../../🔨️modules/📤️export/🖼️svg/🟦️.ts";

const DENOMINATOR = 50;
let hob: Asset;

beforeAll(async () => {
  hob = (await loadCatalogFromFiles(DEFAULT_CATALOG_ROOT)).assets.find((a) => a.id === "gas-hob")!;
});

function dimensionOf(viewpoint: Viewpoint): { quantity: string; value: number; label: string; spanMm: number } {
  const selector = { kind: "scale", scale: { denominator: DENOMINATOR } } as const;
  const state = evaluate(hob, { kind: "pose", pose: hob.presentation.defaultPose }, selector);
  const o = defaultDepictionOptions("dark-line", "en", DENOMINATOR, viewpoint);
  const svg = writeSvg(depict(state, { ...o, representation: selector, annotations: { ...o.annotations, legend: false } }));
  const document = new JSDOM(svg, { contentType: "image/svg+xml" }).window.document;
  const line = document.getElementById("dimension-line")!;
  const y = (id: string): number => Number(/^M[\d.-]+ ([\d.-]+)L/u.exec(document.getElementById(id)!.getAttribute("d")!)![1]);
  const label = [...document.querySelectorAll("text")].map((t) => t.textContent ?? "").find((t) => / m$/u.test(t))!;
  return { quantity: line.getAttribute("data-quantity")!, value: Number(line.getAttribute("data-value")), label, spanMm: y("dimension-extension-bottom") - y("dimension-extension-top") };
}

describe("overall dimension per viewpoint", () => {
  it("labels the rig height in the elevation", () => {
    const d = dimensionOf("elevation");
    expect(d.quantity).toBe("height");
    expect(d.value).toBeCloseTo(hob.dimensions.heightM, 3);
    expect(d.label).toBe("0.09 m");
    expect(d.spanMm / (1000 / DENOMINATOR)).toBeCloseTo(hob.dimensions.heightM, 3);
  });

  it("labels the drawn depth in the plan, not the height", () => {
    const d = dimensionOf("plan");
    expect(d.quantity).toBe("depth");
    expect(d.value).toBeCloseTo(hob.dimensions.depthM, 3);
    expect(d.label).toBe("0.52 m");
    expect(d.spanMm / (1000 / DENOMINATOR)).toBeCloseTo(hob.dimensions.depthM, 2);
  });
});
