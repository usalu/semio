/** 🏷️ Labels fit their controls on the page of an asset: measured as bounding boxes in Chromium, in both languages, at a phone width, at both tablet widths and at a desktop width, on the built site or on a running one.
 *
 * The site is `TEACHING_ARCHITECTURE_STAFFAGE_URL` when set, otherwise the build in `dist/<SEMIO_BUILD_MODE>` (release by default) served from loopback. `TEACHING_ARCHITECTURE_STAFFAGE_BROWSER_CHANNEL` names the Playwright browser channel (`chromium` by default, `chrome` for an installed Chrome), `TEACHING_ARCHITECTURE_STAFFAGE_LABEL_FIT_OUT` names a directory that receives one screenshot per case and the findings as JSON.
 * @see ./🥒️.feature — the scenarios
 * @see ../../../🎯️targets/⚛️react/🔨️modules/🪟️chrome/🟦️.tsx — `Segmented`
 * @see ../../../🎯️targets/⚛️react/🔨️modules/🧮️pipeline/🔤️text/🟦️.ts — the room the texts of a drawing take as the browser sets them */
import { existsSync, mkdirSync, writeFileSync } from "node:fs";
import { readFile } from "node:fs/promises";
import { createServer, type Server } from "node:http";
import { extname, join, resolve, sep } from "node:path";
import { chromium, type Browser } from "playwright";
import { afterAll, beforeAll, expect, it } from "vitest";
import { atTestLevel, testLevelBudgetMs } from "../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";

const LANGUAGES = [{ id: "en", choose: "English" }, { id: "de", choose: "Deutsch" }] as const;
const WINDOWS = [{ width: 320, height: 640, touch: true }, { width: 768, height: 1024, touch: true }, { width: 1024, height: 768, touch: true }, { width: 1440, height: 900, touch: false }] as const;
const TYPES: Readonly<Record<string, string>> = { ".css": "text/css", ".html": "text/html; charset=utf-8", ".js": "text/javascript", ".json": "application/json", ".svg": "image/svg+xml", ".wasm": "application/wasm", ".woff": "font/woff", ".woff2": "font/woff2" };
const ASSET = "walking-commuter";
const HELD = /@ 0[.,]50 s/u;
const out = process.env.TEACHING_ARCHITECTURE_STAFFAGE_LABEL_FIT_OUT;
const release = resolve(import.meta.dirname, "../../📦️packages/🟦️typescript/dist", process.env.SEMIO_BUILD_MODE ?? "release");

/** 📏️ One measured defect. */
type Finding = { readonly kind: string; readonly label: string; readonly by: number };

let server: Server | undefined;
let browser: Browser | undefined;
let base = process.env.TEACHING_ARCHITECTURE_STAFFAGE_URL ?? "";

/** 🗄️ Serves the files of the build over loopback. */
async function serve(root: string): Promise<string> {
  server = createServer(async (request, response) => {
    const pathname = decodeURIComponent(new URL(request.url ?? "/", "http://localhost").pathname);
    const file = resolve(root, "." + (pathname === "/" ? "/index.html" : pathname));
    const content = file.startsWith(root + sep) ? await readFile(file).catch(() => undefined) : undefined;
    response.writeHead(content === undefined ? 404 : 200, { "Content-Type": TYPES[extname(file)] ?? "application/octet-stream" }).end(content);
  });
  await new Promise<void>((ready) => server!.listen(0, "127.0.0.1", ready));
  const address = server.address();
  return `http://127.0.0.1:${typeof address === "object" && address !== null ? address.port : 0}/`;
}

/** 🔬️ Runs in the page: every defect of the labels, segments, controls and drawing texts of the opened asset page. */
function measure(input: { readonly asset: string; readonly touch: boolean }): Finding[] {
  const found: Finding[] = [];
  const page = [...document.querySelectorAll<HTMLElement>(`[data-page="asset:${input.asset}"]`)].find((candidate) => candidate.getClientRects().length > 0);
  if (page === undefined) return [{ kind: "page-missing", label: input.asset, by: 0 }];
  const shown = (element: Element): boolean => element.getClientRects().length > 0 && getComputedStyle(element).visibility !== "hidden";
  const nameOf = (element: Element): string => (element.textContent?.trim() || element.getAttribute("aria-label") || element.tagName).replace(/\s+/gu, " ").slice(0, 60);
  const beyond = (inner: DOMRect, outer: DOMRect): number => Math.max(outer.left - inner.left, inner.right - outer.right, outer.top - inner.top, inner.bottom - outer.bottom);
  const scopes = [page, ...document.querySelectorAll<HTMLElement>('[data-slot="navbar"]')];
  const controls = scopes.flatMap((scope) => [...scope.querySelectorAll<HTMLElement>('button, [role="radio"], a[href], summary, select')]).filter(shown);
  for (const control of controls) {
    const box = control.getBoundingClientRect();
    const walker = document.createTreeWalker(control, NodeFilter.SHOW_TEXT);
    let worst = 0;
    for (let node = walker.nextNode(); node !== null; node = walker.nextNode()) {
      if (node.nodeValue?.trim() === "" || node.parentElement?.closest(".sr-only") != null) continue;
      const range = document.createRange();
      range.selectNodeContents(node);
      for (const rect of range.getClientRects()) worst = Math.max(worst, beyond(rect, box));
    }
    if (worst > 0.5) found.push({ kind: "label-outside-control", label: nameOf(control), by: Math.round(worst * 10) / 10 });
    if (control.scrollWidth > control.clientWidth + 1) found.push({ kind: "label-clipped", label: nameOf(control), by: control.scrollWidth - control.clientWidth });
    const outside = Math.max(-box.left, box.right - document.documentElement.clientWidth);
    if (outside > 0.5) found.push({ kind: "control-outside-window", label: nameOf(control), by: Math.round(outside * 10) / 10 });
    if (input.touch && control.matches('button, [role="radio"], summary, select') && box.height < 43.5) found.push({ kind: "touch-target-low", label: nameOf(control), by: Math.round((44 - box.height) * 10) / 10 });
  }
  for (const group of [...page.querySelectorAll<HTMLElement>('[role="radiogroup"]')].filter(shown)) {
    const frame = group.getBoundingClientRect();
    const segments = [...group.querySelectorAll<HTMLElement>('[role="radio"]')].filter(shown).map((segment) => ({ segment, box: segment.getBoundingClientRect() }));
    segments.forEach(({ segment, box }, index) => {
      if (beyond(box, frame) > 0.5) found.push({ kind: "segment-outside-choice", label: nameOf(segment), by: Math.round(beyond(box, frame) * 10) / 10 });
      for (const other of segments.slice(index + 1)) {
        const over = Math.min(Math.min(box.right, other.box.right) - Math.max(box.left, other.box.left), Math.min(box.bottom, other.box.bottom) - Math.max(box.top, other.box.top));
        if (over > 0.5) found.push({ kind: "segments-overlap", label: `${nameOf(segment)} | ${nameOf(other.segment)}`, by: Math.round(over * 10) / 10 });
      }
    });
  }
  const sideways = document.documentElement.scrollWidth - document.documentElement.clientWidth;
  if (sideways > 0) found.push({ kind: "page-scrolls-sideways", label: "document", by: sideways });
  const figure = page.querySelector<HTMLElement>("[data-viewer] [data-stage] [data-staffage-figure]");
  const texts = figure === null ? [] : [...figure.querySelectorAll<SVGTextElement>("svg text")];
  if (figure === null || texts.length === 0) found.push({ kind: "drawing-missing", label: input.asset, by: 0 });
  else {
    const paper = figure.getBoundingClientRect();
    for (const text of texts) {
      const over = beyond(text.getBoundingClientRect(), paper);
      if (over > 0.5) found.push({ kind: "drawing-text-cut", label: nameOf(text), by: Math.round(over * 10) / 10 });
    }
  }
  return found;
}

beforeAll(async () => {
  if (base === "" && existsSync(join(release, "index.html"))) base = await serve(release);
  if (base !== "") browser = await chromium.launch({ channel: process.env.TEACHING_ARCHITECTURE_STAFFAGE_BROWSER_CHANNEL ?? "chromium", args: ["--disable-crash-reporter", "--disable-crashpad"] });
});

afterAll(async () => {
  await browser?.close();
  server?.close();
});

for (const language of LANGUAGES) {
  for (const window of WINDOWS) {
    atTestLevel(it, "long")(`keeps every label inside its control at ${window.width} px in ${language.choose}`, async () => {
      expect(base, `no site to measure: build it into ${release} or set TEACHING_ARCHITECTURE_STAFFAGE_URL`).not.toBe("");
      const context = await browser!.newContext({ viewport: { width: window.width, height: window.height }, hasTouch: window.touch, isMobile: window.touch, locale: "fr-FR" });
      try {
        const page = await context.newPage();
        await page.goto(base, { waitUntil: "domcontentloaded", timeout: 120_000 });
        const choice = page.getByRole("button", { name: language.choose, exact: true });
        await choice.waitFor({ state: "visible", timeout: 120_000 });
        await (window.touch ? choice.tap() : choice.click());
        await page.waitForFunction((lang) => document.documentElement.lang === lang, language.id, { timeout: 60_000 });
        await page.goto(new URL(`#/humans/${ASSET}?t=0.5`, base).href, { waitUntil: "domcontentloaded", timeout: 120_000 });
        await page.locator(`[data-page="asset:${ASSET}"]:visible [data-viewer] [data-stage] [data-staffage-figure][data-viewpoint="elevation"] svg`).first().waitFor({ state: "visible", timeout: 180_000 });
        await page.evaluate(async () => { await document.fonts.ready; });
        await page.waitForTimeout(1500);
        const caption = await page.locator(`[data-page="asset:${ASSET}"]:visible [data-viewer] [data-stage] [data-staffage-figure] svg text`).allTextContents();
        const findings = await page.evaluate(measure, { asset: ASSET, touch: window.touch });
        if (out !== undefined) {
          mkdirSync(out, { recursive: true });
          await page.screenshot({ path: join(out, `${language.id}-${window.width}.png`), fullPage: true });
          await page.locator(`[data-page="asset:${ASSET}"]:visible [data-viewer]`).first().screenshot({ path: join(out, `${language.id}-${window.width}-viewer.png`) });
          writeFileSync(join(out, `${language.id}-${window.width}.json`), JSON.stringify({ language: language.id, window, caption, findings }, null, 2) + "\n");
        }
        expect(caption.some((text) => HELD.test(text)), `the caption states the held time: ${caption.join(" / ")}`).toBe(true);
        expect(findings).toEqual([]);
      } finally {
        await context.close();
      }
    }, testLevelBudgetMs("long"));
  }
}
