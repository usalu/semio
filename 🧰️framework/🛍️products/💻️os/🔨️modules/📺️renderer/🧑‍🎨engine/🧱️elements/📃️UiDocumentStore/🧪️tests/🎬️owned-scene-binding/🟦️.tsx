/** 🎬 Owned scene preparation, typed scene, scene binding and owned surface scene laws. */
import { exactSchemaDefinitionV1, grant, createTypedWireSupport, type TypedWireVitest, type TypedWireDependencies } from "../🪛️typedwire-helpers/🟦️.tsx";
import { loadRetainedOwnedSceneFixtureText, loadRetainedOwnedSceneFixture, loadRetainedTypedSceneFixture, loadSceneTypedCatalogDocument, loadRetainedSceneTypedSchema, loadSceneTextBytesFixture, loadRetainedSceneJsonFixture, loadSceneJsonDocumentFixture, loadSceneJsonStringFixture, loadScenePackFieldFixture, loadSceneGenericPackFixture, loadRetainedSceneBindingFixture, loadRetainedWireFieldsFixture, loadRetainedSceneNumericFixture } from "@semio-tech/ui/testing/fixtures";

export async function registerOwnedSceneTests(vitest: TypedWireVitest, dependencies: TypedWireDependencies): Promise<void> {
  const { describe, expect, it, vi } = vitest;
  const { DEFAULT_UI_DOCUMENT_LIMITS } = dependencies;
  const { Ajv, close, finish, produce, retire } = await createTypedWireSupport(vitest, dependencies);

  //#region 🧾️TypedWireTests
  describe("TypedWire", () => {
    //#region 🎬️OwnedSceneTests
    it("OwnedScene prepares native tag vectors and long Unicode slices with strict neutral and Node oracles", async () => {
      const { OwnedUiSceneCursor } = await import("@semio-tech/ui-contract/retained/scene");
      const { RetainedUiTypedCursor } = await import("@semio-tech/ui-contract/retained/wire/typed");
      const fixture = await loadRetainedOwnedSceneFixture();
      const rawFixture = await loadRetainedOwnedSceneFixtureText();
      const { encodePackValue, decodeScenePackValue } = await import("@semio-tech/framework/packs");
      const { Buffer } = await import("node:buffer");
      const exactFixture: typeof fixture = JSON.parse(rawFixture);
      const prepare = (bytes: Uint8Array) => {
        const typed = new RetainedUiTypedCursor(encodePackValue({ type: "surface", kind: "canvas-2d", docSchema: "canvas2d@1", doc: { bytes: Array.from(bytes) }, bindings: [] }), "component");
        expect(finish(typed)).toBe("ready"); const input = typed.takeResult()!; close(typed);
        const scene = new OwnedUiSceneCursor(input); retire(input.beginClose()); expect(finish(scene)).toBe("ready"); const document = scene.takeResult()!; close(scene); return document;
      };
      type Document = ReturnType<typeof prepare>;
      const lookup = (document: Document, id: number) => {
        const reader = document.beginRead(id); let value: Extract<ReturnType<typeof reader.advance>, { kind: "value" }>["value"] | undefined;
        for (;;) { const current = reader.advance(grant); if (current.kind === "value") value = current.value; if (current.kind === "complete") break; }
        retire(reader.beginClose()); if (!value) throw new Error("Missing scene record"); return value;
      };
      const text = (document: Document, id: number): string => {
        const reader = document.beginText(id); const chunks: string[] = [];
        for (;;) { const current = reader.advance(grant); if (current.kind === "text") chunks.push(current.value); if (current.kind === "complete") break; }
        retire(reader.beginClose()); return chunks.join("");
      };
      const materialize = (document: Document, id = 0): unknown => {
        const node = lookup(document, id);
        if (node.kind === "unit" || node.kind === "none") return null;
        if (node.kind === "boolean" || node.kind === "float" || node.kind === "char") return node.value;
        if (node.kind === "integer") return Number(node.value);
        if (node.kind === "text") return text(document, id);
        if (node.kind === "bytes") {
          const reader = document.beginBytes(id); const bytes: number[] = [];
          for (;;) { const current = reader.advance(grant); if (current.kind === "bytes") bytes.push(...current.value); if (current.kind === "complete") break; }
          retire(reader.beginClose()); return bytes;
        }
        if (node.kind === "some") return materialize(document, node.first);
        if (node.kind === "variant") { const name = text(document, node.first); const payload = lookup(document, node.first).end; return lookup(document, payload).kind === "unit" ? name : { [name]: materialize(document, payload) }; }
        if (node.kind === "sequence") { const values: unknown[] = []; let child = node.first; for (let i = 0; i < node.count; i++) { values.push(materialize(document, child)); child = lookup(document, child).end; } return values; }
        if (node.kind !== "map") throw new Error("Unknown scene oracle kind");
        const object: Record<string, unknown> = {}; let child = node.first;
        for (let i = 0; i < node.count; i++) { const key = text(document, child); child = lookup(document, child).end; Object.defineProperty(object, key, { value: materialize(document, child), enumerable: true }); child = lookup(document, child).end; } return object;
      };
      for (const vector of exactFixture.cases) {
        const bytes = Buffer.from(vector.hex, "hex"); const document = prepare(bytes); const actual = materialize(document);
        if (vector.name === "prototype-key") { expect(actual).toEqual(vector.value); expect(Object.getPrototypeOf(actual)).toBe(Object.prototype); }
        else expect(actual, vector.name).toEqual(produce(vector.value, () => {}));
        if (vector.name !== "bom-preserved" && vector.name !== "prototype-key") expect(actual, vector.name).toEqual(decodeScenePackValue(bytes));
        if (vector.name === "double") expect(actual).toBe(bytes.readDoubleLE(1));
        if (vector.name === "unicode" || vector.name === "bom-preserved") expect(actual).toBe(new TextDecoder("utf-8", { fatal: true, ignoreBOM: true }).decode(bytes.subarray(2)));
        retire(document.beginClose());
      }
      const textValue = fixture.large.unit.repeat(fixture.large.repeat); const raw = Buffer.from(textValue); const header: number[] = [6]; let length = raw.length;
      do { const byte = length & 127; length = Math.floor(length / 128); header.push(byte | (length ? 128 : 0)); } while (length);
      const document = prepare(Buffer.concat([Buffer.from(header), raw])); const reader = document.beginText(0); const captured = document.capture(); retire(document.beginClose());
      let reconstructed = ""; for (;;) { const current = reader.advance(grant); expect(current.bytes).toBeLessThanOrEqual(4096); if (current.kind === "text") { expect(current.value.length).toBeLessThanOrEqual(256); reconstructed += current.value; } if (current.kind === "complete") break; }
      expect(reconstructed).toBe(raw.toString("utf8")); retire(captured.beginClose()); retire(reader.beginClose());
    });

    it("OwnedScene rejects malformed native packets and forged or rebound scene ownership without retiring the source", async () => {
      const { OwnedUiSceneCursor, OwnedUiSceneDocument, OwnedUiSceneReader, OwnedUiSceneRetirement } = await import("@semio-tech/ui-contract/retained/scene");
      const { RetainedUiTypedCursor } = await import("@semio-tech/ui-contract/retained/wire/typed");
      const fixture = await loadRetainedOwnedSceneFixture();
      const { encodePackValue } = await import("@semio-tech/framework/packs"); const { Buffer } = await import("node:buffer");
      const source = (bytes: Uint8Array) => { const typed = new RetainedUiTypedCursor(encodePackValue({ type: "surface", kind: "canvas-2d", docSchema: "canvas2d@1", doc: { bytes: Array.from(bytes) }, bindings: [] }), "component"); expect(finish(typed)).toBe("ready"); const input = typed.takeResult()!; close(typed); return input; };
      for (const vector of fixture.hostile) {
        const input = source(Buffer.from(vector.hex, "hex")); const scene = new OwnedUiSceneCursor(input);
        expect(finish(scene), vector.name).toBe("rejected"); expect(scene.takeResult()).toBeNull(); expect(scene.failure).not.toBeNull(); close(scene);
        if (input.value.type !== "surface") throw new Error("Expected source component"); expect(input.value.doc.bytes.length).toBe(vector.hex.length / 2); retire(input.beginClose());
      }
      const style = new RetainedUiTypedCursor(encodePackValue({}), "style"); expect(finish(style)).toBe("ready"); const wrong = style.takeResult()!; close(style);
      expect(() => Reflect.construct(OwnedUiSceneCursor, [wrong])).toThrow(); expect(wrong.terminalIsEmpty()).toBe(false); retire(wrong.beginClose());
      const input = source(new Uint8Array([2]));
      for (const constructor of [OwnedUiSceneDocument, OwnedUiSceneReader, OwnedUiSceneRetirement]) expect(() => Reflect.construct(constructor, [{ references: 1, source: input, index: {} }])).toThrow();
      expect(input.terminalIsEmpty()).toBe(false); retire(input.beginClose());
    });

    it("OwnedScene cancellation owns every frame and reader across exact source lifetimes and maximum page envelope", async () => {
      const { OwnedUiSceneCursor } = await import("@semio-tech/ui-contract/retained/scene");
      const { RetainedUiTypedCursor } = await import("@semio-tech/ui-contract/retained/wire/typed");
      const fixture = await loadRetainedOwnedSceneFixture();
      const { encodePackValue } = await import("@semio-tech/framework/packs"); const { Buffer } = await import("node:buffer");
      const source = (bytes: Uint8Array) => { const typed = new RetainedUiTypedCursor(encodePackValue({ type: "surface", kind: "canvas-2d", docSchema: "canvas2d@1", doc: { bytes: Array.from(bytes) }, bindings: [] }), "component"); expect(finish(typed)).toBe("ready"); const input = typed.takeResult()!; close(typed); return input; };
      const raw = Buffer.from(fixture.cases.find(vector => vector.name === "nested-map")!.hex, "hex"); const referenceInput = source(raw); const reference = new OwnedUiSceneCursor(referenceInput); const phases = new Map<string, number>(); let count = 0;
      for (;;) { const current = reference.advance(grant); count++; phases.set(current.phase, count); if (current.kind === "ready") break; if (current.kind === "rejected") throw new Error(reference.failure!); }
      close(reference); retire(referenceInput.beginClose()); expect(count).toBeGreaterThan(100);
      const cutoffs = new Set([0, 1, count, ...phases.values()]);
      for (const cutoff of cutoffs) {
        const input = source(raw); if (input.value.type !== "surface") throw new Error("Expected surface"); const view = input.value.doc.bytes; const cursor = new OwnedUiSceneCursor(input); retire(input.beginClose());
        for (let index = 0; index < cutoff; index++) cursor.advance(grant);
        const progress = cursor.completedBytes; expect(cursor.advance({ maxItems: 0, maxBytes: 4096 }).kind).toBe("blocked"); expect(cursor.completedBytes).toBe(progress);
        cursor.beginClose(); expect(cursor.takeResult()).toBeNull(); expect(cursor.closeStep({ maxItems: 1, maxBytes: 4095 }).kind).toBe("blocked"); expect(view.byteAt(0)).toBe(raw[0]); close(cursor); expect(() => view.byteAt(0)).toThrow();
      }
      const prefix = Buffer.from([6, 0xfc, 0xff, 1]); const bytes = Buffer.concat([prefix, Buffer.alloc(fixture.large.maximumBytes - prefix.length, 97)]); expect(bytes.length).toBe(32768);
      const input = source(bytes); if (input.value.type !== "surface") throw new Error("Expected surface"); const view = input.value.doc.bytes; const cursor = new OwnedUiSceneCursor(input); retire(input.beginClose()); expect(finish(cursor)).toBe("ready"); const document = cursor.takeResult()!; close(cursor);
      const reader = document.beginText(0); const other = document.beginText(0); retire(document.beginClose()); let length = 0;
      for (;;) { const current = reader.advance(grant); if (current.kind === "text") { expect(current.value).toBe("a".repeat(current.value.length)); length += current.value.length; } if (current.kind === "complete") break; }
      expect(length).toBe(32764); retire(reader.beginClose()); expect(view.byteAt(0)).toBe(6); retire(other.beginClose()); expect(() => view.byteAt(0)).toThrow();
      const deep = source(Buffer.concat([Buffer.alloc(fixture.large.depth, 9), Buffer.from([0])])); const parser = new OwnedUiSceneCursor(deep); retire(deep.beginClose()); let prepared = false;
      const records = fixture.large.depth + 1; const height = 2 * Math.ceil(Math.log2(records + 1)); const allocationNodes = 3; const referenceSlots = allocationNodes * 3;
      const allocationTurns = referenceSlots * (referenceSlots + 1) + referenceSlots + allocationNodes * 2 + referenceSlots + 4;
      const recordTurns = 2 * (height + 1) * (allocationTurns + 16) + 9 * height + 64;
      const maximumTurns = records * recordTurns + records * 4;
      let previousBytes = 0; let previousRecords = 0; let stalled = 0;
      for (let index = 0; index < maximumTurns; index++) {
        const current = parser.advance(grant); const bytes = parser.completedBytes; const completed = parser.completedRecords;
        if (current.items > 1 || current.bytes > 4096) throw new Error("Scene step exceeded its actual grant");
        if (bytes < previousBytes || completed < previousRecords) throw new Error("Scene preparation progress regressed");
        stalled = bytes > previousBytes || completed > previousRecords ? 0 : stalled + 1; if (stalled > recordTurns) throw new Error("Scene record exceeded derived dual-AVL continuation bound");
        previousBytes = bytes; previousRecords = completed;
        if (current.kind === "rejected") throw new Error(parser.failure!); if (current.kind === "ready") { prepared = true; break; }
      }
      expect(prepared, JSON.stringify({ bytes: parser.completedBytes, records: parser.completedRecords, expected: fixture.large.depth + 1 })).toBe(true); const nested = parser.takeResult()!; close(parser); expect(nested.size).toBe(fixture.large.depth + 1); retire(nested.beginClose());
    });

    it("OwnedScene cancels every small-packet prefix and rejects long duplicate keys after real incremental comparison", async () => {
      const { OwnedUiSceneCursor } = await import("@semio-tech/ui-contract/retained/scene");
      const { RetainedUiTypedCursor } = await import("@semio-tech/ui-contract/retained/wire/typed");
      const fixture = await loadRetainedOwnedSceneFixture();
      const { encodePackValue } = await import("@semio-tech/framework/packs"); const { Buffer } = await import("node:buffer");
      const source = (bytes: Uint8Array) => { const typed = new RetainedUiTypedCursor(encodePackValue({ type: "surface", kind: "canvas-2d", docSchema: "canvas2d@1", doc: { bytes: Array.from(bytes) }, bindings: [] }), "component"); expect(finish(typed)).toBe("ready"); const input = typed.takeResult()!; close(typed); return input; };
      const original = source(Buffer.from([9, 2])); const reference = new OwnedUiSceneCursor(original); let count = 0;
      for (;;) { count++; if (reference.advance(grant).kind === "ready") break; } close(reference);
      for (let cutoff = 0; cutoff <= count; cutoff++) { const cursor = new OwnedUiSceneCursor(original); for (let index = 0; index < cutoff; index++) cursor.advance(grant); close(cursor); expect(original.terminalIsEmpty()).toBe(false); }
      const copy = source(Buffer.from([9, 2])); if (copy.value.type !== "surface") throw new Error("Expected surface"); const copiedBytes = copy.value.doc.bytes; retire(original.beginClose()); expect(copiedBytes.byteAt(0)).toBe(9); retire(copy.beginClose()); expect(() => copiedBytes.byteAt(0)).toThrow();
      const bytes = Buffer.from(fixture.large.unit.repeat(fixture.large.repeat)); const prefix: number[] = [6]; let length = bytes.length;
      do { const byte = length & 127; length = Math.floor(length / 128); prefix.push(byte | (length ? 128 : 0)); } while (length);
      const input = source(Buffer.concat([Buffer.from([13, 2]), Buffer.from(prefix), bytes, Buffer.from([1]), Buffer.from(prefix), bytes, Buffer.from([2])]));
      const cursor = new OwnedUiSceneCursor(input); let compared = 0;
      for (;;) { const current = cursor.advance(grant); if (current.phase === "scene-key-compare" && current.bytes === 2) compared++; if (current.kind === "rejected") break; if (current.kind === "ready") throw new Error("Duplicate key was admitted"); }
      expect(compared).toBe(bytes.length); expect(cursor.failure).toBe("Duplicate scene map key"); close(cursor);
      const cancelled = new OwnedUiSceneCursor(input); for (;;) { const current = cancelled.advance(grant); if (current.phase === "scene-key-compare" && current.bytes === 2) break; } retire(input.beginClose()); close(cancelled);
      const full = source(Buffer.from("03ffffffffffffffffff01", "hex")); const parser = new OwnedUiSceneCursor(full); retire(full.beginClose()); expect(finish(parser)).toBe("ready"); const document = parser.takeResult()!; close(parser); const reader = document.beginRead(); let actual = 0n;
      for (;;) { const current = reader.advance(grant); if (current.kind === "value" && current.value.kind === "integer") actual = current.value.value; if (current.kind === "complete") break; }
      expect(actual).toBe(Buffer.from("ffffffffffffffff", "hex").readBigUInt64LE()); retire(document.beginClose()); retire(reader.beginClose());
    });

    it("TypedScene validates all fifteen native schema roots and nested records without reconstructing source strings", async () => {
      const { OwnedUiSceneProjectionCursor } = await import("@semio-tech/ui-contract/retained/scene/typed");
      const { OwnedUiSceneCursor } = await import("@semio-tech/ui-contract/retained/scene");
      const { RetainedUiTypedCursor } = await import("@semio-tech/ui-contract/retained/wire/typed");
      const fixture = await loadRetainedTypedSceneFixture();
      const catalog = await loadSceneTypedCatalogDocument();
      const typedSceneSchemaModule = await loadRetainedSceneTypedSchema(); const catalogSchema = exactSchemaDefinitionV1(typedSceneSchemaModule, "TypedScene");
      const { encodePackValue } = await import("@semio-tech/framework/packs"); const { Buffer } = await import("node:buffer");
      expect(new Ajv({ strict: true, allErrors: true }).compile(catalogSchema)(catalog)).toBe(true);
      const size = (value: number): number[] => { const result: number[] = []; do { const byte = value & 127; value = Math.floor(value / 128); result.push(byte | (value ? 128 : 0)); } while (value); return result; };
      const encode = (value: unknown): Uint8Array => {
        if (value === null) return Buffer.from([8]); if (typeof value === "boolean") return Buffer.from([value ? 2 : 1]);
        if (typeof value === "number") { const bytes = Buffer.alloc(9); bytes[0] = 5; bytes.writeDoubleLE(value, 1); return bytes; }
        if (typeof value === "string") { const bytes = Buffer.from(value); return Buffer.concat([Buffer.from([6, ...size(bytes.length)]), bytes]); }
        if (Array.isArray(value)) return Buffer.concat([Buffer.from([10, ...size(value.length)]), ...value.map(encode)]);
        if (typeof value !== "object" || !value) throw new Error("Invalid neutral scene fixture");
        if (Object.hasOwn(value, "$some")) return Buffer.concat([Buffer.from([9]), encode(Reflect.get(value, "$some"))]);
        const entries = Object.entries(value); return Buffer.concat([Buffer.from([13, ...size(entries.length)]), ...entries.flatMap(([key, child]) => [encode(key), encode(child)])]);
      };
      const prepared = (vector: { kind: string; schema: string; value: unknown }) => {
        const wire = new RetainedUiTypedCursor(encodePackValue({ type: "surface", kind: vector.kind, docSchema: vector.schema, doc: { bytes: Array.from(encode(vector.value)) }, bindings: [] }), "component"); expect(finish(wire)).toBe("ready"); const input = wire.takeResult()!; close(wire);
        const parser = new OwnedUiSceneCursor(input); retire(input.beginClose()); expect(finish(parser)).toBe("ready"); const document = parser.takeResult()!; close(parser);
        const cursor = new OwnedUiSceneProjectionCursor(document, { usizeBits: 64 }); retire(document.beginClose()); return cursor;
      };
      expect(fixture.cases.map(value => value.kind)).toEqual(produce(catalog.surfaces.map(value => value.kind), () => {}));
      for (const vector of fixture.cases) {
        const cursor = prepared(vector); expect(finish(cursor), vector.kind).toBe("ready"); const document = cursor.takeResult()!; close(cursor); expect(document.kind).toBe(vector.kind); expect(document.schema).toBe(vector.schema);
        const reader = document.beginRecord(); retire(document.beginClose()); let fields: readonly { readonly name: string; readonly source: number | null }[] = [];
        for (;;) { const current = reader.advance(grant); if (current.kind === "value") fields = current.value.fields; if (current.kind === "complete") break; }
        expect(fields.length).toBeGreaterThan(0); expect(fields.length).toBeLessThanOrEqual(32); retire(reader.beginClose());
      }
      for (const vector of fixture.hostile) { const cursor = prepared(vector); expect(finish(cursor), `${vector.kind}/${vector.schema}`).toBe("rejected"); expect(cursor.failure).not.toBeNull(); expect(cursor.takeResult()).toBeNull(); close(cursor); }
    });

    it("TypedScene cancels every observed projection prefix while retaining the exact source and rejecting forged owners", async () => {
      const typedScene = await import("@semio-tech/ui-contract/retained/scene/typed");
      const { OwnedUiSceneCursor } = await import("@semio-tech/ui-contract/retained/scene");
      const { RetainedUiTypedCursor } = await import("@semio-tech/ui-contract/retained/wire/typed");
      const fixture = await loadRetainedTypedSceneFixture();
      const { encodePackValue } = await import("@semio-tech/framework/packs"); const { Buffer } = await import("node:buffer");
      const packet = Buffer.concat([Buffer.from([13, 1, 6, 6]), Buffer.from("buffer"), Buffer.from([6, 0])]);
      const wire = new RetainedUiTypedCursor(encodePackValue({ type: "surface", kind: "text-editor", docSchema: "text-editor@1", doc: { bytes: Array.from(packet) }, bindings: [] }), "component"); expect(finish(wire)).toBe("ready"); const input = wire.takeResult()!; close(wire);
      const parser = new OwnedUiSceneCursor(input); retire(input.beginClose()); expect(finish(parser)).toBe("ready"); const source = parser.takeResult()!; close(parser);
      const observed = new typedScene.OwnedUiSceneProjectionCursor(source, { usizeBits: 64 }); let count = 0; const phases = new Set<string>();
      for (;;) { const current = observed.advance(grant); count++; phases.add(current.phase); expect(current.items).toBeLessThanOrEqual(fixture.lifecycle.maxItems); expect(current.bytes).toBeLessThanOrEqual(fixture.lifecycle.maxBytes); if (current.kind === "ready") break; expect(current.kind).toBe("pending"); }
      close(observed); expect(phases.has("scene-typed-record")).toBe(true); expect(count).toBeGreaterThan(100);
      for (let cutoff = 0; cutoff <= count; cutoff++) {
        const cursor = new typedScene.OwnedUiSceneProjectionCursor(source, { usizeBits: 64 }); expect(cursor.advance({ maxItems: 0, maxBytes: 4096 }).kind).toBe("blocked"); expect(cursor.advance({ maxItems: 1, maxBytes: 4095 }).kind).toBe("blocked");
        for (let index = 0; index < cutoff; index++) cursor.advance(grant);
        cursor.beginClose(); expect(cursor.takeResult()).toBeNull(); expect(cursor.closeStep({ maxItems: 1, maxBytes: 0 }).kind).toBe("blocked"); close(cursor); expect(source.terminalIsEmpty()).toBe(false);
      }
      for (const constructor of [typedScene.OwnedUiPreparedScene, typedScene.OwnedUiPreparedSceneReader, typedScene.OwnedUiPreparedSceneRetirement]) expect(() => Reflect.construct(constructor, [{}, {}, null])).toThrow(/authority/);
      const final = new typedScene.OwnedUiSceneProjectionCursor(source, { usizeBits: 64 }); retire(source.beginClose()); expect(finish(final)).toBe("ready"); const prepared = final.takeResult()!; close(final); expect(() => Object.defineProperty(prepared, "schema", { value: "forged@1" })).toThrow(); retire(prepared.beginClose());
    });

    it("TypedScene retains long Unicode field slices across independent prepared and raw readers without rebinding source pages", async () => {
      const { OwnedUiSceneProjectionCursor } = await import("@semio-tech/ui-contract/retained/scene/typed");
      const { OwnedUiSceneCursor } = await import("@semio-tech/ui-contract/retained/scene");
      const { RetainedUiTypedCursor } = await import("@semio-tech/ui-contract/retained/wire/typed");
      const fixture = await loadRetainedTypedSceneFixture();
      const { encodePackValue } = await import("@semio-tech/framework/packs"); const { Buffer } = await import("node:buffer");
      const text = fixture.lifecycle.text.repeat(fixture.lifecycle.repeat); const bytes = Buffer.from(text); const prefix: number[] = [13, 1, 6, 6, ...Buffer.from("buffer"), 6]; let remaining = bytes.length;
      do { const byte = remaining & 127; remaining = Math.floor(remaining / 128); prefix.push(byte | (remaining ? 128 : 0)); } while (remaining);
      const create = () => {
        const packet = Buffer.concat([Buffer.from(prefix), bytes]); const wire = new RetainedUiTypedCursor(encodePackValue({ type: "surface", kind: "text-editor", docSchema: "text-editor@1", doc: { bytes: Array.from(packet) }, bindings: [] }), "component"); expect(finish(wire)).toBe("ready"); const input = wire.takeResult()!; close(wire); if (input.value.type !== "surface") throw new Error("Expected surface"); const pages = input.value.doc.bytes;
        const parser = new OwnedUiSceneCursor(input); retire(input.beginClose()); expect(finish(parser)).toBe("ready"); const raw = parser.takeResult()!; close(parser); const cursor = new OwnedUiSceneProjectionCursor(raw, { usizeBits: 64 }); retire(raw.beginClose()); expect(finish(cursor)).toBe("ready"); const prepared = cursor.takeResult()!; close(cursor); return { prepared, pages };
      };
      const first = create(); const equal = create(); const record = first.prepared.beginRecord(); let buffer: number | null = null;
      for (;;) { const current = record.advance(grant); if (current.kind === "value") { const field = current.value.fields.find(value => value.name === "buffer"); expect(field?.literal).toBeNull(); buffer = field?.source ?? null; } if (current.kind === "complete") break; }
      if (buffer === null) throw new Error("Expected a sliced buffer field");
      const reader = first.prepared.beginText(buffer); const alias = first.prepared.capture(); const second = alias.beginText(buffer); retire(first.prepared.beginClose()); retire(alias.beginClose()); retire(record.beginClose()); retire(equal.prepared.beginClose()); expect(() => equal.pages.byteAt(0)).toThrow(); expect(first.pages.byteAt(0)).toBe(13);
      const chunks: string[] = []; for (;;) { const current = reader.advance(grant); if (current.kind === "text") { expect(Buffer.byteLength(current.value)).toBeLessThanOrEqual(fixture.lifecycle.maximumEmittedTextBytes); chunks.push(current.value); } expect(current.bytes).toBeLessThanOrEqual(4096); if (current.kind === "complete") break; }
      expect(chunks.join("")).toBe(new TextDecoder("utf-8", { fatal: true, ignoreBOM: true }).decode(bytes)); expect(chunks.length).toBeGreaterThan(100); retire(reader.beginClose()); expect(first.pages.byteAt(0)).toBe(13);
      expect(second.advance(grant).kind).not.toBe("rejected"); retire(second.beginClose()); expect(() => first.pages.byteAt(0)).toThrow();
    });

    it("SceneTextBytes reads exact owned UTF-8 ranges without whole-text conversion and retires cancelled readers", async () => {
      const { OwnedUiSceneProjectionCursor } = await import("@semio-tech/ui-contract/retained/scene/typed"); const { OwnedUiSceneCursor } = await import("@semio-tech/ui-contract/retained/scene"); const { RetainedUiTypedCursor } = await import("@semio-tech/ui-contract/retained/wire/typed"); const { encodePackValue } = await import("@semio-tech/framework/packs"); const { Buffer } = await import("node:buffer");
      const fixture = await loadSceneTextBytesFixture(); expect(Buffer.from(fixture.text).toString("hex")).toBe(fixture.utf8Hex);
      function create(text: string) {
        const bytes = Buffer.from(text); const prefix = [13, 1, 6, 6, ...Buffer.from("buffer"), 6]; let remaining = bytes.length; do { const digit = remaining % 128; remaining = Math.floor(remaining / 128); prefix.push(digit | (remaining ? 128 : 0)); } while (remaining);
        const wire = new RetainedUiTypedCursor(encodePackValue({ type: "surface", kind: "text-editor", docSchema: "text-editor@1", doc: { bytes: Array.from(Buffer.concat([Buffer.from(prefix), bytes])) }, bindings: [] }), "component"); expect(finish(wire)).toBe("ready"); const input = wire.takeResult()!; close(wire); if (input.value.type !== "surface") throw new Error("Missing fixture Surface"); const pages = input.value.doc.bytes;
        const parser = new OwnedUiSceneCursor(input); retire(input.beginClose()); expect(finish(parser)).toBe("ready"); const raw = parser.takeResult()!; close(parser); const projection = new OwnedUiSceneProjectionCursor(raw, { usizeBits: 64 }); retire(raw.beginClose()); expect(finish(projection)).toBe("ready"); const prepared = projection.takeResult()!; close(projection); const record = prepared.beginRecord(); let source: number | null = null; for (;;) { const current = record.advance(grant); if (current.kind === "value") source = current.value.fields.find(field => field.name === "buffer")?.source ?? null; if (current.kind === "complete") break; } retire(record.beginClose()); if (source === null) throw new Error("Missing buffer field source"); return { prepared, source, pages };
      }
      const small = create(fixture.text);
      for (const vector of fixture.ranges) { const reader = small.prepared.beginTextBytes(small.source, vector.offset, vector.length); const chunks: Uint8Array[] = []; for (;;) { const current = reader.advance(grant); if (current.kind === "bytes") { expect(current.value.length).toBeLessThanOrEqual(fixture.maximumChunkBytes); chunks.push(current.value); } expect(current.bytes).toBeLessThanOrEqual(4096); if (current.kind === "rejected") throw new Error(reader.failure!); if (current.kind === "complete") break; } expect(Buffer.concat(chunks).toString("hex")).toBe(vector.hex); expect(Buffer.concat(chunks)).toEqual(Buffer.from(fixture.text).subarray(vector.offset, vector.offset + vector.length)); retire(reader.beginClose()); }
      for (const vector of fixture.invalid) { if (vector.offset < 0 || vector.length < 0) expect(() => small.prepared.beginTextBytes(small.source, vector.offset, vector.length)).toThrow(/range/); else { const reader = small.prepared.beginTextBytes(small.source, vector.offset, vector.length); let rejected = false; for (let i = 0; i < 256; i++) { const current = reader.advance(grant); if (current.kind === "rejected") { rejected = true; break; } expect(current.kind).not.toBe("bytes"); } expect(rejected).toBe(true); retire(reader.beginClose()); } } retire(small.prepared.beginClose()); expect(() => small.pages.byteAt(0)).toThrow();
      const large = create(fixture.text.repeat(fixture.repeat)); const reader = large.prepared.beginTextBytes(large.source); const cancelled = large.prepared.beginTextBytes(large.source, 1, 4); retire(large.prepared.beginClose()); expect(reader.advance({ maxItems: 0, maxBytes: 4096 }).kind).toBe("blocked"); const chunks: Uint8Array[] = []; for (;;) { const current = reader.advance(grant); if (current.kind === "bytes") chunks.push(current.value); if (current.kind === "complete") break; if (current.kind === "rejected") throw new Error(reader.failure!); } expect(Buffer.concat(chunks)).toEqual(Buffer.from(fixture.text.repeat(fixture.repeat))); expect(chunks.length).toBeGreaterThan(1); retire(reader.beginClose()); expect(large.pages.byteAt(0)).toBe(13); const closing = cancelled.beginClose(); expect(closing.advance({ maxItems: 1, maxBytes: 0 }).kind).toBe("blocked"); retire(closing); expect(() => large.pages.byteAt(0)).toThrow();
    });

    it("OwnedSceneJson streams exact syntax from captured prepared UTF-8 with JSON.parse and Node Buffer oracles", async () => {
      const { OwnedUiSceneJsonCursor } = await import("@semio-tech/ui-contract/retained/scene/typed/json"); const { OwnedUiSceneProjectionCursor } = await import("@semio-tech/ui-contract/retained/scene/typed"); const { OwnedUiSceneCursor } = await import("@semio-tech/ui-contract/retained/scene"); const { RetainedUiTypedCursor } = await import("@semio-tech/ui-contract/retained/wire/typed"); const { encodePackValue } = await import("@semio-tech/framework/packs"); const { Buffer } = await import("node:buffer");
      const fixture = await loadRetainedSceneJsonFixture(); let forgedCaptureCalls = 0; expect(() => Reflect.construct(OwnedUiSceneJsonCursor, [{ capture: () => { forgedCaptureCalls++; return null; } }, 0])).toThrow(); expect(forgedCaptureCalls).toBe(fixture.forgedSourceCaptureCalls);
      function create(text: string, shared = false) {
        const bytes = Buffer.from(text); const prefix = [13, 1, 6, 6, ...Buffer.from("buffer"), 6]; let remaining = bytes.length; do { const digit = remaining % 128; remaining = Math.floor(remaining / 128); prefix.push(digit | (remaining ? 128 : 0)); } while (remaining);
        const typed = new RetainedUiTypedCursor(encodePackValue({ type: "surface", kind: "text-editor", docSchema: "text-editor@1", doc: { bytes: Array.from(Buffer.concat([Buffer.from(prefix), bytes])) }, bindings: [] }), "component"); expect(finish(typed)).toBe("ready"); const input = typed.takeResult()!; close(typed); if (input.value.type !== "surface") throw new Error("Expected scene"); const pages = input.value.doc.bytes; const raw = new OwnedUiSceneCursor(input); retire(input.beginClose()); expect(finish(raw)).toBe("ready"); const document = raw.takeResult()!; close(raw); const projection = new OwnedUiSceneProjectionCursor(document, { usizeBits: 64 }); retire(document.beginClose()); expect(finish(projection)).toBe("ready"); const prepared = projection.takeResult()!; close(projection);
        const reader = prepared.beginRecord(); let field: number | null = null; for (;;) { const current = reader.advance(grant); if (current.kind === "value") field = current.value.fields.find(value => value.name === "buffer")?.source ?? null; if (current.kind === "complete") break; } retire(reader.beginClose()); if (field === null) throw new Error("Missing text source"); const parser = new OwnedUiSceneJsonCursor(prepared, field); const sibling = shared ? new OwnedUiSceneJsonCursor(prepared, field) : null; retire(prepared.beginClose()); return { parser, sibling, bytes, pages };
      }
      const largeText = JSON.stringify({ [fixture.large.key.repeat(fixture.large.repeat)]: fixture.large.value.repeat(fixture.large.repeat) }); const texts = [...fixture.valid.map(value => value.text), largeText];
      for (const text of texts) {
        const { parser, bytes, pages } = create(text); expect(parser.advance({ maxItems: 0, maxBytes: 4096 }).kind).toBe("blocked"); let result: unknown; const stack: { value: unknown[] | Record<string, unknown>; key: string | null }[] = [];
        const put = (value: unknown) => { const parent = stack.at(-1); if (!parent) result = value; else if (Array.isArray(parent.value)) parent.value.push(value); else { if (parent.key === null) throw new Error("Missing JSON key"); Object.defineProperty(parent.value, parent.key, { value, writable: true, configurable: true, enumerable: true }); parent.key = null; } };
        for (let turns = 0;; turns++) { expect(turns).toBeLessThan(bytes.length * 4 + 512); const before = parser.offset; const current = parser.advance(grant); expect(parser.offset).toBeGreaterThanOrEqual(before); expect(parser.offset - before).toBeLessThanOrEqual(1); expect(current.items).toBeLessThanOrEqual(1); expect(current.bytes).toBeLessThanOrEqual(4096); if (current.kind === "rejected") throw new Error(parser.failure!);
          if (current.kind === "token") { const token = current.token; const raw = bytes.subarray(token.start, token.start + token.length).toString("utf8"); if (token.kind === "object" || token.kind === "array") { const value = token.kind === "array" ? [] : {}; put(value); stack.push({ value, key: null }); } else if (token.kind === "end-object" || token.kind === "end-array") stack.pop(); else if (token.kind === "key") stack.at(-1)!.key = JSON.parse(raw); else put(JSON.parse(raw)); }
          if (current.kind === "ready") break;
        }
        expect(result).toEqual(JSON.parse(text)); expect(parser.offset).toBe(bytes.length); close(parser); expect(() => pages.byteAt(0)).toThrow();
      }
      for (const text of fixture.invalid) { expect(() => JSON.parse(text)).toThrow(); const { parser } = create(text); expect(finish(parser)).toBe("rejected"); close(parser); }
      const nested = "[".repeat(fixture.depth) + "0" + "]".repeat(fixture.depth); const deep = create(nested); let stalled = 0;
      for (let turns = 0;; turns++) { expect(turns).toBeLessThan(deep.bytes.length * 4 + 512); const before = deep.parser.offset; const current = deep.parser.advance(grant); expect(current.kind).not.toBe("rejected"); expect(deep.parser.offset).toBeGreaterThanOrEqual(before); expect(deep.parser.offset - before).toBeLessThanOrEqual(1); stalled = deep.parser.offset > before || current.kind === "token" ? 0 : stalled + 1; expect(stalled).toBeLessThan(512); if (current.kind === "ready") break; }
      close(deep.parser); expect(() => deep.pages.byteAt(0)).toThrow();
      for (const cutoff of fixture.cancelPrefixes) { const value = create(nested); for (let n = 0; n < cutoff; n++) value.parser.advance(grant); value.parser.beginClose(); expect(value.parser.closeStep({ maxItems: 0, maxBytes: 4096 }).kind).toBe("blocked"); close(value.parser); expect(() => value.pages.byteAt(0)).toThrow(); }
      const shared = create(largeText, true); shared.parser.advance(grant); close(shared.parser); expect(shared.pages.byteAt(0)).toBe(13); expect(finish(shared.sibling!)).toBe("ready"); expect(shared.sibling!.offset).toBe(shared.bytes.length); close(shared.sibling!); expect(() => shared.pages.byteAt(0)).toThrow();
    });

    it("OwnedSceneJsonDocument publishes flat validated tokens with independent exact span and reader ownership", async () => {
      const { OwnedUiSceneJsonDocumentCursor } = await import("@semio-tech/ui-contract/retained/scene/typed/json/value"); const { OwnedUiSceneProjectionCursor } = await import("@semio-tech/ui-contract/retained/scene/typed"); const { OwnedUiSceneCursor } = await import("@semio-tech/ui-contract/retained/scene"); const { RetainedUiTypedCursor } = await import("@semio-tech/ui-contract/retained/wire/typed"); const { encodePackValue } = await import("@semio-tech/framework/packs"); const { Buffer } = await import("node:buffer");
      const fixture = await loadSceneJsonDocumentFixture(); function create(text: string) {
        const bytes = Buffer.from(text); const prefix = [13, 1, 6, 6, ...Buffer.from("buffer"), 6]; let remaining = bytes.length; do { const digit = remaining % 128; remaining = Math.floor(remaining / 128); prefix.push(digit | (remaining ? 128 : 0)); } while (remaining);
        const typed = new RetainedUiTypedCursor(encodePackValue({ type: "surface", kind: "text-editor", docSchema: "text-editor@1", doc: { bytes: Array.from(Buffer.concat([Buffer.from(prefix), bytes])) }, bindings: [] }), "component"); expect(finish(typed)).toBe("ready"); const input = typed.takeResult()!; close(typed); if (input.value.type !== "surface") throw new Error("Expected exact scene"); const pages = input.value.doc.bytes; const raw = new OwnedUiSceneCursor(input); retire(input.beginClose()); expect(finish(raw)).toBe("ready"); const document = raw.takeResult()!; close(raw); const projection = new OwnedUiSceneProjectionCursor(document, { usizeBits: 64 }); retire(document.beginClose()); expect(finish(projection)).toBe("ready"); const prepared = projection.takeResult()!; close(projection);
        const reader = prepared.beginRecord(); let field: number | null = null; for (;;) { const current = reader.advance(grant); if (current.kind === "value") field = current.value.fields.find(value => value.name === "buffer")?.source ?? null; if (current.kind === "complete") break; } retire(reader.beginClose()); if (field === null) throw new Error("Missing JSON source"); const cursor = new OwnedUiSceneJsonDocumentCursor(prepared, field); retire(prepared.beginClose()); return { cursor, bytes, pages };
      }
      function bound(bytes: number): number { const height = 2 * Math.ceil(Math.log2(bytes + 2)); const allocation = 9 * 10 + 9 + 6 + 9 + 4; return (bytes + 4) * (2 * (height + 1) * (allocation + 16) + 9 * height + 64) + 1024; }
      const { NumericIndexRetirement } = await import("@semio-tech/framework"); const terminal = create("0"); const originalAdvance = NumericIndexRetirement.prototype.advance; let injected = false;
      for (let turn = 0;; turn++) { expect(turn).toBeLessThan(bound(1)); if (terminal.cursor.advance(grant).phase === "json-document-install") break; }
      const terminalSpy = vi.spyOn(NumericIndexRetirement.prototype, "advance").mockImplementation(function (this: InstanceType<typeof NumericIndexRetirement>, budget) { const current = originalAdvance.call(this, budget); if (!injected && current.kind === "complete") { expect(this.terminalIsEmpty()).toBe(true); injected = true; return { kind: "complete", items: 1, bytes: fixture.terminalChildBytes }; } return current; });
      try { for (let turn = 0; !injected; turn++) { expect(turn).toBeLessThan(bound(1)); const current = terminal.cursor.advance(grant); if (injected) { expect(current.kind).toBe("pending"); expect(current.bytes).toBe(fixture.terminalChildBytes); } } } finally { terminalSpy.mockRestore(); }
      const terminalOffset = terminal.cursor.offset; const released = terminal.cursor.advance(grant); expect(released.kind).toBe("pending"); expect(released.bytes).toBe(fixture.terminalReleaseBytes); expect(terminal.cursor.offset).toBe(terminalOffset); expect(finish(terminal.cursor)).toBe("ready"); retire(terminal.cursor.takeResult()!.beginClose()); close(terminal.cursor); expect(() => terminal.pages.byteAt(0)).toThrow();
      const phases = new Map<string, number>();
      const large = JSON.stringify({ [fixture.large.key.repeat(fixture.large.repeat)]: fixture.large.value.repeat(fixture.large.repeat) });
      for (const vector of [...fixture.valid, { name: "large", text: large, keys: [fixture.large.key.repeat(fixture.large.repeat)] }]) {
        const value = create(vector.text); expect(value.cursor.takeResult()).toBeNull(); expect(value.cursor.advance({ maxItems: 0, maxBytes: 4096 }).kind).toBe("blocked");
        for (let turn = 0;; turn++) { expect(turn).toBeLessThan(bound(value.bytes.length)); const before = value.cursor.offset; const current = value.cursor.advance(grant); expect(value.cursor.offset).toBeGreaterThanOrEqual(before); expect(value.cursor.offset - before).toBeLessThanOrEqual(1); expect(current.items).toBeLessThanOrEqual(fixture.maxItems); expect(current.bytes).toBeLessThanOrEqual(fixture.maxBytes); expect(current.kind, value.cursor.failure ?? vector.name).not.toBe("rejected"); if (!phases.has(current.phase)) phases.set(current.phase, turn); if (current.kind === "ready") break; }
        const document = value.cursor.takeResult()!; expect(value.cursor.takeResult()).toBeNull(); close(value.cursor); const alias = document.capture(); const readers = Array.from({ length: fixture.readerCount }, () => document.beginRead()); const span = document.beginSpan(0, value.bytes.length); const missing = document.beginLookup(Number.MAX_SAFE_INTEGER); retire(document.beginClose());
        let missingCount = 0; for (;;) { const current = missing.advance(grant); if (current.kind === "value") missingCount++; if (current.kind === "complete") break; } expect(missingCount).toBe(0); retire(missing.beginClose());
        const keys: string[] = []; let result: unknown; const stack: { value: unknown[] | Record<string, unknown>; key: string | null }[] = [];
        const put = (item: unknown) => { const parent = stack.at(-1); if (!parent) result = item; else if (Array.isArray(parent.value)) parent.value.push(item); else { if (parent.key === null) throw new Error("Missing owned JSON key"); Object.defineProperty(parent.value, parent.key, { value: item, writable: true, configurable: true, enumerable: true }); parent.key = null; } };
        for (;;) { const current = readers[0]!.advance(grant); expect(current.kind).not.toBe("rejected"); if (current.kind === "value") { const token = current.value; const text = value.bytes.subarray(token.start, token.start + token.length).toString("utf8"); if (token.kind === "object" || token.kind === "array") { const item = token.kind === "array" ? [] : {}; put(item); stack.push({ value: item, key: null }); } else if (token.kind === "end-object" || token.kind === "end-array") stack.pop(); else if (token.kind === "key") { const key: string = JSON.parse(text); keys.push(key); stack.at(-1)!.key = key; } else put(JSON.parse(text)); } if (current.kind === "complete") break; }
        expect(result).toEqual(JSON.parse(vector.text)); expect(keys).toEqual(vector.keys); retire(readers[0]!.beginClose()); const first = alias.beginLookup(0); let firstCount = 0; for (;;) { const current = first.advance(grant); if (current.kind === "value") firstCount++; if (current.kind === "complete") break; } expect(firstCount).toBe(1); retire(first.beginClose());
        const prototype: unknown = Object.getPrototypeOf(alias); if (!prototype || typeof prototype !== "object" || !("constructor" in prototype) || typeof prototype.constructor !== "function") throw new Error("Missing JSON owner constructor"); let forged = false; try { Reflect.construct(prototype.constructor, [{}, {}]); forged = true; } catch {} expect(forged).toBe(fixture.forgedRootAccepted); retire(alias.beginClose()); retire(readers[1]!.beginClose()); expect(value.pages.byteAt(0)).toBe(13);
        const chunks: Uint8Array[] = []; for (;;) { const current = span.advance(grant); if (current.kind === "bytes") chunks.push(current.value); if (current.kind === "complete") break; if (current.kind === "rejected") throw new Error(span.failure!); } expect(Buffer.concat(chunks)).toEqual(value.bytes); retire(span.beginClose()); expect(() => value.pages.byteAt(0)).toThrow();
      }
      for (const text of fixture.invalid) { const value = create(text); expect(finish(value.cursor)).toBe("rejected"); expect(value.cursor.takeResult()).toBeNull(); close(value.cursor); expect(() => value.pages.byteAt(0)).toThrow(); }
      const nested = "[".repeat(fixture.depth) + "0" + "]".repeat(fixture.depth); const deep = create(nested); let stalled = 0; const perRecord = bound(1) + 4 * Math.ceil(Math.log2(deep.bytes.length + 2)) * 512;
      for (let turn = 0;; turn++) { expect(turn).toBeLessThan(bound(deep.bytes.length)); const before = deep.cursor.offset; const current = deep.cursor.advance(grant); expect(current.kind, deep.cursor.failure ?? "deep").not.toBe("rejected"); stalled = deep.cursor.offset > before ? 0 : stalled + 1; expect(stalled).toBeLessThan(perRecord); if (current.kind === "ready") break; } const deepDocument = deep.cursor.takeResult()!; close(deep.cursor); retire(deepDocument.beginClose()); expect(() => deep.pages.byteAt(0)).toThrow();
      for (const cutoff of new Set([...fixture.cancelPrefixes, ...phases.values()])) { const value = create(vectorForCancel()); for (let turn = 0; turn < cutoff; turn++) value.cursor.advance(grant); value.cursor.beginClose(); expect(value.cursor.closeStep({ maxItems: 1, maxBytes: 0 }).kind).toBe("blocked"); close(value.cursor); expect(value.cursor.takeResult()).toBeNull(); expect(() => value.pages.byteAt(0)).toThrow(); }
      function vectorForCancel(): string { return fixture.valid[0]!.text; }
      let forgedCalls = 0; expect(() => Reflect.construct(OwnedUiSceneJsonDocumentCursor, [{ capture: () => { forgedCalls++; return null; } }, 0])).toThrow(); expect(forgedCalls).toBe(0);
    });

    it("OwnedSceneJsonString projects selected exact string tokens into bounded UTF-16 pages", async () => {
      const { OwnedUiSceneJsonStringCursor } = await import("@semio-tech/ui-contract/retained/scene/typed/json/string"); const { OwnedUiSceneJsonDocumentCursor } = await import("@semio-tech/ui-contract/retained/scene/typed/json/value"); const { OwnedUiSceneProjectionCursor } = await import("@semio-tech/ui-contract/retained/scene/typed"); const { OwnedUiSceneCursor } = await import("@semio-tech/ui-contract/retained/scene"); const { RetainedUiTypedCursor } = await import("@semio-tech/ui-contract/retained/wire/typed"); const { encodePackValue } = await import("@semio-tech/framework/packs"); const { Buffer } = await import("node:buffer");
      const fixture = await loadSceneJsonStringFixture(); function create(text: string, ordinal: number, sibling = false) {
        const bytes = Buffer.from(text); const prefix = [13, 1, 6, 6, ...Buffer.from("buffer"), 6]; let remaining = bytes.length; do { const digit = remaining % 128; remaining = Math.floor(remaining / 128); prefix.push(digit | (remaining ? 128 : 0)); } while (remaining);
        const typed = new RetainedUiTypedCursor(encodePackValue({ type: "surface", kind: "text-editor", docSchema: "text-editor@1", doc: { bytes: Array.from(Buffer.concat([Buffer.from(prefix), bytes])) }, bindings: [] }), "component"); expect(finish(typed)).toBe("ready"); const input = typed.takeResult()!; close(typed); if (input.value.type !== "surface") throw new Error("Expected exact scene"); const pages = input.value.doc.bytes; const raw = new OwnedUiSceneCursor(input); retire(input.beginClose()); expect(finish(raw)).toBe("ready"); const document = raw.takeResult()!; close(raw); const projection = new OwnedUiSceneProjectionCursor(document, { usizeBits: 64 }); retire(document.beginClose()); expect(finish(projection)).toBe("ready"); const prepared = projection.takeResult()!; close(projection);
        const reader = prepared.beginRecord(); let field: number | null = null; for (;;) { const current = reader.advance(grant); if (current.kind === "value") field = current.value.fields.find(value => value.name === "buffer")?.source ?? null; if (current.kind === "complete") break; } retire(reader.beginClose()); if (field === null) throw new Error("Missing JSON source"); const parser = new OwnedUiSceneJsonDocumentCursor(prepared, field); retire(prepared.beginClose()); expect(finish(parser)).toBe("ready"); const json = parser.takeResult()!; close(parser); const cursor = new OwnedUiSceneJsonStringCursor(json, ordinal); const other = sibling ? new OwnedUiSceneJsonStringCursor(json, ordinal) : null; retire(json.beginClose()); return { cursor, other, bytes, pages };
      }
      const { OwnedUiSceneJsonDocumentReader } = await import("@semio-tech/ui-contract/retained/scene/typed/json/value"); const terminal = create("\"x\"", 0); const originalAdvance = OwnedUiSceneJsonDocumentReader.prototype.advance; let injected = false;
      const terminalSpy = vi.spyOn(OwnedUiSceneJsonDocumentReader.prototype, "advance").mockImplementation(function (this: import("@semio-tech/ui-contract/retained/scene/typed/json/value").OwnedUiSceneJsonDocumentReader, budget) { const current = originalAdvance.call(this, budget); if (!injected && current.kind === "complete") { injected = true; return { kind: "complete", phase: current.phase, items: 1, bytes: fixture.terminalChildBytes }; } return current; });
      try { for (let turn = 0; !injected; turn++) { expect(turn).toBeLessThan(2048); const current = terminal.cursor.advance(grant); if (injected) { expect(current.kind).toBe("pending"); expect(current.bytes).toBe(fixture.terminalChildBytes); } } } finally { terminalSpy.mockRestore(); }
      const released = terminal.cursor.advance(grant); expect(released.kind).toBe("pending"); expect(released.bytes).toBe(fixture.terminalReleaseBytes); close(terminal.cursor); expect(() => terminal.pages.byteAt(0)).toThrow();
      const phases = new Map<string, number>(); const large = fixture.large.value.repeat(fixture.large.repeat); const vectors = [...fixture.valid, { name: "large", text: JSON.stringify(large), ordinal: 0, value: large }];
      for (const vector of vectors) {
        const value = create(vector.text, vector.ordinal); expect(value.cursor.advance({ maxItems: 0, maxBytes: 4096 }).kind).toBe("blocked"); const chunks: string[] = [];
        for (let turn = 0;; turn++) { expect(turn).toBeLessThan(value.bytes.length * 4 + 2048); const before = value.cursor.offset; const current = value.cursor.advance(grant); expect(value.cursor.offset).toBeGreaterThanOrEqual(before); expect(value.cursor.offset - before).toBeLessThanOrEqual(1); expect(current.items).toBeLessThanOrEqual(fixture.maxItems); expect(current.bytes).toBeLessThanOrEqual(fixture.maxBytes); expect(current.kind, value.cursor.failure ?? vector.name).not.toBe("rejected"); if (vector.name === "large" && !phases.has(current.phase)) phases.set(current.phase, turn); if (current.kind === "text") { expect(current.value.length).toBeLessThanOrEqual(fixture.chunkCodeUnits); chunks.push(current.value); } if (current.kind === "ready") break; }
        expect(chunks.join("")).toBe(vector.value); expect(Buffer.from(chunks.join(""), "utf16le")).toEqual(Buffer.from(vector.value, "utf16le")); if (vector.ordinal === 0) expect(chunks.join("")).toBe(JSON.parse(vector.text)); close(value.cursor); expect(() => value.pages.byteAt(0)).toThrow();
      }
      for (const vector of fixture.invalid) { const value = create(vector.text, vector.ordinal); expect(finish(value.cursor)).toBe("rejected"); close(value.cursor); expect(() => value.pages.byteAt(0)).toThrow(); }
      for (const cutoff of new Set([...fixture.cancelPrefixes, ...phases.values()])) { const value = create(JSON.stringify(large), 0); for (let turn = 0; turn < cutoff; turn++) value.cursor.advance(grant); value.cursor.beginClose(); expect(value.cursor.closeStep({ maxItems: 1, maxBytes: 0 }).kind).toBe("blocked"); close(value.cursor); expect(() => value.pages.byteAt(0)).toThrow(); }
      const shared = create(JSON.stringify(large), 0, true); shared.cursor.advance(grant); close(shared.cursor); expect(shared.pages.byteAt(0)).toBe(13); const chunks: string[] = []; for (;;) { const current = shared.other!.advance(grant); if (current.kind === "text") chunks.push(current.value); if (current.kind === "ready") break; if (current.kind === "rejected") throw new Error(shared.other!.failure!); } expect(chunks.join("")).toBe(large); close(shared.other!); expect(() => shared.pages.byteAt(0)).toThrow();
      let captureCalls = 0; expect(() => Reflect.construct(OwnedUiSceneJsonStringCursor, [{ capture: () => { captureCalls++; return null; } }, 0])).toThrow(); expect(captureCalls).toBe(fixture.forgedCaptureCalls);
    });

    it("OwnedScenePackedField decodes captured base64 into independently retired pages with atob and Node Buffer oracles", async () => {
      const { OwnedUiSceneBase64Cursor } = await import("@semio-tech/ui-contract/retained/scene/typed/pack/base64"); const { OwnedUiSceneProjectionCursor } = await import("@semio-tech/ui-contract/retained/scene/typed"); const { OwnedUiSceneCursor } = await import("@semio-tech/ui-contract/retained/scene"); const { RetainedUiTypedCursor } = await import("@semio-tech/ui-contract/retained/wire/typed"); const { encodePackValue } = await import("@semio-tech/framework/packs"); const { Buffer } = await import("node:buffer");
      const fixture = await loadScenePackFieldFixture(); let forgedCaptureCalls = 0; expect(() => Reflect.construct(OwnedUiSceneBase64Cursor, [{ capture: () => { forgedCaptureCalls++; return null; } }, 0])).toThrow(); expect(forgedCaptureCalls).toBe(0);
      function create(text: string) {
        const bytes = Buffer.from(text); const prefix = [13, 1, 6, 6, ...Buffer.from("buffer"), 6]; let remaining = bytes.length; do { const digit = remaining % 128; remaining = Math.floor(remaining / 128); prefix.push(digit | (remaining ? 128 : 0)); } while (remaining);
        const wire = new RetainedUiTypedCursor(encodePackValue({ type: "surface", kind: "text-editor", docSchema: "text-editor@1", doc: { bytes: Array.from(Buffer.concat([Buffer.from(prefix), bytes])) }, bindings: [] }), "component"); expect(finish(wire)).toBe("ready"); const input = wire.takeResult()!; close(wire); if (input.value.type !== "surface") throw new Error("Expected scene"); const pages = input.value.doc.bytes; const raw = new OwnedUiSceneCursor(input); retire(input.beginClose()); expect(finish(raw)).toBe("ready"); const document = raw.takeResult()!; close(raw); const projection = new OwnedUiSceneProjectionCursor(document, { usizeBits: 64 }); retire(document.beginClose()); expect(finish(projection)).toBe("ready"); const prepared = projection.takeResult()!; close(projection);
        const reader = prepared.beginRecord(); let field: number | null = null; for (;;) { const current = reader.advance(grant); if (current.kind === "value") field = current.value.fields.find(value => value.name === "buffer")?.source ?? null; if (current.kind === "complete") break; } retire(reader.beginClose()); if (field === null) throw new Error("Missing text source"); const cursor = new OwnedUiSceneBase64Cursor(prepared, field); retire(prepared.beginClose()); return { cursor, pages, bytes };
      }
      const large = Buffer.from(Array.from({ length: fixture.largeBytes }, (_, index) => index % 256)); const vectors = [...fixture.valid, { name: "large", text: `pk:${large.toString("base64")}`, hex: large.toString("hex") }]; const phases = new Map<string, number>();
      for (const vector of vectors) { const oracle = Buffer.from(atob(vector.text.slice(3)), "latin1"); expect(oracle.toString("hex")).toBe(vector.hex); expect(Buffer.from(vector.text.slice(3), "base64")).toEqual(oracle); const { cursor, pages, bytes } = create(vector.text); expect(cursor.takeResult()).toBeNull(); expect(cursor.advance({ maxItems: 0, maxBytes: 4096 }).kind).toBe("blocked");
        for (let turns = 0;; turns++) { expect(turns).toBeLessThan(bytes.length * 5 + 1024); const before = cursor.sourceBytesRead; const current = cursor.advance(grant); expect(cursor.sourceBytesRead).toBeGreaterThanOrEqual(before); expect(cursor.sourceBytesRead - before).toBeLessThanOrEqual(1); expect(current.items).toBeLessThanOrEqual(1); expect(current.bytes).toBeLessThanOrEqual(4096); expect(current.kind, cursor.failure ?? vector.name).not.toBe("rejected"); if (vector.name === "large" && !phases.has(current.phase)) phases.set(current.phase, turns); if (current.kind === "ready") break; }
        const result = cursor.takeResult()!; expect(cursor.takeResult()).toBeNull(); close(cursor); expect(() => pages.byteAt(0)).toThrow(); expect(result.length).toBe(oracle.length); expect(Buffer.from(Array.from({ length: result.length }, (_, index) => result.byteAt(index)))).toEqual(oracle); const prototype: unknown = Object.getPrototypeOf(result); if (!prototype || typeof prototype !== "object" || !("constructor" in prototype) || typeof prototype.constructor !== "function") throw new Error("Missing packed root constructor"); let forgedRootAccepted = false; try { Reflect.construct(prototype.constructor, [{}, {}]); forgedRootAccepted = true; } catch {} expect(forgedRootAccepted).toBe(fixture.forgedRootAccepted); const alias = result.capture(); retire(result.beginClose()); if (alias.length) expect(alias.byteAt(0)).toBe(oracle[0]); retire(alias.beginClose()); expect(() => alias.byteAt(0)).toThrow();
      }
      for (const text of fixture.invalid) { if (text.startsWith("pk:")) expect(() => atob(text.slice(3))).toThrow(); const { cursor, pages } = create(text); expect(finish(cursor)).toBe("rejected"); expect(cursor.takeResult()).toBeNull(); close(cursor); expect(() => pages.byteAt(0)).toThrow(); }
      for (const cutoff of new Set([...fixture.cancelPrefixes, ...phases.values()])) { const { cursor, pages } = create(`pk:${large.toString("base64")}`); for (let index = 0; index < cutoff; index++) cursor.advance(grant); cursor.beginClose(); expect(cursor.closeStep({ maxItems: 1, maxBytes: 0 }).kind).toBe("blocked"); close(cursor); expect(cursor.takeResult()).toBeNull(); expect(() => pages.byteAt(0)).toThrow(); }
    });

    it("OwnedSceneGenericPack retains original symbols, map pairs and exact scalar spans with native-wire and Node oracles", async () => {
      const { OwnedUiGenericPackCursor } = await import("@semio-tech/ui-contract/retained/scene/typed/pack/value"); const { OwnedUiSceneBase64Cursor } = await import("@semio-tech/ui-contract/retained/scene/typed/pack/base64"); const { OwnedUiSceneProjectionCursor } = await import("@semio-tech/ui-contract/retained/scene/typed"); const { OwnedUiSceneCursor } = await import("@semio-tech/ui-contract/retained/scene"); const { RetainedUiTypedCursor } = await import("@semio-tech/ui-contract/retained/wire/typed"); const { encodePackValue, decodePackValue } = await import("@semio-tech/framework/packs"); const { Buffer } = await import("node:buffer");
      const fixture = await loadSceneGenericPackFixture(); function create(binary: Uint8Array) {
        const bytes = Buffer.from(`pk:${Buffer.from(binary).toString("base64")}`); const prefix = [13, 1, 6, 6, ...Buffer.from("buffer"), 6]; let remaining = bytes.length; do { const digit = remaining % 128; remaining = Math.floor(remaining / 128); prefix.push(digit | (remaining ? 128 : 0)); } while (remaining);
        const wire = new RetainedUiTypedCursor(encodePackValue({ type: "surface", kind: "text-editor", docSchema: "text-editor@1", doc: { bytes: Array.from(Buffer.concat([Buffer.from(prefix), bytes])) }, bindings: [] }), "component"); expect(finish(wire)).toBe("ready"); const input = wire.takeResult()!; close(wire); const raw = new OwnedUiSceneCursor(input); retire(input.beginClose()); expect(finish(raw)).toBe("ready"); const document = raw.takeResult()!; close(raw); const projection = new OwnedUiSceneProjectionCursor(document, { usizeBits: 64 }); retire(document.beginClose()); expect(finish(projection)).toBe("ready"); const prepared = projection.takeResult()!; close(projection);
        const reader = prepared.beginRecord(); let field: number | null = null; for (;;) { const current = reader.advance(grant); if (current.kind === "value") field = current.value.fields.find(value => value.name === "buffer")?.source ?? null; if (current.kind === "complete") break; } retire(reader.beginClose()); if (field === null) throw new Error("Missing packed source"); const base64 = new OwnedUiSceneBase64Cursor(prepared, field); retire(prepared.beginClose()); expect(finish(base64)).toBe("ready"); const source = base64.takeResult()!; close(base64); const cursor = new OwnedUiGenericPackCursor(source); retire(source.beginClose()); return cursor;
      }
      function drive(cursor: InstanceType<typeof OwnedUiGenericPackCursor>, bytes: number): string {
        const height = 2 * Math.ceil(Math.log2(bytes + 1)); const allocations = 9 * 10 + 9 + 6 + 9 + 4; const perRecord = 2 * (height + 1) * (allocations + 16) + 9 * height + 64; const bound = (bytes * 3 + 4) * perRecord;
        for (let turn = 0; turn < bound; turn++) { const before = cursor.offset; const current = cursor.advance(grant); if (cursor.offset < before || cursor.offset - before > 1 || current.items > 1 || current.bytes > 4096) throw new Error("Generic pack exceeded exact grant"); if (current.kind === "ready" || current.kind === "rejected") return current.kind; } throw new Error("Generic pack exceeded byte-derived dual-index bound");
      }
      const { NumericIndexRetirement, NumericIndexReader } = await import("@semio-tech/framework"); const terminal = create(Buffer.from("0001011112", "hex")); terminal.beginClose(); expect(terminal.closeStep(grant).phase).toBe("generic-pack-symbol-close");
      const originalAdvance = NumericIndexRetirement.prototype.advance; const terminalSpy = vi.spyOn(NumericIndexRetirement.prototype, "advance").mockImplementationOnce(function (this: InstanceType<typeof NumericIndexRetirement>, budget) { const current = originalAdvance.call(this, budget); expect(current.kind).toBe("complete"); expect(this.terminalIsEmpty()).toBe(true); return { kind: "complete", items: 1, bytes: fixture.terminalChildBytes }; });
      try { const current = terminal.closeStep(grant); expect(current.kind).toBe("pending"); expect(current.bytes).toBe(fixture.terminalChildBytes); } finally { terminalSpy.mockRestore(); }
      const released = terminal.closeStep(grant); expect(released.kind).toBe("pending"); expect(released.bytes).toBe(fixture.terminalReleaseBytes); close(terminal);
      const largeValues: unknown[] = [{ [fixture.large.key.repeat(fixture.large.repeat)]: true }, fixture.large.text.repeat(fixture.large.repeat), Array.from({ length: fixture.large.items }, (_, index) => index % 2 === 0)];
      const vectors = [...fixture.valid.map(value => ({ bytes: Buffer.from(value.hex, "hex"), expected: value.expected, keys: value.keys ?? null })), ...largeValues.map(expected => ({ bytes: Buffer.from(encodePackValue(expected)), expected, keys: null }))];
      for (const vector of vectors) { expect(decodePackValue(vector.bytes)).toEqual(vector.expected); const cursor = create(vector.bytes); expect(cursor.takeResult()).toBeNull(); expect(cursor.advance({ maxItems: 0, maxBytes: 4096 }).kind).toBe("blocked"); expect(drive(cursor, vector.bytes.length), cursor.failure ?? "generic pack").toBe("ready"); const document = cursor.takeResult()!; close(cursor); const source = document.captureSource(); const reader = document.beginRead(); retire(document.beginClose()); let field = 0; let result: unknown = null; const keys: string[] = []; const stack: { value: unknown[] | Record<string, unknown>; key: string | null }[] = [];
        const put = (value: unknown) => { const parent = stack.at(-1); if (!parent) { if (field === 1) result = value; } else if (Array.isArray(parent.value)) parent.value.push(value); else { if (parent.key === null) throw new Error("Missing generic pack key"); Object.defineProperty(parent.value, parent.key, { value, writable: true, configurable: true, enumerable: true }); parent.key = null; } };
        for (;;) { const current = reader.advance(grant); expect(current.bytes).toBeLessThanOrEqual(4096); if (current.kind === "value") { const token = current.value; if (token.kind === "field") field = token.field; else if (token.kind === "array" || token.kind === "map") { const value = token.kind === "array" ? [] : {}; put(value); stack.push({ value, key: null }); } else if (token.kind === "end-array" || token.kind === "end-map") stack.pop(); else if (token.kind === "key" || token.kind === "string") { const text = Buffer.from(Array.from({ length: token.length }, (_, index) => source.byteAt(token.start + index))).toString("utf8"); if (token.kind === "key") { stack.at(-1)!.key = text; keys.push(text); } else put(text); } else put(token.kind === "number" ? token.value : token.kind === "null" ? null : token.kind === "true"); } if (current.kind === "complete") break; } expect(result).toEqual(JSON.parse(JSON.stringify(vector.expected))); if (vector.keys) expect(keys).toEqual(vector.keys); retire(reader.beginClose()); retire(source.beginClose());
      }
      for (const vector of fixture.integers) { const bytes = Buffer.from(vector.hex, "hex"); const cursor = create(bytes); expect(drive(cursor, bytes.length), cursor.failure ?? vector.name).toBe("ready"); const document = cursor.takeResult()!; close(cursor); const source = document.captureSource(); const reader = document.beginRead(); retire(document.beginClose()); const observed: { kind: string; value: string }[] = [];
        for (;;) { const current = reader.advance(grant); if (current.kind === "value" && (current.value.kind === "int" || current.value.kind === "uint")) { expect(typeof current.value.value).toBe("bigint"); expect(current.value.length).toBeGreaterThan(0); expect(source.byteAt(current.value.start - 1)).toBe(current.value.kind === "int" ? 3 : 4); observed.push({ kind: current.value.kind, value: current.value.value.toString() }); } if (current.kind === "complete") break; }
        expect(observed, vector.name).toEqual(vector.tokens); retire(reader.beginClose()); retire(source.beginClose()); }
      for (const bits of fixture.floatBits) { const bytes = Buffer.from(`0001011105${bits}`, "hex"); const cursor = create(bytes); expect(drive(cursor, bytes.length)).toBe("ready"); const document = cursor.takeResult()!; close(cursor); const reader = document.beginRead(); retire(document.beginClose()); let observed = false; for (;;) { const current = reader.advance(grant); if (current.kind === "value" && current.value.kind === "number") observed = Object.is(current.value.value, bytes.readDoubleLE(5)); if (current.kind === "complete") break; } expect(observed).toBe(true); retire(reader.beginClose()); }
      for (const vector of fixture.invalid) { const bytes = Buffer.from(vector.hex, "hex"); const cursor = create(bytes); expect(drive(cursor, bytes.length)).toBe("rejected"); expect(cursor.takeResult()).toBeNull(); close(cursor); }
      const deepBytes = Buffer.concat([Buffer.from([0,1,1,17]), Buffer.from(Array.from({ length: fixture.depth * 2 }, (_, index) => index % 2 === 0 ? 12 : 1)), Buffer.from([18])]); const deep = create(deepBytes); expect(drive(deep, deepBytes.length)).toBe("ready"); retire(deep.takeResult()!.beginClose()); close(deep);
      for (const cutoff of fixture.cancelPrefixes) { const cursor = create(deepBytes); for (let index = 0; index < cutoff; index++) cursor.advance(grant); close(cursor); expect(cursor.takeResult()).toBeNull(); }
      let called = false; expect(() => Reflect.construct(OwnedUiGenericPackCursor, [{ capture: () => { called = true; return null; } }])).toThrow(); expect(called).toBe(fixture.forgedRootAccepted);
      for (const target of ["reader", "records"] as const) for (const vector of fixture.childSteps) { const cursor = create(Buffer.from("0001011112", "hex")); expect(drive(cursor, 5)).toBe("ready"); const document = cursor.takeResult()!; close(cursor); const reader = target === "reader" ? document.beginRead() : null; const owner = reader ? reader.beginClose() : document.beginClose(); if (!reader) { owner.advance(grant); owner.advance(grant); } const spy = vi.spyOn(NumericIndexRetirement.prototype, "advance").mockImplementationOnce(() => vector.kind === "blocked" ? { kind: "blocked", items: vector.items, bytes: vector.bytes } : vector.kind === "rejected" ? { kind: "rejected", reason: "ordinal-exhausted", items: vector.items, bytes: vector.bytes } : { kind: "pending", items: vector.items, bytes: vector.bytes }); try { const current = owner.advance(grant); expect(current.kind).toBe(vector.expected); expect(current.items).toBe(vector.items); expect(current.bytes).toBe(vector.bytes); expect(owner.terminalIsEmpty()).toBe(false); } finally { spy.mockRestore(); } retire(owner); if (reader) retire(document.beginClose()); }
      for (const vector of fixture.childSteps.filter(value => value.kind !== "rejected")) { const cursor = create(Buffer.from("0001011112", "hex")); expect(drive(cursor, 5)).toBe("ready"); const document = cursor.takeResult()!; close(cursor); const reader = document.beginRead(); const spy = vi.spyOn(NumericIndexReader.prototype, "advance").mockImplementationOnce(() => vector.kind === "blocked" ? { kind: "blocked", items: vector.items, bytes: vector.bytes } : { kind: "pending", items: vector.items, bytes: vector.bytes }); try { const current = reader.advance(grant); expect(current.kind).toBe(vector.expected); expect(current.bytes).toBe(vector.bytes); } finally { spy.mockRestore(); } retire(reader.beginClose()); retire(document.beginClose()); }
    });

    it("SceneBinding atomically owns the exact node and prepared scene or a surfaced diagnostic", async () => {
      const { OwnedUiSceneBindingCursor } = await import("@semio-tech/ui-contract/retained/scene/binding");
      const { RetainedUiTypedCursor, OwnedUiNode } = await import("@semio-tech/ui-contract/retained/wire/typed");
      const fixture = await loadRetainedSceneBindingFixture();
      const fields = await loadRetainedWireFieldsFixture(); const { encodePackValue } = await import("@semio-tech/framework/packs");
      for (const vector of fixture.cases) {
        const record = { ...produce(fields.node, draft => { draft.id = 2 ** 40; }), children: [] };
        const source = { ...record, component: { type: "surface", kind: vector.kind, docSchema: vector.schema, doc: { bytes: vector.packet }, bindings: [] } };
        const typed = new RetainedUiTypedCursor(encodePackValue(source), "node"); expect(finish(typed)).toBe("ready"); const payload = typed.takeResult()!; close(typed); const node = OwnedUiNode.captureFrom(payload); retire(payload.beginClose());
        const cursor = new OwnedUiSceneBindingCursor(node, { usizeBits: 64 }); retire(node.beginClose()); expect(finish(cursor)).toBe("ready"); const binding = cursor.takeResult()!; close(cursor); expect(binding.value.id).toBe(2 ** 40); expect(binding.diagnostic?.code ?? null).toBe(vector.diagnostic);
        const alias = binding.capture(); const reader = alias.beginRecord(); retire(binding.beginClose()); retire(alias.beginClose());
        if (vector.diagnostic) expect(reader).toBeNull(); else { expect(reader).not.toBeNull(); let seen = false; for (;;) { const current = reader!.advance(grant); if (current.kind === "value") seen = current.value.schema === "TextEditorScene"; if (current.kind === "complete") break; } expect(seen).toBe(true); retire(reader!.beginClose()); }
      }
    });

    it("SceneBinding reuses only identical component roots and closes cancellation independently of old issued readers", async () => {
      const { OwnedUiSceneBindingCursor } = await import("@semio-tech/ui-contract/retained/scene/binding");
      const { RetainedUiTypedCursor, OwnedUiNode } = await import("@semio-tech/ui-contract/retained/wire/typed");
      const fixture = await loadRetainedSceneBindingFixture();
      const fields = await loadRetainedWireFieldsFixture(); const { encodePackValue } = await import("@semio-tech/framework/packs");
      const vector = fixture.cases[0]!; const wire = new RetainedUiTypedCursor(encodePackValue({ ...fields.node, children: [], component: { type: "surface", kind: vector.kind, docSchema: vector.schema, doc: { bytes: vector.packet }, bindings: [] } }), "node"); expect(finish(wire)).toBe("ready"); const payload = wire.takeResult()!; close(wire); const node = OwnedUiNode.captureFrom(payload); retire(payload.beginClose());
      const first = new OwnedUiSceneBindingCursor(node, { usizeBits: 64 }); const phases = new Map<string, number>(); let count = 0;
      for (;;) { const current = first.advance(grant); if (!phases.has(current.phase)) phases.set(current.phase, count); count++; if (current.kind === "ready") break; expect(current.kind).toBe("pending"); }
      const binding = first.takeResult()!; close(first); const oldReader = binding.beginRecord()!;
      for (const cutoff of [...phases.values(), count]) { const cancelled = new OwnedUiSceneBindingCursor(node, { usizeBits: 64 }); for (let index = 0; index < cutoff; index++) cancelled.advance(grant); close(cancelled); expect(node.terminalIsEmpty()).toBe(false); }
      const styleWire = new RetainedUiTypedCursor(encodePackValue({}), "style"); expect(finish(styleWire)).toBe("ready"); const style = styleWire.takeResult()!; close(styleWire); const changedStyle = node.replace({ field: "style", payload: style });
      const reuse = new OwnedUiSceneBindingCursor(changedStyle, { usizeBits: 64 }); reuse.considerPrevious(binding); retire(changedStyle.beginClose()); const reusedPhases = new Set<string>(); for (;;) { const current = reuse.advance(grant); reusedPhases.add(current.phase); if (current.kind === "ready") break; expect(current.kind).toBe("pending"); }
      expect(reusedPhases.has("scene-binding-packet")).toBe(false); const reused = reuse.takeResult()!; close(reuse); expect(reused.value.component).toBe(binding.value.component); retire(reused.beginClose());
      const replacement = new RetainedUiTypedCursor(encodePackValue({ type: "surface", kind: vector.kind, docSchema: vector.schema, doc: { bytes: [13, 0] }, bindings: [] }), "component"); expect(finish(replacement)).toBe("ready"); const component = replacement.takeResult()!; close(replacement); const changed = node.replace({ field: "component", payload: component });
      const fresh = new OwnedUiSceneBindingCursor(changed, { usizeBits: 64 }); fresh.considerPrevious(binding); retire(changed.beginClose()); retire(node.beginClose()); retire(binding.beginClose()); expect(finish(fresh)).toBe("ready"); const rejectedScene = fresh.takeResult()!; close(fresh); expect(rejectedScene.diagnostic?.code).toBe("invalid-scene-fields"); retire(rejectedScene.beginClose());
      let oldValid = false; for (;;) { const current = oldReader.advance(grant); if (current.kind === "value") oldValid = current.value.schema === "TextEditorScene"; if (current.kind === "complete") break; } expect(oldValid).toBe(true); retire(oldReader.beginClose());
    });

    it("TypedScene separates exact numeric wire bits from finite geometry and owning-host usize width", async () => {
      const { OwnedUiSceneProjectionCursor } = await import("@semio-tech/ui-contract/retained/scene/typed");
      const { OwnedUiSceneCursor } = await import("@semio-tech/ui-contract/retained/scene");
      const { RetainedUiTypedCursor } = await import("@semio-tech/ui-contract/retained/wire/typed");
      const fixture = await loadRetainedSceneNumericFixture();
      const { Buffer } = await import("node:buffer"); const { encodePackValue } = await import("@semio-tech/framework/packs"); const text = (value: string) => { const bytes = Buffer.from(value); return Buffer.concat([Buffer.from([6, bytes.length]), bytes]); };
      const map = (values: readonly (readonly [string, Uint8Array])[]) => Buffer.concat([Buffer.from([13, values.length]), ...values.flatMap(([key, value]) => [text(key), value])]);
      const scalar = (value: number) => { const bytes = Buffer.alloc(9); bytes[0] = 5; bytes.writeDoubleLE(value, 1); return bytes; };
      const zero = Buffer.from([3, 0]); const some = (value: Uint8Array) => Buffer.concat([Buffer.from([9]), value]);
      const source = (bytes: Uint8Array, kind: string) => { const typed = new RetainedUiTypedCursor(encodePackValue({ type: "surface", kind, docSchema: `${kind}@1`, doc: { bytes: Array.from(bytes) }, bindings: [] }), "component"); expect(finish(typed)).toBe("ready"); const input = typed.takeResult()!; close(typed); const parser = new OwnedUiSceneCursor(input); retire(input.beginClose()); expect(finish(parser)).toBe("ready"); const document = parser.takeResult()!; close(parser); return document; };
      for (const vector of fixture.cases) {
        const raw = Buffer.from(vector.hex, "hex"); const generic = source(raw, "canvas-2d"); const read = generic.beginRead(); retire(generic.beginClose());
        for (;;) { const current = read.advance(grant); if (current.kind === "value") { if (current.value.kind === "float") expect(Object.is(current.value.value, raw.readDoubleLE(1))).toBe(true); else expect(current.value.kind).toBe("integer"); } if (current.kind === "complete") break; } retire(read.beginClose());
        if (vector.type === "i64") continue;
        let kind = "canvas-2d"; let packet: Uint8Array;
        if (vector.type === "usize") {
          kind = "node-graph"; const variadic = map([["slotKey", text("slot")], ["min", raw]]); const operator = map([["id", text("op")], ["extension", text("ext")], ["name", text("name")], ["abbreviation", text("op")], ["icon", text("")], ["summary", text("")], ["variadicInput", some(variadic)]]); packet = map([["operators", Buffer.concat([Buffer.from([10, 1]), operator])]]);
        } else {
          const snapshot = map([["slot", vector.type === "u8" ? raw : zero], ["epoch", vector.type === "u64" ? raw : zero], ["revision", zero], ["generation", zero], ["pageCount", zero], ["byteCount", vector.type === "u32" ? raw : zero]]);
          packet = map([["cameraX", scalar(0)], ["cameraY", scalar(0)], ["zoom", vector.type === "f64" ? raw : scalar(1)], ["layersJson", text("[]")], ["snapshot", some(snapshot)]]);
        }
        const document = source(packet, kind);
        for (const usizeBits of [32, 64] as const) {
          const profile: { usizeBits: 32 | 64 } = { usizeBits }; const cursor = new OwnedUiSceneProjectionCursor(document, profile); profile.usizeBits = usizeBits === 32 ? 64 : 32;
          const expected = (usizeBits === 32 ? vector.native32 : vector.native64) && vector.finite; expect(finish(cursor), `${vector.name}/${usizeBits}`).toBe(expected ? "ready" : "rejected"); if (expected) retire(cursor.takeResult()!.beginClose()); close(cursor);
        }
        retire(document.beginClose());
      }
    });

    it("SceneBinding refuses forged typed source constructors before reading caller-owned roots", async () => {
      const { OwnedUiNode, OwnedUiPayload } = await import("@semio-tech/ui-contract/retained/wire/typed");
      let reads = 0; const forged = Object.defineProperty({}, "value", { get() { reads++; throw new Error("Untrusted value getter"); } });
      expect(() => Reflect.construct(OwnedUiPayload, [forged])).toThrow(/authority/);
      expect(() => Reflect.construct(OwnedUiNode, [forged])).toThrow(/authority/);
      expect(reads).toBe(0);
    });

    it("SceneBindingIndex keeps paired roots through concurrent edit cancellation and insertion-order snapshots", async () => {
      const { OwnedUiNodeReadLease } = await import("@semio-tech/ui-contract/retained/read-lease");
      const { OwnedUiSceneBindingCursor } = await import("@semio-tech/ui-contract/retained/scene/binding");
      const { OwnedUiSceneBindingIndex } = await import("@semio-tech/ui-contract/retained/scene/binding/index");
      const { RetainedUiTypedCursor, OwnedUiNode } = await import("@semio-tech/ui-contract/retained/wire/typed");
      const fixture = await loadRetainedSceneBindingFixture();
      const fields = await loadRetainedWireFieldsFixture(); const { encodePackValue } = await import("@semio-tech/framework/packs");
      const create = (id: number) => { const vector = fixture.cases[0]!; const typed = new RetainedUiTypedCursor(encodePackValue({ ...fields.node, id, children: [], component: { type: "surface", kind: vector.kind, docSchema: vector.schema, doc: { bytes: vector.packet }, bindings: [] } }), "node"); expect(finish(typed)).toBe("ready"); const payload = typed.takeResult()!; close(typed); const node = OwnedUiNode.captureFrom(payload); retire(payload.beginClose()); const cursor = new OwnedUiSceneBindingCursor(node, { usizeBits: 64 }); retire(node.beginClose()); expect(finish(cursor)).toBe("ready"); const result = cursor.takeResult()!; close(cursor); return result; };
      let index = OwnedUiSceneBindingIndex.empty(); const map = new Map<number, true>();
      for (const id of fixture.indexIds) { const binding = create(id); const edit = index.beginSet(binding); retire(binding.beginClose()); expect(finish(edit)).toBe("ready"); const next = edit.takeResult()!; retire(edit.beginClose()); retire(index.beginClose()); index = next; map.set(id, true); }
      const oldReader = index.beginRead(); const first = fixture.indexIds[0]!; const cancelled = index.beginRemove(first); expect(cancelled.advance({ maxItems: 0, maxBytes: 4096 }).kind).toBe("blocked"); cancelled.advance(grant); retire(cancelled.beginClose()); expect(index.size).toBe(2);
      const remove = index.beginRemove(first); expect(finish(remove)).toBe("ready"); const removed = remove.takeResult()!; retire(remove.beginClose()); retire(index.beginClose()); index = removed; map.delete(first);
      const replacement = create(first); const insert = index.beginSet(replacement); retire(replacement.beginClose()); expect(finish(insert)).toBe("ready"); const next = insert.takeResult()!; retire(insert.beginClose()); retire(index.beginClose()); index = next; map.set(first, true);
      const current = index.beginRead(); retire(index.beginClose()); const ids: number[] = [];
      for (;;) { const step = current.advance(grant); if (step.kind === "value") { ids.push(step.id); retire(step.value.beginClose()); } if (step.kind === "complete") break; } expect(ids).toEqual(Array.from(map.keys())); retire(current.beginClose());
      const oldIds: number[] = []; for (;;) { const step = oldReader.advance(grant); if (step.kind === "value") { oldIds.push(step.id); const scene = step.value.beginRecord()!; retire(step.value.beginClose()); let seen = false; for (;;) { const result = scene.advance(grant); if (result.kind === "value") seen = true; if (result.kind === "complete") break; } expect(seen).toBe(true); retire(scene.beginClose()); } if (step.kind === "complete") break; } expect(oldIds).toEqual(fixture.indexIds); retire(oldReader.beginClose());
      const paired = create(first); const lease = new OwnedUiNodeReadLease(first, 0, paired); retire(paired.beginClose()); const oldSnapshot = lease.snapshot;
      const initialChild = oldSnapshot.beginSceneRecord()!; const otherChild = oldSnapshot.beginSceneRecord()!; expect(oldSnapshot.beginSceneRecord()).toBeNull();
      const closingChild = initialChild.beginClose(); expect(oldSnapshot.beginSceneRecord()).toBeNull(); retire(closingChild); const replacementChild = oldSnapshot.beginSceneRecord()!; expect(replacementChild).not.toBeNull();
      const newer = create(first); expect(lease.offer(1, newer)).toBe(true); retire(newer.beginClose()); const latest = lease.snapshot; expect(latest).not.toBe(oldSnapshot); const child = latest.beginSceneRecord()!;
      expect(lease.acknowledge(latest)).toBe(true); lease.advanceRetirement(grant); expect(lease.acknowledge(oldSnapshot)).toBe(false); expect(() => oldSnapshot.beginSceneRecord()).toThrow(/retired/); expect(lease.hasCapacity).toBe(false);
      retire(otherChild.beginClose()); retire(replacementChild.beginClose()); for (let turn = 0; lease.retirementPending && turn < 1000; turn++) lease.advanceRetirement(grant); expect(lease.hasCapacity).toBe(true); expect(latest.hasPreparedScene).toBe(true);
      lease.beginClose(); let completedBeforeChild = false;
      for (let turn = 0; turn < 1000; turn++) { const current = lease.closeStep(grant); if (current.kind === "complete") { completedBeforeChild = true; break; } if (current.kind === "blocked") break; }
      retire(child.beginClose()); close(lease); expect(completedBeforeChild).toBe(false);
    });
    //#endregion 🎬️OwnedSceneTests

    it("OwnedSurfaceScene publishes node and prepared scene atomically and waits for issued scene children on close", async () => {
      const { OwnedUiSurface } = await import("@semio-tech/ui-contract/retained/surface");
      const { OwnedUiOperation } = await import("@semio-tech/ui-contract/retained/operations");
      const { RetainedUiTypedCursor } = await import("@semio-tech/ui-contract/retained/wire/typed");
      const fixture = await loadRetainedSceneBindingFixture(); const fields = await loadRetainedWireFieldsFixture(); const { encodePackValue } = await import("@semio-tech/framework/packs");
      const surface = new OwnedUiSurface({ actor: "scene", instance: 7, surface: "window" }, DEFAULT_UI_DOCUMENT_LIMITS, { usizeBits: 64 }); const id = fields.node.id; let notifiedPrepared = false;
      const subscription = surface.subscribeNode(id, () => { if (surface.view.revision) notifiedPrepared = subscription.snapshot!.hasPreparedScene; }); while (surface.maintenancePending) surface.advanceMaintenance(grant); const previous = subscription.snapshot;
      const vector = fixture.cases[0]!; const typed = new RetainedUiTypedCursor(encodePackValue({ ...fields.node, children: [], component: { type: "surface", kind: vector.kind, docSchema: vector.schema, doc: { bytes: vector.packet }, bindings: [] } }), "node"); expect(finish(typed)).toBe("ready"); const payload = typed.takeResult()!; close(typed);
      const patch = surface.beginPatch(0, 1); patch.pushOperation(OwnedUiOperation.upsert(payload)); retire(payload.beginClose()); expect(finish(patch)).toBe("ready"); patch.pushOperation(OwnedUiOperation.setRoot(id)); expect(finish(patch)).toBe("ready"); patch.finishInput();
      for (;;) { const current = patch.advance(grant); if (!surface.view.revision) expect(subscription.snapshot).toBe(previous); if (current.kind === "ready") break; if (current.kind === "rejected") throw new Error(patch.failure ?? "Surface rejected"); }
      const snapshot = subscription.snapshot!; const prepared = snapshot.hasPreparedScene; const reader = snapshot.beginSceneRecord(); expect(patch.takeAcknowledgement()?.revision).toBe(1); close(patch); surface.acknowledgeRead(subscription, snapshot); while (surface.maintenancePending) surface.advanceMaintenance(grant);
      const managed = surface.openSceneRecord(subscription, snapshot); expect(managed).not.toBeNull(); expect(surface.openSceneRecord(subscription, snapshot)).toBeNull(); expect(subscription.snapshot).toBe(snapshot);
      surface.unsubscribeNode(subscription); surface.beginClose(); let early = false; for (let turn = 0; turn < 1000; turn++) { if (surface.closeStep(grant).kind === "complete") { early = true; break; } }
      expect(() => managed!.advance(grant)).toThrow();
      if (!reader) throw new Error("Expected prepared reader");
      const foreign = new OwnedUiSurface(surface.identity, DEFAULT_UI_DOCUMENT_LIMITS, { usizeBits: 64 }); const other = foreign.subscribeNode(id, () => {});
      expect(foreign.retireSceneRead(other, reader)).toBe(false); expect(reader.terminalIsEmpty()).toBe(false); expect(surface.retireSceneRead(subscription, reader)).toBe(true); expect(reader.terminalIsEmpty()).toBe(true); expect(surface.retireSceneRead(subscription, reader)).toBe(false);
      close(surface); foreign.unsubscribeNode(other); close(foreign); expect(prepared).toBe(true); expect(notifiedPrepared).toBe(true); expect(early).toBe(false);
    });

    it("OwnedSurfaceScene React effect cleanup queues exact children across source replacement and unmount", async () => {
      const { OwnedUiSurface } = await import("@semio-tech/ui-contract/retained/surface");
      const { OwnedUiOperation } = await import("@semio-tech/ui-contract/retained/operations");
      const { RetainedUiTypedCursor } = await import("@semio-tech/ui-contract/retained/wire/typed");
      const { useOwnedUiScene } = await import("../../📖️owned/🟦️.tsx");
      const fixture = await loadRetainedSceneBindingFixture(); const fields = await loadRetainedWireFieldsFixture(); const { encodePackValue } = await import("@semio-tech/framework/packs");
      const { render, act, cleanup } = await import("@testing-library/react"); const { createElement } = await import("react"); const vector = fixture.cases[0]!;
      const create = () => {
        const surface = new OwnedUiSurface({ actor: "effect", instance: 7, surface: "window" }, DEFAULT_UI_DOCUMENT_LIMITS, { usizeBits: 64 });
        const typed = new RetainedUiTypedCursor(encodePackValue({ ...fields.node, children: [], component: { type: "surface", kind: vector.kind, docSchema: vector.schema, doc: { bytes: vector.packet }, bindings: [] } }), "node"); expect(finish(typed)).toBe("ready"); const payload = typed.takeResult()!; close(typed);
        const patch = surface.beginPatch(0, 1); patch.pushOperation(OwnedUiOperation.upsert(payload)); retire(payload.beginClose()); expect(finish(patch)).toBe("ready"); patch.pushOperation(OwnedUiOperation.setRoot(fields.node.id)); expect(finish(patch)).toBe("ready"); patch.finishInput(); expect(finish(patch)).toBe("ready"); expect(patch.takeAcknowledgement()?.revision).toBe(1); close(patch); return surface;
      };
      const first = create(); const second = create(); const views: import("../../📖️owned/🟦️.tsx").OwnedUiSceneEffectView[] = []; const readers: import("@semio-tech/ui-contract/retained/read-lease").OwnedUiSceneRecordView[] = []; let cleanups = 0;
      const consume = (view: import("../../📖️owned/🟦️.tsx").OwnedUiSceneEffectView) => { views.push(view); const a = view.openRecord(); const b = view.openRecord(); expect(a).not.toBeNull(); expect(b).not.toBeNull(); expect(view.openRecord()).toBeNull(); readers.push(a!, b!); return () => { cleanups++; }; };
      function View({ source }: { source: typeof first }) { const record = useOwnedUiScene(source, fields.node.id, consume); return createElement("span", { "aria-label": "Szene / Scene" }, record ? String(record.id) : "pending"); }
      const root = render(createElement(View, { source: first })); await act(async () => { while (first.maintenancePending) first.advanceMaintenance(grant); }); expect(root.getByLabelText("Szene / Scene").textContent).toBe(String(fields.node.id)); expect(views.length).toBe(1);
      root.rerender(createElement(View, { source: second })); close(first); expect(() => readers[0]!.advance(grant)).toThrow(); expect(views[0]!.openRecord()).toBeNull();
      await act(async () => { while (second.maintenancePending) second.advanceMaintenance(grant); }); expect(views.length).toBe(2); expect(readers[0]!.close()).toBe(false); expect(readers[2]!.advance(grant).kind).not.toBe("rejected");
      second.beginClose(); root.unmount(); close(second); expect(() => readers[2]!.advance(grant)).toThrow(); expect(cleanups).toBe(2); expect(first.terminalIsEmpty() && second.terminalIsEmpty()).toBe(true); cleanup();
    });
  });
  //#endregion 🧾️TypedWireTests
}
