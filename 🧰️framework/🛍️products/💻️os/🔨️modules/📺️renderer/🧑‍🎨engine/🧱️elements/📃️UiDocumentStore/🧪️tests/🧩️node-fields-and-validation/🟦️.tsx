/** 🧩 Native component normalisation, typed node fields, node index, operation, validation, hash and read lease laws. */
import { grant, type UiNodeRecord, createTypedWireSupport, type TypedWireVitest, type TypedWireDependencies } from "../🪛️typedwire-helpers/🟦️.tsx";
import { loadRetainedWireTypedFixture, loadRetainedWireFieldsFixture, loadRetainedOwnedNodesFixture, loadRetainedOwnedOperationsFixture, loadRetainedOwnedValidationFixture, loadRetainedOwnedHashFixture, loadRetainedReadLeaseFixture, loadRetainedReadPublicationFixture } from "@semio-tech/framework/ui/testing/fixtures";

export async function registerNodeFieldsAndValidationTests(vitest: TypedWireVitest, dependencies: TypedWireDependencies): Promise<void> {
  const { describe, expect, it } = vitest;
  const { DEFAULT_UI_DOCUMENT_LIMITS, validateUiDocumentCore } = dependencies;
  const { Buffer, close, finish, produce, retire } = await createTypedWireSupport(vitest, dependencies);

  //#region 🧾️TypedWireTests
  describe("TypedWire", () => {
    it("normalizes all native component variants and defaults with strict neutral and Immer oracles", async () => {
      const { RetainedUiTypedCursor } = await import("@semio-tech/framework/modules/ui/contract/retained/wire/typed");
      const fixture = await loadRetainedWireTypedFixture();
      const { encodePackValue } = await import("@semio-tech/framework/packs");
      for (const vector of fixture.components) {
        const cursor = new RetainedUiTypedCursor(encodePackValue(vector.wire), "component");
        expect(cursor.takeResult()).toBeNull(); expect(finish(cursor), cursor.failure ?? vector.wire.type).toBe("ready");
        const payload = cursor.takeResult()!; close(cursor);
        const value = payload.value;
        const projected = value.type === "surface" ? { ...value, doc: { bytes: Array.from({ length: value.doc.bytes.length }, (_, i) => value.doc.bytes.byteAt(i)) } } : value;
        expect(projected).toEqual(JSON.parse(JSON.stringify(produce(vector.expected, () => {}))));
        expect(Object.isFrozen(value)).toBe(true); expect(Reflect.set(value, "type", "forged")).toBe(false);
        if (value.type === "surface") expect("beginClose" in value.doc.bytes).toBe(false);
        retire(payload.beginClose());
      }
      const style = new RetainedUiTypedCursor(encodePackValue({}), "style"); expect(finish(style)).toBe("ready"); const styled = style.takeResult()!; expect(styled.value).toEqual(fixture.defaults.style); close(style); retire(styled.beginClose());
      const accessibility = new RetainedUiTypedCursor(encodePackValue({}), "accessibility"); expect(finish(accessibility)).toBe("ready"); const accessible = accessibility.takeResult()!; expect(accessible.value).toEqual(fixture.defaults.accessibility); close(accessibility); retire(accessible.beginClose());
      for (const vector of fixture.hostile) { const cursor = new RetainedUiTypedCursor(encodePackValue(vector), "component"); expect(finish(cursor)).toBe("rejected"); expect(cursor.takeResult()).toBeNull(); close(cursor); }
    });

    it("validates real upsert and set-component entry roots, safe IDs, and every layout variant", async () => {
      const { RetainedUiTypedCursor } = await import("@semio-tech/framework/modules/ui/contract/retained/wire/typed");
      const { encodePackValue } = await import("@semio-tech/framework/packs");
      const layouts = [
        { kind: "leaf", width: "hug", height: "fill" },
        { kind: "stack", axis: "vertical", gap: "sm", padding: { all: "none" }, align: "stretch", justify: "start", grow: false, wrap: true },
        { kind: "grid", columns: [{ fraction: 2 }, "auto"], rows: ["maxContent"], columnGap: "xs", rowGap: "md", padding: { symmetric: { vertical: "xs", horizontal: "sm" } }, align: "center", justify: "spaceBetween" },
        { kind: "overlay", anchor: "bottomEnd", inset: { each: { top: "none", right: "xs", bottom: "sm", left: "md" } }, dismissible: true },
        { kind: "scroll", axes: "both", padding: { all: "none" }, sizing: { fixed: "xl" } },
        { kind: "absolute", sizingWidth: "fill", sizingHeight: "hug" },
      ];
      for (const layout of layouts) { const cursor = new RetainedUiTypedCursor(encodePackValue(layout), "layout"); expect(finish(cursor), cursor.failure ?? "").toBe("ready"); const owner = cursor.takeResult()!; expect(owner.value).toEqual(layout); close(cursor); retire(owner.beginClose()); }
      const wire = { tag: "upsert", val: { node: encodePackValue({ id: Number.MAX_SAFE_INTEGER, key: "surface", component: { type: "surface", kind: "node-graph", docSchema: "node-graph@1", doc: { bytes: [5, 7, 9] } }, layout: layouts[0], style: {}, activity: "idle", accessibility: {} }) } };
      const node = new RetainedUiTypedCursor(wire.val.node, "node"); expect(finish(node), node.failure ?? "").toBe("ready"); const owner = node.takeResult()!; close(node);
      expect(owner.value.id).toBe(Number.MAX_SAFE_INTEGER); expect(owner.value.disabled).toBe(false); expect(owner.value.children).toEqual([]); expect(owner.value.transition).toBeNull();
      expect(owner.value.component.type).toBe("surface"); if (owner.value.component.type === "surface") expect(owner.value.component.doc.bytes.byteAt(2)).toBe(9);
      retire(owner.beginClose());
      for (const malformed of [{ ...layouts[0], width: "100px" }, { ...layouts[2], columns: [{ fraction: 256 }] }, { ...layouts[0], width: { fixed: "xs", stray: true } }]) { const cursor = new RetainedUiTypedCursor(encodePackValue(malformed), "layout"); expect(finish(cursor)).toBe("rejected"); close(cursor); }
    });

    it("captures exact surface bytes before decoder close and retains older snapshots until their own final retirement", async () => {
      const { RetainedUiTypedCursor } = await import("@semio-tech/framework/modules/ui/contract/retained/wire/typed");
      const { encodePackValue } = await import("@semio-tech/framework/packs");
      const expected = Buffer.alloc(32768); for (let i = 0; i < expected.length; i++) expected[i] = (i * 17 + 3) & 255;
      const wire = { tag: "set-component", val: { node: 7, component: encodePackValue({ type: "surface", kind: "node-graph", docSchema: "node-graph@1", doc: { bytes: Array.from(expected) } }) } };
      const cursor = new RetainedUiTypedCursor(wire.val.component, "component"); expect(finish(cursor), cursor.failure ?? "").toBe("ready"); const published = cursor.takeResult()!; const oldReader = published.capture(); const concurrentReader = published.capture(); close(cursor);
      const value = oldReader.value; if (value.type !== "surface") throw new Error("Expected retained surface"); const borrowed = value.doc.bytes;
      const publicationRetirement = published.beginClose(); expect(publicationRetirement.advance({ maxItems: 0, maxBytes: 4096 }).kind).toBe("blocked"); expect(borrowed.byteAt(32767)).toBe(expected[32767]); retire(publicationRetirement);
      retire(concurrentReader.beginClose()); expect(Buffer.from(Array.from({ length: borrowed.length }, (_, i) => borrowed.byteAt(i)))).toEqual(expected);
      const cancelled = new RetainedUiTypedCursor(encodePackValue({ type: "surface", kind: "node-graph", docSchema: "node-graph@1", doc: { bytes: [4] } }), "component"); expect(finish(cancelled)).toBe("ready"); close(cancelled); expect(borrowed.byteAt(0)).toBe(3);
      retire(oldReader.beginClose()); expect(() => borrowed.byteAt(0)).toThrow(); expect(oldReader.terminalIsEmpty()).toBe(true);
    });

    it("TypedNodeFields retains seven direct fields without ancestor chains across repeated replacement", async () => {
      const { RetainedUiTypedCursor, OwnedUiNode } = await import("@semio-tech/framework/modules/ui/contract/retained/wire/typed");
      const fixture = await loadRetainedWireFieldsFixture();
      const { encodePackValue } = await import("@semio-tech/framework/packs");
      const input = new RetainedUiTypedCursor(encodePackValue(fixture.node), "node"); expect(finish(input)).toBe("ready"); const payload = input.takeResult()!;
      expect(() => Object.defineProperty(payload, "value", { value: { ...payload.value, id: 1 } })).toThrow();
      let current = OwnedUiNode.captureFrom(payload); const oldReader = current.capture(); close(input); retire(payload.beginClose());
      expect(() => Object.defineProperty(current, "value", { value: { ...current.value, id: 1 } })).toThrow();
      const old = oldReader.value; if (old.component.type !== "surface") throw new Error("Expected old surface"); const oldBytes = old.component.doc.bytes;
      const replacement = new RetainedUiTypedCursor(encodePackValue(fixture.replacement), "component"); expect(finish(replacement)).toBe("ready"); const reusable = replacement.takeResult()!; close(replacement);
      for (let i = 0; i < fixture.repeat; i++) {
        const moved = reusable.capture(); const next = current.replace({ field: "component", payload: moved }); expect(moved.terminalIsEmpty()).toBe(true); retire(current.beginClose()); current = next;
        expect(current.value.layout).toBe(old.layout); expect(current.value.accessibility).toBe(old.accessibility); expect(current.value.children).toBe(old.children); expect(current.value.component).toBe(reusable.value);
      }
      expect(Array.from({ length: oldBytes.length }, (_, i) => oldBytes.byteAt(i))).toEqual(fixture.expected.oldBytesAfterReplacement);
      expect(current.value.id).toBe(fixture.expected.id); expect(current.value.component).toEqual(produce(fixture.expected.component, () => {}));
      retire(oldReader.beginClose()); expect(() => oldBytes.byteAt(0)).toThrow(); expect(current.value.layout).toBe(old.layout);
      retire(current.beginClose()); retire(reusable.beginClose());
    });

    it("TypedNodeFields rejects rebound field authority before moving it and cancels without releasing current", async () => {
      const { RetainedUiTypedCursor, OwnedUiNode } = await import("@semio-tech/framework/modules/ui/contract/retained/wire/typed");
      const fixture = await loadRetainedWireFieldsFixture();
      const { encodePackValue } = await import("@semio-tech/framework/packs");
      const input = new RetainedUiTypedCursor(encodePackValue(fixture.node), "node"); expect(finish(input)).toBe("ready"); const payload = input.takeResult()!; const current = OwnedUiNode.captureFrom(payload); close(input); retire(payload.beginClose());
      const style = new RetainedUiTypedCursor(encodePackValue({}), "style"); expect(finish(style)).toBe("ready"); const wrong = style.takeResult()!; close(style);
      expect(() => Reflect.apply(current.replace, current, [{ field: "component", payload: wrong }])).toThrow(); expect(wrong.terminalIsEmpty()).toBe(false);
      const changed = current.replace({ field: "style", payload: wrong }); const discard = changed.beginClose(); expect(discard.advance({ maxItems: 0, maxBytes: 4096 }).kind).toBe("blocked"); retire(discard);
      const value = current.value; if (value.component.type !== "surface") throw new Error("Expected unchanged surface"); expect(value.component.doc.bytes.byteAt(0)).toBe(3); retire(current.beginClose());
    });

    it("TypedNodeFields closes each partial field builder and rejects forged node projection owners", async () => {
      const { RetainedUiTypedCursor, OwnedUiNode } = await import("@semio-tech/framework/modules/ui/contract/retained/wire/typed");
      const fixture = await loadRetainedWireFieldsFixture();
      const { encodePackValue } = await import("@semio-tech/framework/packs");
      const reference = new RetainedUiTypedCursor(encodePackValue(fixture.node), "node"); let count = 0;
      for (let n = 0; n < 10000; n++) { const current = reference.advance(grant); if (current.phase === "typed-normalize") count++; if (current.kind === "ready") break; }
      close(reference); expect(count).toBeGreaterThan(50);
      for (let cutoff = 0; cutoff <= count; cutoff++) {
        const cursor = new RetainedUiTypedCursor(encodePackValue(fixture.node), "node"); let observed = 0;
        for (let n = 0; n < 10000; n++) { const current = cursor.advance(grant); if (current.phase === "typed-normalize" && observed++ === cutoff) break; if (current.kind === "ready") break; }
        close(cursor);
      }
      const wrong = new RetainedUiTypedCursor(encodePackValue(fixture.node.component), "component"); expect(finish(wrong)).toBe("ready"); const owner = wrong.takeResult()!; close(wrong);
      expect(() => Reflect.apply(OwnedUiNode.captureFrom, OwnedUiNode, [owner])).toThrow(); expect(owner.terminalIsEmpty()).toBe(false); retire(owner.beginClose());
    });

    it("OwnedNodeIndex captures exact readers across removal and closes final surface owners in bounded steps", async () => {
      const { OwnedUiNodeIndex } = await import("@semio-tech/framework/modules/ui/contract/retained/nodes");
      const { RetainedUiTypedCursor, OwnedUiNode } = await import("@semio-tech/framework/modules/ui/contract/retained/wire/typed");
      const fixture = await loadRetainedOwnedNodesFixture();
      const fields = await loadRetainedWireFieldsFixture();
      const { encodePackValue } = await import("@semio-tech/framework/packs");
      const node = (id: number) => { const cursor = new RetainedUiTypedCursor(encodePackValue({ ...fields.node, id }), "node"); expect(finish(cursor)).toBe("ready"); const payload = cursor.takeResult()!; const result = OwnedUiNode.captureFrom(payload); close(cursor); retire(payload.beginClose()); return result; };
      let index = OwnedUiNodeIndex.empty();
      const oracle = new Map<number, true>();
      for (const id of fixture.ids) { const input = node(id); const edit = index.beginSet(input); retire(input.beginClose()); expect(finish(edit)).toBe("ready"); const next = edit.takeResult()!; retire(edit.beginClose()); retire(index.beginClose()); index = next; oracle.set(id, true); }
      const old = index.capture(); const lookup = old.beginLookup(fixture.removed);
      let held: ReturnType<typeof OwnedUiNode.captureFrom> | null = null;
      for (;;) { const result = lookup.advance(grant); if (result.kind === "value") held = result.value; if (result.kind === "complete") break; }
      retire(lookup.beginClose()); if (!held || held.value.component.type !== "surface") throw new Error("Expected captured surface node"); const bytes = held.value.component.doc.bytes;
      const removal = index.beginRemove(fixture.removed); expect(finish(removal)).toBe("ready"); const removed = removal.takeResult()!; retire(removal.beginClose()); retire(index.beginClose()); index = removed; oracle.delete(fixture.removed);
      const input = node(fixture.removed); const insertion = index.beginSet(input); retire(input.beginClose()); expect(finish(insertion)).toBe("ready"); const reinserted = insertion.takeResult()!; retire(insertion.beginClose()); retire(index.beginClose()); index = reinserted; oracle.set(fixture.removed, true);
      const order: number[] = []; const reader = index.beginRead();
      for (;;) { const result = reader.advance(grant); expect(result.items).toBeLessThanOrEqual(1); expect(result.bytes).toBeLessThanOrEqual(4096); if (result.kind === "value") { order.push(result.value.value.id); retire(result.value.beginClose()); } if (result.kind === "complete") break; }
      retire(reader.beginClose()); expect(order).toEqual(produce(fixture.expectedReinsertedOrder, () => {})); expect(order).toEqual(Array.from(oracle.keys()));
      retire(index.beginClose()); const closing = old.beginClose(); for (const budget of fixture.grants.slice(0, 2)) expect(closing.advance(budget).kind).toBe("blocked"); retire(closing);
      expect(Array.from({ length: bytes.length }, (_, i) => bytes.byteAt(i))).toEqual(fixture.oldBytes); retire(held.beginClose()); expect(() => bytes.byteAt(0)).toThrow();
      const rejected = node(17); expect(() => index.beginSet(rejected)).toThrow(); expect(rejected.terminalIsEmpty()).toBe(false); retire(rejected.beginClose());
    });

    it("OwnedNodeIndex cancels every persistent edit phase without closing a concurrently captured source", async () => {
      const { OwnedUiNodeIndex } = await import("@semio-tech/framework/modules/ui/contract/retained/nodes");
      const { RetainedUiTypedCursor, OwnedUiNode } = await import("@semio-tech/framework/modules/ui/contract/retained/wire/typed");
      const fields = await loadRetainedWireFieldsFixture();
      const { encodePackValue } = await import("@semio-tech/framework/packs");
      const cursor = new RetainedUiTypedCursor(encodePackValue(fields.node), "node"); expect(finish(cursor)).toBe("ready"); const payload = cursor.takeResult()!; const input = OwnedUiNode.captureFrom(payload); close(cursor); retire(payload.beginClose());
      const source = OwnedUiNodeIndex.empty(); const reference = source.beginSet(input); let count = 0;
      for (;;) { count++; if (reference.advance(grant).kind === "ready") break; } retire(reference.beginClose());
      for (let cutoff = 0; cutoff <= count; cutoff++) { const snapshot = source.capture(); const edit = source.beginSet(input); for (let i = 0; i < cutoff; i++) edit.advance(grant); const closing = edit.beginClose(); expect(edit.takeResult()).toBeNull(); expect(closing.advance({ maxItems: 0, maxBytes: 4096 }).kind).toBe("blocked"); retire(closing); expect(snapshot.size).toBe(0); retire(snapshot.beginClose()); if (input.value.component.type !== "surface") throw new Error("Expected retained original"); expect(input.value.component.doc.bytes.byteAt(0)).toBe(3); }
      retire(source.beginClose()); retire(input.beginClose());
    });

    it("OwnedOperation applies typed field owners and removes subtrees while preserving concurrent source snapshots", async () => {
      const { OwnedUiOperation, OwnedUiOperationCursor } = await import("@semio-tech/framework/modules/ui/contract/retained/operations");
      const { OwnedUiNodeIndex } = await import("@semio-tech/framework/modules/ui/contract/retained/nodes");
      const { RetainedUiTypedCursor } = await import("@semio-tech/framework/modules/ui/contract/retained/wire/typed");
      const fixture = await loadRetainedOwnedOperationsFixture();
      const fields = await loadRetainedWireFieldsFixture();
      const { encodePackValue } = await import("@semio-tech/framework/packs");
      let source = OwnedUiNodeIndex.empty();
      for (const id of [fixture.parent, fixture.child]) {
        const input = new RetainedUiTypedCursor(encodePackValue({ ...fields.node, id, children: id === fixture.parent ? [fixture.child] : [] }), "node"); expect(finish(input)).toBe("ready"); const payload = input.takeResult()!; const op = OwnedUiOperation.upsert(payload); retire(payload.beginClose()); close(input);
        const cursor = new OwnedUiOperationCursor(source, fixture.parent, op); expect(op.terminalIsEmpty()).toBe(true); expect(finish(cursor)).toBe("ready"); const result = cursor.takeResult()!; close(cursor); retire(source.beginClose()); source = result.nodes; retire(result.touched.beginClose());
      }
      const old = source.capture(); const reader = old.beginLookup(fixture.child); let captured: ReturnType<typeof reader.advance> | null = null;
      for (;;) { const result = reader.advance(grant); if (result.kind === "value") captured = result; if (result.kind === "complete") break; } retire(reader.beginClose());
      if (captured?.kind !== "value" || captured.value.value.component.type !== "surface") throw new Error("Expected old surface owner"); const bytes = captured.value.value.component.doc.bytes;
      const input = new RetainedUiTypedCursor(encodePackValue(fixture.replacement), "component"); expect(finish(input)).toBe("ready"); const payload = input.takeResult()!; const op = OwnedUiOperation.field(fixture.child, { field: "component", payload }); retire(payload.beginClose()); close(input);
      const changed = new OwnedUiOperationCursor(source, fixture.parent, op); expect(finish(changed)).toBe("ready"); const replacement = changed.takeResult()!; close(changed); retire(source.beginClose()); source = replacement.nodes;
      expect(Array.from(replacement.touched, ([id]) => id)).toEqual([fixture.child]); retire(replacement.touched.beginClose());
      const check = source.beginLookup(fixture.child); for (;;) { const result = check.advance(grant); if (result.kind === "value") { expect(result.value.value.component).toEqual(produce({ ...fixture.replacement, emphasize: null, dataAttributes: null }, () => {})); retire(result.value.beginClose()); } if (result.kind === "complete") break; } retire(check.beginClose());
      const reference = new OwnedUiOperationCursor(source, fixture.parent, OwnedUiOperation.remove(fixture.parent)); let count = 0; for (;;) { count++; if (reference.advance(grant).kind === "ready") break; } const removed = reference.takeResult()!; expect(removed.nodes.size).toBe(0); expect(Array.from(removed.touched, ([id]) => id)).toEqual(fixture.expected.removedIds); close(reference); retire(removed.nodes.beginClose()); retire(removed.touched.beginClose());
      for (let cutoff = 0; cutoff <= count; cutoff++) { const cursor = new OwnedUiOperationCursor(source, fixture.parent, OwnedUiOperation.remove(fixture.parent)); for (let i = 0; i < cutoff; i++) cursor.advance(grant); cursor.beginClose(); expect(cursor.takeResult()).toBeNull(); expect(cursor.closeStep({ maxItems: 0, maxBytes: 4096 }).kind).toBe("blocked"); close(cursor); expect(source.size).toBe(2); }
      const activity = new RetainedUiTypedCursor(encodePackValue({ activity: "loading", disabled: false }), "activity"); expect(finish(activity)).toBe("ready"); const state = activity.takeResult()!; close(activity); const unknown = new OwnedUiOperationCursor(source, fixture.parent, OwnedUiOperation.activity(123, state)); retire(state.beginClose());
      expect(finish(unknown)).toBe("rejected"); close(unknown); retire(source.beginClose()); retire(old.beginClose()); expect(bytes.byteAt(0)).toBe(3); retire(captured.value.beginClose()); expect(() => bytes.byteAt(0)).toThrow();
    });

    it("OwnedValidation preserves graph violation order, captured byte lifetimes and every cancellation frontier", async () => {
      const { OwnedUiValidationCursor } = await import("@semio-tech/framework/modules/ui/contract/retained/validation");
      const { OwnedUiNodeIndex } = await import("@semio-tech/framework/modules/ui/contract/retained/nodes");
      const { RetainedUiTypedCursor, OwnedUiNode } = await import("@semio-tech/framework/modules/ui/contract/retained/wire/typed");
      const fixture = await loadRetainedOwnedValidationFixture();
      const fields = await loadRetainedWireFieldsFixture();
      const { encodePackValue } = await import("@semio-tech/framework/packs");
      let prefixes = 0;
      for (const vector of fixture.cases) {
        let source = OwnedUiNodeIndex.empty(); const reference = new Map<number, UiNodeRecord>();
        for (const node of vector.nodes) {
          const raw = { ...fields.node, ...node }; const decoder = new RetainedUiTypedCursor(encodePackValue(raw), "node"); expect(finish(decoder)).toBe("ready"); const payload = decoder.takeResult()!; const owner = OwnedUiNode.captureFrom(payload); reference.set(node.id, { ...payload.value, component: { type: "text", value: "oracle", emphasize: null, dataAttributes: null } }); close(decoder); retire(payload.beginClose());
          const edit = source.beginSet(owner); retire(owner.beginClose()); expect(finish(edit)).toBe("ready"); const next = edit.takeResult()!; retire(edit.beginClose()); retire(source.beginClose()); source = next;
        }
        const limits = { ...DEFAULT_UI_DOCUMENT_LIMITS, maxNodes: vector.maxNodes, maxDepth: vector.maxDepth };
        const expected = validateUiDocumentCore(vector.root, new Map(produce(Array.from(reference), () => {})), limits); expect(expected.map((value: { type: string }) => value.type)).toEqual(vector.expected);
        const validator = new OwnedUiValidationCursor(source, vector.root, limits); let count = 0;
        for (;;) { count++; const result = validator.advance(grant); expect(result.bytes).toBeLessThanOrEqual(4096); expect(result.items).toBeLessThanOrEqual(1); if (result.kind === "ready") break; if (result.kind === "rejected") throw new Error(validator.failure ?? "Unexpected validation failure"); }
        const violations = validator.takeResult()!; expect(Array.from(violations, ([, value]) => value)).toEqual(expected); retire(violations.beginClose()); close(validator);
        if (vector.name === "valid-safe53") {
          for (let cutoff = 0; cutoff <= count; cutoff++) { const cursor = new OwnedUiValidationCursor(source, vector.root, limits); for (let i = 0; i < cutoff; i++) cursor.advance(grant); cursor.beginClose(); expect(cursor.takeResult()).toBeNull(); expect(cursor.closeStep({ maxItems: 0, maxBytes: 4096 }).kind).toBe("blocked"); close(cursor); expect(source.size).toBe(2); prefixes++; }
          const reader = source.beginLookup(vector.root!); let held: ReturnType<typeof OwnedUiNode.captureFrom> | null = null;
          for (;;) { const result = reader.advance(grant); if (result.kind === "value") held = result.value; if (result.kind === "complete") break; } retire(reader.beginClose());
          if (!held || held.value.component.type !== "surface") throw new Error("Expected surface byte owner"); const bytes = held.value.component.doc.bytes;
          const cursor = new OwnedUiValidationCursor(source, vector.root, limits); retire(source.beginClose()); expect(finish(cursor)).toBe("ready"); retire(cursor.takeResult()!.beginClose()); close(cursor); expect(bytes.byteAt(0)).toBe(3); retire(held.beginClose()); expect(() => bytes.byteAt(0)).toThrow();
        } else retire(source.beginClose());
      }
      expect(prefixes).toBeGreaterThan(100);
    });

    it("OwnedHash streams exact insertion-ordered JSON bytes while retaining old surface owners through cancellation", async () => {
      const { OwnedUiSnapshotHashCursor } = await import("@semio-tech/framework/modules/ui/contract/retained/hash");
      const { OwnedUiNodeIndex } = await import("@semio-tech/framework/modules/ui/contract/retained/nodes");
      const { RetainedUiTypedCursor, OwnedUiNode } = await import("@semio-tech/framework/modules/ui/contract/retained/wire/typed");
      const fixture = await loadRetainedOwnedHashFixture();
      const fields = await loadRetainedWireFieldsFixture();
      const { encodePackValue } = await import("@semio-tech/framework/packs");
      let source = OwnedUiNodeIndex.empty(); const records: unknown[] = []; let nested: unknown = fixture.text; for (let i = 0; i < fixture.depth; i++) nested = { next: nested };
      for (const [ordinal, id] of fixture.ids.entries()) {
        const component = ordinal === 0 ? { ...fields.node.component, doc: { bytes: Array.from({ length: fixture.surfaceBytes }, (_, i) => i % 251) } } : { type: "extension", extension: "hash", props: { nested, numbers: [-0, 1e-7, 1e21], "0": true, ["__proto__"]: null } };
        const decoder = new RetainedUiTypedCursor(encodePackValue({ ...fields.node, id, component }), "node"); expect(finish(decoder)).toBe("ready"); const payload = decoder.takeResult()!; const node = OwnedUiNode.captureFrom(payload);
        const record = node.value; records.push({ ...record, component: record.component.type === "surface" ? { ...record.component, doc: { bytes: Array.from({ length: record.component.doc.bytes.length }, (_, i) => record.component.type === "surface" ? record.component.doc.bytes.byteAt(i) : 0) } } : record.component });
        close(decoder); retire(payload.beginClose()); const edit = source.beginSet(node); retire(node.beginClose()); expect(finish(edit)).toBe("ready"); const next = edit.takeResult()!; retire(edit.beginClose()); retire(source.beginClose()); source = next;
      }
      const expected = Buffer.from(JSON.stringify(produce({ surface: fixture.surface, revision: fixture.revision, root: fixture.root, nodes: records, layoutEpoch: "0" }, () => {})), "utf8");
      const cursor = new OwnedUiSnapshotHashCursor(source, fixture); const chunks: Uint8Array[] = []; let calls = 0;
      expect(cursor.advance({ maxItems: 0, maxBytes: 4096 }).kind).toBe("blocked");
      for (;;) { calls++; const result = cursor.advance(grant); expect(result.bytes).toBeLessThanOrEqual(4096); expect(result.items).toBeLessThanOrEqual(1); if (result.chunk) chunks.push(result.chunk); if (result.kind === "rejected") throw new Error(cursor.failure ?? "Hash failed"); if (result.kind === "ready") break; }
      expect(Buffer.concat(chunks)).toEqual(expected); let hash = 0x811c9dc5; for (const byte of expected) hash = Math.imul(hash ^ byte, 0x01000193) >>> 0;
      expect(cursor.takeResult()).toEqual({ hash: `${hash.toString(16)}:${fixture.revision}`, byteLength: expected.length }); close(cursor);
      for (const cutoff of [0, 1, 3, Math.floor(calls / 3), Math.floor(calls / 2), calls - 1, calls]) { const cancelled = new OwnedUiSnapshotHashCursor(source, fixture); for (let i = 0; i < cutoff; i++) cancelled.advance(grant); cancelled.beginClose(); expect(cancelled.takeResult()).toBeNull(); close(cancelled); expect(source.size).toBe(fixture.ids.length); }
      const retained = new OwnedUiSnapshotHashCursor(source, fixture); retire(source.beginClose()); expect(finish(retained)).toBe("ready"); expect(retained.takeResult()!.byteLength).toBe(expected.length); close(retained);
    });

    it("ReadLease binds exact consumer commits, keeps stable snapshots and backpressures until retired roots are empty", async () => {
      const { OwnedUiNodeReadLease } = await import("@semio-tech/framework/modules/ui/contract/retained/read-lease");
      const { RetainedUiTypedCursor, OwnedUiNode } = await import("@semio-tech/framework/modules/ui/contract/retained/wire/typed");
      const fixture = await loadRetainedReadLeaseFixture();
      const fields = await loadRetainedWireFieldsFixture();
      const { encodePackValue } = await import("@semio-tech/framework/packs");
      const make = () => { const decoder = new RetainedUiTypedCursor(encodePackValue(fields.node), "node"); expect(finish(decoder)).toBe("ready"); const payload = decoder.takeResult()!; const node = OwnedUiNode.captureFrom(payload); close(decoder); retire(payload.beginClose()); return node; };
      const input = make(); const lease = new OwnedUiNodeReadLease(fixture.node, fixture.versions[0]!, input); const independent = new OwnedUiNodeReadLease(fixture.node, fixture.versions[0]!, input); retire(input.beginClose());
      const initial = lease.snapshot; expect(Reflect.apply(lease.acknowledge, lease, [null])).toBe(false); expect(lease.snapshot).toBe(initial); expect(lease.snapshot).toBe(initial); const firstBytes = initial.record!.component; if (firstBytes.type !== "surface") throw new Error("Expected surface");
      const second = make(); expect(lease.offer(fixture.versions[1]!, second)).toBe(true); retire(second.beginClose()); const later = lease.snapshot;
      expect(lease.acknowledge(independent.snapshot)).toBe(false); expect(lease.acknowledge(initial)).toBe(true); expect(lease.snapshot).toBe(later);
      const third = make(); expect(lease.offer(fixture.versions[2]!, third)).toBe(false); expect(third.terminalIsEmpty()).toBe(false); expect(lease.acknowledge(later)).toBe(true); expect(lease.offer(fixture.versions[2]!, third)).toBe(false);
      expect(lease.advanceRetirement({ maxItems: 0, maxBytes: 4096 }).kind).toBe("blocked"); while (lease.retirementPending) { const result = lease.advanceRetirement(grant); expect(result.bytes).toBeLessThanOrEqual(4096); }
      expect(firstBytes.doc.bytes.byteAt(0)).toBe(3); close(independent); expect(() => firstBytes.doc.bytes.byteAt(0)).toThrow(); expect(() => initial.record).toThrow();
      expect(lease.offer(fixture.versions[2]!, third)).toBe(true); retire(third.beginClose()); const newest = lease.snapshot; expect(lease.acknowledge(initial)).toBe(false); expect(lease.acknowledge(later)).toBe(true); expect(lease.snapshot).toBe(newest);
      const newestComponent = newest.record!.component; if (newestComponent.type !== "surface") throw new Error("Expected surface"); expect(newestComponent.doc.bytes.byteAt(0)).toBe(3); close(lease); expect(() => newestComponent.doc.bytes.byteAt(0)).toThrow(); expect(() => lease.snapshot).toThrow();
      expect(produce(fixture.versions, () => {})).toEqual([0,1,2]);
    });

    it("ReadLease React subscriptions own StrictMode remounts and independently acknowledge exact committed snapshots", async () => {
      const { useOwnedUiNode } = await import("../../📖️owned/🟦️.tsx");
      const { OwnedUiNodeReadLease } = await import("@semio-tech/framework/modules/ui/contract/retained/read-lease");
      const { RetainedUiTypedCursor, OwnedUiNode } = await import("@semio-tech/framework/modules/ui/contract/retained/wire/typed");
      const fields = await loadRetainedWireFieldsFixture();
      const { encodePackValue } = await import("@semio-tech/framework/packs"); const { render, act, cleanup } = await import("@testing-library/react"); const { StrictMode, createElement } = await import("react");
      const decoder = new RetainedUiTypedCursor(encodePackValue(fields.node), "node"); expect(finish(decoder)).toBe("ready"); const payload = decoder.takeResult()!; const node = OwnedUiNode.captureFrom(payload); close(decoder); retire(payload.beginClose());
      type Subscription = import("../../📖️owned/🟦️.tsx").OwnedUiReadSubscription & { readonly lease: InstanceType<typeof OwnedUiNodeReadLease>; readonly notify: () => void };
      const active = new Set<Subscription>(); const closing: Subscription[] = []; const acknowledgements: number[] = [];
      const source: import("../../📖️owned/🟦️.tsx").OwnedUiReadSource = {
        subscribeNode: (id, notify) => { const subscription = { lease: new OwnedUiNodeReadLease(id, 0, node), notify, get snapshot() { return this.lease.snapshot; } }; active.add(subscription); return subscription; },
        acknowledgeRead: (subscription, snapshot) => { for (const value of active) if (value === subscription && value.lease.acknowledge(snapshot)) acknowledgements.push(snapshot.version); },
        unsubscribeNode: subscription => { for (const value of active) if (value === subscription) { active.delete(value); value.lease.beginClose(); closing.push(value); break; } },
      };
      function View({ label }: { label: string }) { const record = useOwnedUiNode(source, fields.node.id); return createElement("span", { "aria-label": label }, record?.component.type === "surface" ? String(record.component.doc.bytes.byteAt(0)) : "pending"); }
      const first = render(createElement(StrictMode, null, createElement(View, { label: "Erste Ansicht" }))); const second = render(createElement(View, { label: "Second view" }));
      expect(first.getByLabelText("Erste Ansicht").textContent).toBe("3"); expect(second.getByLabelText("Second view").textContent).toBe("3"); expect(active.size).toBe(2); expect(closing.length).toBeGreaterThanOrEqual(1);
      for (const subscription of closing.splice(0)) close(subscription.lease);
      const before = acknowledgements.length; await act(async () => { for (const subscription of active) { expect(subscription.lease.offer(1, node)).toBe(true); subscription.notify(); } });
      expect(acknowledgements.slice(before)).toEqual([1,1]); for (const subscription of active) while (subscription.lease.retirementPending) subscription.lease.advanceRetirement(grant);
      first.unmount(); expect(active.size).toBe(1); for (const subscription of closing.splice(0)) close(subscription.lease);
      expect(second.getByLabelText("Second view").textContent).toBe("3"); second.unmount(); expect(active.size).toBe(0); for (const subscription of closing.splice(0)) close(subscription.lease); retire(node.beginClose()); cleanup();
    });

    it("ReadLease source and node replacement reject stale layout acknowledgements while retaining detached owners", async () => {
      const { useOwnedUiNode } = await import("../../📖️owned/🟦️.tsx");
      const { OwnedUiNodeReadLease } = await import("@semio-tech/framework/modules/ui/contract/retained/read-lease");
      const { render, cleanup } = await import("@testing-library/react"); const { createElement } = await import("react");
      type Source = import("../../📖️owned/🟦️.tsx").OwnedUiReadSource;
      type Subscription = import("../../📖️owned/🟦️.tsx").OwnedUiReadSubscription & { readonly lease: InstanceType<typeof OwnedUiNodeReadLease> };
      const owners = () => {
        const active = new Set<Subscription>(); const detached: Subscription[] = []; const accepted: object[] = [];
        const source: Source = {
          subscribeNode: id => { const subscription = { lease: new OwnedUiNodeReadLease(id, 0, null), get snapshot() { return this.lease.snapshot; } }; active.add(subscription); return subscription; },
          acknowledgeRead: (subscription, snapshot) => { for (const value of active) if (value === subscription && value.lease.acknowledge(snapshot)) accepted.push(snapshot); },
          unsubscribeNode: subscription => { for (const value of active) if (value === subscription && active.delete(value)) detached.push(value); },
        };
        return { source, active, detached, accepted };
      };
      const a = owners(); const b = owners();
      function View({ source, id }: { source: Source; id: number }) { useOwnedUiNode(source, id); return createElement("span", null, id); }
      const root = render(createElement(View, { source: a.source, id: 7 })); const old = [...a.active][0]!; const oldToken = old.lease.snapshot;
      root.rerender(createElement(View, { source: a.source, id: 8 })); const replacement = [...a.active][0]!;
      expect(a.detached).toEqual([old]); expect(old.lease.terminalIsEmpty()).toBe(false); expect(replacement.lease.acknowledge(oldToken)).toBe(false);
      const before = a.accepted.length; a.source.acknowledgeRead(old, oldToken); a.source.acknowledgeRead(replacement, oldToken); expect(a.accepted.length).toBe(before);
      const replacementToken = replacement.lease.snapshot; root.rerender(createElement(View, { source: b.source, id: 8 })); const current = [...b.active][0]!;
      expect(a.active.size).toBe(0); expect(a.detached).toEqual([old, replacement]); expect(current.lease.acknowledge(replacementToken)).toBe(false);
      close(old.lease); close(replacement.lease); expect(current.lease.snapshot).not.toBe(replacementToken); expect(current.lease.terminalIsEmpty()).toBe(false);
      root.unmount(); expect(b.active.size).toBe(0); expect(b.detached).toEqual([current]); close(current.lease); cleanup();
    });

    it("ReadPublication exposes all staged reads at one epoch and retains cancelled captures until bounded close", async () => {
      const { OwnedUiNodeReadLease, OwnedUiReadPublication, OwnedUiReadCommit } = await import("@semio-tech/framework/modules/ui/contract/retained/read-lease");
      const fixture = await loadRetainedReadPublicationFixture();
      const publication = new OwnedUiReadPublication(fixture.versions[0]!); const other = new OwnedUiReadPublication(0);
      const a = new OwnedUiNodeReadLease(7, 0, null, publication); const b = new OwnedUiNodeReadLease(8, 0, null, publication);
      const initialA = a.snapshot; const initialB = b.snapshot; const first = publication.begin(1); const foreign = other.begin(1);
      for (const version of fixture.forgedVersions) {
        let staged = false; let forged: typeof first | null = null;
        try { forged = Reflect.construct(OwnedUiReadCommit, [publication, version]); staged = a.stage(forged!, null); } catch { }
        expect(staged).toBe(false); expect(a.hasCapacity).toBe(true); expect(a.snapshot).toBe(initialA); if (forged) expect(publication.publish(forged)).toBe(false);
      }
      expect(() => a.stage(foreign, null)).toThrow(); expect(a.hasCapacity).toBe(true); expect(() => publication.begin(2)).toThrow();
      expect(a.stage(first, null)).toBe(true); expect(a.snapshot).toBe(initialA); expect(b.stage(first, null)).toBe(true); expect(b.snapshot).toBe(initialB);
      expect(publication.publish(first)).toBe(true); expect([a.snapshot.version, b.snapshot.version]).toEqual(produce([1,1], () => {})); expect(publication.cancel(first)).toBe(false);
      expect(a.acknowledge(a.snapshot)).toBe(true); expect(b.acknowledge(b.snapshot)).toBe(true); while (a.retirementPending) a.advanceRetirement(grant); while (b.retirementPending) b.advanceRetirement(grant);
      const stable = a.snapshot; const next = publication.begin(2); expect(a.stage(next, null)).toBe(true); expect(a.snapshot).toBe(stable); expect(publication.cancel(next)).toBe(true); expect(publication.publish(next)).toBe(false);
      expect(a.snapshot).toBe(stable); expect(a.hasCapacity).toBe(false); expect(a.retirementPending).toBe(true); expect(a.advanceRetirement({ maxItems: 0, maxBytes: 4096 }).kind).toBe("blocked");
      while (a.retirementPending) a.advanceRetirement(grant); expect(a.hasCapacity).toBe(true); expect(a.snapshot).toBe(stable);
      const closing = publication.begin(2); expect(a.stage(closing, null)).toBe(true); close(a); expect(publication.cancel(closing)).toBe(true); close(b); expect(other.cancel(foreign)).toBe(true);
    });
  });
  //#endregion 🧾️TypedWireTests
}
