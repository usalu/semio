type TestSource = { readonly directory: string; readonly url: string };

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: any, source: TestSource): Promise<void> {
  const { ArtifactBootstrapAssembler, artifactBootstrapAggregateHash, artifactBootstrapSha256, decodeClientFrame, decodePresencePeer, decodeServerFrame, encodeClientFrame, encodePresencePeer, encodeServerFrame } = dependencies;
  type ArtifactBootstrapControl = any;
  type ArtifactBootstrapLimits = any;
  type ArtifactBootstrapProgress = any;
  type ServerFrame = any;
  type WireArtifactBootstrap = any;
  type WireFrontierSummary = any;

  const { describe, expect, it } = vitest;

  describe("artifact bootstrap protocol", () => {
    type Fixture = Readonly<{
      schemaVersion: number;
      formatVersion: number;
      artifact: Readonly<{
        descriptorHash: string;
        schema: string;
        kind: string;
        packSchemaHash: string;
        baselineFrontier: Readonly<{ documentId: string; headEditOrdinal: number; headEditId: string; lastCommitSeq: number; chainHash: string }>;
        requiredTailFrontier: Readonly<{ documentId: string; headEditOrdinal: number; headEditId: string; lastCommitSeq: number; chainHash: string }>;
      }>;
      payload: Readonly<{ packHex: string; sprHex: string; packLength: number; sprLength: number; packHash: string; sprHash: string; aggregateHash: string }>;
      chunkByteLengths: readonly number[];
      wire: Readonly<{ inlineWelcomeHex: string; chunkedWelcomeHex: string; chunkHex: readonly string[]; doneHex: string }>;
    }>;

    const fromHex = (hex: string): number[] => Array.from(Uint8Array.from(hex.match(/../gu) ?? [], (byte) => Number.parseInt(byte, 16)));
    const toHex = (bytes: Uint8Array | readonly number[]): string => Array.from(bytes, (byte) => byte.toString(16).padStart(2, "0")).join("");
    const frontier = (value: Fixture["artifact"]["baselineFrontier"]): WireFrontierSummary => ({ document_id: value.documentId, head_edit_ordinal: value.headEditOrdinal, head_edit_id: value.headEditId, last_commit_seq: value.lastCommitSeq, chain_hash: fromHex(value.chainHash) });

    async function loadFixture(): Promise<Fixture> {
      const { readFile } = await import("node:fs/promises");
      const { dirname, join } = await import("node:path");
      const { fileURLToPath } = await import("node:url");
      return JSON.parse(await readFile(join(dirname(fileURLToPath(source.url)), "🧫️fixtures/🚀️artifact-bootstrap/🔣️.json"), "utf8")) as Fixture;
    }

    function bootstrapFromFixture(fixture: Fixture, inline: boolean): WireArtifactBootstrap {
      return {
        format_version: fixture.formatVersion,
        descriptor_hash: fromHex(fixture.artifact.descriptorHash),
        artifact_schema: fixture.artifact.schema,
        artifact_kind: fixture.artifact.kind,
        pack_schema_hash: fromHex(fixture.artifact.packSchemaHash),
        baseline_frontier: frontier(fixture.artifact.baselineFrontier),
        pack_hash: fromHex(fixture.payload.packHash),
        spr_hash: fromHex(fixture.payload.sprHash),
        pack_length: fixture.payload.packLength,
        spr_length: fixture.payload.sprLength,
        chunk_count: inline ? 0 : fixture.chunkByteLengths.length,
        aggregate_hash: fromHex(fixture.payload.aggregateHash),
        required_tail_frontier: frontier(fixture.artifact.requiredTailFrontier),
        inline: inline ? { pack: fromHex(fixture.payload.packHex), spr: fromHex(fixture.payload.sprHex) } : null,
      };
    }

    function welcome(bootstrap: WireArtifactBootstrap): ServerFrame {
      return { Welcome: { session_id: "session-bootstrap-1", resume_token: "resume-bootstrap-1", server_frontier: bootstrap.required_tail_frontier, bootstrap: { ArtifactBootstrap: bootstrap } } };
    }

    function chunkFrames(fixture: Fixture, bootstrap: WireArtifactBootstrap): Extract<ServerFrame, { readonly ArtifactBootstrapChunk: unknown }>[] {
      const content = [...fromHex(fixture.payload.packHex), ...fromHex(fixture.payload.sprHex)];
      let offset = 0;
      return fixture.chunkByteLengths.map((length, index) => {
        const bytes = content.slice(offset, offset + length);
        offset += length;
        return { ArtifactBootstrapChunk: { descriptor_hash: bootstrap.descriptor_hash, index, bytes } };
      });
    }

    function control(cancelAt = Number.POSITIVE_INFINITY, now = 0) {
      const progress: ArtifactBootstrapProgress[] = [];
      return {
        progress,
        cancelled: false,
        now,
        isCancelled() { return this.cancelled || progress.length >= cancelAt; },
        nowMs() { return this.now; },
        onProgress(value: ArtifactBootstrapProgress) { progress.push(value); },
      };
    }

    it("validates the neutral descriptor and SHA-256 values with AJV and Node crypto", async () => {
      const fixture = await loadFixture();
      const { readFile } = await import("node:fs/promises");
      const { dirname, join } = await import("node:path");
      const { fileURLToPath } = await import("node:url");
      const { createHash } = await import("node:crypto");
      const { default: Ajv } = await import("ajv");
      const root = join(dirname(fileURLToPath(source.url)), "🧫️fixtures/🚀️artifact-bootstrap");
      const schema = JSON.parse(await readFile(join(dirname(fileURLToPath(source.url)), "🧬️schema/🔣️.json"), "utf8"));
      expect(new Ajv({ strict: true }).addSchema(schema).getSchema(`${schema.$id}#/$defs/ArtifactBootstrapFixture`)!(fixture)).toBe(true);
      const pack = new Uint8Array(fromHex(fixture.payload.packHex));
      const spr = new Uint8Array(fromHex(fixture.payload.sprHex));
      expect(createHash("sha256").update(pack).digest("hex")).toBe(fixture.payload.packHash);
      expect(createHash("sha256").update(spr).digest("hex")).toBe(fixture.payload.sprHash);
      expect(createHash("sha256").update(pack).update(spr).digest("hex")).toBe(fixture.payload.aggregateHash);
      expect(toHex(await artifactBootstrapSha256(pack))).toBe(fixture.payload.packHash);
      expect(toHex(await artifactBootstrapAggregateHash(pack, spr))).toBe(fixture.payload.aggregateHash);
    });

    it("matches canonical inline and chunked frame bytes", async () => {
      const fixture = await loadFixture();
      const inline = bootstrapFromFixture(fixture, true);
      const chunked = bootstrapFromFixture(fixture, false);
      const chunks = chunkFrames(fixture, chunked);
      const done: ServerFrame = { ArtifactBootstrapDone: { descriptor_hash: chunked.descriptor_hash, chunk_count: chunked.chunk_count } };
      expect(toHex(encodeServerFrame(welcome(inline), "command"))).toBe(fixture.wire.inlineWelcomeHex);
      expect(toHex(encodeServerFrame(welcome(chunked), "command"))).toBe(fixture.wire.chunkedWelcomeHex);
      expect(chunks.map((frame) => toHex(encodeServerFrame(frame, "command")))).toEqual(fixture.wire.chunkHex);
      expect(toHex(encodeServerFrame(done, "command"))).toBe(fixture.wire.doneHex);
      for (const hex of [fixture.wire.inlineWelcomeHex, fixture.wire.chunkedWelcomeHex, ...fixture.wire.chunkHex, fixture.wire.doneHex]) {
        const bytes = new Uint8Array(fromHex(hex));
        const decoded = decodeServerFrame(bytes);
        expect(encodeServerFrame(decoded.frame, decoded.lane)).toEqual(bytes);
      }
    });

    it("rejects malformed version, descriptor, order, completeness, size, and hashes atomically", async () => {
      const fixture = await loadFixture();
      const valid = bootstrapFromFixture(fixture, false);
      const frames = chunkFrames(fixture, valid);
      const limits: ArtifactBootstrapLimits = { maxTotalBytes: 64, maxChunks: 4, maxChunkBytes: 12 };
      const expected = valid.descriptor_hash;
      const ctl = control();
      expect(() => new ArtifactBootstrapAssembler({ ...valid, format_version: 2 }, expected, limits, 100, ctl)).toThrow(/version/u);
      expect(() => new ArtifactBootstrapAssembler(valid, Array(32).fill(9), limits, 100, ctl)).toThrow(/descriptor/u);
      expect(() => new ArtifactBootstrapAssembler({ ...valid, pack_length: 65 }, expected, limits, 100, ctl)).toThrow(/bytes/u);
      expect(() => new ArtifactBootstrapAssembler({ ...valid, required_tail_frontier: { ...valid.required_tail_frontier, head_edit_ordinal: 6 } }, expected, limits, 100, ctl)).toThrow(/frontier/u);
      const expired = control(Number.POSITIVE_INFINITY, 100);
      expect(() => new ArtifactBootstrapAssembler(valid, expected, limits, 100, expired)).toThrow(/deadline/u);
      for (const indices of [[1], [0, 0]]) {
        const assembler = new ArtifactBootstrapAssembler(valid, expected, limits, 100, control());
        expect(() => { for (const index of indices) assembler.push(frames[index]!.ArtifactBootstrapChunk, control()); }).toThrow(/index/u);
        expect(assembler.retainedBytes).toBe(0);
      }
      const missing = new ArtifactBootstrapAssembler(valid, expected, limits, 100, control());
      missing.push(frames[0]!.ArtifactBootstrapChunk, control());
      await expect(missing.finish({ descriptor_hash: expected, chunk_count: valid.chunk_count }, control())).rejects.toThrow(/complete/u);
      expect(missing.retainedBytes).toBe(0);
      const invalidByte = new ArtifactBootstrapAssembler(valid, expected, limits, 100, control());
      expect(() => invalidByte.push({ descriptor_hash: expected, index: 0, bytes: [256] }, control())).toThrow(/byte/u);
      expect(invalidByte.retainedBytes).toBe(0);
      const inlineWithInvalidByte = bootstrapFromFixture(fixture, true);
      expect(() => new ArtifactBootstrapAssembler({ ...inlineWithInvalidByte, inline: { pack: [256, ...inlineWithInvalidByte.inline!.pack.slice(1)], spr: inlineWithInvalidByte.inline!.spr } }, expected, limits, 100, control())).toThrow(/byte/u);
      const oversizedChunk = new ArtifactBootstrapAssembler(valid, expected, limits, 100, control());
      expect(() => oversizedChunk.push({ descriptor_hash: expected, index: 0, bytes: Array(13).fill(1) }, control())).toThrow(/chunk/u);
      expect(oversizedChunk.retainedBytes).toBe(0);
      const wrongChunkDescriptor = new ArtifactBootstrapAssembler(valid, expected, limits, 100, control());
      expect(() => wrongChunkDescriptor.push({ descriptor_hash: Array(32).fill(9), index: 0, bytes: [1] }, control())).toThrow(/descriptor/u);
      expect(wrongChunkDescriptor.retainedBytes).toBe(0);
      for (const key of ["pack_hash", "spr_hash", "aggregate_hash"] as const) {
        const changed = { ...valid, [key]: Array(32).fill(0xaa) };
        const assembler = new ArtifactBootstrapAssembler(changed, expected, limits, 100, control());
        for (const frame of frames) assembler.push(frame.ArtifactBootstrapChunk, control());
        await expect(assembler.finish({ descriptor_hash: expected, chunk_count: valid.chunk_count }, control())).rejects.toThrow(/hash/u);
        expect(assembler.retainedBytes).toBe(0);
      }
      expect(() => decodeServerFrame(encodeServerFrame(welcome({ ...valid, format_version: 2 }), "command"))).toThrow(/version/u);
    });

    it("cancels at chunk N with monotonic progress and permits a fresh transfer", async () => {
      const fixture = await loadFixture();
      const bootstrap = bootstrapFromFixture(fixture, false);
      const frames = chunkFrames(fixture, bootstrap);
      const limits: ArtifactBootstrapLimits = { maxTotalBytes: 64, maxChunks: 4, maxChunkBytes: 12 };
      const cancelled = control(2);
      const first = new ArtifactBootstrapAssembler(bootstrap, bootstrap.descriptor_hash, limits, 100, cancelled);
      first.push(frames[0]!.ArtifactBootstrapChunk, cancelled);
      expect(() => first.push(frames[1]!.ArtifactBootstrapChunk, cancelled)).toThrow(/cancel/u);
      expect(first.retainedBytes).toBe(0);
      expect(first.progress.receivedBytes).toBe(12);
      expect(cancelled.progress.map((value) => value.receivedBytes)).toEqual([0, 12]);
      let checks = 0;
      let completions = 0;
      const lateControl: ArtifactBootstrapControl = { isCancelled: () => ++checks >= 6, nowMs: () => 0, onProgress: () => undefined };
      const late = new ArtifactBootstrapAssembler(bootstrap, bootstrap.descriptor_hash, limits, 100, lateControl);
      for (const frame of frames) late.push(frame.ArtifactBootstrapChunk, lateControl);
      await expect(late.finish({ descriptor_hash: bootstrap.descriptor_hash, chunk_count: bootstrap.chunk_count }, lateControl).then((pair) => { completions += 1; return pair; })).rejects.toThrow(/cancel/u);
      expect(completions).toBe(0);
      expect(late.retainedBytes).toBe(0);
      expect(late.progress.receivedBytes).toBe(33);
      const resumedControl = control();
      const resumed = new ArtifactBootstrapAssembler(bootstrap, bootstrap.descriptor_hash, limits, 100, resumedControl);
      for (const frame of frames) resumed.push(frame.ArtifactBootstrapChunk, resumedControl);
      const pair = await resumed.finish({ descriptor_hash: bootstrap.descriptor_hash, chunk_count: bootstrap.chunk_count }, resumedControl);
      expect(toHex(pair.pack)).toBe(fixture.payload.packHex);
      expect(toHex(pair.spr)).toBe(fixture.payload.sprHex);
      expect(resumed.retainedBytes).toBe(0);
      expect(resumed.progress.receivedBytes).toBe(33);
      const values = resumedControl.progress.map((value) => value.receivedBytes);
      expect(values).toEqual([...values].sort((left, right) => left - right));
      expect(values.at(-1)).toBe(fixture.payload.packLength + fixture.payload.sprLength);
    });
  });

  describe("wire fixtures", () => {
      // 🎬️ Shared fixtures: the exact same bytes `store/sync/rs/lib.rs`'s
      // `wire_fixtures_stay_byte_identical_across_rust_and_ts` test generates and verifies Rust-side
      // (19 fixtures, one per `ClientFrame`/`ServerFrame` variant plus a `Bootstrap`/`ApplyOutcome`
      // sub-variant each — see that test's doc). Decoding them here, then re-encoding the decoded
      // value and diffing against the original bytes, proves the TS codec agrees with
      // `protocol_wire`'s Rust codec byte-for-byte, not just shape-wise. `diff.payload`/
      // `inverse.payload` are opaque `DemoOperation::encode_op()` bytes (W5) — this test only checks
      // they're non-empty and format-tagged (`op_rt::OP_BINARY_FORMAT = 1`), not their semantic
      // content (decoding a real op needs `DslVariants`, which this TS-only fallback has no twin of).
      it("decodes the Rust-generated binary wire fixtures byte-identically", async () => {
        const { readFileSync } = await import("node:fs");
        const { fileURLToPath } = await import("node:url");
        const { dirname, join } = await import("node:path");
        // 📦️ Written by `wire_fixtures_stay_byte_identical_across_rust_and_ts` in
        // `📡️wire/🦀️.rs`, which resolves them as `CARGO_MANIFEST_DIR/../../🧫️fixtures/wire`
        // — i.e. beside the os-kernel crate, not under the sync module. The old path here pointed at a
        // pre-restructure location that no longer exists, so this cross-language byte-identity check had
        // been silently ENOENT-ing instead of comparing anything.
        const fixturesDir = join(dirname(fileURLToPath(source.url)), "🧫️fixtures/📡️wire");
  
        function loadClient(name: string) {
          const bytes = new Uint8Array(readFileSync(join(fixturesDir, name)));
          const decoded = decodeClientFrame(bytes);
          expect(encodeClientFrame(decoded.frame, decoded.lane)).toEqual(bytes);
          return decoded;
        }
        function loadServer(name: string) {
          const bytes = new Uint8Array(readFileSync(join(fixturesDir, name)));
          const decoded = decodeServerFrame(bytes);
          expect(encodeServerFrame(decoded.frame, decoded.lane)).toEqual(bytes);
          return decoded;
        }
        function assertOpBinaryPayload(payload: readonly number[]) {
          expect(payload.length).toBeGreaterThan(0);
          expect(payload[0]).toBe(1); // dsl::op_rt::OP_BINARY_FORMAT
        }
  
        const legacyHello = new Uint8Array(readFileSync(join(fixturesDir, "🚫️legacy-client-hello-rejected", "💾️.bin")));
        expect(() => decodeClientFrame(legacyHello)).toThrow(/unknown tag 0/u);
  
        const commands = loadClient("🕹️client-commands/💾️.bin");
        if (typeof commands.frame === "string" || !("Commands" in commands.frame)) throw new Error("expected a Commands frame");
        expect(commands.frame.Commands.envelopes).toHaveLength(1);
        assertOpBinaryPayload(commands.frame.Commands.envelopes[0]?.diff.payload ?? []);
  
        const frontierAdvertise = loadClient("🚩️client-frontier-advertise/💾️.bin");
        if (typeof frontierAdvertise.frame === "string" || !("FrontierAdvertise" in frontierAdvertise.frame)) throw new Error("expected a FrontierAdvertise frame");
  
        const previewPublish = loadClient("📣️client-preview-publish/💾️.bin");
        if (typeof previewPublish.frame === "string" || !("PreviewPublish" in previewPublish.frame)) throw new Error("expected a PreviewPublish frame");
        expect(previewPublish.frame.PreviewPublish.key).toBe("cursor");
  
        const presence = loadClient("🙋️client-presence/💾️.bin");
        if (typeof presence.frame === "string" || !("Presence" in presence.frame)) throw new Error("expected a Presence frame");
        // 👥️ The fixture's `peer` bytes ARE a real `encode_presence_peer` blob now (Rust writes
        // `sample_presence_peer_with_interaction()`), so this decodes them with the TS twin and checks
        // the fields survive the crossing — the strongest form of this assertion, and the one the old
        // `JSON.parse` could never make. It read the blob as JSON and would have failed the moment the
        // fixture became real; it never did, because the fixture path above pointed at a directory that
        // no longer existed.
        // 👥️ A REAL `encode_presence_peer` blob (Rust writes `sample_presence_peer_with_interaction()`,
        // whose own doc says these fixtures exist "so the TS vitest twin exercises every `PresencePeer`
        // v3 flag bit (§C7.1) with a realistic payload"). Decoding it here proves the scalar fields,
        // the bit-5 interaction section, the color/surface/views/ui fields all cross the language
        // boundary, and re-encoding proves it byte-for-byte.
        const peer = decodePresencePeer(new Uint8Array(presence.frame.Presence.peer), [0]);
        expect(peer.actor).toBe("actor-1");
        expect(peer.label).toBe("Ada");
        expect(peer.userId).toBe("user-9");
        expect(peer.role).toBe("owner");
        expect(peer.connectedAtMs).toBe(1_700_000_000_000);
        expect(peer.color).toBe(5);
        expect(peer.surface).toBe("s.space.home@1/*#editor");
        expect(peer.views).toHaveLength(2);
        expect(peer.views[0]).toEqual({ windowId: "w1", space: "world", kind: { kind: "orbit", position: [1, 2, 3], target: [0, 0, 0], up: [0, 1, 0], fov: 45 }, size: [1024, 768], pointer: [0.5, 0.5, 0.5] });
        expect(peer.views[1]).toEqual({ windowId: "w2", space: "canvas", kind: { kind: "canvas", x: 12.5, y: -4, zoom: 1 }, size: [800, 600], pointer: undefined });
        expect(peer.ui).toEqual({ hoveredPath: "row[2]#t1", focusedPath: undefined, pressedPath: undefined });
        expect(peer.interaction?.app_id).toBe("space");
        expect(peer.interaction?.domains).toEqual([
          { domain: "outline", granularity: "task", selected: ["t1", "t2"], hovered: [] },
          { domain: "board", granularity: "card", selected: [], hovered: ["c1"] },
          { domain: "canvas", granularity: "node", selected: ["n9"], hovered: ["n9", "n10"] },
        ]);
        expect(encodePresencePeer(peer)).toEqual(Array.from(new Uint8Array(presence.frame.Presence.peer)));
  
        const creditGrant = loadClient("🎟️client-credit-grant/💾️.bin");
        if (typeof creditGrant.frame === "string" || !("CreditGrant" in creditGrant.frame)) throw new Error("expected a CreditGrant frame");
        expect(creditGrant.frame.CreditGrant.n).toBe(16);
  
        const bye = loadClient("👋️client-bye/💾️.bin");
        expect(bye.frame).toBe("Bye");
  
        const welcomeTail = loadServer("🔗️server-welcome-tail/💾️.bin");
        if (typeof welcomeTail.frame === "string" || !("Welcome" in welcomeTail.frame)) throw new Error("expected a Welcome frame");
        expect(welcomeTail.frame.Welcome.resume_token).toBe("resume-1");
        expect(welcomeTail.frame.Welcome.bootstrap).toBe("Tail");
  
        const welcomeSnapshot = loadServer("📸️server-welcome-snapshot-inline/💾️.bin");
        if (typeof welcomeSnapshot.frame === "string" || !("Welcome" in welcomeSnapshot.frame)) throw new Error("expected a Welcome frame");
        if (welcomeSnapshot.frame.Welcome.bootstrap === "None" || welcomeSnapshot.frame.Welcome.bootstrap === "Tail" || !("Snapshot" in welcomeSnapshot.frame.Welcome.bootstrap)) throw new Error("expected a Snapshot bootstrap");
        expect(welcomeSnapshot.frame.Welcome.bootstrap.Snapshot.inline).toEqual([9, 9, 9]);
  
        const snapshotChunk = loadServer("🧩️server-snapshot-chunk/💾️.bin");
        if (typeof snapshotChunk.frame === "string" || !("SnapshotChunk" in snapshotChunk.frame)) throw new Error("expected a SnapshotChunk frame");
        expect(snapshotChunk.frame.SnapshotChunk.bytes).toEqual([1, 2, 3, 4]);
  
        const snapshotDone = loadServer("🏁️server-snapshot-done/💾️.bin");
        if (typeof snapshotDone.frame === "string" || !("SnapshotDone" in snapshotDone.frame)) throw new Error("expected a SnapshotDone frame");
        expect(snapshotDone.frame.SnapshotDone.seq_count).toBe(4);
  
        const serverCommands = loadServer("🎮️server-commands/💾️.bin");
        if (typeof serverCommands.frame === "string" || !("Commands" in serverCommands.frame)) throw new Error("expected a Commands frame");
        expect(serverCommands.frame.Commands.envelopes).toHaveLength(1);
  
        const ackAccepted = loadServer("✅️server-ack-accepted/💾️.bin");
        if (typeof ackAccepted.frame === "string" || !("Ack" in ackAccepted.frame)) throw new Error("expected an Ack frame");
        expect(ackAccepted.frame.Ack.batch_id).toBe(1);
        expect(ackAccepted.frame.Ack.stages).toHaveLength(3);
  
        const ackTransformed = loadServer("🔀️server-ack-transformed/💾️.bin");
        if (typeof ackTransformed.frame === "string" || !("Ack" in ackTransformed.frame)) throw new Error("expected an Ack frame");
        expect(ackTransformed.frame.Ack.batch_id).toBe(2);
  
        const ackRejected = loadServer("⛔️server-ack-rejected/💾️.bin");
        if (typeof ackRejected.frame === "string" || !("Ack" in ackRejected.frame)) throw new Error("expected an Ack frame");
        expect(ackRejected.frame.Ack.batch_id).toBe(3);
        const rejectedStage = ackRejected.frame.Ack.stages.find((stage) => typeof stage !== "string" && "Applied" in stage);
        if (typeof rejectedStage === "string" || rejectedStage === undefined || !("Applied" in rejectedStage) || typeof rejectedStage.Applied.outcome === "string" || !("Rejected" in rejectedStage.Applied.outcome)) throw new Error("expected a rejected apply outcome");
        expect(rejectedStage.Applied.outcome.Rejected.messages).toEqual([1, 2, 3]);
  
        const preview = loadServer("👁️server-preview/💾️.bin");
        if (typeof preview.frame === "string" || !("Preview" in preview.frame)) throw new Error("expected a Preview frame");
        expect(preview.frame.Preview.key).toBe("cursor");
  
        const serverPresence = loadServer("👥️server-presence/💾️.bin");
        if (typeof serverPresence.frame === "string" || !("Presence" in serverPresence.frame)) throw new Error("expected a Presence frame");
        // 👥️ Two peers, deliberately mixed by the Rust fixture: a plain JSON blob and a real
        // `encode_presence_peer` payload — so this asserts the frame carries opaque per-peer bytes
        // through untouched, and that the real one still decodes into the same peer as above.
        expect(serverPresence.frame.Presence.peers).toHaveLength(2);
        expect(JSON.parse(new TextDecoder().decode(new Uint8Array(serverPresence.frame.Presence.peers[0]!)))).toEqual({ id: "a" });
        expect(decodePresencePeer(new Uint8Array(serverPresence.frame.Presence.peers[1]!), [0])).toEqual(peer);
  
        const creditGrantServer = loadServer("🎫️server-credit-grant/💾️.bin");
        if (typeof creditGrantServer.frame === "string" || !("CreditGrant" in creditGrantServer.frame)) throw new Error("expected a CreditGrant frame");
        expect(creditGrantServer.frame.CreditGrant.n).toBe(32);
  
        const error = loadServer("🚨️server-error/💾️.bin");
        if (typeof error.frame === "string" || !("Error" in error.frame)) throw new Error("expected an Error frame");
        expect(error.frame.Error.code).toBe("rejected");
  
        const session = loadServer("🪪️server-session/💾️.bin");
        if (typeof session.frame === "string" || !("Session" in session.frame)) throw new Error("expected a Session frame");
        expect(session.frame.Session.actor).toBe("actor-1");
        expect(session.frame.Session.color).toBe(5);
      });
  });

}
