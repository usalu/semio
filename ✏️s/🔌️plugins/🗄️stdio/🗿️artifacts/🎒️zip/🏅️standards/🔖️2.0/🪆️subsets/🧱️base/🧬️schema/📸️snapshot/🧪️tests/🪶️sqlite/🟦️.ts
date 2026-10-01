import { Database } from "bun:sqlite";
import { expect, test } from "bun:test";
import fixture from "../../🧫️fixtures/🪶️sqlite/🔣️.json";
import { parseZipSnapshot } from "../../🟦️.ts";
import { ZIP_SQLITE_SCHEMA, zipSnapshotToSqliteDatabase, zipSnapshotFromSqliteDatabase, zipSnapshotValidateSqliteSubset } from "../../🪶️sqlite/🟦️.ts";
import { exportSqliteDatabase, importSqliteDatabase } from "@semio-tech/framework";

test("ZIP full member headers and decompressed data expose shared editable relationships", async () => {
  expect(ZIP_SQLITE_SCHEMA).toBe(await Bun.file(new URL("../../🪶️sqlite/🗄️.sql", import.meta.url)).text());
  const snapshot = parseZipSnapshot(fixture);
  const database = await zipSnapshotToSqliteDatabase(snapshot);
  expect(await zipSnapshotFromSqliteDatabase(database)).toEqual(snapshot);
  const db = Database.deserialize(await exportSqliteDatabase(database));
  try {
    expect(db.query("PRAGMA integrity_check").get()).toEqual({ integrity_check: "ok" });
    expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);
    expect(db.query("SELECT e.name,h.external_attributes,f.tag FROM zip_entry e JOIN zip_central_header h ON h.id=e.id JOIN zip_central_extra_field f ON f.header_id=h.id ORDER BY e.ordinal,f.ordinal").all()).toEqual([{name:snapshot.entries[0]!.name,external_attributes:4294967295,tag:25461},{name:snapshot.entries[0]!.name,external_attributes:4294967295,tag:51966}]);
    db.run("UPDATE zip_entry_byte SET value=42 WHERE entry_id=1 AND ordinal=2");
    db.run("UPDATE zip_central_header SET comment='SQLite Kommentar' WHERE id=1");
    snapshot.entries[0]!.data[2] = 42;
    snapshot.entries[0]!.metadata.central.comment = "SQLite Kommentar";
    expect(await zipSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())))).toEqual(snapshot);
  } finally { db.close(); }
});

test("ZIP header ownership, legacy option presence, widths and budgets reject invalid rows", async () => {
  const snapshot = parseZipSnapshot(fixture);
  const database = await zipSnapshotToSqliteDatabase(snapshot);
  await expect(zipSnapshotToSqliteDatabase(snapshot, { maxValueBytes: 0 })).rejects.toThrow();
  await expect(zipSnapshotFromSqliteDatabase(database, { maxValueBytes: 0 })).rejects.toThrow();
  for (const alteration of ["DELETE FROM zip_local_header WHERE id=2", "UPDATE zip_entry_byte SET entry_id=999 WHERE ordinal=0", "UPDATE zip_entry SET ordinal=0 WHERE ordinal=1", "UPDATE zip_central_header SET legacy_comment_present=0 WHERE id=1", "UPDATE zip_local_extra_field_byte SET value=256 WHERE ordinal=0"]) {
    const db = Database.deserialize(await exportSqliteDatabase(database));
    try { db.run("PRAGMA ignore_check_constraints=ON"); db.run(alteration); await expect(zipSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())))).rejects.toThrow(); } finally { db.close(); }
  }
  const controller = new AbortController();
  snapshot.entries[0]!.data = new Array<number>(2000).fill(1);
  await expect(zipSnapshotToSqliteDatabase(snapshot, { signal: controller.signal, onProgress: event => { if (event.completed >= 256) controller.abort(); } })).rejects.toHaveProperty("name", "AbortError");
});

import subsetFixture from "../../🧫️fixtures/🪶️sqlite/🚦️subsets.json";

test("ZIP exact named snapshot guards use full independent local and central metadata", async () => {
  for (const sample of subsetFixture.cases) {
    const snapshot = parseZipSnapshot(fixture);
    snapshot.entries = [snapshot.entries[1]!];
    const entry = snapshot.entries[0]!;
    entry.name = subsetFixture.entryName;
    entry.metadata.compressionMethod=sample.compressionMethod;
    entry.metadata.local.flags = sample.localFlags; entry.metadata.central.flags = sample.centralFlags;
    entry.metadata.local.versionNeeded = sample.localVersion; entry.metadata.central.versionNeeded = sample.centralVersion;
    entry.metadata.dataDescriptorSignature = true;
    const database = await zipSnapshotToSqliteDatabase(snapshot);
    expect((await zipSnapshotValidateSqliteSubset(snapshot, subsetFixture.acceptedDialects[1]!, database)).map(({code,severity})=>({code,severity:String(severity)}))).toEqual(sample.diagnostics);
    expect(await zipSnapshotValidateSqliteSubset(snapshot, subsetFixture.acceptedDialects[0]!, database)).toEqual([]);
    for (const dialect of subsetFixture.rejectedDialects) await expect(zipSnapshotValidateSqliteSubset(snapshot,dialect,database)).rejects.toThrow("dialect");
  }
});

test("ZIP independently edited relational header flags reach the named semantic guard", async () => {
  const snapshot = parseZipSnapshot(fixture); snapshot.entries=[snapshot.entries[1]!];
  const db = Database.deserialize(await exportSqliteDatabase(await zipSnapshotToSqliteDatabase(snapshot)));
  try {
    db.run("UPDATE zip_local_header SET flags=1 WHERE id=1");
    expect(db.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});
    expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);
    const database = await importSqliteDatabase(new Uint8Array(db.serialize()));
    const restored = await zipSnapshotFromSqliteDatabase(database);
    const diagnostics = await zipSnapshotValidateSqliteSubset(restored,subsetFixture.acceptedDialects[1]!,database);
    expect(diagnostics.map(({code,severity})=>({code,severity:String(severity)}))).toEqual(subsetFixture.cases[1]!.diagnostics);
    expect(diagnostics[0]!.message).toContain("entry 0");
    await expect(zipSnapshotValidateSqliteSubset({...restored,schema:"wrong"},subsetFixture.acceptedDialects[1]!,database)).rejects.toThrow("identity");
  } finally {db.close();}
});

test("ZIP named metadata traversal can cancel before scanning all members", async () => {
  const snapshot = parseZipSnapshot(fixture);snapshot.entries=new Array(600).fill(snapshot.entries[1]!);
  const database = await zipSnapshotToSqliteDatabase(snapshot);
  const controller = new AbortController();let reached=false;
  await expect(zipSnapshotValidateSqliteSubset(snapshot,subsetFixture.acceptedDialects[1]!,database,{signal:controller.signal,onProgress:event=>{if(event.completed===256){reached=true;controller.abort();}}})).rejects.toHaveProperty("name","AbortError");
  expect(reached).toBe(true);
});

test("ZIP wildcard snapshots retain the full native unsigned16 method domain",async()=>{
 for(const method of subsetFixture.methodCodes){
  const raw=structuredClone(fixture);raw.entries[0]!.metadata.compressionMethod=method;const snapshot=parseZipSnapshot(raw);
  const database=await zipSnapshotToSqliteDatabase(snapshot);const db=Database.deserialize(await exportSqliteDatabase(database));
  try{expect(db.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(db.query("SELECT compression_method FROM zip_entry WHERE id=1").get()).toEqual({compression_method:method});db.run("UPDATE zip_entry SET compression_method=65535 WHERE id=1");const edited=await importSqliteDatabase(new Uint8Array(db.serialize()));const restored=await zipSnapshotFromSqliteDatabase(edited);expect(restored.entries[0]!.metadata.compressionMethod).toBe(65535);expect(await zipSnapshotValidateSqliteSubset(restored,subsetFixture.acceptedDialects[0]!,edited)).toEqual([]);expect((await zipSnapshotValidateSqliteSubset(restored,subsetFixture.acceptedDialects[1]!,edited)).some(d=>d.code==="stdio.zip.iso21320.compression-method-unsupported"&&d.severity==="Error")).toBe(true);}finally{db.close();}
 }
 for(const method of subsetFixture.invalidMethodCodes){const raw=structuredClone(fixture);raw.entries[0]!.metadata.compressionMethod=method;expect(()=>parseZipSnapshot(raw)).toThrow("compressionMethod");}
});
