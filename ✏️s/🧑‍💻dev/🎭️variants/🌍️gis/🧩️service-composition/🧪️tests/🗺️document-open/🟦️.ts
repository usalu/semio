/** 🗺️ GIS document open: the GIS wasm lease, the viewer's renderer-unavailable state and the cold pair transfer with inference approval Undo. */
import { stubFetch } from "../../../../../../../🧰️framework/🛍️products/💻️os/🧪️tests/🌐️fetch-stub/🟦️.ts";
import { parseGisMapInferenceApprovalReceiptV1, idleGisMapInferencePortStatusV1 } from "../../../../../../🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/💡️inference/🧬️schema/🟦️.ts";
import { documentServiceRequestV1 } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/💡️inference/🔌️service/🚪️io/🟦️.ts";
import { gisMapDocumentServiceDeclarationV1 } from "../../../../../../🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/💡️inference/🔌️client/🟦️.ts";
import { workerLawFakesV1, executionTargetBytes, WORKER_LAW_CAPABILITY_TWO, FakeHubWebSocket, sampleEnvelope } from "../../../../../../../🧰️framework/🛍️products/💻️os/🧪️tests/🧰️worker-law-fakes/🟦️.ts";
import type { TestSource } from "../../../../../../../🧰️framework/🛍️products/💻️os/🧪️tests/🧰️worker-law-fakes/🟦️.ts";
import type { ServiceCompositionTestDependenciesV1, InferenceOperationV1 } from "../🧰️dependencies/🟦️.ts";

type ActorInstanceLifetime = import("../../../../../../../🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/🚪️lifetime/🟦️.ts").ActorInstanceLifetime;
type ArtifactEvent = import("../../../../../../../🧰️framework/🛍️products/💻️os/🟦️.ts").ArtifactEvent;
type ArtifactState = import("../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🟦️.ts").ArtifactState;
type BackboneWorkerResponse = import("../../../../../../../🧰️framework/🛍️products/💻️os/🟦️.ts").BackboneWorkerResponse;
type BrowserActorChildValue = import("../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧵️child/🧬️schema/🟦️.ts").BrowserActorChildValue;
type BrowserActorUiPatchOfferV1 = import("../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🩹️patch-handoff/🟦️.ts").BrowserActorUiPatchOfferV1;
type DocumentBrowserActorChild = import("../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🟦️.ts").DocumentBrowserActorChild;
type MutationEnvelope = import("../../../../../../../🧰️framework/🔨️modules/📡️replication/🟦️.ts").MutationEnvelope;
type ResolvedPluginViewState = import("../../../../../../../🧰️framework/🔨️modules/🛂️manifest/🟦️.ts").ResolvedPluginViewState;
type WireArtifactBootstrap = import("../../../../../../../🧰️framework/🔨️modules/📡️replication/🟦️.ts").WireArtifactBootstrap;
type Vitest = NonNullable<ImportMeta["vitest"]>;

export async function registerGisDocumentOpenTests(vitest: Vitest, dependencies: ServiceCompositionTestDependenciesV1, source: TestSource): Promise<void> {
  const { beforeEach, describe, expect, it, vi } = vitest;
  const { resetWorkerLaw, executionTargetLeaseFixture, executionTargetHarness, executionTargetBodyResponse, exactDocumentBackboneMessage } = workerLawFakesV1(vitest, dependencies, source);
  const { testSeams, requestDocumentSocketAuthority, sameLeaseFieldsV1, decodePackValue, encodePackValue, executionTargetSha256Hex, clearHubSessionCapability, installHubSessionCapability, hubSessionFetch, openArtifact, artifactState, artifacts, documentRuntimeKeyForConfig, handleTsRequest, DocumentExecutionTargetLease, documentExecutionTargetLeaseMintToken, parseDocumentExecutionTargetLeaseFieldsV1, VerifiedColdDocumentPair, reserveDocumentBrowserActorChild, verifiedColdDocumentPairMintToken, parseGisMapInferenceApprovalReceiptV1, executionTargetHex, idleGisMapInferencePortStatusV1, retainInferenceApprovalUndo, reissueInferenceApprovalUndoForRebootstrap, documentRuntimeKeyV1, bindInferenceApprovalUndoToMountedPair, undoInferenceApproval, dropVerifiedColdDocumentPair, closeArtifactRuntime, browserActorChildCapacity, DOCUMENT_EXECUTION_TARGET_STATUS_TEXT_V1, documentExecutionTargetStatusRoleV1, relayMutationsToHub } = dependencies;
  //#region 🔖️OfflineResilienceTests
  describe("backbone-worker offline resilience", () => {

    beforeEach(resetWorkerLaw);

    /** 🚚️ The backing buffer of a view the worker transfers, refusing a shared one.
     * `Uint8Array.buffer` is an `ArrayBufferLike`, and a `SharedArrayBuffer` is not `Transferable`;
     * `instanceof` is what turns that into a checked narrowing rather than an assertion. */
    function transferableBuffer(bytes: Uint8Array): ArrayBuffer {
      const buffer = bytes.buffer;
      if (!(buffer instanceof ArrayBuffer)) throw new Error("worker transfer requires a non-shared buffer");
      return buffer;
    }

    it("browser execution target lease verifies GIS wasm bytes before plan exchange", async () => {
      const fixture = await executionTargetLeaseFixture();
      const component = executionTargetBytes(fixture.componentHex);
      const descriptor = executionTargetBytes(fixture.descriptorHex);
      const plan = { ...structuredClone(fixture.plan), expiresAtUnixMs: Date.now() + 30_000 };
      const grant = { ...fixture.socketGrant, expiresAtMs: Date.now() + 25_000 };
      const harness = await executionTargetHarness(fixture, (url) => {
        if (url.endsWith("/open-plan")) return Response.json(plan);
        if (url.endsWith("/execution-target/manifest")) return Response.json(fixture.manifest);
        if (url.endsWith("/execution-target/component")) return executionTargetBodyResponse(component);
        if (url.endsWith("/execution-target/descriptor")) return executionTargetBodyResponse(descriptor);
        return Response.json(grant);
      });
      try {
        expect(fixture.plan.surface.rendererTarget).toBe("wasm");
        expect(fixture.manifest.grant.write).toBe(false);
        const authority = await requestDocumentSocketAuthority(harness.state, harness.binding);
        expect(authority.surfaceId).toBe(fixture.manifest.surface.surfaceId);
        expect(harness.requests.map(({ url }) => url)).toEqual([`/_semio/hub${fixture.expected.openPlanPath}`, ...fixture.expected.assetPaths.map((path) => `/_semio/hub${path}`), `/_semio/hub${fixture.expected.socketGrantPath}`]);
        for (const index of [1, 2, 3]) {
          expect(harness.requests[index]!.method).toBe("POST");
          expect(JSON.parse(harness.requests[index]!.body)).toEqual(fixture.intent);
          expect(harness.requests[index]!.body).not.toContain(plan.receipt);
        }
        expect(JSON.parse(harness.requests.at(-1)!.body).planReceipt).toBe(plan.receipt);
        expect(harness.state.executionTargetLease).not.toBeNull();
        expect(harness.state.executionTargetLease!.live).toBe(true);
        expect(sameLeaseFieldsV1(harness.state.executionTargetLease!.fields(), fixture.manifest)).toBe(true);
        const observed = harness.statuses.map(({ code, progress }) => `${code}:${progress?.stage ?? "-"}`);
        expect(observed.at(-1)).toBe(`${fixture.expected.rendererState}:-`);
        expect(new Set(harness.statuses.flatMap(({ progress }) => (progress ? [progress.stage] : [])))).toEqual(new Set(fixture.expected.progressStages));
        const payload = JSON.stringify(harness.statuses);
        for (const fragment of fixture.expected.forbiddenStatusFragments) expect(payload).not.toContain(fragment);
      } finally {
        harness.release();
      }
    });

    it("browser document actor transfers one verified cold pair only after lifecycle ACK and exact page receipts", async () => {
      const fixture = await executionTargetLeaseFixture();
      const corpus = JSON.parse(await (await import("node:fs/promises")).readFile(new URL("./🔨️modules/🔌️plugin/⚛️reactor/📥️cold-pair/🧪️tests/🧫️fixtures/🔣️.json", source.url), "utf8"));
      const actionFixture = JSON.parse(await (await import("node:fs/promises")).readFile(new URL("./🔨️modules/🔌️plugin/🌐️browser-bundle/🎯️action-handoff/🧪️tests/🧫️fixtures/🔣️.json", source.url), "utf8"));
      const { applyPatch } = await import("fast-json-patch");
      const rendererOracle = applyPatch({ revision: 0, nodeKind: null as string | null }, [
        { op: "replace", path: "/revision", value: corpus.browserRender.revision },
        { op: "replace", path: "/nodeKind", value: corpus.browserRender.nodeKind },
      ]).newDocument;
      const renderEvents: string[] = [];
      let renderWakes = 0;
      let activeLifetime: ActorInstanceLifetime | null = null;
      const exact = corpus.exact as {
        packLength: number;
        sprLength: number;
        pageCount: number;
        packPattern: { multiplier: number; addend: number };
        sprPattern: { multiplier: number; addend: number };
        packSha256: string;
        sprSha256: string;
        aggregateSha256: string;
      };
      const pattern = (length: number, multiplier: number, addend: number): Uint8Array => Uint8Array.from({ length }, (_unused, index) => (index * multiplier + addend) & 255);
      const pack = pattern(exact.packLength, exact.packPattern.multiplier, exact.packPattern.addend),
        spr = pattern(exact.sprLength, exact.sprPattern.multiplier, exact.sprPattern.addend),
        publishedPack = Uint8Array.from(pack),
        publishedSpr = Uint8Array.from(spr),
        actorBytes = new TextEncoder().encode("abc"),
        sourceDescriptor = executionTargetBytes(fixture.descriptorHex),
        guest = decodePackValue(sourceDescriptor) as Record<string, any>,
        originalWorker = globalThis.Worker,
        originalFetch = globalThis.fetch;
      const descriptorApp = guest.manifest.apps.find((app: Record<string, unknown>) => app.id === fixture.manifest.surface.appId);
      if (descriptorApp === undefined) throw new Error("direct browser actor fixture: descriptor app absent");
      descriptorApp.role = "editor";
      const secondWindow = { key: "gis2d-inset", bodyKey: "gis2d.play.inset" };
      descriptorApp.windowKinds = [...descriptorApp.windowKinds, { ...descriptorApp.windowKinds[0], id: secondWindow.key, bodyKey: secondWindow.bodyKey }];
      const panelSurfaces = [{ key: "framework.panel.inspection", bodyKey: "gis2d.play.inspection" }, { key: "framework.panel.history", bodyKey: "framework.body.history" }];
      descriptorApp.panelTabs = [
        { kind: { kind: "detailsCategory" }, label: "Details", group: "details", children: [{ kind: { kind: "app", id: panelSurfaces[0]!.key }, label: "Inspection", group: "details", bodyKey: panelSurfaces[0]!.bodyKey, children: [] }] },
        { kind: { kind: "app", id: panelSurfaces[1]!.key }, label: "History", group: "details", bodyKey: panelSurfaces[1]!.bodyKey, children: [] },
      ];
      const panelVisible = (view: ResolvedPluginViewState) => panelSurfaces.map((panel) => ({ tag: "surface-visible", val: { surface: { instance: 0, surface: panel.key }, bodyKey: panel.bodyKey, viewState: encodePackValue(panelViewContext(view)) } }));
      const { panelViewContext, sectionViewContext, windowViewContext } = await import("../../../../../../../🧰️framework/🔨️modules/🛂️manifest/🟦️.ts");
      const { BROWSER_ACTOR_VISIBLE_SURFACES_V1 } = await import("../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🪟️visible-surfaces/🟦️.ts");
      const sectionVisible = (view: ResolvedPluginViewState, mount: boolean) => BROWSER_ACTOR_VISIBLE_SURFACES_V1.sections.filter((section) => mount || !section.static).map((section) => ({ tag: "surface-visible", val: { surface: { instance: 0, surface: section.bodyKey }, bodyKey: section.bodyKey, viewState: encodePackValue(sectionViewContext(view)) } }));
      const secondWindowVisible = (view: ResolvedPluginViewState) => ({ tag: "surface-visible", val: { surface: { instance: 0, surface: secondWindow.key }, bodyKey: secondWindow.bodyKey, viewState: encodePackValue(windowViewContext(view, secondWindow.key)!) } });
      const descriptor = encodePackValue(guest),
        descriptorByteSha256 = await executionTargetSha256Hex(descriptor);
      const { UiDocumentStore } = await import("../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📃️UiDocumentStore/🟦️.tsx");
      const { decodeDocumentBackboneControlV1, encodeDocumentBackboneControlV1 } = await import("../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📡️backbone/🔗️binding/🟦️.ts");
      const { decodeAppCommand, encodeAppFrame } = await import("../../../../../../../🧰️framework/🛍️products/💻️os/🟦️.ts");
      const { readActorBytePage } = await import("../../../../../../../🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/📃️page/🟦️.ts");
      const { createBrowserActorAppCommandRequestV1 } = await import("../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🎯️action-handoff/🎛️command/🟦️.ts");
      const { createBrowserActorUiIntentRequestV1 } = await import("../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🎯️action-handoff/🧭️intent/🟦️.ts");
      const { decodeBrowserActorHostEffectsV1 } = await import("../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🎯️action-handoff/📤️publication/🟦️.ts");
      const windowKindId = fixture.manifest.surface.windowKindId;
      const hostFixture = JSON.parse(await (await import("node:fs/promises")).readFile(new URL("./🔨️modules/🔌️plugin/🌐️browser-bundle/🪟️view-context/🧪️tests/🧫️fixtures/🪟️host-opening-context/🔣️.json", source.url), "utf8"));
      const hostView = { ...hostFixture.valid.viewState, windowId: windowKindId, activeWindowKindId: windowKindId, activeUtilityId: "pan", activeUtilityByWindowId: { [windowKindId]: "pan" }, windowInstances: [{ id: windowKindId, windowKindId }, { id: secondWindow.key, windowKindId: secondWindow.key }] };
      const bodyKey = leaseBodyKey(guest);
      function leaseBodyKey(descriptor: Record<string, any>): string {
        return descriptor.manifest.apps.find((app: Record<string, any>) => app.id === fixture.manifest.surface.appId).windowKinds.find((window: Record<string, any>) => window.id === windowKindId).bodyKey;
      }
      const visibleViews: Record<string, unknown>[] = [];
      let panelRefreshes = 0;
      const received = new Uint8Array(pack.byteLength + spr.byteLength),
        pageIndexes: number[] = [],
        statuses: Extract<BackboneWorkerResponse, { kind: "execution-target-status" }>[] = [],
        historyStatuses: Extract<BackboneWorkerResponse, { kind: "inference-history-status" }>[] = [],
        workerResponses: BackboneWorkerResponse[] = [],
        patchOffers: BrowserActorUiPatchOfferV1[] = [],
        documentControls: string[] = [],
        lifecycleClose: string[] = [],
        actionSequences: bigint[] = [],
        commandSequences: number[] = [],
        commandInvocations: Record<string, unknown>[] = [],
        commandViews: Record<string, unknown>[] = [],
        backboneIngress: Uint8Array[] = [],
        uiStore = new UiDocumentStore(windowKindId);
      let stagedCommandSequence: number | null = null,
        stagedColdPairIngress: Record<string, any> = { tag: "idle" },
        lifecycleAcknowledged = false,
        patchAcknowledged = false,
        patchRejected = false,
        actionPatchAcknowledged = false,
        heldActionPatch: BrowserActorUiPatchOfferV1 | null = null,
        loaded = 0,
        described = 0,
        terminated = 0;
      class ColdPairWorker {
        port!: MessagePort;
        binding!: { schema: string; nonce: string; generation: string };
        postMessage(init: Record<string, any>): void {
          this.port = init.port;
          this.binding = { schema: init.schema, nonce: init.nonce, generation: init.generation };
          this.port.onmessage = (event) => {
            const message = event.data;
            if (message.kind === "load") {
              loaded++;
              expect(new Uint8Array(message.bytes)).toEqual(actorBytes);
              const byteLength = message.bytes.byteLength;
              new Uint8Array(message.bytes).fill(0);
              this.port.postMessage({ ...this.binding, kind: "loaded", byteLength, sha256: message.sha256 });
              return;
            }
            if (message.kind !== "invoke") return;
            if (message.path[0] === "describe") {
              described++;
              const bytes = encodePackValue(guest);
              this.port.postMessage({ ...this.binding, kind: "result", sequence: message.sequence, value: bytes }, [bytes.buffer]);
              this.port.postMessage({ ...this.binding, kind: "transferred", sequence: message.sequence, detached: 1 });
              return;
            }
            if (message.path[1] === "stageCommandPage") {
              const cursor = message.args[0] as Record<string, any>;
              expect(cursor).toMatchObject({ owner: 0n, generation: reservation!.generation, commandIndex: 0, commandCount: 1, instance: 0, pageIndex: 0, pageCount: 1 });
              const staged = decodeAppCommand(message.args[1] as Uint8Array);
              if (!("Command" in staged)) throw new Error("expected browser actor AppCommand::Command");
              stagedCommandSequence = staged.Command.seq;
              expect(cursor.seq).toBe(BigInt(stagedCommandSequence));
              commandSequences.push(stagedCommandSequence);
              commandInvocations.push(decodePackValue(Uint8Array.from(staged.Command.command)) as Record<string, unknown>);
              commandViews.push(decodePackValue(Uint8Array.from(staged.Command.view_state)) as Record<string, unknown>);
              this.port.postMessage({ ...this.binding, kind: "result", sequence: message.sequence, value: null });
              this.port.postMessage({ ...this.binding, kind: "transferred", sequence: message.sequence, detached: 0 });
              return;
            }
            if (message.path[1] === "stageColdPairPage") {
              const staged = message.args[0] as Record<string, any>;
              expect(lifecycleAcknowledged).toBe(true);
              const header = staged.header as Record<string, any>,
                pageIndex = staged.pageIndex as number,
                bytes = staged.bytes as Uint8Array,
                start = pageIndex * corpus.limits.pageBytes;
              expect(pageIndexes).toHaveLength(pageIndex);
              expect(bytes.byteLength).toBe(Math.min(corpus.limits.pageBytes, received.byteLength - start));
              received.set(bytes, start);
              bytes.fill(0);
              pageIndexes.push(pageIndex);
              activeLifetime = header.lifetime;
              stagedColdPairIngress =
                pageIndex + 1 === header.pageCount
                  ? { tag: "applied", val: { lifetime: header.lifetime, transferGeneration: header.transferGeneration, baselineFrontier: header.baselineFrontier, aggregateSha256: header.aggregateSha256 } }
                  : { tag: "page-accepted", val: { lifetime: header.lifetime, transferGeneration: header.transferGeneration, pageIndex, pageCount: header.pageCount } };
              this.port.postMessage({ ...this.binding, kind: "result", sequence: message.sequence, value: null });
              this.port.postMessage({ ...this.binding, kind: "transferred", sequence: message.sequence, detached: 0 });
              return;
            }
            expect(message.path).toEqual(["reactor", "poll"]);
            const events = message.args[0] as Record<string, any>[];
            const open = events[0]?.tag === "instance-open" ? events[0].val : null;
            const close = events[0]?.tag === "instance-close" ? events[0].val : null;
            const lifecycleAck = events[0]?.tag === "instance-lifecycle-ack" ? events[0].val : null;
            const acknowledged = lifecycleAck?.tag === "captured";
            const patchAck = events[0]?.tag === "patch-ack" ? events[0].val : null;
            const patchRejection = events[0]?.tag === "patch-rejected" ? events[0].val : null;
            const panelRefresh = events[0]?.tag === "surface-visible" && events[0].val.surface.surface !== windowKindId;
            if (panelRefresh) {
              expect(events).toEqual([...panelVisible(state.browserActorViewState!), ...sectionVisible(state.browserActorViewState!, false)]);
              panelRefreshes++;
            }
            const visible = events[0]?.tag === "surface-visible" && !panelRefresh ? events[0].val : null;
            const wake = events[0]?.tag === "wake";
            const shellMessage = events[0]?.tag === "message" && events[0].val?.source?.tag === "shell" ? events[0].val : null;
            const backboneMessage = events[0]?.tag === "message" && events[0].val?.source?.tag === "backbone" ? events[0].val : null;
            const uiIntent = events[0]?.tag === "ui-intent" ? events[0].val : null;
            if (uiIntent !== null) {
              expect(uiIntent.instance).toBe(0);
              const decoded = decodePackValue(uiIntent.intent) as Record<string, any>;
              expect(decoded.surface).toBe(`0:${windowKindId}`);
              actionSequences.push(decoded.seq.value);
            }
            const commandSequence = stagedCommandSequence;
            stagedCommandSequence = null;
            if (backboneMessage !== null) backboneIngress.push(Uint8Array.from(backboneMessage.payload));
            if (visible) {
              expect(lifecycleAcknowledged).toBe(true);
              expect(pageIndexes).toHaveLength(exact.pageCount);
              expect(visible.surface).toEqual({ instance: 0, surface: windowKindId });
              expect(visible.bodyKey).toBe(bodyKey);
              const visibleView = decodePackValue(visible.viewState) as Record<string, unknown>;
              expect(visibleView).toEqual(state.browserActorViewState);
              visibleViews.push(visibleView);
              expect(events.slice(1)).toEqual([secondWindowVisible(state.browserActorViewState!), ...panelVisible(state.browserActorViewState!), ...sectionVisible(state.browserActorViewState!, visibleViews.length === 1)]);
            }
            if (wake) {
              expect(renderEvents[0]).toBe("surface-visible");
              renderWakes++;
            }
            if (visible || wake || patchAck || patchRejection) renderEvents.push(events[0].tag);
            const coldPairIngress = stagedColdPairIngress;
            stagedColdPairIngress = { tag: "idle" };
            if (acknowledged) lifecycleAcknowledged = true;
            if (patchAck) {
              expect(patchAck.surface).toEqual({ instance: 0, surface: windowKindId });
              if (patchAck.receipt.patchSequence === 1n) {
                expect(patchAck.revision).toBe(1n);
                patchAcknowledged = true;
              } else {
                expect(patchAck.receipt.patchSequence).toBe(3n);
                expect(patchAck.revision).toBe(2n);
                actionPatchAcknowledged = true;
              }
            }
            if (patchRejection) {
              expect(patchRejection.surface).toEqual({ instance: 0, surface: windowKindId });
              expect(patchRejection.revision).toBe(1n);
              expect(patchRejection.receipt.patchSequence).toBe(2n);
              expect(patchRejection.reason).toBe("revisionMismatch");
              patchRejected = true;
            }
            if (close) lifecycleClose.push("close");
            if (lifecycleAck?.tag === "accepted") lifecycleClose.push("accepted-ack");
            if (lifecycleAck?.tag === "retired") lifecycleClose.push("retired-ack");
            const lifecycleReceipt = open
              ? { tag: "captured", val: { lifetime: { activationGeneration: open.activationGeneration, instanceId: open.instance, guestLifetime: 1n }, requestSequence: open.requestSequence } }
              : close
                ? { tag: "accepted", val: { lifetime: close.lifetime, requestSequence: close.requestSequence, closeGeneration: 1n } }
                : lifecycleAck?.tag === "accepted"
                  ? { tag: "retired", val: { lifetime: lifecycleAck.val.lifetime, requestSequence: lifecycleAck.val.requestSequence, closeGeneration: lifecycleAck.val.closeGeneration } }
                  : null;
            const initialScene = wake && renderWakes === corpus.browserRender.lazyWakeTurns;
            const lifetime = activeLifetime;
            const node = {
              id: 0,
              key: "map-root",
              component: { type: "surface", kind: "tiled-map", docSchema: "tiled-map@1", doc: { bytes: [1, 3, 5] }, bindings: [] },
              layout: { kind: "leaf", width: "fill", height: "fill" },
              style: { variant: "plain", size: "md", density: "standard", tone: "neutral", emphasis: "regular" },
              activity: "idle",
              disabled: false,
              transition: null,
              accessibility: { label: null, description: null, live: "off", shortcut: null, hidden: false },
              bindings: [],
              menu: null,
              children: [],
            };
            const emitsPatch = initialScene || uiIntent !== null || patchAck?.receipt.patchSequence === 1n;
            const nodeBytes = initialScene ? encodePackValue(node) : null;
            const patchReceipt = emitsPatch ? { lifetime: lifetime!, patchSequence: initialScene ? 1n : uiIntent !== null ? 3n : 2n } : null;
            const effects: Record<string, unknown>[] = [];
            let controlReceipt: Uint8Array | null = null;
            if (shellMessage !== null) {
              const command = decodeDocumentBackboneControlV1(shellMessage.payload);
              documentControls.push(command.operation);
              controlReceipt = encodeDocumentBackboneControlV1({ ...command, schema: "semio.plugin.document-backbone-binding-receipt.v1", operation: command.operation === "bind" ? "bound" : "retired" });
              effects.push({ tag: "send-message", val: { target: { tag: "shell", val: 0 }, payload: controlReceipt } });
            }
            let commandBackbone: Uint8Array | null = null;
            const commandMutation = commandSequence === 3
              ? (() => {
                  const mutationId = "direct-command-mutation",
                    invocationId = "direct-command:0:3",
                    forward = encodePackValue({ sequenceNumber: 3 }),
                    inverse = encodePackValue(null),
                    inverseVector = Uint8Array.of(1, inverse.byteLength, ...inverse),
                    mutation = {
                      id: mutationId,
                      document: "0",
                      baseVersion: 0,
                      invocationId,
                      diff: { schema: `${fields.artifact.schema}.operation`, payload: Array.from(forward) },
                      inverse: { targetMutation: mutationId, inverseDiff: { schema: `${fields.artifact.schema}.operation.inverse`, payload: Array.from(inverseVector) }, baseVersion: 0, dependencies: [], undoPolicy: "ExactBaseOnly" },
                      dependencies: [],
                      author: "actor-1",
                      timestamp: { actor: 7, physical_ms: 8, logical: 9 },
                    },
                    envelope: MutationEnvelope = {
                      id: mutationId,
                      actor: "actor-1",
                      document: documentId,
                      schemaVersion: fields.artifact.schema,
                      deps: [],
                      payloadHash: "unused",
                      diff: { schemaId: fields.artifact.schema, payload: { sequenceNumber: 3 } },
                      inverse: { targetOperation: mutationId, inverseDiff: { schemaId: fields.artifact.schema, payload: null }, baseVersion: 0, dependencies: [], undoPolicy: "exactBaseOnly" },
                    };
                  commandBackbone = exactDocumentBackboneMessage(envelope, forward, { actor: 7n, physical_ms: 8n, logical: 9n });
                  return { mutations: Array.from(encodePackValue([mutation])), inverseGroup: Array.from(encodePackValue({ invocationId, mutations: [mutationId], inverseMutations: [mutation.inverse] })) };
                })()
              : null;
            const actionPublication = uiIntent !== null
              ? encodeAppFrame(actionFixture.publication.emit)
              : commandSequence !== null
                ? encodeAppFrame({ Invocation: { in_reply_to: commandSequence, output: [], diagnostics: [], ui_scope: [], history_patch: [], messages: [], mutations: commandMutation?.mutations ?? [], inverse_group: commandMutation?.inverseGroup ?? [] } })
                : null;
            if (actionPublication !== null) {
              effects.push({ tag: "send-message", val: { target: { tag: "shell", val: 0 }, payload: actionPublication } });
              effects.push(actionFixture.publication.hostEffect);
            }
            if (commandBackbone !== null) effects.push({ tag: "send-message", val: { target: { tag: "backbone", val: `actor://${state.runtimeKey}` }, payload: commandBackbone } });
            const resultTransfers: ArrayBuffer[] = initialScene ? [transferableBuffer(nodeBytes!)] : [];
            if (controlReceipt !== null) resultTransfers.push(transferableBuffer(controlReceipt));
            if (actionPublication !== null) resultTransfers.push(transferableBuffer(actionPublication));
            if (commandBackbone !== null) resultTransfers.push(transferableBuffer(commandBackbone));
            this.port.postMessage(
              {
                ...this.binding,
                kind: "result",
                sequence: message.sequence,
                value: {
                  uiPatches: initialScene
                    ? [
                        {
                          surface: { instance: 0, surface: windowKindId },
                          baseRevision: 0n,
                          revision: 1n,
                          ops: [
                            { tag: "upsert", val: { node: nodeBytes } },
                            { tag: "set-root", val: 0n },
                          ],
                        },
                      ]
                    : uiIntent !== null
                      ? [{ surface: { instance: 0, surface: windowKindId }, baseRevision: 1n, revision: 2n, ops: [] }]
                    : patchAck?.receipt.patchSequence === 1n
                      ? [{ surface: { instance: 0, surface: windowKindId }, baseRevision: 0n, revision: 2n, ops: [] }]
                      : [],
                  effects,
                  presence: [],
                  nextWake: null,
                  status: { tag: (visible && visibleViews.length === 1) || (wake && !initialScene) ? "more-work" : "idle" },
                  fuelUsed: 1n,
                  commandIngress: { kind: commandSequence === null ? 0 : 4, cursor: { owner: 0n, generation: 0n, commandIndex: 0, commandCount: 0, instance: 0, seq: 0n, kind: 0, pageIndex: 0, pageCount: 0, itemCount: 0, metadata: 0 }, fault: [] },
                  coldPairIngress,
                  lifecycleReceipt,
                  uiPatchReceipt: patchReceipt === null ? { tag: "none" } : { tag: "some", val: patchReceipt },
                },
              },
              resultTransfers,
            );
            this.port.postMessage({ ...this.binding, kind: "transferred", sequence: message.sequence, detached: resultTransfers.length });
          };
          this.port.start();
          this.port.postMessage({ ...this.binding, kind: "ready" });
        }
        terminate(): void {
          terminated++;
          this.port?.close();
        }
      }
      clearHubSessionCapability();
      expect(installHubSessionCapability(WORKER_LAW_CAPABILITY_TWO)).toBe(true);
      const authorityFixture = JSON.parse(await (await import("node:fs/promises")).readFile(new URL("./🔨️modules/📇️directory/🧪️tests/🧫️fixtures/🪪️session-authority-v1/🔣️.json", source.url), "utf8"));
      const authority = authorityFixture.rows.find((row: { accepted: boolean }) => row.accepted).value;
      const authorityBody = JSON.stringify(authority);
      globalThis.fetch = stubFetch(async () => new Response(authorityBody, { status: 200, headers: { "content-length": String(new TextEncoder().encode(authorityBody).byteLength) } }));
      await hubSessionFetch("/_semio/hub/auth/sessions/me", { method: "GET" }, { timeoutMs: 1000, accept: testSeams.acceptBrowserSessionAuthority });
      const sessionFence = testSeams.captureBrowserSessionOperationFence();
      if (sessionFence === null) throw new Error("browser session operation fence missing after an accepted authority");
      globalThis.fetch = stubFetch(async (input) => {
        expect(String(input).endsWith("/execution-target/browser-actor")).toBe(true);
        return executionTargetBodyResponse(actorBytes);
      });
      const documentId = "cold-browser-pair";
      openArtifact({ documentId, schema: fixture.manifest.artifact.schema, bindings: [], actor: "untrusted-ui-actor" });
      const state = artifactState(documentId)!;
      artifacts.delete(state.runtimeKey);
      const binding = { kind: "hub", dataClass: "persistedShared", baseUrl: fixture.hubOrigin, spaceId: "cold-browser-space", requestedSurfaceId: fixture.manifest.surface.surfaceId } as const;
      state.config = { ...state.config, documentId, schema: fixture.manifest.artifact.schema, bindings: [binding] };
      state.runtimeKey = documentRuntimeKeyForConfig(state.config);
      artifacts.set(state.runtimeKey, state);
      const hostRequest = { kind: "browser-actor-view-state", clientInstanceId: state.openClientInstanceId, scope: { spaceId: binding.spaceId, documentId }, viewState: hostView } as const;
      handleTsRequest({ ...hostRequest, clientInstanceId: "00000000-0000-4000-8000-000000000000" });
      expect(state.browserActorViewState).toBeNull();
      handleTsRequest(hostRequest);
      expect(state.browserActorViewState).toEqual(hostView);
      const fields = structuredClone(fixture.manifest);
      fields.scope = { spaceId: binding.spaceId, documentId };
      fields.grant = { ...fields.grant, write: true };
      fields.surface = { ...fields.surface, role: "editor" };
      fields.package = { ...fields.package, descriptorByteSha256, componentReceipt: { ...fields.package.componentReceipt, semanticDescriptorSha256: descriptorByteSha256 } };
      fields.descriptor = { sha256: descriptorByteSha256, byteLength: descriptor.byteLength };
      // 🧭️ Only the closed-browser-actor variant of the lease carries a source descriptor digest; the
      // `{ kind: "none" }` variant has no such field to rewrite.
      if (fields.browserActor.kind !== "none") fields.browserActor = { ...fields.browserActor, sourceDescriptorByteSha256: descriptorByteSha256 };
      fields.checkpoint = {
        ...fields.checkpoint!,
        aggregateSha256: exact.aggregateSha256,
        baselineFrontier: { ...fields.checkpoint!.baselineFrontier, documentId },
      };
      const lease = new DocumentExecutionTargetLease(documentExecutionTargetLeaseMintToken, parseDocumentExecutionTargetLeaseFieldsV1(fields), fixture.hubOrigin, executionTargetBytes(fixture.componentHex), descriptor);
      state.executionTargetLease = lease;
      state.currentPack = publishedPack;
      state.currentSpr = publishedSpr;
      state.frontier = {
        document_id: documentId,
        head_edit_ordinal: fields.checkpoint.baselineFrontier.headEditOrdinal,
        head_edit_id: fields.checkpoint.baselineFrontier.headEditId,
        last_commit_seq: fields.checkpoint.baselineFrontier.lastCommitSeq,
        chain_hash: fields.checkpoint.baselineFrontier.chainHash,
      };
      const intent = { ...structuredClone(fixture.intent), scope: { spaceId: binding.spaceId, documentId } },
        actorFixtureExpiresAtMs = Date.now() + 600_000,
        receipt = { ...structuredClone(fixture.socketGrant), expiresAtMs: actorFixtureExpiresAtMs };
      lease.admitBrowserActor(documentExecutionTargetLeaseMintToken, receipt, actorFixtureExpiresAtMs + 5_000, { binding, intent, assertCurrent() {} });
      state.actor = receipt.actorId;
      state.hubActorReady = true;
      state.pendingSocketActorId = null;
      const socket = new FakeHubWebSocket("ws://hub.test/cold");
      state.socket = socket as unknown as WebSocket;
      socket.open();
      testSeams.executionTargetStatusObserver = (status) => statuses.push(status);
      testSeams.workerPostTestSink = (message) => {
        workerResponses.push(message);
        if (message.kind === "inference-history-status") {
          historyStatuses.push(message);
          return;
        }
        if (message.kind === "browser-actor-ui-mounted") {
          expect(patchAcknowledged).toBe(true);
          return;
        }
        if (message.kind !== "browser-actor-ui-patch") return;
        patchOffers.push(message);
        expect(message.patches).toHaveLength(1);
        const offered = message.patches[0]!;
        if (offered.baseRevision === 1 && offered.revision === 2) {
          heldActionPatch = message;
          return;
        }
        const before = uiStore.getState();
        const applied = uiStore.applyPatch(offered);
        if (applied.ok) {
          handleTsRequest({
            kind: "browser-actor-ui-patch-result",
            clientInstanceId: message.clientInstanceId,
            scope: message.scope,
            verifiedSurfaceId: message.verifiedSurfaceId,
            activationGeneration: message.activationGeneration,
            instanceId: message.instanceId,
            receipt: message.receipt,
            verdicts: [{ surface: offered.surface, outcome: "acknowledged", revision: uiStore.getRevisionSnapshot() }],
          });
          return;
        }
        expect(uiStore.getState()).toBe(before);
        handleTsRequest({
          kind: "browser-actor-ui-patch-result",
          clientInstanceId: message.clientInstanceId,
          scope: message.scope,
          verifiedSurfaceId: message.verifiedSurfaceId,
          activationGeneration: message.activationGeneration,
          instanceId: message.instanceId,
          receipt: message.receipt,
          verdicts: [{ surface: offered.surface, outcome: "rejected", revision: uiStore.getRevisionSnapshot(), reason: applied.rejection.type }],
        });
      };
      (globalThis as unknown as { Worker: unknown }).Worker = ColdPairWorker;
      let owner: InstanceType<typeof VerifiedColdDocumentPair> | null = null;
      let reservation: Awaited<ReturnType<typeof reserveDocumentBrowserActorChild>> = null;
      try {
        reservation = await reserveDocumentBrowserActorChild(state);
        expect(reservation).not.toBeNull();
        await reservation!.activate(state.socket);
        const bootstrap: WireArtifactBootstrap = {
          format_version: 1,
          descriptor_hash: Array.from(executionTargetBytes(fields.descriptorDigestV1)),
          artifact_schema: fields.artifact.schema,
          artifact_kind: fields.artifact.kind,
          pack_schema_hash: Array.from(executionTargetBytes(fields.artifact.packSchemaHash)),
          baseline_frontier: state.frontier,
          pack_hash: Array.from(executionTargetBytes(exact.packSha256)),
          spr_hash: Array.from(executionTargetBytes(exact.sprSha256)),
          pack_length: pack.byteLength,
          spr_length: spr.byteLength,
          chunk_count: 2,
          aggregate_hash: Array.from(executionTargetBytes(exact.aggregateSha256)),
          required_tail_frontier: state.frontier,
          inline: null,
        };
        owner = new VerifiedColdDocumentPair(verifiedColdDocumentPairMintToken, state, lease, bootstrap, { pack, spr }, { pack: publishedPack, spr: publishedSpr });
        state.verifiedColdPair = owner;
        const approvalReceipt = parseGisMapInferenceApprovalReceiptV1({
          schema: "semio.hub.inference-approval-receipt/v1",
          jobId: "1".repeat(32),
          mutationId: "2".repeat(32),
          commandHash: "3".repeat(64),
          proposalHash: "4".repeat(64),
          applied: true,
          undo: {
            targetId: "5".repeat(32),
            expectedCurrent: {
              documentId,
              headEditOrdinal: fields.checkpoint.baselineFrontier.headEditOrdinal,
              headEditId: fields.checkpoint.baselineFrontier.headEditId,
              lastCommitSeq: fields.checkpoint.baselineFrontier.lastCommitSeq,
              chainSha256: executionTargetHex(Uint8Array.from(fields.checkpoint.baselineFrontier.chainHash)),
            },
          },
        });
        const operation: InferenceOperationV1 = {
          transport: { async call(action, payload, options) {
            const request = documentServiceRequestV1(gisMapDocumentServiceDeclarationV1(), { spaceId: binding.spaceId, documentId }, action, payload);
            return await hubSessionFetch(request.path, { method: request.method, ...(request.body === undefined ? {} : { headers: { "content-type": "application/json" }, body: request.body }) }, options);
          } },
          operationEpoch: 1,
          scope: { spaceId: binding.spaceId, documentId },
          abort: new AbortController(),
          sessionEpoch: testSeams.directorySessionEpoch,
          sessionFence,
          clientInstanceId: state.openClientInstanceId,
          leaseFields: lease.fields(),
          request: null,
          closeRequested: false,
          reconcileRequired: false,
          status: idleGisMapInferencePortStatusV1(),
          turns: 0,
          pollTimer: null,
          inFlight: false,
          cancelSent: false,
          closed: false,
        };
        retainInferenceApprovalUndo(operation, approvalReceipt);
        expect(testSeams.inferenceApprovalUndoOwner).toMatchObject({ phase: "awaiting-mount", clientInstanceId: state.openClientInstanceId });
        expect(historyStatuses).toEqual([]);
        expect(owner.pageCount).toBe(exact.pageCount);
        await reservation!.installColdPair(owner);
        expect(testSeams.inferenceApprovalUndoOwner).toMatchObject({ phase: "available", clientInstanceId: state.openClientInstanceId });
        expect(historyStatuses.at(-1)?.status).toEqual({ phase: "available", canUndo: true, code: null });
        expect(renderEvents).toEqual(corpus.browserRender.events);
        await reservation!.installColdPair(owner);
        expect(renderEvents).toEqual(corpus.browserRender.events);
        expect(pageIndexes).toEqual(Array.from({ length: exact.pageCount }, (_unused, index) => index));
        expect(Buffer.from(received.subarray(0, pack.byteLength)).equals(Buffer.from(pack))).toBe(true);
        expect(Buffer.from(received.subarray(pack.byteLength)).equals(Buffer.from(spr))).toBe(true);
        expect(statuses.filter((status) => status.code === "renderer-unavailable")).toHaveLength(0);
        expect(patchOffers).toHaveLength(2);
        expect(patchAcknowledged).toBe(true);
        expect(patchRejected).toBe(true);
        expect(documentControls).toEqual(["bind"]);
        const mounted = workerResponses.find((message) => message.kind === "browser-actor-ui-mounted") as Extract<BackboneWorkerResponse, { kind: "browser-actor-ui-mounted" }>;
        expect(mounted).toMatchObject({
          scope: fields.scope,
          activeCheckpointId: fields.checkpoint.checkpointId,
          descriptorDigestV1: fields.checkpoint.descriptorDigestV1,
          frontier: fields.checkpoint.baselineFrontier,
          uiRevision: 1,
        });
        expect(uiStore.getRevisionSnapshot()).toBe(rendererOracle.revision);
        expect(uiStore.getState().nodes.get(uiStore.getState().root!)?.component).toMatchObject({ type: "surface", kind: rendererOracle.nodeKind });

        const beforeConfig = JSON.stringify(state.config);
        const beforeFrontier = JSON.stringify(state.frontier);
        handleTsRequest({ ...hostRequest, viewState: { ...hostView, locale: "en", terminology: "native" } });
        await vi.waitFor(() => expect(visibleViews.at(-1)?.locale).toBe("en"));
        expect(visibleViews.at(-1)?.terminology).toBe("native");
        expect(JSON.stringify(state.config)).toBe(beforeConfig);
        expect(JSON.stringify(state.frontier)).toBe(beforeFrontier);
        await vi.waitFor(() => expect((reservation as unknown as { viewRefresh: Promise<void> | null }).viewRefresh).toBeNull());

        const actionRequest = createBrowserActorUiIntentRequestV1(
          {
            scope: { spaceId: binding.spaceId, documentId },
            verifiedSurfaceId: fields.surface.surfaceId,
            appChannelVersion: actionFixture.request.appChannelVersion,
            activationGeneration: reservation!.generation.toString(),
            instanceId: 0,
            surfaceRevision: 1,
            actionSequence: 2,
          },
          windowKindId,
          { ...actionFixture.uiIntent, surface: windowKindId, revision: 1, seq: 9_007_199_254_740_993n },
        );
        handleTsRequest({ ...actionRequest, clientInstanceId: state.openClientInstanceId });
        await vi.waitFor(() => expect(heldActionPatch !== null || workerResponses.some((message) => message.kind === "browser-actor-action-result")).toBe(true));
        expect(workerResponses.find((message) => message.kind === "browser-actor-action-result")).toBeUndefined();
        const remoteEnvelope: MutationEnvelope = {
          id: "remote-direct-turn",
          actor: "remote-actor",
          document: documentId,
          schemaVersion: fixture.manifest.artifact.schema,
          deps: [],
          payloadHash: "unused",
          diff: { schemaId: fixture.manifest.artifact.schema, payload: { source: "remote" } },
          inverse: { targetOperation: "remote-direct-turn", inverseDiff: { schemaId: fixture.manifest.artifact.schema, payload: { source: "before" } }, baseVersion: 0, dependencies: [], undoPolicy: "exactBaseOnly" },
        };
        const remoteMessage = exactDocumentBackboneMessage(remoteEnvelope),
          remoteDelivery = reservation!.receiveBackbone(remoteMessage);
        await Promise.resolve();
        expect(backboneIngress).toHaveLength(0);
        // 🚦️ L5: a second action issued while the first one's patch is still being acknowledged — carrying the
        // revision the main thread had painted when it clicked (1, the held patch is 1→2) — is QUEUED behind that
        // acknowledgement and the remote delivery, never answered `action-busy` / `action-owner-mismatch`.
        const earlyCommandInvocation = {
            ...actionFixture.commandInvocation,
            address: { ...actionFixture.commandInvocation.address, owner: { app: { pluginId: fields.package.pluginId, appId: fields.surface.appId } } },
          },
          earlyCommandView = { ...actionFixture.commandViewState, activeWindowKindId: windowKindId, windowId: windowKindId },
          earlyCommandRequest = createBrowserActorAppCommandRequestV1(
            {
              scope: { spaceId: binding.spaceId, documentId },
              verifiedSurfaceId: fields.surface.surfaceId,
              appChannelVersion: actionFixture.request.appChannelVersion,
              activationGeneration: reservation!.generation.toString(),
              instanceId: 0,
              surfaceRevision: 1,
              actionSequence: 3,
            },
            earlyCommandInvocation,
            earlyCommandView,
          );
        handleTsRequest({ ...earlyCommandRequest, clientInstanceId: state.openClientInstanceId });
        await Promise.resolve();
        await Promise.resolve();
        expect(workerResponses.filter((message) => message.kind === "browser-actor-action-result")).toHaveLength(0);
        expect(commandSequences).toEqual([]);
        const windowRendersBeforeActions = visibleViews.length;
        const actionPatch = heldActionPatch!;
        const actionApplied = uiStore.applyPatch(actionPatch.patches[0]!);
        expect(actionApplied.ok).toBe(true);
        handleTsRequest({
          kind: "browser-actor-ui-patch-result",
          clientInstanceId: state.openClientInstanceId,
          scope: actionPatch.scope,
          verifiedSurfaceId: actionPatch.verifiedSurfaceId,
          activationGeneration: actionPatch.activationGeneration,
          instanceId: actionPatch.instanceId,
          receipt: actionPatch.receipt,
          verdicts: [{ surface: actionPatch.patches[0]!.surface, outcome: "acknowledged", revision: uiStore.getRevisionSnapshot() }],
        });
        await vi.waitFor(() => expect(workerResponses.some((message) => message.kind === "browser-actor-action-result")).toBe(true));
        await remoteDelivery;
        const actionResult = workerResponses.find((message) => message.kind === "browser-actor-action-result") as Extract<BackboneWorkerResponse, { kind: "browser-actor-action-result" }>;
        expect(actionResult).toMatchObject({
          clientInstanceId: state.openClientInstanceId,
          actionSequence: 2,
          surfaceRevision: 1,
          outcome: "guest-applied",
          mutationCount: 0,
        });
        expect(decodeBrowserActorHostEffectsV1(actionResult.hostEffects)).toEqual([actionFixture.publication.projectedEffect]);
        expect(actionSequences).toEqual([9_007_199_254_740_993n]);
        expect(actionPatchAcknowledged).toBe(true);
        expect(backboneIngress).toHaveLength(1);
        expect(backboneIngress[0]).toEqual(remoteMessage);
        expect(workerResponses.some((message) => message.kind === "event" && message.event.kind === "documentBackbone")).toBe(false);
        // ✅ The early command applied AFTER the patch acknowledgement and the remote delivery, against the state the
        // user saw (surfaceRevision 1 while the worker had rendered 2), with the guest's mutation counted.
        await vi.waitFor(() => expect(workerResponses.filter((message) => message.kind === "browser-actor-action-result")).toHaveLength(2));
        const earlyCommandResult = workerResponses.filter((message): message is Extract<BackboneWorkerResponse, { kind: "browser-actor-action-result" }> => message.kind === "browser-actor-action-result").at(-1)!;
        expect(earlyCommandResult).toMatchObject({ actionSequence: 3, surfaceRevision: 1, outcome: "guest-applied", mutationCount: 1 });
        expect(decodeBrowserActorHostEffectsV1(earlyCommandResult.hostEffects)).toEqual([actionFixture.publication.projectedEffect]);
        // 🪟️ The author's own windows are re-projected after its document-changing command (the guest's command turn does
        // not re-render them: C11 measured the author seeing its own edit ~20 s late), never after a zero-mutation intent.
        expect(visibleViews.length - windowRendersBeforeActions).toBe(1);
        expect(commandSequences).toEqual([3]);
        expect(backboneIngress).toHaveLength(1);

        const directActionInvocation = {
            ...actionFixture.actionInvocation,
            address: { ...actionFixture.actionInvocation.address, pluginId: fields.package.pluginId, appId: fields.surface.appId, windowKindId, windowInstanceId: windowKindId },
          },
          directCommandInvocation = {
            ...actionFixture.commandInvocation,
            address: { ...actionFixture.commandInvocation.address, owner: { app: { pluginId: fields.package.pluginId, appId: fields.surface.appId } } },
          },
          directCommandView = { ...actionFixture.commandViewState, activeWindowKindId: windowKindId, windowId: windowKindId };
        for (const [index, invocation] of [directActionInvocation, directCommandInvocation].entries()) {
          const commandRequest = createBrowserActorAppCommandRequestV1(
            {
              scope: { spaceId: binding.spaceId, documentId },
              verifiedSurfaceId: fields.surface.surfaceId,
              appChannelVersion: actionFixture.request.appChannelVersion,
              activationGeneration: reservation!.generation.toString(),
              instanceId: 0,
              surfaceRevision: 2,
              actionSequence: index + 4,
            },
            invocation,
            directCommandView,
          );
          handleTsRequest({ ...commandRequest, clientInstanceId: state.openClientInstanceId });
          await vi.waitFor(() => expect(workerResponses.filter((message) => message.kind === "browser-actor-action-result")).toHaveLength(index + 3));
          const commandResult = workerResponses.filter((message): message is Extract<BackboneWorkerResponse, { kind: "browser-actor-action-result" }> => message.kind === "browser-actor-action-result").at(-1)!;
          expect(commandResult).toMatchObject({ actionSequence: index + 4, surfaceRevision: 2, outcome: "guest-applied", mutationCount: 0 });
          expect(decodeBrowserActorHostEffectsV1(commandResult.hostEffects)).toEqual([actionFixture.publication.projectedEffect]);
        }
        expect(commandSequences).toEqual([3, 4, 5]);
        expect(visibleViews.length - windowRendersBeforeActions).toBe(3);
        expect(commandInvocations).toEqual([earlyCommandInvocation, directActionInvocation, directCommandInvocation]);
        expect(commandViews).toEqual([earlyCommandView, directCommandView, directCommandView]);

        retainInferenceApprovalUndo(operation, approvalReceipt);
        reissueInferenceApprovalUndoForRebootstrap(state);
        const ownerAwaitingA = testSeams.inferenceApprovalUndoOwner!;
        const stateB: ArtifactState = {
          ...state,
          runtimeKey: documentRuntimeKeyV1({ kind: "hub", dataClass: "persistedShared", spaceId: "unrelated-space", documentId: "unrelated-document" }),
          config: { ...state.config, documentId: "unrelated-document", bindings: [{ kind: "hub", dataClass: "persistedShared", baseUrl: fixture.hubOrigin, spaceId: "unrelated-space" }] },
          openClientInstanceId: "unrelated-client",
          browserActorReservation: reservation,
          verifiedColdPair: owner,
        };
        bindInferenceApprovalUndoToMountedPair(stateB, reservation!, owner);
        expect(testSeams.inferenceApprovalUndoOwner).toBe(ownerAwaitingA);
        expect(testSeams.inferenceApprovalUndoOwner?.phase).toBe("awaiting-mount");
        reservation!.bindApprovalUndoIfMounted();
        expect(testSeams.inferenceApprovalUndoOwner?.phase).toBe("available");

        for (const field of ["directoryRevision", "membershipGeneration", "sessionGeneration"] as const) {
          retainInferenceApprovalUndo(operation, approvalReceipt);
          const originalOwner = testSeams.inferenceApprovalUndoOwner!;
          reissueInferenceApprovalUndoForRebootstrap(state);
          const reissued = testSeams.inferenceApprovalUndoOwner!;
          expect(originalOwner.abort.signal.aborted).toBe(true);
          expect(reissued).toMatchObject({ phase: "awaiting-mount", idempotencyKey: originalOwner.idempotencyKey });
          const rotatedFields = structuredClone(fields);
          rotatedFields.revalidation[field] = (rotatedFields.revalidation[field] ?? 0) + 1;
          const rotatedLease = new DocumentExecutionTargetLease(documentExecutionTargetLeaseMintToken, parseDocumentExecutionTargetLeaseFieldsV1(rotatedFields), fixture.hubOrigin, executionTargetBytes(fixture.componentHex), executionTargetBytes(fixture.descriptorHex));
          state.executionTargetLease = rotatedLease;
          reservation!.bindApprovalUndoIfMounted();
          expect(testSeams.inferenceApprovalUndoOwner).toBeNull();
          expect(historyStatuses.at(-1)?.status).toEqual({ phase: "unavailable", canUndo: false, code: null });
          rotatedLease.drop();
          state.executionTargetLease = lease;
        }

        const shareFields = structuredClone(fields);
        delete shareFields.revalidation.sessionGeneration;
        shareFields.revalidation.shareGeneration = 1;
        const shareLease = new DocumentExecutionTargetLease(documentExecutionTargetLeaseMintToken, parseDocumentExecutionTargetLeaseFieldsV1(shareFields), fixture.hubOrigin, executionTargetBytes(fixture.componentHex), executionTargetBytes(fixture.descriptorHex));
        state.executionTargetLease = shareLease;
        retainInferenceApprovalUndo(operation, approvalReceipt);
        reissueInferenceApprovalUndoForRebootstrap(state);
        const rotatedShareFields = structuredClone(shareFields);
        rotatedShareFields.revalidation.shareGeneration = (rotatedShareFields.revalidation.shareGeneration ?? 0) + 1;
        const rotatedShareLease = new DocumentExecutionTargetLease(documentExecutionTargetLeaseMintToken, parseDocumentExecutionTargetLeaseFieldsV1(rotatedShareFields), fixture.hubOrigin, executionTargetBytes(fixture.componentHex), executionTargetBytes(fixture.descriptorHex));
        state.executionTargetLease = rotatedShareLease;
        reservation!.bindApprovalUndoIfMounted();
        expect(testSeams.inferenceApprovalUndoOwner).toBeNull();
        rotatedShareLease.drop();
        shareLease.drop();
        state.executionTargetLease = lease;

        retainInferenceApprovalUndo(operation, approvalReceipt);
        reservation!.bindApprovalUndoIfMounted();
        const submittingOwner = testSeams.inferenceApprovalUndoOwner!;
        expect(submittingOwner.phase).toBe("available");
        const answerUndo: { resolve: ((response: Response) => void) | null } = { resolve: null };
        globalThis.fetch = stubFetch(async (input) => {
          expect(String(input).endsWith("/inference/gis-map/approval-undos")).toBe(true);
          return await new Promise<Response>((resolve) => {
            answerUndo.resolve = resolve;
          });
        });
        const pendingUndo = undoInferenceApproval(submittingOwner.historyEpoch, submittingOwner.clientInstanceId, submittingOwner.scope);
        await vi.waitFor(() => {
          expect(testSeams.inferenceApprovalUndoOwner?.phase).toBe("submitting");
          expect(answerUndo.resolve).not.toBeNull();
        });
        reissueInferenceApprovalUndoForRebootstrap(state);
        const reissuedOwner = testSeams.inferenceApprovalUndoOwner!;
        expect(reissuedOwner.idempotencyKey).toBe(submittingOwner.idempotencyKey);
        expect(submittingOwner.abort.signal.aborted).toBe(true);
        const respondUndo = answerUndo.resolve;
        if (respondUndo === null) throw new Error("approval undo fetch did not enter");
        respondUndo(
          Response.json(
            {
              schema: "semio.hub.gis-map-approval-undo-receipt/v1",
              targetId: approvalReceipt.undo.targetId,
              originalJobId: approvalReceipt.jobId,
              mutationId: "6".repeat(32),
              commandHash: "7".repeat(64),
              applied: true,
              replayed: false,
              frontier: { ...approvalReceipt.undo.expectedCurrent, headEditOrdinal: approvalReceipt.undo.expectedCurrent.headEditOrdinal + 1, headEditId: "undo-edit" },
            },
            {},
          ),
        );
        await pendingUndo;
        expect(testSeams.inferenceApprovalUndoOwner).toBe(reissuedOwner);
        expect(historyStatuses.some((message) => message.historyEpoch === submittingOwner.historyEpoch && message.status.phase === "applied")).toBe(false);
        expect(testSeams.inferenceApprovalUndoOwner?.phase).toBe("awaiting-mount");

        const renderDriver = reservation as unknown as { child: DocumentBrowserActorChild; renderSurface(child: DocumentBrowserActorChild, assertCurrent: () => void): Promise<void> };
        const retainedChild = renderDriver.child;
        for (const row of corpus.browserRender.hostile) {
          let turns = 0,
            current = row.name !== "stale-before-turn";
          const child = {
            async invoke(path: readonly string[], args: BrowserActorChildValue[]): Promise<BrowserActorChildValue> {
              expect(path).toEqual(["reactor", "poll"]);
              const events = args[0] as Record<string, any>[];
              expect(events).toEqual(turns === 0 ? [{ tag: "surface-visible", val: { surface: { instance: 0, surface: windowKindId }, bodyKey, viewState: encodePackValue(state.browserActorViewState!) } }, secondWindowVisible(state.browserActorViewState!), ...panelVisible(state.browserActorViewState!), ...sectionVisible(state.browserActorViewState!, false)] : [{ tag: "wake" }]);
              turns++;
              if (row.name === "stale-after-turn") current = false;
              return {
                uiPatches: [],
                uiPatchReceipt: null,
                lifecycleReceipt: null,
                status: { tag: row.name === "faulted-turn" ? "faulted" : "more-work" },
                coldPairIngress: row.name === "cold-ingress" ? { tag: "page-accepted", val: { lifetime: activeLifetime!, transferGeneration: owner!.transferGeneration, pageIndex: 0, pageCount: owner!.pageCount } } : { tag: "idle" },
              };
            },
          } as unknown as DocumentBrowserActorChild;
          renderDriver.child = child;
          try {
            await expect(
              renderDriver.renderSurface(child, () => {
                if (!current) throw new Error("fixture-stale-render");
              }),
            ).rejects.toThrow(row.error);
          } finally {
            renderDriver.child = retainedChild;
          }
          expect(turns).toBe(row.turns);
        }
        dropVerifiedColdDocumentPair(state);
        expect(() => owner!.page({ activationGeneration: reservation!.generation, instanceId: 0, guestLifetime: 1n }, 0)).toThrow("stale owner");
        expect({ loaded, described, lifecycleAcknowledged, pages: pageIndexes.length, networkChunks: bootstrap.chunk_count, terminated }).toEqual({
          loaded: 1,
          described: 1,
          lifecycleAcknowledged: true,
          pages: exact.pageCount,
          networkChunks: 2,
          terminated: 0,
        });
        expect(panelRefreshes).toBeGreaterThan(0);
        console.log(
          `browser-cold-pair-transfer: lazy-render-events=${renderEvents.join(",")} pages=${pageIndexes.length} bytes=${received.byteLength} lifecycle-ack=1 applied=1 patch-ack=1 patch-rejected=1 tiled-map=1 network-chunks=${bootstrap.chunk_count}`,
        );
      } finally {
        closeArtifactRuntime(state.runtimeKey);
        expect(await reservation?.retirementOutcome).toBe("retired");
        expect(state.browserActorViewState).toBeNull();
        expect(terminated).toBe(1);
        expect(documentControls).toEqual(["bind", "retire"]);
        expect(lifecycleClose).toEqual(["close", "accepted-ack", "retired-ack"]);
        expect(browserActorChildCapacity()).toEqual({ actors: 0, bytes: 0 });
        testSeams.executionTargetStatusObserver = null;
        testSeams.workerPostTestSink = null;
        clearHubSessionCapability();
        globalThis.fetch = originalFetch;
        (globalThis as unknown as { Worker: unknown }).Worker = originalWorker;
      }
    });

    it("browser GIS viewer exposes localized renderer-unavailable after verified lease", async () => {
      const fixture = await executionTargetLeaseFixture();
      const component = executionTargetBytes(fixture.componentHex);
      const descriptor = executionTargetBytes(fixture.descriptorHex);
      const plan = { ...structuredClone(fixture.plan), expiresAtUnixMs: Date.now() + 30_000 };
      const harness = await executionTargetHarness(fixture, (url) => {
        if (url.endsWith("/open-plan")) return Response.json(plan);
        if (url.endsWith("/execution-target/manifest")) return Response.json(fixture.manifest);
        if (url.endsWith("/execution-target/component")) return executionTargetBodyResponse(component);
        if (url.endsWith("/execution-target/descriptor")) return executionTargetBodyResponse(descriptor);
        return Response.json({ ...fixture.socketGrant, expiresAtMs: Date.now() + 25_000 });
      });
      const events: ArtifactEvent[] = [];
      const originalPostMessage = (globalThis as unknown as { postMessage?: unknown }).postMessage;
      try {
        await requestDocumentSocketAuthority(harness.state, harness.binding);
        const terminal = harness.statuses.at(-1)!;
        expect(terminal.code).toBe(fixture.expected.rendererState);
        for (const [code, text] of Object.entries(fixture.expected.status)) {
          expect(DOCUMENT_EXECUTION_TARGET_STATUS_TEXT_V1[code as keyof typeof DOCUMENT_EXECUTION_TARGET_STATUS_TEXT_V1]).toEqual(text);
          expect(documentExecutionTargetStatusRoleV1(code as keyof typeof DOCUMENT_EXECUTION_TARGET_STATUS_TEXT_V1)).toBe(fixture.expected.statusRoles[code]);
        }
        expect(Object.keys(DOCUMENT_EXECUTION_TARGET_STATUS_TEXT_V1).sort()).toEqual(Object.keys(fixture.expected.status).sort());
        const workerSource = await (await import("node:fs/promises")).readFile(new URL("./🔨️modules/🏪️store/👷️worker/🟦️.ts", source.url), "utf8");
        const leaseRegion = workerSource.slice(workerSource.indexOf("//#region 🪪️ExecutionTargetLease"), workerSource.indexOf("//#endregion 🪪️ExecutionTargetLease"));
        expect(leaseRegion).not.toContain("loadPluginModule");
        expect(leaseRegion).not.toContain("ActivationRegistry");
        expect(leaseRegion).not.toContain("load_wasm_plugins");
        expect(leaseRegion).not.toContain("attach_backbone");
        expect(Object.values(fixture.expected.rendererClaims).every((claim) => claim === false)).toBe(true);
        harness.state.hubActorReady = true;
        const originalEmit = FakeHubWebSocket.instances.length;
        relayMutationsToHub(harness.state, [sampleEnvelope()]);
        expect(harness.state.outbox).toHaveLength(0);
        expect(FakeHubWebSocket.instances).toHaveLength(originalEmit);
        expect(events).toHaveLength(0);
      } finally {
        (globalThis as unknown as { postMessage?: unknown }).postMessage = originalPostMessage;
        harness.release();
      }
    });

  });
  //#endregion 🔖️OfflineResilienceTests
}
