#!/usr/bin/env bun
/** 🖥️ `semio-framework-replication` task router: `bun ./📜️script.ts test [quick|long|exhaustive] [args…]`. */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { BundleScript, ScriptRouter, resolveTestLevel, runBundleScriptMain, runCargo, runExactCargoLaws } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runCargo(["test", "-p", "semio-framework-replication", ...rest], this.repoRoot);
  }
}

class BuildScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runCargo(["build", "-p", "semio-framework-replication", ...segments], this.repoRoot);
  }
}

class SourceTestScript extends BundleScript {
  async run(): Promise<void> {
    await import("../../../🌱️value/🗂️ordered/🧫️fixtures/📜️script.ts");
  }
}

class LocalInteractionSourceTestScript extends BundleScript {
  async run(): Promise<void> {
    await import("../../📡️wire/🏠️local-interaction/🧫️fixtures/📜️script.ts");
  }
}

class LocalInteractionNativeTestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runCargo(["test", "-p", "semio-framework-replication", "--lib", "local_interaction_", ...rest], this.repoRoot);
  }
}

/** 🔬️ Independent test-only SPR grammar shared by framing and retained-owner neutral laws. */
export function inspectRetainedSprNeutral(input: Buffer, checksum: (bytes: Uint8Array) => number, hash: (bytes: Buffer) => Buffer, limits = { fileBytes: 67108864, frameBodyBytes: 1048576, records: 8192 }): { end: number; sequence: number; frames: number; tail: number } {
  const fail = (reason: string): never => { throw new Error(reason); };
  if (input.length > limits.fileBytes) fail("capacity");
  if (input.length < 32 || !input.subarray(0, 8).equals(Buffer.from([137,83,80,82,13,10,26,10])) || input.readUInt16LE(8) !== 1 || input.readUInt16LE(10) !== 0
    || input.readUInt32LE(12) !== 1 || input.subarray(24, 32).some(byte => byte !== 0) || checksum(input.subarray(0, 20)) !== input.readUInt32LE(20)) fail("header");
  let cursor = 32; let committed = 32; let sequence = 0; let lastOffset = 0; let frames = 0; let committedFrames = 0;
  let pendingBytes = 0; const pending: Buffer[] = []; let chain = hash(input.subarray(0, 32));
  while (cursor < input.length) {
    const start = cursor; let length = 0n; let complete = false;
    for (let index = 0; index < 10; index++) {
      if (cursor === input.length) break;
      const byte = input[cursor++]!; if (index === 9 && byte > 1) fail("frame");
      length |= BigInt(byte & 127) << BigInt(7 * index);
      if (byte < 128) { if (index && byte === 0) fail("frame"); complete = true; break; }
    }
    if (!complete) break;
    if (length < 2n) fail("frame"); if (length > BigInt(limits.frameBodyBytes)) fail("capacity");
    const bodyStart = cursor; const bodyEnd = cursor + Number(length); const end = bodyEnd + 8;
    if (end > input.length) break;
    if (checksum(input.subarray(bodyStart, bodyEnd)) !== input.readUInt32LE(bodyEnd) || input.readUInt32LE(bodyEnd + 4) !== end - start) fail("frame");
    if (input[bodyStart] === 12 && input[bodyStart + 1] !== 2) fail("commit");
    const flags = input[bodyStart + 1]!; if ((flags & ~31) !== 0 || Boolean(flags & 1) !== Boolean(flags & 28)) fail("frame");
    if (input[bodyStart + 1]! & 1) {
      let raw = bodyStart + 2; let complete = false;
      for (let index = 0; index < 10; index++) {
        if (raw === bodyEnd) fail("frame");
        const byte = input[raw++]!; if (index === 9 && byte > 1) fail("frame");
        if (byte < 128) { if (index && byte === 0) fail("frame"); complete = true; break; }
      }
      if (!complete) fail("frame");
    }
    frames++; if (frames > limits.records) fail("capacity");
    if (input[bodyStart] === 12) {
      const payload = input.subarray(bodyStart + 2, bodyEnd);
      if (input[bodyStart + 1] !== 2 || payload.length !== 64 || end - start !== 75) fail("commit");
      const nextChain = hash(Buffer.concat([chain, ...pending]));
      if (payload.readBigUInt64LE(0) !== BigInt(sequence + 1) || payload.readBigUInt64LE(8) !== BigInt(lastOffset)
        || payload.readBigUInt64LE(16) !== BigInt(pendingBytes) || payload.readUInt32LE(24) !== pending.length
        || payload.subarray(28, 32).some(byte => byte !== 0) || !payload.subarray(32, 64).equals(nextChain)) fail("commit");
      committed = end; sequence++; lastOffset = start; committedFrames = frames; chain = nextChain; pending.length = 0; pendingBytes = 0;
    } else { pending.push(hash(input.subarray(start, end))); pendingBytes += end - start; }
    cursor = end;
  }
  return { end: committed, sequence, frames: committedFrames, tail: input.length - committed };
}

export class RetainedVerificationScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.some(segment => segment !== "--oracle-only")) throw new Error("retained-verification-check accepts only --oracle-only");
    const { default: assert } = await import("node:assert/strict");
    const { default: Ajv } = await import("ajv/dist/2020.js");
    const { default: crc } = await import("crc-32/crc32c.js");
    const { inflateRawSync } = await import("node:zlib");
    const leb = await import("@webassemblyjs/leb128");
    const { blake3Hex } = await import("../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts");
    const owner = join(this.root, "../../📐️format/🔎️verification");
    const fixture = JSON.parse(readFileSync(join(owner, "🧫️fixture/🔣️.json"), "utf8"));
    const schema = JSON.parse(readFileSync(join(owner, "🧬️schema/🔣️.json"), "utf8"));
    const ajv = new Ajv({ strict: true }); const validate = ajv.compile(schema);
    assert(validate(fixture), ajv.errorsText(validate.errors));
    const checksum = (bytes: Uint8Array): number => crc.buf(bytes) >>> 0;
    const hash = (bytes: Uint8Array): Buffer => Buffer.from(blake3Hex(bytes), "hex");
    assert.equal(checksum(Buffer.from("123456789")), 0xe3069283);
    assert.equal(blake3Hex(Buffer.from("abc")), "6437b3ac38465133ffb63b75273a8db548c558465d79db03fd359c6cd5bd9d85");
    const header = Buffer.alloc(32); Buffer.from([137,83,80,82,13,10,26,10]).copy(header);
    header.writeUInt16LE(1, 8); header.writeUInt32LE(1, 12); header.writeUInt32LE(1, 16);
    header.writeUInt32LE(checksum(header.subarray(0, 20)), 20);
    assert.equal(header.toString("hex"), fixture.headerHex);
    const frame = (kind: number, flags: number, payload: Buffer): Buffer => {
      const body = Buffer.concat([Buffer.from([kind, flags]), payload]);
      const size = Buffer.from(leb.encodeU32(body.length)); const tail = Buffer.alloc(8);
      tail.writeUInt32LE(checksum(body)); tail.writeUInt32LE(size.length + body.length + 8, 4);
      return Buffer.concat([size, body, tail]);
    };
    const parts = [header]; let chain = hash(header); let offset = 32; let previous = 0; let recovered = 0;
    for (const commit of fixture.commits) {
      const records = commit.records.map((record: { kind: number; flags: number; payloadHex: string }) => frame(record.kind, record.flags, Buffer.from(record.payloadHex, "hex")));
      const covered = records.reduce((count: number, record: Buffer) => count + record.length, 0);
      assert.equal(covered, commit.coveredBytes); assert.equal(offset + covered, commit.offset); assert.equal(previous, commit.previousOffset);
      chain = hash(Buffer.concat([chain, ...records.map(hash)]));
      const payload = Buffer.alloc(64); payload.writeBigUInt64LE(BigInt(commit.sequence)); payload.writeBigUInt64LE(BigInt(previous), 8);
      payload.writeBigUInt64LE(BigInt(covered), 16); payload.writeUInt32LE(records.length, 24); chain.copy(payload, 32);
      const encoded = frame(12, 2, payload); assert.equal(encoded.length, 75);
      parts.push(...records, encoded); offset += covered + encoded.length; recovered += records.length + 1;
      assert.equal(offset, commit.end); assert.equal(recovered, commit.recoveredFrames); previous = commit.offset;
    }
    const bytes = Buffer.concat(parts);
    const inspect = (input: Buffer, limits = fixture.limits) => inspectRetainedSprNeutral(input, checksum, hash, limits);
    assert.deepEqual(inspect(bytes), { end: 255, sequence: 2, frames: 5, tail: 0 });
    for (const cut of fixture.resume.cuts) {
      const prior = inspect(bytes.subarray(0, cut));
      const last = fixture.commits.filter((row: { end: number }) => row.end <= cut).at(-1);
      const prefix = bytes.subarray(0, prior.end);
      const priorChain = last ? prefix.subarray(last.offset + 35, last.offset + 67) : hash(header);
      const record = fixture.resume.record;
      const encoded = frame(record.kind, record.flags, Buffer.from(record.payloadHex, "hex"));
      const payload = Buffer.alloc(64);
      payload.writeBigUInt64LE(BigInt(prior.sequence + 1)); payload.writeBigUInt64LE(BigInt(last?.offset ?? 0), 8);
      payload.writeBigUInt64LE(BigInt(encoded.length), 16); payload.writeUInt32LE(1, 24);
      hash(Buffer.concat([priorChain, hash(encoded)])).copy(payload, 32);
      const resumed = Buffer.concat([prefix, encoded, frame(12, 2, payload)]);
      assert.equal(resumed.length - prefix.length, fixture.resume.addedBytes);
      assert.deepEqual(resumed.subarray(0, prefix.length), prefix);
      assert.deepEqual(inspect(resumed), { end: resumed.length, sequence: prior.sequence + 1, frames: prior.frames + 2, tail: 0 });
    }
    let recoveryCases = 0;
    for (let end = 32; end <= bytes.length; end++) {
      const commit = fixture.commits.filter((row: { end: number }) => row.end <= end).at(-1);
      assert.deepEqual(inspect(bytes.subarray(0, end)), { end: commit?.end ?? 32, sequence: commit?.sequence ?? 0, frames: commit?.recoveredFrames ?? 0, tail: end - (commit?.end ?? 32) }); recoveryCases++;
    }
    const ids = new Set<string>();
    for (const row of fixture.negative) {
      assert(!ids.has(row.id)); ids.add(row.id); let mutated = Buffer.from(bytes); const limits = { ...fixture.limits };
      if (row.operation === "replace-first-length") mutated = Buffer.concat([mutated.subarray(0, 32), Buffer.from(row.hex, "hex"), mutated.subarray(33)]);
      else if (row.operation === "record-limit") limits.records = row.value;
      else if (row.operation === "file-limit") limits.fileBytes = row.value;
      else {
        const offset = row.operation === "commit-xor" ? 94 + row.offset : row.operation === "second-commit-xor" ? 183 + row.offset : row.offset; mutated[offset] ^= row.value;
        if (row.repairCrc) {
          if (row.operation === "header-xor") mutated.writeUInt32LE(checksum(mutated.subarray(0, 20)), 20);
          else if (row.operation === "second-commit-xor") mutated.writeUInt32LE(checksum(mutated.subarray(181, 247)), 247);
          else mutated.writeUInt32LE(checksum(mutated.subarray(92, 158)), 158);
        }
      }
      assert.throws(() => inspect(mutated, limits), new RegExp(`^Error: ${row.error}$`), row.id);
    }
    const extra = structuredClone(fixture); extra.commits[0].records[0].unowned = true; assert(!validate(extra));
    for (const row of fixture.compressed) {
      assert(!ids.has(row.id)); ids.add(row.id);
      const rawLength = Buffer.from(row.rawLengthHex, "hex"); const stored = Buffer.from(row.storedHex, "hex");
      const encoded = frame(row.kind, row.flags, Buffer.concat([rawLength, stored]));
      const payload = Buffer.alloc(64); payload.writeBigUInt64LE(1n); payload.writeBigUInt64LE(BigInt(encoded.length), 16); payload.writeUInt32LE(1, 24);
      hash(Buffer.concat([hash(header), hash(encoded)])).copy(payload, 32);
      const committed = Buffer.concat([header, encoded, frame(12, 2, payload)]);
      if (row.error === null) {
        const raw = Buffer.from(row.rawHex, "hex"); assert.deepEqual(inflateRawSync(stored), raw);
        assert.deepEqual(rawLength, Buffer.from(leb.encodeU32(raw.length)));
        assert.deepEqual(inspect(committed), { end: committed.length, sequence: 1, frames: 2, tail: 0 });
      } else assert.throws(() => inspect(committed), new RegExp(`^Error: ${row.error}$`), row.id);
    }
    process.stdout.write(`[DEBUG] independent retained SPR oracle: 2 commits, ${recoveryCases} exact LastCommit prefixes, ${fixture.negative.length} strict hostile denials, ${fixture.compressed.length} compressed grammar cases; no typed history publication\n`);
    const source = readFileSync(join(owner, "🦀️.rs"), "utf8");
    const laws = ["retained_spr_verification_matches_neutral_commits_and_torn_prefixes", "retained_spr_verification_rejects_hostile_frames_without_publication", "retained_spr_resume_preserves_exact_prefix_and_commit_chain"];
    for (const law of laws) assert(source.includes(`fn ${law}(`), `missing retained SPR law ${law}`);
    assert(source.includes("fn verify_compressed_fixture(") && source.includes("verify_compressed_fixture(&fixture).await;"), "native compressed raw-length parity law is missing");
    assert(readFileSync(join(owner, "../🦀️.rs"), "utf8").includes("pub async fn resume_verified("), "protocol-owned verified writer resume is missing");
    assert.equal(BigInt(fixture.resume.exhaustedSequence) + 1n, 1n << 64n);
    assert(readFileSync(join(owner, "../🦀️.rs"), "utf8").includes("let next_commit_seq = self.next_commit_seq.checked_add(1)"), "commit sequence must reject exhaustion before writing");
    console.log(`retained SPR resume oracle: ${fixture.resume.cuts.length} exact prefixes, next sequence/previous offset/hash chain preserved`);
    if (segments.includes("--oracle-only")) return;
    assert(readFileSync(join(owner, "../🦀️.rs"), "utf8").includes("pub mod retained;"), "retained SPR module is not mounted; native selection cannot run");
    const receipts = await runExactCargoLaws({ cwd: this.repoRoot, groups: [{ package: "semio-framework-replication", target: { kind: "lib", name: "protocol" }, laws: laws.map(law => `format::retained::tests::${law}`) }] });
    assert.equal(receipts[0]!.assertions, laws.length);
  }
}

class RetainedRecordObservationScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.some(segment => segment !== "--oracle-only")) throw new Error("retained-record-observation-check accepts only --oracle-only");
    const { default: assert } = await import("node:assert/strict");
    const { default: Ajv } = await import("ajv/dist/2020.js");
    const { default: crc } = await import("crc-32/crc32c.js");
    const leb = await import("@webassemblyjs/leb128");
    const { blake3Hex } = await import("../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts");
    const owner = join(this.root, "../../📐️format/🔎️verification/🧾️record");
    const fixture = JSON.parse(readFileSync(join(owner, "🧫️fixture/🔣️.json"), "utf8"));
    const ajv = new Ajv({ strict: true }); const validate = ajv.compile(JSON.parse(readFileSync(join(owner, "🧬️schema/🔣️.json"), "utf8")));
    assert(validate(fixture), ajv.errorsText(validate.errors)); const ids = new Set<string>(); let observed = 0;
    const checksum = (bytes: Uint8Array): number => crc.buf(bytes) >>> 0;
    for (const row of fixture.cases) {
      assert(!ids.has(row.id)); ids.add(row.id);
      const body = Buffer.concat([Buffer.from([row.kind, row.flags]), Buffer.from(row.rawHex, "hex"), Buffer.from(row.payloadHex, "hex"), Buffer.alloc(row.repeat, 97)]);
      const length = Buffer.from(leb.encodeU32(body.length)); const tail = Buffer.alloc(8); tail.writeUInt32LE((checksum(body) ^ Number(row.corruptCrc)) >>> 0); tail.writeUInt32LE(length.length + body.length + 8, 4);
      const bytes = Buffer.concat([Buffer.from(fixture.headerHex, "hex"), length, body, tail]);
      const bodyStart = 32 + length.length; const payloadEnd = bodyStart + body.length;
      assert(row.at <= bytes.length); let readyAt = bodyStart + 2; let rawBytes: number | null = null;
      if (row.flags & 1) {
        const raw = Buffer.from(row.rawHex, "hex"); let value = 0n;
        for (let index = 0; index < raw.length; index++) value |= BigInt(raw[index]! & 127) << BigInt(7 * index);
        rawBytes = Number(value); assert.deepEqual(raw, Buffer.from(leb.encodeU32(rawBytes))); readyAt += raw.length;
      }
      let error: string | null = null;
      if (row.at >= bodyStart + 2 && ((row.flags & ~31) !== 0 || Boolean(row.flags & 1) !== Boolean(row.flags & 28))) error = "frame";
      if (row.at === bytes.length) {
        try { const span = inspectRetainedSprNeutral(bytes, checksum, value => Buffer.from(blake3Hex(value), "hex")); assert.equal(span.sequence, 0); assert.equal(span.end, 32); }
        catch (failure) { error = failure instanceof Error ? failure.message : "unknown"; }
      }
      if (row.cancel && error === null) error = "cancelled";
      const observation = error === null && row.at >= readyAt && row.at < bytes.length
        ? { frameStart: 32, payloadStart: readyAt, payloadEnd, frameEnd: bytes.length, kind: row.kind, flags: row.flags, rawBytes } : null;
      assert.equal(error, row.error, row.id); assert.deepEqual(observation, row.observation, row.id); if (observation) observed++;
    }
    const extra = structuredClone(fixture); extra.cases[0].authority = true; assert(!validate(extra));
    console.log(`[DEBUG] retained SPR observation oracle: ${fixture.cases.length} exact rows, ${observed} scalar observations; compressed raw-length/empty payload/clear/error/cancel; zero commit or input authority`);
    const source = readFileSync(join(owner, "🦀️.rs"), "utf8");
    const law = "retained_record_observation_uses_the_existing_framing_state_without_authority";
    assert(source.includes('include_str!("🧫️fixture/🔣️.json")') && source.includes(`fn ${law}(`));
    assert(source.includes("impl RetainedSprVerification") && !source.includes("fn push("), "metadata must observe the existing scanner, not parse a second framing grammar");
    if (segments.includes("--oracle-only")) return;
    assert(readFileSync(join(owner, "../🦀️.rs"), "utf8").includes("pub mod record;"), "retained record observation remains unmounted");
    const receipts = await runExactCargoLaws({ cwd: this.repoRoot, groups: [{ package: "semio-framework-replication", target: { kind: "lib", name: "protocol" }, laws: [`format::retained::record::tests::${law}`] }] });
    assert.equal(receipts[0]!.assertions, 1);
  }
}

/** 🛡️ Cross-language hostile-input oracle for the exact bounded PresencePeer codec. */
class PresencePeerCodecScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.some(segment => segment !== "--oracle-only")) throw new Error("presence-peer-codec-check accepts only --oracle-only");
    const { default: assert } = await import("node:assert/strict");
    const { default: Ajv } = await import("ajv/dist/2020.js");
    const owner = join(this.root, "../../🧫️fixtures/👥️presence-peer-codec-v1");
    const fixture = JSON.parse(readFileSync(join(owner, "🧪️fixture/🔣️.json"), "utf8"));
    const schema = JSON.parse(readFileSync(join(owner, "🧬️schema/🔣️.json"), "utf8"));
    const validate = new Ajv({ strict: true }).compile(schema);
    assert(validate(fixture), validate.errors?.map(error => `${error.instancePath} ${error.message}`).join("; "));
    const codec = await import(join(this.root, "../../🟦️.ts"));
    assert.deepEqual(codec.PRESENCE_PEER_WIRE_LIMITS_V1, fixture.limits);
    const ids = new Set<string>();
    for (const row of fixture.cases) {
      assert(!ids.has(row.id), `duplicate fixture id ${row.id}`); ids.add(row.id);
      const bytes = Buffer.concat([Buffer.from(row.prefixHex, "hex"), Buffer.alloc(row.repeatCount, Number.parseInt(row.repeatHex, 16)), Buffer.from(row.suffixHex, "hex")]);
      const position: [number] = [0];
      if (row.accepted) {
        const peer = codec.decodePresencePeer(bytes, position);
        assert.equal(position[0], bytes.length, row.id);
        assert.equal(Buffer.from(codec.encodePresencePeer(peer)).toString("hex"), row.canonicalHex, row.id);
        const semantic = JSON.parse(JSON.stringify(peer));
        if (peer.presencePack !== undefined) semantic.presencePack = Buffer.from(peer.presencePack).toString("base64");
        if (peer.interaction !== undefined) { semantic.interaction.appId = peer.interaction.app_id; delete semantic.interaction.app_id; }
        assert.deepEqual(semantic, row.expected, row.id);
      } else {
        assert.throws(() => codec.decodePresencePeer(bytes, position), undefined, row.id);
        assert.equal(position[0], 0, `${row.id} advanced the caller cursor`);
      }
    }
    const extra = structuredClone(fixture); extra.cases[0].authority = true; assert(!validate(extra));
    const source = readFileSync(join(this.root, "../../📡️wire/🦀️.rs"), "utf8");
    const laws = ["presence_peer_decoder_matches_neutral_bounded_exact_corpus", "presence_peer_decoder_rejects_hostile_counts_before_allocation"];
    for (const law of laws) assert(source.includes(`fn ${law}(`), `missing native presence codec law ${law}`);
    assert(source.includes("PRESENCE_PEER_WIRE_LIMITS_V1") && source.includes("reader.position != bytes.len()"), "Rust bounded exact decoder is absent");
    console.log(`presence peer codec oracle: ${fixture.cases.length} neutral Rust/TypeScript vectors, ${fixture.cases.filter((row: { accepted: boolean }) => !row.accepted).length} hostile inputs rejected exactly`);
    if (segments.includes("--oracle-only")) return;
    const receipts = await runExactCargoLaws({ cwd: this.repoRoot, groups: [{ package: "semio-framework-replication", target: { kind: "lib", name: "protocol" }, laws: laws.map(law => `wire::frames::presence_codec_tests::${law}`) }] });
    assert.equal(receipts[0]!.assertions, laws.length);
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("build", BuildScript).register("test-source", SourceTestScript).register("test-local-interaction-source", LocalInteractionSourceTestScript).register("test-local-interaction-native", LocalInteractionNativeTestScript).register("retained-verification-check", RetainedVerificationScript).register("retained-record-observation-check", RetainedRecordObservationScript).register("presence-peer-codec-check", PresencePeerCodecScript);

if (import.meta.main) await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
