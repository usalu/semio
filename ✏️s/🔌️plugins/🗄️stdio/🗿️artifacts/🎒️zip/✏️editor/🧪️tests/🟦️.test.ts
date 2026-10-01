import Ajv from "ajv";
import diffSchema from "../../🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🔣️.json";
import snapshotSchema from "../../🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🔣️.json";
import { parseZipDiff, applyZipDiff, zipInsertionDiff } from "../../🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🟦️.ts";
import Ajv2020 from "ajv/dist/2020.js";
import { applyPatch } from "fast-json-patch";
import { describe, expect, it } from "bun:test";
import fixture from "../🧫️fixtures/🔣️.json";
import schema from "../🎮️commands/✏️set-node/🔣️schema.json";
import checkpointSchema from "../🧵️retained/🔣️schema.json";
import commentMutationSchema from "../../🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/💬set-archive-comment/🧬️schema/🔣️.json";
import { ArchiveTextCursor, archiveCommentUtf8AfterEdit, archiveEntryNodeId, archiveTextRevision, editArchiveText, ZIP_MAXIMUM_TEXT_BYTES } from "../🟦️.ts";
import { parseZipSnapshot } from "../../🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🟦️.ts";

const archiveSchema = () => new Ajv2020().addKeyword({
  keyword: "x-semio-maxUtf8Bytes", type: "string", schemaType: "number",
  validate: (maximum: number, value: string) => Buffer.byteLength(value, "utf8") <= maximum,
}).compile(schema);

describe("guarded archive text editing", () => {
  it("yields once per bounded name while retaining untouched member bytes", () => {
    const snapshot = structuredClone(fixture.snapshot);
    snapshot.entries[0]!.data = new Array(fixture.retainedResolution.preservedPayloadBytes).fill(42);
    const payload = snapshot.entries[0]!.data;
    const cursor = new ArchiveTextCursor();
    const event = { nodeId: archiveEntryNodeId(fixture.rename.name), revision: archiveTextRevision(fixture.rename.name), value: fixture.rename.value };
    for (let index = 0; index < snapshot.entries.length; index++) {
      expect(cursor.advance(snapshot, event)).toBeUndefined();
      expect(cursor.scannedEntries - index).toBe(fixture.retainedResolution.maximumEntriesPerStep);
      expect(snapshot.entries[0]!.data).toBe(payload);
    }
    expect(cursor.advance(snapshot, event)).toEqual([{ mutation: "renameEntry", name: fixture.rename.name, newName: fixture.rename.value }]);
    expect(() => cursor.advance(snapshot, event)).toThrow("stdio.zip.work-complete");
  });
  it("reconstructs the retained cursor from the neutral binary layout without rescanning", () => {
    const event = { nodeId: archiveEntryNodeId(fixture.rename.name), revision: archiveTextRevision(fixture.rename.name), value: fixture.rename.value };
    const cursor = new ArchiveTextCursor();
    for (let index = 0; index < fixture.retainedResolution.resumeAfterEntries; index++) cursor.advance(fixture.snapshot, event);
    const checkpoint = cursor.checkpoint();
    const oracle = Buffer.alloc(fixture.retainedResolution.checkpointBytes);
    oracle.write(fixture.retainedResolution.checkpointMagic, 0, "ascii");
    oracle[4] = fixture.retainedResolution.checkpointVersion;
    oracle[5] = 9;
    oracle.writeBigUInt64LE(BigInt(fixture.retainedResolution.resumeAfterEntries), 8);
    oracle.writeBigUInt64LE(BigInt(fixture.snapshot.entries.length), 24);
    oracle.write(archiveTextRevision(`${event.nodeId}\0${event.revision}\0${event.value}`), 32, "ascii");
    expect(checkpoint).toEqual(new Uint8Array(oracle));
    expect(new Ajv2020({ strict: false }).compile(checkpointSchema)(Array.from(checkpoint))).toBe(true);
    expect(() => cursor.advance(fixture.snapshot, { ...event, value: "changed while scanning" })).toThrow("stdio.zip.checkpoint-context");
    expect(cursor.checkpoint()).toEqual(checkpoint);
    const resumed = new ArchiveTextCursor();
    resumed.restore(checkpoint);
    expect(resumed.scannedEntries).toBe(fixture.retainedResolution.resumeAfterEntries);
    for (let index = resumed.scannedEntries; index < fixture.snapshot.entries.length; index++) expect(resumed.advance(fixture.snapshot, event)).toBeUndefined();
    const resumedMutations = resumed.advance(fixture.snapshot, event)!;
    expect(resumedMutations).toEqual([{ mutation: "renameEntry", name: fixture.rename.name, newName: fixture.rename.value }]);
    const result = applyZipDiff(fixture.snapshot, parseZipDiff({ entries: { modified: [{ name: fixture.rename.name, diff: { name: fixture.rename.value } }] } }));
    expect(result).toEqual(applyPatch(structuredClone(fixture.snapshot), [{ op: "replace", path: "/entries/0/name", value: fixture.rename.value }]).newDocument);
    const wrongCommand = new ArchiveTextCursor();
    wrongCommand.restore(checkpoint);
    expect(() => wrongCommand.advance(fixture.snapshot, { ...event, value: "different.txt" })).toThrow("stdio.zip.checkpoint-context");
    const wrongSnapshot = new ArchiveTextCursor();
    wrongSnapshot.restore(checkpoint);
    expect(() => wrongSnapshot.advance({ ...fixture.snapshot, entries: [] }, event)).toThrow("stdio.zip.checkpoint-context");
    for (const [index, value] of [[0, 0], [4, 2], [5, 255], [6, 1], [8, 255], [16, 255], [32, 255]]) {
      const invalid = checkpoint.slice();
      invalid[index!] = value!;
      expect(() => new ArchiveTextCursor().restore(invalid)).toThrow("stdio.zip.checkpoint-invalid");
    }
    expect(() => new ArchiveTextCursor().restore(checkpoint.subarray(1))).toThrow("stdio.zip.checkpoint-invalid");
  });
  it("inserts and restores removed members at their exact authored position", () => {
    const edit = fixture.insertion;
    const actual = applyZipDiff(fixture.snapshot, zipInsertionDiff(fixture.snapshot, edit.entry, edit.before));
    const expected = applyPatch(structuredClone(fixture.snapshot), [{ op: "add", path: `/entries/${edit.index}`, value: edit.entry }]).newDocument;
    expect(actual).toEqual(expected);
    for (let index = 0; index < fixture.snapshot.entries.length; index++) {
      const entry = fixture.snapshot.entries[index]!;
      const removed = applyZipDiff(fixture.snapshot, parseZipDiff({ entries: { removed: [entry.name] } }));
      const restored = applyZipDiff(removed, zipInsertionDiff(removed, entry, fixture.snapshot.entries[index + 1]?.name));
      expect(restored).toEqual(fixture.snapshot);
    }
    expect(() => zipInsertionDiff(fixture.snapshot, edit.entry, "missing.txt")).toThrow("mutation.target-missing");
  });
  it("preserves authored member order with the JSON Patch oracle", () => {
    const diff = { entries: { order: fixture.entryOrdering.reversedNames } };
    const validate = new Ajv({ strict: false }).addSchema(snapshotSchema).compile(diffSchema);
    expect(validate(diff)).toBe(true);
    const actual = applyZipDiff(fixture.snapshot, parseZipDiff(diff));
    const expected = applyPatch(structuredClone(fixture.snapshot), [{ op: "move", from: "/entries/0", path: "/entries/1" }]).newDocument;
    expect(actual).toEqual(expected);
    expect(applyZipDiff(actual, parseZipDiff({ entries: { order: fixture.snapshot.entries.map((entry) => entry.name) } }))).toEqual(fixture.snapshot);
    for (const order of fixture.entryOrdering.invalidOrders) expect(() => applyZipDiff(fixture.snapshot, parseZipDiff({ entries: { order } }))).toThrow("mutation.apply.invalid-order");
  });
  it("enforces UTF-8 byte boundaries using the schema and independent Node encoder", () => {
    const validate = archiveSchema();
    for (const row of fixture.textByteBoundaries) {
      const edit = { nodeId: "comment", value: row.text.repeat(row.repeat), revision: archiveTextRevision(fixture.snapshot.comment) };
      expect(validate(edit)).toBe(row.valid);
      if (row.valid) expect(editArchiveText(fixture.snapshot, edit)).toEqual([{ mutation: "setArchiveComment", comment: edit.value, commentUtf8: true }]);
      else expect(() => editArchiveText(fixture.snapshot, edit)).toThrow("stdio.zip.text-too-large");
    }
  });
  it("refuses invalid byte values consistently across snapshot and sparse diff schemas", () => {
    const validateSnapshot = new Ajv({ strict: false }).compile(snapshotSchema);
    const validateDiff = new Ajv({ strict: false }).addSchema(snapshotSchema).compile(diffSchema);
    for (const byte of fixture.invalidPayloadBytes) {
      const snapshot = { ...fixture.snapshot, entries: [{ name: "invalid.bin", data: [byte] }] };
      const diff = { entries: { modified: [{ name: fixture.rename.name, diff: { data: [byte] } }] } };
      expect(validateSnapshot(snapshot)).toBe(false);
      expect(validateDiff(diff)).toBe(false);
      expect(() => parseZipSnapshot(snapshot)).toThrow();
      expect(() => parseZipDiff(diff)).toThrow();
    }
    expect(parseZipDiff({ entries: { added: [{ name: "empty.txt" }] } }).entries!.added[0]!.data).toEqual([]);
  });
  it("supplies the shared empty archive defaults before editing", () => {
    expect(parseZipSnapshot(fixture.omittedDefaults)).toEqual(fixture.defaultSnapshot);
  });
  it("retains complete entry header state and refuses invalid compression metadata", () => {
    expect(parseZipSnapshot(fixture.snapshot)).toEqual(fixture.snapshot);
    const changed = structuredClone(fixture.snapshot);
    changed.entries[0]!.metadata.compressionMethod = 65536;
    expect(() => parseZipSnapshot(changed)).toThrow("compressionMethod");
    const invalidExtra = structuredClone(fixture.snapshot);
    invalidExtra.entries[0]!.metadata.local.extraFields.push({ id: 0xcafe, data: [256] });
    expect(() => parseZipSnapshot(invalidExtra)).toThrow("extraFields");
  });
  it("matches the neutral reordered rename with an independent JSON Patch oracle", () => {
    const snapshot = structuredClone(fixture.snapshot);
    snapshot.entries.reverse();
    const edit = { nodeId: archiveEntryNodeId(fixture.rename.name), value: fixture.rename.value, revision: archiveTextRevision(fixture.rename.name) };
    expect(archiveSchema()(edit)).toBe(true);
    const mutations = editArchiveText(snapshot, edit);
    expect(mutations).toEqual([{ mutation: "renameEntry", name: fixture.rename.name, newName: fixture.rename.value }]);
    const expected = applyPatch(structuredClone(snapshot), [{ op: "replace", path: "/entries/1/name", value: fixture.rename.value }]).newDocument;
    expect(expected.entries[0]).toEqual(fixture.snapshot.entries[1]);
    expect(expected.entries[1]).toEqual({ ...fixture.snapshot.entries[0], name: fixture.rename.value });
    expect(() => editArchiveText(expected, edit)).toThrow("stdio.zip.target-missing");
  });

  it("rejects incomplete canonical arguments through the command schema", () => {
    const validate = archiveSchema();
    for (const args of fixture.rejectedArguments) expect(validate(args)).toBe(false);
  });

  it("preserves comments, refuses ambiguous names, and accepts unchanged drafts", () => {
    const snapshot = structuredClone(fixture.snapshot);
    const edit = { nodeId: archiveEntryNodeId(fixture.rename.name), value: fixture.rename.value, revision: archiveTextRevision(fixture.rename.name) };
    for (const nodeId of fixture.rejectedTargets) expect(() => editArchiveText(snapshot, { ...edit, nodeId })).toThrow("stdio.zip.target-missing");
    expect(() => editArchiveText(snapshot, { ...edit, value: snapshot.entries[1]!.name })).toThrow("stdio.zip.name-exists");
    expect(() => editArchiveText(snapshot, { ...edit, value: "" })).toThrow("stdio.zip.name-required");
    expect(() => editArchiveText(snapshot, { ...edit, value: "ä".repeat(Math.floor(ZIP_MAXIMUM_TEXT_BYTES / 2) + 1) })).toThrow("stdio.zip.text-too-large");
    expect(editArchiveText(snapshot, { ...edit, value: fixture.rename.name })).toEqual([]);
    snapshot.entries.push({ ...snapshot.entries[0]! });
    expect(() => editArchiveText(snapshot, edit)).toThrow("stdio.zip.target-ambiguous");
    const comment = { nodeId: "comment", value: "", revision: archiveTextRevision(snapshot.comment) };
    expect(editArchiveText(snapshot, comment)).toEqual([{ mutation: "setArchiveComment", comment: "", commentUtf8: true }]);
    expect(() => editArchiveText({ ...snapshot, comment: "Changed remotely" }, comment)).toThrow("stdio.zip.draft-conflict");
  });

  it("promotes a CP437 archive comment atomically when the replacement needs UTF-8", () => {
    const snapshot = { ...structuredClone(fixture.snapshot), comment: "é", commentUtf8: false };
    const event = { nodeId: "comment", value: "edited 🎒", revision: archiveTextRevision(snapshot.comment) };
    expect(editArchiveText(snapshot, event)).toEqual([{ mutation: "setArchiveComment", comment: event.value, commentUtf8: true }]);
    expect(archiveCommentUtf8AfterEdit(false, "é")).toBe(false);
    expect(archiveCommentUtf8AfterEdit(false, "plain ASCII")).toBe(true);
  });

  it("requires the atomic archive-comment encoding in the schema", () => {
    const validate = new Ajv({ strict: false }).compile(commentMutationSchema);
    expect(validate({ mutation: "setArchiveComment", comment: "edited 🎒", commentUtf8: true })).toBe(true);
    expect(validate({ mutation: "setArchiveComment", comment: "edited 🎒" })).toBe(false);
  });
});
