import { fileURLToPath as testFileUrlToPath } from "node:url";
const testSourceUrl = new URL("../../📜️script.ts", import.meta.url);
/** 🏪️ Group read/cursor visibility oracles: schema-owned contracts plus an independent decision model. */
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import Ajv from "ajv";

//#region 📏️GroupVisibilityOracle
const read = (path: string) => JSON.parse(readFileSync(new URL(path, testSourceUrl.href), "utf8"));

/** 📖️ Proves the group read and cursor fixtures against `os.store` and an independent capture model. */
export function testGroupVisibilityFixtures(): void {
  const contract = read("./🧬️schema/🔣️.json");
  const ajv = new Ajv({ strict: true, allErrors: true });
  ajv.addSchema(contract);
  const validateRead = ajv.getSchema(`${contract.$id}#/$defs/GroupReadVisibility`)!;
  const validateCursor = ajv.getSchema(`${contract.$id}#/$defs/GroupCursorVisibility`)!;

  const groupRead = read("./🧫️fixtures/📖️group-read.json");
  assert(validateRead(groupRead), JSON.stringify(validateRead.errors));
  assert.deepEqual([groupRead.maximumItems, groupRead.maximumBytes], [1, 4096]);
  assert.equal(groupRead.cases.length, 4);
  assert.deepEqual(groupRead.cases.map((row: { id: string }) => row.id), [
    "pending-keeps-live-roots",
    "commit-switches-fresh-readers",
    "post-commit-capture-is-prepared",
    "abort-keeps-live-roots",
  ]);
  for (const row of groupRead.cases) {
    const fresh = row.decision === "committed" ? "prepared" : "old";
    assert.equal(row.fresh, fresh, `${row.id} fresh root follows the decision`);
    assert.equal(row.captured, row.capture === "committed" ? "prepared" : "old", `${row.id} capture is stable`);
    assert.equal(row.oldLeaseCurrent, fresh === "old", `${row.id} old lease`);
    assert.equal(row.preparedLeaseCurrent, fresh === "prepared", `${row.id} prepared lease`);
    const captured = groupRead[row.captured];
    assert.deepEqual(captured.appliedEditIds, captured.history, `${row.id} capture is one coherent envelope`);
  }
  assert(groupRead.prepared.generation > groupRead.old.generation, "prepared advances the generation");
  assert.notEqual(groupRead.prepared.revisionByte, groupRead.old.revisionByte);
  assert(Buffer.byteLength(JSON.stringify(groupRead.prepared)) <= groupRead.maximumBytes);

  const groupCursor = read("./🧫️fixtures/🎯️group-cursor.json");
  assert(validateCursor(groupCursor), JSON.stringify(validateCursor.errors));
  assert.deepEqual([groupCursor.maximumItems, groupCursor.maximumBytes], [1, 4096]);
  assert.deepEqual(groupCursor.laws, [
    "cursor-and-history-use-the-same-decision",
    "private-old-root-remains-owned-until-retirement",
    "foreign-decision-is-rejected",
    "cancel-returns-exact-unpublished-root",
    "one-item-byte-bounded-close",
  ]);
  assert.equal(groupCursor.before.checkpointId, groupCursor.after.checkpointId, "one checkpoint spans the group");
  assert.deepEqual(groupCursor.after.appliedEditIds.slice(0, groupCursor.before.appliedEditIds.length), groupCursor.before.appliedEditIds);
  assert.equal(groupCursor.after.redoEditIds, undefined, "publishing clears the redo cursor");
  assert(Buffer.byteLength(JSON.stringify(groupCursor.after)) <= groupCursor.maximumBytes);

  for (const hostile of [
    { ...groupRead, unexpected: true },
    { ...groupRead, cases: [{ ...groupRead.cases[0], decision: "unknown" }] },
    { ...groupRead, old: { ...groupRead.old, revisionByte: 256 } },
  ]) assert.equal(validateRead(hostile), false, "group read hostile is refused");
  for (const hostile of [
    { ...groupCursor, unexpected: true },
    { ...groupCursor, before: { ...groupCursor.before, unexpected: true } },
    { ...groupCursor, laws: [1] },
  ]) assert.equal(validateCursor(hostile), false, "group cursor hostile is refused");

  console.log(`[DEBUG] group visibility oracle: AJV=2 read-cases=${groupRead.cases.length} laws=${groupCursor.laws.length} hostiles=6; native retained-group execution remains a separate gate`);
}
//#endregion 📏️GroupVisibilityOracle
