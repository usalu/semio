import { strict as assert } from "node:assert";
import { mkdirSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
import fixture from "../../../../🧫️fixtures/🎬️media-app-acceptance/🔣️.json";

type MediaAppReceipt = { readonly id: string; readonly url: string; readonly acceptedWindow: string; readonly acceptedNode: string; readonly token: string; readonly state: string; readonly kind: string; readonly mediaType: string; readonly reason: string; readonly rootCorrelated: boolean; readonly acceptedRoot: { readonly windowId: string; readonly windowGeneration: number; readonly nodeId: number; readonly key: string; readonly role: string } | null };

/** 🎬️ Checks the real guest→accepted WGPU root→frame Worker→DOM reservation without replacing any app or transport. */
export async function runBrowserMediaAppAcceptance(segments: readonly string[]): Promise<void> {
  const options = new Map<string, string>();
  for (let index = 0; index < segments.length; index += 2) {
    const name = segments[index]; const value = segments[index + 1];
    if (!["--serve", "--locale", "--output"].includes(name!) || !value || options.has(name!)) throw new Error("browser-media-acceptance expects --serve <url> --locale en|de --output <ticket-generated-directory>");
    options.set(name!, value);
  }
  const serve = options.get("--serve"); const locale = options.get("--locale"); const output = options.get("--output");
  if (!serve || (locale !== "en" && locale !== "de") || !output) throw new Error("browser-media-acceptance requires serve, explicit locale and output");
  const outputDirectory = resolve(output);
  mkdirSync(outputDirectory, { recursive: true });
  const { chromium } = await import("playwright");
  const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", `--use-angle=${process.platform === "darwin" ? "metal" : process.platform === "win32" ? "d3d11" : "vulkan"}`] });
  const context = await browser.newContext({ locale: locale === "en" ? "en-US" : "de-DE", viewport: { width: 1280, height: 900 } });
  const receipts: MediaAppReceipt[] = [];
  const logs: string[] = [];
  try {
    for (const specimen of fixture.cases) {
      const url = new URL(serve);
      for (const [key, value] of Object.entries({ plugin: fixture.plugin, app: specimen.app, example: fixture.example, role: fixture.role, mode: fixture.mode })) url.searchParams.set(key, value);
      const page = await context.newPage();
      page.on("console", (message) => { logs.push(`${specimen.id} ${message.type()} ${message.text()}`); });
      page.on("pageerror", (error) => { logs.push(`${specimen.id} pageerror ${error.message}`); });
      try {
        await page.goto(url.href, { waitUntil: "domcontentloaded" });
        await page.waitForFunction(() => document.querySelector("[data-semio-os-ready],[data-semio-os-error]") !== null, null, { timeout: 90_000 });
        const fault = await page.locator("[data-semio-os-error]").count() ? await page.locator("[data-semio-os-error]").getAttribute("data-semio-os-error") : null;
        if (fault) throw new Error(`media app renderer fault: ${fault} ${await page.locator("body").innerText()}`);
        const skip = page.getByRole("button", { name: locale === "en" ? "Skip" : "Überspringen", exact: true });
        if (await skip.count() && await skip.isVisible()) await skip.click();
        await page.locator(`[data-media-slot][data-media-state="${fixture.capability}"]`).waitFor({ state: "visible", timeout: 30_000 });
        const receipt = await page.locator(`[data-media-slot][data-media-state="${fixture.capability}"]`).evaluate(async (host, input) => {
          const element = host as HTMLElement;
          const windowId = element.dataset.mediaWindow!; const nodeId = element.dataset.uiNodeId!; const nodeKey = element.dataset.uiNodeKey!;
          const bridge = (window as unknown as { semioWgpuIntrospection?: { dumpAccessibility(windowId: string): Promise<string> } }).semioWgpuIntrospection;
          const projection = JSON.parse(await bridge!.dumpAccessibility(windowId)) as { windows: { windowId: string; windowGeneration: number; nodes: { nodeId: number; key: string; role: string }[] }[] };
          const surface = projection.windows.find((surface) => surface.windowId === windowId);
          const node = surface?.nodes.find((node) => String(node.nodeId) === nodeId && node.key === nodeKey);
          const acceptedRoot = surface && node ? { windowId: surface.windowId, windowGeneration: surface.windowGeneration, nodeId: node.nodeId, key: node.key, role: node.role } : null;
          const rootCorrelated = acceptedRoot !== null;
          console.debug(`[DEBUG] app-backed-media ${input.id} accepted=${windowId}/${nodeId}/${nodeKey} token=${element.dataset.mediaSlot} state=${element.dataset.mediaState} root-correlated=${rootCorrelated}`);
          return { id: input.id, url: location.href, acceptedWindow: windowId, acceptedNode: nodeKey, token: element.dataset.mediaSlot!, state: element.dataset.mediaState!, kind: element.dataset.mediaKind!, mediaType: element.dataset.mediaType!, reason: element.textContent ?? "", rootCorrelated, acceptedRoot };
        }, specimen);
        assert.equal(receipt.state, fixture.capability);
        assert.equal(receipt.kind, specimen.kind);
        assert.equal(receipt.mediaType, specimen.mediaType);
        assert.equal(receipt.reason, fixture.reason[locale]);
        assert.equal(receipt.rootCorrelated, true);
        assert.match(receipt.token, /^[0-9a-f]{64}$/);
        assert.equal(await page.locator("[data-media-slot] audio,[data-media-slot] video").count(), 0);
        assert.equal(await page.locator("#semio-wgpu-accessibility [data-node-id]").evaluateAll((nodes, key) => nodes.filter((node) => node.getAttribute("data-node-key") === key).length, receipt.acceptedNode), 0);
        receipts.push(receipt);
        await page.screenshot({ path: resolve(outputDirectory, `${specimen.id}-${locale}.png`), fullPage: true });
        console.log(`[DEBUG] app-backed-media ${specimen.id} ${locale} accepted-root-worker-dom unsupported verified`);
      } catch (error) {
        await page.screenshot({ path: resolve(outputDirectory, `${specimen.id}-${locale}-fault.png`), fullPage: true }).catch(() => {});
        throw error;
      } finally { await page.close(); }
    }
    writeFileSync(resolve(outputDirectory, `media-app-${locale}.json`), JSON.stringify({ schemaVersion: 1, receipts }, null, 2));
  } finally {
    writeFileSync(resolve(outputDirectory, `media-app-${locale}.log`), logs.join("\n") + "\n");
    await browser.close();
  }
}
