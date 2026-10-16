/** 🏢 Owned instance maintenance, close accounting and surface child laws. */
import { grant, nativeInstanceFixture, createTypedWireSupport, type TypedWireVitest, type TypedWireDependencies } from "../🪛️typedwire-helpers/🟦️.tsx";

export async function registerInstanceMaintenanceAndCloseTests(vitest: TypedWireVitest, dependencies: TypedWireDependencies): Promise<void> {
  const { describe, expect, it, vi } = vitest;
  const { DEFAULT_UI_DOCUMENT_LIMITS } = dependencies;
  const { close, finish, produce } = await createTypedWireSupport(vitest, dependencies);

  //#region 🧾️TypedWireTests
  describe("TypedWire", () => {

    //#region 🏘️InstanceMaintenanceTests
    it("OwnedInstanceMaintenance preserves child refusal and callback failure without losing the queued owner", async () => {
      const { OwnedUiInstance } = await import("../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🏘️instance/🟦️.ts"); const { OwnedUiSurface } = await import("../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🖼️surface/🟦️.ts"); const { default: fixture } = await import("../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️tests/🧫️fixtures/🛠️instance-maintenance/🔣️.json"); for (const vector of fixture.cases) {
        const native = await nativeInstanceFixture(); const owner = new OwnedUiInstance(native.activation, native.lifetime, DEFAULT_UI_DOCUMENT_LIMITS, { usizeBits: 64 }); const lookup = owner.beginSurfaceLookup(native.activation, native.lifetime, "window")!; expect(finish(lookup)).toBe("ready"); const surface = lookup.takeResult()!; close(lookup); let notified = 0; const subscription = surface.subscribeView(() => { notified++; });
        const probe = vi.spyOn(OwnedUiSurface.prototype, "advanceMaintenance").mockImplementationOnce(() => { if (vector.kind === "throw") throw new Error(vector.name); if (vector.kind !== "blocked" && vector.kind !== "rejected" && vector.kind !== "pending") throw new Error("Invalid maintenance fixture kind"); return { kind: vector.kind, phase: vector.name, items: vector.items, bytes: vector.bytes }; });
        try { const current = owner.advanceMaintenance(grant); expect(current.kind, vector.name).toBe(vector.expected); expect(current.bytes).toBe(vector.bytes); expect(current.items).toBe(vector.items); expect(owner.maintenancePending, vector.name).toBe(true); expect(notified).toBe(0); if (vector.kind === "throw") expect(owner.maintenanceFailure).toBe(vector.name); } finally { probe.mockRestore(); }
        for (let turn = 0; owner.maintenancePending && turn < 128; turn++) owner.advanceMaintenance(grant); expect(owner.maintenancePending).toBe(false); expect(notified).toBe(produce(fixture.retryNotifications, () => {})); surface.unsubscribeNode(subscription); close(owner); native.client.disposeAll();
      }
      const native = await nativeInstanceFixture(); const owner = new OwnedUiInstance(native.activation, native.lifetime, DEFAULT_UI_DOCUMENT_LIMITS, { usizeBits: 64 }); const lookup = owner.beginSurfaceLookup(native.activation, native.lifetime, "window")!; expect(finish(lookup)).toBe("ready"); const surface = lookup.takeResult()!; close(lookup); const subscription = surface.subscribeView(() => {});
      const probe = vi.spyOn(OwnedUiSurface.prototype, "advanceMaintenance").mockReturnValueOnce({ kind: "complete", phase: "terminal-child", items: 1, bytes: fixture.terminalChildBytes });
      try { expect(owner.advanceMaintenance(grant)).toMatchObject({ kind: "pending", bytes: fixture.terminalChildBytes }); expect(owner.advanceMaintenance(grant)).toMatchObject({ kind: "pending", bytes: fixture.queueTransitionBytes }); expect(probe).toHaveBeenCalledTimes(1); } finally { probe.mockRestore(); }
      surface.unsubscribeNode(subscription); close(owner); native.client.disposeAll();
    });
    //#endregion 🏘️InstanceMaintenanceTests


    //#region 🏁️InstanceCloseTests
    it("OwnedInstanceCloseAccounting preserves full-grant surface completion before separate wrapper release", async () => {
      const { OwnedUiInstance } = await import("../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🏘️instance/🟦️.ts"); const { OwnedUiSurface } = await import("../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🖼️surface/🟦️.ts"); const { default: fixture } = await import("../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️tests/🧫️fixtures/🚪️instance-close/🔣️.json"); const native = await nativeInstanceFixture(); const owner = new OwnedUiInstance(native.activation, native.lifetime, DEFAULT_UI_DOCUMENT_LIMITS, { usizeBits: 64 }); const lookup = owner.beginSurfaceLookup(native.activation, native.lifetime, "window")!; expect(finish(lookup)).toBe("ready"); lookup.takeResult(); close(lookup); const original = OwnedUiSurface.prototype.closeStep; let completed = false;
      const probe = vi.spyOn(OwnedUiSurface.prototype, "closeStep").mockImplementation(function(this: InstanceType<typeof OwnedUiSurface>, budget) { const current = original.call(this, budget); if (current.kind === "complete" && !completed) { completed = true; return { ...current, items: 1, bytes: fixture.childBytes }; } return current; }); owner.beginClose();
      try { for (let turn = 0; !completed && turn < 256; turn++) { const current = owner.closeStep(grant); if (completed) expect(current).toMatchObject({ kind: "pending", bytes: fixture.childBytes }); } expect(completed).toBe(true); expect(owner.takeRetirementWitness() !== null).toBe(fixture.earlyWitness); expect(owner.closeStep(grant)).toMatchObject({ kind: "pending", bytes: produce(fixture.surfaceReleaseBytes, () => {}) }); } finally { probe.mockRestore(); }
      close(owner); native.client.disposeAll();
    });

    it("OwnedInstanceCloseAccounting issues input retirement separately from full-grant wire completion", async () => {
      const { OwnedUiInstance } = await import("../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🏘️instance/🟦️.ts"); const { OwnedUiWirePatchCursor } = await import("../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/🟦️.ts"); const { default: fixture } = await import("../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️tests/🧫️fixtures/🚪️instance-close/🔣️.json");
      for (const mode of fixture.wireOwners) {
        const native = await nativeInstanceFixture(); const owner = new OwnedUiInstance(native.activation, native.lifetime, DEFAULT_UI_DOCUMENT_LIMITS, { usizeBits: 64 }); native.lease.bindHostRetirement(owner); const lookup = owner.beginSurfaceLookup(native.activation, native.lifetime, "window")!; expect(finish(lookup)).toBe("ready"); const surface = lookup.takeResult()!; close(lookup); const source = await native.source([{ tag: "set-root", val: 1n }]); const patch = owner.beginPatch(source, surface); expect(patch.offer(0)).toBe(true); const closing = mode === "patch" ? patch : owner; const original = OwnedUiWirePatchCursor.prototype.closeStep; let completed = false;
        const probe = vi.spyOn(OwnedUiWirePatchCursor.prototype, "closeStep").mockImplementation(function(this: InstanceType<typeof OwnedUiWirePatchCursor>, budget) { const current = original.call(this, budget); if (this.terminalIsEmpty() && !completed) { completed = true; return { ...current, kind: "complete", items: 1, bytes: fixture.childBytes }; } return current; }); closing.beginClose();
        try { for (let turn = 0; !completed && turn < 512; turn++) { const current = closing.closeStep(grant); if (completed) expect(current, mode).toMatchObject({ kind: "pending", bytes: fixture.childBytes }); } expect(completed).toBe(true); expect(closing.closeStep(grant)).toMatchObject({ kind: "pending", bytes: fixture.inputTokenBytes }); } finally { probe.mockRestore(); }
        const receipt = patch.peekInputReceipt()!; expect(receipt).not.toBeNull(); expect(patch.releaseInputReceipt(receipt)).toBe(true); close(closing); if (mode === "patch") close(owner); native.client.disposeAll();
      }
    });

    it("OwnedInstanceCloseAccounting retains lookup refusal diagnostics and its exact child for retry", async () => {
      const { OwnedUiInstance, OwnedUiSurfaceLookup } = await import("../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🏘️instance/🟦️.ts"); const { default: fixture } = await import("../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️tests/🧫️fixtures/🚪️instance-close/🔣️.json");
      for (const kind of fixture.lookupRefusals) { if (kind !== "blocked" && kind !== "rejected") throw new Error("Invalid lookup fixture kind"); const native = await nativeInstanceFixture(); const owner = new OwnedUiInstance(native.activation, native.lifetime, DEFAULT_UI_DOCUMENT_LIMITS, { usizeBits: 64 }); const lookup = owner.beginSurfaceLookup(native.activation, native.lifetime, "window")!; owner.beginClose(); const probe = vi.spyOn(OwnedUiSurfaceLookup.prototype, "closeStep").mockReturnValueOnce({ kind, phase: "exact-lookup-refusal", items: 0, bytes: 0 }); try { expect(owner.closeStep(grant)).toEqual({ kind, phase: "exact-lookup-refusal", items: 0, bytes: 0 }); expect(lookup.terminalIsEmpty()).toBe(false); expect(owner.takeRetirementWitness()).toBeNull(); } finally { probe.mockRestore(); } close(owner); expect(lookup.terminalIsEmpty()).toBe(true); native.client.disposeAll(); }
    });
    //#endregion 🏁️InstanceCloseTests


    //#region 🖼️SurfaceChildTests
    it("OwnedSurfaceChild preserves private lookup and retirement refusal without losing retry ownership", async () => {
      const { OwnedUiSurface, OwnedUiSurfacePatch } = await import("../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🖼️surface/🟦️.ts"); const { OwnedUiSceneBindingIndexReader, OwnedUiSceneBindingIndexRetirement } = await import("../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🎬️scene/🔗️binding/🗂️index/🟦️.ts"); const { OwnedUiNodeIndexRetirement } = await import("../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🗂️nodes/🟦️.ts");
      const { default: fixture } = await import("../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️tests/🧫️fixtures/🪆️surface-child/🔣️.json"); for (const layer of fixture.layers) for (const kind of fixture.refusals) {
        if (kind !== "blocked" && kind !== "rejected") throw new Error("Invalid surface refusal fixture"); const surface = new OwnedUiSurface({ actor: "surface-child", instance: 7, surface: "window" }, DEFAULT_UI_DOCUMENT_LIMITS, { usizeBits: 64 }); const maintenance = layer === "lookup" || layer === "reader-retirement"; const subscription = maintenance ? surface.subscribeNode(7, () => {}) : null; if (layer === "patch-close") surface.beginPatch(0, 1); if (!maintenance) surface.beginClose(); let hit = false;
        const refusal = (): import("../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🟦️.ts").RetainedUiWireStep => { hit = true; return { kind, phase: layer, items: 0, bytes: 0 }; };
        const probe = layer === "lookup" ? vi.spyOn(OwnedUiSceneBindingIndexReader.prototype, "advance").mockImplementationOnce(refusal) : layer === "reader-retirement" || layer === "binding-retirement" ? vi.spyOn(OwnedUiSceneBindingIndexRetirement.prototype, "advance").mockImplementationOnce(refusal) : layer === "node-retirement" ? vi.spyOn(OwnedUiNodeIndexRetirement.prototype, "advance").mockImplementationOnce(refusal) : vi.spyOn(OwnedUiSurfacePatch.prototype, "closeStep").mockImplementationOnce(refusal);
        try { for (let turn = 0; !hit && turn < 256; turn++) { const current = maintenance ? surface.advanceMaintenance(grant) : surface.closeStep(grant); if (hit) expect(current).toEqual(produce({ kind, phase: layer, items: 0, bytes: 0 }, () => {})); } expect(hit).toBe(true); expect(surface.terminalIsEmpty()).toBe(false); if (maintenance) expect(surface.maintenancePending).toBe(true); } finally { probe.mockRestore(); }
        if (subscription) surface.unsubscribeNode(subscription); close(surface);
      }
    });

    it("OwnedSurfaceChild keeps a full-grant reader completion separate from queue work and lease minting", async () => {
      const { OwnedUiSurface } = await import("../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🖼️surface/🟦️.ts"); const { OwnedUiSceneBindingIndexReader } = await import("../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🎬️scene/🔗️binding/🗂️index/🟦️.ts"); const { default: fixture } = await import("../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️tests/🧫️fixtures/🪆️surface-child/🔣️.json");
      const surface = new OwnedUiSurface({ actor: "surface-child", instance: 7, surface: "window" }, DEFAULT_UI_DOCUMENT_LIMITS, { usizeBits: 64 }); const subscription = surface.subscribeNode(7, () => {}); const original = OwnedUiSceneBindingIndexReader.prototype.advance; let hit = false;
      const probe = vi.spyOn(OwnedUiSceneBindingIndexReader.prototype, "advance").mockImplementation(function(this: import("../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🎬️scene/🔗️binding/🗂️index/🟦️.ts").OwnedUiSceneBindingIndexReader, budget) { const current = original.call(this, budget); if (current.kind === "complete" && !hit) { hit = true; return { ...current, items: 1, bytes: fixture.childBytes }; } return current; }); const closing = vi.spyOn(OwnedUiSceneBindingIndexReader.prototype, "beginClose");
      try { for (let turn = 0; !hit && turn < 256; turn++) { const current = surface.advanceMaintenance(grant); if (hit) expect(current).toMatchObject({ kind: "pending", bytes: fixture.childBytes }); } expect(hit).toBe(true); expect(closing.mock.calls.length > 0).toBe(fixture.earlyReaderClose); expect(surface.advanceMaintenance(grant)).toMatchObject({ kind: "pending", bytes: fixture.queueBytes }); expect(surface.advanceMaintenance(grant)).toMatchObject({ kind: "pending", phase: "subscription-read-captured", bytes: fixture.leaseBytes }); expect(closing).toHaveBeenCalledTimes(1); } finally { probe.mockRestore(); closing.mockRestore(); }
      surface.unsubscribeNode(subscription); close(surface);
    });

    it("OwnedSurfaceChild retains a throwing or over-grant private reader for cancellation", async () => {
      const { OwnedUiSurface } = await import("../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🖼️surface/🟦️.ts"); const { OwnedUiSceneBindingIndexReader } = await import("../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🎬️scene/🔗️binding/🗂️index/🟦️.ts");
      for (const mode of ["throw", "overgrant"]) {
        const surface = new OwnedUiSurface({ actor: "reader-fault", instance: 7, surface: "window" }, DEFAULT_UI_DOCUMENT_LIMITS, { usizeBits: 64 }); const subscription = surface.subscribeNode(7, () => {}); let hit = false;
        const probe = vi.spyOn(OwnedUiSceneBindingIndexReader.prototype, "advance").mockImplementationOnce(() => { hit = true; if (mode === "throw") throw new Error("Exact reader failed"); return { kind: "pending", phase: "reader-overgrant", items: 1, bytes: 4097 }; });
        try { for (let turn = 0; !hit && turn < 32; turn++) { const current = surface.advanceMaintenance(grant); if (hit) { expect(current.kind).toBe("rejected"); expect(current.bytes).toBe(mode === "throw" ? 0 : 4097); } } expect(hit).toBe(true); expect(surface.maintenancePending).toBe(true); expect(surface.maintenanceFailure).not.toBeNull(); } finally { probe.mockRestore(); }
        surface.unsubscribeNode(subscription); close(surface);
      }
    });
    //#endregion 🖼️SurfaceChildTests
  });
  //#endregion 🧾️TypedWireTests
}
