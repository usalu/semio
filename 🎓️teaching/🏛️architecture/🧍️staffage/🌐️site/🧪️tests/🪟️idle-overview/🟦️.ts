/** 🪟️ An idle desktop overview stays within a memory budget: measured in Chromium on the built site (or a running one), the live previews together weigh no more than the live budget, only the previews near the reader are live and the page holds a bounded number of DOM nodes.
 *
 * The site is `TEACHING_ARCHITECTURE_STAFFAGE_URL` when set, otherwise the build in `dist/<SEMIO_BUILD_MODE>` (release by default) served from loopback. `TEACHING_ARCHITECTURE_STAFFAGE_BROWSER_CHANNEL` names the Playwright browser channel (`chromium` by default, `chrome` for an installed Chrome), `TEACHING_ARCHITECTURE_STAFFAGE_IDLE_SECONDS` the idle time (40 s by default), `TEACHING_ARCHITECTURE_STAFFAGE_IDLE_OUT` a directory that receives the samples and a screenshot.
 * @see ./🥒️.feature — the scenarios
 * @see ../../../🎯️targets/⚛️react/🔨️modules/🖼️previews/🚦️live/🟦️.ts — `PREVIEW_LIVE_BUDGET_POINTS`, `PreviewLiveSet` */
import { existsSync, mkdirSync, writeFileSync } from "node:fs";
import { readFile } from "node:fs/promises";
import { createServer, type Server } from "node:http";
import { extname, join, resolve, sep } from "node:path";
import { chromium, type Browser } from "playwright";
import { afterAll, beforeAll, expect, it } from "vitest";
import { atTestLevel, testLevelBudgetMs } from "../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import { PREVIEW_LIVE_BUDGET_POINTS } from "../../../🎯️targets/⚛️react/🔨️modules/🖼️previews/🚦️live/🟦️.ts";

const TYPES: Readonly<Record<string, string>> = { ".css": "text/css", ".html": "text/html; charset=utf-8", ".js": "text/javascript", ".json": "application/json", ".svg": "image/svg+xml", ".wasm": "application/wasm", ".woff": "font/woff", ".woff2": "font/woff2" };
const IDLE_SECONDS = Number(process.env.TEACHING_ARCHITECTURE_STAFFAGE_IDLE_SECONDS ?? "40");
const DOM_NODE_LIMIT = 400_000;
const SAMPLE_MS = 5_000;
const out = process.env.TEACHING_ARCHITECTURE_STAFFAGE_IDLE_OUT;
const release = resolve(import.meta.dirname, "../../📦️packages/🟦️typescript/dist", process.env.SEMIO_BUILD_MODE ?? "release");

/** 📸️ What one sample reads from the page. */
type Sample = { readonly s: number; readonly live: number; readonly still: number; readonly liveWeight: number; readonly drawn: number; readonly liveOutsideRow: readonly string[]; readonly domNodes: number };

let server: Server | undefined;
let browser: Browser | undefined;
let base = process.env.TEACHING_ARCHITECTURE_STAFFAGE_URL ?? "";

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

/** 🔬️ Runs in the page: the live and still previews, what the live ones weigh, the live ones of a card that lie further than one row from the window, and the figures drawn. */
function read(): Omit<Sample, "s" | "domNodes"> {
  const previews = [...document.querySelectorAll<HTMLElement>("[data-preview-state]")];
  const live = previews.filter((element) => element.dataset.previewState === "live");
  const liveOutsideRow: string[] = [];
  for (const element of live) {
    const card = element.closest<HTMLElement>("[data-card]");
    if (card === null) continue;
    const box = card.getBoundingClientRect();
    if (box.height === 0 || box.width === 0) continue;
    if (box.bottom < -box.height || box.top > window.innerHeight + box.height) liveOutsideRow.push(card.getAttribute("data-card") ?? card.textContent?.slice(0, 30) ?? "?");
  }
  return {
    live: live.length,
    still: previews.length - live.length,
    liveWeight: live.reduce((sum, element) => sum + Number(element.dataset.previewWeight ?? 0), 0),
    drawn: previews.filter((element) => element.querySelector("svg") !== null).length,
    liveOutsideRow,
  };
}

beforeAll(async () => {
  if (base === "" && existsSync(join(release, "index.html"))) base = await serve(release);
  if (base !== "") browser = await chromium.launch({ channel: process.env.TEACHING_ARCHITECTURE_STAFFAGE_BROWSER_CHANNEL ?? "chromium", args: ["--disable-crash-reporter", "--disable-crashpad", "--enable-unsafe-swiftshader", "--ignore-gpu-blocklist"] });
});

afterAll(async () => {
  await browser?.close();
  server?.close();
});

atTestLevel(it, "long")("keeps the live previews of an idle desktop overview within the budget", async () => {
  expect(base, `no site to measure: build it into ${release} or set TEACHING_ARCHITECTURE_STAFFAGE_URL`).not.toBe("");
  const context = await browser!.newContext({ viewport: { width: 1440, height: 900 }, locale: "fr-FR" });
  try {
    const page = await context.newPage();
    const session = await context.newCDPSession(page);
    await page.goto(base, { waitUntil: "domcontentloaded", timeout: 120_000 });
    await page.getByRole("button", { name: "English", exact: true }).click();
    await page.waitForFunction(() => document.documentElement.lang === "en", undefined, { timeout: 60_000 });
    await page.reload({ waitUntil: "domcontentloaded" });
    await page.locator("[data-frieze] svg").first().waitFor({ state: "attached", timeout: 180_000 });
    const samples: Sample[] = [];
    const started = Date.now();
    while ((Date.now() - started) / 1000 < IDLE_SECONDS) {
      await page.waitForTimeout(SAMPLE_MS);
      const state = await page.evaluate(read);
      const counters = (await session.send("Memory.getDOMCounters")) as { readonly nodes: number };
      samples.push({ s: Math.round((Date.now() - started) / 1000), ...state, domNodes: counters.nodes });
    }
    if (out !== undefined) {
      mkdirSync(out, { recursive: true });
      await page.screenshot({ path: join(out, "idle-overview.png") });
      writeFileSync(join(out, "idle-overview.json"), JSON.stringify({ budget: PREVIEW_LIVE_BUDGET_POINTS, samples }, null, 2) + "\n");
    }
    expect(samples.length).toBeGreaterThan(0);
    for (const sample of samples) {
      expect(sample.liveWeight, `at ${sample.s} s the live previews weigh ${sample.liveWeight} points`).toBeLessThanOrEqual(PREVIEW_LIVE_BUDGET_POINTS);
      expect(sample.liveOutsideRow, `at ${sample.s} s live previews far from the window`).toEqual([]);
      expect(sample.domNodes, `at ${sample.s} s DOM nodes`).toBeLessThanOrEqual(DOM_NODE_LIMIT);
    }
    const last = samples.at(-1)!;
    expect(last.still, "some previews rest").toBeGreaterThan(0);
    expect(last.live, "fewer previews are live than there are previews").toBeLessThan(last.live + last.still);
    expect(last.drawn, "the previews are drawn").toBeGreaterThan(0);
  } finally {
    await context.close();
  }
}, testLevelBudgetMs("long") + IDLE_SECONDS * 1000);
