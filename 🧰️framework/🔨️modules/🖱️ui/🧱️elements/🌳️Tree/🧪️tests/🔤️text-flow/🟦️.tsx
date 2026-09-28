/** 🔤️ Chromium checks actual label reading order independently of panel placement. */
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { renderToStaticMarkup } from "react-dom/server";
import { compile } from "@tailwindcss/node";
import Ajv2020 from "ajv/dist/2020.js";
import { chromium } from "playwright";
import { describe, expect, it } from "vitest";
import { TreeItem } from "../../🟦️.tsx";
import fixture from "../../🧫️fixtures/🔤️text-flow/🔣️.json";
import schema from "../../🧬️schema/🔤️text-flow/🔣️.json";

const ui = resolve(dirname(fileURLToPath(import.meta.url)), "../../../..");
const cssPath = resolve(ui, "🎨️styling/🖌️ui/🎨️.css");

describe("Tree label reading order", () => {
  it("validates the neutral panel-flow and bilingual label contract", () => {
    expect(new Ajv2020({ strict: true }).compile(schema)(fixture)).toBe(true);
  });

  it("preserves label text order while panel layout flows in either direction", async () => {
    const markup = fixture.flows.flatMap(flow => fixture.labels.map((entry, index) => renderToStaticMarkup(
      <section dir={flow} lang={entry.locale} data-case={flow + "-" + index} style={{ width: 300 }}>
        <TreeItem id={flow + "-" + index} label={<span className="min-w-0 truncate">{entry.text}</span>} activatable onClick={() => {}} />
      </section>,
    ))).join("");
    const compiler = await compile(readFileSync(cssPath, "utf8"), { base: dirname(cssPath), onDependency: () => {} });
    const classes = [...markup.matchAll(/class="([^"]*)"/g)].flatMap(match => match[1].split(" "));
    const styles = compiler.build(classes) + readFileSync(resolve(ui, "🌐️globals/🎨️.css"), "utf8");
    const browser = await chromium.launch({ headless: true });
    try {
      const page = await browser.newPage();
      await page.setContent(markup);
      await page.addStyleTag({ content: styles });
      for (const flow of fixture.flows) for (const [index, entry] of fixture.labels.entries()) {
        const result = await page.locator('[data-case="' + flow + "-" + index + '"] [data-slot="tree-label"]').evaluate(element => {
          const walker = document.createTreeWalker(element, NodeFilter.SHOW_TEXT);
          const characters: { text: string; x: number }[] = [];
          while (walker.nextNode()) {
            const node = walker.currentNode;
            for (let offset = 0; offset < (node.textContent ?? "").length; offset++) {
              const range = document.createRange();
              range.setStart(node, offset);
              range.setEnd(node, offset + 1);
              characters.push({ text: node.textContent![offset], x: range.getBoundingClientRect().x });
            }
          }
          return { visual: characters.sort((a, b) => a.x - b.x).map(row => row.text).join(""), direction: getComputedStyle(element).direction };
        });
        expect(result.direction).toBe(flow);
        expect(result.visual, flow + ":" + entry.locale + ":" + entry.text).toBe(entry.visual);
      }
      console.info("[DEBUG] Chromium preserved twelve bilingual Tree label orders across both panel directions");
    } finally {
      await browser.close();
    }
  }, 30_000);
});

