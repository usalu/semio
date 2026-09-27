// @vitest-environment jsdom

import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { createElement } from "react";
import Ajv2020 from "ajv/dist/2020.js";
import { diffArrays } from "diff";
import { compile } from "@tailwindcss/node";
import { chromium } from "playwright";
import { afterEach, describe, expect, it } from "vitest";
import { cleanup, render } from "@semio-tech/ui-react/test";
import { DiffViewHost } from "../../🧱️elements/🔺️DiffViewHost/🟦️.tsx";

type ExpectedLine = { readonly kind: "equal" | "remove" | "add"; readonly text: string };
type Fixture = {
  readonly before: string;
  readonly after: string;
  readonly mode: "unified" | "split";
  readonly language: string;
  readonly expected: readonly ExpectedLine[];
};
type PresentationFixture = {
  readonly host: { readonly width: number; readonly height: number; readonly inset: number };
  readonly typography: { readonly family: string; readonly size: number; readonly lineHeight: number; readonly gap: number };
  readonly colors: { readonly equal: readonly number[]; readonly added: readonly number[]; readonly removed: readonly number[] };
  readonly unified: { readonly contentWidth: number; readonly rowHeight: number; readonly textX: number };
  readonly split: { readonly paneWidth: number; readonly rightX: number; readonly leftTextX: number; readonly rightTextX: number };
  readonly preWrap: { readonly hostWidth: number; readonly text: string; readonly expectedLines: readonly string[] };
};

const suiteRoot = dirname(fileURLToPath(import.meta.url));
const repoRoot = resolve(suiteRoot, "../../../../../../../..");
const fixture = JSON.parse(readFileSync(resolve(repoRoot, "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🆚️diff-view-produced-surface/🔣️.json"), "utf8")) as Fixture;
const schema = JSON.parse(readFileSync(resolve(repoRoot, "🧰️framework/🔨️modules/🖱️ui/🧬️schema/🆚️diff-view-produced-surface/🔣️.json"), "utf8"));
const presentation = JSON.parse(readFileSync(resolve(repoRoot, "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🆚️diff-view-presentation/🔣️.json"), "utf8")) as PresentationFixture;
const presentationSchema = JSON.parse(readFileSync(resolve(repoRoot, "🧰️framework/🔨️modules/🖱️ui/🧬️schema/🆚️diff-view-presentation/🔣️.json"), "utf8"));

function thirdPartyLines(): ExpectedLine[] {
  return diffArrays(fixture.before.split("\n"), fixture.after.split("\n")).flatMap((change) => {
    const kind: ExpectedLine["kind"] = change.added ? "add" : change.removed ? "remove" : "equal";
    return change.value.map((text) => ({ kind, text }));
  });
}

function node(mode: "unified" | "split") {
  return {
    type: "componentScene",
    hostId: "diff-fixture-host",
    surfaceId: "diff-fixture",
    controllerId: "fixture",
    componentKind: "diffView",
    presence: {},
    diffView: { before: fixture.before, after: fixture.after, mode, language: fixture.language },
  };
}

afterEach(() => cleanup());

describe("authored DiffView surface parity", () => {
  it("measures the current styled React DiffView reference", async () => {
    const browser = await chromium.launch({ headless: true });
    try {
      const page = await browser.newPage({ viewport: { width: 800, height: 400 } });
      const stylesPath = resolve(repoRoot, "🧰️framework/🔨️modules/🖱️ui/🎨️styling/🖌️ui/🎨️.css");
      const compiler = await compile(readFileSync(stylesPath, "utf8"), { base: dirname(stylesPath), from: stylesPath, onDependency: () => {} });
      const styledCss = (container: HTMLElement): string => {
        const candidates = [...new Set([...container.querySelectorAll("[class]")].flatMap(element => [...element.classList]))];
        return compiler.build(candidates).replace(/url\("\/🖼️assets\/([^"\n]+)"\)/gu, (_, path) => `url("data:font/woff2;base64,${readFileSync(resolve(repoRoot, "🧰️framework/🔨️modules/🖼️assets", path)).toString("base64")}")`);
      };
      for (const mode of ["unified", "split"] as const) {
        const view = render(createElement(DiffViewHost, { node: node(mode), onAction: () => {} } as any));
        const css = styledCss(view.container);
        await page.setContent(`<div class="semio-scope" style="width:${presentation.host.width}px;height:${presentation.host.height}px">${view.container.innerHTML}</div>`);
        await page.addStyleTag({ content: css });
        await page.evaluate(() => document.fonts.ready);
        const observed = await page.evaluate(() => {
          const host = document.querySelector<HTMLElement>(".semio-diff-view-host")!;
          const rows = [...host.querySelectorAll<HTMLElement>(".semio-diff-view-unified > div,.semio-diff-view-split-pane")];
          const rect = (element: Element) => { const bounds = element.getBoundingClientRect(); return [bounds.x, bounds.y, bounds.width, bounds.height]; };
          const rgba = (color: string) => { const canvas = document.createElement("canvas"); const ctx = canvas.getContext("2d")!; ctx.fillStyle = color; ctx.fillRect(0, 0, 1, 1); return [...ctx.getImageData(0, 0, 1, 1).data]; };
          return { host: rect(host), padding: getComputedStyle(host).padding, rows: rows.map(row => ({ rect: rect(row), spans: [...row.children].map(rect), color: rgba(getComputedStyle(row).color), font: getComputedStyle(row).fontFamily, size: getComputedStyle(row).fontSize, lineHeight: getComputedStyle(row).lineHeight, gap: getComputedStyle(row).gap, text: row.textContent })) };
        });
        expect(observed.host).toEqual([0, 0, presentation.host.width, presentation.host.height]);
        expect(observed.padding).toBe(`${presentation.host.inset}px`);
        expect(observed.rows.every((row) => row.font.startsWith(`"${presentation.typography.family}"`))).toBe(true);
        expect(observed.rows.every((row) => row.size === `${presentation.typography.size}px` && row.lineHeight === `${presentation.typography.lineHeight}px` && row.gap === `${presentation.typography.gap}px`)).toBe(true);
        if (mode === "unified") {
          expect(observed.rows.map((row) => row.color)).toEqual(fixture.expected.map((line) => presentation.colors[line.kind === "add" ? "added" : line.kind === "remove" ? "removed" : "equal"]));
          expect(observed.rows.every((row) => row.rect[2] === presentation.unified.contentWidth && row.rect[3] === presentation.unified.rowHeight)).toBe(true);
          expect(observed.rows[0]?.spans[3]?.[0]).toBe(presentation.unified.textX);
        } else {
          expect(observed.rows.every((row) => row.rect[2] === presentation.split.paneWidth && row.rect[3]! >= presentation.typography.lineHeight)).toBe(true);
          expect(observed.rows[0]?.rect[0]).toBe(presentation.host.inset - 0.0125);
          expect(observed.rows[1]?.rect[0]).toBe(presentation.split.rightX);
          expect(observed.rows[0]?.spans[1]?.[0]).toBe(presentation.split.leftTextX);
          expect(observed.rows[1]?.spans[1]?.[0]).toBe(presentation.split.rightTextX);
          for (let index = 0; index < observed.rows.length; index += 2) expect(observed.rows[index]?.rect[3]).toBe(observed.rows[index + 1]?.rect[3]);
        }
        view.unmount();
      }

      const narrow = render(createElement(DiffViewHost, { node: { ...node("unified"), diffView: { before: presentation.preWrap.text, after: presentation.preWrap.text, mode: "unified", language: fixture.language } }, onAction: () => {} } as any));
      await page.setContent(`<div class="semio-scope" style="width:${presentation.preWrap.hostWidth}px;height:${presentation.host.height}px">${narrow.container.innerHTML}</div>`);
      await page.addStyleTag({ content: styledCss(narrow.container) });
      await page.evaluate(() => document.fonts.ready);
      const wrapped = await page.evaluate(() => {
        const node = document.querySelector(".semio-diff-view-unified > div > span:last-child")!.firstChild!;
        const lines = new Map<number, string>();
        for (let index = 0; index < node.textContent!.length; index += 1) {
          const range = document.createRange();
          range.setStart(node, index);
          range.setEnd(node, index + 1);
          const y = range.getBoundingClientRect().y;
          lines.set(y, `${lines.get(y) ?? ""}${node.textContent![index]}`);
        }
        return [...lines.values()];
      });
      expect(wrapped).toEqual(presentation.preWrap.expectedLines);
      narrow.unmount();
    } finally {
      await browser.close();
    }
  }, 90_000);

  it("validates the neutral corpus against the independent diff oracle", () => {
    const validate = new Ajv2020({ strict: true, allErrors: true }).compile(schema);
    expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
    const validatePresentation = new Ajv2020({ strict: true, allErrors: true }).compile(presentationSchema);
    expect(validatePresentation(presentation), JSON.stringify(validatePresentation.errors)).toBe(true);
    expect(thirdPartyLines()).toEqual(fixture.expected);
  });

  it("renders the actual React host with the same unified operations and split panes", () => {
    const unified = render(createElement(DiffViewHost, { node: node("unified"), onAction: () => {} } as any));
    const rows = [...unified.container.querySelectorAll<HTMLElement>(".semio-diff-view-unified > div")];
    expect(rows.map((row) => row.querySelectorAll("span").item(3).textContent)).toEqual(fixture.expected.map((line) => line.text));
    expect(rows.map((row) => (row.className.includes("text-diff-added") ? "add" : row.className.includes("text-destructive") ? "remove" : "equal"))).toEqual(fixture.expected.map((line) => line.kind));
    unified.unmount();

    const split = render(createElement(DiffViewHost, { node: node("split"), onAction: () => {} } as any));
    expect(split.container.querySelectorAll(".semio-diff-view-split-row")).toHaveLength(4);
    expect(split.container.querySelectorAll(".semio-diff-view-split-pane")).toHaveLength(8);
  });
});
