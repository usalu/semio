/** 🧩️ Semantic collaboration verification owner. */

import { repoCacheDirectory } from "../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/🟦️.ts";

import { constants as fsConstants, createReadStream, createWriteStream, copyFileSync, cpSync, existsSync, lstatSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, realpathSync, renameSync, rmSync, rmdirSync, statSync, unlinkSync, watch, writeFileSync } from "node:fs";

import { tmpdir } from "node:os";

import { basename, dirname, isAbsolute, join, relative, resolve } from "node:path";

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
} from "../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

import { generatePluginRegistry, type PluginRegistryEntry } from "../../../🔌️plugin/📇️registry/🔎️discovery/🟦️.ts";

import { DEFAULT_HOST_VARIANT } from "../../../🔌️plugin/📇️registry/🤖️generated/🎮️playgrounds/🟦️.ts";

import { PLUGIN_BUILD_TARGETS, PLUGIN_HOST_CONFIGS } from "../../../🔌️plugin/📇️registry/🤖️generated/🧩️plugins/🟦️.ts";

import { MODULE_BRIDGE_FILE, MODULE_SHARD_DIRECTORY, MODULE_HOT_SWAP_FILE, MODULE_PLUGIN_ROUTE, MODULE_EXTENSION_ROUTE, moduleDirectoryName, moduleIdForDirectoryName, moduleRoutePath } from "../../../🔌️plugin/📇️registry/📦️deployment/🟦️.ts";

const repoRoot = getWorkspaceRoot();

import { buildPlugin, preparePluginBuildTargets } from "../../../🔌️plugin/🏗️build/🏃️execution/🟦️.ts";

import { PLAYWRIGHT_MODULE_SPECIFIER, pluginOutRoot } from "../../../🔌️plugin/🏗️build/📋️plan/🟦️.ts";

import { ensurePluginRegistry } from "../../../🔌️plugin/📇️registry/🔄️refresh/🟦️.ts";

import { acquirePluginBuildLease, markPluginBuildLeaseReady, releasePluginBuildLease, waitForPluginBuildLeaseReady } from "../../♻️activation/🔐️lease/🟦️.ts";

import { awaitChildExit, awaitHttpOk, awaitTcpReady } from "../../♻️activation/🩺️readiness/🟦️.ts";

import { ensureAppleDeveloperDir } from "../../⚙️engine/📤️publication/🟦️.ts";

import { spaceE2eAssert } from "../🎬️studio/🟦️.ts";



//#endregion 🔖️CatalogSmokeVerify

//#region 🔖️CollabE2e
/** 🤝️ Two-user hub+shell end-to-end collaboration proof — ticket
 * `26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS`, lane 3-C. Boots the real hub plus two
 * independent `s` react dev servers (one per user) and drives them as two separate Playwright browser
 * contexts through the ticket's whole collaboration story: space creation replication, sharing, artifact
 * creation replication, live co-editing, presence, check-in, admin visibility, and hub-restart
 * persistence. Every scenario step is reported individually (`STEP n: PASS/FAIL`) and the run continues
 * past a failing step where it safely can — see the ticket's worker-brief "Reality check" section for
 * why several steps are expected to hit real, still-open upstream gaps this lane does not own. */
const COLLAB_E2E_PORT_MIN = 7400;

const COLLAB_E2E_PORT_MAX = 7498;

const COLLAB_E2E_HUB_BOOT_BUDGET_MS = Number(process.env.COLLAB_E2E_HUB_BOOT_BUDGET_MS ?? 300_000);

const COLLAB_E2E_PREBUILD_BUDGET_MS = Number(process.env.COLLAB_E2E_PREBUILD_BUDGET_MS ?? 1_800_000);

const COLLAB_E2E_DEV_BOOT_BUDGET_MS = Number(process.env.COLLAB_E2E_DEV_BOOT_BUDGET_MS ?? 300_000);

const COLLAB_E2E_ADMIN_TOKEN = "e2e-admin";

const COLLAB_E2E_USER1_EMAIL = "user1@semio.dev";

const COLLAB_E2E_USER2_EMAIL = "user2@semio.dev";

const COLLAB_E2E_STEP_NAMES = [
  "user1 creates a public studio space from Home; user2's Home shows the same row",
  "user1 shares the space with user2 as author; user2 opens /spaces/{id}",
  "user1 creates a writer artifact; the row appears in both tables and opens an editor for user1",
  "user2 opens the same artifact; user1 types and user2 sees the text",
  "#s-presence-peers shows 2 peers in both shells",
  "user1 checks in with a message; history shows it and the space table's updated column moves for both",
  "admin: /admin/api/connections lists both connections with their surfaces; /admin returns HTML",
  "hub restarts against the same OS_HUB_DATA; user2 reloads and the space + artifact are still there",
] as const;

type CollabStepOutcome = { readonly step: number; readonly name: string; readonly pass: boolean; readonly detail: string };

/** 📁️ Ticket folder — scratch logs/screenshots for this lane's own probes, per the worker-brief. */
function collabOutDir(): string {
  const dir = join(repoRoot, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS");
  mkdirSync(dir, { recursive: true });
  return dir;
}

/** 🔌️ Resolves one collab-e2e port: an explicit env override, else the first free port in
 * `[COLLAB_E2E_PORT_MIN, COLLAB_E2E_PORT_MAX]` not already claimed by an earlier call this run. */
function collabScanPort(envVar: string, taken: Set<number>): number {
  const override = process.env[envVar];
  if (override) {
    const port = Number(override);
    taken.add(port);
    return port;
  }
  for (let port = COLLAB_E2E_PORT_MIN; port <= COLLAB_E2E_PORT_MAX; port++) {
    if (taken.has(port) || isDevPortInUse("127.0.0.1", port)) continue;
    taken.add(port);
    return port;
  }
  throw new Error(`collab e2e: no free port in ${COLLAB_E2E_PORT_MIN}-${COLLAB_E2E_PORT_MAX} for ${envVar}`);
}

/** 🚀️ Spawns the real hub (`bun 🌎️hub/📦️packages/🦀️rust/📜️script.ts dev`, i.e. `cargo run` against the
 * default (sqlite) feature set — never `--all-features`, contract-freeze Amendment 2) on `port` against
 * a fresh `dataDir`, and waits for a real HTTP response before returning. */
async function collabStartHub(port: number, dataDir: string, logPath: string): Promise<SpawnDaemonHandle> {
  const hubScript = join(repoRoot, "./🌎️hub/📦️packages/🦀️rust/📜️script.ts");
  const logStream = createWriteStream(logPath);
  const daemon = spawnDaemon("bun", [hubScript, "dev"], {
    cwd: join(repoRoot, "./🌎️hub/📦️packages/🦀️rust"),
    env: { ...process.env, OS_HUB_PORT: String(port), OS_HUB_DATA: dataDir, OS_HUB_ADMIN_TOKEN: COLLAB_E2E_ADMIN_TOKEN },
    stdio: "pipe",
  });
  daemon.child.stdout?.pipe(logStream);
  daemon.child.stderr?.pipe(logStream);
  const baseUrl = `http://127.0.0.1:${port}`;
  const outcome = await awaitHttpOk(`${baseUrl}/admin/api/overview`, {
    deadlineMs: COLLAB_E2E_HUB_BOOT_BUDGET_MS,
    intervalMs: 500,
    init: { headers: { authorization: `Bearer ${COLLAB_E2E_ADMIN_TOKEN}` } },
    isDead: () => daemon.child.exitCode !== null,
  });
  if (outcome === "ready") return daemon;
  if (outcome === "dead") throw new Error(`hub exited early (code ${daemon.child.exitCode}) — see ${logPath}`);
  daemon.kill();
  logStream.end();
  throw new Error(`hub did not become ready on port ${port} within ${COLLAB_E2E_HUB_BOOT_BUDGET_MS}ms — see ${logPath}`);
}

/** 🎯️ The ONLY plugin crates this scenario touches: every host plugin id the generated catalog
 * declares (`🤖️generated/🧩️plugins/🟦️.ts`'s `PLUGIN_HOST_CONFIGS` — the `space` crate hosting Home and
 * Studio, never the `s` playground VARIANT that merely selects it) plus `"writer"`, the stdio-free
 * artifact kind this scenario creates (the brief's other suggestion, `"note"`, is a confirmed
 * pre-existing break — see `collabPrebuildPlugins`'s own doc comment). Building only these (not the
 * full ~58-crate catalog `buildPluginsStreaming(DEFAULT_HOST_VARIANT)` would otherwise attempt) turns a
 * 20-40 minute run into a sub-minute one and matches the coordinator's own guidance: build just what the
 * scenario needs, per-crate try/catch, then gate on the artifacts actually existing. */
const COLLAB_E2E_REQUIRED_PLUGIN_IDS: readonly string[] = [...PLUGIN_HOST_CONFIGS.map((entry) => entry.pluginId), "writer"];

/** 📁️ The exact `.core.wasm` path `buildPlugin` (this same file, `🔖️PluginSizeMeasurement` region's
 * neighbor) writes for `target` — mirrors its own `jsBase`/`componentBase` derivation so this check
 * looks for precisely what a successful build would have produced, not a guess. */
function collabPluginArtifactPath(target: PluginRegistryEntry): string {
  const jsBase = target.wasmOut.replace(/\.wasm$/, "");
  return join(pluginOutRoot, moduleDirectoryName(target.pluginId), `${jsBase}_component.core.wasm`);
}

/** 🧱️ Prepares only the plugin components consumed by the collaboration scenario. */
async function collabPrebuildPlugins(): Promise<void> {
  ensureAppleDeveloperDir();
  const lease = acquirePluginBuildLease(DEFAULT_HOST_VARIANT, 0);
  if (lease.role === "follower") {
    console.log(`[collab-e2e] plugin builds owned by pid ${lease.lease.pid}; waiting for ready`);
    await waitForPluginBuildLeaseReady(DEFAULT_HOST_VARIANT, COLLAB_E2E_PREBUILD_BUDGET_MS);
  } else {
    try {
      await ensurePluginRegistry(DEFAULT_HOST_VARIANT);
      const targets = await preparePluginBuildTargets(DEFAULT_HOST_VARIANT);
      const required = targets.filter((target) => COLLAB_E2E_REQUIRED_PLUGIN_IDS.includes(target.pluginId));
      const foundIds = new Set(required.map((target) => target.pluginId));
      for (const pluginId of COLLAB_E2E_REQUIRED_PLUGIN_IDS) {
        if (!foundIds.has(pluginId)) console.error(`[collab-e2e] WARNING: no registry entry for required plugin "${pluginId}" at all — catalog may have changed`);
      }
      for (const target of required) {
        try {
          await buildPlugin(target);
        } catch (error) {
          console.error(`[collab-e2e] required plugin build failed: ${target.pluginId}`, error);
        }
      }
      markPluginBuildLeaseReady(DEFAULT_HOST_VARIANT);
    } finally {
      releasePluginBuildLease(DEFAULT_HOST_VARIANT);
    }
  }
  const targets = await preparePluginBuildTargets(DEFAULT_HOST_VARIANT);
  const missing: string[] = [];
  for (const pluginId of COLLAB_E2E_REQUIRED_PLUGIN_IDS) {
    const target = targets.find((entry) => entry.pluginId === pluginId);
    if (!target || !existsSync(collabPluginArtifactPath(target))) missing.push(pluginId);
  }
  if (missing.length > 0) {
    throw new Error(`collab e2e: required plugin wasm artifact(s) missing after build: ${missing.join(", ")} — see the compiler error printed above (search "required plugin build failed: ${missing[0]}")`);
  }
}

/** ▶️ Spawns one user's `s` react dev server and waits for its port to accept connections. `dev` means
 * activate-then-serve, so the Nx activation chain reuses whatever `collabPrebuildPlugins` already
 * produced and only supplies the profiled browser modules and the receipt `ServeScript` demands. */
async function collabStartUserDevServer(opts: { readonly port: number; readonly hubUrl: string; readonly user: string; readonly dataDir: string; readonly logPath: string }): Promise<SpawnDaemonHandle> {
  const devScript = join(repoRoot, "./🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts");
  const logStream = createWriteStream(opts.logPath);
  const daemon = spawnDaemon("bun", [devScript, "dev"], {
    cwd: join(repoRoot, "./🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript"),
    env: { ...process.env, SEMIO_PLUGIN: "s", SEMIO_RENDERER: "react", S_OS_PORT: String(opts.port), S_HUB_URL: opts.hubUrl, S_USER: opts.user, S_DATA_DIR: opts.dataDir },
    stdio: "pipe",
  });
  daemon.child.stdout?.pipe(logStream);
  daemon.child.stderr?.pipe(logStream);
  const outcome = await awaitTcpReady("127.0.0.1", opts.port, {
    deadlineMs: COLLAB_E2E_DEV_BOOT_BUDGET_MS,
    intervalMs: 500,
    isDead: () => daemon.child.exitCode !== null,
  });
  if (outcome === "ready") return daemon;
  if (outcome === "dead") throw new Error(`dev server for ${opts.user} exited early (code ${daemon.child.exitCode}) — see ${opts.logPath}`);
  daemon.kill();
  logStream.end();
  throw new Error(`dev server for ${opts.user} did not open port ${opts.port} within ${COLLAB_E2E_DEV_BOOT_BUDGET_MS}ms — see ${opts.logPath}`);
}

//#region 🔖️CollabE2eDom
/** 🕹️ Clicks a shell-frozen toolbar-button id (contract §C0: `#s-home-create-space`,
 * `#s-space-create-artifact`) directly — lane 4-F wired these as real, always-present `UiNode::Button`
 * elements above their respective tables (dispatching with no args, which each command's own handler
 * treats as "open the dialog"), replacing the earlier command-palette hunt this harness used before
 * that landed: the palette's arg-carrying-command path opens the bottom-middle command PANEL form, not
 * a `[data-slot="dialog-box"]` modal, so it could never have satisfied `collabWaitForDialog` anyway. */
async function collabClickToolbarButton(page: import("playwright").Page, elementId: string): Promise<void> {
  const button = page.locator(`[id="${elementId}"]`);
  spaceE2eAssert((await button.count()) > 0, `toolbar button #${elementId} does not exist`);
  await button.click();
}

async function collabWaitForDialog(page: import("playwright").Page): Promise<void> {
  await page.locator('[data-slot="dialog-box"]').waitFor({ state: "visible", timeout: 15_000 });
}

async function collabSubmitDialog(page: import("playwright").Page): Promise<void> {
  await page.locator('[id="ui.dialog.submit"]').click();
  await page.locator('[data-slot="dialog-box"]').waitFor({ state: "hidden", timeout: 15_000 });
}

/** 🕹️ Opens a `<Select id={triggerId}>` (Radix, portal-rendered) and clicks the option with `optionText`. */
async function collabSelectOption(page: import("playwright").Page, triggerId: string, optionText: string): Promise<void> {
  await page.locator(`#${triggerId}`).click();
  await page.getByRole("option", { name: optionText, exact: true }).click();
  await page.waitForTimeout(150);
}

async function collabRowIds(page: import("playwright").Page, prefix: "space" | "artifact"): Promise<Set<string>> {
  const ids = await page.locator(`[data-row-id^="${prefix}:"]`).evaluateAll((elements) => elements.map((element) => element.getAttribute("data-row-id") ?? ""));
  return new Set(ids);
}

/** ⏳️ Polls `page` until a `data-row-id` with `prefix` appears that was not in `before`, returning the
 * bare id (prefix stripped). Used for both same-page ("the row appears") and cross-page ("user2 sees the
 * same row") assertions — the caller decides which page to poll. */
async function collabWaitForNewRow(page: import("playwright").Page, prefix: "space" | "artifact", before: ReadonlySet<string>, deadlineMs: number): Promise<string> {
  const deadline = Date.now() + deadlineMs;
  while (Date.now() < deadline) {
    const current = await collabRowIds(page, prefix);
    for (const id of current) {
      if (!before.has(id)) return id.slice(prefix.length + 1);
    }
    await page.waitForTimeout(500);
  }
  throw new Error(`timeout waiting for a new ${prefix}: row`);
}

async function collabWaitForRow(page: import("playwright").Page, prefix: "space" | "artifact", id: string, deadlineMs: number): Promise<void> {
  const deadline = Date.now() + deadlineMs;
  while (Date.now() < deadline) {
    if ((await page.locator(`[data-row-id="${prefix}:${id}"]`).count()) > 0) return;
    await page.waitForTimeout(500);
  }
  throw new Error(`timeout waiting for [data-row-id="${prefix}:${id}"]`);
}

async function collabScreenshot(page: import("playwright").Page, label: string): Promise<void> {
  try {
    await page.screenshot({ path: join(collabOutDir(), `🧪️3-c-${label}.png`) });
  } catch {
    // 🏁️ Best-effort — a screenshot failure must never mask the real assertion failure it was taken for.
  }
}

//#endregion 🔖️CollabE2eDom

/** 🎬️ The whole 8-step scenario, run against two already-booted `s` react dev servers and a live hub.
 * Each step is wrapped so a failure is recorded and the run continues to the next step wherever the
 * remaining steps can still be meaningfully attempted. */
async function collabRunScenario(
  user1: import("playwright").Page,
  user2: import("playwright").Page,
  hubBaseUrl: string,
): Promise<{ readonly results: CollabStepOutcome[]; readonly spaceId: string | undefined; readonly artifactId: string | undefined }> {
  const results: CollabStepOutcome[] = [];
  const record = (step: number, pass: boolean, detail: string): void => {
    results.push({ step, name: COLLAB_E2E_STEP_NAMES[step - 1]!, pass, detail });
    console.log(`STEP ${step}: ${pass ? "PASS" : "FAIL"}: ${COLLAB_E2E_STEP_NAMES[step - 1]} — ${detail}`);
  };

  let spaceId: string | undefined;
  let artifactId: string | undefined;

  // STEP 1
  try {
    const beforeUser1 = await collabRowIds(user1, "space");
    const spaceName = `Collab Studio ${Date.now()}`;
    await collabClickToolbarButton(user1, "s-home-create-space");
    await collabWaitForDialog(user1);
    await user1.locator("#name").fill(spaceName);
    await collabSelectOption(user1, "kind", "Studio");
    await collabSelectOption(user1, "visibility", "Public");
    await collabSubmitDialog(user1);
    spaceId = await collabWaitForNewRow(user1, "space", beforeUser1, 30_000);
    await collabWaitForRow(user2, "space", spaceId, 60_000);
    record(1, true, `space ${spaceId} created and replicated to user2's Home within budget`);
  } catch (error) {
    await collabScreenshot(user1, "step1-user1");
    await collabScreenshot(user2, "step1-user2");
    record(1, false, error instanceof Error ? error.message : String(error));
  }

  // STEP 2
  if (spaceId) {
    try {
      const row = user1.locator(`[data-row-id="space:${spaceId}"]`);
      await row.getByTitle(/share/i).click();
      await collabWaitForDialog(user1);
      await user1.locator("#email").fill(COLLAB_E2E_USER2_EMAIL);
      await collabSelectOption(user1, "role", "Author");
      await collabSubmitDialog(user1);
      await user2.goto(`${new URL(user2.url()).origin}/spaces/${spaceId}`, { waitUntil: "domcontentloaded" });
      await user2.locator(".semio-table-host").first().waitFor({ state: "visible", timeout: 30_000 });
      record(2, true, `user2 opened /spaces/${spaceId} and the Space app's artifact table rendered`);
    } catch (error) {
      await collabScreenshot(user1, "step2-user1");
      await collabScreenshot(user2, "step2-user2");
      record(2, false, error instanceof Error ? error.message : String(error));
    }
  } else {
    record(2, false, "skipped — no space id from STEP 1");
  }

  // STEP 3
  if (spaceId) {
    try {
      await user1.goto(`${new URL(user1.url()).origin}/spaces/${spaceId}`, { waitUntil: "domcontentloaded" });
      await user1.locator(".semio-table-host").first().waitFor({ state: "visible", timeout: 30_000 });
      const beforeUser1 = await collabRowIds(user1, "artifact");
      await collabClickToolbarButton(user1, "s-space-create-artifact");
      await collabWaitForDialog(user1);
      await user1.locator("#name").fill("Collab Writer");
      await collabSelectOption(user1, "kindId", "Writer");
      await collabSubmitDialog(user1);
      artifactId = await collabWaitForNewRow(user1, "artifact", beforeUser1, 30_000);
      await collabWaitForRow(user2, "artifact", artifactId, 30_000);
      const editorOpened = (await user1.locator('textarea, [contenteditable="true"]').count()) > 0;
      spaceE2eAssert(
        editorOpened,
        "no editable text surface appeared for user1 after createArtifact — inspect the direct Space opening effect, exact Shell session and scope, Hub open-plan/catalog verification, socket Session and retained document UI; the Space relay already includes documentId, spaceId and schema",
      );
      record(3, true, `artifact ${artifactId} created, row replicated to user2, editor surface present for user1`);
    } catch (error) {
      await collabScreenshot(user1, "step3-user1");
      await collabScreenshot(user2, "step3-user2");
      record(3, false, error instanceof Error ? error.message : String(error));
    }
  } else {
    record(3, false, "skipped — no space id from STEP 1");
  }

  // STEP 4
  if (spaceId && artifactId) {
    try {
      const row2 = user2.locator(`[data-row-id="artifact:${artifactId}"]`);
      await row2.getByTitle(/open/i).click();
      await user2.waitForTimeout(1_000);
      const editor1 = user1.locator('textarea, [contenteditable="true"]').first();
      const editor2 = user2.locator('textarea, [contenteditable="true"]').first();
      spaceE2eAssert((await editor1.count()) > 0, "user1 has no editable text surface open (see STEP 3)");
      spaceE2eAssert((await editor2.count()) > 0, "user2 has no editable text surface open after clicking the artifact row's open button");
      const probeText = `collab-probe-${Date.now()}`;
      await editor1.click();
      await editor1.type(probeText);
      const deadline = Date.now() + 30_000;
      let seen = "";
      while (Date.now() < deadline) {
        seen = (await editor2.inputValue().catch(() => editor2.innerText().catch(() => ""))) ?? "";
        if (seen.includes(probeText)) break;
        await user2.waitForTimeout(500);
      }
      spaceE2eAssert(
        seen.includes(probeText),
        `user2's editor never showed user1's typed text ${JSON.stringify(probeText)} (last seen: ${JSON.stringify(seen.slice(-200))}) — both editors are likely unbound ephemeral instances rather than the same hub-synced document (same root cause as STEP 3)`,
      );
      record(4, true, "user1's typed text propagated to user2's editor");
    } catch (error) {
      await collabScreenshot(user1, "step4-user1");
      await collabScreenshot(user2, "step4-user2");
      record(4, false, error instanceof Error ? error.message : String(error));
    }
  } else {
    record(4, false, "skipped — no artifact id from STEP 3");
  }

  // STEP 5
  try {
    const peers1 = user1.locator('[id="s-presence-peers"]');
    const peers2 = user2.locator('[id="s-presence-peers"]');
    spaceE2eAssert(
      (await peers1.count()) > 0,
      "#s-presence-peers does not exist in the React shell (🧰️framework/…/renderer/…/ShellHost/🟦️.tsx never imports or renders PresenceBar — confirmed by grep; lane 2-D wired presence only into the wgpu Shell, 🧊️component.rs, which per the ticket brief does not compile this wave)",
    );
    spaceE2eAssert((await peers2.count()) > 0, "#s-presence-peers does not exist in user2's shell either");
    const roster1 = await peers1.locator('[data-row-id^="peer:"]').count();
    const roster2 = await peers2.locator('[data-row-id^="peer:"]').count();
    spaceE2eAssert(roster1 === 2, `user1's presence roster has ${roster1} peer(s), expected 2`);
    spaceE2eAssert(roster2 === 2, `user2's presence roster has ${roster2} peer(s), expected 2`);
    record(5, true, "both shells show a 2-peer presence roster");
  } catch (error) {
    await collabScreenshot(user1, "step5-user1");
    await collabScreenshot(user2, "step5-user2");
    record(5, false, error instanceof Error ? error.message : String(error));
  }

  // STEP 6
  if (spaceId && artifactId) {
    try {
      await user1.goto(`${new URL(user1.url()).origin}/spaces/${spaceId}`, { waitUntil: "domcontentloaded" });
      await collabWaitForRow(user1, "artifact", artifactId, 30_000);
      const rowBefore1 =
        (await user1
          .locator(`[data-row-id="artifact:${artifactId}"]`)
          .innerText()
          .catch(() => "")) ?? "";
      const rowBefore2 =
        (await user2
          .locator(`[data-row-id="artifact:${artifactId}"]`)
          .innerText()
          .catch(() => "")) ?? "";
      const historyTab = user1.locator('[data-tab-id="framework.panel.history"]');
      spaceE2eAssert((await historyTab.count()) > 0, "no framework.panel.history tab found — cannot reach #s-checkin");
      await historyTab.click();
      const checkinButton = user1.locator('[id="s-checkin"]');
      await checkinButton.waitFor({ state: "visible", timeout: 10_000 });
      await checkinButton.click();
      const message = `collab check-in ${Date.now()}`;
      await user1.locator('[id="s-checkin-message"]').fill(message);
      const historyEntryVisible = user1.getByText(message, { exact: false });
      await user1.locator('[id="s-checkin-message"]').press("Enter");
      await historyEntryVisible.first().waitFor({ state: "visible", timeout: 15_000 });
      await user1.goto(`${new URL(user1.url()).origin}/spaces/${spaceId}`, { waitUntil: "domcontentloaded" });
      await collabWaitForRow(user1, "artifact", artifactId, 30_000);
      const rowAfter1Deadline = Date.now() + 30_000;
      let rowAfter1 = rowBefore1;
      while (Date.now() < rowAfter1Deadline) {
        rowAfter1 =
          (await user1
            .locator(`[data-row-id="artifact:${artifactId}"]`)
            .innerText()
            .catch(() => "")) ?? "";
        if (rowAfter1 !== rowBefore1) break;
        await user1.waitForTimeout(1_000);
      }
      spaceE2eAssert(rowAfter1 !== rowBefore1, `user1's space table row for ${artifactId} did not change after check-in (before: ${JSON.stringify(rowBefore1)}, after: ${JSON.stringify(rowAfter1)})`);
      await user2.goto(`${new URL(user2.url()).origin}/spaces/${spaceId}`, { waitUntil: "domcontentloaded" });
      await collabWaitForRow(user2, "artifact", artifactId, 30_000);
      const rowAfter2Deadline = Date.now() + 30_000;
      let rowAfter2 = rowBefore2;
      while (Date.now() < rowAfter2Deadline) {
        rowAfter2 =
          (await user2
            .locator(`[data-row-id="artifact:${artifactId}"]`)
            .innerText()
            .catch(() => "")) ?? "";
        if (rowAfter2 !== rowBefore2) break;
        await user2.waitForTimeout(1_000);
      }
      spaceE2eAssert(rowAfter2 !== rowBefore2, `user2's space table row for ${artifactId} did not change after user1's check-in (before: ${JSON.stringify(rowBefore2)}, after: ${JSON.stringify(rowAfter2)})`);
      record(6, true, "check-in dispatched and the space table's row changed for both users");
    } catch (error) {
      await collabScreenshot(user1, "step6-user1");
      await collabScreenshot(user2, "step6-user2");
      record(6, false, error instanceof Error ? error.message : String(error));
    }
  } else {
    record(6, false, "skipped — no space/artifact id from earlier steps");
  }

  // STEP 7
  try {
    const connectionsRes = await fetch(`${hubBaseUrl}/admin/api/connections`, { headers: { authorization: `Bearer ${COLLAB_E2E_ADMIN_TOKEN}` } });
    spaceE2eAssert(connectionsRes.ok, `GET /admin/api/connections returned ${connectionsRes.status}`);
    const connections = (await connectionsRes.json()) as readonly Record<string, unknown>[];
    const text = JSON.stringify(connections);
    spaceE2eAssert(text.includes(COLLAB_E2E_USER1_EMAIL) || text.includes("user1"), `/admin/api/connections does not mention user1: ${text.slice(0, 500)}`);
    spaceE2eAssert(text.includes(COLLAB_E2E_USER2_EMAIL) || text.includes("user2"), `/admin/api/connections does not mention user2: ${text.slice(0, 500)}`);
    const adminRes = await fetch(`${hubBaseUrl}/admin`, { headers: { authorization: `Bearer ${COLLAB_E2E_ADMIN_TOKEN}` } });
    spaceE2eAssert(adminRes.ok, `GET /admin returned ${adminRes.status}`);
    const contentType = adminRes.headers.get("content-type") ?? "";
    spaceE2eAssert(contentType.includes("html"), `GET /admin content-type is ${contentType}, expected html`);
    record(
      7,
      true,
      "/admin/api/connections names both users; /admin returns HTML — note: /admin is a client-rendered SPA shell, so the raw HTML byte stream itself does not literally embed the user names (verified via /admin/api/connections instead)",
    );
  } catch (error) {
    record(7, false, error instanceof Error ? error.message : String(error));
  }

  return { results, spaceId, artifactId };
}

/** 🔁️ STEP 8 — restarts the hub against the SAME `dataDir` and the SAME port, then reloads `user2` and
 * confirms the space + artifact rows survive. Deliberately the SAME port (not a fresh one, unlike lane
 * 3-E's Node-only harness): the browser's `S_HUB_URL` is baked into its bundle at Vite `define`-time
 * (contract §C0), so only a same-port restart lets a plain page reload — not a dev-server restart —
 * reconnect; the temp `dataDir` is what actually proves persistence here, matching the brief's own
 * "restart the hub against the same `OS_HUB_DATA`" wording literally. Runs at the orchestration level
 * (not inside `collabRunScenario`) since it needs the hub daemon handle, not just a base URL. */
async function collabRunRestartStep(opts: {
  readonly record: (step: number, pass: boolean, detail: string) => void;
  readonly hubDaemon: SpawnDaemonHandle;
  readonly hubPort: number;
  readonly hubDataDir: string;
  readonly user2: import("playwright").Page;
  readonly spaceId: string | undefined;
  readonly artifactId: string | undefined;
}): Promise<SpawnDaemonHandle> {
  if (!opts.spaceId || !opts.artifactId) {
    opts.record(8, false, "skipped — no space/artifact id from earlier steps");
    return opts.hubDaemon;
  }
  try {
    opts.hubDaemon.kill();
    // 🧵️ We hold the hub's own `child` handle — await its `exit` event via 🔖️PollHelpers's
    // `awaitChildExit` instead of polling `exitCode` (THE RULE above). Same 30s budget as before.
    const exited = await awaitChildExit(opts.hubDaemon.child, 30_000);
    spaceE2eAssert(exited === "exited", "hub process did not exit within 30s of being killed");
    const portFreed = await awaitTcpReady("127.0.0.1", opts.hubPort, { deadlineMs: 30_000, intervalMs: 250, mode: "closed" });
    spaceE2eAssert(portFreed === "ready", `port ${opts.hubPort} never freed up after the hub exited`);
    const newHubDaemon = await collabStartHub(opts.hubPort, opts.hubDataDir, join(collabOutDir(), "🧪️3-c-hub-restart.txt"));
    await opts.user2.reload({ waitUntil: "domcontentloaded" });
    await opts.user2.goto(`${new URL(opts.user2.url()).origin}/spaces/${opts.spaceId}`, { waitUntil: "domcontentloaded" });
    await collabWaitForRow(opts.user2, "artifact", opts.artifactId, 60_000);
    opts.record(8, true, `hub restarted against the same OS_HUB_DATA (${opts.hubDataDir}) on the same port; user2 still sees space ${opts.spaceId} and artifact ${opts.artifactId} after reload`);
    return newHubDaemon;
  } catch (error) {
    await collabScreenshot(opts.user2, "step8-user2");
    opts.record(8, false, error instanceof Error ? error.message : String(error));
    return opts.hubDaemon;
  }
}

/** 🎬️ Orchestrates the full harness: port scan, temp data dirs, hub boot, plugin prebuild, two `s`
 * react dev servers, two independent Playwright browser contexts, the 8-step scenario, and teardown of
 * every spawned process (hub + both dev servers + browser) even on failure. Writes `STEP n: PASS/FAIL`
 * lines plus a final summary, and sets a non-zero exit code if any step failed. */
async function runCollabE2eVerify(): Promise<void> {
  const outDir = collabOutDir();
  const taken = new Set<number>();
  const hubPort = collabScanPort("S_COLLAB_HUB_PORT", taken);
  const user1Port = collabScanPort("S_COLLAB_USER1_PORT", taken);
  const user2Port = collabScanPort("S_COLLAB_USER2_PORT", taken);
  console.log(`[collab-e2e] ports: hub=${hubPort} user1=${user1Port} user2=${user2Port}`);

  const hubDataDir = mkdtempSync(join(tmpdir(), "semio-collab-hub-"));
  const user1DataDir = mkdtempSync(join(tmpdir(), "semio-collab-u1-"));
  const user2DataDir = mkdtempSync(join(tmpdir(), "semio-collab-u2-"));

  let hubDaemon: SpawnDaemonHandle | undefined;
  let user1Daemon: SpawnDaemonHandle | undefined;
  let user2Daemon: SpawnDaemonHandle | undefined;
  let browser: import("playwright").Browser | undefined;
  const results: CollabStepOutcome[] = [];
  const record = (step: number, pass: boolean, detail: string): void => {
    results.push({ step, name: COLLAB_E2E_STEP_NAMES[step - 1]!, pass, detail });
    console.log(`STEP ${step}: ${pass ? "PASS" : "FAIL"}: ${COLLAB_E2E_STEP_NAMES[step - 1]} — ${detail}`);
  };

  const teardown = async (): Promise<void> => {
    try {
      await browser?.close();
    } catch {
      // 🏁️ Best-effort.
    }
    for (const daemon of [user1Daemon, user2Daemon, hubDaemon]) {
      try {
        daemon?.kill();
      } catch {
        // 🏁️ Best-effort — a teardown failure must never mask the real run's outcome.
      }
    }
  };

  try {
    try {
      hubDaemon = await collabStartHub(hubPort, hubDataDir, join(outDir, "🧪️3-c-hub-boot.txt"));
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      console.error(`[collab-e2e] hub failed to boot — every scenario step is reported FAIL: ${message}`);
      for (let step = 1; step <= 8; step++) record(step, false, `blocked — hub never became ready: ${message}`);
      throw error;
    }

    try {
      await collabPrebuildPlugins();
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      console.error(`[collab-e2e] plugin prebuild failed — every scenario step is reported FAIL: ${message}`);
      for (let step = 1; step <= 8; step++) record(step, false, `blocked — plugin prebuild failed: ${message}`);
      throw error;
    }

    const hubBaseUrl = `http://127.0.0.1:${hubPort}`;
    try {
      [user1Daemon, user2Daemon] = await Promise.all([
        collabStartUserDevServer({ port: user1Port, hubUrl: hubBaseUrl, user: COLLAB_E2E_USER1_EMAIL, dataDir: user1DataDir, logPath: join(outDir, "🧪️3-c-user1-dev.txt") }),
        collabStartUserDevServer({ port: user2Port, hubUrl: hubBaseUrl, user: COLLAB_E2E_USER2_EMAIL, dataDir: user2DataDir, logPath: join(outDir, "🧪️3-c-user2-dev.txt") }),
      ]);
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      console.error(`[collab-e2e] a shell dev server never booted — every scenario step is reported FAIL: ${message}`);
      for (let step = 1; step <= 8; step++) record(step, false, `blocked — shells did not boot: ${message}`);
      throw error;
    }

    // 🎭️ Matches `SetupScript`'s own install location (the shared cache root's `tools/ms-playwright`) —
    // without this, Playwright falls back to the OS default cache (`~/Library/Caches/ms-playwright`),
    // which can hold a different/older browser revision than the one this repo's `playwright` version
    // expects (confirmed during this lane's own iteration: the default cache had `chromium-1223`, this
    // repo's `playwright` wanted `chromium_headless_shell-1234`, which only exists under the repo-scoped path).
    process.env.PLAYWRIGHT_BROWSERS_PATH = process.env.PLAYWRIGHT_BROWSERS_PATH ?? repoCacheDirectory(repoRoot, "tools", "ms-playwright");
    const { chromium } = await import(PLAYWRIGHT_MODULE_SPECIFIER);
    browser = await chromium.launch({ headless: true });
    const context1 = await browser.newContext();
    const context2 = await browser.newContext();
    const user1Page = await context1.newPage();
    const user2Page = await context2.newPage();
    const pageErrors: string[] = [];
    /** 🔬️ Full browser-side visibility for whichever page this is attached to: every `console.*` level
     * (not just uncaught exceptions), failed HTTP requests, and the lifecycle + received frames of every
     * WebSocket the page opens (the `/directory/socket/v1` subscription in particular) — printed immediately so
     * they interleave chronologically with the harness's own `STEP n:` lines in the run log, instead of
     * being buffered and dumped out of order at the end. Added per the ticket's w4-h diagnosis: the prior
     * harness only captured `pageerror`, so a silently-caught `console.warn`/`console.error` inside
     * `ShellHost` was invisible even though it was the only place the real failing branch could show up. */
    const attachBrowserDiagnostics = (page: import("playwright").Page, label: string): void => {
      page.on("pageerror", (err) => pageErrors.push(`${label}: ${String(err)}`));
      page.on("console", (msg) => console.log(`[collab-e2e:console] ${label} [${msg.type()}] ${msg.text()}`));
      page.on("requestfailed", (request) => console.log(`[collab-e2e:network] ${label} requestfailed: ${request.method()} ${request.url()} — ${request.failure()?.errorText ?? "unknown"}`));
      page.on("response", (response) => {
        const url = response.url();
        if (!url.includes("/auth/") && !url.includes("/directory/")) return;
        const status = response.status();
        const auth = response.request().headers()["authorization"] ?? "none";
        const postData = response.request().postData() ?? "";
        console.log(`[collab-e2e:network] ${label} response: ${response.request().method()} ${url} auth=${auth} body=${postData.slice(0, 300)} — ${status}`);
      });
      page.on("websocket", (ws) => {
        console.log(`[collab-e2e:ws] ${label} opened: ${ws.url()}`);
        ws.on("framereceived", (frame) => console.log(`[collab-e2e:ws] ${label} recv: ${(typeof frame.payload === "string" ? frame.payload : "<binary>").slice(0, 800)}`));
        ws.on("close", () => console.log(`[collab-e2e:ws] ${label} closed: ${ws.url()}`));
        ws.on("socketerror", (error) => console.log(`[collab-e2e:ws] ${label} socketerror: ${ws.url()} — ${error}`));
      });
    };
    attachBrowserDiagnostics(user1Page, "user1");
    attachBrowserDiagnostics(user2Page, "user2");

    await user1Page.goto(`http://127.0.0.1:${user1Port}/`, { waitUntil: "domcontentloaded", timeout: 120_000 });
    await user2Page.goto(`http://127.0.0.1:${user2Port}/`, { waitUntil: "domcontentloaded", timeout: 120_000 });
    await user1Page.locator(".semio-table-host").first().waitFor({ state: "visible", timeout: 120_000 });
    await user2Page.locator(".semio-table-host").first().waitFor({ state: "visible", timeout: 120_000 });
    await user1Page.waitForTimeout(2_000);
    await user2Page.waitForTimeout(2_000);

    const scenario = await collabRunScenario(user1Page, user2Page, hubBaseUrl);
    for (const outcome of scenario.results) results.push(outcome);

    hubDaemon = await collabRunRestartStep({ record, hubDaemon: hubDaemon!, hubPort, hubDataDir, user2: user2Page, spaceId: scenario.spaceId, artifactId: scenario.artifactId });

    const ignorableGpuFragments = ["NoCompatibleDevice"];
    const criticalErrors = pageErrors.filter((message) => !ignorableGpuFragments.some((fragment) => message.includes(fragment)));
    if (criticalErrors.length > 0) console.warn(`[collab-e2e] page errors observed (not a step on their own, informational): ${criticalErrors.join(" | ")}`);
  } finally {
    await teardown();
  }

  const passed = results.filter((outcome) => outcome.pass).length;
  console.log(`[collab-e2e] summary: ${passed}/${results.length} steps passed`);
  for (const outcome of results) console.log(`  STEP ${outcome.step}: ${outcome.pass ? "PASS" : "FAIL"}: ${outcome.name}`);
  if (passed !== results.length) process.exitCode = 1;
}

export { COLLAB_E2E_ADMIN_TOKEN, COLLAB_E2E_DEV_BOOT_BUDGET_MS, COLLAB_E2E_HUB_BOOT_BUDGET_MS, COLLAB_E2E_PORT_MAX, COLLAB_E2E_PORT_MIN, COLLAB_E2E_PREBUILD_BUDGET_MS, COLLAB_E2E_REQUIRED_PLUGIN_IDS, COLLAB_E2E_STEP_NAMES, COLLAB_E2E_USER1_EMAIL, COLLAB_E2E_USER2_EMAIL, CollabStepOutcome, collabClickToolbarButton, collabOutDir, collabPluginArtifactPath, collabPrebuildPlugins, collabRowIds, collabRunRestartStep, collabRunScenario, collabScanPort, collabScreenshot, collabSelectOption, collabStartHub, collabStartUserDevServer, collabSubmitDialog, collabWaitForDialog, collabWaitForNewRow, collabWaitForRow, runCollabE2eVerify };
