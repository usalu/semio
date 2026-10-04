import { strict as assert } from "node:assert";
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
import fixture from "../../../../🧫️fixtures/🪆️embedded-browser/🔣️.json";

type Configuration = { renderer: "react" | "wgpu"; serve: string; libraryModuleUrl: string; styleModuleUrl?: string; sessionFactoryModules?: { moduleUrl: string; exportName: string }[]; rendererModuleUrl?: string; rendererWasmUrl?: string; frameWorkerUrl?: string; roots: { rootId: string; plugins: { pluginId: string; moduleUrl: string }[] }[] };
type State = { id: string; locale: string; ready: boolean; error: string | null; intro: boolean; palette: boolean; controls: { id: string; rect: number[] }[]; surfaces: string[]; text: string; canvases: number; layout: number[] };
type Receipt = { phase: string; before: State[]; after: State[]; workers: number; input: unknown[]; evidence?: Readonly<Record<string, unknown>> };

/** 🪆️ Requires explicit per-root plugin authority and real public browser module URLs. */
export function parseEmbeddedBrowserConfiguration(value: unknown): Configuration {
  assert.ok(value && typeof value === "object" && !Array.isArray(value));
  const c = value as Configuration;
  assert.ok(c.renderer === "react" || c.renderer === "wgpu");
  const allowed = ["renderer", "serve", "libraryModuleUrl", "styleModuleUrl", "sessionFactoryModules", "rendererModuleUrl", "rendererWasmUrl", "frameWorkerUrl", "roots"];
  assert.ok(Object.keys(c).every(k => allowed.includes(k)));
  for (const key of ["serve", "libraryModuleUrl", ...(c.renderer === "wgpu" ? ["rendererModuleUrl", "rendererWasmUrl", "frameWorkerUrl"] : ["styleModuleUrl"])]) {
    const url = new URL((c as unknown as Record<string, string>)[key]!);
    assert.ok(["http:", "https:"].includes(url.protocol) && !url.username && !url.password);
  }
  assert.equal(c.roots?.length, fixture.roots.length);
  for (const [index, root] of c.roots.entries()) {
    assert.deepEqual(Object.keys(root).sort(), ["plugins", "rootId"]);
    assert.equal(root.rootId, fixture.roots[index]!.rootId);
    assert.ok(root.plugins.length > 0 && root.plugins.length <= 64);
    const ids = new Set<string>();
    for (const plugin of root.plugins) {
      assert.deepEqual(Object.keys(plugin).sort(), ["moduleUrl", "pluginId"]);
      assert.ok(plugin.pluginId.length > 0 && !ids.has(plugin.pluginId)); ids.add(plugin.pluginId);
      const url = new URL(plugin.moduleUrl); assert.ok(["http:", "https:"].includes(url.protocol) && !url.username && !url.password);
    }
    assert.deepEqual([...ids].sort(), [...fixture.roots[index]!.requiredPlugins].sort());
  }
  if (c.renderer === "react") {
    assert.ok(c.sessionFactoryModules && c.sessionFactoryModules.length > 0 && c.sessionFactoryModules.length <= 64);
    for (const source of c.sessionFactoryModules) {
      assert.deepEqual(Object.keys(source).sort(), ["exportName", "moduleUrl"]);
      assert.match(source.exportName, /^[A-Za-z_$][A-Za-z0-9_$]*$/);
      const url = new URL(source.moduleUrl); assert.ok(["http:", "https:"].includes(url.protocol) && !url.username && !url.password);
    }
  }
  return c;
}

/** 🎮️ Exercises physical isolation and awaited retirement on actual independently mounted renderers. */
export async function runEmbeddedBrowserAcceptance(args: readonly string[]): Promise<void> {
  const options = new Map<string, string>();
  for (let i = 0; i < args.length; i += 2) {
    assert.ok(["--configuration", "--output"].includes(args[i]!) && args[i + 1] && !options.has(args[i]!));
    options.set(args[i]!, args[i + 1]!);
  }
  assert.ok(options.has("--configuration") && options.has("--output"), "embedded acceptance requires --configuration and --output");
  const config = parseEmbeddedBrowserConfiguration(JSON.parse(readFileSync(resolve(options.get("--configuration")!), "utf8")));
  const output = resolve(options.get("--output")!); mkdirSync(output, { recursive: true });
  const { chromium } = await import("playwright");
  const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", `--use-angle=${process.platform === "darwin" ? "metal" : process.platform === "win32" ? "d3d11" : "vulkan"}`] });
  const context = await browser.newContext({ viewport: { width: fixture.viewport.width, height: fixture.viewport.height }, deviceScaleFactor: fixture.viewport.deviceScaleFactor });
  const page = await context.newPage(); const logs: string[] = []; const receipts: Receipt[] = []; const workers = new Set<unknown>();
  const log = (message: string) => { logs.push(message); if (logs.length > 8192) logs.shift(); writeFileSync(resolve(output, `embedded-${config.renderer}.log`), logs.join("\n") + "\n"); };
  page.on("console", m => log(`${m.type()} ${m.text()}`)); page.on("pageerror", e => log(`pageerror ${e.message}`));
  page.on("worker", worker => { if (config.frameWorkerUrl && new URL(worker.url()).pathname === new URL(config.frameWorkerUrl).pathname) { workers.add(worker); worker.on("close", () => workers.delete(worker)); } });
  const persist = () => { writeFileSync(resolve(output, `embedded-${config.renderer}.json`), JSON.stringify({ schemaVersion: 1, configuration: config, receipts }, null, 2)); writeFileSync(resolve(output, `embedded-${config.renderer}.log`), logs.join("\n") + "\n"); };
  try {
    const harness = new URL("__semio_embedded_acceptance__", config.serve).href;
    await page.route(harness, route => route.fulfill({ contentType: "text/html", body: '<!doctype html><html><head><meta charset="utf-8"></head><body></body></html>' }));
    await page.goto(harness, { waitUntil: "domcontentloaded" });
    await page.evaluate(() => { localStorage.clear(); document.body.replaceChildren(); document.body.style.cssText = "margin:0;background:#eee"; });
    await page.evaluate(async ({ config, fixture }) => {
      const state = window as unknown as { embeddedAcceptance: { mounts: Record<string, any>; stale: Record<string, any>; inputs: unknown[]; boot: (index: number) => Promise<void>; focusOwner: () => string | null } };
      if (config.styleModuleUrl) await import(config.styleModuleUrl);
      const module = await import(config.libraryModuleUrl);
      const mounts: Record<string, any> = {}; const stale: Record<string, any> = {}; const inputs: unknown[] = [];
      for (const spec of fixture.roots) {
        const root = document.createElement("div"); root.id = spec.rootId; const [x, y, width, height] = spec.rect;
        root.style.cssText = `position:absolute;left:${x}px;top:${y}px;width:${width}px;height:${height}px;overflow:hidden`; document.body.appendChild(root);
      }
      const focusOwner = () => config.renderer === "react" ? module.embeddedReactFocusOwner() : document.activeElement?.closest('[id^="shell-"]')?.id ?? null;
      for (const type of ["pointerdown", "pointerup", "keydown", "keyup"]) document.addEventListener(type, event => { const e = event as PointerEvent & KeyboardEvent; inputs.push({ type, trusted: event.isTrusted, root: (event.target as Element)?.closest('[id^="shell-"]')?.id ?? null, admittedRoot: focusOwner(), key: e.key ?? null, x: e.clientX ?? null, y: e.clientY ?? null }); }, true);
      const boot = async (index: number) => {
        const spec = fixture.roots[index]!; const plugins = config.roots[index]!.plugins;
        const frameWorkerUrl = index === 0 && !mounts[spec.rootId] ? undefined : config.frameWorkerUrl;
        if (mounts[spec.rootId]) stale[spec.rootId] = mounts[spec.rootId];
        mounts[spec.rootId] = config.renderer === "wgpu" ? await module.bootFrameworkOsWgpu({ rootId: spec.rootId, plugin: spec.plugin, plugins, locks: { locale: spec.locale }, rendererModuleUrl: config.rendererModuleUrl }, { rendererWasmUrl: config.rendererWasmUrl, frameWorkerUrl, suppressAutoIntroduction: spec.suppressAutoIntroduction }) : await module.mountEmbeddedReactOracle({ ...spec, plugins, sessionFactoryModules: config.sessionFactoryModules });
      };
      state.embeddedAcceptance = { mounts, stale, inputs, boot, focusOwner };
      await boot(0); await boot(1);
    }, { config, fixture });
    const observe = async (): Promise<State[]> => page.evaluate(async ({ renderer, fixture }) => {
      const mounted = (window as any).embeddedAcceptance.mounts;
      return Promise.all(fixture.roots.map(async spec => {
        const root = document.getElementById(spec.rootId)!; let controls: State["controls"]; let surfaces: string[];
        if (!root.children.length) return { id: spec.rootId, locale: "", ready: false, error: null, intro: false, palette: false, controls: [], surfaces: [], text: "", canvases: 0, layout: [0, 0] };
        if (renderer === "wgpu") {
          const chrome = JSON.parse(await mounted[spec.rootId].introspection.dumpChrome());
          controls = chrome.hits.map((h: any) => ({ id: h.controlId, rect: h.rect }));
          surfaces = chrome.surfaces.map((s: any) => `${s.level}:${s.id}`).sort();
        } else {
          const box = root.getBoundingClientRect();
          controls = Array.from(root.querySelectorAll<HTMLElement>("[id]")).map(e => { const r = e.getBoundingClientRect(); return { id: e.id, rect: [r.x - box.x, r.y - box.y, r.width, r.height] }; }).filter(c => c.rect[2]! > 0 && c.rect[3]! > 0);
          surfaces = Array.from(root.querySelectorAll("[data-window-id],[data-level=panel],[data-level=dialog]")).map(e => e.getAttribute("data-window-id") ?? e.id ?? e.getAttribute("data-level")!).sort();
        }
        const ready = root.hasAttribute("data-semio-os-ready") || root.querySelector("[data-shell-ready]") !== null;
        const layout = root.querySelector(renderer === "wgpu" ? "canvas" : '[data-slot="layout"]')?.getBoundingClientRect();
        return { id: spec.rootId, locale: root.lang || root.querySelector("[lang]")?.getAttribute("lang") || "", ready, error: root.getAttribute("data-semio-os-error") || root.querySelector("[data-shell-error]")?.getAttribute("data-shell-error") || null, intro: controls.some(c => c.id === fixture.introductionControl), palette: root.querySelector('[role="combobox"]') !== null, controls, surfaces, text: root.textContent ?? "", canvases: root.querySelectorAll("canvas").length, layout: [layout?.width ?? 0, layout?.height ?? 0] };
      }));
    }, { renderer: config.renderer, fixture });
    const wait = async (predicate: (states: State[]) => boolean, label: string) => {
      const deadline = Date.now() + 90_000;
      do { const states = await observe(); assert.ok(states.every(s => !s.error), JSON.stringify(states)); if (predicate(states)) return states; await page.waitForTimeout(150); } while (Date.now() < deadline);
      writeFileSync(resolve(output, `embedded-${config.renderer}-failed-state.json`), JSON.stringify({ phase: label, states: await observe() }, null, 2));
      throw new Error(`embedded acceptance timed out: ${label}`);
    };
    const record = async (phase: string, action: () => Promise<void>, law: (before: State[], after: State[]) => boolean) => {
      const before = await observe(); const cursor = await page.evaluate(() => (window as any).embeddedAcceptance.inputs.length);
      await action(); const after = await wait(states => law(before, states), phase);
      const input = await page.evaluate(cursor => (window as any).embeddedAcceptance.inputs.slice(cursor), cursor);
      assert.ok(input.every((event: any) => event.trusted));
      if (phase.startsWith("pointer-") || phase.startsWith("keyboard-")) {
        const owner = phase.endsWith("-a") ? "shell-a" : "shell-b";
        assert.ok(input.length > 0 && input.every((event: any) => (event.root ?? (event.type.startsWith("key") ? event.admittedRoot : null)) === owner), "physical input reached a foreign shell");
      }
      receipts.push({ phase, before, after, workers: workers.size, input }); persist();
      await page.screenshot({ path: resolve(output, `${config.renderer}-${phase}.png`) });
      console.log(`[DEBUG] embedded ${config.renderer} ${phase} accepted workers=${workers.size}`);
    };
    const suppress = async (id: string, value: boolean) => page.evaluate(({ id, value }) => (window as any).embeddedAcceptance.mounts[id].setIntroductionSuppressed(value), { id, value });
    await record("boot", async () => {}, (_, s) => s.every(s => s.ready) && !s[0]!.intro && s[1]!.intro);
    assert.deepEqual(receipts[0]!.after.map(s => s.locale), fixture.roots.map(s => s.locale));
    assert.deepEqual(receipts[0]!.after.map(s => s.layout), fixture.roots.map(s => s.rect.slice(2)), "shell chrome must use its own root dimensions");
    if (config.renderer === "wgpu") receipts[0]!.evidence = { rootAWorkerUrl: "public-library-default", rootBWorkerUrl: config.frameWorkerUrl };
    assert.equal(workers.size, config.renderer === "wgpu" ? 2 : 0);
    await record("release-a-introduction", () => suppress("shell-a", false), (_, s) => s[0]!.intro && s[1]!.intro);
    await record("suppress-b-introduction", () => suppress("shell-b", true), (_, s) => s[0]!.intro && !s[1]!.intro);
    await record("suppress-a-introduction", () => suppress("shell-a", true), (_, s) => s.every(s => !s.intro));
    for (const [index, id] of ["shell-a", "shell-b"].entries()) {
      await record(`pointer-${index === 0 ? "a" : "b"}`, async () => {
        const state = (await observe())[index]!; const control = state.controls.find(c => c.id === fixture.settingsControl); assert.ok(control);
        const root = await page.locator(`#${id}`).boundingBox(); assert.ok(root); const [x, y, width, height] = control.rect;
        await page.mouse.click(root.x + x! + width! / 2, root.y + y! + height! / 2);
      }, (b, s) => JSON.stringify(b[index]!.surfaces) !== JSON.stringify(s[index]!.surfaces) && JSON.stringify(b[1 - index]!.surfaces) === JSON.stringify(s[1 - index]!.surfaces));
      await record(`keyboard-${index === 0 ? "a" : "b"}`, async () => {
        const control = (await observe())[index]!.controls.find(c => c.id === fixture.settingsControl); assert.ok(control);
        const root = await page.locator(`#${id}`).boundingBox(); assert.ok(root); const [x, y, width, height] = control.rect;
        await page.mouse.click(root.x + x! + width! / 2, root.y + y! + height! / 2);
        const owner = await page.evaluate(() => (window as any).embeddedAcceptance.focusOwner());
        assert.equal(owner, id, "physical focus belongs to the selected shell");
        await page.keyboard.press(process.platform === "darwin" ? "Meta+p" : "Control+p");
      }, (_, s) => s[index]!.palette && !s[1 - index]!.palette);
      await page.keyboard.press("Escape"); await wait(s => !s[index]!.palette, "palette retirement");
    }
    await record("replace-a", () => page.evaluate(() => (window as any).embeddedAcceptance.boot(0)), (_, s) => s[0]!.ready && !s[0]!.intro && s[1]!.ready);
    assert.equal(workers.size, config.renderer === "wgpu" ? 2 : 0);
    await record("stale-retire-a", () => page.evaluate(async () => { const old = (window as any).embeddedAcceptance.stale["shell-a"]; await old(); if (old.introspection) { let code = ""; try { await old.introspection.dumpChrome(); } catch (e) { code = (e as Error).message; } if (code !== "wgpu-mount-retired") throw new Error("retired introspection remained live"); } }), (_, s) => s.every(s => s.ready));
    await record("retire-a", () => page.evaluate(async () => { const mount = (window as any).embeddedAcceptance.mounts["shell-a"]; await Promise.all([mount(), mount()]); }), (_, s) => !s[0]!.ready && s[0]!.canvases === 0 && s[1]!.ready);
    assert.equal(await page.locator("#shell-a").evaluate(e => e.children.length), 0); assert.equal(workers.size, config.renderer === "wgpu" ? 1 : 0);
    console.log(`[DEBUG] embedded ${config.renderer} retire-a awaited worker-close and empty owner`);
    await record("retire-b", () => page.evaluate(async () => { await (window as any).embeddedAcceptance.mounts["shell-b"](); }), (_, s) => s.every(s => !s.ready && s.canvases === 0));
    assert.equal(await page.locator("#shell-b").evaluate(e => e.children.length), 0); assert.equal(workers.size, 0);
    console.log(`[DEBUG] embedded ${config.renderer} retire-b awaited worker-close and empty owner`);
    await page.evaluate(harness => {
      const iframe = document.createElement("iframe"); iframe.id = "embedded-iframe"; iframe.allow = "webgpu"; iframe.src = harness;
      iframe.style.cssText = "position:absolute;left:20px;top:60px;width:760px;height:880px;border:0"; document.body.appendChild(iframe);
    }, harness);
    const iframe = page.frameLocator("#embedded-iframe"); await iframe.locator("body").waitFor();
    const frame = page.frames().find(f => f !== page.mainFrame() && f.url() === harness); assert.ok(frame);
    await frame.evaluate(async ({ config, fixture }) => {
      const root = document.createElement("div"); root.id = "iframe-shell"; root.style.cssText = "width:760px;height:880px"; document.body.appendChild(root);
      const module = await import(config.libraryModuleUrl); const spec = fixture.roots[0]!; const plugins = config.roots[0]!.plugins;
      if (config.styleModuleUrl) await import(config.styleModuleUrl);
      (window as any).iframeMount = config.renderer === "wgpu" ? await module.bootFrameworkOsWgpu({ rootId: root.id, plugin: spec.plugin, plugins, locks: { locale: spec.locale }, rendererModuleUrl: config.rendererModuleUrl }, { rendererWasmUrl: config.rendererWasmUrl, frameWorkerUrl: config.frameWorkerUrl, suppressAutoIntroduction: false }) : await module.mountEmbeddedReactOracle({ ...spec, rootId: root.id, plugins, sessionFactoryModules: config.sessionFactoryModules, suppressAutoIntroduction: false });
    }, { config, fixture });
    await iframe.locator("[data-semio-os-ready],[data-shell-ready]").first().waitFor({ timeout: 90_000 });
    await page.waitForTimeout(1500);
    const iframeIntro = await frame.evaluate(async ({ renderer, control }) => renderer === "wgpu" ? JSON.parse(await (window as any).iframeMount.introspection.dumpChrome()).hits.some((h: any) => h.controlId === control) : document.getElementById(control) !== null, { renderer: config.renderer, control: fixture.introductionControl });
    assert.equal(iframeIntro, false); assert.equal(workers.size, config.renderer === "wgpu" ? 1 : 0);
    await frame.evaluate(async () => { await (window as any).iframeMount(); });
    assert.equal(await iframe.locator("#iframe-shell").evaluate(e => e.children.length), 0); assert.equal(workers.size, 0);
    receipts.push({ phase: "iframe", before: [], after: [], workers: workers.size, input: [], evidence: { forcedIntroductionSuppression: !iframeIntro, awaitedRetirement: true } });
    assert.deepEqual(receipts.map(r => r.phase), fixture.phases);
    console.log(`[DEBUG] embedded ${config.renderer} iframe forced suppression and awaited retirement accepted`);
    persist();
  } catch (error) {
    log(`[DEBUG] embedded acceptance failed ${error instanceof Error ? error.stack : String(error)}`);
    await page.screenshot({ path: resolve(output, `embedded-${config.renderer}-fault.png`) }).catch(() => {});
    throw error;
  } finally { persist(); await browser.close(); }
}
