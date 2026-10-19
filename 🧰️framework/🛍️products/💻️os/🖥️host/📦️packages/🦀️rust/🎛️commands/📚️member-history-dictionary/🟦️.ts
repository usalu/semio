/** 📚️ The member history dictionary, record and foundation checks of the OS host router.
 *
 * @see ../../📜️script.ts — the package script that registers the commands of this module
 */
import { scriptCompilerAdmissionControlV1 as exactCargoCallerControlV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { unsignedLebOracleV1 } from "../../../../../../../🔨️modules/🎒️pack/📦️packages/🟦️typescript/📐️format/🧪️testing/🔮️oracles/🔢️leb/🟦️.ts";
import { configuredExactCargoLawPolicyV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️laws/🟦️.ts";
import { join } from "node:path";
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { runAdmittedExactCargoLawsV1 } from "@semio-tech/process/cargo-test";
import { BundleScript } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { blake3Hex } from "../../../../../../../📦️packages/🟦️typescript/🔏️hash/🟦️.ts";
import { MemberHistoryIdentitySourceScript, MemberHistoryIdScript, MemberHistoryInputScript } from "../🪪️member-history-identity/🟦️.ts";

/** 🔤️ Test-only retained UTF-8 scalar model shared by payload and actual-input owner oracles. */
function dictionaryUtf8Neutral() {
  const scratch: number[] = []; let scalarBytes = 0;
  const malformed = (): never => { throw new Error("malformed"); };
  return {
    scratch,
    push(byte: number): void {
      if (!scratch.length && byte < 128) return;
      if (!scratch.length) scalarBytes = byte >= 194 && byte <= 223 ? 2 : byte >= 224 && byte <= 239 ? 3 : byte >= 240 && byte <= 244 ? 4 : malformed();
      else if ((byte & 192) !== 128) malformed();
      scratch.push(byte);
      if (scratch.length === scalarBytes) {
        try { new TextDecoder("utf-8", { fatal: true, ignoreBOM: true }).decode(Buffer.from(scratch)); } catch { malformed(); }
        scratch.fill(0); scratch.length = 0; scalarBytes = 0;
      }
    },
    finish(): void { if (scratch.length) malformed(); },
    close(grant: number): number { const count = Math.min(grant, scratch.length); scratch.fill(0, scratch.length - count); scratch.length -= count; if (!scratch.length) scalarBytes = 0; return count; },
  };
}

export class MemberHistoryRecordScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.some(segment => segment !== "--oracle-only")) throw new Error("member-history-record-check accepts only --oracle-only");
    const leb = unsignedLebOracleV1();
    const owner = join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/📜️history/🗂️dictionary/🧾️record");
    const fixture = JSON.parse(readFileSync(join(owner, "🧪️tests/🧫️fixtures/🔣️.json"), "utf8"));
    let accepted = 0; const ids = new Set<string>();
    const fail: (reason: string) => never = reason => { throw new Error(reason); };
    for (const row of fixture.cases) {
      assert(!ids.has(row.id)); ids.add(row.id);
      for (const grant of fixture.grants) {
        const bytes = Buffer.concat([Buffer.from(row.hex, "hex"), Buffer.alloc(row.repeat?.count ?? 0, row.repeat?.byte ?? 0)]);
        let offset = 0; let stage = "format"; let pending: string | null = null; let value = 0n; let digits: number[] = [];
        let base = 0; let entryStart = 0; let entryLength = 0; const events: unknown[] = [];
        let remainingEntries = 0; let remainingBytes = 0; let entries = 0; const utf8 = dictionaryUtf8Neutral(); let error: string | null = null;
        const entry = () => { entries++; remainingEntries--; pending = "entry"; events.push(["entry", entryStart, entryLength]); stage = remainingEntries ? "length" : "done"; };
        try {
          while (offset < bytes.length) {
            let fuel = grant;
            while (fuel && offset < bytes.length) {
              if (pending !== null) fail("state"); if (stage === "done") fail("malformed");
              const byte = bytes[offset++]!; fuel--;
              if (stage === "format") { if (byte !== 1) fail("malformed"); stage = "base"; }
              else if (stage === "text") {
                utf8.push(byte);
                remainingBytes--; if (!remainingBytes) { utf8.finish(); entry(); }
              } else {
                if (digits.length === 9 && byte > 1) fail("malformed");
                value |= BigInt(byte & 127) << BigInt(digits.length * 7); digits.push(byte);
                if (byte < 128) {
                  if (digits.length > 1 && byte === 0) fail("malformed");
                  if (value <= 0xffffffffn) assert(Buffer.from(digits).equals(Buffer.from(leb.encodeUnsigned32(Number(value)))));
                  if (stage === "base") { base = Number(value); stage = "count"; }
                  else if (stage === "count") { if (value > 8192n) fail("capacity"); remainingEntries = Number(value); pending = "begin"; events.push(["begin", base, remainingEntries]); stage = value ? "length" : "done"; }
                  else {
                    if (value > 1048576n) fail("capacity"); if (value > BigInt(bytes.length - offset)) fail("malformed");
                    remainingBytes = Number(value); entryStart = offset; entryLength = remainingBytes; if (!remainingBytes) entry(); else stage = "text";
                  }
                  digits = []; value = 0n;
                }
              }
              if (row.cancelAt === offset) fail("cancelled");
              if (pending !== row.hold) pending = null;
            }
          }
          if (pending !== null) fail("state"); if (stage !== "done") fail("malformed");
        } catch (failure) { error = failure instanceof Error ? failure.message : "unknown"; }
        const expectedEvents = fixture.events[row.id].flatMap((event: [string, number, number, number?, number?]) => event[0] === "begin" ? [event] : Array.from({ length: event[3]! }, (_, index) => ["entry", event[1] + index * event[4]!, event[2]]));
        assert.deepEqual(events, expectedEvents, row.id);
        assert.equal(error, row.error, row.id); assert.equal(offset, row.offset, row.id); assert.equal(entries, row.entries, row.id); assert.equal(utf8.scratch.length, row.scratchBytes, row.id);
        let retired = 0;
        while (utf8.scratch.length) retired += utf8.close(grant);
        assert.equal(retired, row.scratchBytes, row.id); if (grant === 1 && error === null) accepted++;
      }
    }
    assert.deepEqual(Object.keys(fixture.events).sort(), [...ids].sort());
    console.log(`[TRACE] dictionary payload cursor oracle: ${accepted} accepted / ${fixture.cases.length - accepted} denied exact wires ×3 grants; exact ordered Begin/base/count and Entry/ranges, UTF8 scratch0..4, event fences, canonicalLEB, pinned retirement; no input authority`);
    const implementation = readFileSync(join(owner, "🦀️.rs"), "utf8");
    assert(implementation.includes('#[path = "🧪️tests/🔬️unit/🦀️.rs"]'));
    const source = readFileSync(join(owner, "🧪️tests/🔬️unit/🦀️.rs"), "utf8");
    assert(source.includes('🧪️tests/🧫️fixtures/🔣️.json")'), "native payload cursor must consume its exact neutral fixture");
    assert(source.includes('fixture["events"][name]') && source.includes("assert_eq!(events, expected_events"), "native payload cursor must compare every exact ordered neutral event trace");
    const laws = ["retained_dictionary_delta_matches_neutral_text_ranges_without_publication", "retained_dictionary_delta_rejects_tail_and_preserves_partial_utf8_until_close"];
    for (const law of laws) assert(source.includes(`fn ${law}`));
    if (segments.includes("--oracle-only")) return;
    assert(readFileSync(join(owner, "../🦀️.rs"), "utf8").includes("mod record;"), "dictionary record cursor remains unmounted");
    const receipts = await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0), invocation: this.invocation, policy: { ...configuredExactCargoLawPolicyV1() }, cwd: this.repoRoot, groups: [{ package: "semio-framework-os-kernel", target: { kind: "lib" }, laws }] });
    assert.equal(receipts[0]!.assertions, 2);
  }
}

export class MemberHistoryFoundationScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.some(segment => segment !== "--oracle-only")) throw new Error("member-history-foundation-check accepts only --oracle-only");
    const replicationRoot = join(this.repoRoot, "🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust");
    const { RetainedVerificationScript } = await import(join(replicationRoot, "📜️script.ts"));
    await new RetainedVerificationScript(replicationRoot, this.repoRoot, this.invocation).run(["--oracle-only"]);
    await new MemberHistoryInputScript(this.root, this.repoRoot, this.invocation).run(["--oracle-only"]);
    await new MemberHistoryIdScript(this.root, this.repoRoot, this.invocation).run(["--oracle-only"]);
    await new MemberHistoryIdentitySourceScript(this.root, this.repoRoot, this.invocation).run([]);
    if (segments.includes("--oracle-only")) return;
    const mounts = [
      ["🧰️framework/🔨️modules/📡️replication/📐️format/🦀️.rs", "pub mod retained;"],
      ["🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/🦀️.rs", "pub(crate) mod history;"],
      ["🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📜️history/🦀️.rs", "pub(crate) mod identity;"],
    ];
    for (const [path, declaration] of mounts) assert(readFileSync(join(this.repoRoot, path!), "utf8").includes(declaration!));
    const receipts = await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0), invocation: this.invocation, policy: { ...configuredExactCargoLawPolicyV1() }, cwd: this.repoRoot, groups: [
      { package: "semio-framework-replication", target: { kind: "lib", name: "protocol" }, laws: ["format::retained::tests::retained_spr_verification_matches_neutral_commits_and_torn_prefixes", "format::retained::tests::retained_spr_verification_rejects_hostile_frames_without_publication"] },
      { package: "semio-framework-os-kernel", target: { kind: "lib" }, laws: ["retained_history_id_cursor_matches_neutral_bytes_and_refuses_unowned_resolution"] },
      { package: "semio-framework-os-store-member-open-tests", target: { kind: "lib" }, laws: ["member_history_verification_retains_input_and_bounds_verified_handoff", "member_history_verification_rechecks_every_owner_transition_and_retires_exact_bytes"] },
    ] });
    assert.deepEqual(receipts.map(receipt => receipt.assertions), [2, 3]);
    console.log("[TRACE] retained history foundation: exactly5 native assertions; framing2 + private input2 + tagged-ID1; no dictionary/member factory/publication claim");
  }
}

export class MemberHistoryDictionaryScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.some(segment => segment !== "--oracle-only")) throw new Error("member-history-dictionary-check accepts only --oracle-only");
    const leb = unsignedLebOracleV1();
    const { default: crc } = await import("crc-32/crc32c.js");
    const { inspectRetainedSprNeutral } = await import(join(this.repoRoot, "🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/📜️script.ts"));
    const owner = join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/📜️history/🗂️dictionary");
    const fixture = JSON.parse(readFileSync(join(owner, "🧪️tests/🧫️fixtures/🔣️.json"), "utf8"));
    
    const checksum = (bytes: Uint8Array): number => crc.buf(bytes) >>> 0;
    const hash = (bytes: Buffer): Buffer => Buffer.from(blake3Hex(bytes), "hex");
    const uint = (value: number): Buffer => Buffer.from(leb.encodeUnsigned32(value));
    const fail: (reason: string) => never = reason => { throw new Error(reason); };
    const frame = (kind: number, payload: Buffer, critical = kind !== 65): Buffer => {
      const body = Buffer.concat([Buffer.from([kind, critical ? 2 : 0]), payload]); const prefix = uint(body.length); const tail = Buffer.alloc(8);
      tail.writeUInt32LE(checksum(body)); tail.writeUInt32LE(prefix.length + body.length + 8, 4); return Buffer.concat([prefix, body, tail]);
    };
    const dictionary = (base: number, entries: Buffer[]): Buffer => Buffer.concat([Buffer.from([1]), uint(base), uint(entries.length), ...entries.flatMap(entry => [uint(entry.length), entry])]);
    const build = (row: { operation: string; value?: number; index?: number; hex?: string }): Buffer => {
      let first = fixture.dictionary.map((entry: string) => Buffer.from(entry)); const second = fixture.secondDictionary.map((entry: string) => Buffer.from(entry));
      if (row.operation === "invalid-utf8") first[0] = Buffer.from([0xc3, 0x28]);
      if (row.operation === "record-scratch") first[row.index!] = Buffer.from(row.hex!, "hex");
      if (row.operation === "empty-identity") first[0] = Buffer.alloc(0);
      if (row.operation === "bom-identity") first[0] = Buffer.concat([Buffer.from([239, 187, 191]), first[0]]);
      let firstDelta = dictionary(row.operation === "first-base" ? row.value! : 0, first);
      if (row.operation === "delta-tail") firstDelta = Buffer.concat([firstDelta, Buffer.from([0])]);
      const rawDocument = row.operation === "raw-document" ? Buffer.from(row.hex!, "hex") : null;
      const document = rawDocument ? Buffer.concat([Buffer.from([1, 0]), uint(rawDocument.length), rawDocument, Buffer.from([1, 1])]) : Buffer.from(fixture.docHex, "hex");
      const records = [frame(3, firstDelta, row.operation !== "noncritical-dictionary"), frame(1, document, row.operation !== "noncritical-document")];
      if (row.operation === "duplicate-document") records.push(frame(1, Buffer.from(fixture.docHex, "hex")));
      records.push(frame(3, dictionary(row.operation === "second-base" ? row.value! : first.length, second)));
      if (row.operation === "extra-entries") records.push(frame(3, dictionary(10, Array.from({ length: row.value! }, (_, index) => Buffer.from(`entry-${index}`)))));
      if (row.operation === "unused-empty") records.push(frame(3, dictionary(10, [Buffer.alloc(0)])));
      const composition = Buffer.from(fixture.compositionHex, "hex");
      if (row.operation === "lookup-missing") composition[3] = 127;
      if (row.operation === "wrong-owner") composition[5] = 6;
      if (row.operation === "wrong-dialect") composition[13] = 3;
      if (row.operation === "earlier-foreign") { const foreign = Buffer.from(composition); foreign[5] = 6; records.push(frame(65, foreign)); }
      if (row.operation === "earlier-malformed") records.push(frame(65, Buffer.from([1, 7])));
      if (row.operation === "noncritical-transition") records.push(frame(67, Buffer.from([1]), false));
      if (row.operation !== "missing-composition") records.push(frame(65, composition, row.operation === "critical-composition"));
      const header = Buffer.alloc(32); Buffer.from([137, 83, 80, 82, 13, 10, 26, 10]).copy(header); header.writeUInt16LE(1, 8);
      header.writeUInt32LE(1, 12); header.writeUInt32LE(1, 16); header.writeUInt32LE(checksum(header.subarray(0, 20)), 20);
      const commit = Buffer.alloc(64); commit.writeBigUInt64LE(1n); commit.writeBigUInt64LE(BigInt(records.reduce((sum, item) => sum + item.length, 0)), 16);
      commit.writeUInt32LE(records.length, 24); hash(Buffer.concat([hash(header), ...records.map(hash)])).copy(commit, 32);
      const parts = [header, ...records, frame(12, commit)];
      if (row.operation === "uncommitted-tail") parts.push(frame(3, dictionary(10, [Buffer.from("uncommitted")] )));
      return Buffer.concat(parts);
    };
    type Range = { offset: number; length: number };
    type Event = { stage: string; entries: number; pages: number; offset?: number };
    const authorityOf = (fields: string[]) => ({ artifactId: fields[0], schema: fixture.expected.schema, dialect: fields.slice(1, 4), owner: [`${fields[4]}!${fields[5]}@${fields[6]}/${fields[7]}`, fields[8], fields[9]] });
    assert.deepEqual(authorityOf(fixture.requestIdentity), fixture.expected);
    const model = (bytes: Buffer, row: { operation: string; value?: number; index?: number; text?: string }) => {
      const span = inspectRetainedSprNeutral(bytes, checksum, hash); assert.equal(span.sequence, 1);
      const limits = { ...fixture.limits };
      if (row.operation === "entry-limit") limits.dictionaryEntries = row.value;
      if (row.operation === "byte-limit") limits.dictionaryBytes = row.value;
      const requestIdentity = [...fixture.requestIdentity];
      if (row.operation === "request-field") {
        assert.equal(Buffer.byteLength(row.text!), Buffer.byteLength(requestIdentity[row.index!])); requestIdentity[row.index!] = row.text;
        if (row.index === 0 || row.index === 9) { requestIdentity[0] = row.text; requestIdentity[9] = row.text; }
      }
      assert.equal(requestIdentity[0], requestIdentity[9], "retained requests were admitted with exact child identity");
      const expected = authorityOf(requestIdentity);
      const utf8 = dictionaryUtf8Neutral();
      const state = { entries: [] as Range[], provisional: [] as Range[], pages: 0, dictionaryBytes: 0, ready: false, error: null as string | null, holder: "owner", handoffs: 0, copies: 0, parses: 0, units: 0 };
      const scratch = { pending: [] as number[], lookup: [] as number[], id: [] as number[] };
      const ownerEvents: [string, number, number][] = [];
      const ownerEvent = (stage: string) => ownerEvents.push([stage, state.entries.length, state.pages]);
      const event = (stage: string, offset?: number): Event => ({ stage, entries: state.entries.length, pages: state.pages, offset });
      const reader = (start: number, end: number) => {
        let at = start;
        function* byte(): Generator<Event, number> {
          if (at === end) fail("malformed");
          const offset = at; const value = bytes[at++]!; assert.equal(scratch.pending.length, 0); scratch.pending.push(value);
          state.copies++; yield event("copy", offset); yield event("framing", offset);
          state.parses++; scratch.pending[0] = 0; scratch.pending.pop(); yield event("parse", offset); return value;
        }
        function* number(): Generator<Event, number> {
          const start = at; let value = 0n;
          for (let digit = 0; digit < 10; digit++) {
            const next = yield* byte(); if (digit === 9 && next > 1) fail("malformed"); value |= BigInt(next & 127) << BigInt(7 * digit);
            if (next < 128) {
              if ((digit && next === 0) || value > 0xffffffffn) fail("malformed");
              assert(bytes.subarray(start, at).equals(uint(Number(value)))); return Number(value);
            }
          }
          return fail("malformed");
        }
        function* take(length: number): Generator<Event, Range> {
          if (length > end - at) fail("malformed"); const range = { offset: at, length };
          for (let index = 0; index < length; index++) yield* byte(); return range;
        }
        return { byte, number, take, position: () => at, end: () => { if (at !== end) fail("malformed"); } };
      };
      const decode = (range: Range): string => {
        if (range.offset + range.length > span.end) fail("state");
        try { return new TextDecoder("utf-8", { fatal: true, ignoreBOM: true }).decode(bytes.subarray(range.offset, range.offset + range.length)); } catch { return fail("malformed"); }
      };
      function* id(input: ReturnType<typeof reader>): Generator<Event, string> {
        assert.equal(scratch.id.length, 0); const tag = yield* input.byte(); let range: Range;
        if (tag === 0) {
          const length = yield* input.number(); if (length > limits.identityBytes) fail("capacity"); range = { offset: input.position(), length };
          for (let index = 0; index < length; index++) { scratch.id.push(yield* input.byte()); yield event("id-raw"); }
        }
        else if (tag === 1 || tag === 2) {
          const index = yield* input.number(); if (index >= state.entries.length) fail("malformed"); range = state.entries[index]!; yield event("lookup");
          if (range.length > limits.identityBytes - (tag === 2 ? 37 : 0)) fail("capacity");
          for (let index = 0; index < range.length; index++) {
            scratch.lookup.push(bytes[range.offset + index]!); state.copies++; yield event("lookup-copy");
            scratch.id.push(scratch.lookup[0]!); scratch.lookup[0] = 0; scratch.lookup.pop(); state.parses++; yield event("lookup-parse");
          }
        } else return fail("malformed");
        let value = decode(range); if (!value || /\p{Cc}/u.test(value)) fail("identity");
        if (tag === 2) { const uuidRange = yield* input.take(16); const hex = bytes.subarray(uuidRange.offset, uuidRange.offset + 16).toString("hex"); value += `-${hex.slice(0, 8)}-${hex.slice(8, 12)}-${hex.slice(12, 16)}-${hex.slice(16, 20)}-${hex.slice(20)}`; scratch.id = [...Buffer.from(value)]; }
        yield event("id-complete");
        while (scratch.id.length) { scratch.id[scratch.id.length - 1] = 0; scratch.id.pop(); yield event("id-retire"); }
        return value;
      }
      function* scan(): Generator<Event> {
        yield event("begin"); let at = 32; let records = 0; let document: string[] | null = null; let overlay: { owner: string[] | null; dialect: string[] | null } | null = null;
        while (at < span.end) {
          if (++records > limits.records) fail("capacity"); const framing = reader(at, span.end); const size = yield* framing.number();
          const bodyStart = framing.position(); const end = bodyStart + size; if (end + 8 > span.end) fail("malformed");
          const input = reader(bodyStart, end); const kind = yield* input.byte(); const flags = yield* input.byte();
          if (((kind === 1 || kind === 3 || kind === 67) && flags !== 2) || (kind === 65 && flags !== 0)) fail("malformed");
          if (kind === 3) {
            if ((yield* input.byte()) !== 1 || (yield* input.number()) !== state.entries.length) fail("malformed");
            const count = yield* input.number(); if (count > limits.dictionaryEntries - state.entries.length) fail("capacity"); state.provisional = []; ownerEvent("delta-begin");
            for (let entry = 0; entry < count; entry++) {
              const length = yield* input.number(); if (length > limits.dictionaryBytes - state.dictionaryBytes) fail("capacity");
              state.dictionaryBytes += length; const range = { offset: input.position(), length };
              for (let byte = 0; byte < length; byte++) { utf8.push(yield* input.byte()); yield event(`dictionary-byte:${entry}:${byte + 1}`); }
              utf8.finish(); decode(range);
              const required = Math.ceil((state.entries.length + state.provisional.length + 1) / limits.pageEntries);
              if (required > limits.pages) fail("capacity");
              if (required > state.pages) { state.pages++; yield event("page-admission"); }
              state.provisional.push(range); ownerEvent("entry"); if (state.entries.length === 0 && entry === 0) yield event("first-entry");
            }
            input.end(); state.entries.push(...state.provisional); state.provisional = []; ownerEvent("delta"); yield event(state.entries.length === 7 ? "first-delta" : "delta");
          } else if (kind === 1) {
            if (document !== null || (yield* input.byte()) !== 1) fail("malformed"); document = [yield* id(input), yield* id(input)]; input.end();
          } else if (kind === 65) {
            if ((yield* input.byte()) !== 1) fail("malformed"); const presence = yield* input.byte(); if (presence & ~3) fail("malformed");
            const owner = presence & 1 ? [yield* id(input), yield* id(input), yield* id(input)] : null;
            const dialect = presence & 2 ? [yield* id(input), yield* id(input), yield* id(input)] : null;
            input.end(); overlay = { owner, dialect }; yield event("composition");
          }
          at = end + 8;
        }
        if (JSON.stringify(document) !== JSON.stringify([expected.artifactId, expected.schema])
          || JSON.stringify(overlay?.owner) !== JSON.stringify(expected.owner) || JSON.stringify(overlay?.dialect) !== JSON.stringify(expected.dialect)) fail("identity");
        state.ready = true; yield event("ready");
      }
      const iterator = scan();
      const next = (): IteratorResult<Event> => { if (state.error !== null || state.holder !== "owner") return { done: true, value: undefined }; state.units++; return iterator.next(); };
      const take = (): string | null => {
        if (state.holder !== "owner") return "stale"; if (state.error) return state.error; if (!state.ready) return "pending";
        state.holder = "witness"; state.handoffs++; return null;
      };
      const close = (grant: number) => {
        state.holder = "retiring"; const inputBytes = bytes.length + 2; const identityBytes = requestIdentity.reduce((sum: number, value: string) => sum + Buffer.byteLength(value), 0);
        const ownerScratch = { pendingBytes: scratch.pending.length, lookupBytes: scratch.lookup.length, idBytes: scratch.id.length };
        const scratchBytes = utf8.scratch.length + ownerScratch.pendingBytes + ownerScratch.lookupBytes + ownerScratch.idBytes;
        let indexBytes = state.pages * limits.pageBytes; let input = inputBytes + identityBytes; let retired = 0; let releases = 0; let currentPage = limits.pageBytes;
        while (scratch.pending.length || scratch.lookup.length || scratch.id.length || utf8.scratch.length || indexBytes || input) {
          const before = retired; let fuel = grant;
          const retained = [scratch.pending, scratch.lookup, scratch.id].find(bytes => bytes.length);
          if (retained) { while (fuel && retained.length) { retained[retained.length - 1] = 0; retained.pop(); fuel--; retired++; } }
          else if (utf8.scratch.length) retired += utf8.close(fuel);
          else if (indexBytes) {
            const used = Math.min(fuel, currentPage); fuel -= used; currentPage -= used; indexBytes -= used; retired += used;
            if (!currentPage) { releases++; currentPage = limits.pageBytes; }
          } else { const used = Math.min(fuel, input); input -= used; retired += used; }
          assert(retired - before <= grant); assert.equal(take(), "stale");
        }
        assert.equal(releases, state.pages); assert.equal(retired, scratchBytes + state.pages * limits.pageBytes + inputBytes + identityBytes);
        state.entries.length = 0; state.provisional.length = 0; state.pages = 0; state.holder = "terminal";
        return { retired, inputBytes, identityBytes, releases, scratchBytes, ...ownerScratch };
      };
      return { state, span, next, take, close, ownerEvents, scratchBytes: () => utf8.scratch.length };
    };
    const ids = new Set<string>(); let accepted = 0; let maxRetired = 0;
    for (const row of fixture.cases) {
      assert(!ids.has(row.id)); ids.add(row.id);
      for (const grant of fixture.grants) {
        const run = model(build(row), row); let terminal = false;
        try { while (!terminal) { for (let fuel = grant; fuel && !terminal; fuel--) terminal = Boolean(run.next().done); } }
        catch (error) { run.state.error = error instanceof Error ? error.message : "unknown"; }
        assert.equal(run.state.error, row.error, row.id); assert.equal(run.state.entries.length, row.entries, row.id); assert.equal(run.state.pages, row.pages, row.id);
        if (row.operation === "unchanged") { assert.deepEqual(run.ownerEvents, fixture.ownerEvents); assert.deepEqual(run.state.entries.map(range => [range.offset, range.length]), fixture.dictionaryRanges); }
        if (row.error === null) { assert.equal(run.take(), null); assert.equal(run.take(), "stale"); } else assert.equal(run.take(), row.error);
        const closed = run.close(grant); assert.equal(closed.inputBytes, row.inputBytes, row.id); assert.equal(closed.retired, row.retiredBytes, row.id); maxRetired = Math.max(maxRetired, closed.retired);
      }
      if (row.error === null) accepted++;
    }
    for (const row of fixture.lifecycle) {
      assert(!ids.has(row.id)); ids.add(row.id);
      for (const grant of fixture.grants) {
        const run = model(build({ operation: "unchanged" }), { operation: "unchanged" }); let found = false;
        while (!found) { const step = run.next(); assert(!step.done); found = step.value.stage === (row.at === "witness" ? "ready" : row.at); }
        if (row.at === "witness") assert.equal(run.take(), null);
        const authority = { ...fixture.authority, cancelled: false, clock: true };
        if (row.event === "cancel") authority.cancelled = true;
        if (row.event === "generation") authority.generation++;
        if (row.event === "operation") authority.operation++;
        if (row.event === "expired") authority.nowUs = authority.expiresAtUs;
        if (row.event === "clock-absent") authority.clock = false;
        run.state.error = authority.operation !== fixture.authority.operation || authority.generation !== fixture.authority.generation ? "stale"
          : authority.cancelled ? "cancelled" : !authority.clock || authority.nowUs >= authority.expiresAtUs ? "expired" : null;
        if (row.at === "ready" && row.error === null) assert.equal(run.take(), null);
        assert.equal(run.state.error, row.error); assert.equal(run.state.entries.length, row.entries); assert.equal(run.state.pages, row.pages); assert.equal(run.state.handoffs, row.handoffs);
        const units = run.state.units; assert(run.next().done); assert.equal(run.state.units, units); if (run.state.handoffs) assert.equal(run.take(), "stale");
        const retired = run.close(grant); assert.equal(retired.identityBytes, 72); assert.equal(retired.inputBytes, row.inputBytes); assert.equal(retired.retired, row.retiredBytes);
      }
    }
    for (const row of fixture.recordRetirement) {
      assert(!ids.has(row.id)); ids.add(row.id);
      for (const grant of fixture.grants) {
        const run = model(build({ operation: "record-scratch", index: row.index, hex: row.hex }), { operation: "unchanged" });
        try {
          while (true) {
            const step = run.next(); if (step.done) break;
            if (row.cancelAfter !== null && step.value.stage === `dictionary-byte:${row.index}:${row.cancelAfter}`) { run.state.error = "cancelled"; break; }
          }
        } catch (failure) { run.state.error = failure instanceof Error ? failure.message : "unknown"; }
        assert.equal(run.state.error, row.error, row.id); assert.equal(run.state.entries.length, row.entries, row.id); assert.equal(run.state.pages, row.pages, row.id);
        assert.equal(run.scratchBytes(), row.scratchBytes, row.id); assert.equal(run.take(), row.error); const units = run.state.units;
        assert(run.next().done); assert.equal(run.state.units, units);
        const closed = run.close(grant); assert.equal(closed.inputBytes, row.inputBytes, row.id); assert.equal(closed.scratchBytes, row.scratchBytes, row.id); assert.equal(closed.retired, row.retiredBytes, row.id);
      }
    }
    for (const row of fixture.ownerRetirement) {
      assert(!ids.has(row.id)); ids.add(row.id);
      for (const grant of fixture.grants) {
        const run = model(build(row), { operation: "unchanged" }); let occurrence = 0;
        while (true) {
          const step = run.next(); assert(!step.done, row.id);
          if (step.value.stage === row.stage && (row.offset === null || step.value.offset === row.offset) && ++occurrence === row.occurrence) break;
        }
        run.state.error = "cancelled"; assert.equal(run.take(), row.error); assert.equal(run.state.entries.length, row.entries); assert.equal(run.state.pages, row.pages);
        const units = run.state.units; assert(run.next().done); assert.equal(run.state.units, units);
        const closed = run.close(grant);
        for (const key of ["pendingBytes", "lookupBytes", "idBytes", "inputBytes", "retiredBytes"]) assert.equal(key === "retiredBytes" ? closed.retired : closed[key as keyof typeof closed], row[key], `${row.id}:${key}`);
      }
    }
    console.log(`[TRACE] retained dictionary owner oracle: ${accepted} accepted / ${fixture.cases.length - accepted} denied committed SPR histories, ${fixture.lifecycle.length} owner traces + ${fixture.recordRetirement.length} payload scratch + ${fixture.ownerRetirement.length} pending-copy/ID scratch traces × 3 grants; retirement up to ${maxRetired} bytes; no Rust or typed publication claim`);
    const source = readFileSync(join(owner, "🦀️.rs"), "utf8");
    const nativeLaws = readFileSync(join(owner, "🧪️tests/🗂️dictionary/🦀️.rs"), "utf8");
    for (const law of ["member_history_dictionary_is_atomic_and_bounded_by_neutral_records", "member_history_dictionary_retains_every_denied_owner_until_exact_close"]) assert(nativeLaws.includes(`fn ${law}`));
    assert(nativeLaws.includes('🧪️tests/🧫️fixtures/🔣️.json")') && nativeLaws.includes('fixture["recordRetirement"]'), "owner native laws must consume the exact fixture including scratch retirement");
    assert(nativeLaws.includes('fixture["ownerRetirement"]'), "native owner must cover retained wire-copy, lookup-copy and tagged-ID scratch");
    assert(source.includes("RetainedSprVerification::new") && source.includes("observe_record_header()") && source.includes("id.is_complete()") && source.includes("copy_verified_history_chunk"), "owner must reuse the exact retained framing/ID/input contracts");
    assert(readFileSync(join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📜️history/🛂️identity/🪪️id/🦀️.rs"), "utf8").includes("fn is_complete("), "record owner requires a non-poisoning tagged-ID completion query");
    if (segments.includes("--oracle-only")) return;
    assert(readFileSync(join(owner, "../🦀️.rs"), "utf8").includes("mod dictionary;"), "native dictionary owner laws require the coordinated parent mount");
    const receipts = await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0), invocation: this.invocation, policy: { ...configuredExactCargoLawPolicyV1() }, cwd: this.repoRoot, groups: [{ package: "semio-framework-os-store-member-open-tests", target: { kind: "lib" }, laws: ["member_history_dictionary_is_atomic_and_bounded_by_neutral_records", "member_history_dictionary_retains_every_denied_owner_until_exact_close"] }] });
    assert.equal(receipts[0]!.assertions, 2);
  }
}
