#!/usr/bin/env bun
/** @emoji 🎨️ `@semio-tech/framework-renderer-react` task router. */
import { readFileSync } from "node:fs";
import assert from "node:assert/strict";
import { join } from "node:path";
import { fileURLToPath } from "node:url";
import Ajv, { type ValidateFunction } from "ajv";
import { BundleScript, ScriptRouter, runBundleScriptMain, resolveTestLevel, runBunx, runVitest } from "../../../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

const MODULE_SCHEMAS = {
  renderer: "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧬️schema/🔣️.json",
  directory: "🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🔣️.json",
  presence: "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👥️presence/🧬️schema/🔣️.json",
} as const;

/** 🧬️ Compiles one named `$defs` export of an owning `🧬️schema/` module against its draft-07 `$id`. */
function ownedExport(repoRoot: string, scope: keyof typeof MODULE_SCHEMAS, exportId: string): ValidateFunction {
  const doc = JSON.parse(readFileSync(join(repoRoot, MODULE_SCHEMAS[scope]), "utf8")) as { $id: string };
  const compiled = new Ajv({ strict: true, allErrors: true }).addSchema(doc).getSchema(`${doc.$id}#/$defs/${exportId}`);
  if (!compiled) throw new Error(`${scope} schema module publishes no export ${exportId}`);
  return compiled as ValidateFunction;
}

class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { rest } = resolveTestLevel(segments);
    runVitest(this.root, rest, "vitest.config.ts");
  }
}

/** 📍 Executes exact document-scope and newly-created session handoff laws. */
class DocumentOpeningScopeCheckScript extends BundleScript {
  run(segments: string[]): void {
    if (segments.length !== 0) throw new Error("document-opening-scope-check accepts no arguments");
    process.env.SEMIO_TEST_LEVEL = "long";
    runVitest(this.root, ["../../../../🧪️tests/🚪️opening/🟦️.ts", "--silent=false", "--reporter=verbose"], "vitest.config.ts");
  }
}

/** 🔗️ Executes the registered AgentBridge codec and inference-state laws. */
class AgentBridgeCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length !== 0) throw new Error("agent-bridge-check accepts no arguments");
    process.env.SEMIO_TEST_LEVEL = "long";
    process.env.SEMIO_INCLUDE_AGENT_BRIDGE = "1";
    await runVitest(this.root, ["--run", "--silent=false"], "vitest.config.ts");
  }
}

class TypecheckScript extends BundleScript {
  run(segments: string[]): void {
    runBunx(["tsc", "--noEmit", "-p", "tsconfig.json", ...segments], this.root);
  }
}

/** 🎥️ Executes the typed InteractionState tutorial capture, diff, and playback laws. */
class TutorialInteractionCheckScript extends BundleScript {
  run(segments: string[]): void {
    if (segments.length !== 0) throw new Error("tutorial-interaction-check accepts no arguments");
    process.env.SEMIO_TEST_LEVEL = "long";
    runVitest(
      this.root,
      [
        "../../../../🧪️tests/🔬️index/🟦️.ts",
        "--silent=false",
        "--reporter=verbose",
        "--testNamePattern=interaction recording|decodes the bounded actor interaction capture|captures the observed typed interaction|plays full and sparse typed selections|projects tutorial selection playback|records comma-bearing selection|APPLY_TUTORIAL_UI_SNAPSHOT restores",
      ],
      "vitest.config.ts",
    );
  }
}

/** 🌊️ Executes the shared Flow browser runtime and mounted React session-retirement laws. */
class FlowBrowserRuntimeCheckScript extends BundleScript {
  run(segments: string[]): void {
    if (segments.length !== 0) throw new Error("flow-browser-runtime-check accepts no arguments");
    process.env.SEMIO_TEST_LEVEL = "long";
    runVitest(
      this.root,
      [
        "../../../../🧪️tests/🔬️index/🟦️.ts",
        "--silent=false",
        "--reporter=verbose",
        "--testNamePattern=retains one shared Flow browser runtime|retires a graph host unmounted before its open reply",
      ],
      "vitest.config.ts",
    );
  }
}

/** 🌱️ Executes the retained artifact-creation progress, cancellation and accessibility laws. */
class ArtifactCreationProgressCheckScript extends BundleScript {
  run(segments: string[]): void {
    if (segments.length !== 0) throw new Error("artifact-creation-progress-check accepts no arguments");
    process.env.SEMIO_TEST_LEVEL = "long";
    runVitest(
      this.root,
      ["../../../../🧪️tests/🔬️artifact-creation-ready-opening/🟦️.ts", "--silent=false", "--reporter=verbose"],
      "vitest.config.ts",
    );
    process.env.SEMIO_INCLUDE_BACKBONE_WORKER = "1";
    runVitest(
      this.root,
      [join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts"), "--silent=false", "--reporter=verbose", "--testNamePattern=space artifact creation owner"],
      "vitest.config.ts",
    );
  }
}

/** 📇️ Proves the visible retained Home identity/ACK bridge and language-neutral boundaries. */
export function directoryHomeBootstrapOracle(repoRoot: string): number {
  const contractRoot = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/📇️directory-bootstrap");
  const fixture = JSON.parse(readFileSync(join(contractRoot, "🔣️.json"), "utf8")) as {
    receipt: Readonly<Record<string, unknown>>;
    identities: Readonly<Record<"a" | "b", Readonly<{ userId: string; displayName: string }>>>;
    hostile: readonly { readonly id: string; readonly patch: Readonly<Record<string, unknown>> }[];
    labels: Readonly<Record<"en" | "de", readonly string[]>>;
  };
  const receiptExport = ownedExport(repoRoot, "directory", "DirectoryProjectionReceiptV1");
  const identityExport = ownedExport(repoRoot, "directory", "DirectoryHomeIdentityV1");
  const stepExport = ownedExport(repoRoot, "directory", "DirectoryHomeBootstrapStepV1");
  const labelsExport = ownedExport(repoRoot, "directory", "DirectoryHomeBootstrapLabelsV1");
  assert(receiptExport(fixture.receipt), JSON.stringify(receiptExport.errors));
  for (const identity of Object.values(fixture.identities)) assert(identityExport(identity), JSON.stringify(identityExport.errors));
  for (const step of fixture.lifecycle) assert(stepExport(step), JSON.stringify(stepExport.errors));
  assert(labelsExport(fixture.labels), JSON.stringify(labelsExport.errors));
  const receiptKeys = ["schema", "sessionBindingSha256", "authorizationGeneration", "throughSeqInclusive", "receiptSha256"].sort();
  const receipt = (value: unknown): boolean => {
    if (!value || typeof value !== "object" || Array.isArray(value) || JSON.stringify(Object.keys(value).sort()) !== JSON.stringify(receiptKeys)) return false;
    const row = value as Record<string, unknown>;
    return (
      row.schema === "semio.space.home.directory-projection-receipt.v1" &&
      typeof row.sessionBindingSha256 === "string" &&
      /^[0-9a-f]{64}$/u.test(row.sessionBindingSha256) &&
      Number.isSafeInteger(row.authorizationGeneration) &&
      Number(row.authorizationGeneration) > 0 &&
      Number.isSafeInteger(row.throughSeqInclusive) &&
      Number(row.throughSeqInclusive) >= 0 &&
      typeof row.receiptSha256 === "string" &&
      /^[0-9a-f]{64}$/u.test(row.receiptSha256)
    );
  };
  assert(receipt(fixture.receipt));
  for (const row of fixture.hostile) assert.equal(receipt({ ...structuredClone(fixture.receipt), ...row.patch }), false, row.id);
  const owner = readFileSync(join(contractRoot, "🟦️.tsx"), "utf8");
  const shell = readFileSync(join(contractRoot, "../🟦️.tsx"), "utf8");
  const runtime = readFileSync(join(contractRoot, "../../🔌️PluginRuntime/🟦️.tsx"), "utf8");
  assert(owner.includes("await owner.plugin.handleAction") && owner.includes("parseDirectoryProjectionReceiptV1(response.output)"));
  assert(owner.indexOf("await owner.plugin.handleAction") < owner.indexOf('kind: "directory-bootstrap-ack"'));
  assert(owner.indexOf('directoryActionInvocation(owner, "setClient"') < owner.indexOf('kind: "directory-bootstrap-open"'));
  assert(owner.includes("input.identity.userId") && owner.includes("input.identity.displayName") && owner.includes("invocationTerminal(response)"));
  assert(owner.includes("ownsInstance") && owner.includes("if (owner.ownsInstance)") && owner.includes("await beforeAcknowledge?.(owner)"));
  assert(owner.includes('kind: "directory-bootstrap-reject"') && owner.includes('kind: "directory-bootstrap-close"'));
  assert(shell.includes('message.kind === "directory-event-page"') && shell.includes("directoryHomeOwnerRef"));
  assert(shell.includes("openDirectoryHomeOwnerV1") && !shell.includes('kind: "directory-open", baseUrl: resolved.hubBaseUrl'));
  assert(shell.includes("instance: { instanceId: visibleSession.instanceId, viewState: visibleSession.viewState }") && shell.includes("directoryHomeOpeningRef.current.catch"));
  assert(shell.includes("identity: { userId: identity.userId, displayName: identity.displayName }") && shell.includes("await refreshDirectoryHomeRef.current(active)"));
  assert(runtime.includes("directory projection receipt was exposed before typed-operation terminal publication"));
  assert(runtime.includes("terminalOutputs.length === 1") && runtime.includes("output = terminalOutputs[0]!.val"));
  assert.deepEqual(Object.keys(fixture.labels).sort(), ["de", "en"]);
  assert(fixture.labels.en.every((label: string) => label.length > 0) && fixture.labels.de.every((label: string) => label.length > 0));
  assert.deepEqual(Object.keys(fixture.identities).sort(), ["a", "b"]);
  assert(fixture.identities.a.userId !== fixture.identities.b.userId && Object.values(fixture.identities).every((identity) => identity.userId.length > 0 && identity.displayName.length > 0));
  return 29;
}

class DirectoryHomeBootstrapCheckScript extends BundleScript {
  run(segments: string[]): void {
    if (segments.length !== 0) throw new Error("directory-home-bootstrap-check accepts no arguments");
    console.log(`directory-home-bootstrap-oracle: checks=${directoryHomeBootstrapOracle(this.repoRoot)} clean`);
    process.env.SEMIO_TEST_LEVEL = "long";
    runVitest(this.root, ["../../../../🧪️tests/📇️directory-home-bootstrap/🟦️.tsx"], "vitest.config.ts");
    runVitest(this.root, ["../../../../🧱️elements/🔌️PluginRuntime/🟦️.tsx", "--testNamePattern=validates fixed result page authority and preserves document and download effects"], "vitest.config.ts");
  }
}

/** 🎟️ Proves the language-neutral invite transfer machine and its typed browser/worker bridge. */
export function directoryInviteCapabilityOracle(repoRoot: string): number {
  const contractRoot = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🎟️invite-capability");
  const fixture = JSON.parse(readFileSync(join(contractRoot, "🔣️.json"), "utf8")) as {
    states: readonly string[];
    transitions: readonly { readonly from: string; readonly input: string; readonly to: string; readonly discloses: boolean; readonly erases: boolean }[];
    duplicateResult: string;
    shellRoute: Readonly<{ actionId: string; spaceId: string; operationEpoch: number; openingPhase: string; requestKind: string }>;
    disclosureAuthority: readonly string[];
    focusPhases: readonly string[];
    labels: Readonly<Record<"en" | "de", readonly string[]>>;
  };
  const transferExport = ownedExport(repoRoot, "directory", "InviteCapabilityTransferV1");
  assert(transferExport(fixture), JSON.stringify(transferExport.errors));
  assert.deepEqual(fixture.states, ["available", "copying", "failed", "copied", "closed"]);
  assert.deepEqual(
    fixture.transitions.filter((row) => row.discloses).map((row) => [row.from, row.input, row.to]),
    [
      ["available", "request", "copying"],
      ["failed", "request", "copying"],
    ],
  );
  assert(fixture.transitions.filter((row) => row.erases).every((row) => row.to === "copied" || row.to === "closed"));
  assert.equal(fixture.duplicateResult, "reject-without-disclosure");
  assert.deepEqual(fixture.shellRoute, { actionId: "os.directory.open-administration", spaceId: "space-admin-01", operationEpoch: 7, openingPhase: "loading", requestKind: "directory-administration-open" });
  assert.deepEqual(fixture.disclosureAuthority, ["author"]);
  assert.deepEqual(fixture.focusPhases, ["ready", "denied", "stale", "failed", "cancelled"]);
  assert.deepEqual(Object.keys(fixture.labels).sort(), ["de", "en"]);
  assert(Object.values(fixture.labels).every((labels) => labels.length === 3 && labels.every((label) => label.length > 0)));
  const worker = readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts"), "utf8");
  const shell = readFileSync(join(contractRoot, "../🟦️.tsx"), "utf8");
  const pane = readFileSync(join(contractRoot, "../../🛂️SpaceAdministration/🟦️.tsx"), "utf8");
  const request = worker.slice(worker.indexOf("function requestDirectoryAdministrationCapability"), worker.indexOf("function settleDirectoryAdministrationCapability"));
  const settle = worker.slice(worker.indexOf("function settleDirectoryAdministrationCapability"), worker.indexOf("function closeDirectoryAdministration"));
  assert(request.includes('inviteCapabilityStatus = "copying"') && request.includes('kind: "directory-administration-capability"'));
  assert(request.includes("operation.authorPage") && request.indexOf("operation.authorPage") < request.indexOf('kind: "directory-administration-capability"'));
  assert(request.includes("if (!operation.authorPage)") && request.includes("operation.inviteToken = null"));
  assert(settle.includes("if (copied)") && settle.includes("operation.inviteToken = null") && settle.includes('inviteCapabilityStatus = "failed"'));
  assert(worker.includes('kind: "directory-administration-capability-rejected"') && worker.includes('code: "mismatch"') && worker.includes('code: "already-settled"'));
  assert(shell.includes('kind: "directory-administration-capability-result"') && shell.includes(".then((copied)") && shell.includes("if (clipboard === undefined) return false"));
  assert(shell.includes("shellSpaceAdministrationOpening") && shell.includes("spaceAdministrationRef.current") && shell.includes("shellSpaceAdministrationCapabilityAllowed"));
  assert(!shell.includes("console.log(inviteToken)") && !shell.includes("setInviteToken"));
  assert(pane.includes('inviteCapabilityStatus === "copying"') && pane.includes("labels.copyStatus[inviteCapabilityStatus]"));
  return 21;
}

class DirectoryInviteCapabilityCheckScript extends BundleScript {
  run(segments: string[]): void {
    if (segments.length !== 0) throw new Error("directory-invite-capability-check accepts no arguments");
    console.log(`directory-invite-capability-oracle: checks=${directoryInviteCapabilityOracle(this.repoRoot)} clean`);
    process.env.SEMIO_TEST_LEVEL = "long";
    runVitest(this.root, ["../../../../🧪️tests/🏛️space-administration/🟦️.tsx"], "vitest.config.ts");
    process.env.SEMIO_INCLUDE_BACKBONE_WORKER = "1";
    runVitest(this.root, [join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts"), "--testNamePattern=backbone-worker space administration"], "vitest.config.ts");
  }
}

/** 👥️ Proves exact-scope lifecycle keys and worker-verified host-only presence projection. */
export function scopedPresenceOracle(repoRoot: string): number {
  const contractRoot = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/👥️presence-scope");
  const fixture = JSON.parse(readFileSync(join(contractRoot, "🔣️.json"), "utf8")) as {
    documentId: string;
    cases: readonly { readonly scope: { readonly spaceId: string; readonly documentId: string }; readonly runtimeKey: string; readonly surface: string }[];
    close: { readonly clearsRuntimeKey: string; readonly preservesRuntimeKey: string };
    routes: readonly string[];
    rejected: readonly string[];
  };
  const bindingExport = ownedExport(repoRoot, "presence", "ScopedPresenceBindingV1");
  const closeExport = ownedExport(repoRoot, "presence", "ScopedPresenceCloseV1");
  const routeExport = ownedExport(repoRoot, "presence", "PresenceActorRouteV1");
  const rejectionExport = ownedExport(repoRoot, "presence", "ScopedPresenceRejectionV1");
  for (const row of fixture.cases) assert(bindingExport(row), JSON.stringify(bindingExport.errors));
  assert(closeExport(fixture.close), JSON.stringify(closeExport.errors));
  for (const route of fixture.routes) assert(routeExport(route), JSON.stringify(routeExport.errors));
  for (const rejected of fixture.rejected) assert(rejectionExport(rejected), JSON.stringify(rejectionExport.errors));
  const runtimeKey = (scope: { readonly spaceId: string; readonly documentId: string }): string => {
    const spaceBytes = new TextEncoder().encode(scope.spaceId).length;
    const documentBytes = new TextEncoder().encode(scope.documentId).length;
    return `v1:${spaceBytes}:${documentBytes}:${scope.spaceId}${scope.documentId}`;
  };
  assert(fixture.cases.every((row) => row.scope.documentId === fixture.documentId && runtimeKey(row.scope) === row.runtimeKey));
  assert.equal(new Set(fixture.cases.map((row) => row.scope.spaceId)).size, 2);
  assert.deepEqual(
    fixture.routes,
    fixture.cases.map((row) => `actor://${row.runtimeKey}`),
  );
  assert.notEqual(fixture.close.clearsRuntimeKey, fixture.close.preservesRuntimeKey);
  assert.deepEqual(fixture.rejected, ["missing-scope", "mismatched-scope", "missing-surface", "mismatched-surface"]);
  const shell = readFileSync(join(contractRoot, "../🟦️.tsx"), "utf8");
  const hostBootstrap = readFileSync(join(contractRoot, "../🪪️host-bootstrap/🟦️.tsx"), "utf8");
  const projection = readFileSync(join(contractRoot, "🟦️.ts"), "utf8");
  const browser = readFileSync(join(contractRoot, "🌐️browser/🟦️.tsx"), "utf8");
  const worker = readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts"), "utf8");
  const wire = readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🟦️.ts"), "utf8");
  assert(shell.includes("scopedPresencePeersV1") && shell.includes("presencePeersByRuntimeKey"));
  assert(shell.includes("portByRuntimeKey") && shell.includes("operationRuntimeKey") && !shell.includes("portByDocumentId"));
  assert(shell.includes("inferencePortOwnerRef") && shell.includes("owners.length === 1"));
  assert(shell.includes("inferencePortStatusRuntimeKeyV1(inferencePortOwnerRef.current") && hostBootstrap.includes("message.scope.spaceId !== owner.scope.spaceId") && hostBootstrap.includes("message.scope.documentId !== owner.scope.documentId"));
  assert(shell.includes("retainInferencePortOwnerAfterCloseV1(inferenceOwner, runtimeKey)") && worker.includes('documentRuntimeKeyV1({ kind: "hub", ...inferencePort.scope }) === runtimeKey'));
  assert(shell.includes('from "./👥️presence-scope/🟦️.ts"'));
  assert(!shell.includes("presencePeersJson"));
  assert(shell.includes("registerPluginBackboneRoute(runtimeKey, relayPluginBackboneMessage)"));
  assert(shell.includes("documentId: entry.documentId") && shell.includes("spaceId: entry.scope.spaceId"));
  assert(shell.includes("const actorUri = `actor://${runtimeKey}`"));
  assert(worker.includes("presenceCandidate") && worker.includes("presenceAuthority"));
  assert(worker.includes('emitEvent(state, { kind: "presence", peers: [] })'));
  assert(worker.includes("const scope = artifactScope(state)"));
  assert(wire.includes("workerWireScopeV1") && wire.includes('event: { kind: "presence", peers: [] }'));
  assert(projection.includes("peer.surface === message.verifiedSurfaceId"));
  assert(projection.includes('role === "owner" || role === "member"') && projection.includes('role === "viewer"'));
  assert(browser.includes('new Worker(config.workerUrl, { type: "module" })'));
  assert(browser.includes("new MessageChannel()") && browser.includes("for (const row of config.cases)"));
  assert(browser.includes('kind: "close"') && browser.includes('kind: "presenceHeartbeat"'));
  assert(browser.includes('data-shell-host="scoped-presence"'));
  return 29;
}

class ScopedPresenceCheckScript extends BundleScript {
  run(segments: string[]): void {
    if (segments.length !== 0) throw new Error("scoped-presence-check accepts no arguments");
    console.log(`scoped-presence-oracle: checks=${scopedPresenceOracle(this.repoRoot)} clean`);
    process.env.SEMIO_TEST_LEVEL = "long";
    runVitest(this.root, ["../../../../🧪️tests/👥️scoped-presence/🟦️.tsx"], "vitest.config.ts");
    process.env.SEMIO_INCLUDE_BACKBONE_WORKER = "1";
    runVitest(this.root, [join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts"), "--testNamePattern=backbone-worker scope-safe presence"], "vitest.config.ts");
  }
}

//#region 🔖️LintScript
const REGION_BALANCE_FILES = ["🟦️.tsx"] as const;

/** 🧭️Counts unmatched `//#region` / `//#endregion` markers per file — a typo'd region silently corrupts the file's canonical structure. */
function collectRegionBalanceViolations(root: string): string[] {
  const violations: string[] = [];
  for (const name of REGION_BALANCE_FILES) {
    const text = readFileSync(join(root, name), "utf8");
    const opens = (text.match(/#region\b/g) ?? []).length;
    const closes = (text.match(/#endregion\b/g) ?? []).length;
    if (opens !== closes) {
      violations.push(`${name}: ${opens} #region marker(s) vs ${closes} #endregion marker(s)`);
    }
  }
  return violations;
}

/** 🧭️Every host registered in `COMPONENT_SCENE_HOSTS` must have exactly one `export function XxxHost(...: ComponentSceneHostProps)` in `🟦️.tsx` — the contract the host registry table dispatches against. */
function collectHostSignatureViolations(root: string): string[] {
  const violations: string[] = [];
  const text = readFileSync(join(root, "🟦️.tsx"), "utf8");
  const registryNames = [...text.matchAll(/lazyHost\(\(\) => Promise\.resolve\(\{ ([A-Z][A-Za-z0-9]*Host) \}\), "\1"\)/g)].map((m) => m[1]!);
  const hostExportCounts = new Map<string, number>();
  for (const m of text.matchAll(/^export function ([A-Z][A-Za-z0-9]*Host)\([^)]*: ComponentSceneHostProps\)/gm)) {
    hostExportCounts.set(m[1]!, (hostExportCounts.get(m[1]!) ?? 0) + 1);
  }
  for (const name of registryNames) {
    const count = hostExportCounts.get(name) ?? 0;
    if (count === 0) {
      violations.push(`🟦️.tsx: no exported component matching ${name}(...: ComponentSceneHostProps)`);
    } else if (count > 1) {
      violations.push(`🟦️.tsx: multiple ${name} exports matching the host contract`);
    }
  }
  return violations;
}

class LintScript extends BundleScript {
  run(_segments: string[]): void {
    const violations = [...collectRegionBalanceViolations(this.root), ...collectHostSignatureViolations(this.root)];
    if (violations.length === 0) {
      console.log("framework-renderer-react: region/host-contract lint passed");
      return;
    }
    console.error(`framework-renderer-react: found ${violations.length} lint violation(s):`);
    for (const v of violations) console.error(`  ${v}`);
    process.exit(1);
  }
}
//#endregion 🔖️LintScript

const router = new ScriptRouter(fileURLToPath(new URL(".", import.meta.url)))
  .register("test", TestScript)
  .register("lint", LintScript)
  .register("typecheck", TypecheckScript)
  .register("tutorial-interaction-check", TutorialInteractionCheckScript)
  .register("flow-browser-runtime-check", FlowBrowserRuntimeCheckScript)
  .register("artifact-creation-progress-check", ArtifactCreationProgressCheckScript)
  .register("agent-bridge-check", AgentBridgeCheckScript)
  .register("directory-home-bootstrap-check", DirectoryHomeBootstrapCheckScript)
  .register("directory-invite-capability-check", DirectoryInviteCapabilityCheckScript)
  .register("scoped-presence-check", ScopedPresenceCheckScript)
  .register("document-opening-scope-check", DocumentOpeningScopeCheckScript);

await runBundleScriptMain(router, import.meta.url);
