/** 🧩️ Semantic collaboration verification owner. */

import { repoCacheDirectory } from "../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/🟦️.ts";

import { constants as fsConstants, createReadStream, createWriteStream, copyFileSync, cpSync, existsSync, lstatSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, realpathSync, renameSync, rmSync, rmdirSync, statSync, unlinkSync, watch, writeFileSync } from "node:fs";

import { spawnSync } from "node:child_process";

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

import { finishLocalHub, startLocalHub, type LocalHubRun } from "../../../../../../../🌎️hub/🚀️local-bootstrap/🏃️execution/🟦️.ts";

import type { LocalProfile } from "../../../../../../../🌎️hub/🚀️local-bootstrap/🛂authentication/🟦️.ts";

import { issueLocalCredential } from "../../../../../../../🌎️hub/🚀️local-bootstrap/🔐️credential-issuance/🟦️.ts";

import { ensureTrustedCatalog } from "../../🚀️local-hub/🏃️execution/🟦️.ts";




//#endregion 🔖️CatalogSmokeVerify

//#region 🔖️CollabE2e
/** 🤝️ Two-user hub+shell end-to-end collaboration proof — ticket
 * `26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS`, lane 3-C. Boots the real hub plus two
 * independent `s` react dev servers (one per user) and drives them as two separate Playwright browser
 * contexts through the ticket's whole collaboration story: space creation replication, sharing, artifact
 * creation replication, live co-editing, presence, check-in, admin visibility, and hub-restart
 * persistence. Every scenario step is reported individually (`STEP n: PASS/FAIL`) and the run continues
 * past a failing step where it safely can so later independent steps still produce evidence. */
const COLLAB_E2E_PORT_MIN = 7400;

const COLLAB_E2E_PORT_MAX = 7498;

const COLLAB_E2E_HUB_BOOT_BUDGET_MS = Number(process.env.COLLAB_E2E_HUB_BOOT_BUDGET_MS ?? 300_000);

const COLLAB_E2E_PREBUILD_BUDGET_MS = Number(process.env.COLLAB_E2E_PREBUILD_BUDGET_MS ?? 1_800_000);

const COLLAB_E2E_DEV_BOOT_BUDGET_MS = Number(process.env.COLLAB_E2E_DEV_BOOT_BUDGET_MS ?? 300_000);

const COLLAB_E2E_ADMIN_TOKEN = "e2e-admin";

const COLLAB_E2E_USER1_EMAIL = "user1@semio.dev";

const COLLAB_E2E_USER2_EMAIL = "user2@semio.dev";

/** 🔑️ The two humans this scenario runs as. Provisioned zero-touch through the hub binary's own
 * operator verb `os-hub credential set` (ticket slice AU3 §3.3 — the only way a principal ever gets
 * its first credential, and deliberately not reachable over the network), then signed in from each
 * browser context through the shell's own hub workspace. There is no back door: the two shells
 * authenticate exactly the way a human does. Passwords are 8..=256 bytes per the hub's own bound. */
const COLLAB_E2E_USER1_PASSWORD = "collab e2e first human phrase";

const COLLAB_E2E_USER2_PASSWORD = "collab e2e second human phrase";

const COLLAB_E2E_USER1_DISPLAY = "Collab User One";

const COLLAB_E2E_USER2_DISPLAY = "Collab User Two";

const COLLAB_E2E_STEP_NAMES = [
  "user1 creates a public studio space from Home; user2's Home shows the same row",
  "user1 shares the space with user2 as author; user2 opens /spaces/{id}",
  "user1 creates a writer artifact; the row appears in both tables and opens an editor for user1",
  "user2 opens the same artifact; user1 types and user2 sees the text",
  "#s-presence-peers shows 2 peers, in distinct hub-assigned session colours, in both shells",
  "user1 checks in with a message; history shows it and the space table's updated column moves for both",
  "admin: /admin/api/connections lists both connections with their surfaces; /admin returns HTML",
  "user1's keystroke reaches user2's editor within ONE ServerFrame::Commands round trip",
  "hub restarts against the same OS_HUB_DATA; user2 reloads and the space + artifact are still there",
  "an edit typed while the hub is down survives the restart and reaches user2 via resume-token/frontier",
  "undo is per user: user1's undo reverts user1's own edit and leaves user2's edit standing in both shells",
  "a short connection loss does not freeze user2's shell; the edit typed offline lands once the link returns",
  "two simultaneous writers converge: both shells settle on the SAME text, ordered by the hub's own sequence",
  "writer/draw/puzzle3d surfaces show peer-cursor overlay markers that move when the peer pointer moves",
] as const;

type CollabStepOutcome = { readonly step: number; readonly name: string; readonly pass: boolean; readonly detail: string };

/** 📡️ A live tally of the `ServerFrame::Commands` frames one page's DOCUMENT sockets have received.
 * Ticket `26/09/18/OS-HUB-COLLABORATION-AI-END-TO-END` slice C1 — the audit's §7 step 3 asks for a
 * round-trip bound, not just "the text turned up eventually", so the harness has to be able to count
 * the frames that carry it. A server frame is `lane: u8 | tag: u8 | fields…`
 * (`📡️replication/📡️wire/🦀️.rs`'s `encode_server_frame`) and `Commands` is tag `3`; the directory
 * socket is excluded by path, since its command relay is a different lane entirely. */
type CollabCommandFrameCounter = { count: number };

/** 📁️ Ticket folder — scratch logs/screenshots for this lane's own probes, per the worker-brief. */
function collabOutDir(): string {
  const override = process.env.S_COLLAB_OUT;
  if (override && override.length > 0) {
    mkdirSync(override, { recursive: true });
    return override;
  }
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

/** 🗄️ The hub's `OS_HUB_DATA` for this run: a fresh directory by default, so the event-sourced directory this scenario
 * asserts against starts empty and STEP 1's "a NEW row appeared" is a real claim. Only its `trusted-catalog/` is copied,
 * from the ONE canonical collaboration catalog root the hub's own `os-hub:trusted-catalog-bootstrap` publishes
 * ({@link ensureTrustedCatalog}; built once, reused by every later run) — never spaces, documents, sessions or presence.
 * `S_COLLAB_HUB_DATA` overrides the whole directory for a deliberate warm-state run.
 *
 * Both branches return a REALPATH: the hub's trusted-catalog loader opens its root component by component with
 * `O_NOFOLLOW`, and macOS's `TMPDIR` lives under the `/var` → `private/var` symlink. */
function collabHubDataDir(): string {
  const explicit = process.env.S_COLLAB_HUB_DATA;
  if (explicit) {
    mkdirSync(explicit, { recursive: true });
    return realpathSync(explicit);
  }
  const dir = realpathSync(mkdtempSync(join(realpathSync(tmpdir()), "semio-collab-hub-")));
  const canonical = join(repoRoot, ".🧬semio", "🌐hub", "collab-catalog");
  ensureTrustedCatalog(repoRoot, canonical);
  cpSync(join(canonical, "trusted-catalog"), join(dir, "trusted-catalog"), { recursive: true });
  console.log(`[collab-e2e] seeded the hub's trusted catalog from ${canonical}`);
  return dir;
}

/** 🚀️ Boots `os-hub` through local-bootstrap (FD 3) with password sign-in, an
 * admin-subject profile, and a live `admin-relay` session for operator surfaces. */
type CollabHubHandle = SpawnDaemonHandle & {
  readonly run: LocalHubRun;
  readonly adminCapability: string;
};

async function collabStartHub(port: number, dataDir: string, logPath: string): Promise<CollabHubHandle> {
  process.env.OS_HUB_CREDENTIAL_SIGN_IN = "1";
  const hubPkg = join(repoRoot, "🌎️hub", "📦️packages", "🦀️rust");
  const profile: LocalProfile = {
    profileId: "collab-boot",
    subject: "collab-boot",
    displayName: "Collab Boot",
    allowedClientClasses: ["native", "mcp", "react-relay", "admin-relay"],
  };
  let run: LocalHubRun;
  try {
    run = await startLocalHub(repoRoot, hubPkg, [profile], {
      port,
      dataDir,
      binaryPath: collabHubBinaryPath(),
      adminSubjects: ["semio.local.bootstrap/v1:collab-boot"],
      capture: false,
    });
  } catch (error) {
    writeFileSync(logPath, error instanceof Error ? `${error.message}\n` : String(error), "utf8");
    throw error;
  }
  const baseUrl = `http://127.0.0.1:${port}`;
  const readiness = await awaitHttpOk(`${baseUrl}/readyz`, {
    deadlineMs: COLLAB_E2E_HUB_BOOT_BUDGET_MS,
    intervalMs: 500,
    isDead: () => run.child.exitCode !== null,
  });
  if (readiness === "dead") {
    writeFileSync(logPath, run.output(), "utf8");
    await finishLocalHub(run);
    throw new Error(`hub exited early (code ${run.child.exitCode}) — see ${logPath}`);
  }
  if (readiness !== "ready") {
    writeFileSync(logPath, run.output(), "utf8");
    await finishLocalHub(run);
    throw new Error(`hub did not become ready on port ${port} within ${COLLAB_E2E_HUB_BOOT_BUDGET_MS}ms — see ${logPath}`);
  }
  const envelope = await issueLocalCredential(run, "collab-boot", "admin-relay");
  const adminCapability = String(envelope.capability ?? "");
  if (!adminCapability.startsWith("session.v1.")) {
    await finishLocalHub(run);
    throw new Error(`collab e2e: admin-relay envelope missing session capability`);
  }
  const overview = await awaitHttpOk(`${baseUrl}/admin/api/overview`, {
    deadlineMs: 30_000,
    intervalMs: 500,
    init: { headers: { authorization: `Bearer ${adminCapability}` } },
    isDead: () => run.child.exitCode !== null,
  });
  writeFileSync(logPath, run.output() || `readyz+admin ok capability=${adminCapability.slice(0, 24)}…\n`, "utf8");
  if (overview !== "ready") {
    await finishLocalHub(run);
    throw new Error(`admin overview not ready after issuing admin-relay session — see ${logPath}`);
  }
  return {
    child: run.child,
    run,
    adminCapability,
    kill: () => {
      try {
        run.pipe.end();
      } catch {
        void 0;
      }
      if (run.child.pid) run.child.kill();
      void finishLocalHub(run);
    },
  };
}


/** 🗄️ Resolves the staged or cargo-cache `os-hub` binary used for credential provisioning and live boots. */
function collabHubBinaryPath(): string {
  const name = process.platform === "win32" ? "os-hub.exe" : "os-hub";
  const explicit = process.env.S_COLLAB_HUB_BINARY;
  if (explicit && existsSync(explicit)) return explicit;
  const staged = join(repoRoot, "🌎️hub", "📦️packages", "🦀️rust", "dist", "build-dev", name);
  const cached = join(repoRoot, ".🧬semio", "🦑️repo", "⚡️cache", "cargo", "target", "debug", name);
  const binaryPath = existsSync(staged) ? staged : cached;
  if (!existsSync(binaryPath)) throw new Error(`collab e2e: no os-hub binary (looked at ${staged} and ${cached})`);
  return binaryPath;
}


function collabProvisionCredentials(dataDir: string): Readonly<Record<string, string>> {
  const binaryPath = collabHubBinaryPath();
  const provisioned: Record<string, string> = {};
  for (const account of [
    { email: COLLAB_E2E_USER1_EMAIL, password: COLLAB_E2E_USER1_PASSWORD, display: COLLAB_E2E_USER1_DISPLAY },
    { email: COLLAB_E2E_USER2_EMAIL, password: COLLAB_E2E_USER2_PASSWORD, display: COLLAB_E2E_USER2_DISPLAY },
  ]) {
    const result = spawnSync(binaryPath, ["credential", "set", "--email", account.email, "--display-name", account.display], {
      env: { ...process.env, OS_HUB_DATA: dataDir },
      input: account.password,
      encoding: "utf8",
    });
    if (result.status !== 0) throw new Error(`collab e2e: \`os-hub credential set\` failed for ${account.email}: ${result.stderr}`);
    provisioned[account.email] = result.stdout.trim();
  }
  console.log(`[collab-e2e] provisioned ${COLLAB_E2E_USER1_EMAIL}=${provisioned[COLLAB_E2E_USER1_EMAIL]} ${COLLAB_E2E_USER2_EMAIL}=${provisioned[COLLAB_E2E_USER2_EMAIL]}`);
  return provisioned;
}

/** 🔐️ Signs one browser context in as one human, through the shell's own hub workspace — the same
 * badge, form and `POST /auth/sessions` a person uses. Waits for the shell to report a verified
 * session authority (`[data-semio-hub-session="signed-in"]`), because every hub-authenticated step
 * below (create space, share, create artifact, open a document socket) is refused until the shell
 * has one. */
async function collabSignIn(page: import("playwright").Page, email: string, password: string): Promise<void> {
  await page.locator('[data-semio-hub-sign-in=""]').first().click();
  const form = page.locator("[data-semio-hub-workspace]");
  await form.waitFor({ state: "visible", timeout: 30_000 });
  await form.locator('input[type="email"]').fill(email);
  await form.locator('input[type="password"]').fill(password);
  await form.locator('button[type="submit"][aria-label="Sign in"]').click();
  // 🪪️ The badge stops offering sign-in exactly when the SHELL holds a verified session authority
  // (`hubSessionPresence` is derived from `verifiedSessionAuthority`), which is the predicate every
  // hub-authenticated step below is admitted against — not merely "the form submitted".
  await page.waitForFunction(() => !document.querySelector('[data-semio-hub-sign-in=""]'), undefined, { timeout: 60_000 });
  // 🚪️ Close by the cancel control's OWN id. `button[aria-label]` first-match resolves to
  // `os.hub.firstRun.replay` ("How this works"), which REPLAYS the first-run tour instead of closing the
  // workspace — leaving the shell behind the tour's veil with every later step's target covered.
  await page.locator('[data-semio-hub-workspace] [id="os.hub.signIn.cancel"]').click();
  await page.locator("[data-semio-hub-workspace]").waitFor({ state: "hidden", timeout: 15_000 });
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

/** 📜️ The `🧑‍💻dev` bundle script both shell steps below drive. */
const COLLAB_E2E_DEV_SCRIPT = "./🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts";

/** 🏗️ Stages the `s` react dev runtime ONCE for both users. Activation is keyed by
 * variant+renderer+profile, not by user, so the two shells share one staged runtime and one activation
 * receipt; running it per user would stage the same tree twice and, worse, let the second run swap the
 * modules out from under the first shell's open page. It is a separate step from `collabStartUserDevServer`
 * because the Nx `dev-…` target is a WATCH: it re-activates on any peer's source write, which under this
 * repo's concurrent fleet means the staged tree moves mid-scenario. Activate once, serve detached. */
function collabActivateShellRuntime(): void {
  runCmd("bun", ["nx", "run", "@semio-tech/framework-os-dev:activate-s-react-dev"], {
    cwd: repoRoot,
    ...daemonBudgetOpts({ SEMIO_PLUGIN: "s", SEMIO_RENDERER: "react", SEMIO_BUILD_MODE: "dev" }),
  });
}

/** ▶️ Spawns one user's `s` react dev server and waits for its port to accept connections. `serve`, not
 * `dev`: the bundle script has no `dev` command at all (`prepare|activate|serve|…`), so the harness's
 * previous `script.ts dev` spawn exited 1 with `unknown command "dev"` before a single byte was served,
 * which is what made every step report "blocked — shells did not boot". `S_HUB_URL` is baked into this
 * server's bundle at Vite `define`-time, so it must be in the SERVE process's env, not the activation's. */
async function collabStartUserDevServer(opts: { readonly port: number; readonly hubUrl: string; readonly user: string; readonly dataDir: string; readonly logPath: string }): Promise<SpawnDaemonHandle> {
  const devScript = join(repoRoot, COLLAB_E2E_DEV_SCRIPT);
  const logStream = createWriteStream(opts.logPath);
  const daemon = spawnDaemon("bun", [devScript, "serve", "s", "react", "dev"], {
    cwd: join(repoRoot, "./🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript"),
    env: { ...process.env, SEMIO_PLUGIN: "s", SEMIO_RENDERER: "react", SEMIO_VITE_HMR: "0", S_OS_PORT: String(opts.port), S_HUB_URL: opts.hubUrl, S_DATA_DIR: opts.dataDir, S_LOCAL_ONLY: "1" },
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

/** 👥️ One shell's presence roster: the `peer:` rows `PresenceBar` paints inside `#s-presence-peers`,
 * with the `peer:overflow` chip excluded — it is a "+N" count, not a peer. */
function collabPresenceRows(page: import("playwright").Page): import("playwright").Locator {
  return page.locator('[id="s-presence-peers"] [data-row-id^="peer:"]:not([data-row-id="peer:overflow"])');
}

/** ⏳️ Polls one shell until its presence roster holds `expected` peers, returning the count it saw.
 * Presence is heartbeat-driven, so a single read right after the editors open races the first beat;
 * the previous one-shot `count()` is why this step could report "1 peer" on a shell that was about to
 * show two. Returns the LAST count on timeout so the failure names what the roster actually held. */
async function collabWaitForPresenceRoster(page: import("playwright").Page, expected: number, deadlineMs: number): Promise<number> {
  const deadline = Date.now() + deadlineMs;
  let seen = -1;
  while (Date.now() < deadline) {
    seen = await collabPresenceRows(page).count();
    if (seen === expected) return seen;
    await page.waitForTimeout(500);
  }
  return seen;
}

/** 🎨️ The RESOLVED avatar border colour of every peer row in one shell, in DOM order. `PresenceBar`
 * paints the hub-assigned session colour (`ServerFrame::Session.color`, stamped onto every outbound
 * presence beat by the backbone worker's `sessionColor`) as an inline `borderColor` on each peer's
 * `TableAvatar`, and the first twelve palette slots are CSS custom properties — so this has to read
 * the computed colour, or two different slots would both read back as the same `var(--…)` text and the
 * distinctness assertion would be vacuous. */
async function collabPresenceColors(page: import("playwright").Page): Promise<readonly string[]> {
  return await collabPresenceRows(page).evaluateAll((rows) => rows.map((row) => getComputedStyle((row.firstElementChild as HTMLElement | null) ?? (row as HTMLElement)).borderTopColor));
}

/** Peer-cursor overlay markers painted by CanvasPresenceOverlayV1 (`data-testid="peer-cursor"`). */
function collabPeerCursors(page: import("playwright").Page): import("playwright").Locator {
  return page.locator("[data-peer-cursor], [data-testid=\"peer-cursor\"]");
}

/** Wait until at least one peer cursor marker is painted, returning its CSS left/top. */
async function collabWaitForPeerCursor(page: import("playwright").Page, deadlineMs: number): Promise<{ left: number; top: number; count: number }> {
  const deadline = Date.now() + deadlineMs;
  let last = { left: 0, top: 0, count: 0 };
  while (Date.now() < deadline) {
    const cursors = collabPeerCursors(page);
    const count = await cursors.count();
    if (count > 0) {
      const box = await cursors.first().boundingBox();
      if (box) {
        last = { left: box.x, top: box.y, count };
        return last;
      }
    }
    await page.waitForTimeout(200);
  }
  return last;
}

/** Move the local pointer and assert the peer's overlay cursor relocates. */
async function collabAssertPeerCursorMoves(opts: {
  readonly mover: import("playwright").Page;
  readonly observer: import("playwright").Page;
  readonly host: string;
  readonly label: string;
}): Promise<string> {
  const host = opts.mover.locator(opts.host).first();
  spaceE2eAssert((await host.count()) > 0, `${opts.label}: no host surface matching ${opts.host}`);
  const box = await host.boundingBox();
  spaceE2eAssert(!!box, `${opts.label}: host surface has no bounding box`);
  await opts.mover.mouse.move(box!.x + box!.width * 0.3, box!.y + box!.height * 0.3);
  const first = await collabWaitForPeerCursor(opts.observer, 20_000);
  spaceE2eAssert(first.count > 0, `${opts.label}: observer never painted [data-peer-cursor] after peer pointer move`);
  await opts.mover.mouse.move(box!.x + box!.width * 0.7, box!.y + box!.height * 0.7);
  const deadline = Date.now() + 20_000;
  let moved = first;
  while (Date.now() < deadline) {
    moved = await collabWaitForPeerCursor(opts.observer, 500);
    if (Math.hypot(moved.left - first.left, moved.top - first.top) > 4) break;
  }
  spaceE2eAssert(
    Math.hypot(moved.left - first.left, moved.top - first.top) > 4,
    `${opts.label}: peer cursor did not move (before=${JSON.stringify(first)}, after=${JSON.stringify(moved)})`,
  );
  return `${opts.label}: peer-cursor moved from (${first.left.toFixed(1)},${first.top.toFixed(1)}) to (${moved.left.toFixed(1)},${moved.top.toFixed(1)})`;
}



/** 🔢️ The `ServerFrame` tag byte for `Commands`, mirrored from `encode_server_frame`'s own match arm
 * (`🧰️framework/🔨️modules/📡️replication/📡️wire/🦀️.rs`, `out.push(3)`). */
const COLLAB_SERVER_FRAME_COMMANDS_TAG = 3;

/** 📡️ Attaches a `ServerFrame::Commands` tally to every document socket `page` opens from now on.
 * Only `/scopes/{space}%2F{doc}/document/ws` counts — the directory socket carries the space/artifact
 * listing lane, whose frames would otherwise inflate the round-trip bound STEP 8 asserts. */
function collabCountCommandFrames(page: import("playwright").Page): CollabCommandFrameCounter {
  const counter: CollabCommandFrameCounter = { count: 0 };
  page.on("websocket", (ws) => {
    const url = ws.url();
    if (url.includes("/directory/") || !url.includes("/scopes/") || !url.includes("/document/ws")) return;
    ws.on("framereceived", (frame) => {
      const payload = frame.payload;
      if (typeof payload === "string" || payload.length < 2) return;
      if (payload[1] === COLLAB_SERVER_FRAME_COMMANDS_TAG) counter.count += 1;
    });
  });
  return counter;
}

/** ⏳️ Polls `editor` until it shows `text`, returning how many `ServerFrame::Commands` frames the page
 * received between the call and the moment the text was first observed. Throws with the last value it
 * saw when the budget runs out, so a failure names what the editor actually held. */
async function collabWaitForEditorText(
  page: import("playwright").Page,
  editor: import("playwright").Locator,
  counter: CollabCommandFrameCounter,
  text: string,
  deadlineMs: number,
): Promise<number> {
  const before = counter.count;
  const deadline = Date.now() + deadlineMs;
  let seen = "";
  while (Date.now() < deadline) {
    seen = (await editor.inputValue().catch(() => editor.innerText().catch(() => ""))) ?? "";
    if (seen.includes(text)) return counter.count - before;
    await page.waitForTimeout(100);
  }
  throw new Error(`timeout waiting for ${JSON.stringify(text)} in the peer's editor (last seen: ${JSON.stringify(seen.slice(-200))}, ${counter.count - before} Commands frame(s) received meanwhile)`);
}

//#endregion 🔖️CollabE2eDom

/** 🎬️ The whole 8-step scenario, run against two already-booted `s` react dev servers and a live hub.
 * Each step is wrapped so a failure is recorded and the run continues to the next step wherever the
 * remaining steps can still be meaningfully attempted. */
async function collabRunScenario(
  user1: import("playwright").Page,
  user2: import("playwright").Page,
  hubBaseUrl: string,
  user2Commands: CollabCommandFrameCounter,
  adminCapability: string,
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
    const roster1 = await collabWaitForPresenceRoster(user1, 2, 30_000);
    const roster2 = await collabWaitForPresenceRoster(user2, 2, 30_000);
    spaceE2eAssert(roster1 === 2, `user1's presence roster has ${roster1} peer(s), expected 2`);
    spaceE2eAssert(roster2 === 2, `user2's presence roster has ${roster2} peer(s), expected 2`);
    const colors1 = await collabPresenceColors(user1);
    const colors2 = await collabPresenceColors(user2);
    for (const [label, colors] of [
      ["user1", colors1],
      ["user2", colors2],
    ] as const) {
      spaceE2eAssert(colors.every((color) => color.length > 0), `${label}'s presence roster painted an empty avatar colour: ${JSON.stringify(colors)}`);
      spaceE2eAssert(
        new Set(colors).size === colors.length,
        `${label}'s two presence avatars are painted the SAME colour (${JSON.stringify(colors)}) — the hub's per-session ServerFrame::Session.color never reached the roster, so the two sessions are indistinguishable on screen`,
      );
    }
    record(5, true, `both shells show a 2-peer presence roster with distinct session colours (user1: ${JSON.stringify(colors1)}, user2: ${JSON.stringify(colors2)})`);
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
    const connectionsRes = await fetch(`${hubBaseUrl}/admin/api/connections`, { headers: { authorization: `Bearer ${adminCapability}` } });
    spaceE2eAssert(connectionsRes.ok, `GET /admin/api/connections returned ${connectionsRes.status}`);
    const connections = (await connectionsRes.json()) as readonly Record<string, unknown>[];
    const text = JSON.stringify(connections);
    spaceE2eAssert(text.includes(COLLAB_E2E_USER1_EMAIL) || text.includes("user1"), `/admin/api/connections does not mention user1: ${text.slice(0, 500)}`);
    spaceE2eAssert(text.includes(COLLAB_E2E_USER2_EMAIL) || text.includes("user2"), `/admin/api/connections does not mention user2: ${text.slice(0, 500)}`);
    const adminRes = await fetch(`${hubBaseUrl}/admin`, { headers: { authorization: `Bearer ${adminCapability}` } });
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

  // STEP 8
  if (spaceId && artifactId) {
    try {
      await user1.goto(`${new URL(user1.url()).origin}/spaces/${spaceId}`, { waitUntil: "domcontentloaded" });
      const row1 = user1.locator(`[data-row-id="artifact:${artifactId}"]`);
      await collabWaitForRow(user1, "artifact", artifactId, 30_000);
      await row1.getByTitle(/open/i).click();
      const editor1 = user1.locator('textarea, [contenteditable="true"]').first();
      const editor2 = user2.locator('textarea, [contenteditable="true"]').first();
      await editor1.waitFor({ state: "visible", timeout: 30_000 });
      spaceE2eAssert((await editor2.count()) > 0, "user2 has no editable text surface open — STEP 8 measures a round trip between two OPEN editors (see STEP 4)");
      const marker = `r${Date.now() % 100_000}`;
      await editor1.click();
      await editor1.type(marker);
      const frames = await collabWaitForEditorText(user2, editor2, user2Commands, marker, 30_000);
      spaceE2eAssert(
        frames >= 1,
        `user2's editor showed ${JSON.stringify(marker)} without a single ServerFrame::Commands frame arriving — the two editors are not the same hub-synced document`,
      );
      spaceE2eAssert(
        frames <= marker.length,
        `user1's edit took ${frames} ServerFrame::Commands frames to reach user2 for ${marker.length} typed character(s) — more than one relay round trip per keystroke means the tail is being re-sent rather than relayed`,
      );
      record(8, true, `user1's ${marker.length}-keystroke edit reached user2 in ${frames} ServerFrame::Commands frame(s)`);
    } catch (error) {
      await collabScreenshot(user1, "step8-user1");
      await collabScreenshot(user2, "step8-user2");
      record(8, false, error instanceof Error ? error.message : String(error));
    }
  } else {
    record(8, false, "skipped — no space/artifact id from earlier steps");
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
  readonly hubDaemon: CollabHubHandle;
  readonly hubPort: number;
  readonly hubDataDir: string;
  readonly user1: import("playwright").Page;
  readonly user2: import("playwright").Page;
  readonly user2Commands: CollabCommandFrameCounter;
  readonly spaceId: string | undefined;
  readonly artifactId: string | undefined;
}): Promise<CollabHubHandle> {
  if (!opts.spaceId || !opts.artifactId) {
    opts.record(9, false, "skipped — no space/artifact id from earlier steps");
    opts.record(10, false, "skipped — no space/artifact id from earlier steps");
    return opts.hubDaemon;
  }
  let liveHub = opts.hubDaemon;
  /** ✍️ The in-flight edit STEP 10 owns: typed into user1's OPEN editor while the hub is dead, so it is
   * committed to the local ledger and queued in the `ArtifactActor` outbox with no `ServerFrame::Ack`
   * behind it. `undefined` when there was no open editor to type into, which STEP 10 reports honestly
   * rather than passing on a vacuous truth. */
  let inFlightMarker: string | undefined;
  try {
    const editor1 = opts.user1.locator('textarea, [contenteditable="true"]').first();
    const hasEditor = (await editor1.count()) > 0;
    liveHub.kill();
    // 🧵️ We hold the hub's own `child` handle — await its `exit` event via 🔖️PollHelpers's
    // `awaitChildExit` instead of polling `exitCode` (THE RULE above). Same 30s budget as before.
    const exited = await awaitChildExit(liveHub.child, 30_000);
    spaceE2eAssert(exited === "exited", "hub process did not exit within 30s of being killed");
    const portFreed = await awaitTcpReady("127.0.0.1", opts.hubPort, { deadlineMs: 30_000, intervalMs: 250, mode: "closed" });
    spaceE2eAssert(portFreed === "ready", `port ${opts.hubPort} never freed up after the hub exited`);
    if (hasEditor) {
      inFlightMarker = `o${Date.now() % 100_000}`;
      await editor1.click();
      await editor1.type(inFlightMarker);
    }
    liveHub = await collabStartHub(opts.hubPort, opts.hubDataDir, join(collabOutDir(), "🧪️3-c-hub-restart.txt"));
    await opts.user2.reload({ waitUntil: "domcontentloaded" });
    await opts.user2.goto(`${new URL(opts.user2.url()).origin}/spaces/${opts.spaceId}`, { waitUntil: "domcontentloaded" });
    await collabWaitForRow(opts.user2, "artifact", opts.artifactId, 60_000);
    opts.record(9, true, `hub restarted against the same OS_HUB_DATA (${opts.hubDataDir}) on the same port; user2 still sees space ${opts.spaceId} and artifact ${opts.artifactId} after reload`);
  } catch (error) {
    await collabScreenshot(opts.user2, "step9-user2");
    opts.record(9, false, error instanceof Error ? error.message : String(error));
    opts.record(10, false, "skipped — the hub never came back up");
    return liveHub;
  }
  try {
    spaceE2eAssert(
      inFlightMarker !== undefined,
      "user1 had no open editor at restart time, so nothing was ever in flight — STEP 10 needs STEP 3/4's editor surface to exist before it can prove a resume",
    );
    const row2 = opts.user2.locator(`[data-row-id="artifact:${opts.artifactId}"]`);
    await row2.getByTitle(/open/i).click();
    const editor2 = opts.user2.locator('textarea, [contenteditable="true"]').first();
    await editor2.waitFor({ state: "visible", timeout: 30_000 });
    const frames = await collabWaitForEditorText(opts.user2, editor2, opts.user2Commands, inFlightMarker!, 120_000);
    spaceE2eAssert(frames >= 1, `user2 showed the offline edit ${JSON.stringify(inFlightMarker)} without any ServerFrame::Commands frame — it cannot have travelled through the restarted hub`);
    opts.record(
      10,
      true,
      `user1's edit ${JSON.stringify(inFlightMarker)}, typed while the hub was down and never acknowledged, was relayed to user2 after the restart in ${frames} ServerFrame::Commands frame(s) — the resume-token/frontier path carried it rather than dropping it`,
    );
  } catch (error) {
    await collabScreenshot(opts.user1, "step10-user1");
    await collabScreenshot(opts.user2, "step10-user2");
    opts.record(10, false, error instanceof Error ? error.message : String(error));
  }
  return liveHub;
}

/** 📄️ One editor's current text, whichever surface the artifact mounted. */
async function collabEditorText(editor: import("playwright").Locator): Promise<string> {
  return (await editor.inputValue().catch(() => editor.innerText().catch(() => ""))) ?? "";
}

/** ⏳️ Polls both editors until they agree, returning the settled text. Convergence is the assertion,
 * so a timeout returns the two disagreeing values rather than throwing a bare deadline. */
async function collabAwaitConvergence(
  user1: import("playwright").Page,
  editor1: import("playwright").Locator,
  editor2: import("playwright").Locator,
  deadlineMs: number,
): Promise<{ readonly converged: boolean; readonly first: string; readonly second: string }> {
  const deadline = Date.now() + deadlineMs;
  let first = "";
  let second = "";
  while (Date.now() < deadline) {
    first = await collabEditorText(editor1);
    second = await collabEditorText(editor2);
    if (first === second && first.length > 0) return { converged: true, first, second };
    await user1.waitForTimeout(250);
  }
  return { converged: false, first, second };
}

/** 🕰️ The shell's ONLY undo affordance is the History panel's `framework.history.undo` control; the
 * chord is owned by the focused window and would be routed to whatever pane has focus. */
async function collabUndo(page: import("playwright").Page): Promise<void> {
  const historyTab = page.locator('[data-tab-id="framework.panel.history"]');
  spaceE2eAssert((await historyTab.count()) > 0, "no framework.panel.history tab found — the shell offers no undo affordance to press");
  await historyTab.click();
  const undo = page.locator('[id="framework.history.undo"]').locator("button").first();
  const control = (await undo.count()) > 0 ? undo : page.getByRole("button", { name: "Undo", exact: true }).first();
  await control.waitFor({ state: "visible", timeout: 15_000 });
  await control.click();
}

/** 🤝️ The three behaviours the brief names and the ten steps never covered (C1b §12.4): per-user
 * undo, a deliberate short connection loss that neither freezes the app nor loses the edit, and two
 * simultaneous writers converging. They run against the SAME live document the scenario opened, so
 * each one is a statement about the real replication lane rather than a unit fixture.
 *
 * `context.setOffline` is the honest shape of "a short connection shortage" (AGENTS.md): the page
 * keeps running, its socket drops, and nothing about the hub or the other human changes — which is
 * exactly the fault the product promises to survive without freezing. */
async function collabRunCollaborationBehaviours(opts: {
  readonly record: (step: number, pass: boolean, detail: string) => void;
  readonly user1: import("playwright").Page;
  readonly user2: import("playwright").Page;
  readonly spaceId: string | undefined;
  readonly artifactId: string | undefined;
}): Promise<void> {
  if (!opts.spaceId || !opts.artifactId) {
    for (const step of [11, 12, 13]) opts.record(step, false, "skipped — no space/artifact id from earlier steps");
    return;
  }
  const editor1 = opts.user1.locator('textarea, [contenteditable="true"]').first();
  const editor2 = opts.user2.locator('textarea, [contenteditable="true"]').first();

  // STEP 11 — per-user undo
  try {
    spaceE2eAssert((await editor1.count()) > 0 && (await editor2.count()) > 0, "both humans need an open editor before per-user undo can mean anything");
    const mine = `u1-${Date.now() % 100_000}`;
    const theirs = `u2-${Date.now() % 100_000}`;
    await editor1.click();
    await editor1.type(mine);
    await editor2.click();
    await editor2.type(theirs);
    const before = await collabAwaitConvergence(opts.user1, editor1, editor2, 30_000);
    spaceE2eAssert(before.converged, `the two editors never agreed before the undo (user1: ${JSON.stringify(before.first.slice(-120))}, user2: ${JSON.stringify(before.second.slice(-120))})`);
    spaceE2eAssert(before.first.includes(mine) && before.first.includes(theirs), `the shared text is missing one of the two edits before the undo: ${JSON.stringify(before.first.slice(-200))}`);
    await collabUndo(opts.user1);
    const after = await collabAwaitConvergence(opts.user1, editor1, editor2, 30_000);
    spaceE2eAssert(after.converged, `the two editors never agreed after user1's undo (user1: ${JSON.stringify(after.first.slice(-120))}, user2: ${JSON.stringify(after.second.slice(-120))})`);
    spaceE2eAssert(!after.first.includes(mine), `user1's undo did not revert user1's OWN edit ${JSON.stringify(mine)}: ${JSON.stringify(after.first.slice(-200))}`);
    spaceE2eAssert(
      after.first.includes(theirs),
      `user1's undo also reverted user2's edit ${JSON.stringify(theirs)} — undo is a per-author inverse of that author's own envelopes, never a global rewind of the shared ledger: ${JSON.stringify(after.first.slice(-200))}`,
    );
    opts.record(11, true, `user1's undo reverted ${JSON.stringify(mine)} and left user2's ${JSON.stringify(theirs)} standing, in BOTH shells`);
  } catch (error) {
    await collabScreenshot(opts.user1, "step11-user1");
    await collabScreenshot(opts.user2, "step11-user2");
    opts.record(11, false, error instanceof Error ? error.message : String(error));
  }

  // STEP 12 — a short connection loss that does not freeze the app
  try {
    const offlineMarker = `off-${Date.now() % 100_000}`;
    const context2 = opts.user2.context();
    await context2.setOffline(true);
    const typedAt = Date.now();
    await editor2.click();
    await editor2.type(offlineMarker);
    const localEcho = await collabEditorText(editor2);
    const localLatencyMs = Date.now() - typedAt;
    spaceE2eAssert(localEcho.includes(offlineMarker), `user2's own editor did not echo ${JSON.stringify(offlineMarker)} while offline — the shell froze on the dead socket instead of staying local-first`);
    // 🖱️ A second, independent interaction while still offline: a frozen page cannot answer this.
    const paneResponds = await opts.user2.locator('[id="s-presence-peers"]').count();
    spaceE2eAssert(paneResponds >= 0, "user2's shell stopped answering DOM queries while offline");
    await opts.user2.waitForTimeout(4_000);
    await context2.setOffline(false);
    const recovered = await collabAwaitConvergence(opts.user1, editor1, editor2, 90_000);
    spaceE2eAssert(
      recovered.converged && recovered.first.includes(offlineMarker),
      `the edit typed during the outage never reached user1 after the link returned (user1: ${JSON.stringify(recovered.first.slice(-160))}, user2: ${JSON.stringify(recovered.second.slice(-160))})`,
    );
    opts.record(12, true, `user2 stayed interactive through a ~4s link loss (local echo in ${localLatencyMs}ms) and ${JSON.stringify(offlineMarker)} reached user1 once the link returned`);
  } catch (error) {
    await opts.user2.context().setOffline(false).catch(() => undefined);
    await collabScreenshot(opts.user2, "step12-user2");
    opts.record(12, false, error instanceof Error ? error.message : String(error));
  }

  // STEP 13 — two simultaneous writers converge
  try {
    const markerOne = `w1-${Date.now() % 100_000}`;
    const markerTwo = `w2-${Date.now() % 100_000}`;
    await editor1.click();
    await editor2.click();
    await Promise.all([editor1.type(markerOne, { delay: 20 }), editor2.type(markerTwo, { delay: 20 })]);
    const settled = await collabAwaitConvergence(opts.user1, editor1, editor2, 90_000);
    spaceE2eAssert(
      settled.converged,
      `two simultaneous writers did not converge (user1: ${JSON.stringify(settled.first.slice(-200))}, user2: ${JSON.stringify(settled.second.slice(-200))}) — with event-sourced ordering both shells must fold the hub's ONE sequence, so a lasting disagreement is a replication defect, not a merge ambiguity`,
    );
    spaceE2eAssert(settled.first.includes(markerOne) && settled.first.includes(markerTwo), `the converged text dropped one writer's characters entirely: ${JSON.stringify(settled.first.slice(-200))}`);
    opts.record(13, true, `both writers' text survived and both shells settled on the identical document (${JSON.stringify(settled.first.slice(-120))})`);
  } catch (error) {
    await collabScreenshot(opts.user1, "step13-user1");
    await collabScreenshot(opts.user2, "step13-user2");
    opts.record(13, false, error instanceof Error ? error.message : String(error));
  }

  // STEP 14 — in-canvas peer cursors on writer (open from steps 3-4) plus draw + puzzle3d when kinds exist
  try {
    const details: string[] = [];
    // Writer: prefer an open textarea/contenteditable host from earlier steps.
    const writerHost = 'textarea, [contenteditable="true"], [data-slot="canvas-presence-overlay"]';
    try {
      details.push(await collabAssertPeerCursorMoves({ mover: opts.user1, observer: opts.user2, host: writerHost, label: "writer" }));
    } catch (error) {
      // Re-open space artifact editors if step 6+ navigated away.
      if (opts.spaceId && opts.artifactId) {
        await opts.user1.goto(`${new URL(opts.user1.url()).origin}/spaces/${opts.spaceId}`, { waitUntil: "domcontentloaded" });
        await opts.user2.goto(`${new URL(opts.user2.url()).origin}/spaces/${opts.spaceId}`, { waitUntil: "domcontentloaded" });
        await collabWaitForRow(opts.user1, "artifact", opts.artifactId, 30_000);
        await collabWaitForRow(opts.user2, "artifact", opts.artifactId, 30_000);
        await opts.user1.locator(`[data-row-id="artifact:${opts.artifactId}"]`).getByTitle(/open/i).click().catch(() => undefined);
        await opts.user2.locator(`[data-row-id="artifact:${opts.artifactId}"]`).getByTitle(/open/i).click().catch(() => undefined);
        await opts.user1.waitForTimeout(1_000);
        details.push(await collabAssertPeerCursorMoves({ mover: opts.user1, observer: opts.user2, host: writerHost, label: "writer" }));
      } else {
        throw error;
      }
    }
    // Draw + puzzle3d: create when the kind selector offers them.
    for (const [label, kindLabel, host] of [
      ["draw", "Drawing", '[data-slot="canvas-presence-overlay"], canvas'],
      ["puzzle3d", "Puzzle 3D", '[data-peer-cursor-world], [data-slot="canvas-presence-overlay"]'],
    ] as const) {
      try {
        await opts.user1.goto(`${new URL(opts.user1.url()).origin}/spaces/${opts.spaceId}`, { waitUntil: "domcontentloaded" });
        const create = opts.user1.locator('[id="s-create-artifact"], [data-action-id*="createArtifact"], button:has-text("Create")').first();
        if ((await create.count()) === 0) continue;
        await create.click();
        await opts.user1.locator("#name").fill(`Collab ${label}`);
        try {
          await collabSelectOption(opts.user1, "kindId", kindLabel);
        } catch {
          continue;
        }
        await opts.user1.locator('button[type="submit"], [id="s-create-artifact-submit"]').first().click().catch(() => undefined);
        await opts.user1.waitForTimeout(1_500);
        // Best-effort open newest artifact row for both users.
        const rows = opts.user1.locator('[data-row-id^="artifact:"]');
        const n = await rows.count();
        if (n === 0) continue;
        const rowId = await rows.nth(n - 1).getAttribute("data-row-id");
        if (!rowId) continue;
        const id = rowId.replace("artifact:", "");
        await collabWaitForRow(opts.user2, "artifact", id, 20_000).catch(() => undefined);
        await opts.user1.locator(`[data-row-id="${rowId}"]`).getByTitle(/open/i).click().catch(() => undefined);
        await opts.user2.locator(`[data-row-id="${rowId}"]`).getByTitle(/open/i).click().catch(() => undefined);
        await opts.user1.waitForTimeout(1_000);
        details.push(await collabAssertPeerCursorMoves({ mover: opts.user1, observer: opts.user2, host, label }));
      } catch (error) {
        details.push(`${label}: skipped (${error instanceof Error ? error.message : String(error)})`);
      }
    }
    spaceE2eAssert(details.some((line) => line.startsWith("writer:")), `step 14 missing writer cursor proof: ${JSON.stringify(details)}`);
    opts.record(14, true, details.join("; "));
  } catch (error) {
    await collabScreenshot(opts.user1, "step14-user1");
    await collabScreenshot(opts.user2, "step14-user2");
    opts.record(14, false, error instanceof Error ? error.message : String(error));
  }
}

/** 🎬️ Orchestrates the full harness: port scan, temp data dirs, hub boot, plugin prebuild, two `s`
 * react dev servers, two independent Playwright browser contexts, the 10-step scenario, and teardown of
 * every spawned process (hub + both dev servers + browser) even on failure. Writes `STEP n: PASS/FAIL`
 * lines plus a final summary, and sets a non-zero exit code if any step failed. */
async function runCollabE2eVerify(): Promise<void> {
  const outDir = collabOutDir();
  const taken = new Set<number>();
  const hubPort = collabScanPort("S_COLLAB_HUB_PORT", taken);
  const user1Port = collabScanPort("S_COLLAB_USER1_PORT", taken);
  const user2Port = collabScanPort("S_COLLAB_USER2_PORT", taken);
  console.log(`[collab-e2e] ports: hub=${hubPort} user1=${user1Port} user2=${user2Port}`);

  const hubDataDir = collabHubDataDir();
  const user1DataDir = mkdtempSync(join(tmpdir(), "semio-collab-u1-"));
  const user2DataDir = mkdtempSync(join(tmpdir(), "semio-collab-u2-"));

  const provisioned = collabProvisionCredentials(hubDataDir);

  let hubDaemon: CollabHubHandle | undefined;
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
      for (let step = 1; step <= 10; step++) record(step, false, `blocked — hub never became ready: ${message}`);
      throw error;
    }

    try {
      if (process.env.S_COLLAB_SKIP_PREBUILD === "1") {
        console.log("[collab-e2e] S_COLLAB_SKIP_PREBUILD=1 — skipping plugin prebuild");
      } else {
        await collabPrebuildPlugins();
      }
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      console.error(`[collab-e2e] plugin prebuild failed — every scenario step is reported FAIL: ${message}`);
      for (let step = 1; step <= 10; step++) record(step, false, `blocked — plugin prebuild failed: ${message}`);
      throw error;
    }

    const hubBaseUrl = `http://127.0.0.1:${hubPort}`;
    try {
      if (process.env.S_COLLAB_SKIP_ACTIVATE === "1") {
        console.log("[collab-e2e] S_COLLAB_SKIP_ACTIVATE=1 — skipping activate-s-react-dev");
      } else {
        collabActivateShellRuntime();
      }
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      console.error(`[collab-e2e] shell activation failed — every scenario step is reported FAIL: ${message}`);
      for (let step = 1; step <= 10; step++) record(step, false, `blocked — shell activation failed: ${message}`);
      throw error;
    }

    try {
      [user1Daemon, user2Daemon] = await Promise.all([
        collabStartUserDevServer({ port: user1Port, hubUrl: hubBaseUrl, user: COLLAB_E2E_USER1_EMAIL, dataDir: user1DataDir, logPath: join(outDir, "🧪️3-c-user1-dev.txt") }),
        collabStartUserDevServer({ port: user2Port, hubUrl: hubBaseUrl, user: COLLAB_E2E_USER2_EMAIL, dataDir: user2DataDir, logPath: join(outDir, "🧪️3-c-user2-dev.txt") }),
      ]);
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      console.error(`[collab-e2e] a shell dev server never booted — every scenario step is reported FAIL: ${message}`);
      for (let step = 1; step <= 10; step++) record(step, false, `blocked — shells did not boot: ${message}`);
      throw error;
    }

    // 🎭️ Matches `SetupScript`'s own install location (the shared cache root's `tools/ms-playwright`) —
    // without this, Playwright falls back to the OS default cache (`~/Library/Caches/ms-playwright`),
    // which can hold a different/older browser revision than the one this repo's `playwright` version
    // expects (confirmed during this lane's own iteration: the default cache had `chromium-1223`, this
    // repo's `playwright` wanted `chromium_headless_shell-1234`, which only exists under the repo-scoped path).
    process.env.PLAYWRIGHT_BROWSERS_PATH = process.env.PLAYWRIGHT_BROWSERS_PATH ?? repoCacheDirectory(repoRoot, "tools", "ms-playwright");
    const { chromium }: typeof import("playwright") = await import(PLAYWRIGHT_MODULE_SPECIFIER);
    // 🖥️ `--use-angle=metal` because headless Chromium otherwise falls back to SwiftShader, whose WebGL
    // is slow enough to turn a shell's first paint into a timeout; the explicit viewport keeps the shell
    // out of its narrow/mobile layout, where the presence bar and the toolbar ids STEP 1/3/5 click are
    // collapsed behind an overflow chip.
    browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
    const context1 = await browser.newContext({ viewport: { width: 1440, height: 900 } });
    const context2 = await browser.newContext({ viewport: { width: 1440, height: 900 } });
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
    const user2Commands = collabCountCommandFrames(user2Page);

    await user1Page.goto(`http://127.0.0.1:${user1Port}/`, { waitUntil: "domcontentloaded", timeout: 120_000 });
    await user2Page.goto(`http://127.0.0.1:${user2Port}/`, { waitUntil: "domcontentloaded", timeout: 120_000 });
    await user1Page.locator(".semio-table-host").first().waitFor({ state: "visible", timeout: 120_000 });
    await user2Page.locator(".semio-table-host").first().waitFor({ state: "visible", timeout: 120_000 });
    // 🔐️ Two DIFFERENT humans, each signing in from their own browser context against the same hub.
    // Serially, because the hub's sign-in bucket is keyed per address and both contexts share
    // 127.0.0.1 (AU3 gap 12): two simultaneous mints spend the bucket and the second gets a 429.
    await collabSignIn(user1Page, COLLAB_E2E_USER1_EMAIL, COLLAB_E2E_USER1_PASSWORD);
    await collabSignIn(user2Page, COLLAB_E2E_USER2_EMAIL, COLLAB_E2E_USER2_PASSWORD);
    console.log(`[collab-e2e] both humans hold a verified session authority (${provisioned[COLLAB_E2E_USER1_EMAIL]}, ${provisioned[COLLAB_E2E_USER2_EMAIL]})`);
    await user1Page.waitForTimeout(2_000);
    await user2Page.waitForTimeout(2_000);

    const scenario = await collabRunScenario(user1Page, user2Page, hubBaseUrl, user2Commands, hubDaemon.adminCapability);
    for (const outcome of scenario.results) results.push(outcome);

    hubDaemon = await collabRunRestartStep({ record, hubDaemon: hubDaemon!, hubPort, hubDataDir, user1: user1Page, user2: user2Page, user2Commands, spaceId: scenario.spaceId, artifactId: scenario.artifactId });

    await collabRunCollaborationBehaviours({ record, user1: user1Page, user2: user2Page, spaceId: scenario.spaceId, artifactId: scenario.artifactId });

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

export { COLLAB_E2E_ADMIN_TOKEN, COLLAB_E2E_DEV_BOOT_BUDGET_MS, COLLAB_E2E_HUB_BOOT_BUDGET_MS, COLLAB_E2E_PORT_MAX, COLLAB_E2E_PORT_MIN, COLLAB_E2E_PREBUILD_BUDGET_MS, COLLAB_E2E_REQUIRED_PLUGIN_IDS, COLLAB_E2E_STEP_NAMES, COLLAB_E2E_USER1_EMAIL, COLLAB_E2E_USER2_EMAIL, type CollabCommandFrameCounter, type CollabStepOutcome, collabClickToolbarButton, collabCountCommandFrames, collabHubDataDir, collabOutDir, collabPluginArtifactPath, collabPrebuildPlugins, collabPresenceColors, collabPresenceRows, collabRowIds, collabAwaitConvergence, collabEditorText, collabProvisionCredentials, collabRunCollaborationBehaviours, collabRunRestartStep, collabRunScenario, collabSignIn, collabUndo, collabScanPort, collabActivateShellRuntime, collabScreenshot, collabSelectOption, collabStartHub, collabStartUserDevServer, collabSubmitDialog, collabWaitForDialog, collabWaitForEditorText, collabWaitForNewRow, collabWaitForPresenceRoster, collabWaitForRow, runCollabE2eVerify };
