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
  const classes = [...corpus.sizes.flatMap(row => utilities.map(([prefix]) => prefix + "-[" + row.rem + "]")), ...corpus.rounding.map(row => row.semanticClass)];
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
    for (const root of corpus.rootFontSizesPx) for (const row of corpus.rounding) {
      await page.setContent("<style>" + css + "</style><div id=\"capsule\"></div>");
      await page.evaluate(({ root, row }) => {
        document.documentElement.style.fontSize = root + "px";
        document.body.style.margin = "0";
        document.body.style.background = "white";
        const element = document.getElementById("capsule")!;
        element.style.width = row.widthRem + "rem";
        element.style.height = row.heightRem + "rem";
        element.style.background = "black";
        element.style.borderRadius = Math.min(row.widthRem, row.heightRem) * root / 2 + "px";
      }, { root, row });
      const shape = () => page.locator("#capsule").evaluate(element => {
        const box = element.getBoundingClientRect(), radius = Math.min(box.width, box.height) / 2;
        const hit = (x: number, y: number) => document.elementFromPoint(box.x + x, box.y + y) === element;
        const corners = [false, true].flatMap(right => [false, true].map(bottom => hit(right ? box.width - radius * 0.05 : radius * 0.05, bottom ? box.height - radius * 0.05 : radius * 0.05)));
        return { width: box.width, height: box.height, center: hit(box.width / 2, box.height / 2), corners };
      });
      const reference = await shape();
      await page.locator("#capsule").evaluate((element, className) => {
        element.style.borderRadius = "";
        element.className = className;
      }, row.semanticClass);
      const actual = await shape();
      assert.equal(reference.center, true, row.id + " native reference center");
      assert.deepEqual(reference.corners, [false, false, false, false], row.id + " native reference corners");
      assert.deepEqual(actual, reference, row.id + " capsule bounds, center and clipped corners at " + root + "px");
      const radius = await page.locator("#capsule").evaluate(element => Number.parseFloat(getComputedStyle(element).borderTopLeftRadius));
      assert.equal(radius >= Math.min(actual.width, actual.height) / 2, true, row.id + " full capsule radius at " + root + "px");
      assertions += 4;
    }
    return assertions;
  } finally {
    await browser.close();
  }
}
