/** 🔸️ The artifact bootstrap protocol phase of registerTests1. */
import type { buildArtifactBootstrapAssemblerState } from "../🔶️artifact-bootstrap-assembler/🟦️.ts";
import type { ArtifactBootstrapControl, ArtifactBootstrapLimits, ArtifactBootstrapProgress, ServerFrame, TestSource, WireArtifactBootstrap, WireFrontierSummary } from "../🟢️harness/🟦️.ts";

/** 🔹 Registers the artifact bootstrap protocol tests of registerTests1. */
export function registerArtifactBootstrapProtocolTests({ describe, source, it, expect, decodeServerFrame, encodeServerFrame, artifactBootstrapSha256, artifactBootstrapAggregateHash, ArtifactBootstrapAssembler }: Pick<ReturnType<typeof buildArtifactBootstrapAssemblerState>, "describe" | "it" | "expect" | "decodeServerFrame" | "encodeServerFrame" | "artifactBootstrapSha256" | "artifactBootstrapAggregateHash" | "ArtifactBootstrapAssembler"> & { readonly source: TestSource }) {
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
      return JSON.parse(await readFile(join(dirname(fileURLToPath(source.url)), "🧪️tests/🧫️fixtures/🚀️artifact-bootstrap/🔣️.json"), "utf8")) as Fixture;
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

    it("admits a genesis rebootstrap baseline naming no head edit and refuses a later one", () => {
      const control = (ordinal: number, commit: number, head: string) => ({ space_id: "space-1", document_id: "document-1", checkpoint_id: Array(32).fill(1), descriptor_hash: Array(32).fill(2), baseline_frontier: { document_id: "document-1", head_edit_ordinal: ordinal, head_edit_id: head, last_commit_seq: commit, chain_hash: Array(32).fill(0) } });
      for (const admitted of [control(0, 0, ""), control(5, 2, "edit-5")]) expect(decodeServerFrame(encodeServerFrame({ RebootstrapRequired: { control: admitted } }, "command")).frame).toEqual({ RebootstrapRequired: { control: admitted } });
      for (const refused of [control(5, 2, ""), control(0, 1, ""), control(3, 0, "")]) expect(() => decodeServerFrame(encodeServerFrame({ RebootstrapRequired: { control: refused } }, "command"))).toThrow("rebootstrap control identity is invalid");
    });

    it("validates the neutral descriptor and SHA-256 values with AJV and Node crypto", async () => {
      const fixture = await loadFixture();
      const { readFile } = await import("node:fs/promises");
      const { dirname, join } = await import("node:path");
      const { fileURLToPath } = await import("node:url");
      const { createHash } = await import("node:crypto");
      const { default: Ajv } = await import("ajv");
      const root = join(dirname(fileURLToPath(source.url)), "🧪️tests/🧫️fixtures/🚀️artifact-bootstrap");
      
      
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
}
