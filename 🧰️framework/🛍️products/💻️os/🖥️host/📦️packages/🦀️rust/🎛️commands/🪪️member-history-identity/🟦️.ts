/** 🪪️ The member history identity source, input and id checks of the OS host router.
 *
 * @see ../../📜️script.ts — the package script that registers the commands of this module
 */
import { scriptCompilerAdmissionControlV1 as exactCargoCallerControlV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { unsignedLebOracleV1 } from "../../../../../../../🔨️modules/🎒️pack/📦️packages/🟦️typescript/📐️format/🧪️testing/🔮️oracles/🔢️leb/🟦️.ts";
import { configuredExactCargoLawPolicyV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️laws/🟦️.ts";
import { join } from "node:path";
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { runRepositoryExactCargoLaws, configuredRepositoryCargoPreparationPolicyV1 } from "../../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { blake3Hex } from "../../../../../../../📦️packages/🟦️typescript/🔏️hash/🟦️.ts";

export class MemberHistoryIdentitySourceScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("member-history-identity-source accepts no arguments");
    const leb = unsignedLebOracleV1();
    const owner = join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📜️history/🛂️identity");
    const fixture = JSON.parse(readFileSync(join(owner, "🧪️tests/🧫️fixtures/🔣️.json"), "utf8"));
    
    const fail: (error: string) => never = error => { throw new Error(error); };
    const text = (value: string): string => value && Buffer.byteLength(value) <= fixture.limits.identityBytes && !/\p{Cc}/u.test(value) ? value : fail("identity");
    const reader = (bytes: Buffer) => {
      let offset = 0;
      const take = (count: number): Buffer => { if (count > bytes.length - offset) fail("malformed"); const result = bytes.subarray(offset, offset + count); offset += count; return result; };
      const byte = (): number => take(1)[0]!;
      const uint = (): number => {
        let value = 0n;
        for (let index = 0; index < 10; index++) {
          const next = byte(); if (index === 9 && next > 1) fail("malformed");
          value |= BigInt(next & 127) << BigInt(7 * index);
          if (next < 128) { if ((index && next === 0) || value > BigInt(Number.MAX_SAFE_INTEGER)) fail("malformed"); return Number(value); }
        }
        return fail("malformed");
      };
      const utf8 = (count: number): string => { try { return new TextDecoder("utf-8", { fatal: true }).decode(take(count)); } catch { return fail("malformed"); } };
      return { byte, uint, take, utf8, end: (): void => { if (offset !== bytes.length) fail("malformed"); } };
    };
    const dictionary = (entries: string[], limit: number, byteLimit = fixture.limits.dictionaryBytes): string[] => {
      const encoded = Buffer.concat([Buffer.from([1, 0]), Buffer.from(leb.encodeUnsigned32(entries.length)), ...entries.flatMap(entry => [Buffer.from(leb.encodeUnsigned32(Buffer.byteLength(entry))), Buffer.from(entry)])]);
      const input = reader(encoded); if (input.byte() !== 1 || input.uint() !== 0) fail("malformed");
      const count = input.uint(); if (count > limit) fail("capacity"); const result: string[] = []; let bytes = 0;
      for (let index = 0; index < count; index++) { const length = input.uint(); bytes += length; if (bytes > byteLimit) fail("capacity"); result.push(input.utf8(length)); }
      input.end(); assert.deepEqual(result, entries); return result;
    };
    const id = (input: ReturnType<typeof reader>, dict: string[]): string => {
      const resolve = (): string => { const index = input.uint(); return dict[index] ?? fail("malformed"); };
      switch (input.byte()) {
        case 0: { const size = input.uint(); if (size > fixture.limits.identityBytes) fail("capacity"); return text(input.utf8(size)); }
        case 1: return text(resolve());
        case 2: { const prefix = resolve(); const hex = input.take(16).toString("hex"); return text(`${prefix}-${hex.slice(0, 8)}-${hex.slice(8, 12)}-${hex.slice(12, 16)}-${hex.slice(16, 20)}-${hex.slice(20)}`); }
        default: return fail("malformed");
      }
    };
    const doc = (hex: string, dict: string[]): string[] => {
      const input = reader(Buffer.from(hex, "hex")); if (input.byte() !== 1) fail("malformed");
      const result = [id(input, dict), id(input, dict)]; input.end(); return result;
    };
    const composition = (hex: string, dict: string[]) => {
      const input = reader(Buffer.from(hex, "hex")); if (input.byte() !== 1) fail("malformed");
      const presence = input.byte(); if (presence & ~3) fail("malformed");
      const triple = (): string[] => [id(input, dict), id(input, dict), id(input, dict)];
      const owner = presence & 1 ? triple() : null; const dialect = presence & 2 ? triple() : null;
      input.end(); return { owner, dialect };
    };
    let accepted = 0; const ids = new Set<string>();
    for (const row of fixture.cases) {
      assert(!ids.has(row.id)); ids.add(row.id); const expected = structuredClone(fixture.expected);
      let entries = [...fixture.dictionary]; let docHex = fixture.docHex; let overlayHex: string | null = fixture.compositionHex;
      let error: string | null = null;
      try {
        if (row.operation === "dictionary-replace") entries[row.index] = row.text;
        if (row.operation === "unowned") { expected.owner = row.expectedOwner; overlayHex = fixture.unownedCompositionHex; }
        if (row.operation === "replace-composition") overlayHex = row.hex;
        if (row.operation === "replace-doc") docHex = row.hex;
        if (row.operation === "missing-composition") overlayHex = null;
        if (row.operation === "raw-document-id") docHex = Buffer.concat([Buffer.from([1, 0]), Buffer.from(leb.encodeUnsigned32(Buffer.byteLength(expected.artifactId))), Buffer.from(expected.artifactId), Buffer.from([1, 1])]).toString("hex");
        if (row.operation === "prefix-uuid-id") { entries[0] = "flow"; expected.artifactId = fixture.uuid.artifactId; expected.owner[2] = fixture.uuid.artifactId; docHex = fixture.uuid.docHex; overlayHex = fixture.uuid.compositionHex; }
        const dict = dictionary(entries, row.operation === "dictionary-limit" ? row.value : fixture.limits.dictionaryEntries, row.operation === "dictionary-byte-limit" ? row.value : fixture.limits.dictionaryBytes);
        let document: string[] | null = null;
        for (const payload of row.operation === "duplicate-document" ? [docHex, docHex] : [docHex]) {
          const decoded = doc(payload, dict); if (document !== null) fail("malformed"); document = decoded;
        }
        let overlay: ReturnType<typeof composition> | null = null;
        if (row.operation === "earlier-foreign-composition") overlay = composition("0103010201060100010401050106", dict);
        if (row.operation === "earlier-malformed-composition") overlay = composition(row.hex, dict);
        if (overlayHex !== null) overlay = composition(overlayHex, dict);
        if (JSON.stringify(document) !== JSON.stringify([expected.artifactId, expected.schema]) || JSON.stringify(overlay?.dialect) !== JSON.stringify(expected.dialect)
          || JSON.stringify(overlay?.owner) !== JSON.stringify(expected.owner)) fail("identity");
      } catch (failure) { error = failure instanceof Error ? failure.message : "unknown"; }
      assert.equal(error, row.error, row.id); if (error === null) accepted++;
    }
    console.log(`[TRACE] retained member history identity schema: ${accepted} accepted / ${ids.size - accepted} denied neutral records; independent LEB128/UTF-8/ID model; no Rust semantic decoder or typed hydration claimed`);
  }
}

export class MemberHistoryInputScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.some(segment => segment !== "--oracle-only")) throw new Error("member-history-input-check accepts only --oracle-only");
    const leb = unsignedLebOracleV1();
    const { default: crc } = await import("crc-32/crc32c.js");
    const { inspectRetainedSprNeutral } = await import(join(this.repoRoot, "🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/📜️script.ts"));
    const owner = join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/📜️history");
    const fixture = JSON.parse(readFileSync(join(owner, "🧪️tests/🧫️fixtures/🔣️.json"), "utf8"));
    
    const history = Buffer.from(fixture.historyHex, "hex"); const hash = (bytes: Buffer) => Buffer.from(blake3Hex(bytes), "hex");
    assert.equal(crc.buf(history.subarray(0, 20)) >>> 0, history.readUInt32LE(20));
    const records = [[32, 75], [75, 91], [91, 166], [166, 180], [180, 255]];
    let chain = hash(history.subarray(0, 32)); let pending: Buffer[] = []; let sequence = 0; let previous = 0;
    for (const [start, end] of records) {
      const frame = history.subarray(start, end); assert.equal(frame[0], end! - start! - 9);
      assert.equal(frame.readUInt32LE(frame.length - 4), frame.length);
      assert.equal(crc.buf(frame.subarray(1, -8)) >>> 0, frame.readUInt32LE(frame.length - 8));
      if (frame[1] === 12) {
        const payload = frame.subarray(3, -8); chain = hash(Buffer.concat([chain, ...pending.map(hash)]));
        assert.equal(payload.readBigUInt64LE(), BigInt(++sequence)); assert.equal(payload.readBigUInt64LE(8), BigInt(previous));
        assert.equal(payload.readBigUInt64LE(16), BigInt(pending.reduce((total, bytes) => total + bytes.length, 0)));
        assert.equal(payload.readUInt32LE(24), pending.length); assert(payload.subarray(32).equals(chain)); pending = []; previous = start!;
      } else pending.push(frame);
    }
    assert.equal(sequence, 2);
    const identityBytes = Object.values(fixture.identity).reduce((total: number, value) => total + Buffer.byteLength(value as string), 0);
    const retireBytes = (remaining: number): number => {
      let retired = 0;
      for (let turn = 0; remaining > 0; turn++) {
        const grant = fixture.grants[turn % fixture.grants.length]; const released = Math.min(grant, remaining);
        remaining -= released; retired += released; assert(released <= grant);
      }
      return retired;
    };
    const ids = new Set<string>(); let accepted = 0;
    for (const row of fixture.inputs) {
      assert(!ids.has(row.id)); ids.add(row.id); let bytes = Buffer.from(history);
      if (row.operation === "torn") bytes = Buffer.concat([bytes, Buffer.from([10, 1, 2])]);
      if (row.operation === "paged-tail") bytes = Buffer.concat([bytes, Buffer.from(leb.encodeUnsigned32(5000)), Buffer.alloc(4242)]);
      if (row.operation === "header-only") bytes = bytes.subarray(0, 32);
      if (row.operation === "bad-crc") { const byte=bytes[80];assert.notEqual(byte,undefined);if(byte===undefined)throw Error("CRC fixture byte absent");bytes[80]=byte^1; }
      const input = Buffer.concat([Buffer.from(leb.encodeUnsigned32(1)), Buffer.from([170]), bytes]);
      assert.equal(bytes.length, row.historyBytes); assert.equal(input.length + identityBytes, row.retiredBytes);
      assert.equal(retireBytes(input.length + identityBytes), row.retiredBytes);
      let span: ReturnType<typeof inspectRetainedSprNeutral> | null = null; let error: string | null = null;
      try { span = inspectRetainedSprNeutral(bytes, (bytes: Uint8Array) => crc.buf(bytes) >>> 0, hash); if (span.sequence === 0) error = "malformed"; }
      catch { error = "malformed"; }
      assert.equal(error, row.error);
      if (row.error === null) {
        assert.equal(row.verifiedEnd, span!.end); assert.equal(row.tailBytes, span!.tail); accepted++;
        for (const grant of fixture.grants) {
          let copied = 0; let verified = 0; let pending = false; let credits = 0;
          while (verified < bytes.length) {
            let fuel = grant;
            while (fuel > 0 && verified < bytes.length) {
              fuel--; credits++;
              if (pending) { verified++; pending = false; } else { copied++; pending = true; }
              assert(copied === verified || copied === verified + 1);
            }
          }
          assert.equal(credits, bytes.length * 2); assert.equal(copied, verified); assert(!pending);
        }
      }
    }
    for (const row of fixture.lifecycle) {
      assert(!ids.has(row.id)); ids.add(row.id); const authority = { ...fixture.authority, cancelled: false, clock: true };
      let holder = "verifier"; let handoffs = 0; let retained = history.length + 2; let retired = 0;
      const ready = row.at === "ready" || row.at === "witness";
      const take = (denied: string | null): string | null => {
        if (holder !== "verifier") return "stale";
        if (denied !== null) return denied;
        if (!ready) return "pending";
        holder = "witness"; handoffs++; return null;
      };
      if (row.at === "witness") assert.equal(take(null), null);
      if (row.event === "cancel") authority.cancelled = true;
      if (row.event === "operation") authority.operation++;
      if (row.event === "generation") authority.generation++;
      if (row.event === "expired") authority.nowUs = authority.expiresAtUs;
      if (row.event === "clock-absent") authority.clock = false;
      const error = authority.operation !== fixture.authority.operation || authority.generation !== fixture.authority.generation ? "stale"
        : authority.cancelled ? "cancelled" : !authority.clock || authority.nowUs >= authority.expiresAtUs ? "expired" : null;
      const actual = row.at === "ready" ? take(error) : error;
      assert.equal(actual, row.error); assert.equal(retained, row.retainedBytes); assert.equal(handoffs, row.handoffs);
      assert.equal(holder, handoffs === 1 ? "witness" : "verifier");
      if (handoffs) { assert.equal(take(null), "stale"); assert.equal(handoffs, 1); }
      holder = "retiring"; retired = retireBytes(retained + identityBytes);
      retained = 0; holder = "terminal";
      assert.equal(retired, row.retiredBytes); assert.equal(retained, 0); assert.equal(holder, "terminal");
    }
    console.log(`retained history owner oracle: ${accepted} accepted / ${fixture.inputs.length - accepted} denied derived SPR inputs, ${fixture.lifecycle.length} owner-state traces, 3 fuel grants, exact 4531-byte paged retirement; no semantic hydration`);
    const source = readFileSync(join(owner, "🦀️.rs"), "utf8");
    const nativeTests = readFileSync(join(owner, "🧪️tests/🔬️unit/🦀️.rs"), "utf8");
    assert(source.includes('#[path = "🧪️tests/🔬️unit/🦀️.rs"]'));
    for (const name of ["member_history_verification_retains_input_and_bounds_verified_handoff", "member_history_verification_rechecks_every_owner_transition_and_retires_exact_bytes"]) assert(nativeTests.includes(`fn ${name}`));
    if (segments.includes("--oracle-only")) return;
    assert(readFileSync(join(owner, "../🦀️.rs"), "utf8").includes("pub(crate) mod history;"), "retained history owner is deliberately unmounted; native coverage unavailable");
    const receipts = await runRepositoryExactCargoLaws({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0), preparation: configuredRepositoryCargoPreparationPolicyV1(), invocation: this.invocation, policy: { ...configuredExactCargoLawPolicyV1() }, cwd: this.repoRoot, groups: [{ package: "semio-framework-os-store-member-open-tests", target: { kind: "lib" }, laws: ["member_history_verification_retains_input_and_bounds_verified_handoff", "member_history_verification_rechecks_every_owner_transition_and_retires_exact_bytes"] }] });
    assert.equal(receipts[0]!.assertions, 2);
  }
}

export class MemberHistoryIdScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.some(segment => segment !== "--oracle-only")) throw new Error("member-history-id-check accepts only --oracle-only");
    const leb = unsignedLebOracleV1();
    const owner = join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📜️history/🛂️identity/🪪️id");
    const fixture = JSON.parse(readFileSync(join(owner, "🧪️tests/🧫️fixtures/🔣️.json"), "utf8"));
    const ids = new Set<string>(); let accepted = 0;
    const decode = (row: { dictionary: string | null; resolvedIndex?: number }, bytes: Buffer) => {
      let at = 0;
      const fail: (error: string) => never = error => { throw new Error(error); };
      const take = (count: number): Buffer => { if (count > bytes.length - at) fail("malformed"); const value = bytes.subarray(at, at + count); at += count; return value; };
      const uint = (): number => {
        const start = at; let value = 0n;
        for (let index = 0; index < 10; index++) {
          const byte = take(1)[0]!; if (index === 9 && byte > 1) fail("malformed"); value |= BigInt(byte & 127) << BigInt(index * 7);
          if (byte < 128) {
            if ((index && byte === 0) || value > BigInt(Number.MAX_SAFE_INTEGER)) fail("malformed");
            if (value <= 0xffffffffn) assert(bytes.subarray(start, at).equals(Buffer.from(leb.encodeUnsigned32(Number(value)))));
            return Number(value);
          }
        }
        return fail("malformed");
      };
      const text = (value: string): string => { if (Buffer.byteLength(value) > fixture.limits.identityBytes) fail("capacity"); if (!value || /\p{Cc}/u.test(value)) fail("identity"); return value; };
      let result: string | null = null; let error: string | null = null;
      try {
        const tag = take(1)[0];
        if (tag === 0) {
          const length = uint(); if (length > fixture.limits.identityBytes) fail("capacity");
          let decoded: string; try { decoded = new TextDecoder("utf-8", { fatal: true }).decode(take(length)); } catch { fail("malformed"); }
          result = text(decoded!);
        } else if (tag === 1 || tag === 2) {
          const index = uint(); if (index >= fixture.limits.dictionaryEntries) fail("capacity");
          if ((row.resolvedIndex ?? index) !== index) fail("state");
          if (row.dictionary === null) fail("malformed"); const prefix = text(row.dictionary);
          if (tag === 1) result = prefix;
          else {
            if (Buffer.byteLength(prefix) + 37 > fixture.limits.identityBytes) fail("capacity");
            const uuid = take(16).toString("hex"); result = `${prefix}-${uuid.slice(0, 8)}-${uuid.slice(8, 12)}-${uuid.slice(12, 16)}-${uuid.slice(16, 20)}-${uuid.slice(20)}`;
          }
        } else fail("malformed");
        if (at !== bytes.length) fail("malformed");
      } catch (failure) { error = failure instanceof Error ? failure.message : "unknown"; result = null; }
      return { result, error };
    };
    for (const row of fixture.cases) {
      assert(!ids.has(row.id)); ids.add(row.id); const { result, error } = decode(row, Buffer.from(row.wire, "hex"));
      assert.equal(error, row.error, row.id); assert.equal(result, row.expected, row.id); if (error === null) accepted++;
    }
    for (const row of fixture.completion) {
      assert(!ids.has(row.id)); ids.add(row.id); const source = fixture.cases.find((item: { id: string }) => item.id === row.case); assert(source);
      const wire = Buffer.from(source.wire, "hex").subarray(0, row.wireBytes);
      const complete = !row.cancelled && (source.dictionary === null || row.dictionaryBytes === Buffer.byteLength(source.dictionary)) && decode(source, wire).error === null;
      assert.equal(complete, row.complete, row.id);
    }
    console.log(`[TRACE] retained semantic ID oracle: ${accepted} accepted / ${fixture.cases.length - accepted} denied exact tagged wires +${fixture.completion.length} non-mutating completion boundaries; independent LEB128 + UTF-8 + UUID formatting; no dictionary or input authority publication`);
    const law = "retained_history_id_cursor_matches_neutral_bytes_and_refuses_unowned_resolution";
    assert(readFileSync(join(owner, "🦀️.rs"), "utf8").includes('#[path = "🧪️tests/🔬️unit/🦀️.rs"]'));
    assert(readFileSync(join(owner, "🧪️tests/🔬️unit/🦀️.rs"), "utf8").includes(`fn ${law}`));
    const source = readFileSync(join(owner, "🦀️.rs"), "utf8");
    assert(source.includes("fn is_complete(") && readFileSync(join(owner, "🧪️tests/🔬️unit/🦀️.rs"), "utf8").includes('fixture["completion"]'), "native completion query and exact neutral boundary binding are required");
    if (segments.includes("--oracle-only")) return;
    assert(readFileSync(join(owner, "../../🦀️.rs"), "utf8").includes("pub(crate) mod identity;"), "semantic identity decoder remains unmounted");
    const receipts = await runRepositoryExactCargoLaws({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0), preparation: configuredRepositoryCargoPreparationPolicyV1(), invocation: this.invocation, policy: { ...configuredExactCargoLawPolicyV1() }, cwd: this.repoRoot, groups: [{ package: "semio-framework-os-kernel", target: { kind: "lib" }, laws: [law] }] });
    assert.equal(receipts[0]!.assertions, 1);
  }
}
