// #region 🧲️Header
// 💻️ framework/ui/tests/🖼️storybook-frame-visuals/component.ts
// Specs: Chromium geometry and paint assertions for the frame and chip stories that jsdom and markup tests cannot see.
// Summary: Opens each story of `cases.json` in the built Storybook, measures boxes and samples screenshot pixels, and asserts the visual laws of the rectangular frame model: chip clearance, inner chip edges, full-width active outlines, content planes under the chips, rows spanning their frame, one set of scroll bands, window outlines and a canvas that fits its host.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲️Header

import Ajv from "ajv/dist/2020.js";
import { expect, test, type Page } from "../../../../../.storybook/🧪️tests/🎭️playwright/🟦️.ts";
import cases from "./cases.json" with { type: "json" };
import schema from "./cases.schema.json" with { type: "json" };

interface Box { readonly left: number; readonly top: number; readonly right: number; readonly bottom: number }
type Side = "top" | "right" | "bottom" | "left";

/** 🧱️ Opens a story and waits until it finished rendering and the border compositor is idle. */
async function open(page: Page, baseURL: string | undefined, id: string, viewport = cases.viewport): Promise<void> {
  await page.setViewportSize(viewport);
  await page.goto(`${baseURL}iframe.html?id=${encodeURIComponent(id)}&viewMode=story`, { waitUntil: "domcontentloaded", timeout: 120000 });
  await page.waitForFunction(story => {
    const render = (globalThis as typeof globalThis & { __STORYBOOK_PREVIEW__?: { currentRender?: { id: string; phase: string } } }).__STORYBOOK_PREVIEW__?.currentRender;
    return render?.id === story && render.phase === "finished";
  }, id, { timeout: 60000 });
  await page.evaluate(() => document.fonts.ready);
  await page.waitForFunction(() => document.querySelector("[data-shared-border-pending]") === null, undefined, { timeout: 10000 }).catch(() => undefined);
  await page.waitForTimeout(400);
}

const boxOf = async (page: Page, selector: string, index = 0): Promise<Box> => page.locator(selector).nth(index).evaluate(element => {
  const rect = element.getBoundingClientRect();
  return { left: rect.left, top: rect.top, right: rect.right, bottom: rect.bottom };
});

/** 🖍️ The width in pixels that a line of `stroke` covers across a one-pixel-deep strip: every pixel contributes the share of the stroke colour it carries (the red-minus-green distance of the stroke, measured against the strip's own background). */
async function lineCoverage(page: Page, side: Side, frame: Box, along: number, stroke: string): Promise<number> {
  const horizontal = side === "top" || side === "bottom";
  const edge = side === "top" ? frame.top : side === "bottom" ? frame.bottom : side === "left" ? frame.left : frame.right;
  const clip = horizontal ? { x: Math.round(along), y: Math.round(edge) - 4, width: 2, height: 8 } : { x: Math.round(edge) - 4, y: Math.round(along), width: 8, height: 2 };
  const png = (await page.screenshot({ clip, animations: "disabled" })).toString("base64");
  return page.evaluate(async ([data, vertical, stroke]) => {
    const image = new Image();
    image.src = `data:image/png;base64,${data}`;
    await image.decode();
    const canvas = document.createElement("canvas");
    canvas.width = image.width; canvas.height = image.height;
    const context = canvas.getContext("2d")!;
    context.drawImage(image, 0, 0);
    const swatch = document.createElement("canvas").getContext("2d")!;
    swatch.fillStyle = stroke as string; swatch.fillRect(0, 0, 1, 1);
    const [sr, sg] = swatch.getImageData(0, 0, 1, 1).data;
    const pixels = (context.getImageData(0, 0, canvas.width, canvas.height).data);
    const along = vertical ? canvas.height : canvas.width, across = vertical ? canvas.width : canvas.height;
    const redness = (position: number, depth: number) => { const index = vertical ? (position * canvas.width + depth) * 4 : (depth * canvas.width + position) * 4; return pixels[index]! - pixels[index + 1]!; };
    let baseline = Infinity, total = 0;
    for (let position = 0; position < along; position += 1) for (let depth = 0; depth < across; depth += 1) baseline = Math.min(baseline, redness(position, depth));
    for (let position = 0; position < along; position += 1) for (let depth = 0; depth < across; depth += 1) total += Math.max(0, redness(position, depth) - baseline) / (sr! - sg! - baseline);
    return total / along;
  }, [png, !horizontal, stroke] as const);
}

const sample = async (page: Page, x: number, y: number): Promise<readonly [number, number, number]> => {
  const png = (await page.screenshot({ clip: { x: Math.round(x), y: Math.round(y), width: 1, height: 1 }, animations: "disabled" })).toString("base64");
  return page.evaluate(async data => {
    const image = new Image();
    image.src = `data:image/png;base64,${data}`;
    await image.decode();
    const canvas = document.createElement("canvas");
    canvas.width = 1; canvas.height = 1;
    const context = canvas.getContext("2d")!;
    context.drawImage(image, 0, 0);
    const [r, g, b] = context.getImageData(0, 0, 1, 1).data;
    return [r!, g!, b!] as const;
  }, png);
};

test.describe("frame and chip visuals", () => {
  test.describe.configure({ timeout: 180000 });
  test.beforeAll(() => { expect(new Ajv().validate(schema, cases)).toBe(true); });

  test("the tab chip clears the first text row and draws its inner edges and dividers", async ({ page, baseURL }) => {
    await open(page, baseURL, cases.stories.tabs);
    const chip = await boxOf(page, '[data-slot="tabs-list"]');
    const first = await boxOf(page, '[data-slot="tabs-content"] span');
    expect(first.top, "the first text row starts below the chip row").toBeGreaterThanOrEqual(chip.bottom - 0.5);
    const lines = await page.locator('[data-slot="tabs-list"] > svg path').evaluateAll(paths => paths.map(path => path.getAttribute("d") ?? ""));
    const segments = lines.flatMap(d => [...d.matchAll(/M\s*([\d.]+)\s+([\d.]+)\s+L\s*([\d.]+)\s+([\d.]+)/g)].map(match => ({ x1: Number(match[1]), y1: Number(match[2]), x2: Number(match[3]), y2: Number(match[4]) })));
    const height = chip.bottom - chip.top;
    expect(segments.some(line => line.y1 === line.y2 && line.y1 > height - 2 && Math.abs(line.x2 - line.x1) > 20), "the chip draws its bottom edge").toBe(true);
    expect(segments.filter(line => line.x1 === line.x2 && line.y2 - line.y1 > height - 4).length, "the chip draws its end edge and one divider per tab boundary").toBeGreaterThanOrEqual(3);
  });

  test("the layout keeps its content, panels and console between the bands", async ({ page, baseURL }) => {
    await open(page, baseURL, cases.stories.layout);
    const layout = await boxOf(page, '[data-slot="layout"]');
    expect(layout.bottom - layout.top).toBeGreaterThanOrEqual(384);
    const navbar = await boxOf(page, '[data-slot="navbar"]');
    const footer = await boxOf(page, '[data-slot="footer"]');
    expect(footer.top - navbar.bottom, "the canvas area between the bands is not collapsed").toBeGreaterThanOrEqual(200);
    for (const text of ["Main Window", "Side Window", "Console output...", "Properties content"]) {
      const visible = await page.getByText(text, { exact: false }).first().evaluate(element => { const rect = element.getBoundingClientRect(); return rect.width > 0 && rect.height > 0 && rect.bottom <= innerHeight && rect.top >= 0; });
      expect(visible, text).toBe(true);
    }
  });

  test("the select popup has one scroll band per end and a line between every row", async ({ page, baseURL }) => {
    await open(page, baseURL, cases.stories.select);
    await page.locator("[role=combobox]").first().click();
    await page.waitForSelector('[data-slot="select-content"]');
    await page.waitForTimeout(400);
    expect(await page.locator('[data-slot="select-scroll-up-button"]').count()).toBe(1);
    expect(await page.locator('[data-slot="select-scroll-down-button"]').count()).toBe(1);
    const rows = await page.locator('[data-slot="select-item"]').evaluateAll(items => items.map(item => { const rect = item.getBoundingClientRect(); return { top: rect.top, bottom: rect.bottom }; }));
    const content = await boxOf(page, '[data-slot="select-content"]');
    const drawn = await page.locator('[data-slot="select-content"] svg[data-shared-border-overlay] path').evaluateAll(paths => paths.map(path => path.getAttribute("d") ?? ""));
    const horizontals = drawn.flatMap(d => [...d.matchAll(/M\s*([\d.-]+)\s+([\d.-]+)\s+L\s*([\d.-]+)\s+([\d.-]+)/g)].filter(match => match[2] === match[4]).map(match => content.top + Number(match[2])));
    for (let index = 1; index < rows.length; index += 1) {
      const seam = rows[index]!.top;
      expect(horizontals.some(y => Math.abs(y - seam) <= cases.rowTolerance), `a line between row ${index - 1} and ${index}`).toBe(true);
    }
  });

  test("the context menu rows span the frame, are separated by lines and the menu paints no browser focus ring", async ({ page, baseURL }) => {
    await open(page, baseURL, cases.stories.contextMenu);
    await page.locator("#storybook-root > *").first().click({ button: "right" });
    await page.waitForSelector('[data-slot="context-menu-primary"]');
    await page.waitForTimeout(500);
    const frame = await boxOf(page, '[data-slot="context-menu-content"]');
    const rows = await page.locator('[data-slot="context-menu-primary"] [role^="menuitem"]').evaluateAll(items => items.map(item => { const rect = item.getBoundingClientRect(); return { left: rect.left, right: rect.right, top: rect.top }; }));
    expect(rows.length).toBeGreaterThan(2);
    for (const row of rows) {
      expect(Math.abs(row.left - frame.left), "row starts at the frame edge").toBeLessThanOrEqual(cases.rowTolerance);
      expect(Math.abs(row.right - frame.right), "row ends at the frame edge").toBeLessThanOrEqual(cases.rowTolerance);
    }
    const ring = await page.locator('[role="menu"]').first().evaluate(menu => { menu.focus(); const style = getComputedStyle(menu); return { style: style.outlineStyle, width: style.outlineWidth }; });
    expect(ring.style === "none" || parseFloat(ring.width) === 0).toBe(true);
    const menu = await boxOf(page, '[data-slot="context-menu-primary"]');
    const drawn = await page.locator('[data-slot="context-menu-primary"] > svg[data-shared-border-overlay] path').evaluateAll(paths => paths.map(path => path.getAttribute("d") ?? ""));
    const horizontals = drawn.flatMap(d => [...d.matchAll(/M\s*([\d.-]+)\s+([\d.-]+)\s+L\s*([\d.-]+)\s+([\d.-]+)/g)].filter(match => match[2] === match[4]).map(match => menu.top + Number(match[2])));
    const tops = await page.locator('[data-slot="context-menu-primary"] [role^="menuitem"], [data-slot="context-menu-primary"] [role="separator"]').evaluateAll(items => items.map(item => item.getBoundingClientRect().top).sort((a, b) => a - b));
    for (const seam of tops.slice(1)) expect(horizontals.some(y => Math.abs(y - seam) <= cases.rowTolerance), `a line on the seam at ${seam}`).toBe(true);
  });

  test("an active window outline is closed on all four sides at full width", async ({ page, baseURL }) => {
    await open(page, baseURL, cases.stories.quad);
    const frame = await boxOf(page, '[data-window-frame][data-active="true"]');
    const stroke = await page.locator('[data-window-frame][data-active="true"] path[data-shared-border-state="active"]').first().evaluate(path => { const probe = document.createElement("i"); probe.style.color = path.getAttribute("stroke") ?? ""; document.body.append(probe); const color = getComputedStyle(probe).color; probe.remove(); return color; });
    const along = { top: frame.left + (frame.right - frame.left) * 0.7, bottom: frame.left + (frame.right - frame.left) * 0.7, left: frame.top + (frame.bottom - frame.top) * 0.5, right: frame.top + (frame.bottom - frame.top) * 0.5 };
    for (const side of ["top", "right", "bottom", "left"] as const) {
      const coverage = await lineCoverage(page, side, frame, along[side], stroke);
      expect(coverage, `the ${side} edge of the active window`).toBeGreaterThanOrEqual(cases.minLineCoverage);
    }
  });

  test("the window content surface runs edge to edge under the chip rows", async ({ page, baseURL }) => {
    for (const id of [cases.stories.quad, cases.stories.stack]) {
      await open(page, baseURL, id);
      const frame = await boxOf(page, "[data-window-frame]");
      const chips = await page.locator('[data-window-frame] [data-window-frame-chip][data-dock="top"]').evaluateAll(items => items.map(item => item.getBoundingClientRect().right));
      const body = await sample(page, (frame.left + frame.right) / 2 + 40, (frame.top + frame.bottom) / 2 - 60);
      const strip = await sample(page, Math.max(...chips) + 40, frame.top + 6);
      expect(strip, `the cap row beside the chips shows the content surface in ${id}`).toEqual(body);
    }
  });

  test("windows draw their outline and the divider between them, and the canvas fits its host", async ({ page, baseURL }) => {
    await open(page, baseURL, cases.stories.canvas);
    const left = await boxOf(page, '[data-slot="window"]', 0);
    const right = await boxOf(page, '[data-slot="window"]', 1);
    const canvas = await boxOf(page, '[data-slot="canvas"]');
    expect(canvas.bottom, "the canvas is not cut by the viewport").toBeLessThanOrEqual(cases.viewport.height);
    const chips = await page.locator('[data-slot="canvas-stage"] [data-window-frame-chip]').evaluateAll(items => items.map(item => item.getBoundingClientRect().bottom));
    for (const bottom of chips) expect(bottom).toBeLessThanOrEqual(cases.viewport.height);
    const outlined = await page.locator('[data-slot="window"] > svg[data-shared-border-overlay] path').count();
    expect(outlined, "every window paints outline segments").toBeGreaterThanOrEqual(2);
    const stroke = await page.locator('[data-slot="window"] > svg[data-shared-border-overlay] path').first().evaluate(path => getComputedStyle(path).stroke);
    const background = await sample(page, (left.left + left.right) / 2, left.top + 80);
    const near = await Promise.all([-2, -1, 0, 1].map(offset => sample(page, left.right + offset, left.top + 120)));
    expect(near.some(pixel => pixel.some((value, index) => Math.abs(value - background[index]!) > 12)), `a divider line lies on the seam of the windows (${stroke})`).toBe(true);
    expect(right.left).toBeGreaterThanOrEqual(left.right - 1);
  });

  test("every status dot paints its tone", async ({ page, baseURL }) => {
    await open(page, baseURL, cases.stories.status);
    const dots = await page.locator('[data-slot="status-chip"] > span:first-child').evaluateAll(items => items.map(item => getComputedStyle(item).backgroundColor));
    expect(dots).toHaveLength(4);
    for (const color of dots) expect(color === "rgba(0, 0, 0, 0)" || color === "transparent", "a tone dot is filled").toBe(false);
    expect(new Set(dots).size, "the tones are told apart").toBeGreaterThanOrEqual(3);
  });

  test("windows of a nested layout share their canvas instead of squeezing all but one to nothing", async ({ page, baseURL }) => {
    await open(page, baseURL, cases.stories.nested);
    const widths = await page.locator('[data-slot="window"]').evaluateAll(items => items.map(item => item.getBoundingClientRect().width));
    expect(widths).toHaveLength(3);
    for (const width of widths) expect(width).toBeGreaterThan(100);
  });
});
