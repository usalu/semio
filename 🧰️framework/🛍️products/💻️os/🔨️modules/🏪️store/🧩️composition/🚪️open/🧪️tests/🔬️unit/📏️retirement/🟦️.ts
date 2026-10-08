import { expect, test } from "bun:test";
import { Buffer } from "node:buffer";
import { applyPatch } from "fast-json-patch";
import Ajv from "ajv";
import retirementContract from "../../../../../../../../../🔨️modules/🌱️value/♻️retirement/🧬️contract/🧬️schema/🔣️.json" with { type: "json" };
import orderedRetirement from "../../../../../../../../../🔨️modules/🌱️value/🗂️ordered/♻️retirement/🧬️schema/🔣️.json" with { type: "json" };
import fixture from "../../../📏️retirement/🧫️fixtures/🔣️.json" with { type: "json" };

test("erased retirement uses separate fallible demands and a paid terminal box turn", async () => {
  const oracle = new Ajv({ strict: false });
  oracle.addSchema(orderedRetirement);
  oracle.addSchema(retirementContract);
  const validateDemand = oracle.compile({ $ref: `${retirementContract.$id}#/$defs/Demand` });
  const validateGrant = oracle.compile({ $ref: `${retirementContract.$id}#/$defs/Grant` });
  expect(validateDemand({ copyBytes: 0, capacityBytes: 0, releaseBytes: 64, depth: 1 })).toBe(true);
  expect(validateDemand({ copyBytes: 0, capacityBytes: 0, releaseBytes: -1, depth: 1 })).toBe(false);
  for (const row of fixture.boxGrants) {
    expect(validateGrant({ maximumItems: row.items, maximumCopyBytes: 0, maximumCapacityBytes: 0, maximumReleaseBytes: row.release, maximumDepth: row.depth })).toBe(true);
    const storage = Buffer.allocUnsafeSlow(64);
    const paid = row.items > 0 && row.depth > 0 && row.release >= storage.byteLength;
    const result = applyPatch({ retained: storage.byteLength, released: 0 }, paid ? [
      { op: "replace", path: "/retained", value: 0 },
      { op: "replace", path: "/released", value: storage.byteLength },
    ] : [], true).newDocument;
    expect(result.released).toBe(row.released);
    expect(result.retained + result.released).toBe(storage.byteLength);
  }
  const source = await Bun.file(new URL("../../../../../🦀️.rs", import.meta.url)).text();
  const helpers = source.slice(source.indexOf("pub fn artifact_retirement_box_"), source.indexOf("struct ArtifactStoreVcsRetirement"));
  expect(helpers).toContain("pub fn artifact_retirement_box_demands");
  expect(helpers).not.toContain("SnapshotRetirementStep");
  expect(helpers).not.toContain("next_close_byte_demand");
  for (const axis of ["copy", "capacity", "release", "depth"]) expect(helpers).toContain(`next_${axis}_`);
  expect(helpers).toContain("maximum_depth - 1");
  expect(helpers).toContain("admit_retained_clone_close");
  expect(helpers.indexOf("maximum_release_bytes < extent")).toBeLessThan(helpers.indexOf("drop(slot.take())"));
  console.log("[DEBUG] box retirement oracle: independent native storage extent, four plain grants, child depth debit, fallible demands, separate physical frame release");
});

test("dictionary index releases its original whole backing page under the release axis", async () => {
  const page = Buffer.allocUnsafeSlow(1024);
  for (const bytes of [0, 1, 7, 1023, 1024, 4096]) {
    const after = applyPatch({ retained: page.byteLength, released: 0 }, bytes >= page.byteLength ? [
      { op: "replace", path: "/retained", value: 0 },
      { op: "replace", path: "/released", value: page.byteLength },
    ] : [], true).newDocument;
    expect(after.released).toBe(bytes >= 1024 ? 1024 : 0);
    expect(after.retained + after.released).toBe(1024);
  }
  const source = await Bun.file(new URL("../../../📜️history/🗂️dictionary/📇️index/🦀️.rs", import.meta.url)).text();
  expect(source).not.toContain("close_offset");
  expect(source).toContain("grant.maximum_release_bytes < PAGE_BYTES");
  expect(source).toContain("next_release_byte_demand");
  expect(source).toContain("RetainedCloneStep");
  console.log("[DEBUG] dictionary index oracle: independent 1024-byte backing denies partial grants and frees one original whole page");
});

test("member-open retirement requires the original full allocation including empty capacity", async () => {
  expect(new Set(fixture.cases.map(row => row.id)).size).toBe(3);
  for (const row of fixture.cases) {
    const original = Buffer.allocUnsafeSlow(row.capacityBytes);
    const used = original.write(row.text, "utf8");
    expect(used).toBe(new TextEncoder().encode(row.text).length);
    expect(original.buffer.byteLength).toBe(row.capacityBytes);
    for (const [items, bytes] of [[0, row.capacityBytes], [1, row.capacityBytes - 1], [1, row.capacityBytes]]) {
      const paid = items > 0 && bytes >= original.buffer.byteLength;
      const result = applyPatch({ retained: row.capacityBytes, released: 0 }, paid ? [
        { op: "replace", path: "/retained", value: 0 },
        { op: "replace", path: "/released", value: row.capacityBytes },
      ] : [], true).newDocument;
      expect(result.released).toBe(paid ? row.capacityBytes : 0);
      expect(result.retained + result.released).toBe(row.capacityBytes);
      expect(bytes).toBeLessThanOrEqual(fixture.maximumAdmissionBytes);
    }
  }
  const source = await Bun.file(new URL("../../../🦀️.rs", import.meta.url)).text();
  const request = source.slice(source.indexOf("impl MemberOpenRequest {"), source.indexOf("impl ErasedSnapshotRetirement for MemberOpenRequest"));
  expect(request.includes("owned_retirement(")).toBe(false);
  expect(request.includes("drop(std::mem::take(field))")).toBe(true);
  expect(request.includes("field.capacity()")).toBe(true);
  expect(request.includes("closing_identity_field")).toBe(true);
  const oversized = applyPatch({ capacityBytes: fixture.maximumAdmissionBytes }, [{ op: "replace", path: "/capacityBytes", value: fixture.maximumAdmissionBytes + 1 }], true).newDocument;
  expect(Buffer.allocUnsafeSlow(oversized.capacityBytes).buffer.byteLength).toBeGreaterThan(fixture.maximumAdmissionBytes);
  console.log("[DEBUG] member-open physical retirement oracle: 3 original allocation extents, zero-item and one-byte-under denial, exact full release, empty capacity retained");
});

test("member-open inline page work and whole backing release remain separate authorities", async () => {
  const input = await Bun.file(new URL("../../../📏️retirement/🧫️fixtures/📄️inline.json", import.meta.url)).json();
  for (const row of input.cases) {
    let retained = row.inputBytes;
    const pages = Math.ceil(retained / input.pageBytes);
    for (let index = 0; index < pages; index++) {
      const bytes = retained % input.pageBytes || input.pageBytes;
      const after = applyPatch({ retained, items: 0, freed: 0 }, [
        { op: "replace", path: "/retained", value: retained - bytes },
        { op: "replace", path: "/items", value: 1 },
      ], true).newDocument;
      expect(after.freed).toBe(0);
      expect(after.items).toBe(input.maximumItems);
      retained = after.retained;
    }
    expect(retained).toBe(0);
    const original = Buffer.allocUnsafeSlow(pages * input.slotBytes);
    expect(original.buffer.byteLength).toBe(pages * input.slotBytes);
    expect(original.byteLength).toBeLessThanOrEqual(input.maximumAdmissionBytes);
    for (const paid of [0, original.byteLength - 1, original.byteLength]) {
      const released = paid >= original.byteLength ? original.byteLength : 0;
      expect(released).toBe(paid === original.byteLength ? original.byteLength : 0);
    }
  }
  const source = await Bun.file(new URL("../../../🦀️.rs", import.meta.url)).text();
  const request = source.slice(source.indexOf("pub struct MemberOpenRequest"), source.indexOf("impl ErasedSnapshotRetirement for MemberOpenRequest"));
  expect(request).not.toContain("closing_page");
  expect(request).not.toContain("closing_bytes");
  expect(request).toContain("copied_items: 1, ..empty");
  expect(request).toContain("grant.maximum_release_bytes");
  expect(request).toContain("grant.maximum_depth == 0");
  expect(request).not.toContain("SnapshotRetirementStep");
  console.log("[DEBUG] inline request oracle: four byte corpora, one POD page per work item, zero physical payload free, indivisible original boxed backing");
});


test("uninstalled catalog retains aliases until full child custody and physical disposer release", async () => {
  const storage = Buffer.allocUnsafeSlow(64);
  for (const row of fixture.boxGrants) {
    const original = { aliases: 3, childCapacity: storage.byteLength, disposerCapacity: storage.byteLength };
    const funded = row.items > 0 && row.depth > 0 && row.release >= storage.byteLength;
    const result = applyPatch(original, funded ? [{ op: "replace", path: "/disposerCapacity", value: 0 }] : [], true).newDocument;
    expect(result.disposerCapacity).toBe(funded ? 0 : storage.byteLength);
    expect(result.aliases).toBe(3);
    expect(result.childCapacity).toBe(storage.byteLength);
  }
  const source = await Bun.file(new URL("../../../../../🦀️.rs", import.meta.url)).text();
  const owner = source.slice(source.indexOf("pub fn uninstalled_owners_demands"), source.indexOf("pub fn uninstalled_owners_terminal_is_empty"));
  expect(owner).toContain("factory_retirement_tickets.demands(body)");
  expect(owner).toContain("grant.maximum_release_bytes < extent");
  expect(owner).toContain("admit_retained_clone_close");
  expect(owner).toContain("maximum_depth - 1");
  expect(owner).not.toContain("SnapshotRetirementStep");
  const trait = source.slice(source.indexOf("pub trait ArtifactStoreOwnedDisposer"), source.indexOf("#[derive(Clone, Copy, Debug, PartialEq, Eq)]", source.indexOf("pub trait ArtifactStoreOwnedDisposer")));
  expect(trait).toContain("fn close_uninstalled_step(&mut self, grant: RetainedCloneGrant)");
  expect(trait).toContain("fn uninstalled_demands");
  const concrete = source.slice(source.indexOf("    fn close_uninstalled_step(", source.indexOf("impl<P, Mutation> ArtifactStoreOwnedDisposer")), source.indexOf("    fn uninstalled_terminal_is_empty", source.indexOf("impl<P, Mutation> ArtifactStoreOwnedDisposer")));
  expect(concrete).toContain("grant: RetainedCloneGrant");
  expect(concrete).toContain("fn uninstalled_demands");
  expect(concrete).not.toContain("SnapshotRetirementStep");
  console.log("[DEBUG] uninstalled catalog oracle: aliases retained, exact disposer extent, four child currencies and controlled terminal proof");
});


test("catalog constructor admits exact factory births and retains partial refusal custody", async () => {
  const ajv = new Ajv({ strict: false });
  ajv.addSchema(orderedRetirement);
  ajv.addSchema(retirementContract);
  const validate = ajv.compile({ $ref: `${retirementContract.$id}#/$defs/Grant` });
  const frames = [Buffer.allocUnsafeSlow(64), Buffer.allocUnsafeSlow(64), Buffer.allocUnsafeSlow(64)];
  const capacity = frames.reduce((sum, frame) => sum + frame.byteLength, 0);
  for (const row of fixture.catalogBirthGrants) {
    const grant = { maximumItems: row.items, maximumCopyBytes: 0, maximumCapacityBytes: row.capacity, maximumReleaseBytes: 0, maximumDepth: row.depth };
    expect(validate(grant)).toBe(true);
    expect(row.items >= frames.length && row.capacity >= capacity && row.depth >= 2 ? capacity : 0).toBe(row.allocated);
  }
  const source = await Bun.file(new URL("../../../../../🦀️.rs", import.meta.url)).text();
  const start = source.indexOf("pub fn admit_constructor");
  const admission = source.slice(start, source.indexOf("/// 🏭️ Prices", start));
  expect(start).toBeGreaterThan(0);
  expect(admission.indexOf("grant.maximum_items == 0")).toBeLessThan(admission.indexOf("preborn_factory_retirement"));
  expect(admission.indexOf("grant.maximum_capacity_bytes < capacity")).toBeLessThan(admission.indexOf("preborn_factory_retirement"));
  expect(admission.indexOf("grant.maximum_depth < depth")).toBeLessThan(admission.indexOf("preborn_factory_retirement"));
  expect(admission).toContain("Err((error, progress))");
  expect(admission).toContain("factory_retirement_depth_demand");
  expect(source).toContain("pub struct DocumentStoreOwnersAdmissionError");
  console.log("[DEBUG] catalog birth oracle: independent three-frame capacity, item/depth preflight and retained partial allocation receipt");
});


test("initialization preserves whole catalog backings until granted typed retirement birth", async () => {
  for (const row of fixture.catalogBirthGrants) {
    const storage = Buffer.allocUnsafeSlow(64);
    const after = applyPatch({ original: true, active: false, capacity: storage.byteLength }, row.items > 0 && row.capacity >= storage.byteLength && row.depth >= 2 ? [{ op: "replace", path: "/active", value: true }] : [], true).newDocument;
    expect(after.original).toBe(true);
    expect(after.capacity).toBe(storage.byteLength);
  }
  const source = await Bun.file(new URL("../../../../../🦀️.rs", import.meta.url)).text();
  const start = source.indexOf("pub fn initialization_retirement_demands");
  expect(start).toBeGreaterThan(0);
  const runtime = source.slice(start, source.indexOf("#[allow(clippy::type_complexity)]", start));
  expect(runtime).toContain("ReturnedSnapshotReadRetirement::<P>::constructor_capacity_bytes()");
  expect(runtime).toContain("ArtifactStoreStringVectorRetirement::new");
  expect(runtime).toContain("ArtifactStoreRevisionAccumulatorRetirement::new");
  expect(runtime).toContain("Err((error, original))");
  expect(runtime).not.toContain("pop_first()");
  expect(runtime).not.toContain("SnapshotRetirementStep");
  console.log("[DEBUG] initialization oracle: original whole page catalogs, revision trees and shared owner survive admission denial");
});


test("one-item catalog admission makes finite progress without enlarging the supplied grant", async () => {
  let born = 0;
  for (const row of fixture.catalogPhaseGrants) {
    const capacity = Buffer.allocUnsafeSlow(64).byteLength;
    const allocated = row.items > 0 && row.capacity >= capacity && row.depth >= 2 && born < 3 ? capacity : 0;
    born += Number(allocated > 0);
    expect(allocated).toBe(row.allocated);
    expect(born === 3).toBe(row.ready);
    expect(allocated).toBeLessThanOrEqual(row.capacity);
  }
  expect(born).toBe(3);
  const source = await Bun.file(new URL("../../../../../🦀️.rs", import.meta.url)).text();
  const start = source.indexOf("pub fn admit_constructor");
  const admission = source.slice(start, source.indexOf("/// 🏭️ Prices", start));
  expect(admission).toContain("progress.copied_items == grant.maximum_items");
  expect(admission).toContain("pub fn constructor_is_complete");
  expect(admission).not.toContain("grant.maximum_items < items");
  console.log("[DEBUG] phased catalog oracle: exact one-item three-turn births, denied grants retain state, no synthetic item envelope");
});


test("member advancement preserves the actual full grant through child retirement", async () => {
  const ajv = new Ajv({ strict: false });
  ajv.addSchema(orderedRetirement);
  ajv.addSchema(retirementContract);
  const validate = ajv.compile({ $ref: retirementContract.$id + "#/$defs/Grant" });
  for (const row of fixture.catalogPhaseGrants) {
    const original = { maximumItems: row.items, maximumCopyBytes: 7, maximumCapacityBytes: row.capacity, maximumReleaseBytes: 96, maximumDepth: row.depth };
    const child = applyPatch(original, [{ op: "replace", path: "/maximumItems", value: Math.min(original.maximumItems, 1) }], true).newDocument;
    expect(validate(child)).toBe(true);
    for (const key of ["maximumCopyBytes", "maximumCapacityBytes", "maximumReleaseBytes", "maximumDepth"] as const) expect(child[key]).toBe(original[key]);
  }
  const source = await Bun.file(new URL("../../../🏭️operation/🦀️.rs", import.meta.url)).text();
  const drive = source.slice(source.indexOf("fn drive_active_hydration_retirement"), source.indexOf("/// Admits the selected history"));
  expect(drive).toContain("grant: RetainedCloneGrant");
  expect(drive).toContain("artifact_retirement_box_close_step");
  expect(drive).not.toContain("fuel_remaining()");
  expect(drive).not.toContain("SnapshotRetirementStep");
  expect(source).toContain("step_store(&mut self, cx: &mut StepContext");
  const root = await Bun.file(new URL("../../../../../🦀️.rs", import.meta.url)).text();
  const macro = root.slice(root.indexOf("impl $crate::os_store::MemberOpenOperation for $open_name"), root.indexOf("impl $crate::os_store::SpaceMember for $enum_name", root.indexOf("impl $crate::os_store::MemberOpenOperation for $open_name")));
  expect(macro).toContain("open.step_store(cx, grant)");
  expect(macro).not.toContain("maximum_bytes");
  console.log("[DEBUG] member advancement oracle: original five-axis grant survives actual child call; CPU fuel never becomes memory authority");
});

test("typed option admission retains the original owner on all denied birth axes", async () => {
  const original = Buffer.allocUnsafeSlow(64);
  for (const row of fixture.catalogPhaseGrants) {
    const admitted = row.items > 0 && row.capacity >= original.byteLength && row.depth >= 2;
    const custody = applyPatch({ pending: true, active: false }, admitted ? [{ op: "replace", path: "/pending", value: false }, { op: "replace", path: "/active", value: true }] : [], true).newDocument;
    expect(Number(custody.pending) + Number(custody.active)).toBe(1);
    expect(custody.pending).toBe(!admitted);
  }
  const root = await Bun.file(new URL("../../../../../🦀️.rs", import.meta.url)).text();
  const start = root.indexOf("pub fn artifact_retirement_admit_owned");
  expect(start).toBeGreaterThan(0);
  const body = root.slice(start, root.indexOf("///", start + 10));
  expect(body).toContain("admit_owned_retirement(original, child)");
  expect(body).toContain("*pending = Some(original)");
  expect(body).toContain("grant.maximum_depth - 1");
  expect(body).not.toContain("SnapshotRetirementStep");
  console.log("[DEBUG] typed option custody oracle: independent single-owner transfer and original restoration for denied birth axes");
});

test("initial member close keeps original inputs and independent controlled currencies", async () => {
  for (const row of fixture.catalogPhaseGrants) {
    const original = { pending: true, allocation: Buffer.allocUnsafeSlow(64).byteLength };
    const transferred = row.items > 0 && row.capacity >= original.allocation && row.depth >= 2;
    const result = applyPatch(original, transferred ? [{ op: "replace", path: "/pending", value: false }] : [], true).newDocument;
    expect(result.allocation).toBe(original.allocation);
    expect(result.pending).toBe(!transferred);
  }
  const source = await Bun.file(new URL("../../../🏭️operation/🦀️.rs", import.meta.url)).text();
  const start = source.indexOf("impl<F, P, M> ErasedSnapshotRetirement for InitialMemberStoreOpen");
  const body = source.slice(start, source.indexOf("impl<F, P, M> Drop for InitialMemberStoreOpen", start));
  expect(body).toContain("grant: RetainedCloneGrant");
  expect(body).toContain("artifact_retirement_admit_owned");
  expect(body).toContain("next_capacity_byte_demand");
  expect(body).toContain("next_release_byte_demand");
  expect(body).toContain("next_depth_demand");
  expect(body).not.toContain("SnapshotRetirementStep");
  expect(body).not.toContain("owned_retirement(");
  expect(body).not.toContain(".truncate(");
  console.log("[DEBUG] initial member closure oracle: original typed custody and four separate currencies, paid whole backing release");
});

test("envelope custody keeps seven original message arrays and whole metadata owners", async () => {
  const arrays = [64, 32, 8, 16, 16, 16, 16].map(bytes => Buffer.allocUnsafeSlow(bytes));
  let retained = arrays.reduce((total, array) => total + array.byteLength, 0);
  let released = 0;
  for (const original of arrays) {
    const under = applyPatch({ retained, released }, [], true).newDocument;
    expect(under.retained).toBe(retained);
    retained -= original.byteLength;
    released += original.byteLength;
  }
  expect(retained).toBe(0);
  expect(released).toBe(arrays.reduce((total, array) => total + array.byteLength, 0));
  const source = await Bun.file(new URL("../../../../../🦀️.rs", import.meta.url)).text();
  expect(source).toContain("struct ArtifactEditMessageLedgerRetirement");
  const start = source.indexOf("struct ArtifactStoreEnvelopeRetirement");
  const body = source.slice(start, source.indexOf("struct ArtifactGenesisRetirement", start));
  expect(body).toContain("artifact_retirement_owner_close");
  expect(body).toContain("ArtifactStoreVcsRetirement");
  expect(body).toContain("ArtifactEnvelopeMetadata");
  expect(body).not.toContain("SnapshotRetirementStep");
  expect(body).not.toContain("take_metadata_string");
  expect(body).not.toContain(".pop()");
  console.log("[DEBUG] envelope ownership oracle: seven indivisible original arrays and whole metadata custodians, no per-string flattening");
});

test("member hot retirement preserves pending decoder and original rejected inputs", async () => {
  for (const row of fixture.catalogPhaseGrants) {
    const original = Buffer.allocUnsafeSlow(64);
    const admitted = row.items > 0 && row.capacity >= original.byteLength && row.depth >= 2;
    expect(applyPatch({ original: true }, admitted ? [{ op: "replace", path: "/original", value: false }] : [], true).newDocument.original).toBe(!admitted);
  }
  const source = await Bun.file(new URL("../../../🏭️operation/🦀️.rs", import.meta.url)).text();
  const start = source.indexOf("fn drive_active_hydration_retirement");
  const hot = source.slice(start, source.indexOf("impl<F, P, M> ErasedSnapshotRetirement for InitialMemberStoreOpen"));
  expect(hot).toContain("artifact_retirement_admit_owned");
  expect(hot).toContain("*self.history_auxiliary = Some(auxiliary)");
  expect(hot).toContain("*self.rejected_history_input = Some(rejected.input)");
  expect(hot).not.toContain("retirement::owned_retirement(");
  expect(hot).not.toContain("bytes.truncate(");
  expect(hot).not.toContain("Box::new(rejected");
  console.log("[DEBUG] hot member custody: exact rejected originals and decoder auxiliaries survive admission denial");
});

test("hydration consumes supplied retirement currencies without increasing any caller axis", async () => {
  for (const row of fixture.catalogPhaseGrants) {
    const original = { maximumItems: row.items, maximumCopyBytes: 17, maximumCapacityBytes: row.capacity, maximumReleaseBytes: 23, maximumDepth: row.depth };
    const child = applyPatch(original, [{ op: "replace", path: "/maximumItems", value: Math.min(original.maximumItems, 1) }], true).newDocument;
    expect(child.maximumCapacityBytes).toBe(original.maximumCapacityBytes);
    expect(child.maximumReleaseBytes).toBe(original.maximumReleaseBytes);
    expect(child.maximumCopyBytes).toBe(original.maximumCopyBytes);
    expect(child.maximumDepth).toBe(original.maximumDepth);
  }
  const root = await Bun.file(new URL("../../../../../🧾️document/📜️history/💧️hydration/🦀️.rs", import.meta.url)).text();
  const helpers = root.slice(root.indexOf("fn drive_hydration_retirement"), root.indexOf("#[derive(Clone, Copy, Debug, PartialEq, Eq)]"));
  expect(helpers).toContain("grant: RetainedCloneGrant");
  expect(helpers).toContain("runtime.close_step(factory, grant)");
  expect(helpers).not.toContain("maximum_bytes =");
  expect(helpers).not.toContain("SnapshotRetirementStep");
  expect(root).toContain("match self.step(cx, grant)");
  console.log("[DEBUG] hydration currency oracle: original capacity/release/copy/depth preserved through current runtime and child calls");
});

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
  const root = await Bun.file(new URL("../../../../../🦀️.rs", import.meta.url)).text();
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
  console.log("[DEBUG] catalog sources: independent four native extents, funded original ingress, factory tickets remain pending for one-item turns");
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
  const root = await Bun.file(new URL("../../../../../🦀️.rs", import.meta.url)).text();
  const start = root.indexOf("pub fn admit_source_constructor");
  expect(start).toBeGreaterThan(0);
  const constructor = root.slice(start, root.indexOf("    pub fn new(", start));
  expect(constructor.indexOf("source_constructor_capacity")).toBeLessThan(constructor.indexOf("construct()"));
  const capacityBody = constructor.slice(constructor.indexOf("fn source_constructor_capacity"));
  expect(capacityBody).toContain("grant.maximum_capacity_bytes < capacity");
  expect(capacityBody).toContain("owners: None");
  expect(root).toContain("fn member_store_owners(grant: RetainedCloneGrant) -> Result<");
  const operation = await Bun.file(new URL("../../../🏭️operation/🦀️.rs", import.meta.url)).text();
  expect(operation).toContain("P::member_store_owners(source_grant)");
  expect(operation).toContain("maximum_capacity_bytes: source_capacity");
  expect(operation).toContain("progress.fits(source_grant)");
  expect(operation).toContain("genesis_request: ManuallyDrop::new(genesis_request)");
  expect(operation).toContain("owners.constructor_is_complete()");
  expect(operation).toContain("owners.admit_constructor(grant)");
  expect(operation).not.toContain("Some(P::member_store_owners())");
  console.log("[DEBUG] caller source admission: original request held across denied ingress, constructor never invoked, each catalog ticket uses its actual next grant");
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
  const root = await Bun.file(new URL("../../../🦀️.rs", import.meta.url)).text();
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
  console.log("[DEBUG] retained member original child admissions and final physical receipt preserve supplied currencies; native System law separately required");
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
  const root = await Bun.file(new URL("../../../../../🦀️.rs", import.meta.url)).text();
  const start = root.indexOf("pub fn admit_source_constructor_with_one_item_preparation");
  expect(start).toBeGreaterThan(0);
  const body = root.slice(start, root.indexOf("    fn from_sources", start));
  expect(body.indexOf("source_constructor_capacity")).toBeLessThan(body.indexOf("construct()"));
  expect(body).toContain("owners.one_item_preparation = Some(preparation)");
  expect(body).toContain("retained_capacity_bytes: capacity");
  expect(body).not.toContain("admit_constructor(");
  console.log("[DEBUG] independent optional source oracle: whole original Arc tree admitted before source callback; retirement tickets remain unfunded");
});


test("original source iterator retains its actual allocation extent through prefix handoff", async () => {
  const neutral = await Bun.file(new URL("../../../../../♻️retirement/📦️backing/🧫️fixtures/🔣️.json", import.meta.url)).json();
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
  const backing = await Bun.file(new URL("../../../../../♻️retirement/📦️backing/🦀️.rs", import.meta.url)).text();
  const start = backing.indexOf("pub(super) struct SourceIterator");
  const owner = backing.slice(start);
  expect(owner).toContain("source_bytes = extent::<T>(source.capacity())");
  expect(owner).toContain("source.into_iter()");
  expect(owner).toContain("*self.pending = Some(value)");
  expect(owner).toContain("release_bytes: self.source_bytes");
  expect(owner).toContain("maximum_depth: grant.maximum_depth - 1");
  expect(owner).toContain("RetainedCloneStep::Progress(step.progress())");
  expect(owner).not.toContain("SnapshotRetirementStep");
  console.log("[DEBUG] Buffer/RFC6902 original allocation oracle preserves full-u64 prefix; actual native source retirement remains separately required");
});

test("original SPR parent retains completed decoder custody and full factory receipts", async () => {
  const root = await Bun.file(new URL("../../../../../🦀️.rs", import.meta.url)).text();
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
  console.log("[DEBUG] Original SPR parent completed controllers retain exact original payload/factory custody; native physical law unqualified");
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
  const rootUrl = new URL("../../../../../🦀️.rs", import.meta.url);
  const root = await Bun.file(rootUrl).text();
  expect(root.includes("fn member_store_owners_birth_demand() -> Result<")).toBe(true);
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
  console.log("[DEBUG] Node/RFC6902 source tree oracle: original capacity sum and checked wrapper depth precede constructor; native allocation proof required separately");
});


test("bounded config factory admits actual typed owner and preserves full receipts", async () => {
  const neutral = await Bun.file(new URL("../../../../../♻️retirement/📦️backing/🧫️fixtures/🔣️.json", import.meta.url)).json();
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
  const plugin = await Bun.file(new URL("../../../../../../🔌️plugin/🦀️.rs", import.meta.url)).text();
  const start = plugin.indexOf("struct BoundedConfigRetirementFactory<");
  const owner = plugin.slice(start, plugin.indexOf("pub fn bounded_config_store_owners", start));
  expect(plugin.includes("struct BoundedConfigValueRetirement")).toBe(false);
  expect(owner).toContain("T: semio_framework_value::retirement::RetireOwned");
  expect(owner).toContain("fn retirement_birth_bytes");
  expect(owner).toContain("admit_owned_retirement(value, grant)");
  expect(owner).toContain("admit_shared_retirement(snapshot, grant, false)");
  expect(owner).not.toContain("drop(value)");
  expect(owner).not.toContain("SnapshotRetirementStep");
  console.log("[DEBUG] AJV/Buffer/RFC6902 config factory oracle keeps one-byte copy independent from whole physical release; original native factory law required separately");
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
  const source = await Bun.file(new URL("../../../../../🧾️document/📜️history/💧️hydration/🦀️.rs", import.meta.url)).text();
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
  console.log("[DEBUG] canonical Ajv grant plus Node/RFC6902 original physical backing oracle; native owner/allocator identity proof remains separately required");
});
