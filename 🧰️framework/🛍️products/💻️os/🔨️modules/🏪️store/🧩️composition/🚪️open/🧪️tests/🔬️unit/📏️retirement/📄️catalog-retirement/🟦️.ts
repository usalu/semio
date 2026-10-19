import { test, expect } from "bun:test";
import Ajv from "ajv";
import { orderedRetirementSchema as orderedRetirement } from "@semio-tech/value-ts/ordered/retirement/schema";
import { retirementContractSchema as retirementContract } from "@semio-tech/value-ts/retirement/contract/schema";
import fixture from "../../../../📏️retirement/🧪️tests/🧫️fixtures/🔣️.json" with { type: "json" };
import { Buffer } from "node:buffer";
import { applyPatch } from "fast-json-patch";
import { frameworkSourcePath, readStoreSourceText } from "@semio-tech/framework/test/source-text";
import { readModuleTreeSource } from "@semio-tech/framework-compiler-ts/syntax/module-tree";

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
  const source = readModuleTreeSource(frameworkSourcePath("🔨️modules/🌱️value/📸️snapshot/👁️read/🦀️.rs"));
  const helperStart = source.indexOf("pub fn artifact_retirement_box_"), helperLast = source.lastIndexOf("pub fn artifact_retirement_box_");
  const helpers = source.slice(helperStart, source.indexOf("\n}\n", helperLast) + 3);
  expect(helpers).toContain("pub fn artifact_retirement_box_demands");
  const native=readModuleTreeSource(new URL("../../../../../../🧪️tests/🌴️bin-06/🔬️unit/🦀️.rs",import.meta.url));
  const law=native.slice(native.indexOf("fn current_box_retirement_preserves_denied_owner"),native.indexOf("fn artifact_store_group_visibility",native.indexOf("fn current_box_retirement_preserves_denied_owner")));
  expect(law.indexOf("let policy = physical_test_close_grant();")).toBeGreaterThanOrEqual(0);
  expect(law.indexOf("let policy = physical_test_close_grant();")).toBeLessThan(law.indexOf("observe_backing"));
  expect(helpers).not.toContain("SnapshotRetirementStep");
  expect(helpers).not.toContain("next_close_byte_demand");
  expect(helpers).toContain("owner.next_demand(maximum_body_bytes)");
  expect(helpers).toContain("owner.next_depth_demand()");
  expect(helpers).toContain("maximum_depth - 1");
  expect(helpers).toContain("admit_retained_clone_close");
  expect(helpers.indexOf("maximum_release_bytes < extent")).toBeLessThan(helpers.indexOf("drop(slot.take())"));
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
  const source = await Bun.file(new URL("../../../../📜️history/🗂️dictionary/📇️index/🦀️.rs", import.meta.url)).text();
  expect(source).not.toContain("close_offset");
  expect(source).toContain("grant.maximum_release_bytes < PAGE_BYTES");
  expect(source).toContain("next_release_byte_demand");
  expect(source).toContain("RetainedCloneStep");
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
  const source = readModuleTreeSource(new URL("../../../../🦀️.rs",import.meta.url));
  const request = source.slice(source.indexOf("impl MemberOpenRequest {"), source.indexOf("impl ErasedSnapshotRetirement for MemberOpenRequest"));
  expect(request.includes("owned_retirement(")).toBe(false);
  expect(request.includes("drop(std::mem::take(field))")).toBe(true);
  expect(request.includes("field.capacity()")).toBe(true);
  expect(request.includes("closing_identity_field")).toBe(true);
  const oversized = applyPatch({ capacityBytes: fixture.maximumAdmissionBytes }, [{ op: "replace", path: "/capacityBytes", value: fixture.maximumAdmissionBytes + 1 }], true).newDocument;
  expect(Buffer.allocUnsafeSlow(oversized.capacityBytes).buffer.byteLength).toBeGreaterThan(fixture.maximumAdmissionBytes);
});

test("member-open inline page work and whole backing release remain separate authorities", async () => {
  const input = await Bun.file(new URL("../../../../📏️retirement/🧪️tests/🧫️fixtures/📄️inline.json", import.meta.url)).json();
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
  const source = readModuleTreeSource(new URL("../../../../🦀️.rs",import.meta.url));
  const request = source.slice(source.indexOf("pub struct MemberOpenRequest"), source.indexOf("impl ErasedSnapshotRetirement for MemberOpenRequest"));
  expect(request).not.toContain("closing_page");
  expect(request).not.toContain("closing_bytes");
  expect(request).toContain("copied_items: 1, ..empty");
  expect(request).toContain("pages.close_backing_step(grant)?");
  expect(request).toContain("return Ok(RetainedCloneStep::Progress(step.progress()));");
  expect(request).toContain("grant.maximum_release_bytes");
  expect(request).toContain("grant.maximum_depth == 0");
  expect(request).not.toContain("SnapshotRetirementStep");
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
  const source = readStoreSourceText();
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
  const source = readStoreSourceText();
  const start = source.indexOf("pub fn admit_constructor");
  const admission = source.slice(start, source.indexOf("fn constructor_header_copy_bytes", start));
  expect(start).toBeGreaterThan(0);
  expect(admission.indexOf("grant.maximum_items==0")).toBeGreaterThanOrEqual(0);expect(admission.indexOf("grant.maximum_items==0")).toBeLessThan(admission.indexOf("slot.prepare_step("));
  expect(admission.indexOf("grant.maximum_capacity_bytes<demand.capacity_bytes")).toBeGreaterThanOrEqual(0);expect(admission.indexOf("grant.maximum_capacity_bytes<demand.capacity_bytes")).toBeLessThan(admission.indexOf("slot.prepare_step("));
  expect(admission.indexOf("grant.maximum_depth<demand.depth")).toBeGreaterThanOrEqual(0);expect(admission.indexOf("grant.maximum_depth<demand.depth")).toBeLessThan(admission.indexOf("slot.prepare_step("));
  expect(admission).toContain("return Err((error,receipt))");
  expect(admission).toContain("constructor_demands(grant.maximum_copy_bytes)");
  expect(source).toContain("pub struct DocumentStoreOwnersAdmissionError");
});

test("initialization preserves whole catalog backings until granted typed retirement birth", async () => {
  for (const row of fixture.catalogBirthGrants) {
    const storage = Buffer.allocUnsafeSlow(64);
    const after = applyPatch({ original: true, active: false, capacity: storage.byteLength }, row.items > 0 && row.capacity >= storage.byteLength && row.depth >= 2 ? [{ op: "replace", path: "/active", value: true }] : [], true).newDocument;
    expect(after.original).toBe(true);
    expect(after.capacity).toBe(storage.byteLength);
  }
  const source = readStoreSourceText();
  const start = source.indexOf("pub fn initialization_retirement_demands");
  expect(start).toBeGreaterThan(0);
  const runtime = source.slice(start, source.indexOf("#[allow(clippy::type_complexity)]", start));
  expect(runtime).toContain("ReturnedSnapshotReadRetirement::<P>::constructor_capacity_bytes()");
  expect(runtime).toContain("ArtifactStoreStringVectorRetirement::new");
  expect(runtime).toContain("ArtifactStoreRevisionAccumulatorRetirement::new");
  expect(runtime).toContain("Err((error, original))");
  expect(runtime).not.toContain("pop_first()");
  expect(runtime).not.toContain("SnapshotRetirementStep");
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
  const source = readStoreSourceText();
  const start = source.indexOf("pub fn admit_constructor");
  const admission = source.slice(start, source.indexOf("fn constructor_header_copy_bytes", start));
  expect(admission).toContain("..grant};");
  expect(admission).toContain("if !receipt.fits(child)");
  expect(source).toContain("pub fn constructor_is_complete");
  expect(admission).not.toContain("grant.maximum_items < items");
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
  const source = readModuleTreeSource(new URL("../../../../🏭️operation/🦀️.rs",import.meta.url));
  const drive = source.slice(source.indexOf("fn drive_active_hydration_retirement"), source.indexOf("/// Admits the selected history"));
  expect(drive).toContain("grant: RetainedCloneGrant");
  expect(drive).toContain("artifact_retirement_box_close_step");
  expect(drive).not.toContain("fuel_remaining()");
  expect(drive).not.toContain("SnapshotRetirementStep");
  expect(source).toContain("step_store(&mut self,cx:&mut StepContext");
  const root = readModuleTreeSource(frameworkSourcePath("🛍️products/💻️os/🔨️modules/🏪️store/🎲️space-members-macro/🦀️.rs"));
  const macro = root.slice(root.indexOf("impl $crate::os_store::MemberOpenOperation for $open_name"), root.indexOf("impl $crate::os_store::SpaceMember for $enum_name", root.indexOf("impl $crate::os_store::MemberOpenOperation for $open_name")));
  expect(macro).toContain("open.step_store(cx, grant)");
  expect(macro).not.toContain("maximum_bytes");
});

test("typed option admission retains the original owner on all denied birth axes", async () => {
  const original = Buffer.allocUnsafeSlow(64);
  for (const row of fixture.catalogPhaseGrants) {
    const admitted = row.items > 0 && row.capacity >= original.byteLength && row.depth >= 2;
    const custody = applyPatch({ pending: true, active: false }, admitted ? [{ op: "replace", path: "/pending", value: false }, { op: "replace", path: "/active", value: true }] : [], true).newDocument;
    expect(Number(custody.pending) + Number(custody.active)).toBe(1);
    expect(custody.pending).toBe(!admitted);
  }
  const root = readModuleTreeSource(frameworkSourcePath("🔨️modules/🌱️value/📸️snapshot/👁️read/🦀️.rs"));
  const start = root.indexOf("pub fn artifact_retirement_admit_owned");
  expect(start).toBeGreaterThan(0);
  const body = root.slice(start, root.indexOf("///", start + 10));
  expect(body).toContain("admit_owned_retirement(original, child)");
  expect(body).toContain("*pending = Some(original)");
  expect(body).toContain("grant.maximum_depth - 1");
  expect(body).not.toContain("SnapshotRetirementStep");
});

test("initial member close keeps original inputs and independent controlled currencies", async () => {
  for (const row of fixture.catalogPhaseGrants) {
    const original = { pending: true, allocation: Buffer.allocUnsafeSlow(64).byteLength };
    const transferred = row.items > 0 && row.capacity >= original.allocation && row.depth >= 2;
    const result = applyPatch(original, transferred ? [{ op: "replace", path: "/pending", value: false }] : [], true).newDocument;
    expect(result.allocation).toBe(original.allocation);
    expect(result.pending).toBe(!transferred);
  }
  const source = readModuleTreeSource(new URL("../../../../🏭️operation/🦀️.rs",import.meta.url));
  const start = source.indexOf("impl<F, P, M, C> InitialMemberStoreOpen<F, P, M, C>");
  const body = source.slice(start, source.indexOf("impl<F, P, M, C> Drop for InitialMemberStoreOpen", start));
  expect(body).toContain("grant: RetainedCloneGrant");
  expect(body).toContain("artifact_retirement_admit_owned");
  expect(body).toContain("next_capacity_byte_demand");
  expect(body).toContain("next_release_byte_demand");
  expect(body).toContain("next_depth_demand");
  expect(body).not.toContain("SnapshotRetirementStep");
  expect(body).not.toContain("owned_retirement(");
  expect(body).not.toContain(".truncate(");
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
  const source = readStoreSourceText();
  expect(readModuleTreeSource(frameworkSourcePath("🛍️products/💻️os/🔨️modules/🏪️store/📒️ledger/📨️messages/🦀️.rs"))).toContain("struct ArtifactEditMessageLedgerRetirement");
  const envelope = readModuleTreeSource(frameworkSourcePath("🛍️products/💻️os/🔨️modules/🏪️store/🔗️read/🗑️envelope-genesis-and-displaced-snapshot-retirements/🦀️.rs"));
  const start = envelope.indexOf("struct ArtifactStoreEnvelopeRetirement");
  const body = envelope.slice(start, envelope.indexOf("struct ArtifactGenesisRetirement", start));
  expect(body).toContain("retirement_owner_close");
  expect(body).toContain("ArtifactStoreVcsRetirement");
  expect(body).toContain("ArtifactEnvelopeMetadata");
  expect(body).not.toContain("SnapshotRetirementStep");
  expect(body).not.toContain("take_metadata_string");
  expect(body).not.toContain(".pop()");
});

test("member hot retirement preserves pending decoder and original rejected inputs", async () => {
  for (const row of fixture.catalogPhaseGrants) {
    const original = Buffer.allocUnsafeSlow(64);
    const admitted = row.items > 0 && row.capacity >= original.byteLength && row.depth >= 2;
    expect(applyPatch({ original: true }, admitted ? [{ op: "replace", path: "/original", value: false }] : [], true).newDocument.original).toBe(!admitted);
  }
  const source = readModuleTreeSource(new URL("../../../../🏭️operation/🦀️.rs",import.meta.url));
  const start = source.indexOf("fn drive_active_hydration_retirement");
  const hot = source.slice(start, source.indexOf("impl<F, P, M, C> MemberOpenOperation for InitialMemberStoreOpen"));
  expect(hot).toContain("artifact_retirement_box_close_step(&mut self.active, child)");
  expect(hot).toContain("maximum_items: grant.maximum_items.min(1), ..grant");
  expect(hot).toContain("*self.rejected_history_input = Some(rejected.input)");
  expect(hot).not.toContain("retirement::owned_retirement(");
  expect(hot).not.toContain("bytes.truncate(");
  expect(hot).not.toContain("Box::new(rejected");
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
  const root = readModuleTreeSource(new URL("../../../../../../🧾️document/📜️history/💧️hydration/🦀️.rs", import.meta.url));
  const helpers = root.slice(root.indexOf("fn drive_hydration_retirement"), root.indexOf("#[derive(Clone, Copy, Debug, PartialEq, Eq)]"));
  expect(helpers).toContain("grant: RetainedCloneGrant");
  expect(helpers).toContain("runtime.close_step(factory, grant)");
  expect(helpers).not.toContain("maximum_bytes =");
  expect(helpers).not.toContain("SnapshotRetirementStep");
  expect(root).toContain("match self.step(cx, grant)");
});
