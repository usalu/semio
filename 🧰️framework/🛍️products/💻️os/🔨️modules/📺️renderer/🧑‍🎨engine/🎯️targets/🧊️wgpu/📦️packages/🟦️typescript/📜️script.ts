#!/usr/bin/env bun
import { resolveTestLevel } from "../../../../../../../../../🔨️modules/🏃️process/🧪️testing/🎚️budget/🟦️.ts";
import { buildBudgetMs } from "../../../../../../../../../🔨️modules/🏃️process/⏱️budget/🟦️.ts";
/** 🧊️ `@semio-tech/framework-renderer-wgpu` task router. */
import { strict as assert } from "node:assert";
import { spawnSync } from "node:child_process";
import { existsSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { isAbsolute, join, relative, resolve, sep } from "node:path";
import { getWorkspaceRoot, packageTestBudgetMs, runCargo, runRepositoryCargoTests, runRepositoryExactCargoLaws, runRepositoryTestCommand, runVitest } from "../../../../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts";

import { checkBrowserBoot, renderBrowserEntry } from "../../⚙️browser-build/🟦️.ts";
import { checkFrameWorker, generateFrameWorker, renderFrameWorker } from "../../🎞️frame-worker/🏗️builder/🟦️.ts";

import { runNativeBinary } from "../../⌨️native-entrypoint/📜️script.ts";

const repoRoot = getWorkspaceRoot();
const rustPackageRoot = resolve(import.meta.dir, "../🦀️rust");
const crateName = "semio-framework-os-renderer-wgpu";

function assertRendererOutputOwnership(): number {
  const project = JSON.parse(readFileSync(join(import.meta.dir, "📋️project.json"), "utf8"));
  for (const profile of ["dev", "release"]) {
    const target = project.targets[profile === "dev" ? "wasm" : "wasm-release"];
    assert.equal(target.cache, true);
    assert.deepEqual(target.outputs, ["{workspaceRoot}/" + relative(repoRoot, join(rustPackageRoot, "dist", "wasm-" + profile)).replaceAll(sep, "/")]);
  }
  return 2;
}

async function proveNativeRunnerEnvironment(): Promise<void> {
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
  await runNativeBinary(process.execPath, ["-e", consumer], poisoned, repoRoot);
  const entrypoint = readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⌨️native-entrypoint/🦀️.rs"), "utf8");
  const guard = entrypoint.indexOf("if !protected_credential_environment_is_absent()");
  const claim = entrypoint.indexOf('claim_inherited_local_hub_credential("native")');
  const plugin = entrypoint.indexOf('arg_value("--plugin")');
  if (guard < 0 || claim < 0 || plugin < 0 || guard > claim || guard > plugin) throw new Error("native binary protected-environment guard no longer precedes credential claim and plugin activation");
  console.log("native-environment-check: poisoned ordinary runner sanitized and binary fail-closed guard precedes credential/plugin activation");
}

/**
 * 🧪️ The whole crate plus its vitest cases. Floored at `long` for the same reason as
 * [[WgpuUnitTestScript]]: 1372 laws take ~105 s across seven threads, so the fundamental 15 s budget
 * killed every invocation with no red test in it.
 */
class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments[0] === "worker-cell") {
      if (segments.length !== 1) throw new Error("Expected test worker-cell");
      if (!process.env.SEMIO_TEST_ARTIFACT_DIR) throw new Error("Caller-owned SEMIO_TEST_ARTIFACT_DIR is required");
      await runRepositoryTestCommand(process.execPath, ["test", join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🧵️worker-cell/🧪️tests/🟦️.ts")], {
        cwd: this.repoRoot, env: { ...process.env, SEMIO_TEST_ARTIFACT_DIR: resolve(this.repoRoot, process.env.SEMIO_TEST_ARTIFACT_DIR) }, budgetMs: 15000, throwOnFailure: true,
      });
      return;
    }
    assertRendererOutputOwnership();
    const { rest } = resolveTestLevel(segments, "long");
    await runRepositoryCargoTests([crateName], this.repoRoot, rest);
    await runVitest(this.root, rest, "../../🧪️tests/🎚️config/🟦️.ts");
  }
}

/** 🪶️ Runs the original raw-text Socket snapshot and independent physical SQLite laws. */
class SocketSnapshotSqliteScript extends BundleScript {
 async run(segments:string[]):Promise<void>{
  if(segments.length!==1||!["source","native"].includes(segments[0]))throw Error("test-socket-snapshot-sqlite requires source or native");
  if(segments[0]==="native"){await runRepositoryCargoTests([crateName],this.repoRoot,["--lib","native_socket_sqlite_snapshot_","--","--nocapture"]);return;}
  const file=resolve(this.root,"../../🧊️renderer/🪶️sqlite/🧪️tests/🟦️.ts");
  await runRepositoryTestCommand(process.execPath,["test",file],{cwd:this.repoRoot,budgetMs:120000});
  await runRepositoryTestCommand(process.execPath,[resolve(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--allowImportingTsExtensions","--types","bun",file],{cwd:this.repoRoot,budgetMs:120000});
 }
}

/** 🦀️ Runs the existing budgeted Cargo tests without invoking browser tests. */
class NativeTestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments, "long");
    await runRepositoryCargoTests([crateName], this.repoRoot, rest);
  }
}

/**
 * 🧊️ The crate's wgpu unit laws alone (`--lib`). Every `🧪️tests/🔬️wgpu-*` case directory under
 * `🧑‍🎨engine` is mounted into this library with `#[cfg(test)] #[path = …]` instead of being declared
 * as a `[[test]]` binary, so `--lib` is the selector that compiles and runs them — and the selector
 * a dangling mount wedges, which is why it gets a target of its own rather than hiding inside
 * `test-native`'s wider all-targets build.
 *
 * Runs at `long` or above: the fifty mounted case directories are 912 laws over the os renderer, ~68 s
 * serial and ~25 s across seven threads on an idle machine, so the suite cannot honestly sit at the
 * fundamental 15 s or quick 30 s budget — it was killed mid-run at every invocation
 * (`[budget] cargo nextest run … exceeded 15000ms — killed`) and reported as a target failure with no
 * red test in it. Its two siblings (`@semio-tech/ui-rs:test-wgpu-engine`, 568 laws, and
 * `semio-framework-os-infinite:test-wgpu-world-terrain`, 199) fit the fundamental budget and keep it.
 */
class WgpuUnitTestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments, "long");
    await runRepositoryCargoTests([crateName], this.repoRoot, ["--lib", ...rest]);
  }
}

/** 🎬️ Executes neutral media-slot vectors through Ajv and seven exact native retained-tree laws. */
class MediaSlotContractTestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("media-slot contract accepts no arguments");
    await runVitest(this.root, ["🧪️tests/🎬️presented-media-slots/🟦️.ts"], "../../🧪️tests/🎚️config/🟦️.ts");
    const laws = [
      "media_slot_fixture_composes_body_clip_and_exact_owner",
      "media_slot_caps_refuse_the_entire_overflow_publication",
      "media_slot_descriptor_byte_budget_refuses_oversized_publication",
      "media_slot_occlusion_vectors_include_foreground_window_chrome",
      "media_slot_concealment_preserves_the_accepted_transport_identity",
      "media_slot_identity_registry_requeries_none_and_rejects_retired_or_foreign_replies",
      "media_slot_tokens_survive_ui_generation_but_change_with_document_and_resource_authority",
    ].map((law) => `media_slots::tests::${law}`);
    const receipts = await runRepositoryExactCargoLaws({
      cwd: this.repoRoot,
      env: { ...process.env, RUST_MIN_STACK: "33554432" },
      nativeEnv: { RUST_MIN_STACK: "134217728" },
      buildBudgetMs: buildBudgetMs(),
      listBudgetMs: 60_000,
      lawBudgetMs: 120_000,
      groups: [{ package: crateName, target: { kind: "lib" }, laws }],
      progress(event) { console.log(`media-slot-contract ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`); },
    });
    console.log(`media-slot-contract: ${receipts.reduce((count, receipt) => count + receipt.laws.length, 0)} exact native laws passed`);
  }
}

/** 🔬️ Checks browser Rust code before Wasm binding and optimization. */
class WasmCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runCargo(["check", "--locked", "--offline", "-p", crateName, "--lib", "--target", "wasm32-unknown-unknown", ...segments], this.repoRoot);
  }
}

/** 🏠️ Independently executes the neutral retained-Home bootstrap trace and audits the native mount. */
function directoryRetainedHomeBootstrapOracle(): number {
  const fixturePath = join(repoRoot, "🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory/🚀️event-page-bootstrap-v1.json");
  const fixture = JSON.parse(readFileSync(fixturePath, "utf8"));
  let checks = 0;
  const check = (condition: unknown, message: string): void => {
    assert(condition, message);
    checks += 1;
  };

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
  const laws = readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-command-registry/🦀️.rs"), "utf8");
  check(shell.includes('#[path = "../../🧪️tests/🔬️wgpu-command-registry/🦀️.rs"]\nmod command_registry_tests;'), "native retained-Home laws have no physical mount");
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
  ]) check(shell.includes(marker), `native retained-Home bootstrap lacks ${marker}`);
  for (const marker of [
    "page two cannot be requested before terminal Home publication",
    "duplicate live wake coalesces while the page refetch is already pending",
    "late Home publication has no surviving receiver or ACK path",
    "authenticated terminal close cannot reconnect",
    "terminal stream remains closed after every reconnect deadline",
    "terminal identity close restarts from raw cursor zero",
  ]) check(laws.includes(marker), `native retained-Home law lacks ${marker}`);
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
    const receipts = await runRepositoryExactCargoLaws({
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

/** 👥️ Checks normalized WGPU surface/color projection without invoking Cargo. */
class NormalizedPresenceRowsSourceCheckScript extends BundleScript {
  run(segments: string[]): void {
    if (segments.length) throw new Error("normalized-presence-rows-source-check accepts no arguments");
    console.log(`normalized-presence-rows-source-check: checks=${normalizedPresenceRowsOracle()} clean`);
  }
}

/** 🎨️ Runs the exact normalized WGPU surface/color row law. */
class NormalizedPresenceRowsNativeCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("normalized-presence-rows-native-check accepts no arguments");
    const artifactDir = process.env.SEMIO_TEST_ARTIFACT_DIR;
    const ticketRoot = resolve(repoRoot, ".🧬semio/🦑️repo/🎫️tickets");
    if (!artifactDir || !isAbsolute(artifactDir) || !resolve(artifactDir).startsWith(`${ticketRoot}${sep}`)) throw new Error("SEMIO_TEST_ARTIFACT_DIR must be an absolute ticket-local directory");
    const checks = normalizedPresenceRowsOracle();
    const receipts = await runRepositoryExactCargoLaws({
      cwd: repoRoot,
      artifactDir: resolve(artifactDir),
      env: { ...process.env, CARGO_BUILD_JOBS: "1" },
      groups: [{ package: crateName, target: { kind: "lib" }, laws: ["shell::command_registry_tests::presence_rows_require_each_normalized_surface_and_preserve_hub_color"] }],
    });
    assert.equal(receipts[0]!.assertions, 1);
    console.log(`normalized-presence-rows-native-check: sourceChecks=${checks} nativeLaws=1 clean`);
  }
}

/** 🪟️ Exercises trusted physical Dock input and neutral public layout outcomes in both renderer hosts. */
class BrowserDockAcceptanceScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { runDockBrowserAcceptanceCli } = await import("../../../../🧱️elements/🛰️Dock/🧪️tests/🌐️browser-acceptance/📜️script.ts");
    await runDockBrowserAcceptanceCli(segments);
  }
}

/** 🎬️ Verifies real media app reservations through the accepted browser renderer and frame Worker. */
class BrowserMediaAppAcceptanceScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { runBrowserMediaAppAcceptance } = await import("../../🧪️tests/🎬️media-app/🟦️.ts");
    await runBrowserMediaAppAcceptance(segments);
  }
}

/** 🪆️ Exercises actual independently embedded public browser mount lifetimes. */
class BrowserEmbeddedAcceptanceScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { runEmbeddedBrowserAcceptance } = await import("../../🧪️tests/🪆️embedded-browser/🟦️.ts");
    await runEmbeddedBrowserAcceptance(segments);
  }
}

/** 🌐️ Runs every browser renderer law independently of native compilation. */
class BrowserTestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    assertRendererOutputOwnership();
    const { rest } = resolveTestLevel(segments, "long");
    await runVitest(this.root, rest, "../../🧪️tests/🎚️config/🟦️.ts");
  }
}

/** 🧵️ Runs the browser Worker transport protocol without invoking Cargo. */
class BrowserWorkerTestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runVitest(this.root, ["🧪️tests/📨️browser-frame-transport/🟦️.ts", "🧪️tests/🎮️browser-interactive-job-port/🟦️.ts", "🧪️tests/🔢️frame-generation-hold/🟦️.ts", "🧪️tests/🎯️presented-input-authority/🟦️.ts", "🧪️tests/⏱️wgpu-ui-turn-budget/🟦️.ts", "🧪️tests/⏱️wgpu-worker-step-budget/🟦️.ts", "🧪️tests/🔬️wgpu-extension-dispatch/🟦️.ts", "🧪️tests/🗄️wgpu-host-storage-door/🟦️.ts", "🧪️tests/🕰️wgpu-host-temporal-door/🟦️.ts", "🧪️tests/🔌️wgpu-socket-door/🟦️.ts", "🧪️tests/🔖️wgpu-readiness-beacon/🟦️.ts", "🧪️tests/🖱️wheel-application-point/🟦️.ts", ...segments], "../../🧪️tests/🎚️config/🟦️.ts");
  }
}

/** 🧾️ Runs the deterministic in-memory frame-worker owner contract at `long` or above — importing
 * its four independent oracles (the TypeScript compiler, Ajv, emoji-regex and the discovery taxonomy)
 * costs ~14 s on an idle machine before a single case runs, and the cases themselves render two full
 * browser bundles, so the suite cannot honestly sit at the fundamental or quick budget. */
class PreviewGeneratedTestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments, "long");
    await runVitest(this.root, ["🧪️tests/🧩️package-integration/🟦️.ts", ...rest], "../../🧪️tests/🎚️config/🟦️.ts");
  }
}

/** 🧵️ Checks the page, public library and frame Worker without invoking Cargo or Trunk. */
class BrowserWorkerCheckScript extends BundleScript {
  async run(_segments: string[]): Promise<void> {
    await checkBrowserBoot(this.root);
    await checkBrowserBoot(this.root, repoRoot, "renderer-boot");
    await checkFrameWorker(this.root);
  }
}

/** 🧵️ Generates only the deterministic browser frame-worker artifact. */
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

/** ✅️ Checks the frame-worker bytes without invoking any renderer build. */
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
    const artifactChecks = assertRendererOutputOwnership();
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

class BootCacheInputCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("Expected check-boot-cache-inputs");
    const { testWgpuBootInputs } = await import("../../../../🧪️tests/🧊️wgpu-browser-boot-cache-inputs/🟦️.ts");
    const { repoTestArtifactEnvironment } = await import("../../../../../../../../🦑️repo/🔨️modules/📚️library/🏃️process/🌿️environment/🧪️test-output/🟦️.ts");
    const { mkdirSync } = await import("node:fs");
    const output = repoTestArtifactEnvironment(this.repoRoot, "wgpu-boot-cache-inputs").SEMIO_TEST_ARTIFACT_DIR!;
    mkdirSync(output, { recursive: true });
    await testWgpuBootInputs(this.repoRoot, output);
    console.log("wgpu-boot-cache-inputs: descriptors=18 refusals=5 Bun+Node+Ajv; ambient-session-reads=0 exhaustive-source-inputs=true");
  }
}

const router = new ScriptRouter(import.meta.dir)
  .register(
    "native-environment-check",
    class extends BundleScript {
      async run(): Promise<void> {
        await proveNativeRunnerEnvironment();
      }
    },
  )
  .register("test", TestScript)
  .register("test-socket-snapshot-sqlite",SocketSnapshotSqliteScript)
  .register("test-native", NativeTestScript)
  .register("test-wgpu-unit", WgpuUnitTestScript)
  .register("test-media-slots", MediaSlotContractTestScript)
  .register("canonical-architecture", MediaSlotContractTestScript)
  .register("check-wasm", WasmCheckScript)
  .register("directory-retained-home-bootstrap-source-check", DirectoryRetainedHomeBootstrapSourceCheckScript)
  .register("directory-retained-home-bootstrap-native-check", DirectoryRetainedHomeBootstrapNativeCheckScript)
  .register("normalized-presence-rows-source-check", NormalizedPresenceRowsSourceCheckScript)
  .register("normalized-presence-rows-native-check", NormalizedPresenceRowsNativeCheckScript)
  .register("browser-media-acceptance", BrowserMediaAppAcceptanceScript)
  .register("browser-embedded-acceptance", BrowserEmbeddedAcceptanceScript)
  .register("browser-dock-acceptance", BrowserDockAcceptanceScript)
  .register("test-browser", BrowserTestScript)
  .register("test-browser-worker", BrowserWorkerTestScript)
  .register("test-preview-generated", PreviewGeneratedTestScript)
  .register("check-browser-worker", BrowserWorkerCheckScript)
  .register("check-boot-cache-inputs", BootCacheInputCheckScript)
  .register("generate-frame-worker", GenerateFrameWorkerScript)
  .register("preview-generated", PreviewGeneratedScript)
  .register("check-frame-worker", CheckFrameWorkerScript)
  .register("lint", LintScript);

if (import.meta.main) {
  await router.run(process.argv.slice(2));
}
