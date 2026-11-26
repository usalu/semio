import { test, expect } from "bun:test";
import { colord, extend } from "colord";
import a11yPlugin from "colord/plugins/a11y";
import { contrastRatio } from "@semio-tech/ui-styling";
import { readFileSync } from "node:fs";
import { evaluate } from "mathjs";
import { loadPrintDesignTokens, renderChipGlass, renderPrintLatexTokenStylesheet } from "../🧮️resolution/🟦️.ts";
import fixture from "./🔣️.json";
extend([a11yPlugin]);
const stylesheet = renderPrintLatexTokenStylesheet(), paints = new Map([...stylesheet.matchAll(/\\definecolor\{([^}]+)\}\{HTML\}\{([^}]+)\}/g)].map(row => [row[1]!, "#" + row[2]!]));
for (const theme of fixture.themes) for (const pair of fixture.pairs) test("Print " + theme + " " + pair.role + " clears normal text contrast", () => {
  const text = paints.get("semio-chrome-" + theme + "-" + pair.text), surface = paints.get(pair.surface === "primary" || pair.surface === "secondary" || pair.surface === "tertiary" ? "semio-" + pair.surface : "semio-chrome-" + theme + "-" + pair.surface);
  expect(text).toBeDefined(); expect(surface).toBeDefined();
  const actual = contrastRatio(text!, surface!), oracle = colord(text!).contrast(surface!);
  expect(actual).toBeGreaterThanOrEqual(fixture.minimum);
  expect(Math.floor(actual * 100) / 100).toBeCloseTo(oracle, 2);
});
for (const row of fixture.regressions) test("Print rejects prior unreadable pair " + row.foreground + "/" + row.background, () => {
  expect(contrastRatio(row.foreground, row.background)).toBeLessThan(row.maximum);
  expect(colord(row.foreground).contrast(row.background)).toBeLessThan(row.maximum);
});

for (const row of fixture.chips) test("Print " + row.theme + " " + row.level + " chip clears normal text contrast after compositing", () => {
  const { levels } = loadPrintDesignTokens();
  const alpha = 1 - levels!.names.indexOf(row.level) * levels!.glassAlphaStep;
  const tint = levels!.chipTintPercent / 100;
  const paint = (name: string) => colord(paints.get("semio-chrome-" + row.theme + "-" + name)!);
  const foreground = paint("foreground"), interior = paint("window"), glass = paint(row.level).toRgb();
  const base = interior.toRgb();
  expect(interior.toHex()).toBe(row.interior);
  expect(paint("base").toHex()).toBe(row.margin);
  expect(paint(row.level).toHex()).toBe(row.glass);
  const text = foreground.toRgb();
  const channel = (component: "r" | "g" | "b") => Number(evaluate("round((surface * alpha + base * (1 - alpha)) * (1 - tint) + label * tint)", {
    surface: glass[component], base: base[component], label: text[component], alpha, tint,
  }));
  const composite = colord({ r: channel("r"), g: channel("g"), b: channel("b") }).toHex();
  expect(composite).toBe(row.interiorComposite);
  expect(foreground.contrast(composite)).toBeGreaterThanOrEqual(fixture.minimum);
  expect(contrastRatio(foreground.toHex(), composite)).toBeGreaterThanOrEqual(fixture.minimum);
});

test("Print chip glass is projected from the levels of the token source and matches independent arithmetic", () => {
  const { levels } = loadPrintDesignTokens();
  const sheet = renderPrintLatexTokenStylesheet();
  expect(sheet).toContain(renderChipGlass().join("\n"));
  expect(readFileSync(new URL("../../../🖋️latex/semio-tokens.sty", import.meta.url), "utf8")).toBe(sheet);
  levels!.names.forEach((name, index) => expect(sheet).toContain("\\newcommand{\\semio@glass@alpha@" + name + "}{" + Number(evaluate("round(1 - k * s, 4)", { k: index, s: levels!.glassAlphaStep })) + "}"));
  expect(sheet).toContain("\\newcommand{\\semio@chip@tint}{" + Number(evaluate("t / 100", { t: levels!.chipTintPercent })) + "}");
  const shown = Number(evaluate("(1 - a) * (1 - t)", { a: 1 - levels!.glassAlphaStep, t: levels!.chipTintPercent / 100 }));
  expect(shown).toBeCloseTo(0.108, 3);
});
test("Print rejects a token source without levels instead of falling back to copied values", () => {
  const tokens = loadPrintDesignTokens();
  expect(() => renderChipGlass({ ...tokens, levels: undefined })).toThrow("tokens.levels missing");
});
