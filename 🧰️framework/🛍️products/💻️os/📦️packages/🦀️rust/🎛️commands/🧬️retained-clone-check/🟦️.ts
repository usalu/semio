/** 🧬️ Retained clone command: the fixture shapes of retained clone choices, grants, ordered map paging and paged copies, and the retained clone check.
 *
 * @see ../../📜️script.ts — the package script that registers the commands of this module
 */
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { BundleScript } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { readModuleTreeSource } from "@semio-tech/framework-compiler-ts/syntax/module-tree";

//#region 🧬️RetainedCloneFixtures
/** 🎭️ One example payload for retained-clone choice behavior. */
type RetainedCloneChoiceV1 =
  | { readonly kind: "unit" }
  | { readonly kind: "text"; readonly text: string }
  | { readonly kind: "nested"; readonly rows: readonly { readonly id: number; readonly text: string }[] };

/** 🎟️ The copy grant a retained-clone corpus hands one step. */
type RetainedCloneGrantV1 = { readonly maximumItems: number; readonly maximumCopyBytes: number; readonly maximumCapacityBytes: number; readonly maximumDepth: number };

/** 📦️ `🧬️retained-clone/🧫️fixtures/📦️nested` — the nested-value clone corpus describes. */
type RetainedCloneNestedFixtureV1 = {
  readonly source: {
    readonly title: string;
    readonly optional: string | null;
    readonly choice: RetainedCloneChoiceV1;
    readonly choices: readonly RetainedCloneChoiceV1[];
    readonly fixedArray: readonly [number, number, number, number];
    readonly pair: readonly [string, number];
    readonly triple: readonly [string, number, string];
    readonly labels: Readonly<Record<string, string>>;
  };
  readonly payloadByteLength: number;
  readonly payloadModulo: number;
  readonly recursiveDepth: number;
  readonly grant: RetainedCloneGrantV1;
  readonly insufficientCapacityBytes: number;
  readonly immutableLease: { readonly captured: string; readonly externalAfterCapture: string; readonly expected: string };
  readonly cancellationStops: readonly number[];
};

/** 🎯️ The state an ordered-map operation row must leave behind. */
type RetainedOrderedMapExpectationV1 = { readonly found: boolean; readonly ordinal: number; readonly entryCount: number };

/** 🗺️ `🗺️ordered-map/🧫️fixtures/📦️paging` — a lookup carries no value; an insert or duplicate carries the value it offers. */
type RetainedOrderedMapOperationV1 =
  | { readonly kind: "lookup"; readonly key: string; readonly expected: RetainedOrderedMapExpectationV1 }
  | { readonly kind: "insert" | "duplicate"; readonly key: string; readonly value: string; readonly expected: RetainedOrderedMapExpectationV1 };

/** 🗺️ `🗺️ordered-map/🧫️fixtures/📦️paging` — the paging, growth and immutable-lookup corpus describes. */
type RetainedOrderedMapPagingFixtureV1 = {
  readonly pageCapacity: 16;
  readonly entryCount: number;
  readonly keyPrefix: string;
  readonly valuePrefix: string;
  readonly longKeyByteLength: number;
  readonly comparisonGrant: { readonly maximumItems: number; readonly maximumBytes: number };
  readonly progressChannels: { readonly comparisonOnly: true; readonly capacityOnly: true; readonly minimumMovedItems: number };
  readonly repeatedGrowth: { readonly entryCount: 32; readonly insertions: 17; readonly keyPrefix: string; readonly valuePrefix: string; readonly expectedEntryCount: 49 };
  readonly immutableLookup: { readonly capturedTarget: string; readonly externalAfterCapture: string; readonly expectedOrdinal: number };
  readonly operations: readonly RetainedOrderedMapOperationV1[];
};

/** 🧩️ `🧩preparation/🧪️fixtures/📦️lifecycle` — one preparation outcome row. */
type RetainedClonePreparationCaseV1 = {
  readonly id: string;
  readonly kind: "success" | "rejection" | "cancel" | "stale" | "fault" | "overBudget";
  readonly initial: number;
  readonly value: number;
  readonly interruptAfterTurns: number;
  readonly expected: { readonly published: boolean; readonly value: number; readonly history: number; readonly terminalEmpty: true };
};

/** 🧩️ `🧩preparation/🧪️fixtures/📦️lifecycle` — the preparation lifecycle corpus describes. */
type RetainedClonePreparationFixtureV1 = {
  readonly grant: { readonly maximumItems: 1; readonly maximumBytes: number; readonly maximumDepth: number };
  readonly largeCapacity: { readonly stringByteLength: number; readonly expectedCode: "retained-clone.step-grant-too-small" };
  readonly handoff: {
    readonly sourceEntries: number;
    readonly copiedEntriesBeforeLoss: number;
    readonly closeGrant: { readonly maximumItems: 1; readonly maximumBytes: 4096 };
    readonly expected: { readonly driverOwnsCursor: true; readonly sourcePreserved: true; readonly closeRequiresMultipleTurns: true; readonly terminalEmpty: true };
  };
  readonly cases: readonly RetainedClonePreparationCaseV1[];
};

/** 📋️ `📋️paged-list/🧫️fixtures/📦️copy` — the paged-list copy corpus describes. */
type RetainedPagedListCopyFixtureV1 = {
  readonly maximumEntries: 1024;
  readonly entryCount: 513;
  readonly valuePrefix: string;
  readonly grant: { readonly maximumItems: 5; readonly maximumCopyBytes: 32; readonly maximumCapacityBytes: 4096; readonly maximumDepth: 64 };
  readonly cancellationAfterEntries: 173;
  readonly expected: { readonly ordered: true; readonly sourcePreserved: true; readonly copyRequiresMultipleTurns: true; readonly closeRequiresMultipleTurns: true; readonly terminalEmpty: true };
};

/** 📦️ `📦️paged/🧫️fixtures/📦️owners` — retained paged text and map ownership corpus. */
type RetainedPagedOwnerFixtureV1 = {
  readonly grant: { readonly maximumItems: number; readonly maximumCopyBytes: number; readonly maximumCapacityBytes: 4096; readonly maximumDepth: number };
  readonly text: { readonly segment: string; readonly repetitions: number; readonly capacityBytes: 8192 };
  readonly bytes: { readonly length: 6145; readonly capacityBytes: 8192; readonly multiplier: number; readonly increment: number };
  readonly map: { readonly entryCount: 70; readonly capacityEntries: 128; readonly keyPrefix: string; readonly valuePrefix: string };
  readonly expected: { readonly copyRequiresMultipleTurns: true; readonly closeRequiresMultipleTurns: true; readonly ordered: true; readonly terminalEmpty: true };
};
//#endregion 🧬️RetainedCloneFixtures

/** 🧬️ Verifies retained-clone behavior against the platform structured-clone oracle. */
export class RetainedCloneCheckScript extends BundleScript {
  run(segments: string[]): void {
    if (segments.length) throw new Error("retained-clone-check accepts no arguments");
    const root = join(this.repoRoot, "🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🧪️tests/🧫️fixtures/📦️nested");
    
    const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8"));
    
    
    assert.deepEqual(structuredClone(fixture), fixture);
    assert.equal(fixture.source.choices.length, 3);
    assert.equal(fixture.source.fixedArray.length, 4);
    assert(fixture.grant.maximumDepth >= fixture.recursiveDepth);
    const capturedLease = structuredClone(fixture.immutableLease.captured);
    let externalLease = fixture.immutableLease.captured;
    externalLease = fixture.immutableLease.externalAfterCapture;
    assert.equal(capturedLease, fixture.immutableLease.expected);
    assert.notEqual(externalLease, capturedLease);
    assert(fixture.cancellationStops.some((stop: number) => stop > fixture.payloadByteLength / fixture.grant.maximumCopyBytes));
    const mapRoot = join(this.repoRoot, "🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🧪️tests/🧫️fixtures/📦️paging");
    
    const mapDescriptor = JSON.parse(readFileSync(join(mapRoot, "🔣️.json"), "utf8"));
    
    
    const entries = Array.from({ length: mapDescriptor.entryCount }, (_, ordinal) => [`${mapDescriptor.keyPrefix}${ordinal.toString().padStart(4, "0")}`, `${mapDescriptor.valuePrefix}${ordinal}`] as [string, string]);
    const oracle = new Map(entries);
    const ordered = [...oracle.entries()].sort(([left], [right]) => left < right ? -1 : left > right ? 1 : 0);
    const lowerBound = (key: string): number => {
      let low = 0;
      let high = ordered.length;
      while (low < high) {
        const middle = low + Math.floor((high - low) / 2);
        const row=ordered[middle];assert(row,"Actual sorted lookup row required");if (row[0] < key) low = middle + 1;
        else high = middle;
      }
      return low;
    };
    for (const operation of mapDescriptor.operations) {
      const ordinal = lowerBound(operation.key);
      const found = ordinal < ordered.length && ordered[ordinal]?.[0] === operation.key;
      if (operation.kind === "insert") ordered.splice(ordinal, 0, [operation.key, operation.value]);
      assert.equal(operation.kind === "insert" ? true : found, operation.expected.found, operation.kind);
      assert.equal(ordinal, operation.expected.ordinal, operation.kind);
      assert.equal(ordered.length, operation.expected.entryCount, operation.kind);
    }
    const longKey = "k".repeat(mapDescriptor.longKeyByteLength);
    assert.equal(new TextEncoder().encode(longKey).byteLength, mapDescriptor.longKeyByteLength);
    assert(longKey < `${longKey}z`);
    assert.equal(mapDescriptor.progressChannels.comparisonOnly, true);
    assert.equal(mapDescriptor.progressChannels.capacityOnly, true);
    assert(mapDescriptor.progressChannels.minimumMovedItems >= 1);
    const growth = new Map(Array.from({ length: mapDescriptor.repeatedGrowth.entryCount }, (_, ordinal) => [`${mapDescriptor.repeatedGrowth.keyPrefix}${(ordinal * 2).toString().padStart(4, "0")}`, `${mapDescriptor.repeatedGrowth.valuePrefix}${ordinal}`]));
    for (let ordinal = 0; ordinal < mapDescriptor.repeatedGrowth.insertions; ordinal += 1) growth.set(`${mapDescriptor.repeatedGrowth.keyPrefix}${(ordinal * 2 + 1).toString().padStart(4, "0")}`, `${mapDescriptor.repeatedGrowth.valuePrefix}insert-${ordinal}`);
    assert.equal(growth.size, mapDescriptor.repeatedGrowth.expectedEntryCount);
    const capturedTarget = structuredClone(mapDescriptor.immutableLookup.capturedTarget);
    let externalTarget = mapDescriptor.immutableLookup.capturedTarget;
    externalTarget = mapDescriptor.immutableLookup.externalAfterCapture;
    assert.equal(lowerBound(capturedTarget), mapDescriptor.immutableLookup.expectedOrdinal);
    assert.notEqual(externalTarget, capturedTarget);
    const preparationRoot = join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧬️snapshot-clone/🧪️tests/🧫️fixtures/📦️lifecycle");
    
    const preparationFixture = JSON.parse(readFileSync(join(preparationRoot, "🔣️.json"), "utf8"));
    
    
    assert(preparationFixture.largeCapacity.stringByteLength > preparationFixture.grant.maximumBytes);
    assert.equal(preparationFixture.largeCapacity.expectedCode, "retained-clone.step-grant-too-small");
    const handoffSource = Array.from({ length: preparationFixture.handoff.sourceEntries }, (_, ordinal) => `handoff-${ordinal.toString().padStart(2, "0")}`);
    const handoffSourceOracle = structuredClone(handoffSource);
    let activeCursor: null | { retainedSource: string[]; partialOutput: string[] } = {
      retainedSource: handoffSourceOracle,
      partialOutput: handoffSourceOracle.slice(0, preparationFixture.handoff.copiedEntriesBeforeLoss),
    };
    const storeCloseQueue: { retainedSource: string[]; partialOutput: string[] }[] = [activeCursor];
    activeCursor = null;
    let handoffTurns = 0;
    for (;;) {
      const entry=storeCloseQueue[0];if(!entry||!entry.partialOutput.length)break;
      entry.partialOutput.pop();
      handoffTurns += 1;
    }
    storeCloseQueue.shift();
    assert.equal(activeCursor, null);
    assert.deepEqual(handoffSource, handoffSourceOracle);
    assert(handoffTurns > 1);
    assert.equal(storeCloseQueue.length, 0);
    assert.equal(preparationFixture.handoff.expected.driverOwnsCursor, true);
    assert.equal(preparationFixture.handoff.expected.sourcePreserved, true);
    assert.equal(preparationFixture.handoff.expected.closeRequiresMultipleTurns, true);
    assert.equal(preparationFixture.handoff.expected.terminalEmpty, true);
    const storeSource = readModuleTreeSource(join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/⚙️engine/🧱️store-components/📥️lifted/🏪️store/🦀️.rs"));
    const handoffMethod = storeSource.slice(storeSource.indexOf("pub fn handoff_batch_publication_close("), storeSource.indexOf("pub fn maintenance_retirements_terminal_is_empty", storeSource.indexOf("pub fn handoff_batch_publication_close(")));
    assert(handoffMethod.indexOf("displaced_retirements.reserve(1)?") < handoffMethod.indexOf("publication.take()"), "Store reserves surviving close authority before taking the scheduler owner");
    assert(readFileSync(join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📬️one-item-owners/📥️lifted/📬️publication/🦀️.rs"), "utf8").includes("ErasedSnapshotRetirement for ArtifactStoreBatchPublication"));
    const pluginSource = readFileSync(join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs"), "utf8");
    assert(pluginSource.includes("fn handoff_mounted_durable_publication("));
    assert(pluginSource.match(/handoff_mounted_durable_publication\(/g)?.length ?? 0 >= 4, "cancel, stale and terminal-error paths transfer durable publications");
    const toolRunSource = readModuleTreeSource(join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏯️tool-run/🦀️.rs"));
    assert(toolRunSource.includes("store.handoff_batch_publication_close(&mut owner)"), "tool-run replacement and abort transfer publications to Store maintenance");
    for (const row of preparationFixture.cases) {
      let value = row.initial;
      let history = 0;
      if (row.kind === "success") {
        value = row.value;
        history = 1;
      } else if (row.kind === "stale") {
        value += 1;
        history = 1;
      }
      assert.equal(value, row.expected.value, row.id);
      assert.equal(history, row.expected.history, row.id);
      assert.equal(row.expected.published, row.kind === "success", row.id);
      assert.equal(row.expected.terminalEmpty, true, row.id);
    }
    const pagedRoot = join(this.repoRoot, "🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/📋️paged-list/🧪️tests/🧫️fixtures/📦️copy");
    
    const pagedFixture = JSON.parse(readFileSync(join(pagedRoot, "🔣️.json"), "utf8"));
    
    
    const pagedValues = Array.from({ length: pagedFixture.entryCount }, (_, ordinal) => `${pagedFixture.valuePrefix}${ordinal}`);
    const pagedOracle = structuredClone(pagedValues);
    assert.deepEqual(pagedOracle, pagedValues);
    assert.equal(pagedOracle.length, pagedFixture.entryCount);
    assert(pagedFixture.entryCount <= pagedFixture.maximumEntries);
    assert(pagedFixture.cancellationAfterEntries > 0 && pagedFixture.cancellationAfterEntries < pagedFixture.entryCount);
    assert(new TextEncoder().encode(pagedOracle.join("")).byteLength > pagedFixture.grant.maximumCapacityBytes);
    assert.equal(pagedFixture.expected.ordered, true);
    assert.equal(pagedFixture.expected.sourcePreserved, true);
    assert.equal(pagedFixture.expected.copyRequiresMultipleTurns, true);
    assert.equal(pagedFixture.expected.closeRequiresMultipleTurns, true);
    assert.equal(pagedFixture.expected.terminalEmpty, true);
    const pagedOwnerRoot = join(this.repoRoot, "🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/📦️paged/🧪️tests/🧫️fixtures/📦️owners");
    
    const pagedOwnerFixture = JSON.parse(readFileSync(join(pagedOwnerRoot, "🔣️.json"), "utf8"));
    
    
    const pagedTextOracle = structuredClone(pagedOwnerFixture.text.segment.repeat(pagedOwnerFixture.text.repetitions));
    assert.equal(new TextEncoder().encode(pagedTextOracle).byteLength <= pagedOwnerFixture.text.capacityBytes, true);
    assert(new TextEncoder().encode(pagedTextOracle).byteLength > pagedOwnerFixture.grant.maximumCopyBytes);
    const pagedBytesSource = Uint8Array.from({ length: pagedOwnerFixture.bytes.length }, (_, ordinal) => (ordinal * pagedOwnerFixture.bytes.multiplier + pagedOwnerFixture.bytes.increment) & 255);
    const pagedBytesOracle = structuredClone(pagedBytesSource);
    assert.deepEqual(pagedBytesOracle, pagedBytesSource);
    assert.equal(pagedBytesOracle.byteLength <= pagedOwnerFixture.bytes.capacityBytes, true);
    assert(pagedBytesOracle.byteLength > pagedOwnerFixture.grant.maximumCopyBytes);
    const pagedMapOracle = structuredClone(Array.from({ length: pagedOwnerFixture.map.entryCount }, (_, ordinal) => [`${pagedOwnerFixture.map.keyPrefix}${ordinal.toString().padStart(3, "0")}`, `${pagedOwnerFixture.map.valuePrefix}${ordinal}`] as const));
    assert.equal(pagedMapOracle.length, pagedOwnerFixture.map.entryCount);
    assert(pagedMapOracle.length <= pagedOwnerFixture.map.capacityEntries);
    assert.deepEqual(pagedMapOracle.map(([key]) => key), [...pagedMapOracle].map(([key]) => key));
    assert.equal(pagedOwnerFixture.expected.copyRequiresMultipleTurns, true);
    assert.equal(pagedOwnerFixture.expected.closeRequiresMultipleTurns, true);
    assert.equal(pagedOwnerFixture.expected.ordered, true);
    assert.equal(pagedOwnerFixture.expected.terminalEmpty, true);
    console.log(`retained-clone-check: choices=${fixture.source.choices.length} payload=${fixture.payloadByteLength} cancellation=${fixture.cancellationStops.length} orderedMap=${ordered.length} growth=${growth.size} preparation=${preparationFixture.cases.length} paged=${pagedOracle.length} pagedText=${new TextEncoder().encode(pagedTextOracle).byteLength} pagedBytes=${pagedBytesOracle.byteLength} pagedMap=${pagedMapOracle.length}`);
  }
}
