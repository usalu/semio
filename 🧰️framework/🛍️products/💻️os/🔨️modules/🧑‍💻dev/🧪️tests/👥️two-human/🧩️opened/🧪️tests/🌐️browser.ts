import { expect, test } from "bun:test";
import { createServer } from "node:http";
import { withBuildTestInvocationV1 } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/📥️invocation/🟦️.ts";
import { resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { existsSync } from "node:fs";
import { ensureRepositoryBrowserPathV1, prepareRepositoryBrowserV1 } from "../../../../../../../🦑️repo/🔨️modules/📚️library/🧪️testing/🌐️browser/🧰️preparation/🟦️.ts";
import { PLAYWRIGHT_MODULE_SPECIFIER, browserLaunchOptionsV1 } from "../../../../../../../🦑️repo/🔨️modules/📚️library/🧪️testing/🌐️browser/🟦️.ts";
import { openedBlobMimeV1, openedUiOwnerV1 } from "../🔒️adapter/🟦️.ts";
import { openSessions } from "../../🔑️sessions/🟦️.ts";

test("real browser port observes, interacts, chooses, downloads and aborts", async () => {
  const repoRoot = fileURLToPath(new URL("../../../../../../../../../", import.meta.url));
  const html = '<button id="click" onclick="this.textContent=\'clicked\'">start</button><input id="key"><input id="file" type="file" multiple><a id="download" download="report.txt" href="data:text/plain,hello">download</a><button class="duplicate">a</button><button class="duplicate">b</button>' + '<span class="many">x</span>'.repeat(65) + '<p id="long">' + 'x'.repeat(8193) + '</p>' + `<canvas id="canvas" width="16" height="16"></canvas><a id="blob" download="report.json">blob</a><a id="revoked">revoked</a><script>const ctx=document.getElementById('canvas').getContext('2d');ctx.fillStyle='red';ctx.fillRect(0,0,8,16);ctx.fillStyle='blue';ctx.fillRect(8,0,8,16);const good=URL.createObjectURL(new Blob(['{}'],{type:'application/json'}));document.getElementById('blob').href=good;const gone=URL.createObjectURL(new Blob(['x'],{type:'text/plain'}));document.getElementById('revoked').href=gone;URL.revokeObjectURL(gone)</script>`;
  const server = createServer((_request, response) => { response.writeHead(200, { "content-type": "text/html" }); response.end(html); });
  await new Promise<void>((resolve, reject) => { server.once("error", reject); server.listen(0, "127.0.0.1", resolve); });
  try {
    await withBuildTestInvocationV1(async invocation => prepareRepositoryBrowserV1(repoRoot, invocation));
    ensureRepositoryBrowserPathV1(repoRoot, process.env);
    const { chromium }: typeof import("playwright") = await import(PLAYWRIGHT_MODULE_SPECIFIER);
    const browser = await chromium.launch(browserLaunchOptionsV1());
    try {
      const page = await browser.newPage();
      const address = server.address();
      if (address === null || typeof address === "string") throw Error("Missing loopback address");
      await page.goto(`http://127.0.0.1:${address.port}`);
      const controller = new AbortController(), owner = openedUiOwnerV1(page, controller.signal), ui = owner.ui;
      expect((await ui.observe("#click", ["id"]))[0]?.text).toBe("start");
      expect((await ui.canvas("#canvas"))?.colors).toBe(2);
      await ui.click("#click");
      expect((await ui.observe("#click", []))[0]?.text).toBe("clicked");
      await ui.press("#key", "A");
      await ui.chooseFiles("#file", [resolve(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/👥️two-human/🧩️opened/🧫️fixtures/🔣️.json")]);
      const file = await page.locator("#file").evaluate((element: HTMLInputElement) => element.files?.length);
      expect(file).toBe(1);
      const temporary = page.waitForEvent("download").then(download => download.path());
      const download = await ui.download("#download");
      const temporaryPath = await temporary;
      expect(download.name).toBe("report.txt");
      expect(new TextDecoder().decode(download.bytes)).toBe("hello");
      expect(temporaryPath === null || existsSync(temporaryPath)).toBe(false);
      expect(download.mime).toBeNull();
      const liveUrl = (await ui.observe("#blob", ["href"]))[0]!.attributes.href!;
      const revokedUrl = (await ui.observe("#revoked", ["href"]))[0]!.attributes.href!;
      expect(await openedBlobMimeV1(page, liveUrl)).toBe("application/json");
      expect(await openedBlobMimeV1(page, revokedUrl)).toBeNull();
      expect(await openedBlobMimeV1(page, "http://127.0.0.1/")).toBeNull();
      const blobDownload = await ui.download("#blob");
      expect(blobDownload.name).toBe("report.json");
      expect(blobDownload.mime).toBe("application/json");
      expect(new TextDecoder().decode(blobDownload.bytes)).toBe("{}");
      await expect(ui.click(".duplicate")).rejects.toThrow("exactly one");
      await expect(ui.observe(".many", [])).rejects.toThrow("bound");
      await expect(ui.observe("#long", [])).rejects.toThrow("bound");
      expect((await ui.observe("#long", [], false))[0]?.text).toBe("");
      await expect(ui.observe("#click", Array(17).fill("id"))).rejects.toThrow("bound");
      await expect(ui.chooseFiles("#file", ["relative"])).rejects.toThrow("bound");
      await expect(ui.chooseFiles("#file", [])).rejects.toThrow("bound");
      const pending = ui.click("#absent");
      setTimeout(() => controller.abort(new Error("cancelled")), 50);
      await expect(pending).rejects.toThrow("cancelled");
      await owner.finish();
      expect(() => ui.observe("#click", [])).toThrow("closed");
      const baseline = browser.contexts().length;
      let created = 0;
      const failingBrowser = new Proxy(browser, { get(target, key) {
        if (key === "newContext") return async (options: Parameters<typeof browser.newContext>[0]) => { const context = await target.newContext(options); if (++created === 2) await context.close(); return context; };
        const value = Reflect.get(target, key, target);
        return typeof value === "function" ? value.bind(target) : value;
      } });
      const humans = [{ label: "a", email: "a@example.test", password: "unused" }, { label: "b", email: "b@example.test", password: "unused" }];
      await expect(openSessions(failingBrowser, ["http://127.0.0.1"], humans, "en-US")).rejects.toThrow();
      expect(created).toBe(2);
      expect(browser.contexts().length).toBe(baseline);
    } finally { await browser.close(); }
  } finally { await new Promise<void>((resolve, reject) => server.close(error => error ? reject(error) : resolve())); }
}, 60_000);
