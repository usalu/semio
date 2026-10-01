import assert from "node:assert/strict";
import Ajv from "ajv";
import { compile } from "tailwindcss";
import { chromium } from "playwright";
import corpus from "../../🧫️fixtures/📏️relative-sizing/🔣️.json";
import schema from "../../🧬️schema/📏️relative-sizing/🔣️.json";

const utilities = [
  ["w", "width"], ["h", "height"], ["min-w", "minWidth"], ["max-w", "maxWidth"],
  ["min-h", "minHeight"], ["max-h", "maxHeight"], ["gap", "gap"],
  ["text", "fontSize"], ["leading", "lineHeight"],
] as const;

/** 📏️Proves authored relative dimensions through actual Tailwind utilities and Chromium. */
export async function proveRelativeStylingSizesV1(): Promise<number> {
  assert.equal(new Ajv({ strict: true }).compile(schema)(corpus), true);
  for (const row of corpus.sizes) {
    assert.equal(row.pixels.length, corpus.rootFontSizesPx.length);
    assert.deepEqual(corpus.rootFontSizesPx.map(font => Number.parseFloat(row.rem) * font), row.pixels);
  }
  const compiler = await compile("@tailwind utilities;");
  const classes = corpus.sizes.flatMap(row => utilities.map(([prefix]) => prefix + "-[" + row.rem + "]"));
  const css = compiler.build(classes);
  const browser = await chromium.launch({ headless: true });
  try {
    const page = await browser.newPage();
    await page.setContent("<style>" + css + "</style>");
    const measurements = await page.evaluate(({ sizes, roots, utilities }) => roots.map(root => {
      document.documentElement.style.fontSize = root + "px";
      return sizes.map(row => utilities.map(([prefix, property]) => {
        const element = document.createElement("div");
        element.className = prefix + "-[" + row.rem + "]";
        document.body.append(element);
        const value = Number.parseFloat(getComputedStyle(element)[property]);
        element.remove();
        return value;
      }));
    }), { sizes: corpus.sizes, roots: corpus.rootFontSizesPx, utilities });
    let assertions = 0;
    corpus.rootFontSizesPx.forEach((root, rootIndex) => corpus.sizes.forEach((row, sizeIndex) => utilities.forEach(([prefix], utilityIndex) => {
      assert.equal(measurements[rootIndex]![sizeIndex]![utilityIndex], row.pixels[rootIndex], prefix + "-[" + row.rem + "] at " + root + "px");
      assertions++;
    })));
    return assertions;
  } finally {
    await browser.close();
  }
}
