#!/usr/bin/env bun
/** @emoji 🧊️ `@semio-tech/framework-renderer-wgpu` task router. */
import { strict as assert } from "node:assert";
import { copyFileSync, existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { isAbsolute, join, relative, resolve, sep } from "node:path";
import Ajv from "ajv";
import {
  BundleScript,
  ScriptRouter,
  SEMIO_ASSET_SERVER_PORT,
  SEMIO_ASSET_BASE_URL_ENV,
  buildBudgetMs,
  daemonBudgetOpts,
  getWorkspaceRoot,
  orchestratorBudgetOpts,
  resolveTestLevel,
  runBundleScriptMain,
  runCargoTestBudgeted,
  runExactCargoLaws,
  runCmdStatus,
  runVitest,
  spawnDaemon,
  frameworkOsPlaygroundDefaultPort,
  loadFrameworkOsPlaygroundCatalog,
} from "../../../../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { startAssetServer } from "../../../../../../../../../🔨️modules/🖱️ui/🎨️styling/🏗️builder/🌐️vite/🟦️.ts";
import { nativeRendererBinary } from "../../🏗️compiler/🦀️native/📜️script.ts";
import { pluginModulesRoot } from "../../../../../../🧑‍💻dev/♻️activation/🟦️.ts";
import type { PlaygroundAssetSpec } from "../../../../../../🔌️plugin/📇️registry/🤖️generated/🎮️playgrounds/🟦️.ts";

import { assertBundleModuleRoutes, checkBrowserBoot, renderBrowserEntry } from "../../⚙️browser-build/🟦️.ts";
import { checkFrameWorker, generateFrameWorker, renderFrameWorker } from "../../🎞️frame-worker/🏗️builder/🟦️.ts";

const repoRoot = getWorkspaceRoot();
const rustPackageRoot = resolve(import.meta.dir, "../🦀️rust");
const crateName = "semio-framework-os-renderer-wgpu";
const outDir = join(repoRoot, ".🧬semio/🦑️repo/⚡️cache/📺️renderer-modules/🧊️wgpu");
const NATIVE_RUNNER_BENIGN_ENV_KEY = "SEMIO_DIRECT_CHILD_BENIGN";
const NATIVE_RUNNER_BENIGN_ENV_VALUE = "preserved";

function assertRendererCacheHome(): number {
  const configuredOutDir = relative(rustPackageRoot, outDir).replaceAll(sep, "/");
  const config = readFileSync(join(rustPackageRoot, "Trunk.toml"), "utf8");
  assert.equal(/^dist = "([^"]+)"$/mu.exec(config)?.[1], configuredOutDir, "Trunk must emit WGPU renderer modules inside .🧬semio/🦑️repo");
  return 1;
}

function nativeRunnerEnvironmentKeyIsProtected(key: string): boolean {
  const normalized = key.toUpperCase();
  return (
    normalized === "S_USER" ||
    normalized === "VITE_S_USER" ||
    normalized === "S_HUB_URL" ||
    normalized.includes("TOKEN") ||
    normalized.includes("SESSION") ||
    normalized.includes("CREDENTIAL") ||
    normalized.includes("BEARER") ||
    normalized.includes("CAPABILITY") ||
    normalized.includes("AUTHORIZATION") ||
    normalized.includes("COOKIE")
  );
}

function nativeRunnerEnvironment(source: NodeJS.ProcessEnv): NodeJS.ProcessEnv {
  const environment: NodeJS.ProcessEnv = {};
  for (const [key, value] of Object.entries(source)) if (!nativeRunnerEnvironmentKeyIsProtected(key)) environment[key] = value;
  environment[NATIVE_RUNNER_BENIGN_ENV_KEY] = NATIVE_RUNNER_BENIGN_ENV_VALUE;
  return environment;
}

function runNativeBinary(executable: string, args: readonly string[], environment: NodeJS.ProcessEnv): number {
  return runCmdStatus(executable, [...args], { cwd: repoRoot, env: nativeRunnerEnvironment(environment), ...daemonBudgetOpts() });
}

function proveNativeRunnerEnvironment(): void {
  const poisoned = {
    ...process.env,
    S_USER: "poison-user",
    VITE_S_USER: "poison-vite-user",
    S_HUB_URL: "poison-origin",
    S_SESSION: "poison-session",
    NPM_TOKEN: "poison-token",
    S_LOCAL_CREDENTIAL_FD: "3",
    AUTHORIZATION: "poison-authorization",
    COOKIE: "poison-cookie",
  };
  const consumer = String.raw`
const keys = Object.keys(process.env).map(key => key.toUpperCase());
const protectedKey = key => key === "S_USER" || key === "VITE_S_USER" || key === "S_HUB_URL" || key.includes("TOKEN") || key.includes("SESSION") || key.includes("CREDENTIAL") || key.includes("BEARER") || key.includes("CAPABILITY") || key.includes("AUTHORIZATION") || key.includes("COOKIE");
process.exit(keys.some(protectedKey) || process.env.SEMIO_DIRECT_CHILD_BENIGN !== "preserved" ? 1 : 0);`;
  if (runNativeBinary(process.execPath, ["-e", consumer], poisoned) !== 0) throw new Error("ordinary WGPU native runner leaked protected parent environment");
  const entrypoint = readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⌨️native-entrypoint/🦀️.rs"), "utf8");
  const guard = entrypoint.indexOf("if !protected_credential_environment_is_absent()");
  const claim = entrypoint.indexOf('claim_inherited_local_hub_credential("native")');
  const plugin = entrypoint.indexOf('arg_value("--plugin")');
  if (guard < 0 || claim < 0 || plugin < 0 || guard > claim || guard > plugin) throw new Error("native binary protected-environment guard no longer precedes credential claim and plugin activation");
  console.log("native-environment-check: poisoned ordinary runner sanitized and binary fail-closed guard precedes credential/plugin activation");
}

//#region 🌐️ DevServer
/** @emoji 👥️ Full `process.env` passthrough for the spawned `trunk` child — raw (unprefixed)
 * `S_HUB_URL`/`S_USER`/`S_DATA_DIR` reach it exactly as set by the launching `dev` process (the wgpu
 * user launchers in `.vscode/🧩️launch.seed.jsonc`'s `devLaunchers.s.users`), since native/wasm-in-trunk
 * code reads `std::env` directly rather than through a `import.meta.env.VITE_*` compile-time define. */
function trunkEnv(): NodeJS.ProcessEnv {
  const env = { ...process.env };
  delete env.NO_COLOR;
  delete env.FORCE_COLOR;
  if (env.SEMIO_PARITY_QUIET_CARGO === "1") env.RUSTFLAGS = [env.RUSTFLAGS, "-Awarnings"].filter(Boolean).join(" ");
  return env;
}

/** 🌐️ Runs a long-lived child without blocking Bun's asset-server event loop. */
async function runInteractiveCommand(command: string, args: string[], cwd: string, env: NodeJS.ProcessEnv): Promise<number> {
  const daemon = spawnDaemon(command, args, { cwd, env });
  const terminate = () => daemon.kill();
  process.once("SIGINT", terminate);
  process.once("SIGTERM", terminate);
  try {
    return await new Promise<number>((resolve, reject) => {
      daemon.child.once("error", reject);
      daemon.child.once("exit", (code) => resolve(code ?? 1));
    });
  } finally {
    process.off("SIGINT", terminate);
    process.off("SIGTERM", terminate);
    daemon.kill();
  }
}

/** @emoji 🔁️ Publishes Trunk's wasm-bindgen pair under the stable names the React shell's
 * `🎬️renderer-boot` requests (`/renderer-modules/wgpu/semio_framework_renderer_wgpu.js`). `Trunk.toml`
 * pins `filehash = false`, so both emitted names ARE the crate id and are addressed by name; the
 * previous scan for a `<crate>-` prefix could only ever match the content-hashed names Trunk no longer
 * emits, and failed every non-serving build with "missing trunk wgpu renderer js artifact" after Trunk
 * itself reported success. */
function syncStableRendererArtifacts(): void {
  const js = join(outDir, `${crateName}.js`);
  const wasm = join(outDir, `${crateName}_bg.wasm`);
  for (const artifact of [js, wasm]) if (!existsSync(artifact)) throw new Error(`missing trunk wgpu renderer artifact ${relative(repoRoot, artifact)}`);
  copyFileSync(js, join(outDir, "semio_framework_renderer_wgpu.js"));
  copyFileSync(wasm, join(outDir, "semio-framework-renderer-wgpu_bg.wasm"));
}

function assetServerBaseUrl(): string {
  return `http://127.0.0.1:${SEMIO_ASSET_SERVER_PORT}`;
}

/** @emoji 🗂️ The active playground variant's declared asset specs (tile-proxy, mesh-collection, static-dir). */
function variantAssetSpecs(variant: string): readonly PlaygroundAssetSpec[] {
  const row = loadFrameworkOsPlaygroundCatalog().find((entry) => entry.variant === variant);
  return row?.assets ?? [];
}

/** @emoji 🌐️ Generic dev-time asset server bootstrap driven by the active playground's declared
 * asset specs — mesh-collection/static-dir are served here so Trunk proxies and native
 * `SEMIO_ASSET_BASE_URL` can resolve `/mesh/*` (and fixture routes) without Vite. */
function ensureAssetServer(variant: string): void {
  const specs = variantAssetSpecs(variant);
  if (specs.length === 0) return;
  startAssetServer(repoRoot, SEMIO_ASSET_SERVER_PORT, specs);
  console.log(`asset server serving at ${assetServerBaseUrl()} (${specs.map((s) => `${s.kind}:${s.route}`).join(", ")})`);
}

/** 🎯️Resolves the `--app <appId>` args for `semio-wgpu-native` from the catalog row matching `filterPlugin`, or `[]` when the row has no `app`. */
function resolveNativeAppArgs(catalog: ReturnType<typeof loadFrameworkOsPlaygroundCatalog>, filterPlugin: string): string[] {
  const row = catalog.find((r) => r.variant === filterPlugin);
  return row?.app ? ["--app", row.app] : [];
}

class TrunkBuildScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await checkBrowserBoot(this.root);
    await checkFrameWorker(this.root);
    assertBundleModuleRoutes(this.root);
    mkdirSync(outDir, { recursive: true });
    const release = segments.includes("--release") || segments.includes("--dist");
    const args = ["build", "--config", "Trunk.toml"];
    if (release) args.push("--release");
    if (runCmdStatus("trunk", args, { cwd: rustPackageRoot, env: trunkEnv(), budgetMs: buildBudgetMs() }) !== 0) throw new Error("trunk build failed for wgpu renderer");
    syncStableRendererArtifacts();
    console.log(`trunk built wgpu renderer -> ${outDir}`);
  }
}

class TrunkServeScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await checkBrowserBoot(this.root);
    await checkFrameWorker(this.root);
    assertBundleModuleRoutes(this.root);
    const program = process.env.SEMIO_PLUGIN ?? process.env.PLAYGROUND_APP_KIND ?? "s";
    ensureAssetServer(program);
    const catalog = loadFrameworkOsPlaygroundCatalog();
    const defaultPort = String(frameworkOsPlaygroundDefaultPort(catalog, program, "wgpu"));
    const port = process.env.S_OS_PORT ?? defaultPort;
    const extra = segments.filter((segment, index, all) => segment !== "--port" && all[index - 1] !== "--port");
    const args = ["serve", "--config", "Trunk.toml", "--port", port, ...extra];
    const profile = process.env.SEMIO_BUILD_MODE === "ship" ? "release" : "dev";
    if (process.env.SEMIO_PARITY_QUIET_CARGO === "1") args.push("--ignore", pluginModulesRoot(profile));
    if ((await runInteractiveCommand("trunk", args, rustPackageRoot, trunkEnv())) !== 0) throw new Error("trunk serve failed for wgpu renderer");
  }
}
//#endregion 🌐️ DevServer

/** ⚖️ Reads one option for the headless native scale benchmark. */
function scaleModeArgValue(segments: readonly string[], flag: string): string | undefined {
  const index = segments.indexOf(flag);
  return index >= 0 ? segments[index + 1] : undefined;
}

function scaleModePassthroughArgs(segments: readonly string[]): string[] {
  const args: string[] = [];
  for (const flag of ["--scale", "--scale-wasm", "--shards", "--report"]) {
    const value = scaleModeArgValue(segments, flag);
    if (value) args.push(flag, value);
  }
  return args;
}

class NativeRunScript extends BundleScript {
  async run([profile, ...segments]: string[]): Promise<void> {
    if (!["dev", "release"].includes(profile) || segments.some((argument) => argument === "--release" || argument === "--dist")) throw new Error("Select native or native-release through Nx");
    const executable = nativeRendererBinary(rustPackageRoot, profile);
    if (segments.includes("--scale")) {
      if (runNativeBinary(executable, scaleModePassthroughArgs(segments), process.env) !== 0) {
        throw new Error("native wgpu scale-bench run failed");
      }
      return;
    }
    const filterPlugin = segments[0] || process.env.SEMIO_PLUGIN || "s";
    // 🧊️ Delegates to the Nx-cached `activate-<variant>-wgpu-<profile>` target (`playgroundPreparationTargets`
    // in `…🦑️repo/🔨️modules/📚️library/🟨️.mjs`) instead of a raw `plugin` build — a warm cache restores the
    // per-variant/profile module directory below instantly rather than rebuilding the whole catalog serially.
    if (runCmdStatus("bun", ["nx", "run", `@semio-tech/framework-os-dev:activate-${filterPlugin}-wgpu-${profile}`], { cwd: repoRoot, ...orchestratorBudgetOpts() }) !== 0) throw new Error(`Plugin activation failed: ${filterPlugin}`);
    ensureAssetServer(filterPlugin);
    const nativeEnv = nativeRunnerEnvironment(process.env);
    nativeEnv.SEMIO_PLUGIN_MODULES = pluginModulesRoot(profile as "dev" | "release");
    if (variantAssetSpecs(filterPlugin).length > 0) {
      nativeEnv[SEMIO_ASSET_BASE_URL_ENV] = assetServerBaseUrl();
    }
    const catalog = loadFrameworkOsPlaygroundCatalog();
    const appArgs = resolveNativeAppArgs(catalog, filterPlugin);
    // 🧪️ ticket 26/08/17/FINISH-HUB-SPACES-COLLABORATION-END-TO-END — `--smoke` passes straight
    // through to `semio-wgpu-native` (boots headless, dumps the widget tree as JSON, exits) instead of
    // opening a real window; an honest way to drive/observe this shell in an environment that cannot.
    const smokeArgs = segments.includes("--smoke") ? ["--smoke"] : [];
    if (runNativeBinary(executable, ["--plugin", filterPlugin, ...appArgs, ...smokeArgs], nativeEnv) !== 0) {
      throw new Error("native wgpu renderer run failed");
    }
  }
}

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    assertRendererCacheHome();
    const { rest } = resolveTestLevel(segments);
    await runCargoTestBudgeted([crateName], this.repoRoot, rest);
    await runVitest(this.root, rest, "vitest.config.ts");
  }
}

/** 🦀️ Runs the existing budgeted Cargo tests without invoking browser tests. */
class NativeTestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runCargoTestBudgeted([crateName], this.repoRoot, rest);
  }
}

/** 🏠️ Independently executes the neutral retained-Home bootstrap trace and audits the native mount. */
function directoryRetainedHomeBootstrapOracle(): number {
  const fixturePath = join(repoRoot, "🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory/🚀️event-page-bootstrap-v1.json");
  const schemaPath = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🔣️.json");
  const fixture = JSON.parse(readFileSync(fixturePath, "utf8"));
  const schemaModule = JSON.parse(readFileSync(schemaPath, "utf8"));
  const ajv = new Ajv({ strict: true, allErrors: true });
  ajv.addSchema(schemaModule);
  const validate = ajv.getSchema(`${schemaModule.$id}#/$defs/DirectoryEventPageBootstrapTraceV1`);
  assert(validate, "os.directory schema module must export DirectoryEventPageBootstrapTraceV1");
  let checks = 0;
  const check = (condition: unknown, message: string): void => {
    assert(condition, message);
    checks += 1;
  };
  check(validate(fixture), JSON.stringify(validate.errors));
  const unknown = structuredClone(fixture);
  unknown.clientAck = true;
  check(!validate(unknown), "neutral bootstrap trace must reject client-owned ACK authority");
  const duplicateFields = structuredClone(fixture);
  duplicateFields.forgedAckFields[4] = duplicateFields.forgedAckFields[0];
  check(!validate(duplicateFields), "neutral bootstrap trace must require every ACK field exactly once");

  let epoch = fixture.bootstrapEpoch;
  let after = fixture.initialAfter;
  let pending: any = null;
  let live = false;
  let closed = false;
  let acknowledgements = 0;
  const present = (page: any): any => {
    check(!closed && !live && pending === null && page.afterSeqExclusive === after && page.throughSeqInclusive >= after, "page ordering mismatch");
    pending = structuredClone(page);
    return { bootstrapEpoch: epoch, sessionBindingSha256: page.sessionBindingSha256, authorizationGeneration: page.authorizationGeneration, throughSeqInclusive: page.throughSeqInclusive, receiptSha256: page.receiptSha256 };
  };
  const acknowledge = (receipt: any): "fetch" | "live" => {
    assert(pending && !closed);
    for (const key of fixture.forgedAckFields) assert.deepEqual(receipt[key], key === "bootstrapEpoch" ? epoch : pending[key]);
    after = pending.throughSeqInclusive;
    live = !pending.hasMore;
    pending = null;
    acknowledgements += 1;
    return live ? "live" : "fetch";
  };
  const first = present(fixture.pages[0]);
  check(after === fixture.initialAfter && acknowledgements === 0, "presentation advanced before terminal Home publication");
  assert.throws(() => present(fixture.pages[1]));
  check(after === fixture.initialAfter && acknowledgements === 0, "page two was admitted before page one's terminal Home publication");
  for (const field of fixture.forgedAckFields) {
    const forged = { ...first, [field]: typeof first[field] === "number" ? first[field] + 1 : "d".repeat(64) };
    assert.throws(() => acknowledge(forged));
    check(after === fixture.initialAfter && acknowledgements === 0, `${field} forgery advanced the cursor`);
  }
  check(acknowledge(first) === "fetch" && after === fixture.pages[0].throughSeqInclusive, "first terminal receipt did not request the next page");
  const second = present(fixture.pages[1]);
  check(acknowledge(second) === "live" && after === fixture.expectedSocketSince, "final receipt did not establish the exact socket cursor");
  let dirtyRefetches = 0;
  for (const _wakeup of fixture.wakeups) {
    if (live) {
      live = false;
      dirtyRefetches += 1;
    }
  }
  check(dirtyRefetches === 1 && after === fixture.expectedSocketSince, "live wakeups did not coalesce at the durable cursor");
  pending = structuredClone(fixture.pages[0]);
  after = fixture.retry.expectedAfter;
  pending = null;
  check(after === fixture.retry.transportAfter && after === fixture.retry.expectedAfter, "retry changed the acknowledged cursor");
  closed = true;
  const beforeLateAck = acknowledgements;
  check(closed && acknowledgements === beforeLateAck && fixture.cancellation.expectedAckCount === 0, "late cancellation result emitted an ACK");
  epoch = fixture.rebootstrap.nextEpoch;
  after = fixture.rebootstrap.expectedAfter;
  live = false;
  pending = null;
  closed = false;
  check(epoch > fixture.bootstrapEpoch && after === 0, "rebootstrap did not fence the prior epoch at zero");
  let terminalReconnects = 0;
  const beforeTerminalEpoch = epoch;
  if (fixture.terminalClose.code === 4401) {
    epoch += 1;
    after = fixture.terminalClose.expectedAfter;
  } else {
    terminalReconnects += 1;
  }
  check(epoch === fixture.terminalClose.expectedBootstrapEpoch && epoch > beforeTerminalEpoch && after === 0 && terminalReconnects === fixture.terminalClose.expectedReconnects, "terminal authenticated close did not produce one newer fenced identity rebootstrap");
  check(fixture.retainedHome.instanceId !== fixture.retainedHome.switchedVisibleInstanceId && fixture.retainedHome.expectedDestroyCount === 1, "retained Home fixture does not distinguish visible-session ownership");

  const shellPath = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs");
  const shell = readFileSync(shellPath, "utf8");
  for (const marker of [
    "struct DirectoryHomeProjection",
    "DirectoryEventPageBootstrapV1",
    "start_directory_event_page_fetch",
    "start_directory_home_publication",
    "terminal_directory_home_ack",
    "applyDirectoryEventPage",
    "stream_acknowledged(since)",
    "home.wake(rebootstrap)",
    "runner.take_terminal()",
    "take_destroy_authority",
    "page two cannot be requested before terminal Home publication",
    "duplicate live wake coalesces while the page refetch is already pending",
    "late Home publication has no surviving receiver or ACK path",
    "authenticated terminal close cannot reconnect",
    "terminal stream remains closed after every reconnect deadline",
    "terminal identity close restarts from raw cursor zero",
  ]) check(shell.includes(marker), `native retained-Home bootstrap lacks ${marker}`);
  check(!shell.includes("client.stream(0)"), "native global directory lane still opens an observed-frontier stream");
  check(!shell.includes("dispatch_directory_event_batch") && !shell.includes("fold_directory_events_action"), "native global directory lane still folds raw events");
  check((shell.match(/program\.destroy_app\(instance_id\)/g) ?? []).length >= 2, "drop and hot reload do not consume retained Home destruction authority");
  return checks;
}

class DirectoryRetainedHomeBootstrapSourceCheckScript extends BundleScript {
  run(segments: string[]): void {
    if (segments.length) throw new Error("directory-retained-home-bootstrap-source-check accepts no arguments");
    console.log(`directory-retained-home-bootstrap-source-check: checks=${directoryRetainedHomeBootstrapOracle()} clean`);
  }
}

class DirectoryRetainedHomeBootstrapNativeCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("directory-retained-home-bootstrap-native-check accepts no arguments");
    const artifactDir = process.env.SEMIO_TEST_ARTIFACT_DIR;
    const ticketRoot = resolve(repoRoot, ".🧬semio/🦑️repo/🎫️tickets");
    if (!artifactDir || !isAbsolute(artifactDir) || !resolve(artifactDir).startsWith(`${ticketRoot}${sep}`)) {
      throw new Error("SEMIO_TEST_ARTIFACT_DIR must be an absolute ticket-local directory");
    }
    const checks = directoryRetainedHomeBootstrapOracle();
    const receipts = await runExactCargoLaws({
      cwd: repoRoot,
      artifactDir: resolve(artifactDir),
      env: { ...process.env, CARGO_BUILD_JOBS: "1" },
      groups: [{
        package: crateName,
        target: { kind: "lib" },
        laws: [
          "shell::command_registry_tests::directory_home_bootstrap_waits_for_terminal_config_ack_and_retains_home_across_visibility",
          "shell::command_registry_tests::directory_home_bootstrap_retries_cancels_and_rebootstraps_without_cursor_loss",
          "shell::command_registry_tests::directory_home_terminal_receipt_rejects_unknown_fields_and_nonreceipt_effects",
        ],
      }],
      progress(event) { console.log(`directory-retained-home-bootstrap ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`); },
    });
    console.log(`directory-retained-home-bootstrap-native-receipts: ${JSON.stringify(receipts)}`);
    console.log(`directory-retained-home-bootstrap-native-check: sourceChecks=${checks} nativeLaws=3 clean`);
  }
}

function normalizedPresenceRowsOracle(): number {
  const shell = readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs"), "utf8");
  const markers = [
    '.filter(|peer| peer.surface.as_deref() == Some(target_surface))',
    "color: peer.color",
    "fn presence_rows_require_each_normalized_surface_and_preserve_hub_color(",
    "peer(\"c\", None, 9)",
    "vec![(\"b\", Some(8))]",
  ];
  for (const marker of markers) assert(shell.includes(marker), `normalized WGPU presence rows lack ${marker}`);
  assert(!shell.includes("PresencePeer` itself\n/// carries no `surface` field"), "obsolete no-surface authority claim remains");
  return markers.length + 1;
}

/** @emoji 👥️ Checks normalized WGPU surface/color projection without invoking Cargo. */
class NormalizedPresenceRowsSourceCheckScript extends BundleScript {
  run(segments: string[]): void {
    if (segments.length) throw new Error("normalized-presence-rows-source-check accepts no arguments");
    console.log(`normalized-presence-rows-source-check: checks=${normalizedPresenceRowsOracle()} clean`);
  }
}

/** @emoji 🎨️ Runs the exact normalized WGPU surface/color row law. */
class NormalizedPresenceRowsNativeCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("normalized-presence-rows-native-check accepts no arguments");
    const artifactDir = process.env.SEMIO_TEST_ARTIFACT_DIR;
    const ticketRoot = resolve(repoRoot, ".🧬semio/🦑️repo/🎫️tickets");
    if (!artifactDir || !isAbsolute(artifactDir) || !resolve(artifactDir).startsWith(`${ticketRoot}${sep}`)) throw new Error("SEMIO_TEST_ARTIFACT_DIR must be an absolute ticket-local directory");
    const checks = normalizedPresenceRowsOracle();
    const receipts = await runExactCargoLaws({
      cwd: repoRoot,
      artifactDir: resolve(artifactDir),
      env: { ...process.env, CARGO_BUILD_JOBS: "1" },
      groups: [{ package: crateName, target: { kind: "lib" }, laws: ["shell::command_registry_tests::presence_rows_require_each_normalized_surface_and_preserve_hub_color"] }],
    });
    assert.equal(receipts[0]!.assertions, 1);
    console.log(`normalized-presence-rows-native-check: sourceChecks=${checks} nativeLaws=1 clean`);
  }
}

/** @emoji 🧵️ Runs the browser Worker transport protocol without invoking Cargo. */
class BrowserWorkerTestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runVitest(this.root, ["🧪️tests/📨️browser-frame-transport/🟦️.ts", "🧪️tests/🎮️browser-interactive-job-port/🟦️.ts", "🧪️tests/⏱️wgpu-ui-turn-budget/🟦️.ts", "🧪️tests/⏱️wgpu-worker-step-budget/🟦️.ts", "🧪️tests/🔬️wgpu-extension-dispatch/🟦️.ts", ...segments], "vitest.config.ts");
  }
}

/** @emoji 🧾️ Runs the deterministic in-memory frame-worker owner contract at `long` or above — importing
 * its four independent oracles (the TypeScript compiler, Ajv, emoji-regex and the discovery taxonomy)
 * costs ~14 s on an idle machine before a single case runs, and the cases themselves render two full
 * browser bundles, so the suite cannot honestly sit at the fundamental or quick budget. */
class PreviewGeneratedTestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments, "long");
    await runVitest(this.root, ["🧪️tests/🧩️package-integration/🟦️.ts", ...rest], "vitest.config.ts");
  }
}

/** @emoji 🧵️ Bundles both browser isolates without invoking Cargo or Trunk. */
class BrowserWorkerCheckScript extends BundleScript {
  async run(_segments: string[]): Promise<void> {
    await checkBrowserBoot(this.root);
    await checkFrameWorker(this.root);
  }
}

/** @emoji 🧵️ Generates only the deterministic browser frame-worker artifact. */
class GenerateFrameWorkerScript extends BundleScript {
  async run(): Promise<void> {
    await generateFrameWorker(this.root);
    console.log("framework-renderer-wgpu: generated 🎞️frame-worker.js");
  }
}

/** 🧾️ Emits the canonical read-only generator protocol from the same in-memory browser bundle. */
class PreviewGeneratedScript extends BundleScript {
  async run(): Promise<void> {
    const artifact = await renderFrameWorker(this.root);
    const nodes = [{ bytesBase64: Buffer.from(artifact.content).toString("base64"), mode: 0o644, nodeKind: "file" as const, path: relative(repoRoot, artifact.path).replaceAll("\\", "/").normalize("NFC") }];
    process.stdout.write(`${JSON.stringify({ contractId: "wgpu-frame-worker", nodes, schemaVersion: 1, staleRemovals: [] })}\n`);
  }
}

/** @emoji ✅️ Checks the frame-worker bytes without invoking any renderer build. */
class CheckFrameWorkerScript extends BundleScript {
  async run(): Promise<void> {
    await checkFrameWorker(this.root);
    console.log("framework-renderer-wgpu: 🎞️frame-worker.js is fresh");
  }
}

//#region 🔖️LintScript
/** 🎨️Raw color-construction calls (`Rgba::new`/`from_srgb8`) must live only inside `framework/ui/wgpu`'s theme module — the renderer takes every color via `ui_wgpu::Theme`. */
function collectWgpuColorLiteralViolations(bundleRoot: string): string[] {
  const libPath = join(bundleRoot, "../../🧊️renderer/🦀️.rs");
  if (!existsSync(libPath)) return [];
  const text = readFileSync(libPath, "utf8");
  const violations: string[] = [];
  const lines = text.split("\n");
  for (let i = 0; i < lines.length; i++) {
    const line = lines[i]!;
    if (/\bRgba::new\(|\bfrom_srgb8\(/.test(line)) {
      violations.push(`🦀️.rs:${i + 1}: ${line.trim()}`);
    }
  }
  return violations;
}

class LintScript extends BundleScript {
  run(_segments: string[]): void {
    const artifactChecks = assertRendererCacheHome();
    const violations = collectWgpuColorLiteralViolations(this.root);
    if (violations.length === 0) {
      console.log(`framework-renderer-wgpu: color-literal and artifact-home lint passed (${artifactChecks} checks)`);
      return;
    }
    console.error(`framework-renderer-wgpu: found ${violations.length} raw color-construction call(s) outside framework/ui/wgpu theme:`);
    for (const v of violations.slice(0, 40)) console.error(`  ${v}`);
    if (violations.length > 40) console.error(`  … and ${violations.length - 40} more`);
    process.exit(1);
  }
}
//#endregion 🔖️LintScript

const router = new ScriptRouter(import.meta.dir)
  .register("wasm", TrunkBuildScript)
  .register("build", TrunkBuildScript)
  .register("serve", TrunkServeScript)
  .register("dev", TrunkServeScript)
  .register("native", NativeRunScript)
  .register(
    "native-environment-check",
    class extends BundleScript {
      run(): void {
        proveNativeRunnerEnvironment();
      }
    },
  )
  .register("test", TestScript)
  .register("test-native", NativeTestScript)
  .register("directory-retained-home-bootstrap-source-check", DirectoryRetainedHomeBootstrapSourceCheckScript)
  .register("directory-retained-home-bootstrap-native-check", DirectoryRetainedHomeBootstrapNativeCheckScript)
  .register("normalized-presence-rows-source-check", NormalizedPresenceRowsSourceCheckScript)
  .register("normalized-presence-rows-native-check", NormalizedPresenceRowsNativeCheckScript)
  .register("test-browser-worker", BrowserWorkerTestScript)
  .register("test-preview-generated", PreviewGeneratedTestScript)
  .register("check-browser-worker", BrowserWorkerCheckScript)
  .register("generate-frame-worker", GenerateFrameWorkerScript)
  .register("preview-generated", PreviewGeneratedScript)
  .register("check-frame-worker", CheckFrameWorkerScript)
  .register("lint", LintScript);

if (import.meta.main) {
  await runBundleScriptMain(router, import.meta.url, { defaultCommand: "wasm" });
}
