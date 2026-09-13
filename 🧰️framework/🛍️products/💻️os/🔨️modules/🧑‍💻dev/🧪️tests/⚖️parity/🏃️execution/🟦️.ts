/** 🧩️ Semantic parity execution owner. */

import { repoCacheDirectory } from "../../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/🟦️.ts";

import {
  BundleScript,
  ScriptRouter,
  buildBudgetMs,
  daemonBudgetOpts,
  describeDevPortOccupant,
  devServerUrl,
  getWorkspaceRoot,
  getRepoMetaDir,
  isDevPortInUse,
  loadFrameworkOsPlaygroundCatalog,
  wgpuDevPlayUrl,
  runBundleScriptMain,
  runCmd,
  runCmdStatus,
  runBunxStatus,
  runNodeBinStatus,
  runProbe,
  runVitest,
  spawnDaemon,
  type SpawnDaemonHandle,
  runViteBunxDev,
  frameworkOsPlaygroundDefaultPort,
  frameworkOsLockedPrefsEnv,
  resolveTestLevel,
  atTestLevel,
  cargoProfileDir,
  selectComponentWasmProfile,
  semioBuildMode,
  semioShipEnv,
} from "../../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

import { DEFAULT_HOST_VARIANT } from "../../../../🔌️plugin/📇️registry/🤖️generated/🎮️playgrounds/🟦️.ts";

const repoRoot = getWorkspaceRoot();

import { PLAYWRIGHT_MODULE_SPECIFIER, playgroundCatalog } from "../../../../🔌️plugin/🏗️build/📋️plan/🟦️.ts";

import { ParityServerHandle, findFreeParityPortPair, parityDevUrl, parityPortsForShard, prebuildParityPlugin, startParityDevServer, stopParityDevServer } from "../🌐️server-pool/🟦️.ts";

import { BootStatus, ParityPlaygroundReport, ParityRenderer, compareParityStructural, dumpReactStructure, dumpWgpuFrameStats, dumpWgpuStructure } from "../🏗️structure/🟦️.ts";

import { isParityStaleBridge, parityOutDir, writeParityReport } from "../📊️report/🟦️.ts";

import { PARITY_PROBE_CATALOG, PARITY_STATE_PROBE_SUITE, ProbeRunResult, runParityProbe, runParityProbeSuite } from "../🔬️probe/🟦️.ts";

import { PARITY_PIXEL_REGION_KINDS, compareParityRegion, decodeParityScreenshot } from "../🖼️pixels/🟦️.ts";



//#endregion 🔖️PixelCompare

//#region 🔖️Triage
const PARITY_BOOT_TIMEOUT_MS = Number(process.env.PARITY_RUNTIME_BOOT_TIMEOUT_MS ?? 180_000);

/** 🧬️terra-parity-rebaseline: the exact TypeError `🟦️.ts`'s `loadActor` throws —
 * `const api = await bridge.createActorApi(actorId);` on a module whose export is `undefined` — once it
 * crosses `ShardClient.activate`'s reject (`🧵️shard-client.ts` `entry.reject(graftWorkerStack(...))`) and
 * surfaces as an unhandled rejection on the page. Matched on BOTH the property-access phrasing V8 uses
 * (`bridge.createActorApi is not a function` / `undefined is not an object (evaluating
 * 'bridge.createActorApi')`) and the bare symbol, so a wording change in one engine doesn't silently stop
 * matching in the other. Deliberately narrow — this must not catch unrelated "X is not a function" defects,
 * which are real regressions, not stale fixtures. */
const PARITY_STALE_BRIDGE_RE = /createActorApi/;

/** 🪜️Boot-triage ladder — each rung is a distinct terminal status, never conflated with a structural/pixel mismatch. */
async function triageParityBoot(page: import("playwright").Page, renderer: ParityRenderer, url: string): Promise<{ readonly status: BootStatus; readonly detail?: string }> {
  const pageErrors: string[] = [];
  page.on("pageerror", (e) => pageErrors.push(String(e)));
  page.on("console", (m) => {
    if (m.type() === "error") pageErrors.push(m.text());
  });
  const staleBridgeHit = (): string | undefined => pageErrors.find((e) => PARITY_STALE_BRIDGE_RE.test(e));
  try {
    await page.goto(url, { waitUntil: "domcontentloaded", timeout: 30_000 });
  } catch (e) {
    return { status: "SERVER-FAIL", detail: String(e) };
  }
  if (renderer === "react") {
    try {
      await page.waitForFunction(() => document.querySelectorAll("#root *").length > 20, { timeout: PARITY_BOOT_TIMEOUT_MS });
    } catch {
      const stale = staleBridgeHit();
      return stale ? { status: "STALE-BRIDGE", detail: stale } : { status: "BOOT-TIMEOUT", detail: "react #root never populated" };
    }
    const nodeCount = await page.evaluate(() => document.querySelectorAll("[data-ui-path]").length);
    if (nodeCount === 0) {
      const stale = staleBridgeHit();
      return stale ? { status: "STALE-BRIDGE", detail: stale } : { status: "DUMP-EMPTY", detail: "no data-ui-path nodes" };
    }
    // 🩹️ The shell itself can mount (>20 root nodes, non-empty dump) while ONE plugin/extension actor
    // inside it fails to activate — that failure never blocks `#root`, so it must be checked even on an
    // otherwise-PASSing boot, or a stale-bridge variant silently reports PASS.
    const stale = staleBridgeHit();
    return stale ? { status: "STALE-BRIDGE", detail: stale } : { status: "PASS" };
  }
  try {
    await page.waitForFunction(() => document.querySelector("#semio-wgpu-canvas") != null, { timeout: PARITY_BOOT_TIMEOUT_MS });
  } catch {
    const stale = staleBridgeHit();
    return stale ? { status: "STALE-BRIDGE", detail: stale } : { status: "BOOT-TIMEOUT", detail: "wgpu canvas never mounted" };
  }
  if (pageErrors.some((e) => /NoCompatibleDevice|WebGPU/i.test(e))) return { status: "ENV-FAIL", detail: pageErrors.join(" | ") };
  try {
    await page.waitForFunction(() => typeof (window as unknown as { semioWgpuIntrospection?: { dumpStructure?: unknown } }).semioWgpuIntrospection?.dumpStructure === "function", { timeout: PARITY_BOOT_TIMEOUT_MS });
  } catch {
    const stale = staleBridgeHit();
    return stale ? { status: "STALE-BRIDGE", detail: stale } : { status: "BOOT-TIMEOUT", detail: "wgpu introspection hook never appeared" };
  }
  const dump = await dumpWgpuStructure(page);
  if (dump.nodes.length === 0) {
    const stale = staleBridgeHit();
    return stale ? { status: "STALE-BRIDGE", detail: stale } : { status: "DUMP-EMPTY", detail: "wgpu structural dump empty (plugin-bridge/kernel wiring)" };
  }
  const stats = await dumpWgpuFrameStats(page);
  if (stats && stats.drawCalls === 0) return { status: "BLANK-PAINT", detail: "zero draw calls (paint pipeline)" };
  const stale = staleBridgeHit();
  return stale ? { status: "STALE-BRIDGE", detail: stale } : { status: "PASS" };
}

//#endregion 🔖️Report

//#region 🔖️Sweep
/** 🎭️ Points playwright at the shared cache root's `tools/ms-playwright` that `📜️script.ts setup`
 * actually populates (`bunx playwright install --with-deps chromium`). Without this, `chromium.launch()`
 * falls back to the user-global `~/Library/Caches/ms-playwright`, which holds whatever an unrelated
 * project installed — here a stale `chromium_headless_shell-1223` against the required `-1234` — and
 * every parity run dies with "Executable doesn't exist" suggesting `npx playwright install`, i.e. a
 * download, for a browser the repo had already installed. The storybook runner (root `📜️script.ts`,
 * `🔖️TestScript`) already sets this.
 *
 * terra-parity-rebaseline: hoisted out of `verifyParityVariant` (the only call site that set this
 * before today) into a shared helper, and now ALSO called from `ParityTriageScript`/`ParityProbeScript`
 * — both launch `chromium` directly without going through `verifyParityVariant` and were dying on the
 * exact same stale-global-cache error, which is what made even a single `parity triage <variant>`
 * unrunnable (measured: `Executable doesn't exist at .../ms-playwright/chromium_headless_shell-1234/...`,
 * exit 1) before this fix — never mind a 58-variant sweep. */
function ensureParityPlaywrightBrowsersPath(): void {
  process.env.PLAYWRIGHT_BROWSERS_PATH ??= repoCacheDirectory(repoRoot, "tools", "ms-playwright");
}

async function verifyParityVariant(variant: string, ports: { readonly react: number; readonly wgpu: number }, opts: { readonly skipDev?: boolean } = {}): Promise<ParityPlaygroundReport> {
  const start = Date.now();
  ensureParityPlaywrightBrowsersPath();
  const { chromium } = await import(PLAYWRIGHT_MODULE_SPECIFIER);
  const browser = await chromium.launch({ headless: process.env.HEADED !== "1", args: ["--use-angle=swiftshader", "--enable-unsafe-swiftshader", "--enable-unsafe-webgpu"] });
  let reactServer: ParityServerHandle | undefined;
  let wgpuServer: ParityServerHandle | undefined;
  try {
    if (!opts.skipDev) {
      await prebuildParityPlugin(variant);
      reactServer = await startParityDevServer("react", variant, ports.react);
      wgpuServer = await startParityDevServer("wgpu", variant, ports.wgpu);
    }
    const reactPage = await browser.newPage({ viewport: { width: 1280, height: 720 } });
    const wgpuPage = await browser.newPage({ viewport: { width: 1280, height: 720 } });
    const reactBoot = await triageParityBoot(reactPage, "react", parityDevUrl("react", variant, ports.react));
    const wgpuBoot = await triageParityBoot(wgpuPage, "wgpu", parityDevUrl("wgpu", variant, ports.wgpu));
    if (reactBoot.status !== "PASS" || wgpuBoot.status !== "PASS") {
      return { variant, boot: { react: reactBoot.status, wgpu: wgpuBoot.status, detail: reactBoot.detail ?? wgpuBoot.detail }, durationMs: Date.now() - start };
    }
    await reactPage.waitForTimeout(400);
    await wgpuPage.waitForTimeout(400);
    const reactDump = await dumpReactStructure(reactPage);
    const wgpuDump = await dumpWgpuStructure(wgpuPage);
    const structural = compareParityStructural(reactDump, wgpuDump);
    const outDir = parityOutDir();
    const reactPng = await decodeParityScreenshot(reactPage, await reactPage.screenshot());
    const wgpuPng = await decodeParityScreenshot(wgpuPage, await wgpuPage.screenshot());
    const wgpuPaths = new Set(wgpuDump.nodes.map((n) => n.path));
    const regionNodes = reactDump.nodes.filter((n) => PARITY_PIXEL_REGION_KINDS.has(n.kind) && wgpuPaths.has(n.path));
    const regions = await Promise.all(regionNodes.map((n) => compareParityRegion(reactPage, reactPng, wgpuPng, n, outDir, variant)));
    const failingRegions = regions.filter((r) => r.ratio > r.threshold);
    // 🎬️Runs regardless of the structural/pixel outcome above (not gated on their PASS) — behavioral
    // parity is a distinct axis (interaction-driven dynamic state vs. static end-state), and a
    // static mismatch elsewhere shouldn't hide whether app state still transitions correctly. Wrapped
    // defensively: a probe-runner exception (e.g. a page closing mid-step) must not take down the
    // whole `verifyParityVariant` call, only degrade `behavioral` to a diagnosable FAIL.
    let behavioral: ProbeRunResult | undefined;
    try {
      behavioral = await runParityProbe(reactPage, wgpuPage, PARITY_STATE_PROBE_SUITE.steps);
    } catch (e) {
      behavioral = { status: "FAIL", steps: [{ index: 0, step: { kind: "settle", ms: 0 }, status: "FAIL", detail: `probe runner threw: ${String(e)}` }] };
    }
    return {
      variant,
      boot: { react: reactBoot.status, wgpu: wgpuBoot.status },
      structural,
      pixel: { status: failingRegions.length === 0 ? "PASS" : "FAIL", regions: failingRegions },
      behavioral,
      durationMs: Date.now() - start,
    };
  } finally {
    await browser.close();
    if (reactServer) stopParityDevServer(reactServer);
    if (wgpuServer) stopParityDevServer(wgpuServer);
  }
}

class ParitySmokeScript extends BundleScript {
  async run(): Promise<void> {
    const variant = process.env.SEMIO_PLUGIN || DEFAULT_HOST_VARIANT;
    const report = await verifyParityVariant(variant, findFreeParityPortPair());
    console.log(JSON.stringify(report, null, 2));
    if (report.boot.react !== "PASS" || report.boot.wgpu !== "PASS") {
      throw new Error(`parity smoke FAILED: boot react=${report.boot.react} wgpu=${report.boot.wgpu}${report.boot.detail ? ` (${report.boot.detail})` : ""}`);
    }
    console.log(`parity smoke PASS for ${variant}: structural=${report.structural?.status} pixel=${report.pixel?.status} behavioral=${report.behavioral?.status} (${report.durationMs}ms)`);
  }
}

class ParityTriageScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const variant = segments[0] || process.env.SEMIO_PLUGIN || DEFAULT_HOST_VARIANT;
    const ports = findFreeParityPortPair();
    ensureParityPlaywrightBrowsersPath();
    const { chromium } = await import(PLAYWRIGHT_MODULE_SPECIFIER);
    const browser = await chromium.launch({ headless: process.env.HEADED !== "1", args: ["--use-angle=swiftshader", "--enable-unsafe-swiftshader", "--enable-unsafe-webgpu"] });
    // 🩹️terra-parity-rebaseline: `reactServer`/`wgpuServer` used to be `const`, ASSIGNED BEFORE this
    // `try`, so a throw from the SECOND `startParityDevServer` call (e.g. wgpu's cold cargo build
    // exceeding `PARITY_BOOT_BUDGET_MS`) left the first server's process running with nothing left
    // holding a reference to stop it — confirmed leaked in practice (a react `vite` + a wgpu `trunk`
    // process both still bound to their ports well after the command had exited). `let` + assignment
    // INSIDE the try, guarded in `finally`, is the same safe shape `verifyParityVariant` already uses.
    let reactServer: ParityServerHandle | undefined;
    let wgpuServer: ParityServerHandle | undefined;
    try {
      reactServer = await startParityDevServer("react", variant, ports.react);
      wgpuServer = await startParityDevServer("wgpu", variant, ports.wgpu);
      const reactPage = await browser.newPage({ viewport: { width: 1280, height: 720 } });
      const wgpuPage = await browser.newPage({ viewport: { width: 1280, height: 720 } });
      const reactBoot = await triageParityBoot(reactPage, "react", parityDevUrl("react", variant, ports.react));
      const wgpuBoot = await triageParityBoot(wgpuPage, "wgpu", parityDevUrl("wgpu", variant, ports.wgpu));
      console.log(`triage ${variant}: react=${reactBoot.status}${reactBoot.detail ? ` (${reactBoot.detail})` : ""}`);
      console.log(`triage ${variant}: wgpu=${wgpuBoot.status}${wgpuBoot.detail ? ` (${wgpuBoot.detail})` : ""}`);
    } finally {
      await browser.close();
      if (reactServer) stopParityDevServer(reactServer);
      if (wgpuServer) stopParityDevServer(wgpuServer);
    }
  }
}

/** 🎬️Standalone entry point for JUST the behavioral probe suite — boots both dev servers, triages
 * boot, then runs `PARITY_PROBE_CATALOG[suiteName]` (default `"shell"`) without paying for the
 * structural/pixel comparison `verifyParityVariant` also does. Useful for iterating on a probe suite
 * itself without re-running the (slower) full `verify`. */
class ParityProbeScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const variant = segments[0] || process.env.SEMIO_PLUGIN || DEFAULT_HOST_VARIANT;
    const suiteName = segments[1] || "state";
    const suite = PARITY_PROBE_CATALOG[suiteName];
    if (!suite) throw new Error(`unknown probe suite: ${suiteName} (known: ${Object.keys(PARITY_PROBE_CATALOG).join(", ")})`);
    const ports = findFreeParityPortPair();
    ensureParityPlaywrightBrowsersPath();
    const { chromium } = await import(PLAYWRIGHT_MODULE_SPECIFIER);
    const browser = await chromium.launch({ headless: process.env.HEADED !== "1", args: ["--use-angle=swiftshader", "--enable-unsafe-swiftshader", "--enable-unsafe-webgpu"] });
    // 🩹️terra-parity-rebaseline: same leak fix as `ParityTriageScript` above — see its comment.
    let reactServer: ParityServerHandle | undefined;
    let wgpuServer: ParityServerHandle | undefined;
    try {
      reactServer = await startParityDevServer("react", variant, ports.react);
      wgpuServer = await startParityDevServer("wgpu", variant, ports.wgpu);
      const reactPage = await browser.newPage({ viewport: { width: 1280, height: 720 } });
      const wgpuPage = await browser.newPage({ viewport: { width: 1280, height: 720 } });
      const reactBoot = await triageParityBoot(reactPage, "react", parityDevUrl("react", variant, ports.react));
      const wgpuBoot = await triageParityBoot(wgpuPage, "wgpu", parityDevUrl("wgpu", variant, ports.wgpu));
      if (reactBoot.status !== "PASS" || wgpuBoot.status !== "PASS") {
        throw new Error(`parity probe FAILED: boot react=${reactBoot.status} wgpu=${wgpuBoot.status}${(reactBoot.detail ?? wgpuBoot.detail) ? ` (${reactBoot.detail ?? wgpuBoot.detail})` : ""}`);
      }
      const result = await runParityProbeSuite(reactPage, wgpuPage, suite);
      console.log(JSON.stringify(result, null, 2));
      console.log(`probe ${variant}/${suiteName}: ${result.status} (${result.steps.length} step(s))`);
      if (result.status !== "PASS") throw new Error(`parity probe ${variant}/${suiteName} FAILED`);
    } finally {
      await browser.close();
      if (reactServer) stopParityDevServer(reactServer);
      if (wgpuServer) stopParityDevServer(wgpuServer);
    }
  }
}

class ParityVerifyScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const variants = segments.filter((s) => !s.startsWith("--"));
    if (variants.length === 0) throw new Error("usage: parity verify <variant…>");
    const skipDev = process.env.SKIP_DEV === "1";
    const ports = skipDev ? parityPortsForShard(0) : findFreeParityPortPair();
    const reports: ParityPlaygroundReport[] = [];
    for (const variant of variants) {
      const report = await verifyParityVariant(variant, ports, { skipDev });
      reports.push(report);
      console.log(`${variant}: boot=${report.boot.react}/${report.boot.wgpu} structural=${report.structural?.status ?? "-"} pixel=${report.pixel?.status ?? "-"} behavioral=${report.behavioral?.status ?? "-"}`);
    }
    writeParityReport(reports);
    // 🪜️terra-parity-rebaseline: STALE-BRIDGE excluded — see `isParityStaleBridge`'s doc on `writeParityReport`.
    const staleBridge = reports.filter(isParityStaleBridge);
    const failed = reports.filter((r) => !isParityStaleBridge(r) && (r.boot.react !== "PASS" || r.boot.wgpu !== "PASS" || r.structural?.status === "FAIL" || r.pixel?.status === "FAIL" || r.behavioral?.status === "FAIL"));
    if (staleBridge.length > 0) console.log(`parity verify: ${staleBridge.length}/${reports.length} STALE-BRIDGE, excluded from the pass/fail verdict: ${staleBridge.map((r) => r.variant).join(", ")}`);
    if (failed.length > 0) throw new Error(`parity verify: ${failed.length}/${reports.length} playground(s) failed`);
  }
}

class ParitySweepScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const shardArg = segments.find((s) => s.startsWith("--shard="))?.slice("--shard=".length);
    const [shardIndex, shardCount] = shardArg ? shardArg.split("/").map(Number) : [0, 1];
    const variants = playgroundCatalog.map((r) => r.variant).filter((_, i) => i % (shardCount ?? 1) === (shardIndex ?? 0));
    const reports: ParityPlaygroundReport[] = [];
    for (const variant of variants) {
      let report: ParityPlaygroundReport;
      try {
        report = await verifyParityVariant(variant, parityPortsForShard(shardIndex ?? 0));
      } catch (error) {
        report = { variant, boot: { react: "SERVER-FAIL", wgpu: "SERVER-FAIL", detail: String(error) }, durationMs: 0 };
      }
      reports.push(report);
      console.log(`sweep ${variant}: boot=${report.boot.react}/${report.boot.wgpu} structural=${report.structural?.status ?? "-"} pixel=${report.pixel?.status ?? "-"} behavioral=${report.behavioral?.status ?? "-"}`);
    }
    writeParityReport(reports);
    // 🪜️terra-parity-rebaseline: STALE-BRIDGE excluded — see `isParityStaleBridge`'s doc on `writeParityReport`.
    const staleBridge = reports.filter(isParityStaleBridge);
    const failed = reports.filter((r) => !isParityStaleBridge(r) && (r.boot.react !== "PASS" || r.boot.wgpu !== "PASS" || r.structural?.status === "FAIL" || r.pixel?.status === "FAIL" || r.behavioral?.status === "FAIL"));
    const passed = reports.length - failed.length - staleBridge.length;
    console.log(`parity sweep complete: ${passed}/${reports.length} PASS · ${staleBridge.length}/${reports.length} STALE-BRIDGE · ${failed.length}/${reports.length} FAIL`);
    if (staleBridge.length > 0) console.log(`stale-bridge (excluded from verdict): ${staleBridge.map((r) => r.variant).join(", ")}`);
    if (failed.length > 0) throw new Error(`parity sweep: ${failed.length}/${reports.length} playground(s) failed`);
  }
}

export { PARITY_BOOT_TIMEOUT_MS, PARITY_STALE_BRIDGE_RE, ParityProbeScript, ParitySmokeScript, ParitySweepScript, ParityTriageScript, ParityVerifyScript, ensureParityPlaywrightBrowsersPath, triageParityBoot, verifyParityVariant };
