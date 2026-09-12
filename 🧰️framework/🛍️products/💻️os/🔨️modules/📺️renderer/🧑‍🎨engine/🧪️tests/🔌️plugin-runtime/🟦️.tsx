import type { HistoryPatch, PluginRegistryEntry, UiNodeRecord } from "@semio-tech/framework";
import type { LocalInteractionCapture } from "@semio-tech/framework-replication";
import type { AppChannelClient } from "@semio-tech/framework-os";
import { encodeAppFrame } from "@semio-tech/framework-os";
import type { ActivationRegistry, TurnOutcome } from "../../../../../../../🔨️modules/🎠️kernel/🟦️.ts";
import type { ShardBudget, ShardClient, ShardEventEnvelope, ShardInstanceLifecycleLease, ShardJobStep, ShardWorkerLike } from "../../../../../../../🔨️modules/🎭️actor/📮️shard-client/🟦️.ts";
import type { PluginManifest } from "../../🧱️elements/🐚️Shell/🟦️.tsx";
import type { PluginRuntimeTestDependenciesV1, PluginWasmHandle, RetainedSurface, WireTurnResult, WireVariant } from "../../🧱️elements/🔌️PluginRuntime/🟦️.tsx";

/** 🧩️ One packed text leaf of a paged text carrier: slice 0 in `value`, slices 1..32 as
 * `dataAttributes` keyed `01`..`32` — the exact node shape `section_text_chunks`
 * (`🔌️plugin/🦀️.rs`) publishes. */
function packedCarrierLeaf(id: number, value: string, dataAttributes: { readonly [key: string]: string } | null): UiNodeRecord {
  return {
    id, key: `c${id - 1}`, component: { type: "text", value, emphasize: null, dataAttributes },
    layout: { kind: "leaf", width: "hug", height: "hug" }, style: { variant: "plain", size: "md", density: "standard", tone: "neutral", emphasis: "regular" }, activity: "idle", disabled: false,
    transition: null, accessibility: { label: null, description: null, live: "off", shortcut: null, hidden: false }, bindings: [], menu: null, children: [],
  } as UiNodeRecord;
}

/** 🧩️ The carrier root `paged_text_carrier` stamps with the reserved section body key. */
function packedCarrierRoot(bodyKey: string, children: readonly number[]): UiNodeRecord {
  return {
    id: 0, key: bodyKey,
    component: { type: "container", role: "plain", label: null, description: null, required: null, error: null, defaultOpen: null, dropOverlay: null },
    layout: { kind: "stack", axis: "vertical", gap: "none", padding: { all: "none" }, align: "stretch", justify: "start", grow: false, wrap: false },
    style: { variant: "plain", size: "md", density: "standard", tone: "neutral", emphasis: "regular" }, activity: "idle", disabled: false,
    transition: null, accessibility: { label: null, description: null, live: "off", shortcut: null, hidden: false }, bindings: [], menu: null, children: [...children],
  } as UiNodeRecord;
}

export async function registerTests1(vitest: Pick<typeof import("vitest"), "describe" | "expect" | "it" | "vi">, dependencies: PluginRuntimeTestDependenciesV1, source: { url: string }): Promise<void> {
  const { testState, leftoverShellInvocationFrames, ActivationRegistry, ActorDocumentBindingV1, adaptPluginHandle, assertAddressedInvocation, AppChannelClient, AppChannelRequestSequence, applyRetainedWindowPatches, applyUiPatch, applyUiPatchToRetained, ArtifactMutationRouter, assertShardJspiAvailable, BACKBONE_HOT_MESSAGE_MAXIMUM_BYTES, buildShardClientOptions, coerceTurnResult, coerceWireBytes, commandIngressFaultDisplay, computeDependencyLevels, consumeTypedOperationEffects, createShardCommandIngressPages, createTurnOutcomeBroadcast, currentPluginRuntimeActor, decodeActorUiPatchReceipt, decodeAppFrame, decodeBackboneMessage, decodeConflictsFromWire, decodeFaultFromWire, decodeForeignStep, decodeInvocationResultPacks, decodeLocalInteractionCaptureJson, decodeMergeReportFromWire, decodeMutationEnvelopesPack, decodePackValue, decodePackWire, decodeWirePack, decodeWirePatchOps, DEFAULT_SHARD_BUDGET, drainTypedOperationTurns, DIRECTORY_PROJECTION_RECEIPT_SCHEMA, emptyUiDocumentState, encodeActorUiPatchReceipt, encodeDocumentBackboneControlV1, encodeMutationOrigin, encodePackValue, enqueuePluginTurn, faultDisplayMessage, fetchDescriptorManifest, fnv1aHex, getActivationRegistry, getPluginTurnScheduler, getShardClient, getThunkScheduler, handlePluginShardLost, forgetInstanceForRecovery, onPluginInstancesLost, PLUGIN_ACTOR_INSTANCE_LOST_FAULT, rememberInstanceForRecovery, hasRequiredUiPatches, InstanceDirectory, invocationFromFrames, isShardLostError, loadPluginModule, loadPluginModulesInDependencyOrder, LOCAL_INTERACTION_CAPTURE_MAX_BYTES, localInteractionIdentityEquals, MAX_TRANSACTION_DEPTH, nextGlobalInstanceId, normalizeWireUiNodeRecord, notePluginLoadProgress, orderPluginRegistryEntries, OwnedResidentLedger, packWireNatural, patchAckEvents, pendingCoalescedTurns, pendingCompletionEffects, pendingLifecycleTurns, pendingTurnEffects, performContextMenu, performInvocation, PLUGIN_BOOT_SHARD_LOST_FAULT, PLUGIN_OPERATION_DRAIN_BUDGET, PLUGIN_TURN_MAILBOX_CAPACITY, PLUGIN_UI_CONTINUATION_BATCH_SIZE, PLUGIN_UI_CONTINUATION_LIMIT, PLUGIN_UI_QUIESCENT_CONTINUATIONS, PLUGIN_UI_ZERO_PROGRESS_CONTINUATION_LIMIT, PluginBootShardLostError, pluginLoadProgress, pluginLoadProgressAt, pluginSurfaceRef, poolConcurrency, rejectionCodeFromBytes, releasePendingLifecycleTurn, rendererResidentLedger, resolveDescriptorBeforeRuntime, retainedSurfaceHash, retainedSurfaceId, retainedSurfacesForActor, retainedSurfaceToBuiltNode, retainedSurfaceToSnapshot, retainedUiRefreshResponse, uiRefreshSectionUnchanged, retainedWindowByActor, retainTurnUiPatches, runBounded, sectionValueFromBuiltNode, runPluginLifecycleTurn, SEGMENTED_DOWNLOAD_MARKER_PREFIX, SemioFaultError, SERIALIZE_PER_ACTOR_MAILBOX_CAPACITY, serializeCommandIngressForActor, serializePerActor, setPluginRuntimeActor, settleAcknowledgedPluginTurns, settlePluginTurn, SHARD_LIVENESS_POLICY, SHARD_WORKER_URL, ShardClient, sharedPluginTurnScheduler, sharedThunkScheduler, shellFrameBytes, submitPluginLifecycleTurn, submitPluginTurn, teardownPluginActor, tearingDownPluginActors, TransactionCoordinator, TurnScheduler, TYPED_OPERATION_ACK_MAGIC, TYPED_OPERATION_PAGE_MAGIC, TYPED_OPERATION_PARK_CAPACITY, TYPED_OPERATION_PARK_EVICTION_FAULT, TYPED_OPERATION_PENDING_OUTPUT, TYPED_OPERATION_TERMINAL_OUTPUT, TYPED_OPERATION_TERMINAL_SEEN, TYPED_OPERATION_UNATTRIBUTED_FAULT, typedOperationAcknowledgements, TypedOperationCall, TypedOperationRouter, typedOperationResult, uiRefreshBodyKeys, uiRefreshSectionTargets, uiRefreshSurfaceEvents, wireEffectToFriendly, wireExtensionInvocation, wireNatural, wirePatchSurfaceId, wireTurnStatusTag, withTypedOperationCall, yieldPluginUiContinuation } = dependencies;
  const { describe, expect, it, vi } = vitest;
  describe("isolated job admission batch", () => {
    it("fails-before one slice per admission; passes-after a yield-sized batch", () => {
      const { isolatedJobStepsPerSerializedAdmission } = dependencies;
      expect(isolatedJobStepsPerSerializedAdmission(1)).toBe(1);
      expect(isolatedJobStepsPerSerializedAdmission(32)).toBe(32);
      expect(isolatedJobStepsPerSerializedAdmission(0)).toBe(1);
    });
    it("reads history_patch from a leftover job-completed Invocation send-message", () => {
      const { invocationFromFrames, encodePackValue } = dependencies;
      const patch = { cursor: 3, canUndo: true, canRedo: true, upserts: [{ seq: 2, actionId: "os.resizeWindow", label: "Resize Window", kind: "shell", timestamp: "0", revertible: false }] };
      const leftover = [{
        tag: "send-message",
        val: {
          target: { tag: "shell", val: "1" },
          payload: Array.from(encodeAppFrame({
            Invocation: {
              in_reply_to: 0,
              output: [],
              diagnostics: [],
              ui_scope: [],
              history_patch: Array.from(encodePackValue(patch)),
              messages: [],
              mutations: [],
              inverse_group: [],
            },
          })),
        },
      }, { tag: "replay-shell-command", val: { actionId: "os.resizeWindow", args: encodePackValue({ width: 800, height: 600 }) } }];
      const response = invocationFromFrames([{ Invocation: { in_reply_to: 1, output: [], diagnostics: [], ui_scope: [], history_patch: [], messages: [], mutations: [], inverse_group: [] } }], leftover, "action");
      expect(response.historyPatch).toEqual(patch);
      expect(response.requestedEffects).toEqual([{ replayShellCommand: { actionId: "os.resizeWindow", args: { width: 800, height: 600 } } }]);
      expect(leftoverShellInvocationFrames(leftover)).toHaveLength(1);
    });

    // 📋️ Ticket 26/09/09/PROCEDURAL-3D-END-TO-END (`📓️selection-dedupe-2026-09-12.md`): a reserved job
    // publishes its result frame on `send-message` whatever it did, so warning "clipboard-write missing"
    // whenever a leftover carried one fired on EVERY reserved-job completion — measured on every
    // `interactionSelect`/`interactionHover` of the generation3d preview. Nothing in the leftover list
    // declares that a clipboard write was expected, so there is no predicate to warn on.
    it("never warns about a missing clipboard-write on a job completion that only carries send-message effects", () => {
      const { promoteShellSendMessages } = dependencies;
      const warn = vi.spyOn(console, "warn").mockImplementation(() => {});
      try {
        const leftover = [
          { tag: "send-message", val: { target: { tag: "shell", val: "1" }, payload: [1, 2, 3] } },
          { tag: "send-message", val: { target: { tag: "shell", val: "1" }, payload: [4, 5, 6] } },
        ];
        expect(promoteShellSendMessages(1, leftover, [])).toEqual(leftover);
        expect(warn.mock.calls.flat().filter((entry) => String(entry).includes("clipboard"))).toEqual([]);
      } finally {
        warn.mockRestore();
      }
    });

    it("fails-before every step polls UI; passes-after a 128-step stride", () => {
      const { isolatedJobUiPollEverySteps } = dependencies;
      expect(isolatedJobUiPollEverySteps(1, 1), "fails-before: stride 1 polls every step").toBe(true);
      expect(isolatedJobUiPollEverySteps(0)).toBe(false);
      expect(isolatedJobUiPollEverySteps(127)).toBe(false);
      expect(isolatedJobUiPollEverySteps(128)).toBe(true);
      expect(isolatedJobUiPollEverySteps(256)).toBe(true);
    });
  });
  describe("shared context-menu ViewModel", () => {
    it("sends the same current locale and terminology to two mounted surfaces while preserving each window", async () => {
      const contextMenu = vi.fn(async () => []);
      const client = { contextMenu };
      const base = { locale: "de", terminology: "reuse" };
      await performContextMenu(client, { menu: { id: "canvas", args: null }, surface: { surfaceId: "first", kind: "canvas" }, windowInstanceId: "window-a" }, { ...base, windowId: "window-a" });
      await performContextMenu(client, { menu: { id: "canvas", args: null }, surface: { surfaceId: "second", kind: "canvas" }, windowInstanceId: "window-b" }, { ...base, windowId: "window-b" });
      expect(contextMenu.mock.calls.map((call: any[]) => call[0].viewState)).toEqual([
        { ...base, windowId: "window-a" },
        { ...base, windowId: "window-b" },
      ]);
      console.info("[DEBUG] two context-menu surfaces received the current shared OS locale and terminology");
    });
  });
  describe("surface render ViewModel", () => {
    it("binds two instances of one body to distinct surfaces, the leftover default window alias, and packed window projections", () => {
      const { DEFAULT_LEFTOVER_WINDOW_SURFACE } = dependencies;
      const events = uiRefreshSurfaceEvents(7, {
        viewState: {
          locale: "de",
          terminology: "reuse",
          windowInstances: [
            { id: "first", windowKindId: "canvas" },
            { id: "second", windowKindId: "canvas" },
          ],
          activeUtilityByWindowId: { first: "inspect", second: "measure" },
        },
        windows: [
          { key: "first", bodyKey: "canvas-body" },
          { key: "second", bodyKey: "canvas-body" },
          { key: "unauthored" },
        ],
        panels: [{ key: "details", bodyKey: "details-body" }, { key: "host-panel" }],
      });
      expect(events.map((event: { payload: { surface: { surface: string }; bodyKey: string } }) => [event.payload.surface.surface, event.payload.bodyKey])).toEqual([
        ["first", "canvas-body"],
        ["second", "canvas-body"],
        [DEFAULT_LEFTOVER_WINDOW_SURFACE, "canvas-body"],
        ["details", "details-body"],
      ]);
      const views = events.map((event: { payload: { viewState: Uint8Array } }) => decodePackValue(event.payload.viewState));
      expect(views).toMatchObject([
        { locale: "de", terminology: "reuse", windowId: "first", activeWindowKindId: "canvas", activeUtilityId: "inspect" },
        { locale: "de", terminology: "reuse", windowId: "second", activeWindowKindId: "canvas", activeUtilityId: "measure" },
        { locale: "de", terminology: "reuse", windowId: "second", activeWindowKindId: "canvas", activeUtilityId: "measure" },
        { locale: "de", terminology: "reuse" },
      ]);
      expect((views[3] as { windowId?: string | null }).windowId == null).toBe(true);
      console.info("[DEBUG] concrete render surfaces received packed locale, terminology, and per-window utility context");
    });
  });
  describe("reserved refresh sections", () => {
    it("mounts one retained surface per requested section, keyed by the neutral fixture's reserved body key, carrying the unnarrowed view", async () => {
      const { default: reserved } = await import("../../../../../../../🔨️modules/🛂️manifest/🧫️fixtures/🔬️ui-refresh-section/🔣️.json");
      const viewState = {
        locale: "de",
        terminology: "reuse",
        windowId: "first",
        activeToolId: "fill",
        windowInstances: [{ id: "first", windowKindId: "canvas" }, { id: "second", windowKindId: "canvas" }],
        activeUtilityByWindowId: { first: "inspect" },
      };
      const request = { viewState, windows: [], panels: [], engagements: {}, measures: { hash: "abc" }, tools: {}, catalogue: {} };
      expect(uiRefreshSectionTargets(request).map((section: { key: string; bodyKey: string }) => [section.key, section.bodyKey]))
        .toEqual(reserved.sections.map((section: { key: string; bodyKey: string }) => [section.key, section.bodyKey]));
      const events = uiRefreshSurfaceEvents(9, request);
      expect(events.map((event: { payload: { surface: { surface: string }; bodyKey: string } }) => [event.payload.surface.surface, event.payload.bodyKey]))
        .toEqual(reserved.sections.map((section: { bodyKey: string }) => [section.bodyKey, section.bodyKey]));
      for (const event of events) expect(decodePackValue((event as { payload: { viewState: Uint8Array } }).payload.viewState)).toMatchObject(viewState);
      expect(uiRefreshSectionTargets({ viewState })).toEqual([]);
      expect(uiRefreshBodyKeys(request)).toEqual([]);
      console.info("[DEBUG] section surfaces mounted with reserved body keys and the full window/tool view state");
    });

    it("projects a section's chunked text carrier back into the shell's own measures map", async () => {
      const { default: reserved } = await import("../../../../../../../🔨️modules/🛂️manifest/🧫️fixtures/🔬️ui-refresh-section/🔣️.json");
      const measures = { "puzzle3d-main-top": [{ kind: "slider", id: "puzzle3d-fill-count", value: 12, min: 1, max: 64, step: 1, onChange: { controller: "fill", action: "setCount" } }] };
      const payload = JSON.stringify(measures);
      const chunks = [payload.slice(0, 17), payload.slice(17)];
      const measuresSection = reserved.sections.find((section: { key: string }) => section.key === "measures")!;
      const leaf = (id: number, value: string): UiNodeRecord => ({
        id, key: `c${id - 1}`, component: { type: "text", value, emphasize: null, dataAttributes: null },
        layout: { kind: "leaf", width: "hug", height: "hug" }, style: { variant: "plain", size: "md", density: "standard", tone: "neutral", emphasis: "regular" }, activity: "idle", disabled: false,
        transition: null, accessibility: { label: null, description: null, live: "off", shortcut: null, hidden: false }, bindings: [], menu: null, children: [],
      });
      const root: UiNodeRecord = {
        id: 0, key: measuresSection.bodyKey,
        component: { type: "container", role: "plain", label: null, description: null, required: null, error: null, defaultOpen: null, dropOverlay: null },
        layout: { kind: "stack", axis: "vertical", gap: "none", padding: { all: "none" }, align: "stretch", justify: "start", grow: false, wrap: false },
        style: { variant: "plain", size: "md", density: "standard", tone: "neutral", emphasis: "regular" }, activity: "idle", disabled: false,
        transition: null, accessibility: { label: null, description: null, live: "off", shortcut: null, hidden: false }, bindings: [], menu: null, children: [1, 2],
      };
      const { surface, desynced } = applyUiPatchToRetained(null, {
        surface: measuresSection.bodyKey, revision: 1, baseRevision: 0,
        ops: [{ type: "upsert", ...leaf(1, chunks[0]) }, { type: "upsert", ...leaf(2, chunks[1]) }, { type: "upsert", ...root }, { type: "setRoot", id: 0 }],
      });
      expect(desynced).toBe(false);
      const built = retainedSurfaceToBuiltNode(surface!);
      if (built === null) throw new Error("fixture surface has no root");
      expect(sectionValueFromBuiltNode(measuresSection.bodyKey, built, "instance 7")).toEqual(measures);
      expect(() => sectionValueFromBuiltNode("framework.section.tools", built, "instance 7")).toThrow("plugin-ui.section-root-mismatch");
      const retained = new Map<string, RetainedSurface>([[retainedSurfaceId(7, measuresSection.bodyKey), surface!]]);
      const response = retainedUiRefreshResponse(7, { viewState: {}, measures: {}, tools: {} }, retained);
      expect(response.measures).toMatchObject({ key: "measures", value: measures });
      expect(response.measures?.hash).not.toBe("");
      expect(response.tools).toBeUndefined();
      console.info("[DEBUG] measures section projected from its retained chunk carrier into the shell refresh cache shape");
    });

    it("reassembles a PACKED carrier leaf — value plus every sorted dataAttributes slice — against the neutral paged-text-carrier fixture", async () => {
      const { default: carrier } = await import("../../../../../../../🔨️modules/🛂️manifest/🧫️fixtures/🔬️paged-text-carrier/🔣️.json");
      expect(carrier.slices.join("")).toBe(carrier.payload);
      expect(carrier.payload.length).toBe(carrier.payloadBytes);
      expect(carrier.payloadBytes).toBeGreaterThan(carrier.textMaxBytes);
      expect(carrier.slices[0].length).toBe(carrier.textMaxBytes);
      expect(() => JSON.parse(carrier.slices[0])).toThrow();
      expect(carrier.slices.length).toBeGreaterThan(1);
      expect(carrier.slices.length).toBeLessThanOrEqual(carrier.packSlices);
      const dataAttributes = Object.fromEntries(carrier.slices.slice(1).map((slice: string, offset: number) => [String(offset + 1).padStart(2, "0"), slice]));
      const { surface, desynced } = applyUiPatchToRetained(null, {
        surface: carrier.rootKey, revision: 1, baseRevision: 0,
        ops: [
          { type: "upsert", ...packedCarrierLeaf(1, carrier.slices[0], dataAttributes) },
          { type: "upsert", ...packedCarrierRoot(carrier.rootKey, [1]) },
          { type: "setRoot", id: 0 },
        ],
      });
      expect(desynced).toBe(false);
      const built = retainedSurfaceToBuiltNode(surface!);
      if (built === null) throw new Error("fixture surface has no root");
      expect(sectionValueFromBuiltNode(carrier.rootKey, built, "instance 7")).toEqual(JSON.parse(carrier.payload));
      console.info(`[DEBUG] packed carrier leaf reassembled ${carrier.payloadBytes} bytes from 1 value slice and ${carrier.slices.length - 1} dataAttributes slices`);
    });

    it("raises a typed fault naming the reserved section and its producer when a carrier's payload is not valid JSON", async () => {
      const { default: carrier } = await import("../../../../../../../🔨️modules/🛂️manifest/🧫️fixtures/🔬️paged-text-carrier/🔣️.json");
      const { surface } = applyUiPatchToRetained(null, {
        surface: carrier.rootKey, revision: 1, baseRevision: 0,
        ops: [
          { type: "upsert", ...packedCarrierLeaf(1, carrier.slices[0], null) },
          { type: "upsert", ...packedCarrierRoot(carrier.rootKey, [1]) },
          { type: "setRoot", id: 0 },
        ],
      });
      const built = retainedSurfaceToBuiltNode(surface!);
      if (built === null) throw new Error("fixture surface has no root");
      let raised: unknown;
      try { sectionValueFromBuiltNode(carrier.rootKey, built, "procedural#1 instance 7"); } catch (error) { raised = error; }
      expect(raised).toBeInstanceOf(SemioFaultError);
      const fault = (raised as InstanceType<typeof SemioFaultError>).fault;
      expect(fault.code).toBe("plugin-ui.section-payload-not-json");
      expect(fault.scope.bodyKey).toBe(carrier.rootKey);
      expect(fault.message).toContain("procedural#1 instance 7");
      expect(fault.message).toContain(String(carrier.textMaxBytes));
      expect(fault.causes?.[0]?.message ?? "").not.toBe("");
      console.info(`[DEBUG] truncated carrier raised ${fault.code} naming ${fault.scope.bodyKey} and its producer instead of a bare SyntaxError`);
    });
  });
  it("RendererResidentComposition never replaces a closing composition ledger", async () => {
      const { execFileSync } = await import("node:child_process"); const { fileURLToPath, pathToFileURL } = await import("node:url"); const { dirname, resolve } = await import("node:path"); const { default: fixture } = await import("../../💾️resident/🧫️fixtures/🔣️.json"); const moduleUrl = pathToFileURL(resolve(dirname(fileURLToPath(source.url)), "../../💾️resident/🟦️.ts")).href;
      const childSource = `const { rendererResidentLedger } = await import(process.argv[1]); const first = rendererResidentLedger(); first.beginClose(); const result = first.closeStep({maxItems:1,maxBytes:256}); const second = rendererResidentLedger(); const admission = second.prepareAdmission({},'data',{maxItems:1,maxBytes:296}); process.stdout.write(JSON.stringify({same:first===second,terminal:first.terminalIsEmpty(),result:result.kind,admission:admission.kind}));`;
      const actual = JSON.parse(execFileSync("node", ["--experimental-transform-types", "--input-type=module", "--eval", childSource, moduleUrl], { encoding: "utf8", timeout: 10000 }));
      expect(actual).toEqual({ same: !fixture.replacesClosingLedger, terminal: true, result: "complete", admission: "rejected" });
    });
  
    it("RendererResidentComposition shares one exact ledger and preserves both consumers' charges", async () => {
      const { rendererResidentLedger } = await import("../../💾️resident/🟦️.ts"); const { default: fixture } = await import("../../💾️resident/🧫️fixtures/🔣️.json");
      const { default: rendererModule } = await import("../../../🧬️schema/🔣️.json"); const { default: resident } = await import("../../../../../../../🔨️modules/🌱️value/💾️resident/🧬️schema/🔣️.json"); const { default: Ajv } = await import("ajv"); const { produce } = await import("immer");
      expect(new Ajv({ strict: true }).addSchema(resident).addSchema(rendererModule).getSchema(`${rendererModule.$id}#/$defs/RendererResidentPolicyV1`)!(fixture.capacity)).toBe(true);
      const react = rendererResidentLedger(); const wgpu = rendererResidentLedger(); expect(react === wgpu).toBe(fixture.sameLedger); expect(react.capacity).toEqual(fixture.capacity);
      expect({ bytes: react.capacity.bytes - react.capacity.control.bytes, slots: react.capacity.slots - react.capacity.control.slots, owners: react.capacity.owners - react.capacity.control.owners }).toEqual(fixture.data);
      const grant = { maxItems: 1, maxBytes: 4096 }; const firstOwner = {}; const secondOwner = {};
      expect(react.prepareAdmission(firstOwner, "data", grant).kind).toBe("pending"); const firstCell = react.preparedAdmission(firstOwner); if (!firstCell) throw new Error("React admission cell missing");
      expect(react.claimAdmission(firstOwner, firstCell, grant).kind).toBe("ready"); const first = react.reserveRecord("data", fixture.recordEnvelope, firstCell, grant).record;
      expect(wgpu.prepareAdmission(secondOwner, "data", grant).kind).toBe("pending"); const secondCell = wgpu.preparedAdmission(secondOwner); if (!secondCell) throw new Error("WGPU admission cell missing");
      expect(wgpu.claimAdmission(secondOwner, secondCell, grant).kind).toBe("ready"); const second = wgpu.reserveRecord("data", fixture.recordEnvelope, secondCell, grant).record;
      if (!first || !second) throw new Error("Renderer composition fixture admission refused");
      const expected = produce({ bytes: 0, slots: 0, owners: 0 }, state => { for (const envelope of [fixture.recordEnvelope, fixture.cellEnvelope, fixture.intrinsicRecordEnvelope]) { state.bytes += envelope.bytes * 2; state.slots += envelope.slots * 2; state.owners += envelope.owners * 2; } }); expect(expected).toEqual(fixture.twoRecordUsage);
      expect(react.usage.data).toEqual(fixture.twoRecordUsage); expect(wgpu.usage.data).toEqual(fixture.twoRecordUsage);
      first.beginClose(); expect(first.closeStep(grant).kind).toBe("complete");
      expect(first.terminalIsEmpty()).toBe(false); firstCell.beginClose(); expect(firstCell.closeStep({ maxItems: 1, maxBytes: fixture.recordCloseBytes[1]! }).kind).toBe("pending"); expect(first.terminalIsEmpty()).toBe(true); expect(firstCell.closeStep({ maxItems: 1, maxBytes: fixture.recordCloseBytes[2]! }).kind).toBe("complete");
      expect(wgpu.usage.data).toEqual(produce(fixture.twoRecordUsage, state => { state.bytes /= 2; state.slots /= 2; state.owners /= 2; }));
      second.beginClose(); expect(second.closeStep(grant).kind).toBe("complete"); secondCell.beginClose(); expect(secondCell.closeStep({ maxItems: 1, maxBytes: fixture.recordCloseBytes[1]! }).kind).toBe("pending"); expect(secondCell.closeStep({ maxItems: 1, maxBytes: fixture.recordCloseBytes[2]! }).kind).toBe("complete"); expect(react.usage.data).toEqual(fixture.afterUnusedClose);
      expect(fixture.capacity.bytes).toBe(fixture.aggregateUiPolicyBytes); expect(fixture.surfaceUiPolicyBytes).toBe(8388608);
    });
  
    describe("extension invocation WIT request identity", () => {
      it("rejects narrowed, exhausted or malformed request identities", () => {
        for (const req of [0n, -1n, 0x10000000000000000n, 1, "1", undefined]) {
          expect(() => wireExtensionInvocation({ tag: "invoke-extension", val: { req, params: { extensionId: "text", capability: "evaluate", payload: [] } } })).toThrow("extension.request-id-invalid");
        }
      });
  
      it("preserves nested UTF-8 payloads and exact u64 ids in both renderer decoders", async () => {
        const { default: fixture } = await import("../../🧱️elements/🏛️ShellHost/🧫️fixtures/🔣️extension-invocation.json");
        const { wireEffectToFriendly: sharedDecode } = await import("../../../../../../../🔨️modules/🎭️actor/🖼️wire-turn/🟦️.ts");
        const requestJson = JSON.stringify({ ...fixture.request, label: "Hölzer 日本語" });
        for (const id of fixture.requestIds) {
          const req = BigInt(id);
          const wire = { tag: "invoke-extension", val: { req, params: { extensionId: fixture.extensionId, capability: fixture.capability, payload: new TextEncoder().encode(requestJson) } } };
          const expected = { invokeExtension: { req, extensionId: fixture.extensionId, capability: fixture.capability, requestJson } };
          expect(wireEffectToFriendly(wire)).toEqual(expected);
          expect(sharedDecode(wire, decodePackValue)).toEqual(expected);
        }
      });
    });

    describe("host effect address decoding", () => {
      it("reads every request-carrying effect out of its nested WIT params record", async () => {
        const { default: fixture } = await import("../../🧱️elements/🔌️PluginRuntime/🧫️fixtures/🎯️host-effect-address.json");
        const nested = (tag: string, req: number, params: Record<string, unknown>) => ({ tag, val: { req, params } });
        const { req: dispatchReq, ...dispatchParams } = fixture.dispatchAction;
        expect(wireEffectToFriendly(nested("dispatch-action", dispatchReq, dispatchParams))).toEqual({ dispatchAction: { req: dispatchReq, action: fixture.dispatchAction.action, args: undefined, delayMs: fixture.dispatchAction.delayMs } });
        const { req: windowReq, ...windowParams } = fixture.openWindow;
        expect(wireEffectToFriendly(nested("open-window", windowReq, windowParams))).toEqual({ openWindow: { req: windowReq, kind: fixture.openWindow.kind, params: undefined } });
        const { req: dialogReq, ...dialogParams } = fixture.openDialog;
        expect(wireEffectToFriendly(nested("open-dialog", dialogReq, dialogParams))).toEqual({ openDialog: { req: dialogReq, dialogId: fixture.openDialog.dialogId, args: undefined } });
        const { req: spawnReq, ...spawnParams } = fixture.spawnPluginInstance;
        expect(wireEffectToFriendly(nested("spawn-plugin-instance", spawnReq, spawnParams))).toEqual({ spawnPluginInstance: { req: spawnReq, ...spawnParams } });
      });

      it("drops a dispatch-action whose action id never survived the boundary", async () => {
        const { default: fixture } = await import("../../🧱️elements/🔌️PluginRuntime/🧫️fixtures/🎯️host-effect-address.json");
        const warn = vi.spyOn(console, "warn").mockImplementation(() => {});
        try {
          // The pre-fix flat read: the payload sits at the top level, so `params.action` is absent.
          expect(wireEffectToFriendly({ tag: "dispatch-action", val: { ...fixture.dispatchAction } })).toBeNull();
          expect(wireEffectToFriendly({ tag: "dispatch-action", val: { req: fixture.dispatchAction.req, params: { action: "", delayMs: 0 } } })).toBeNull();
          expect(warn).toHaveBeenCalledTimes(2);
        } finally {
          warn.mockRestore();
        }
      });

      it("refuses an unaddressed invocation with a typed renderer fault naming its window kind", async () => {
        const { default: fixture } = await import("../../🧱️elements/🔌️PluginRuntime/🧫️fixtures/🎯️host-effect-address.json");
        const address = { pluginId: "procedural", appId: fixture.spawnPluginInstance.appId, windowKindId: fixture.windowKindId, actionId: "" };
        let thrown: unknown;
        try {
          assertAddressedInvocation({ address, arguments: {} }, "action", fixture.instanceId);
        } catch (error) {
          thrown = error;
        }
        expect(thrown).toBeInstanceOf(SemioFaultError);
        const fault = (thrown as InstanceType<typeof SemioFaultError>).fault;
        expect(fault.code).toBe(fixture.unaddressedFaultCode);
        expect(fault.origin).toBe(fixture.unaddressedFaultOrigin);
        expect(fault.message).toContain(fixture.windowKindId);
        expect(fault.scope.instanceId).toBe(String(fixture.instanceId));
        expect(() => assertAddressedInvocation({ address: { ...address, actionId: fixture.dispatchAction.action } }, "action", fixture.instanceId)).not.toThrow();
        expect(() => assertAddressedInvocation({ address: { commandId: "" } }, "command", fixture.instanceId)).toThrow(SemioFaultError);
        expect(() => assertAddressedInvocation({ address: { commandId: "open" } }, "command", fixture.instanceId)).not.toThrow();
      });
    });

    describe("extension invocation completion publication", () => {
      async function withRequester(turn: (actor: string, events: readonly ShardEventEnvelope[]) => Promise<WireTurnResult>, run: (handle: PluginWasmHandle, instance: number, activation: { replace(): void; captures(): number; guardedTurns(): number }) => Promise<void>, hooks: { activate?: (actor: string) => Promise<void>; open?: (actor: string) => Promise<void>; dispose?: (actor: string) => void; jobs?: { start?: (job: bigint, kind: string, input: Uint8Array) => void; step?: (job: bigint, budget: { fuel: bigint; deadlineMs: number }) => ShardJobStep; cancel?: (job: bigint) => void } } = {}): Promise<void> {
        const { encodeActorInstanceLifecycle } = await import("../../../../../../../🔨️modules/🎭️actor/🚪️lifetime/🟦️.ts");
        const previous = { registry: testState.sharedActivationRegistry, shard: testState.sharedShardClient, fetch: globalThis.fetch };
        const idle: WireTurnResult = { uiPatches: [], effects: [], nextWake: null, status: { tag: "idle" } };
        let generation = 1n;
        let captures = 0;
        let guardedTurns = 0;
        const disposed = new Set<string>();
        const dispose = (actor: string) => { if (!disposed.has(actor)) { disposed.add(actor); hooks.dispose?.(actor); } };
        const dispatch = async (actor: string, events: readonly ShardEventEnvelope[]) => {
          if (events.some(event => event.kind === "instance-open")) { await hooks.open?.(actor); return idle; }
          return turn(actor, events);
        };
        testState.sharedActivationRegistry = { registerManifest: () => {}, activate: async (_plugin: string, actor: string) => hooks.activate?.(actor), touch: () => {}, cancel: dispose } as unknown as ActivationRegistry;
        testState.sharedShardClient = {
          turn: dispatch,
          startJob: async (_actorId: string, job: bigint, kind: string, input: Uint8Array) => { hooks.jobs?.start?.(job, kind, input); },
          stepJob: async (_actorId: string, job: bigint, budget: { fuel: bigint; deadlineMs: number }) => hooks.jobs?.step?.(job, budget) ?? ({ status: "running" } as ShardJobStep),
          cancelJob: async (_actorId: string, job: bigint) => { hooks.jobs?.cancel?.(job); },
          captureInstanceLifecycle: (actorId: string, instanceId: number) => {
            const activationGeneration = generation;
            const activation = { actorId, activationGeneration, assertActive: () => { if (generation !== activationGeneration) throw new Error("actor-activation.revoked"); } };
            const lifetime = { activationGeneration, instanceId, guestLifetime: 1n };
            const openRequest = { kind: "open" as const, activationGeneration, instanceId, requestSequence: 1 };
            const captured = { kind: "captured" as const, lifetime, requestSequence: openRequest.requestSequence };
            let phase: "opening" | "captured" | "open" | "closing" | "accepted" | "retired" | "complete" = "opening";
            let pending: import("../../../../../../../🔨️modules/🎭️actor/🚪️lifetime/🟦️.ts").ActorInstanceLifecycleReceipt | null = null;
            const closeRequest = { kind: "close" as const, lifetime, requestSequence: 2 };
            const accepted = { kind: "accepted" as const, lifetime, requestSequence: closeRequest.requestSequence, closeGeneration: 1n };
            const retired = { ...accepted, kind: "retired" as const };
            return {
              activation,
              openRequest,
              get lifetime() { return phase === "opening" ? null : lifetime; },
              get pendingReceipt() { return pending; },
              interruptedTurn: null,
              open: async () => { await hooks.open?.(actorId); phase = "captured"; pending = captured; return { ...idle, lifecycleReceipt: encodeActorInstanceLifecycle(captured) }; },
              poll: async () => idle,
              beginClose: () => { if (phase === "open") phase = "closing"; return closeRequest; },
              close: async () => { phase = "accepted"; pending = accepted; return { ...idle, lifecycleReceipt: encodeActorInstanceLifecycle(accepted) }; },
              acknowledge: async (receipt: typeof captured | typeof accepted | typeof retired) => {
                if (receipt.kind === "captured") { phase = "open"; pending = null; return idle; }
                if (receipt.kind === "accepted") { phase = "retired"; pending = retired; return { ...idle, lifecycleReceipt: encodeActorInstanceLifecycle(retired) }; }
                phase = "complete"; pending = null; return idle;
              },
              bindHostRetirement: () => {},
              captureUiPatchAuthority: () => { throw new Error("fixture-native-ui-not-configured"); },
              submitUiAcknowledgement: async () => { throw new Error("fixture-native-ui-not-configured"); },
              dispose: () => dispose(actorId),
              progress: () => ({ kind: phase, failure: null }),
            };
          },
          captureActorActivation: (actorId: string) => {
            captures += 1;
            const activationGeneration = generation;
            return { actorId, activationGeneration, assertActive: () => { if (generation !== activationGeneration) throw new Error("actor-activation.revoked"); }, turn: (events: readonly ShardEventEnvelope[]) => { guardedTurns += 1; return dispatch(actorId, events); } };
          },
          dispose,
        } as unknown as ShardClient;
        globalThis.fetch = (async () => new Response(JSON.stringify({ manifest: { pluginId: "extension-requester", apps: [] } }), { headers: { "content-type": "application/json" } })) as typeof fetch;
        let handle: PluginWasmHandle | undefined;
        try {
          handle = await loadPluginModule("extension-requester", "https://fixture.invalid/plugin.js");
          await run(handle, await handle.createApp("fixture"), { replace: () => { generation += 1n; }, captures: () => captures, guardedTurns: () => guardedTurns });
        } finally {
          await handle?.dispose();
          testState.sharedActivationRegistry = previous.registry;
          testState.sharedShardClient = previous.shard;
          globalThis.fetch = previous.fetch;
        }
      }
  
      it("settles the exact completion and returns frames and host effects", async () => {
        const { default: fixture } = await import("../../🧱️elements/🏛️ShellHost/🧫️fixtures/🔣️extension-invocation.json");
        const { encodeAppFrame } = await import("@semio-tech/framework-os");
        const bytes = (value: unknown) => Array.from(encodePackValue(value));
        let instance = 0;
        const submitted: ShardEventEnvelope[][] = [];
        await withRequester(async (_actor, events) => {
          submitted.push([...events]);
          if (submitted.length === 1) return { uiPatches: [], effects: [{ tag: "notify", val: { message: fixture.completion.notification } }], nextWake: null, status: { tag: "more-work" } };
          if (submitted.length === 2) return {
            uiPatches: [],
            effects: [{ tag: "send-message", val: { target: { tag: "shell", val: String(instance) }, payload: Array.from(encodeAppFrame({ Invocation: { in_reply_to: 0, output: bytes(fixture.response), diagnostics: bytes([]), ui_scope: bytes(fixture.completion.uiScope), history_patch: bytes(fixture.completion.historyPatch), messages: [], mutations: [], inverse_group: [] } })) } }],
            nextWake: null, status: { tag: "idle" },
          };
          return { uiPatches: [], effects: [], nextWake: null, status: { tag: "idle" } };
        }, async (handle, opened) => {
          instance = opened;
          const req = BigInt(fixture.requestIds[2]!);
          const outcome = { ok: encodePackValue(fixture.response) };
          const response = await handle.captureExtensionCompletion!(instance, req).complete(outcome);
          expect(submitted).toEqual([[{ kind: "completed", payload: { req, outcome: { tag: "ok", val: Array.from(outcome.ok) } } }], []]);
          expect(response).toMatchObject({ output: fixture.response, requestedEffects: [{ notify: { message: fixture.completion.notification } }], uiScope: fixture.completion.uiScope, historyPatch: fixture.completion.historyPatch });
        });
      });
  
      // 💼️ Ticket 26/09/02/PUZZLE-3D-END-TO-END W-J. `Effect::SpawnJob` used to reach
      // `wireEffectToFriendly`'s `default:` arm and be dropped with a `[DEBUG]` warning, and nothing
      // in this renderer ever called `ShardClient.startJob`/`stepJob` — so on this target EVERY
      // plugin-authored isolated job was started zero times and stepped zero times, silently. That is
      // what left puzzle 3d's fill planning frozen at zero in the browser.
      it("starts an isolated job, steps it to its terminal and feeds job-completed back into the actor", async () => {
        const { encodeAppFrame } = await import("@semio-tech/framework-os");
        const { default: fixture } = await import("../../🧱️elements/🏛️ShellHost/🧫️fixtures/🔣️extension-invocation.json");
        const bytes = (value: unknown) => Array.from(encodePackValue(value));
        const started: Array<{ job: bigint; kind: string; input: readonly number[] }> = [];
        const stepped: bigint[] = [];
        const submitted: ShardEventEnvelope[][] = [];
        let instance = 0;
        await withRequester(async (_actor, events) => {
          submitted.push([...events]);
          if (submitted.length > 1) return { uiPatches: [], effects: [], nextWake: null, status: { tag: "idle" } };
          return {
            uiPatches: [],
            effects: [
              { tag: "spawn-job", val: { job: 7n, kind: "semio.puzzle3d.fill", input: new Uint8Array([4, 2]), placement: { tag: "isolated" } } },
              { tag: "send-message", val: { target: { tag: "shell", val: String(instance) }, payload: Array.from(encodeAppFrame({ Invocation: { in_reply_to: 0, output: bytes(fixture.response), diagnostics: bytes([]), ui_scope: bytes(fixture.completion.uiScope), history_patch: bytes(fixture.completion.historyPatch), messages: [], mutations: [], inverse_group: [] } })) } },
            ],
            nextWake: null,
            status: { tag: "idle" },
          };
        }, async (handle, opened) => {
          instance = opened;
          const response = await handle.captureExtensionCompletion!(instance, BigInt(fixture.requestIds[2]!)).complete({ ok: encodePackValue(fixture.response) });
          expect(response.requestedEffects).toEqual([]);
          for (let turn = 0; turn < 64 && !submitted.some(events => events.some(event => event.kind === "job-completed")); turn += 1) await new Promise(resolve => setTimeout(resolve, 0));
          expect(started).toEqual([{ job: 7n, kind: "semio.puzzle3d.fill", input: [4, 2] }]);
          expect(stepped).toEqual([7n, 7n, 7n]);
          const completion = submitted.flat().find(event => event.kind === "job-completed");
          expect(completion?.payload).toEqual({ job: 7n, outcome: { tag: "ok", val: [1] } });
        }, {
          jobs: {
            start: (job, kind, input) => { started.push({ job, kind, input: Array.from(input) }); },
            step: (job) => { stepped.push(job); return stepped.length < 3 ? { status: "running" } : { status: "done", value: new Uint8Array([1]) }; },
          },
        });
      });

      // ⛽️ Ticket 26/09/02/PUZZLE-3D-END-TO-END W-F5. W-J's driver bounded a job's LIFETIME at a
      // constant `PLUGIN_JOB_STEP_LIMIT = 1 << 16` and cancelled whatever was still running there. A
      // `semio.puzzle3d.fill` plan of up to 1 000 placements reaches that in ~40 s of ticking, and the
      // cancel tore the guest's live envelope down mid-run (browser-measured 2026-09-10: `unreachable`
      // → `shard 0 lost` → `actor-activation.revoked`). The native host bounds the same jobs PER SLICE
      // and never over their lifetime — `ShardLoop::pump` grants one `job_budget_from_grant` per turn
      // and has no step counter at all — so this driver must agree: same budget every slice, terminal
      // only from the guest.
      it("bounds a job per slice and never ends one the guest is still running", async () => {
        const { encodeAppFrame } = await import("@semio-tech/framework-os");
        const { default: fixture } = await import("../../🧱️elements/🏛️ShellHost/🧫️fixtures/🔣️extension-invocation.json");
        const bytes = (value: unknown) => Array.from(encodePackValue(value));
        const legacyHostLifetimeCap = 1 << 16;
        const slicesBeforeTerminal = legacyHostLifetimeCap + 1_024;
        const budgets = new Set<string>();
        const cancelled: bigint[] = [];
        const submitted: ShardEventEnvelope[][] = [];
        let stepped = 0;
        let instance = 0;
        await withRequester(async (_actor, events) => {
          submitted.push([...events]);
          if (submitted.length > 1) return { uiPatches: [], effects: [], nextWake: null, status: { tag: "idle" } };
          return {
            uiPatches: [],
            effects: [
              { tag: "spawn-job", val: { job: 11n, kind: "semio.puzzle3d.fill", input: new Uint8Array([1]), placement: { tag: "isolated" } } },
              { tag: "send-message", val: { target: { tag: "shell", val: String(instance) }, payload: Array.from(encodeAppFrame({ Invocation: { in_reply_to: 0, output: bytes(fixture.response), diagnostics: bytes([]), ui_scope: bytes(fixture.completion.uiScope), history_patch: bytes(fixture.completion.historyPatch), messages: [], mutations: [], inverse_group: [] } })) } },
            ],
            nextWake: null,
            status: { tag: "idle" },
          };
        }, async (handle, opened) => {
          instance = opened;
          await handle.captureExtensionCompletion!(instance, BigInt(fixture.requestIds[2]!)).complete({ ok: encodePackValue(fixture.response) });
          for (let turn = 0; turn < 4_096 && !submitted.some(events => events.some(event => event.kind === "job-completed")); turn += 1) await new Promise(resolve => setTimeout(resolve, 0));
          expect(stepped).toBe(slicesBeforeTerminal);
          expect(cancelled).toEqual([]);
          expect([...budgets]).toEqual([`${DEFAULT_SHARD_BUDGET.fuel}/${DEFAULT_SHARD_BUDGET.wallMs}`]);
          const completion = submitted.flat().find(event => event.kind === "job-completed");
          expect(completion?.payload).toEqual({ job: 11n, outcome: { tag: "ok", val: [9] } });
        }, {
          jobs: {
            step: (_job, budget) => {
              stepped += 1;
              budgets.add(`${budget.fuel}/${budget.deadlineMs}`);
              return stepped < slicesBeforeTerminal ? { status: "running" } : { status: "done", value: new Uint8Array([9]) };
            },
            cancel: (job) => { cancelled.push(job); },
          },
        });
      });

      it("reconciles failed actual actor bindings without giving a successor stale control", async () => {
        const { default: fixture } = await import("../../🧱️elements/🔌️PluginRuntime/📡️backbone/🧫️fixtures/🔣️.json");
        const { decodeDocumentBackboneControlV1, encodeDocumentBackboneControlV1 } = await import("../../../../🔌️plugin/📡️backbone/🔗️binding/🟦️.ts");
        for (const row of fixture.bindingRetirement) {
          const commands: string[] = [];
          let instance = 0, current = true;
          await withRequester(async (_actor, events) => {
            const effects: WireVariant[] = [];
            for (const event of events) {
              if (event.kind !== "message") continue;
              const value = event.payload as { source: { tag: string }; payload: number[] };
              if (value.source.tag !== "shell") continue;
              const command = decodeDocumentBackboneControlV1(Uint8Array.from(value.payload));
              commands.push(command.operation);
              const firstBind = command.bindingGeneration === 1n && command.operation === "bind";
              const refused = firstBind && row.result === "refused";
              if (firstBind && row.result === "transport") throw new Error("transport lost");
              if (firstBind && row.result === "replaced") current = false;
              const effect: WireVariant = { tag: "send-message", val: { target: { tag: "shell", val: instance }, payload: encodeDocumentBackboneControlV1({ ...command, schema: "semio.plugin.document-backbone-binding-receipt.v1", operation: refused ? "refused" : command.operation === "bind" ? "bound" : "retired", ...(refused ? { code: "binding-refused" } : {}) }) } };
              effects.push(effect);
              if (firstBind && row.result === "duplicate") effects.push(effect);
            }
            return { uiPatches: [], effects, nextWake: null, status: { tag: "idle" } };
          }, async (handle, opened) => {
            instance = opened;
            const source = { runtimeKey: "map", clientInstanceId: "client-a", scope: null, current: () => current, send: () => {} };
            await expect(handle.bindDocumentPort!(instance, source)).rejects.toThrow();
            expect(commands, row.result).toEqual(row.sequence);
            current = true;
            const successor = await handle.bindDocumentPort!(instance, { ...source, clientInstanceId: "client-b" });
            await successor.retire();
            expect(commands, row.result).toEqual([...row.sequence, "bind", "retire"]);
          });
        }
        console.log("[DEBUG] Actual PluginRuntime bind recovery: refused=1 uncertain=2 stale-presentation=1 successors=4");
      });
  
      it("binds the actual actor document port and retires it before guest disposal", async () => {
        const { decodeDocumentBackboneControlV1, encodeDocumentBackboneControlV1 } = await import("../../../../🔌️plugin/📡️backbone/🔗️binding/🟦️.ts");
        const { encodeBackboneMessage } = await import("@semio-tech/framework-os");
        const outgoing = encodeBackboneMessage({ kind: "ack", opIds: ["operation"] });
        let outgoingUri = "actor://map";
        const commands: string[] = [], delivered: number[][] = [], sent: number[][] = [];
        let instance = 0;
        await withRequester(async (_actor, events) => {
          const effects: WireVariant[] = [];
          for (const event of events) {
            if (event.kind !== "message") continue;
            const value = event.payload as { source: { tag: string; val: string }; payload: number[] };
            if (value.source.tag === "shell") {
              const command = decodeDocumentBackboneControlV1(Uint8Array.from(value.payload));
              commands.push(command.operation);
              effects.push({ tag: "send-message", val: { target: { tag: "shell", val: instance }, payload: encodeDocumentBackboneControlV1({ ...command, schema: "semio.plugin.document-backbone-binding-receipt.v1", operation: command.operation === "bind" ? "bound" : "retired" }) } });
            } else {
              delivered.push([...value.payload]);
              effects.push({ tag: "send-message", val: { target: { tag: "backbone", val: outgoingUri }, payload: outgoing } });
            }
          }
          return { uiPatches: [], effects, nextWake: null, status: { tag: "idle" } };
        }, async (handle, opened, activation) => {
          instance = opened;
          const source = { runtimeKey: "map", clientInstanceId: "client-a", scope: null };
          const port = await handle.bindDocumentPort!(instance, { ...source, current: () => true, send: payload => { sent.push([...payload]); } });
          expect(commands).toEqual(["bind"]);
          expect(await port.receive(source, outgoing)).toBe(true);
          expect(delivered).toEqual([[...outgoing]]);
          expect(sent).toEqual([[...outgoing]]);
          outgoingUri = "actor://foreign";
          await expect(port.receive(source, outgoing)).rejects.toThrow("actor-document-port.foreign-uri");
          expect(activation.guardedTurns()).toBeGreaterThanOrEqual(2);
          await handle.destroyApp(instance);
          expect(commands).toEqual(["bind", "retire"]);
          expect(await port.receive(source, outgoing)).toBe(false);
        });
      });
  
      it("awaits actual actor handle disposal until its exact document retirement settles", async () => {
        const { default: fixture } = await import("../../🧱️elements/🔌️PluginRuntime/📡️backbone/🧫️fixtures/🔣️.json");
        const { default: deepEqual } = await import("fast-deep-equal");
        const { decodeDocumentBackboneControlV1, encodeDocumentBackboneControlV1 } = await import("../../../../🔌️plugin/📡️backbone/🔗️binding/🟦️.ts");
        let release!: () => void, entered!: () => void;
        const gate = new Promise<void>(resolve => { release = resolve; });
        const retiring = new Promise<void>(resolve => { entered = resolve; });
        const events: string[] = [];
        await withRequester(async (_actor, input) => {
          const effects: WireVariant[] = [];
          for (const event of input) {
            if (event.kind !== "message") continue;
            const value = event.payload as { source: { tag: string }; payload: number[] };
            if (value.source.tag !== "shell") continue;
            const command = decodeDocumentBackboneControlV1(Uint8Array.from(value.payload));
            events.push(command.operation);
            if (command.operation === "retire") { entered(); await gate; events.push("retired"); }
            effects.push({ tag: "send-message", val: { target: { tag: "shell", val: command.instanceId }, payload: encodeDocumentBackboneControlV1({ ...command, schema: "semio.plugin.document-backbone-binding-receipt.v1", operation: command.operation === "bind" ? "bound" : "retired" }) } });
          }
          return { uiPatches: [], effects, nextWake: null, status: { tag: "idle" } };
        }, async (handle, instance) => {
          const port = await handle.bindDocumentPort!(instance, { runtimeKey: "map", clientInstanceId: "client", scope: null, current: () => true, send: () => {} });
          const closing = handle.dispose();
          try {
            expect(closing).toBeInstanceOf(Promise);
            expect(handle.dispose()).toBe(closing);
            let settled = false;
            void closing.then(() => { settled = true; });
            await retiring;
            expect(settled).toBe(false);
            expect(events).toEqual(fixture.disposal.bound.slice(0, 2));
            expect(await port.receive({ runtimeKey: "map", clientInstanceId: "client", scope: null }, Uint8Array.of(1))).toBe(false);
            await expect(handle.createApp("late")).rejects.toThrow("plugin-handle.closed");
          } finally { release(); await closing; }
          expect(deepEqual(events, fixture.disposal.bound)).toBe(true);
        }, { dispose: () => { events.push("dispose"); } });
        console.log("[DEBUG] Actual actor disposal waits for exact Retired and closes creation admission");
      });

      it("awaits actual actor pending creation without opening an orphan after disposal", async () => {
        const { default: fixture } = await import("../../🧱️elements/🔌️PluginRuntime/📡️backbone/🧫️fixtures/🔣️.json");
        const { default: deepEqual } = await import("fast-deep-equal");
        for (const row of fixture.disposal.opening) {
          let release!: () => void, entered!: () => void, target = "";
          const gate = new Promise<void>(resolve => { release = resolve; });
          const pending = new Promise<void>(resolve => { entered = resolve; });
          const events: string[] = [];
          await withRequester(async () => ({ uiPatches: [], effects: [], nextWake: null, status: { tag: "idle" } }), async (handle, instance) => {
            const original = `extension-requester#${instance}`;
            target = "pending";
            const opening = handle.createApp("pending");
            const observed = opening.then(value => ({ value }), error => ({ error }));
            await pending;
            const closing = handle.dispose();
            try {
              expect(closing).toBeInstanceOf(Promise);
              let settled = false;
              void closing.then(() => { settled = true; }, () => {});
              await Promise.resolve();
              expect(settled).toBe(false);
              expect(target).not.toBe(original);
              expect(events).not.toContain("dispose");
            } finally { release(); await closing; }
            expect(await observed).toMatchObject({ error: { message: "plugin-handle.closed" } });
            expect(deepEqual(events, row.events), row.phase).toBe(true);
          }, {
            activate: async actor => {
              if (!target) return;
              target = actor; events.push("activate");
              if (row.phase === "activation") { entered(); await gate; }
              events.push("activated");
            },
            open: async actor => {
              if (actor !== target) return;
              events.push("open");
              if (row.phase === "open") { entered(); await gate; }
              events.push("opened");
            },
            dispose: actor => { if (actor === target) events.push("dispose"); },
          });
        }
        console.log("[DEBUG] Actual actor disposal retains activation/open flights and prevents late instance publication");
      });

      it("rejects a framed completion fault without swallowing its structured fields", async () => {
        const { default: fixture } = await import("../../🧱️elements/🏛️ShellHost/🧫️fixtures/🔣️extension-invocation.json");
        const { encodeAppFrame } = await import("@semio-tech/framework-os");
        let instance = 0;
        await withRequester(async () => ({ uiPatches: [], effects: [{ tag: "send-message", val: { target: { tag: "shell", val: String(instance) }, payload: Array.from(encodeAppFrame({ Error: { in_reply_to: null, fault: Array.from(encodePackValue(fixture.fault)), report: [] } })) } }], nextWake: null, status: { tag: "idle" } }), async (handle, opened) => {
          instance = opened;
          await expect(handle.captureExtensionCompletion!(instance, BigInt(fixture.requestId)).complete({ ok: encodePackValue(fixture.response) })).rejects.toMatchObject({ fault: fixture.fault });
        });
      });
  
      it("does not publish a late completion after its originating instance is destroyed", async () => {
        const { default: fixture } = await import("../../🧱️elements/🏛️ShellHost/🧫️fixtures/🔣️extension-invocation.json");
        const entered = Promise.withResolvers<void>();
        const result = Promise.withResolvers<WireTurnResult>();
        let turns = 0;
        await withRequester(async () => { turns += 1; entered.resolve(); return result.promise; }, async (handle, instance) => {
          const completion = handle.captureExtensionCompletion!(instance, BigInt(fixture.requestId)).complete({ ok: encodePackValue(fixture.response) });
          const observed = expect(completion).rejects.toThrow("no actor");
          await entered.promise;
          const retiring = handle.destroyApp(instance);
          let retired = false;
          void retiring.then(() => { retired = true; });
          await Promise.resolve();
          expect(retired).toBe(false);
          result.resolve({ uiPatches: [], effects: [{ tag: "notify", val: { message: fixture.completion.notification } }], nextWake: null, status: { tag: "idle" } });
          await observed;
          await retiring;
          expect(turns).toBe(1);
          expect(retainedWindowByActor.has(`extension-requester#${instance}`)).toBe(false);
        });
      });
  
      it("captures activation before queued completion and rejects a same-name replacement before dispatch", async () => {
        const { default: fixture } = await import("../../🧱️elements/🏛️ShellHost/🧫️fixtures/🔣️extension-invocation.json");
        let dispatched = 0;
        await withRequester(async () => { dispatched += 1; return { uiPatches: [], effects: [], nextWake: null, status: { tag: "idle" } }; }, async (handle, instance, activation) => {
          const entered = Promise.withResolvers<void>();
          const release = Promise.withResolvers<void>();
          const held = serializeCommandIngressForActor(`extension-requester#${instance}`, async () => { entered.resolve(); await release.promise; });
          await entered.promise;
          const completing = handle.captureExtensionCompletion!(instance, BigInt(fixture.requestId)).complete({ ok: encodePackValue(fixture.response) });
          const observed = expect(completing).rejects.toThrow("actor-activation.revoked");
          const captures = activation.captures();
          activation.replace();
          release.resolve();
          await held;
          await observed;
          expect(captures).toBe(1);
          expect(activation.captures()).toBe(1);
          expect(dispatched).toBe(0);
        });
      });
  
      it("refuses publication after in-flight activation replacement without running a new continuation", async () => {
        const { default: fixture } = await import("../../🧱️elements/🏛️ShellHost/🧫️fixtures/🔣️extension-invocation.json");
        const entered = Promise.withResolvers<void>();
        const release = Promise.withResolvers<WireTurnResult>();
        let dispatched = 0;
        await withRequester(async () => { dispatched += 1; entered.resolve(); return release.promise; }, async (handle, instance, activation) => {
          const completing = handle.captureExtensionCompletion!(instance, BigInt(fixture.requestId)).complete({ ok: encodePackValue(fixture.response) });
          const observed = expect(completing).rejects.toThrow("actor-activation.revoked");
          await entered.promise;
          activation.replace();
          release.resolve({ uiPatches: [], effects: [{ tag: "notify", val: { message: "stale activation" } }], nextWake: null, status: { tag: "idle" } });
          await observed;
          expect(activation.captures()).toBe(1);
          expect(activation.guardedTurns()).toBe(1);
          expect(dispatched).toBe(1);
          expect(retainedWindowByActor.get(`extension-requester#${instance}`)?.size ?? 0).toBe(0);
        });
      });
  
      it("keeps the original activation lease through every completion continuation", async () => {
        const { default: fixture } = await import("../../🧱️elements/🏛️ShellHost/🧫️fixtures/🔣️extension-invocation.json");
        const entered = Promise.withResolvers<void>();
        const release = Promise.withResolvers<WireTurnResult>();
        let dispatched = 0;
        await withRequester(async () => {
          dispatched += 1;
          if (dispatched === 1) return { uiPatches: [], effects: [], nextWake: null, status: { tag: "more-work" } };
          entered.resolve();
          return release.promise;
        }, async (handle, instance, activation) => {
          const completing = handle.captureExtensionCompletion!(instance, BigInt(fixture.requestId)).complete({ ok: encodePackValue(fixture.response) });
          const observed = expect(completing).rejects.toThrow("actor-activation.revoked");
          await entered.promise;
          activation.replace();
          release.resolve({ uiPatches: [], effects: [], nextWake: null, status: { tag: "idle" } });
          await observed;
          expect(dispatched).toBe(2);
          expect(activation.captures()).toBe(1);
          expect(activation.guardedTurns()).toBe(2);
        });
      });
  
      it("rejects a completion captured before evaluation when that activation is later replaced", async () => {
        const { default: fixture } = await import("../../🧱️elements/🏛️ShellHost/🧫️fixtures/🔣️extension-invocation.json");
        let dispatched = 0;
        await withRequester(async () => { dispatched += 1; return { uiPatches: [], effects: [], nextWake: null, status: { tag: "idle" } }; }, async (handle, instance, activation) => {
          const completion = handle.captureExtensionCompletion!(instance, BigInt(fixture.requestId));
          activation.replace();
          await expect(completion.complete({ ok: encodePackValue(fixture.response) })).rejects.toThrow("actor-activation.revoked");
          expect(dispatched).toBe(0);
          expect(activation.captures()).toBe(1);
        });
      });
  
      it("claims one completion submission per captured request", async () => {
        const { default: fixture } = await import("../../🧱️elements/🏛️ShellHost/🧫️fixtures/🔣️extension-invocation.json");
        let dispatched = 0;
        await withRequester(async () => { dispatched += 1; return { uiPatches: [], effects: [], nextWake: null, status: { tag: "idle" } }; }, async (handle, instance) => {
          const completion = handle.captureExtensionCompletion!(instance, BigInt(fixture.requestId));
          expect(Object.isFrozen(completion)).toBe(true);
          expect([completion.instanceId, completion.req]).toEqual([instance, BigInt(fixture.requestId)]);
          await completion.complete({ ok: encodePackValue(fixture.response) });
          await expect(completion.complete({ ok: encodePackValue(fixture.response) })).rejects.toThrow("extension.completion-already-submitted");
          expect(dispatched).toBe(1);
        });
      });
    });
  
    function encodeForeignStepBytes(step: {
      readonly target: { readonly artifactId: string; readonly artifactKind: string };
      readonly mutationId: string;
      readonly payload: readonly number[];
      readonly label: string;
    }): Uint8Array {
      return encodePackValue(step);
    }
  
    function encodeFaultBytes(code: string): Uint8Array {
      return encodePackValue({ origin: "os", code, severity: "error", message: code, scope: {}, retryable: false });
    }
  
    describe("command-ingress fault diagnostics", () => {
      it("decodes the normalized scalar-wire fault payload instead of hiding the terminal cause", () => {
        const bytes = encodeFaultBytes("plugin.command-rejected");
        expect(commandIngressFaultDisplay({ tag: "fault", val: { cursor: {}, fault: { tag: "fault", val: Array.from(bytes) } } })).toBe("plugin.command-rejected: plugin.command-rejected");
      });
    });
  
    describe("instance-open retained UI lifecycle", () => {
      it("preserves document effects returned by a refresh even before any surface is retained", async () => {
        const { default: fixture } = await import("../../../../🔌️plugin/⚛️reactor/🧫️fixtures/🔣️.json");
        const response = retainedUiRefreshResponse(7, { viewState: {} }, new Map(), [{ tag: "load-document", val: fixture.effects.loadDocument }]);
        expect(response.requestedEffects).toEqual([{ loadDocument: fixture.effects.loadDocument }]);
      });
  
      it("reports a refresh fault frame instead of returning an unchanged surface", async () => {
        const { encodeAppFrame } = await import("@semio-tech/framework-os");
        const { default: fixture } = await import("../../../../🔌️plugin/⚛️reactor/🧫️fixtures/🔣️.json");
        const payload = encodeAppFrame({ Error: { in_reply_to: null, fault: Array.from(encodeFaultBytes(fixture.wire.fault)), report: [] } });
        expect(() => retainedUiRefreshResponse(7, { viewState: {} }, new Map(), [{ tag: "send-message", val: { target: { tag: "shell", val: "7" }, payload } }])).toThrow(fixture.wire.fault);
      });
  
      it("refreshes window and panel surfaces from the language-agnostic ownership cases", async () => {
        const { default: fixture } = await import("../../🧱️elements/🔌️PluginRuntime/🧫️fixtures/🔄️surface-refresh.json");
        for (const testCase of fixture.cases) {
          const bodyKeys = uiRefreshBodyKeys(testCase.request);
          expect(bodyKeys, testCase.name).toEqual(testCase.bodyKeys);
          const retained = new Map<string, RetainedSurface>();
          for (const target of [...(testCase.request.windows ?? []), ...(testCase.request.panels ?? [])]) {
            const bodyKey = target.bodyKey ?? target.key;
            const root: UiNodeRecord = {
              id: 0, key: bodyKey, component: { type: "text", value: bodyKey, emphasize: null, dataAttributes: null },
              layout: { kind: "leaf", width: "hug", height: "hug" }, style: { variant: "plain", size: "md", density: "standard", tone: "neutral", emphasis: "regular" }, activity: "idle", disabled: false,
              transition: null, accessibility: { label: null, description: null, live: "off", shortcut: null, hidden: false }, bindings: [], menu: null, children: [],
            };
            const { surface, desynced } = applyUiPatchToRetained(null, {
              surface: target.key, revision: 1, baseRevision: 0,
              ops: [{ type: "upsert", ...root }, { type: "setRoot", id: 0 }],
            });
            expect(desynced).toBe(false);
            retained.set(retainedSurfaceId(7, target.key), surface!);
          }
          const response = retainedUiRefreshResponse(7, testCase.request, retained);
          expect(response.windows?.map((entry) => entry.key)).toEqual(testCase.windowKeys);
          expect(response.panels?.map((entry) => entry.key)).toEqual(testCase.panelKeys);
          for (const entry of [...(response.windows ?? []), ...(response.panels ?? [])]) {
            expect(entry.value).toMatchObject({ component: { type: "text" } });
            expect(entry.hash).not.toBe("");
          }
        }
      });

      /** 🪞️ Ticket 26/09/02 wave B4. `buildUiRefreshRequest` stamps every requested entry with the
       * host's cached hash so an unchanged body can answer without a payload; before this law both
       * projectors ignored it and rebuilt every requested tree on every refresh — measured on the live
       * Nakagin document switch at 51 projections / 1 275 retained-node walks for ONE switch, and,
       * worse, a fresh `BuiltNode` per unchanged body defeats `mergeRecordPreservingIdentity` and
       * re-reconciles the whole shell subtree. The clause that matters is `value === undefined` with
       * the hash echoed back: that is what lets the shell keep its exact object reference. */
      it("omits the payload of a requested body whose hash the host already holds and re-projects only the one that changed", () => {
        const body = (key: string, value: string): UiNodeRecord => ({
          id: 0, key, component: { type: "text", value, emphasize: null, dataAttributes: null },
          layout: { kind: "leaf", width: "hug", height: "hug" }, style: { variant: "plain", size: "md", density: "standard", tone: "neutral", emphasis: "regular" }, activity: "idle", disabled: false,
          transition: null, accessibility: { label: null, description: null, live: "off", shortcut: null, hidden: false }, bindings: [], menu: null, children: [],
        });
        const publish = (previous: RetainedSurface | null, key: string, revision: number, value: string): RetainedSurface => {
          const { surface, desynced } = applyUiPatchToRetained(previous, { surface: key, revision, baseRevision: revision - 1, ops: [{ type: "upsert", ...body(key, value) }, { type: "setRoot", id: 0 }] });
          expect(desynced).toBe(false);
          return surface!;
        };
        const retained = new Map<string, RetainedSurface>([
          [retainedSurfaceId(7, "steady"), publish(null, "steady", 1, "steady-one")],
          [retainedSurfaceId(7, "moving"), publish(null, "moving", 1, "moving-one")],
        ]);
        const request = { viewState: {}, windows: [{ key: "steady", bodyKey: "steady" }, { key: "moving", bodyKey: "moving" }] };
        const first = retainedUiRefreshResponse(7, request, retained);
        expect(first.windows?.map((entry) => entry.value !== undefined)).toEqual([true, true]);
        const hashes = Object.fromEntries((first.windows ?? []).map((entry) => [entry.key, entry.hash]));

        const echoed = retainedUiRefreshResponse(7, { ...request, windows: request.windows.map((target) => ({ ...target, hash: hashes[target.key] })) }, retained);
        expect(echoed.windows).toEqual([{ key: "steady", hash: hashes.steady }, { key: "moving", hash: hashes.moving }]);
        expect(echoed.windows?.every((entry) => entry.value === undefined)).toBe(true);

        retained.set(retainedSurfaceId(7, "moving"), publish(retained.get(retainedSurfaceId(7, "moving"))!, "moving", 2, "moving-two"));
        const partial = retainedUiRefreshResponse(7, { ...request, windows: request.windows.map((target) => ({ ...target, hash: hashes[target.key] })) }, retained);
        expect(partial.windows?.find((entry) => entry.key === "steady")).toEqual({ key: "steady", hash: hashes.steady });
        const moved = partial.windows?.find((entry) => entry.key === "moving");
        expect(moved?.hash).not.toBe(hashes.moving);
        expect(moved?.value).toMatchObject({ component: { type: "text", value: "moving-two" } });
        console.info(`[DEBUG] refresh skip: ${JSON.stringify({ projectedFirst: 2, skippedOnEcho: 2, projectedAfterOneEdit: 1 })}`);
      });

      it("holds the skip predicate to exact hash equality on a rooted surface", () => {
        expect(uiRefreshSectionUnchanged("abc", { root: 0, hash: "abc" })).toBe(true);
        expect(uiRefreshSectionUnchanged("abc", { root: 0, hash: "abd" })).toBe(false);
        expect(uiRefreshSectionUnchanged(undefined, { root: 0, hash: "abc" })).toBe(false);
        expect(uiRefreshSectionUnchanged("", { root: 0, hash: "" })).toBe(false);
        expect(uiRefreshSectionUnchanged("abc", { root: null, hash: "abc" })).toBe(false);
      });

      it("acknowledges only patches that identify the exact retained surface", () => {
        const receipt = { lifetime: { activationGeneration: 1n, instanceId: 4, guestLifetime: 1n }, patchSequence: 1n };
        const turn = { uiPatches: [], effects: [], nextWake: null, status: { tag: "idle" }, uiPatchReceipt: encodeActorUiPatchReceipt(receipt) } as unknown as WireTurnResult;
        expect(
          patchAckEvents(turn, [
            { surface: pluginSurfaceRef(4, "workflow"), revision: 9n, baseRevision: 8n, ops: [] },
            { revision: 1n, baseRevision: 0n, ops: [] },
          ]),
        ).toEqual([{ kind: "patch-ack", payload: { receipt, surface: pluginSurfaceRef(4, "workflow"), revision: 9n } }]);
      });
  
      it("retains the first render patch so an unchanged surface-visible probe can reuse it", () => {
        const actorId = "initial-render-retention-test#1";
        expect(pluginSurfaceRef(1, "workflow")).toEqual({ instance: 1, surface: "workflow" });
        const root: UiNodeRecord = {
          id: 0,
          key: "root",
          component: { type: "text", value: "ready", emphasize: null, dataAttributes: null },
          layout: { kind: "leaf", width: "hug", height: "hug" },
          style: { variant: "plain", size: "md", density: "standard", tone: "neutral", emphasis: "regular" },
          activity: "idle",
          disabled: false,
          transition: null,
          accessibility: { label: null, description: null, live: "off", shortcut: null, hidden: false },
          bindings: [],
          menu: null,
          children: [],
        };
        retainedWindowByActor.delete(actorId);
        try {
          retainTurnUiPatches(actorId, {
            uiPatches: [{ revision: 1n, baseRevision: 0n, ops: [{ tag: "upsert", val: { node: Array.from(encodePackValue(root)) } }, { tag: "set-root", val: 0n }] }],
          });
          const retained = retainedWindowByActor.get(actorId)?.get("window");
          expect(retained).toMatchObject({ surface: "window", revision: 1, root: 0 });
          expect(() => retainedSurfaceHash(retainedSurfaceToSnapshot(retained!))).not.toThrow();
          expect(retainedSurfaceToBuiltNode(retained!)).toMatchObject({ key: "root", component: { type: "text", value: "ready" }, children: [] });
        } finally {
          retainedWindowByActor.delete(actorId);
        }
      });
    });
  
    type FakeHandleOptions = {
      readonly prepareForeign?: (instanceId: number) => readonly Uint8Array[];
      readonly prepareRejection?: Uint8Array;
      readonly commitRejects?: boolean;
      readonly pack?: { readonly pack: Uint8Array; readonly spr: Uint8Array } | null;
    };
  
    const fakeLocalInteraction = (instanceId: number): LocalInteractionCapture => ({
      identity: { appInstanceId: instanceId, generation: "1", revision: "0".repeat(64), documentRevision: "0".repeat(64), topologyRevision: "0".repeat(64) },
      state: { selection: {}, activeMode: {}, activeGranularity: {} },
    });
  
    function fakeHandle(pluginId: string, calls: string[], commitOrder: string[], options: FakeHandleOptions = {}): PluginWasmHandle {
      return {
        pluginId,
        manifest: {} as unknown as PluginManifest,
        createApp: async () => 0,
        destroyApp: async () => {},
        takeSegmentedDownloadChunk: async () => undefined,
        handleAction: async () => ({ output: null, mutations: [], inverseGroup: { invocationId: "", mutations: [], inverseMutations: [] } }),
        refreshUi: async () => ({}),
        contextMenu: async () => [],
        readHistory: async () => ({ cursor: 0 }) as unknown as HistoryPatch,
        readLocalInteraction: async (instanceId) => fakeLocalInteraction(instanceId),
        subscribeOperationCompletions: () => () => {},
        readWindowConfigPacks: async () => [],
        loadWindowConfigPack: async () => {},
        documentPack: (instanceId) => (options.pack !== undefined ? options.pack : { pack: new Uint8Array([1]), spr: new Uint8Array([instanceId]) }),
        transactionPrepare: async (instanceId, _txnId, _request) => {
          calls.push(`${pluginId}:${instanceId}:prepare`);
          if (options.prepareRejection) return { foreign: [], rejection: options.prepareRejection };
          return { foreign: options.prepareForeign?.(instanceId) ?? [], rejection: null };
        },
        transactionCommit: async (instanceId, _txnId) => {
          if (options.commitRejects) return { rejection: encodeFaultBytes("transaction.commit-failed") };
          commitOrder.push(`${pluginId}:${instanceId}`);
          return { editId: `edit-${pluginId}-${instanceId}` };
        },
        transactionRollback: async (instanceId) => {
          calls.push(`${pluginId}:${instanceId}:rollback`);
        },
        transactionUndo: async (instanceId) => {
          calls.push(`${pluginId}:${instanceId}:undo`);
        },
        transactionRedo: async (instanceId) => {
          calls.push(`${pluginId}:${instanceId}:redo`);
        },
        setMergePolicy: async () => {},
        resolveConflict: async () => ({ mergeReport: null, conflicts: null }),
        readConflicts: async () => [],
        dispose: async () => {},
      };
    }
  
    describe("PluginRuntime TransactionCoordinator", () => {
      it("runs proposal -> prepare x2 -> commit, committing in reverse discovery order", async () => {
        const calls: string[] = [];
        const commitOrder: string[] = [];
        const directory = new InstanceDirectory();
        directory.register("artifact-b", { pluginId: "b-plugin", instanceId: 20, artifactKind: "s.b.doc" });
        const router = new ArtifactMutationRouter();
        router.registerOwner("s.b.doc", "s.b#mutate");
        const plugins = new Map<string, PluginWasmHandle>([
          ["a-plugin", fakeHandle("a-plugin", calls, commitOrder)],
          ["b-plugin", fakeHandle("b-plugin", calls, commitOrder)],
        ]);
        const coordinator = new TransactionCoordinator(directory, router, plugins);
  
        const foreign = encodeForeignStepBytes({ target: { artifactId: "artifact-b", artifactKind: "s.b.doc" }, mutationId: "s.b#mutate", payload: [7], label: "duplicate" });
        const outcome = await coordinator.run({
          initiatorPluginId: "a-plugin",
          initiatorInstanceId: 10,
          initiatorArtifactId: "artifact-a",
          initiatorArtifactKind: "s.a.doc",
          localOps: [new Uint8Array([1])],
          description: "duplicate widget",
          foreign: [foreign],
        });
  
        expect(outcome.ok).toBe(true);
        if (!outcome.ok) return;
        expect(calls).toEqual(["a-plugin:10:prepare", "b-plugin:20:prepare"]);
        // 🎯️ Reverse discovery order: initiator (a) discovered first, foreign target (b) discovered
        // second — commit visits b before a (contract freeze §5.6).
        expect(commitOrder).toEqual(["b-plugin:20", "a-plugin:10"]);
        expect(outcome.editIds.get("artifact-a")).toBe("edit-a-plugin-10");
        expect(outcome.editIds.get("artifact-b")).toBe("edit-b-plugin-20");
      });
  
      it("undoGroup fans TransactionUndo out to every member of a completed transaction", async () => {
        const calls: string[] = [];
        const commitOrder: string[] = [];
        const directory = new InstanceDirectory();
        directory.register("artifact-b", { pluginId: "b-plugin", instanceId: 20, artifactKind: "s.b.doc" });
        const router = new ArtifactMutationRouter();
        router.registerOwner("s.b.doc", "s.b#mutate");
        const plugins = new Map<string, PluginWasmHandle>([
          ["a-plugin", fakeHandle("a-plugin", calls, commitOrder)],
          ["b-plugin", fakeHandle("b-plugin", calls, commitOrder)],
        ]);
        const coordinator = new TransactionCoordinator(directory, router, plugins);
        const foreign = encodeForeignStepBytes({ target: { artifactId: "artifact-b", artifactKind: "s.b.doc" }, mutationId: "s.b#mutate", payload: [7], label: "x" });
        const outcome = await coordinator.run({
          initiatorPluginId: "a-plugin",
          initiatorInstanceId: 10,
          initiatorArtifactId: "artifact-a",
          initiatorArtifactKind: "s.a.doc",
          localOps: [new Uint8Array([1])],
          description: "x",
          foreign: [foreign],
        });
        expect(outcome.ok).toBe(true);
        if (!outcome.ok) return;
  
        calls.length = 0;
        const undoResult = await coordinator.undoGroup(outcome.txnId);
        expect(undoResult.ok).toBe(true);
        expect(new Set(calls)).toEqual(new Set(["a-plugin:10:undo", "b-plugin:20:undo"]));
  
        const unknownUndo = await coordinator.undoGroup("not-a-real-group");
        expect(unknownUndo.ok).toBe(false);
      });
  
      it("rolls back a commit-failed transaction: undoes what already committed, rolls back the rest", async () => {
        const calls: string[] = [];
        const commitOrder: string[] = [];
        const directory = new InstanceDirectory();
        directory.register("artifact-b", { pluginId: "b-plugin", instanceId: 20, artifactKind: "s.doc" });
        directory.register("artifact-c", { pluginId: "c-plugin", instanceId: 30, artifactKind: "s.doc" });
        const router = new ArtifactMutationRouter();
        router.registerOwner("s.doc", "s#mutate");
        const plugins = new Map<string, PluginWasmHandle>([
          ["a-plugin", fakeHandle("a-plugin", calls, commitOrder)],
          ["b-plugin", fakeHandle("b-plugin", calls, commitOrder, { commitRejects: true })],
          ["c-plugin", fakeHandle("c-plugin", calls, commitOrder)],
        ]);
        const coordinator = new TransactionCoordinator(directory, router, plugins);
        const foreignB = encodeForeignStepBytes({ target: { artifactId: "artifact-b", artifactKind: "s.doc" }, mutationId: "s#mutate", payload: [1], label: "x" });
        const foreignC = encodeForeignStepBytes({ target: { artifactId: "artifact-c", artifactKind: "s.doc" }, mutationId: "s#mutate", payload: [2], label: "x" });
        const outcome = await coordinator.run({
          initiatorPluginId: "a-plugin",
          initiatorInstanceId: 10,
          initiatorArtifactId: "artifact-a",
          initiatorArtifactKind: "s.doc",
          localOps: [new Uint8Array([1])],
          description: "x",
          foreign: [foreignB, foreignC],
        });
        expect(outcome).toEqual({ ok: false, code: "transaction.commit-failed" });
        // discovery order was [a, b, c]; commit visits c (succeeds) then b (fails) — c is already
        // committed so it gets undone, a (never reached) gets rolled back alongside the failing b.
        expect(commitOrder).toEqual(["c-plugin:30"]);
        expect(calls).toContain("c-plugin:30:undo");
        expect(calls).toContain("a-plugin:10:rollback");
        expect(calls).toContain("b-plugin:20:rollback");
      });
  
      it("reaches transaction.unknown-target when the initiator plugin has no registered handle", async () => {
        const coordinator = new TransactionCoordinator(new InstanceDirectory(), new ArtifactMutationRouter(), new Map());
        const outcome = await coordinator.run({
          initiatorPluginId: "missing-plugin",
          initiatorInstanceId: 1,
          initiatorArtifactId: "artifact-a",
          initiatorArtifactKind: "s.a.doc",
          localOps: [],
          description: "x",
          foreign: [],
        });
        expect(outcome).toEqual({ ok: false, code: "transaction.unknown-target" });
      });
  
      it("reaches transaction.unknown-target when InstanceDirectory has no entry for the foreign target", async () => {
        const calls: string[] = [];
        const commitOrder: string[] = [];
        const plugins = new Map<string, PluginWasmHandle>([["a-plugin", fakeHandle("a-plugin", calls, commitOrder)]]);
        const coordinator = new TransactionCoordinator(new InstanceDirectory(), new ArtifactMutationRouter(), plugins);
        const foreign = encodeForeignStepBytes({ target: { artifactId: "artifact-unknown", artifactKind: "s.b.doc" }, mutationId: "s.b#mutate", payload: [1], label: "x" });
        const outcome = await coordinator.run({
          initiatorPluginId: "a-plugin",
          initiatorInstanceId: 10,
          initiatorArtifactId: "artifact-a",
          initiatorArtifactKind: "s.a.doc",
          localOps: [],
          description: "x",
          foreign: [foreign],
        });
        expect(outcome).toEqual({ ok: false, code: "transaction.unknown-target" });
      });
  
      it("reaches transaction.unknown-mutation when the router has no entry for a foreign step", async () => {
        const calls: string[] = [];
        const commitOrder: string[] = [];
        const directory = new InstanceDirectory();
        directory.register("artifact-b", { pluginId: "b-plugin", instanceId: 20, artifactKind: "s.b.doc" });
        const plugins = new Map<string, PluginWasmHandle>([
          ["a-plugin", fakeHandle("a-plugin", calls, commitOrder)],
          ["b-plugin", fakeHandle("b-plugin", calls, commitOrder)],
        ]);
        const coordinator = new TransactionCoordinator(directory, new ArtifactMutationRouter(), plugins);
        const foreign = encodeForeignStepBytes({ target: { artifactId: "artifact-b", artifactKind: "s.b.doc" }, mutationId: "s.b#nope", payload: [1], label: "x" });
        const outcome = await coordinator.run({
          initiatorPluginId: "a-plugin",
          initiatorInstanceId: 10,
          initiatorArtifactId: "artifact-a",
          initiatorArtifactKind: "s.a.doc",
          localOps: [],
          description: "x",
          foreign: [foreign],
        });
        expect(outcome).toEqual({ ok: false, code: "transaction.unknown-mutation" });
      });
  
      it("reaches transaction.contribution-not-permitted when a contributed mutation has no planner wired", async () => {
        const calls: string[] = [];
        const commitOrder: string[] = [];
        const directory = new InstanceDirectory();
        directory.register("artifact-b", { pluginId: "b-plugin", instanceId: 20, artifactKind: "s.b.doc" });
        const router = new ArtifactMutationRouter();
        router.registerContributed(
          "s.b.doc",
          "aec-building",
          "b-plugin",
          { mutationId: "s.b#aec-building:add-room", semantics: { verb: "add", entity: "room", kind: "add-room", record: "Room" }, schemaVersion: 1, algorithmVersion: 1 },
          true,
        );
        const plugins = new Map<string, PluginWasmHandle>([
          ["a-plugin", fakeHandle("a-plugin", calls, commitOrder)],
          ["b-plugin", fakeHandle("b-plugin", calls, commitOrder)],
        ]);
        const coordinator = new TransactionCoordinator(directory, router, plugins); // no planner injected
        const foreign = encodeForeignStepBytes({ target: { artifactId: "artifact-b", artifactKind: "s.b.doc" }, mutationId: "s.b#aec-building:add-room", payload: [1], label: "x" });
        const outcome = await coordinator.run({
          initiatorPluginId: "a-plugin",
          initiatorInstanceId: 10,
          initiatorArtifactId: "artifact-a",
          initiatorArtifactKind: "s.a.doc",
          localOps: [],
          description: "x",
          foreign: [foreign],
        });
        expect(outcome).toEqual({ ok: false, code: "transaction.contribution-not-permitted" });
      });
  
      it("plans and prepares a contributed mutation using the target's cached document pack", async () => {
        const calls: string[] = [];
        const commitOrder: string[] = [];
        const directory = new InstanceDirectory();
        directory.register("artifact-b", { pluginId: "b-plugin", instanceId: 20, artifactKind: "s.b.doc" });
        const router = new ArtifactMutationRouter();
        router.registerContributed(
          "s.b.doc",
          "aec-building",
          "b-plugin",
          { mutationId: "s.b#aec-building:add-room", semantics: { verb: "add", entity: "room", kind: "add-room", record: "Room" }, schemaVersion: 1, algorithmVersion: 1 },
          true,
        );
        const seenPlanRequests: { readonly targetPack: Uint8Array; readonly targetSpr: Uint8Array }[] = [];
        const targetPack = { pack: new Uint8Array([9, 9]), spr: new Uint8Array([8]) };
        const plugins = new Map<string, PluginWasmHandle>([
          ["a-plugin", fakeHandle("a-plugin", calls, commitOrder)],
          ["b-plugin", fakeHandle("b-plugin", calls, commitOrder, { pack: targetPack })],
        ]);
        const coordinator = new TransactionCoordinator(directory, router, plugins, async (contributorPluginId, request) => {
          seenPlanRequests.push({ targetPack: request.targetPack, targetSpr: request.targetSpr });
          expect(contributorPluginId).toBe("aec-building");
          return { ops: [new Uint8Array([42])], label: "aec-building add-room" };
        });
        const foreign = encodeForeignStepBytes({ target: { artifactId: "artifact-b", artifactKind: "s.b.doc" }, mutationId: "s.b#aec-building:add-room", payload: [1], label: "x" });
        const outcome = await coordinator.run({
          initiatorPluginId: "a-plugin",
          initiatorInstanceId: 10,
          initiatorArtifactId: "artifact-a",
          initiatorArtifactKind: "s.a.doc",
          localOps: [],
          description: "x",
          foreign: [foreign],
        });
        expect(outcome.ok).toBe(true);
        expect(seenPlanRequests).toEqual([{ targetPack: targetPack.pack, targetSpr: targetPack.spr }]);
      });
  
      it("reaches transaction.cycle when the same (artifact, mutation, payload) step repeats", async () => {
        const calls: string[] = [];
        const commitOrder: string[] = [];
        const directory = new InstanceDirectory();
        directory.register("artifact-b", { pluginId: "b-plugin", instanceId: 20, artifactKind: "s.b.doc" });
        const router = new ArtifactMutationRouter();
        router.registerOwner("s.b.doc", "s.b#mutate");
        const plugins = new Map<string, PluginWasmHandle>([
          ["a-plugin", fakeHandle("a-plugin", calls, commitOrder)],
          ["b-plugin", fakeHandle("b-plugin", calls, commitOrder)],
        ]);
        const coordinator = new TransactionCoordinator(directory, router, plugins);
        const foreign = encodeForeignStepBytes({ target: { artifactId: "artifact-b", artifactKind: "s.b.doc" }, mutationId: "s.b#mutate", payload: [1], label: "x" });
        const outcome = await coordinator.run({
          initiatorPluginId: "a-plugin",
          initiatorInstanceId: 10,
          initiatorArtifactId: "artifact-a",
          initiatorArtifactKind: "s.a.doc",
          localOps: [],
          description: "x",
          foreign: [foreign, foreign],
        });
        expect(outcome).toEqual({ ok: false, code: "transaction.cycle" });
      });
  
      it("reaches transaction.depth-exceeded when a foreign-step chain runs past MAX_TRANSACTION_DEPTH", async () => {
        const calls: string[] = [];
        const commitOrder: string[] = [];
        const directory = new InstanceDirectory();
        const chainLength = MAX_TRANSACTION_DEPTH + 4;
        for (let index = 1; index <= chainLength; index += 1) {
          directory.register(`artifact-${index}`, { pluginId: "chain-plugin", instanceId: index, artifactKind: "s.chain.doc" });
        }
        const router = new ArtifactMutationRouter();
        router.registerOwner("s.chain.doc", "s.chain#step");
        const chainHandle: PluginWasmHandle = {
          pluginId: "chain-plugin",
          manifest: {} as unknown as PluginManifest,
          createApp: async () => 0,
          destroyApp: async () => {},
          takeSegmentedDownloadChunk: async () => undefined,
          handleAction: async () => ({ output: null, mutations: [], inverseGroup: { invocationId: "", mutations: [], inverseMutations: [] } }),
          refreshUi: async () => ({}),
          contextMenu: async () => [],
          readHistory: async () => ({ cursor: 0 }) as unknown as HistoryPatch,
          readLocalInteraction: async (instanceId) => fakeLocalInteraction(instanceId),
          subscribeOperationCompletions: () => () => {},
          readWindowConfigPacks: async () => [],
          loadWindowConfigPack: async () => {},
          documentPack: () => ({ pack: new Uint8Array([1]), spr: new Uint8Array([2]) }),
          transactionPrepare: async (instanceId) => {
            calls.push(`chain:${instanceId}:prepare`);
            const next = instanceId + 1;
            if (next > chainLength) return { foreign: [], rejection: null };
            return {
              foreign: [encodeForeignStepBytes({ target: { artifactId: `artifact-${next}`, artifactKind: "s.chain.doc" }, mutationId: "s.chain#step", payload: [next], label: "x" })],
              rejection: null,
            };
          },
          transactionCommit: async (instanceId) => {
            commitOrder.push(`chain:${instanceId}`);
            return { editId: `edit-${instanceId}` };
          },
          transactionRollback: async (instanceId) => {
            calls.push(`chain:${instanceId}:rollback`);
          },
          transactionUndo: async () => {},
          transactionRedo: async () => {},
          setMergePolicy: async () => {},
          resolveConflict: async () => ({ mergeReport: null, conflicts: null }),
          readConflicts: async () => [],
          dispose: async () => {},
        };
        const plugins = new Map<string, PluginWasmHandle>([
          ["a-plugin", fakeHandle("a-plugin", calls, commitOrder)],
          ["chain-plugin", chainHandle],
        ]);
        const coordinator = new TransactionCoordinator(directory, router, plugins);
        const foreign = encodeForeignStepBytes({ target: { artifactId: "artifact-1", artifactKind: "s.chain.doc" }, mutationId: "s.chain#step", payload: [1], label: "x" });
        const outcome = await coordinator.run({
          initiatorPluginId: "a-plugin",
          initiatorInstanceId: 10,
          initiatorArtifactId: "artifact-a",
          initiatorArtifactKind: "s.a.doc",
          localOps: [],
          description: "x",
          foreign: [foreign],
        });
        expect(outcome).toEqual({ ok: false, code: "transaction.depth-exceeded" });
      });
  
      it("passes a member's TransactionPrepared.rejection code straight through — instance-busy, generation-mismatch, and the member-rejected default all reachable", async () => {
        const directory = new InstanceDirectory();
        const router = new ArtifactMutationRouter();
        for (const code of ["transaction.instance-busy", "transaction.generation-mismatch", "not-a-real-fault-code"]) {
          const calls: string[] = [];
          const commitOrder: string[] = [];
          const plugins = new Map<string, PluginWasmHandle>([["a-plugin", fakeHandle("a-plugin", calls, commitOrder, { prepareRejection: code === "not-a-real-fault-code" ? new Uint8Array([255, 255, 255]) : encodeFaultBytes(code) })]]);
          const coordinator = new TransactionCoordinator(directory, router, plugins);
          const outcome = await coordinator.run({
            initiatorPluginId: "a-plugin",
            initiatorInstanceId: 10,
            initiatorArtifactId: "artifact-a",
            initiatorArtifactKind: "s.a.doc",
            localOps: [],
            description: "x",
            foreign: [],
          });
          const expectedCode = code === "not-a-real-fault-code" ? "transaction.member-rejected" : code;
          expect(outcome).toEqual({ ok: false, code: expectedCode });
        }
      });
    });
  
    describe("PluginRuntime documentPack/transaction wire adapter", () => {
      it("keeps the exact channel subscribed through refused close and releases only that channel after retry", async () => {
        const { default: fixture } = await import("../../🧱️elements/🔌️PluginRuntime/🧫️fixtures/🔒️channel-close.json");
        const { default: rendererModule } = await import("../../../🧬️schema/🔣️.json");
        const schema = { $ref: `${rendererModule.$id}#/$defs/PluginRuntimeChannelCloseV1` };
        const { default: Ajv } = await import("ajv");
        const { produce } = await import("immer");
        expect(new Ajv({ strict: true }).addSchema(rendererModule).compile(schema)(fixture)).toBe(true);
        const returned: number[] = [];
        let subscriptions = 0;
        let rejectClose!: (reason: unknown) => void;
        let resolveClose!: () => void;
        const lease = {
          handle: {
            manifest: async () => encodePackValue({ pluginId: "close-fixture", apps: [] }), createApp: async () => fixture.instance,
            destroyApp: () => new Promise<void>((resolve, reject) => { resolveClose = resolve; rejectClose = reject; }),
            enqueue: () => {}, takeSegmentedDownloadChunk: async () => undefined,
            outcomes: { [Symbol.asyncIterator](): AsyncIterator<TurnOutcome> { const id = subscriptions++; return { next: () => new Promise(() => {}), return: async () => { returned.push(id); return { done: true, value: undefined }; } }; } },
            dispose: async () => {},
          },
          release: async () => {},
        };
        const handle = await adaptPluginHandle("close-fixture", lease); const instance = await handle.createApp("fixture");
        expect(returned).toEqual(fixture.refusal.before);
        const first = handle.destroyApp(instance); const refused = expect(first).rejects.toThrow("native close refused");
        expect(returned).toEqual(fixture.refusal.before);
        rejectClose(new Error("native close refused")); await refused;
        expect(returned).toEqual(fixture.refusal.afterFailure); expect(handle.documentPack(instance)).toBeNull();
        const retry = handle.destroyApp(instance); resolveClose(); await retry;
        expect(returned).toEqual(produce(fixture.refusal.afterFailure as number[], state => { state.push(0); }));
        expect(returned).toEqual(fixture.refusal.afterRetry);
      });
  
      it("settles an old channel close without removing a replacement using the same numeric instance", async () => {
        const { default: fixture } = await import("../../🧱️elements/🔌️PluginRuntime/🧫️fixtures/🔒️channel-close.json");
        const returned: number[] = [];
        let subscriptions = 0;
        const closes: Array<() => void> = [];
        const lease = {
          handle: {
            manifest: async () => encodePackValue({ pluginId: "close-fixture", apps: [] }), createApp: async () => fixture.instance,
            destroyApp: () => new Promise<void>(resolve => { closes.push(resolve); }),
            enqueue: () => {}, takeSegmentedDownloadChunk: async () => undefined,
            outcomes: { [Symbol.asyncIterator](): AsyncIterator<TurnOutcome> { const id = subscriptions++; return { next: () => new Promise(() => {}), return: async () => { returned.push(id); return { done: true, value: undefined }; } }; } },
            dispose: async () => {},
          },
          release: async () => {},
        };
        const handle = await adaptPluginHandle("close-fixture", lease); const instance = await handle.createApp("old");
        const first = handle.destroyApp(instance); await handle.createApp("replacement");
        expect(returned).toEqual(fixture.replacement.whileClosing);
        closes[0]!(); await first;
        expect(returned).toEqual(fixture.replacement.afterOldClose); expect(handle.documentPack(instance)).toBeNull();
        const replacement = handle.destroyApp(instance); closes[1]!(); await replacement;
        expect(returned).toEqual(fixture.replacement.afterReplacementClose);
      });
  
      it("adaptPluginHandle's documentPack/transactionPrepare/transactionCommit/transactionRollback/transactionUndo/transactionRedo frame through AppChannelClient", async () => {
        const { decodeAppCommand, encodeAppFrame } = await import("@semio-tech/framework-os");
        const seenCommands: unknown[] = [];
        const turnBroadcast = createTurnOutcomeBroadcast<TurnOutcome>();
        const fakeLease = {
          handle: {
            manifest: async () => encodePackValue({ pluginId: "b-plugin", label: "B", version: "1.0.0", apps: [], workflows: [], examples: [] }),
            createApp: async () => 20,
            destroyApp: async () => {},
            takeSegmentedDownloadChunk: async () => undefined,
            enqueue: (instanceId: number, events: readonly Uint8Array[]) => {
              const commands = events.map((frame) => decodeAppCommand(frame));
              seenCommands.push(...commands);
              const frames = commands.flatMap((command) => {
                if ("transactionPrepare" in command) {
                  return [encodeAppFrame({ transactionPrepared: { txn_id: command.transactionPrepare.txn_id, foreign: [], rejection: [] } }), encodeAppFrame({ Done: { in_reply_to: command.transactionPrepare.seq } })];
                }
                if ("transactionCommit" in command) {
                  return [encodeAppFrame({ transactionCommitted: { txn_id: command.transactionCommit.txn_id, edit_id: "edit-1" } }), encodeAppFrame({ Done: { in_reply_to: command.transactionCommit.seq } })];
                }
                if ("transactionRollback" in command) return [encodeAppFrame({ transactionRolledBack: { txn_id: command.transactionRollback.txn_id } }), encodeAppFrame({ Done: { in_reply_to: command.transactionRollback.seq } })];
                if ("transactionUndo" in command) return [encodeAppFrame({ Done: { in_reply_to: command.transactionUndo.seq } })];
                if ("transactionRedo" in command) return [encodeAppFrame({ Done: { in_reply_to: command.transactionRedo.seq } })];
                if ("ReadDocument" in command) {
                  return [encodeAppFrame({ Document: { in_reply_to: command.ReadDocument.seq, pack: [5, 5], spr: [6], ops: "" } })];
                }
                if ("ReadWindowConfigs" in command) {
                  return [encodeAppFrame({ WindowConfigs: { in_reply_to: command.ReadWindowConfigs.seq, entries: [{ window_id: "graph-a", window_kind_id: "graph", envelope_pack: [7] }] } })];
                }
                if ("LoadWindowConfig" in command) return [encodeAppFrame({ Done: { in_reply_to: command.LoadWindowConfig.seq } })];
                throw new Error(`unexpected command ${JSON.stringify(command)}`);
              });
              turnBroadcast.push({ instanceId, frames });
            },
            outcomes: turnBroadcast.stream,
            dispose: async () => {},
          },
          release: async () => {},
        };
        const handle = await adaptPluginHandle("b-plugin", fakeLease);
        const instanceId = await handle.createApp("app-b");
  
        // 📦️ documentPack() is null until the underlying AppChannelClient has observed a document —
        // this adapter only ever surfaces the CACHE, it never issues a document round trip itself.
        expect(handle.documentPack(instanceId)).toBeNull();
  
        const prepareOutcome = await handle.transactionPrepare(instanceId, "txn-1", { form: "owner", mutationId: "s.b#mutate", payload: new Uint8Array([1]) });
        expect(prepareOutcome).toEqual({ foreign: [], rejection: null });
  
        const commitOutcome = await handle.transactionCommit(instanceId, "txn-1");
        expect(commitOutcome).toEqual({ editId: "edit-1" });
  
        await handle.transactionRollback(instanceId, "txn-2");
        await handle.transactionUndo(instanceId, "grp-1");
        await handle.transactionRedo(instanceId, "grp-1");
        expect(await handle.readWindowConfigPacks(instanceId)).toEqual([{ window_id: "graph-a", window_kind_id: "graph", envelope_pack: [7] }]);
        await handle.loadWindowConfigPack(instanceId, { window_id: "graph-b", window_kind_id: "graph", envelope_pack: [8] });
  
        expect(seenCommands).toEqual([
          { transactionPrepare: { seq: 1, txn_id: "txn-1", mutation_id: "s.b#mutate", payload: [1], prepared_ops: [], label: "", origin: [] } },
          { transactionCommit: { seq: 2, txn_id: "txn-1" } },
          { transactionRollback: { seq: 3, txn_id: "txn-2" } },
          { transactionUndo: { seq: 4, group_id: "grp-1" } },
          { transactionRedo: { seq: 5, group_id: "grp-1" } },
          { ReadWindowConfigs: { seq: 6 } },
          { LoadWindowConfig: { seq: 7, entry: { window_id: "graph-b", window_kind_id: "graph", envelope_pack: [8] } } },
        ]);
      });
  
      it("documentPack() reflects the cache after loadAppDocumentPack() — the adapter reads the SAME live channel it just loaded through", async () => {
        const { decodeAppCommand, encodeAppFrame } = await import("@semio-tech/framework-os");
        const turnBroadcast = createTurnOutcomeBroadcast<TurnOutcome>();
        const fakeLease = {
          handle: {
            manifest: async () => encodePackValue({ pluginId: "b-plugin", label: "B", version: "1.0.0", apps: [], workflows: [], examples: [] }),
            createApp: async () => 20,
            destroyApp: async () => {},
            takeSegmentedDownloadChunk: async () => undefined,
            enqueue: (instanceId: number, events: readonly Uint8Array[]) => {
              const commands = events.map((frame) => decodeAppCommand(frame));
              const frames = commands.map((command) => {
                if (!("LoadDocument" in command)) throw new Error(`unexpected command ${JSON.stringify(command)}`);
                return encodeAppFrame({ Done: { in_reply_to: command.LoadDocument.seq } });
              });
              turnBroadcast.push({ instanceId, frames });
            },
            outcomes: turnBroadcast.stream,
            dispose: async () => {},
          },
          release: async () => {},
        };
        const handle = await adaptPluginHandle("b-plugin", fakeLease);
        const instanceId = await handle.createApp("app-b");
        expect(handle.documentPack(instanceId)).toBeNull();
        await handle.loadAppDocumentPack?.(instanceId, new Uint8Array([1, 2]), new Uint8Array([3]));
        expect(handle.documentPack(instanceId)).toEqual({ pack: new Uint8Array([1, 2]), spr: new Uint8Array([3]) });
      });
  
      /** 📜️ `ops` is the third half of the frame, not decoration: `🏛️ShellHost`'s contributions push
       * reads it (`documentSourcesFromPack` → `resolveDocumentOperatorKinds`) and treats a MISSING
       * one as `{ status: "unresolved", reason: "document-ops-missing" }`, so an adapter that drops
       * it silently unresolves every operator scope. This law therefore pins the text crossing
       * verbatim, not merely that a document was read. */
      it("readAppDocumentPack() returns the AppFrame::Document pack/spr/ops, and null when the reply carries no document frame", async () => {
        const { decodeAppCommand, encodeAppFrame } = await import("@semio-tech/framework-os");
        const turnBroadcast = createTurnOutcomeBroadcast<TurnOutcome>();
        let replyWithDocument = true;
        const fakeLease = {
          handle: {
            manifest: async () => encodePackValue({ pluginId: "c-plugin", label: "C", version: "1.0.0", apps: [], workflows: [], examples: [] }),
            createApp: async () => 30,
            destroyApp: async () => {},
            takeSegmentedDownloadChunk: async () => undefined,
            enqueue: (instanceId: number, events: readonly Uint8Array[]) => {
              const commands = events.map((frame) => decodeAppCommand(frame));
              const frames = commands.map((command) => {
                if (!("ReadDocument" in command)) throw new Error(`unexpected command ${JSON.stringify(command)}`);
                const seq = command.ReadDocument.seq;
                return replyWithDocument ? encodeAppFrame({ Document: { in_reply_to: seq, pack: [7, 8], spr: [9], ops: "(extrude (polygon 6))" } }) : encodeAppFrame({ Done: { in_reply_to: seq } });
              });
              turnBroadcast.push({ instanceId, frames });
            },
            outcomes: turnBroadcast.stream,
            dispose: async () => {},
          },
          release: async () => {},
        };
        const handle = await adaptPluginHandle("c-plugin", fakeLease);
        const instanceId = await handle.createApp("app-c");
        expect(await handle.readAppDocumentPack?.(instanceId)).toEqual({ pack: new Uint8Array([7, 8]), spr: new Uint8Array([9]), ops: "(extrude (polygon 6))" });
        replyWithDocument = false;
        expect(await handle.readAppDocumentPack?.(instanceId)).toBeNull();
      });
    });
  
    //#region 🧪️terra-web-plugin-runtime
    /** 🧪️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-web-plugin-runtime). No real sleeps anywhere
     * below — every wait is a `queueMicrotask` flush or a deferred promise the test itself settles. */
    const flushMicrotasks = async (times = 4): Promise<void> => {
      for (let index = 0; index < times; index += 1) await new Promise<void>((resolve) => queueMicrotask(resolve));
    };
  
    describe("submitPluginTurn (TurnScheduler-backed turn dispatch replacing the old unbounded actorTurnQueue)", () => {
      function withFakeShardClient<T>(turnImpl: (actorId: string, events: readonly ShardEventEnvelope[], budget: ShardBudget) => Promise<unknown>, run: () => Promise<T>): Promise<T> {
        const previous = testState.sharedShardClient;
        testState.sharedShardClient = { turn: turnImpl } as unknown as ShardClient;
        return run().finally(() => {
          testState.sharedShardClient = previous;
        });
      }
  
      it("schedules captured lifecycle work through the original owner after operation revocation", async () => {
        const { default: Ajv } = await import("ajv");
        const { default: fixture } = await import("../../🧱️elements/🔌️PluginRuntime/🧫️fixtures/⏱️lifecycle-scheduler.json");
        const { default: rendererModule } = await import("../../../🧬️schema/🔣️.json");
        const schema = { $ref: `${rendererModule.$id}#/$defs/PluginRuntimeLifecycleSchedulerV1` };
        const { encodeActorInstanceLifecycle } = await import("../../../../../../../🔨️modules/🎭️actor/🚪️lifetime/🟦️.ts");
        const { OwnedUiInstance } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🏘️instance/🟦️.ts");
        expect(new Ajv({ strict: true }).addSchema(rendererModule).compile(schema)(fixture)).toBe(true);
        const sent: Array<{ kind: string; requestId: string; events?: readonly ShardEventEnvelope[] }> = [];
        const worker: ShardWorkerLike = { onmessage: null, onerror: null, postMessage(message) { sent.push(message as typeof sent[number]); }, terminate() {} };
        const client = new ShardClient({ residentLedger: new OwnedResidentLedger({ bytes: 1048576, slots: 4096, owners: 4096, control: { bytes: 65536, slots: 256, owners: 256 } }), shardCount: 1, createWorker: () => worker });
        async function answer<T>(pending: Promise<T>, value: unknown): Promise<T> { await flushMicrotasks(8); const message = sent.at(-1)!; worker.onmessage!({ data: { kind: "result", requestId: message.requestId, ok: true, value } }); return pending; }
        await answer(client.activate(fixture.actor, "/fixture.js", [], DEFAULT_SHARD_BUDGET), undefined);
        const owner = client.captureInstanceLifecycle(fixture.actor, fixture.instance);
        const lifetime = { activationGeneration: owner.activation.activationGeneration, instanceId: fixture.instance, guestLifetime: BigInt(fixture.guestLifetime) };
        const captured = { kind: "captured" as const, lifetime, requestSequence: owner.openRequest.requestSequence };
        const input = { appId: "fixture", actor: {}, config: new Uint8Array(), assets: [], capabilities: [], quotas: new Uint8Array() };
        const raw = { uiPatches: [], effects: [], nextWake: null, status: { tag: "idle" }, lifecycleReceipt: encodeActorInstanceLifecycle(captured) };
        const opened = await answer(submitPluginLifecycleTurn(owner, { kind: "open", input }, "Interactive"), raw);
        expect(opened.owner).toBe(owner); expect(opened.raw).toBe(raw); expect(opened.turn.original).toBe(raw); expect(opened.turn.lifecycleReceipt).toBe(raw.lifecycleReceipt);
        const ui = new OwnedUiInstance(owner.activation, lifetime, { maxNodes: 128, maxDepth: 16, maxChildren: 32, maxTextBytes: 4096, maxPatchOps: 128, maxPatchBytes: 65536 }, { usizeBits: 32 });
        owner.bindHostRetirement(ui);
        const phases = [owner.progress().kind]; const kinds = [sent.at(-1)!.events?.[0]?.kind ?? null];
        const plainidle = { uiPatches: [], effects: [], nextWake: null, status: { tag: "idle" } };
        await answer(submitPluginLifecycleTurn(owner, { kind: "receipt-ack", receipt: captured }, "Interactive"), plainidle); phases.push(owner.progress().kind); kinds.push(sent.at(-1)!.events?.[0]?.kind ?? null);
        await answer(submitPluginLifecycleTurn(owner, { kind: "poll" }, "UserVisible"), plainidle); phases.push(owner.progress().kind); kinds.push(sent.at(-1)!.events?.[0]?.kind ?? null);
        const request = owner.beginClose(); ui.beginClose();
        await expect(submitPluginTurn(fixture.actor, [{ kind: "app-command", payload: {} }], "Interactive", undefined, undefined, owner.activation)).rejects.toThrow(/revoked/);
        const accepted = { kind: "accepted" as const, lifetime, requestSequence: request.requestSequence, closeGeneration: BigInt(fixture.closeGeneration) };
        const retired = { ...accepted, kind: "retired" as const };
        await answer(submitPluginLifecycleTurn(owner, { kind: "close" }, "Interactive"), { ...plainidle, lifecycleReceipt: encodeActorInstanceLifecycle(accepted) }); phases.push(owner.progress().kind); kinds.push(sent.at(-1)!.events?.[0]?.kind ?? null);
        await answer(submitPluginLifecycleTurn(owner, { kind: "receipt-ack", receipt: accepted }, "Interactive"), { ...plainidle, lifecycleReceipt: encodeActorInstanceLifecycle(retired) }); phases.push(owner.progress().kind); kinds.push(sent.at(-1)!.events?.[0]?.kind ?? null);
        while (ui.closeStep({ maxItems: 1, maxBytes: 4096 }).kind !== "complete") {}
        const retirement = ui.takeRetirementWitness()!;
        const failed = submitPluginLifecycleTurn(owner, { kind: "receipt-ack", receipt: retired, retirement }, "Interactive");
        const observed = expect(failed).rejects.toThrow("actor-lifecycle.ack-not-admitted");
        await answer(failed.catch(() => undefined), { ...plainidle, status: { tag: "faulted", val: new Uint8Array([1]) } }); await observed; expect(owner.pendingReceipt).toEqual(retired);
        await answer(submitPluginLifecycleTurn(owner, { kind: "receipt-ack", receipt: retired, retirement }, "Interactive"), plainidle); phases.push(owner.progress().kind); kinds.push(sent.at(-1)!.events?.[0]?.kind ?? null);
        expect(phases).toEqual(fixture.phases); expect(kinds).toEqual(fixture.events);
        const before = sent.length;
        for (const kind of fixture.refusedWork) await expect(submitPluginLifecycleTurn(owner, { kind, events: [{ kind: "app-command", payload: {} }], run: () => { throw new Error("Unowned callback"); } } as never, "Interactive")).rejects.toThrow("actor-lifecycle.work-kind");
        expect(sent).toHaveLength(before);
        teardownPluginActor(fixture.actor); client.disposeAll();
      });
  
      it("schedules captured lifecycle UI ACKs with their exact private source and successful submission receipt", async () => {
        const { default: fixture } = await import("../../🧱️elements/🔌️PluginRuntime/🧫️fixtures/⏱️lifecycle-scheduler.json");
        const { encodeActorInstanceLifecycle } = await import("../../../../../../../🔨️modules/🎭️actor/🚪️lifetime/🟦️.ts");
        const { encodeActorUiPatchReceipt } = await import("../../../../../../../🔨️modules/🎭️actor/🚪️lifetime/🩹️patch/🟦️.ts");
        const { OwnedUiInstance } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🏘️instance/🟦️.ts");
        const sent: Array<{ kind: string; requestId: string; events?: readonly ShardEventEnvelope[] }> = [];
        const worker: ShardWorkerLike = { onmessage: null, onerror: null, postMessage(message) { sent.push(message as typeof sent[number]); }, terminate() {} };
        const client = new ShardClient({ residentLedger: new OwnedResidentLedger({ bytes: 1048576, slots: 4096, owners: 4096, control: { bytes: 65536, slots: 256, owners: 256 } }), shardCount: 1, createWorker: () => worker });
        async function answer<T>(pending: Promise<T>, value: unknown): Promise<T> { await flushMicrotasks(8); worker.onmessage!({ data: { kind: "result", requestId: sent.at(-1)!.requestId, ok: true, value } }); return pending; }
        const plain = { uiPatches: [], effects: [], nextWake: null, status: { tag: "idle" } };
        const actorId = `${fixture.actor}-ui-ack`;
        await answer(client.activate(actorId, "/fixture.js", [], DEFAULT_SHARD_BUDGET), undefined);
        const owner = client.captureInstanceLifecycle(actorId, fixture.instance);
        const lifetime = { activationGeneration: owner.activation.activationGeneration, instanceId: fixture.instance, guestLifetime: BigInt(fixture.guestLifetime) };
        const captured = { kind: "captured" as const, lifetime, requestSequence: owner.openRequest.requestSequence };
        await answer(submitPluginLifecycleTurn(owner, { kind: "open", input: { appId: "fixture", actor: {}, config: new Uint8Array(), assets: [], capabilities: [], quotas: new Uint8Array() } }, "Interactive"), { ...plain, lifecycleReceipt: encodeActorInstanceLifecycle(captured) });
        const ui = new OwnedUiInstance(owner.activation, lifetime, { maxNodes: 128, maxDepth: 16, maxChildren: 32, maxTextBytes: 4096, maxPatchOps: 128, maxPatchBytes: 65536 }, { usizeBits: 32 }); owner.bindHostRetirement(ui);
        await answer(submitPluginLifecycleTurn(owner, { kind: "receipt-ack", receipt: captured }, "Interactive"), plain);
        const value = fixture.uiAcknowledgement;
        const receipt = { lifetime, patchSequence: BigInt(value.patchSequence) };
        const original = { ...plain, uiPatchReceipt: encodeActorUiPatchReceipt(receipt), uiPatches: [{ surface: { instance: fixture.instance, surface: value.surface }, revision: BigInt(value.revision), baseRevision: BigInt(value.baseRevision), ops: [] }] };
        const polled = await answer(submitPluginLifecycleTurn(owner, { kind: "poll" }, "UserVisible"), original);
        expect(polled.turn.uiPatchReceipt).toBe(original.uiPatchReceipt);
        const source = owner.captureUiPatchAuthority(original, 0);
        expect(source.value.operationCount).toBe(value.operationCount);
        const grant = { maxItems: 1, maxBytes: 4096 };
        const lookup = ui.beginSurfaceLookup(owner.activation, lifetime, value.surface)!;
        for (let count = 0; lookup.advance(grant).kind !== "ready"; count++) if (count > 1024) throw new Error("Fixture lookup did not complete");
        const facade = lookup.takeResult()!; lookup.beginClose(); while (lookup.closeStep(grant).kind !== "complete") {}
        const patch = ui.beginPatch(source, facade); patch.finishInput();
        for (let count = 0; patch.advance(grant).kind !== "ready"; count++) if (count > 1024) throw new Error("Fixture publication did not complete");
        const token = patch.peekAcknowledgement()!;
        const close = owner.beginClose(); ui.beginClose();
        const post = worker.postMessage; worker.postMessage = () => { throw new Error("fixture post refusal"); };
        await expect(submitPluginLifecycleTurn(owner, { kind: "issued-ui-ack", source, token }, "Interactive")).rejects.toThrow("fixture post refusal");
        expect(patch.peekAcknowledgement()).toBe(token); worker.postMessage = post;
        const submitted = await answer(submitPluginLifecycleTurn(owner, { kind: "issued-ui-ack", source, token }, "Interactive"), plain);
        expect(submitted.owner).toBe(owner); expect(submitted.raw).toBe(plain); expect(sent.at(-1)!.events?.[0]?.kind).toBe("patch-ack");
        expect(sent.at(-1)!.events?.[0]?.payload).toEqual({ receipt, surface: { instance: fixture.instance, surface: value.surface }, revision: BigInt(value.revision) });
        expect(patch.acceptAcknowledgement(submitted.submission)).toBe(true); expect(patch.acceptAcknowledgement(submitted.submission)).toBe(false);
        const accepted = { kind: "accepted" as const, lifetime, requestSequence: close.requestSequence, closeGeneration: BigInt(fixture.closeGeneration) };
        const retired = { ...accepted, kind: "retired" as const };
        await answer(submitPluginLifecycleTurn(owner, { kind: "close" }, "Interactive"), { ...plain, lifecycleReceipt: encodeActorInstanceLifecycle(accepted) });
        await answer(submitPluginLifecycleTurn(owner, { kind: "receipt-ack", receipt: accepted }, "Interactive"), { ...plain, lifecycleReceipt: encodeActorInstanceLifecycle(retired) });
        for (let count = 0; ui.closeStep(grant).kind !== "complete"; count++) if (count > 1024) throw new Error("Fixture UI close did not complete");
        await answer(submitPluginLifecycleTurn(owner, { kind: "receipt-ack", receipt: retired, retirement: ui.takeRetirementWitness()! }, "Interactive"), plain);
        expect(owner.progress().kind).toBe("complete"); teardownPluginActor(actorId); client.disposeAll();
      });

      it("composes the production plugin runtime with one real UI owner through exact patch ACK and retirement", async () => {
        const { default: Ajv } = await import("ajv");
        const { default: equal } = await import("fast-deep-equal");
        const { default: fixture } = await import("../../🧱️elements/🔌️PluginRuntime/🧫️fixtures/⏱️lifecycle-scheduler.json");
        const { default: rendererModule } = await import("../../../🧬️schema/🔣️.json");
        const { encodeActorInstanceLifecycle } = await import("../../../../../../../🔨️modules/🎭️actor/🚪️lifetime/🟦️.ts");
        expect(new Ajv({ strict: true }).addSchema(rendererModule).compile({ $ref: `${rendererModule.$id}#/$defs/PluginRuntimeLifecycleSchedulerV1` })(fixture)).toBe(true);
        const previous = { registry: testState.sharedActivationRegistry, shard: testState.sharedShardClient, fetch: globalThis.fetch };
        const sent: Array<{ readonly kind: string; readonly events: readonly string[] }> = [];
        const plain = { uiPatches: [], effects: [], nextWake: null, status: { tag: "idle" } };
        let lifetime: { readonly activationGeneration: bigint; readonly instanceId: number; readonly guestLifetime: bigint } | null = null;
        const closeGeneration = BigInt(fixture.closeGeneration);
        const node: UiNodeRecord = { id: 0, key: "root", component: { type: "text", value: "owned", emphasize: null, dataAttributes: null }, layout: { kind: "leaf", width: "hug", height: "hug" }, style: { variant: "plain", size: "md", density: "standard", tone: "neutral", emphasis: "regular" }, activity: "idle", disabled: false, transition: null, accessibility: { label: null, description: null, live: "off", shortcut: null, hidden: false }, bindings: [], menu: null, children: [] };
        const worker: ShardWorkerLike = {
          onmessage: null,
          onerror: null,
          postMessage(raw) {
            const message = raw as { readonly kind: string; readonly requestId?: string; readonly activationGeneration?: bigint; readonly events?: readonly ShardEventEnvelope[] };
            const events = (message.events ?? []).map(event => event.kind);
            if (message.kind === "dispose") { sent.push({ kind: message.kind, events }); return; }
            const requestId = message.requestId;
            if (!requestId) return;
            let value: unknown = undefined;
            if (message.kind === "turn") {
              sent.push({ kind: message.kind, events });
              const first = message.events?.[0];
              if (first?.kind === "instance-open") {
                const payload = first.payload as { readonly instance: number; readonly activationGeneration: bigint; readonly requestSequence: number };
                lifetime = { activationGeneration: payload.activationGeneration, instanceId: payload.instance, guestLifetime: BigInt(fixture.guestLifetime) };
                const captured = { kind: "captured" as const, lifetime, requestSequence: payload.requestSequence };
                value = {
                  ...plain,
                  lifecycleReceipt: encodeActorInstanceLifecycle(captured),
                  uiPatchReceipt: encodeActorUiPatchReceipt({ lifetime, patchSequence: BigInt(fixture.uiAcknowledgement.patchSequence) }),
                  uiPatches: [{ surface: { instance: payload.instance, surface: fixture.runtimeUiComposition.surface }, revision: 1n, baseRevision: 0n, ops: [{ tag: "upsert", val: { node: encodePackValue(node) } }, { tag: "set-root", val: 0n }] }],
                };
              } else if (first?.kind === "instance-close") {
                const close = first.payload as { readonly requestSequence: number };
                value = { ...plain, lifecycleReceipt: encodeActorInstanceLifecycle({ kind: "accepted", lifetime: lifetime!, requestSequence: close.requestSequence, closeGeneration }) };
              } else if (first?.kind === "instance-lifecycle-ack") {
                const receipt = (first.payload as { readonly receipt: { readonly kind: string; readonly requestSequence: number } }).receipt;
                value = receipt.kind === "accepted" ? { ...plain, lifecycleReceipt: encodeActorInstanceLifecycle({ kind: "retired", lifetime: lifetime!, requestSequence: receipt.requestSequence, closeGeneration }) } : plain;
              } else value = plain;
            }
            queueMicrotask(() => worker.onmessage?.({ data: { kind: "result", requestId, ok: true, value } }));
          },
          terminate() {},
        };
        const client = new ShardClient({ residentLedger: new OwnedResidentLedger({ bytes: 1048576, slots: 4096, owners: 4096, control: { bytes: 65536, slots: 256, owners: 256 } }), shardCount: 1, createWorker: () => worker });
        testState.sharedShardClient = client;
        testState.sharedActivationRegistry = { registerManifest: () => {}, activate: async (_plugin: string, actorId: string) => client.activate(actorId, "/fixture.js", [], DEFAULT_SHARD_BUDGET), touch: () => {}, cancel: (actorId: string) => client.dispose(actorId) } as unknown as ActivationRegistry;
        globalThis.fetch = (async () => new Response(JSON.stringify({ manifest: { pluginId: "owned-ui", apps: [] } }), { headers: { "content-type": "application/json" } })) as typeof fetch;
        let handle: PluginWasmHandle | null = null;
        try {
          handle = await loadPluginModule("owned-ui", "https://fixture.invalid/plugin.js");
          const instance = await handle.createApp("fixture");
          const actorId = `owned-ui#${instance}`;
          expect(retainedWindowByActor.has(actorId)).toBe(false);
          const openEvents = sent.filter(entry => entry.kind === "turn").flatMap(entry => entry.events);
          expect(openEvents).toEqual(fixture.runtimeUiComposition.openEvents);
          expect(equal(openEvents, fixture.runtimeUiComposition.openEvents)).toBe(true);
          const response = await handle.refreshUi(instance, { viewState: { locale: "en", terminology: "native", windowInstances: [{ id: fixture.runtimeUiComposition.surface, windowKindId: "fixture" }] }, windows: [{ key: fixture.runtimeUiComposition.surface, bodyKey: "root" }] });
          expect(response.windows).toMatchObject([{ key: fixture.runtimeUiComposition.surface, value: { key: "root", component: { type: "text", value: "owned" }, children: [] } }]);
          expect(retainedWindowByActor.has(actorId)).toBe(false);
          const beforeClose = sent.flatMap(entry => entry.kind === "dispose" ? [entry.kind] : entry.events).length;
          await handle.destroyApp(instance);
          const allEvents = sent.flatMap(entry => entry.kind === "dispose" ? [entry.kind] : entry.events);
          const closeEvents = allEvents.slice(beforeClose);
          expect(closeEvents).toEqual(fixture.runtimeUiComposition.closeEvents);
          expect(equal(closeEvents, fixture.runtimeUiComposition.closeEvents)).toBe(true);
          expect(retainedWindowByActor.has(actorId)).toBe(false);
          console.info("[DEBUG] production UI lifetime: nativePatchAcks=1 realSurface=1 renderSource=%s terminal=%s", fixture.runtimeUiComposition.renderSource, fixture.runtimeUiComposition.terminalPhase);
        } finally {
          await handle?.dispose();
          testState.sharedActivationRegistry = previous.registry;
          testState.sharedShardClient = previous.shard;
          globalThis.fetch = previous.fetch;
        }
      });

      /** ⚖️ LAW: one 8 KiB command costs the host EXACTLY its page count plus the continuations the
       * guest itself asked for — the host contributes no round trip of its own.
       *
       * 🥽️ Ticket 26/09/02 wave B22 §1.3 measured **84.2 worker round trips per `registerBrushMesh`**
       * (842 posts over 10 consecutive mesh-only settles) and wave B24 attributed every one of them to
       * the guest: `plugin_exchange` answered `Pending` after each ingress move, and
       * `plugin_continue_typed_operations` advanced exactly one publication unit per call. This law
       * pins the OTHER half — that `runQueuedTurn` never polls while the guest is busy, never re-sends
       * an accepted page, and never issues a refresh per continuation — so the census stays readable as
       * `pages + guestContinuations` and a host-side regression cannot hide inside the guest's number.
       * Its native twins are `a_command_page_set_reaches_its_terminal_status_in_one_turn_per_page` and
       * the continuation bound in `retained_operation_continues_after_command_admission_until_
       * publication_and_retirement`. */
      it("spends one worker turn per command page plus the guest's own continuations and none of its own", async () => {
        const { encodeActorInstanceLifecycle } = await import("../../../../../../../🔨️modules/🎭️actor/🚪️lifetime/🟦️.ts");
        const COMMAND_BYTES = 8_192;
        const invocation = { address: { pluginId: "ingress-census", appId: "fixture", windowKindId: "editor", actionId: "registerBrushMesh" }, arguments: { payload: "m".repeat(7_800) } };
        const census: Array<{ readonly guestContinuations: number; readonly bytes: number; readonly pages: number; readonly turns: number }> = [];
        for (const guestContinuations of [0, 1, 4]) {
          const previous = { registry: testState.sharedActivationRegistry, shard: testState.sharedShardClient, fetch: globalThis.fetch };
          const plain = { uiPatches: [], effects: [], nextWake: null };
          const more = { ...plain, status: { tag: "more-work" } };
          const idle = { ...plain, status: { tag: "idle" } };
          const kinds: string[] = [];
          let counting = false;
          let bytes = 0;
          let declared = 0;
          let remaining = guestContinuations;
          let lifetime: { readonly activationGeneration: bigint; readonly instanceId: number; readonly guestLifetime: bigint } | null = null;
          const worker: ShardWorkerLike = {
            onmessage: null, onerror: null, terminate() {},
            postMessage(raw) {
              const message = raw as { readonly kind: string; readonly requestId?: string; readonly events?: readonly ShardEventEnvelope[]; readonly commandPage?: { readonly cursor: { readonly pageIndex: number; readonly pageCount: number }; readonly bytes: Uint8Array } };
              if (!message.requestId) return;
              let value: unknown = idle;
              if (message.kind === "turn") {
                const first = message.events?.[0];
                if (first?.kind === "instance-open") {
                  const payload = first.payload as { readonly instance: number; readonly activationGeneration: bigint; readonly requestSequence: number };
                  lifetime = { activationGeneration: payload.activationGeneration, instanceId: payload.instance, guestLifetime: 1n };
                  value = { ...idle, lifecycleReceipt: encodeActorInstanceLifecycle({ kind: "captured", lifetime, requestSequence: payload.requestSequence }) };
                } else if (first?.kind === "instance-close") {
                  value = { ...idle, lifecycleReceipt: encodeActorInstanceLifecycle({ kind: "accepted", lifetime: lifetime!, requestSequence: (first.payload as { readonly requestSequence: number }).requestSequence, closeGeneration: 1n }) };
                } else if (first?.kind === "instance-lifecycle-ack") {
                  const receipt = (first.payload as { readonly receipt: { readonly kind: string; readonly requestSequence: number } }).receipt;
                  value = receipt.kind === "accepted" ? { ...idle, lifecycleReceipt: encodeActorInstanceLifecycle({ kind: "retired", lifetime: lifetime!, requestSequence: receipt.requestSequence, closeGeneration: 1n }) } : idle;
                } else if (message.commandPage) {
                  const cursor = message.commandPage.cursor;
                  if (counting) {
                    kinds.push(`command-page#${cursor.pageIndex}`);
                    bytes += message.commandPage.bytes.length;
                    declared = cursor.pageCount;
                  }
                  value = cursor.pageIndex + 1 === cursor.pageCount
                    ? { ...(remaining > 0 ? more : idle), commandIngress: { tag: "command-complete", val: {} } }
                    : { ...more, commandIngress: { tag: "page-accepted", val: {} } };
                } else if (counting) {
                  kinds.push("settle-continuation");
                  remaining -= 1;
                  value = remaining > 0 ? more : idle;
                }
              }
              queueMicrotask(() => worker.onmessage?.({ data: { kind: "result", requestId: message.requestId, ok: true, value } }));
            },
          };
          const client = new ShardClient({ residentLedger: new OwnedResidentLedger({ bytes: 1048576, slots: 4096, owners: 4096, control: { bytes: 65536, slots: 256, owners: 256 } }), shardCount: 1, createWorker: () => worker });
          testState.sharedShardClient = client;
          testState.sharedActivationRegistry = { registerManifest: () => {}, activate: async (_plugin: string, actorId: string) => client.activate(actorId, "/fixture.js", [], DEFAULT_SHARD_BUDGET), touch: () => {}, cancel: (actorId: string) => client.dispose(actorId) } as unknown as ActivationRegistry;
          globalThis.fetch = (async () => new Response(JSON.stringify({ manifest: { pluginId: "ingress-census", apps: [] } }), { headers: { "content-type": "application/json" } })) as typeof fetch;
          let handle: PluginWasmHandle | null = null;
          try {
            handle = await loadPluginModule("ingress-census", "https://fixture.invalid/plugin.js");
            const instance = await handle.createApp("fixture");
            counting = true;
            await handle.handleAction(instance, JSON.stringify({ ...invocation, address: { ...invocation.address, instanceId: String(instance) } }), { locale: "en", terminology: "native", windowInstances: [] } as never);
            counting = false;
            expect(bytes).toBeGreaterThan(COMMAND_BYTES - 1_024);
            expect(bytes).toBeLessThanOrEqual(COMMAND_BYTES);
            expect(declared).toBe(2);
            expect(kinds).toEqual([...Array.from({ length: declared }, (_, page) => `command-page#${page}`), ...Array.from({ length: guestContinuations }, () => "settle-continuation")]);
            census.push({ guestContinuations, bytes, pages: declared, turns: kinds.length });
          } finally {
            try { await handle?.dispose(); } finally {
              client.disposeAll();
              testState.sharedActivationRegistry = previous.registry;
              testState.sharedShardClient = previous.shard;
              globalThis.fetch = previous.fetch;
            }
          }
        }
        for (const row of census) expect(row.turns).toBe(row.pages + row.guestContinuations);
        console.info("[DEBUG] command ingress census: %s", census.map((row) => `${row.bytes}B/${row.pages}p +${row.guestContinuations}cont → ${row.turns} worker turns`).join(" | "));
      });


      it("retries actual actor retirement with the original witness after final acknowledgement failure", async () => {
        const { default: fixture } = await import("../../🧱️elements/🔌️PluginRuntime/📡️backbone/🧫️fixtures/🔣️.json");
        const { default: schema } = await import("../../../🧬️schema/🔣️.json");
        const { default: Ajv } = await import("ajv");
        const { default: equal } = await import("fast-deep-equal");
        const { encodeActorInstanceLifecycle } = await import("../../../../../../../🔨️modules/🎭️actor/🚪️lifetime/🟦️.ts");
        const { OwnedUiInstance } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🏘️instance/🟦️.ts");
        expect(new Ajv({ strict: true }).addSchema(schema).getSchema(`${schema.$id}#/$defs/ActorDocumentPortFixtureV1`)!(fixture)).toBe(true);
        for (const row of fixture.disposal.retirementRetry) {
          const previous = { registry: testState.sharedActivationRegistry, shard: testState.sharedShardClient, fetch: globalThis.fetch };
          const plain = { uiPatches: [], effects: [], nextWake: null, status: { tag: "idle" } };
          const events: string[] = [];
          let lifetime: { activationGeneration: bigint; instanceId: number; guestLifetime: bigint } | null = null;
          let refused = false;
          const worker: ShardWorkerLike = {
            onmessage: null, onerror: null, terminate() {},
            postMessage(raw) {
              const message = raw as { kind: string; requestId?: string; events?: readonly ShardEventEnvelope[] };
              if (message.kind === "dispose") { events.push("dispose"); return; }
              if (!message.requestId) return;
              let value: unknown = undefined;
              if (message.kind === "turn") {
                const first = message.events?.[0];
                if (first?.kind === "instance-open") {
                  const payload = first.payload as { activationGeneration: bigint; instance: number; requestSequence: number };
                  lifetime = { activationGeneration: payload.activationGeneration, instanceId: payload.instance, guestLifetime: 1n };
                  events.push("open");
                  value = { ...plain, lifecycleReceipt: encodeActorInstanceLifecycle({ kind: "captured", lifetime, requestSequence: payload.requestSequence }) };
                } else if (first?.kind === "instance-close") {
                  const payload = first.payload as { requestSequence: number };
                  events.push("close");
                  value = { ...plain, lifecycleReceipt: encodeActorInstanceLifecycle({ kind: "accepted", lifetime: lifetime!, requestSequence: payload.requestSequence, closeGeneration: 1n }) };
                } else if (first?.kind === "instance-lifecycle-ack") {
                  const receipt = (first.payload as { receipt: { kind: string; requestSequence: number } }).receipt;
                  events.push(`ack:${receipt.kind}`);
                  if (receipt.kind === "retired" && !refused) {
                    refused = true;
                    if (row.failure === "transport") throw new Error("fixture-final-ack-transport");
                    value = { ...plain, status: { tag: "faulted", val: new Uint8Array([1]) } };
                  } else value = receipt.kind === "accepted" ? { ...plain, lifecycleReceipt: encodeActorInstanceLifecycle({ kind: "retired", lifetime: lifetime!, requestSequence: receipt.requestSequence, closeGeneration: 1n }) } : plain;
                } else value = plain;
              }
              queueMicrotask(() => worker.onmessage?.({ data: { kind: "result", requestId: message.requestId, ok: true, value } }));
            },
          };
          const client = new ShardClient({ residentLedger: new OwnedResidentLedger({ bytes: 1048576, slots: 4096, owners: 4096, control: { bytes: 65536, slots: 256, owners: 256 } }), shardCount: 1, createWorker: () => worker });
          const captured = vi.spyOn(client, "captureInstanceLifecycle");
          const witnesses = vi.spyOn(OwnedUiInstance.prototype, "takeRetirementWitness");
          testState.sharedShardClient = client;
          testState.sharedActivationRegistry = { registerManifest: () => {}, activate: async (_plugin: string, actorId: string) => client.activate(actorId, "/fixture.js", [], DEFAULT_SHARD_BUDGET), touch: () => {}, cancel: () => {} } as unknown as ActivationRegistry;
          globalThis.fetch = (async () => new Response(JSON.stringify({ manifest: { pluginId: "retirement-retry", apps: [] } }), { headers: { "content-type": "application/json" } })) as typeof fetch;
          let handle: PluginWasmHandle | null = null;
          try {
            handle = await loadPluginModule("retirement-retry", "https://fixture.invalid/plugin.js");
            const instance = await handle.createApp("fixture");
            const lease = captured.mock.results[0]!.value as ShardInstanceLifecycleLease;
            await expect(handle.destroyApp(instance)).rejects.toThrow(row.failure === "transport" ? "fixture-final-ack-transport" : "actor-lifecycle.ack-not-admitted");
            expect(lease.pendingReceipt?.kind).toBe("retired");
            expect(lease.progress().kind).toBe("blocked");
            expect(events).toEqual(row.events.slice(0, -2));
            expect(witnesses).toHaveBeenCalledTimes(row.witnesses);
            const receipt = lease.pendingReceipt;
            await handle.destroyApp(instance);
            expect(lease.progress().kind).toBe("complete");
            expect(lease.pendingReceipt).toBeNull();
            expect(receipt?.kind).toBe("retired");
            expect(witnesses).toHaveBeenCalledTimes(row.witnesses);
            expect(equal(events, row.events)).toBe(true);
            await handle.destroyApp(instance);
            expect(events).toEqual(row.events);
            console.info("[DEBUG] actual runtime final ACK retry: failure=%s witnesses=%d dispose=1", row.failure, witnesses.mock.calls.length);
          } finally {
            try { await handle?.dispose(); } finally {
              captured.mockRestore(); witnesses.mockRestore(); client.disposeAll();
              testState.sharedActivationRegistry = previous.registry; testState.sharedShardClient = previous.shard; globalThis.fetch = previous.fetch;
            }
          }
        }
      });

      it("schedules captured lifecycle without silent eviction when either ingress fills the actor queue", async () => {
        const { default: fixture } = await import("../../🧱️elements/🔌️PluginRuntime/🧫️fixtures/⏱️lifecycle-scheduler.json");
        expect(fixture.mailbox.capacity).toBe(PLUGIN_TURN_MAILBOX_CAPACITY);
        for (const incoming of fixture.mailbox.overflow) {
          const actorId = `${fixture.actor}-capacity-${incoming}`;
          let release!: () => void;
          const raw = { uiPatches: [], effects: [], nextWake: null, status: { tag: "idle" } };
          const gate = new Promise<typeof raw>(resolve => { release = () => resolve(raw); });
          let first = true; let lifecycleDispatched = 0;
          const owner = { activation: { actorId }, async poll() { lifecycleDispatched++; return raw; } } as unknown as ShardInstanceLifecycleLease;
          await withFakeShardClient(async () => { if (first) { first = false; return gate; } return raw; }, async () => {
            const running = submitPluginTurn(actorId, [], "Interactive"); await flushMicrotasks(8);
            const pending = submitPluginLifecycleTurn(owner, { kind: "poll" }, "UserVisible");
            let lifecycleSettled = false; void pending.then(() => { lifecycleSettled = true; }, () => {});
            const commands = Array.from({ length: fixture.mailbox.capacity - 1 }, () => submitPluginTurn(actorId, [], "Interactive"));
            const overflow = (incoming === "operation" ? submitPluginTurn(actorId, [], "Interactive") : submitPluginLifecycleTurn(owner, { kind: "poll" }, "Interactive")).then(() => "accepted", () => "refused");
            expect(lifecycleDispatched).toBe(0);
            release(); await running; await Promise.all(commands); const outcome = await overflow; await flushMicrotasks(8);
            expect(outcome).toBe(fixture.mailbox.outcome); expect(lifecycleDispatched).toBe(1); expect(lifecycleSettled).toBe(true); expect((await pending).owner).toBe(owner);
            teardownPluginActor(actorId);
          });
        }
      });
  
      it("continues admitted operations after surfaces are retained and ACKs each exact result", async () => {
        const { Buffer } = await import("node:buffer");
        const { default: Ajv } = await import("ajv");
        const { default: equal } = await import("fast-deep-equal");
        const { default: settlement } = await import("../../🧱️elements/🔌️PluginRuntime/🧫️fixtures/📬️typed-operation-settlement.json");
        const { default: rendererModule } = await import("../../../🧬️schema/🔣️.json");
        const schema = { $ref: `${rendererModule.$id}#/$defs/PluginRuntimeTypedOperationSettlementV1` };
        expect(new Ajv({ strict: true }).addSchema(rendererModule).compile(schema)(settlement)).toBe(true);
        const { default: fixture } = await import("../../../../🔌️plugin/⚛️reactor/🧫️fixtures/🔣️.json");
        const token = Buffer.alloc(25);
        token.writeUInt32LE(fixture.wire.receiver, 0);
        token.writeBigUInt64LE(BigInt(fixture.wire.operation), 4);
        token.writeBigUInt64LE(BigInt(fixture.wire.generation), 12);
        token.writeUInt32LE(fixture.wire.sequence, 20);
        token[24] = fixture.wire.attempt;
        const expectedAck = Buffer.concat([Buffer.from("semio.typed-operation-ack.v1\0"), token]);
        const payload = Buffer.from(fixture.wire.payload);
        const length = Buffer.alloc(4);
        length.writeUInt32LE(payload.length);
        let turns = 0;
        await withFakeShardClient(async (_actor, events) => {
          if (turns > 0) expect(events).toEqual([{ kind: "message", payload: { source: { tag: "shell", val: String(fixture.wire.receiver) }, payload: Array.from(expectedAck) } }]);
          const lane = fixture.wire.lanes[turns++];
          const effects = lane === undefined ? [] : [{ tag: "send-message", val: { target: { tag: "shell", val: String(fixture.wire.receiver) }, payload: Array.from(Buffer.concat([Buffer.from("semio.typed-operation-page.v1\0"), token, Buffer.from([lane]), length, payload])) } }];
          return { uiPatches: [], effects, nextWake: null, status: { tag: lane === undefined ? "idle" : "more-work" } };
        }, async () => {
          const result = await settlePluginTurn("retained-operation#7", { uiPatches: [], effects: [], nextWake: null, status: { tag: "more-work" } }, "Interactive", new Set(), undefined, true);
          expect(turns).toBe(fixture.wire.lanes.length + 1);
          expect(result.effects).toEqual(settlement.retainedEffects);
          expect(equal(result.effects, settlement.retainedEffects)).toBe(true);
          expect(result.status).toEqual(settlement.status);
          const invocation = invocationFromFrames([], result.effects, "action");
          expect({ output: invocation.output, requestedEffects: invocation.requestedEffects }).toEqual(settlement.invocation);
          console.info("[DEBUG] typed-operation settlement: exact ACKs=%d retainedTerminal=1 hostEffects=%d", fixture.wire.lanes.length, invocation.requestedEffects?.length);
        });
      });
  
      it("does not replay already acknowledged ingress publications during settlement", async () => {
        const { Buffer } = await import("node:buffer");
        const { default: equal } = await import("fast-deep-equal");
        const { default: settlement } = await import("../../🧱️elements/🔌️PluginRuntime/🧫️fixtures/📬️typed-operation-settlement.json");
        const { default: fixture } = await import("../../../../🔌️plugin/⚛️reactor/🧫️fixtures/🔣️.json");
        const results: WireTurnResult[] = fixture.wire.lanes.map((lane, sequence) => {
          const header = Buffer.alloc(30);
          header.writeUInt32LE(fixture.wire.receiver, 0);
          header.writeBigUInt64LE(BigInt(fixture.wire.operation), 4);
          header.writeBigUInt64LE(BigInt(fixture.wire.generation), 12);
          header.writeUInt32LE(sequence, 20);
          header[24] = fixture.wire.attempt;
          header[25] = lane;
          return { uiPatches: [], effects: [{ tag: "send-message", val: { target: { tag: "shell", val: String(fixture.wire.receiver) }, payload: Array.from(Buffer.concat([Buffer.from("semio.typed-operation-page.v1\0"), header])) } }], nextWake: null, status: { tag: "more-work" } };
        });
        const pending = typedOperationAcknowledgements(results.at(-1)!);
        await withFakeShardClient(async (_actor, events) => {
          expect(events).toEqual(pending);
          return { uiPatches: [], effects: [], nextWake: null, status: { tag: "idle" } };
        }, async () => {
          const result = await settleAcknowledgedPluginTurns("retained-ingress#7", results, pending);
          expect(result.effects).toEqual(settlement.retainedEffects);
          expect(equal(consumeTypedOperationEffects(result.effects), settlement.retainedEffects)).toBe(true);
          expect(result.status).toEqual(settlement.status);
          const invocation = invocationFromFrames([], result.effects, "action");
          expect({ output: invocation.output, requestedEffects: invocation.requestedEffects }).toEqual(settlement.invocation);
          console.info("[DEBUG] typed-operation ingress settlement: retainedTerminal=1 replayedPublications=0 hostEffects=%d", invocation.requestedEffects?.length);
        });
      });
  
      it("validates fixed result page authority and preserves document and download effects", async () => {
        const { Buffer } = await import("node:buffer");
        const { default: fixture } = await import("../../../../🔌️plugin/⚛️reactor/🧫️fixtures/🔣️.json");
        const wire = (lane: number, text: string, receiver = fixture.wire.receiver): WireVariant => {
          const body = Buffer.alloc(30);
          body.writeUInt32LE(fixture.wire.receiver, 0);
          body.writeBigUInt64LE(BigInt(fixture.wire.operation), 4);
          body.writeBigUInt64LE(BigInt(fixture.wire.generation), 12);
          body.writeUInt32LE(fixture.wire.sequence, 20);
          body[24] = fixture.wire.attempt;
          body[25] = lane;
          const payload = Buffer.from(text);
          body.writeUInt32LE(payload.length, 26);
          return { tag: "send-message", val: { target: { tag: "shell", val: String(receiver) }, payload: Array.from(Buffer.concat([Buffer.from("semio.typed-operation-page.v1\0"), body, payload])) } };
        };
        expect(typedOperationResult(wire(10, "x".repeat(4_096)))?.payload.length).toBe(4_096);
        for (const invalid of [wire(15, ""), wire(10, "x".repeat(4_097)), wire(10, "", 8)]) expect(() => typedOperationResult(invalid)).toThrow("authority");
        const truncated = wire(10, "x");
        (truncated.val as { payload: number[] }).payload.pop();
        expect(() => typedOperationResult(truncated)).toThrow("authority");
        expect(wireEffectToFriendly({ tag: "load-document", val: fixture.effects.loadDocument })).toEqual({ loadDocument: fixture.effects.loadDocument });
        const download = consumeTypedOperationEffects([wire(9, JSON.stringify(fixture.effects.download))]);
        expect(download.map(wireEffectToFriendly)).toEqual([{ downloadMediaExport: { filename: fixture.effects.download[0], mimeType: fixture.effects.download[1], data: fixture.wire.operation, encoding: "semio-segmented-handle-v1:identity" } }]);
        const receipt = {
          schema: DIRECTORY_PROJECTION_RECEIPT_SCHEMA,
          sessionBindingSha256: "a".repeat(64),
          authorizationGeneration: 7,
          throughSeqInclusive: 11,
          receiptSha256: "b".repeat(64),
        };
        const receiptEvent = wire(7, JSON.stringify({ kind: DIRECTORY_PROJECTION_RECEIPT_SCHEMA, payload: receipt }));
        expect(() => invocationFromFrames([], consumeTypedOperationEffects([receiptEvent]), "action")).toThrow("before typed-operation terminal");
        const terminalOutput = consumeTypedOperationEffects([receiptEvent, wire(10, "typed-operation-complete")]);
        expect(invocationFromFrames([], terminalOutput, "action").output).toEqual(receipt);
        const splitOutput = consumeTypedOperationEffects([...consumeTypedOperationEffects([receiptEvent]), ...consumeTypedOperationEffects([wire(10, "typed-operation-complete")])]);
        expect(invocationFromFrames([], splitOutput, "action").output).toEqual(receipt);
        let acknowledged = false;
        await withFakeShardClient(async (_actor, events) => {
          acknowledged = events.length === 1 && events[0]?.kind === "message";
          return { uiPatches: [], effects: [], nextWake: null, status: { tag: "idle" } };
        }, async () => {
          await expect(settlePluginTurn("faulted-operation#7", { uiPatches: [], effects: [wire(11, fixture.wire.fault)], nextWake: null, status: { tag: "idle" } }, "Interactive", new Set(), undefined, true)).rejects.toThrow(fixture.wire.fault);
          expect(acknowledged).toBe(true);
        });
      });
  
      it("routes a foreign typed-operation fault page to its own call and never to the observing call", async () => {
        const { Buffer } = await import("node:buffer");
        const { default: Ajv } = await import("ajv");
        const { default: routing } = await import("../../🧱️elements/🔌️PluginRuntime/🧫️fixtures/📬️typed-operation-routing.json");
        const { default: rendererModule } = await import("../../../🧬️schema/🔣️.json");
        const schema = { $ref: `${rendererModule.$id}#/$defs/PluginRuntimeTypedOperationRoutingV1` };
        expect(new Ajv({ strict: true }).addSchema(rendererModule).compile(schema)(routing)).toBe(true);
        expect([TYPED_OPERATION_PARK_CAPACITY, TYPED_OPERATION_UNATTRIBUTED_FAULT, TYPED_OPERATION_PARK_EVICTION_FAULT]).toEqual([routing.capacity, routing.unattributedCode, routing.evictionCode]);
        const page = (operation: string, sequence: number, lane: number, text: string) => {
          const body = Buffer.alloc(30);
          body.writeUInt32LE(routing.receiver, 0);
          body.writeBigUInt64LE(BigInt(operation), 4);
          body.writeBigUInt64LE(1n, 12);
          body.writeUInt32LE(sequence, 20);
          body[24] = 1;
          body[25] = lane;
          const payload = Buffer.from(text);
          body.writeUInt32LE(payload.length, 26);
          return { tag: "send-message", val: { target: { tag: "shell", val: String(routing.receiver) }, payload: Array.from(Buffer.concat([Buffer.from("semio.typed-operation-page.v1\0"), body, payload])) } };
        };
        const actorId = `routed-operation#${routing.receiver}`;
        const faulted = { uiPatches: [], effects: [page(routing.foreignOperation, routing.faultSequence, 11, routing.fault)], nextWake: null, status: { tag: "idle" } };
        const acknowledged: unknown[][] = [];
        let observerSettled = false;
        await expect(withTypedOperationCall(actorId, "owner", async (owner: any) => {
          expect(consumeTypedOperationEffects([page(routing.foreignOperation, routing.firstSequence, 0, "first")], owner)).toEqual([]);
          await withTypedOperationCall(actorId, "observer", async (observer: any) => {
            await withFakeShardClient(async (_actor, events) => {
              acknowledged.push([...events]);
              return { uiPatches: [], effects: [], nextWake: null, status: { tag: "idle" } };
            }, async () => {
              const settled = await settlePluginTurn(actorId, faulted, "Interactive", new Set(), undefined, true, undefined, observer);
              expect(settled.effects).toEqual([]);
            });
          });
          observerSettled = true;
        })).rejects.toThrow(routing.fault);
        expect(observerSettled).toBe(true);
        expect(acknowledged).toEqual([typedOperationAcknowledgements(faulted)]);
        teardownPluginActor(actorId);
        console.info("[DEBUG] typed-operation routing: observer settled=%s acknowledgedTurns=%d", observerSettled, acknowledged.length);
      });

      it("bounds parked typed-operation faults at a fixed capacity and surfaces the evicted oldest", async () => {
        const { default: routing } = await import("../../🧱️elements/🔌️PluginRuntime/🧫️fixtures/📬️typed-operation-routing.json");
        const reported: { code: string; message: string }[] = [];
        const router = new TypedOperationRouter((code: string, message: string) => reported.push({ code, message }));
        const owner = router.open("owner");
        const observer = router.open("observer");
        const total = routing.capacity + routing.overflow;
        for (let index = 0; index < total; index += 1) {
          const operation = BigInt(routing.ownedOperation) + BigInt(index);
          expect(router.observe(owner, operation, routing.firstSequence)).toBe(true);
          expect(router.fault(observer, operation, routing.faultSequence, `${routing.fault} ${index}`)).toBe(false);
        }
        expect(reported.map((entry) => entry.code)).toEqual(Array.from({ length: routing.overflow }, () => routing.evictionCode));
        const drained: string[] = [];
        for (let fault = owner.takeFault(); fault !== null; fault = owner.takeFault()) drained.push(fault);
        expect([drained.length, drained[0], drained.at(-1)]).toEqual([routing.capacity, `${routing.fault} ${routing.overflow}`, `${routing.fault} ${total - 1}`]);
        const unowned = BigInt(routing.foreignOperation);
        expect(router.fault(observer, unowned, routing.faultSequence, routing.fault)).toBe(false);
        expect(reported.at(-1)).toEqual({ code: routing.unattributedCode, message: `${routing.fault} (operation ${unowned} sequence ${routing.faultSequence} observed by observer)` });
        owner.close();
        observer.close();
        expect(reported.length).toBe(routing.overflow + 1);
        console.info("[DEBUG] typed-operation park capacity: parked=%d evicted=%d unattributed=1", drained.length, routing.overflow);
      });

      it("keeps the command reply when publication supplies only an unsolicited UI scope", async () => {
        const { default: fixture } = await import("../../../../🔌️plugin/⚛️reactor/🧫️fixtures/🔣️.json");
        const bytes = (value: unknown) => Array.from(encodePackValue(value));
        const frames = [
          { Invocation: { in_reply_to: 1, output: bytes({ operationId: fixture.wire.operation }), diagnostics: bytes([]), ui_scope: bytes({ kind: "none" }), history_patch: [], messages: [], mutations: [], inverse_group: [] } },
          { Invocation: { in_reply_to: 0, output: [], diagnostics: [], ui_scope: bytes({ kind: "full" }), history_patch: [], messages: [], mutations: [], inverse_group: [] } },
        ];
        const client = { command: async () => frames } as unknown as AppChannelClient;
        const result = await performInvocation(client, 7, { address: { pluginId: "procedural", appId: "s.procedural.procedural3d@1/*#editor", windowKindId: "procedural-main", actionId: "commitFixture" }, arguments: {} }, "action", { locale: "en", terminology: "native" });
        expect(result.output).toEqual({ operationId: fixture.wire.operation });
        expect(result.uiScope).toEqual({ kind: "full" });
      });
  
      it("drains an actor's more-work turns until the reconciled UI patch is publishable", async () => {
        let continuationCount = 0;
        await withFakeShardClient(
          async () => {
            continuationCount += 1;
            return { uiPatches: [{ revision: 1, baseRevision: 0, ops: [] }], effects: [], nextWake: null, status: { tag: "idle" } };
          },
          async () => {
            const actorId = "turn-test-more-work-actor";
            const result = await settlePluginTurn(actorId, { uiPatches: [], effects: [], nextWake: null, status: { tag: "more-work" } } as unknown as WireTurnResult, "UserVisible");
            expect(continuationCount).toBe(1);
            expect(result.uiPatches).toHaveLength(1);
          },
        );
      });
  
      it("acknowledges each retained surface before requesting the next bounded publication", async () => {
        const { default: fixture } = await import("../../🧱️elements/🔌️PluginRuntime/🧫️fixtures/🔄️surface-refresh.json");
        const surfaces = fixture.acknowledgement.surfaces;
        const submitted: string[][] = [];
        await withFakeShardClient(
          async (_actor, events) => {
            const acknowledgements = events.filter((event) => event.kind === "patch-ack").map((event) => (event.payload as { surface: { surface: string } }).surface.surface);
            submitted.push(acknowledgements);
            return {
              uiPatches: acknowledgements.includes(surfaces[0]!) ? [{ surface: pluginSurfaceRef(7, surfaces[1]!), revision: 1, baseRevision: 0, ops: [] }] : [],
              effects: [], nextWake: null, status: { tag: "idle" },
              uiPatchReceipt: encodeActorUiPatchReceipt({ lifetime: { activationGeneration: 1n, instanceId: 7, guestLifetime: 1n }, patchSequence: 1n }),
            };
          },
          async () => {
            const result = await settlePluginTurn(
              "bounded-panel-publication#1",
              { uiPatches: [{ surface: pluginSurfaceRef(7, surfaces[0]!), revision: 1, baseRevision: 0, ops: [] }], effects: [], nextWake: null, status: { tag: "more-work" }, uiPatchReceipt: encodeActorUiPatchReceipt({ lifetime: { activationGeneration: 1n, instanceId: 7, guestLifetime: 1n }, patchSequence: 1n }) },
              "UserVisible",
              new Set(surfaces.map((surface) => retainedSurfaceId(7, surface))),
              (turn) => patchAckEvents(turn, turn.uiPatches),
            );
            expect(submitted).toEqual(surfaces.map((surface) => [surface]));
            expect(submitted.every((batch) => batch.length <= fixture.acknowledgement.maxUnacknowledged)).toBe(true);
            expect(result.uiPatches.map((patch) => patch.surface?.surface)).toEqual(surfaces);
          },
        );
      });
  
      it("rejects an idle actor that did not publish a requested surface instead of retaining a loading placeholder", async () => {
        const { default: fixture } = await import("../../🧱️elements/🔌️PluginRuntime/🧫️fixtures/🔄️surface-refresh.json");
        const { instance, requested, published, missing } = fixture.quiescent;
        await expect(settlePluginTurn(
          "idle-missing-surface#1",
          { uiPatches: published.map((surface) => ({ surface: pluginSurfaceRef(instance, surface), revision: 1, baseRevision: 0, ops: [] })), effects: [], nextWake: null, status: { tag: "idle" } },
          "UserVisible",
          new Set(requested.map((surface) => retainedSurfaceId(instance, surface))),
        )).rejects.toThrow(`missing=${JSON.stringify(missing)}`);
      });
  
      /** 😴️ Wave W-B14 (ticket 26/09/02): a settle with nothing required used to stop at the FIRST
       * acknowledgement-free continuation, which ended the turn of a mutation whose typed operation
       * was still in flight. An actor that says `more-work` is quiesced FOR THIS TURN only once it
       * has published, acknowledged and emitted nothing for a whole continuation batch — and that is
       * a quiet return, not the stall fault, because nothing was requested of it. */
      it("treats an actor answering more-work with nothing to publish as quiesced only after a whole silent batch", async () => {
        let continuationCount = 0;
        await withFakeShardClient(
          async () => {
            continuationCount += 1;
            return { uiPatches: [], effects: [], nextWake: null, status: { tag: "more-work" } };
          },
          async () => {
            const result = await settlePluginTurn(
              "empty-required-drain#1",
              { uiPatches: [], effects: [], nextWake: null, status: { tag: "more-work" } } as unknown as WireTurnResult,
              "Interactive",
              new Set(),
              undefined,
              true,
            );
            expect(continuationCount).toBe(PLUGIN_UI_QUIESCENT_CONTINUATIONS);
            expect(PLUGIN_UI_QUIESCENT_CONTINUATIONS).toBe(PLUGIN_UI_ZERO_PROGRESS_CONTINUATION_LIMIT);
            expect(result.uiPatches).toHaveLength(0);
          },
        );
      });

      /** 📄️ The drop W-B14 fixes: `addObjectKind`/`addTargetVolume` reached the guest and settled with
       * `frameKinds:["Invocation","Ephemeral"]` and `historyUpserts:0` — the mutation's own frame is
       * emitted several acknowledgement-free continuations after the command itself completed, and the
       * old stop ended the turn before it, so the publication was dropped. */
      it("collects a mutation frame emitted several acknowledgement-free continuations after the command completed", async () => {
        const documentEffect = { tag: "send-message", val: { target: { tag: "shell", val: "7" }, payload: [1, 2, 3] } };
        let continuationCount = 0;
        await withFakeShardClient(
          async () => {
            continuationCount += 1;
            const publishing = continuationCount === PLUGIN_UI_QUIESCENT_CONTINUATIONS;
            return { uiPatches: [], effects: publishing ? [documentEffect] : [], nextWake: null, status: { tag: publishing ? "idle" : "more-work" } };
          },
          async () => {
            const result = await settlePluginTurn(
              "mutation-publication#1",
              { uiPatches: [], effects: [], nextWake: null, status: { tag: "more-work" } } as unknown as WireTurnResult,
              "Interactive",
              new Set(),
              undefined,
              true,
            );
            expect(continuationCount).toBe(PLUGIN_UI_QUIESCENT_CONTINUATIONS);
            expect(result.effects).toEqual([documentEffect]);
          },
        );
      });

      /** 🔇️ A settle is silent while it works: its former per-continuation `console.warn` charged
       * 108–244 ms of main thread per example switch (`📓️2026-09-11-wave-B4-main-thread-refresh.md`
       * §5.2, 2 505–6 624 lines). Diagnosis belongs to the faults, which name the actor, the pending
       * surfaces and the bound they hit. */
      it("prints no console warning while a healthy actor settles its continuations", async () => {
        const warn = vi.spyOn(console, "warn").mockImplementation(() => {});
        try {
          let continuationCount = 0;
          await withFakeShardClient(
            async () => {
              continuationCount += 1;
              const done = continuationCount === 3;
              return {
                uiPatches: [{ surface: pluginSurfaceRef(7, "quiet"), revision: continuationCount, baseRevision: continuationCount - 1, ops: [] }],
                effects: [], nextWake: null, status: { tag: done ? "idle" : "more-work" },
              };
            },
            async () => {
              const result = await settlePluginTurn(
                "quiet-settle#1",
                { uiPatches: [], effects: [], nextWake: null, status: { tag: "more-work" } } as unknown as WireTurnResult,
                "Interactive",
                new Set(),
                undefined,
                true,
              );
              expect(continuationCount).toBe(3);
              expect(result.uiPatches).toHaveLength(3);
            },
          );
          expect(warn).not.toHaveBeenCalled();
        } finally {
          warn.mockRestore();
        }
      });

      /** 🛑️ Wave W-B2 (ticket 26/09/02): a guest that answers `more-work` while publishing,
       * acknowledging and emitting NOTHING is not "still working", it is stalled — and the settle
       * loop used to spend all 4 096 continuations (4 096 host round trips) discovering that, then
       * throw a timeout naming neither the surface nor the reason. It must now stop after the
       * contract-derived zero-progress bound and name the surface it is waiting on. */
      it("fails fast and names the pending surface when a guest answers more-work without publishing anything", async () => {
        let continuationCount = 0;
        await withFakeShardClient(
          async () => {
            continuationCount += 1;
            return { uiPatches: [], effects: [], nextWake: null, status: { tag: "more-work" } };
          },
          async () => {
            const surface = retainedSurfaceId(7, "puzzle3d-main-perspective");
            await expect(settlePluginTurn(
              "zero-progress-settle#1",
              { uiPatches: [], effects: [], nextWake: null, status: { tag: "more-work" } } as unknown as WireTurnResult,
              "Interactive",
              new Set([surface]),
            )).rejects.toThrow(`pending=${JSON.stringify([surface])}`);
            expect(continuationCount).toBe(PLUGIN_UI_ZERO_PROGRESS_CONTINUATION_LIMIT);
            expect(PLUGIN_UI_ZERO_PROGRESS_CONTINUATION_LIMIT).toBeLessThan(PLUGIN_UI_CONTINUATION_LIMIT);
            console.info("[DEBUG] zero-progress settle: continuations=%d limit=%d spinLimit=%d", continuationCount, PLUGIN_UI_ZERO_PROGRESS_CONTINUATION_LIMIT, PLUGIN_UI_CONTINUATION_LIMIT);
          },
        );
      });

      /** ✅️ The successful path is untouched: any continuation that publishes resets the streak, so a
       * guest still publishing OTHER surfaces while the requested one is pending keeps its full
       * continuation budget — the bound is on zero progress, not on continuations. */
      it("does not fast-fail a guest that keeps publishing while the requested surface is still pending", async () => {
        let continuationCount = 0;
        const total = PLUGIN_UI_ZERO_PROGRESS_CONTINUATION_LIMIT * 2;
        await withFakeShardClient(
          async () => {
            continuationCount += 1;
            const last = continuationCount >= total;
            return {
              uiPatches: [{ surface: pluginSurfaceRef(7, last ? "puzzle3d-main-perspective" : "puzzle3d-outliner"), revision: 1, baseRevision: 0, ops: [] }],
              effects: [], nextWake: null, status: { tag: last ? "idle" : "more-work" },
            };
          },
          async () => {
            const result = await settlePluginTurn(
              "slow-but-progressing-settle#1",
              { uiPatches: [], effects: [], nextWake: null, status: { tag: "more-work" } } as unknown as WireTurnResult,
              "Interactive",
              new Set([retainedSurfaceId(7, "puzzle3d-main-perspective")]),
            );
            expect(continuationCount).toBe(total);
            expect(result.uiPatches).toHaveLength(total);
          },
        );
      });

      it("does not chase background work during instance-open before a UI surface is requested", async () => {
        let continuationCount = 0;
        await withFakeShardClient(
          async () => {
            continuationCount += 1;
            return { uiPatches: [], effects: [], nextWake: null, status: { tag: "more-work" } };
          },
          async () => {
            const result = await settlePluginTurn(
              "instance-open-with-background-work#1",
              { uiPatches: [], effects: [], nextWake: null, status: { tag: "more-work" } } as unknown as WireTurnResult,
              "Interactive",
              new Set(),
            );
            expect(continuationCount).toBe(0);
            expect(result.uiPatches).toHaveLength(0);
          },
        );
      });
  
      it("drains until every missing requested surface has published its first patch", async () => {
        let continuationCount = 0;
        await withFakeShardClient(
          async () => {
            continuationCount += 1;
            const surface = continuationCount === 1 ? "workflow" : "preview";
            return {
              uiPatches: [{ surface: pluginSurfaceRef(7, surface), revision: 1, baseRevision: 0, ops: [] }],
              effects: [],
              nextWake: null,
              status: { tag: continuationCount === 1 ? "more-work" : "idle" },
            };
          },
          async () => {
            const actorId = "turn-test-multiple-surfaces-actor";
            const result = await settlePluginTurn(
              actorId,
              { uiPatches: [], effects: [], nextWake: null, status: { tag: "more-work" } } as unknown as WireTurnResult,
              "UserVisible",
              new Set([retainedSurfaceId(7, "workflow"), retainedSurfaceId(7, "preview")]),
            );
            expect(continuationCount).toBe(2);
            expect(result.uiPatches.map((patch) => patch.surface?.surface)).toEqual(["workflow", "preview"]);
          },
        );
      });
  
      /** 📏️ A large retained surface may still take far more continuations than the former 1 024
       * ceiling — the settle is bounded by PROGRESS, not by round trips. What it may not do is spend
       * them in silence: W-B2 (ticket 26/09/02) made every retirement ladder page-priced, so a guest
       * that publishes, acknowledges and emits nothing for
       * {@link PLUGIN_UI_ZERO_PROGRESS_CONTINUATION_LIMIT} consecutive turns is stalled, not busy —
       * this guest publishes its section surfaces as it goes and its requested one at the end. */
      it("allows a large retained surface to reconcile beyond the former continuation ceiling while it keeps publishing", async () => {
        let continuationCount = 0;
        await withFakeShardClient(
          async () => {
            continuationCount += 1;
            const done = continuationCount === 1_025;
            const progressing = continuationCount % 32 === 0;
            return {
              uiPatches: done
                ? [{ surface: pluginSurfaceRef(9, "large"), revision: 1, baseRevision: 0, ops: [] }]
                : progressing
                  ? [{ surface: pluginSurfaceRef(9, "large-sections"), revision: continuationCount, baseRevision: continuationCount - 1, ops: [] }]
                  : [],
              effects: [],
              nextWake: null,
              status: { tag: done ? "idle" : "more-work" },
            };
          },
          async () => {
            const result = await settlePluginTurn(
              "turn-test-large-surface-actor",
              { uiPatches: [], effects: [], nextWake: null, status: { tag: "more-work" } } as unknown as WireTurnResult,
              "UserVisible",
              new Set([retainedSurfaceId(9, "large")]),
            );
            expect(continuationCount).toBe(1_025);
            expect(continuationCount).toBeGreaterThan(PLUGIN_UI_ZERO_PROGRESS_CONTINUATION_LIMIT);
            expect(result.uiPatches.at(-1)?.surface?.surface).toBe("large");
          },
        );
      });
  
      it("yields the browser event loop while a retained surface needs several continuation batches", async () => {
        let continuationCount = 0;
        let browserTaskObserved = false;
        const browserTask = yieldPluginUiContinuation().then(() => {
          browserTaskObserved = true;
        });
        await withFakeShardClient(
          async () => {
            continuationCount += 1;
            return {
              uiPatches: continuationCount === PLUGIN_UI_CONTINUATION_BATCH_SIZE + 1 ? [{ surface: pluginSurfaceRef(9, "cooperative"), revision: 1, baseRevision: 0, ops: [] }] : [],
              effects: [],
              nextWake: null,
              status: { tag: continuationCount === PLUGIN_UI_CONTINUATION_BATCH_SIZE + 1 ? "idle" : "more-work" },
            };
          },
          async () => {
            const result = await settlePluginTurn(
              "turn-test-cooperative-surface-actor",
              { uiPatches: [], effects: [], nextWake: null, status: { tag: "more-work" } } as unknown as WireTurnResult,
              "UserVisible",
              new Set([retainedSurfaceId(9, "cooperative")]),
            );
            expect(browserTaskObserved).toBe(true);
            expect(result.uiPatches).toHaveLength(1);
          },
        );
        await browserTask;
      });
  
      it("does not poll for an unchanged refresh when every requested surface is already retained", async () => {
        let continuationCount = 0;
        await withFakeShardClient(
          async () => {
            continuationCount += 1;
            return { uiPatches: [], effects: [], nextWake: null, status: { tag: "more-work" } };
          },
          async () => {
            const result = await settlePluginTurn(
              "turn-test-retained-surfaces-actor",
              { uiPatches: [], effects: [], nextWake: null, status: { tag: "more-work" } } as unknown as WireTurnResult,
              "UserVisible",
              new Set(),
            );
            expect(continuationCount).toBe(0);
            expect(result.uiPatches).toHaveLength(0);
          },
        );
      });
  
      it("dispatches a queued Interactive-lane turn before an already-queued UserVisible-lane turn for the SAME actor, regardless of arrival order", async () => {
        const dispatchOrder: string[] = [];
        await withFakeShardClient(
          async (_actorId, events) => {
            dispatchOrder.push((events[0]!.payload as { readonly marker: string }).marker);
            return { uiPatches: [], effects: [], nextWake: null };
          },
          async () => {
            const actorId = "turn-test-lane-actor";
            // 🎯️ UserVisible enqueued FIRST, Interactive second — both land before the scheduler's first
            // microtask pump, so dispatch order must reflect lane priority, not arrival order.
            const userVisible = submitPluginTurn(actorId, [{ kind: "surface-visible", payload: { marker: "user-visible-1" } }], "UserVisible", "surface-visible");
            const interactive = submitPluginTurn(actorId, [{ kind: "app-command", payload: { marker: "interactive-1" } }], "Interactive");
            await Promise.all([userVisible, interactive]);
            expect(dispatchOrder).toEqual(["interactive-1", "user-visible-1"]);
          },
        );
      });
  
      it("collapses a 200-call coalescing burst to a single dispatched turn, resolving EVERY waiter (not just the last) with the winning result", async () => {
        let dispatchCount = 0;
        await withFakeShardClient(
          async (_actorId, events) => {
            dispatchCount += 1;
            return { uiPatches: [], effects: [{ tag: "notify", val: { message: String((events[0]!.payload as { readonly marker: number }).marker) } }], nextWake: null };
          },
          async () => {
            const actorId = "turn-test-coalesce-actor";
            const results = await Promise.all(
              Array.from({ length: 200 }, (_, index) => submitPluginTurn(actorId, [{ kind: "surface-visible", payload: { marker: index } }], "UserVisible", "surface-visible")),
            );
            expect(dispatchCount).toBe(1);
            for (const result of results) expect(result.effects).toEqual(results[0]!.effects);
          },
        );
      });
  
      it("teardown rejects queued turn waiters before disposing the actor transport", async () => {
        let release!: () => void;
        const gate = new Promise<unknown>((resolve) => {
          release = () => resolve({ uiPatches: [], effects: [], nextWake: null });
        });
        await withFakeShardClient(
          async () => gate,
          async () => {
            const actorId = "turn-test-teardown-actor";
            const inFlight = submitPluginTurn(actorId, [{ kind: "app-command", payload: { marker: "in-flight" } }], "Interactive");
            await flushMicrotasks();
            const queued = submitPluginTurn(actorId, [{ kind: "app-command", payload: { marker: "queued" } }], "Interactive");
            teardownPluginActor(actorId);
            await expect(queued).rejects.toThrow(`plugin actor ${actorId} disposed`);
            release();
            await expect(inFlight).resolves.toMatchObject({ uiPatches: [], effects: [] });
          },
        );
      });
  
      it("surfaces Rejected once an actor's mailbox is genuinely full of distinct turns, instead of growing without bound", async () => {
        await withFakeShardClient(
          () => new Promise(() => {}), // 🎯️ never settles — every submitted turn for this actor stays queued behind the first, in flight forever.
          async () => {
            const actorId = "turn-test-rejected-actor";
            const settled: Array<"accepted" | "rejected"> = [];
            // 🎯️ One turn dispatches immediately (goes "in flight"); the rest queue behind it in the same
            // lane (no coalescing, no lower lane to evict) until PLUGIN_TURN_MAILBOX_CAPACITY (32) is hit.
            for (let index = 0; index < 41; index += 1) {
              submitPluginTurn(actorId, [{ kind: "app-command", payload: { marker: `t-${index}` } }], "Interactive").then(
                () => settled.push("accepted"),
                () => settled.push("rejected"),
              );
            }
            await flushMicrotasks();
            expect(settled.filter((outcome) => outcome === "rejected").length).toBeGreaterThan(0);
          },
        );
      });
  
      it("never dispatches a second turn for an actor before the first settles, even while a DIFFERENT actor's turns run concurrently", async () => {
        const inFlightByActor = new Map<string, number>();
        const maxInFlightByActor = new Map<string, number>();
        await withFakeShardClient(
          async (actorId) => {
            inFlightByActor.set(actorId, (inFlightByActor.get(actorId) ?? 0) + 1);
            maxInFlightByActor.set(actorId, Math.max(maxInFlightByActor.get(actorId) ?? 0, inFlightByActor.get(actorId)!));
            await flushMicrotasks(2);
            inFlightByActor.set(actorId, inFlightByActor.get(actorId)! - 1);
            return { uiPatches: [], effects: [], nextWake: null };
          },
          async () => {
            const actorA = "turn-test-interleave-a";
            const actorB = "turn-test-interleave-b";
            await Promise.all([
              submitPluginTurn(actorA, [{ kind: "app-command", payload: {} }], "Interactive"),
              submitPluginTurn(actorA, [{ kind: "app-command", payload: {} }], "Interactive"),
              submitPluginTurn(actorA, [{ kind: "app-command", payload: {} }], "Interactive"),
              submitPluginTurn(actorB, [{ kind: "app-command", payload: {} }], "Interactive"),
            ]);
            expect(maxInFlightByActor.get(actorA)).toBe(1);
            expect(maxInFlightByActor.get(actorB)).toBe(1);
          },
        );
      });
    });
  
    describe("PluginRuntime shard-loss wiring (real restore, not just a console.error)", () => {
      it("handlePluginShardLost delegates to ActivationRegistry.handleShardLost for EXACTLY the affected actorIds", () => {
        const restoreCalls: Array<{ readonly shardIndex: number; readonly actorIds: readonly string[] }> = [];
        const fakeRegistry = { handleShardLost: (shardIndex: number, actorIds: readonly string[]) => restoreCalls.push({ shardIndex, actorIds }) } as unknown as ActivationRegistry;
        const previous = testState.sharedActivationRegistry;
        testState.sharedActivationRegistry = fakeRegistry;
        try {
          handlePluginShardLost(2, ["plugin-a#1", "plugin-b#7"]);
          expect(restoreCalls).toEqual([{ shardIndex: 2, actorIds: ["plugin-a#1", "plugin-b#7"] }]);
        } finally {
          testState.sharedActivationRegistry = previous;
        }
      });
  
      /** ⚖️ LAW: restoring the ACTOR is not restoring the APP. A watchdog kill takes no checkpoint, so
       * the guest comes back with no `createApp` instance at all and every later command page fails
       * the guest's own liveness check (`plugin.command-page-invalid`) — the session is dead until the
       * user reloads. The runtime must therefore retire each lost instance's bookkeeping AND announce
       * it, so whoever created the instance can create it again
       * (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️extension-evaluate-budget-2026-09-12.md`). */
      it("handlePluginShardLost retires the app instances those actors hosted and announces exactly them", () => {
        const released: string[] = [];
        const announced: Array<{ readonly shardIndex: number; readonly lost: readonly { readonly pluginId: string; readonly instanceId: number; readonly actorId: string }[] }> = [];
        const previous = testState.sharedActivationRegistry;
        testState.sharedActivationRegistry = { handleShardLost: () => {} } as unknown as ActivationRegistry;
        const unsubscribe = onPluginInstancesLost((shardIndex, lost) => announced.push({ shardIndex, lost }));
        rememberInstanceForRecovery({ pluginId: "procedural", instanceId: 1, actorId: "procedural#1", release: () => released.push("procedural#1") });
        rememberInstanceForRecovery({ pluginId: "other", instanceId: 4, actorId: "other#4", release: () => released.push("other#4") });
        try {
          // 🪪️ `flow-extension-brep#request` is an extension's REQUEST actor: it owns no app instance,
          // so it is restored and contributes nothing to announce.
          handlePluginShardLost(0, ["procedural#1", "flow-extension-brep#request"]);
          expect(released).toEqual(["procedural#1"]);
          expect(announced).toHaveLength(1);
          expect(announced[0]?.shardIndex).toBe(0);
          expect(announced[0]?.lost).toEqual([{ pluginId: "procedural", instanceId: 1, actorId: "procedural#1" }]);
          // 🧹️ Retiring is idempotent: a second loss of the same actor announces nothing, because the
          // instance is already gone.
          handlePluginShardLost(0, ["procedural#1"]);
          expect(released).toEqual(["procedural#1"]);
          expect(announced).toHaveLength(1);
          // 🧹️ An instance that closed normally is forgotten, so its later shard loss is silent too.
          forgetInstanceForRecovery("other#4");
          handlePluginShardLost(1, ["other#4"]);
          expect(released).toEqual(["procedural#1"]);
          expect(announced).toHaveLength(1);
        } finally {
          unsubscribe();
          forgetInstanceForRecovery("procedural#1");
          forgetInstanceForRecovery("other#4");
          testState.sharedActivationRegistry = previous;
        }
      });

      /** ⚖️ LAW: a shard loss with NO live app instance on it announces nothing at all — an extension
       * request actor dying is a retryable request failure, not a lost session. */
      it("a lost shard that hosted no app instance announces nothing", () => {
        const announced: unknown[] = [];
        const previous = testState.sharedActivationRegistry;
        testState.sharedActivationRegistry = { handleShardLost: () => {} } as unknown as ActivationRegistry;
        const unsubscribe = onPluginInstancesLost((_shardIndex, lost) => announced.push(lost));
        try {
          handlePluginShardLost(3, ["flow-extension-brep#request"]);
          expect(announced).toEqual([]);
          expect(PLUGIN_ACTOR_INSTANCE_LOST_FAULT).toBe("plugin.actor-instance-lost");
        } finally {
          unsubscribe();
          testState.sharedActivationRegistry = previous;
        }
      });

      it("buildShardClientOptions wires onShardLost to handlePluginShardLost (not a bare console.error) and sizes shardCount via poolConcurrency", () => {
        const fakeCreateWorker = () => ({ postMessage: () => {}, terminate: () => {}, onmessage: null, onerror: null }) as unknown as ShardWorkerLike;
        const options = buildShardClientOptions(fakeCreateWorker);
        expect(options.onShardLost).toBe(handlePluginShardLost);
        expect(options.shardCount).toBe(poolConcurrency());
        expect(options.createWorker).toBe(fakeCreateWorker);
      });
    });
  
    describe("fetchDescriptorManifest AbortSignal", () => {
      it("propagates an aborted signal's fetch rejection instead of silently falling back to an empty manifest", async () => {
        const controller = new AbortController();
        controller.abort();
        const originalFetch = globalThis.fetch;
        globalThis.fetch = (async () => {
          throw new DOMException("aborted", "AbortError");
        }) as typeof fetch;
        try {
          await expect(fetchDescriptorManifest("p", "https://x/p.js", controller.signal)).rejects.toThrow();
        } finally {
          globalThis.fetch = originalFetch;
        }
      });
  
      it("propagates a network failure without manufacturing an empty descriptor", async () => {
        const originalFetch = globalThis.fetch;
        const failure = new Error("network down");
        globalThis.fetch = (async () => {
          throw failure;
        }) as typeof fetch;
        try {
          await expect(fetchDescriptorManifest("p", "https://x/p.js")).rejects.toBe(failure);
        } finally {
          globalThis.fetch = originalFetch;
        }
      });
  
      it("rejects the dev server's HTML SPA fallback without a parse warning", async () => {
        const originalFetch = globalThis.fetch;
        const originalWarn = console.warn;
        let warningCount = 0;
        console.warn = () => {
          warningCount += 1;
        };
        globalThis.fetch = (async () => new Response("<!doctype html>", { status: 200, headers: { "content-type": "text/html" } })) as typeof fetch;
        try {
          await expect(fetchDescriptorManifest("p", "https://x/p.js")).rejects.toThrow("plugin.descriptor-invalid");
          expect(warningCount).toBe(0);
        } finally {
          console.warn = originalWarn;
          globalThis.fetch = originalFetch;
        }
      });
    });
  
    describe("loadPluginModulesInDependencyOrder — level-parallel boot", () => {
      it("runs independent siblings in parallel within a level, holding a dependent until its dependency's WHOLE level finishes", async () => {
        const started: string[] = [];
        const releaseFns = new Map<string, () => void>();
        const loadModule = (pluginId: string): Promise<PluginWasmHandle> =>
          new Promise<PluginWasmHandle>((resolve) => {
            started.push(pluginId);
            releaseFns.set(pluginId, () => resolve(fakeHandle(pluginId, [], [])));
          });
        const entries: PluginRegistryEntry[] = [
          { pluginId: "a", moduleUrl: "https://x/a.js", dependencies: [] },
          { pluginId: "b", moduleUrl: "https://x/b.js", dependencies: [] },
          { pluginId: "c", moduleUrl: "https://x/c.js", dependencies: [{ pluginId: "a", version: "*" }] },
        ];
        const resultPromise = loadPluginModulesInDependencyOrder(entries, { loadModule, concurrency: 4 });
        await flushMicrotasks();
        expect(new Set(started)).toEqual(new Set(["a", "b"]));
        expect(started).not.toContain("c");
        releaseFns.get("a")!();
        releaseFns.get("b")!();
        await flushMicrotasks();
        expect(started).toContain("c");
        releaseFns.get("c")!();
        const result = await resultPromise;
        expect(result.handles.map((handle) => handle.pluginId)).toEqual(["a", "b", "c"]);
        expect(result.errors).toEqual([]);
        expect(result.loadFailures).toEqual([]);
      });
  
      it("bounds within-level concurrency to the given limit — a third independent sibling waits for a free slot", async () => {
        const started: string[] = [];
        const releaseFns: Array<() => void> = [];
        const loadModule = (pluginId: string): Promise<PluginWasmHandle> =>
          new Promise<PluginWasmHandle>((resolve) => {
            started.push(pluginId);
            releaseFns.push(() => resolve(fakeHandle(pluginId, [], [])));
          });
        const entries: PluginRegistryEntry[] = ["a", "b", "c"].map((id) => ({ pluginId: id, moduleUrl: `https://x/${id}.js`, dependencies: [] }));
        const resultPromise = loadPluginModulesInDependencyOrder(entries, { loadModule, concurrency: 2 });
        await flushMicrotasks();
        expect(started).toHaveLength(2);
        releaseFns[0]!();
        await flushMicrotasks();
        expect(started).toHaveLength(3);
        releaseFns[1]!();
        releaseFns[2]!();
        await resultPromise;
      });
  
      it("cascades a runtime load failure to skip dependents, while unrelated siblings still load — a distinct PluginLoadFailure, not a PluginGraphError", async () => {
        const loadModule = (pluginId: string): Promise<PluginWasmHandle> => (pluginId === "a" ? Promise.reject(new Error("boom")) : Promise.resolve(fakeHandle(pluginId, [], [])));
        const entries: PluginRegistryEntry[] = [
          { pluginId: "a", moduleUrl: "https://x/a.js", dependencies: [] },
          { pluginId: "b", moduleUrl: "https://x/b.js", dependencies: [] },
          { pluginId: "c", moduleUrl: "https://x/c.js", dependencies: [{ pluginId: "a", version: "*" }] },
        ];
        const result = await loadPluginModulesInDependencyOrder(entries, { loadModule, concurrency: 4 });
        expect(result.handles.map((handle) => handle.pluginId)).toEqual(["b"]);
        expect(result.errors).toEqual([]);
        expect(result.loadFailures.map((failure) => failure.pluginId).sort()).toEqual(["a", "c"]);
      });
  
      it("defaults its concurrency bound to poolConcurrency() when the caller doesn't override it", async () => {
        const cap = poolConcurrency();
        let inFlight = 0;
        let maxInFlight = 0;
        const loadModule = (pluginId: string): Promise<PluginWasmHandle> =>
          new Promise<PluginWasmHandle>((resolve) => {
            inFlight += 1;
            maxInFlight = Math.max(maxInFlight, inFlight);
            queueMicrotask(() => {
              inFlight -= 1;
              resolve(fakeHandle(pluginId, [], []));
            });
          });
        const entries: PluginRegistryEntry[] = Array.from({ length: cap + 6 }, (_, index) => ({ pluginId: `p${index}`, moduleUrl: `https://x/p${index}.js`, dependencies: [] }));
        await loadPluginModulesInDependencyOrder(entries, { loadModule });
        expect(maxInFlight).toBeLessThanOrEqual(cap);
        expect(maxInFlight).toBeGreaterThan(0);
      });
  
      it("aborts cleanly: aborting mid-boot stops starting new loads without throwing, while an already-started load still settles normally", async () => {
        const controller = new AbortController();
        const started: string[] = [];
        const loadModule = (pluginId: string): Promise<PluginWasmHandle> => {
          started.push(pluginId);
          if (pluginId === "a") controller.abort(); // 🎯️ abort while level 0 ("a") is already in flight.
          return Promise.resolve(fakeHandle(pluginId, [], []));
        };
        const entries: PluginRegistryEntry[] = [
          { pluginId: "a", moduleUrl: "https://x/a.js", dependencies: [] },
          { pluginId: "b", moduleUrl: "https://x/b.js", dependencies: [{ pluginId: "a", version: "*" }] },
        ];
        const result = await loadPluginModulesInDependencyOrder(entries, { loadModule, signal: controller.signal });
        // "a" (already in flight when abort fired) settles normally; "b" (level 1, not yet started) never starts.
        expect(started).toEqual(["a"]);
        expect(result.handles.map((handle) => handle.pluginId)).toEqual(["a"]);
        expect(result.loadFailures.some((failure) => failure.pluginId === "b")).toBe(true);
      });
    });

  describe("typed-operation completion drain", () => {
    const idle = { tag: "idle" } as const;
    const moreWork = { tag: "moreWork" } as const;

    it("stops at the first idle settle instead of polling forever", async () => {
      const statuses: unknown[] = [moreWork, moreWork, idle, moreWork];
      let polls = 0;
      const outcome = await drainTypedOperationTurns(PLUGIN_OPERATION_DRAIN_BUDGET, () => true, async () => ({ status: statuses[polls++], nextWake: null }), async () => {});
      expect(outcome).toEqual({ polls: 3, stopped: "idle", nextWake: null });
      expect(polls).toBe(3);
      console.info("[DEBUG] typed-operation drain stopped at the idle settle after 3 polls");
    });

    it("never exceeds its poll budget when the actor never goes idle", async () => {
      let polls = 0;
      const outcome = await drainTypedOperationTurns(8, () => true, async () => { polls += 1; return { status: moreWork, nextWake: null }; }, async () => {});
      expect(outcome).toEqual({ polls: 8, stopped: "budget", nextWake: null });
      expect(polls).toBe(8);
    });

    it("stops the moment its instance stops being live, without settling again", async () => {
      let polls = 0;
      const outcome = await drainTypedOperationTurns(16, () => polls < 2, async () => { polls += 1; return { status: moreWork, nextWake: null }; }, async () => {});
      expect(outcome).toEqual({ polls: 2, stopped: "closed", nextWake: null });
      expect(polls).toBe(2);
    });

    it("surfaces the last settle's next-wake so an idle actor can be re-armed", async () => {
      const outcome = await drainTypedOperationTurns(4, () => true, async () => ({ status: idle, nextWake: 42 }), async () => {});
      expect(outcome).toEqual({ polls: 1, stopped: "idle", nextWake: 42 });
    });

    /** ⏱️ Counts every wall-clock primitive a drain could reach for, so "no backoff" is asserted rather
     * than read off the source. `requestAnimationFrame`/`requestIdleCallback` are in the list because
     * neither fires in a hidden tab: evaluation must keep converging when nobody is looking. */
    const countTimerPrimitives = <T,>(run: () => Promise<T>): Promise<{ result: T; timers: number; elapsedMs: number }> => {
      const host = globalThis as unknown as Record<string, unknown>;
      const names = ["setTimeout", "setInterval", "requestAnimationFrame", "requestIdleCallback"] as const;
      const originals = names.map((name) => [name, host[name]] as const);
      let timers = 0;
      for (const [name, original] of originals) {
        if (typeof original !== "function") continue;
        host[name] = (...args: unknown[]) => { timers += 1; return (original as (...rest: unknown[]) => unknown)(...args); };
      }
      const started = Date.now();
      return run().then(
        (result) => { for (const [name, original] of originals) host[name] = original; return { result, timers, elapsedMs: Date.now() - started }; },
        (error) => { for (const [name, original] of originals) host[name] = original; throw error; },
      );
    };

    it("re-polls every more-work answer on a continuation, never on a wall clock", async () => {
      const answers = 64;
      let polls = 0;
      const { result, timers, elapsedMs } = await countTimerPrimitives(async () =>
        drainTypedOperationTurns(answers + 8, () => true, async () => { polls += 1; return { status: polls < answers ? moreWork : idle, nextWake: null }; }, yieldPluginUiContinuation));
      expect(result).toEqual({ polls: answers, stopped: "idle", nextWake: null });
      expect(polls).toBe(answers);
      expect(timers).toBe(0);
      expect(elapsedMs).toBeLessThan(1_000);
      console.info(`[DEBUG] more-work pump: ${answers} answers → ${polls} polls in ${elapsedMs} ms with ${timers} timer calls`);
    });

    it("yields the thread to a contended peer between polls instead of sleeping on it", async () => {
      let polls = 0;
      let peerTurns = 0;
      let peerLive = true;
      const peer = (async () => { while (peerLive) { peerTurns += 1; await yieldPluginUiContinuation(); } })();
      const { result, timers, elapsedMs } = await countTimerPrimitives(async () =>
        drainTypedOperationTurns(48, () => true, async () => { polls += 1; return { status: polls < 32 ? moreWork : idle, nextWake: null }; }, yieldPluginUiContinuation));
      peerLive = false;
      await peer;
      expect(result.stopped).toBe("idle");
      expect(result.polls).toBe(32);
      expect(peerTurns).toBeGreaterThanOrEqual(result.polls);
      expect(timers).toBe(0);
      expect(elapsedMs).toBeLessThan(1_000);
      console.info(`[DEBUG] contended more-work pump: ${result.polls} polls interleaved with ${peerTurns} peer turns in ${elapsedMs} ms, ${timers} timer calls`);
    });
  });

  describe("typed-operation completion delivery", () => {
    it("hands one completion its own effects exactly once and never the invocation's", async () => {
      const instanceId = 4242;
      let push: ((outcome: TurnOutcome) => void) | null = null;
      const broadcast = createTurnOutcomeBroadcast<TurnOutcome>();
      push = (outcome) => broadcast.push(outcome);
      const handle = {
        manifest: async () => encodePackValue({ id: "fixture", apps: [] }),
        createApp: async () => instanceId,
        destroyApp: async () => {},
        takeSegmentedDownloadChunk: async () => undefined,
        enqueue: () => {},
        outcomes: broadcast.stream,
        dispose: async () => {},
      } as unknown as Parameters<typeof adaptPluginHandle>[1]["handle"];
      const adapted = await adaptPluginHandle("fixture", { handle, release: async () => {} });
      await adapted.createApp("app.demo");
      const seen: unknown[] = [];
      const unsubscribe = adapted.subscribeOperationCompletions(instanceId, (completion) => seen.push(completion));
      pendingCompletionEffects.set(instanceId, [{ tag: "notify", val: { message: "done" } }]);
      pendingTurnEffects.set(instanceId, [{ tag: "notify", val: { message: "invocation" } }]);
      push({ instanceId, frames: [encodeAppFrame({ OperationCompleted: { operation: 9, revision: 3, ui_scope: [], history_patch: Array.from(encodePackValue({ cursor: 2, upserts: [] })) } })] });
      for (let tick = 0; tick < 8 && seen.length === 0; tick += 1) await Promise.resolve();
      expect(seen).toHaveLength(1);
      expect(seen[0]).toMatchObject({ instanceId, operation: 9, revision: 3, requestedEffects: [{ notify: { message: "done" } }] });
      expect(pendingCompletionEffects.has(instanceId)).toBe(false);
      expect(pendingTurnEffects.get(instanceId)).toEqual([{ tag: "notify", val: { message: "invocation" } }]);
      unsubscribe();
      pendingTurnEffects.delete(instanceId);
      console.info("[DEBUG] one typed-operation completion reached its subscriber once with its own effects");
    });
  });

  it("keeps reserved command ingress Interactive ahead of catalog Background and stamps a reply when none arrives", async () => {
    const { commandIngressLaneForActionV1, commandIngressNeedsReplyStampV1 } = dependencies;
    const { default: fixture } = await import("../../🧱️elements/🔌️PluginRuntime/🧫️fixtures/command-ingress-lane.json");
    for (const row of fixture.lanes) {
      expect(commandIngressLaneForActionV1(row.actionId), row.actionId).toBe(row.lane);
    }
    for (const row of fixture.stamps) {
      expect(commandIngressNeedsReplyStampV1(row.replySequences, row.seq), row.id).toBe(row.stamp);
    }
    console.log("[DEBUG] Command ingress: reserved-interactive=4 catalog-background=1 empty-stamp=1 matched-skip=1 foreign-stamp=1 missing-seq=1");
  });


  it("packs completion-result.fault the same way the guest decodes it", async () => {
    const { default: fixture } = await import("../../../../🔌️plugin/⚛️reactor/🧫️fixtures/extension-result-fault-pack.json");
    const packed = encodePackValue(fixture.fault);
    expect(() => JSON.parse(new TextDecoder().decode(packed))).toThrow();
    const decoded = decodePackValue(packed) as { code: string; message: string };
    expect(decoded.code).toBe("extension.missing");
    expect(decoded.message).toBe("no such extension");
    console.info("[DEBUG] completion-result.fault pack recovers code and message");
  });


  describe("leftover brushPreview hash-bust", () => {
    it("omits the cached world-body hash when leftover brush hover is a vortex", async () => {
      const { leftoverBrushPreviewWindowHash, leftoverBrushPreviewRefreshScope, leftoverBrushPreviewRefreshReady } = await import("../../🧱️elements/🔌️PluginRuntime/🟦️.tsx");
      expect(leftoverBrushPreviewWindowHash("brush", "seed-left-001:v0", "abc")).toBeUndefined();
      expect(leftoverBrushPreviewWindowHash("select", "seed-left-001:v0", "abc")).toBe("abc");
      expect(leftoverBrushPreviewWindowHash("brush", null, "abc")).toBe("abc");
      expect(leftoverBrushPreviewRefreshScope("brush", "seed-left-001:v0")).toEqual({ kind: "full" });
      expect(leftoverBrushPreviewRefreshScope("select", "seed-left-001:v0")).toBeNull();
      expect(leftoverBrushPreviewRefreshReady("interactionHover", "brush", "seed-left-001:v0")).toBe(false);
      expect(leftoverBrushPreviewRefreshReady("suggestionsTick", "brush", "seed-left-001:v0")).toBe(true);
      expect(leftoverBrushPreviewRefreshReady("interactionHover", "brush", "seed-left-001:v0", '{"targetVortexFullId":"seed-left-001:v0"}')).toBe(true);
    });
  });

  describe("leftover brush guest hover retain", () => {
    it("keeps the leftover vortex id on an armed brush window", async () => {
      const { leftoverBrushRetainGuestHoverV1 } = await import("../../🧱️elements/🌐️World3dHost/🟦️.tsx");
      const leftover = { ids: [], hoveredId: "seed-left-001:v0", hoveredDomain: "vortex", gumballActive: false, gumballAnchorId: null, activeUtility: "brush" };
      expect(leftoverBrushRetainGuestHoverV1("brush", leftover)).toBe("seed-left-001:v0");
      expect(leftoverBrushRetainGuestHoverV1("select", leftover)).toBeUndefined();
      expect(leftoverBrushRetainGuestHoverV1("brush", { ...leftover, hoveredId: null })).toBeUndefined();
    });
  });

}
