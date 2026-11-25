/** 🧬️ The parsed SVG templates are bounded by what they weigh, not by how many they are: a live figure of a heavy asset shows a different markup of thousands of nodes on every frame, and a thousand kept fragments of those are gigabytes of detached nodes.
 * @see ./🥒️.feature — the scenarios
 * @see ../../🔨️modules/🧮️pipeline/🧬️svg/🟦️.ts — the template cache */
import { afterEach, describe, expect, it, vi } from "vitest";
import { SVG_TEMPLATE_BUDGET_CHARACTERS, svgTemplateLoad, writeSvgMarkup } from "../../🔨️modules/🧮️pipeline/🧬️svg/🟦️.ts";

const frame = (index: number, segments: number): string => `<svg viewBox="0 0 1 1"><path data-frame="${index}" d="${"M0 0L1 1".repeat(segments)}"/></svg>`;

afterEach(() => void vi.restoreAllMocks());

describe("🧬️ svg templates", () => {
  it("never keeps more template weight than the budget and shows the latest frame", () => {
    const host = document.createElement("div");
    const heavy = frame(0, 20_000).length;
    const frames = Math.ceil((SVG_TEMPLATE_BUDGET_CHARACTERS / heavy) * 3);
    let peak = 0;
    for (let index = 0; index < frames; index += 1) {
      writeSvgMarkup(host, frame(index, 20_000));
      peak = Math.max(peak, svgTemplateLoad().characters);
    }
    expect(peak).toBeLessThanOrEqual(SVG_TEMPLATE_BUDGET_CHARACTERS);
    expect(peak).toBeGreaterThan(SVG_TEMPLATE_BUDGET_CHARACTERS / 2);
    expect(svgTemplateLoad().templates).toBeLessThan(frames);
    expect(host.querySelector("path")?.getAttribute("data-frame")).toBe(String(frames - 1));
  });

  it("parses each frame of a short loop once", () => {
    const host = document.createElement("div");
    const created = vi.spyOn(document, "createElement");
    for (let cycle = 0; cycle < 2; cycle += 1) for (let index = 0; index < 12; index += 1) writeSvgMarkup(host, frame(1000 + index, 10));
    expect(created.mock.calls.filter(([tag]) => tag === "template")).toHaveLength(12);
    expect(host.querySelector("path")?.getAttribute("data-frame")).toBe("1011");
  });

  it("shows a frame heavier than the whole budget without keeping it", () => {
    const host = document.createElement("div");
    const before = svgTemplateLoad();
    const giant = frame(5000, Math.ceil(SVG_TEMPLATE_BUDGET_CHARACTERS / 8) + 1);
    expect(giant.length).toBeGreaterThan(SVG_TEMPLATE_BUDGET_CHARACTERS);
    writeSvgMarkup(host, giant);
    expect(host.querySelector("path")?.getAttribute("data-frame")).toBe("5000");
    expect(svgTemplateLoad()).toEqual(before);
  });
});
