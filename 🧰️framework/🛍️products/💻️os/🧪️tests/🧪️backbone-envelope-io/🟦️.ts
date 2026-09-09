type TestSource = { readonly directory: string; readonly url: string };

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: any, source: TestSource): Promise<void> {
  const { BACKBONE_ENVELOPE_RETRY_WINDOW_MS, readBackboneEnvelope, writeBackboneEnvelope } = dependencies;

  const { afterEach, describe, expect, it, vi } = vitest;

  describe("backbone envelope io", () => {
    const originalFetch = globalThis.fetch;
    afterEach(() => {
      globalThis.fetch = originalFetch;
      vi.useRealTimers();
    });

    it("readBackboneEnvelope retries a transient transport failure and then succeeds, with no real sleep", async () => {
      vi.useFakeTimers();
      let calls = 0;
      globalThis.fetch = vi.fn(async () => {
        calls += 1;
        if (calls < 3) throw new Error("connection refused");
        return { ok: true, status: 200, arrayBuffer: async () => new Uint8Array([1, 2, 3]).buffer } as unknown as Response;
      }) as unknown as typeof fetch;
      const promise = readBackboneEnvelope("folder:///doc");
      await vi.runAllTimersAsync();
      const result = await promise;
      expect(calls).toBe(3);
      expect(Array.from(result ?? [])).toEqual([1, 2, 3]);
    });

    it("readBackboneEnvelope does NOT retry a definitive non-404 server response", async () => {
      let calls = 0;
      globalThis.fetch = vi.fn(async () => {
        calls += 1;
        return { ok: false, status: 500, arrayBuffer: async () => new ArrayBuffer(0) } as unknown as Response;
      }) as unknown as typeof fetch;
      // 🪧️ no intervening `await` between creating the promise and `.rejects` consuming it — a
      // definitive failure settles in one microtask hop, with no timer involved at all.
      await expect(readBackboneEnvelope("folder:///doc")).rejects.toThrow("backbone read failed (500)");
      expect(calls).toBe(1);
    });

    it("readBackboneEnvelope gives up after its retry window instead of hanging forever", async () => {
      vi.useFakeTimers();
      globalThis.fetch = vi.fn(async () => {
        throw new Error("connection refused");
      }) as unknown as typeof fetch;
      const promise = readBackboneEnvelope("folder:///doc");
      let settled = false;
      promise.then(
        () => (settled = true),
        () => (settled = true),
      );
      await vi.advanceTimersByTimeAsync(BACKBONE_ENVELOPE_RETRY_WINDOW_MS + 1_000);
      expect(settled).toBe(true);
      await expect(promise).rejects.toThrow();
    });

    it("readBackboneEnvelope returns null on 404 without retrying", async () => {
      let calls = 0;
      globalThis.fetch = vi.fn(async () => {
        calls += 1;
        return { ok: false, status: 404, arrayBuffer: async () => new ArrayBuffer(0) } as unknown as Response;
      }) as unknown as typeof fetch;
      const result = await readBackboneEnvelope("folder:///doc");
      expect(result).toBeNull();
      expect(calls).toBe(1);
    });

    it("writeBackboneEnvelope does NOT retry on transport failure (duplicate-write safety)", async () => {
      let calls = 0;
      globalThis.fetch = vi.fn(async () => {
        calls += 1;
        throw new Error("connection refused");
      }) as unknown as typeof fetch;
      await expect(writeBackboneEnvelope("folder:///doc", new Uint8Array([1]))).rejects.toThrow("connection refused");
      expect(calls).toBe(1);
    });

    it("writeBackboneEnvelope propagates an external abort promptly with no leaked timer", async () => {
      const controller = new AbortController();
      globalThis.fetch = vi.fn((_url: string, init?: RequestInit) => {
        return new Promise((_resolve, reject) => {
          init?.signal?.addEventListener("abort", () => reject(init.signal!.reason ?? new Error("aborted")));
        });
      }) as unknown as typeof fetch;
      const promise = writeBackboneEnvelope("folder:///doc", new Uint8Array([1]), controller.signal);
      controller.abort(new Error("caller cancelled"));
      await expect(promise).rejects.toThrow("caller cancelled");
    });
  });

}

export async function registerTests2(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: any, source: TestSource): Promise<void> {
  const { APP_CHANNEL_VERSION, AppChannelClient, AppChannelRequestSequence, INVOCATION_RESULT_PACK_MAXIMUM_BYTES, applyBackboneMessage, backboneKindFromUri, buildFileBackboneUri, buildFolderBackboneUri, buildFrameworkSyncUtilities, buildRemoteBackboneUri, clonePackValue, createTurnOutcomeBroadcast, decodeAppCommand, decodeAppFrame, decodeBackboneMessage, decodeConflictsFromWire, decodeDispatchReportFromWire, decodeDocumentPackBytes, decodeDocumentPackSnapshot, decodeInvocationResultPacks, decodeMergeReportFromWire, decodePackValue, decodePresencePeer, decodeScenePackValue, encodeAppCommand, encodeAppFrame, encodeBackboneMessage, encodeDocumentPackBundle, encodeDocumentPackBytes, encodePackValue, encodePresencePeer, faultMessages, isPackByteVector, isPackInteger, packInt, packUInt, packValueToExactJson, parseRemoteBackboneUri, planWorkflow } = dependencies;
  type AppChannelHandle = any;
  type AppCommandValue = any;
  type AppFrameValue = any;
  type ArtifactPresencePeer = any;
  type BinaryBackboneMessage = any;
  type Conflict = any;
  type DispatchReport = any;
  type LocalInteractionIdentity = any;
  type LocalInteractionQueryReply = any;
  type LocalInteractionQueryToken = any;
  type MediaContract = any;
  type MergeReport = any;
  type OsWorkflow = any;
  type OsWorkflowNode = any;
  type PackInteger = any;
  type PackValue = any;
  type TurnOutcome = any;

  const { describe, expect, it } = vitest;

  describe("@semio-tech/framework-os backbone", () => {
    it("classifies backbone uri kinds", () => {
      expect(backboneKindFromUri("file:///tmp/a.json")).toBe("file");
      expect(backboneKindFromUri("folder:///tmp")).toBe("folder");
      expect(backboneKindFromUri("remote://host:1234/doc-1")).toBe("remote");
      expect(backboneKindFromUri("other://x")).toBe("unknown");
    });

    it("builds and parses backbone uris", () => {
      expect(buildFileBackboneUri("tmp/a.json")).toBe("file:///tmp/a.json");
      expect(buildFolderBackboneUri("tmp")).toBe("folder:///tmp");
      expect(buildRemoteBackboneUri("localhost:1234", "studio-1", "doc-1")).toBe("remote://localhost:1234/studio-1/doc-1");
      expect(parseRemoteBackboneUri("remote://localhost:1234/studio-1/doc-1")).toEqual({ hostPort: "localhost:1234", spaceId: "studio-1", documentId: "doc-1" });
      expect(parseRemoteBackboneUri("remote://localhost:1234/doc-1")).toBeNull();
      expect(parseRemoteBackboneUri("file:///tmp/a.json")).toBeNull();
    });

    it("packs and unpacks document bundles", () => {
      const bundle = encodeDocumentPackBundle({ nodes: [] });
      expect(decodeDocumentPackSnapshot(bundle)).toEqual({ nodes: [] });
    });

    it("round-trips backbone snapshot messages", () => {
      const message: BinaryBackboneMessage = { kind: "snapshot", pack: new Uint8Array([1, 2]), spr: new Uint8Array([3]) };
      const round = decodeBackboneMessage(encodeBackboneMessage(message));
      expect(round.kind).toBe("snapshot");
      if (round.kind !== "snapshot") return;
      expect(Array.from(round.pack)).toEqual([1, 2]);
      expect(Array.from(round.spr)).toEqual([3]);
    });

    it("applies a snapshot backbone message by overwriting the stored bundle", () => {
      const snapshot = encodeBackboneMessage({ kind: "snapshot", pack: new Uint8Array([9]), spr: new Uint8Array() });
      const result = applyBackboneMessage(encodeDocumentPackBytes(new Uint8Array([1]), new Uint8Array()), snapshot);
      expect(decodeDocumentPackBytes(result).pack).toEqual(new Uint8Array([9]));
    });

    it("throws when applying operations without native store", () => {
      const message = encodeBackboneMessage({ kind: "mutations", envelopes: Uint8Array.of(0) });
      expect(() => applyBackboneMessage(encodeDocumentPackBytes(new Uint8Array(), new Uint8Array()), message)).toThrow("native store");
    });

    it("throws when applying operations before a snapshot exists", () => {
      const message = encodeBackboneMessage({ kind: "mutations", envelopes: Uint8Array.of(0) });
      expect(() => applyBackboneMessage(null, message)).toThrow("cannot append operations before a snapshot exists");
    });

    it("throws on an unknown backbone message tag", () => {
      expect(() => decodeBackboneMessage(new Uint8Array([99]))).toThrow("invalid format");
    });

    it("builds sync utilities reflecting the active backbone kind", () => {
      const utilities = buildFrameworkSyncUtilities("folder:///tmp");
      expect(utilities.map((utility) => utility.id)).toEqual(["framework.sync.file", "framework.sync.folder", "framework.sync.remote"]);
      expect(utilities.find((utility) => utility.id === "framework.sync.folder")?.pressed).toBe(true);
      expect(utilities.find((utility) => utility.id === "framework.sync.file")?.pressed).toBe(false);
    });
  });

  describe("@semio-tech/framework-os workflow", () => {
    const mediaContract = (): MediaContract => ({ kindId: "2d.drawing", mediaType: { class: "data", form: "value" }, wire: { kind: "document", schema: "2d.drawing" } });
    const mediaNode = (id: string, instanceId: string): OsWorkflowNode => ({
      id,
      instanceId,
      x: 0,
      y: 0,
      width: 160,
      height: 72,
      inputs: [{ id: `${instanceId}:in`, artifactKind: "2d.drawing", direction: "in" }],
      outputs: [{ id: `${instanceId}:out`, artifactKind: "2d.drawing", direction: "out" }],
    });

    it("plans a single delivery across one dirty edge", () => {
      const graph: OsWorkflow = {
        schema: "os.workflow",
        nodes: [mediaNode("node-1", "app-1"), mediaNode("node-2", "app-2")],
        edges: [{ id: "edge-1", sourceNodeId: "node-1", sourcePortId: "app-1:out", targetNodeId: "node-2", targetPortId: "app-2:in", contract: mediaContract() }],
      };
      const deliveries = planWorkflow(graph, new Set(["app-1"]));
      expect(deliveries).toEqual([{ edgeId: "edge-1", producerInstanceId: "app-1", producerPortId: "app-1:out", consumerInstanceId: "app-2", consumerPortId: "app-2:in" }]);
    });

    it("plans a chain in topological order when only the root is dirty", () => {
      const graph: OsWorkflow = {
        schema: "os.workflow",
        nodes: [mediaNode("node-1", "app-1"), mediaNode("node-2", "app-2"), mediaNode("node-3", "app-3")],
        edges: [
          { id: "edge-ab", sourceNodeId: "node-1", sourcePortId: "app-1:out", targetNodeId: "node-2", targetPortId: "app-2:in", contract: mediaContract() },
          { id: "edge-bc", sourceNodeId: "node-2", sourcePortId: "app-2:out", targetNodeId: "node-3", targetPortId: "app-3:in", contract: mediaContract() },
        ],
      };
      const deliveries = planWorkflow(graph, new Set(["app-1"]));
      expect(deliveries.map((delivery) => delivery.edgeId)).toEqual(["edge-ab", "edge-bc"]);
    });

    it("plans a diamond with one delivery per incoming edge", () => {
      const graph: OsWorkflow = {
        schema: "os.workflow",
        nodes: [mediaNode("node-1", "app-a"), mediaNode("node-2", "app-b"), mediaNode("node-3", "app-c"), mediaNode("node-4", "app-d")],
        edges: [
          { id: "edge-ab", sourceNodeId: "node-1", sourcePortId: "app-a:out", targetNodeId: "node-2", targetPortId: "app-b:in", contract: mediaContract() },
          { id: "edge-ac", sourceNodeId: "node-1", sourcePortId: "app-a:out", targetNodeId: "node-3", targetPortId: "app-c:in", contract: mediaContract() },
          { id: "edge-bd", sourceNodeId: "node-2", sourcePortId: "app-b:out", targetNodeId: "node-4", targetPortId: "app-d:in", contract: mediaContract() },
          { id: "edge-cd", sourceNodeId: "node-3", sourcePortId: "app-c:out", targetNodeId: "node-4", targetPortId: "app-d:in", contract: mediaContract() },
        ],
      };
      const deliveries = planWorkflow(graph, new Set(["app-a"]));
      const edgeIds = deliveries.map((delivery) => delivery.edgeId);
      expect(edgeIds).toHaveLength(4);
      expect(edgeIds.indexOf("edge-bd")).toBeGreaterThan(edgeIds.indexOf("edge-ab"));
      expect(edgeIds.indexOf("edge-cd")).toBeGreaterThan(edgeIds.indexOf("edge-ac"));
    });

    it("plans nothing when no instance is dirty", () => {
      const graph: OsWorkflow = {
        schema: "os.workflow",
        nodes: [mediaNode("node-1", "app-1"), mediaNode("node-2", "app-2")],
        edges: [{ id: "edge-1", sourceNodeId: "node-1", sourcePortId: "app-1:out", targetNodeId: "node-2", targetPortId: "app-2:in", contract: mediaContract() }],
      };
      expect(planWorkflow(graph, new Set())).toEqual([]);
    });

    it("plans nothing for a dirty node with no outgoing edges", () => {
      const graph: OsWorkflow = { schema: "os.workflow", nodes: [mediaNode("node-1", "app-1")], edges: [] };
      expect(planWorkflow(graph, new Set(["app-1"]))).toEqual([]);
    });

    // 🔬️ Rust owns semantic DSL/SPK decoding and canonical equivalence. This language-neutral check
    // keeps the source corpus paired without depending on a browser ABI or a generated wasm package.
    it("pairs every shared workflow DSL fixture with a pack fixture", async () => {
      const { readdirSync, readFileSync } = await import("node:fs");
      const { fileURLToPath } = await import("node:url");
      const { dirname, join } = await import("node:path");
      const here = dirname(fileURLToPath(source.url));
      const fixturesDir = join(here, "🧫️fixtures");
      const owners = readdirSync(fixturesDir, { withFileTypes: true }).filter((entry) => entry.isDirectory() && readdirSync(join(fixturesDir, entry.name)).some((name) => name === "🗣️.dsl" || name === "📦️.spk"));
      expect(owners.length).toBeGreaterThanOrEqual(5);
      for (const owner of owners) {
        expect(readFileSync(join(fixturesDir, owner.name, "🗣️.dsl"), "utf8").length).toBeGreaterThan(0);
        expect(readFileSync(join(fixturesDir, owner.name, "📦️.spk")).byteLength).toBeGreaterThan(0);
      }
    });
  });

  describe("@semio-tech/framework-os PackValueCodec", () => {
    function bytesToHex(bytes: Uint8Array): string {
      return Array.from(bytes)
        .map((byte) => byte.toString(16).padStart(2, "0"))
        .join("");
    }
    function hexToBytes(hex: string): Uint8Array {
      const out = new Uint8Array(hex.length / 2);
      for (let i = 0; i < out.length; i++) out[i] = Number.parseInt(hex.substring(i * 2, i * 2 + 2), 16);
      return out;
    }

    // 🔬️ Ground truth captured verbatim from `cargo test -p semio-framework-os-kernel
    // pack_wire_value_fixture_corpus_hex_dump -- --nocapture` (`store/rs/lib.rs`'s
    // `🔖️PackValueFixtures` region) — the REAL bytes `pack_rt::encode_wire_value` produces (the
    // `encode_record_body`-backed sibling of `encode_json_value`, see this file's
    // `🔖️PackValueCodec` header doc for why the container-backed encoding was replaced).
    // `encode_record_body`'s grammar has no compression anywhere it is fully deterministic, so
    // both `encodePackValue` and `decodePackValue` are asserted BYTE-EXACT against these, unlike
    // the old DEFLATE-backed encoding this replaced (which was only decode-exact). The corpus's
    // integer rows are `DslValue::uint`/`int` on the Rust side and therefore `TAG_UINT`/`TAG_INT`
    // here — a JS `number` is `TAG_F64` and could not carry them.
    const packValueFixtures: ReadonlyArray<readonly [string, PackValue, string]> = [
      ["null", null, "0001011112"],
      ["bool_true", true, "0001011102"],
      ["bool_false", false, "0001011101"],
      ["int_zero", packUInt(0n), "000101110400"],
      ["int_negative_one", packInt(-1n), "000101110301"],
      ["float_pi", 3.14, "00010111051f85eb51b81e0940"],
      ["float_whole_number", 2.0, "00010111050000000000000040"],
      ["string_empty", "", "01000101110600"],
      ["string_escapes", 'hello\nworld with "quotes"', "011968656c6c6f0a776f726c642077697468202271756f746573220101110600"],
      ["array_empty", [], "000101110c00"],
      ["array_ints", [packUInt(1n), packUInt(2n), packUInt(3n)], "000101110c03040104020403"],
      ["object_empty", {}, "000101111000"],
      ["object_mixed", { a: packUInt(1n), b: [true, null] }, "00010111100207016104010701620c020212"],
      [
        "nested_deep",
        { a: { b: { c: [packUInt(1n), packUInt(2n), { d: "leaf" }] } } },
        "01046c6561660101111001070161100107016210010701630c030401040210010701640600",
      ],
    ];

    it.each(packValueFixtures)("decodes real Rust encode_wire_value bytes for %s", (_name, expected, hex) => {
      expect(decodePackValue(hexToBytes(hex))).toEqual(expected);
    });

    it.each(packValueFixtures)("encodes byte-exact against real Rust encode_wire_value output for %s", (_name, value, hex) => {
      expect(bytesToHex(encodePackValue(value))).toBe(hex);
    });

    it.each(packValueFixtures)("round-trips %s through encodePackValue/decodePackValue", (_name, value) => {
      expect(decodePackValue(encodePackValue(value))).toEqual(value);
    });
  });

  describe("@semio-tech/framework-os PackValueCodec dynamic integers", () => {
    function bytesToHex(bytes: Uint8Array): string {
      return Array.from(bytes)
        .map((byte) => byte.toString(16).padStart(2, "0"))
        .join("");
    }
    function hexToBytes(hex: string): Uint8Array {
      const out = new Uint8Array(hex.length / 2);
      for (let i = 0; i < out.length; i++) out[i] = Number.parseInt(hex.substring(i * 2, i * 2 + 2), 16);
      return out;
    }
    // 🔣️ The shared, language-neutral corpus both hosts read — see
    // `💻️os/🧫️fixtures/🎒️pack-dynamic-integer-v1`. Its `wireHex` is generated by an independent
    // BigInt/LEB128 oracle and asserted by the native law
    // `pack_wire_value_preserves_integer_variants_at_u64_i64_boundaries`.
    function corpusValue(node: Record<string, any>): PackValue {
      if (typeof node.uint === "string") return packUInt(BigInt(node.uint));
      if (typeof node.int === "string") return packInt(BigInt(node.int));
      if (typeof node.f64LeHex === "string") return new DataView(hexToBytes(node.f64LeHex).buffer).getFloat64(0, true);
      if (Array.isArray(node.list)) return node.list.map(corpusValue);
      if (Array.isArray(node.map)) return Object.fromEntries((node.map as [string, Record<string, any>][]).map(([key, value]) => [key, corpusValue(value)]));
      throw new Error(`unsupported corpus node ${JSON.stringify(node)}`);
    }

    it("validates the neutral corpus against its own schema", async () => {
      const { default: Ajv } = await import("ajv");
      const [{ default: corpus }, { default: schema }] = await Promise.all([import("../../🧫️fixtures/🎒️pack-dynamic-integer-v1/🔣️.json"), import("../../🔨️modules/🎒️pack/🌱️value/🧬️schema/🔣️.json")]);
      const packValueExport = new Ajv({ strict: true, allErrors: true }).addSchema(schema).getSchema(`${(schema as { $id: string }).$id}#/$defs/PackDynamicIntegerV1`)!;
      expect(packValueExport(corpus)).toBe(true);
      expect(corpus.accept).toHaveLength(6);
      expect(corpus.reject.map((row: { id: string }) => row.id)).toEqual(["truncated-u64", "u64-overflow", "nonminimal-u64", "nonminimal-zigzag-i64"]);
    });

    it("encodes and decodes every accepted corpus row byte-exactly with its exact carrier kind", async () => {
      const { default: corpus } = await import("../../🧫️fixtures/🎒️pack-dynamic-integer-v1/🔣️.json");
      for (const row of corpus.accept) {
        const value = corpusValue(row.value);
        expect(bytesToHex(encodePackValue(value)), row.id).toBe(row.wireHex);
        const decoded = decodePackValue(hexToBytes(row.wireHex));
        expect(decoded, row.id).toEqual(value);
        if (row.variant === "uint" || row.variant === "int") {
          expect(isPackInteger(decoded)).toBe(true);
          expect((decoded as PackInteger).kind).toBe(row.variant);
          const magnitude = row.value[row.variant];
          if (typeof magnitude !== "string") throw new Error("integer corpus magnitude is missing");
          expect((decoded as PackInteger).value).toBe(BigInt(magnitude));
        }
        expect(bytesToHex(encodePackValue(decoded)), row.id).toBe(row.wireHex);
      }
    });

    it("rejects every corpus reject row at the documented boundary", async () => {
      const { default: corpus } = await import("../../🧫️fixtures/🎒️pack-dynamic-integer-v1/🔣️.json");
      for (const row of corpus.reject) {
        const bytes = hexToBytes(row.wireHex);
        if (row.outcome === "decode-error") expect(() => decodePackValue(bytes), row.id).toThrow();
        else {
          const decoded = decodePackValue(bytes);
          if (!row.decodes) throw new Error("noncanonical corpus decoded value is missing");
          expect(decoded, row.id).toEqual(corpusValue(row.decodes));
          expect(bytesToHex(encodePackValue(decoded)), row.id).not.toBe(row.wireHex);
        }
      }
    });

    it("keeps int and uint distinct, survives nesting, and refuses unminted carriers", () => {
      expect(bytesToHex(encodePackValue(packInt(7n)))).not.toBe(bytesToHex(encodePackValue(packUInt(7n))));
      expect((decodePackValue(encodePackValue(packInt(7n))) as PackInteger).kind).toBe("int");
      const nested = { u: packUInt((1n << 64n) - 1n), rows: [packInt(-(2n ** 63n)), { deep: packUInt(2n ** 53n + 1n) }] };
      const round = decodePackValue(encodePackValue(nested)) as Record<string, PackValue>;
      expect(round).toEqual(nested);
      expect(bytesToHex(encodePackValue(round))).toBe(bytesToHex(encodePackValue(nested)));
      expect(isPackInteger({ kind: "uint", value: 1n })).toBe(false);
      expect(() => encodePackValue({ kind: "uint", value: 1n } as unknown as PackValue)).toThrow("unminted");
      expect(() => packUInt(-1n)).toThrow("u64");
      expect(() => packInt(2n ** 63n)).toThrow("i64");
      expect(() => packUInt(2n ** 64n)).toThrow("u64");
    });

    it("clones carriers structurally and projects only safe integers onto JSON", () => {
      const source = { id: packUInt(2n ** 53n + 1n), rows: [packInt(-7n)] };
      const cloned = clonePackValue(source) as typeof source;
      expect(cloned).toEqual(source);
      expect(isPackInteger(cloned.id)).toBe(true);
      expect(isPackInteger((structuredClone(source) as typeof source).id)).toBe(false);
      expect(packValueToExactJson({ safe: packUInt(9007199254740991n), signed: packInt(-7n), plain: 1.5 })).toEqual({ safe: 9007199254740991, signed: -7, plain: 1.5 });
      expect(() => packValueToExactJson({ big: packUInt(2n ** 53n + 1n) })).toThrow("exactly");
      expect(isPackByteVector([0, 255])).toBe(true);
      expect(isPackByteVector([packUInt(1n)])).toBe(false);
      expect(isPackByteVector([1.5])).toBe(false);
      expect(isPackByteVector([256])).toBe(false);
    });

    it("keeps a negative zero's sign bit, byte-for-byte with Rust normalize_f64", () => {
      expect(bytesToHex(encodePackValue(-0))).toBe("00010111050000000000000080");
      expect(Object.is(decodePackValue(hexToBytes("00010111050000000000000080")), -0)).toBe(true);
    });
  });

  describe("@semio-tech/framework-os ScenePackCodec", () => {
    it("decodes the byte-exact Rust TableScene fixture without a schema-specific field mirror", () => {
      const hex = "0d02060b636f6c756d6e734a736f6e060f5b7b226964223a226e616d65227d5d0608726f77734a736f6e06025b5d";
      const bytes = new Uint8Array(hex.length / 2);
      for (let index = 0; index < bytes.length; index += 1) bytes[index] = Number.parseInt(hex.slice(index * 2, index * 2 + 2), 16);
      expect(decodeScenePackValue(bytes)).toEqual({ columnsJson: '[{"id":"name"}]', rowsJson: "[]" });
    });
  });

  describe("@semio-tech/framework-os AppChannelCodec", () => {
    it("round-trips the app-typed presence pack through the document-presence wire", () => {
      const peer: ArtifactPresencePeer = {
        actor: "actor-1",
        connectedAtMs: 42,
        label: "One",
        presencePack: [1, 2, 3],
        color: 4,
        surface: "s.space.home@1/*#editor",
        views: [{ windowId: "w1", space: "canvas", kind: { kind: "canvas", x: 1, y: 2, zoom: 1.5 }, size: [800, 600], pointer: [10, 20, 0] }],
        ui: { hoveredPath: "row[0]#a" },
      };
      expect(decodePresencePeer(new Uint8Array(encodePresencePeer(peer)), [0])).toEqual(peer);
    });

    const sampleCommands: readonly AppCommandValue[] = [
      { ConfigCommand: { seq: 1, command: [4, 5] } },
      { Command: { seq: 2, command: [1], view_state: [2, 3] } },
      { CommandText: { seq: 3, line: "move 1 2" } },
      { ContextMenu: { seq: 5, request: [9, 9] } },
      { ArtifactCommand: { seq: 6, command: [7] } },
      { ApplyEnvelopes: { seq: 7, envelopes: [] } },
      { LoadDocument: { seq: 8, pack: [1, 2, 3], spr: [4, 5, 6] } },
      { ReadDocument: { seq: 9 } },
      { LoadConfig: { seq: 10, pack: [1], spr: [2] } },
      { ReadConfig: { seq: 11 } },
      { LoadWindowConfig: { seq: 12, entry: { window_id: "w1", window_kind_id: "graph", envelope_pack: [1, 2] } } },
      { ReadWindowConfigs: { seq: 13 } },
      { MediaIn: { seq: 14, port: "in-1", descriptor: [1], data: [2, 3] } },
      { MediaOut: { seq: 15, port: "out-1", request: [4] } },
      { MediaFingerprint: { seq: 16, port: "fp-1" } },
      { PureCommand: { seq: 17, command: [1], document: [2], document_spr: [3], config: [4], config_spr: [5], draft: [6], draft_spr: [7] } },
      { LoadChildren: { seq: 18, entries: [{ slot: "s", child_id: "c", dialect: "d", envelope_pack: [1] }] } },
      { ReadChildren: { seq: 19 } },
      { ReadHistory: { seq: 20 } },
      { transactionPrepare: { seq: 21, txn_id: "txn-1", mutation_id: "s.demo#kind", payload: [1, 2], prepared_ops: [], label: "", origin: [] } },
      { transactionPrepare: { seq: 22, txn_id: "txn-1", mutation_id: "", payload: [], prepared_ops: [[1], [2, 2]], label: "step-1", origin: [9] } },
      { transactionCommit: { seq: 23, txn_id: "txn-1" } },
      { transactionRollback: { seq: 24, txn_id: "txn-1" } },
      { transactionUndo: { seq: 25, group_id: "grp-1" } },
      { transactionRedo: { seq: 26, group_id: "grp-1" } },
      { openArtifact: { seq: 27, artifact_ref: "s.cad.cad@1/*#viewer", role: 0, plugin_id: "", app_id: "" } },
      { openArtifact: { seq: 28, artifact_ref: "s.cad.cad@1/*#editor", role: 1, plugin_id: "cad", app_id: "s.cad.cad@1/*#editor" } },
      { setDefaultApp: { seq: 29, artifact_kind: "s.cad.cad", standard: "1", subset: "*", role: 1, plugin_id: "cad", app_id: "s.cad.cad@1/*#editor" } },
      { clearDefaultApp: { seq: 30, artifact_kind: "s.cad.cad", standard: "1", subset: "*", role: 0 } },
      { setMergePolicy: { seq: 31, policy: 1 } },
      { resolveConflict: { seq: 32, conflict_id: "conflict-1", resolution: 0 } },
      { readConflicts: { seq: 33 } },
      { presence: { seq: 34, own_color: 3, peers: [[1, 2], [9]] } },
      { presence: { seq: 35, own_color: null, peers: [] } },
    ];

    const sampleFrames: readonly AppFrameValue[] = [
      { Done: { in_reply_to: 1 } },
      { Invocation: { in_reply_to: 2, output: [1], diagnostics: [], ui_scope: [], history_patch: [], messages: [], mutations: [], inverse_group: [] } },
      { Invocation: { in_reply_to: 2, output: [1], diagnostics: [], ui_scope: [], history_patch: [], messages: [9], mutations: [10], inverse_group: [11] } },
      { DocumentChanged: { envelopes: [[1, 2]], origin: "remote" } },
      { Document: { in_reply_to: 6, pack: [1, 2], spr: [3, 4], ops: "op-log" } },
      { WindowConfigs: { in_reply_to: 6, entries: [{ window_id: "w1", window_kind_id: "graph", envelope_pack: [1, 2] }] } },
      { ContextMenu: { in_reply_to: 7, items: [1, 2, 3] } },
      { Media: { in_reply_to: 8, port: "out-1", descriptor: [1], data: [2] } },
      { MediaFingerprint: { in_reply_to: 9, port: "fp-1", fingerprint: [1, 2, 3, 4] } },
      { Error: { in_reply_to: 10, fault: [1, 2, 3], report: [6] } },
      { Error: { in_reply_to: null, fault: [4, 5], report: [] } },
      { Emit: { in_reply_to: 11, document_ops: [1], config_ops: [2], draft_ops: [3], output: [4], diagnostics: [5] } },
      { Draft: { in_reply_to: 12, pack: [1], spr: [2], ops: "d" } },
      { Children: { in_reply_to: 13, entries: [{ slot: "s", child_id: "c", dialect: "d", envelope_pack: [1] }] } },
      { Ephemeral: { presence: [1, 2], presence_generation: 3, transient_generation: 4, interaction: [7] } },
      { Ephemeral: { presence: [1, 2], presence_generation: 3, transient_generation: 4, interaction: [] } },
      { HistorySnapshot: { in_reply_to: 14, history_patch: [1] } },
      { transactionProposal: { in_reply_to: 15, proposal_id: "prop-1", local_ops: [[1]], description: "move", coalesce_key: "k-1", foreign: [[2, 3]] } },
      { transactionPrepared: { txn_id: "txn-1", foreign: [[1]], rejection: [] } },
      { transactionPrepared: { txn_id: "txn-1", foreign: [], rejection: [1, 2] } },
      { transactionCommitted: { txn_id: "txn-1", edit_id: "edit-1" } },
      { transactionRolledBack: { txn_id: "txn-1" } },
      { MergeReport: { in_reply_to: 16, report: [1, 2] } },
      { MergeReport: { in_reply_to: null, report: [] } },
      { Conflicts: { in_reply_to: 17, conflicts: [3] } },
      { Conflicts: { in_reply_to: null, conflicts: [] } },
      { UiPatch: { in_reply_to: 18, surface: "1:body", kind: "window", revision: 2, base_revision: 1, ops: [3] } },
      { UiPatch: { in_reply_to: null, surface: "1:body", kind: "window", revision: 1, base_revision: 0, ops: [] } },
      { UiSnapshotEnd: { revision: 5 } },
    ];

    it("projects exact packed invocation mutations and inverse ownership without legacy field aliases", () => {
      const inverse = { targetMutation: "mutation-1", inverseDiff: { schema: "s.map.patch@1", payload: [9] }, baseVersion: 7, undoPolicy: "ExactBaseOnly" } as const;
      const mutation = {
        id: "mutation-1",
        document: "340282366920938463463374607431768211455",
        baseVersion: 7,
        invocationId: "invocation-1",
        diff: { schema: "s.map.patch@1", payload: [1, 2] },
        inverse,
        author: "actor-1",
        timestamp: { actor: 3, physical_ms: 5, logical: 1 },
      } as const;
      const inverseGroup = { invocationId: "invocation-1", mutations: ["mutation-1"], inverseMutations: [inverse], memberEdits: [{ document: mutation.document, editId: "edit-1" }] } as const;
      expect(decodeInvocationResultPacks({ mutations: encodePackValue([mutation]), inverse_group: encodePackValue(inverseGroup) })).toEqual({ mutations: [mutation], inverseGroup });
      expect(() => decodeInvocationResultPacks({ mutations: encodePackValue([{ ...mutation, artifact: 1 }]), inverse_group: encodePackValue(inverseGroup) })).toThrow("invalid exact fields");
      expect(() => decodeInvocationResultPacks({ mutations: encodePackValue([mutation]), inverse_group: [] })).toThrow("must be published together");
    });

    it("rejects invocation result packs outside the command transport byte authority", () => {
      const oversized = new Array<number>(INVOCATION_RESULT_PACK_MAXIMUM_BYTES + 1).fill(0);
      expect(() => encodeAppFrame({ Invocation: { in_reply_to: 1, output: [], diagnostics: [], ui_scope: [], history_patch: [], messages: [], mutations: oversized, inverse_group: [] } })).toThrow("exceeds command transport authority");
      expect(() => decodeInvocationResultPacks({ mutations: oversized, inverse_group: [] })).toThrow("exceeds command transport authority");
    });

    it.each(sampleCommands.map((cmd) => [cmd] as const))("round-trips AppCommand %j", (cmd) => {
      expect(decodeAppCommand(encodeAppCommand(cmd))).toEqual(cmd);
    });

    it.each(sampleFrames.map((frame) => [frame] as const))("round-trips AppFrame %j", (frame) => {
      expect(decodeAppFrame(encodeAppFrame(frame))).toEqual(frame);
    });

    it("tags every AppCommand variant per the agreed contract order", () => {
      expect(encodeAppCommand({ ConfigCommand: { seq: 0, command: [] } })[0]).toBe(0);
      expect(encodeAppCommand({ Command: { seq: 0, command: [], view_state: [] } })[0]).toBe(1);
      expect(encodeAppCommand({ ReadChildren: { seq: 0 } })[0]).toBe(15);
      expect(encodeAppCommand({ ReadHistory: { seq: 0 } })[0]).toBe(16);
      expect(encodeAppCommand({ transactionPrepare: { seq: 0, txn_id: "", mutation_id: "", payload: [], prepared_ops: [], label: "", origin: [] } })[0]).toBe(17);
      expect(encodeAppCommand({ transactionCommit: { seq: 0, txn_id: "" } })[0]).toBe(18);
      expect(encodeAppCommand({ transactionRollback: { seq: 0, txn_id: "" } })[0]).toBe(19);
      expect(encodeAppCommand({ transactionUndo: { seq: 0, group_id: "" } })[0]).toBe(20);
      expect(encodeAppCommand({ transactionRedo: { seq: 0, group_id: "" } })[0]).toBe(21);
      expect(encodeAppCommand({ openArtifact: { seq: 0, artifact_ref: "", role: 0, plugin_id: "", app_id: "" } })[0]).toBe(22);
      expect(encodeAppCommand({ setDefaultApp: { seq: 0, artifact_kind: "", standard: "", subset: "", role: 0, plugin_id: "", app_id: "" } })[0]).toBe(23);
      expect(encodeAppCommand({ clearDefaultApp: { seq: 0, artifact_kind: "", standard: "", subset: "", role: 0 } })[0]).toBe(24);
      expect(encodeAppCommand({ setMergePolicy: { seq: 0, policy: 0 } })[0]).toBe(25);
      expect(encodeAppCommand({ resolveConflict: { seq: 0, conflict_id: "", resolution: 0 } })[0]).toBe(26);
      expect(encodeAppCommand({ readConflicts: { seq: 0 } })[0]).toBe(27);
      expect(encodeAppCommand({ presence: { seq: 0, own_color: null, peers: [] } })[0]).toBe(28);
      expect(encodeAppCommand({ LocalInteractionQuery: { seq: 0, command: { kind: "read", requestId: 0 } } })[0]).toBe(29);
      expect(encodeAppCommand({ LoadWindowConfig: { seq: 0, entry: { window_id: "", window_kind_id: "", envelope_pack: [] } } })[0]).toBe(30);
      expect(encodeAppCommand({ ReadWindowConfigs: { seq: 0 } })[0]).toBe(31);
    });

    it("tags every AppFrame variant per the agreed contract order", () => {
      expect(encodeAppFrame({ Done: { in_reply_to: 0 } })[0]).toBe(0);
      expect(encodeAppFrame({ Invocation: { in_reply_to: 0, output: [], diagnostics: [], ui_scope: [], history_patch: [], messages: [], mutations: [], inverse_group: [] } })[0]).toBe(1);
      expect(encodeAppFrame({ Error: { in_reply_to: null, fault: [], report: [] } })[0]).toBe(9);
      expect(encodeAppFrame({ Ephemeral: { presence: [], presence_generation: 0, transient_generation: 0, interaction: [] } })[0]).toBe(13);
      expect(encodeAppFrame({ HistorySnapshot: { in_reply_to: 0, history_patch: [] } })[0]).toBe(14);
      expect(encodeAppFrame({ transactionProposal: { in_reply_to: 0, proposal_id: "", local_ops: [], description: "", coalesce_key: "", foreign: [] } })[0]).toBe(15);
      expect(encodeAppFrame({ transactionPrepared: { txn_id: "", foreign: [], rejection: [] } })[0]).toBe(16);
      expect(encodeAppFrame({ transactionCommitted: { txn_id: "", edit_id: "" } })[0]).toBe(17);
      expect(encodeAppFrame({ transactionRolledBack: { txn_id: "" } })[0]).toBe(18);
      expect(encodeAppFrame({ MergeReport: { in_reply_to: null, report: [] } })[0]).toBe(19);
      expect(encodeAppFrame({ Conflicts: { in_reply_to: null, conflicts: [] } })[0]).toBe(20);
      expect(encodeAppFrame({ UiPatch: { in_reply_to: null, surface: "", kind: "", revision: 0, base_revision: 0, ops: [] } })[0]).toBe(21);
      expect(encodeAppFrame({ UiSnapshotEnd: { revision: 0 } })[0]).toBe(22);
      expect(encodeAppFrame({ WindowConfigs: { in_reply_to: 0, entries: [] } })[0]).toBe(24);
    });

    /**
     * 🔒️ Cross-language drift guard: the exact same fixture values and golden hex committed in
     * `protocol_channel`'s own `🔖️Corpus` region (`🔨️modules/📡️protocol/🧵️channel/📦️packages/🦀️rust/📦️lib.rs`,
     * `channel_command_fixture_corpus`/`channel_command_fixture_hex` and their `AppFrame` twins) —
     * sourced by running the real `encode_app_command`/`encode_app_frame` and copying their
     * printed `[DEBUG] AppCommand::<label> = <hex>` output (`cargo test -p semio-protocol-channel
     * -- --nocapture`), NOT hand-computed. Any future change to either codec that shifts these
     * bytes fails on exactly one side, forcing a deliberate update of both this table and the Rust
     * golden hex in the same change.
     */
    it("matches protocol_channel's own golden hex fixture corpus, byte-for-byte", () => {
            const commandFixtures: readonly (readonly [string, AppCommandValue])[] = [
        ["ConfigCommand", { ConfigCommand: { seq: 1, command: [9] } }],
        ["Command", { Command: { seq: 1, command: [1], view_state: [] } }],
        ["CommandText", { CommandText: { seq: 1, line: "go" } }],
        ["ContextMenu", { ContextMenu: { seq: 1, request: [1] } }],
        ["ArtifactCommand", { ArtifactCommand: { seq: 1, command: [1] } }],
        ["ApplyEnvelopes", { ApplyEnvelopes: { seq: 1, envelopes: [] } }],
        ["LoadDocument", { LoadDocument: { seq: 1, pack: [1], spr: [2] } }],
        ["ReadDocument", { ReadDocument: { seq: 1 } }],
        ["LoadConfig", { LoadConfig: { seq: 1, pack: [1], spr: [2] } }],
        ["ReadConfig", { ReadConfig: { seq: 1 } }],
        ["LoadWindowConfig", { LoadWindowConfig: { seq: 1, entry: { window_id: "w1", window_kind_id: "graph", envelope_pack: [1, 2] } } }],
        ["ReadWindowConfigs", { ReadWindowConfigs: { seq: 1 } }],
        ["MediaIn", { MediaIn: { seq: 1, port: "p", descriptor: [1], data: [2] } }],
        ["MediaOut", { MediaOut: { seq: 1, port: "p", request: [1] } }],
        ["MediaFingerprint", { MediaFingerprint: { seq: 1, port: "p" } }],
        ["PureCommand", { PureCommand: { seq: 1, command: [1], document: [2], document_spr: [3], config: [4], config_spr: [5], draft: [6], draft_spr: [7] } }],
        ["LoadChildren", { LoadChildren: { seq: 1, entries: [{ slot: "s", child_id: "c", dialect: "d", envelope_pack: [1] }] } }],
        ["ReadChildren", { ReadChildren: { seq: 1 } }],
        ["ReadHistory", { ReadHistory: { seq: 1 } }],
      ];
            const commandGoldenHex: Readonly<Record<string, string>> = {
        ConfigCommand: "00010109",
        Command: "0101010100",
        CommandText: "020102676f",
        ContextMenu: "03010101",
        ArtifactCommand: "04010101",
        ApplyEnvelopes: "050100",
        LoadDocument: "060101010102",
        ReadDocument: "0701",
        LoadConfig: "080101010102",
        ReadConfig: "0901",
        LoadWindowConfig: "1e01027731056772617068020102",
        ReadWindowConfigs: "1f01",
        MediaIn: "0a01017001010102",
        MediaOut: "0b0101700101",
        MediaFingerprint: "0c010170",
        PureCommand: "0d010101010201030104010501060107",
        LoadChildren: "0e01010173016301640101",
        ReadChildren: "0f01",
        ReadHistory: "1001",
      };
            const frameFixtures: readonly (readonly [string, AppFrameValue])[] = [
        ["Done", { Done: { in_reply_to: 1 } }],
        ["Invocation", { Invocation: { in_reply_to: 1, output: [1], diagnostics: [], ui_scope: [], history_patch: [], messages: [], mutations: [], inverse_group: [] } }],
        ["DocumentChanged", { DocumentChanged: { envelopes: [], origin: "o" } }],
        ["Document", { Document: { in_reply_to: 1, pack: [1], spr: [2], ops: "o" } }],
        ["Config", { Config: { in_reply_to: 1, pack: [1], spr: [2], ops: "c" } }],
        ["WindowConfigs", { WindowConfigs: { in_reply_to: 1, entries: [{ window_id: "w1", window_kind_id: "graph", envelope_pack: [1, 2] }, { window_id: "w2", window_kind_id: "graph", envelope_pack: [3] }] } }],
        ["ConfigChanged", { ConfigChanged: { envelopes: [], origin: "o" } }],
        ["ContextMenu", { ContextMenu: { in_reply_to: 1, items: [1] } }],
        ["Media", { Media: { in_reply_to: 1, port: "p", descriptor: [1], data: [2] } }],
        ["MediaFingerprint", { MediaFingerprint: { in_reply_to: 1, port: "p", fingerprint: [1] } }],
        ["Error", { Error: { in_reply_to: null, fault: [99], report: [] } }],
        ["Emit", { Emit: { in_reply_to: 1, document_ops: [1], config_ops: [], draft_ops: [], output: [2], diagnostics: [] } }],
        ["Draft", { Draft: { in_reply_to: 1, pack: [1], spr: [2], ops: "d" } }],
        ["Children", { Children: { in_reply_to: 1, entries: [{ slot: "s", child_id: "c", dialect: "d", envelope_pack: [1] }] } }],
        ["Ephemeral", { Ephemeral: { presence: [1, 2], presence_generation: 3, transient_generation: 4, interaction: [] } }],
        ["HistorySnapshot", { HistorySnapshot: { in_reply_to: 1, history_patch: [1] } }],
        ["UiPatch", { UiPatch: { in_reply_to: 1, surface: "1:body", kind: "window", revision: 3, base_revision: 2, ops: [9] } }],
        ["UiSnapshotEnd", { UiSnapshotEnd: { revision: 6 } }],
      ];
            const frameGoldenHex: Readonly<Record<string, string>> = {
        Done: "0001",
        Invocation: "01010101000000000000",
        DocumentChanged: "0200016f",
        Document: "030101010102016f",
        Config: "0401010101020163",
        WindowConfigs: "1801020277310567726170680201020277320567726170680103",
        ConfigChanged: "0500016f",
        ContextMenu: "06010101",
        Media: "0701017001010102",
        MediaFingerprint: "080101700101",
        Error: "0900016300",
        Emit: "0a0101010000010200",
        Draft: "0b01010101020164",
        Children: "0c01010173016301640101",
        Ephemeral: "0d020102030400",
        HistorySnapshot: "0e010101",
        UiPatch: "15010106313a626f64790677696e646f7703020109",
        UiSnapshotEnd: "1606",
      };
      const hex = (bytes: Uint8Array) => Array.from(bytes, (b) => b.toString(16).padStart(2, "0")).join("");
      for (const [label, value] of commandFixtures) expect(hex(encodeAppCommand(value)), `AppCommand::${label}`).toBe(commandGoldenHex[label]);
      for (const [label, value] of frameFixtures) expect(hex(encodeAppFrame(value)), `AppFrame::${label}`).toBe(frameGoldenHex[label]);
    });

    it("matches shared concrete-window config persistence wire vectors", async () => {
      const { readFileSync } = await import("node:fs");
      const { fileURLToPath } = await import("node:url");
      const { dirname, join } = await import("node:path");
      const fixture = JSON.parse(readFileSync(join(dirname(fileURLToPath(source.url)), "🧫️fixtures", "📡️channel", "🪟️window-config.json"), "utf8")) as Record<string, string>;
      const hex = (bytes: Uint8Array) => Buffer.from(bytes).toString("hex");
      expect(hex(encodeAppCommand({ LoadWindowConfig: { seq: 1, entry: { window_id: "w1", window_kind_id: "graph", envelope_pack: [1, 2] } } }))).toBe(fixture.loadWindowConfig);
      expect(hex(encodeAppCommand({ ReadWindowConfigs: { seq: 1 } }))).toBe(fixture.readWindowConfigs);
      expect(
        hex(
          encodeAppFrame({
            WindowConfigs: {
              in_reply_to: 1,
              entries: [
                { window_id: "w1", window_kind_id: "graph", envelope_pack: [1, 2] },
                { window_id: "w2", window_kind_id: "graph", envelope_pack: [3] },
              ],
            },
          }),
        ),
      ).toBe(fixture.windowConfigs);
    });

    /**
     * 🔗️ Cross-language drift guard for the M2 transaction variants (tags 22-26/19-22): both this
     * suite and `protocol_channel`'s `channel_transaction_fixtures_match_shared_cross_language_json_vectors`
     * Rust test load the SAME two JSON files under `🧫️fixtures/📡️channel/` — there is exactly one
     * committed hex string per label, not a copy per language, so a codec change that shifts these
     * bytes on either side fails here or there, never silently in both at once.
     */
    it("pins APP_CHANNEL_VERSION against the shared cross-language channel version", async () => {
      const { readFileSync } = await import("node:fs");
      const { fileURLToPath } = await import("node:url");
      const { dirname, join } = await import("node:path");
      const here = dirname(fileURLToPath(source.url));
      const pin = JSON.parse(readFileSync(join(here, "🧫️fixtures", "📡️channel", "🔖️channel-version.json"), "utf8")) as { channelVersion: number };
      expect(APP_CHANNEL_VERSION).toBe(pin.channelVersion);
    });

    it("matches the shared cross-language transaction fixture vectors, byte-for-byte", async () => {
      const { readFileSync } = await import("node:fs");
      const { fileURLToPath } = await import("node:url");
      const { dirname, join } = await import("node:path");
      const here = dirname(fileURLToPath(source.url));
      const channelFixturesDir = join(here, "🧫️fixtures", "📡️channel");
      const commandVectors = JSON.parse(readFileSync(join(channelFixturesDir, "🧾️app-command-transaction.json"), "utf8")) as Record<string, string>;
      const frameVectors = JSON.parse(readFileSync(join(channelFixturesDir, "📨️app-frame-transaction.json"), "utf8")) as Record<string, string>;
      const hex = (bytes: Uint8Array) => Array.from(bytes, (b) => b.toString(16).padStart(2, "0")).join("");

      const commandCases: Readonly<Record<string, AppCommandValue>> = {
        TransactionPrepareOwner: { transactionPrepare: { seq: 1, txn_id: "t", mutation_id: "m", payload: [9], prepared_ops: [], label: "", origin: [] } },
        TransactionPreparePrePlanned: { transactionPrepare: { seq: 2, txn_id: "t", mutation_id: "", payload: [], prepared_ops: [[1], [2, 2]], label: "l", origin: [9] } },
        TransactionCommit: { transactionCommit: { seq: 3, txn_id: "t" } },
        TransactionRollback: { transactionRollback: { seq: 4, txn_id: "t" } },
        TransactionUndo: { transactionUndo: { seq: 5, group_id: "g" } },
        TransactionRedo: { transactionRedo: { seq: 6, group_id: "g" } },
      };
      const frameCases: Readonly<Record<string, AppFrameValue>> = {
        TransactionProposal: { transactionProposal: { in_reply_to: 1, proposal_id: "p", local_ops: [[1]], description: "d", coalesce_key: "k", foreign: [] } },
        TransactionPrepared: { transactionPrepared: { txn_id: "t", foreign: [[1]], rejection: [] } },
        TransactionCommitted: { transactionCommitted: { txn_id: "t", edit_id: "e" } },
        TransactionRolledBack: { transactionRolledBack: { txn_id: "t" } },
      };

      expect(Object.keys(commandVectors).sort()).toEqual(Object.keys(commandCases).sort());
      expect(Object.keys(frameVectors).sort()).toEqual(Object.keys(frameCases).sort());
      for (const [label, value] of Object.entries(commandCases)) {
        expect(hex(encodeAppCommand(value)), `AppCommand::${label}`).toBe(commandVectors[label]);
        expect(decodeAppCommand(new Uint8Array(Buffer.from(commandVectors[label]!, "hex")))).toEqual(value);
      }
      for (const [label, value] of Object.entries(frameCases)) {
        expect(hex(encodeAppFrame(value)), `AppFrame::${label}`).toBe(frameVectors[label]);
        expect(decodeAppFrame(new Uint8Array(Buffer.from(frameVectors[label]!, "hex")))).toEqual(value);
      }
    });

    /**
     * 🔗️ Cross-language drift guard for the C3 opening variants (tags 27-29): both this suite and
     * `protocol_channel`'s `channel_opening_fixtures_match_shared_cross_language_json_vectors` Rust
     * test load the SAME JSON file under `🧫️fixtures/📡️channel/` — no `AppFrame` variants were added
     * for opening, so only the command-side vector file exists.
     */
    it("matches the shared cross-language opening fixture vectors, byte-for-byte", async () => {
      const { readFileSync } = await import("node:fs");
      const { fileURLToPath } = await import("node:url");
      const { dirname, join } = await import("node:path");
      const here = dirname(fileURLToPath(source.url));
      const channelFixturesDir = join(here, "🧫️fixtures", "📡️channel");
      const commandVectors = JSON.parse(readFileSync(join(channelFixturesDir, "🚪️app-command-opening.json"), "utf8")) as Record<string, string>;
      const hex = (bytes: Uint8Array) => Array.from(bytes, (b) => b.toString(16).padStart(2, "0")).join("");

      const commandCases: Readonly<Record<string, AppCommandValue>> = {
        OpenArtifactResolve: { openArtifact: { seq: 1, artifact_ref: "s.cad.cad@1/*#viewer", role: 0, plugin_id: "", app_id: "" } },
        OpenArtifactExplicit: { openArtifact: { seq: 2, artifact_ref: "s.cad.cad@1/*#editor", role: 1, plugin_id: "cad", app_id: "s.cad.cad@1/*#editor" } },
        SetDefaultApp: { setDefaultApp: { seq: 3, artifact_kind: "s.cad.cad", standard: "1", subset: "*", role: 1, plugin_id: "cad", app_id: "s.cad.cad@1/*#editor" } },
        ClearDefaultApp: { clearDefaultApp: { seq: 4, artifact_kind: "s.cad.cad", standard: "1", subset: "*", role: 0 } },
      };

      expect(Object.keys(commandVectors).sort()).toEqual(Object.keys(commandCases).sort());
      for (const [label, value] of Object.entries(commandCases)) {
        expect(hex(encodeAppCommand(value)), `AppCommand::${label}`).toBe(commandVectors[label]);
        expect(decodeAppCommand(new Uint8Array(Buffer.from(commandVectors[label]!, "hex")))).toEqual(value);
      }
    });

    /**
     * 🔗️ Cross-language drift guard for the C8 merge-policy/conflict variants (tags 30-32/23-24)
     * plus the extended `Invocation`/`Error` frames: both this suite and `protocol_channel`'s
     * `channel_merge_fixtures_match_shared_cross_language_json_vectors` Rust test load the SAME two
     * JSON files under `🧫️fixtures/📡️channel/` — see contract-freeze.md §C8 of
     * `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS/`.
     */
    it("matches the shared cross-language merge fixture vectors, byte-for-byte", async () => {
      const { readFileSync } = await import("node:fs");
      const { fileURLToPath } = await import("node:url");
      const { dirname, join } = await import("node:path");
      const here = dirname(fileURLToPath(source.url));
      const channelFixturesDir = join(here, "🧫️fixtures", "📡️channel");
      const commandVectors = JSON.parse(readFileSync(join(channelFixturesDir, "🔀️app-command-merge.json"), "utf8")) as Record<string, string>;
      const frameVectors = JSON.parse(readFileSync(join(channelFixturesDir, "📢️app-frame-merge.json"), "utf8")) as Record<string, string>;
      const hex = (bytes: Uint8Array) => Array.from(bytes, (b) => b.toString(16).padStart(2, "0")).join("");

      const commandCases: Readonly<Record<string, AppCommandValue>> = {
        SetMergePolicy: { setMergePolicy: { seq: 5, policy: 1 } },
        ResolveConflict: { resolveConflict: { seq: 6, conflict_id: "conflict-1", resolution: 0 } },
        ReadConflicts: { readConflicts: { seq: 7 } },
      };
      const frameCases: Readonly<Record<string, AppFrameValue>> = {
        MergeReport: { MergeReport: { in_reply_to: 1, report: [1] } },
        Conflicts: { Conflicts: { in_reply_to: null, conflicts: [2] } },
        Invocation: { Invocation: { in_reply_to: 1, output: [1], diagnostics: [], ui_scope: [], history_patch: [], messages: [9], mutations: [10], inverse_group: [11] } },
        Error: { Error: { in_reply_to: null, fault: [99], report: [7] } },
      };

      expect(Object.keys(commandVectors).sort()).toEqual(Object.keys(commandCases).sort());
      expect(Object.keys(frameVectors).sort()).toEqual(Object.keys(frameCases).sort());
      for (const [label, value] of Object.entries(commandCases)) {
        expect(hex(encodeAppCommand(value)), `AppCommand::${label}`).toBe(commandVectors[label]);
        expect(decodeAppCommand(new Uint8Array(Buffer.from(commandVectors[label]!, "hex")))).toEqual(value);
      }
      for (const [label, value] of Object.entries(frameCases)) {
        expect(hex(encodeAppFrame(value)), `AppFrame::${label}`).toBe(frameVectors[label]);
        expect(decodeAppFrame(new Uint8Array(Buffer.from(frameVectors[label]!, "hex")))).toEqual(value);
      }
    });
  });

  describe("@semio-tech/framework-os AppChannelClient", () => {
    it("local interaction outer wire matches strict fixtures and the independent LEB128 oracle", async () => {
      const { readFileSync } = await import("node:fs");
      const { default: Ajv } = await import("ajv");
      const oracleModule = "@webassemblyjs/leb128/lib/leb.js";
      const imported: unknown = await import(oracleModule);
      if (!imported || typeof imported !== "object") throw new Error("invalid LEB128 oracle module");
      const oracle: unknown = Reflect.get(imported, "default");
      if (!oracle || typeof oracle !== "object") throw new Error("invalid LEB128 oracle interface");
      const encodeUnsigned: unknown = Reflect.get(oracle, "encodeUIntBuffer");
      if (typeof encodeUnsigned !== "function") throw new Error("missing LEB128 oracle encoder");
      const fixture = JSON.parse(readFileSync(new URL("./🧫️fixtures/🏠️local-interaction/🧪️query/🔣️.json", source.url), "utf8"));
      const module = JSON.parse(readFileSync(new URL("./🧬️schema/🔣️.json", source.url), "utf8"));
      const validate = new Ajv({ strict: true }).addSchema(module).getSchema(`${module.$id}#/$defs/LocalInteractionV1`)!;
      expect(validate(fixture)).toBe(true);
      expect(validate({ ...fixture, lateTokenAccepted: true })).toBe(false);
      expect(validate({ ...fixture, terminalBeforeClosed: true })).toBe(false);
      const read: AppCommandValue = { LocalInteractionQuery: { seq: 9, command: { kind: "read", requestId: "13" } } };
      const rejected: AppFrameValue = { LocalInteractionQuery: { reply: { kind: "rejected", requestId: "13", code: "busy" } } };
      const u64 = (value: bigint): number[] => {
        const bytes = Buffer.alloc(8); bytes.writeBigUInt64LE(value);
        const encoded: unknown = encodeUnsigned(bytes);
        if (!(encoded instanceof Uint8Array)) throw new Error("invalid LEB128 oracle bytes");
        return Array.from(encoded);
      };
      expect([...encodeAppCommand(read)]).toEqual([29, ...u64(9n), ...u64(2n), 0, ...u64(13n)]);
      expect(Buffer.from(encodeAppCommand(read)).toString("hex")).toBe(fixture.outerReadHex);
      expect(Buffer.from(encodeAppFrame(rejected)).toString("hex")).toBe(fixture.outerRejectedHex);
      const receipt: AppFrameValue = { Done: { in_reply_to: 2 } };
      expect([...encodeAppFrame(receipt)]).toEqual([0, ...u64(2n)]);
      expect(Buffer.from(encodeAppFrame(receipt)).toString("hex")).toBe(fixture.receiptHex);
      for (const row of fixture.sequenceCases) {
        if (row.result === null) continue;
        const sequence = row.result.sequence;
        expect([...encodeAppCommand({ ReadDocument: { seq: sequence } })]).toEqual([7, ...u64(BigInt(sequence))]);
        if (row.result.request !== null) {
          const inner = [0, ...u64(BigInt(row.result.request))];
          expect([...encodeAppCommand({ LocalInteractionQuery: { seq: sequence, command: { kind: "read", requestId: row.result.request } } })]).toEqual([29, ...u64(BigInt(sequence)), ...u64(BigInt(inner.length)), ...inner]);
        }
      }
      expect(decodeAppCommand(encodeAppCommand(read))).toEqual(read);
      expect(decodeAppFrame(encodeAppFrame(rejected))).toEqual(rejected);
      expect(() => decodeAppCommand(Uint8Array.from([...encodeAppCommand(read), 0]))).toThrow();
      expect(() => decodeAppFrame(Uint8Array.from([...encodeAppFrame(rejected), 0]))).toThrow();
    });

    it("local interaction client fixture lifecycles preserve ACK ownership and ordinary replies", async () => {
      const { readFileSync } = await import("node:fs");
      const fixture = JSON.parse(readFileSync(new URL("./🧫️fixtures/🏠️local-interaction/🧪️query/🔣️.json", source.url), "utf8"));
      for (const row of fixture.lifecycles) {
        const broadcast = createTurnOutcomeBroadcast<TurnOutcome>();
        const sent: AppCommandValue[] = [];
        const handle: AppChannelHandle = { enqueue: (_id, events) => {
          for (const command of events.map(decodeAppCommand)) {
            sent.push(command);
            if ("LocalInteractionQuery" in command && !(["disposal-before-read-receipt", "coalesced-query-and-ordinary-receipts"].includes(row.id) && command.LocalInteractionQuery.command.kind === "read")) broadcast.push({ instanceId: 7, frames: [encodeAppFrame({ Done: { in_reply_to: command.LocalInteractionQuery.seq } })] });
          }
        }, outcomes: broadcast.stream };
        const client = new AppChannelClient(handle, new AppChannelRequestSequence(), 7, "fixture");
        const abort = new AbortController();
        let consume!: () => void;
        const consumed = new Promise<void>((resolve) => { consume = resolve; });
        let complete = false;
        let queryResult: Promise<LocalInteractionIdentity | unknown> | undefined;
        let ordinary: Promise<AppFrameValue[]> | undefined;
        let ordinaryComplete = false;
        const token: LocalInteractionQueryToken = { requestId: "1", queryGeneration: "41", identity: { appInstanceId: 7, generation: "9007199254740993", revision: "11".repeat(32), documentRevision: "22".repeat(32), topologyRevision: "33".repeat(32) }, ordinal: "0" };
        const push = (reply: LocalInteractionQueryReply) => broadcast.push({ instanceId: 7, frames: [encodeAppFrame({ LocalInteractionQuery: { reply } })] });
        const flush = async () => { for (let index = 0; index < 12; index += 1) await Promise.resolve(); };
        for (const event of row.events) {
          if (event === "read") queryResult = client.readLocalInteractionPages(() => {
            if (row.id === "synchronous-consumer-failure") throw new Error("synchronous consumer fixture");
            if (row.id === "consumer-failure") return Promise.reject(new Error("consumer fixture"));
            return consumed;
          }, abort.signal).then((identity) => { complete = true; return identity; }, (error: unknown) => { complete = true; return error; });
          else if (event === "ordinary") ordinary = client.readDocument().then((frames) => { ordinaryComplete = true; return frames; });
          else if (event === "started") push({ kind: "started", token });
          else if (event === "abort") abort.abort();
          else if (event === "dispose") client.dispose();
          else if (event === "malformed") broadcast.push({ instanceId: 7, frames: [Uint8Array.from(Buffer.from(fixture.malformedFrameHex, "hex"))] });
          else if (event === "readReceipt") broadcast.push({ instanceId: 7, frames: [encodeAppFrame({ Done: { in_reply_to: 1 } })] });
          else if (event === "wrongReceipt" || event === "duplicateReadReceipt") {
            broadcast.push({ instanceId: 7, frames: [encodeAppFrame({ Done: { in_reply_to: event === "wrongReceipt" ? 999 : 1 } })] });
            await flush();
            expect(ordinaryComplete).toBe(false);
          }
          else if (event === "ordinaryDone") { broadcast.push({ instanceId: 7, frames: [encodeAppFrame({ Done: { in_reply_to: 3 } })] }); expect(await ordinary).toEqual([{ Done: { in_reply_to: 3 } }]); }
          else if (event === "ordinaryEmpty") { broadcast.push({ instanceId: 7, frames: [] }); await flush(); expect(ordinaryComplete).toBe(false); }
          else if (event === "mixedReceipts") {
            broadcast.push({ instanceId: 7, frames: [encodeAppFrame({ Done: { in_reply_to: 1 } }), encodeAppFrame({ Done: { in_reply_to: 3 } })] });
            await flush(); expect(ordinaryComplete).toBe(true);
            expect(await ordinary).toEqual([{ Done: { in_reply_to: 3 } }]);
          }
          else if (event === "startedWithNotice" || event === "pageWithNotice") {
            const reply: LocalInteractionQueryReply = event === "startedWithNotice" ? { kind: "started", token } : { kind: "page", page: { ...token, terminal: true, bytes: [0xe2, 0x9c, 0x93] } };
            broadcast.push({ instanceId: 7, frames: [encodeAppFrame({ LocalInteractionQuery: { reply } }), encodeAppFrame({ Ephemeral: { presence: [], presence_generation: 0, transient_generation: 0, interaction: [] } }), encodeAppFrame({ UiPatch: { in_reply_to: null, surface: "fixture", kind: "graph", revision: 1, base_revision: 0, ops: [] } })] });
            await flush(); expect(ordinaryComplete).toBe(false);
          }
          else if (event === "page") push({ kind: "page", page: { ...token, terminal: true, bytes: [0xe2, 0x9c, 0x93] } });
          else if (event === "consume") consume();
          else if (event === "consumeError") await flush();
          else if (event === "ack" || event === "cancel") {
            await flush();
            const last = sent.at(-1);
            expect(last && "LocalInteractionQuery" in last ? last.LocalInteractionQuery.command : null).toEqual({ kind: event === "ack" ? "acknowledge" : "cancel", token });
          } else if (event === "closed") {
            expect(complete).toBe(false);
            push({ kind: "closed", token, cancelled: row.cancelled });
          }
          await flush();
        }
        const result = await queryResult;
        expect(complete).toBe(true);
        if (!row.cancelled) expect(result).toEqual(token.identity);
        else expect(result).toBeInstanceOf(Error);
        client.dispose();
      }
    });
    it("local interaction sequence admission matches the checked shared-owner fixture", async () => {
      const { readFileSync } = await import("node:fs");
      const fixture = JSON.parse(readFileSync(new URL("./🧫️fixtures/🏠️local-interaction/🧪️query/🔣️.json", source.url), "utf8"));
      for (const row of fixture.sequenceCases) {
        const owner = new AppChannelRequestSequence(row.sequence, BigInt(row.request));
        const allocate = () => row.operation === "query" ? owner.nextQuery() : { sequence: owner.nextSequence(), cancelSequence: null, request: null };
        if (row.result === null) expect(allocate).toThrow();
        else expect(allocate()).toEqual(row.result);
        expect(owner.checkpoint()).toEqual(row.after);
      }
      for (const invalid of [-1, Number.MAX_SAFE_INTEGER + 1, NaN, 0.5]) expect(() => new AppChannelRequestSequence(invalid)).toThrow();
      for (const invalid of [-1n, 0x1_0000_0000_0000_0000n]) expect(() => new AppChannelRequestSequence(0, invalid)).toThrow();
    });

    it("local interaction reopened clients reject delayed Started pages and ordinary receipts", async () => {
      const { readFileSync } = await import("node:fs");
      const fixture = JSON.parse(readFileSync(new URL("./🧫️fixtures/🏠️local-interaction/🧪️query/🔣️.json", source.url), "utf8")).reopen;
      const owner = new AppChannelRequestSequence();
      const broadcast = createTurnOutcomeBroadcast<TurnOutcome>();
      const sent: AppCommandValue[] = [];
      const handle: AppChannelHandle = { enqueue: (_instance, bytes) => {
        for (const command of bytes.map(decodeAppCommand)) {
          sent.push(command);
          if ("LocalInteractionQuery" in command) broadcast.push({ instanceId: 7, frames: [encodeAppFrame({ Done: { in_reply_to: command.LocalInteractionQuery.seq } })] });
        }
      }, outcomes: broadcast.stream };
      const flush = async () => { for (let index = 0; index < 16; index += 1) await Promise.resolve(); };
      const push = (reply: LocalInteractionQueryReply) => broadcast.push({ instanceId: 7, frames: [encodeAppFrame({ LocalInteractionQuery: { reply } })] });
      const oldToken: LocalInteractionQueryToken = { requestId: fixture.requests[0], queryGeneration: "41", identity: { appInstanceId: 7, generation: "7", revision: "11".repeat(32), documentRevision: "22".repeat(32), topologyRevision: "33".repeat(32) }, ordinal: "0" };
      const newToken: LocalInteractionQueryToken = { ...oldToken, requestId: fixture.requests[1], queryGeneration: "42" };
      const first = new AppChannelClient(handle, owner, 7, "fixture");
      const firstResult = first.readLocalInteractionPages(async () => {}).catch((error: unknown) => error);
      first.dispose();
      await flush();
      push({ kind: "started", token: oldToken });
      await flush();
      push({ kind: "closed", token: oldToken, cancelled: true });
      expect(await firstResult).toBeInstanceOf(Error);
      const second = new AppChannelClient(handle, owner, 7, "fixture");
      let consumed = 0;
      let ordinaryComplete = false;
      const secondResult = second.readLocalInteractionPages(async () => { consumed += 1; });
      const ordinary = second.readDocument().then((frames) => { ordinaryComplete = true; return frames; });
      for (const sequence of [...fixture.readSequences.slice(0, 1), ...fixture.cancelSequences.slice(0, 1), 999]) broadcast.push({ instanceId: 7, frames: [encodeAppFrame({ Done: { in_reply_to: sequence } })] });
      push({ kind: "started", token: oldToken });
      push({ kind: "page", page: { ...oldToken, terminal: true, bytes: [1] } });
      push({ kind: "closed", token: oldToken, cancelled: false });
      await flush();
      expect(consumed).toBe(0);
      expect(ordinaryComplete).toBe(false);
      push({ kind: "started", token: newToken });
      push({ kind: "page", page: { ...newToken, terminal: true, bytes: [2] } });
      await flush();
      expect(consumed).toBe(1);
      expect(sent.at(-1)).toEqual({ LocalInteractionQuery: { seq: fixture.ackSequence, command: { kind: "acknowledge", token: newToken } } });
      broadcast.push({ instanceId: 7, frames: [encodeAppFrame({ Done: { in_reply_to: fixture.ordinarySequence } })] });
      expect(await ordinary).toEqual([{ Done: { in_reply_to: fixture.ordinarySequence } }]);
      push({ kind: "closed", token: newToken, cancelled: false });
      expect(await secondResult).toEqual(newToken.identity);
      expect(sent.filter((command) => "LocalInteractionQuery" in command && command.LocalInteractionQuery.command.kind === "read").map((command) => "LocalInteractionQuery" in command ? command.LocalInteractionQuery.seq : 0)).toEqual(fixture.readSequences);
      second.dispose();
    });

    it("local interaction exhausted admission leaves no query slot and retains a cancellation sequence", async () => {
      const broadcast = createTurnOutcomeBroadcast<TurnOutcome>();
      const sent: AppCommandValue[] = [];
      const handle: AppChannelHandle = { enqueue: (_instance, bytes) => {
        for (const command of bytes.map(decodeAppCommand)) {
          sent.push(command);
          broadcast.push({ instanceId: 7, frames: [encodeAppFrame({ Done: { in_reply_to: Object.values(command)[0]!.seq } })] });
        }
      }, outcomes: broadcast.stream };
      const exhausted = new AppChannelClient(handle, new AppChannelRequestSequence(Number.MAX_SAFE_INTEGER - 1), 7, "fixture");
      await expect(exhausted.readLocalInteractionPages(async () => {})).rejects.toThrow("sequence-exhausted");
      await expect(exhausted.readLocalInteractionPages(async () => {})).rejects.toThrow("sequence-exhausted");
      expect(sent).toHaveLength(0);
      await exhausted.readDocument();
      expect(sent).toEqual([{ ReadDocument: { seq: Number.MAX_SAFE_INTEGER } }]);
      exhausted.dispose();
      sent.length = 0;
      const last = new AppChannelClient(handle, new AppChannelRequestSequence(Number.MAX_SAFE_INTEGER - 2), 7, "fixture");
      const result = last.readLocalInteractionPages(async () => {}).catch((error: unknown) => error);
      const token: LocalInteractionQueryToken = { requestId: "1", queryGeneration: "43", identity: { appInstanceId: 7, generation: "7", revision: "11".repeat(32), documentRevision: "22".repeat(32), topologyRevision: "33".repeat(32) }, ordinal: "0" };
      broadcast.push({ instanceId: 7, frames: [encodeAppFrame({ LocalInteractionQuery: { reply: { kind: "started", token } } }), encodeAppFrame({ LocalInteractionQuery: { reply: { kind: "page", page: { ...token, terminal: true, bytes: [1] } } } })] });
      for (let index = 0; index < 16; index += 1) await Promise.resolve();
      expect(sent.at(-1)).toEqual({ LocalInteractionQuery: { seq: Number.MAX_SAFE_INTEGER, command: { kind: "cancel", token } } });
      broadcast.push({ instanceId: 7, frames: [encodeAppFrame({ LocalInteractionQuery: { reply: { kind: "closed", token, cancelled: true } } })] });
      expect(await result).toBeInstanceOf(Error);
      last.dispose();
    });
    /** 🧪️ A fake handle that decodes whatever {@link AppChannelClient} `enqueue`d and pushes
     * caller-supplied frames back as this SAME instance's next outcome — enough to assert the client
     * frames/unframes correctly (and correlates replies through the real `outcomes` broadcast) without
     * a real plugin instance. */
    function fakeHandle(reply: (instanceId: number, commands: AppCommandValue[]) => AppFrameValue[]): AppChannelHandle {
      const broadcast = createTurnOutcomeBroadcast<TurnOutcome>();
      return {
        enqueue: (instanceId, events) => {
          const commands = events.map(decodeAppCommand);
          const frames = reply(instanceId, commands).map(encodeAppFrame);
          broadcast.push({ instanceId, frames });
        },
        outcomes: broadcast.stream,
      };
    }

    it("command() allocates an incrementing seq and returns every frame the batch produced", async () => {
      const seqsSeen: number[] = [];
      const handle = fakeHandle((_instanceId, commands) => {
        const cmd = commands[0];
        if (cmd && "Command" in cmd) seqsSeen.push(cmd.Command.seq);
        return [
          { Invocation: { in_reply_to: seqsSeen.at(-1) ?? 0, output: [1], diagnostics: [], ui_scope: [], history_patch: [], messages: [], mutations: [], inverse_group: [] } },
          { UiPatch: { in_reply_to: seqsSeen.at(-1) ?? 0, surface: "1:body", kind: "window", revision: 1, base_revision: 0, ops: [] } },
        ];
      });
      const client = new AppChannelClient(handle, new AppChannelRequestSequence(), 1, "app.demo");
      const first = await client.command(new Uint8Array([1, 2]), { cursor: 0 });
      const second = await client.command(new Uint8Array([3]), { cursor: 1 });
      expect(seqsSeen).toEqual([1, 2]);
      expect(first).toHaveLength(2);
      expect(second).toHaveLength(2);
    });

    it("configure()/readDocument()/loadDocument() frame the right AppCommand variant", async () => {
      const seen: AppCommandValue[] = [];
      const handle = fakeHandle((_instanceId, commands) => {
        seen.push(...commands);
        return [{ Done: { in_reply_to: Object.values(commands[0]!)[0]!.seq } }];
      });
      const client = new AppChannelClient(handle, new AppChannelRequestSequence(), 1, "app.demo");
      await client.configure({ locale: "en" });
      await client.readDocument();
      await client.loadDocument(new Uint8Array([1]), new Uint8Array([2]));
      expect(seen[0]).toEqual({ ConfigCommand: { seq: 1, command: Array.from(encodePackValue({ locale: "en" })) } });
      expect(seen[1]).toEqual({ ReadDocument: { seq: 2 } });
      expect(seen[2]).toEqual({ LoadDocument: { seq: 3, pack: [1], spr: [2] } });
    });

    it("loads and reads exact persisted-local window config envelopes", async () => {
      const seen: AppCommandValue[] = [];
      const expected = [
        { window_id: "graph-a", window_kind_id: "graph", envelope_pack: [1, 2] },
        { window_id: "graph-b", window_kind_id: "graph", envelope_pack: [3] },
      ];
      const handle = fakeHandle((_instanceId, commands) => {
        seen.push(...commands);
        const command = commands[0]!;
        if ("ReadWindowConfigs" in command) return [{ WindowConfigs: { in_reply_to: command.ReadWindowConfigs.seq, entries: expected } }];
        return [{ Done: { in_reply_to: Object.values(command)[0]!.seq } }];
      });
      const client = new AppChannelClient(handle, new AppChannelRequestSequence(), 1, "app.demo");
      const source = { window_id: "graph-a", window_kind_id: "graph", envelope_pack: [1, 2] };
      await client.loadWindowConfig(source);
      const actual = await client.readWindowConfigs();
      source.envelope_pack.fill(255);
      expect(seen).toEqual([
        { LoadWindowConfig: { seq: 1, entry: { window_id: "graph-a", window_kind_id: "graph", envelope_pack: [1, 2] } } },
        { ReadWindowConfigs: { seq: 2 } },
      ]);
      expect(actual).toEqual(expected);
    });

    it("publishes only accepted document cache candidates and owns both byte arrays", async () => {
      const { readFileSync } = await import("node:fs");
      const { default: Ajv } = await import("ajv");
      const fixture = JSON.parse(readFileSync(new URL("./🧫️fixtures/📦️document-cache/🔣️.json", source.url), "utf8"));
      const schema = JSON.parse(readFileSync(new URL("./🧬️schema/🔣️.json", source.url), "utf8"));
      const validate = new Ajv({ strict: true }).addSchema(schema).getSchema(`${schema.$id}#/$defs/DocumentCacheAcceptanceV1`)!;
      expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
      expect(validate({ ...fixture, optimistic: true })).toBe(false);
      const pair = (value: { pack: number[]; spr: number[] }) => ({ pack: Uint8Array.from(value.pack), spr: Uint8Array.from(value.spr) });
      for (const row of fixture.cases) {
        const broadcast = createTurnOutcomeBroadcast<TurnOutcome>();
        let sent = 0;
        let refuseEnqueue = false;
        const client = new AppChannelClient({
          outcomes: broadcast.stream,
          enqueue: (_instanceId: number, commands: Uint8Array[]) => {
            if (refuseEnqueue) throw new Error("document-cache.enqueue");
            sent = Object.values(decodeAppCommand(commands[0]!))[0]!.seq;
          },
        }, new AppChannelRequestSequence(), 1, "cache");
        try {
          const seed = pair(fixture.initial);
          const initial = client.loadDocument(seed.pack, seed.spr);
          broadcast.push({ instanceId: 1, frames: [encodeAppFrame({ Done: { in_reply_to: sent } })] });
          await initial;
          expect(client.documentPack()).toEqual(pair(fixture.initial));
          const candidate = pair(fixture.candidate);
          refuseEnqueue = row.outcome === "enqueue";
          const loading = client.loadDocument(candidate.pack, candidate.spr);
          const settled = loading.then(() => "resolved", () => "rejected");
          candidate.pack.fill(255);
          candidate.spr.fill(255);
          expect(client.documentPack()).toEqual(pair(fixture.initial));
          const document = { Document: { in_reply_to: sent, pack: fixture.reply.pack, spr: fixture.reply.spr, ops: "" } };
          const error = { Error: { in_reply_to: sent, fault: [99], report: [] } };
          if (row.outcome === "transport") broadcast.push({ instanceId: 1, error: new Error("document-cache.transport") });
          else if (row.outcome !== "enqueue") broadcast.push({ instanceId: 1, frames: (
            row.outcome === "done" ? [{ Done: { in_reply_to: sent } }] :
            row.outcome === "document" ? [document] :
            row.outcome === "error-document" ? [document, error] : [error]
          ).map(encodeAppFrame) });
          await settled;
          expect(client.documentPack()).toEqual(pair(fixture[row.expected]));
          const exposed = client.documentPack()!;
          exposed.pack.fill(254);
          exposed.spr.fill(254);
          expect(client.documentPack()).toEqual(pair(fixture[row.expected]));
          client.dispose();
          expect(client.documentPack()).toBeNull();
        } finally {
          client.dispose();
          broadcast.complete();
        }
      }
    });

    it("caches the document pack from accepted loadDocument arguments without a document echo", async () => {
      const handle = fakeHandle(() => [{ Done: { in_reply_to: 1 } }]);
      const client = new AppChannelClient(handle, new AppChannelRequestSequence(), 1, "app.demo");
      expect(client.documentPack()).toBeNull();
      await client.loadDocument(new Uint8Array([1, 2]), new Uint8Array([3]));
      expect(client.documentPack()).toEqual({ pack: new Uint8Array([1, 2]), spr: new Uint8Array([3]) });
    });

    it("caches the document pack from every AppFrame::Document reply, most recent wins", async () => {
      const handle = fakeHandle((_instanceId, commands) => {
        const cmd = commands[0];
        if (cmd && "ReadDocument" in cmd) {
          return [{ Document: { in_reply_to: cmd.ReadDocument.seq, pack: [9, 9], spr: [8], ops: "" } }];
        }
        return [{ Done: { in_reply_to: 1 } }];
      });
      const client = new AppChannelClient(handle, new AppChannelRequestSequence(), 1, "app.demo");
      await client.readDocument();
      expect(client.documentPack()).toEqual({ pack: new Uint8Array([9, 9]), spr: new Uint8Array([8]) });
    });

    it("transactionPrepareOwner()/transactionPreparePlanned()/transactionCommit()/transactionRollback()/transactionUndo()/transactionRedo() frame the right AppCommand variant", async () => {
      const seen: AppCommandValue[] = [];
      const handle = fakeHandle((_instanceId, commands) => {
        seen.push(...commands);
        return [{ Done: { in_reply_to: Object.values(commands[0]!)[0]!.seq } }];
      });
      const client = new AppChannelClient(handle, new AppChannelRequestSequence(), 1, "app.demo");
      await client.transactionPrepareOwner("txn-1", "s.doc#kind", new Uint8Array([1]));
      await client.transactionPreparePlanned("txn-1", [new Uint8Array([2]), new Uint8Array([3])], "duplicate", new Uint8Array([4]));
      await client.transactionCommit("txn-1");
      await client.transactionRollback("txn-1");
      await client.transactionUndo("grp-1");
      await client.transactionRedo("grp-1");
      expect(seen[0]).toEqual({ transactionPrepare: { seq: 1, txn_id: "txn-1", mutation_id: "s.doc#kind", payload: [1], prepared_ops: [], label: "", origin: [] } });
      expect(seen[1]).toEqual({ transactionPrepare: { seq: 2, txn_id: "txn-1", mutation_id: "", payload: [], prepared_ops: [[2], [3]], label: "duplicate", origin: [4] } });
      expect(seen[2]).toEqual({ transactionCommit: { seq: 3, txn_id: "txn-1" } });
      expect(seen[3]).toEqual({ transactionRollback: { seq: 4, txn_id: "txn-1" } });
      expect(seen[4]).toEqual({ transactionUndo: { seq: 5, group_id: "grp-1" } });
      expect(seen[5]).toEqual({ transactionRedo: { seq: 6, group_id: "grp-1" } });
    });

    /**
     * 🔗️ Rust↔TS parity for the C8 merge/conflict surface, driven through {@link AppChannelClient}'s
     * OWN public methods rather than the raw `encodeAppCommand`/`decodeAppFrame` codec functions the
     * `AppChannelCodec` suite already asserts against the same two files — this is the "does the
     * CLIENT layer itself send/surface the new commands and frames correctly" half of the parity
     * story (contract-freeze §C8/§C9, ticket
     * `26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS`). Four throwaway
     * `configure({})` calls burn seq 1-4 so `setMergePolicy`/`resolveConflict`/`readConflicts` land on
     * the exact seq (5/6/7) the golden vectors were baked against.
     */
    it("setMergePolicy()/resolveConflict()/readConflicts() match the shared cross-language merge command vectors, byte-for-byte", async () => {
      const { readFileSync } = await import("node:fs");
      const { fileURLToPath } = await import("node:url");
      const { dirname, join } = await import("node:path");
      const here = dirname(fileURLToPath(source.url));
      const commandVectors = JSON.parse(readFileSync(join(here, "🧫️fixtures", "📡️channel", "🔀️app-command-merge.json"), "utf8")) as Record<string, string>;
      const hex = (bytes: Uint8Array) => Array.from(bytes, (b) => b.toString(16).padStart(2, "0")).join("");

      const seen: AppCommandValue[] = [];
      const handle = fakeHandle((_instanceId, commands) => {
        seen.push(...commands);
        return [{ Done: { in_reply_to: Object.values(commands[0]!)[0]!.seq } }];
      });
      const client = new AppChannelClient(handle, new AppChannelRequestSequence(), 1, "app.demo");
      await client.configure({});
      await client.configure({});
      await client.configure({});
      await client.configure({});
      await client.setMergePolicy("Normal");
      await client.resolveConflict("conflict-1", "accept");
      await client.readConflicts();

      expect(seen[4]).toEqual({ setMergePolicy: { seq: 5, policy: 1 } });
      expect(seen[5]).toEqual({ resolveConflict: { seq: 6, conflict_id: "conflict-1", resolution: 0 } });
      expect(seen[6]).toEqual({ readConflicts: { seq: 7 } });
      expect(hex(encodeAppCommand(seen[4]!)), "AppCommand::SetMergePolicy").toBe(commandVectors.SetMergePolicy);
      expect(hex(encodeAppCommand(seen[5]!)), "AppCommand::ResolveConflict").toBe(commandVectors.ResolveConflict);
      expect(hex(encodeAppCommand(seen[6]!)), "AppCommand::ReadConflicts").toBe(commandVectors.ReadConflicts);
    });

    it("command() surfaces unsolicited MergeReport/Conflicts frames and the extended Invocation.messages/Error.report fields verbatim", async () => {
      const handle = fakeHandle(() => [
        { Invocation: { in_reply_to: 1, output: [], diagnostics: [], ui_scope: [], history_patch: [], messages: [9], mutations: [], inverse_group: [] } },
        { MergeReport: { in_reply_to: null, report: [1] } },
        { Conflicts: { in_reply_to: null, conflicts: [2] } },
      ]);
      const client = new AppChannelClient(handle, new AppChannelRequestSequence(), 1, "app.demo");
      const frames = await client.command(new Uint8Array([1]), {});
      expect(frames).toHaveLength(3);
      const invocation = frames.find((frame): frame is Extract<AppFrameValue, { readonly Invocation: unknown }> => "Invocation" in frame);
      const mergeReport = frames.find((frame): frame is Extract<AppFrameValue, { readonly MergeReport: unknown }> => "MergeReport" in frame);
      const conflicts = frames.find((frame): frame is Extract<AppFrameValue, { readonly Conflicts: unknown }> => "Conflicts" in frame);
      expect(invocation?.Invocation.messages).toEqual([9]);
      expect(mergeReport?.MergeReport.report).toEqual([1]);
      expect(conflicts?.Conflicts.conflicts).toEqual([2]);
    });

    /**
     * 🔗️ Round-trips the frozen TS shapes ({@link DispatchReport}/{@link MergeReport}/{@link
     * Conflict}) through {@link encodePackValue}/the new `decode*FromWire` helpers — proving the
     * field-name mapping this lane derived from Rust's `#[serde(rename_all = "camelCase")]`
     * (`policy`/`worst`/`messages`, `insertionIndex`, `editId`→`edit_id` NOT renamed inside
     * `ConflictKind`'s struct variants, `MergePolicy`'s bare un-camelCased variant names) actually
     * decodes the way the wire's `store::pack_rt::encode_wire_value`-backed `encode_wire_serialized`
     * would produce it, since no live Rust `DispatchReport`/`MergeReport`/`Conflict` pack bytes are
     * checked into a fixture yet (only the outer `AppFrame` framing is, in `📢️app-frame-merge.json`).
     */
    it("faultMessages()/decodeDispatchReportFromWire()/decodeMergeReportFromWire()/decodeConflictsFromWire() decode the frozen TS report shapes", () => {
      const dispatchReport: DispatchReport = {
        policy: "Vigilant",
        worst: "warning",
        messages: [{ level: "warning", code: "mutation.clamped", message: "value clamped to range" }],
      };
      const reportBytes = Array.from(encodePackValue(dispatchReport));
      expect(decodeDispatchReportFromWire(reportBytes, decodePackValue)).toEqual(dispatchReport);
      expect(faultMessages(reportBytes, decodePackValue)).toEqual(dispatchReport.messages);
      expect(faultMessages([], decodePackValue)).toEqual([]);

      const mergeReport: MergeReport = {
        policy: "Normal",
        accepted: true,
        insertionIndex: 3,
        replayed: [{ edit_id: "e1", messages: [{ level: "info", code: "mutation.cascade", message: "cascaded" }] }],
        worst: "info",
        conflict: null,
      };
      expect(decodeMergeReportFromWire(Array.from(encodePackValue(mergeReport)), decodePackValue)).toEqual(mergeReport);
      expect(decodeMergeReportFromWire([], decodePackValue)).toBeNull();

      const conflicts: readonly Conflict[] = [
        {
          id: "conflict-abc",
          kind: { kind: "degraded", edit_ids: ["e1"] },
          status: "open",
          messages: [{ level: "error", code: "mutation.target-missing", message: "target missing" }],
          actors: ["actor-1"],
          timestamp: { actor: 1, physical_ms: 100, logical: 0 },
        },
      ];
      expect(decodeConflictsFromWire(Array.from(encodePackValue(conflicts)), decodePackValue)).toEqual(conflicts);
      expect(decodeConflictsFromWire([], decodePackValue)).toEqual([]);
    });
  });

  // 🕸️ `@semio-tech/framework`'s `PluginGraph`/`InstanceDirectory`/`ArtifactMutationRouter`/
  // `ArtifactInferenceRouter` (ticket 26/08/16/PLUGIN-DEPENDENCIES-ARTIFACT-CONTRIBUTIONS-AND-COMPOSITE-MUTATIONS,
  // W2-B) have no in-source-testing harness of their own (`@semio-tech/framework`'s vitest only
  // `includeSource`s `🟦️.ts`, not the module file they're defined in) — dynamically importing the
  // real workspace package here exercises them under a config that DOES run, without this file taking
  // on a static dependency on `@semio-tech/framework`'s runtime exports.
  describe("@semio-tech/framework PluginGraph", () => {
    it("validates a graph with every dependency present and version-satisfying", async () => {
      const { validatePluginDependencyGraph } = await import("@semio-tech/framework");
      expect(
        validatePluginDependencyGraph([
          { pluginId: "a", version: "1.2.3" },
          { pluginId: "b", version: "1.0.0", dependencies: [{ pluginId: "a", version: "^1.0.0" }] },
        ]),
      ).toEqual([]);
    });

    it("reports a missing dependency", async () => {
      const { validatePluginDependencyGraph } = await import("@semio-tech/framework");
      expect(validatePluginDependencyGraph([{ pluginId: "b", dependencies: [{ pluginId: "missing", version: "*" }] }])).toEqual([
        { code: "transaction.dependency-missing", pluginId: "b", dependsOn: "missing" },
      ]);
    });

    it("reports a version mismatch", async () => {
      const { validatePluginDependencyGraph } = await import("@semio-tech/framework");
      expect(
        validatePluginDependencyGraph([
          { pluginId: "a", version: "2.0.0" },
          { pluginId: "b", dependencies: [{ pluginId: "a", version: "^1.0.0" }] },
        ]),
      ).toEqual([{ code: "transaction.version-mismatch", pluginId: "b", dependsOn: "a", required: "^1.0.0", actual: "2.0.0" }]);
    });

    it("resolves a diamond load order deterministically, tie-broken lexicographically", async () => {
      const { resolvePluginLoadOrder } = await import("@semio-tech/framework");
      const result = resolvePluginLoadOrder([
        {
          pluginId: "d",
          dependencies: [
            { pluginId: "b", version: "*" },
            { pluginId: "c", version: "*" },
          ],
        },
        { pluginId: "c", dependencies: [{ pluginId: "a", version: "*" }] },
        { pluginId: "b", dependencies: [{ pluginId: "a", version: "*" }] },
        { pluginId: "a" },
      ]);
      expect(result.errors).toEqual([]);
      expect(result.order).toEqual(["a", "b", "c", "d"]);
    });

    it("names every member of a cycle", async () => {
      const { resolvePluginLoadOrder } = await import("@semio-tech/framework");
      const result = resolvePluginLoadOrder([
        { pluginId: "a", dependencies: [{ pluginId: "b", version: "*" }] },
        { pluginId: "b", dependencies: [{ pluginId: "a", version: "*" }] },
      ]);
      expect(result.order).toEqual([]);
      expect(result.errors).toEqual([{ code: "transaction.cycle", members: ["a", "b"] }]);
    });

    it("versionSatisfies matches the frozen grammar (*, =, ^, ~, >=), including caret's leading-zero tiers", async () => {
      const { versionSatisfies } = await import("@semio-tech/framework");
      expect(versionSatisfies("1.2.3", "*")).toBe(true);
      expect(versionSatisfies("1.2.3", "=1.2.3")).toBe(true);
      expect(versionSatisfies("1.2.4", "=1.2.3")).toBe(false);
      expect(versionSatisfies("1.9.0", "^1.2.3")).toBe(true);
      expect(versionSatisfies("2.0.0", "^1.2.3")).toBe(false);
      expect(versionSatisfies("0.2.9", "^0.2.3")).toBe(true);
      expect(versionSatisfies("0.3.0", "^0.2.3")).toBe(false);
      expect(versionSatisfies("0.0.9", "^0.0.3")).toBe(false);
      expect(versionSatisfies("0.0.3", "^0.0.3")).toBe(true);
      expect(versionSatisfies("1.2.9", "~1.2.3")).toBe(true);
      expect(versionSatisfies("1.3.0", "~1.2.3")).toBe(false);
      expect(versionSatisfies("1.2.3", ">=1.2.3")).toBe(true);
      expect(versionSatisfies("9.9.9", ">=1.2.3")).toBe(true);
      expect(versionSatisfies("1.2.2", ">=1.2.3")).toBe(false);
    });

    it("orderPluginRegistryEntries drops only the blocked entries, dependency-orders the rest", async () => {
      const { orderPluginRegistryEntries } = await import("@semio-tech/framework");
      const result = orderPluginRegistryEntries([
        { pluginId: "b", moduleUrl: "b.js", dependencies: [{ pluginId: "a", version: "*" }] },
        { pluginId: "a", moduleUrl: "a.js" },
        { pluginId: "broken", moduleUrl: "broken.js", dependencies: [{ pluginId: "missing", version: "*" }] },
      ]);
      expect(result.order.map((entry) => entry.pluginId)).toEqual(["a", "b"]);
      expect(result.errors).toEqual([{ code: "transaction.dependency-missing", pluginId: "broken", dependsOn: "missing" }]);
    });

    it("pluginGraphErrorMessage renders a real English and a real German message", async () => {
      const { pluginGraphErrorMessage } = await import("@semio-tech/framework");
      const error = { code: "transaction.dependency-missing" as const, pluginId: "b", dependsOn: "a" };
      expect(pluginGraphErrorMessage(error, "en")).toContain("needs");
      expect(pluginGraphErrorMessage(error, "de")).toContain("benötigt");
    });

    it("PluginGraph.canUnload refuses while a loaded dependent exists, allows once it's gone", async () => {
      const { PluginGraph } = await import("@semio-tech/framework");
      const graph = new PluginGraph([{ pluginId: "a" }, { pluginId: "b", dependencies: [{ pluginId: "a", version: "*" }] }]);
      expect(graph.canUnload("a", new Set(["a", "b"]))).toBe(false);
      expect(graph.canUnload("a", new Set(["a"]))).toBe(true);
    });
  });

  describe("@semio-tech/framework InstanceDirectory and ArtifactRouters", () => {
    it("InstanceDirectory registers, resolves, and unregisters", async () => {
      const { InstanceDirectory } = await import("@semio-tech/framework");
      const directory = new InstanceDirectory();
      directory.register("artifact-1", { pluginId: "cad", instanceId: 3, artifactKind: "s.cad.model" });
      expect(directory.resolve("artifact-1")).toEqual({ pluginId: "cad", instanceId: 3, artifactKind: "s.cad.model" });
      directory.unregister("artifact-1");
      expect(directory.resolve("artifact-1")).toBeUndefined();
    });

    it("ArtifactMutationRouter accepts a byte-identical re-registration, rejects a conflicting one", async () => {
      const { ArtifactMutationRouter } = await import("@semio-tech/framework");
      const router = new ArtifactMutationRouter();
      router.registerOwner("s.cad.model", "s.cad#add-wall");
      router.registerOwner("s.cad.model", "s.cad#add-wall");
      expect(router.resolve("s.cad.model", "s.cad#add-wall")).toEqual({ kind: "owner" });
      expect(() =>
        router.registerContributed(
          "s.cad.model",
          "aec-building",
          "cad",
          { mutationId: "s.cad#add-wall", semantics: { verb: "add", entity: "wall", kind: "add-wall", record: "Wall" }, schemaVersion: 1, algorithmVersion: 1 },
          true,
        ),
      ).toThrow(/conflict/);
    });

    it("ArtifactMutationRouter.registerContributed rejects a contributor that doesn't depend on the owner", async () => {
      const { ArtifactMutationRouter } = await import("@semio-tech/framework");
      const router = new ArtifactMutationRouter();
      expect(() =>
        router.registerContributed(
          "s.cad.model",
          "aec-building",
          "cad",
          { mutationId: "s.cad#aec-building:add-room", semantics: { verb: "add", entity: "room", kind: "add-room", record: "Room" }, schemaVersion: 1, algorithmVersion: 1 },
          false,
        ),
      ).toThrow(/not a direct dependency/);
    });

    it("ArtifactInferenceRouter enforces owner === contributor and orders the depends_on DAG", async () => {
      const { ArtifactInferenceRouter } = await import("@semio-tech/framework");
      const router = new ArtifactInferenceRouter();
      router.registerContributed(
        "s.cad.model",
        {
          owner: "aec-building",
          artifactKind: "s.cad.model",
          artifactSchema: "s.cad.model",
          artifactSchemaVersion: 1,
          documentSchema: "s.cad",
          documentSchemaVersion: 1,
          inferenceSchema: "s.aec-building.load-path",
          inferenceSchemaVersion: 1,
          algorithmVersion: 1,
          policyVersion: 1,
          contributor: "aec-building",
          dependsOn: [],
        },
        true,
      );
      expect(router.resolve("s.cad.model", "s.aec-building.load-path")).toEqual({ kind: "contributed", pluginId: "aec-building" });
      expect(router.dependencyOrder()).toEqual(["s.cad.model s.aec-building.load-path"]);
    });

    it("ArtifactInferenceRouter.registerContributed rejects owner !== contributor", async () => {
      const { ArtifactInferenceRouter } = await import("@semio-tech/framework");
      const router = new ArtifactInferenceRouter();
      expect(() =>
        router.registerContributed(
          "s.cad.model",
          {
            owner: "someone-else",
            artifactKind: "s.cad.model",
            artifactSchema: "s.cad.model",
            artifactSchemaVersion: 1,
            documentSchema: "s.cad",
            documentSchemaVersion: 1,
            inferenceSchema: "s.aec-building.load-path",
            inferenceSchemaVersion: 1,
            algorithmVersion: 1,
            policyVersion: 1,
            contributor: "aec-building",
            dependsOn: [],
          },
          true,
        ),
      ).toThrow(/owner\/contributor mismatch/);
    });
  });

}

export async function registerTests3(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: any, source: TestSource): Promise<void> {
  const { descriptorDigestEncodingV1, descriptorDigestV1, emptyDirectoryReadModel, foldAll } = dependencies;
  type DirectoryEvent = any;
  type DirectoryReadModel = any;
  type DocumentDescriptor = any;

  const { describe, expect, it } = vitest;

  describe("@semio-tech/framework-os directory", () => {
    const loadFixtureEvents = async (): Promise<DirectoryEvent[]> => {
      const { readFileSync } = await import("node:fs");
      const { fileURLToPath } = await import("node:url");
      const { dirname, join } = await import("node:path");
      const here = dirname(fileURLToPath(source.url));
      const raw = readFileSync(join(here, "🧫️fixtures", "📇️directory", "⚡️events.json"), "utf8");
      return (JSON.parse(raw) as { events: DirectoryEvent[] }).events;
    };

    it("folds the golden fixture into the expected projection (parity with the Rust twin)", async () => {
      const events = await loadFixtureEvents();
      const model: DirectoryReadModel = foldAll(emptyDirectoryReadModel(), events);

      expect(model.cursor).toBe(16);
      expect(model.spaces.size).toBe(1);
      expect(model.spaces.has("sp-atelier-amara")).toBe(false);

      const studio = model.spaces.get("sp-studio-fabrication");
      expect(studio?.view.name).toBe("Fabrication Studio");
      expect(studio?.view.visibility).toBe("public");
      expect(studio?.view.kind).toBe("archive");
      expect(studio?.view.memberCount).toBe(2);

      const roles = (studio?.members ?? []).map((member) => [member.userId, member.role]).sort();
      expect(roles).toEqual([
        ["u-amara", "spectator"],
        ["u-devon", "spectator"],
      ]);

      const devon = studio?.members.find((member) => member.userId === "u-devon");
      expect(devon?.email).toBe("devon@semio.dev");
    });

    it("is idempotent on replay", async () => {
      const events = await loadFixtureEvents();
      const once = foldAll(emptyDirectoryReadModel(), events);
      const twice = foldAll(once, events);
      expect(twice).toEqual(once);
    });

    it("preserves the language-neutral document descriptor fixture canonically", async () => {
      const { readFileSync } = await import("node:fs");
      const { fileURLToPath } = await import("node:url");
      const { dirname, join } = await import("node:path");
      const here = dirname(fileURLToPath(source.url));
      const fixture = JSON.parse(readFileSync(join(here, "🧫️fixtures", "📇️directory", "🪪️document-descriptor.json"), "utf8")) as { valid: import("./🔨️modules/📇️directory/🧬️schema/🟦️.ts").DocumentDescriptor; canonical: string };
      expect(JSON.stringify(fixture.valid)).toBe(fixture.canonical);
    });

    it("derives the domain-separated descriptor digest independently of JSON", async () => {
      const { createHash } = await import("node:crypto");
      const { readFileSync } = await import("node:fs");
      const { fileURLToPath } = await import("node:url");
      const { dirname, join } = await import("node:path");
      const here = dirname(fileURLToPath(source.url));
      const fixture = JSON.parse(readFileSync(join(here, "🧫️fixtures", "📇️directory", "🛡️artifact-authority.json"), "utf8")) as { descriptor: DocumentDescriptor; descriptorEncodingHex: string; descriptorDigestV1: number[] };
      const encoded = descriptorDigestEncodingV1(fixture.descriptor);
      expect(Buffer.from(encoded).toString("hex")).toBe(fixture.descriptorEncodingHex);
      expect(Array.from(await descriptorDigestV1(fixture.descriptor))).toEqual(fixture.descriptorDigestV1);
      expect(Array.from(createHash("sha256").update(encoded).digest())).toEqual(fixture.descriptorDigestV1);
      expect(Buffer.from(encoded).includes(Buffer.from(JSON.stringify(fixture.descriptor)))).toBe(false);
      expect(() => descriptorDigestEncodingV1({ ...fixture.descriptor, spaceId: "" })).toThrow();
      expect(() => descriptorDigestEncodingV1({ ...fixture.descriptor, packSchemaHash: "AA".repeat(32) })).toThrow();
      expect(() => descriptorDigestEncodingV1({ ...fixture.descriptor, bootstrapFrontier: { ...fixture.descriptor.bootstrapFrontier, headSeq: Number.MAX_SAFE_INTEGER + 1 } })).toThrow();
    });

    it("projects document.announced without allowing replay to redefine it", async () => {
      const { readFileSync } = await import("node:fs");
      const { fileURLToPath } = await import("node:url");
      const { dirname, join } = await import("node:path");
      const here = dirname(fileURLToPath(source.url));
      const fixture = JSON.parse(readFileSync(join(here, "🧫️fixtures", "📇️directory", "🪪️document-descriptor.json"), "utf8")) as { valid: import("./🔨️modules/📇️directory/🧬️schema/🟦️.ts").DocumentDescriptor; conflictingSchemaHash: import("./🔨️modules/📇️directory/🧬️schema/🟦️.ts").DocumentDescriptor };
      const created: DirectoryEvent = { seq: 1, id: "space", hlc: { physicalMs: 1, logical: 0 }, actor: { kind: "system", id: "system:test" }, spaceId: fixture.valid.spaceId, body: { kind: "space.created", spaceId: fixture.valid.spaceId, name: "Fixture", spaceKind: "studio", visibility: "private", ownerUserId: "user-owner" }, recordedAtMs: 1 };
      const announced: DirectoryEvent = { seq: 2, id: "document", hlc: { physicalMs: 2, logical: 0 }, actor: { kind: "user", id: "user:user-owner#test" }, spaceId: fixture.valid.spaceId, body: { kind: "document.announced", descriptor: fixture.valid }, recordedAtMs: 2 };
      const conflictReplay: DirectoryEvent = { ...announced, seq: 3, id: "conflict", body: { kind: "document.announced", descriptor: fixture.conflictingSchemaHash } };
      const model = foldAll(emptyDirectoryReadModel(), [created, announced, conflictReplay]);

      expect(model.spaces.get(fixture.valid.spaceId)?.documents).toEqual([fixture.valid]);
      expect(model.spaces.get(fixture.valid.spaceId)?.view.documentCount).toBe(1);
    });
  });

}

export async function registerTests4(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: any, source: TestSource): Promise<void> {
  const { BACKBONE_WORKER_WIRE_MAGIC, DIRECTORY_HTTP_TIMEOUT_MS, DirectoryClient, HUB_HEALTHY_RESET_MS, HUB_RECONNECT_MAX_MS, HUB_RECONNECT_MIN_MS, decodeBackboneWorkerRequest, decodeBackboneWorkerResponse, decodePackValue, encodeBackboneMessage, encodeBackboneWorkerRequest, encodeBackboneWorkerResponse, encodePackValue, fetchWithTimeout, parseBrowserActorUiPatchOfferV1, parseBrowserActorUiPatchResultV1, parseDirectorySpaceAdministrationPageV1, parseDocumentBackboneMessage } = dependencies;
  type BackboneWorkerRequest = any;
  type BackboneWorkerResponse = any;
  type BrowserActorUiMountedV1 = any;
  type DirectoryEvent = any;
  type DirectoryStreamMessage = any;
  type SocketGrantIssuerV1 = any;
  type SocketGrantReceiptV1 = any;

  const { describe, expect, it, vi } = vitest;

  class FakeDirectoryWebSocket {
    static instances: FakeDirectoryWebSocket[] = [];
    readonly url: string;
    readonly protocol = "semio.socket.v1";
    readyState = 0;
    onopen: (() => void) | null = null;
    onmessage: ((event: { data: string }) => void) | null = null;
    onclose: ((event: CloseEvent) => void) | null = null;
    onerror: (() => void) | null = null;
    constructor(url: string, readonly protocols?: string | string[]) {
      this.url = url;
      FakeDirectoryWebSocket.instances.push(this);
    }
    send(): void {}
    close(): void {
      this.readyState = 3;
    }
    triggerOpen(): void {
      this.readyState = 1;
      this.onopen?.();
    }
    triggerMessage(message: DirectoryStreamMessage): void {
      this.onmessage?.({ data: JSON.stringify(message) });
    }
    triggerClose(code = 1006): void {
      this.readyState = 3;
      this.onclose?.({ code } as CloseEvent);
    }
  }

  const testSocketGrantReceipt: SocketGrantReceiptV1 = {
    schema: "semio.hub.socket-grant/v1",
    protocol: "semio.socket.v1",
    grant: `socket.v1.${"1".repeat(32)}.${"2".repeat(64)}`,
    actorId: `hub.v1.${"3".repeat(64)}`,
    expiresAtMs: Number.MAX_SAFE_INTEGER,
  };
  const testSocketGrantIssuer: SocketGrantIssuerV1 = {
    issueDirectory: async () => testSocketGrantReceipt,
    issueDirectoryScoped: async () => testSocketGrantReceipt,
    issueDocument: async () => testSocketGrantReceipt,
  };
  const testDirectoryClient = (): DirectoryClient => new DirectoryClient("http://hub.test", { socketGrantIssuer: testSocketGrantIssuer });

  function sampleDirectoryEvent(seq: number): DirectoryEvent {
    return {
      seq,
      id: `evt-${seq}`,
      hlc: { physicalMs: seq, logical: 0 },
      actor: { kind: "user", id: "u-1" },
      body: { kind: "space.renamed", spaceId: "sp-1", name: `space ${seq}` },
      recordedAtMs: seq,
    };
  }

  async function sampleCanonicalDirectoryEventPage(after = 3): Promise<string> {
    const unsigned = {
      schema: "semio.directory.event-page.v1" as const,
      sessionBindingSha256: "a".repeat(64),
      authorizationGeneration: 7,
      afterSeqExclusive: after,
      throughSeqInclusive: after + 2,
      hasMore: true,
      events: [] as DirectoryEvent[],
    };
    const digest = new Uint8Array(await globalThis.crypto.subtle.digest("SHA-256", new TextEncoder().encode(JSON.stringify(unsigned))));
    const receiptSha256 = Array.from(digest, (byte) => byte.toString(16).padStart(2, "0")).join("");
    return JSON.stringify({ ...unsigned, receiptSha256 });
  }

  describe("document backbone worker wire", () => {
    it("admits inference opening only after its exact worker receipt", async () => {
      const { readFileSync } = await import("node:fs");
      const { default: Ajv } = await import("ajv");
      const { default: equal } = await import("fast-deep-equal");
      const fixture = JSON.parse(readFileSync(new URL("./🔨️modules/💡️inference/🚪️opening/🧫️fixtures/🔣️.json", source.url), "utf8"));
      const schema = JSON.parse(readFileSync(new URL("./🔨️modules/💡️inference/🚪️opening/🧬️schema/🔣️.json", source.url), "utf8"));
      expect(new Ajv({ strict: true }).compile(schema)(fixture)).toBe(true);
      const { InferencePortOpeningMailboxV1, parseInferencePortClosedV1, parseInferencePortOpeningResultV1 } = await import("../../🔨️modules/💡️inference/🚪️opening/🟦️.ts");
      expect(equal(parseInferencePortClosedV1(fixture.closed), fixture.closed)).toBe(true);
      expect(equal(decodeBackboneWorkerResponse(encodeBackboneWorkerResponse(fixture.closed)), fixture.closed)).toBe(true);
      for (const extra of [{ authority: "forged" }, { operationEpoch: 0 }, { kind: "inference-port-opened" }]) expect(() => parseInferencePortClosedV1({ ...fixture.closed, ...extra })).toThrow();
      const sent: unknown[] = [];
      const mailbox = new InferencePortOpeningMailboxV1((request) => { sent.push(request); });
      try {
        const pending = mailbox.open(fixture.request);
        expect(equal(sent, [fixture.request])).toBe(true);
        expect(mailbox.settle({ ...fixture.opened, operationEpoch: 20 })).toBe(false);
        expect(mailbox.settle({ ...fixture.opened, scope: { ...fixture.opened.scope, documentId: "foreign" } })).toBe(false);
        await expect(mailbox.open(fixture.request)).rejects.toThrow("pending");
        expect(mailbox.settle(fixture.opened)).toBe(true);
        expect(equal(await pending, fixture.opened)).toBe(true);
        expect(mailbox.settle(fixture.opened)).toBe(false);
        await expect(mailbox.open(fixture.request)).rejects.toThrow("replayed");
        const refused = expect(mailbox.open({ ...fixture.request, operationEpoch: 22 })).rejects.toThrow("inference.capacity");
        expect(mailbox.settle(fixture.opened)).toBe(false);
        expect(mailbox.settle({ ...fixture.refused, operationEpoch: 22 })).toBe(true);
        await refused;
        const closed = expect(mailbox.open({ ...fixture.request, operationEpoch: 23 })).rejects.toThrow("retired");
        mailbox.close("retired");
        await closed;
        expect(mailbox.settle(fixture.opened)).toBe(false);
        expect(equal(decodeBackboneWorkerResponse(encodeBackboneWorkerResponse(fixture.opened)), fixture.opened)).toBe(true);
        for (const extra of [{ code: "inference.capacity" }, { authority: "forged" }, { operationEpoch: 0 }]) expect(() => parseInferencePortOpeningResultV1({ ...fixture.opened, ...extra })).toThrow();
        const valid = new Ajv({ strict: true }).compile(schema);
        for (const [field, code] of [["indeterminate", "inference.capacity"], ["refused", "inference.transport"]]) {
          const hostile = { ...fixture[field!], code };
          expect(() => parseInferencePortOpeningResultV1(hostile)).toThrow();
          expect(valid({ ...fixture, [field!]: hostile })).toBe(false);
        }
      } finally { mailbox.close("test cleanup"); }
    });

    it("projects real browser intent publications and preserves bounded owned inference effects", async () => {
      const { readFileSync } = await import("node:fs");
      const { default: Ajv } = await import("ajv");
      const { default: equal } = await import("fast-deep-equal");
      const fixture = JSON.parse(readFileSync(new URL("./🔨️modules/🔌️plugin/🌐️browser-bundle/🎯️action-handoff/🧫️fixtures/🔣️.json", source.url), "utf8"));
      const schema = JSON.parse(readFileSync(new URL("./🔨️modules/🔌️plugin/🌐️browser-bundle/🎯️action-handoff/🧬️schema/🔣️.json", source.url), "utf8"));
      const valid = new Ajv({ strict: true }).compile(schema);
      expect(valid(fixture), JSON.stringify(valid.errors)).toBe(true);
      const { decodeBrowserActorCommandPublicationV1, decodeBrowserActorIntentPublicationV1, encodeBrowserActorHostEffectV1, decodeBrowserActorHostEffectsV1, publishBrowserActorHostEffectsV1, requireBrowserActorCommandBackboneProjectionV1 } = await import("../../🔨️modules/🔌️plugin/🌐️browser-bundle/🎯️action-handoff/📤️publication/🟦️.ts");
      const { encodeAppFrame, encodePackValue } = await import("../../🟦️.ts");
      const { parseBrowserActorActionResultV1 } = await import("../../🔨️modules/🔌️plugin/🌐️browser-bundle/🎯️action-handoff/🟦️.ts");
      const emit = encodeAppFrame(fixture.publication.emit);
      expect(decodeBrowserActorIntentPublicationV1(emit)).toEqual({ kind: "emit" });
      expect(decodeBrowserActorIntentPublicationV1(encodeAppFrame(fixture.publication.error)).kind).toBe("error");
      expect(() => decodeBrowserActorIntentPublicationV1(encodeAppFrame({ Error: { ...fixture.publication.error.Error, in_reply_to: 0 } }))).toThrow();
      expect(() => decodeBrowserActorIntentPublicationV1(new Uint8Array([...emit, 0]))).toThrow();
      expect(() => decodeBrowserActorIntentPublicationV1(encodeAppFrame({ Emit: { ...fixture.publication.emit.Emit, in_reply_to: 1 } }))).toThrow();
      const invocation = encodeAppFrame({ Invocation: { in_reply_to: fixture.commandRequest.actionSequence, output: [], diagnostics: [], ui_scope: [], history_patch: [], messages: [], mutations: [], inverse_group: [] } });
      const emptyInvocation = decodeBrowserActorCommandPublicationV1(invocation, fixture.commandRequest.actionSequence);
      expect(emptyInvocation.kind).toBe("invocation");
      if (emptyInvocation.kind !== "invocation") throw new Error("expected invocation");
      expect(() => requireBrowserActorCommandBackboneProjectionV1(emptyInvocation, [])).not.toThrow();
      expect(decodeBrowserActorCommandPublicationV1(encodeAppFrame({ Error: { in_reply_to: fixture.commandRequest.actionSequence, fault: [], report: [] } }), fixture.commandRequest.actionSequence)).toEqual({ kind: "error", reason: "action-guest-refused" });
      expect(() => decodeBrowserActorCommandPublicationV1(invocation, fixture.commandRequest.actionSequence + 1)).toThrow();
      expect(() => decodeBrowserActorCommandPublicationV1(new Uint8Array([...invocation, 0]), fixture.commandRequest.actionSequence)).toThrow();
      expect(() => decodeBrowserActorCommandPublicationV1(encodeAppFrame({ Invocation: { in_reply_to: fixture.commandRequest.actionSequence, output: [], diagnostics: [], ui_scope: [], history_patch: [1], messages: [], mutations: [], inverse_group: [] } }), fixture.commandRequest.actionSequence)).toThrow("action-publication-unprojected");
      const mutationId = "command-mutation-1",
        invocationId = "command:0:1",
        forward = [1, 2],
        inverse = [3, 4],
        inverseVector = [1, 2, 3, 4],
        mutation = {
          id: mutationId,
          document: "0",
          baseVersion: 0,
          invocationId,
          diff: { schema: "demo/v1.operation", payload: forward },
          inverse: { targetMutation: mutationId, inverseDiff: { schema: "demo/v1.operation.inverse", payload: inverseVector }, baseVersion: 0, dependencies: [], undoPolicy: "ExactBaseOnly" },
          dependencies: ["prior"],
          author: "actor-1",
          timestamp: { actor: 7, physical_ms: 8, logical: 9 },
        },
        inverseGroup = { invocationId, mutations: [mutationId], inverseMutations: [mutation.inverse] },
        mutationFrame = encodeAppFrame({ Invocation: { in_reply_to: fixture.commandRequest.actionSequence, output: [], diagnostics: [], ui_scope: [], history_patch: [], messages: [], mutations: Array.from(encodePackValue([mutation])), inverse_group: Array.from(encodePackValue(inverseGroup)) } }),
        mutationPublication = decodeBrowserActorCommandPublicationV1(mutationFrame, fixture.commandRequest.actionSequence),
        envelope = { mutation_id: mutationId, actor: "actor-1", dependencies: ["prior"], diff: { schema: "demo/v1", payload: Uint8Array.from(forward) }, inverse: { schema: "demo/v1", payload: Uint8Array.from(inverse) }, timestamp: { actor: 7n, physical_ms: 8n, logical: 9n } };
      expect(mutationPublication.kind).toBe("invocation");
      if (mutationPublication.kind !== "invocation") throw new Error("expected mutation invocation");
      expect(() => requireBrowserActorCommandBackboneProjectionV1(mutationPublication, [envelope])).not.toThrow();
      expect(() => requireBrowserActorCommandBackboneProjectionV1(mutationPublication, [])).toThrow("action-publication-unprojected");
      expect(() => requireBrowserActorCommandBackboneProjectionV1(mutationPublication, [{ ...envelope, diff: { ...envelope.diff, payload: Uint8Array.of(9) } }])).toThrow("action-publication-unprojected");
      expect(() => requireBrowserActorCommandBackboneProjectionV1(mutationPublication, [{ ...envelope, inverse: { ...envelope.inverse, payload: Uint8Array.of(9) } }])).toThrow("action-publication-unprojected");
      const encoded = encodeBrowserActorHostEffectV1(fixture.publication.hostEffect);
      expect(equal(encoded, Array.from(encodePackValue(fixture.publication.projectedEffect)))).toBe(true);
      expect(equal(decodeBrowserActorHostEffectsV1([encoded]), [fixture.publication.projectedEffect])).toBe(true);
      const external = encodeBrowserActorHostEffectV1(fixture.publication.externalHostEffect);
      expect(equal(external, Array.from(encodePackValue(fixture.publication.externalProjectedEffect)))).toBe(true);
      expect(equal(decodeBrowserActorHostEffectsV1([external]), [fixture.publication.externalProjectedEffect])).toBe(true);
      for (const url of ["javascript:alert(1)", "data:text/html,script", "file:///secret", "/relative", "https://user:secret@example.invalid", "https://example.invalid/\nforged", "https://example.invalid/" + "a".repeat(2048)]) {
        expect(() => encodeBrowserActorHostEffectV1({ tag: "open-external-url", val: { url } })).toThrow();
      }
      const result = { ...fixture.acknowledged, hostEffects: [encoded] };
      expect(equal(parseBrowserActorActionResultV1(result), result)).toBe(true);
      expect(result.outcome).toBe("guest-applied");
      expect(() => parseBrowserActorActionResultV1({ ...result, outcome: "acknowledged" })).toThrow();
      expect(() => parseBrowserActorActionResultV1({ ...fixture.rejected, hostEffects: [encoded] })).toThrow();
      for (const effect of [
        { ...fixture.publication.hostEffect, val: { kind: "foreign" } },
        { ...fixture.publication.hostEffect, val: { kind: "gis-map-bounds-region", documentId: "forged" } },
        { tag: "invoke-extension", val: {} },
        { tag: "publish-event", val: {} },
      ]) expect(() => encodeBrowserActorHostEffectV1(effect)).toThrow();
      expect(() => decodeBrowserActorHostEffectsV1([[...encoded, 0]])).toThrow();
      expect(() => decodeBrowserActorHostEffectsV1(new Array(65).fill(encoded))).toThrow();
      expect(() => parseBrowserActorActionResultV1({ ...result, hostEffects: [new Array(262144).fill(1), [1]] })).toThrow();
      const published: unknown[] = [];
      const publish = (effect: unknown) => { published.push(effect); };
      await expect(publishBrowserActorHostEffectsV1([encoded], () => false, publish)).rejects.toThrow("retired");
      expect(published).toEqual([]);
      await expect(publishBrowserActorHostEffectsV1([encoded, [...encoded, 0]], () => true, publish)).rejects.toThrow();
      expect(published).toEqual([]);
      await publishBrowserActorHostEffectsV1([encoded], () => true, publish);
      expect(equal(published, [fixture.publication.projectedEffect])).toBe(true);
      await expect(publishBrowserActorHostEffectsV1([encoded, encoded], () => true, publish)).rejects.toThrow();
      expect(equal(published, [fixture.publication.projectedEffect])).toBe(true);
    });

    it("settles only the exact live browser intent mailbox and retires pending work", async () => {
      const { readFileSync } = await import("node:fs");
      const { default: equal } = await import("fast-deep-equal");
      const fixture = JSON.parse(readFileSync(new URL("./🔨️modules/🔌️plugin/🌐️browser-bundle/🎯️action-handoff/🧫️fixtures/🔣️.json", source.url), "utf8"));
      const { BrowserActorActionMailboxV1 } = await import("../../🔨️modules/🔌️plugin/🌐️browser-bundle/🎯️action-handoff/📮️requests/🟦️.ts");
      const { actionSequence: _sequence, ...owner } = fixture.request;
      const intent = { ...fixture.uiIntent, seq: BigInt(fixture.uiIntent.seq) };
      const sent: any[] = [];
      const mailbox = new BrowserActorActionMailboxV1((request) => { sent.push(request); }, 1000);
      try {
        const pending = mailbox.dispatchIntent(owner, "map", intent);
        expect(sent).toHaveLength(1);
        expect(sent[0].actionSequence).toBe(1);
        await expect(mailbox.dispatchIntent(owner, "map", intent)).rejects.toThrow("pending");
        for (const hostile of fixture.hostileResults.slice(0, 3)) expect(mailbox.settle({ ...hostile, actionSequence: 1 })).toBe(false);
        expect(mailbox.settle(fixture.acknowledged)).toBe(false);
        const acknowledged = { ...fixture.acknowledged, actionSequence: 1 };
        expect(mailbox.settle(acknowledged)).toBe(true);
        expect(equal(await pending, acknowledged)).toBe(true);
        expect(mailbox.settle(acknowledged)).toBe(false);
        const next = mailbox.dispatchIntent(owner, "map", intent);
        const retired = expect(next).rejects.toThrow("retired");
        expect(sent[1].actionSequence).toBe(2);
        mailbox.close("retired");
        await retired;
        expect(mailbox.settle({ ...fixture.acknowledged, actionSequence: 2 })).toBe(false);
        await expect(mailbox.dispatchIntent(owner, "map", intent)).rejects.toThrow("closed");
      } finally {
        mailbox.close("test cleanup");
      }
      const failing = new BrowserActorActionMailboxV1(() => { throw new Error("enqueue failed"); });
      await expect(failing.dispatchIntent(owner, "map", intent)).rejects.toThrow("enqueue failed");
      failing.close("test cleanup");
      const retried: number[] = [];
      const retry = new BrowserActorActionMailboxV1((request) => { retried.push(request.actionSequence); });
      try {
        await expect(retry.dispatchIntent(owner, "map", { ...intent, revision: 4 })).rejects.toThrow("stale surface");
        expect(retried).toEqual([]);
        const refused = expect(retry.dispatchIntent(owner, "map", intent)).rejects.toThrow(fixture.rejected.reason);
        expect(retried).toEqual([2]);
        expect(retry.settle({ ...fixture.rejected, actionSequence: 2 })).toBe(true);
        await refused;
        const next = retry.dispatchIntent(owner, "map", intent);
        expect(retried).toEqual([2, 3]);
        expect(retry.settle({ ...fixture.acknowledged, actionSequence: 3 })).toBe(true);
        await next;
      } finally {
        retry.close("test cleanup");
      }
      vi.useFakeTimers();
      const timed = new BrowserActorActionMailboxV1(() => {}, 10);
      try {
        const expired = expect(timed.dispatchIntent(owner, "map", intent)).rejects.toThrow("unconfirmed");
        await vi.advanceTimersByTimeAsync(10);
        await expired;
        await expect(timed.dispatchIntent(owner, "map", intent)).rejects.toThrow("closed");
      } finally {
        timed.close("test cleanup");
        vi.useRealTimers();
      }
    });

    it("preserves the complete browser UI intent and its unsigned sequence above JSON precision", async () => {
      const { readFileSync } = await import("node:fs");
      const { default: Ajv } = await import("ajv");
      const { default: equal } = await import("fast-deep-equal");
      const fixture = JSON.parse(readFileSync(new URL("./🔨️modules/🔌️plugin/🌐️browser-bundle/🎯️action-handoff/🧫️fixtures/🔣️.json", source.url), "utf8"));
      const schema = JSON.parse(readFileSync(new URL("./🔨️modules/🔌️plugin/🌐️browser-bundle/🎯️action-handoff/🧬️schema/🔣️.json", source.url), "utf8"));
      const valid = new Ajv({ strict: true }).compile(schema);
      expect(valid(fixture), JSON.stringify(valid.errors)).toBe(true);
      const { createBrowserActorUiIntentRequestV1 } = await import("../../🔨️modules/🔌️plugin/🌐️browser-bundle/🎯️action-handoff/🧭️intent/🟦️.ts");
      const { decodePackValue, packUInt } = await import("../../🟦️.ts");
      const intent = { ...fixture.uiIntent, seq: BigInt(fixture.uiIntent.seq) };
      const request = createBrowserActorUiIntentRequestV1(fixture.request, "map", intent);
      expect(request.actionSequence).toBe(9);
      expect(equal(decodePackValue(new Uint8Array(request.payload.bytes)), { ...intent, surface: "0:map", revision: packUInt(3n), node: packUInt(42n), action: { ...intent.action, version: packUInt(1n) }, seq: packUInt(0xffffffffffffffffn) })).toBe(true);
      expect(intent.surface).toBe("map");
      expect(valid({ ...fixture, uiIntent: { ...fixture.uiIntent, seq: "0" } })).toBe(true);
      expect((decodePackValue(new Uint8Array(createBrowserActorUiIntentRequestV1(fixture.request, "map", { ...intent, seq: 0n }).payload.bytes)) as Readonly<Record<string, unknown>>).seq).toEqual(packUInt(0n));
      for (const extra of [{ surface: "other" }, { revision: 4 }, { node: -1 }, { seq: 0x10000000000000000n }]) expect(() => createBrowserActorUiIntentRequestV1(fixture.request, "map", { ...intent, ...extra })).toThrow();
      for (const extra of [{ activationGeneration: "18446744073709551616" }, { actionSequence: 9007199254740992 }, { surfaceRevision: 9007199254740992 }]) {
        expect(valid({ ...fixture, request: { ...fixture.request, ...extra } })).toBe(false);
        expect(() => createBrowserActorUiIntentRequestV1({ ...fixture.request, ...extra }, "map", intent)).toThrow();
      }
    });

    it("admits only exact direct browser intent owners on the private worker wire", async () => {
      const { readFileSync } = await import("node:fs");
      const { default: equal } = await import("fast-deep-equal");
      const fixture = JSON.parse(readFileSync(new URL("./🔨️modules/🔌️plugin/🌐️browser-bundle/🎯️action-handoff/🧫️fixtures/🔣️.json", source.url), "utf8"));
      const request = { ...fixture.request, clientInstanceId: "12345678-1234-4123-8123-123456789abc" };
      const response = { ...fixture.acknowledged, clientInstanceId: "12345678-1234-4123-8123-123456789abc" };
      expect(equal(decodeBackboneWorkerRequest(encodeBackboneWorkerRequest(request)), request)).toBe(true);
      expect(equal(decodeBackboneWorkerResponse(encodeBackboneWorkerResponse(response)), response)).toBe(true);
      for (const extra of [{ clientInstanceId: "" }, { authority: "forged" }, { command: [1] }, { surfaceRevision: 0 }, { payload: { kind: "ui-intent", bytes: [] } }]) {
        expect(() => decodeBackboneWorkerRequest(encodeBackboneWorkerRequest({ ...request, ...extra }))).toThrow();
      }
      for (const extra of [{ clientInstanceId: "" }, { authority: "forged" }, { outcome: "rejected" }, { actionSequence: 0 }]) {
        expect(() => decodeBackboneWorkerResponse(encodeBackboneWorkerResponse({ ...response, ...extra }))).toThrow();
      }
    });

    const fromHex = (hex: string): Uint8Array => new Uint8Array(Buffer.from(hex, "hex"));

    it("retains one exact causal OpBinary through actor send and receive frames", () => {
      const batch = fromHex("01016d0164016100017301aa016902bbcc03ffffffffffffffffff0105");
      const message = encodeBackboneMessage({ kind: "mutations", envelopes: batch });
      const clientInstanceId = "12345678-1234-4123-8123-123456789abc";
      const request = { kind: "send", documentId: "d", clientInstanceId, message: { kind: "documentBackbone", message } } as const;
      const response = { kind: "event", documentId: "d", clientInstanceId, event: { kind: "documentBackbone", message } } as const;
      const requestRound = decodeBackboneWorkerRequest(encodeBackboneWorkerRequest(request));
      const responseRound = decodeBackboneWorkerResponse(encodeBackboneWorkerResponse(response));
      expect(requestRound).toEqual(request);
      expect(responseRound).toEqual(response);
      expect(parseDocumentBackboneMessage((requestRound as typeof request).message.message).envelopes[0]?.timestamp.physical_ms).toBe(0xffff_ffff_ffff_ffffn);
    });

    it("rejects non-mutation, noncanonical and over-cap actor messages", () => {
      const snapshot = encodeBackboneMessage({ kind: "snapshot", pack: new Uint8Array(), spr: new Uint8Array() });
      const ack = encodeBackboneMessage({ kind: "ack", opIds: [] });
      const trailing = encodeBackboneMessage({ kind: "mutations", envelopes: fromHex("0000") });
      expect(() => parseDocumentBackboneMessage(snapshot)).toThrow("mutations required");
      expect(() => parseDocumentBackboneMessage(ack)).toThrow("mutations required");
      expect(() => parseDocumentBackboneMessage(trailing)).toThrow("trailing-bytes");
      expect(() => parseDocumentBackboneMessage(new Uint8Array(262_145))).toThrow("hot byte limit");

      const malformedFields = encodePackValue({ kind: "send", documentId: "d", clientInstanceId: "12345678-1234-4123-8123-123456789abc", message: { kind: "documentBackbone", message: Array.from(encodeBackboneMessage({ kind: "mutations", envelopes: Uint8Array.of(0) })), legacy: [] } });
      expect(() => decodeBackboneWorkerRequest(new Uint8Array([BACKBONE_WORKER_WIRE_MAGIC, ...malformedFields]))).toThrow("invalid document backbone message");
    });
  });

  describe("DirectoryClient event page", () => {
    it("preserves canonical response text and rejects a substituted request frontier", async () => {
      const canonical = await sampleCanonicalDirectoryEventPage();
      const text = vi.fn(async () => canonical);
      const request = vi.fn(async (..._args: Parameters<typeof fetchWithTimeout>) => ({ ok: true, status: 200, statusText: "OK", headers: { get: () => "application/json" }, json: async () => { throw new Error("event pages never use response.json"); }, text }));
      const page = await new DirectoryClient("https://hub.test", { request: request as unknown as typeof fetchWithTimeout }).eventPage(3);
      expect(page.canonicalJson).toBe(canonical);
      expect(page.afterSeqExclusive).toBe(3);
      expect(page.throughSeqInclusive).toBe(5);
      expect(text).toHaveBeenCalledTimes(1);
      expect(request.mock.calls[0]?.[0]).toBe("https://hub.test/directory/event-page/v1?after=3");

      const substituted = await sampleCanonicalDirectoryEventPage(4);
      const hostileRequest = vi.fn(async () => ({ ok: true, status: 200, statusText: "OK", headers: { get: () => "application/json" }, json: async () => ({}), text: async () => substituted }));
      await expect(new DirectoryClient("https://hub.test", { request: hostileRequest as unknown as typeof fetchWithTimeout }).eventPage(3)).rejects.toThrow("response frontier mismatch");

      const abort = new AbortController();
      const cancelledRequest = vi.fn(async () => ({ ok: true, status: 200, statusText: "OK", headers: { get: () => "application/json" }, json: async () => ({}), text: async () => { abort.abort(new Error("page owner closed")); return canonical; } }));
      await expect(new DirectoryClient("https://hub.test", { request: cancelledRequest as unknown as typeof fetchWithTimeout }).eventPage(3, { signal: abort.signal })).rejects.toThrow("page owner closed");
    });
  });

  async function sampleCanonicalAdministrationPage(access: "public" | "author", overrides: Record<string, unknown> = {}): Promise<string> {
    const space = access === "public"
      ? { id: "space-1", name: "Public", kind: "studio", visibility: "public", memberCount: 2, documentCount: 1, createdAtMs: 1, updatedAtMs: 2 }
      : { id: "space-1", name: "Authored", kind: "studio", visibility: "private", ownerUserId: "user-a", role: "author", memberCount: 2, documentCount: 0, activeConnections: 0, createdAtMs: 1, updatedAtMs: 2 };
    const documents = access === "public"
      ? [{ documentId: "document-public", artifactKind: "note.document", artifactSchema: "note.document@1", owner: { pluginId: "note", packageId: "note", version: "1", packageHash: "11".repeat(32) }, packSchemaHash: "22".repeat(32) }]
      : [];
    const base = {
      access,
      schema: "semio.directory.space-administration-page.v1" as const,
      sessionBindingSha256: access === "public" ? "0".repeat(64) : "a".repeat(64),
      authorizationGeneration: access === "public" ? 0 : 7,
      spaceId: "space-1",
      space,
    };
    const unsigned = access === "public"
      ? { ...base, documents: { rows: documents } }
      : {
          ...base,
          members: { rows: [
            { userId: "user-a", email: "a@example.invalid", displayName: "A", role: "author", owner: true },
            { userId: "user-b", email: "b@example.invalid", displayName: "B", role: "spectator", owner: false },
          ] },
          documents: { rows: documents },
          invites: { rows: [{ inviteId: "invite-1", role: "spectator", createdAtMs: 20, expiresAtMs: 900, revoked: false, accepted: false }] },
          capabilities: { renameSpace: true, setVisibility: true, deleteSpace: true, upsertMember: true, removeMember: true, createInvite: true, revokeInvite: true },
        };
    const sealed = { ...unsigned, ...overrides };
    const digest = new Uint8Array(await globalThis.crypto.subtle.digest("SHA-256", new TextEncoder().encode(JSON.stringify(sealed))));
    const receiptSha256 = Array.from(digest, (byte) => byte.toString(16).padStart(2, "0")).join("");
    return JSON.stringify({ ...sealed, receiptSha256 });
  }

  function administrationResponse(canonical: string): unknown {
    return { ok: true, status: 200, statusText: "OK", headers: { get: () => "application/json" }, json: async () => { throw new Error("administration pages never use response.json"); }, text: async () => canonical };
  }

  describe("DirectorySpaceAdministrationPageV1", () => {
    it("keeps invites and capabilities author-only and rejects a substituted receipt", async () => {
      const authored = await sampleCanonicalAdministrationPage("author");
      const page = await parseDirectorySpaceAdministrationPageV1(authored);
      if (page.access !== "author") throw new Error("author discriminator did not narrow");
      expect(page.members.rows.map((row) => row.userId)).toEqual(["user-a", "user-b"]);
      expect(page.members.rows[0]!.owner).toBe(true);
      expect(page.capabilities.removeMember).toBe(true);
      expect(page.invites.rows).toHaveLength(1);
      expect(authored).not.toContain("secretDigest");
      expect(authored).not.toContain("selector");

      const publicPage = await parseDirectorySpaceAdministrationPageV1(await sampleCanonicalAdministrationPage("public"));
      if (publicPage.access !== "public") throw new Error("public discriminator did not narrow");
      expect("members" in publicPage).toBe(false);
      expect("invites" in publicPage).toBe(false);
      expect("capabilities" in publicPage).toBe(false);

      await expect(parseDirectorySpaceAdministrationPageV1(authored.replace(/"receiptSha256":"[0-9a-f]{64}"/u, `"receiptSha256":"${"b".repeat(64)}"`))).rejects.toThrow("receipt-mismatch");
      await expect(parseDirectorySpaceAdministrationPageV1(`${authored} `)).rejects.toThrow();
      await expect(parseDirectorySpaceAdministrationPageV1(authored.replace('"spaceId":"space-1"', '"spaceId":"space-2"'))).rejects.toThrow();
      await expect(parseDirectorySpaceAdministrationPageV1(authored.replace('{"access":"author"', '{"actor":"user:secret","access":"author"'))).rejects.toThrow();
    });

    it("fetches the exact canonical bytes and refuses a foreign space or a bad cursor", async () => {
      const canonical = await sampleCanonicalAdministrationPage("author");
      const request = vi.fn(async (..._args: Parameters<typeof fetchWithTimeout>) => administrationResponse(canonical));
      const client = new DirectoryClient("https://hub.test", { request: request as unknown as typeof fetchWithTimeout });
      const fetched = await client.spaceAdministrationPage("space-1");
      expect(fetched.canonicalJson).toBe(canonical);
      expect(request.mock.calls[0]?.[0]).toBe("https://hub.test/directory/spaces/space-1");

      await client.spaceAdministrationPage("space-1", "m.6162.deadbeef");
      expect(request.mock.calls[1]?.[0]).toBe("https://hub.test/directory/spaces/space-1?cursor=m.6162.deadbeef");
      await expect(client.spaceAdministrationPage("space-1", "not a cursor")).rejects.toThrow("invalid cursor");
      await expect(client.spaceAdministrationPage("space-9")).rejects.toThrow("response space mismatch");
    });
  });

  describe("DirectoryClient.stream", () => {
    it("round trips scoped directory worker ownership without flattening scope", () => {
      const scope = { spaceId: "space/a", documentId: "document b" };
      const request: BackboneWorkerRequest = { kind: "directory-scope-open", baseUrl: "http://hub.test", scope, since: 7 };
      const response: BackboneWorkerResponse = { kind: "directory-scope-revoked", scope };
      expect(decodeBackboneWorkerRequest(encodeBackboneWorkerRequest(request))).toEqual(request);
      expect(decodeBackboneWorkerResponse(encodeBackboneWorkerResponse(response))).toEqual(response);
    });

    it("document opening attempt stays outer-wire-owned without widening the browser patch contract", async () => {
      const clientInstanceId = "33333333-3333-4333-8333-333333333333";
      const requests: readonly BackboneWorkerRequest[] = [
        { kind: "open", documentId: "same-document", clientInstanceId, schema: "gis.map", actor: "caller", bindings: [{ kind: "hub", baseUrl: "https://hub.test", spaceId: "space-a" }] },
        { kind: "send", documentId: "same-document", spaceId: "space-a", clientInstanceId, message: { kind: "externalChanged" } },
        { kind: "close", documentId: "same-document", spaceId: "space-a", clientInstanceId },
      ];
      for (const request of requests) expect(decodeBackboneWorkerRequest(encodeBackboneWorkerRequest(request))).toEqual(request);

      const event: BackboneWorkerResponse = { kind: "event", documentId: "same-document", clientInstanceId, scope: { spaceId: "space-a", documentId: "same-document" }, event: { kind: "status", persisted: true, pendingMutations: 0, remote: { kind: "live", peerCount: 1 } } };
      expect(decodeBackboneWorkerResponse(encodeBackboneWorkerResponse(event))).toEqual(event);
      const fixture = JSON.parse(await (await import("node:fs/promises")).readFile(new URL("./🔨️modules/🔌️plugin/🌐️browser-bundle/🩹️patch-handoff/🧫️fixtures/🔣️.json", source.url), "utf8"));
      const offer = parseBrowserActorUiPatchOfferV1(fixture.offer);
      const result = parseBrowserActorUiPatchResultV1(fixture.acknowledged);
      expect(decodeBackboneWorkerResponse(encodeBackboneWorkerResponse({ ...offer, clientInstanceId }))).toEqual({ ...offer, clientInstanceId });
      expect(decodeBackboneWorkerRequest(encodeBackboneWorkerRequest({ ...result, clientInstanceId }))).toEqual({ ...result, clientInstanceId });
      expect(Object.hasOwn(parseBrowserActorUiPatchOfferV1(fixture.offer), "clientInstanceId")).toBe(false);
      expect(Object.hasOwn(parseBrowserActorUiPatchResultV1(fixture.acknowledged), "clientInstanceId")).toBe(false);
      const mounted: BrowserActorUiMountedV1 = {
        kind: "browser-actor-ui-mounted",
        scope: { spaceId: "space-a", documentId: "same-document" },
        clientInstanceId,
        activationGeneration: "41",
        instanceId: 0,
        verifiedSurfaceId: "s.gis.gismap@1/viewer",
        catalogGenerationId: "1".repeat(64),
        componentSha256: "2".repeat(64),
        descriptorSha256: "3".repeat(64),
        browserActorSha256: "4".repeat(64),
        activeCheckpointId: "5".repeat(64),
        descriptorDigestV1: "6".repeat(64),
        frontier: { documentId: "same-document", headEditOrdinal: 7, headEditId: "edit-7", lastCommitSeq: 7, chainHash: new Array(32).fill(7) },
        uiRevision: 7,
      };
      expect(decodeBackboneWorkerResponse(encodeBackboneWorkerResponse(mounted))).toEqual(mounted);
      const mountedUnknown = encodePackValue({ ...mounted, grant: "forbidden" });
      expect(() => decodeBackboneWorkerResponse(new Uint8Array([BACKBONE_WORKER_WIRE_MAGIC, ...mountedUnknown]))).toThrow("invalid mounted UI fields");
      const mountedForeignFrontier = encodePackValue({ ...mounted, frontier: { ...mounted.frontier, documentId: "foreign" } });
      expect(() => decodeBackboneWorkerResponse(new Uint8Array([BACKBONE_WORKER_WIRE_MAGIC, ...mountedForeignFrontier]))).toThrow("invalid mounted UI identity");

      const missing = new Uint8Array([BACKBONE_WORKER_WIRE_MAGIC, ...encodePackValue({ kind: "event", documentId: "same-document", event: event.event })]);
      expect(() => decodeBackboneWorkerResponse(missing)).toThrow("invalid client instance id");
      const malformed = new Uint8Array([BACKBONE_WORKER_WIRE_MAGIC, ...encodePackValue({ kind: "close", documentId: "same-document", clientInstanceId: "not-an-owner" })]);
      expect(() => decodeBackboneWorkerRequest(malformed)).toThrow("invalid client instance id");
    });

    it("withholds creation authority and document coordinates until an exact ready status", () => {
      const requestId = "1".repeat(32);
      const clientInstanceId = "12345678-1234-4123-8123-123456789abc";
      const catalogRequest: BackboneWorkerRequest = { kind: "space-artifact-creation-catalog-open", clientInstanceId, spaceId: "space-a" };
      const catalog: BackboneWorkerResponse = {
        kind: "space-artifact-creation-catalog",
        clientInstanceId,
        spaceId: "space-a",
        catalogGenerationId: "3".repeat(64),
        kinds: [{ kindId: "s.gis.gismap", schema: "s.gis.gismap", dialect: { artifactKind: "s.gis.gismap", standard: "1", subset: "any" }, label: { en: "GIS Map", de: "GIS-Karte" } }],
      };
      const request: BackboneWorkerRequest = { kind: "space-artifact-create", requestId, spaceId: "space-a", kindId: "s.gis.gismap", name: "Shared Map" };
      const ready: BackboneWorkerResponse = {
        kind: "space-artifact-creation-status",
        requestId,
        spaceId: "space-a",
        phase: "ready",
        ready: { documentId: `artifact-${"2".repeat(32)}`, kindId: "s.gis.gismap", artifactSchema: "s.gis.gismap", parentDialect: { artifactKind: "s.gis.gismap", standard: "1", subset: "any" } },
      };
      expect(decodeBackboneWorkerRequest(encodeBackboneWorkerRequest(catalogRequest))).toEqual(catalogRequest);
      for (const phase of ["loading", "ready", "unavailable"] as const) {
        const status: BackboneWorkerResponse = { kind: "space-artifact-creation-catalog-status", clientInstanceId, spaceId: "space-a", phase };
        expect(decodeBackboneWorkerResponse(encodeBackboneWorkerResponse(status))).toEqual(status);
      }
      expect(decodeBackboneWorkerResponse(encodeBackboneWorkerResponse(catalog))).toEqual(catalog);
      expect(decodeBackboneWorkerRequest(encodeBackboneWorkerRequest(request))).toEqual(request);
      expect(decodeBackboneWorkerRequest(encodeBackboneWorkerRequest({ kind: "space-artifact-create-cancel", requestId, spaceId: "space-a" }))).toEqual({ kind: "space-artifact-create-cancel", requestId, spaceId: "space-a" });
      expect(decodeBackboneWorkerResponse(encodeBackboneWorkerResponse(ready))).toEqual(ready);
      const leaked = encodePackValue({ ...ready, phase: "preparing" });
      expect(() => decodeBackboneWorkerResponse(new Uint8Array([BACKBONE_WORKER_WIRE_MAGIC, ...leaked]))).toThrow("invalid fields");
      const forgedCatalog = encodePackValue({ ...catalog, kinds: [{ ...catalog.kinds[0], descriptor: "forbidden" }] });
      expect(() => decodeBackboneWorkerResponse(new Uint8Array([BACKBONE_WORKER_WIRE_MAGIC, ...forgedCatalog]))).toThrow("invalid kind fields");
      const forgedCatalogStatus = encodePackValue({ kind: "space-artifact-creation-catalog-status", clientInstanceId, spaceId: "space-a", phase: "empty" });
      expect(() => decodeBackboneWorkerResponse(new Uint8Array([BACKBONE_WORKER_WIRE_MAGIC, ...forgedCatalogStatus]))).toThrow("invalid owner");
      const mismatched = encodePackValue({ ...ready, ready: { ...ready.ready!, parentDialect: { ...ready.ready!.parentDialect, artifactKind: "s.note" } } });
      expect(() => decodeBackboneWorkerResponse(new Uint8Array([BACKBONE_WORKER_WIRE_MAGIC, ...mismatched]))).toThrow("invalid ready identity");
      const overposted = encodePackValue({ ...request, descriptor: "forbidden" });
      expect(() => decodeBackboneWorkerRequest(new Uint8Array([BACKBONE_WORKER_WIRE_MAGIC, ...overposted]))).toThrow("invalid space artifact creation fields");
    });

    it("keeps inference status scope and validated preview exact across the private worker wire", () => {
      const jobId = "1".repeat(32);
      const proposalHash = "2".repeat(64);
      const scope = { spaceId: "space-a", documentId: "same-document" };
      const response: BackboneWorkerResponse = {
        kind: "inference-port-status",
        operationEpoch: 7,
        scope,
        status: {
          phase: "offered",
          jobId,
          cursor: 3,
          completed: 4,
          total: 4,
          proposalHash,
          preview: { schema: "semio.hub.gis-map-inference-preview/v1", jobId, proposalHash, regionId: `inference-${jobId}`, ring: [[7, 46], [9, 46], [9, 48], [7, 48], [7, 46]] },
          cancelRequested: false,
          code: null,
        },
      };
      expect(decodeBackboneWorkerResponse(encodeBackboneWorkerResponse(response))).toEqual(response);
      const malformedScope = encodeBackboneWorkerResponse(response).map((byte) => byte);
      const decoded = decodePackValue(malformedScope.subarray(1)) as Record<string, unknown>;
      const badScope = encodePackValue({ ...decoded, scope: { spaceId: "space-b", documentId: "same-document", requestedSurface: "forbidden" } });
      expect(() => decodeBackboneWorkerResponse(new Uint8Array([BACKBONE_WORKER_WIRE_MAGIC, ...badScope]))).toThrow("invalid inference scope");
      const badStatus = encodePackValue({ ...decoded, status: { ...(decoded.status as Record<string, unknown>), preview: { ...response.status.preview, ring: [[7, 46], [9, 46], [8, 48], [7, 48], [7, 46]] } } });
      expect(() => decodeBackboneWorkerResponse(new Uint8Array([BACKBONE_WORKER_WIRE_MAGIC, ...badStatus]))).toThrow("invalid-preview");
    });

    it("binds one document scope and treats close 4401 as terminal without reacquiring", async () => {
      vi.useFakeTimers();
      try {
        FakeDirectoryWebSocket.instances = [];
        (globalThis as unknown as { WebSocket: unknown }).WebSocket = FakeDirectoryWebSocket;
        const revoked = vi.fn();
        const handle = testDirectoryClient().streamScoped({ spaceId: "space/a", documentId: "document b" }, 4, () => {}, revoked);
        await Promise.resolve();
        const socket = FakeDirectoryWebSocket.instances[0]!;
        expect(socket.url).toBe("ws://hub.test/directory/spaces/space%2Fa/documents/document%20b/socket/v1?since=4");
        socket.triggerClose(4401);
        await Promise.resolve();
        await vi.advanceTimersByTimeAsync(HUB_RECONNECT_MAX_MS * 2);
        expect(revoked).toHaveBeenCalledTimes(1);
        expect(FakeDirectoryWebSocket.instances).toHaveLength(1);
        handle.close();
      } finally {
        vi.useRealTimers();
      }
    });

    it("replays then goes live with no gap and no duplicate", async () => {
      FakeDirectoryWebSocket.instances = [];
      (globalThis as unknown as { WebSocket: unknown }).WebSocket = FakeDirectoryWebSocket;
      const received: DirectoryStreamMessage[] = [];
      const client = testDirectoryClient();
      const handle = client.stream(0, (message) => received.push(message));
      await Promise.resolve();
      const socket = FakeDirectoryWebSocket.instances[0]!;
      expect(socket.url).toBe("ws://hub.test/directory/socket/v1?since=0");
      expect(socket.protocols).toEqual(["semio.socket.v1", testSocketGrantReceipt.grant]);
      socket.triggerOpen();
      socket.triggerMessage({ kind: "event", event: sampleDirectoryEvent(1) });
      socket.triggerMessage({ kind: "event", event: sampleDirectoryEvent(2) });
      socket.triggerMessage({ kind: "heartbeat", headSeq: 2 });
      expect(received.map((message) => message.kind)).toEqual(["event", "event", "heartbeat"]);
      expect(received).toHaveLength(3);
      handle.close();
    });

    it("reconnects an acknowledged stream only from the last Home-committed frontier", async () => {
      vi.useFakeTimers();
      const randomSpy = vi.spyOn(Math, "random").mockReturnValue(0);
      try {
        FakeDirectoryWebSocket.instances = [];
        (globalThis as unknown as { WebSocket: unknown }).WebSocket = FakeDirectoryWebSocket;
        const handle = testDirectoryClient().streamAcknowledged(3, () => {});
        await Promise.resolve();
        const first = FakeDirectoryWebSocket.instances[0]!;
        first.triggerMessage({ kind: "event", event: sampleDirectoryEvent(99) });
        first.triggerMessage({ kind: "heartbeat", headSeq: 101 });
        first.triggerClose();
        await Promise.resolve();
        await vi.advanceTimersByTimeAsync(HUB_RECONNECT_MIN_MS);
        const second = FakeDirectoryWebSocket.instances[1]!;
        expect(second.url).toBe("ws://hub.test/directory/socket/v1?since=3");
        handle.acknowledge(5);
        expect(() => handle.acknowledge(4)).toThrow("invalid acknowledged frontier");
        second.triggerClose();
        await Promise.resolve();
        await vi.advanceTimersByTimeAsync(HUB_RECONNECT_MIN_MS);
        expect(FakeDirectoryWebSocket.instances[2]!.url).toBe("ws://hub.test/directory/socket/v1?since=5");
        handle.close();
      } finally {
        randomSpy.mockRestore();
        vi.useRealTimers();
      }
    });

    it("reconnects resuming from the last seen seq, with jittered backoff within bounds", async () => {
      vi.useFakeTimers();
      const randomSpy = vi.spyOn(Math, "random");
      try {
        FakeDirectoryWebSocket.instances = [];
        (globalThis as unknown as { WebSocket: unknown }).WebSocket = FakeDirectoryWebSocket;
        randomSpy.mockReturnValue(0); // 🎯️ pins the jitter draw to the lower bound of its range.
        const client = testDirectoryClient();
        const handle = client.stream(0, () => {});
        await Promise.resolve();
        const first = FakeDirectoryWebSocket.instances[0]!;
        first.triggerMessage({ kind: "event", event: sampleDirectoryEvent(7) });
        first.triggerClose();
        await Promise.resolve(); // 🪧️ let the rejection's microtask reach retryWithJitteredBackoff's catch (which schedules the backoff timer) before advancing fake time — advanceTimersByTime does not itself drain microtasks.

        // attempt 1: cap = min(MAX, MIN·2¹) = 2·MIN; random()=0 ⇒ delay lands exactly on the lower
        // bound (MIN) — no reconnect fires a tick earlier, and one fires the instant it's due.
        await vi.advanceTimersByTimeAsync(HUB_RECONNECT_MIN_MS - 1);
        expect(FakeDirectoryWebSocket.instances).toHaveLength(1);
        await vi.advanceTimersByTimeAsync(1);
        expect(FakeDirectoryWebSocket.instances).toHaveLength(2);
        const second = FakeDirectoryWebSocket.instances[1]!;
        expect(second.url).toBe("ws://hub.test/directory/socket/v1?since=7"); // resumes from lastSeq, never the original `since`.

        randomSpy.mockReturnValue(1); // 🎯️ pins the jitter draw to the upper bound of its range.
        second.triggerClose();
        await Promise.resolve();

        // attempt 2: cap = min(MAX, MIN·2²) = 4·MIN; random()=1 ⇒ delay lands exactly on that upper
        // bound — proving the delay grows and stays within [MIN, cap], not a fixed exponential value.
        const attempt2Cap = Math.min(HUB_RECONNECT_MAX_MS, HUB_RECONNECT_MIN_MS * 2 ** 2);
        await vi.advanceTimersByTimeAsync(attempt2Cap - 1);
        expect(FakeDirectoryWebSocket.instances).toHaveLength(2);
        await vi.advanceTimersByTimeAsync(1);
        expect(FakeDirectoryWebSocket.instances).toHaveLength(3);

        handle.close();
      } finally {
        randomSpy.mockRestore();
        vi.useRealTimers();
      }
    });

    it("never throws into the caller on a malformed frame", async () => {
      FakeDirectoryWebSocket.instances = [];
      (globalThis as unknown as { WebSocket: unknown }).WebSocket = FakeDirectoryWebSocket;
      const received: DirectoryStreamMessage[] = [];
      const client = testDirectoryClient();
      const handle = client.stream(0, (message) => received.push(message));
      await Promise.resolve();
      const socket = FakeDirectoryWebSocket.instances[0]!;
      expect(() => socket.onmessage?.({ data: "not json" })).not.toThrow();
      socket.triggerMessage({ kind: "heartbeat", headSeq: 0 });
      expect(received).toHaveLength(1);
      handle.close();
    });

    it("close() stops the reconnect loop — no further socket is ever opened", async () => {
      vi.useFakeTimers();
      try {
        FakeDirectoryWebSocket.instances = [];
        (globalThis as unknown as { WebSocket: unknown }).WebSocket = FakeDirectoryWebSocket;
        const client = testDirectoryClient();
        const handle = client.stream(0, () => {});
        await Promise.resolve();
        const first = FakeDirectoryWebSocket.instances[0]!;
        handle.close();
        first.triggerClose();
        vi.advanceTimersByTime(HUB_RECONNECT_MAX_MS * 2);
        expect(FakeDirectoryWebSocket.instances).toHaveLength(1);
      } finally {
        vi.useRealTimers();
      }
    });

    it("(a) a drop after sustained health resets the backoff — reconnects near HUB_RECONNECT_MIN_MS, not at an escalated delay", async () => {
      vi.useFakeTimers();
      const randomSpy = vi.spyOn(Math, "random");
      try {
        FakeDirectoryWebSocket.instances = [];
        (globalThis as unknown as { WebSocket: unknown }).WebSocket = FakeDirectoryWebSocket;
        randomSpy.mockReturnValue(0); // 🎯️ pins every jittered delay to its lower bound.
        const client = testDirectoryClient();
        const handle = client.stream(0, () => {});
        await Promise.resolve();
        const first = FakeDirectoryWebSocket.instances[0]!;
        first.triggerOpen();

        // 🩺️ let it prove itself healthy, then drop it.
        await vi.advanceTimersByTimeAsync(HUB_HEALTHY_RESET_MS);
        first.triggerClose();
        await Promise.resolve(); // let the resolved connectOnce reach runCycles' next-cycle setup.

        // The next cycle is primed: its first (synthetic) failure is attempt 1 of a FRESH counter —
        // cap = min(MAX, MIN·2¹) = 2·MIN; random()=0 ⇒ delay lands exactly on the lower bound, MIN —
        // never the far larger delay an un-reset counter would have reached by this point.
        await vi.advanceTimersByTimeAsync(HUB_RECONNECT_MIN_MS - 1);
        expect(FakeDirectoryWebSocket.instances).toHaveLength(1);
        await vi.advanceTimersByTimeAsync(1);
        expect(FakeDirectoryWebSocket.instances).toHaveLength(2);

        handle.close();
      } finally {
        randomSpy.mockRestore();
        vi.useRealTimers();
      }
    });

    it("(b) rapid accept-then-drop cycling never crosses the health threshold — backoff keeps escalating, never resets", async () => {
      vi.useFakeTimers();
      const randomSpy = vi.spyOn(Math, "random");
      try {
        FakeDirectoryWebSocket.instances = [];
        (globalThis as unknown as { WebSocket: unknown }).WebSocket = FakeDirectoryWebSocket;
        randomSpy.mockReturnValue(1); // 🎯️ pins every jittered delay to its upper bound (== cap), so a reset shows up unmistakably as a delay dropping back down.
        const client = testDirectoryClient();
        const handle = client.stream(0, () => {});
        await Promise.resolve();

        let instanceCount = 1;
        for (let attempt = 1; attempt <= 3; attempt++) {
          const socket = FakeDirectoryWebSocket.instances[instanceCount - 1]!;
          socket.triggerOpen();
          socket.triggerClose(); // drops instantly — nowhere near HUB_HEALTHY_RESET_MS, so `healthy` stays false.
          await Promise.resolve();
          const cap = Math.min(HUB_RECONNECT_MAX_MS, HUB_RECONNECT_MIN_MS * 2 ** attempt);
          await vi.advanceTimersByTimeAsync(cap - 1);
          expect(FakeDirectoryWebSocket.instances).toHaveLength(instanceCount); // still escalated — a reset would have reconnected far sooner than this growing cap.
          await vi.advanceTimersByTimeAsync(1);
          instanceCount += 1;
          expect(FakeDirectoryWebSocket.instances).toHaveLength(instanceCount);
        }

        handle.close();
      } finally {
        randomSpy.mockRestore();
        vi.useRealTimers();
      }
    });

    it("(c) close() during a healthy-but-not-yet-reset connection cancels promptly, clears the health timer, and never reconnects", async () => {
      vi.useFakeTimers();
      try {
        FakeDirectoryWebSocket.instances = [];
        (globalThis as unknown as { WebSocket: unknown }).WebSocket = FakeDirectoryWebSocket;
        const client = testDirectoryClient();
        const handle = client.stream(0, () => {});
        await Promise.resolve();
        const first = FakeDirectoryWebSocket.instances[0]!;
        first.triggerOpen();
        await vi.advanceTimersByTimeAsync(HUB_HEALTHY_RESET_MS / 2); // health timer armed, not yet fired.
        handle.close();
        first.triggerClose();
        await vi.advanceTimersByTimeAsync(HUB_RECONNECT_MAX_MS * 2);
        expect(FakeDirectoryWebSocket.instances).toHaveLength(1); // no reconnect was ever attempted.
        expect(vi.getTimerCount()).toBe(0); // 🛟️ neither the health timer nor any backoff timer is left pending.
      } finally {
        vi.useRealTimers();
      }
    });
  });

  describe("DirectoryClient http (getJson/postJson timeout + abort)", () => {
    const originalFetch = globalThis.fetch;

    it("a hung server rejects at the timeout instead of hanging the caller forever", async () => {
      vi.useFakeTimers();
      try {
        globalThis.fetch = vi.fn((_url: string, init?: RequestInit) => {
          return new Promise((_resolve, reject) => {
            init?.signal?.addEventListener("abort", () => reject(init.signal!.reason ?? new Error("aborted")));
          });
        }) as unknown as typeof fetch;
        const client = new DirectoryClient("http://hub.test");
        const promise = client.me(); // 🪪️ the identity/boot-path call finding 1 is about.
        let settled = false;
        promise.then(
          () => (settled = true),
          () => (settled = true),
        );
        await vi.advanceTimersByTimeAsync(DIRECTORY_HTTP_TIMEOUT_MS + 1_000);
        expect(settled).toBe(true); // never hangs — this is what lets the boot path's own
        // catch-all ("hub unreachable, staying offline") actually run instead of awaiting forever.
        await expect(promise).rejects.toThrow();
      } finally {
        globalThis.fetch = originalFetch;
        vi.useRealTimers();
      }
    });

    it("an external abort cancels promptly, without ever waiting out the timeout", async () => {
      vi.useFakeTimers();
      try {
        globalThis.fetch = vi.fn((_url: string, init?: RequestInit) => {
          return new Promise((_resolve, reject) => {
            init?.signal?.addEventListener("abort", () => reject(init.signal!.reason ?? new Error("aborted")));
          });
        }) as unknown as typeof fetch;
        const client = new DirectoryClient("http://hub.test");
        const controller = new AbortController();
        const promise = client.spaces({ signal: controller.signal });
        controller.abort(new Error("caller cancelled"));
        // no `vi.advanceTimersByTime*` call at all: if this depended on the timeout timer firing,
        // fake timers would leave it pending forever and this await would hang the test.
        await expect(promise).rejects.toThrow("caller cancelled");
      } finally {
        globalThis.fetch = originalFetch;
        vi.useRealTimers();
      }
    });

    it("still resolves normally when the server answers promptly", async () => {
      globalThis.fetch = vi.fn(async () => ({ ok: true, status: 200, json: async () => [] })) as unknown as typeof fetch;
      try {
        const client = new DirectoryClient("http://hub.test");
        await expect(client.spaces()).resolves.toEqual([]);
      } finally {
        globalThis.fetch = originalFetch;
      }
    });
  });

}
