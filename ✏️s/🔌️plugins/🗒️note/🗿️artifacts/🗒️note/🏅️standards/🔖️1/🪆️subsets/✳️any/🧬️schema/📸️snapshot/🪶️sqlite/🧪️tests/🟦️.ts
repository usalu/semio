/** 🗒️ Language-neutral note graph and independent SQLite interoperability laws. */
import { expect, test } from "bun:test";
import { Database } from "bun:sqlite";
import { noteSnapshotToSqliteDatabase, noteSnapshotFromSqliteDatabase, NOTE_SQLITE_SCHEMA } from "../🟦️.ts";
import { exportSqliteDatabase, importSqliteDatabase } from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
import { binary64 } from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔢️ieee754/🟦️.ts";
import { parseNoteSnapshot, type NoteSnapshot } from "../../🟦️.ts";

async function fixture(): Promise<NoteSnapshot> {
  const value = await Bun.file(new URL("../🧫️fixtures/🔣️.json", import.meta.url)).json();
  value.linkedArtifact.pin.blob.size = BigInt(value.linkedArtifact.pin.blob.size);
  return parseNoteSnapshot(value);
}

test("Note complete concrete backing contract retains owned diagnostics and full independent state", async () => {
  const plan = await Bun.file(new URL("../🧫️fixtures/🔢️ieee.json", import.meta.url)).json();
  expect(plan.backing).toEqual({ authority: "completeSystemAllocatorRequests", phases: ["projectSnapshot", "reconstructSnapshot"], ceilings: ["zero", "exact", "oneBelow", "cumulative"], cancellation: ["start", "materializedInterior"], diagnosticOwnership: "actualCapacity", retirementRefund: false });
  const { default: Ajv } = await import("ajv/dist/2020");
  const schema = await Bun.file(new URL("../🧫️fixtures/💰️backing/🧬️schema/🔣️.json", import.meta.url)).json();
  const validate = new Ajv({ strict: true }).compile(schema);
  expect(validate(plan.backing)).toBe(true);
  for (const invalid of [{ ...plan.backing, extra: true }, { ...plan.backing, retirementRefund: true }, { ...plan.backing, authority: "estimatedSlots" }, { ...plan.backing, cancellation: ["start"] }]) expect(validate(invalid)).toBe(false);
  const expected = await fixture();
  const bytes = await exportSqliteDatabase(await noteSnapshotToSqliteDatabase(expected));
  const sql = Database.deserialize(bytes, { safeIntegers: true });
  try {
    expect(sql.query("PRAGMA integrity_check").get()).toEqual({ integrity_check: "ok" });
    expect(sql.query("PRAGMA foreign_key_check").all()).toEqual([]);
    expect(sql.query("SELECT COUNT(DISTINCT kind) AS kinds FROM note_block").get()).toEqual({ kinds: 6n });
    expect(sql.query("SELECT COUNT(*) AS count FROM note_cell").get()).toEqual({ count: 3n });
    expect(await noteSnapshotFromSqliteDatabase(await importSqliteDatabase(sql.serialize()))).toEqual(expected);
  } finally { sql.close(); }
});

test("Note preserves every authored entity and permits relational text edits", async () => {
  expect(NOTE_SQLITE_SCHEMA).toBe(await Bun.file(new URL("../🗄️.sql", import.meta.url)).text());
  const snapshot = await fixture();
  const projected = await noteSnapshotToSqliteDatabase(snapshot);
  const bytes = await exportSqliteDatabase(projected);
  const sql = Database.deserialize(bytes);
  expect(sql.query("PRAGMA integrity_check").get()).toEqual({ integrity_check: "ok" });
  expect(sql.query("PRAGMA foreign_key_check").all()).toEqual([]);
  expect(sql.query("SELECT COUNT(*) AS count FROM note_block").get()).toEqual({ count: 8 });
  expect(sql.query("SELECT COUNT(*) AS count FROM note_cell").get()).toEqual({ count: 3 });
  expect(await noteSnapshotFromSqliteDatabase(await importSqliteDatabase(bytes))).toEqual(snapshot);
  sql.run("UPDATE note_run SET text='Relational edit 🌠' WHERE id=1");
  const restored = await noteSnapshotFromSqliteDatabase(await importSqliteDatabase(sql.serialize()));
  const group = restored.blocks[0]!;
  expect(group.kind).toBe("group");
  if (group.kind === "group" && group.children[0]?.kind === "text") expect(group.children[0].content.paragraphs[1]!.runs[0]!.text).toBe("Relational edit 🌠");
  sql.close();
});

test("Note owns complete binary64 state and unsigned blob pin sizes", async () => {
  const plan = await Bun.file(new URL("../🧫️fixtures/🔢️ieee.json", import.meta.url)).json();
  for (const item of plan.binary64) {
    const snapshot = await fixture();
    const value = { bits: BigInt(`0x${item.bits}`) };
    snapshot.gridSpacing = value;
    snapshot.gridSubdivisions = null;
    snapshot.assets!["image-1"]!.width = value;
    snapshot.blocks[0]!.x = value;
    if (snapshot.linkedArtifact?.pin.kind === "snapshot") snapshot.linkedArtifact.pin.blob.size = 18446744073709551615n;
    const bytes = await exportSqliteDatabase(await noteSnapshotToSqliteDatabase(snapshot));
    const restored = await noteSnapshotFromSqliteDatabase(await importSqliteDatabase(bytes));
    expect(restored).toEqual(snapshot);
    const sql = Database.deserialize(bytes);
    expect(sql.query("SELECT CAST(grid_spacing_bits AS TEXT) AS bits,grid_spacing_class AS class,grid_spacing IS NULL AS nullQuery FROM note_document").get()).toEqual({ bits: BigInt.asIntN(64,value.bits).toString(), class: item.class, nullQuery: item.class === "nan" ? 1 : 0 });
    sql.close();
  }
});

test("Note bounds work and rejects orphaned, reordered and cyclic entities", async () => {
  const snapshot = await fixture();
  await expect(noteSnapshotToSqliteDatabase(snapshot, { maxRows: 1 })).rejects.toThrow();
  await expect(noteSnapshotToSqliteDatabase(snapshot, { maxValueBytes: 1 })).rejects.toThrow();
  const controller = new AbortController();
  controller.abort();
  await expect(noteSnapshotToSqliteDatabase(snapshot, { signal: controller.signal })).rejects.toThrow();
  const valid = await noteSnapshotToSqliteDatabase(snapshot);
  for (const [name, column, value] of [["note_run", 1, 999n], ["note_point", 2, 4n], ["note_block", 2, 1n]] as const) {
    const db = { tables: valid.tables.map(table => table.name === name ? { ...table, rows: table.rows.map((row, index) => index === 0 ? { ...row, values: row.values.map((cell, index) => index === column ? value : cell) } : row) } : table) };
    await expect(noteSnapshotFromSqliteDatabase(db)).rejects.toThrow();
  }
});

test("Note iteratively reconstructs the neutral deep containment fixture", async () => {
  const plan = await Bun.file(new URL("../🧫️fixtures/🌲️deep.json", import.meta.url)).json();
  const snapshot = await fixture();
  let block: NoteSnapshot["blocks"][number] = { kind: "math", id: plan.leafId, name: "leaf", x: binary64(0), y: binary64(0), width: binary64(10), height: binary64(10), rotation: binary64(0), visible: true, locked: false, tex: plan.leafText, displayMode: true };
  for (let i = 0; i < plan.depth; i++) block = { kind: "group", id: `group-${i}`, name: "group", x: binary64(0), y: binary64(0), width: binary64(10), height: binary64(10), rotation: binary64(0), visible: true, locked: false, children: [block] };
  snapshot.blocks = [block];
  const bytes = await exportSqliteDatabase(await noteSnapshotToSqliteDatabase(snapshot));
  const sql = Database.deserialize(bytes);
  expect(sql.query("WITH RECURSIVE groups(id,depth) AS (SELECT id,0 FROM note_block WHERE parent_id IS NULL UNION ALL SELECT b.id,g.depth+1 FROM note_block b JOIN groups g ON b.parent_id=g.id) SELECT COUNT(*) AS count,MAX(depth) AS depth FROM groups").get()).toEqual({ count: plan.expectedBlocks, depth: plan.depth });
  sql.close();
  const restored = await noteSnapshotFromSqliteDatabase(await importSqliteDatabase(bytes));
  let node = restored.blocks[0]!;
  for (let i = 0; i < plan.depth; i++) { if (node.kind !== "group") throw new Error("expected group"); expect(node.children.length).toBe(1); node = node.children[0]!; }
  expect(node.id).toBe(plan.leafId);
});


test("Note direct domain rows use their exact allowance without file metadata",async()=>{
  const {default:Ajv}=await import("ajv/dist/2020");const plan=await Bun.file(new URL("../🧫️fixtures/🚦️control.json",import.meta.url)).json();const schema=await Bun.file(new URL("../🧫️fixtures/🚦️control.schema.json",import.meta.url)).json();expect(new Ajv({strict:true}).validate(schema,plan)).toBe(true);
  const snapshot=await fixture();snapshot.blocks=[];snapshot.assets={};snapshot.linkedArtifact=null;snapshot.title="x".repeat(plan.longTextBytes);
  const database=await noteSnapshotToSqliteDatabase(snapshot,{maxRows:plan.directRows});expect(database.tables.reduce((count,table)=>count+table.rows.length,0)).toBe(plan.directRows);expect(await noteSnapshotFromSqliteDatabase(database,{maxRows:plan.directRows})).toEqual(snapshot);
  const sql=Database.deserialize(await exportSqliteDatabase(database,{maxRows:plan.directRows}));try{expect(sql.query("SELECT COUNT(*) AS count,LENGTH(title) AS length FROM note_document").get()).toEqual({count:plan.directRows,length:plan.longTextBytes});}finally{sql.close();}
});
