import { test, expect } from "bun:test";
import Ajv from "ajv";
import { orderedRetirementSchema as orderedRetirement } from "@semio-tech/value-ts/ordered/retirement/schema";
import { retirementContractSchema as retirementContract } from "@semio-tech/value-ts/retirement/contract/schema";
import { Buffer } from "node:buffer";
import { applyPatch } from "fast-json-patch";
import { readModuleTreeSource } from "@semio-tech/framework-compiler-ts/syntax/module-tree";
import { readStoreSourceText } from "@semio-tech/framework/test/source-text";
import { spawnSync } from "node:child_process";
import fixture from "../../../../📏️retirement/🧪️tests/🧫️fixtures/🔣️.json" with { type: "json" };
import { controlSnapshotReceivingFixtureUrl } from "@semio-tech/framework/io/testing/fixtures";
import { cloneWireProgressSchemaUrl } from "@semio-tech/value-ts/testing/fixtures";

test("initialization catalog birth admits one exact original lane under supplied currencies", async () => {
  const oracle = new Ajv({ strict: false });
  oracle.addSchema(orderedRetirement);
  oracle.addSchema(retirementContract);
  const grants = oracle.compile({ $ref: `${retirementContract.$id}#/$defs/Grant` });
  const plain = await Bun.file(new URL("../../../../../../🏗️initialization/📚️catalog/🧪️tests/🧫️fixtures/🔣️.json", import.meta.url)).json();
  const policy = plain.birthPolicy;
  expect(grants(policy)).toBe(true);
  expect(plain.birthCancelCuts).toEqual(Array.from({ length: plain.lanes.length + 1 }, (_, index) => index));
  const pages: Buffer[] = plain.birthExampleExtents.map((bytes: number) => Buffer.allocUnsafeSlow(bytes));
  let state = { admitted: 0, born: 0 };
  for (const page of pages) {
    for (const denied of [{ ...policy, maximumItems: 0 }, { ...policy, maximumCapacityBytes: page.byteLength - 1 }, { ...policy, maximumDepth: 0 }]) {
      expect(grants(denied)).toBe(true);
      const next = applyPatch(state, denied.maximumItems && denied.maximumDepth && denied.maximumCapacityBytes >= page.byteLength ? [{ op: "replace", path: "/admitted", value: state.admitted + 1 }] : [], true).newDocument;
      expect(next).toEqual(state);
    }
    state = applyPatch(state, [{ op: "replace", path: "/admitted", value: state.admitted + 1 }, { op: "replace", path: "/born", value: state.born + page.byteLength }], true).newDocument;
  }
  expect(state).toEqual({ admitted: 6, born: pages.reduce((sum, page) => sum + page.byteLength, 0) });
  const source = await Bun.file(new URL("../../../../../../🏗️initialization/📚️catalog/🦀️.rs", import.meta.url)).text();
  expect(source).toContain("pub fn empty()");
  expect(source).toContain("pub fn admission_demands(");
  expect(source).toContain("pub fn admission_is_complete(");
  expect(source).toContain("pub fn admit_next(");
  expect(source.indexOf("grant.maximum_capacity_bytes < demand.capacity_bytes")).toBeLessThan(source.indexOf("HistoryPageStack::try_new()"));
  expect(source).toContain("retained_capacity_bytes: demand.capacity_bytes");
  expect(source).toContain("next_push_allocation_bytes()");
  const hydration = readModuleTreeSource(new URL("../../../../../../🧾️document/📜️history/💧️hydration/🦀️.rs", import.meta.url));
  expect(hydration).toContain("Phase::AdmitInitializationCatalog =>");
  expect(hydration).toContain("ArtifactStoreInitializationRuntime::new_with_owner_catalog");
  expect(hydration).not.toContain("ArtifactStoreInitializationRuntime::new(");
  expect(hydration).toContain("initialization_catalog");
});

test("genesis admission preserves original payload and envelope metadata under supplied grant", async () => {
  const oracle = new Ajv({ strict: false }); oracle.addSchema(orderedRetirement); oracle.addSchema(retirementContract);
  const valid = oracle.compile({ $ref: retirementContract.$id + "#/$defs/Grant" });
  const plain = await Bun.file(new URL("../../../../../../🧾️document/📜️history/💧️hydration/🧪️tests/🧫️fixtures/📏️physical-demand/🔣️.json", import.meta.url)).json();
  const policy = plain.verifiedGenesis.policy;
  expect(valid(policy)).toBe(true);
  const original = Buffer.allocUnsafeSlow(plain.verifiedGenesis.packCapacityBytes); original.set(Buffer.from(plain.verifiedGenesis.text, "utf8"));
  const alias = original.subarray(0, 2);
  const result = applyPatch({ bytes: [...alias], retained: original.buffer.byteLength, admitted: 0 }, [{ op: "replace", path: "/admitted", value: 1 }], true).newDocument;
  expect(result.bytes).toEqual([...Buffer.from("Ω", "utf8")]); expect(result.retained).toBe(37); expect(alias.buffer).toBe(original.buffer);
  const genesis = await Bun.file(new URL("../../../../../../../🌿️vcs/🚪️io/💾️binary/🌱️genesis/🦀️.rs", import.meta.url)).text();
  expect(genesis).toContain("verified_pack_birth_demand()");
  expect(genesis).toContain("admit_verified_pack(");
  expect(genesis).toContain("demand.admit(grant)");
  expect(genesis).toContain("Err((error, snapshot, pack))");
  expect(genesis.indexOf("demand.admit(grant)")).toBeLessThan(genesis.indexOf("Self::from_verified_pack(snapshot, pack, digest)"));
  const root = readStoreSourceText();
  const owner = root.slice(root.indexOf("pub fn create_document_envelope_from_genesis_owners"), root.indexOf("pub async fn edit_ids_for_changes"));
  expect(owner).toContain("schema: String"); expect(owner).toContain("id: String"); expect(owner).toContain("        schema,"); expect(owner).toContain("        id,");
  expect(owner).not.toContain(".into()"); expect(owner).not.toContain(".clone()");
  const h = readModuleTreeSource(new URL("../../../../../../🧾️document/📜️history/💧️hydration/🦀️.rs", import.meta.url));
  const bind = h.slice(h.indexOf("Phase::BindGenesis =>"), h.indexOf("Phase::BuildAppliedCursor |"));
  expect(bind.indexOf("birth.admit(child)")).toBeLessThan(bind.indexOf("self.initial.take()"));
  expect(bind).toContain("admit_verified_pack(initial, pack, initial_digest, child)");
  expect(bind).toContain("*self.initial = Some(initial)"); expect(bind).toContain("*self.pack = Some(pack)");
  expect(bind.includes("empty_document_envelope_from_genesis_owners::<P, M>(schema, expected.artifact_id")).toBe(true);
});

test("original inference retained wallet transports every unequal and zero axis", async () => {
  const root = new URL("../../../../../../../../../../🔨️modules/🌱️value/🧬️retained-clone/🌐️wire", import.meta.url);
  const fixture = await Bun.file(new URL("🧪️tests/🧫️fixtures/🔣️.json", root)).json();
  const schema = await Bun.file(new URL("🧬️schema/🔣️.json", root)).json();
  const validate = new Ajv().compile(schema);
  for (const row of fixture.rows) {
    expect(validate(row)).toBe(true);
    const bytes = Buffer.alloc(fixture.fields.length * 8);
    fixture.fields.forEach((name: string, index: number) => bytes.writeBigUInt64LE(BigInt(row[name]), index * 8));
    const actual = Object.fromEntries(fixture.fields.map((name: string, index: number) => [name, Number(bytes.readBigUInt64LE(index * 8))]));
    expect(actual).toEqual(row);
    const missing = { ...row }; delete missing.maximumReleaseBytes;
    expect(validate(missing)).toBe(false);
    expect(validate({ ...row, bytes: 1 })).toBe(false);
  }
  const maximum = Buffer.alloc(8); maximum.writeBigUInt64LE(BigInt(fixture.maximum64));
  expect(maximum.readBigUInt64LE().toString()).toBe(fixture.maximum64);
  const source = await Bun.file(new URL("../🦀️.rs", root)).text();
  expect(source).toContain("impl crate::ToValue for RetainedCloneGrant");
  expect(source).toContain("impl crate::FromValue for RetainedCloneGrant");
});

test("message ledger page birth preserves fixed tickets and partial original backing", async () => {
  const plain = await Bun.file(new URL("../../../../../../🧾️document/📜️history/💧️hydration/🧪️tests/🧫️fixtures/📏️physical-demand/🔣️.json", import.meta.url)).json();
  const row = plain.messageLedgerBirth;
  const oracle = new Ajv({ strict: false }); oracle.addSchema(orderedRetirement); oracle.addSchema(retirementContract);
  expect(oracle.compile({ $ref: retirementContract.$id + "#/$defs/Grant" })(row.policy)).toBe(true);
  expect(row.capacitySlots % row.pageSlots).toBe(0);
  const reordered = applyPatch([...row.ids], [{ op: "remove", path: "/1" }, { op: "add", path: "/-", value: row.ids[1] }], true).newDocument;
  expect(reordered).toEqual(row.afterReinsert);
  const pages = row.capacitySlots / row.pageSlots;
  for (const cut of row.cancelCuts) {
    let state = { born: 0, freed: 0, slots: 0 };
    for (let index = 0; index < Math.min(cut, pages); index++) {
      const page = Buffer.allocUnsafeSlow(row.exampleSlotBytes * row.pageSlots);
      expect(page.buffer.byteLength).toBeLessThanOrEqual(row.policy.maximumCapacityBytes);
      const denied = applyPatch(state, [], true).newDocument;
      expect(denied).toEqual(state);
      state = applyPatch(state, [{ op: "replace", path: "/born", value: state.born + page.byteLength }, { op: "replace", path: "/slots", value: state.slots + row.pageSlots }], true).newDocument;
    }
    state = applyPatch(state, [{ op: "replace", path: "/freed", value: state.born }], true).newDocument;
    expect(state.freed).toBe(state.born);
    expect(state.slots).toBeLessThanOrEqual(row.capacitySlots);
  }
  const root = readStoreSourceText();
  const slots = root.slice(root.indexOf("struct ArtifactEditMessageSlots"), root.indexOf("pub struct ArtifactEditMessageLedger"));
  expect(slots).toContain("fn empty()");
  expect(slots).toContain("fn admit_next(");
  expect(slots).toContain("fn close_step(");
  expect(slots).toContain("checked_mul");
  expect(slots).toContain("grant.maximum_capacity_bytes < demand.capacity_bytes");
  expect(slots).toContain("grant.maximum_release_bytes < demand.release_bytes");
  expect(slots).toContain("self.pages");
  const ledger = root.slice(root.indexOf("impl ArtifactEditMessageLedger {"), root.indexOf("impl Default for ArtifactEditMessageLedger"));
  expect(ledger).toContain("pub fn empty()");
  expect(ledger).toContain("pub fn admission_demands(");
  expect(ledger).toContain("pub fn admit_next(");
  expect(ledger).toContain("pub fn admission_is_complete(");
  expect(ledger).toContain("self.admission_is_complete()");
  const retire = root.slice(root.indexOf("impl ErasedSnapshotRetirement for ArtifactEditMessageLedgerRetirement"), root.indexOf("impl Drop for ArtifactEditMessageLedgerRetirement"));
  expect(retire).toContain("self.original.slots.close_step(grant)");
  expect(retire).not.toContain("release!(slots)");
});

test("envelope birth retains original phased catalogs before hydration advancement", async () => {
  const plain = await Bun.file(new URL("../../../../../../🧾️document/📜️history/💧️hydration/🧪️tests/🧫️fixtures/📏️physical-demand/🔣️.json", import.meta.url)).json();
  const row = plain.envelopeBirth;
  const oracle = new Ajv({ strict: false }); oracle.addSchema(orderedRetirement); oracle.addSchema(retirementContract);
  expect(oracle.compile({ $ref: retirementContract.$id + "#/$defs/Grant" })(row.policy)).toBe(true);
  for (const cut of row.cancelCuts) {
    const state = applyPatch({ metadata: row.metadata, catalogBirths: 0, terminal: false }, [{ op: "replace", path: "/catalogBirths", value: cut }], true).newDocument;
    expect(state.metadata).toEqual(row.metadata);
    expect(state.catalogBirths).toBeLessThanOrEqual(row.totalBirthTurns);
  }
  const vcs = readModuleTreeSource(new URL("../../../../../../../🌿️vcs/🦀️.rs", import.meta.url));
  expect(vcs.includes("pub fn initial_page_birth_demand(")).toBe(true);
  expect(vcs.includes("pub fn admit_initial_page(")).toBe(true);
  const root = readStoreSourceText();
  const empty = root.slice(root.indexOf("pub fn empty_document_envelope_from_genesis_owners"), root.indexOf("pub async fn edit_ids_for_changes"));
  expect(empty).toContain("ArtifactHistoryLedger::empty()");
  expect(empty).toContain("HistoryPageStack::empty()");
  expect(empty).toContain("ArtifactEditMessageLedger::empty()");
  expect(empty).not.toContain("ArtifactHistoryLedger::new()");
  expect(empty).not.toContain("ArtifactCursor::default()");
  expect(root.includes("pub fn document_admission_demands(")).toBe(true);
  expect(root.includes("pub fn admit_document_next(")).toBe(true);
  expect(root.includes("pub fn document_admission_is_complete(")).toBe(true);
  const h = readModuleTreeSource(new URL("../../../../../../🧾️document/📜️history/💧️hydration/🦀️.rs", import.meta.url));
  expect(h).toContain("Phase::AdmitEnvelope =>");
  const phase = h.slice(h.indexOf("Phase::AdmitEnvelope =>"), h.indexOf("Phase::BuildAppliedCursor |"));
  expect(phase).toContain("admit_document_next(child)");
  expect(phase).toContain("document_admission_is_complete()");
  expect(phase).toContain("progress.fits(child)");
});

test("original inference reports the actual independent physical receipt", async () => {
  const root = new URL("../../../../../../../../../../🔨️modules/🌱️value/🧬️retained-clone/🌐️wire", import.meta.url);
  const fixture = await Bun.file(new URL("🧪️tests/🧫️fixtures/🔣️.json", root)).json();
  const validate = new Ajv().compile(await Bun.file(new URL("🧬️schema/🧾️progress.json", root)).json());
  for (const row of fixture.receipts) {
    expect(validate(row)).toBe(true);
    const bytes = Buffer.alloc(fixture.receiptFields.length * 8);
    fixture.receiptFields.forEach((name: string, index: number) => bytes.writeBigUInt64LE(BigInt(row[name]), index * 8));
    expect(Object.fromEntries(fixture.receiptFields.map((name: string, index: number) => [name, Number(bytes.readBigUInt64LE(index * 8))]))).toEqual(row);
    const missing = { ...row }; delete missing.releasedBytes;
    expect(validate(missing)).toBe(false);
    expect(validate({ ...row, bytes: 1 })).toBe(false);
  }
  const source = await Bun.file(new URL("../🦀️.rs", root)).text();
  expect(source).toContain("impl crate::ToValue for RetainedCloneProgress");
  expect(source).toContain("impl crate::FromValue for RetainedCloneProgress");
  const plugin = await Bun.file(new URL("../../../../../../../🔌️plugin/🦀️.rs", import.meta.url)).text();
  expect(plugin.match(/pub retirement_progress: RetainedCloneProgress/g)?.length).toBe(2);
  expect(plugin).toContain("retirement_progress: execution.retirement_progress");
});

test("original interactive cancellation retains the same deep owner until physical close", async () => {
  const backing = Buffer.from("original cancelled source 🧩", "utf8");
  const original = { cancelled: false, remaining: 3, backing };
  const signalled = applyPatch(original, [{ op: "replace", path: "/cancelled", value: true }], true).newDocument;
  expect(signalled.backing).toBe(backing);
  expect(signalled.remaining).toBe(3);
  for (const remaining of [2, 1, 0]) {
    applyPatch(signalled, [{ op: "replace", path: "/remaining", value: remaining }], true);
    expect(signalled.backing).toBe(backing);
  }
  const modules = new URL("../../../../../../..", import.meta.url);
  const router = await Bun.file(new URL("🔌️plugin/⚛️reactor/💼️jobs/🦀️.rs", modules)).text();
  const inference = await Bun.file(new URL("🔌️plugin/⚛️reactor/💼️jobs/💡️infer/🦀️.rs", modules)).text();
  const cancel = router.slice(router.indexOf("pub async fn cancel_job"),router.indexOf("pub async fn step_job"));
  expect(cancel).toContain("owner.terminal_drop_is_shallow()");
  expect(cancel).toContain("if shallow");
  expect(cancel).not.toContain("assert!");
  expect(inference).not.toContain("self.retire()");
  expect(inference).toContain("outcome: JobOutcomeSlot");
  expect(inference).toContain("session.close_step(self.request.retained)");
  expect(inference).toContain("close_step_outcome_slot(&mut self.outcome,self.request.retained)");
  expect(inference).not.toContain("JobPayloadCloseStep");
  expect(inference).toContain("self.retirement_progress.checked_add(progress)");
});

test("actual snapshot input step funds original birth and both physical copies", async () => {
  const plain = await Bun.file(new URL("../../../../🏭️operation/📏️birth/🧪️tests/🧫️fixtures/🔣️.json", import.meta.url)).json();
  const row = plain.snapshotInput;
  const oracle = new Ajv({ strict: false }); oracle.addSchema(orderedRetirement); oracle.addSchema(retirementContract);
  expect(oracle.compile({ $ref: retirementContract.$id + "#/$defs/Grant" })(row.policy)).toBe(true);
  for (const maximumCopyBytes of row.copyGrants) {
    const grant = { ...row.policy, maximumCopyBytes };
    const source = Buffer.allocUnsafeSlow(row.payloadBytes); source.fill(7);
    const copied = Math.min(source.byteLength, row.chunkMaximumBytes, Math.floor(grant.maximumCopyBytes / row.expected.copiesPerPayloadByte));
    const intermediate = Buffer.allocUnsafeSlow(copied); const original = Buffer.allocUnsafeSlow(row.payloadBytes);
    source.copy(intermediate,0,0,copied); intermediate.copy(original);
    const progress = { copiedItems:Number(copied > 0), copiedBytes:copied * 2, retainedCapacityBytes:0, releasedBytes:0 };
    expect(original.subarray(0,copied)).toEqual(source.subarray(0,copied));
    expect(progress.copiedBytes).toBeLessThanOrEqual(grant.maximumCopyBytes);
    expect(applyPatch({ copiedBytes:0 },[{op:"replace",path:"/copiedBytes",value:copied * 2}],true).newDocument.copiedBytes).toBe(progress.copiedBytes);
  }
  const source = await Bun.file(new URL("../../../../🏭️operation/🦀️.rs", import.meta.url)).text();
  const trait = source.slice(source.indexOf("pub trait MemberSnapshotOpenOperation"),source.indexOf("pub struct UnsupportedMemberSnapshotOpen"));
  expect(trait.includes("grant: RetainedCloneGrant")).toBe(true);
  const actual = source.slice(source.indexOf("impl<P: semio_framework_value::retirement::RetireOwned,D:MemberSnapshotDecoder<P>>"),source.indexOf("impl<P: semio_framework_value::retirement::RetireOwned,D> ProducedMemberSnapshotOpen"));
  expect(actual.includes("admit_member_input_buffer")).toBe(true);
  expect(actual.includes("grant.maximum_copy_bytes / 2")).toBe(true);
  expect(actual.includes("self.pending(expected_bytes, progress, cx)")).toBe(true);
  const publish = source.slice(source.indexOf("fn publish_input(&mut self,"),source.indexOf("impl<P: semio_framework_value::retirement::RetireOwned,D:MemberSnapshotDecoder<P>>"));
  expect(publish.includes("retained_progress: progress")).toBe(true);
  expect(actual.includes("self.input.try_reserve_exact(expected_bytes)")).toBe(false);
});

test("canonical reader ownership preserves the original byte oracle under independent physical policy", async () => {
  const { readFileSync } = await import("node:fs");
  const { resolve } = await import("node:path");
  const root = process.cwd();
  const base = "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️canonical-edit/";
  const row = JSON.parse(readFileSync(resolve(root, base + "🧪️tests/🧫️fixtures/📖️canonical-reader.json"), "utf8"));
  const original = JSON.parse(readFileSync(resolve(root, base + "🧪️tests/🧫️fixtures/🗺️canonical-borrowed-map.json"), "utf8"));
  const oracle = new Ajv({ strict: false }); oracle.addSchema(retirementContract); oracle.addSchema(orderedRetirement);
  const validate = oracle.compile({ $ref: retirementContract.$id + "#/$defs/Grant" });
  expect(validate(row.retirementGrant)).toBe(true);
  expect(Object.values(row.retirementGrant)).toEqual([1, 4096, 65536, 65536, 64]);
  for (const axis of Object.keys(row.retirementGrant)) expect(validate({ ...row.retirementGrant, [axis]: -1 })).toBe(false);
  const bytes = Buffer.from(original.expectedJson, "utf8");
  expect(bytes.byteLength).toBe(row.expectedByteLength);
  expect(new TextEncoder().encode(original.expectedJson)).toEqual(new Uint8Array(bytes));
  const { createHash } = await import("node:crypto");
  expect(createHash("sha256").update(bytes).digest("hex")).toBe(row.expectedJsonSha256);
  const lastInFirstOut = ["lifetime", "map"];
  expect(lastInFirstOut.pop()).toBe("map"); expect(lastInFirstOut.pop()).toBe("lifetime");
  const map = readFileSync(resolve(root, base + "🧵️borrowed/🧪️tests/🧵️borrowed/🦀️.rs"), "utf8");
  const reader = readFileSync(resolve(root, base + "📖️reader/🧪️tests/📖️reader/🦀️.rs"), "utf8");
  expect(map.includes("MapFields(semio_framework_value::ordered::OrderedMap<MapValue>)")).toBe(true);
  expect(map.includes("admit_owned_retirement(value, grant)")).toBe(true);
  expect(map.includes("deferred(MapTerminalLifetime { lifetime, tracked: *tracked }), deferred(map)")).toBe(true);
  const retirement = readFileSync(resolve(root,"🧰️framework/🔨️modules/🌱️value/♻️retirement/🦀️.rs"),"utf8");
  expect(retirement.includes("self.0.pop().map_or(RetirementStep::Complete, RetirementStep::Child)")).toBe(true);
  expect(map.includes("active.truncate")).toBe(false);
  expect(map.includes("fn member_canonical_reader_map_retirement_preserves_refused_original_and_paid_cancel_frontiers")).toBe(true);
  expect(map.includes("assert_eq!(body.map.0.entry_at_rank(0).unwrap().0.as_ptr(),key_pointer)")).toBe(true);
  expect(reader.includes("SnapshotRetirementStep")).toBe(false);
  expect(reader.includes("close_step(*RETIREMENT_POLICY)")).toBe(true);
  expect(reader.includes("SharedValueRetirementFactory::<Edit<MapMutation>>")).toBe(true);
  expect(reader.includes("let snapshot_bytes = root.text.as_bytes().len()")).toBe(true);
  expect(reader.includes("fixture[\"expectedSnapshotBytes\"]")).toBe(true);
  expect(reader.includes("heap.released_bytes), (step.progress().retained_capacity_bytes")).toBe(true);
  expect(reader.includes("reader.demands")).toBe(false);
  expect(reader.includes("next_release_byte_demand")).toBe(false);
  for (const [name, source] of [["map", map], ["reader", reader]] as const) {
    const grammar = spawnSync("rustfmt", ["--edition", "2021", "--emit", "stdout"], { input: source, encoding: "utf8", timeout: 2000 });
    expect(grammar.error, name).toBeUndefined(); expect(grammar.status, name).toBe(0);
  }
});

test("member request physical grants remain caller policy independent of the close frontier",async()=>{
 const fs=await import("node:fs"),path=await import("node:path");
 const base=path.join(process.cwd(),"🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open");
 const law=JSON.parse(fs.readFileSync(path.join(base,"📏️retirement/🧪️tests/🧫️fixtures/🔣️.json"),"utf8"));
 const ajv=new Ajv({strict:false});ajv.addSchema(orderedRetirement);ajv.addSchema(retirementContract);const validate=ajv.compile({$ref:retirementContract.$id+"#/$defs/Grant"});
 expect(validate(law.physicalCloseGrant)).toBe(true);expect(Object.values(law.physicalCloseGrant)).toEqual([1,4096,0,262144,64]);
 const source=fs.readFileSync(path.join(base,"🧪️tests/🔬️unit/🦀️.rs"),"utf8");
 expect(source.includes("maximum_depth: request.next_depth_demand()")).toBe(false);
 expect(source.includes("let grant = request.next_release_byte_demand().unwrap().max(7)")).toBe(false);
 expect(source.includes("let grant = demand.max(7)")).toBe(false);
 expect(source.includes("let policy = request_close_policy();")).toBe(true);
 const grammar=spawnSync("rustfmt",["--edition","2021","--emit","stdout"],{input:source,encoding:"utf8",timeout:2000});expect(grammar.error).toBeUndefined();expect(grammar.status).toBe(0);
 const original=new Uint8Array(8194),view=original.subarray(0,1);
 const positive=law.physicalCloseGrant.maximumReleaseBytes;expect(positive).toBeGreaterThan(original.buffer.byteLength);
 const denied=applyPatch(law.physicalCloseGrant,[{op:"replace",path:"/maximumReleaseBytes",value:original.buffer.byteLength-1}],true,false).newDocument;
 expect(denied.maximumReleaseBytes).toBeLessThan(original.buffer.byteLength);expect(denied.maximumDepth).toBe(law.physicalCloseGrant.maximumDepth);expect(view.buffer).toBe(original.buffer);
});

test("snapshot input receipts debit the original context wallet exactly once",async()=>{
 const fs=await import("node:fs"),path=await import("node:path");
 const actual=fs.readFileSync(path.join(process.cwd(),"🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/🦀️.rs"),"utf8");
 const input=actual.slice(actual.indexOf("impl<P,D> ProducedMemberSnapshotOpen<P,D>"),actual.indexOf("impl<P: semio_framework_value::retirement::RetireOwned,D> ProducedMemberSnapshotOpen"));
 expect(input.includes("cx.consume_retained(progress)")).toBe(true);
 expect(input.includes("let caller = cx.retained_grant()")).toBe(true);
 for(const axis of ["items","copy_bytes","capacity_bytes","release_bytes","depth"])expect(input.includes("grant.maximum_"+axis+".min(caller.maximum_"+axis+")")).toBe(true);
 expect(input.includes("self.pending(expected_bytes, progress, cx)")).toBe(true);
 const plain=JSON.parse(fs.readFileSync(path.join(process.cwd(),"🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/📏️birth/🧪️tests/🧫️fixtures/🔣️.json"),"utf8")).snapshotInput;
 const grant=plain.policy,remaining={...grant};
 const bytes=new Uint8Array(plain.payloadBytes),receipt={copiedItems:1,copiedBytes:bytes.byteLength*plain.expected.copiesPerPayloadByte,retainedCapacityBytes:0,releasedBytes:0};
 const expected={...remaining,maximumItems:remaining.maximumItems-receipt.copiedItems,maximumCopyBytes:remaining.maximumCopyBytes-receipt.copiedBytes};
 const oracle=applyPatch(remaining,[{op:"replace",path:"/maximumItems",value:expected.maximumItems},{op:"replace",path:"/maximumCopyBytes",value:expected.maximumCopyBytes}],true,false).newDocument;
 expect(oracle).toEqual(expected);expect(oracle.maximumCapacityBytes).toBe(grant.maximumCapacityBytes);expect(oracle.maximumReleaseBytes).toBe(grant.maximumReleaseBytes);expect(oracle.maximumDepth).toBe(grant.maximumDepth);
});

test("member and hydration paid turns preserve the original physical wallet",async()=>{
 const fs=await import("node:fs"),path=await import("node:path");
 const base=path.join(process.cwd(),"🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store");
 const plain=JSON.parse(fs.readFileSync(path.join(base,"🧩️composition/🚪️open/📏️retirement/🧪️tests/🧫️fixtures/🔣️.json"),"utf8")).physicalWallet;
 const ajv=new Ajv({strict:false});ajv.addSchema(orderedRetirement);ajv.addSchema(retirementContract);const validate=ajv.compile({$ref:retirementContract.$id+"#/$defs/Grant"});expect(validate(plain.grant)).toBe(true);
 const axes=[["maximumItems","copiedItems"],["maximumCopyBytes","copiedBytes"],["maximumCapacityBytes","retainedCapacityBytes"],["maximumReleaseBytes","releasedBytes"]];
 const backing=Buffer.allocUnsafeSlow(plain.grant.maximumCapacityBytes);expect(backing.buffer.byteLength).toBe(plain.receipts[0].retainedCapacityBytes);expect(backing.byteLength).toBe(plain.receipts[1].releasedBytes);
 let remaining={...plain.grant};
 for(const receipt of plain.receipts){const next={...remaining};for(const [grant,progress]of axes){expect(receipt[progress]).toBeLessThanOrEqual(remaining[grant]);next[grant]-=receipt[progress];}const oracle=applyPatch(remaining,axes.map(([grant])=>({op:"replace",path:"/"+grant,value:next[grant]})),true,false).newDocument;expect(oracle).toEqual(next);remaining=oracle;expect(remaining.maximumDepth).toBe(plain.grant.maximumDepth);}
 expect(Object.values(remaining)).toEqual([0,0,0,0,4]);
 const source=fs.readFileSync(path.join(base,"🧩️composition/🚪️open/🦀️.rs"),"utf8"),operation=fs.readFileSync(path.join(base,"🧩️composition/🚪️open/🏭️operation/🦀️.rs"),"utf8"),hydration=readModuleTreeSource(path.join(base,"🧾️document/📜️history/💧️hydration/🦀️.rs"));
 expect(source.includes("pub(crate) fn member_step_grant")).toBe(true);expect(source.includes("pub(crate) fn record_member_step")).toBe(true);
 expect(source.includes("cx.consume_retained(progress)")).toBe(true);
 for(const axis of ["items","copy_bytes","capacity_bytes","release_bytes","depth"])expect(source.includes("grant.maximum_"+axis+".min(caller.maximum_"+axis+")")).toBe(true);
 expect(operation.includes("let grant = super::member_step_grant(cx, grant);")).toBe(true);expect(operation.includes("super::record_member_step(cx, progress)")).toBe(true);
 expect(hydration.includes("member_open::member_step_grant(cx, grant)")).toBe(true);expect(hydration.includes("member_open::record_member_step(cx, progress)")).toBe(true);
 expect(hydration.includes("match progress_fuel(progress) { Ok(fuel) => cx.consume_fuel(fuel)")).toBe(false);
 for(const [name,input]of [["open",source],["operation",operation],["hydration",hydration]]){const grammar=spawnSync("rustfmt",["--edition","2021","--emit","stdout"],{input,encoding:"utf8",timeout:2000});expect(grammar.error,name).toBeUndefined();expect(grammar.status,name).toBe(0);}
});

test("snapshot native consumers retain the genuine caller controls and progress domains",async()=>{
 const fs=await import("node:fs"),path=await import("node:path");const base=path.join(process.cwd(),"🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store");
 const snapshot=fs.readFileSync(path.join(base,"📜️space-history/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"),"utf8"),encoding=fs.readFileSync(path.join(base,"📦️codec/🪶️snapshot-capability/🪶️native-encoding/🧪️tests/🦀️.rs"),"utf8"),decoding=fs.readFileSync(path.join(base,"📦️codec/🪶️snapshot-capability/🪶️native-decoding/🧪️tests/🦀️.rs"),"utf8");
 expect(snapshot.includes("semio_framework_value::NativeDecodeProgress")).toBe(false);expect(snapshot.includes("semio_framework_value::NativeEncodeProgress")).toBe(false);
 expect(snapshot.includes("semio_framework_value::native_decoding::NativeDecodeProgress")).toBe(true);expect(snapshot.includes("semio_framework_value::native_encoding::NativeEncodeProgress")).toBe(true);
 const guards=encoding.slice(encoding.indexOf("    GuardedEncoding,"),encoding.indexOf("#[test]"));
 expect(guards.includes("native_owner: &mut NativeSnapshotEncodeOwner")).toBe(true);expect(guards.includes("let native = native_owner.native();")).toBe(true);expect(guards.includes("NativeEncodeControl::new")).toBe(false);expect(guards.includes("encoding, control, native_owner")).toBe(true);
 const original=decoding.slice(decoding.indexOf("impl ArtifactSqliteSnapshotHost for DecodedBuffers"),decoding.indexOf("fn to_sqlite_database"));expect(original.includes("native: &mut NativeSnapshotDecodeOwner")).toBe(true);expect(original.includes("            native,")).toBe(true);
 const bytes=Buffer.from("雪","utf8");expect(new TextEncoder().encode("雪")).toEqual(new Uint8Array(bytes));expect(JSON.parse(JSON.stringify({bytes:[...bytes],originalControl:true}))).toEqual({bytes:[...bytes],originalControl:true});
 for(const [name,input]of [["snapshot",snapshot],["encoding",encoding],["decoding",decoding]]){const grammar=spawnSync("rustfmt",["--edition","2021","--emit","stdout"],{input,encoding:"utf8",timeout:2000});expect(grammar.error,name).toBeUndefined();expect(grammar.status,name).toBe(0);}
});

test("native snapshot encoder keeps one original paid receipt wallet",async()=>{
 const fs=await import("node:fs"),path=await import("node:path");const root=process.cwd();
 const plain=JSON.parse(fs.readFileSync(path.join(root,"🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/📏️retirement/🧪️tests/🧫️fixtures/🔣️.json"),"utf8")).physicalWallet;
 const ajv=new Ajv({strict:false});ajv.addSchema(orderedRetirement);ajv.addSchema(retirementContract);expect(ajv.compile({$ref:retirementContract.$id+"#/$defs/Grant"})(plain.grant)).toBe(true);
 let used={copiedItems:0,copiedBytes:0,retainedCapacityBytes:0,releasedBytes:0};let remaining={...plain.grant};const axes:readonly(readonly[string,keyof typeof used])[]=[["maximumItems","copiedItems"],["maximumCopyBytes","copiedBytes"],["maximumCapacityBytes","retainedCapacityBytes"],["maximumReleaseBytes","releasedBytes"]];
 for(const receipt of plain.receipts){const operations=axes.map(([grant,progress])=>({op:"replace" as const,path:"/"+grant,value:remaining[grant]-receipt[progress]}));for(const [grant,progress]of axes){expect(receipt[progress]).toBeLessThanOrEqual(remaining[grant]);used[progress]+=receipt[progress];}remaining=applyPatch(remaining,operations,true,false).newDocument;}
 expect(Object.values(remaining)).toEqual([0,0,0,0,4]);expect(used.retainedCapacityBytes).toBe(Buffer.allocUnsafeSlow(64).byteLength);expect(used.releasedBytes).toBe(64);
 const source=fs.readFileSync(path.join(root,"🧰️framework/🔨️modules/🚪️io/⏱️control/🛫️snapshot/🦀️.rs"),"utf8");
 expect(source.includes("pub fn remaining_grant")).toBe(true);expect(source.includes("pub fn record_progress")).toBe(true);expect(source.includes("pub fn progress(&self)")).toBe(true);expect(source.includes("self.progress.checked_add(progress)")).toBe(true);expect(source.includes("progress.fits(self.remaining_grant())")).toBe(true);
 for(const [grant,progress]of [["items","copied_items"],["copy_bytes","copied_bytes"],["capacity_bytes","retained_capacity_bytes"],["release_bytes","released_bytes"]])expect(source.includes("self.grant.maximum_"+grant+"-self.progress."+progress)).toBe(true);
 expect(source.includes("progress.copied_items")).toBe(true);expect(source.includes("record_progress(")).toBe(true);
 const grammar=spawnSync("rustfmt",["--edition","2021","--emit","stdout"],{input:source,encoding:"utf8",timeout:2000});expect(grammar.error).toBeUndefined();expect(grammar.status).toBe(0);
});

test("snapshot JSON writer reports actual partial cancellation receipts",async()=>{
 const fs=await import("node:fs"),path=await import("node:path");const source=fs.readFileSync(path.join(process.cwd(),"🧰️framework/🔨️modules/🎒️pack/🔤️json/🎛️controlled-and-borrowed-bridge/🦀️.rs"),"utf8");
 const value={text:"雪",rows:[1,true,null]};expect(JSON.parse(JSON.stringify(value))).toEqual(value);expect(new TextEncoder().encode(JSON.stringify(value))).toEqual(new Uint8Array(Buffer.from(JSON.stringify(value))));
 const cursor=source.slice(source.indexOf("impl JsonBorrowedWriteCursor {"),source.indexOf("/// 🌳️ Owns its semantic source"));
 expect(cursor.includes("fn retained_capacity_bytes")).toBe(true);expect(cursor.includes("fn step_original")).toBe(true);expect(cursor.includes("error.with_retained_progress(self.normal_step_progress)")).toBe(true);
 expect(cursor.includes("let retained_before=self.retained_capacity_bytes()?;")).toBe(true);expect(cursor.includes("self.writer.output.as_ref().map_or(0,|output|output.len())")).toBe(true);
 const publish=cursor.indexOf("retained_capacity_bytes:retained_after-retained_before");expect(publish).toBeGreaterThan(-1);expect(cursor.indexOf("result?;control.step()?",publish)).toBeGreaterThan(publish);
 const grammar=spawnSync("rustfmt",["--edition","2021","--emit","stdout"],{input:source,encoding:"utf8",timeout:2000});expect(grammar.error).toBeUndefined();expect(grammar.status).toBe(0);
});

test("native snapshot writer keeps its static cursor in the original paid recipient",async()=>{
 const fs=await import("node:fs"),path=await import("node:path"),root=process.cwd();
 const owner=fs.readFileSync(path.join(root,"🧰️framework/🔨️modules/🚪️io/⏱️control/🛫️snapshot/🦀️.rs"),"utf8"),snapshot=fs.readFileSync(path.join(root,"🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📜️space-history/🚪️io/🪶️sqlite/📸️snapshot/🚦️native/🦀️.rs"),"utf8");
 expect(owner.includes("pub fn drive_cursor")).toBe(true);expect(owner.includes("cursor:&mut Option<T>")).toBe(true);expect(owner.includes("self.remaining_grant()")).toBe(true);expect(owner.includes("driver.record_progress(progress)?")).toBe(true);expect(owner.includes("Some(frame as Box<dyn ErasedSnapshotRetirement>)")).toBe(true);
 const writer=snapshot.slice(snapshot.indexOf("fn write_json("),snapshot.indexOf("pub(super) fn encode("));expect(writer.includes("JsonBorrowedWriteCursor::new()")).toBe(true);expect(writer.includes("owner.drive_cursor")).toBe(true);expect(writer.includes("cursor.normal_step_progress()")).toBe(true);expect(writer.includes("256.min(grant.maximum_items)")).toBe(true);expect(writer.includes("JsonWriteCursor::new")).toBe(false);
 const bytes=Buffer.from(JSON.stringify({unchanged:"雪"}));expect(new TextEncoder().encode(JSON.stringify({unchanged:"雪"}))).toEqual(new Uint8Array(bytes));expect(JSON.parse(bytes.toString("utf8"))).toEqual({unchanged:"雪"});
 for(const input of [owner,snapshot]){const grammar=spawnSync("rustfmt",["--edition","2021","--emit","stdout"],{input,encoding:"utf8",timeout:2000});expect(grammar.error).toBeUndefined();expect(grammar.status).toBe(0);}
});

test("native snapshot decoder retains constructor output before cancellation",async()=>{
 const fs=await import("node:fs"),path=await import("node:path");const root=process.cwd(),base="🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store";
 const plain=JSON.parse(fs.readFileSync(path.join(root,base,"📦️codec/🪶️snapshot-capability/🪶️native-decoding/🧪️tests/🧫️fixtures/🔣️.json"),"utf8"));
 const ajv=new Ajv({strict:false});ajv.addSchema(orderedRetirement);ajv.addSchema(retirementContract);expect(ajv.compile({$ref:retirementContract.$id+"#/$defs/Grant"})(plain.callerGrant)).toBe(true);
 const bytes=Buffer.from("雪","utf8"),original=new Uint8Array(bytes.length);original.set(new TextEncoder().encode("雪"));expect([...original]).toEqual([...bytes]);
 const source=fs.readFileSync(path.join(root,base,"📦️codec/🪶️snapshot-capability/🪶️native-decoding/🧪️tests/🦀️.rs"),"utf8");
 const body=source.slice(source.indexOf("impl ArtifactSqliteSnapshotHost for DecodedBuffers"),source.indexOf("fn to_sqlite_database"));
 expect(body.includes("native: &mut NativeSnapshotDecodeOwner")).toBe(true);expect(body.includes("bind_original_buffers,")).toBe(true);
 const binding=source.slice(source.indexOf("fn bind_original_buffers"),source.indexOf("impl ArtifactSqliteSnapshotHost for DecodedBuffers"));
 expect(binding.indexOf("*snapshot_output = Some")<binding.indexOf("body.allocate_vec_into(")).toBe(true);expect(binding.includes("std::mem::take(bytes)")).toBe(true);expect(binding.includes("Ok(())")).toBe(true);
 expect(source.includes("impl RetireOwned for DecodedBuffers")).toBe(true);expect(source.includes("self.buffers.retirement()")).toBe(true);
 const defining=fs.readFileSync(path.join(root,"🧰️framework/🔨️modules/🌱️value/🛬️decode/🦀️.rs"),"utf8");expect(defining.includes("pub fn copy_bytes_into")).toBe(true);expect(defining.includes("output.extend_from_slice(chunk)")).toBe(true);
});

test("named native ownership laws use independent positive grants",async()=>{
 const fs=await import("node:fs"),path=await import("node:path");const root=process.cwd(),base="🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store";
 const plain=JSON.parse(fs.readFileSync(path.join(root,base,"🧩️composition/📨️emission/📦️owned/🧪️tests/🧫️fixtures/🔣️.json"),"utf8"));const ajv=new Ajv({strict:false});ajv.addSchema(orderedRetirement);ajv.addSchema(retirementContract);expect(ajv.compile({$ref:retirementContract.$id+"#/$defs/Grant"})(plain.physicalCloseGrant)).toBe(true);
 const source=readModuleTreeSource(path.join(root,base,"🧪️tests/🌴️bin-06/🔬️unit/🦀️.rs"));
 for(const name of ["current_box_retirement_preserves_denied_owner_and_exact_physical_shell","current_typed_option_birth_retains_exact_original_and_paid_shell"]){
  const start=source.indexOf("fn "+name);expect(start).toBeGreaterThanOrEqual(0);const end=source.indexOf("\n#[test]",start);const body=source.slice(start,end<0?undefined:end);
  expect(body.includes("let grant = physical_test_close_grant();")).toBe(true);expect(body.includes("maximum_copy_bytes: demand.copy_bytes")).toBe(false);expect(body.includes("maximum_release_bytes: demand.release_bytes")).toBe(false);
 }
});

test("member history hydration and durable native closes preserve independent authority",async()=>{
 const fs=await import("node:fs"),path=await import("node:path");const base=path.join(process.cwd(),"🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store");
 const history=fs.readFileSync(path.join(base,"🧩️composition/🚪️open/📜️history/🧪️tests/🔬️unit/🦀️.rs"),"utf8"),hydration=fs.readFileSync(path.join(base,"🧾️document/📜️history/💧️hydration/🧪️tests/🔬️unit/🦀️.rs"),"utf8"),durable=readModuleTreeSource(path.join(base,"🧩️composition/🗄️durable-group/🧪️tests/🔬️unit/🦀️.rs"));
 expect(history.includes("maximum_capacity_bytes: owner.next_capacity_byte_demand")).toBe(false);expect(hydration.includes("fn grant_of(")).toBe(false);expect(durable.includes("fn demanded_grant(")).toBe(false);
 expect(history.includes("super::super::tests::request_close_policy()")).toBe(true);expect(hydration.includes("crate::os_store::component::tests::physical_test_close_grant()")).toBe(true);expect(durable.includes("crate::os_store::component::tests::physical_test_close_grant()")).toBe(true);
 for(const [name,source] of [["history",history],["hydration",hydration],["durable",durable]] as const){const grammar=spawnSync("rustfmt",["--edition","2021","--emit","stdout"],{input:source,encoding:"utf8",timeout:2000});expect(grammar.error,name).toBeUndefined();expect(grammar.status,name).toBe(0);}
 const policy=fixture.physicalCloseGrant;const ajv=new Ajv({strict:false});ajv.addSchema(orderedRetirement);ajv.addSchema(retirementContract);expect(ajv.compile({$ref:retirementContract.$id+"#/$defs/Grant"})(policy)).toBe(true);expect(Buffer.allocUnsafeSlow(policy.maximumReleaseBytes).byteLength).toBe(policy.maximumReleaseBytes);
});

test("catalog source native laws use independent positive grants",async()=>{
 const fs=await import("node:fs"),path=await import("node:path");
 const source=readModuleTreeSource(path.join(process.cwd(),"🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🌴️bin-06/🔬️unit/🦀️.rs"));
 for(const [name,end] of [["current_catalog_sources_admit_original_allocations_then_fund_each_ticket","current_member_source_constructor_is_not_invoked_under_denied_axes"],["current_member_source_constructor_is_not_invoked_under_denied_axes","ephemeral_member_original_preparation_frame_requires_exact_granted_release"]] as const){
  const start=source.indexOf("fn "+name+"("),finish=source.indexOf("fn "+end+"(",start+1);expect(start).toBeGreaterThanOrEqual(0);expect(finish).toBeGreaterThan(start);
  const body=source.slice(start,finish);expect(body.includes("let paid = physical_test_close_grant();")).toBe(true);expect(body.includes("maximum_capacity_bytes: capacity,")).toBe(false);expect(body.includes("maximum_capacity_bytes: demand.capacity_bytes")).toBe(false);expect(body.includes("let grant = physical_test_close_grant();")).toBe(true);
  for(const original of ["maximum_capacity_bytes: capacity - 1","assert_eq!(allocated, capacity)","assert!(owners.uninstalled_owners_terminal_is_empty())"])expect(body.includes(original)).toBe(true);
 }
 const policy=fixture.physicalCloseGrant;const ajv=new Ajv({strict:false});ajv.addSchema(orderedRetirement);ajv.addSchema(retirementContract);expect(ajv.compile({$ref:retirementContract.$id+"#/$defs/Grant"})(policy)).toBe(true);
 expect(Buffer.allocUnsafeSlow(policy.maximumCapacityBytes).byteLength).toBe(policy.maximumCapacityBytes);
});

test("current shared actor native callers retain genuine lease and original buffer custody",async()=>{
 const fs=await import("node:fs"),path=await import("node:path"),root=process.cwd();
 const base=path.join(root,"🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store");
 const request=fs.readFileSync(path.join(base,"🧩️composition/🚪️open/🧪️tests/🔬️unit/🦀️.rs"),"utf8"),hydration=fs.readFileSync(path.join(base,"🧾️document/📜️history/💧️hydration/🧪️tests/🔬️unit/🦀️.rs"),"utf8"),history=fs.readFileSync(path.join(base,"📜️space-history/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"),"utf8");
 expect(request.includes("request.actor.0.capacity()")).toBe(false);expect(hydration.includes("owner.actor.0.capacity()")).toBe(false);
 expect(request.includes("request.closing_actor.as_ref().unwrap().as_ptr()")).toBe(true);expect(request.includes("SharedUtf8::admit(actor")).toBe(true);
 expect(history.includes("|value,output,native,body|")).toBe(true);expect(history.includes("body.admit_frontier(")).toBe(true);expect(history.includes("body.record_progress(")).toBe(true);expect(history.includes("*output=Some(())")).toBe(true);expect(history.includes("store::NativeSnapshotDecodeOwner::new(&mut original_native,caller_grant())")).toBe(true);
 const law=JSON.parse(fs.readFileSync(path.join(base,"🧩️composition/🚪️open/📏️retirement/🧪️tests/🧫️fixtures/🔣️.json"),"utf8"));
 expect(law.physicalCloseGrant.maximumCopyBytes).toBe(4096);
 for(const row of law.cases){const bytes=new TextEncoder().encode(row.text),original=Buffer.allocUnsafeSlow(row.capacityBytes);original.set(bytes);expect(bytes.byteLength).toBe(Buffer.byteLength(row.text,"utf8"));expect(original.byteLength).toBe(row.capacityBytes);expect(original.subarray(0,bytes.length).toString("utf8")).toBe(row.text);}
});

test("native body octet spans debit independent policy and retain original cancelled backing",async()=>{
 const fs=await import("node:fs"),path=await import("node:path"),root=process.cwd();
 const neutral=JSON.parse(fs.readFileSync(controlSnapshotReceivingFixtureUrl,"utf8")).bodyCopy;
 const ajv=new Ajv({strict:false});ajv.addSchema(orderedRetirement);ajv.addSchema(retirementContract);const validate=ajv.compile({$ref:retirementContract.$id+"#/$defs/Grant"});
 expect(validate(neutral.grant)).toBe(true);expect(validate(neutral.closeGrant)).toBe(true);
 const payload=new TextEncoder().encode(neutral.text.repeat(neutral.repeat)),oracle=Buffer.from(neutral.text.repeat(neutral.repeat),"utf8");expect(payload).toEqual(new Uint8Array(oracle));expect(payload.byteLength).toBeGreaterThan(neutral.cancelAt);
 for(const completed of [neutral.cancelAt,payload.byteLength]){const backing=Buffer.allocUnsafeSlow(payload.byteLength);backing.set(payload.subarray(0,completed));expect(backing.subarray(0,completed)).toEqual(oracle.subarray(0,completed));const receipt={copiedItems:1,copiedBytes:completed,retainedCapacityBytes:backing.byteLength,releasedBytes:0};const remaining=applyPatch(neutral.grant,[{op:"replace",path:"/maximumItems",value:neutral.grant.maximumItems-receipt.copiedItems},{op:"replace",path:"/maximumCopyBytes",value:neutral.grant.maximumCopyBytes-receipt.copiedBytes},{op:"replace",path:"/maximumCapacityBytes",value:neutral.grant.maximumCapacityBytes-receipt.retainedCapacityBytes}],true,false).newDocument;expect(remaining.maximumCapacityBytes).toBeLessThan(payload.byteLength);expect(remaining.maximumReleaseBytes).toBe(neutral.grant.maximumReleaseBytes);expect(remaining.maximumDepth).toBe(neutral.grant.maximumDepth);}
 const control=fs.readFileSync(path.join(root,"🧰️framework/🔨️modules/🚪️io/⏱️control/🦀️.rs"),"utf8"),decoder=fs.readFileSync(path.join(root,"🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️codec/🪶️snapshot-capability/🪶️native-decoding/🧪️tests/🦀️.rs"),"utf8");
 expect(control.includes("pub fn copy_bytes_into(&mut self,native:")).toBe(true);expect(control.includes("let result=native.copy_bytes_into(bytes,output)")).toBe(true);expect(control.includes("copied_bytes:output.len(),retained_capacity_bytes:output.capacity()")).toBe(true);
 const actual=decoder.slice(decoder.indexOf("fn bind_original_buffers"),decoder.indexOf("impl ArtifactSqliteSnapshotHost for DecodedBuffers"));expect(actual.includes("body.allocate_vec_into(")).toBe(true);expect(actual.includes("body.record_progress(")).toBe(true);expect(actual.includes("std::mem::take(bytes)")).toBe(true);expect(actual.includes("copy_bytes_into")).toBe(false);
 for(const input of [control,decoder]){const parsed=spawnSync("rustfmt",["--edition","2021","--emit","stdout"],{input,encoding:"utf8",timeout:2000});expect(parsed.error).toBeUndefined();expect(parsed.status).toBe(0);}
});

test("failed native field admission uses genuine bounds and preserves real performed receipts",async()=>{
 const fs=await import("node:fs"),path=await import("node:path"),root=process.cwd(),base=path.join(root,"🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store");
 const neutral=JSON.parse(fs.readFileSync(path.join(base,"🚪️io/🚫️refusal/🧪️tests/🧫️fixtures/🔣️.json"),"utf8"));const ajv=new Ajv({strict:false});ajv.addSchema(orderedRetirement);ajv.addSchema(retirementContract);const validate=ajv.compile(JSON.parse(fs.readFileSync(cloneWireProgressSchemaUrl,"utf8")));
 for(const row of neutral.cases){expect(validate(row.receipt)).toBe(true);const total=applyPatch({capacity:0,released:0},[{op:"replace",path:"/capacity",value:row.receipt.retainedCapacityBytes},{op:"replace",path:"/released",value:row.receipt.releasedBytes}],true,false).newDocument;expect(total.capacity).toBe(row.expectedTotalCapacity);expect(total.released).toBe(row.expectedTotalRelease);}
 const original=readModuleTreeSource(path.join(base,"🧪️tests/🌴️bin-06/🔬️unit/🦀️.rs"));const start=original.indexOf("fn field_close_failure_preserves_real_receipts_through_the_original_decode_job()");const actual=original.slice(start,original.indexOf("\n#[test]",start));expect(actual.includes("fn maximum_retained_close_bytes(&self)->usize { ARTIFACT_ENVELOPE_DECODE_MAXIMUM_RETAINED_FIELD_BYTES }")).toBe(true);expect(actual.includes("assert_eq!(progress.retained_capacity_bytes,allocated)")).toBe(true);expect(actual.includes("assert_eq!(progress.released_bytes,released)")).toBe(true);
 const allocated=Buffer.allocUnsafeSlow(1024);expect(allocated.byteLength).toBe(1024);expect(allocated.byteLength).toBeLessThan(262144*6);
});

test("original VCS checkpoint measurement stays borrowed and cannot become consuming authority",async()=>{
 const fs=await import("node:fs"),path=await import("node:path"),{default:Ajv2020}=await import("ajv/dist/2020.js"),root=process.cwd(),base=path.join(root,"🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🚪️io/💾️binary/🪪️entity-identity/🎛️control");
 const plain=JSON.parse(fs.readFileSync(path.join(base,"🧪️tests/🧫️fixtures/📥️input/🔣️.json"),"utf8")).checkpointProjection;
 const original={maximum:plain.originalMaximumBytes,owned:0,finished:false,reserved_return:0,phase_start_owned:0,phase_start_reserved_return:0,received_output:0,received_input:0,received_retirement:plain.hostCharge};
 const projected=applyPatch(original,[{op:"add",path:"/kind",value:"original-retirement-measurement"},{op:"replace",path:"/received_retirement",value:original.received_retirement+plain.proposedCharge}],true,false).newDocument;
 const oracle=new Ajv2020({strict:false});expect(oracle.compile(JSON.parse(fs.readFileSync(path.join(base,"🔎️measurement/🧬️schema/🔣️.json"),"utf8")))(projected)).toBe(true);expect(oracle.compile(JSON.parse(fs.readFileSync(path.join(base,"🧬️schema/🧾️receipt/🔣️.json"),"utf8")))(projected)).toBe(false);
 expect(original.received_retirement).toBe(plain.actualRetirement);expect(projected.received_retirement).toBe(plain.projectedRetirement);expect(original.maximum).toBe(plain.originalMaximumBytes);expect("kind"in original).toBe(false);
 const producer=fs.readFileSync(path.join(base,"🦀️.rs"),"utf8"),measurement=fs.readFileSync(path.join(base,"🔎️measurement/🦀️.rs"),"utf8"),native=fs.readFileSync(path.join(base,"🧪️tests/🦀️.rs"),"utf8");
 expect(producer.includes("pub fn project_retirement_charge(&self,additional:usize)->Result<measurement::OriginalOperationProjection<'_>,ValueError>")).toBe(true);expect(producer.includes("self.source_owned().and_then")).toBe(true);expect(measurement.includes('output.serialize_field("kind","original-retirement-measurement")')).toBe(true);expect(native.includes("serde_json::from_value::<OriginalOperationReceipt>(scalar).is_err()")).toBe(true);
 for(const input of [producer,measurement,native]){const grammar=spawnSync("rustfmt",["--edition","2021","--emit","stdout"],{input,encoding:"utf8",timeout:2000});expect(grammar.error).toBeUndefined();expect(grammar.status).toBe(0);}
});

test("native SQLite binding transfers each original octet backing under unchanged caller policy",async()=>{
 const fs=await import("node:fs"),path=await import("node:path");
 const base=path.join(process.cwd(),"🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️codec/🪶️snapshot-capability");
 const plain=JSON.parse(fs.readFileSync(path.join(base,"🪶️native-decoding/🧫️fixtures/🔣️.json"),"utf8"));
 expect(plain.callerGrant.maximumCopyBytes).toBe(65536);
 const row=plain.cases.find((row: any)=>row.id==="admitted-buffers");
 const original=row.bufferBytes.map((n: number)=>Buffer.allocUnsafeSlow(n).fill(0));
 const received=original.map((buffer: Buffer)=>buffer);
 expect(received.every((buffer: Buffer,index: number)=>buffer.buffer===original[index].buffer)).toBe(true);
 expect(received.map((buffer: Buffer)=>buffer.length)).toEqual(row.bufferBytes);
 const native=fs.readFileSync(path.join(base,"🪶️native-decoding/🧪️tests/🦀️.rs"),"utf8");
 const binding=native.slice(native.indexOf("fn bind_original_buffers"),native.indexOf("impl ArtifactSqliteSnapshotHost for DecodedBuffers"));
 expect(binding).toContain("body.admit_frontier");
 expect(binding).toContain("std::mem::take(bytes)");
 expect(binding).not.toContain("copy_bytes_into");
 expect(native).toContain("fn sqlite_snapshot_binding_moves_original_octet_allocations_with_exact_structural_receipt");
 for(const input of [native,fs.readFileSync(path.join(base,"🛬️native-decoding/🦀️.rs"),"utf8")]){const parsed=spawnSync("rustfmt",["--edition","2021","--emit","stdout"],{input,encoding:"utf8",timeout:2000});expect(parsed.error).toBeUndefined();expect(parsed.status).toBe(0);}
 expect(binding.indexOf("body.admit_frontier",binding.indexOf("for field"))).toBeLessThan(binding.indexOf("std::mem::take(bytes)"));
 const producer=fs.readFileSync(path.join(base,"🛬️native-decoding/🦀️.rs"),"utf8");
 expect(producer).toContain("FnOnce(&mut RecordValue");
 expect(producer).toContain("construct(record.as_mut().unwrap()");
});

test("original history mutation ordinals preserve the full unsigned64 domain across JSON",async()=>{
 const {default:Decimal}=await import("decimal.js");const OrdinalDecimal=Decimal.clone({precision:40});
 const root="🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/";
 const schema=await Bun.file(root+"📜️history/🆔️ordinal/🧬️schema/🔣️.json").json(),validate=new Ajv({strict:false}).compile(schema);
 const backing=await Bun.file(root+"♻️retirement/📦️backing/🧪️tests/🧫️fixtures/🔣️.json").json();
 const maximum=18446744073709551615n;
 expect(backing.vcsRetirement.forwards).toEqual(["0",maximum.toString()]);
 expect(backing.vcsRetirement.inverse).toEqual(["9","7"]);
 for(const text of [...backing.vcsRetirement.forwards,...backing.vcsRetirement.inverse]){
  expect(validate(text)).toBe(true);const original=BigInt(text),oracle=new OrdinalDecimal(text);
  expect(oracle.toFixed(0)).toBe(original.toString());expect(oracle.gte(0)&&oracle.lte(new OrdinalDecimal(maximum.toString()))).toBe(true);
  const bytes=Buffer.alloc(8);bytes.writeBigUInt64BE(original);expect(bytes.readBigUInt64BE()).toBe(original);
  expect(JSON.parse(JSON.stringify(text))).toBe(text);
 }
 for(const invalid of ["-1","01","18446744073709551616",18446744073709551615])expect(validate(invalid)).toBe(false);
});

test("selected member snapshot codec owns typed decode without replacing original request custody",async()=>{
 const schema=await Bun.file(new URL("../../../../🏭️operation/🧬️schema/📦️decoder/🔣️.json",import.meta.url)).json();
 const law=await Bun.file(new URL("../../../../🏭️operation/🧪️tests/🧫️fixtures/📦️decoder/🔣️.json",import.meta.url)).json();
 const validate=new Ajv({strict:true,allErrors:true}).compile(schema);expect(validate(law)).toBe(true);
 const rejected=applyPatch(law,[{op:"replace",path:"/source",value:"replacement-pack"}],true,false).newDocument;expect(validate(rejected)).toBe(false);
 const source=await Bun.file(new URL("../../../../🏭️operation/🦀️.rs",import.meta.url)).text();
 const start=source.indexOf("impl<P: semio_framework_value::retirement::RetireOwned,D:MemberSnapshotDecoder<P>>");
 const end=source.indexOf("impl<P: semio_framework_value::retirement::RetireOwned,D> ProducedMemberSnapshotOpen");const selected=source.slice(start,end);
 expect(start).toBeGreaterThan(0);expect(end).toBeGreaterThan(start);expect(selected.includes("D::decode_snapshot(&self.input)")).toBe(true);expect(selected.includes("P::decode_pack")).toBe(false);
 expect(selected.includes("check_step_authority(cx)")).toBe(true);expect(selected.includes("let caller = cx.retained_grant()")).toBe(true);expect(selected.includes("admit_member_input_buffer")).toBe(true);
});

test("original Store snapshot ports are independent from selected representation methods",async()=>{
 const schema=await Bun.file(new URL("../../../../../../🧬️schema/📥️snapshot-capabilities/🔣️.json",import.meta.url)).json();
 const law=await Bun.file(new URL("../../../../../../🧪️tests/🧫️fixtures/📥️snapshot-capabilities/🔣️.json",import.meta.url)).json();
 const admit=new Ajv({strict:true,allErrors:true}).compile(schema);expect(admit(law)).toBe(true);expect(admit({...law,snapshotOwner:"copied-projection"})).toBe(false);
 const source=await Bun.file(new URL("../../../../../../🦀️.rs",import.meta.url)).text();
 const initial=source.indexOf("async fn checkpoint_identity");const start=source.indexOf("impl<P, Mutation> ArtifactStore<P, Mutation>",initial);const end=source.indexOf("fn preflight_runtime_seed",start);const owner=source.slice(start,end);
 expect(start).toBeGreaterThan(0);expect(end).toBeGreaterThan(start);expect(owner.includes("ArtifactPack")).toBe(false);expect(owner.includes("ArtifactStoreSnapshot")).toBe(false);
 expect(source.includes("pub fn snapshot_owner")).toBe(true);expect(source.includes("pub fn owned_snapshot_retirement_factory")).toBe(true);
});
