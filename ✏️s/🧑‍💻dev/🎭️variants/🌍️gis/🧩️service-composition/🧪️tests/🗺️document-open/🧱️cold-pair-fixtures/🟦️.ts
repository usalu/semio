/** 🧱️ The cold pair fixtures: the exact pack and spr pattern, the guest descriptor, the visible surface helpers and the recorded worker traffic.
 *
 * 🧱️ Child module of {@link ../🟦️.ts | registerGisDocumentOpenTests}: a phase of the cold pair transfer test over the same registrar bindings, moved verbatim. */
import { workerLawFakesV1, executionTargetBytes } from "@semio-tech/framework-os/testing/worker-law-fakes";
import type { TestSource } from "@semio-tech/framework-os/testing/worker-law-fakes";
import type { ServiceCompositionTestDependenciesV1 } from "../../🧰️dependencies/🟦️.ts";
import type { ColdPairProbe, BackboneWorkerResponse, BrowserActorUiPatchOfferV1, ResolvedPluginViewState } from "../📐️document-open-types/🟦️.ts";

/** 🧪️ Reads the corpus, derives the exact pack and spr pattern, rewrites the guest descriptor and builds the recorded traffic arrays. */
export async function prepareColdPairFixtures({ fakes, dependencies, source }: {
  readonly fakes: ReturnType<typeof workerLawFakesV1>;
  readonly dependencies: ServiceCompositionTestDependenciesV1;
  readonly source: TestSource;
}) {
  const { executionTargetLeaseFixture } = fakes;
  const { decodePackValue, encodePackValue, executionTargetSha256Hex } = dependencies;
  const fixture = await executionTargetLeaseFixture();
  const corpus = JSON.parse(await (await import("node:fs/promises")).readFile(new URL("./🔨️modules/🔌️plugin/⚛️reactor/📥️cold-pair/🧪️tests/🧫️fixtures/🔣️.json", source.url), "utf8"));
  const actionFixture = JSON.parse(await (await import("node:fs/promises")).readFile(new URL("./🔨️modules/🔌️plugin/🌐️browser-bundle/🎯️action-handoff/🧪️tests/🧫️fixtures/🔣️.json", source.url), "utf8"));
  const { applyPatch } = await import("fast-json-patch");
  const rendererOracle = applyPatch({ revision: 0, nodeKind: null as string | null }, [
    { op: "replace", path: "/revision", value: corpus.browserRender.revision },
    { op: "replace", path: "/nodeKind", value: corpus.browserRender.nodeKind },
  ]).newDocument;
  const renderEvents: string[] = [];
  const probe: ColdPairProbe = { renderWakes: 0, activeLifetime: null, panelRefreshes: 0, stagedCommandSequence: null, stagedColdPairIngress: { tag: "idle" }, lifecycleAcknowledged: false, patchAcknowledged: false, patchRejected: false, actionPatchAcknowledged: false, heldActionPatch: null, loaded: 0, described: 0, terminated: 0, owner: null, reservation: null };
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
  const { panelViewContext, sectionViewContext, windowViewContext } = await import("@semio-tech/framework/manifest");
  const { BROWSER_ACTOR_VISIBLE_SURFACES_V1 } = await import("@semio-tech/framework-os/modules/store/worker/visible-surfaces");
  const sectionVisible = (view: ResolvedPluginViewState, mount: boolean) => BROWSER_ACTOR_VISIBLE_SURFACES_V1.sections.filter((section) => mount || !section.static).map((section) => ({ tag: "surface-visible", val: { surface: { instance: 0, surface: section.bodyKey }, bodyKey: section.bodyKey, viewState: encodePackValue(sectionViewContext(view)) } }));
  const secondWindowVisible = (view: ResolvedPluginViewState) => ({ tag: "surface-visible", val: { surface: { instance: 0, surface: secondWindow.key }, bodyKey: secondWindow.bodyKey, viewState: encodePackValue(windowViewContext(view, secondWindow.key)!) } });
  const descriptor = encodePackValue(guest),
    descriptorByteSha256 = await executionTargetSha256Hex(descriptor);
  const { UiDocumentStore } = await import("@semio-tech/framework-os/modules/renderer/engine/elements/UiDocumentStore");
  const { decodeDocumentBackboneControlV1, encodeDocumentBackboneControlV1 } = await import("@semio-tech/framework/backbone/binding");
  const { decodeAppCommand, encodeAppFrame } = await import("@semio-tech/framework-os");
  const { readActorBytePage } = await import("@semio-tech/framework-actor/page");
  const { createBrowserActorAppCommandRequestV1 } = await import("@semio-tech/framework/browser-bundle/action-handoff/command");
  const { createBrowserActorUiIntentRequestV1 } = await import("@semio-tech/framework/browser-bundle/action-handoff/intent");
  const { decodeBrowserActorHostEffectsV1 } = await import("@semio-tech/framework/browser-bundle/action-handoff/publication");
  const windowKindId = fixture.manifest.surface.windowKindId;
  const hostFixture = JSON.parse(await (await import("node:fs/promises")).readFile(new URL("./🔨️modules/🔌️plugin/🌐️browser-bundle/🪟️view-context/🧪️tests/🧫️fixtures/🪟️host-opening-context/🔣️.json", source.url), "utf8"));
  const hostView = { ...hostFixture.valid.viewState, windowId: windowKindId, activeWindowKindId: windowKindId, activeUtilityId: "pan", activeUtilityByWindowId: { [windowKindId]: "pan" }, windowInstances: [{ id: windowKindId, windowKindId }, { id: secondWindow.key, windowKindId: secondWindow.key }] };
  const bodyKey = leaseBodyKey(guest);
  function leaseBodyKey(descriptor: Record<string, any>): string {
    return descriptor.manifest.apps.find((app: Record<string, any>) => app.id === fixture.manifest.surface.appId).windowKinds.find((window: Record<string, any>) => window.id === windowKindId).bodyKey;
  }
  const visibleViews: Record<string, unknown>[] = [];
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
  return { actorBytes, fixture, hostView, descriptorByteSha256, descriptor, exact, publishedPack, publishedSpr, statuses, workerResponses, historyStatuses, probe, patchOffers, uiStore, guest, decodeAppCommand, commandSequences, commandInvocations, commandViews, corpus, pageIndexes, received, windowKindId, panelVisible, sectionVisible, actionSequences, backboneIngress, bodyKey, visibleViews, secondWindowVisible, renderEvents, lifecycleClose, decodeDocumentBackboneControlV1, documentControls, encodeDocumentBackboneControlV1, encodeAppFrame, actionFixture, pack, spr, rendererOracle, createBrowserActorUiIntentRequestV1, createBrowserActorAppCommandRequestV1, decodeBrowserActorHostEffectsV1, originalFetch, originalWorker };
}
