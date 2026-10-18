import { test, expect } from "bun:test";
import { Buffer } from "node:buffer";
import fixture from "../../../../📏️retirement/🧪️tests/🧫️fixtures/🔣️.json" with { type: "json" };
import { applyPatch } from "fast-json-patch";
import { readStoreSourceText } from "@semio-tech/framework/test/source-text";
import Ajv from "ajv";
import { orderedRetirementSchema as orderedRetirement } from "@semio-tech/value-ts/ordered/retirement/schema";
import { retirementContractSchema as retirementContract } from "@semio-tech/value-ts/retirement/contract/schema";
import { readModuleTreeSource, sliceModuleTreeSource } from "@semio-tech/framework-compiler-ts/syntax/module-tree";
import { fileURLToPath } from "node:url";
import { spawnSync } from "node:child_process";

test("catalog source birth precedes allocations and retains unfunded factory tickets", async () => {
  const sources = [64, 64, 64, 64].map(bytes => Buffer.allocUnsafeSlow(bytes));
  const extent = sources.reduce((sum, owner) => sum + owner.byteLength, 0);
  for (const row of fixture.sourceBirthGrants) {
    const admitted = row.items > 0 && row.capacity >= extent && row.depth > 0;
    const next = applyPatch({ originals: 4, tickets: 0, born: 0 }, admitted ? [{ op: "replace", path: "/born", value: extent }] : [], true).newDocument;
    expect(next.originals).toBe(4);
    expect(next.tickets).toBe(0);
    expect(next.born).toBe(row.allocated);
  }
  const root = readStoreSourceText();
  const start = root.indexOf("pub fn admit_sources<");
  expect(start).toBeGreaterThan(0);
  const body = root.slice(start, root.indexOf("    pub fn new(", start));
  expect(body.indexOf("grant.maximum_capacity_bytes < capacity")).toBeLessThan(body.indexOf("Self::from_sources("));
  const sourceBody = body.slice(body.indexOf("fn from_sources<"));
  expect(sourceBody).toContain("Arc::new(snapshots)");
  expect(sourceBody).toContain("Box::new(disposer)");
  expect(body).toContain("factory_retirement_tickets: semio_framework_value::FactoryChildTickets");
  expect(body).not.toContain("preborn_factory_retirement");
  expect(body).toContain("retained_capacity_bytes: capacity");
});

test("member catalog admission funds ingress before source construction and progresses tickets separately", async () => {
  for (const row of fixture.sourceBirthGrants) {
    const original = { request: 1, constructorCalls: 0, tickets: 0 };
    const accepted = row.items > 0 && row.capacity >= 256 && row.depth > 0;
    const projected = applyPatch(original, accepted ? [{ op: "replace", path: "/constructorCalls", value: 1 }] : [], true).newDocument;
    expect(projected.request).toBe(1);
    expect(projected.constructorCalls).toBe(accepted ? 1 : 0);
    expect(projected.tickets).toBe(0);
    expect(accepted ? Buffer.allocUnsafeSlow(256).byteLength : 0).toBe(row.allocated);
  }
  const root = await Bun.file(new URL("../../../../../../🏗️initialization/🏠️owners/🦀️.rs", import.meta.url)).text();
  const start = root.indexOf("pub fn admit_source_constructor");
  expect(start).toBeGreaterThan(0);
  const constructor = root.slice(start, root.indexOf("    pub fn new(", start));
  expect(constructor.indexOf("source_constructor_capacity")).toBeLessThan(constructor.indexOf("construct()"));
  const capacityBody = constructor.slice(constructor.indexOf("fn source_constructor_capacity"));
  expect(capacityBody).toContain("grant.maximum_capacity_bytes < capacity");
  expect(capacityBody).toContain("owners: None");
  const memberContract=await Bun.file(new URL("../../../../../../🏗️initialization/🏠️owners/🏭️member/🧬️schema/🦀️.rs",import.meta.url)).text();
  expect(memberContract).toContain("fn member_store_owners(&self,grant:RetainedCloneGrant)->Result<");
  const operation = await Bun.file(new URL("../../../../🏭️operation/🦀️.rs", import.meta.url)).text();
  expect(operation).toContain("producer.member_store_owners(source_grant)");
  expect(operation).toContain("maximum_capacity_bytes: source_capacity");
  expect(operation).toContain("progress.fits(source_grant)");
  expect(operation).toContain("genesis_request: ManuallyDrop::new(genesis_request)");
  expect(operation).toContain("owners.constructor_is_complete()");
  expect(operation).toContain("owners.admit_constructor(grant)");
  expect(operation).not.toContain("Some(P::member_store_owners())");
});

test("retained member open preserves supplied currencies and every original child receipt", async () => {
  expect(fixture.retainedMemberClose.constructorAdmission).toBe("before-original-payload-transfer");
  expect(fixture.retainedMemberClose.terminalReceipt).toBe("full-final-catalog-release");
  for (const row of fixture.boxGrants) {
    const source = Buffer.allocUnsafeSlow(64);
    const admitted = row.items > 0 && row.depth > 0 && row.release >= source.byteLength;
    const state = applyPatch({ retained: source.byteLength, released: 0 }, admitted ? [{ op: "replace", path: "/retained", value: 0 }, { op: "replace", path: "/released", value: source.byteLength }] : [], true).newDocument;
    expect(state.released).toBe(row.released);
    expect(state.retained + state.released).toBe(source.byteLength);
  }
  const root = await Bun.file(new URL("../../../../🦀️.rs", import.meta.url)).text();
  const start = root.indexOf("impl<P, M> ErasedSnapshotRetirement for MemberStoreOpenRetained");
  const close = root.slice(start, root.indexOf("impl<P, M> Drop for MemberStoreOpenRetained", start));
  expect(close).not.toContain("SnapshotRetirementStep");
  expect(close).not.toContain("retirement::owned_retirement(");
  expect(close).toContain("maximum_depth: grant.maximum_depth - 1");
  expect(close).toContain("admit_artifact_retirement");
  expect(close).toContain("admit_typed_controlled_retirement");
  expect(close).toContain("*self.initial = Some(initial)");
  expect(close).toContain("*self.history = Some(history)");
  expect(close).toContain("RetainedCloneStep::Complete(step.progress())");
});

test("optional preparation source birth funds the whole original Arc tree before invocation", async () => {
  const originals = [256, 64].map(bytes => Buffer.allocUnsafeSlow(bytes));
  const capacity = originals.reduce((bytes, owner) => bytes + owner.byteLength, 0);
  for (const row of fixture.preparationSourceBirthGrants) {
    const funded = row.items > 0 && row.depth > 0 && row.capacity >= capacity;
    const projected = applyPatch({ calls: 0, capacity: 0, tickets: 0 }, funded ? [{ op: "replace", path: "/calls", value: 1 }, { op: "replace", path: "/capacity", value: capacity }] : [], true).newDocument;
    expect(projected.capacity).toBe(row.allocated);
    expect(projected.calls).toBe(funded ? 1 : 0);
    expect(projected.tickets).toBe(0);
  }
  const root = readStoreSourceText();
  const start = root.indexOf("pub fn admit_source_constructor_with_one_item_preparation");
  expect(start).toBeGreaterThan(0);
  const body = root.slice(start, root.indexOf("    fn from_sources", start));
  expect(body.indexOf("source_constructor_capacity")).toBeLessThan(body.indexOf("construct()"));
  expect(body).toContain("owners.one_item_preparation = Some(preparation)");
  expect(body).toContain("retained_capacity_bytes: capacity");
  expect(body).not.toContain("admit_constructor(");
});

test("original source iterator retains its actual allocation extent through prefix handoff", async () => {
  const neutral = await Bun.file(new URL("../../../../../../♻️retirement/📦️backing/🧪️tests/🧫️fixtures/🔣️.json", import.meta.url)).json();
  const law = neutral.sourceIterator;
  const source = Buffer.allocUnsafeSlow(law.backingCapacity * 8);
  law.values.forEach((value: string, index: number) => source.writeBigUInt64LE(BigInt(value), index * 8));
  const prefix = source.subarray(law.consumedPrefix * 8, law.values.length * 8);
  expect(prefix.buffer).toBe(source.buffer);
  expect(prefix.byteOffset - source.byteOffset).toBe(law.consumedPrefix * 8);
  expect(Array.from({ length: prefix.byteLength / 8 }, (_, index) => prefix.readBigUInt64LE(index * 8).toString())).toEqual(law.values.slice(law.consumedPrefix));
  for (const grant of [{ items: 0, release: source.byteLength }, { items: 1, release: source.byteLength - 1 }, { items: 1, release: source.byteLength }]) {
    const funded = grant.items > 0 && grant.release >= source.byteLength;
    const state = applyPatch({ retained: source.byteLength, released: 0 }, funded ? [{ op: "replace", path: "/retained", value: 0 }, { op: "replace", path: "/released", value: source.byteLength }] : [], true).newDocument;
    expect(state.released).toBe(funded ? source.byteLength : 0);
    expect(state.retained + state.released).toBe(source.byteLength);
  }
  const backing = await Bun.file(new URL("../../../../../../♻️retirement/📦️backing/🦀️.rs", import.meta.url)).text();
  const start = backing.indexOf("pub struct SourceIterator");
  const owner = backing.slice(start);
  expect(owner).toContain("source_bytes = extent::<T>(source.capacity())");
  expect(owner).toContain("source.into_iter()");
  expect(owner).toContain("*self.pending = Some(value)");
  expect(owner).toContain("release_bytes: self.source_bytes");
  expect(owner).toContain("maximum_depth: grant.maximum_depth - 1");
  expect(owner).toContain("RetainedCloneStep::Progress(step.progress())");
  expect(owner).not.toContain("SnapshotRetirementStep");
});

test("original SPR parent retains completed decoder custody and full factory receipts", async () => {
  const root = readStoreSourceText();
  const start = root.indexOf("struct ArtifactOwnedSprEditAuthority<");
  const owner = root.slice(start, root.indexOf("struct ArtifactOwnedSprEditDecoder", start));
  expect(owner).toContain("completed_arrays:");
  expect(owner).toContain("*self.completed_arrays[usize::from(field_id == 4)] = Some(authority)");
  expect(owner).toContain("*authority.values = Some(values)");
  expect(owner).toContain("*self.value = Some(value)");
  expect(owner).toContain("*self.forwards = Some(values)");
  expect(owner).toContain("*self.inverse = Some(values)");
  expect(owner).toContain("RetainedCloneStep::Complete(step.progress())");
  expect(owner).toContain("self.factories_close.iter().all(Option::is_none)");
  expect(owner).not.toContain("SnapshotRetirementStep");
  expect(owner).not.toContain("fn next_close_byte_demand");
});

test("source factory tree quotes capacity and depth before its constructor callback", async () => {
  for (const row of fixture.sourceTreeBirths) {
    const original = Buffer.allocUnsafeSlow(row.sourceBytes);
    const wrapper = Buffer.allocUnsafeSlow(row.wrapperBytes);
    const result = applyPatch({ capacity: original.byteLength, depth: row.sourceDepth }, [
      { op: "replace", path: "/capacity", value: original.byteLength + wrapper.byteLength },
      { op: "replace", path: "/depth", value: row.sourceDepth + 1 },
    ], true).newDocument;
    expect(result).toEqual({ capacity: row.quotedBytes, depth: row.quotedDepth });
    for (const depth of [0, row.sourceDepth, row.quotedDepth]) expect(depth >= result.depth).toBe(depth === row.quotedDepth);
  }
  const rootUrl = new URL("../../../../../../🦀️.rs", import.meta.url);
  const root = await Bun.file(rootUrl).text();
  const memberContract=await Bun.file(new URL("🏗️initialization/🏠️owners/🏭️member/🧬️schema/🦀️.rs",rootUrl)).text();
  expect(memberContract.includes("fn member_store_owners_birth_demand(&self)->Result<")).toBe(true);
  expect(root.includes("fn member_store_owners_birth_bytes()")).toBe(false);
  const wire = await Bun.file(new URL("🧵️operation-wire/🦀️.rs", rootUrl)).text();
  const start = wire.indexOf("pub fn operation_wire_preparation_factory_source_birth_demand");
  expect(start).toBeGreaterThan(0);
  const quote = wire.slice(start, wire.indexOf("pub fn operation_wire_preparation_factory<", start));
  expect(quote).toContain("factory_arc_birth_bytes");
  expect(quote).toContain("inner.capacity_bytes");
  expect(quote).toContain("inner.depth.checked_add(1)");
  expect(quote).not.toContain("factory_constructor_birth_bytes");
  const admissionStart = root.indexOf("fn source_constructor_capacity");
  const admission = root.slice(admissionStart, root.indexOf("    fn from_sources", admissionStart));
  expect(admission).toContain("additional_birth.depth.max(1)");
  expect(admission).toContain("let depth = additional_birth.depth.max(1)");
  expect(admission).toContain("grant.maximum_depth < depth");
});

test("bounded config factory admits actual typed owner and preserves full receipts", async () => {
  const neutral = await Bun.file(new URL("../../../../../../♻️retirement/📦️backing/🧪️tests/🧫️fixtures/🔣️.json", import.meta.url)).json();
  const law = neutral.boundedConfigFactory;
  const payload = Buffer.from(law.body.repeat(law.repeat), "utf8");
  const oracle = new Ajv({ strict: false });
  oracle.addSchema(orderedRetirement);
  oracle.addSchema(retirementContract);
  const validate = oracle.compile({ $ref: retirementContract.$id + "#/$defs/Grant" });
  for (const release of [0, 1, payload.byteLength - 1, payload.byteLength]) {
    const grant = { maximumItems: 1, maximumCopyBytes: law.maximumCopyBytes, maximumCapacityBytes: 0, maximumReleaseBytes: release, maximumDepth: 1 };
    expect(validate(grant)).toBe(true);
    const receipt = applyPatch({ retained: payload.byteLength, released: 0 }, release >= payload.byteLength ? [{ op: "replace", path: "/retained", value: 0 }, { op: "replace", path: "/released", value: payload.byteLength }] : [], true).newDocument;
    expect(receipt.retained + receipt.released).toBe(payload.byteLength);
    expect(receipt.released).toBe(release === payload.byteLength ? payload.byteLength : 0);
  }
  const plugin = await Bun.file(new URL("../../../../../../../🔌️plugin/🦀️.rs", import.meta.url)).text();
  const start = plugin.indexOf("struct BoundedConfigRetirementFactory<");
  const owner = plugin.slice(start, plugin.indexOf("pub fn bounded_config_store_owners", start));
  expect(plugin.includes("struct BoundedConfigValueRetirement")).toBe(false);
  expect(owner).toContain("T: semio_framework_value::retirement::RetireOwned");
  expect(owner).toContain("fn retirement_birth_bytes");
  expect(owner).toContain("admit_owned_retirement(value, grant)");
  expect(owner).toContain("admit_shared_retirement(snapshot, grant, false)");
  expect(owner).not.toContain("drop(value)");
  expect(owner).not.toContain("SnapshotRetirementStep");
});

test("hydration close preserves original owners and separate physical currencies", async () => {
  const ajv = new Ajv({ strict: false });
  ajv.addSchema(orderedRetirement);
  ajv.addSchema(retirementContract);
  const validGrant = ajv.compile({ $ref: retirementContract.$id + "#/$defs/Grant" });
  for (const row of fixture.cases) {
    const original = Buffer.allocUnsafeSlow(row.capacityBytes);
    original.write(row.text, "utf8");
    for (const grant of [{ items: 0, release: original.byteLength }, { items: 1, release: original.byteLength - 1 }, { items: 1, release: original.byteLength }]) {
      expect(validGrant({ maximumItems: grant.items, maximumCopyBytes: 0, maximumCapacityBytes: 0, maximumReleaseBytes: grant.release, maximumDepth: 1 })).toBe(true);
      const funded = grant.items > 0 && grant.release >= original.byteLength;
      const state = applyPatch({ retained: original.byteLength, freed: 0 }, funded ? [{ op: "replace", path: "/retained", value: 0 }, { op: "replace", path: "/freed", value: original.byteLength }] : [], true).newDocument;
      expect(state.retained + state.freed).toBe(original.buffer.byteLength);
      expect(state.freed).toBe(funded ? original.byteLength : 0);
      if (!funded) expect(original.buffer.byteLength).toBe(row.capacityBytes);
    }
  }
  const source = readModuleTreeSource(new URL("../../../../../../🧾️document/📜️history/💧️hydration/🦀️.rs", import.meta.url));
  const start = source.indexOf("impl<P, M> ErasedSnapshotRetirement for RetainedPersistedDocumentHydration");
  const body = source.slice(start, source.indexOf("impl<P, M> Drop for RetainedPersistedDocumentHydration", start));
  expect(body.includes("SnapshotRetirementStep")).toBe(false);
  expect(body.includes("next_close_byte_demand")).toBe(false);
  expect(body.includes("retirement::owned_retirement(")).toBe(false);
  expect(body.includes("Arc::into_inner(")).toBe(false);
  expect(body.includes(".truncate(")).toBe(false);
  for (const axis of ["copy", "capacity", "release", "depth"]) expect(body.includes("fn next_" + axis + "_")).toBe(true);
  expect(body.includes("artifact_retirement_admit_owned")).toBe(true);
  expect(body.includes("*self.initial = Some(original)")).toBe(true);
  expect(body.includes("*self.envelope = Some(original)")).toBe(true);
  expect(body.includes("self.actor.0.capacity()")).toBe(true);
  expect(body.includes("RetainedCloneStep::Complete(step.progress())")).toBe(true);
  expect(body.includes("if self.terminal {")).toBe(false);
  expect(body.includes("if self.terminal_is_empty_unbounded()")).toBe(true);
  const demandStart = source.indexOf("fn close_demands(&self, body:");
  const demands = source.slice(demandStart, start);
  expect(demands.includes("if self.terminal {")).toBe(false);
  expect(demands.includes("if self.terminal_is_empty_unbounded()")).toBe(true);
  const terminalStart = source.indexOf("fn terminal_is_empty_unbounded(");
  expect(source.slice(terminalStart).includes("self.actor.0.capacity() == 0")).toBe(true);
  for (const ready of [false, true]) {
    const original = Buffer.allocUnsafeSlow(17);
    const state = applyPatch({ ready, retained: original.byteLength, terminal: false }, [{ op: "replace", path: "/ready", value: true }], true).newDocument;
    expect(state.terminal).toBe(false);
    expect(state.retained).toBe(original.buffer.byteLength);
  }
});

test("hydration normal pack retirement retains physical backing until granted", async () => {
  for (const row of fixture.cases) {
    const original = Buffer.allocUnsafeSlow(row.capacityBytes);
    const logical = Math.min(original.byteLength, 1);
    const shortened = original.subarray(0, original.byteLength - logical);
    expect(shortened.buffer).toBe(original.buffer);
    expect(shortened.buffer.byteLength).toBe(row.capacityBytes);
    const receipt = applyPatch({ logical: original.byteLength, physical: original.buffer.byteLength, released: 0 }, [{ op: "replace", path: "/logical", value: shortened.byteLength }], true).newDocument;
    expect(receipt.physical).toBe(original.byteLength);
    expect(receipt.released).toBe(0);
  }
  const source = readModuleTreeSource(new URL("../../../../../../🧾️document/📜️history/💧️hydration/🦀️.rs", import.meta.url));
  const start = source.indexOf("Phase::RetirePack =>");
  const body = source.slice(start, source.indexOf("Phase::CloseEnvelopeRuntime =>", start));
  expect(body.includes(".truncate(")).toBe(false);
  expect(body.includes("self.pack.take()")).toBe(false);
  expect(body.includes("artifact_retirement_admit_owned")).toBe(true);
  expect(body.includes("grant.maximum_items.min(1)")).toBe(true);
});

test("hydration fold consumes supplied independent physical authority", async () => {
  for (const grant of [{ items: 0, copy: 1, capacity: 0, release: 0, depth: 2 }, { items: 1, copy: 1, capacity: 7, release: 19, depth: 2 }, { items: 1, copy: 0, capacity: 19, release: 7, depth: 3 }]) {
    const child = applyPatch(structuredClone(grant), [{ op: "replace", path: "/items", value: Math.min(1, grant.items) }, { op: "replace", path: "/depth", value: grant.depth - 1 }], true).newDocument;
    expect(child.capacity).toBe(grant.capacity);
    expect(child.release).toBe(grant.release);
    expect(child.copy).toBe(grant.copy);
    expect(child.depth).toBe(grant.depth - 1);
  }
  const source = readModuleTreeSource(new URL("../../../../../../🧾️document/📜️history/💧️hydration/🦀️.rs", import.meta.url));
  const start = source.indexOf("Phase::Fold =>");
  const body = source.slice(start, source.indexOf("Phase::BindGenesis =>", start));
  expect(body.includes("next_step_grant(")).toBe(false);
  expect(body.includes("hydration_child_grant(grant, cx.fuel_remaining())")).toBe(true);
  const childStart = source.indexOf("fn hydration_child_grant(");
  const child = source.slice(childStart, source.indexOf("fn progress_fuel(", childStart));
  expect(child.includes("..grant")).toBe(true);
  expect(child.includes("grant.maximum_depth.checked_sub(1)")).toBe(true);
  expect(child.includes("grant.maximum_copy_bytes.min(bytes)")).toBe(true);
  expect(child.includes("maximum_capacity_bytes:")).toBe(false);
  expect(child.includes("maximum_release_bytes:")).toBe(false);
  expect(body.includes("progress.fits(fold_grant)")).toBe(true);
});

test("ready config auxiliary owner needs capacity and no transferred catalog", async () => {
  const backing = Buffer.allocUnsafeSlow(37);
  expect(backing.subarray(0, 0).byteLength).toBe(0);
  expect(backing.subarray(0, 0).buffer.byteLength).toBe(37);
  const state = applyPatch({ outputReady: true, catalogRetained: false, auxiliaryCapacity: backing.byteLength, terminal: false }, [], true).newDocument;
  expect(state.terminal).toBe(false);
  expect(state.auxiliaryCapacity).toBe(backing.buffer.byteLength);
  const source = readModuleTreeSource(new URL("../../../../../../🎚️config/📥️retained/🦀️.rs", import.meta.url));
  expect(source.includes("&& self.actor.0.is_empty()")).toBe(false);
  const start = source.indexOf("fn retirement_demands(");
  const body = source.slice(start, source.indexOf("impl<P, M> ErasedSnapshotRetirement", start));
  const guards = [...body.matchAll(/let owners = self\.owners\.as_ref\(\)/g)].map(match => match.index);
  expect(guards.length).toBe(2);
  expect(body.slice(guards[0] - 150, guards[0]).includes("if let Some(value) = self.pending_snapshot")).toBe(true);
  const close = source.slice(source.indexOf("fn close_step(&mut self, grant:"), source.indexOf("fn terminal_is_empty(&self)"));
  expect(close.includes("if self.pending_edit.is_some()")).toBe(true);
  expect(close.includes("self.pending_edit.take()")).toBe(true);
  expect(close.includes("controlled!(expected_id)")).toBe(true);
  expect(close.includes("self.actor.0 = value")).toBe(true);
});

test("completed original publication retains factory and physical frame custody", async () => {
  const neutral = await Bun.file(new URL("../../../../../../♻️retirement/📦️backing/🧪️tests/🧫️fixtures/🔣️.json", import.meta.url)).json();
  const law = neutral.completedRecordCustody;
  for (const wordBytes of law.wordBytes) {
  const original = Buffer.allocUnsafeSlow(law.factorySources * law.pointerWordsPerCapability * wordBytes);
  const published = applyPatch({ envelope: true, factories: law.factorySources, factoryBytes: original.byteLength, sourceReleased: 0, closing: false }, [{ op: "replace", path: "/envelope", value: false }, { op: "replace", path: "/closing", value: true }], true).newDocument;
  expect(published.sourceReleased).toBe(law.publicationSourceReleaseBytes);
  expect(published.factories).toBe(law.factorySources);
  expect(published.factoryBytes).toBe(original.byteLength);
  expect(published.closing).toBe(true);
  }
  const root = readStoreSourceText();
  const start = root.indexOf("pub struct ArtifactEnvelopeCompletedRecordOwner<");
  const owner = root.slice(start, root.indexOf("struct ArtifactEnvelopeCompletedRecordSlot", start));
  expect(owner.includes("factory_retirement:")).toBe(true);
  expect(owner).toContain("self.initial_snapshot_factory.is_none()");
  expect(owner).toContain("self.mutation_factory.is_none()");
  expect(owner).toContain("self.factory_retirement.iter().all(Option::is_none)");
  expect(owner).toContain("FactoryAuthority::new(factory)");
  expect(owner).toContain("RetainedCloneStep::Complete(step.progress())");
  const registryStart = root.indexOf("impl<P, Mutation> ArtifactEnvelopeCompletedRecordRegistry<P, Mutation>");
  const publication = root.slice(root.indexOf("    pub fn try_publish_to", registryStart), root.indexOf("    pub fn try_detach", registryStart));
  expect(publication).toContain("state.closing |= 1u64 << index");
  expect(publication.includes("drop(owner)")).toBe(false);
});

test("hydration current authored Rust syntax parses against original source", async () => {
  const physical = await Bun.file(new URL("../../../../../../🧾️document/📜️history/💧️hydration/🧪️tests/🧫️fixtures/📏️physical-demand/🔣️.json", import.meta.url)).json();
  const oracle = new Ajv({ strict: false });
  oracle.addSchema(orderedRetirement);
  oracle.addSchema(retirementContract);
  expect(oracle.compile({ $ref: retirementContract.$id + "#/$defs/Grant" })(physical.readyAuxiliaryPolicy)).toBe(true);
  for (const relative of ["../../../../../../🔌️plugin/🧪️tests/🧩️composition/🦀️.rs", "../../../../../🧾️document/📜️history/💧️hydration/🦀️.rs", "../../../../../🧾️document/📜️history/💧️hydration/🧪️tests/🔬️unit/🦀️.rs", "../../../../../🎚️config/📥️retained/🦀️.rs", "../../../../../../../../../🔨️modules/📡️replication/🔗️causal/🔀️transition/🔁️fold/🦀️.rs", "../../../../../../../../../🔨️modules/📡️replication/🔗️causal/🔀️transition/🧪️tests/🔬️unit/🦀️.rs", "../../../../../..//📡️spr/📜️history/🧪️tests/🔬️unit/🦀️.rs", "../../../../../../../../../🔨️modules/📡️replication/🧾️wire/♻️retirement/🦀️.rs", "../../../../../../../../../🔨️modules/📡️replication/🧾️wire/♻️retirement/🧪️tests/🦀️.rs", "../../../../../🧩️composition/🚪️open/🏭️operation/🦀️.rs", "../../../../../🧩️composition/🚪️open/🧪️tests/🔬️unit/🦀️.rs", "../../../../../🏗️initialization/📚️catalog/🦀️.rs", "../../../../../🏗️initialization/📚️catalog/🧪️tests/🔬️unit/🦀️.rs", "../../../../../../🌿️vcs/🚪️io/💾️binary/🌱️genesis/🦀️.rs", "../../../../../../🌿️vcs/🦀️.rs"]) {
    const path = fileURLToPath(new URL(`../${relative}`, import.meta.url));
    expect([...path].length).toBeLessThanOrEqual(256);
    const result = spawnSync("rustfmt", ["--edition", "2024", "--config", "skip_children=true", "--emit", "stdout", path], { encoding: "utf8", timeout: 2000 });
    expect(result.error).toBeUndefined();
    if (result.status !== 0) {}
    expect(result.status).toBe(0);
  }
});

test("hydration target decoder preserves original supplied physical authority", async () => {
  const source = readModuleTreeSource(new URL("../../../../../../🧾️document/📜️history/💧️hydration/🦀️.rs", import.meta.url));
  const start = source.indexOf("Phase::DecodeTarget =>");
  const body = source.slice(start, source.indexOf("Phase::TargetInputs =>", start));
  expect(body.includes("next_step_grant(")).toBe(false);
  expect(body.includes("hydration_child_grant(grant, cx.fuel_remaining())")).toBe(true);
  expect(body.includes("progress.fits(decode_grant)")).toBe(true);
  expect(body.includes("progress_fuel(progress)")).toBe(true);
});

test("history fold terminal output retains actual physical cleanup receipt", async () => {
  const oracle = new Ajv({ strict: false });
  const valid = oracle.compile({ ...orderedRetirement, $ref: "#/$defs/Progress" });
  for (const outcome of ["Ready", "Rejected"]) {
    const backing = Buffer.allocUnsafeSlow(73);
    const result = applyPatch({ outcome, progress: { copiedItems: 1, copiedBytes: 0, retainedCapacityBytes: 0, releasedBytes: 0, complete: false } }, [{ op: "replace", path: "/progress/releasedBytes", value: backing.buffer.byteLength }, { op: "replace", path: "/progress/complete", value: outcome === "Ready" }], true).newDocument;
    expect(valid(result.progress)).toBe(true);
    expect(result.progress.releasedBytes).toBe(73);
    expect(result.outcome).toBe(outcome);
  }
  const source = await Bun.file(new URL("../../../../../../../../../../🔨️modules/📡️replication/🔗️causal/🔀️transition/🔁️fold/🦀️.rs", import.meta.url)).text();
  const start = source.indexOf("pub enum HistoryFoldJobStep");
  const declaration = source.slice(start, source.indexOf("pub struct HistoryFoldJob", start));
  expect(declaration.includes("Ready { value: T, progress: RetainedCloneProgress }")).toBe(true);
  expect(declaration.includes("Rejected { progress: RetainedCloneProgress }")).toBe(true);
  expect(source.includes("HistoryFoldJobStep::Ready { value, progress }")).toBe(true);
  expect(source.includes("HistoryFoldJobStep::Rejected { progress }")).toBe(true);
  expect(source.includes("pub fn rejection(&self) -> Option<&crate::ProtocolError>")).toBe(true);
  expect(source.includes("pub fn take_rejection(&mut self) -> Option<crate::ProtocolError>")).toBe(true);
  const closed = source.slice(source.indexOf("fn cleanup_one("), source.indexOf("pub fn finish_cold("));
  expect(source).not.toContain("fn error_demand(");
  expect(source).toContain("crate::protocol_cause_retirement_demand(error)");
  expect(closed).toContain("crate::close_protocol_cause_one(error, grant)");
  expect(closed).toContain("drop(self.result.take())");
  for (const currency of ["copy_bytes", "capacity_bytes", "release_bytes", "depth"]) expect(source).toContain("demand." + currency);
  const original = Buffer.allocUnsafeSlow(37);
  const retained = applyPatch({ output: "Rejected", originalCapacity: original.byteLength, released: 0 }, [], true).newDocument;
  expect(retained.originalCapacity).toBe(original.buffer.byteLength);
  expect(retained.released).toBe(0);
});

test("original document Store disposer forwards full grants and terminal receipt", async () => {
  const neutral = await Bun.file(new URL("../../../../../../♻️retirement/📦️backing/🧪️tests/🧫️fixtures/🔣️.json", import.meta.url)).json();
  const law = neutral.storeDisposer;
  const oracle = new Ajv({ strict: false });
  oracle.addSchema(orderedRetirement);
  oracle.addSchema(retirementContract);
  expect(oracle.compile({ $ref: retirementContract.$id + "#/$defs/Grant" })({ maximumItems: 1, maximumCopyBytes: law.copyBytes, maximumCapacityBytes: law.capacityBytes, maximumReleaseBytes: law.releaseBytes, maximumDepth: law.depth })).toBe(true);
  const backing = Buffer.allocUnsafeSlow(law.releaseBytes);
  const result = applyPatch({ retained: backing.byteLength, released: 0 }, [{ op: "replace", path: "/retained", value: 0 }, { op: "replace", path: "/released", value: backing.byteLength }], true).newDocument;
  expect(result.released).toBe(law.terminalReceipt.releasedBytes);
  expect(law.releaseBytes).toBeGreaterThan(law.copyBytes);
  const root = await Bun.file(new URL("../../../../../../../🔌️plugin/🦀️.rs", import.meta.url)).text();
  const trait = root.slice(root.indexOf("pub trait ArtifactOwnedDisposer<T>"), root.indexOf("//#region 🎛️BoundedConfigStoreOwners"));
  expect(trait).toContain("retirement_demands");
  expect(trait).toContain("grant: RetainedCloneGrant");
  expect(trait).toContain("PluginLifecycleStep");
  const start = root.indexOf("impl<P, Mutation> ArtifactOwnedDisposer<ArtifactStore<P, Mutation>>");
  const body = root.slice(start, root.indexOf("pub trait ArtifactStoreInitializationAuthority", start));
  expect(body).toContain("owner.close_owned_demands(body)");
  expect(body).toContain("owner.close_owned_store_step(grant)");
  expect(body).toContain("admit_retained_clone_close");
  expect(body).toContain("PluginLifecycleStep::retained(step");
  expect(body).not.toContain("SnapshotRetirementStep");
});

test("hydration fold outputs retain original custody before receipt refusal", async () => {
  for (const accepted of [false, true]) {
    const original = Buffer.allocUnsafeSlow(37);
    const state = applyPatch({ original: original.byteLength, retained: 0, accepted }, [{ op: "replace", path: "/retained", value: original.buffer.byteLength }], true).newDocument;
    expect(state.retained).toBe(37);
    expect(state.original).toBe(37);
  }
  const h = readModuleTreeSource(new URL("../../../../../../🧾️document/📜️history/💧️hydration/🦀️.rs", import.meta.url));
  const ready = h.slice(h.indexOf("HistoryFoldJobStep::Ready { value: (fold"), h.indexOf("Phase::BindGenesis =>", h.indexOf("Phase::Fold =>")));
  expect(ready.indexOf("*self.fold = Some(fold)")).toBeLessThan(ready.indexOf("if !progress.fits(fold_grant)"));
  expect(h).toContain("pending_target: ManuallyDrop<Option<crate::os_spr::HistoryTransition>>");
  const target = h.slice(h.indexOf("Phase::DecodeTarget =>"), h.indexOf("Phase::SeedApplied |"));
  expect(target.indexOf("*self.pending_target = Some(target)")).toBeLessThan(target.indexOf("if !progress.fits(decode_grant)"));
  expect(target).toContain("artifact_retirement_admit_owned(&mut self.pending_target");
  expect(target).not.toContain("let target = self.pending_target.take()");
  const c = readModuleTreeSource(new URL("../../../../../../🎚️config/📥️retained/🦀️.rs", import.meta.url));
  const config = c.slice(c.indexOf("HistoryFoldJobStep::Ready { value: (fold"), c.indexOf("Phase::BindGenesis =>", c.indexOf("Phase::Fold =>")));
  expect(config.indexOf("*self.fold_auxiliary = Some((replay_order, conflicts))")).toBeLessThan(config.indexOf("if !progress.fits(grant)"));
  expect(c).toContain("controlled!(fold_auxiliary)");
  expect(c).toContain("&& self.fold_auxiliary.is_none()");
});

test("member input buffer birth preserves explicit capacity authority before allocation", async () => {
  const oracle = new Ajv({ strict: false }); oracle.addSchema(orderedRetirement); oracle.addSchema(retirementContract);
  const valid = oracle.compile({ $ref: retirementContract.$id + "#/$defs/Grant" });
  const plain = await Bun.file(new URL("../../../../🏭️operation/📏️birth/🧪️tests/🧫️fixtures/🔣️.json", import.meta.url)).json();
  const capacity = plain.inputBuffer.capacityBytes;
  expect(valid(plain.inputBuffer.policy)).toBe(true);
  for (const grant of [
    { maximumItems: 0, maximumCopyBytes: 0, maximumCapacityBytes: capacity, maximumReleaseBytes: 0, maximumDepth: 1 },
    { maximumItems: 1, maximumCopyBytes: 0, maximumCapacityBytes: capacity - 1, maximumReleaseBytes: 0, maximumDepth: 1 },
    { maximumItems: 1, maximumCopyBytes: 0, maximumCapacityBytes: capacity, maximumReleaseBytes: 0, maximumDepth: 0 },
    { maximumItems: 1, maximumCopyBytes: 0, maximumCapacityBytes: capacity, maximumReleaseBytes: 0, maximumDepth: 1 }
  ]) {
    expect(valid(grant)).toBe(true);
    const admitted = grant.maximumItems >= 1 && grant.maximumCapacityBytes >= capacity && grant.maximumDepth >= 1;
    const state = applyPatch({ retainedCapacity: 0 }, admitted ? [{ op: "replace", path: "/retainedCapacity", value: Buffer.allocUnsafeSlow(capacity).buffer.byteLength }] : [], true).newDocument;
    expect(state.retainedCapacity).toBe(admitted ? capacity : 0);
  }
  const source = await Bun.file(new URL("../../../../🏭️operation/🦀️.rs", import.meta.url)).text();
  expect(source).toContain("fn admit_member_input_buffer(");
  const birth = source.slice(source.indexOf("fn admit_member_input_buffer("), source.indexOf("pub enum MemberSnapshotOpenStep"));
  expect(birth.indexOf("grant.maximum_capacity_bytes < capacity")).toBeLessThan(birth.indexOf("try_reserve_exact(capacity)"));
  for (const phase of ["Phase::CaptureGenesis =>", "Phase::CopyHistory =>"]) {
    const start = source.indexOf(phase);
    const body = source.slice(start, source.indexOf("            Phase::", start + phase.length));
    expect(body).toContain("admit_member_input_buffer");
    expect(body).not.toContain("Vec::with_capacity(total)");
    expect(body).toContain("grant.maximum_copy_bytes / 2");
  }
});

test("original evaluation resource scopes preserve complete physical grants", async () => {
  const neutral = await Bun.file(new URL("../../../../../../♻️retirement/📦️backing/🧪️tests/🧫️fixtures/🔣️.json", import.meta.url)).json();
  const law = neutral.evaluationResources;
  const backing = Buffer.allocUnsafeSlow(law.releaseBytes);
  const state = applyPatch({ source: backing.byteLength, admitted: law.capacityBytes, physical: 0 }, [{ op: "replace", path: "/physical", value: backing.byteLength + law.capacityBytes }], true).newDocument;
  expect(state.physical).toBe(state.source + state.admitted);
  expect(law.copyBytes).toBeLessThan(law.releaseBytes);
  const sdkRoot = new URL("../../../../../../../🌊️flow/🧩️extensions/🕸️wasm/🦀️.rs", import.meta.url);
  const sdk = readModuleTreeSource(sdkRoot);
  const input = sliceModuleTreeSource(sdkRoot, "pub struct EvaluationInputPreparation", "const EVALUATION_JOB_CAPACITY");
  expect(input).toContain("retirement_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>");
  expect(input).toContain("evaluation_admit_original(&mut self.parser,&mut self.active,grant)");
  expect(input).toContain("evaluation_child_close(&mut self.active,grant)");
  expect(input).toContain("self.encoding.is_none()&&self.decoding.is_none()&&self.active.is_none()");
  expect(input).toContain("RetirementStep::Progress(progress)");
  const owner = sliceModuleTreeSource(sdkRoot, "pub struct ExtensionEvaluationResources", "// #endregion ⏱️EvaluationJobs");
  expect(owner).toContain("fn retirement_demands");
  expect(owner).toContain("grant:RetainedCloneGrant");
  expect(owner).toContain("RegistryLeaseRetirement");
  expect(owner).not.toContain("next_close_byte_demand");
  expect(owner).not.toContain("PluginCloseStep");
  const cancelled = sliceModuleTreeSource(sdkRoot, "pub fn retire_cancelled_evaluations_close_step", "pub fn evaluation_retirement_pending");
  expect(cancelled).not.toContain("key.clone()");
  expect(cancelled).toContain("evaluation_registry_demands(&registry,identity,grant.maximum_copy_bytes)");
  const demands = sliceModuleTreeSource(sdkRoot, "fn evaluation_controlled_demands", "pub fn retire_cancelled_evaluations_close_step");
  for (const name of ["next_copy_byte_demand", "next_capacity_byte_demand", "next_release_byte_demand", "next_depth_demand", "retained.retirement_demands(copy,false,false)"]) expect(demands).toContain(name);
  expect(cancelled).toContain("extract_slot_if");
  expect(cancelled).toContain("registry.retiring_key=Some");
  expect(cancelled).toContain("registry.retiring_backing=Some");
  expect(cancelled).toContain("grant:RetainedCloneGrant");
  expect(sdk).not.toContain("RetirementStep::ProcessedBytes");
});

test("hydration target binding retains original empty physical address backing", async () => {
  const original = Buffer.allocUnsafeSlow(37); const empty = original.subarray(0, 0);
  const replacement = Buffer.allocUnsafeSlow(19);
  const bound = applyPatch({ retainedOriginal: empty.buffer.byteLength, current: 0, released: 0 }, [{ op: "replace", path: "/current", value: replacement.buffer.byteLength }], true).newDocument;
  expect(bound.retainedOriginal).toBe(37); expect(bound.current).toBe(19); expect(bound.released).toBe(0);
  const h = readModuleTreeSource(new URL("../../../../../../🧾️document/📜️history/💧️hydration/🦀️.rs", import.meta.url));
  const begin = h.slice(h.indexOf("Phase::BeginTargets =>"), h.indexOf("Phase::DecodeTarget =>"));
  expect(begin).toContain("artifact_retirement_admit_owned(&mut self.target_address");
  const target = h.slice(h.indexOf("Phase::TargetInputs =>"), h.indexOf("Phase::SeedApplied |"));
  expect(target).toContain("std::mem::replace(&mut source[self.record_index].target");
  expect(target).toContain("*self.target_address = Some(original)");
});
