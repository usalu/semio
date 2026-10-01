import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import Ajv from "ajv";
import colorString from "color-string";
import { chromium } from "playwright";
import { createElement, Fragment } from "react";
import { renderToStaticMarkup } from "react-dom/server";
import { DataTable, SwatchGrid, galleryCardStyle } from "../../../📖️stories/🧭️coordination/🟦️.tsx";
import corpus from "../../🧫️fixtures/🎭️color-primitives/🔣️.json";
import schema from "../../🧬️schema/🎭️color-primitives/🔣️.json";

/** 🎭️Proves native primitive equivalence and customization of actual edge/gallery styles. */
export async function proveStylingColorPrimitivesV1(): Promise<number> {
  assert.equal(new Ajv({ strict: true }).compile(schema)(corpus), true);
  for (const row of corpus.colors) {
    assert.deepEqual(colorString.get(row.before)?.value, row.rgba, row.id + ": independent original color");
    assert.deepEqual(colorString.get(row.after)?.value, row.rgba, row.id + ": independent canonical color");
  }
  const css = readFileSync(new URL("../../../🖌️ui/🎨️.css", import.meta.url), "utf8");
  const edgeRule = /\.temp \.react-flow__edge-path\s*\{[^}]*\}/.exec(css)?.[0];
  assert.ok(edgeRule?.includes("stroke: var(--color-primary)"), "actual edge accent consumes the semantic variable");
  const browser = await chromium.launch({ headless: true });
  try {
    const page = await browser.newPage();
    await page.setContent("<style>" + edgeRule + "</style><svg class='temp'><path class='react-flow__edge-path'/></svg>");
    const result = await page.evaluate(colors => {
      const properties = ["color", "boxShadow", "maskImage"] as const;
      const observed = colors.map(row => properties.map(property => {
        const evaluate = (color: string): string => {
          const element = document.createElement("div");
          element.style[property] = property === "color" ? color : property === "boxShadow" ? "0 1px 2px " + color : "linear-gradient(" + color + ", " + color + ")";
          document.body.append(element);
          const value = getComputedStyle(element)[property];
          element.remove();
          return value;
        };
        return [evaluate(row.before), evaluate(row.after)];
      }));
      const edge = document.querySelector(".react-flow__edge-path")!;
      const accents = ["black", "white"].map(color => {
        document.documentElement.style.setProperty("--color-primary", color);
        return getComputedStyle(edge).stroke;
      });
      const escaped = document.createElement("div");
      escaped.style.cssText = "color: \\23 abc;";
      return { observed, accents, escapedColor: escaped.style.color };
    }, corpus.colors);
    let assertions = 0;
    for (const properties of result.observed) for (const [before, after] of properties) {
      assert.notEqual(before, "none", "native primitive syntax is admitted");
      assert.equal(before, after, "native alpha and paint computation is unchanged");
      assertions++;
    }
    assert.deepEqual(result.accents, ["rgb(0, 0, 0)", "rgb(255, 255, 255)"]);
    assert.equal(result.escapedColor, "", "escaped hash is an identifier, not a CSS color");
    const gallery = renderToStaticMarkup(createElement(Fragment, null,
      createElement("article", { id: "gallery-card", style: galleryCardStyle }),
      createElement("div", { id: "gallery-swatches" }, createElement(SwatchGrid, { title: "Owned Palette", entries: [["sample", [255, 255, 255, 255]]] })),
      createElement("div", { id: "gallery-table" }, createElement(DataTable, { columns: ["Owned Token"], rows: [["sample"]] }))));
    await page.setContent(gallery);
    const chrome = await page.evaluate(rows => ["black", "white"].map(color => {
      document.documentElement.style.setProperty("--border-normal-color", color);
      return rows.map(row => {
        const element = document.querySelector(row.selector);
        if (!element) throw Error("Missing actual gallery element: " + row.id);
        const paint = getComputedStyle(element).getPropertyValue(row.property === "borderTopColor" ? "border-top-color" : "border-bottom-color");
        const canvas = document.createElement("canvas");
        canvas.width = canvas.height = 1;
        const context = canvas.getContext("2d")!;
        context.fillStyle = paint;
        context.fillRect(0, 0, 1, 1);
        return [...context.getImageData(0, 0, 1, 1).data];
      });
    }), corpus.galleries);
    for (const [index, color] of ["black", "white"].entries()) for (const [rowIndex, row] of corpus.galleries.entries()) {
      const rgb = colorString.get(color)!.value.slice(0, 3);
      assert.deepEqual(chrome[index]![rowIndex], [...rgb, Math.round(row.alpha * 255)], row.id + ": native semantic border " + color);
      assertions++;
    }
    return assertions + result.accents.length + 1;
  } finally {
    await browser.close();
  }
}
