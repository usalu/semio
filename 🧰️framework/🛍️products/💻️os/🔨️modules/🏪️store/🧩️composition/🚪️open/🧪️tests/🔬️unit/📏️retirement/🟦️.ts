import { expect, test } from "bun:test";
import { Buffer } from "node:buffer";
import { spawnSync } from "node:child_process";
import { fileURLToPath } from "node:url";
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
  const native=await Bun.file(new URL("../../../../../🧪️tests/🔬️unit/🦀️.rs",import.meta.url)).text();
  const law=native.slice(native.indexOf("fn current_box_retirement_preserves_denied_owner"),native.indexOf("fn artifact_store_group_visibility",native.indexOf("fn current_box_retirement_preserves_denied_owner")));
  expect(law.indexOf("let policy = physical_test_close_grant();")).toBeGreaterThanOrEqual(0);
  expect(law.indexOf("let policy = physical_test_close_grant();")).toBeLessThan(law.indexOf("observe_backing"));
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
  expect(request).toContain("pages.close_backing_step(grant)?");
  expect(request).toContain("return Ok(RetainedCloneStep::Progress(step.progress()));");
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
  console.log("[DEBUG] canonical Ajv grant plus Node/RFC6902 original physical backing oracle; ready output retains auxiliary capacity until paid close; native owner/allocator identity proof remains separately required");
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
  const source = await Bun.file(new URL("../../../../../🧾️document/📜️history/💧️hydration/🦀️.rs", import.meta.url)).text();
  const start = source.indexOf("Phase::RetirePack =>");
  const body = source.slice(start, source.indexOf("Phase::CloseEnvelopeRuntime =>", start));
  expect(body.includes(".truncate(")).toBe(false);
  expect(body.includes("self.pack.take()")).toBe(false);
  expect(body.includes("artifact_retirement_admit_owned")).toBe(true);
  expect(body.includes("grant.maximum_items.min(1)")).toBe(true);
  console.log("[DEBUG] Buffer/RFC6902 logical shortening retains exact physical allocation; normal Pack owner is admitted before paid retirement");
});

test("hydration fold consumes supplied independent physical authority", async () => {
  for (const grant of [{ items: 0, copy: 1, capacity: 0, release: 0, depth: 2 }, { items: 1, copy: 1, capacity: 7, release: 19, depth: 2 }, { items: 1, copy: 0, capacity: 19, release: 7, depth: 3 }]) {
    const child = applyPatch(structuredClone(grant), [{ op: "replace", path: "/items", value: Math.min(1, grant.items) }, { op: "replace", path: "/depth", value: grant.depth - 1 }], true).newDocument;
    expect(child.capacity).toBe(grant.capacity);
    expect(child.release).toBe(grant.release);
    expect(child.copy).toBe(grant.copy);
    expect(child.depth).toBe(grant.depth - 1);
  }
  const source = await Bun.file(new URL("../../../../../🧾️document/📜️history/💧️hydration/🦀️.rs", import.meta.url)).text();
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
  console.log("[DEBUG] RFC6902 independent authority oracle preserves capacity and release; a producer quote cannot increase caller authority");
});

test("ready config auxiliary owner needs capacity and no transferred catalog", async () => {
  const backing = Buffer.allocUnsafeSlow(37);
  expect(backing.subarray(0, 0).byteLength).toBe(0);
  expect(backing.subarray(0, 0).buffer.byteLength).toBe(37);
  const state = applyPatch({ outputReady: true, catalogRetained: false, auxiliaryCapacity: backing.byteLength, terminal: false }, [], true).newDocument;
  expect(state.terminal).toBe(false);
  expect(state.auxiliaryCapacity).toBe(backing.buffer.byteLength);
  const source = await Bun.file(new URL("../../../../../🎚️config/📥️retained/🦀️.rs", import.meta.url)).text();
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
  console.log("[DEBUG] Buffer/RFC6902 config Ready keeps empty logical String capacity; transferred catalog is required only for its retained children");
});


test("completed original publication retains factory and physical frame custody", async () => {
  const neutral = await Bun.file(new URL("../../../../../♻️retirement/📦️backing/🧫️fixtures/🔣️.json", import.meta.url)).json();
  const law = neutral.completedRecordCustody;
  for (const wordBytes of law.wordBytes) {
  const original = Buffer.allocUnsafeSlow(law.factorySources * law.pointerWordsPerCapability * wordBytes);
  const published = applyPatch({ envelope: true, factories: law.factorySources, factoryBytes: original.byteLength, sourceReleased: 0, closing: false }, [{ op: "replace", path: "/envelope", value: false }, { op: "replace", path: "/closing", value: true }], true).newDocument;
  expect(published.sourceReleased).toBe(law.publicationSourceReleaseBytes);
  expect(published.factories).toBe(law.factorySources);
  expect(published.factoryBytes).toBe(original.byteLength);
  expect(published.closing).toBe(true);
  }
  const root = await Bun.file(new URL("../../../../../🦀️.rs", import.meta.url)).text();
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
  console.log("[DEBUG] Buffer/RFC6902 original publication oracle retains factory allocations and routes same source owner to original close pump; native System law remains required");
});

test("hydration current authored Rust syntax parses against original source", async () => {
  const physical = await Bun.file(new URL("../../../../../🧾️document/📜️history/💧️hydration/🧪️tests/🧫️fixtures/📏️physical-demand/🔣️.json", import.meta.url)).json();
  const oracle = new Ajv({ strict: false });
  oracle.addSchema(orderedRetirement);
  oracle.addSchema(retirementContract);
  expect(oracle.compile({ $ref: retirementContract.$id + "#/$defs/Grant" })(physical.readyAuxiliaryPolicy)).toBe(true);
  for (const relative of ["../../../../../../🔌️plugin/🧪️tests/🧩️composition/🦀️.rs", "../../../../../🧾️document/📜️history/💧️hydration/🦀️.rs", "../../../../../🧾️document/📜️history/💧️hydration/🧪️tests/🔬️unit/🦀️.rs", "../../../../../🎚️config/📥️retained/🦀️.rs", "../../../../../../../../../🔨️modules/📡️replication/🔗️causal/🔀️transition/🔁️fold/🦀️.rs", "../../../../../../../../../🔨️modules/📡️replication/🔗️causal/🔀️transition/🧪️tests/🔬️unit/🦀️.rs", "../../../../../..//📡️spr/📜️history/🧪️tests/🔬️unit/🦀️.rs", "../../../../../../../../../🔨️modules/📡️replication/🧾️wire/♻️retirement/🦀️.rs", "../../../../../../../../../🔨️modules/📡️replication/🧾️wire/♻️retirement/🧪️tests/🦀️.rs", "../../../../../🧩️composition/🚪️open/🏭️operation/🦀️.rs", "../../../../../🧩️composition/🚪️open/🧪️tests/🔬️unit/🦀️.rs", "../../../../../🏗️initialization/📚️catalog/🦀️.rs", "../../../../../🏗️initialization/📚️catalog/🧪️tests/🔬️unit/🦀️.rs", "../../../../../../🌿️vcs/🚪️io/💾️binary/🌱️genesis/🦀️.rs", "../../../../../../🌿️vcs/🦀️.rs"]) {
    const path = fileURLToPath(new URL(relative, import.meta.url));
    expect([...path].length).toBeLessThanOrEqual(256);
    const result = spawnSync("rustfmt", ["--edition", "2024", "--config", "skip_children=true", "--emit", "stdout", path], { encoding: "utf8", timeout: 2000 });
    expect(result.error).toBeUndefined();
    if (result.status !== 0) console.log("[DEBUG] Rustfmt original-source parse refusal " + result.stderr);
    expect(result.status).toBe(0);
  }
  console.log("[DEBUG] genuine grant Ajv and installed Rustfmt parse original sources; grammar is not native type or allocator runtime proof");
});

test("hydration target decoder preserves original supplied physical authority", async () => {
  const source = await Bun.file(new URL("../../../../../🧾️document/📜️history/💧️hydration/🦀️.rs", import.meta.url)).text();
  const start = source.indexOf("Phase::DecodeTarget =>");
  const body = source.slice(start, source.indexOf("Phase::TargetInputs =>", start));
  expect(body.includes("next_step_grant(")).toBe(false);
  expect(body.includes("hydration_child_grant(grant, cx.fuel_remaining())")).toBe(true);
  expect(body.includes("progress.fits(decode_grant)")).toBe(true);
  expect(body.includes("progress_fuel(progress)")).toBe(true);
  console.log("[DEBUG] target decode receiver reuses current independent authority rather than requested frontier authority");
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
  const source = await Bun.file(new URL("../../../../../../../../../🔨️modules/📡️replication/🔗️causal/🔀️transition/🔁️fold/🦀️.rs", import.meta.url)).text();
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
  console.log("[DEBUG] canonical genuine Progress/Ajv and Buffer/RFC6902 preserve terminal physical cleanup for both outcomes");
});

test("original document Store disposer forwards full grants and terminal receipt", async () => {
  const neutral = await Bun.file(new URL("../../../../../♻️retirement/📦️backing/🧫️fixtures/🔣️.json", import.meta.url)).json();
  const law = neutral.storeDisposer;
  const oracle = new Ajv({ strict: false });
  oracle.addSchema(orderedRetirement);
  oracle.addSchema(retirementContract);
  expect(oracle.compile({ $ref: retirementContract.$id + "#/$defs/Grant" })({ maximumItems: 1, maximumCopyBytes: law.copyBytes, maximumCapacityBytes: law.capacityBytes, maximumReleaseBytes: law.releaseBytes, maximumDepth: law.depth })).toBe(true);
  const backing = Buffer.allocUnsafeSlow(law.releaseBytes);
  const result = applyPatch({ retained: backing.byteLength, released: 0 }, [{ op: "replace", path: "/retained", value: 0 }, { op: "replace", path: "/released", value: backing.byteLength }], true).newDocument;
  expect(result.released).toBe(law.terminalReceipt.releasedBytes);
  expect(law.releaseBytes).toBeGreaterThan(law.copyBytes);
  const root = await Bun.file(new URL("../../../../../../🔌️plugin/🦀️.rs", import.meta.url)).text();
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
  console.log("[DEBUG] original Store disposer independent Node backing release and RFC6902 terminal receipt conserve all currencies");
});

test("hydration fold outputs retain original custody before receipt refusal", async () => {
  for (const accepted of [false, true]) {
    const original = Buffer.allocUnsafeSlow(37);
    const state = applyPatch({ original: original.byteLength, retained: 0, accepted }, [{ op: "replace", path: "/retained", value: original.buffer.byteLength }], true).newDocument;
    expect(state.retained).toBe(37);
    expect(state.original).toBe(37);
  }
  const h = await Bun.file(new URL("../../../../../🧾️document/📜️history/💧️hydration/🦀️.rs", import.meta.url)).text();
  const ready = h.slice(h.indexOf("HistoryFoldJobStep::Ready { value: (fold"), h.indexOf("Phase::BindGenesis =>", h.indexOf("Phase::Fold =>")));
  expect(ready.indexOf("*self.fold = Some(fold)")).toBeLessThan(ready.indexOf("if !progress.fits(fold_grant)"));
  expect(h).toContain("pending_target: ManuallyDrop<Option<crate::os_spr::HistoryTransition>>");
  const target = h.slice(h.indexOf("Phase::DecodeTarget =>"), h.indexOf("Phase::SeedApplied |"));
  expect(target.indexOf("*self.pending_target = Some(target)")).toBeLessThan(target.indexOf("if !progress.fits(decode_grant)"));
  expect(target).toContain("artifact_retirement_admit_owned(&mut self.pending_target");
  expect(target).not.toContain("let target = self.pending_target.take()");
  const c = await Bun.file(new URL("../../../../../🎚️config/📥️retained/🦀️.rs", import.meta.url)).text();
  const config = c.slice(c.indexOf("HistoryFoldJobStep::Ready { value: (fold"), c.indexOf("Phase::BindGenesis =>", c.indexOf("Phase::Fold =>")));
  expect(config.indexOf("*self.fold_auxiliary = Some((replay_order, conflicts))")).toBeLessThan(config.indexOf("if !progress.fits(grant)"));
  expect(c).toContain("controlled!(fold_auxiliary)");
  expect(c).toContain("&& self.fold_auxiliary.is_none()");
  console.log("[DEBUG] Buffer/RFC6902 original receiver custody precedes receipt refusal; retained config auxiliary capacities join funded cleanup");
});

test("member input buffer birth preserves explicit capacity authority before allocation", async () => {
  const oracle = new Ajv({ strict: false }); oracle.addSchema(orderedRetirement); oracle.addSchema(retirementContract);
  const valid = oracle.compile({ $ref: retirementContract.$id + "#/$defs/Grant" });
  const plain = await Bun.file(new URL("../../../🏭️operation/📏️birth/🧫️fixtures/🔣️.json", import.meta.url)).json();
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
  const source = await Bun.file(new URL("../../../../../🧩️composition/🚪️open/🏭️operation/🦀️.rs", import.meta.url)).text();
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
  console.log("[DEBUG] canonical Grant/Ajv and Buffer/RFC6902 original birth oracle refuses item, capacity and depth independently before native allocation");
});

test("original evaluation resource scopes preserve complete physical grants", async () => {
  const neutral = await Bun.file(new URL("../../../../../♻️retirement/📦️backing/🧫️fixtures/🔣️.json", import.meta.url)).json();
  const law = neutral.evaluationResources;
  const backing = Buffer.allocUnsafeSlow(law.releaseBytes);
  const state = applyPatch({ source: backing.byteLength, admitted: law.capacityBytes, physical: 0 }, [{ op: "replace", path: "/physical", value: backing.byteLength + law.capacityBytes }], true).newDocument;
  expect(state.physical).toBe(state.source + state.admitted);
  expect(law.copyBytes).toBeLessThan(law.releaseBytes);
  const sdk = await Bun.file(new URL("../../../../../../🌊️flow/🧩️extensions/🕸️wasm/🦀️.rs", import.meta.url)).text();
  const input = sdk.slice(sdk.indexOf("pub struct EvaluationInputPreparation"), sdk.indexOf("const EVALUATION_JOB_CAPACITY"));
  expect(input).toContain("retirement_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>");
  expect(input).toContain("evaluation_admit_original(&mut self.parser,&mut self.active,grant)");
  expect(input).toContain("evaluation_child_close(&mut self.active,grant)");
  expect(input).toContain("self.encoding.is_none()&&self.decoding.is_none()&&self.active.is_none()");
  expect(input).toContain("RetirementStep::Progress(progress)");
  const start = sdk.indexOf("pub struct ExtensionEvaluationResources");
  const owner = sdk.slice(start, sdk.indexOf("// #endregion ⏱️EvaluationJobs", start));
  expect(owner).toContain("fn retirement_demands");
  expect(owner).toContain("grant:RetainedCloneGrant");
  expect(owner).toContain("RegistryLeaseRetirement");
  expect(owner).not.toContain("next_close_byte_demand");
  expect(owner).not.toContain("PluginCloseStep");
  const cancelled = sdk.slice(sdk.indexOf("pub fn retire_cancelled_evaluations_close_step"), sdk.indexOf("pub fn evaluation_retirement_pending"));
  expect(cancelled).not.toContain("key.clone()");
  expect(cancelled).toContain("evaluation_registry_demands(&registry,identity,grant.maximum_copy_bytes)");
  const demands = sdk.slice(sdk.indexOf("fn evaluation_controlled_demands"), sdk.indexOf("pub fn retire_cancelled_evaluations_close_step"));
  for (const name of ["next_copy_byte_demand", "next_capacity_byte_demand", "next_release_byte_demand", "next_depth_demand", "retained.retirement_demands(copy,false,false)"]) expect(demands).toContain(name);
  expect(cancelled).toContain("extract_slot_if");
  expect(cancelled).toContain("registry.retiring_key=Some");
  expect(cancelled).toContain("registry.retiring_backing=Some");
  expect(cancelled).toContain("grant:RetainedCloneGrant");
  expect(sdk).not.toContain("RetirementStep::ProcessedBytes");
  console.log("[DEBUG] original SDK resource law preserves independent actual source+cursor backing, same scope and complete caller authority");
});

test("hydration target binding retains original empty physical address backing", async () => {
  const original = Buffer.allocUnsafeSlow(37); const empty = original.subarray(0, 0);
  const replacement = Buffer.allocUnsafeSlow(19);
  const bound = applyPatch({ retainedOriginal: empty.buffer.byteLength, current: 0, released: 0 }, [{ op: "replace", path: "/current", value: replacement.buffer.byteLength }], true).newDocument;
  expect(bound.retainedOriginal).toBe(37); expect(bound.current).toBe(19); expect(bound.released).toBe(0);
  const h = await Bun.file(new URL("../../../../../🧾️document/📜️history/💧️hydration/🦀️.rs", import.meta.url)).text();
  const begin = h.slice(h.indexOf("Phase::BeginTargets =>"), h.indexOf("Phase::DecodeTarget =>"));
  expect(begin).toContain("artifact_retirement_admit_owned(&mut self.target_address");
  const target = h.slice(h.indexOf("Phase::TargetInputs =>"), h.indexOf("Phase::SeedApplied |"));
  expect(target).toContain("std::mem::replace(&mut source[self.record_index].target");
  expect(target).toContain("*self.target_address = Some(original)");
  console.log("[DEBUG] Node original empty view capacity and RFC6902 binding retain original target allocation until paid canonical owner cleanup");
});

test("initialization catalog birth admits one exact original lane under supplied currencies", async () => {
  const oracle = new Ajv({ strict: false });
  oracle.addSchema(orderedRetirement);
  oracle.addSchema(retirementContract);
  const grants = oracle.compile({ $ref: `${retirementContract.$id}#/$defs/Grant` });
  const plain = await Bun.file(new URL("../../../../../🏗️initialization/📚️catalog/🧫️fixtures/🔣️.json", import.meta.url)).json();
  const policy = plain.birthPolicy;
  expect(grants(policy)).toBe(true);
  expect(plain.birthCancelCuts).toEqual(Array.from({ length: plain.lanes.length + 1 }, (_, index) => index));
  const pages = plain.birthExampleExtents.map((bytes: number) => Buffer.allocUnsafeSlow(bytes));
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
  const source = await Bun.file(new URL("../../../../../🏗️initialization/📚️catalog/🦀️.rs", import.meta.url)).text();
  expect(source).toContain("pub fn empty()");
  expect(source).toContain("pub fn admission_demands(");
  expect(source).toContain("pub fn admission_is_complete(");
  expect(source).toContain("pub fn admit_next(");
  expect(source.indexOf("grant.maximum_capacity_bytes < demand.capacity_bytes")).toBeLessThan(source.indexOf("HistoryPageStack::try_new()"));
  expect(source).toContain("retained_capacity_bytes: demand.capacity_bytes");
  expect(source).toContain("next_push_allocation_bytes()");
  const hydration = await Bun.file(new URL("../../../../../🧾️document/📜️history/💧️hydration/🦀️.rs", import.meta.url)).text();
  expect(hydration).toContain("Phase::AdmitInitializationCatalog =>");
  expect(hydration).toContain("ArtifactStoreInitializationRuntime::new_with_owner_catalog");
  expect(hydration).not.toContain("ArtifactStoreInitializationRuntime::new(");
  expect(hydration).toContain("initialization_catalog");
  console.log("[DEBUG] initialization admission: independent Node native storage + RFC6902 original state; one admitted lane, zero/partial grants retain, partial catalog remains owned");
});

test("genesis admission preserves original payload and envelope metadata under supplied grant", async () => {
  const oracle = new Ajv({ strict: false }); oracle.addSchema(orderedRetirement); oracle.addSchema(retirementContract);
  const valid = oracle.compile({ $ref: retirementContract.$id + "#/$defs/Grant" });
  const plain = await Bun.file(new URL("../../../../../🧾️document/📜️history/💧️hydration/🧪️tests/🧫️fixtures/📏️physical-demand/🔣️.json", import.meta.url)).json();
  const policy = plain.verifiedGenesis.policy;
  expect(valid(policy)).toBe(true);
  const original = Buffer.allocUnsafeSlow(plain.verifiedGenesis.packCapacityBytes); original.set(Buffer.from(plain.verifiedGenesis.text, "utf8"));
  const alias = original.subarray(0, 2);
  const result = applyPatch({ bytes: [...alias], retained: original.buffer.byteLength, admitted: 0 }, [{ op: "replace", path: "/admitted", value: 1 }], true).newDocument;
  expect(result.bytes).toEqual([...Buffer.from("Ω", "utf8")]); expect(result.retained).toBe(37); expect(alias.buffer).toBe(original.buffer);
  const genesis = await Bun.file(new URL("../../../../../../🌿️vcs/🚪️io/💾️binary/🌱️genesis/🦀️.rs", import.meta.url)).text();
  expect(genesis).toContain("verified_pack_birth_demand()");
  expect(genesis).toContain("admit_verified_pack(");
  expect(genesis).toContain("demand.admit(grant)");
  expect(genesis).toContain("Err((error, snapshot, pack))");
  expect(genesis.indexOf("demand.admit(grant)")).toBeLessThan(genesis.indexOf("Self::from_verified_pack(snapshot, pack, digest)"));
  const root = await Bun.file(new URL("../../../../../🦀️.rs", import.meta.url)).text();
  const owner = root.slice(root.indexOf("pub fn create_document_envelope_from_genesis_owners"), root.indexOf("pub async fn edit_ids_for_changes"));
  expect(owner).toContain("schema: String"); expect(owner).toContain("id: String"); expect(owner).toContain("        schema,"); expect(owner).toContain("        id,");
  expect(owner).not.toContain(".into()"); expect(owner).not.toContain(".clone()");
  const h = await Bun.file(new URL("../../../../../🧾️document/📜️history/💧️hydration/🦀️.rs", import.meta.url)).text();
  const bind = h.slice(h.indexOf("Phase::BindGenesis =>"), h.indexOf("Phase::BuildAppliedCursor |"));
  expect(bind.indexOf("birth.admit(child)")).toBeLessThan(bind.indexOf("self.initial.take()"));
  expect(bind).toContain("admit_verified_pack(initial, pack, initial_digest, child)");
  expect(bind).toContain("*self.initial = Some(initial)"); expect(bind).toContain("*self.pack = Some(pack)");
  expect(bind.includes("empty_document_envelope_from_genesis_owners::<P, M>(schema, expected.artifact_id")).toBe(true);
  console.log("[DEBUG] genesis granted original owner admission and Node UTF8/RFC6902 alias identity; native allocator proof remains required");
});


test("original inference retained wallet transports every unequal and zero axis", async () => {
  const root = new URL("../../../../../../../../../🔨️modules/🌱️value/🧬️retained-clone/🌐️wire/", import.meta.url);
  const fixture = await Bun.file(new URL("🧫️fixtures/🔣️.json", root)).json();
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
  console.log("[DEBUG] Ajv/Node u64 wallet oracle preserves every axis, zero grants and full u64 through checked canonical ownership transport");
});

test("message ledger page birth preserves fixed tickets and partial original backing", async () => {
  const plain = await Bun.file(new URL("../../../../../🧾️document/📜️history/💧️hydration/🧪️tests/🧫️fixtures/📏️physical-demand/🔣️.json", import.meta.url)).json();
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
  const root = await Bun.file(new URL("../../../../../🦀️.rs", import.meta.url)).text();
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
  console.log("[DEBUG] genuine Grant/Ajv and original Buffer/RFC6902 paged ledger: unchanged8192 tickets, fixed independent policy, partial pages retain until paid release");
});

test("envelope birth retains original phased catalogs before hydration advancement", async () => {
  const plain = await Bun.file(new URL("../../../../../🧾️document/📜️history/💧️hydration/🧪️tests/🧫️fixtures/📏️physical-demand/🔣️.json", import.meta.url)).json();
  const row = plain.envelopeBirth;
  const oracle = new Ajv({ strict: false }); oracle.addSchema(orderedRetirement); oracle.addSchema(retirementContract);
  expect(oracle.compile({ $ref: retirementContract.$id + "#/$defs/Grant" })(row.policy)).toBe(true);
  for (const cut of row.cancelCuts) {
    const state = applyPatch({ metadata: row.metadata, catalogBirths: 0, terminal: false }, [{ op: "replace", path: "/catalogBirths", value: cut }], true).newDocument;
    expect(state.metadata).toEqual(row.metadata);
    expect(state.catalogBirths).toBeLessThanOrEqual(row.totalBirthTurns);
  }
  const vcs = await Bun.file(new URL("../../../../../../🌿️vcs/🦀️.rs", import.meta.url)).text();
  expect(vcs.includes("pub fn initial_page_birth_demand(")).toBe(true);
  expect(vcs.includes("pub fn admit_initial_page(")).toBe(true);
  const root = await Bun.file(new URL("../../../../../🦀️.rs", import.meta.url)).text();
  const empty = root.slice(root.indexOf("pub fn empty_document_envelope_from_genesis_owners"), root.indexOf("pub async fn edit_ids_for_changes"));
  expect(empty).toContain("ArtifactHistoryLedger::empty()");
  expect(empty).toContain("HistoryPageStack::empty()");
  expect(empty).toContain("ArtifactEditMessageLedger::empty()");
  expect(empty).not.toContain("ArtifactHistoryLedger::new()");
  expect(empty).not.toContain("ArtifactCursor::default()");
  expect(root.includes("pub fn document_admission_demands(")).toBe(true);
  expect(root.includes("pub fn admit_document_next(")).toBe(true);
  expect(root.includes("pub fn document_admission_is_complete(")).toBe(true);
  const h = await Bun.file(new URL("../../../../../🧾️document/📜️history/💧️hydration/🦀️.rs", import.meta.url)).text();
  expect(h).toContain("Phase::AdmitEnvelope =>");
  const phase = h.slice(h.indexOf("Phase::AdmitEnvelope =>"), h.indexOf("Phase::BuildAppliedCursor |"));
  expect(phase).toContain("admit_document_next(child)");
  expect(phase).toContain("document_admission_is_complete()");
  expect(phase).toContain("progress.fits(child)");
  console.log("[DEBUG] independent genuine Grant/Ajv + RFC6902 original metadata oracle; envelope phased owners precede hydration advancement, native physical proof remains separate");
});


test("original inference reports the actual independent physical receipt", async () => {
  const root = new URL("../../../../../../../../../🔨️modules/🌱️value/🧬️retained-clone/🌐️wire/", import.meta.url);
  const fixture = await Bun.file(new URL("🧫️fixtures/🔣️.json", root)).json();
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
  const plugin = await Bun.file(new URL("../../../../../../🔌️plugin/🦀️.rs", import.meta.url)).text();
  expect(plugin.match(/pub retirement_progress: RetainedCloneProgress/g)?.length).toBe(2);
  expect(plugin).toContain("retirement_progress: execution.retirement_progress");
  console.log("[DEBUG] Ajv and Node receipt oracle preserves original work, copy, capacity and release independently");
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
  const modules = new URL("../../../../../../", import.meta.url);
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
  console.log("[DEBUG] original cancellation custody remains parked in its same router slot until actual terminal close");
});


test("actual snapshot input step funds original birth and both physical copies", async () => {
  const plain = await Bun.file(new URL("../../../🏭️operation/📏️birth/🧫️fixtures/🔣️.json", import.meta.url)).json();
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
  const source = await Bun.file(new URL("../../../🏭️operation/🦀️.rs", import.meta.url)).text();
  const trait = source.slice(source.indexOf("pub trait MemberSnapshotOpenOperation"),source.indexOf("pub struct UnsupportedMemberSnapshotOpen"));
  expect(trait.includes("grant: RetainedCloneGrant")).toBe(true);
  const actual = source.slice(source.indexOf("impl<P: ArtifactPack"),source.indexOf("impl<P: semio_framework_value::retirement::RetireOwned> PackMemberSnapshotOpen"));
  expect(actual.includes("admit_member_input_buffer")).toBe(true);
  expect(actual.includes("grant.maximum_copy_bytes / 2")).toBe(true);
  expect(actual.includes("self.pending(expected_bytes, progress, cx)")).toBe(true);
  const publish = source.slice(source.indexOf("fn publish_input(&mut self,"),source.indexOf("impl<P: ArtifactPack"));
  expect(publish.includes("retained_progress: progress")).toBe(true);
  expect(actual.includes("self.input.try_reserve_exact(expected_bytes)")).toBe(false);
  console.log("[DEBUG] Actual snapshot input fixed grant/Ajv and original Buffer/RFC6902 oracle pays independent birth and two physical copies, native System proof remains separate");
});

test("canonical reader ownership preserves the original byte oracle under independent physical policy", async () => {
  const { readFileSync } = await import("node:fs");
  const { resolve } = await import("node:path");
  const root = process.cwd();
  const base = "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️canonical-edit/";
  const row = JSON.parse(readFileSync(resolve(root, base + "🧫️fixtures/📖️canonical-reader.json"), "utf8"));
  const original = JSON.parse(readFileSync(resolve(root, base + "🧫️fixtures/🗺️canonical-borrowed-map.json"), "utf8"));
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
  console.log("[DEBUG] original canonical reader 4925 UTF8 Buffer/TextEncoder/hash oracle preserved; actual LIFO owner closes map before lifetime; genuine five-axis policy independent, actual System native law authored but unrun");
});

test("member request physical grants remain caller policy independent of the close frontier",async()=>{
 const fs=await import("node:fs"),path=await import("node:path");
 const base=path.join(process.cwd(),"🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open");
 const law=JSON.parse(fs.readFileSync(path.join(base,"📏️retirement/🧫️fixtures/🔣️.json"),"utf8"));
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
 console.log("[DEBUG] member request native policy has fixed independent release262144/depth64; frontier observers price negative bounds only, original physical pointer/allocator laws remain native-unrun");
});

test("snapshot input receipts debit the original context wallet exactly once",async()=>{
 const fs=await import("node:fs"),path=await import("node:path");
 const actual=fs.readFileSync(path.join(process.cwd(),"🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/🦀️.rs"),"utf8");
 const input=actual.slice(actual.indexOf("impl<P> PackMemberSnapshotOpen<P>"),actual.indexOf("impl<P: semio_framework_value::retirement::RetireOwned> PackMemberSnapshotOpen"));
 expect(input.includes("cx.consume_retained(progress)")).toBe(true);
 expect(input.includes("let caller = cx.retained_grant()")).toBe(true);
 for(const axis of ["items","copy_bytes","capacity_bytes","release_bytes","depth"])expect(input.includes("grant.maximum_"+axis+".min(caller.maximum_"+axis+")")).toBe(true);
 expect(input.includes("self.pending(expected_bytes, progress, cx)")).toBe(true);
 const plain=JSON.parse(fs.readFileSync(path.join(process.cwd(),"🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/📏️birth/🧫️fixtures/🔣️.json"),"utf8")).snapshotInput;
 const grant=plain.policy,remaining={...grant};
 const bytes=new Uint8Array(plain.payloadBytes),receipt={copiedItems:1,copiedBytes:bytes.byteLength*plain.expected.copiesPerPayloadByte,retainedCapacityBytes:0,releasedBytes:0};
 const expected={...remaining,maximumItems:remaining.maximumItems-receipt.copiedItems,maximumCopyBytes:remaining.maximumCopyBytes-receipt.copiedBytes};
 const oracle=applyPatch(remaining,[{op:"replace",path:"/maximumItems",value:expected.maximumItems},{op:"replace",path:"/maximumCopyBytes",value:expected.maximumCopyBytes}],true,false).newDocument;
 expect(oracle).toEqual(expected);expect(oracle.maximumCapacityBytes).toBe(grant.maximumCapacityBytes);expect(oracle.maximumReleaseBytes).toBe(grant.maximumReleaseBytes);expect(oracle.maximumDepth).toBe(grant.maximumDepth);
 console.log("[DEBUG] Same original context wallet debits genuine supplied input receipt once; immutable caller policy and JSONPatch independent axes preserved, actual native System proof unrun");
});


test("member and hydration paid turns preserve the original physical wallet",async()=>{
 const fs=await import("node:fs"),path=await import("node:path");
 const base=path.join(process.cwd(),"🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store");
 const plain=JSON.parse(fs.readFileSync(path.join(base,"🧩️composition/🚪️open/📏️retirement/🧫️fixtures/🔣️.json"),"utf8")).physicalWallet;
 const ajv=new Ajv({strict:false});ajv.addSchema(orderedRetirement);ajv.addSchema(retirementContract);const validate=ajv.compile({$ref:retirementContract.$id+"#/$defs/Grant"});expect(validate(plain.grant)).toBe(true);
 const axes=[["maximumItems","copiedItems"],["maximumCopyBytes","copiedBytes"],["maximumCapacityBytes","retainedCapacityBytes"],["maximumReleaseBytes","releasedBytes"]];
 const backing=Buffer.allocUnsafeSlow(plain.grant.maximumCapacityBytes);expect(backing.buffer.byteLength).toBe(plain.receipts[0].retainedCapacityBytes);expect(backing.byteLength).toBe(plain.receipts[1].releasedBytes);
 let remaining={...plain.grant};
 for(const receipt of plain.receipts){const next={...remaining};for(const [grant,progress]of axes){expect(receipt[progress]).toBeLessThanOrEqual(remaining[grant]);next[grant]-=receipt[progress];}const oracle=applyPatch(remaining,axes.map(([grant])=>({op:"replace",path:"/"+grant,value:next[grant]})),true,false).newDocument;expect(oracle).toEqual(next);remaining=oracle;expect(remaining.maximumDepth).toBe(plain.grant.maximumDepth);}
 expect(Object.values(remaining)).toEqual([0,0,0,0,4]);
 const source=fs.readFileSync(path.join(base,"🧩️composition/🚪️open/🦀️.rs"),"utf8"),operation=fs.readFileSync(path.join(base,"🧩️composition/🚪️open/🏭️operation/🦀️.rs"),"utf8"),hydration=fs.readFileSync(path.join(base,"🧾️document/📜️history/💧️hydration/🦀️.rs"),"utf8");
 expect(source.includes("pub(crate) fn member_step_grant")).toBe(true);expect(source.includes("pub(crate) fn record_member_step")).toBe(true);
 expect(source.includes("cx.consume_retained(progress)")).toBe(true);
 for(const axis of ["items","copy_bytes","capacity_bytes","release_bytes","depth"])expect(source.includes("grant.maximum_"+axis+".min(caller.maximum_"+axis+")")).toBe(true);
 expect(operation.includes("let grant = super::member_step_grant(cx, grant);")).toBe(true);expect(operation.includes("super::record_member_step(cx, progress)")).toBe(true);
 expect(hydration.includes("member_open::member_step_grant(cx, grant)")).toBe(true);expect(hydration.includes("member_open::record_member_step(cx, progress)")).toBe(true);
 expect(hydration.includes("match progress_fuel(progress) { Ok(fuel) => cx.consume_fuel(fuel)")).toBe(false);
 for(const [name,input]of [["open",source],["operation",operation],["hydration",hydration]]){const grammar=spawnSync("rustfmt",["--edition","2021","--emit","stdout"],{input,encoding:"utf8",timeout:2000});expect(grammar.error,name).toBeUndefined();expect(grammar.status,name).toBe(0);}
 console.log("[DEBUG] Genuine same-context wallet accepts actual constructor/Fold/catalog/retirement receipts once and intersects all axes before nested work; independent Buffer/RFC6902 full-currency conservation, native whole-turn proof pending");
});

test("snapshot native consumers retain the genuine caller controls and progress domains",async()=>{
 const fs=await import("node:fs"),path=await import("node:path");const base=path.join(process.cwd(),"🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store");
 const snapshot=fs.readFileSync(path.join(base,"📜️space-history/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"),"utf8"),encoding=fs.readFileSync(path.join(base,"📦️codec/🪶️snapshot-capability/🪶️native-encoding/🧪️tests/🦀️.rs"),"utf8"),decoding=fs.readFileSync(path.join(base,"📦️codec/🪶️snapshot-capability/🪶️native-decoding/🧪️tests/🦀️.rs"),"utf8");
 expect(snapshot.includes("semio_framework_value::NativeDecodeProgress")).toBe(false);expect(snapshot.includes("semio_framework_value::NativeEncodeProgress")).toBe(false);
 expect(snapshot.includes("semio_framework_value::native_decoding::NativeDecodeProgress")).toBe(true);expect(snapshot.includes("semio_framework_value::native_encoding::NativeEncodeProgress")).toBe(true);
 const guards=encoding.slice(encoding.indexOf("    GuardedEncoding,"),encoding.indexOf("#[test]"));
 expect(guards.includes("native_owner: &mut NativeSnapshotEncodeOwner")).toBe(true);expect(guards.includes("let native = native_owner.native();")).toBe(true);expect(guards.includes("NativeEncodeControl::new")).toBe(false);expect(guards.includes("encoding, control, native_owner")).toBe(true);
 const original=decoding.slice(decoding.indexOf("impl ArtifactSqliteSnapshot for DecodedBuffers"),decoding.indexOf("fn to_sqlite_database"));expect(original.includes("native: &mut NativeSnapshotDecodeOwner")).toBe(true);expect(original.includes("            native,")).toBe(true);
 const bytes=Buffer.from("雪","utf8");expect(new TextEncoder().encode("雪")).toEqual(new Uint8Array(bytes));expect(JSON.parse(JSON.stringify({bytes:[...bytes],originalControl:true}))).toEqual({bytes:[...bytes],originalControl:true});
 for(const [name,input]of [["snapshot",snapshot],["encoding",encoding],["decoding",decoding]]){const grammar=spawnSync("rustfmt",["--edition","2021","--emit","stdout"],{input,encoding:"utf8",timeout:2000});expect(grammar.error,name).toBeUndefined();expect(grammar.status,name).toBe(0);}
 console.log("[DEBUG] Current exact native progress domains and supplied original codec controls forwarded; independent UTF8 oracle preserved, actual compiler/runtime renewal required");
});

test("native snapshot encoder keeps one original paid receipt wallet",async()=>{
 const fs=await import("node:fs"),path=await import("node:path");const root=process.cwd();
 const plain=JSON.parse(fs.readFileSync(path.join(root,"🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/📏️retirement/🧫️fixtures/🔣️.json"),"utf8")).physicalWallet;
 const ajv=new Ajv({strict:false});ajv.addSchema(orderedRetirement);ajv.addSchema(retirementContract);expect(ajv.compile({$ref:retirementContract.$id+"#/$defs/Grant"})(plain.grant)).toBe(true);
 let used={copiedItems:0,copiedBytes:0,retainedCapacityBytes:0,releasedBytes:0};let remaining={...plain.grant};const axes=[["maximumItems","copiedItems"],["maximumCopyBytes","copiedBytes"],["maximumCapacityBytes","retainedCapacityBytes"],["maximumReleaseBytes","releasedBytes"]];
 for(const receipt of plain.receipts){const operations=axes.map(([grant,progress])=>({op:"replace",path:"/"+grant,value:remaining[grant]-receipt[progress]}));for(const [grant,progress]of axes){expect(receipt[progress]).toBeLessThanOrEqual(remaining[grant]);used[progress]+=receipt[progress];}remaining=applyPatch(remaining,operations,true,false).newDocument;}
 expect(Object.values(remaining)).toEqual([0,0,0,0,4]);expect(used.retainedCapacityBytes).toBe(Buffer.allocUnsafeSlow(64).byteLength);expect(used.releasedBytes).toBe(64);
 const source=fs.readFileSync(path.join(root,"🧰️framework/🔨️modules/🚪️io/⏱️control/🛫️snapshot/🦀️.rs"),"utf8");
 expect(source.includes("pub fn remaining_grant")).toBe(true);expect(source.includes("pub fn record_progress")).toBe(true);expect(source.includes("pub fn progress(&self)")).toBe(true);expect(source.includes("self.progress.checked_add(progress)")).toBe(true);expect(source.includes("progress.fits(self.remaining_grant())")).toBe(true);
 for(const [grant,progress]of [["items","copied_items"],["copy_bytes","copied_bytes"],["capacity_bytes","retained_capacity_bytes"],["release_bytes","released_bytes"]])expect(source.includes("self.grant.maximum_"+grant+"-self.progress."+progress)).toBe(true);
 expect(source.includes("progress.copied_items")).toBe(true);expect(source.includes("record_progress(")).toBe(true);
 const grammar=spawnSync("rustfmt",["--edition","2021","--emit","stdout"],{input:source,encoding:"utf8",timeout:2000});expect(grammar.error).toBeUndefined();expect(grammar.status).toBe(0);
 console.log("[DEBUG] Native snapshot original produced receipt wallet conserves independent full-currency policy; normal codec producer/cancel/native allocator proof remains separate");
});

test("snapshot JSON writer reports actual partial cancellation receipts",async()=>{
 const fs=await import("node:fs"),path=await import("node:path");const source=fs.readFileSync(path.join(process.cwd(),"🧰️framework/🔨️modules/🎒️pack/🔤️json/🦀️.rs"),"utf8");
 const value={text:"雪",rows:[1,true,null]};expect(JSON.parse(JSON.stringify(value))).toEqual(value);expect(new TextEncoder().encode(JSON.stringify(value))).toEqual(new Uint8Array(Buffer.from(JSON.stringify(value))));
 const cursor=source.slice(source.indexOf("impl JsonBorrowedWriteCursor {"),source.indexOf("/// 🌳️ Owns its semantic source"));
 expect(cursor.includes("fn retained_capacity_bytes")).toBe(true);expect(cursor.includes("fn step_original")).toBe(true);expect(cursor.includes("error.with_retained_progress(self.normal_step_progress)")).toBe(true);
 expect(cursor.includes("let retained_before=self.retained_capacity_bytes()?;")).toBe(true);expect(cursor.includes("self.writer.output.as_ref().map_or(0,|output|output.len())")).toBe(true);
 const publish=cursor.indexOf("retained_capacity_bytes:retained_after-retained_before");expect(publish).toBeGreaterThan(-1);expect(cursor.indexOf("result?;control.step()?",publish)).toBeGreaterThan(publish);
 const grammar=spawnSync("rustfmt",["--edition","2021","--emit","stdout"],{input:source,encoding:"utf8",timeout:2000});expect(grammar.error).toBeUndefined();expect(grammar.status).toBe(0);
 console.log("[DEBUG] JSON writer unchanged independent Serde-compatible JSON/UTF8 oracle; actual partial allocation/copy receipt precedes cancellation, native System proof pending");
});

test("native snapshot writer keeps its static cursor in the original paid recipient",async()=>{
 const fs=await import("node:fs"),path=await import("node:path"),root=process.cwd();
 const owner=fs.readFileSync(path.join(root,"🧰️framework/🔨️modules/🚪️io/⏱️control/🛫️snapshot/🦀️.rs"),"utf8"),snapshot=fs.readFileSync(path.join(root,"🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📜️space-history/🚪️io/🪶️sqlite/📸️snapshot/🚦️native/🦀️.rs"),"utf8");
 expect(owner.includes("pub fn drive_cursor")).toBe(true);expect(owner.includes("cursor:&mut Option<T>")).toBe(true);expect(owner.includes("self.remaining_grant()")).toBe(true);expect(owner.includes("driver.record_progress(progress)?")).toBe(true);expect(owner.includes("Some(frame as Box<dyn ErasedSnapshotRetirement>)")).toBe(true);
 const writer=snapshot.slice(snapshot.indexOf("fn write_json("),snapshot.indexOf("pub(super) fn encode("));expect(writer.includes("JsonBorrowedWriteCursor::new()")).toBe(true);expect(writer.includes("owner.drive_cursor")).toBe(true);expect(writer.includes("cursor.normal_step_progress()")).toBe(true);expect(writer.includes("256.min(grant.maximum_items)")).toBe(true);expect(writer.includes("JsonWriteCursor::new")).toBe(false);
 const bytes=Buffer.from(JSON.stringify({unchanged:"雪"}));expect(new TextEncoder().encode(JSON.stringify({unchanged:"雪"}))).toEqual(new Uint8Array(bytes));expect(JSON.parse(bytes.toString("utf8"))).toEqual({unchanged:"雪"});
 for(const input of [owner,snapshot]){const grammar=spawnSync("rustfmt",["--edition","2021","--emit","stdout"],{input,encoding:"utf8",timeout:2000});expect(grammar.error).toBeUndefined();expect(grammar.status).toBe(0);}
 console.log("[DEBUG] Original static writer cursor/body/output custody bound to remaining full grant and exact produced receipts; typed projection and native System tests remain separate");
});

test("native snapshot decoder retains constructor output before cancellation",async()=>{
 const fs=await import("node:fs"),path=await import("node:path");const root=process.cwd(),base="🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store";
 const plain=JSON.parse(fs.readFileSync(path.join(root,base,"📦️codec/🪶️snapshot-capability/🪶️native-decoding/🧫️fixtures/🔣️.json"),"utf8"));
 const ajv=new Ajv({strict:false});ajv.addSchema(orderedRetirement);ajv.addSchema(retirementContract);expect(ajv.compile({$ref:retirementContract.$id+"#/$defs/Grant"})(plain.callerGrant)).toBe(true);
 const bytes=Buffer.from("雪","utf8"),original=new Uint8Array(bytes.length);original.set(new TextEncoder().encode("雪"));expect([...original]).toEqual([...bytes]);
 const source=fs.readFileSync(path.join(root,base,"📦️codec/🪶️snapshot-capability/🪶️native-decoding/🧪️tests/🦀️.rs"),"utf8");
 const body=source.slice(source.indexOf("impl ArtifactSqliteSnapshot for DecodedBuffers"),source.indexOf("fn to_sqlite_database"));
 expect(body.includes("native: &mut NativeSnapshotDecodeOwner")).toBe(true);expect(body.includes("bind_original_buffers,")).toBe(true);
 const binding=source.slice(source.indexOf("fn bind_original_buffers"),source.indexOf("impl ArtifactSqliteSnapshot for DecodedBuffers"));
 expect(binding.indexOf("*snapshot_output = Some")<binding.indexOf("body.allocate_vec_into(")).toBe(true);expect(binding.includes("std::mem::take(bytes)")).toBe(true);expect(binding.includes("Ok(())")).toBe(true);
 expect(source.includes("impl RetireOwned for DecodedBuffers")).toBe(true);expect(source.includes("self.buffers.retirement()")).toBe(true);
 const defining=fs.readFileSync(path.join(root,"🧰️framework/🔨️modules/🌱️value/🛬️decode/🦀️.rs"),"utf8");expect(defining.includes("pub fn copy_bytes_into")).toBe(true);expect(defining.includes("output.extend_from_slice(chunk)")).toBe(true);
 console.log("[DEBUG] Native decoder original typed slot precedes actual buffer births, callerGrant canonical, Buffer/TextEncoder bytes unchanged; native allocator/cancel laws required");
});

test("named native ownership laws use independent positive grants",async()=>{
 const fs=await import("node:fs"),path=await import("node:path");const root=process.cwd(),base="🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store";
 const plain=JSON.parse(fs.readFileSync(path.join(root,base,"🧩️composition/📨️emission/📦️owned/🧫️fixtures/🔣️.json"),"utf8"));const ajv=new Ajv({strict:false});ajv.addSchema(orderedRetirement);ajv.addSchema(retirementContract);expect(ajv.compile({$ref:retirementContract.$id+"#/$defs/Grant"})(plain.physicalCloseGrant)).toBe(true);
 const source=fs.readFileSync(path.join(root,base,"🧪️tests/🔬️unit/🦀️.rs"),"utf8");
 for(const name of ["current_box_retirement_preserves_denied_owner_and_exact_physical_shell","current_typed_option_birth_retains_exact_original_and_paid_shell"]){
  const start=source.indexOf("fn "+name);expect(start).toBeGreaterThanOrEqual(0);const end=source.indexOf("\n#[test]",start);const body=source.slice(start,end<0?undefined:end);
  expect(body.includes("let grant = physical_test_close_grant();")).toBe(true);expect(body.includes("maximum_copy_bytes: demand.copy_bytes")).toBe(false);expect(body.includes("maximum_release_bytes: demand.release_bytes")).toBe(false);
 }
 console.log("[DEBUG] Native named positive cleanup uses immutable plain canonical five-axis policy; frontier quotes only fund denial reductions/physical equality, native System laws unrun");
});

test("member history hydration and durable native closes preserve independent authority",async()=>{
 const fs=await import("node:fs"),path=await import("node:path");const base=path.join(process.cwd(),"🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store");
 const history=fs.readFileSync(path.join(base,"🧩️composition/🚪️open/📜️history/🧪️tests/🔬️unit/🦀️.rs"),"utf8"),hydration=fs.readFileSync(path.join(base,"🧾️document/📜️history/💧️hydration/🧪️tests/🔬️unit/🦀️.rs"),"utf8"),durable=fs.readFileSync(path.join(base,"🧩️composition/🗄️durable-group/🧪️tests/🔬️unit/🦀️.rs"),"utf8");
 expect(history.includes("maximum_capacity_bytes: owner.next_capacity_byte_demand")).toBe(false);expect(hydration.includes("fn grant_of(")).toBe(false);expect(durable.includes("fn demanded_grant(")).toBe(false);
 expect(history.includes("super::super::tests::request_close_policy()")).toBe(true);expect(hydration.includes("crate::os_store::component::tests::physical_test_close_grant()")).toBe(true);expect(durable.includes("crate::os_store::component::tests::physical_test_close_grant()")).toBe(true);
 for(const [name,source] of [["history",history],["hydration",hydration],["durable",durable]] as const){const grammar=spawnSync("rustfmt",["--edition","2021","--emit","stdout"],{input:source,encoding:"utf8",timeout:2000});expect(grammar.error,name).toBeUndefined();expect(grammar.status,name).toBe(0);}
 const policy=fixture.physicalCloseGrant;const ajv=new Ajv({strict:false});ajv.addSchema(orderedRetirement);ajv.addSchema(retirementContract);expect(ajv.compile({$ref:retirementContract.$id+"#/$defs/Grant"})(policy)).toBe(true);expect(Buffer.allocUnsafeSlow(policy.maximumReleaseBytes).byteLength).toBe(policy.maximumReleaseBytes);
 console.log("[DEBUG] Native History/Hydration/Durable cleanup positive authority remains immutable original policy; demanded reductions retain negative denial proofs, native allocator assertions unrun");
});


test("catalog source native laws use independent positive grants",async()=>{
 const fs=await import("node:fs"),path=await import("node:path");
 const source=fs.readFileSync(path.join(process.cwd(),"🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs"),"utf8");
 for(const [name,end] of [["current_catalog_sources_admit_original_allocations_then_fund_each_ticket","current_member_source_constructor_is_not_invoked_under_denied_axes"],["current_member_source_constructor_is_not_invoked_under_denied_axes","ephemeral_member_original_preparation_frame_requires_exact_granted_release"]] as const){
  const start=source.indexOf("fn "+name+"("),finish=source.indexOf("fn "+end+"(",start+1);expect(start).toBeGreaterThanOrEqual(0);expect(finish).toBeGreaterThan(start);
  const body=source.slice(start,finish);expect(body.includes("let paid = physical_test_close_grant();")).toBe(true);expect(body.includes("maximum_capacity_bytes: capacity,")).toBe(false);expect(body.includes("maximum_capacity_bytes: demand.capacity_bytes")).toBe(false);expect(body.includes("let grant = physical_test_close_grant();")).toBe(true);
  for(const original of ["maximum_capacity_bytes: capacity - 1","assert_eq!(allocated, capacity)","assert!(owners.uninstalled_owners_terminal_is_empty())"])expect(body.includes(original)).toBe(true);
 }
 const policy=fixture.physicalCloseGrant;const ajv=new Ajv({strict:false});ajv.addSchema(orderedRetirement);ajv.addSchema(retirementContract);expect(ajv.compile({$ref:retirementContract.$id+"#/$defs/Grant"})(policy)).toBe(true);
 expect(Buffer.allocUnsafeSlow(policy.maximumCapacityBytes).byteLength).toBe(policy.maximumCapacityBytes);
 console.log("[DEBUG] Original catalog native positive policy is immutable and canonical; actual allocator/source/refusal pointer laws preserved, native System assertions unrun");
});


test("current shared actor native callers retain genuine lease and original buffer custody",async()=>{
 const fs=await import("node:fs"),path=await import("node:path"),root=process.cwd();
 const base=path.join(root,"🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store");
 const request=fs.readFileSync(path.join(base,"🧩️composition/🚪️open/🧪️tests/🔬️unit/🦀️.rs"),"utf8"),hydration=fs.readFileSync(path.join(base,"🧾️document/📜️history/💧️hydration/🧪️tests/🔬️unit/🦀️.rs"),"utf8"),history=fs.readFileSync(path.join(base,"📜️space-history/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"),"utf8");
 expect(request.includes("request.actor.0.capacity()")).toBe(false);expect(hydration.includes("owner.actor.0.capacity()")).toBe(false);
 expect(request.includes("request.closing_actor.as_ref().unwrap().as_ptr()")).toBe(true);expect(request.includes("SharedUtf8::admit(actor")).toBe(true);
 expect(history.includes("|value,output,native,body|")).toBe(true);expect(history.includes("body.admit_frontier(")).toBe(true);expect(history.includes("body.record_progress(")).toBe(true);expect(history.includes("*output=Some(())")).toBe(true);expect(history.includes("store::NativeSnapshotDecodeOwner::new(&mut original_native,caller_grant())")).toBe(true);
 const law=JSON.parse(fs.readFileSync(path.join(base,"🧩️composition/🚪️open/📏️retirement/🧫️fixtures/🔣️.json"),"utf8"));
 expect(law.physicalCloseGrant.maximumCopyBytes).toBe(4096);
 for(const row of law.cases){const bytes=new TextEncoder().encode(row.text),original=Buffer.allocUnsafeSlow(row.capacityBytes);original.set(bytes);expect(bytes.byteLength).toBe(Buffer.byteLength(row.text,"utf8"));expect(original.byteLength).toBe(row.capacityBytes);expect(original.subarray(0,bytes.length).toString("utf8")).toBe(row.text);}
 console.log("[DEBUG] Shared actor source retains original payload buffer after funded lease transfer; fixed policy/Buffer/TextEncoder oracle preserved, native identity and System law unrun");
});

test("native body octet spans debit independent policy and retain original cancelled backing",async()=>{
 const fs=await import("node:fs"),path=await import("node:path"),root=process.cwd();
 const neutral=JSON.parse(fs.readFileSync(path.join(root,"🧰️framework/🔨️modules/🚪️io/⏱️control/🛫️snapshot/🧫️fixtures/🪆️receiving/🔣️.json"),"utf8")).bodyCopy;
 const ajv=new Ajv({strict:false});ajv.addSchema(orderedRetirement);ajv.addSchema(retirementContract);const validate=ajv.compile({$ref:retirementContract.$id+"#/$defs/Grant"});
 expect(validate(neutral.grant)).toBe(true);expect(validate(neutral.closeGrant)).toBe(true);
 const payload=new TextEncoder().encode(neutral.text.repeat(neutral.repeat)),oracle=Buffer.from(neutral.text.repeat(neutral.repeat),"utf8");expect(payload).toEqual(new Uint8Array(oracle));expect(payload.byteLength).toBeGreaterThan(neutral.cancelAt);
 for(const completed of [neutral.cancelAt,payload.byteLength]){const backing=Buffer.allocUnsafeSlow(payload.byteLength);backing.set(payload.subarray(0,completed));expect(backing.subarray(0,completed)).toEqual(oracle.subarray(0,completed));const receipt={copiedItems:1,copiedBytes:completed,retainedCapacityBytes:backing.byteLength,releasedBytes:0};const remaining=applyPatch(neutral.grant,[{op:"replace",path:"/maximumItems",value:neutral.grant.maximumItems-receipt.copiedItems},{op:"replace",path:"/maximumCopyBytes",value:neutral.grant.maximumCopyBytes-receipt.copiedBytes},{op:"replace",path:"/maximumCapacityBytes",value:neutral.grant.maximumCapacityBytes-receipt.retainedCapacityBytes}],true,false).newDocument;expect(remaining.maximumCapacityBytes).toBeLessThan(payload.byteLength);expect(remaining.maximumReleaseBytes).toBe(neutral.grant.maximumReleaseBytes);expect(remaining.maximumDepth).toBe(neutral.grant.maximumDepth);}
 const control=fs.readFileSync(path.join(root,"🧰️framework/🔨️modules/🚪️io/⏱️control/🦀️.rs"),"utf8"),decoder=fs.readFileSync(path.join(root,"🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️codec/🪶️snapshot-capability/🪶️native-decoding/🧪️tests/🦀️.rs"),"utf8");
 expect(control.includes("pub fn copy_bytes_into(&mut self,native:")).toBe(true);expect(control.includes("let result=native.copy_bytes_into(bytes,output)")).toBe(true);expect(control.includes("copied_bytes:output.len(),retained_capacity_bytes:output.capacity()")).toBe(true);
 const actual=decoder.slice(decoder.indexOf("fn bind_original_buffers"),decoder.indexOf("impl ArtifactSqliteSnapshot for DecodedBuffers"));expect(actual.includes("body.allocate_vec_into(")).toBe(true);expect(actual.includes("body.record_progress(")).toBe(true);expect(actual.includes("std::mem::take(bytes)")).toBe(true);expect(actual.includes("copy_bytes_into")).toBe(false);
 for(const input of [control,decoder]){const parsed=spawnSync("rustfmt",["--edition","2021","--emit","stdout"],{input,encoding:"utf8",timeout:2000});expect(parsed.error).toBeUndefined();expect(parsed.status).toBe(0);}
 console.log("[DEBUG] Native octet body fixed policy/Buffer/TextEncoder/JSONPatch oracle complete; original producer helper route explicit, native System/cancel law unrun");
});

test("failed native field admission uses genuine bounds and preserves real performed receipts",async()=>{
 const fs=await import("node:fs"),path=await import("node:path"),root=process.cwd(),base=path.join(root,"🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store");
 const neutral=JSON.parse(fs.readFileSync(path.join(base,"🚪️io/🚫️refusal/🧫️fixtures/🔣️.json"),"utf8"));const ajv=new Ajv({strict:false});ajv.addSchema(orderedRetirement);ajv.addSchema(retirementContract);const validate=ajv.compile(JSON.parse(fs.readFileSync(path.join(root,"🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🌐️wire/🧬️schema/🧾️progress.json"),"utf8")));
 for(const row of neutral.cases){expect(validate(row.receipt)).toBe(true);const total=applyPatch({capacity:0,released:0},[{op:"replace",path:"/capacity",value:row.receipt.retainedCapacityBytes},{op:"replace",path:"/released",value:row.receipt.releasedBytes}],true,false).newDocument;expect(total.capacity).toBe(row.expectedTotalCapacity);expect(total.released).toBe(row.expectedTotalRelease);}
 const original=fs.readFileSync(path.join(base,"🧪️tests/🔬️unit/🦀️.rs"),"utf8");const start=original.indexOf("fn field_close_failure_preserves_real_receipts_through_the_original_decode_job()");const actual=original.slice(start,original.indexOf("\n#[test]",start));expect(actual.includes("fn maximum_retained_close_bytes(&self)->usize { ARTIFACT_ENVELOPE_DECODE_MAXIMUM_RETAINED_FIELD_BYTES }")).toBe(true);expect(actual.includes("assert_eq!(progress.retained_capacity_bytes,allocated)")).toBe(true);expect(actual.includes("assert_eq!(progress.released_bytes,released)")).toBe(true);
 const allocated=Buffer.allocUnsafeSlow(1024);expect(allocated.byteLength).toBe(1024);expect(allocated.byteLength).toBeLessThan(262144*6);
 console.log("[DEBUG] Native field legal defining admission bounds preserve original performed receipts; canonical per-progress Ajv/JSONPatch/Buffer oracle, actual System renewal required");
});

test("original VCS checkpoint measurement stays borrowed and cannot become consuming authority",async()=>{
 const fs=await import("node:fs"),path=await import("node:path"),{default:Ajv2020}=await import("ajv/dist/2020.js"),root=process.cwd(),base=path.join(root,"🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🚪️io/💾️binary/🪪️entity-identity/🎛️control");
 const plain=JSON.parse(fs.readFileSync(path.join(base,"🧫️fixtures/📥️input/🔣️.json"),"utf8")).checkpointProjection;
 const original={maximum:plain.originalMaximumBytes,owned:0,finished:false,reserved_return:0,phase_start_owned:0,phase_start_reserved_return:0,received_output:0,received_input:0,received_retirement:plain.hostCharge};
 const projected=applyPatch(original,[{op:"add",path:"/kind",value:"original-retirement-measurement"},{op:"replace",path:"/received_retirement",value:original.received_retirement+plain.proposedCharge}],true,false).newDocument;
 const oracle=new Ajv2020({strict:false});expect(oracle.compile(JSON.parse(fs.readFileSync(path.join(base,"🔎️measurement/🧬️schema/🔣️.json"),"utf8")))(projected)).toBe(true);expect(oracle.compile(JSON.parse(fs.readFileSync(path.join(base,"🧬️schema/🧾️receipt/🔣️.json"),"utf8")))(projected)).toBe(false);
 expect(original.received_retirement).toBe(plain.actualRetirement);expect(projected.received_retirement).toBe(plain.projectedRetirement);expect(original.maximum).toBe(plain.originalMaximumBytes);expect("kind"in original).toBe(false);
 const producer=fs.readFileSync(path.join(base,"🦀️.rs"),"utf8"),measurement=fs.readFileSync(path.join(base,"🔎️measurement/🦀️.rs"),"utf8"),native=fs.readFileSync(path.join(base,"🧪️tests/🦀️.rs"),"utf8");
 expect(producer.includes("pub fn project_retirement_charge(&self,additional:usize)->Result<measurement::OriginalOperationProjection<'_>,ValueError>")).toBe(true);expect(producer.includes("self.source_owned().and_then")).toBe(true);expect(measurement.includes('output.serialize_field("kind","original-retirement-measurement")')).toBe(true);expect(native.includes("serde_json::from_value::<OriginalOperationReceipt>(scalar).is_err()")).toBe(true);
 for(const input of [producer,measurement,native]){const grammar=spawnSync("rustfmt",["--edition","2021","--emit","stdout"],{input,encoding:"utf8",timeout:2000});expect(grammar.error).toBeUndefined();expect(grammar.status).toBe(0);}
 console.log("[DEBUG] Actual genuine borrowed VCS measurement contract rejects projected DTO as consuming receipt; original maximum/actual charge preserved via JSONPatch/Ajv2020, native renewal required");
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
 const binding=native.slice(native.indexOf("fn bind_original_buffers"),native.indexOf("impl ArtifactSqliteSnapshot for DecodedBuffers"));
 expect(binding).toContain("body.admit_frontier");
 expect(binding).toContain("std::mem::take(bytes)");
 expect(binding).not.toContain("copy_bytes_into");
 expect(native).toContain("fn sqlite_snapshot_binding_moves_original_octet_allocations_with_exact_structural_receipt");
 for(const input of [native,fs.readFileSync(path.join(base,"🛬️native-decoding/🦀️.rs"),"utf8")]){const parsed=spawnSync("rustfmt",["--edition","2021","--emit","stdout"],{input,encoding:"utf8",timeout:2000});expect(parsed.error).toBeUndefined();expect(parsed.status).toBe(0);}
 expect(binding.indexOf("body.admit_frontier",binding.indexOf("for field"))).toBeLessThan(binding.indexOf("std::mem::take(bytes)"));
 const producer=fs.readFileSync(path.join(base,"🛬️native-decoding/🦀️.rs"),"utf8");
 expect(producer).toContain("FnOnce(&mut RecordValue");
 expect(producer).toContain("construct(record.as_mut().unwrap()");
 console.log("[DEBUG] SQLite binding original Buffer allocation identity, unchanged65536 caller copy authority, paid structural move before original slot transfer");
});

test("original history mutation ordinals preserve the full unsigned64 domain across JSON",async()=>{
 const {default:Decimal}=await import("decimal.js");const OrdinalDecimal=Decimal.clone({precision:40});
 const root="🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/";
 const schema=await Bun.file(root+"📜️history/🆔️ordinal/🧬️schema/🔣️.json").json(),validate=new Ajv({strict:false}).compile(schema);
 const backing=await Bun.file(root+"♻️retirement/📦️backing/🧫️fixtures/🔣️.json").json();
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
 console.log("[DEBUG] exact u64 history ordinals BigInt/decimal.js/Ajv/native-width independent receiving proof");
});
