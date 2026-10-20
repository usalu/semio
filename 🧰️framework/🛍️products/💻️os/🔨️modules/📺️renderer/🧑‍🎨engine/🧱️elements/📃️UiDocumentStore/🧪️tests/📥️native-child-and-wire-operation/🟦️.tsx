/** 📥 Owned native child, wire operation and owned instance binding laws. */
import { grant, createTypedWireSupport, type TypedWireVitest, type TypedWireDependencies } from "../🪛️typedwire-helpers/🟦️.tsx";
import { loadRetainedNativeChildFixture, loadRetainedWireOperationsFixture, loadRetainedInstanceOwnerFixture } from "@semio-tech/ui/testing/fixtures";

export async function registerNativeChildAndWireOperationTests(vitest: TypedWireVitest, dependencies: TypedWireDependencies): Promise<void> {
  const { describe, expect, it, vi } = vitest;
  const { DEFAULT_UI_DOCUMENT_LIMITS } = dependencies;
  const { close, finish, produce, retire } = await createTypedWireSupport(vitest, dependencies);

  //#region 🧾️TypedWireTests
  describe("TypedWire", () => {
    //#region 📥️NativeChildTests
    it("OwnedNativeChild preserves decode refusal and raw over-grant work through both operation layers", async () => {
      const { OwnedUiWireOperationCursor, OwnedUiWirePatchCursor } = await import("@semio-tech/ui-contract/retained/operations/wire");
      const { RetainedUiTypedCursor } = await import("@semio-tech/ui-contract/retained/wire/typed"); const { OwnedUiSurface } = await import("@semio-tech/ui-contract/retained/surface");
      const fixture = await loadRetainedNativeChildFixture(); const { encodePackValue } = await import("@semio-tech/framework/packs"); const deepStrictEqual: typeof import("node:assert").deepStrictEqual = (await import("node:assert")).deepStrictEqual;
      for (const layer of ["operation-decode", "stream-decode"]) for (const kind of fixture.refusals) for (const bytes of [0, fixture.overGrantBytes]) {
        const current: import("@semio-tech/ui-contract/retained/wire").RetainedUiWireStep = { kind: kind === "blocked" ? "blocked" : "rejected", phase: layer, items: bytes ? 1 : 0, bytes };
        const surface = new OwnedUiSurface({ actor: "child", instance: 7, surface: "window" }, DEFAULT_UI_DOCUMENT_LIMITS, { usizeBits: 64 });
        const cursor = layer === "operation-decode" ? new OwnedUiWireOperationCursor("set-component", 7, encodePackValue({ type: "text", value: "Child" })) : new OwnedUiWirePatchCursor(surface, 0, 1, 1);
        if (cursor instanceof OwnedUiWirePatchCursor) expect(cursor.offer(0, { tag: "set-root", val: 7n })).toBe(true);
        const spy = layer === "operation-decode" ? vi.spyOn(RetainedUiTypedCursor.prototype, "advance").mockReturnValueOnce(current) : vi.spyOn(OwnedUiWireOperationCursor.prototype, "advance").mockReturnValueOnce(current);
        try { deepStrictEqual(cursor.advance(grant), { ...current, kind: bytes > grant.maxBytes ? "rejected" : current.kind }); expect(cursor.terminalIsEmpty()).toBe(false); }
        finally { spy.mockRestore(); close(cursor); close(surface); }
      }
    });

    it("OwnedNativeChild preserves exact close refusal without unlinking real payload and decoder owners", async () => {
      const { OwnedUiWireOperationCursor, OwnedUiWirePatchCursor } = await import("@semio-tech/ui-contract/retained/operations/wire"); const { RetainedUiTypedCursor, UiPayloadRetirement } = await import("@semio-tech/ui-contract/retained/wire/typed");
      const { OwnedUiOperationRetirement } = await import("@semio-tech/ui-contract/retained/operations"); const { OwnedUiSurface } = await import("@semio-tech/ui-contract/retained/surface"); const fixture = await loadRetainedNativeChildFixture(); const { encodePackValue } = await import("@semio-tech/framework/packs"); const deepStrictEqual: typeof import("node:assert").deepStrictEqual = (await import("node:assert")).deepStrictEqual;
      for (const layer of fixture.layers.filter(value => !value.endsWith("decode"))) for (const kind of fixture.refusals) {
        const current: import("@semio-tech/ui-contract/retained/wire").RetainedUiWireStep = { kind: kind === "blocked" ? "blocked" : "rejected", phase: layer, items: 0, bytes: 0 };
        const surface = new OwnedUiSurface({ actor: "child-close", instance: 7, surface: "window" }, DEFAULT_UI_DOCUMENT_LIMITS, { usizeBits: 64 });
        const cursor = layer.startsWith("stream") ? new OwnedUiWirePatchCursor(surface, 0, 1, 1) : new OwnedUiWireOperationCursor("set-component", 7, encodePackValue({ type: "text", value: "Close" }));
        if (cursor instanceof OwnedUiWirePatchCursor) { expect(cursor.offer(0, { tag: "set-component", val: { node: 7n, component: encodePackValue({ type: "text", value: "Stream close" }) } })).toBe(true); if (layer === "stream-operation-retirement") { let captured = false; for (let turn = 0; !captured && turn < 10000; turn++) captured = cursor.advance(grant).phase === "native-patch-result"; expect(captured).toBe(true); } }
        else if (layer === "payload-retirement") { let reached = false; for (let turn = 0; !reached && turn < 10000; turn++) reached = cursor.advance(grant).phase === "native-payload-release"; expect(reached).toBe(true); }
        else if (layer === "operation-retirement") expect(finish(cursor)).toBe("ready");
        if (layer !== "payload-retirement") cursor.beginClose(); let called = false;
        const fail = () => { called = true; return current; };
        const spy = layer === "payload-retirement" ? vi.spyOn(UiPayloadRetirement.prototype, "advance").mockImplementation(fail) : layer === "decoder-close" ? vi.spyOn(RetainedUiTypedCursor.prototype, "closeStep").mockImplementation(fail) : layer === "stream-input-close" ? vi.spyOn(OwnedUiWireOperationCursor.prototype, "closeStep").mockImplementation(fail) : vi.spyOn(OwnedUiOperationRetirement.prototype, "advance").mockImplementation(fail);
        try { let observed: typeof current | null = null; for (let turn = 0; !called && turn < 10000; turn++) observed = layer === "payload-retirement" ? cursor.advance(grant) : cursor.closeStep(grant); expect(called, layer).toBe(true); deepStrictEqual(observed, current); expect(cursor.terminalIsEmpty()).toBe(false); }
        finally { spy.mockRestore(); close(cursor); close(surface); }
      }
    });

    it("OwnedNativeChild separates terminal full-grant work and retains thrown close ownership", async () => {
      const { OwnedUiWireOperationCursor, OwnedUiWirePatchCursor } = await import("@semio-tech/ui-contract/retained/operations/wire"); const { RetainedUiTypedCursor } = await import("@semio-tech/ui-contract/retained/wire/typed"); const { OwnedUiSurface } = await import("@semio-tech/ui-contract/retained/surface"); const fixture = await loadRetainedNativeChildFixture(); const { encodePackValue } = await import("@semio-tech/framework/packs");
      for (const layer of fixture.terminalLayers) {
        const surface = new OwnedUiSurface({ actor: "terminal-child", instance: 7, surface: "window" }, DEFAULT_UI_DOCUMENT_LIMITS, { usizeBits: 64 }); const cursor = layer === "decoder-close" ? new OwnedUiWireOperationCursor("set-component", 7, encodePackValue({ type: "text", value: "Terminal" })) : new OwnedUiWirePatchCursor(surface, 0, 1, 1); if (cursor instanceof OwnedUiWirePatchCursor) expect(cursor.offer(0, { tag: "set-component", val: { node: 7n, component: encodePackValue({ type: "text", value: "Terminal" }) } })).toBe(true); cursor.beginClose(); let terminal = false;
        const typed = RetainedUiTypedCursor.prototype.closeStep; const native = OwnedUiWireOperationCursor.prototype.closeStep;
        const spy = layer === "decoder-close" ? vi.spyOn(RetainedUiTypedCursor.prototype, "closeStep").mockImplementation(function (this: import("@semio-tech/ui-contract/retained/wire/typed").RetainedUiTypedCursor<"component">, budget) { const result = typed.call(this, budget); if (this.terminalIsEmpty()) { terminal = true; return { ...result, kind: "complete", items: 1, bytes: fixture.childBytes }; } return result; }) : vi.spyOn(OwnedUiWireOperationCursor.prototype, "closeStep").mockImplementation(function (this: import("@semio-tech/ui-contract/retained/operations/wire").OwnedUiWireOperationCursor, budget) { const result = native.call(this, budget); if (this.terminalIsEmpty()) { terminal = true; return { ...result, kind: "complete", items: 1, bytes: fixture.childBytes }; } return result; });
        try { let observed = cursor.closeStep(grant); for (let turn = 0; !terminal && turn < 10000; turn++) observed = cursor.closeStep(grant); expect(terminal).toBe(true); expect(observed).toMatchObject({ kind: "pending", items: 1, bytes: fixture.childBytes }); expect(cursor.terminalIsEmpty()).toBe(false); expect(cursor.closeStep(grant)).toMatchObject({ kind: "pending", bytes: fixture.releaseBytes }); }
        finally { spy.mockRestore(); close(cursor); close(surface); }
      }
      const cursor = new OwnedUiWireOperationCursor("set-component", 7, encodePackValue({ type: "text", value: "Fault" })); cursor.beginClose(); const spy = vi.spyOn(RetainedUiTypedCursor.prototype, "closeStep").mockImplementationOnce(() => { throw new Error(fixture.throwReason); });
      try { expect(cursor.closeStep(grant)).toMatchObject({ kind: "rejected", items: 0, bytes: 0 }); expect(cursor.failure).toBe(fixture.throwReason); expect(cursor.terminalIsEmpty()).toBe(false); }
      finally { spy.mockRestore(); close(cursor); }
    });
    //#endregion 📥️NativeChildTests

    it("OwnedWireOperation consumes all eleven native tags into paired publication before its exact ACK", async () => {
      const { OwnedUiWireOperationCursor } = await import("@semio-tech/ui-contract/retained/operations/wire");
      const { OwnedUiSurface } = await import("@semio-tech/ui-contract/retained/surface");
      const fixture = await loadRetainedWireOperationsFixture(); const { encodePackValue } = await import("@semio-tech/framework/packs"); const { Buffer } = await import("node:buffer");
      const surface = new OwnedUiSurface({ actor: "wire", instance: 7, surface: "window" }, DEFAULT_UI_DOCUMENT_LIMITS, { usizeBits: 64 }); const subscription = surface.subscribeNode(fixture.root, () => {}); while (surface.maintenancePending) surface.advanceMaintenance(grant); const original = surface.view; const patch = surface.beginPatch(0, 1); const tags = new Set<string>();
      const apply = (tag: string, target: unknown, payload?: unknown) => { const input = new OwnedUiWireOperationCursor(tag, target, payload); expect(input.advance({ maxItems: 0, maxBytes: 4096 }).kind).toBe("blocked"); expect(finish(input)).toBe("ready"); patch.pushOperation(input.takeResult()!); close(input); expect(finish(patch)).toBe("ready"); expect(surface.view).toBe(original); expect(patch.takeAcknowledgement()).toBeNull(); tags.add(tag); };
      const rootBytes = encodePackValue(fixture.node); const alias = new Uint8Array(rootBytes.buffer); apply("upsert", undefined, rootBytes); expect(alias.byteLength).toBe(0);
      apply("upsert", undefined, encodePackValue({ ...fixture.node, id: fixture.child, key: "child" }));
      for (const field of fixture.fields) apply(field.tag, BigInt(fixture.root), encodePackValue(field.value));
      apply("set-children", BigInt(fixture.root), new BigUint64Array()); apply("set-root", BigInt(fixture.root)); apply("remove", BigInt(fixture.child));
      patch.finishInput(); expect(finish(patch)).toBe("ready"); const record = subscription.snapshot!.record!; expect(record.component).toMatchObject({ type: "text", value: "Neu / New 😀" }); expect(record.disabled).toBe(true); expect(record.accessibility.label).toBe("Ansicht / View 😀"); expect(tags.size).toBe(11);
      const bytes = Buffer.from(JSON.stringify(produce({ surface: "window", revision: 1, root: fixture.root, nodes: [record], layoutEpoch: "0" }, () => {}))); let hash = 0x811c9dc5; for (const byte of bytes) hash = Math.imul(hash ^ byte, 0x01000193) >>> 0;
      expect(patch.takeAcknowledgement()).toEqual({ actor: "wire", instance: 7, surface: "window", revision: 1, hash: `${hash.toString(16)}:1` }); close(patch); surface.unsubscribeNode(subscription); close(surface);
    });

    it("OwnedWireOperation rejects hostile transport inputs before transfer and closes every admitted prefix", async () => {
      const { OwnedUiWireOperationCursor } = await import("@semio-tech/ui-contract/retained/operations/wire");
      const { RetainedUiChildIdsCursor } = await import("@semio-tech/ui-contract/retained/wire/typed");
      const fixture = await loadRetainedWireOperationsFixture(); const { encodePackValue } = await import("@semio-tech/framework/packs");
      const hostile = new Set<string>();
      const refuse = (name: string, tag: string, target: unknown, payload?: unknown) => { const bytes = payload instanceof Uint8Array ? payload.byteLength : null; expect(() => new OwnedUiWireOperationCursor(tag, target, payload)).toThrow(); if (payload instanceof Uint8Array) expect(payload.byteLength).toBe(bytes); hostile.add(name); };
      refuse("unknown-tag", "unsupported", 7, encodePackValue(null)); refuse("unexpected-payload", "remove", 7, encodePackValue(null)); refuse("unsafe-node-id", "set-component", 9007199254740992n, encodePackValue(null)); refuse("negative-node-id", "set-root", -1); refuse("wrong-payload-kind", "upsert", undefined, []);
      const backing = new Uint8Array(32); refuse("payload-subview", "upsert", undefined, backing.subarray(1)); expect(backing.byteLength).toBe(32);
      const shared = new Uint8Array(new SharedArrayBuffer(32)); refuse("payload-shared-buffer", "upsert", undefined, shared);
      refuse("children-over-capacity", "set-children", 7, new BigUint64Array(fixture.nativeChildren.maximum + 1));
      let getterCalls = 0; const accessor: unknown[] = []; Object.defineProperty(accessor, 0, { enumerable: true, get() { getterCalls++; return 1; } });
      refuse("children-accessor", "set-children", 7, accessor);
      const unsafeChildren = new OwnedUiWireOperationCursor("set-children", 7, new BigUint64Array([9007199254740992n])); expect(finish(unsafeChildren)).toBe("rejected"); expect(unsafeChildren.takeResult()).toBeNull(); close(unsafeChildren); hostile.add("children-unsafe-id");
      expect(getterCalls).toBe(0); expect([...hostile].sort()).toEqual([...fixture.hostile].sort());
      const maximum = BigUint64Array.from({ length: fixture.nativeChildren.maximum }, (_, index) => BigInt(fixture.nativeChildren.values[index % 3]!)); const expected = JSON.parse(JSON.stringify(Array.from(maximum, Number))); const alias = new BigUint64Array(maximum.buffer);
      const child = new RetainedUiChildIdsCursor(maximum); expect(alias.byteLength).toBe(0); expect(finish(child)).toBe("ready"); const payload = child.takeResult()!; expect(payload.value).toEqual(expected); expect(Object.is(payload.value[0], -0)).toBe(false); retire(payload.beginClose()); close(child);
      const phases = new Set<string>(); let prefixes = 0;
      for (let prefix = 0; ; prefix++) {
        const input = new OwnedUiWireOperationCursor("upsert", undefined, encodePackValue(fixture.node)); let ready = false;
        for (let index = 0; index < prefix; index++) { const current = input.advance(grant); phases.add(current.phase); expect(current.bytes).toBeLessThanOrEqual(grant.maxBytes); expect(current.items).toBeLessThanOrEqual(1); if (current.kind === "ready") { ready = true; break; } if (current.kind === "rejected") throw new Error(input.failure!); }
        input.beginClose(); expect(input.takeResult()).toBeNull(); expect(input.closeStep({ maxItems: 1, maxBytes: 0 }).kind).toBe("blocked"); close(input); expect(input.terminalIsEmpty()).toBe(true); prefixes++; if (ready) break; expect(prefix).toBeLessThan(4096);
      }
      expect(prefixes).toBeGreaterThan(20); expect([...phases]).toEqual(expect.arrayContaining(["typed-ready", "native-operation-retire", "native-payload-release", "native-operation-ready"]));
    });

    it("OwnedWireOperation transfers intrinsic buffer ownership without reading shadowed view metadata", async () => {
      const { OwnedUiWireOperationCursor } = await import("@semio-tech/ui-contract/retained/operations/wire"); const { encodePackValue } = await import("@semio-tech/framework/packs");
      const bytes = encodePackValue({ type: "text", value: "Sicher / Safe" }); const original = new Uint8Array(bytes.buffer); const unrelated = new ArrayBuffer(bytes.byteLength); let getters = 0;
      Object.defineProperty(bytes, "buffer", { get() { getters++; return unrelated; } });
      const input = new OwnedUiWireOperationCursor("set-component", 7, bytes); close(input); expect(original.byteLength).toBe(0); expect(unrelated.byteLength).toBeGreaterThan(0); expect(getters).toBe(0);
    });

    it("OwnedWireOperation accepts intrinsic native fields from another realm at the public entry", async () => {
      const { OwnedUiWireOperationCursor } = await import("@semio-tech/ui-contract/retained/operations/wire"); const { runInNewContext } = await import("node:vm"); const { encodePackValue } = await import("@semio-tech/framework/packs");
      const packet = encodePackValue({ type: "text", value: "Bereich / Realm 😀" }); const bytes: unknown = runInNewContext("new Uint8Array(bytes)", { bytes: Array.from(packet) }); const children: unknown = runInNewContext("new BigUint64Array([0n, 1099511627776n, 9007199254740991n])");
      if (!ArrayBuffer.isView(bytes) || !ArrayBuffer.isView(children)) throw new Error("VM did not create native views"); expect(bytes instanceof Uint8Array).toBe(false); expect(children instanceof BigUint64Array).toBe(false);
      for (const [tag, payload] of [["set-component", bytes], ["set-children", children]] as const) { const input = new OwnedUiWireOperationCursor(tag, 7, payload); expect(payload.byteLength).toBe(0); expect(finish(input)).toBe("ready"); retire(input.takeResult()!.beginClose()); close(input); }
    });

    it("OwnedWireOperation mint refuses arbitrary normalized roots before touching source getters", async () => {
      const { OwnedUiOperation, OwnedUiOperationRetirement } = await import("@semio-tech/ui-contract/retained/operations");
      let reads = 0; const source = { get kind() { reads++; return "root"; }, get id() { reads++; return 7; } };
      let rejected = false; let forged: ReturnType<typeof OwnedUiOperation.remove> | undefined;
      try { forged = Reflect.construct(OwnedUiOperation, [source]); } catch { rejected = true; }
      if (forged) retire(forged.beginClose()); expect(rejected).toBe(true); expect(reads).toBe(0);
      expect(() => Reflect.construct(OwnedUiOperationRetirement, [source])).toThrow(/authority/); expect(reads).toBe(0);
    });

    it("OwnedWireOperation stream retains one exact ordinal through close and preserves committed late-cancel ACK", async () => {
      const { OwnedUiWirePatchCursor } = await import("@semio-tech/ui-contract/retained/operations/wire"); const { OwnedUiSurface } = await import("@semio-tech/ui-contract/retained/surface");
      const fixture = await loadRetainedWireOperationsFixture(); const { encodePackValue } = await import("@semio-tech/framework/packs");
      const surface = new OwnedUiSurface({ actor: "stream", instance: 7, surface: "window" }, DEFAULT_UI_DOCUMENT_LIMITS, { usizeBits: 64 }); const original = surface.view;
      const patch = new OwnedUiWirePatchCursor(surface, 0, 1, fixture.stream.operations); let reads = 0; const forbidden = { get tag() { reads++; throw new Error("Unadmitted page access"); } };
      expect(patch.offer(1, forbidden)).toBe(false); expect(reads).toBe(0); const bytes = encodePackValue(fixture.node); expect(patch.offer(0, { tag: "upsert", val: { node: bytes } })).toBe(true); expect(bytes.byteLength).toBe(0); expect(patch.offer(0, forbidden)).toBe(false); expect(patch.offer(1, forbidden)).toBe(false); expect(reads).toBe(0);
      expect(finish(patch)).toBe("ready"); expect(surface.view).toBe(original); expect(patch.takeAcknowledgement()).toBeNull(); expect(patch.offer(1, forbidden)).toBe(false); expect(patch.takePageReceipt()).toEqual({ ordinal: 0 }); expect(patch.takePageReceipt()).toBeNull(); expect(patch.offer(0, forbidden)).toBe(false);
      expect(patch.offer(1, { tag: "set-root", val: BigInt(fixture.root) })).toBe(true); expect(finish(patch)).toBe("ready"); expect(patch.takePageReceipt()).toEqual({ ordinal: 1 }); expect(surface.view).toBe(original); patch.finishInput();
      while (surface.view === original) { const current = patch.advance(grant); if (current.kind === "rejected") throw new Error(patch.failure!); expect(current.bytes).toBeLessThanOrEqual(4096); }
      patch.beginClose(); let blocked = false; for (let i = 0; i < 100_000; i++) { const current = patch.closeStep(grant); if (current.kind === "blocked") { expect(current.phase).toBe("surface-acknowledgement"); blocked = true; break; } if (current.kind === "rejected" || current.kind === "complete") throw new Error("Committed receipt was lost"); }
      expect(blocked).toBe(true); expect(patch.takeAcknowledgement()).toMatchObject({ actor: "stream", instance: 7, surface: "window", revision: 1 }); close(patch);
      const cancelled = new OwnedUiWirePatchCursor(surface, 1, 2, 1); const pending = encodePackValue(fixture.node); expect(cancelled.offer(0, { tag: "upsert", val: { node: pending } })).toBe(true); cancelled.beginClose(); close(cancelled); expect(cancelled.takeAcknowledgement()).toBeNull(); expect(surface.view.revision).toBe(1); close(surface);
    });

    it("OwnedWireOperation native retirement closes known fields without claiming unknown wrapper ownership", async () => {
      const { OwnedUiWireOperationCursor } = await import("@semio-tech/ui-contract/retained/operations/wire"); const fixture = await loadRetainedWireOperationsFixture(); const { encodePackValue } = await import("@semio-tech/framework/packs"); const { Buffer } = await import("node:buffer");
      const bytes = encodePackValue(fixture.node); const unknown = { bytes: new Uint8Array(fixture.rawRetirement.unknownBytes).fill(17) }; const original = { tag: "upsert", val: { node: bytes, unknown }, unknown }; const cursor = OwnedUiWireOperationCursor.fromNative(original); expect(bytes.byteLength).toBe(0); close(cursor); expect(original.unknown).toBe(unknown); expect(original.val.unknown).toBe(unknown); expect(Buffer.from(unknown.bytes)).toEqual(Buffer.alloc(fixture.rawRetirement.unknownBytes, 17));
      const children = BigUint64Array.from(fixture.nativeChildren.values, BigInt); const child = OwnedUiWireOperationCursor.fromNative({ tag: "set-children", val: { node: 7n, children } }); expect(children.byteLength).toBe(0); close(child); close(OwnedUiWireOperationCursor.fromNative({ tag: "remove", val: 7n }));
      const untouched = encodePackValue({}); expect(() => OwnedUiWireOperationCursor.fromNative({ tag: "set-style", val: { node: 9007199254740992n, style: untouched } })).toThrow(/range/); expect(untouched.byteLength).toBeGreaterThan(0); let reads = 0; expect(() => OwnedUiWireOperationCursor.fromNative({ tag: "upsert", get val() { reads++; return { node: untouched }; } })).toThrow(/accessor/); expect(reads).toBe(0); expect(untouched.byteLength).toBeGreaterThan(0);
    });

    it("OwnedInstance binds the exact activation and guest lifetime while closing old readers independently", async () => {
      const { OwnedUiInstance } = await import("@semio-tech/ui-contract/retained/instance");
      const fixture = await loadRetainedInstanceOwnerFixture(); let active = true;
      const activation = Object.freeze({ actorId: fixture.actor, activationGeneration: BigInt(fixture.activationGeneration), assertActive() { if (!active) throw new Error("Revoked activation"); }, async turn() { return {}; } }); const foreign = Object.freeze({ ...activation });
      const oldLife = Object.freeze({ activationGeneration: activation.activationGeneration, instanceId: fixture.instanceId, guestLifetime: BigInt(fixture.guestLifetimes[0]!) }); const nextLife = produce(oldLife, draft => { draft.guestLifetime = BigInt(fixture.guestLifetimes[1]!); });
      const old = new OwnedUiInstance(activation, oldLife, DEFAULT_UI_DOCUMENT_LIMITS, { usizeBits: 64 }); const next = new OwnedUiInstance(activation, nextLife, DEFAULT_UI_DOCUMENT_LIMITS, { usizeBits: 64 });
      expect(old.beginSurfaceLookup(foreign, oldLife, "window")).toBeNull(); expect(old.beginSurfaceLookup(activation, nextLife, "window")).toBeNull();
      const first = old.beginSurfaceLookup(activation, oldLife, "window")!; expect(old.beginSurfaceLookup(activation, oldLife, "inspector")).toBeNull(); expect(first.advance({ maxItems: 0, maxBytes: 4096 }).kind).toBe("blocked"); expect(finish(first)).toBe("ready"); const view = first.takeResult()!; close(first);
      const again = old.beginSurfaceLookup(activation, oldLife, "window")!; expect(finish(again)).toBe("ready"); expect(again.takeResult()).toBe(view); close(again);
      const other = next.beginSurfaceLookup(activation, nextLife, "window")!; expect(finish(other)).toBe("ready"); const replacement = other.takeResult()!; close(other); expect(replacement).not.toBe(view);
      const reader = view.subscribeNode(7, () => {}); while (old.maintenancePending) old.advanceMaintenance(grant); expect(reader.snapshot?.record).toBeUndefined();
      expect(OwnedUiInstance.matches(old, activation, nextLife)).toBe(false); active = false; expect(() => old.beginSurfaceLookup(activation, oldLife, "inspector")).toThrow(/Revoked/);
      old.beginClose(); let waiting = false; for (let i = 0; i < 100; i++) { const current = old.closeStep(grant); if (current.kind === "blocked") { waiting = true; expect(current.phase).toBe("surface-readers"); break; } } expect(waiting).toBe(true); view.unsubscribeNode(reader); close(old); expect(old.terminalIsEmpty()).toBe(true); expect(next.terminalIsEmpty()).toBe(false); expect(replacement.view.revision).toBe(0); close(next);
    });
  });
  //#endregion 🧾️TypedWireTests
}
