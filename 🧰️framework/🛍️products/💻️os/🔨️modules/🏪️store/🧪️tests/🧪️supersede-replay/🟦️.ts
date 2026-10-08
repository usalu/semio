/** ✏️ Supersede-replay examples: an independent Report-mode replay of the demo family built on
 * fast-json-patch reproduces every expected state, per-mutation outcome and finalize verdict the Rust store is checked
 * against (`🏪️store/🧪️tests/🧪️supersede-replay/🦀️.rs`). */
import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";

import Ajv from "ajv";
import { Database } from "bun:sqlite";
import { blake3 } from "@noble/hashes/blake3.js";
import { blake3Hex } from "../../../../../../🔨️modules/🔏️hash/🟦️.ts";

import { applyPatch, type Operation } from "fast-json-patch";

//#region 🧮️DemoOracle
type Snapshot = { n: number | null };
type DemoOperation = { operation: "setN"; n: number } | { operation: "addN"; delta: number } | { operation: "deleteN" } | { operation: "assignN"; n?: number | null };
type Level = "info" | "warning" | "error" | "fatal";
type Message = { level: Level; code: string };
type Outcome = { edit: number; op: number; worst: Level | null; codes: string[]; superseded: boolean; withdrawn: boolean };
type InvalidInput = { invalid: { schema: string; payloadHex: string } };
type Case = { name: string; initial: Snapshot; edits: DemoOperation[][]; supersessions: { edit: number; op: number; replacement: DemoOperation | InvalidInput | "withdrawn" }[]; expected: { state: Snapshot; outcomes: Outcome[]; blocksFinalize: boolean } };

const read = (path: string) => JSON.parse(readFileSync(new URL(path, import.meta.url), "utf8"));
const rank: Record<Level, number> = { info: 0, warning: 1, error: 2, fatal: 3 };
const saturate = (value: number) => Math.min(2147483647, Math.max(-2147483648, value));
const missing: Message = { level: "error", code: "mutation.target-missing" };

test("retained clone alternating grants keep one combined native byte allowance", () => {
  const law = read("../../🧬️snapshot-clone/🧪️fixtures/📦️lifecycle/🔣️.json");
  expect(law.grant.alternateCapacityPayloadAndRelease).toBe(true);
  for (const axis of ["capacity", "payload", "release"] as const) {
    const source = {capacity:0,payload:0,release:0};
    source[axis] = law.grant.maximumBytes;
    const oracle = applyPatch({capacity:0,payload:0,release:0}, [{op:"replace",path:`/${axis}`,value:law.grant.maximumBytes}], true, false).newDocument;
    expect(source).toEqual(oracle);
    expect(oracle.capacity + oracle.payload + oracle.release).toBe(law.grant.combinedCapacityCopyAndReleaseMaximum);
  }
  console.log("[DEBUG] retained clone capacity/payload/release alternating grants preserve one 4096-byte allowance");
});

/** 🧮️ One demo operation as the JSON Patch it writes and the messages it raises against `state`. */
function diff(operation: DemoOperation, state: Snapshot): { patch: Operation[]; messages: Message[] } {
  switch (operation.operation) {
    case "setN":
      return state.n === null ? { patch: [], messages: [missing] } : { patch: [{ op: "replace", path: "/n", value: operation.n }], messages: [] };
    case "addN":
      return state.n === null ? { patch: [], messages: [missing] } : { patch: [{ op: "replace", path: "/n", value: saturate(state.n + operation.delta) }], messages: [{ level: "info", code: "mutation.cascade" }] };
    case "deleteN":
      return { patch: state.n === null ? [] : [{ op: "replace", path: "/n", value: null }], messages: [] };
    case "assignN":
      return { patch: [{ op: "replace", path: "/n", value: operation.n ?? null }], messages: [] };
  }
}

/** ⏪️ Replays every recorded operation in applied order, each through its effective input, keeping and recording outcomes. */
function replay(testCase: Case): Case["expected"] {
  let state: Snapshot = structuredClone(testCase.initial);
  const replacements = new Map(testCase.supersessions.map((supersession) => [`${supersession.edit}/${supersession.op}`, supersession.replacement]));
  const outcomes: Outcome[] = [];
  testCase.edits.forEach((edit, editIndex) =>
    edit.forEach((original, opIndex) => {
      const key = `${editIndex}/${opIndex}`;
      const superseded = replacements.has(key);
      const replacement = replacements.get(key);
      if (replacement === "withdrawn") {
        outcomes.push({ edit: editIndex, op: opIndex, worst: null, codes: [], superseded, withdrawn: true });
        return;
      }
      if (replacement !== undefined && "invalid" in replacement) {
        outcomes.push({ edit: editIndex, op: opIndex, worst: "fatal", codes: ["mutation.invariant"], superseded, withdrawn: false });
        return;
      }
      const { patch, messages } = diff(replacement ?? original, state);
      state = applyPatch(state, patch, true, false).newDocument;
      const worst = messages.reduce<Level | null>((known, message) => (known === null || rank[message.level] > rank[known] ? message.level : known), null);
      outcomes.push({ edit: editIndex, op: opIndex, worst, codes: messages.map((message) => message.code), superseded, withdrawn: false });
    }),
  );
  return { state, outcomes, blocksFinalize: outcomes.some((outcome) => outcome.worst === "error" || outcome.worst === "fatal") };
}
//#endregion 🧮️DemoOracle

//#region 🧪️Corpus
const corpus = read("../../🧫️fixtures/🧫️supersede-replay/🔣️.json");

test("cold history fixtures declare finite caller identity admission", () => {
  const law = corpus.identityAdmission;
  const database = new Database(":memory:");
  try { expect(database.query<{ ceiling: number }, number[]>("SELECT ?1 * ?2 + ?3 AS ceiling").get(law.maximumOperations, law.bodyBytesPerOperation, law.commandScaffoldBytes)?.ceiling).toBe(law.maximumOwnedBytes); }
  finally { database.close(); }
  const source = readFileSync(new URL("./🦀️.rs", import.meta.url), "utf8");
  expect(source).toContain("EntityIdentityAuthority::new(FIXTURE_IDENTITY_BYTE_CEILING");
  expect(source).toContain("const FIXTURE_IDENTITY_BYTE_CEILING: usize = 201 * semio_framework_job::JOB_PAYLOAD_PAGE_BYTES");
});

test("🎟️ semantic preparation validates independent physical currencies", () => {
  const law = read("../../🔁️replay/🎮️operation/🧫️fixtures/🔣️.json").turnReceipts;
  const database = new Database(":memory:");
  try {
    const query = database.query<{ accepted: number }, number[]>("SELECT (?1 <= 1 AND ?2 <= ?3 AND ?4 <= ?5 AND ?6 <= ?7 AND ?2 + ?4 <= ?8) AS accepted");
    for (const row of law.cases) expect(Boolean(query.get(row.items, row.copy, row.copyGrant, row.capacity, row.capacityGrant, row.release, row.releaseGrant, law.maximumBodyCapacityBytes)?.accepted)).toBe(row.accepted);
  } finally { database.close(); }
  const source = readFileSync(new URL("../../🔁️replay/🎮️operation/🦀️.rs", import.meta.url), "utf8");
  expect(source).toContain("Pending(RetainedCloneProgress)");
  expect(source).toContain("progress.fits(grant.retained_grant())");
  for (const lane of ["copy", "capacity", "release", "depth"]) expect(source).toContain(`next_advance_${lane}_demand`);
});

test("📨️ selected diagnostic copy belongs to the neutral message owner", () => {
  const base = "../../../../../../🔨️modules/📡️replication/🎮️mutation/📨️messages/📋️copy/";
  const law = read(`${base}🧫️fixtures/🔣️.json`);
  for (const row of law.cases) for (const text of [row.code, row.message, ...row.target]) expect(new TextDecoder("utf-8", { fatal: true }).decode(new TextEncoder().encode(text))).toBe(text);
  expect(readFileSync(new URL(`${base}🦀️.rs`, import.meta.url), "utf8").includes("pub struct MessageCopyCursor")).toBe(true);
});

test("📨️ cooperative message copy matches independent UTF-8 and JSON Patch owners", () => {
  const base = "../../../../../../🔨️modules/📡️replication/🎮️mutation/📨️messages/📋️copy/";
  const law = read(`${base}🧫️fixtures/🔣️.json`);
  for (const row of law.cases) {
    for (const bytes of law.byteGrants) {
      const values = [row.code, row.message, ...row.target].map((text: string) => {
        const source = new TextEncoder().encode(text);
        const decoder = new TextDecoder("utf-8", { fatal: true });
        let copied = "";
        for (let at = 0; at < source.byteLength; at += bytes) copied += decoder.decode(source.subarray(at, at + bytes), { stream: true });
        return copied + decoder.decode();
      });
      const output = applyPatch({ code: "", message: "", target: [] }, [
        { op: "replace", path: "/code", value: values[0] }, { op: "replace", path: "/message", value: values[1] }, { op: "replace", path: "/target", value: values.slice(2) },
      ], true, false).newDocument;
      expect(output).toEqual(row);
    }
  }
  expect(law.emptySegments.count * 24 <= law.capacityGrant).toBe(law.emptySegments.accepted);
});

test("📨️ copied diagnostic closure admits each actual backing release", () => {
  const base = "../../../../../../🔨️modules/📡️replication/🎮️mutation/📨️messages/📋️copy/";
  const law = read(`${base}🧫️fixtures/🔣️.json`);
  for (const row of law.cases) {
    const owners = [row.code, row.message, ...row.target].map((value: string) => Buffer.byteLength(value, "utf8")).filter((bytes: number) => bytes > 0);
    let current = { owners };
    while (current.owners.length) {
      const demand = current.owners[0];
      expect(demand <= 0).toBe(law.closure.emptyReleaseAccepted);
      expect(demand <= demand - 1).toBe(law.closure.partialReleaseAccepted);
      expect(demand).toBeLessThanOrEqual(law.closure.maximumReleaseBytes);
      const next = applyPatch(current, [{ op: "remove", path: "/owners/0" }], true, false).newDocument;
      expect(current.owners.length - next.owners.length).toBe(law.closure.sameTurnBackingReleases);
      current = next;
    }
  }
  const producer = readFileSync(new URL(`${base}🦀️.rs`, import.meta.url), "utf8");
  for (const axis of ["copy", "capacity", "release", "depth"]) expect(producer.includes(`pub fn next_close_${axis}_byte_demand`) || (axis === "depth" && producer.includes("pub fn next_close_depth_demand"))).toBe(true);
  expect(producer.includes("vec![output]")).toBe(false);
  console.log("[DEBUG] diagnostic closure independent UTF-8 allocation ledger removes one exact backing after full grant");
});

test("🎮️ cooperative semantic preparation matches independent JSON Patch before refusal policy", () => {
  const law = read("../../🔁️replay/🎮️operation/🧫️fixtures/🔣️.json");
  const capacity = BigInt(law.capacityRelease.emptySlots) * BigInt(law.capacityRelease.slotBytes);
  expect(capacity <= BigInt(law.capacityRelease.bodyBytes)).toBe(law.capacityRelease.ordinaryGrantAccepted);
  expect(capacity <= capacity).toBe(law.capacityRelease.exactGrantAccepted);
  const inverse = Array.from({length:law.pagedInverse.rows}, (_, index) => index % 256);
  const copiedInverse = applyPatch({}, [{op:"add",path:"/inverse",value:inverse}], true, false).newDocument.inverse;
  expect(copiedInverse).toEqual(inverse);
  expect(Math.ceil(inverse.length / law.pagedInverse.maximumTurnBytes)).toBeGreaterThanOrEqual(law.pagedInverse.minimumBackingReleases);

  for (const row of law.cases) {
    const forward: Operation[] = row.applyRefused ? [] : [{ op: "replace", path: "/text", value: row.input }];
    expect(applyPatch({ text: row.base }, forward, true, false).newDocument.text).toBe(row.expected.state);
    const inverse: Operation[] = row.inverseRefused || row.applyRefused ? [] : [{ op: "replace", path: "/text", value: row.base }];
    expect(inverse.length === 0 ? null : (inverse[0] as { value: unknown }).value).toBe(row.expected.inverse);
    expect([...(row.inverseRefused ? ["mutation.inverse-refused"] : []), ...(row.applyRefused ? ["mutation.apply.refused"] : [])]).toEqual(row.expected.fatalCodes);
    for (const grant of law.grants) {
      for (const source of [row.base, row.input]) {
        const chunks = [...source].map((scalar: string) => new TextEncoder().encode(scalar));
        expect(chunks.every((chunk: Uint8Array) => chunk.byteLength <= grant)).toBe(true);
        const decoder = new TextDecoder("utf-8", { fatal: true });
        expect(chunks.map((chunk: Uint8Array) => decoder.decode(chunk, { stream: true })).join("") + decoder.decode()).toBe(source);
        for (const cancelledAt of law.cancelAt) {
          const prefix = chunks.slice(0, cancelledAt);
          expect(prefix.map((chunk: Uint8Array) => chunk.byteLength).reduce((a: number, b: number) => a + b, 0)).toBeLessThanOrEqual(cancelledAt * grant);
          const cancelledCalls = prefix.length;
          prefix.splice(0);
          expect(prefix.length).toBe(0);
          expect(cancelledCalls).toBe(Math.min(chunks.length, cancelledAt));
        }
      }
    }
  }
});



test("🧮️ an independent fast-json-patch replay reproduces every expected state and outcome", () => {
  for (const testCase of corpus.cases as Case[]) {
    expect(replay(testCase), testCase.name).toEqual(testCase.expected);
  }
});
//#endregion 🧪️Corpus

/** ⌛️ The same bounded-read law is projected independently with JSON Patch before either cursor runs. */
test("🕰️ bounded history reads stop at the selected mutation and replay the complete changed history", () => {
  const law = corpus.boundedHistoryRead;
  expect(law.closeGrant.maximumItems).toBe(1);
  expect(law.closeGrant.maximumReleaseBytes).toBe(4096);
  expect(new Uint8Array(law.physicalInversePageBytes).byteLength).toBe(4096);
  expect(law.physicalInversePageBytes).toBeLessThanOrEqual(law.closeGrant.maximumReleaseBytes);
  const prefix = { name: "preview", initial: law.initial, edits: law.edits.slice(0, law.target.edit + 1).map((edit: DemoOperation[], index: number) => index === law.target.edit ? edit.slice(0, law.target.op) : edit), supersessions: law.accepted };
  const before = replay(prefix as Case).state;
  expect(before.n).toBe(law.expected.before);
  expect(applyPatch(before, diff(law.replacement, before).patch, true, false).newDocument.n).toBe(law.expected.preview);
  expect(replay({ ...prefix, edits: law.edits, supersessions: [...law.accepted, { ...law.target, replacement: law.replacement }] } as Case).state.n).toBe(law.expected.head);
  const committed = replay({ ...prefix, edits: law.edits, supersessions: [] } as Case).state;
  for (const stop of law.cancelAfterSteps) {
    let retained = { head: committed.n, owners: law.edits.flat().slice(0, stop) };
    while (retained.owners.length) {
      retained = applyPatch(retained, [{ op: "remove", path: "/owners/0" }], true, false).newDocument;
      expect(retained.head === committed.n).toBe(law.expected.cancelPreservesHead);
    }
    expect(retained.owners.length === 0).toBe(law.expected.cancelRetirementCompletes);
  }
});

/** 🌱️ Independent pack and hash authorities distinguish equal decoded values from stored-byte identity. */
test("🌱️ actor and stored genesis law preserves supplied bytes across encoder choices", () => {
  const law = read("../../🧫️fixtures/🧫️actor-genesis/🔣️.json");
  const digests: string[] = [];
  for (const row of law.packs) {
    const bytes = Uint8Array.from(Buffer.from(row.hex, "hex"));
    expect(JSON.parse(new TextDecoder().decode(bytes))).toEqual(row.decoded);
    const independent = Buffer.from(blake3(bytes)).toString("hex");
    expect(blake3Hex(bytes)).toBe(independent);
    const copied = Buffer.from(bytes).toString("hex");
    expect(copied).toBe(row.hex);
    for (const grant of [1, 2, 7]) {
      const retained = blake3.create();
      let completed = 0;
      while (completed < bytes.length) {
        const end = Math.min(bytes.length, completed + grant);
        retained.update(bytes.subarray(completed, end));
        expect(end - completed).toBeLessThanOrEqual(grant);
        completed = end;
      }
      expect(Buffer.from(retained.digest()).toString("hex")).toBe(independent);
    }
    digests.push(independent);
  }
  expect(new Set(digests).size).toBe(law.packs.length);
  expect(law.actors.bareAuthor).toBe(law.actors.opened);
  expect(law.actors.transitionAuthor).toBe(law.actors.opened);
  expect(law.actors.reopened).toBe(law.actors.opened);
  const admitsAuthors = new Ajv({ strict: true }).compile(read("../../📜️space-history/🧬️schema/✍️authors/🔣️.json"));
  for (const row of law.retainedAdmissions) {
    const admitted = applyPatch({ editAuthor: law.actors.opened, operationAuthor: law.actors.opened }, [
      { op: "replace", path: "/editAuthor", value: row.editAuthor },
      { op: "replace", path: "/operationAuthor", value: row.operationAuthor },
    ], true, false).newDocument;
    expect(admitsAuthors(admitted)).toBe(row.accepted);
  }
});

/** ⏳️ The platform UTF-8 decoder and JSON Patch independently validate granted history decoding and cancellation. */
test("⏳️ bounded fold and decoder neutral law preserves UTF-8 and cancellation owners", () => {
  const base = "../../../../../../🔨️modules/📡️replication/🔗️causal/🔀️transition/🔁️fold/";
  const law = read(`${base}🧫️fixtures/🔣️.json`);
  for (const row of law.decodeCases) {
    const payload = Uint8Array.from(Buffer.from(row.payloadHex, "hex"));
    const length = payload[1];
    let decoded: string | undefined;
    try { decoded = new TextDecoder("utf-8", { fatal: true }).decode(payload.subarray(2, 2 + length)); } catch {}
    const valid = payload[0] === 4 && decoded !== undefined && payload.length === 3 + length && payload[2 + length] === 0;
    expect(valid).toBe(row.valid);
    if (!valid) continue;
    expect(decoded).toBe(row.checkpoint);
    for (const grant of law.byteGrants) {
      const decoder = new TextDecoder("utf-8", { fatal: true });
      let text = "";
      for (let at = 2; at < 2 + length; at += grant) text += decoder.decode(payload.subarray(at, Math.min(at + grant, 2 + length)), { stream: true });
      text += decoder.decode();
      expect(text).toBe(row.checkpoint);
    }
  }
  for (const row of law.envelopeCases) {
    const bytes = Uint8Array.from(Buffer.from(row.payloadHex, "hex"));
    let at = 0;
    const scalar = () => { if (at >= bytes.length || bytes[at] >= 128) throw new Error("neutral scalar refuses truncation/oversized varint"); return bytes[at++]; };
    const blob = () => { const size = scalar(); if (at + size > bytes.length) throw new Error("truncated neutral blob"); const value = bytes.subarray(at, at + size); at += size; return value; };
    const text = () => new TextDecoder("utf-8", { fatal: true }).decode(blob());
    let admitted = false;
    try {
      expect(text()).toBe("m"); expect(text()).toBe("d");
      const actor = text();
      expect(scalar()).toBe(0); expect(scalar()).toBe(0); expect(scalar()).toBe(0);
      expect(text()).toBe("s"); const payload = Buffer.from(blob()).toString("hex");
      expect(text()).toBe("s"); expect(blob().length).toBe(0);
      expect([scalar(), scalar(), scalar()]).toEqual([1, 2, 3]);
      const flags = scalar();
      admitted = flags === 0 && at === bytes.length;
      if (admitted) { expect(actor).toBe(row.actor); expect(payload).toBe(row.diffPayloadHex); }
    } catch {}
    expect(admitted).toBe(row.valid);
  }
  for (const cancelledAt of law.cancelAt) {
    let state = { completed: cancelledAt, owners: Array.from({ length: cancelledAt }, (_, position) => position) };
    while (state.owners.length) state = applyPatch(state, [{ op: "remove", path: "/owners/0" }], true, false).newDocument;
    expect(state.completed === cancelledAt).toBe(law.expected.noWorkAfterCancel);
    expect(state.owners.length === 0).toBe(law.expected.terminalEmpty);
  }
});
