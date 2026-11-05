// #region 🧲️Header
// 💻️ ✏️s/🔌️plugins/🧩️puzzle/🧪️tests/🧊️storybook-spatial/🟦️.ts
// Specs: End-to-end smoke + light-interaction checks for the `puzzle/3d`, `puzzle/5d`, and `infinite` Storybook scopes' new real-fixture stories, plus `puzzle/2d`'s `Fixtures.stories.tsx`.
// Summary: Every story gets the same base assertion as the sibling 2D Storybook case's `expectBoardStory` (page loads, no page/console errors, `#storybook-root` mounts) via `expectStoryLoads`; a handful of stories get an additional readout/interaction assertion (fixture counts via each story's `data-testid` debug `<pre>`, the puzzle-5d timeline scrub, the mock WasmCanvas pointer counter, ReferenceMedia's per-file load status).
// Summary: The registered Storybook browser runner includes this case and exercises it against a built Storybook with the puzzle and infinite scopes active.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲️Header

import { expect, test, type Locator, type Page } from "../../../../../.storybook/🧪️tests/🎭️playwright/🟦️.ts";
import { PerspectiveCamera, Vector3 } from "three";
import worldCorpus from "../🧫️fixtures/🌐️world-session/🔣️.json";
import { storyId, type StoryIndexEntry } from "../📚️storybook-index/🟦️.ts";

function significantConsoleErrors(messages: string[]): string[] {
  return messages.filter((text) => !/Failed to load resource:.*\b40[0-9]\b/i.test(text));
}

/** 🧪️ Base assertion shared by every story in this file, minus the 2D board-specific canvas lookup. */
async function expectStoryLoads(page: Page, title: string, name: string): Promise<() => void> {
  const response = await page.request.get("index.json");
  expect(response.ok()).toBe(true);
  const index = await response.json() as { entries: Record<string, StoryIndexEntry> };
  const id = storyId(index.entries, title, name);
  const pageErrors: Error[] = [];
  const consoleErrors: string[] = [];
  page.on("pageerror", (error) => pageErrors.push(error));
  page.on("console", (message) => {
    if (message.type() === "error") consoleErrors.push(message.text());
  });

  await page.goto(`iframe.html?id=${encodeURIComponent(id)}&viewMode=story`, { waitUntil: "domcontentloaded" });
  await expect(page.locator("body")).not.toContainText("Couldn't find story matching");
  await expect(page.locator("body")).not.toContainText("Failed to load the Storybook preview file");
  await page.waitForFunction(() => {
    const root = document.querySelector("#storybook-root");
    return !!root && root.childElementCount > 0;
  });

  return () => {
    expect(pageErrors.map((error) => error.message)).toEqual([]);
    expect(significantConsoleErrors(consoleErrors)).toEqual([]);
  };
}

async function expectWorldInstances(page: Page, count: number): Promise<void> {
  const host = page.locator(".semio-world-3d-host");
  await expect(host.locator("canvas")).toBeVisible();
  await expect.poll(async () => JSON.parse(await host.getAttribute("data-instances-json") ?? "[]").length).toBe(count);
}

async function expectWorldSelection(page: Page, ids: readonly string[]): Promise<void> {
  const host = page.locator(".semio-world-3d-host");
  for (const attribute of ["data-selection-json", "data-guest-selection-json"]) {
    await expect.poll(async () => JSON.parse(await host.getAttribute(attribute) ?? "null")?.selectedIds).toEqual(ids);
  }
}

type WorldPoint = [number, number, number];

async function clickCanvasPoint(page: Page, canvas: Locator, x: number, y: number): Promise<void> {
  await expect.poll(() => canvas.evaluate((element, point) => document.elementFromPoint(point.x, point.y) === element, { x, y })).toBe(true);
  await page.mouse.click(x, y);
}

async function clickFirstWorldInstance(page: Page, expectedId: string): Promise<void> {
  const host = page.locator(".semio-world-3d-host");
  const canvas = host.locator("canvas");
  const box = await canvas.boundingBox();
  expect(box).toBeTruthy();
  const instances = JSON.parse(await host.getAttribute("data-instances-json") ?? "[]") as { id: string; position: WorldPoint }[];
  expect(instances[0]!.id).toBe(expectedId);
  const pose = JSON.parse(await host.getAttribute("data-viewport-camera-json") ?? "null") as { position: WorldPoint; target: WorldPoint; up: WorldPoint | null; fov: number | null; zoom: number; projection: unknown };
  expect(pose.projection).toBe("perspective");
  const camera = new PerspectiveCamera(pose.fov ?? 45, box!.width / box!.height, 0.1, 1e8);
  camera.position.set(...pose.position);
  camera.up.set(...(pose.up ?? [0, 0, 1] as WorldPoint));
  camera.lookAt(...pose.target);
  camera.zoom = pose.zoom;
  camera.updateProjectionMatrix();
  camera.updateMatrixWorld();
  const point = new Vector3(...instances[0]!.position).project(camera);
  expect(Math.abs(point.x)).toBeLessThan(1);
  expect(Math.abs(point.y)).toBeLessThan(1);
  await clickCanvasPoint(page, canvas, box!.x + (point.x + 1) * box!.width / 2, box!.y + (1 - point.y) * box!.height / 2);
}

async function clickWorldBackground(page: Page): Promise<void> {
  const canvas = page.locator(".semio-world-3d-host canvas");
  const box = await canvas.boundingBox();
  expect(box).toBeTruthy();
  await clickCanvasPoint(page, canvas, box!.x + worldCorpus.emptyCornerInset, box!.y + worldCorpus.emptyCornerInset);
}

//#region puzzle/2d Fixtures
test("puzzle2d fixtures nakagin: real 180-node/179-edge fixture round-trips through Board2dHost", async ({ page }) => {
  const assertNoErrors = await expectStoryLoads(page, "🧩️puzzle🩻️2d/Fixtures", "Nakagin Capsule Tower");
  const debug = JSON.parse(await page.getByTestId("puzzle2d-fixture-debug").innerText());
  expect(debug.nodeCount).toBe(180);
  expect(debug.edgeCount).toBe(179);
  await expect(page.locator(".semio-board-2d-host")).toHaveAttribute("data-board-snapshot-parsed", "true");
  await expect(page.locator(".semio-board-2d-host canvas")).toBeVisible();
  assertNoErrors();
});

test("puzzle2d fixtures concrete forest: real fixture round-trips through Board2dHost", async ({ page }) => {
  const assertNoErrors = await expectStoryLoads(page, "🧩️puzzle🩻️2d/Fixtures", "Concrete Forest");
  const debug = JSON.parse(await page.getByTestId("puzzle2d-fixture-debug").innerText());
  expect(debug.nodeCount).toBe(1);
  expect(debug.edgeCount).toBe(0);
  await expect(page.locator(".semio-board-2d-host")).toHaveAttribute("data-board-snapshot-parsed", "true");
  await expect(page.locator(".semio-board-2d-host canvas")).toBeVisible();
  assertNoErrors();
});
//#endregion puzzle/2d Fixtures

//#region puzzle/3d World
test("puzzle3d world concrete forest: real fixture round-trips through World3dHost", async ({ page }) => {
  const assertNoErrors = await expectStoryLoads(page, "🧩️puzzle🧊️3d", "Concrete Forest");
  const debug = JSON.parse(await page.getByTestId("puzzle3d-world-debug").innerText());
  expect(debug.objectCount).toBe(1);
  expect(debug.selection).toEqual([]);
  await expectWorldInstances(page, debug.objectCount);
  await clickFirstWorldInstance(page, worldCorpus.concreteForest.selectedId);
  await expect.poll(async () => JSON.parse(await page.getByTestId("puzzle3d-world-debug").innerText()).selection).toEqual([worldCorpus.concreteForest.selectedId]);
  await expectWorldSelection(page, [worldCorpus.concreteForest.selectedId]);
  await clickWorldBackground(page);
  await expect.poll(async () => JSON.parse(await page.getByTestId("puzzle3d-world-debug").innerText()).selection).toEqual(worldCorpus.emptySelection);
  await expectWorldSelection(page, worldCorpus.emptySelection);
  assertNoErrors();
});

test("puzzle3d world nakagin: real 180-object fixture round-trips through World3dHost", async ({ page }) => {
  const assertNoErrors = await expectStoryLoads(page, "🧩️puzzle🧊️3d", "Nakagin Capsule Tower");
  const debug = JSON.parse(await page.getByTestId("puzzle3d-world-debug").innerText());
  expect(debug.objectCount).toBe(180);
  await expectWorldInstances(page, debug.objectCount);
  assertNoErrors();
});
//#endregion puzzle/3d World

//#region puzzle/5d Timeline
test("puzzle5d timeline nakagin: boots fully assembled (revealCount === partCount), scrubbing to the oldest checkpoint reveals just 1 part", async ({ page }) => {
  const assertNoErrors = await expectStoryLoads(page, "🧩️puzzle🕐️5d", "Nakagin Capsule Tower");
  const debug = page.getByTestId("puzzle5d-timeline-debug");
  const before = JSON.parse(await debug.innerText());
  expect(before.partCount).toBe(180);
  expect(before.revealCount).toBe(180);
  await expectWorldInstances(page, before.partCount);

  await page.locator(".semio-graph-timeline-host").getByRole("button").last().click();
  await expect.poll(async () => JSON.parse(await debug.innerText()).revealCount).toBe(1);
  await expectWorldInstances(page, 1);
  await clickWorldBackground(page);
  await page.evaluate(() => new Promise<void>(resolve => requestAnimationFrame(() => resolve())));
  expect(JSON.parse(await debug.innerText()).selection).toEqual(worldCorpus.emptySelection);
  await expectWorldSelection(page, worldCorpus.emptySelection);
  assertNoErrors();
});
//#endregion puzzle/5d Timeline

//#region infinite/WasmCanvas
test("infinite WasmCanvas mock session: paints without WASM and counts pointer events", async ({ page }) => {
  const assertNoErrors = await expectStoryLoads(page, "♾️infinite/WasmCanvas", "Mock Session");
  const canvas = page.locator(".semio-graph-wasm-canvas-story canvas");
  await expect(canvas).toBeVisible();
  const box = await canvas.boundingBox();
  expect(box).toBeTruthy();
  await page.mouse.click(box!.x + box!.width / 2, box!.y + box!.height / 2);
  await expect.poll(async () => JSON.parse(await page.getByTestId("graph-wasm-canvas-debug").innerText()).pointerEvents).toBeGreaterThan(0);
  assertNoErrors();
});
//#endregion infinite/WasmCanvas

//#region infinite/WorldR3f
test("infinite WorldR3f chunked field: renders a canvas with the chunked/LOD layer primitives, no console errors", async ({ page }) => {
  const assertNoErrors = await expectStoryLoads(page, "♾️infinite/WorldR3f", "Chunked Field");
  await expect(page.locator(".semio-world-r3f-story canvas")).toBeVisible();
  assertNoErrors();
});
//#endregion infinite/WorldR3f

//#region infinite/ReferenceMedia
test("infinite ReferenceMedia: every real infinite/fixture file reaches loaded status via referenceMediaPort", async ({ page }) => {
  const assertNoErrors = await expectStoryLoads(page, "♾️infinite/ReferenceMedia", "All Fixtures");
  for (const label of ["sketch.png", "abbau-aufbau-masterarbeit-grundriss.jpg", "rathaus-ahlen-grundriss.png", "site.pdf"]) {
    await expect(page.getByTestId(`reference-media-status-${label}`)).toContainText("status: loaded", { timeout: 30000 });
  }
  assertNoErrors();
});
//#endregion infinite/ReferenceMedia
