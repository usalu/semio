// #region 🧲️Header
// 💻️ .storybook/framework-hosts-wasm.spec.ts
// Specs: End-to-end checks for the framework renderer hosts backed by prebuilt Rust/WASM engines, inside the
// aggregated Storybook static build: `NodeGraphHost` (workflow + flow-graph variants), `TextEditorHost`,
// `Paint2dHost`, `TiledMapHost`, `🗺️WorldTerrainLayer`, `World3dHost`.
// Summary: Loads each story's `iframe.html?id=...`, asserts a clean boot (no page/console errors, storybook-root
// populated) and a host-specific marker element/text. Tile/DEM fetches against intentionally-missing storybook
// paths produce benign `Failed to load resource … 404` console entries, which `significantConsoleErrors` filters
// out (mirrors `puzzle-2d.spec.ts`).
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲️Header

import { expect, test, type Page } from "../../../../../../../../.storybook/🧪️tests/🎭️playwright/🟦️.ts";
import { PNG } from "pngjs";
import terrariumFixture from "../../../../../../../🔨️modules/🗺️surface/🏔️terrain/🧪️tests/🧫️fixtures/🏔️terrarium-tile/🔣️.json" with { type: "json" };

//#region Helpers
function significantConsoleErrors(messages: string[]): string[] {
  return messages.filter((text) => !/Failed to load resource:.*\b404\b/i.test(text));
}

async function expectHostStory(page: Page, storyId: string): Promise<void> {
  const pageErrors: Error[] = [];
  const consoleErrors: string[] = [];
  page.on("pageerror", (error) => pageErrors.push(error));
  page.on("console", (message) => {
    if (message.type() === "error") consoleErrors.push(message.text());
  });

  await page.goto(`iframe.html?id=${storyId}&viewMode=story`, { waitUntil: "domcontentloaded" });
  await expect(page.locator("body")).not.toContainText("Couldn't find story matching");
  await expect(page.locator("body")).not.toContainText("Failed to load the Storybook preview file");
  await page.waitForFunction(() => {
    const root = document.querySelector("#storybook-root");
    return !!root && root.childElementCount > 0;
  });

  expect(pageErrors.map((error) => error.message)).toEqual([]);
  expect(significantConsoleErrors(consoleErrors)).toEqual([]);
}
//#endregion Helpers

//#region NodeGraphHost
test("NodeGraphHost workflow: boots the real GraphSession WASM engine and renders the host shell", async ({ page }) => {
  await expectHostStory(page, "🛠️framework🔌️hosts-nodegraphhost--workflow");
  await expect(page.locator(".semio-node-graph-host")).toBeVisible();
});

test("NodeGraphHost flow graph: boots the real FlowSession WASM engine (isFlowGraphScene routing)", async ({ page }) => {
  await expectHostStory(page, "🛠️framework🔌️hosts-nodegraphhost--flow-graph");
  await expect(page.locator(".semio-node-graph-host")).toBeVisible();
});
//#endregion NodeGraphHost

//#region TextEditorHost
test("TextEditorHost: boots the real EditorSession WASM engine and shows the document buffer", async ({ page }) => {
  await expectHostStory(page, "🛠️framework🔌️hosts-texteditorhost--jack-document");
  await expect(page.locator(".semio-text-editor-host")).toBeVisible();
  await expect(page.locator("body")).toContainText("MATCH");
});

test("TextEditorHost with diagnostics: renders the diagnostics readout", async ({ page }) => {
  await expectHostStory(page, "🛠️framework🔌️hosts-texteditorhost--with-diagnostics");
  await expect(page.locator("body")).toContainText("example diagnostic");
});
//#endregion TextEditorHost

//#region Paint2dHost
test("Paint2dHost composite view: boots the real RasterSession WASM engine", async ({ page }) => {
  await expectHostStory(page, "🛠️framework🔌️hosts-paint2dhost--composite-view");
  await expect(page.locator(".semio-paint-2d-canvas-surface")).toBeVisible();
  await expect(page.locator('[data-view-mode="composite"]').first()).toBeVisible();
});

test("Paint2dHost navigator view: shows the composite viewport overlay channel", async ({ page }) => {
  await expectHostStory(page, "🛠️framework🔌️hosts-paint2dhost--navigator-view");
  await expect(page.locator('[data-view-mode="navigator"]').first()).toBeVisible();
});
//#endregion Paint2dHost

//#region TiledMapHost
test("TiledMapHost vector render: boots the real MapSession WASM engine", async ({ page }) => {
  await expectHostStory(page, "🛠️framework🔌️hosts-tiledmaphost--vector-render");
});

test("TiledMapHost image render: boots with the image render mode", async ({ page }) => {
  await expectHostStory(page, "🛠️framework🔌️hosts-tiledmaphost--image-render");
});
//#endregion TiledMapHost

//#region World3dHost
test("World3dHost minimal viewport: renders the pure r3f viewport (no WASM engine)", async ({ page }) => {
  await expectHostStory(page, "🛠️framework🔌️hosts-world3dhost--minimal-viewport");
});

test("World3dHost terrain viewport: mounts WorldTerrainLayer against the real TerrainSession WASM engine", async ({ page }) => {
  await expectHostStory(page, "🛠️framework🔌️hosts-world3dhost--terrain-viewport");
});
//#endregion World3dHost

//#region WorldTerrainLayer
test("WorldTerrainLayer hypsometric ramp: boots the real TerrainSession WASM engine inside a standalone r3f Canvas", async ({ page }) => {
  await expectHostStory(page, "🛠️framework🔌️hosts-worldterrainlayer--hypsometric-ramp");
});

test("WorldTerrainLayer retries a failed elevation request for the visible camera target", async ({ page }) => {
  const png = new PNG({ width: 256, height: 256 });
  for (let offset = 0; offset < png.data.length; offset += 4) png.data.set(terrariumFixture.pixel, offset);
  const bytes = PNG.sync.write(png);
  const { data } = PNG.sync.read(bytes);
  expect((data.readUInt16BE(0) + data[2] / 256 - 32768) * terrariumFixture.exaggeration).toBe(terrariumFixture.expectedMeshHeight);
  const centerPath = "/storybook-missing-dem/14/8635/5383.png";
  let attempts = 0;
  await page.route("**/storybook-missing-dem/**/*.png", async (route) => {
    if (new URL(route.request().url()).pathname !== centerPath) {
      await route.fulfill({ status: 404 });
      return;
    }
    attempts += 1;
    await route.fulfill(attempts === 1 ? { status: 503 } : { contentType: "image/png", body: bytes });
  });
  const response = page.waitForResponse((result) => result.ok() && new URL(result.url()).pathname === centerPath);
  await page.goto("iframe.html?id=🛠️framework🔌️hosts-worldterrainlayer--hypsometric-ramp&viewMode=story", { waitUntil: "domcontentloaded" });
  expect((await response).headers()["content-type"]).toBe("image/png");
  expect(attempts).toBe(2);
});
//#endregion WorldTerrainLayer
