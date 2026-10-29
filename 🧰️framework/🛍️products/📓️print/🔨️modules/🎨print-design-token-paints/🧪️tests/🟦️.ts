import { test, expect } from "bun:test";
import { colord, extend } from "colord";
import a11yPlugin from "colord/plugins/a11y";
import { contrastRatio } from "@semio-tech/ui-styling";
import { renderPrintLatexTokenStylesheet } from "../🧮️resolution/🟦️.ts";
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
