/** 📐️ The overall dimension of small objects: when the span between the extension lines is shorter than the value is tall, the value moves above the upper extension line on a leader instead of sitting on the extension lines and ticks.
 *
 * THE REFERENCE is jsdom reading the written SVG: the text box (font size over the baseline) against the paper y of both extension lines and both ticks.
 *
 * @see ./🥒️.feature — the scenarios
 */
import { JSDOM } from "jsdom";
import { beforeAll, describe, expect, it } from "vitest";
import type { Asset } from "../../🧬️schema/🟦️.ts";
import { DEFAULT_CATALOG_ROOT, loadCatalogFromFiles } from "../../🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { DIMENSION_FIT_MM } from "../../🔨️modules/🎨️depiction/🏷️annotations/🟦️.ts";
import { defaultDepictionOptions, depict } from "../../🔨️modules/🎨️depiction/🟦️.ts";
import { evaluate } from "../../🔨️modules/🧮️evaluation/🟦️.ts";
import { writeSvg } from "../../🔨️modules/📤️export/🖼️svg/🟦️.ts";

const DENOMINATOR = 50;
const TEXT_ABOVE = 0.8;
const TEXT_BELOW = 0.25;
let catalog: readonly Asset[];

beforeAll(async () => {
  catalog = (await loadCatalogFromFiles(DEFAULT_CATALOG_ROOT)).assets;
});

type Layout = { spanMm: number; leader: boolean; textTop: number; textBottom: number; lines: number[]; ticks: number[]; textX: number; tickRight: number; label: string };

function layoutOf(id: string): Layout {
  const asset = catalog.find((candidate) => candidate.id === id)!;
  const selector = { kind: "scale", scale: { denominator: DENOMINATOR } } as const;
  const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, selector);
  const o = defaultDepictionOptions("dark-line", "en", DENOMINATOR, "elevation");
  const svg = writeSvg(depict(state, { ...o, representation: selector, annotations: { ...o.annotations, legend: false } }));
  const document = new JSDOM(svg, { contentType: "image/svg+xml" }).window.document;
  const numbers = (element: string): number[] => document.getElementById(element)!.getAttribute("d")!.match(/-?\d+(?:\.\d+)?/gu)!.map(Number);
  const top = numbers("dimension-extension-top")[1]!;
  const bottom = numbers("dimension-extension-bottom")[1]!;
  const text = document.getElementById("dimension-text")!;
  const size = Number(text.getAttribute("font-size"));
  const baseline = Number(text.getAttribute("y"));
  const tickTop = numbers("dimension-tick-top");
  const tickBottom = numbers("dimension-tick-bottom");
  return {
    spanMm: bottom - top,
    leader: document.getElementById("dimension-leader") !== null,
    textTop: baseline - size * TEXT_ABOVE,
    textBottom: baseline + size * TEXT_BELOW,
    lines: [top, bottom],
    ticks: [Math.min(tickTop[1]!, tickTop[3]!), Math.max(tickTop[1]!, tickTop[3]!), Math.min(tickBottom[1]!, tickBottom[3]!), Math.max(tickBottom[1]!, tickBottom[3]!)],
    textX: Number(text.getAttribute("x")),
    tickRight: Math.max(tickTop[0]!, tickTop[2]!, tickBottom[0]!, tickBottom[2]!),
    label: text.textContent ?? "",
  };
}

describe("dimension label on small objects at 1:50", () => {
  for (const [id, label] of [["tealight", "0.04 m"], ["thermostatic-radiator-valve", undefined]] as const)
    it(`lifts the label of the ${id} clear of the extension lines and ticks`, () => {
      const layout = layoutOf(id);
      expect(layout.spanMm).toBeLessThan(DIMENSION_FIT_MM);
      expect(layout.leader).toBe(true);
      if (label !== undefined) expect(layout.label).toBe(label);
      expect(layout.textBottom).toBeLessThan(Math.min(...layout.lines, ...layout.ticks));
      expect(layout.textX).toBeGreaterThan(layout.tickRight);
    });

  it("keeps the label between the extension lines of an object tall enough for it", () => {
    const layout = layoutOf("standing-adult");
    expect(layout.spanMm).toBeGreaterThanOrEqual(DIMENSION_FIT_MM);
    expect(layout.leader).toBe(false);
    expect(layout.textTop).toBeGreaterThan(layout.lines[0]!);
    expect(layout.textBottom).toBeLessThan(layout.lines[1]!);
  });
});
