/** 🏅 OwnedWire and SurfaceBytePages suites. */
import { createTypedWireSupport, type TypedWireVitest, type TypedWireDependencies } from "../🪛️typedwire-helpers/🟦️.tsx";
import { loadRetainedWireDecodeFixture, loadWireSurfaceBytesFixture } from "@semio-tech/ui/testing/fixtures";

export async function registerOwnedWireAndBytePagesTests(vitest: TypedWireVitest, dependencies: TypedWireDependencies): Promise<void> {
  const { describe, expect, it } = vitest;
  const { Buffer, produce } = await createTypedWireSupport(vitest, dependencies);
  const { RetainedUiWireValueCursor } = await import("@semio-tech/ui-contract/retained/wire");

  const wireFixture = await loadRetainedWireDecodeFixture();


  //#region 📦️OwnedWireTests
  function wireClose(cursor: InstanceType<typeof RetainedUiWireValueCursor>): void {
    cursor.beginClose();
    for (let n = 0; n < 100_000; n++) {
      const step = cursor.closeStep({ maxItems: 1, maxBytes: 4096 });
      expect(step.items).toBeLessThanOrEqual(1); expect(step.bytes).toBeLessThanOrEqual(4096);
      if (step.kind === "complete") { expect(cursor.terminalIsEmpty()).toBe(true); return; }
    }
    throw new Error("Owned wire retirement failed to terminate");
  }

  function wireReady(cursor: InstanceType<typeof RetainedUiWireValueCursor>): "ready" | "rejected" {
    for (let n = 0; n < 100_000; n++) {
      const step = cursor.advance({ maxItems: 1, maxBytes: 4096 });
      expect(step.items).toBeLessThanOrEqual(1); expect(step.bytes).toBeLessThanOrEqual(4096);
      if (step.kind === "ready" || step.kind === "rejected") return step.kind;
    }
    throw new Error("Owned wire decode failed to terminate");
  }

  describe("OwnedWire", () => {
    it("validates strict neutral native bounds and canonical Rust byte vectors", async () => {
      const { decodePackValue } = await import("@semio-tech/framework/packs");
      for (const vector of wireFixture.vectors) {
        const input = Uint8Array.from(Buffer.from(vector.hex, "hex"));
        const reference = decodePackValue(input);
        const cursor = new RetainedUiWireValueCursor(input);
        expect(input.byteLength).toBe(0);
        expect(cursor.advance({ maxItems: 1, maxBytes: 4096 }).bytes).toBeGreaterThan(0);
        expect(wireReady(cursor)).toBe("ready");
        expect(cursor.value).toEqual(vector.expected); expect(cursor.value).toEqual(reference);
        wireClose(cursor);
      }
    });

    it("rejects malformed canonical framing and values without publishing a partial root", () => {
      for (const vector of wireFixture.hostile) {
        const cursor = new RetainedUiWireValueCursor(Uint8Array.from(Buffer.from(vector.hex, "hex")));
        expect(wireReady(cursor), vector.name).toBe("rejected"); expect(cursor.value).toBeUndefined();
        wireClose(cursor);
      }
    });

    it("owns large nested immutable values and preserves __proto__ as data", async () => {
      const { encodePackValue } = await import("@semio-tech/framework/packs");
      const value = Array.from({ length: wireFixture.large.rows }, (_, id) => ({ id, text: wireFixture.large.text.repeat(wireFixture.large.repeat), nested: { enabled: true } }));
      const input = encodePackValue(value);
      expect(input.length).toBeGreaterThan(4096);
      const cursor = new RetainedUiWireValueCursor(input);
      expect(wireReady(cursor)).toBe("ready"); expect(cursor.value).toEqual(JSON.parse(JSON.stringify(produce(value, () => {}))));
      expect(Object.isFrozen(cursor.value)).toBe(true);
      const result = cursor.value;
      expect(Array.isArray(result)).toBe(true);
      if (Array.isArray(result)) {
        expect(Reflect.set(result, "0", null)).toBe(false);
        expect(Reflect.set(result[0], "text", "forged")).toBe(false);
        expect(Object.isFrozen(result[0].nested)).toBe(true);
      }
      wireClose(cursor);
      const hostileName = new RetainedUiWireValueCursor(encodePackValue(JSON.parse('{"__proto__":{"safe":true}}')));
      expect(wireReady(hostileName)).toBe("ready"); expect(Object.hasOwn(hostileName.value as object, "__proto__")).toBe(true);
      expect(Object.getPrototypeOf(hostileName.value)).toBe(Object.prototype);
      wireClose(hostileName);
    });

    it("cancels at every decoding phase and rejects shared or aliased input ownership", async () => {
      const { encodePackValue } = await import("@semio-tech/framework/packs");
      const value = { a: [false, 42, { deep: "unicode😀" }], b: null };
      const phases = new Set<string>();
      for (let stop = 0; stop < 300; stop++) {
        const cursor = new RetainedUiWireValueCursor(encodePackValue(value));
        expect(cursor.advance({ maxItems: 0, maxBytes: 4096 }).kind).toBe("blocked");
        let ready = false;
        for (let n = 0; n < stop; n++) { const step = cursor.advance({ maxItems: 1, maxBytes: 4096 }); phases.add(step.phase); if (step.kind === "ready") { ready = true; break; } }
        wireClose(cursor); expect(cursor.value).toBeUndefined();
        if (ready) break;
      }
      expect(phases.size).toBeGreaterThanOrEqual(7);
      const shared = new Uint8Array(new SharedArrayBuffer(8));
      expect(() => new RetainedUiWireValueCursor(shared)).toThrow(/ownership/);
      const entire = new Uint8Array(32); const subview = entire.subarray(4, 12);
      expect(() => new RetainedUiWireValueCursor(subview)).toThrow(/ownership/); expect(entire.byteLength).toBe(32);
      const memory = new WebAssembly.Memory({ initial: 1 });
      expect(() => new RetainedUiWireValueCursor(new Uint8Array(memory.buffer))).toThrow(/ownership/); expect(memory.buffer.byteLength).toBe(65536);
      const bytes = encodePackValue(value); const alias = new Uint8Array(bytes.buffer);
      const first = new RetainedUiWireValueCursor(bytes);
      expect(alias.byteLength).toBe(0);
      expect(() => new RetainedUiWireValueCursor(alias)).toThrow(/ownership/);
      wireClose(first);
    });

    it("links admitted scalar and collection bounds to the owning native schema", async () => {
      const { encodePackValue } = await import("@semio-tech/framework/packs");
      const { readFileSync } = await import("node:fs");
      const { resolve } = await import("node:path");
      const action = readFileSync(resolve(process.cwd(), "../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/🎬️action.rs"), "utf8");
      expect(action).toContain(`UI_TEXT_MAX_BYTES: usize = ${wireFixture.nativeBounds.textBytes};`);
      expect(action).toContain(`UI_VALUE_MAX_ITEMS: usize = ${wireFixture.nativeBounds.collectionItems};`);
      for (const value of ["x".repeat(512), Array.from({ length: 256 }, (_, i) => i)]) {
        const cursor = new RetainedUiWireValueCursor(encodePackValue(value));
        expect(cursor.advance({ maxItems: 1, maxBytes: 4095 }).kind).toBe("blocked");
        expect(wireReady(cursor)).toBe("ready"); expect(cursor.value).toEqual(value);
        cursor.beginClose(); expect(cursor.closeStep({ maxItems: 1, maxBytes: 4095 }).kind).toBe("blocked"); wireClose(cursor);
      }
      const overflow = new RetainedUiWireValueCursor(encodePackValue("x".repeat(513)));
      expect(wireReady(overflow)).toBe("rejected"); wireClose(overflow);
    });
  });
  //#endregion 📦️OwnedWireTests


  //#region 📦️SurfaceBytePagesTests
  describe("SurfaceBytePages", () => {
    it("matches Node Buffer at every native page boundary and preserves exact captured owners", async () => {
      const { UiSurfaceByteBuilder } = await import("@semio-tech/ui-contract/retained/wire/bytes");
      const fixture = await loadWireSurfaceBytesFixture();
      for (const length of fixture.lengths) {
        const expected = Buffer.from(Array.from({ length }, (_, i) => (i * fixture.pattern.multiply + fixture.pattern.add) % fixture.pattern.modulo));
        const builder = new UiSurfaceByteBuilder(length);
        expect(builder.advance({ maxItems: 1, maxBytes: 4095 }, 3).kind).toBe("blocked");
        let offset = 0;
        for (let turn = 0; turn < 100_000; turn++) {
          const step = builder.advance({ maxItems: 1, maxBytes: 4096 }, expected[offset]);
          expect(step.bytes).toBeLessThanOrEqual(4096); expect(step.items).toBeLessThanOrEqual(1);
          if (step.accepted) offset++;
          if (step.kind === "ready") break;
        }
        expect(offset).toBe(length);
        const value = builder.takeResult(); expect(value).not.toBeNull(); expect(builder.terminalIsEmpty()).toBe(true);
        const captured = value!.capture(); const retirement = value!.beginClose();
        expect(value!.terminalIsEmpty()).toBe(true); expect(() => value!.byteAt(0)).toThrow(/closed/);
        expect(retirement.advance({ maxItems: 1, maxBytes: 4096 }).kind).toBe("complete");
        expect(Buffer.from(Array.from({ length }, (_, index) => captured.byteAt(index)))).toEqual(expected);
        const final = captured.beginClose(); let released = 0;
        for (let turn = 0; turn < 1000; turn++) {
          const step = final.advance({ maxItems: 1, maxBytes: 4096 }); released += step.bytes;
          expect(step.bytes).toBeLessThanOrEqual(4096); expect(step.items).toBeLessThanOrEqual(1);
          if (step.kind === "complete") break;
        }
        expect(final.terminalIsEmpty()).toBe(true); expect(released).toBeGreaterThanOrEqual(length);
      }
      for (const length of fixture.invalidLengths) expect(() => new UiSurfaceByteBuilder(length)).toThrow(RangeError);
    });

    it("retires cancellation before allocation, between pages, and after preparation without transferring partial bytes", async () => {
      const { UiSurfaceByteBuilder } = await import("@semio-tech/ui-contract/retained/wire/bytes");
      for (const stop of [0, 1, 2, 3, 255, 256, 257, 258, 260, 600]) {
        const builder = new UiSurfaceByteBuilder(513);
        for (let n = 0; n < stop; n++) builder.advance({ maxItems: 1, maxBytes: 4096 }, n % 256);
        const close = builder.beginClose(); expect(builder.takeResult()).toBeNull();
        expect(close.advance({ maxItems: 0, maxBytes: 4096 }).kind).toBe("blocked");
        for (let n = 0; n < 100; n++) if (close.advance({ maxItems: 1, maxBytes: 4096 }).kind === "complete") break;
        expect(close.terminalIsEmpty()).toBe(true); expect(builder.terminalIsEmpty()).toBe(true);
      }
      const invalid = new UiSurfaceByteBuilder(1);
      expect(invalid.advance({ maxItems: 1, maxBytes: 4096 }, 256).kind).toBe("rejected");
      expect(invalid.takeResult()).toBeNull(); expect(invalid.beginClose().advance({ maxItems: 1, maxBytes: 4096 }).kind).toBe("complete");
    });

    it("decodes the full native SurfaceDoc numeric-list wire into pages without changing ordinary UiValue bounds", async () => {
      const { UiSurfaceBytes } = await import("@semio-tech/ui-contract/retained/wire/bytes");
      const fixture = await loadWireSurfaceBytesFixture();
      const { encodePackValue } = await import("@semio-tech/framework/packs");
      const expected = Buffer.from(Array.from({ length: fixture.maximumBytes }, (_, i) => (i * fixture.pattern.multiply + fixture.pattern.add) % fixture.pattern.modulo));
      const component = { type: "surface", kind: "node-graph", docSchema: "node-graph@1", doc: { bytes: Array.from(expected) }, bindings: [] };
      const denied = new RetainedUiWireValueCursor(encodePackValue(component));
      expect(wireReady(denied)).toBe("rejected"); wireClose(denied);
      for (const profile of ["component", "node"] as const) {
        const cursor = new RetainedUiWireValueCursor(encodePackValue(profile === "node" ? { component } : component), profile);
        expect(wireReady(cursor), cursor.failure ?? "").toBe("ready");
        const decoded = cursor.value;
        if (!decoded || typeof decoded !== "object") throw new Error("Expected owned component record");
        const resolved: unknown = profile === "node" ? Reflect.get(decoded, "component") : decoded;
        if (!resolved || typeof resolved !== "object") throw new Error("Expected component");
        const document: unknown = Reflect.get(resolved, "doc");
        if (!document || typeof document !== "object") throw new Error("Expected SurfaceDoc");
        const bytes: unknown = Reflect.get(document, "bytes");
        expect(bytes).toBeInstanceOf(UiSurfaceBytes);
        if (!(bytes instanceof UiSurfaceBytes)) throw new Error("Expected paged bytes");
        const capture = bytes.capture(); wireClose(cursor);
        expect(bytes.terminalIsEmpty()).toBe(true);
        expect(Buffer.from(Array.from({ length: capture.length }, (_, i) => capture.byteAt(i)))).toEqual(expected);
        const close = capture.beginClose();
        for (let n = 0; n < 1000; n++) if (close.advance({ maxItems: 1, maxBytes: 4096 }).kind === "complete") break;
        expect(close.terminalIsEmpty()).toBe(true);
      }
      for (const bytes of [[256], [false], [1.5], Array.from({ length: fixture.maximumBytes + 1 }, () => 0)]) {
        const cursor = new RetainedUiWireValueCursor(encodePackValue({ doc: { bytes }, type: "surface" }), "component");
        expect(wireReady(cursor)).toBe("rejected"); wireClose(cursor);
      }
    });
  });
  //#endregion 📦️SurfaceBytePagesTests
}
