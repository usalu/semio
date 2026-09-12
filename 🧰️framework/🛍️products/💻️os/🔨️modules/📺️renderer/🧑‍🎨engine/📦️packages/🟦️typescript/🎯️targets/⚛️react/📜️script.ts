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
  interaction: "🧰️framework/🔨️modules/🕹️interaction/🧬️schema/🔣️.json",
  directory: "🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🔣️.json",
  presence: "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👥️presence/🧬️schema/🔣️.json",
} as const;

/** 🧬️ A draft-07 validator that treats the annotation-only `discriminator` keyword as data. */
const ownedAjv = (): Ajv => new Ajv({ strict: true, allErrors: true }).addKeyword("x-semio-note");

/** 🧬️ Compiles one named `$defs` export of an owning `🧬️schema/` module against its draft-07 `$id`. */
function ownedExport(repoRoot: string, scope: keyof typeof MODULE_SCHEMAS, exportId: string): ValidateFunction {
  const doc = JSON.parse(readFileSync(join(repoRoot, MODULE_SCHEMAS[scope]), "utf8")) as { $id: string };
  const compiled = ownedAjv().addSchema(doc).getSchema(`${doc.$id}#/$defs/${exportId}`);
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
        "../../../../🧪️tests/🔬️engine-contract/🟦️.ts",
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
        "../../../../🧪️tests/🔬️engine-contract/🟦️.ts",
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
  const shellHostRoot = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost");
  const contractRoot = join(shellHostRoot, "📇️directory-bootstrap");
  const fixture = JSON.parse(readFileSync(join(shellHostRoot, "🧫️fixtures/📇️directory-bootstrap/🔣️.json"), "utf8")) as {
    receipt: Readonly<Record<string, unknown>>;
    identities: Readonly<Record<"a" | "b", Readonly<{ userId: string; displayName: string }>>>;
    hostile: readonly { readonly id: string; readonly patch: Readonly<Record<string, unknown>> }[];
    lifecycle: readonly { readonly id: string; readonly action: string; readonly expected: string }[];
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
  const shellHostRoot = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost");
  const contractRoot = join(shellHostRoot, "👥️presence-scope");
  const fixture = JSON.parse(readFileSync(join(shellHostRoot, "🧫️fixtures/👥️presence-scope/🔣️.json"), "utf8")) as {
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
  assert(shell.includes("entry.plugin.bindDocumentPort(entry.session.instanceId") && !shell.includes("registerPluginBackboneRoute("));
  assert(shell.includes("documentId: entry.documentId") && shell.includes("spaceId: entry.scope.spaceId"));
  assert(shell.includes('message: { kind: "documentBackbone", message }') && shell.includes("receiveDocumentBackbone(runtimeKey, entry, event.message)"));
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

/** 🖱️ Re-derives the `World3dHost` gesture→action law from `🖱️pointer-gestures.json` WITHOUT React, three
 * or the DOM — a second, independent implementation of the same fixture the mounted jsdom suite
 * (`🧪️tests/🖱️world3d-interaction`) plays through the real component, so neither side can drift alone. Also
 * pins the source call sites, because a mounted suite driven by a mock still passes when a gesture is
 * quietly re-pointed at a different verb. */
export function world3dPointerGestureOracle(repoRoot: string): number {
  const hostRoot = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost");
  type Target = { readonly granularity: string; readonly id: string };
  type Gesture = {
    readonly id: string;
    readonly kind: string;
    readonly instanceId?: string;
    readonly ids?: readonly string[];
    readonly granularity?: string;
    readonly modifiers?: { readonly shiftKey?: boolean; readonly ctrlKey?: boolean; readonly metaKey?: boolean };
    readonly steps?: readonly { readonly position: readonly number[]; readonly target: readonly number[]; readonly zoom: number }[];
    readonly path?: readonly { readonly x: number; readonly y: number }[];
    readonly expect: { readonly action: string; readonly domainId?: string; readonly channel?: string; readonly merge?: string; readonly method?: string; readonly targets?: readonly Target[]; readonly dispatches?: number; readonly camera?: Record<string, unknown> };
  };
  const fixture = JSON.parse(readFileSync(join(hostRoot, "🧫️fixtures/🖱️pointer-gestures.json"), "utf8")) as {
    scene: { surfaceId: string; controllerId: string; windowInstanceId: string; domainId: string; domainGranularityId: string; instances: readonly { id: string; interactionId: string }[] };
    viewport: { width: number; height: number };
    cameraDebounceMs: number;
    marqueeDragThresholdPx: number;
    gestures: readonly Gesture[];
    wiring: { selectionArgsBuilder: string; hoverArgsBuilder: string; cameraArgsBuilder: string; targetsBuilder: string; selectionTargetsAreASet: string; mergeVocabularyFixture: string; mergeIsNotTranslated: string; deletedMergeBuilders: readonly string[]; deadVerbs: readonly string[] };
  };
  const scene = fixture.scene;
  /** 🖱️ The merge word a modifier set resolves to — the oracle's own copy of `marqueeModeFromModifiers`,
   * written from the documented law rather than imported from the host. It emits the SCHEMA words
   * (`🕹️interaction/🧫️fixtures/🎯️merge-modes.json`): ticket 26/09/09/PROCEDURAL-3D-END-TO-END deleted
   * the host's private `add`/`remove`/`toggle` translation, which the framework rejects outright. */
  const mergeWord = (modifiers: { readonly shiftKey?: boolean; readonly ctrlKey?: boolean; readonly metaKey?: boolean } = {}): string => {
    const shift = modifiers.shiftKey === true;
    const control = modifiers.ctrlKey === true || modifiers.metaKey === true;
    if (shift && control) return "invertive";
    if (shift) return "additive";
    if (control) return "subtractive";
    return "replace";
  };
  /** 🎯️ The oracle's own copy of `interactionTargetsForInstances` — order-preserving dedup of the
   * topology ids a set of rendered instance ids stands for. */
  const topologyTargets = (ids: readonly string[]): readonly string[] => {
    const seen = new Set<string>();
    const targets: string[] = [];
    for (const id of ids) {
      const target = scene.instances.find((instance) => instance.id === id)?.interactionId ?? id;
      if (seen.has(target)) continue;
      seen.add(target);
      targets.push(target);
    }
    return targets;
  };
  let checks = 0;
  const check = (condition: boolean, message: string): void => {
    assert(condition, message);
    checks += 1;
  };
  check(scene.instances.length > 1 && new Set(scene.instances.map((instance) => instance.interactionId)).size === 1, "the scene must render several instances of ONE topology target, or dedup proves nothing");
  check(fixture.cameraDebounceMs > 0 && fixture.marqueeDragThresholdPx > 0, "camera debounce and marquee threshold must be positive");
  for (const gesture of fixture.gestures) {
    const expected = gesture.expect;
    if (gesture.kind === "instance-pick") {
      const instance = scene.instances.find((entry) => entry.id === gesture.instanceId);
      assert(instance, `gesture ${gesture.id} names no scene instance`);
      check(expected.action === "interactionSelect", `${gesture.id}: a pick is an interactionSelect`);
      check(expected.method === "pick" && expected.domainId === scene.domainId, `${gesture.id}: a pick carries method=pick on the scene domain`);
      check(expected.merge === mergeWord(gesture.modifiers), `${gesture.id}: merge must be ${mergeWord(gesture.modifiers)}`);
      assert.deepEqual(expected.targets, [{ granularity: "object", id: instance.interactionId }], `${gesture.id}: a pick names the TOPOLOGY id at object granularity`);
      checks += 1;
    } else if (gesture.kind === "selection-args") {
      const ids = gesture.ids ?? [];
      check(new Set(ids).size < ids.length, `${gesture.id}: the builder law must be handed a REPEATED id or it proves nothing`);
      check(expected.action === "interactionSelect" && expected.method === "pick", `${gesture.id}: the args builder builds a pick select`);
      assert.deepEqual(
        expected.targets,
        [...new Set(ids)].map((id) => ({ granularity: gesture.granularity ?? "object", id })),
        `${gesture.id}: a selection is a SET of topology ids — the args builder emits each id at most once, in first-seen order`,
      );
      checks += 1;
    } else if (gesture.kind === "instance-hover") {
      const instance = scene.instances.find((entry) => entry.id === gesture.instanceId);
      assert(instance, `gesture ${gesture.id} names no scene instance`);
      check(expected.action === "interactionHover" && expected.channel === "pointer", `${gesture.id}: hover travels on the pointer channel`);
      assert.deepEqual(expected.targets, [{ granularity: scene.domainGranularityId, id: instance.interactionId }], `${gesture.id}: hover reports the SCENE granularity, not the pick granularity`);
      checks += 1;
    } else if (gesture.kind === "background-click") {
      check(expected.action === "interactionSelect" && expected.merge === mergeWord(gesture.modifiers), `${gesture.id}: an empty click still dispatches a select`);
      assert.deepEqual(expected.targets, [], `${gesture.id}: an empty click clears with an EMPTY target list`);
      checks += 1;
    } else if (gesture.kind === "camera-gesture") {
      const steps = gesture.steps ?? [];
      check(steps.length > 1 && expected.dispatches === 1, `${gesture.id}: several steps must debounce into exactly one setCamera`);
      assert.deepEqual(expected.camera, { ...steps[steps.length - 1] }, `${gesture.id}: the dispatched pose is the LAST step, never an intermediate one`);
      checks += 1;
    } else if (gesture.kind === "marquee") {
      const path = gesture.path ?? [];
      const span = Math.hypot(path[path.length - 1]!.x - path[0]!.x, path[path.length - 1]!.y - path[0]!.y);
      check(path.length > 1 && span > fixture.marqueeDragThresholdPx, `${gesture.id}: the path must exceed the drag threshold or it is a click`);
      check(expected.merge === "replace", `${gesture.id}: a marquee release always replaces`);
      assert.deepEqual(
        expected.targets,
        topologyTargets(scene.instances.map((instance) => instance.id)).map((id) => ({ granularity: "object", id })),
        `${gesture.id}: a full-viewport marquee collapses every rendered instance onto its deduplicated topology targets`,
      );
      checks += 1;
    } else {
      assert.fail(`gesture ${gesture.id} has an unknown kind ${gesture.kind}`);
    }
  }
  const host = readFileSync(join(hostRoot, "🟦️.tsx"), "utf8");
  check(host.includes(`dispatch("interactionSelect", ${fixture.wiring.selectionArgsBuilder}(interactionDomainId, "object", [record?.interactionId ?? id], merge))`), "the instance pick must still build its args through the selection builder");
  // 🏁️ Hover travels on `dispatchSettled`, `dispatch`'s awaitable twin: the coalescing dispatcher bounds the
  // lane at ONE outstanding round trip only when it can see the promise `onAction` settles on, and `dispatch`
  // discards it (ticket 26/09/02 wave B33 §3 — 72 hover turns enqueued by one 70-move storm, 11 settled).
  check(host.includes(`return dispatchSettled("interactionHover", ${fixture.wiring.hoverArgsBuilder}(interactionDomainId, interactionGranularity, target))`), "the instance hover must still build its args through the hover builder AND hand the coalescing gate its awaitable");
  check(host.includes(`dispatch("interactionSelect", ${fixture.wiring.selectionArgsBuilder}(interactionDomainId, interactionGranularity, [], merge))`), "the empty click must still clear through the selection builder");
  check(host.includes(`dispatch("setCamera", ${fixture.wiring.cameraArgsBuilder}(`), "the debounced camera sync must still build its args through the camera builder");
  check(host.includes(`const domainTargets = ${fixture.wiring.targetsBuilder}(instancesRef.current, preview.mergedInstanceIds)`), "the marquee release must still resolve instances onto topology targets");
  check(host.includes(`dispatch("interactionSelect", ${fixture.wiring.selectionArgsBuilder}(interactionDomainId, "object", domainTargets, "replace"))`), "the marquee release must still replace on the scene domain");
  check(host.includes(fixture.wiring.selectionTargetsAreASet), "the selection args builder must keep collapsing repeated topology ids — the host never emits a duplicate target");
  check(host.includes('return { domainId, targets: JSON.stringify(targets), merge, method: "pick" };'), "the selection args shape is the wire contract and must stay verbatim");
  check(host.includes('return { domainId, channel: "pointer", targets: JSON.stringify(targets) };'), "the hover args shape is the wire contract and must stay verbatim");
  for (const verb of fixture.wiring.deadVerbs) check(!host.includes(`dispatch("${verb}"`), `${verb} is framework-owned now and must not be dispatched from a world host`);
  const vocabulary = (JSON.parse(readFileSync(join(repoRoot, fixture.wiring.mergeVocabularyFixture), "utf8") ) as { vocabulary: readonly string[]; deletedWords: readonly string[] });
  for (const gesture of fixture.gestures) {
    if (gesture.expect.merge === undefined) continue;
    check(vocabulary.vocabulary.includes(gesture.expect.merge), `${gesture.id}: merge "${gesture.expect.merge}" is outside the ONE schema vocabulary ${vocabulary.vocabulary.join("/")}`);
  }
  check(host.includes(fixture.wiring.mergeIsNotTranslated), "the selection args builder must take a typed MergeMode — a host that re-types it as a bare string can invent a private vocabulary again");
  for (const builder of fixture.wiring.deletedMergeBuilders) check(!host.includes(`function ${builder}(`), `${builder} translated the resolved MergeMode into a private word and is deleted — no adapter`);
  for (const word of vocabulary.deletedWords) check(!host.includes(`return "${word}";`), `the deleted merge word "${word}" must not be produced anywhere in the host`);
  return checks;
}

/** 🧮️ Re-derives the framework selection SET law from `🔌️plugin/🧫️fixtures/🕹️selection-set.json` without
 * Rust — the language-neutral twin of the reserved-job laws in `🔬️plugin-runtime-plugin-builder-contract`.
 * A selection is a set of topology ids: `Select` is idempotent per id whatever the merge, and however
 * many mirror domains (`vortex`) the leftover projection flattens. Also pins the ONE projection the
 * Rust side must keep sharing between its two flatten sites — the live 2026-09-12 defect was exactly
 * two copies of that fold, one deduplicating and one not. */
export function interactionSelectionSetOracle(repoRoot: string): number {
  const pluginRoot = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin");
  type Case = { readonly id: string; readonly why: string; readonly seed: readonly string[]; readonly merge: string; readonly targets: readonly string[]; readonly selectedIds: readonly string[] };
  const fixture = JSON.parse(readFileSync(join(pluginRoot, "🧫️fixtures/🕹️selection-set.json"), "utf8")) as {
    domain: string;
    granularity: string;
    mirrorDomain: string;
    cases: readonly Case[];
    wiring: { file: string; projection: string; callSites: readonly string[]; lockedFromSelectedIds: string };
  };
  let checks = 0;
  const check = (condition: boolean, message: string): void => {
    assert(condition, message);
    checks += 1;
  };
  /** 🧮️ The oracle's own `next_selection` + leftover-flatten, written from the documented law: a
   * replace keeps the batch's distinct ids, an additive appends only genuinely new ones, and the
   * publication flattens the app domain plus its `vortex` mirror into ONE set. */
  const publish = (seed: readonly string[], merge: string, targets: readonly string[]): readonly string[] => {
    const domain = merge === "replace" ? [...new Set(targets)] : [...seed, ...targets.filter((id) => !seed.includes(id))];
    const mirrored = [...domain, ...domain];
    const published: string[] = [];
    for (const id of mirrored) if (!published.includes(id)) published.push(id);
    return published;
  };
  check(fixture.cases.length > 0 && fixture.cases.some((entry) => entry.merge === "replace") && fixture.cases.some((entry) => entry.merge === "additive"), "the set law must cover BOTH the replace and the additive merge");
  check(fixture.cases.some((entry) => new Set(entry.targets).size < entry.targets.length), "at least one case must repeat a target inside one batch or the batch half of the law proves nothing");
  check(fixture.cases.some((entry) => entry.merge === "additive" && entry.targets.every((id) => entry.seed.includes(id))), "at least one case must re-add an already selected id or idempotence proves nothing");
  for (const entry of fixture.cases) {
    assert.deepEqual(publish(entry.seed, entry.merge, entry.targets), [...entry.selectedIds], `${entry.id}: ${entry.why}`);
    checks += 1;
    check(new Set(entry.selectedIds).size === entry.selectedIds.length, `${entry.id}: selectedIds must name every id at most once`);
  }
  const source = readFileSync(join(repoRoot, fixture.wiring.file), "utf8");
  check(source.includes(fixture.wiring.projection), "the ONE shared leftover-selection projection must still exist");
  for (const site of fixture.wiring.callSites) check(source.includes(site), `every flatten site must go through the shared projection — missing ${site.trim()}`);
  check(source.includes(fixture.wiring.lockedFromSelectedIds), "`locked` must be derived from the deduplicated selectedIds, never from a second flatten");
  return checks;
}

/** 🎯️ Re-derives the ONE selection merge vocabulary from `🕹️interaction/🧫️fixtures/🎯️merge-modes.json`
 * without Rust: every word is validated against the OWNED schema export (`MergeMode` of
 * `🕹️interaction/🧬️schema/🔣️.json`) through ajv — a third-party validator, so the vocabulary is proved
 * against the schema by something other than our own matcher — every deleted word is proved to FAIL
 * that same validator, the per-mode set algebra (including `range` and its documented degradation on
 * an unordered domain) is re-implemented from the fixture's written law, and the source anchors of all
 * four implementations are pinned. Live defect 26/09/09/PROCEDURAL-3D-END-TO-END: two vocabularies in
 * one tree made every modifier-click on a domain-bound world scene fault. */
export function selectionMergeVocabularyOracle(repoRoot: string): number {
  type Row = { readonly id: string; readonly modifiers: { readonly shiftKey?: boolean; readonly ctrlKey?: boolean; readonly metaKey?: boolean }; readonly pick: string; readonly componentPick: string };
  type Case = { readonly id: string; readonly why: string; readonly orderedTopology?: readonly string[]; readonly seed: readonly string[]; readonly anchor: string | null; readonly merge: string; readonly targets: readonly string[]; readonly selectedIds: readonly string[]; readonly anchorId: string };
  const fixturePath = "🧰️framework/🔨️modules/🕹️interaction/🧫️fixtures/🎯️merge-modes.json";
  const fixture = JSON.parse(readFileSync(join(repoRoot, fixturePath), "utf8")) as {
    schema: { file: string; export: string; graphql: string };
    vocabulary: readonly string[];
    deletedWords: readonly string[];
    unknownMergeFault: string;
    modifierPolicy: { rows: readonly Row[]; neverEmitted: readonly string[] };
    topology: { domain: string; granularity: string; ordered: readonly string[] };
    range: { decision: string; law: string; unorderedDomains: string };
    cases: readonly Case[];
    wiring: {
      codec: { file: string; toWire: string; fromWire: string };
      domainPath: { file: string; decoder: string; worldPath: string };
      host: { file: string; selectionArgs: string; componentPick: string; deletedBuilders: readonly string[] };
    };
  };
  let checks = 0;
  const check = (condition: boolean, message: string): void => {
    assert(condition, message);
    checks += 1;
  };

  // 🧬️ Schema-first: the words are the schema's, proved by ajv rather than by a second hand-written list.
  const validate = ownedExport(repoRoot, "interaction", fixture.schema.export);
  for (const word of fixture.vocabulary) check(validate(word) === true, `"${word}" is in the fixture vocabulary but the owned ${fixture.schema.export} schema export rejects it`);
  for (const word of fixture.deletedWords) check(validate(word) !== true, `the deleted word "${word}" must NOT validate against the owned ${fixture.schema.export} schema export`);
  const schemaEnum = ((JSON.parse(readFileSync(join(repoRoot, fixture.schema.file), "utf8")) as { $defs: Record<string, { enum?: readonly string[] }> }).$defs[fixture.schema.export]?.enum ?? []) as readonly string[];
  assert.deepEqual([...fixture.vocabulary], [...schemaEnum], "the fixture vocabulary IS the schema enum, in the schema's own order — a second, hand-maintained list is exactly the defect");
  checks += 1;
  const graphql = readFileSync(join(repoRoot, fixture.schema.graphql), "utf8");
  for (const word of fixture.vocabulary) check(graphql.includes(`\n  ${word.toUpperCase()}\n`), `the GraphQL schema leaf must declare ${word.toUpperCase()} — the two schema spellings must not drift`);

  // 🖱️ The modifier→merge map: the oracle's own copy of `marqueeModeFromModifiers` + `componentPickMergeMode`.
  const resolve = (modifiers: Row["modifiers"]): string => {
    const shift = modifiers.shiftKey === true;
    const control = modifiers.ctrlKey === true || modifiers.metaKey === true;
    if (shift && control) return "invertive";
    if (shift) return "additive";
    if (control) return "subtractive";
    return "replace";
  };
  check(fixture.modifierPolicy.rows.length >= 4, "the modifier policy must cover the bare click and every multi-select chord");
  for (const row of fixture.modifierPolicy.rows) {
    check(row.pick === resolve(row.modifiers), `${row.id}: a whole-instance pick resolves to ${resolve(row.modifiers)}`);
    check(row.componentPick === (row.pick === "replace" ? "invertive" : row.pick), `${row.id}: a component pick differs from an instance pick ONLY in that a bare click toggles`);
    check(fixture.vocabulary.includes(row.pick) && fixture.vocabulary.includes(row.componentPick), `${row.id}: both resolved modes must be schema words`);
    for (const never of fixture.modifierPolicy.neverEmitted) check(row.pick !== never && row.componentPick !== never, `${row.id}: a world viewport has no ordered topology, so it must never resolve to "${never}"`);
  }

  // 🧮️ The per-mode set algebra, re-implemented from the fixture's written law (not imported).
  const merge = (entry: Case): { readonly ids: readonly string[]; readonly anchorId: string } => {
    const ordered = entry.orderedTopology ?? fixture.topology.ordered;
    const last = entry.targets[entry.targets.length - 1]!;
    if (entry.merge === "range") {
      const anchor = entry.anchor ?? entry.seed[entry.seed.length - 1] ?? last;
      const from = ordered.indexOf(anchor);
      const to = ordered.indexOf(last);
      if (from < 0 || to < 0) return { ids: [last], anchorId: last };
      return { ids: ordered.slice(Math.min(from, to), Math.max(from, to) + 1), anchorId: anchor };
    }
    const distinct = [...new Set(entry.targets)];
    if (entry.merge === "replace") return { ids: distinct, anchorId: last };
    if (entry.merge === "additive") return { ids: [...entry.seed, ...distinct.filter((id) => !entry.seed.includes(id))], anchorId: last };
    if (entry.merge === "subtractive") return { ids: entry.seed.filter((id) => !distinct.includes(id)), anchorId: last };
    const ids = [...entry.seed];
    for (const id of distinct) {
      const at = ids.indexOf(id);
      if (at >= 0) ids.splice(at, 1);
      else ids.push(id);
    }
    return { ids, anchorId: last };
  };
  for (const word of fixture.vocabulary) check(fixture.cases.some((entry) => entry.merge === word), `no case exercises the merge "${word}" — every word of the vocabulary must carry a law`);
  check(fixture.cases.some((entry) => (entry.orderedTopology ?? []).length === 0 && entry.merge === "range"), "the range decision is only pinned if a case runs it against an UNORDERED domain");
  for (const entry of fixture.cases) {
    const produced = merge(entry);
    assert.deepEqual([...produced.ids], [...entry.selectedIds], `${entry.id}: ${entry.why}`);
    checks += 1;
    check(produced.anchorId === entry.anchorId, `${entry.id}: the published anchor must be ${entry.anchorId}`);
    check(new Set(entry.selectedIds).size === entry.selectedIds.length, `${entry.id}: a selection is a SET`);
  }

  // 📌️ Source anchors — one decoder, one world path, one host builder, and no private spelling left.
  const codec = readFileSync(join(repoRoot, fixture.wiring.codec.file), "utf8");
  check(codec.includes(fixture.wiring.codec.toWire), "the ONE wire-label encoder must still exist next to the MergeMode type");
  check(codec.includes(fixture.wiring.codec.fromWire), "the ONE wire-label decoder must still exist next to the MergeMode type");
  const plugin = readFileSync(join(repoRoot, fixture.wiring.domainPath.file), "utf8");
  check(plugin.includes(fixture.wiring.domainPath.decoder), "the domain path must decode through the shared codec, never through its own match arms");
  check(plugin.includes(fixture.wiring.domainPath.worldPath), "the non-domain world path must take a decoded MergeMode, so a private word is unrepresentable");
  const host = readFileSync(join(repoRoot, fixture.wiring.host.file), "utf8");
  check(host.includes(fixture.wiring.host.selectionArgs), "the host's selection args builder must take a typed MergeMode");
  check(host.includes(fixture.wiring.host.componentPick), "the component pick's one deviation must be a named MergeMode→MergeMode function, not a word table");
  for (const builder of fixture.wiring.host.deletedBuilders) check(!host.includes(`function ${builder}(`), `${builder} is deleted — greenfield, no adapter between two vocabularies`);
  for (const word of fixture.deletedWords) {
    check(!plugin.includes(`"${word}" =>`), `the plugin must not match the deleted merge word "${word}"`);
    check(!host.includes(`return "${word}";`), `the host must not produce the deleted merge word "${word}"`);
  }
  check(fixture.unknownMergeFault.includes("unknown merge"), "the fixture must name the fault an out-of-vocabulary merge raises");
  return checks;
}

/** 🖱️ Executes the mounted `World3dHost` gesture laws next to their language-neutral Node twin. */
/** 🔀️ Executes the surface-role/mode switching laws over the language-neutral switching fixture. */
class SurfaceSwitchCheckScript extends BundleScript {
  run(segments: string[]): void {
    if (segments.length !== 0) throw new Error("surface-switch-check accepts no arguments");
    process.env.SEMIO_TEST_LEVEL = "long";
    runVitest(this.root, ["../../../../🧪️tests/🔀️surface-switch/🟦️.ts", "--silent=false", "--reporter=verbose"], "vitest.config.ts");
  }
}

/** 📌️ Runs the view-state carriage laws: contributions are not a view-state field, and `panelJson`
 * is the one long field, bounded at the schema capacity. */
class ViewStateCarriageCheckScript extends BundleScript {
  run(segments: string[]): void {
    if (segments.length !== 0) throw new Error("view-state-carriage-check accepts no arguments");
    process.env.SEMIO_TEST_LEVEL = "long";
    runVitest(this.root, ["../../../../🧪️tests/📌️view-state-carriage/🟦️.ts", "--silent=false", "--reporter=verbose"], "vitest.config.ts");
  }
}

class World3dInteractionCheckScript extends BundleScript {
  run(segments: string[]): void {
    if (segments.length !== 0) throw new Error("world3d-interaction-check accepts no arguments");
    console.log(`world3d-pointer-gesture-oracle: checks=${world3dPointerGestureOracle(this.repoRoot)} clean`);
    console.log(`interaction-selection-set-oracle: checks=${interactionSelectionSetOracle(this.repoRoot)} clean`);
    console.log(`selection-merge-vocabulary-oracle: checks=${selectionMergeVocabularyOracle(this.repoRoot)} clean`);
    process.env.SEMIO_TEST_LEVEL = "long";
    runVitest(this.root, ["../../../../🧪️tests/🖱️world3d-interaction/🟦️.tsx", "--silent=false", "--reporter=verbose"], "vitest.config.ts");
  }
}

//#region 🪪️SurfaceHostRetentionOracle
type RetentionBody = { readonly key: string; readonly component: string; readonly children?: readonly RetentionBody[] };

/** 🪪️ Re-derives the retained-surface law from `🖱️ui/🧬️contract/🧫️fixtures/🪪️surface-host-retention.json`
 * with no React, no jsdom and none of our renderer code — a framework-free twin of the vitest half.
 *
 * It re-implements `builtNodeToSnapshot`'s pre-order DFS numbering and the sibling-key rule from the
 * fixture's own written law, then runs a minimal reconciler over the fixture's four refreshes TWICE:
 * once keyed on the authored key (the rule now shipped) and once keyed on the minted id (the rule
 * that shipped before). The first must mount the surface exactly once; the second must mount it more
 * than once — so the twin proves the law AND witnesses the defect it replaces, on the same data.
 * The fixture document itself is validated by ajv, a third-party validator, rather than by the shape
 * this function happens to read.
 *
 * Live defect 26/09/09/PROCEDURAL-3D-END-TO-END: `window:procedural-main` grew four port rows on an
 * eval settle, the node-graph surface renumbered 30 → 34, and React rebuilt the flow host — a second
 * wasm session, a second canvas and a second wasm-side surface per boot. */
export function surfaceHostRetentionOracle(repoRoot: string): number {
  const fixturePath = "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🪪️surface-host-retention.json";
  const fixture = JSON.parse(readFileSync(join(repoRoot, fixturePath), "utf8")) as {
    surface: string;
    surfaceKey: string;
    canvasKey: string;
    expected: { surfaceHostMounts: number; surfaceAttaches: number; surfaceDomId: string; canvasDomId: string; surfaceReactKeyPath: readonly string[]; surfaceNodeIdByRefresh: readonly number[]; bodyNodeCountByRefresh: readonly number[] };
    siblingKeyCases: readonly { name: string; siblings: readonly { id: number; key: string }[]; expected: readonly string[] }[];
    refreshes: readonly { name: string; statusJson: string; body: RetentionBody }[];
  };
  let checks = 0;
  const check = (condition: boolean, message: string): void => {
    assert(condition, message);
    checks += 1;
  };

  // 🧬️ Third-party structural validation of the fixture document before a single law is read off it.
  const shape = {
    type: "object",
    $defs: {
      body: {
        type: "object",
        required: ["key", "component"],
        properties: {
          key: { type: "string", minLength: 1 },
          component: { enum: ["container", "tree", "treeSection", "treeItem", "surface"] },
          children: { type: "array", items: { $ref: "#/$defs/body" } },
        },
      },
    },
    required: ["surface", "surfaceKey", "canvasKey", "expected", "siblingKeyCases", "refreshes"],
    properties: {
      surface: { type: "string", minLength: 1 },
      surfaceKey: { type: "string", minLength: 1 },
      canvasKey: { type: "string", minLength: 1 },
      expected: {
        type: "object",
        required: ["surfaceHostMounts", "surfaceAttaches", "surfaceDomId", "canvasDomId", "surfaceReactKeyPath", "surfaceNodeIdByRefresh", "bodyNodeCountByRefresh"],
        properties: {
          surfaceHostMounts: { type: "integer", const: 1 },
          surfaceAttaches: { type: "integer", const: 1 },
          surfaceDomId: { type: "string" },
          canvasDomId: { type: "string" },
          surfaceReactKeyPath: { type: "array", minItems: 1, items: { type: "string", pattern: "^k:" } },
          surfaceNodeIdByRefresh: { type: "array", minItems: 2, items: { type: "integer", minimum: 1 } },
          bodyNodeCountByRefresh: { type: "array", minItems: 2, items: { type: "integer", minimum: 1 } },
        },
      },
      siblingKeyCases: {
        type: "array",
        minItems: 3,
        items: {
          type: "object",
          required: ["name", "why", "siblings", "expected"],
          properties: {
            name: { type: "string", minLength: 1 },
            why: { type: "string", minLength: 1 },
            siblings: { type: "array", minItems: 1, items: { type: "object", required: ["id", "key"], properties: { id: { type: "integer", minimum: 1 }, key: { type: "string" } } } },
            expected: { type: "array", minItems: 1, items: { type: "string", pattern: "^(k:|#)" } },
          },
        },
      },
      refreshes: {
        type: "array",
        minItems: 3,
        items: {
          type: "object",
          required: ["name", "why", "statusJson", "body"],
          properties: {
            name: { type: "string", minLength: 1 },
            why: { type: "string", minLength: 1 },
            statusJson: { type: "string", minLength: 1 },
            body: { $ref: "#/$defs/body" },
          },
        },
      },
    },
  };
  check(ownedAjv().compile(shape)(fixture) === true, "the retention fixture must satisfy its own declared document shape");

  /** 🧬️ `builtNodeToSnapshot`'s numbering, re-derived: an id is taken BEFORE the children are walked. */
  const mint = (body: RetentionBody): Map<string, number> => {
    const ids = new Map<string, number>();
    let next = 1;
    const walk = (node: RetentionBody): void => {
      ids.set(node.key, next);
      next += 1;
      for (const child of node.children ?? []) walk(child);
    };
    walk(body);
    return ids;
  };
  const countNodes = (node: RetentionBody): number => 1 + (node.children ?? []).reduce((total, child) => total + countNodes(child), 0);

  fixture.refreshes.forEach((refresh, index) => {
    check(countNodes(refresh.body) === fixture.expected.bodyNodeCountByRefresh[index], `refresh "${refresh.name}" must carry ${fixture.expected.bodyNodeCountByRefresh[index]} nodes`);
    check(mint(refresh.body).get(fixture.surfaceKey) === fixture.expected.surfaceNodeIdByRefresh[index], `refresh "${refresh.name}" must mint the surface at id ${fixture.expected.surfaceNodeIdByRefresh[index]}`);
    check(JSON.parse(refresh.statusJson) !== null, `refresh "${refresh.name}" must carry a parseable status map — the refreshes must differ by status, not only by shape`);
  });
  check(new Set(fixture.expected.surfaceNodeIdByRefresh).size > 1, "the fixture must actually renumber the surface, or the retention law it states is vacuous");
  check(new Set(fixture.refreshes.map((refresh) => refresh.statusJson)).size === fixture.refreshes.length, "every refresh must carry a DIFFERENT status map");

  /** 🪪️ `uiSiblingReactKeys`, re-derived from the fixture's written law rather than imported. */
  const siblingKeys = (siblings: readonly { readonly id: number; readonly key: string }[]): string[] => {
    const seen = new Map<string, number>();
    for (const sibling of siblings) if (sibling.key) seen.set(sibling.key, (seen.get(sibling.key) ?? 0) + 1);
    return siblings.map((sibling) => (sibling.key && seen.get(sibling.key) === 1 ? `k:${sibling.key}` : `#${sibling.id}`));
  };
  for (const sample of fixture.siblingKeyCases) {
    assert.deepEqual(siblingKeys(sample.siblings), [...sample.expected], `sibling key case "${sample.name}"`);
    checks += 1;
  }

  /** 🌳️ A minimal reconciler: a mounted subtree survives a refresh when its parent's child-key run
   * still names it, and is torn down and remounted when it does not. `mountsOf` answers how many
   * times the surface was created across the whole refresh sequence. */
  const mountsOf = (keyed: "authored" | "minted"): { readonly mounts: number; readonly path: readonly string[] } => {
    let mounted = new Map<string, string>();
    let mounts = 0;
    let path: readonly string[] = [];
    for (const refresh of fixture.refreshes) {
      const ids = mint(refresh.body);
      const next = new Map<string, string>();
      const walk = (node: RetentionBody, prefix: readonly string[]): void => {
        const children = node.children ?? [];
        const runKeys = keyed === "authored"
          ? siblingKeys(children.map((child) => ({ id: ids.get(child.key)!, key: child.key })))
          : children.map((child) => `#${ids.get(child.key)!}`);
        children.forEach((child, index) => {
          const here = [...prefix, runKeys[index]!];
          next.set(here.join("/"), child.key);
          if (child.key === fixture.surfaceKey) path = here;
          walk(child, here);
        });
      };
      walk(refresh.body, []);
      const surfacePath = path.join("/");
      if (mounted.get(surfacePath) !== fixture.surfaceKey) mounts += 1;
      mounted = next;
    }
    return { mounts, path };
  };

  const retained = mountsOf("authored");
  check(retained.mounts === fixture.expected.surfaceHostMounts, `keyed on the authored key, ${fixture.refreshes.length} refreshes must mount the surface host exactly ${fixture.expected.surfaceHostMounts}×, not ${retained.mounts}×`);
  assert.deepEqual([...retained.path], [...fixture.expected.surfaceReactKeyPath], "the surface's reconciliation path must be the authored key path the fixture declares");
  checks += 1;
  const renumbered = mountsOf("minted");
  check(renumbered.mounts > fixture.expected.surfaceHostMounts, `keyed on the minted id the surface must remount (it did ${renumbered.mounts}× here) — a twin that cannot witness the defect cannot prove the fix`);

  // 📌️ Source anchors: the rule must live in the interpreter, and the container must not have kept the id.
  const interpreter = readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🟦️.tsx"), "utf8");
  check(interpreter.includes("export function uiSiblingReactKeys("), "`uiSiblingReactKeys` must be the interpreter's one reconciliation-key rule");
  check(interpreter.includes("export function uiChildReactKeys("), "`uiChildReactKeys` must resolve a child-id run against the live document state");
  check(!interpreter.includes("<UiNodeView key={childId}"), "a child run keyed on the DFS-minted id is the defect this fixture exists for");
  check(interpreter.includes("uiChildReactKeys(store.getState(), childIds)"), "`ContainerView` must reconcile its children on the authored keys");
  return checks;
}
//#endregion 🪪️SurfaceHostRetentionOracle

/** 🪪️ Runs the retained-surface-host laws: the framework-free twin over the language-neutral fixture,
 * then the React half that witnesses DOM identity and the single attach. */
class SurfaceRetentionCheckScript extends BundleScript {
  run(segments: string[]): void {
    if (segments.length !== 0) throw new Error("surface-retention-check accepts no arguments");
    console.log(`surface-host-retention-oracle: checks=${surfaceHostRetentionOracle(this.repoRoot)} clean`);
    process.env.SEMIO_TEST_LEVEL = "long";
    runVitest(this.root, ["../../../../🧱️elements/🗣️Interpreter/🟦️.tsx", "../../../../🧪️tests/🔬️engine-contract/🟦️.ts", "--silent=false", "--reporter=verbose", "-t", "retained surface host|sibling reconciliation keys"], "vitest.config.ts");
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
  .register("world3d-interaction-check", World3dInteractionCheckScript)
  .register("surface-switch-check", SurfaceSwitchCheckScript)
  .register("surface-retention-check", SurfaceRetentionCheckScript)
  .register("document-opening-scope-check", DocumentOpeningScopeCheckScript)
  .register("view-state-carriage-check", ViewStateCarriageCheckScript);

await runBundleScriptMain(router, import.meta.url);
