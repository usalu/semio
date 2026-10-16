/** 🖥 Shared context-menu and surface render view models, reserved refresh sections and the renderer resident composition ledger. */
import type { UiNodeRecord } from "@semio-tech/framework";
import type { RetainedSurface } from "../../🧱️elements/🔌️PluginRuntime/🟦️.tsx";
import { type PluginRuntimeVitest, type PluginRuntimeDependencies, type PluginRuntimeSource } from "../🛠️plugin-runtime-helpers/🟦️.tsx";
import { loadUiRefreshSectionFixture, loadPagedTextCarrierFixture } from "../../../../../../../🔨️modules/🛂️manifest/🧪️tests/🧫️fixtures/🟦️.ts";
import { loadValueResidentSchema } from "../../../../../../../🔨️modules/🌱️value/🧪️tests/🧫️fixtures/🟦️.ts";

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

export async function registerViewModelSurfacesTests(vitest: PluginRuntimeVitest, dependencies: PluginRuntimeDependencies, source: PluginRuntimeSource): Promise<void> {
  const { describe, expect, it, vi } = vitest;
  const { applyUiPatchToRetained, decodePackValue, performContextMenu, retainedSurfaceId, retainedSurfaceToBuiltNode, retainedUiRefreshResponse, sectionValueFromBuiltNode, SemioFaultError, uiRefreshBodyKeys, uiRefreshSectionTargets, uiRefreshSurfaceEvents } = dependencies;
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
    });
  });

  describe("surface render ViewModel", () => {
    it("binds two instances of one body to distinct surfaces and mints no synthetic window alias", () => {
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
        ["details", "details-body"],
      ]);
      const views = events.map((event: { payload: { viewState: Uint8Array } }) => decodePackValue(event.payload.viewState));
      expect(views).toMatchObject([
        { locale: "de", terminology: "reuse", windowId: "first", activeWindowKindId: "canvas", activeUtilityId: "inspect" },
        { locale: "de", terminology: "reuse", windowId: "second", activeWindowKindId: "canvas", activeUtilityId: "measure" },
        { locale: "de", terminology: "reuse" },
      ]);
      expect((views[2] as { windowId?: string | null }).windowId == null).toBe(true);
    });
  });

  describe("reserved refresh sections", () => {
    it("mounts one retained surface per requested section, keyed by the neutral fixture's reserved body key, carrying the unnarrowed view", async () => {
      const reserved = await loadUiRefreshSectionFixture();
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
    });

    it("projects a section's chunked text carrier back into the shell's own measures map", async () => {
      const reserved = await loadUiRefreshSectionFixture();
      const measures = { "puzzle3d-main-top": [{ kind: "slider", id: "puzzle3d-fill-count", value: 12, min: 1, max: 64, step: 1, onChange: { controller: "fill", action: "setCount" } }] };
      const payload = JSON.stringify(measures);
      const chunks:readonly[string,string]=[payload.slice(0,17),payload.slice(17)];
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
    });

    it("reassembles a PACKED carrier leaf — value plus every sorted dataAttributes slice — against the neutral paged-text-carrier fixture", async () => {
      const carrier = await loadPagedTextCarrierFixture();
      const firstSlice=carrier.slices.at(0);if(firstSlice===undefined)throw Error("Actual paged text carrier first slice required");
      expect(carrier.slices.join("")).toBe(carrier.payload);
      expect(carrier.payload.length).toBe(carrier.payloadBytes);
      expect(carrier.payloadBytes).toBeGreaterThan(carrier.textMaxBytes);
      expect(firstSlice.length).toBe(carrier.textMaxBytes);
      expect(() => JSON.parse(firstSlice)).toThrow();
      expect(carrier.slices.length).toBeGreaterThan(1);
      expect(carrier.slices.length).toBeLessThanOrEqual(carrier.packSlices);
      const dataAttributes = Object.fromEntries(carrier.slices.slice(1).map((slice: string, offset: number) => [String(offset + 1).padStart(2, "0"), slice]));
      const { surface, desynced } = applyUiPatchToRetained(null, {
        surface: carrier.rootKey, revision: 1, baseRevision: 0,
        ops: [
          { type: "upsert", ...packedCarrierLeaf(1, firstSlice, dataAttributes) },
          { type: "upsert", ...packedCarrierRoot(carrier.rootKey, [1]) },
          { type: "setRoot", id: 0 },
        ],
      });
      expect(desynced).toBe(false);
      const built = retainedSurfaceToBuiltNode(surface!);
      if (built === null) throw new Error("fixture surface has no root");
      expect(sectionValueFromBuiltNode(carrier.rootKey, built, "instance 7")).toEqual(JSON.parse(carrier.payload));
    });

    it("raises a typed fault naming the reserved section and its producer when a carrier's payload is not valid JSON", async () => {
      const carrier = await loadPagedTextCarrierFixture();
      const firstSlice=carrier.slices.at(0);if(firstSlice===undefined)throw Error("Actual paged text carrier first slice required");
      const { surface } = applyUiPatchToRetained(null, {
        surface: carrier.rootKey, revision: 1, baseRevision: 0,
        ops: [
          { type: "upsert", ...packedCarrierLeaf(1, firstSlice, null) },
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
    });
  });

  it("RendererResidentComposition never replaces a closing composition ledger", async () => {
      const { execFileSync } = await import("node:child_process"); const { fileURLToPath, pathToFileURL } = await import("node:url"); const { dirname, resolve } = await import("node:path"); const { default: fixture } = await import("../../💾️resident/🧪️tests/🧫️fixtures/🔣️.json"); const moduleUrl = pathToFileURL(resolve(dirname(fileURLToPath(source.url)), "../../../../../../../🔨️modules/🖼️canvas/💾️renderer-resident/🟦️.ts")).href;
      const childSource = `const { rendererResidentLedger } = await import(process.argv[1]); const first = rendererResidentLedger(); first.beginClose(); const result = first.closeStep({maxItems:1,maxBytes:256}); const second = rendererResidentLedger(); const admission = second.prepareAdmission({},'data',{maxItems:1,maxBytes:296}); process.stdout.write(JSON.stringify({same:first===second,terminal:first.terminalIsEmpty(),result:result.kind,admission:admission.kind}));`;
      const actual = JSON.parse(execFileSync("node", ["--experimental-transform-types", "--input-type=module", "--eval", childSource, moduleUrl], { encoding: "utf8", timeout: 10000 }));
      expect(actual).toEqual({ same: !fixture.replacesClosingLedger, terminal: true, result: "complete", admission: "rejected" });
    });

    it("RendererResidentComposition shares one exact ledger and preserves both consumers' charges", async () => {
      const { rendererResidentLedger } = await import("../../../../../../../🔨️modules/🖼️canvas/💾️renderer-resident/🟦️.ts"); const { default: fixture } = await import("../../💾️resident/🧪️tests/🧫️fixtures/🔣️.json");
      const { default: rendererModule } = await import("../../../🧬️schema/🔣️.json"); const resident = await loadValueResidentSchema(); const { default: Ajv } = await import("ajv"); const { produce } = await import("immer");
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
}
