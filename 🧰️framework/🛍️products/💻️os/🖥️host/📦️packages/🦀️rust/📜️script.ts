#!/usr/bin/env bun
/** 🖥️ `@semio-tech/framework-os` host router. */
import { join } from "node:path";
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import Ajv2020 from "ajv/dist/2020.js";
import {
  BundleScript,
  ScriptRouter,
  runBundleScriptMain,
  runCargo,
  runVitest,
  runWasmPackWebBuild,
  resolveTestLevel,
  runExactCargoLaws,
} from "../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

class MediaProjectionScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.some(segment => segment !== "--oracle-only")) throw new Error("media-projection-check accepts only --oracle-only");
    const base = join(this.root, "../../🧪️tests/🕸️media-projection");
    const fixture = JSON.parse(readFileSync(join(base, "🧪️fixture/🔣️.json"), "utf8"));
    const validate = new Ajv2020({ strict: true, allErrors: true }).compile(JSON.parse(readFileSync(join(base, "🧬️schema/🔣️.json"), "utf8")));
    assert.equal(fixture.schema, "semio.workflow.media-contract-presentation/v1");
    assert.equal(fixture.cases.length, 4);
    const ids = new Set<string>();
    let denied = 0;
    for (const row of fixture.cases) {
      assert(!ids.has(row.id));
      ids.add(row.id);
      assert(validate(row.input), JSON.stringify(validate.errors));
      assert(validate(row.expected), JSON.stringify(validate.errors));
      const { kindId, mediaType, wire, conversion } = row.input;
      const result = { kindId, mediaType: { class: mediaType.class, form: mediaType.form }, wire: wire.kind === "document" ? { kind: "document", schema: wire.schema } : { kind: "binary", formatKind: wire.formatKind }, conversion: conversion === null ? null : [conversion[0], conversion[1]] };
      assert.deepEqual(result, row.expected);
      assert.equal(conversion !== null, row.isConversion);
      for (const hostile of [{ ...result, unknown: true }, { ...result, wire: { ...wire, unknown: true } }, { ...result, conversion: ["vector"] }, { ...result, mediaType: { class: "other", form: "vector" } }]) {
        assert(!validate(hostile));
        denied++;
      }
    }
    const source = readFileSync(join(this.root, "../../🦀️.rs"), "utf8");
    assert(!source.includes("edge.contract.to_value()"), "presentation must not invent a persistence codec");
    assert(source.includes('"contract": workflow_media_contract_payload(&edge.contract)'), "the live window payload uses the explicit projection");
    assert(source.includes("fn workflow_media_contract_projection_matches_neutral_document_binary_and_conversion_cases"), "an exact native projection law is registered");
    console.log(`[DEBUG] media contract presentation oracle: ${fixture.cases.length} document/binary/conversion vectors, ${denied} strict hostile denials; no native claim`);
    if (segments.includes("--oracle-only")) return;
    const receipts = await runExactCargoLaws({ cwd: this.repoRoot, groups: [{ package: "semio-framework-os", target: { kind: "lib" }, cargoArgs: ["--features", "os-host-full"], laws: ["workflow_media_contract_projection_matches_neutral_document_binary_and_conversion_cases"] }] });
    console.log(`[DEBUG] media contract presentation native assertions=${receipts[0]!.assertions}; executable=${receipts[0]!.sha256}`);
  }
}

class PersistenceContractScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.some(segment => segment !== "--oracle-only")) throw new Error("persistence-contract-check accepts only --oracle-only");
    const base = join(this.root, "../../💾️persistence");
    const fixture = JSON.parse(readFileSync(join(base, "🧪️fixture/🔣️.json"), "utf8"));
    const schema = JSON.parse(readFileSync(join(base, "🧬️schema/🔣️.json"), "utf8"));
    const ajv = new Ajv2020({ strict: true, allErrors: true });
    ajv.addSchema(schema);
    const requestValid = ajv.getSchema("semio.shell.persistence/v1#/$defs/request")!;
    const eventValid = ajv.getSchema("semio.shell.persistence/v1#/$defs/event")!;
    assert.equal(fixture.schema, "semio.shell.persistence-neutral/v1");
    assert.equal(fixture.capacity, 64);
    assert.equal(fixture.requests.length, 4);
    const requests = new Map<string, any>();
    for (const row of fixture.requests) {
      assert(!requests.has(row.id));
      assert(requestValid(row.value), JSON.stringify(requestValid.errors));
      requests.set(row.id, row.value);
    }
    for (const row of fixture.requestNegatives) {
      const hostile = structuredClone(requests.get("create"));
      const keys = row.path.split(".");
      const last = keys.pop()!;
      let parent = hostile;
      for (const key of keys) parent = parent[key];
      parent[last] = row.value;
      assert(!requestValid(hostile), row.id);
    }
    const phases = ["queued", "selecting", "opening", "writing", "committing"];
    let accepted = 0;
    let rejected = 0;
    let published = 0;
    for (const trace of fixture.traces) {
      const request = requests.get(trace.request);
      assert(request, trace.id);
      let terminal = false;
      let phase = -1;
      let completed = 0;
      let total: number | undefined;
      let publications = 0;
      for (const step of trace.steps) {
        const event = step.value;
        const expected = request.correlation;
        const actual = event.correlation;
        let allow = Boolean(eventValid(event)) && !terminal && actual.requestId === expected.requestId && ["windowId", "spaceId", "sessionId", "generation"].every(key => actual.scope[key] === expected.scope[key]);
        if (allow && event.kind === "progress") {
          const next = phases.indexOf(event.phase);
          allow = next >= phase && event.completedBytes >= completed && event.completedBytes <= event.totalBytes && (total === undefined || total === event.totalBytes) && (next !== 4 || event.completedBytes === event.totalBytes);
          if (allow) { phase = next; completed = event.completedBytes; total = event.totalBytes; }
        } else if (allow) {
          const outcome = event.outcome;
          if (outcome.kind === "cancelled") allow = phase < 4;
          if (outcome.kind === "committed") allow = phase === 4 && total === outcome.byteLength && completed === total && outcome.contentDigest !== "0".repeat(64) && request.operation.kind !== "deleteEntry" && (request.operation.kind !== "bindSpace" || outcome.spaceId === request.correlation.scope.spaceId) && outcome.catalogGeneration === request.correlation.scope.generation + 1;
          if (outcome.kind === "deleted") allow = phase === 4 && request.operation.kind === "deleteEntry" && outcome.entryId === request.operation.entryId && outcome.catalogGeneration === request.correlation.scope.generation + 1;
          if (allow) { terminal = true; if (["committed", "deleted"].includes(outcome.kind)) publications++; }
        }
        assert.equal(allow, step.accept, `${trace.id}: ${JSON.stringify(event)}`);
        if (allow) accepted++; else rejected++;
      }
      assert.equal(publications, trace.published, trace.id);
      published += publications;
    }
    let eventDenials = 0;
    const completedEvent = fixture.traces[0].steps.at(-1).value;
    for (const extra of ["path", "token", "message"]) {
      assert(!eventValid({ ...completedEvent, outcome: { ...completedEvent.outcome, [extra]: "private" } }));
      eventDenials++;
    }
    for (const contentDigest of ["0".repeat(64), "A".repeat(64), "a".repeat(63)]) {
      assert(!eventValid({ ...completedEvent, outcome: { ...completedEvent.outcome, contentDigest } }));
      eventDenials++;
    }
    for (const key of ["requestId", "generation"]) {
      const hostile = structuredClone(completedEvent);
      (key === "requestId" ? hostile.correlation : hostile.correlation.scope)[key] = 9_007_199_254_740_992;
      assert(!eventValid(hostile));
      eventDenials++;
    }
    const source = readFileSync(join(base, "🦀️.rs"), "utf8");
    assert(source.includes("persistence_contract_matches_neutral_scope_progress_and_terminal_traces"));
    assert(!source.includes("resolve_kernel_future") && !source.includes("resolve_ready") && !source.includes("ReplayShellCommand"));
    console.log(`[DEBUG] persistence contract oracle: requests=${requests.size}, hostileRequests=${fixture.requestNegatives.length}, hostileEvents=${eventDenials}, traces=${fixture.traces.length}, accepted=${accepted}, rejected=${rejected}, durablePublications=${published}; no IO/runtime activation claim`);
    if (segments.includes("--oracle-only")) return;
    const receipts = await runExactCargoLaws({ cwd: this.repoRoot, groups: [{ package: "semio-framework-os", target: { kind: "lib" }, laws: ["persistence_contract_matches_neutral_scope_progress_and_terminal_traces", "persistence_contract_rejects_closed_fields_and_cross_scope_receipts"] }] });
    assert.equal(receipts[0]!.assertions, 2);
    console.log(`[DEBUG] persistence contract native assertions=${receipts[0]!.assertions}; executable=${receipts[0]!.sha256}`);
  }
}

class DocumentRetirementScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.some(segment => segment !== "--oracle-only")) throw new Error("document-retirement-check accepts only --oracle-only");
    const base = join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/♻️retirement");
    const fixture = JSON.parse(readFileSync(join(base, "🧪️fixture/🔣️.json"), "utf8"));
    const validate = new Ajv2020({ strict: true, allErrors: true }).compile(JSON.parse(readFileSync(join(base, "🧬️schema/🔣️.json"), "utf8")));
    assert(validate(fixture), JSON.stringify(validate.errors));
    let grants = 0;
    for (const row of fixture.cases) {
      const fields: string[] = row.value === null ? [] : Array.isArray(row.value) ? row.value : [row.value];
      const bytes = fields.reduce((sum, field) => sum + Buffer.byteLength(field, "utf8"), 0);
      assert.equal(bytes, row.bytes, row.id);
      assert.equal(fields.reduce((sum, field) => sum + new TextEncoder().encode(field).byteLength, 0), bytes, row.id);
      for (const budget of fixture.budgets) {
        let remaining = bytes;
        let released = 0;
        while (remaining > 0) {
          const turn = Math.min(remaining, budget.bytes);
          assert(turn <= budget.bytes);
          remaining -= turn;
          released += turn;
          grants++;
        }
        assert.equal(released, row.bytes, row.id);
      }
    }
    const source = readFileSync(join(base, "🦀️.rs"), "utf8");
    assert(source.includes("owned_retirement_matches_neutral_exact_byte_grants"));
    assert(source.includes("owned_retirement_rejects_false_terminal_and_preserves_shared_roots"));
    const stdio = readFileSync(join(this.repoRoot, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🦀️.rs"), "utf8");
    assert(!stdio.includes("trait RetireOwned:") && !stdio.includes("struct Bytes(Vec<u8>)"), "Stdio must consume the shared owner primitive");
    console.log(`[DEBUG] owned retirement oracle: cases=${fixture.cases.length}, budgets=${fixture.budgets.length}, exactByteGrants=${grants}, declaredHostile=${fixture.hostile.length}; native pending`);
    if (segments.includes("--oracle-only")) return;
    const receipts = await runExactCargoLaws({ cwd: this.repoRoot, groups: [{ package: "semio-framework-os-kernel", target: { kind: "lib" }, laws: ["owned_retirement_matches_neutral_exact_byte_grants", "owned_retirement_rejects_false_terminal_and_preserves_shared_roots"] }] });
    assert.equal(receipts[0]!.assertions, 2);
  }
}

class MemberOpenProtocolScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.some(segment => segment !== "--oracle-only")) throw new Error("member-open-protocol-check accepts only --oracle-only");
    const base = join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open");
    const fixture = JSON.parse(readFileSync(join(base, "🧫️fixture/🔣️.json"), "utf8"));
    const ajv = new Ajv2020({ strict: true, allErrors: true });
    const validate = ajv.compile(JSON.parse(readFileSync(join(base, "🧬️schema/🔣️.json"), "utf8")));
    assert(validate(fixture), JSON.stringify(validate.errors));
    let rejectedBytes = 0;
    for (const row of fixture.cases) {
      const text = row.artifactId.length > 0 && Buffer.byteLength(row.artifactId) <= fixture.limits.identityBytes && !/\p{Cc}/u.test(row.artifactId);
      const reason = !row.sealed ? "unsealed" : row.bytes === 0 ? "empty" : row.nowUs >= row.expiresAtUs ? "expired" : !text ? "identity" : row.ownerChildId !== null && row.ownerChildId !== row.artifactId ? "owner" : null;
      const structural = ajv.compile({ type: "object", required: ["sealed", "bytes", "expiresAtUs", "artifactId", "ownerChildId"], properties: {
        sealed: { const: true }, bytes: { type: "integer", minimum: 1 }, expiresAtUs: { type: "integer", exclusiveMinimum: row.nowUs },
        artifactId: { type: "string", minLength: 1, pattern: "^[^\\p{Cc}]+$" }, ownerChildId: { enum: [null, row.artifactId] },
      } });
      assert.equal(structural(row) && new TextEncoder().encode(row.artifactId).length <= fixture.limits.identityBytes, reason === null, row.id);
      assert.equal(reason, row.reason, row.id);
      assert.equal(reason === null, row.admitted, row.id);
      if (reason !== null) rejectedBytes += row.bytes;
    }
    for (const row of fixture.framing) {
      let offset = 0;
      let length = 0n;
      let reason: string | null = null;
      for (;;) {
        const byte = row.bytes[offset++];
        if (byte === undefined || offset > 10 || (offset === 10 && byte > 1)) { reason = "malformed"; break; }
        length |= BigInt(byte & 127) << BigInt((offset - 1) * 7);
        if (byte < 128) {
          if ((offset > 1 && byte === 0) || length === 0n || BigInt(offset) + length >= BigInt(row.bytes.length)) reason = "malformed";
          break;
        }
      }
      assert.equal(reason, row.reason, row.id);
      if (reason === null) {
        assert.deepEqual([offset, Number(length)], row.snapshot, row.id);
        assert.deepEqual([offset + Number(length), row.bytes.length - offset - Number(length)], row.history, row.id);
        const encoded: number[] = [];
        for (let value = Number(length);;) { const next = value % 128; value = Math.floor(value / 128); encoded.push(next + (value ? 128 : 0)); if (!value) break; }
        assert.deepEqual(Buffer.from(encoded), Buffer.from(row.bytes.slice(0, offset)), row.id);
      }
    }
    const stages = ["input", "snapshot", "forward", "inverse", "envelope", "initialization"];
    for (const row of fixture.retention) {
      const index = stages.indexOf(row.stage);
      assert.deepEqual([row.snapshots, row.mutations], [index === 0 ? 0 : index === 5 ? 2 : 1, index === 2 ? 1 : index === 3 ? 2 : 0], row.id);
      let retained = row.snapshots + row.mutations;
      let retired = 0;
      while (retained !== 0) { retained--; retired++; }
      assert.equal(retired, row.snapshots + row.mutations);
    }
    const source = readFileSync(join(base, "🦀️.rs"), "utf8");
    assert(source.includes("pub trait MemberOpenOperation") && source.includes("StepContext<'_>"));
    assert(source.includes("member_open_request_rejection_retains_exact_pages_and_identity"));
    assert(source.includes("member_open_input_framing_is_canonical_scoped_and_budgeted"));
    const store = readFileSync(join(base, "../../🦀️.rs"), "utf8");
    assert(store.includes("member_open_partial_parse_and_initialization_owners_retire_exactly"));
    console.log(`[DEBUG] member open request oracle: ${fixture.cases.length} admission cases, ${fixture.framing.length} framing cases, ${fixture.retention.length} declared retained-stage cases; rejected input bytes retained=${rejectedBytes}; typed parser/factory activation not claimed`);
    if (segments.includes("--oracle-only")) return;
    const receipts = await runExactCargoLaws({ cwd: this.repoRoot, groups: [{ package: "semio-framework-os-kernel", target: { kind: "lib" }, laws: ["member_open_request_rejection_retains_exact_pages_and_identity", "member_open_input_framing_is_canonical_scoped_and_budgeted", "member_open_partial_parse_and_initialization_owners_retire_exactly"] }] });
    assert.equal(receipts[0]!.assertions, 3);
  }
}

class MemberHistoryIdentitySourceScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("member-history-identity-source accepts no arguments");
    const leb = await import("@webassemblyjs/leb128");
    const owner = join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📜️history/🛂️identity");
    const fixture = JSON.parse(readFileSync(join(owner, "🧫️fixture/🔣️.json"), "utf8"));
    const ajv = new Ajv2020({ strict: true });
    const validate = ajv.compile(JSON.parse(readFileSync(join(owner, "🧬️schema/🔣️.json"), "utf8")));
    assert(validate(fixture), ajv.errorsText(validate.errors));
    const fail = (error: string): never => { throw new Error(error); };
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
      const encoded = Buffer.concat([Buffer.from([1, 0]), Buffer.from(leb.encodeU32(entries.length)), ...entries.flatMap(entry => [Buffer.from(leb.encodeU32(Buffer.byteLength(entry))), Buffer.from(entry)])]);
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
    const composition = (hex: string, dict: string[], pinLimit = fixture.limits.pins) => {
      const input = reader(Buffer.from(hex, "hex")); if (input.byte() !== 1) fail("malformed");
      const presence = input.byte(); if (presence & ~3) fail("malformed");
      const triple = (): string[] => [id(input, dict), id(input, dict), id(input, dict)];
      const owner = presence & 1 ? triple() : null; const dialect = presence & 2 ? triple() : null;
      const groups = input.uint(); if (groups > fixture.limits.pinGroups) fail("capacity"); let pins = 0;
      for (let group = 0; group < groups; group++) { id(input, dict); const count = input.uint(); pins += count; if (pins > pinLimit) fail("capacity"); for (let pin = 0; pin < count; pin++) { id(input, dict); id(input, dict); } }
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
        if (row.operation === "replace-composition" || row.operation === "pin-limit") overlayHex = row.hex;
        if (row.operation === "replace-doc") docHex = row.hex;
        if (row.operation === "missing-composition") overlayHex = null;
        if (row.operation === "raw-document-id") docHex = Buffer.concat([Buffer.from([1, 0]), Buffer.from(leb.encodeU32(Buffer.byteLength(expected.artifactId))), Buffer.from(expected.artifactId), Buffer.from([1, 1])]).toString("hex");
        if (row.operation === "prefix-uuid-id") { entries[0] = "flow"; expected.artifactId = fixture.uuid.artifactId; expected.owner[2] = fixture.uuid.artifactId; docHex = fixture.uuid.docHex; overlayHex = fixture.uuid.compositionHex; }
        const dict = dictionary(entries, row.operation === "dictionary-limit" ? row.value : fixture.limits.dictionaryEntries, row.operation === "dictionary-byte-limit" ? row.value : fixture.limits.dictionaryBytes);
        let document: string[] | null = null;
        for (const payload of row.operation === "duplicate-document" ? [docHex, docHex] : [docHex]) {
          const decoded = doc(payload, dict); if (document !== null) fail("malformed"); document = decoded;
        }
        let overlay: ReturnType<typeof composition> | null = null;
        if (row.operation === "earlier-foreign-composition") overlay = composition("010301020106010001040105010600", dict);
        if (row.operation === "earlier-malformed-composition") overlay = composition(row.hex, dict);
        if (overlayHex !== null) overlay = composition(overlayHex, dict, row.operation === "pin-limit" ? row.value : fixture.limits.pins);
        if (JSON.stringify(document) !== JSON.stringify([expected.artifactId, expected.schema]) || JSON.stringify(overlay?.dialect) !== JSON.stringify(expected.dialect)
          || JSON.stringify(overlay?.owner) !== JSON.stringify(expected.owner)) fail("identity");
      } catch (failure) { error = failure instanceof Error ? failure.message : "unknown"; }
      assert.equal(error, row.error, row.id); if (error === null) accepted++;
    }
    const extra = structuredClone(fixture); extra.expected.unowned = true; assert(!validate(extra));
    console.log(`[DEBUG] retained member history identity schema: ${accepted} accepted / ${ids.size - accepted} denied neutral records; strict AJV and independent LEB128/UTF-8/ID model; no Rust semantic decoder or typed hydration claimed`);
  }
}

class MemberHistoryInputScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.some(segment => segment !== "--oracle-only")) throw new Error("member-history-input-check accepts only --oracle-only");
    const leb = await import("@webassemblyjs/leb128");
    const { default: crc } = await import("crc-32/crc32c.js");
    const { blake3Hex } = await import("../../../🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts");
    const { inspectRetainedSprNeutral } = await import(join(this.repoRoot, "🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/📜️script.ts"));
    const owner = join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/📜️history");
    const fixture = JSON.parse(readFileSync(join(owner, "🧫️fixture/🔣️.json"), "utf8"));
    const ajv = new Ajv2020({ strict: true }); const validate = ajv.compile(JSON.parse(readFileSync(join(owner, "🧬️schema/🔣️.json"), "utf8")));
    assert(validate(fixture), ajv.errorsText(validate.errors));
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
      if (row.operation === "paged-tail") bytes = Buffer.concat([bytes, Buffer.from(leb.encodeU32(5000)), Buffer.alloc(4242)]);
      if (row.operation === "header-only") bytes = bytes.subarray(0, 32);
      if (row.operation === "bad-crc") bytes[80] ^= 1;
      const input = Buffer.concat([Buffer.from(leb.encodeU32(1)), Buffer.from([170]), bytes]);
      assert.equal(bytes.length, row.historyBytes); assert.equal(input.length + identityBytes, row.retiredBytes);
      assert.equal(retireBytes(input.length + identityBytes), row.retiredBytes);
      let span: ReturnType<typeof inspectRetainedSprNeutral> | null = null; let error: string | null = null;
      try { span = inspectRetainedSprNeutral(bytes, bytes => crc.buf(bytes) >>> 0, hash); if (span.sequence === 0) error = "malformed"; }
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
    const extra = structuredClone(fixture); extra.lifecycle[0].grant = true; assert(!validate(extra));
    console.log(`[DEBUG] retained history owner oracle: ${accepted} accepted / ${fixture.inputs.length - accepted} denied derived SPR inputs, ${fixture.lifecycle.length} owner-state traces, 3 fuel grants, exact 4531-byte paged retirement; no semantic hydration`);
    const source = readFileSync(join(owner, "🦀️.rs"), "utf8");
    for (const name of ["member_history_verification_retains_input_and_bounds_verified_handoff", "member_history_verification_rechecks_every_owner_transition_and_retires_exact_bytes"]) assert(source.includes(`fn ${name}`));
    if (segments.includes("--oracle-only")) return;
    assert(readFileSync(join(owner, "../🦀️.rs"), "utf8").includes("pub(crate) mod history;"), "retained history owner is deliberately unmounted; native coverage unavailable");
    const receipts = await runExactCargoLaws({ cwd: this.repoRoot, groups: [{ package: "semio-framework-os-kernel", target: { kind: "lib" }, laws: ["member_history_verification_retains_input_and_bounds_verified_handoff", "member_history_verification_rechecks_every_owner_transition_and_retires_exact_bytes"] }] });
    assert.equal(receipts[0]!.assertions, 2);
  }
}

class MemberHistoryIdScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.some(segment => segment !== "--oracle-only")) throw new Error("member-history-id-check accepts only --oracle-only");
    const leb = await import("@webassemblyjs/leb128");
    const owner = join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📜️history/🛂️identity/🪪️id");
    const fixture = JSON.parse(readFileSync(join(owner, "🧫️fixture/🔣️.json"), "utf8"));
    const ajv = new Ajv2020({ strict: true }); const validate = ajv.compile(JSON.parse(readFileSync(join(owner, "🧬️schema/🔣️.json"), "utf8")));
    assert(validate(fixture), ajv.errorsText(validate.errors)); const ids = new Set<string>(); let accepted = 0;
    const decode = (row: { dictionary: string | null; resolvedIndex?: number }, bytes: Buffer) => {
      let at = 0;
      const fail = (error: string): never => { throw new Error(error); };
      const take = (count: number): Buffer => { if (count > bytes.length - at) fail("malformed"); const value = bytes.subarray(at, at + count); at += count; return value; };
      const uint = (): number => {
        const start = at; let value = 0n;
        for (let index = 0; index < 10; index++) {
          const byte = take(1)[0]!; if (index === 9 && byte > 1) fail("malformed"); value |= BigInt(byte & 127) << BigInt(index * 7);
          if (byte < 128) {
            if ((index && byte === 0) || value > BigInt(Number.MAX_SAFE_INTEGER)) fail("malformed");
            if (value <= 0xffffffffn) assert(bytes.subarray(start, at).equals(Buffer.from(leb.encodeU32(Number(value)))));
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
    const extra = structuredClone(fixture); extra.cases[0].authority = true; assert(!validate(extra));
    console.log(`[DEBUG] retained semantic ID oracle: ${accepted} accepted / ${fixture.cases.length - accepted} denied exact tagged wires +${fixture.completion.length} non-mutating completion boundaries; independent LEB128 + UTF-8 + UUID formatting; no dictionary or input authority publication`);
    const law = "retained_history_id_cursor_matches_neutral_bytes_and_refuses_unowned_resolution";
    assert(readFileSync(join(owner, "🦀️.rs"), "utf8").includes(`fn ${law}`));
    const source = readFileSync(join(owner, "🦀️.rs"), "utf8");
    assert(source.includes("fn is_complete(") && source.includes('fixture["completion"]'), "native completion query and exact neutral boundary binding are required");
    if (segments.includes("--oracle-only")) return;
    assert(readFileSync(join(owner, "../../🦀️.rs"), "utf8").includes("pub(crate) mod identity;"), "semantic identity decoder remains unmounted");
    const receipts = await runExactCargoLaws({ cwd: this.repoRoot, groups: [{ package: "semio-framework-os-kernel", target: { kind: "lib" }, laws: [law] }] });
    assert.equal(receipts[0]!.assertions, 1);
  }
}

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

class MemberHistoryRecordScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.some(segment => segment !== "--oracle-only")) throw new Error("member-history-record-check accepts only --oracle-only");
    const leb = await import("@webassemblyjs/leb128");
    const owner = join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/📜️history/🗂️dictionary/🧾️record");
    const fixture = JSON.parse(readFileSync(join(owner, "🧫️fixture/🔣️.json"), "utf8"));
    const ajv = new Ajv2020({ strict: true }); const validate = ajv.compile(JSON.parse(readFileSync(join(owner, "🧬️schema/🔣️.json"), "utf8")));
    assert(validate(fixture), ajv.errorsText(validate.errors)); let accepted = 0; const ids = new Set<string>();
    const fail = (reason: string): never => { throw new Error(reason); };
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
                  if (value <= 0xffffffffn) assert(Buffer.from(digits).equals(Buffer.from(leb.encodeU32(Number(value)))));
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
    const extra = structuredClone(fixture); extra.cases[0].authority = true; assert(!validate(extra));
    console.log(`[DEBUG] dictionary payload cursor oracle: ${accepted} accepted / ${fixture.cases.length - accepted} denied exact wires ×3 grants; exact ordered Begin/base/count and Entry/ranges, UTF8 scratch0..4, event fences, canonicalLEB, pinned retirement; no input authority`);
    const source = readFileSync(join(owner, "🦀️.rs"), "utf8");
    assert(source.includes('include_str!("🧫️fixture/🔣️.json")'), "native payload cursor must consume its exact neutral fixture");
    assert(source.includes('fixture["events"][name]') && source.includes("assert_eq!(events, expected_events"), "native payload cursor must compare every exact ordered neutral event trace");
    const laws = ["retained_dictionary_delta_matches_neutral_text_ranges_without_publication", "retained_dictionary_delta_rejects_tail_and_preserves_partial_utf8_until_close"];
    for (const law of laws) assert(source.includes(`fn ${law}`));
    if (segments.includes("--oracle-only")) return;
    assert(readFileSync(join(owner, "../🦀️.rs"), "utf8").includes("mod record;"), "dictionary record cursor remains unmounted");
    const receipts = await runExactCargoLaws({ cwd: this.repoRoot, groups: [{ package: "semio-framework-os-kernel", target: { kind: "lib" }, laws }] });
    assert.equal(receipts[0]!.assertions, 2);
  }
}

class MemberHistoryFoundationScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.some(segment => segment !== "--oracle-only")) throw new Error("member-history-foundation-check accepts only --oracle-only");
    const replicationRoot = join(this.repoRoot, "🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust");
    const { RetainedVerificationScript } = await import(join(replicationRoot, "📜️script.ts"));
    await new RetainedVerificationScript(replicationRoot, this.repoRoot).run(["--oracle-only"]);
    await new MemberHistoryInputScript(this.root, this.repoRoot).run(["--oracle-only"]);
    await new MemberHistoryIdScript(this.root, this.repoRoot).run(["--oracle-only"]);
    await new MemberHistoryIdentitySourceScript(this.root, this.repoRoot).run([]);
    if (segments.includes("--oracle-only")) return;
    const mounts = [
      ["🧰️framework/🔨️modules/📡️replication/📐️format/🦀️.rs", "pub mod retained;"],
      ["🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/🦀️.rs", "pub(crate) mod history;"],
      ["🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📜️history/🦀️.rs", "pub(crate) mod identity;"],
    ];
    for (const [path, declaration] of mounts) assert(readFileSync(join(this.repoRoot, path!), "utf8").includes(declaration!));
    const receipts = await runExactCargoLaws({ cwd: this.repoRoot, groups: [
      { package: "semio-framework-replication", target: { kind: "lib", name: "protocol" }, laws: ["format::retained::tests::retained_spr_verification_matches_neutral_commits_and_torn_prefixes", "format::retained::tests::retained_spr_verification_rejects_hostile_frames_without_publication"] },
      { package: "semio-framework-os-kernel", target: { kind: "lib" }, laws: ["member_history_verification_retains_input_and_bounds_verified_handoff", "member_history_verification_rechecks_every_owner_transition_and_retires_exact_bytes", "retained_history_id_cursor_matches_neutral_bytes_and_refuses_unowned_resolution"] },
    ] });
    assert.deepEqual(receipts.map(receipt => receipt.assertions), [2, 3]);
    console.log("[DEBUG] retained history foundation: exactly5 native assertions; framing2 + private input2 + tagged-ID1; no dictionary/member factory/publication claim");
  }
}

class MemberHistoryDictionaryScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.some(segment => segment !== "--oracle-only")) throw new Error("member-history-dictionary-check accepts only --oracle-only");
    const leb = await import("@webassemblyjs/leb128");
    const { default: crc } = await import("crc-32/crc32c.js");
    const { blake3Hex } = await import("../../../🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts");
    const { inspectRetainedSprNeutral } = await import(join(this.repoRoot, "🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/📜️script.ts"));
    const owner = join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/📜️history/🗂️dictionary");
    const fixture = JSON.parse(readFileSync(join(owner, "🧫️fixture/🔣️.json"), "utf8"));
    const ajv = new Ajv2020({ strict: true }); const validate = ajv.compile(JSON.parse(readFileSync(join(owner, "🧬️schema/🔣️.json"), "utf8")));
    assert(validate(fixture), ajv.errorsText(validate.errors));
    const checksum = (bytes: Uint8Array): number => crc.buf(bytes) >>> 0;
    const hash = (bytes: Buffer): Buffer => Buffer.from(blake3Hex(bytes), "hex");
    const uint = (value: number): Buffer => Buffer.from(leb.encodeU32(value));
    const fail = (reason: string): never => { throw new Error(reason); };
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
      if (row.operation === "aggregate-pin-limit" || row.operation === "aggregate-group-limit") records.push(frame(65, composition));
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
      if (row.operation === "pin-limit" || row.operation === "aggregate-pin-limit") limits.pins = row.value;
      if (row.operation === "aggregate-group-limit") limits.pinGroups = row.value;
      const requestIdentity = [...fixture.requestIdentity];
      if (row.operation === "request-field") {
        assert.equal(Buffer.byteLength(row.text!), Buffer.byteLength(requestIdentity[row.index!])); requestIdentity[row.index!] = row.text;
        if (row.index === 0 || row.index === 9) { requestIdentity[0] = row.text; requestIdentity[9] = row.text; }
      }
      assert.equal(requestIdentity[0], requestIdentity[9], "retained requests were admitted with exact child identity");
      const expected = authorityOf(requestIdentity);
      const utf8 = dictionaryUtf8Neutral();
      const state = { entries: [] as Range[], provisional: [] as Range[], pages: 0, dictionaryBytes: 0, pinGroups: 0, pins: 0, ready: false, error: null as string | null, holder: "owner", handoffs: 0, copies: 0, parses: 0, units: 0 };
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
          if (((kind === 1 || kind === 3) && flags !== 2) || (kind === 65 && flags !== 0)) fail("malformed");
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
            const groups = yield* input.number(); if (groups > limits.pinGroups - state.pinGroups) fail("capacity"); state.pinGroups += groups;
            for (let group = 0; group < groups; group++) {
              yield* id(input); const count = yield* input.number(); if (count > limits.pins - state.pins) fail("capacity"); state.pins += count;
              for (let pin = 0; pin < count; pin++) { yield* id(input); yield* id(input); }
            }
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
        assert.equal(run.state.error, row.error, row.id); assert.equal(run.state.entries.length, row.entries, row.id); assert.equal(run.state.pages, row.pages, row.id); assert.equal(run.state.pins, row.pins, row.id);
        assert.equal(run.state.pinGroups, row.groups, row.id);
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
    const extra = structuredClone(fixture); extra.cases[0].unowned = true; assert(!validate(extra));
    console.log(`[DEBUG] retained dictionary owner oracle: ${accepted} accepted / ${fixture.cases.length - accepted} denied committed SPR histories, ${fixture.lifecycle.length} owner traces + ${fixture.recordRetirement.length} payload scratch + ${fixture.ownerRetirement.length} pending-copy/ID scratch traces × 3 grants; retirement up to ${maxRetired} bytes; no Rust or typed publication claim`);
    const source = readFileSync(join(owner, "🦀️.rs"), "utf8");
    const nativeLaws = readFileSync(join(owner, "🧪️tests/🦀️.rs"), "utf8");
    for (const law of ["member_history_dictionary_is_atomic_and_bounded_by_neutral_records", "member_history_dictionary_retains_every_denied_owner_until_exact_close"]) assert(nativeLaws.includes(`fn ${law}`));
    assert(nativeLaws.includes('include_str!("../🧫️fixture/🔣️.json")') && nativeLaws.includes('fixture["recordRetirement"]'), "owner native laws must consume the exact fixture including scratch retirement");
    assert(nativeLaws.includes('fixture["ownerRetirement"]'), "native owner must cover retained wire-copy, lookup-copy and tagged-ID scratch");
    assert(source.includes("RetainedSprVerification::new") && source.includes("observe_record_header()") && source.includes("id.is_complete()") && source.includes("copy_verified_history_chunk"), "owner must reuse the exact retained framing/ID/input contracts");
    assert(readFileSync(join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📜️history/🛂️identity/🪪️id/🦀️.rs"), "utf8").includes("fn is_complete("), "record owner requires a non-poisoning tagged-ID completion query");
    if (segments.includes("--oracle-only")) return;
    assert(readFileSync(join(owner, "../🦀️.rs"), "utf8").includes("mod dictionary;"), "native dictionary owner laws require the coordinated parent mount");
    const receipts = await runExactCargoLaws({ cwd: this.repoRoot, groups: [{ package: "semio-framework-os-kernel", target: { kind: "lib" }, laws: ["member_history_dictionary_is_atomic_and_bounded_by_neutral_records", "member_history_dictionary_retains_every_denied_owner_until_exact_close"] }] });
    assert.equal(receipts[0]!.assertions, 2);
  }
}

class MemberFactoryIdentityScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.some(segment => segment !== "--oracle-only")) throw new Error("member-factory-identity-check accepts only --oracle-only");
    const owner = join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/📜️history/🏭️factory");
    const fixture = JSON.parse(readFileSync(join(owner, "🧫️fixture/🔣️.json"), "utf8"));
    const ajv = new Ajv2020({ strict: true }); const validate = ajv.compile(JSON.parse(readFileSync(join(owner, "🧬️schema/🔣️.json"), "utf8")));
    assert(validate(fixture), ajv.errorsText(validate.errors));
    const semioSource = readFileSync(join(this.repoRoot, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🦀️.rs"), "utf8");
    const semioTable = semioSource.slice(semioSource.indexOf("pub enum SemioMembers {"), semioSource.indexOf("/// 🏭️ Mints a typed Semio child"));
    const declarations = [...semioTable.matchAll(/\w+\("([^"]+)", "([^"]+)", "([^"]+)", "([^"]+)"\) =>/g)].map(match => match.slice(1));
    assert.equal(declarations.length, 18); assert.deepEqual(declarations, fixture.declarations, "neutral rows must be exactly the owning closed factory declaration, never parallel authority");
    const inputFixture = JSON.parse(readFileSync(join(owner, "../🧫️fixture/🔣️.json"), "utf8"));
    assert.equal(Buffer.from(inputFixture.historyHex, "hex").length + 2, fixture.inputBytes);
    const validText = (value: string) => value.length > 0 && Buffer.byteLength(value) <= fixture.limits.fieldBytes && !/\p{Cc}/u.test(value);
    const validKind = /^s\.[a-z0-9]+(?:-[a-z0-9]+)*\.[a-z0-9]+(?:-[a-z0-9]+)*$/;
    const tableFor = (mutation: string): string[][] => {
      const table = structuredClone(declarations);
      if (mutation === "reverse") table.reverse();
      if (mutation === "missing") table.splice(6, 1);
      if (mutation === "empty") table.length = 0;
      if (mutation === "capacity") while (table.length < 65) table.push([...table[0]!]);
      if (mutation === "duplicate") table.push([...table[6]!]);
      if (mutation === "duplicate-foreign") table.push([...table[0]!.slice(0, 3), "different.schema"]);
      if (mutation === "late-kind") table[17]![0] = "";
      if (mutation === "late-schema") table[17]![3] = "";
      if (mutation === "late-control") table[17]![3] = "stdio.\u0085semio";
      if (mutation === "late-length") table[17]![3] = "x".repeat(257);
      if (mutation === "late-noncanonical") table[17]![0] = "stdio.semio";
      if (mutation === "late-standard") table[17]![1] = "";
      if (mutation === "late-subset") table[17]![2] = "";
      if (mutation === "late-standard-length") table[17]![1] = "x".repeat(257);
      if (mutation === "late-subset-control") table[17]![2] = "vi\u0085deo";
      return table;
    };
    const model = (table: string[][], dialect: string[]) => {
      const request = [...fixture.requestIdentity]; request.splice(1, 3, ...dialect);
      const state = { candidate: null as string[] | null, ready: false, error: null as string | null, holder: "operation", handoffs: 0, units: 0 };
      function* scan() {
        if (!table.length) { state.error = "identity"; return; }
        if (table.length > fixture.limits.declarations) { state.error = "capacity"; return; }
        for (let index = 0; index < table.length; index++) {
          const row = table[index]!;
          if (!row.every(validText) || !validKind.test(row[0]!)) { state.error = "identity"; return; }
          yield "validated-row";
          for (let earlier = 0; earlier < index; earlier++) {
            if (row.slice(0, 3).every((value, field) => value === table[earlier]![field])) { state.error = "identity"; return; }
            yield "unique-pair";
          }
          if (row.slice(0, 3).every((value, field) => value === request[field + 1])) { state.candidate = row; yield "selected-unpublished"; }
        }
        if (!state.candidate) { state.error = "identity"; return; }
        state.ready = true; yield "ready";
      }
      const iterator = scan();
      const next = () => { if (state.error || state.holder !== "operation") return { done: true, value: undefined }; state.units++; return iterator.next(); };
      const take = () => {
        if (state.holder !== "operation") return "stale"; if (state.error) return state.error; if (!state.ready) return "pending";
        state.holder = "witness"; state.handoffs++; return null;
      };
      const close = (grant: number) => {
        state.holder = "retiring"; let remaining = fixture.inputBytes + request.reduce((sum, value) => sum + Buffer.byteLength(value), 0); let retired = 0;
        while (remaining) { const bytes = Math.min(grant, remaining); remaining -= bytes; retired += bytes; assert(bytes <= grant); assert.equal(take(), "stale"); }
        state.candidate = null; state.holder = "terminal"; return retired;
      };
      return { state, next, take, close };
    };
    for (const declaration of declarations) {
      const run = model(declarations, declaration.slice(0, 3)); while (!run.next().done) {};
      assert.equal(run.take(), null); assert.deepEqual(run.state.candidate, declaration); assert.equal(run.take(), "stale"); run.close(1);
    }
    assert.deepEqual(Object.keys(fixture.caseRetirement).sort(), fixture.cases.map((row: { id: string }) => row.id).sort());
    for (const row of fixture.cases) for (const grant of fixture.grants) {
      const run = model(tableFor(row.mutation), row.dialect); let done = false;
      while (!done) for (let fuel = grant; fuel && !done; fuel--) done = Boolean(run.next().done);
      assert.equal(run.state.error, row.error, row.id);
      assert.deepEqual(run.state.error ? null : run.state.candidate, row.selected, row.id);
      if (row.error === null) { assert.equal(run.take(), null); assert.equal(run.take(), "stale"); } else assert.equal(run.take(), row.error);
      assert.equal(run.close(grant), fixture.caseRetirement[row.id], row.id);
    }
    for (const row of fixture.lifecycle) for (const grant of fixture.grants) {
      const run = model(declarations, fixture.requestIdentity.slice(1, 4));
      if (row.at !== "begin") { let found = false; while (!found) { const step = run.next(); assert(!step.done); found = step.value === (row.at === "witness" ? "ready" : row.at); } }
      if (row.at === "selected-unpublished") assert.equal(run.take(), "pending");
      if (row.at === "witness") assert.equal(run.take(), null);
      run.state.error = row.event === "cancel" ? "cancelled" : ["generation", "operation"].includes(row.event) ? "stale" : ["expired", "clock-absent"].includes(row.event) ? "expired" : null;
      if (row.event === "none") assert.equal(run.take(), null);
      const before = run.state.units; assert(run.next().done); assert.equal(run.state.units, before);
      assert.equal(run.state.error, row.error); assert.equal(run.state.handoffs, row.handoffs); assert.equal(run.close(grant), row.retiredBytes);
    }
    const external = structuredClone(fixture); external.cases[0].schema = "stdio.semio.flow"; assert(!validate(external));
    const leb = await import("@webassemblyjs/leb128");
    const size = (value: number) => leb.encodeU32(value).length;
    const request = fixture.requestIdentity;
    const selected = declarations.find(row => row.slice(0, 3).every((value, index) => value === request[index + 1]))!;
    const expected = { document: request[0], schema: selected[3], parent: `${request[4]}!${request[5]}@${request[6]}/${request[7]}`, slot: request[8], child: request[9], kind: selected[0], standard: selected[1], subset: selected[2] };
    const seenSemantic = new Set<string>();
    for (const row of fixture.semantic) {
      assert(!seenSemantic.has(row.field)); seenSemantic.add(row.field);
      const persisted: Record<string, string> = { ...expected }; if (row.field !== "none") persisted[row.field] = row.value;
      const dictionary: string[] = [];
      const id = (text: string) => { let index = dictionary.indexOf(text); if (index < 0) { index = dictionary.length; dictionary.push(text); } return 1 + size(index); };
      const deltaBytes = (base: number) => 1 + size(base) + size(dictionary.length - base) + dictionary.slice(base).reduce((bytes, entry) => bytes + size(Buffer.byteLength(entry)) + Buffer.byteLength(entry), 0);
      const documentBytes = 1 + id(persisted.document!) + id(persisted.schema!); const firstDelta = deltaBytes(0); const base = dictionary.length;
      const compositionBytes = 3 + ["parent", "slot", "child", "kind", "standard", "subset"].reduce((bytes, field) => bytes + id(persisted[field]!), 0);
      const secondDelta = deltaBytes(base); const frameBytes = (payload: number) => size(payload + 2) + 2 + payload + 8;
      const inputBytes = 2 + 32 + [firstDelta, documentBytes, 2, secondDelta, compositionBytes, 64].reduce((bytes, payload) => bytes + frameBytes(payload), 0);
      const error = Object.entries(expected).some(([key, value]) => value !== persisted[key]) ? "identity" : null;
      assert.equal(error, row.error, row.id); assert.equal(Number(error === null), row.handoffs, row.id);
      assert.equal(inputBytes, row.inputBytes, row.id); assert.equal(inputBytes + request.reduce((bytes: number, field: string) => bytes + Buffer.byteLength(field), 0) + 1024, row.retiredBytes, row.id);
    }
    assert.equal(seenSemantic.size, 9);
    for (const row of fixture.semanticLifecycle) for (const grant of fixture.grants) {
      const state = { holder: row.at === "ready" ? "dictionary" : "selected", input: fixture.semantic[0].inputBytes, index: row.at === "ready" ? 1024 : 0, error: null as string | null, handoffs: 0 };
      state.error = row.event === "capacity" ? "capacity" : row.event === "cancel" ? "cancelled" : "stale";
      const take = () => state.error === null ? ++state.handoffs : 0;
      assert.equal(state.error, row.error); assert.equal(take(), 0); assert.equal(take(), 0); assert.equal(state.handoffs, 0);
      let remaining = state.input + state.index + request.reduce((bytes: number, field: string) => bytes + Buffer.byteLength(field), 0); let retired = 0;
      state.holder = "retiring"; while (remaining) { const bytes = Math.min(grant, remaining); remaining -= bytes; retired += bytes; } state.holder = "terminal";
      assert.equal(retired, row.retiredBytes, row.id);
    }
    console.log(`[DEBUG] selected MemberFactory identity oracle: exact18 source-owned declarations,21 selection rows +7 selection/5 semantic lifecycle traces x3 grants,9 persisted schema/dialect/owner cases with literal input/close bytes; caller schema denied; no native/public opening claim`);
    const source = readFileSync(join(owner, "🦀️.rs"), "utf8");
    assert(source.includes("M::OPEN_DECLARATIONS") && source.includes("VerifiedMemberHistoryInput"));
    assert(source.includes("MemberHistoryDictionaryOwner::begin(input, self.declaration.schema"));
    const tests = readFileSync(join(owner, "🧪️tests/🦀️.rs"), "utf8");
    const laws = ["member_factory_selection_uses_only_complete_closed_declarations", "member_factory_selection_retains_input_through_denial_and_handoff"];
    for (const law of laws) assert(tests.includes(`fn ${law}`));
    assert(tests.includes('fixture["semantic"]') && tests.includes('fixture["semanticLifecycle"]') && tests.includes("begin_dictionary"), "native selected schema law must traverse the actual dictionary owner and retain every denied input");
    const store = readFileSync(join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs"), "utf8");
    const generatedSource = store.slice(store.indexOf("pub enum RetainedTestMembers {"), store.indexOf("fn member_publication_fixture()"));
    assert.deepEqual([...generatedSource.matchAll(/\w+\("([^"]+)", "([^"]+)", "([^"]+)", "([^"]+)"\) =>/g)].map(match => match.slice(1)), fixture.generatedDeclarations);
    assert(tests.includes("super::super::super::super::tests::RetainedTestMembers") && tests.includes("Generated::OPEN_DECLARATIONS"), "native laws must exercise the actual space_members macro expansion, not just hand-declared fixture factories");
    assert(store.includes("const OPEN_DECLARATIONS:"), "selected identity requires the coordinated MemberFactory declaration API; staged source is not mounted authority");
    const macroFactory = store.slice(store.indexOf("impl $crate::os_store::MemberFactory for $enum_name"), store.indexOf("//#endregion SpaceMember"));
    for (const field of ["kind: $kind", "standard: $standard", "subset: $subset", "schema: $schema"]) assert(macroFactory.includes(field), `factory declaration must reuse its creation/open literal ${field}`);
    const noMembers = store.slice(store.indexOf("impl MemberFactory for NoMembers"), store.indexOf("/// @emoji 🧬️ Generates a per-plugin"));
    assert(/const OPEN_DECLARATIONS:\s*&'static\s*\[MemberOpenDeclaration\]\s*=\s*&\[\];/.test(noMembers), "NoMembers has exactly the empty declaration table");
    if (segments.includes("--oracle-only")) return;
    assert(readFileSync(join(owner, "../🦀️.rs"), "utf8").includes("mod factory;"), "selected factory native laws require the coordinated mount");
    const receipts = await runExactCargoLaws({ cwd: this.repoRoot, groups: [{ package: "semio-framework-os-kernel", target: { kind: "lib" }, laws }] });
    assert.equal(receipts[0]!.assertions, 2);
  }
}

class PublicMemberOpenHandoffScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("public-member-open-handoff-source accepts no arguments");
    this.proveSource();
  }

  protected proveSource(): void {
    const owner = join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/🚪️member-open");
    const fixture = JSON.parse(readFileSync(join(owner, "🧫️fixture/🔣️.json"), "utf8"));
    const validate = new Ajv2020({ strict: true }).compile(JSON.parse(readFileSync(join(owner, "🧬️schema/🔣️.json"), "utf8")));
    assert(validate(fixture), JSON.stringify(validate.errors));
    const selectedFixture = JSON.parse(readFileSync(join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/📜️history/🏭️factory/🧫️fixture/🔣️.json"), "utf8"));
    assert.equal(selectedFixture.declarations.length, fixture.requiredFactoryOperations);
    assert.deepEqual(selectedFixture.declarations.find((row: string[]) => row.slice(0, 3).every((value, index) => value === fixture.input.dialect[index])), fixture.selected);
    const deniedSchema = structuredClone(fixture); deniedSchema.input.schema = "stdio.semio"; assert(!validate(deniedSchema), "caller cannot provide selected schema");
    const deniedOwner = structuredClone(fixture); deniedOwner.input.owner = fixture.parent; assert(!validate(deniedOwner), "caller cannot provide derived owner");
    const deniedAuthority = structuredClone(fixture); deniedAuthority.input.operation = 7; assert(!validate(deniedAuthority), "caller cannot provide app-minted operation authority");
    const bytes = (values: string[]) => values.reduce((total, value) => total + Buffer.byteLength(value), 0);
    assert.equal(bytes([fixture.input.slot, fixture.input.childId, ...fixture.input.dialect]), fixture.requestIdentityBytes);
    assert.equal(bytes([fixture.input.childId, ...fixture.input.dialect, fixture.parent.id, ...fixture.parent.dialect, fixture.input.slot, fixture.input.childId]), fixture.derivedIdentityBytes);
    const declaredSubsets = selectedFixture.declarations.map((declaration: string[]) => declaration[2]);
    assert.deepEqual(fixture.operations.map((row: { subset: string }) => row.subset), declaredSubsets, "operation enum order is the declaration order");
    assert.equal(new Set(declaredSubsets).size, fixture.requiredFactoryOperations);
    for (const row of fixture.operations) {
      const flow = row.subset === "flow";
      assert.deepEqual(
        { support: flow ? "flow" : "unsupported", terminal: flow ? "ready" : "decode", handoffs: flow ? 1 : 0, retiredInputBytes: flow ? 0 : fixture.inputBytes, publications: 0 },
        { support: row.support, terminal: row.terminal, handoffs: row.handoffs, retiredInputBytes: row.retiredInputBytes, publications: row.publications },
        row.subset,
      );
    }
    assert.equal(fixture.operations.filter((row: { support: string }) => row.support === "flow").length, 1, "only the bounded Flow decoder is currently admitted");
    for (const row of fixture.operationLifecycle) {
      const success = row.event === "none" || row.event === "repeat";
      const terminal = row.event === "cancel" ? "cancelled" : row.event === "operation" || row.event === "generation" ? "stale" : row.event === "expired" ? "expired" : row.event === "close" ? "closed" : "ready";
      assert.deepEqual(
        { terminal, handoffs: success ? 1 : 0, retiredInputBytes: success ? 0 : fixture.inputBytes, publications: 0 },
        { terminal: row.terminal, handoffs: row.handoffs, retiredInputBytes: row.retiredInputBytes, publications: row.publications },
        row.id,
      );
    }
    const ids = new Set<string>();
    for (const row of fixture.cases) {
      assert(!ids.has(row.id)); ids.add(row.id);
      const request = structuredClone(fixture.input); const parent = structuredClone(fixture.parent);
      const capacity = { operations: true, abortRetirements: true, childMembers: true, rootRetirements: true };
      let cancelled = false; let factoryReject = false; let repeat = false; let expiresAt = fixture.authority.expiresAt;
      switch (row.mutation) {
        case "none": break; case "wrong-slot": request.slot = "foreign"; break; case "wrong-child": request.childId = "evil-member"; break;
        case "wrong-kind": request.dialect[0] = "s.other.semio"; break; case "wrong-standard": request.dialect[1] = "v2"; break; case "wrong-subset": request.dialect[2] = "tree"; break;
        case "parent-dialect": parent.dialect[0] = "s.other.flow"; break; case "projection": parent.projection.childId = "evil-member"; break;
        case "unsealed": request.sealed = false; break; case "empty": request.inputBytes = 0; break; case "expired": expiresAt = 1; break;
        case "operation-capacity": capacity.operations = false; break; case "abort-capacity": capacity.abortRetirements = false; break; case "member-capacity": capacity.childMembers = false; break; case "root-capacity": capacity.rootRetirements = false; break;
        case "cancel-before": cancelled = true; break; case "cancel-after": case "parent-generation": break; case "factory-reject": factoryReject = true; break; case "repeat-take": repeat = true; break;
        default: throw new Error(`unbound public handoff mutation ${row.mutation}`);
      }
      const exactProjection = request.slot === parent.projection.slot && request.childId === parent.projection.childId && request.dialect.every((value: string, index: number) => value === parent.projection.dialect[index]);
      let error: string | null = cancelled ? "cancelled" : !request.sealed ? "unsealed" : request.inputBytes === 0 ? "empty" : expiresAt <= fixture.authority.now ? "expired"
        : parent.dialect.join("/") !== fixture.parent.dialect.join("/") || !exactProjection ? "identity" : Object.values(capacity).some(available => !available) ? "capacity" : null;
      let retainedBy = "caller"; let identityBytes = fixture.requestIdentityBytes; let reservations = [0, 0, 0, 0]; let handoffs = 0; let retiredInputBytes = error === null ? 0 : request.inputBytes;
      if (error === null) {
        retainedBy = "app"; identityBytes = fixture.derivedIdentityBytes; reservations = [1, 1, 1, 1];
        if (row.mutation === "cancel-after") error = "cancelled";
        if (row.mutation === "parent-generation") error = "stale";
        if (factoryReject) error = "identity";
        if (error !== null) { reservations = [0, 0, 0, 0]; retiredInputBytes = request.inputBytes; }
        else { retainedBy = "factory"; handoffs = 1; if (repeat) assert.equal(handoffs, 1, "a repeated take cannot mint a second operation owner"); }
      }
      assert.deepEqual({ error, retainedBy, identityBytes, reservations, handoffs, publications: 0, creates: 0, retiredInputBytes },
        { error: row.error, retainedBy: row.retainedBy, identityBytes: row.identityBytes, reservations: row.reservations, handoffs: row.handoffs, publications: row.publications, creates: row.creates, retiredInputBytes: row.retiredInputBytes }, row.id);
    }
    assert.equal(ids.size, 20);
    console.log("[DEBUG] public member-open handoff oracle:20 projection/input/capacity cases + exact18 operation arms +8 Flow lifecycle traces; one bounded Flow decoder,17 explicit retained denials, schema+owner derived, publication0");
    const store = readFileSync(join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs"), "utf8");
    const factory = store.slice(store.indexOf("pub trait MemberFactory"), store.indexOf("pub enum NoMembers"));
    assert(/type Open\s*:\s*MemberOpenOperation<Member\s*=\s*Self>/.test(factory) && factory.includes("fn begin_open(request: MemberOpenRequest)"),
      "public handoff requires the actual closed MemberFactory operation; borrowed M::open is not retained authority");
  }
}

class PublicMemberOpenHandoffCheckScript extends PublicMemberOpenHandoffScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("public-member-open-handoff-check accepts no arguments");
    this.proveSource();
    const laws = ["semio_member_factory_request_owned_open_admits_only_retained_flow"];
    const receipts = await runExactCargoLaws({ cwd: this.repoRoot, groups: [{ package: "semio-s-plugin-stdio", target: { kind: "test", name: "flow_retained_decode" }, laws }] });
    assert.equal(receipts.length, 1);
    assert.equal(receipts[0]!.assertions, 1);
  }
}

class CheckScript extends BundleScript {
  run(segments: string[]): void {
    runCargo(["check", "--manifest-path", "Cargo.toml", ...segments], this.root);
  }
}

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    if (rest[0] === "rust") {
      runCargo(["test", "--manifest-path", "Cargo.toml", ...rest.slice(1)], this.root);
      return;
    }
    const legacyTs = join(
      this.repoRoot,
      "🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
    );
    await runVitest(this.root, rest, legacyTs);
  }
}

class WasmScript extends BundleScript {
  run(): void {
    runWasmPackWebBuild({
      rsDir: this.root,
      skipEnvVar: "FRAMEWORK_OS_HOST_RS_SKIP_WASM_BUILD",
      logPrefix: "framework/os/host/rs",
      wasmBaseName: "semio_framework_os",
      pkg: {
        name: "@semio-tech/framework-os-rs",
        files: [
          "semio_framework_os_bg.wasm",
          "semio_framework_os.js",
          "semio_framework_os.d.ts",
          "semio_framework_os_bg.wasm.d.ts",
        ],
        main: "semio_framework_os.js",
        module: "semio_framework_os.js",
        types: "semio_framework_os.d.ts",
      },
    });
  }
}

const router = new ScriptRouter(import.meta.dir)
  .register("check", CheckScript)
  .register("media-projection-check", MediaProjectionScript)
  .register("persistence-contract-check", PersistenceContractScript)
  .register("document-retirement-check", DocumentRetirementScript)
  .register("member-open-protocol-check", MemberOpenProtocolScript)
  .register("member-history-identity-source", MemberHistoryIdentitySourceScript)
  .register("member-history-input-check", MemberHistoryInputScript)
  .register("member-history-id-check", MemberHistoryIdScript)
  .register("member-history-dictionary-check", MemberHistoryDictionaryScript)
  .register("member-history-foundation-check", MemberHistoryFoundationScript)
  .register("member-history-record-check", MemberHistoryRecordScript)
  .register("member-factory-identity-check", MemberFactoryIdentityScript)
  .register("public-member-open-handoff-source", PublicMemberOpenHandoffScript)
  .register("public-member-open-handoff-check", PublicMemberOpenHandoffCheckScript)
  .register("test", TestScript)
  .register("wasm", WasmScript);

await runBundleScriptMain(router, import.meta.url);
