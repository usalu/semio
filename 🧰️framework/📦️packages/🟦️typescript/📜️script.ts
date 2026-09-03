#!/usr/bin/env bun
/** @emoji 🧰️ `@semio-tech/framework` router: `bun ./📜️script.ts test`. */
import { strict as assert } from "node:assert";
import { spawnSync } from "node:child_process";
import { BundleScript, ScriptRouter, resolveTestLevel, runBundleScriptMain, runVitest } from "../../🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

function retainedUiNativeStripOnly(): void {
  const source = new URL("../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🟦️.ts", import.meta.url).href;
  const numeric = new URL("../../🔨️modules/🌱️value/🗂️ordered/🔢️numeric/🟦️.ts", import.meta.url).href;
  const fixture = new URL("../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔣️owned-nodes.json", import.meta.url).href;
  const validation = new URL("../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🛡️validation/🟦️.ts", import.meta.url).href;
  const nodes = new URL("../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🗂️nodes/🟦️.ts", import.meta.url).href;
  const validationFixture = new URL("../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔣️owned-validation.json", import.meta.url).href;
  const hash = new URL("../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🔢️hash/🟦️.ts", import.meta.url).href;
  const hashFixture = new URL("../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔣️owned-hash.json", import.meta.url).href;
  const readLease = new URL("../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📖️read-lease/🟦️.ts", import.meta.url).href;
  const readLeaseFixture = new URL("../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔣️read-lease.json", import.meta.url).href;
  const program = `
    import assert from "node:assert/strict";
    import { createHash } from "node:crypto";
    import { readFileSync } from "node:fs";
    import { fileURLToPath } from "node:url";
    const { RetainedUiNumericTable, RetainedUiSiblingKeys } = await import(${JSON.stringify(source)});
    const { NumericIndex } = await import(${JSON.stringify(numeric)});
    const { OwnedUiValidationCursor } = await import(${JSON.stringify(validation)});
    const { OwnedUiNodeIndex } = await import(${JSON.stringify(nodes)});
    const { OwnedUiSnapshotHashCursor } = await import(${JSON.stringify(hash)});
    const { OwnedUiNodeReadLease, OwnedUiReadPublication } = await import(${JSON.stringify(readLease)});
    const fixture = JSON.parse(readFileSync(fileURLToPath(${JSON.stringify(fixture)}), "utf8"));
    const validationFixture = JSON.parse(readFileSync(fileURLToPath(${JSON.stringify(validationFixture)}), "utf8"));
    const hashFixture = JSON.parse(readFileSync(fileURLToPath(${JSON.stringify(hashFixture)}), "utf8"));
    const readLeaseFixture = JSON.parse(readFileSync(fileURLToPath(${JSON.stringify(readLeaseFixture)}), "utf8"));
    assert.equal(fixture.version, 1);
    for (const law of ["index-entry-owns-exact-node", "cancel-at-every-edit-phase", "zero-grant-does-not-close", "deleted-reinserted-id-gets-new-ordinal"]) assert.ok(fixture.laws.includes(law));
    let grant = fixture.grants[2];
    let grants = 0;
    const retired = [];
    const grantOwner = () => { grants++; return grant; };
    const table = new RetainedUiNumericTable(NumericIndex.empty(), grantOwner, value => retired.push(value));
    assert.equal(table.grant, grantOwner);
    const drain = generator => {
      let turns = 0;
      for (; turns < 100000; turns++) {
        const step = generator.next();
        if (step.done) return { value: step.value, turns };
        assert.equal(typeof step.value, "number");
        assert.ok(step.value <= grant.maxBytes);
      }
      assert.fail("retained UI native generator did not terminate");
    };
    const entries = owner => {
      const values = [];
      const generator = owner.entries();
      for (let turns = 0; turns < 100000; turns++) {
        const step = generator.next();
        if (step.done) return values;
        if (Array.isArray(step.value)) values.push(step.value);
        else { assert.equal(typeof step.value, "number"); assert.ok(step.value <= grant.maxBytes); }
      }
      assert.fail("retained UI native reader did not terminate");
    };
    const close = owner => {
      for (let turns = 0; turns < 100000; turns++) {
        const step = owner.closeStep(grant);
        assert.ok(step.bytes <= grant.maxBytes);
        if (step.complete) return turns + 1;
      }
      assert.fail("retained UI native owner did not retire");
    };
    const retire = owner => {
      for (let turns = 0; turns < 100000; turns++) {
        const step = owner.advance(grant);
        assert.ok(step.items <= grant.maxItems && step.bytes <= grant.maxBytes);
        if (step.kind === "complete") return turns + 1;
        assert.notEqual(step.kind, "blocked");
      }
      assert.fail("retained UI native retirement did not terminate");
    };
    const finishValidation = cursor => {
      for (let turns = 0; turns < 100000; turns++) {
        const step = cursor.advance(grant);
        assert.ok(step.items <= grant.maxItems && step.bytes <= grant.maxBytes);
        if (step.kind === "ready") return turns + 1;
        assert.notEqual(step.kind, "blocked");
        assert.notEqual(step.kind, "rejected", cursor.failure);
      }
      assert.fail("retained UI native validation did not terminate");
    };
    const closeValidation = cursor => {
      cursor.beginClose();
      assert.equal(cursor.closeStep({ maxItems: 0, maxBytes: 4096 }).kind, "blocked");
      for (let turns = 0; turns < 100000; turns++) {
        const step = cursor.closeStep(grant);
        assert.ok(step.items <= grant.maxItems && step.bytes <= grant.maxBytes);
        if (step.kind === "complete") { assert.equal(cursor.terminalIsEmpty(), true); return turns + 1; }
      }
      assert.fail("retained UI native validation close did not terminate");
    };
    const values = fixture.ids.map((id, offset) => Object.freeze({ id, byte: fixture.oldBytes[offset] }));
    for (let offset = 0; offset < values.length; offset++) {
      const operation = table.set(fixture.ids[offset], values[offset]);
      if (offset === 0) {
        grant = fixture.grants[0];
        assert.deepStrictEqual(operation.next(), { value: 0, done: false });
        assert.equal(table.size, 0);
        grant = fixture.grants[2];
      }
      drain(operation);
    }
    assert.deepStrictEqual(entries(table).map(entry => entry[0]), fixture.ids);
    const removed = values[fixture.ids.indexOf(fixture.removed)];
    drain(table.remove(fixture.removed));
    assert.equal(retired.filter(value => value === removed).length, 1);
    const replacement = Object.freeze({ id: fixture.removed, byte: 255 });
    drain(table.set(fixture.removed, replacement));
    assert.deepStrictEqual(entries(table).map(entry => entry[0]), fixture.expectedReinsertedOrder);
    assert.equal(drain(table.lookup(fixture.removed)).value, replacement);
    const tableCloseTurns = close(table);
    for (const value of [...values, replacement]) assert.equal(retired.filter(item => item === value).length, 1);
    const cancelledRetired = [];
    const cancelled = new RetainedUiNumericTable(NumericIndex.empty(), grantOwner, value => cancelledRetired.push(value));
    const sourceValue = Object.freeze({ id: 17 });
    const candidateValue = Object.freeze({ id: 18 });
    drain(cancelled.set(17, sourceValue));
    const candidate = cancelled.set(18, candidateValue);
    const prefix = candidate.next();
    assert.equal(prefix.done, false);
    assert.equal(cancelled.index.get(17), sourceValue);
    assert.equal(cancelled.index.get(18), undefined);
    const cancellationCloseTurns = close(cancelled);
    assert.equal(cancelledRetired.filter(value => value === sourceValue).length, 1);
    assert.equal(cancelledRetired.filter(value => value === candidateValue).length, 1);
    const siblings = new RetainedUiSiblingKeys(grantOwner);
    const key = String.fromCodePoint(945, 0x1f9e9).repeat(64);
    assert.equal(drain(siblings.insert(key)).value, false);
    assert.equal(drain(siblings.insert(key)).value, true);
    drain(siblings.clear());
    assert.equal(validationFixture.version, 1);
    for (const law of ["exact-old-index-capture", "all-phase-cancel", "zero-grant-no-progress", "violation-order-parity"]) assert.ok(validationFixture.laws.includes(law));
    const emptyVector = validationFixture.cases.find(vector => vector.name === "empty");
    assert.ok(emptyVector);
    const limits = { maxNodes: emptyVector.maxNodes, maxDepth: emptyVector.maxDepth, maxChildren: 4096, maxTextBytes: 65536, maxPatchOps: 4096, maxPatchBytes: 1048576 };
    const validationSource = OwnedUiNodeIndex.empty();
    const validator = new OwnedUiValidationCursor(validationSource, emptyVector.root, limits);
    assert.equal(validator.advance({ maxItems: 0, maxBytes: grant.maxBytes }).kind, "blocked");
    const validationTurns = finishValidation(validator);
    const violations = validator.takeResult();
    assert.ok(violations);
    assert.deepStrictEqual(Array.from(violations, ([, value]) => value.type), emptyVector.expected);
    retire(violations.beginClose());
    const validationCloseTurns = closeValidation(validator);
    let validationCancellations = 0;
    for (let cutoff = 0; cutoff <= validationTurns; cutoff++) {
      const source = OwnedUiNodeIndex.empty();
      const cursor = new OwnedUiValidationCursor(source, emptyVector.root, limits);
      for (let turn = 0; turn < cutoff; turn++) cursor.advance(grant);
      closeValidation(cursor);
      assert.equal(cursor.takeResult(), null);
      assert.equal(source.size, 0);
      retire(source.beginClose());
      validationCancellations++;
    }
    retire(validationSource.beginClose());
    const capturedSource = OwnedUiNodeIndex.empty();
    const captured = new OwnedUiValidationCursor(capturedSource, emptyVector.root, limits);
    retire(capturedSource.beginClose());
    finishValidation(captured);
    const capturedViolations = captured.takeResult();
    assert.ok(capturedViolations);
    assert.deepStrictEqual(Array.from(capturedViolations), []);
    retire(capturedViolations.beginClose());
    closeValidation(captured);
    assert.equal(hashFixture.version, 1);
    for (const law of ["node-json-buffer-byte-parity", "insertion-order-preserved", "frozen-source-identity", "surface-bytes-as-canonical-array", "cancel-frames-before-node", "zero-grant-no-hash", "no-whole-snapshot-materialization"]) assert.ok(hashFixture.laws.includes(law));
    let nested = hashFixture.text;
    for (let depth = 0; depth < hashFixture.depth; depth++) nested = { next: nested };
    const bytes = Object.freeze({ length: hashFixture.surfaceBytes, byteAt: offset => { assert.ok(offset >= 0 && offset < hashFixture.surfaceBytes); return offset % 251; } });
    const records = hashFixture.ids.map((id, ordinal) => Object.freeze({ id, key: "node:" + ordinal, component: ordinal === 0 ? { type: "surface", kind: "node-graph", docSchema: "node-graph@1", doc: { bytes } } : { type: "extension", extension: "hash", props: { nested, text: hashFixture.text, numbers: [-0, 1e-7, 1e21] } }, children: [] }));
    let captures = 0;
    let capturedCloses = 0;
    let nodeOpens = 0;
    let nodeCloses = 0;
    const completedRetirement = complete => {
      let done = false;
      return { advance(current) { assert.ok(current.maxItems >= 1 && current.maxBytes >= 4096); if (!done) { done = true; complete(); } return { kind: "complete", phase: "oracle-close", items: 0, bytes: 0 }; }, terminalIsEmpty() { return done; } };
    };
    const capturedIndex = () => {
      let closed = false;
      return {
        get size() { assert.equal(closed, false); return records.length; },
        beginRead() {
          assert.equal(closed, false);
          let offset = 0;
          let readerClosed = false;
          return {
            advance(current) {
              assert.equal(readerClosed, false); assert.ok(current.maxItems >= 1 && current.maxBytes >= 4096);
              if (offset === records.length) return { kind: "complete", phase: "oracle-read", items: 0, bytes: 0 };
              const value = records[offset]; const id = value.id; const low = offset++;
              return { kind: "value", phase: "oracle-read", id, ordinal: { high: 0, low }, value: { value, beginClose() { nodeOpens++; return completedRetirement(() => nodeCloses++); } }, items: 1, bytes: 64 };
            },
            beginClose() { assert.equal(readerClosed, false); readerClosed = true; return completedRetirement(() => {}); }
          };
        },
        beginClose() { assert.equal(closed, false); closed = true; return completedRetirement(() => capturedCloses++); }
      };
    };
    const hashSource = { get size() { return records.length; }, capture() { captures++; return capturedIndex(); } };
    const expectedRecords = records.map((record, ordinal) => ordinal === 0 ? { ...record, component: { ...record.component, doc: { bytes: Array.from({ length: hashFixture.surfaceBytes }, (_, offset) => offset % 251) } } } : record);
    const expectedBytes = Buffer.from(JSON.stringify({ surface: hashFixture.surface, revision: hashFixture.revision, root: hashFixture.root, nodes: expectedRecords, layoutEpoch: "0" }), "utf8");
    const metadata = { surface: hashFixture.surface, revision: hashFixture.revision, root: hashFixture.root };
    const closeHash = cursor => {
      cursor.beginClose(); assert.equal(cursor.closeStep({ maxItems: 0, maxBytes: grant.maxBytes }).kind, "blocked");
      for (let turns = 0; turns < 100000; turns++) { const step = cursor.closeStep(grant); assert.ok(step.items <= grant.maxItems && step.bytes <= grant.maxBytes); if (step.kind === "complete") { assert.equal(cursor.terminalIsEmpty(), true); return turns + 1; } }
      assert.fail("retained UI native hash close did not terminate");
    };
    const hasher = new OwnedUiSnapshotHashCursor(hashSource, metadata);
    assert.equal(hasher.advance({ maxItems: 0, maxBytes: grant.maxBytes }).kind, "blocked");
    const actualDigest = createHash("sha256");
    let hashValue = 0x811c9dc5;
    let hashBytes = 0;
    let hashCalls = 0;
    let hashChunks = 0;
    for (; hashCalls < 100000; hashCalls++) {
      const step = hasher.advance(grant);
      assert.ok(step.items <= grant.maxItems && step.bytes <= grant.maxBytes);
      if (step.chunk) {
        assert.ok(step.chunk.length <= 256);
        for (const byte of step.chunk) { assert.equal(byte, expectedBytes[hashBytes++]); hashValue = Math.imul(hashValue ^ byte, 0x01000193) >>> 0; }
        actualDigest.update(step.chunk); hashChunks++;
      }
      if (step.kind === "ready") { hashCalls++; break; }
      assert.notEqual(step.kind, "rejected", hasher.failure);
    }
    assert.equal(hashBytes, expectedBytes.length);
    assert.ok(hashChunks > 1);
    assert.equal(actualDigest.digest("hex"), createHash("sha256").update(expectedBytes).digest("hex"));
    assert.deepStrictEqual(hasher.takeResult(), { hash: hashValue.toString(16) + ":" + hashFixture.revision, byteLength: expectedBytes.length });
    assert.equal(hasher.takeResult(), null);
    const hashCloseTurns = closeHash(hasher);
    const hashFrontiers = [...new Set([0, 1, 3, Math.floor(hashCalls / 3), Math.floor(hashCalls / 2), hashCalls - 1, hashCalls])].filter(value => value >= 0);
    for (const cutoff of hashFrontiers) {
      const cursor = new OwnedUiSnapshotHashCursor(hashSource, metadata);
      for (let turn = 0; turn < cutoff; turn++) cursor.advance(grant);
      closeHash(cursor);
      assert.equal(cursor.takeResult(), null);
      assert.equal(hashSource.size, records.length);
    }
    assert.equal(captures, hashFrontiers.length + 1);
    assert.equal(capturedCloses, captures);
    assert.equal(nodeCloses, nodeOpens);
    assert.equal(readLeaseFixture.version, 1);
    assert.equal(readLeaseFixture.capacity, 2);
    assert.deepStrictEqual(readLeaseFixture.laws, ["repeated-read-stable-no-capture", "two-issued-root-backpressure", "exact-consumer-ack", "stale-ack-preserves-newer", "retirement-before-capacity-reuse", "independent-subscribers", "unsubscribe-drains-speculative-root", "zero-grant-close-preserves-bytes", "aborted-render-mints-no-owner", "null-is-not-an-issued-token"]);
    const readGrant = { maxItems: 1, maxBytes: 4096 };
    let readCaptures = 0;
    let readCloses = 0;
    const readOwner = version => {
      const value = Object.freeze({ id: readLeaseFixture.node, key: "read:" + version, component: { type: "extension", extension: "read", props: { version } }, children: [] });
      return { value, capture() { readCaptures++; let closing = false; return { value, beginClose() { assert.equal(closing, false); closing = true; let retired = false; return { advance(current) { assert.ok(current.maxItems >= 1 && current.maxBytes >= 4096); if (!retired) { retired = true; readCloses++; } return { kind: "complete", phase: "oracle-read-node-close", items: 0, bytes: 0 }; }, terminalIsEmpty() { return retired; } }; } }; } };
    };
    const drainRead = lease => {
      let turns = 0;
      for (; turns < 1000 && lease.retirementPending; turns++) { const step = lease.advanceRetirement(readGrant); assert.ok(step.items <= 1 && step.bytes <= 4096); assert.notEqual(step.kind, "blocked"); }
      assert.equal(lease.retirementPending, false); return turns;
    };
    const closeRead = lease => {
      lease.beginClose(); assert.equal(lease.closeStep({ maxItems: 0, maxBytes: 4096 }).kind, "blocked");
      for (let turns = 0; turns < 1000; turns++) { const step = lease.closeStep(readGrant); assert.ok(step.items <= 1 && step.bytes <= 4096); if (step.kind === "complete") { assert.equal(lease.terminalIsEmpty(), true); return turns + 1; } }
      assert.fail("retained UI native read lease did not close");
    };
    class ReferenceReadLease {
      constructor(version) { this.publication = version; this.visible = version; this.staged = null; this.retiring = false; }
      stage(version) { assert.equal(this.staged, null); assert.ok(version > this.publication); this.staged = version; }
      cancel() { assert.notEqual(this.staged, null); this.staged = null; }
      publish() { assert.notEqual(this.staged, null); this.publication = this.staged; this.visible = this.staged; this.staged = null; this.retiring = true; }
      retire() { assert.equal(this.retiring, true); this.retiring = false; }
    }
    const reference = new ReferenceReadLease(readLeaseFixture.versions[0]);
    const publication = new OwnedUiReadPublication(reference.publication);
    const shared = readOwner(reference.visible);
    const read = new OwnedUiNodeReadLease(readLeaseFixture.node, reference.visible, shared, publication);
    const independentRead = new OwnedUiNodeReadLease(readLeaseFixture.node, reference.visible, shared, publication);
    const initialRead = read.snapshot;
    assert.equal(read.snapshot, initialRead);
    assert.equal(read.snapshot, initialRead);
    assert.equal(initialRead.version, reference.visible);
    assert.equal(initialRead.record.component.props.version, reference.visible);
    assert.equal(Reflect.apply(read.acknowledge, read, [null]), false);
    assert.equal(read.acknowledge(independentRead.snapshot), false);
    const cancelledCommit = publication.begin(readLeaseFixture.versions[1]); reference.stage(readLeaseFixture.versions[1]);
    assert.equal(read.stage(cancelledCommit, readOwner(reference.staged)), true);
    assert.equal(read.snapshot, initialRead);
    assert.equal(publication.cancel(cancelledCommit), true); reference.cancel();
    const closesBeforeZeroGrant = readCloses;
    assert.equal(read.advanceRetirement({ maxItems: 0, maxBytes: 4096 }).kind, "blocked");
    assert.equal(readCloses, closesBeforeZeroGrant);
    const readCancelTurns = drainRead(read);
    assert.equal(read.snapshot, initialRead);
    assert.equal(read.hasCapacity, true);
    const publishedCommit = publication.begin(readLeaseFixture.versions[1]); reference.stage(readLeaseFixture.versions[1]);
    assert.equal(read.stage(publishedCommit, readOwner(reference.staged)), true);
    assert.equal(publication.publish(publishedCommit), true); reference.publish();
    const publishedRead = read.snapshot;
    assert.notEqual(publishedRead, initialRead);
    assert.equal(publishedRead.version, reference.visible);
    assert.equal(read.acknowledge(initialRead), true);
    assert.equal(read.acknowledge(publishedRead), true);
    const readRetirementTurns = drainRead(read); reference.retire();
    assert.equal(read.snapshot, publishedRead);
    assert.equal(read.acknowledge(initialRead), false);
    const independentReadCloseTurns = closeRead(independentRead);
    const readCloseTurns = closeRead(read);
    assert.equal(readCaptures, 4);
    assert.equal(readCloses, 4);
    let readCancellations = 0;
    for (let cutoff = 0; cutoff <= readRetirementTurns; cutoff++) {
      const capturesBefore = readCaptures; const closesBefore = readCloses;
      const candidate = new OwnedUiNodeReadLease(readLeaseFixture.node, readLeaseFixture.versions[0], readOwner(readLeaseFixture.versions[0]));
      assert.equal(candidate.offer(readLeaseFixture.versions[1], readOwner(readLeaseFixture.versions[1])), true);
      const candidateNext = candidate.snapshot;
      const captureAfterTwoRoots = readCaptures;
      assert.equal(candidate.offer(readLeaseFixture.versions[2], readOwner(readLeaseFixture.versions[2])), false);
      assert.equal(readCaptures, captureAfterTwoRoots);
      assert.equal(candidate.acknowledge(candidateNext), true);
      for (let turn = 0; turn < cutoff; turn++) candidate.advanceRetirement(readGrant);
      closeRead(candidate);
      assert.equal(readCaptures - capturesBefore, 2);
      assert.equal(readCloses - closesBefore, 2);
      readCancellations++;
    }
    assert.equal(readCaptures, readCloses);
    process.stdout.write(JSON.stringify({ laws: 25, grants, tableCloseTurns, retired: retired.length, cancellationCloseTurns, cancellations: 1, siblingInsertions: 2, validationTurns, validationCloseTurns, validationCancellations, validationCases: 1, hashBytes, hashCalls, hashChunks, hashCloseTurns, hashCancellations: hashFrontiers.length, hashCases: 1, readCancelTurns, readRetirementTurns, readCloseTurns, independentReadCloseTurns, readCancellations, readCaptures, readCloses, readCases: 1 }));
  `;
  const environment = { ...process.env };
  delete environment.NO_COLOR;
  const child = spawnSync("node", ["--experimental-strip-types", "--input-type=module", "--eval", program], { encoding: "utf8", env: environment });
  assert.equal(child.status, 0, child.stderr);
  assert.equal(child.signal, null);
  const result = JSON.parse(child.stdout) as { laws: number; grants: number; tableCloseTurns: number; retired: number; cancellationCloseTurns: number; cancellations: number; siblingInsertions: number; validationTurns: number; validationCloseTurns: number; validationCancellations: number; validationCases: number; hashBytes: number; hashCalls: number; hashChunks: number; hashCloseTurns: number; hashCancellations: number; hashCases: number; readCancelTurns: number; readRetirementTurns: number; readCloseTurns: number; independentReadCloseTurns: number; readCancellations: number; readCaptures: number; readCloses: number; readCases: number };
  assert.equal(result.laws, 25);
  assert(result.grants > 0 && result.tableCloseTurns > 0 && result.cancellationCloseTurns > 0);
  assert(result.validationTurns > 0 && result.validationCloseTurns > 0 && result.validationCancellations > 0);
  assert(result.hashBytes > 0 && result.hashCalls > 0 && result.hashChunks > 1 && result.hashCloseTurns > 0 && result.hashCancellations > 0);
  assert(result.readCancelTurns > 0 && result.readRetirementTurns > 0 && result.readCloseTurns > 0 && result.independentReadCloseTurns > 0 && result.readCancellations > 0);
  assert.equal(result.readCaptures, result.readCloses);
  assert.deepEqual({ retired: result.retired, cancellations: result.cancellations, siblingInsertions: result.siblingInsertions, validationCases: result.validationCases, hashCases: result.hashCases, readCases: result.readCases }, { retired: 4, cancellations: 1, siblingInsertions: 2, validationCases: 1, hashCases: 1, readCases: 1 });
  console.log(`[DEBUG] retained UI native strip-only oracle: ${result.laws} fixture laws, ${result.grants} grants, ${result.retired} one-time retirements, ${result.cancellations} table cancellation, ${result.validationCancellations} validation cancellations, ${result.hashBytes} hash bytes in ${result.hashChunks} chunks over ${result.hashCalls} advances, ${result.hashCancellations} hash cancellations, ${result.readCancellations} read cancellations, ${result.readCaptures} read captures/releases, ${result.readCancelTurns} cancel turns, ${result.readRetirementTurns} publish-retirement turns, ${result.readCloseTurns} final read close turns`);
}

class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { rest } = resolveTestLevel(segments);
    retainedUiNativeStripOnly();
    runVitest(this.root, rest);
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
