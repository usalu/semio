// #region 🧲️Header
// 💻️ ✏️s/🔌️plugins/🧩️puzzle/🧪️tests/◻️storybook-2d/🟦️.ts
// Specs: End-to-end checks for the framework renderer's puzzle 2d board host inside the aggregated Storybook static build.
// Summary: Drives the WASM `BoardSession` through real pointer/wheel/keyboard input and asserts against the story's `puzzle2d-board-debug` readout (selection/camera/utility/counts) — GPU pixels are not asserted, only the CPU-side session state the debug readout reflects.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲️Header

import { expect, test, type Locator, type Page } from "../../../../../.storybook/🧪️tests/🎭️playwright/🟦️.ts";
import { writeFileSync } from "node:fs";
import snapshotCorpus from "../🧫️fixtures/📸️board-session/🔣️.json";
import keyboardCorpus from "../🧫️fixtures/⌨️story-keyboard/🔣️.json";
import { storyId, type StoryIndexEntry } from "../📚️storybook-index/🟦️.ts";

type Puzzle2dBoardDebug = {
  readonly selection: readonly string[];
  readonly camera: { readonly x: number; readonly y: number; readonly zoom: number };
  readonly activeUtility: string;
  readonly nodeCount: number;
  readonly edgeCount: number;
};

type Box = { readonly x: number; readonly y: number; readonly width: number; readonly height: number };

function significantConsoleErrors(messages: string[]): string[] {
  return messages.filter((text) => !/Failed to load resource:.*\b404\b/i.test(text));
}

async function readPuzzle2dDebug(debug: Locator): Promise<Puzzle2dBoardDebug> {
  const text = await debug.innerText();
  return JSON.parse(text) as Puzzle2dBoardDebug;
}

/** 📐️ Canonical `screenX = (worldX - camera.x) * zoom + width / 2` transform shared across board renderers — used to click a known fixture node by its world position. */
function worldToClientPoint(box: Box, camera: Puzzle2dBoardDebug["camera"], world: { readonly x: number; readonly y: number }): { readonly clientX: number; readonly clientY: number } {
  return {
    clientX: box.x + (world.x - camera.x) * camera.zoom + box.width / 2,
    clientY: box.y + (world.y - camera.y) * camera.zoom + box.height / 2,
  };
}

async function expectBoardStory(page: Page, name: string, locale?: string): Promise<{ readonly canvas: Locator; readonly debug: Locator; readonly assertNoErrors: () => void }> {
  const response = await page.request.get("index.json");
  expect(response.ok()).toBe(true);
  const index = await response.json() as { entries: Record<string, StoryIndexEntry> };
  const id = storyId(index.entries, "🧩️puzzle🩻️2d", name);
  const pageErrors: Error[] = [];
  const consoleErrors: string[] = [];
  page.on("pageerror", (error) => pageErrors.push(error));
  page.on("console", (message) => {
    if (message.type() === "error") consoleErrors.push(message.text());
  });

  const globals = locale ? "&globals=locale:" + encodeURIComponent(locale) : "";
  await page.goto(`iframe.html?id=${encodeURIComponent(id)}&viewMode=story${globals}`, { waitUntil: "domcontentloaded" });
  await expect(page.locator("body")).not.toContainText("Couldn't find story matching");
  await expect(page.locator("body")).not.toContainText("Failed to load the Storybook preview file");
  await page.waitForFunction(() => {
    const root = document.querySelector("#storybook-root");
    return !!root && root.childElementCount > 0;
  });

  const debug = page.getByTestId("puzzle2d-board-debug");
  await expect(debug).toBeVisible();
  const host = page.locator(".semio-board-2d-host");
  try {
    await expect(host).toHaveAttribute("data-board-snapshot-parsed", "true");
    await expect(host).toHaveAttribute("data-board-nodes", "2");
    await expect(host.locator("canvas")).toBeVisible();
  } catch (error) {
    writeFileSync(test.info().outputPath("errors.json"), JSON.stringify({ pageErrors: pageErrors.map(error => ({ message: error.message, stack: error.stack })), consoleErrors }, null, 2));
    throw error;
  }
  const canvas = page.locator(".semio-board-2d-host canvas");

  expect(pageErrors.map((error) => error.message)).toEqual([]);
  expect(significantConsoleErrors(consoleErrors)).toEqual([]);
  return { canvas, debug, assertNoErrors: () => {
    expect(pageErrors.map((error) => error.message)).toEqual([]);
    expect(significantConsoleErrors(consoleErrors)).toEqual([]);
  } };
}

test("puzzle2d overview select: loads with the initial fixture reflected in the debug readout", async ({ page }) => {
  const { debug, assertNoErrors } = await expectBoardStory(page, "Overview Select");
  const state = await readPuzzle2dDebug(debug);
  expect({ selection: state.selection, activeUtility: state.activeUtility, nodeCount: state.nodeCount, edgeCount: state.edgeCount }).toEqual(snapshotCorpus.expected);
  assertNoErrors();
});

test("puzzle2d overview select: clicking a node selects it, clicking empty space clears it", async ({ page }) => {
  const { canvas, debug, assertNoErrors } = await expectBoardStory(page, "Overview Select");
  const box = await canvas.boundingBox();
  expect(box).toBeTruthy();
  const before = await readPuzzle2dDebug(debug);

  const alphaPoint = worldToClientPoint(box!, before.camera, { x: 0, y: 0 });
  await page.mouse.click(alphaPoint.clientX, alphaPoint.clientY);
  await expect.poll(async () => (await readPuzzle2dDebug(debug)).selection).toEqual(["alpha"]);

  await page.mouse.click(box!.x + 4, box!.y + 4);
  await expect.poll(async () => (await readPuzzle2dDebug(debug)).selection).toEqual([]);
  assertNoErrors();
});

test("puzzle2d overview select: wheel zoom updates the camera in the debug readout", async ({ page }) => {
  const { canvas, debug, assertNoErrors } = await expectBoardStory(page, "Overview Select");
  const box = await canvas.boundingBox();
  expect(box).toBeTruthy();
  const before = await readPuzzle2dDebug(debug);

  await page.mouse.move(box!.x + box!.width / 2, box!.y + box!.height / 2);
  for (let index = 0; index < 12; index += 1) {
    await page.mouse.wheel(0, -120);
  }
  await expect.poll(async () => (await readPuzzle2dDebug(debug)).camera.zoom).toBeGreaterThan(before.camera.zoom);
  assertNoErrors();
});

test("puzzle2d overview select: Delete removes the selected node", async ({ page }) => {
  const { canvas, debug, assertNoErrors } = await expectBoardStory(page, "Overview Select");
  const box = await canvas.boundingBox();
  expect(box).toBeTruthy();
  const before = await readPuzzle2dDebug(debug);

  const betaPoint = worldToClientPoint(box!, before.camera, { x: 280, y: 120 });
  await page.mouse.move(betaPoint.clientX, betaPoint.clientY);
  await page.mouse.click(betaPoint.clientX, betaPoint.clientY);
  await expect.poll(async () => (await readPuzzle2dDebug(debug)).selection).toEqual(["beta"]);

  await page.keyboard.press("Delete");
  await expect.poll(async () => (await readPuzzle2dDebug(debug)).nodeCount).toBe(1);
  await expect.poll(async () => (await readPuzzle2dDebug(debug)).selection).toEqual([]);
  assertNoErrors();
});

test("puzzle2d overview select: Ctrl/Cmd+A selects every node", async ({ page }) => {
  const { canvas, debug, assertNoErrors } = await expectBoardStory(page, "Overview Select");
  const box = await canvas.boundingBox();
  expect(box).toBeTruthy();
  await page.mouse.click(box!.x + box!.width / 2, box!.y + box!.height / 2);
  await page.keyboard.press("ControlOrMeta+a");
  await expect.poll(async () => (await readPuzzle2dDebug(debug)).selection.length).toBe(2);
  assertNoErrors();
});

test("puzzle2d lasso select: polygon selects alpha while excluding beta inside its bounding rectangle", async ({ page }) => {
  const { canvas, debug, assertNoErrors } = await expectBoardStory(page, "Lasso Select");
  const box = await canvas.boundingBox();
  expect(box).toBeTruthy();
  const before = await readPuzzle2dDebug(debug);
  const points = snapshotCorpus.input.lasso.map(point => worldToClientPoint(box!, before.camera, point));
  await page.mouse.move(points[0]!.clientX, points[0]!.clientY);
  await page.mouse.down();
  for (const point of points.slice(1)) await page.mouse.move(point.clientX, point.clientY, { steps: 8 });
  await page.mouse.up();
  await expect.poll(async () => (await readPuzzle2dDebug(debug)).selection).toEqual(snapshotCorpus.afterLasso);
  assertNoErrors();
});

test("puzzle2d brush utility: story boots with the brush utility active", async ({ page }) => {
  const { debug, assertNoErrors } = await expectBoardStory(page, "Brush Utility");
  const state = await readPuzzle2dDebug(debug);
  expect(state.activeUtility).toBe("brush");
  assertNoErrors();
});

async function performBrushAltLeaveCommit(page: Page, canvasBox: Box): Promise<void> {
  await page.keyboard.down("Alt");
  await page.mouse.move(canvasBox.x - 20, canvasBox.y - 20);
  await page.keyboard.up("Alt");
}

test("puzzle2d brush placement: hovering a node with one free handle commits nothing", async ({ page }) => {
  const { canvas, debug, assertNoErrors } = await expectBoardStory(page, "Brush Placement");
  const box = await canvas.boundingBox();
  expect(box).toBeTruthy();
  const before = await readPuzzle2dDebug(debug);

  const alphaPoint = worldToClientPoint(box!, before.camera, { x: 0, y: 0 });
  await page.mouse.move(alphaPoint.clientX, alphaPoint.clientY);
  await page.mouse.move(alphaPoint.clientX + 4, alphaPoint.clientY + 4);

  const after = await readPuzzle2dDebug(debug);
  expect(after.nodeCount).toBe(before.nodeCount);
  expect(after.edgeCount).toBe(before.edgeCount);
  assertNoErrors();
});

test("puzzle2d brush placement: committing places a node and links it", async ({ page }) => {
  const { canvas, debug, assertNoErrors } = await expectBoardStory(page, "Brush Placement");
  const box = await canvas.boundingBox();
  expect(box).toBeTruthy();
  const before = await readPuzzle2dDebug(debug);

  const alphaPoint = worldToClientPoint(box!, before.camera, { x: 0, y: 0 });
  await page.mouse.move(alphaPoint.clientX, alphaPoint.clientY);
  await performBrushAltLeaveCommit(page, box!);

  await expect.poll(async () => (await readPuzzle2dDebug(debug)).nodeCount).toBe(before.nodeCount + 1);
  await expect.poll(async () => (await readPuzzle2dDebug(debug)).edgeCount).toBe(before.edgeCount + 1);
  assertNoErrors();
});

test("puzzle2d forced lod pane: non-interactive pane ignores pointer input", async ({ page }) => {
  const { canvas, debug, assertNoErrors } = await expectBoardStory(page, "Forced Lod Pane");
  const box = await canvas.boundingBox();
  expect(box).toBeTruthy();
  const before = await readPuzzle2dDebug(debug);
  const point = worldToClientPoint(box!, before.camera, snapshotCorpus.input.node);
  await page.mouse.click(point.clientX, point.clientY);
  await page.evaluate(() => new Promise<void>(resolve => requestAnimationFrame(() => resolve())));
  const after = await readPuzzle2dDebug(debug);
  expect(after.selection).toEqual(before.selection);
  assertNoErrors();
});

for (const locale of ["en", "de"] as const) test("puzzle2d keyboard commands stay scoped to the focused interactive board (" + locale + ")", async ({ page }) => {
  const { canvas, debug, assertNoErrors } = await expectBoardStory(page, "Overview Select", locale);
  const group = page.getByRole("group", { name: keyboardCorpus.groupLabels[locale], exact: true });
  await expect(group).toBeVisible();
  await canvas.click();
  await expect(group).toBeFocused();
  await page.keyboard.press(keyboardCorpus.selectAll);
  await expect.poll(async () => (await readPuzzle2dDebug(debug)).selection.length).toBe(keyboardCorpus.selectedCount);
  await page.evaluate((value) => {
    const input = document.createElement("input");
    input.setAttribute("aria-label", "Story text input");
    input.value = value;
    document.querySelector(".semio-board-2d-host")!.append(input);
    input.focus();
  }, keyboardCorpus.input);
  const input = page.getByRole("textbox", { name: "Story text input" });
  await page.keyboard.press(keyboardCorpus.selectAll);
  await page.keyboard.press(keyboardCorpus.delete);
  await expect(input).toHaveValue(keyboardCorpus.afterInputDelete.input);
  expect((await readPuzzle2dDebug(debug)).nodeCount).toBe(keyboardCorpus.afterInputDelete.nodes);
  await canvas.click();
  await page.keyboard.press(keyboardCorpus.selectAll);
  await page.keyboard.press(keyboardCorpus.delete);
  await expect.poll(async () => (await readPuzzle2dDebug(debug)).nodeCount).toBe(keyboardCorpus.afterBoardDelete.nodes);
  expect((await readPuzzle2dDebug(debug)).edgeCount).toBe(keyboardCorpus.afterBoardDelete.edges);
  assertNoErrors();
});
