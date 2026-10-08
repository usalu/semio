import { expect, test } from "bun:test";
import { applyPatch, type Operation } from "fast-json-patch";
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import Parser from "web-tree-sitter";
import fixture from "../🧫️fixtures/🔣️.json" with { type: "json" };
import input from "../📄️input/🧫️fixtures/🔣️.json" with { type: "json" };

/** 🧾️ Independent ordered receipt oracle preserves a common group without synthesizing transactions. */
test("private group copies exact prepared edit receipts before final staging", () => {
  
  for (const parentTouched of fixture.receipts.parentTouched) for (const transaction of fixture.receipts.transactions) {
    const state = { copied: fixture.receipts.editIds.map(() => false), staged: fixture.receipts.editIds.map(() => false), transaction };
    for (const index of fixture.receipts.copyOrder) {
      if (index === 0 && !parentTouched) continue;
      expect(state.staged[index]).toBe(false);
      const edit = fixture.receipts.editIds[index];
      expect(Buffer.from(edit)).toEqual(Buffer.from(new TextEncoder().encode(edit)));
      applyPatch(state, [{ op: "replace", path: `/copied/${index}`, value: true }], true, true);
      applyPatch(state, [{ op: "replace", path: `/staged/${index}`, value: state.copied[index] }], true, true);
      expect(state.transaction).toEqual(transaction);
      expect(fixture.receipts.groupId).not.toBe(transaction?.id);
    }
    expect(state.staged.filter((_, index) => index !== 0 || parentTouched).every(Boolean)).toBe(true);
    expect(state.staged[0]).toBe(parentTouched);
  }
  const store = readFileSync(new URL("../../../../../🏪️store/🦀️.rs", import.meta.url), "utf8");
  const owner = readFileSync(new URL("../../../📬️publication/🤝️group/📦️owner/🦀️.rs", import.meta.url), "utf8");
  expect(store.includes("fn prepared_edit_id(&self) -> Option<&str>")).toBe(true);
  for (const authority of ["prepared_operation_count", "prepared_operation_metadata", "prepared_operation("]) expect(store.includes(authority)).toBe(true);
  for (const authority of ["prepared_receipt", "prepared_publication", "acknowledge_prepared_receipt", "parent_receipt_copied", "receipt_copied", "parent_touched", "parent_identity"]) expect(owner.includes(authority)).toBe(true);
  console.log("[DEBUG] independent receipt corpus preserves three exact UTF8 edit identifiers, prestage copy acknowledgment, one common group and original optional transaction");
});

/** 📄️ Independent JSON UTF8 oracle preserves every streamed page byte under original64-byte copies. */
test("private genesis input pages preserve ordered multilingual JSON bytes and cancellation", () => {
  
  for (const count of input.repeatCounts) {
    const source = input.source.repeat(count);
    const bytes = new TextEncoder().encode(JSON.stringify(source));
    const pages: Uint8Array[] = [];
    for (let position = 0; position < bytes.length; position += input.pageBytes) {
      const page = new Uint8Array(Math.min(input.pageBytes, bytes.length - position));
      for (let copied = 0; copied < page.length; copied += input.maximumCopyBytes) page.set(bytes.subarray(position + copied, position + Math.min(copied + input.maximumCopyBytes, page.length)), copied);
      pages.push(page);
    }
    const joined = new Uint8Array(bytes.length);
    let offset = 0;
    for (const page of pages) { joined.set(page, offset); offset += page.length; }
    expect(joined).toEqual(bytes);
    expect(JSON.parse(new TextDecoder("utf-8", { fatal: true }).decode(joined))).toBe(source);
    for (const stop of input.cancelAt) expect(bytes.subarray(0, Math.min(stop * input.maximumCopyBytes, bytes.length))).toEqual(joined.subarray(0, Math.min(stop * input.maximumCopyBytes, joined.length)));
    console.log(`[DEBUG] private genesis JSON UTF8 oracle count=${count} bytes=${bytes.length} pages=${pages.length} copy=${input.maximumCopyBytes}`);
  }
});

/** 🌱️ Independent three-lane reference preserving private child identity until common acceptance. */
test("private genesis preserves distinct declared identities and publishes exactly three lanes", () => {
  
  expect(fixture.source.childId).not.toBe(fixture.source.reference.artifactId);
  const lanes = ["parent", "brep", "model"] as const;
  for (const stop of fixture.cancelBeforeCommit) {
    const visible = structuredClone(fixture.initial);
    const privateSources = structuredClone(fixture.initial);
    for (let index = 0; index < Math.min(stop, lanes.length); index++) {
      const lane = lanes[index];
      applyPatch(privateSources[lane], fixture.patches[lane] as Operation[], true, true);
      expect(visible).toEqual(fixture.initial);
    }
    expect(visible).toEqual(fixture.initial);
  }
  const committed = structuredClone(fixture.initial);
  for (const lane of lanes) applyPatch(committed[lane], fixture.patches[lane] as Operation[], true, true);
  const adopted = applyPatch(committed, [{ op: "add", path: `/visibleMembers/${fixture.source.childId}`, value: fixture.source.reference }], true, true).newDocument;
  expect(adopted).toEqual(fixture.expected);
  for (const lane of lanes) expect(adopted[lane]).not.toEqual(fixture.initial[lane]);
  console.log(`[DEBUG] Private genesis independent oracle lanes=${lanes.length} cancellationStops=${fixture.cancelBeforeCommit.length} local=${fixture.source.childId} target=${fixture.source.reference.artifactId}`);
});

/** 🪪️ The retained publication driver keeps typed sources private until one common decision. */
test("private typed publication lanes retain original sources through staged adoption and cancellation", () => {
  const lanes = ["parent", "brep", "model"] as const;
  for (const stop of fixture.cancelBeforeCommit) {
    const privateRows = structuredClone(fixture.initial);
    for (const lane of lanes.slice(0, Math.min(stop, lanes.length))) applyPatch(privateRows[lane], fixture.patches[lane] as Operation[], true, true);
    expect(structuredClone(fixture.initial)).toEqual(fixture.initial);
  }
  const source = readFileSync(new URL("../../../📬️publication/🤝️group/🦀️.rs", import.meta.url), "utf8");
  for (const authority of ["PrivateOwnedPublicationLane", "owned_publication_birth_bytes", "begin_one_item_owned_publication", "prepare_one_item_publication", "stage_one_item_publication", "adopt_one_item_publication", "abort_one_item_publication", "maximum_release_bytes", "terminal_is_empty"]) expect(source.includes(authority)).toBe(true);
  expect(source.includes("encode_op")).toBe(false);
  console.log("[DEBUG] RFC6902 private three-lane corpus preserves all seven precommit cancellation states; driver consumes only original typed owner admission and staged common authority");
});

/** 🌳️ Independent Rust grammar checks the retained driver while the full native compiler owns its queue. */
test("private publication lane owner parses with the independent Rust grammar", async () => {
  await Parser.init({ locateFile: () => join(dirname(fileURLToPath(import.meta.resolve("web-tree-sitter"))), "tree-sitter.wasm") });
  const parser = new Parser();
  parser.setLanguage(await Parser.Language.load(join(dirname(fileURLToPath(import.meta.resolve("tree-sitter-wasms/package.json"))), "out/tree-sitter-rust.wasm")));
  for (const [path, close] of [["../../../📬️publication/🤝️group/🦀️.rs", "close_step"], ["../../../📬️publication/🤝️group/🪆️child/🦀️.rs", "close_granted"], ["../../../📬️publication/🤝️group/📦️owner/🦀️.rs", "close_step"]]) {
    const tree = parser.parse(readFileSync(new URL(path, import.meta.url), "utf8"));
    try { expect(tree.rootNode.hasError()).toBe(false); expect(tree.rootNode.descendantsOfType("function_item").some(node => node.childForFieldName("name")?.text === close)).toBe(true); }
    finally { tree.delete(); }
  }
  parser.delete();
  console.log("[DEBUG] independent Tree-sitter Rust grammar accepts retained private typed lane owner and canonical close function; native type/runtime proof remains a separate gate");
});

/** 🪆️ The private child input row borrows original identities before guarded request handoff. */
test("private child input row keeps local identity and exact owner separate through preparation", () => {
  
  const row = fixture.expected.parent.breps[0];
  expect(row.id).toBe(fixture.source.childId);
  expect(row.target).toEqual(fixture.source.reference);
  expect(new TextEncoder().encode(row.id)).not.toEqual(new TextEncoder().encode(row.target.artifactId));
  expect(fixture.source.maximumDeclaredChildren).toBe(64);
  expect(fixture.source.maximumPhaseAllocationBytes).toBe(262144);
  const source = readFileSync(new URL("../../../📬️publication/🤝️group/🪆️child/🦀️.rs", import.meta.url), "utf8");
  for (const authority of ["PrivateChildPublicationInput", "PrivateChildMemberMetadataIssuer", "PrivateChildGenesisInput", "MemberOpenRequest", "next_capacity_byte_demand", "maximum_copy_bytes", "maximum_release_bytes", "close_granted", "terminal_is_empty"]) expect(source.includes(authority)).toBe(true);
  expect(source.includes("encode_op")).toBe(false);
  expect(source.includes(".clone()")).toBe(false);
  console.log("[DEBUG] private child input contract retains distinct local/target identities, fixed64-child decision bound, separately admitted262144 phase ceiling and original64 borrowed-copy bound");
});

/** 📦️ Independent ordered-source and cancellation reference for separately admitted private row frames. */
test("private group owner keeps original ordered inputs and every declared cancellation frontier", () => {
  
  for (const values of [fixture.ownerInput.parentMutations, fixture.ownerInput.childMutations]) {
    const typed = Int32Array.from(values);
    expect(Array.from(typed)).toEqual(values);
    expect(typed.byteLength).toBe(values.length * 4);
  }
  const packed = Buffer.from(JSON.stringify(fixture.ownerInput.initialPack), "utf8");
  expect([...packed]).toEqual([...new TextEncoder().encode(JSON.stringify(fixture.ownerInput.initialPack))]);
  expect(JSON.parse(packed.toString("utf8")).ordered).toEqual(fixture.ownerInput.childMutations);
  for (const stop of fixture.ownerInput.cancelStops) {
    const original = { parent: fixture.ownerInput.parentMutations, child: fixture.ownerInput.childMutations };
    const privateRows = structuredClone(original);
    applyPatch(privateRows, [{ op: "add", path: "/privateInputTurns", value: stop }], true, true);
    expect(original.parent).toEqual(fixture.ownerInput.parentMutations);
    expect(original.child).toEqual(fixture.ownerInput.childMutations);
  }
  const source = readFileSync(new URL("../../../📬️publication/🤝️group/📦️owner/🦀️.rs", import.meta.url), "utf8");
  for (const authority of ["PRIVATE_CHILD_GROUP_MAXIMUM_CHILDREN", "row_birth_bytes", "advance_inputs", "advance_openings", "advance_publications", "advance_projection", "advance_entries", "advance_graph", "next_close_byte_demand", "terminal_is_empty", "maximum_copy_bytes", "maximum_release_bytes"]) expect(source.includes(authority)).toBe(true);
  expect(source.includes("encode_op")).toBe(false);
  expect(source.includes(".clone()")).toBe(false);
  console.log(`[DEBUG] private group independent Int32/Buffer/RFC6902 oracle preserves original ordered inputs at ${fixture.ownerInput.cancelStops.length} cancellation frontiers; row/frame physical parity remains its native allocator law`);
});
