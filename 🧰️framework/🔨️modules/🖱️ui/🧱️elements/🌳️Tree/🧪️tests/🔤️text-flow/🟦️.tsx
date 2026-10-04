/** 🔤️ Chromium checks actual label reading order independently of panel placement. */
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { renderToStaticMarkup } from "react-dom/server";
import { compile } from "@tailwindcss/node";
import Ajv2020 from "ajv/dist/2020.js";
import { chromium } from "playwright";
import { describe, expect, it } from "vitest";
import { Tree, TreeContext, TreeItem } from "../../🟦️.tsx";
import { FlowProvider } from "../../../../🔨️modules/🧭️flow-direction-context/🟦️.tsx";
import fixture from "../../🧫️fixtures/🔤️text-flow/🔣️.json";
import schema from "../../🧬️schema/🔤️text-flow/🔣️.json";
import anchorFixture from "../../🧫️fixtures/📏️row-anchor/🔣️.json";
import anchorSchema from "../../🧬️schema/📏️row-anchor/🔣️.json";

const ui = resolve(dirname(fileURLToPath(import.meta.url)), "../../../..");
const cssPath = resolve(ui, "🎨️styling/🖌️ui/🎨️.css");

describe("Tree label reading order", () => {
  it("anchors standalone row guides to the inherited row, custom row or explicit center", async () => {
    const validate = new Ajv2020({ strict: true }).compile(anchorSchema);
    expect(validate(anchorFixture)).toBe(true);
    expect(validate({ ...anchorFixture, ambientRow: 40 })).toBe(false);
    const markup = anchorFixture.directions.flatMap(direction => anchorFixture.cases.map(entry => renderToStaticMarkup(
      <section data-anchor-case={direction + "-" + entry.id} style={{ width: 300, "--ui-spacing": anchorFixture.uiSpacingPx + "px", "--tree-indent-per-level": anchorFixture.indentPx + "px", "--tree-toggle-width": anchorFixture.togglePx + "px", "--size-small": entry.workbenchPx / 1.5 + "px", "--size-workbench": entry.workbenchPx + "px", "--tree-row-height": entry.rowPx === null ? undefined : entry.rowPx + "px", "--tree-gutter-center": entry.centerPx === null ? undefined : entry.centerPx + "px" } as React.CSSProperties}>
        <TreeContext.Provider value={{ level: 0, isLastAtLevel: [], showLines: true, isTree: false, indentMultiplier: 1, direction: direction as "down" | "up" }}>
          <TreeItem id={direction + "-" + entry.id} label="Guide" defaultOpen><span>Child</span></TreeItem>
        </TreeContext.Provider>
        <div data-anchor-reference style={{ position: "relative", height: entry.rowPx ?? anchorFixture.inheritedRowPx }}><span style={{ position: "absolute", top: entry.expectedPx }}>Reference</span></div>
      </section>,
    ))).join("");
    const compiler = await compile(readFileSync(cssPath, "utf8"), { base: dirname(cssPath), onDependency: () => {} });
    const classes = [...markup.matchAll(/class="([^"]*)"/g)].flatMap(match => match[1].split(" "));
    const browser = await chromium.launch({ headless: true });
    try {
      const page = await browser.newPage();
      await page.setContent(markup);
      await page.addStyleTag({ content: compiler.build(classes) + readFileSync(resolve(ui, "🌐️globals/🎨️.css"), "utf8") });
      for (const direction of anchorFixture.directions) for (const entry of anchorFixture.cases) {
        const result = await page.locator('[data-anchor-case="' + direction + "-" + entry.id + '"]').evaluate(element => {
          const row = element.querySelector('[data-slot="tree-item-row"]')!;
          const gutter = row.querySelector('[data-slot="tree-gutter"]')!;
          const slot = gutter.querySelector('[data-slot="tree-gutter-slot"]')!;
          const stem = gutter.querySelector('[data-slot="tree-branch-stem"]')!;
          const reference = element.querySelector('[data-anchor-reference]')!;
          const referenceMarker = reference.firstElementChild!;
          const stemRect = stem.getBoundingClientRect(), gutterRect = gutter.getBoundingClientRect();
          return { row: row.getBoundingClientRect().height, gutter: gutterRect.height, slot: parseFloat(getComputedStyle(slot).top), stemTop: stemRect.top - gutterRect.top, stemBottom: gutterRect.bottom - stemRect.bottom, reference: referenceMarker.getBoundingClientRect().top - reference.getBoundingClientRect().top };
        });
        console.info("[DEBUG] Tree row anchor", direction, entry.id, JSON.stringify(result));
        expect(result.row).toBe(entry.rowPx ?? anchorFixture.inheritedRowPx);
        expect(result.gutter).toBe(result.row);
        expect(result.reference).toBe(entry.expectedPx);
        expect(result.slot, direction + ":" + entry.id).toBe(result.reference);
        expect(direction === "up" ? result.stemBottom : result.stemTop).toBe(result.reference);
      }
      console.info("[DEBUG] Tree standalone guide geometry: six actual Chromium and independent reference layouts");
    } finally {
      await browser.close();
    }
  }, 30_000);

  it("keeps a retained Tree root in content reading order inside either panel flow", async () => {
    const validate = new Ajv2020({ strict: true }).compile(schema);
    expect(validate(fixture)).toBe(true);
    expect(validate({ ...fixture, root: { ...fixture.root, direction: "rtl" } })).toBe(false);
    expect(validate({ ...fixture, root: { ...fixture.root, panelMirror: true } })).toBe(false);
    const markup = fixture.flows.flatMap(flow => fixture.labels.map((entry, index) => renderToStaticMarkup(
      <section dir={flow} lang={entry.locale} data-case={flow + "-" + index} style={{ width: 300 }}>
        <FlowProvider inline={flow as "ltr" | "rtl"}>
          <Tree sections={[]} emptyState={<TreeItem id={flow + "-" + index} label={entry.text} />} />
        </FlowProvider>
      </section>,
    ))).join("");
    const browser = await chromium.launch({ headless: true });
    try {
      const page = await browser.newPage();
      await page.setContent(markup);
      for (const flow of fixture.flows) for (const [index, entry] of fixture.labels.entries()) {
        const result = await page.locator('[data-case="' + flow + "-" + index + '"]').evaluate(element => {
          const root = element.querySelector('[role="tree"]')!;
          const label = root.querySelector('[data-slot="tree-label"]')!;
          const text = label.firstChild!;
          const characters = Array.from(label.textContent!, (character, offset) => {
            const range = document.createRange();
            range.setStart(text, offset);
            range.setEnd(text, offset + 1);
            return { character, x: range.getBoundingClientRect().x };
          });
          return { panel: element.getAttribute("dir"), root: root.getAttribute("dir"), inline: getComputedStyle(label).direction, visual: characters.sort((a, b) => a.x - b.x).map(row => row.character).join("") };
        });
        expect(result.panel).toBe(flow);
        expect(result.root).toBe(fixture.root.direction);
        expect(result.inline).toBe(fixture.root.rowInline);
        expect(result.visual).toBe(entry.visual);
      }
      console.info("[DEBUG] Tree root reading order: both panel flows, twelve bilingual browser cases");
    } finally {
      await browser.close();
    }
  }, 30_000);

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
    } finally {
      await browser.close();
    }
  }, 30_000);
});
