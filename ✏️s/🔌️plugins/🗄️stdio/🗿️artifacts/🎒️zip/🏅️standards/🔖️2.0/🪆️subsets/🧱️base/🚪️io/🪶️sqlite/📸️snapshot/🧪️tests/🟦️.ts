import refusalFixture from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/⚠️refusal/🧫️fixtures/🔣️.json";
const canceledKind=refusalFixture.cases.find(item=>item.id==="canceled-projection")!.expectedKind;
import { Database } from "bun:sqlite";
import { expect, test } from "bun:test";
import fixture from "../🧫️fixtures/🔣️.json";
import { parseZipSnapshot } from "../../../../🧬️schema/📸️snapshot/🟦️.ts";
import { ZIP_SQLITE_SCHEMA, zipSnapshotToSqliteDatabase, zipSnapshotFromSqliteDatabase, zipSnapshotValidateSqliteSubset } from "../🟦️.ts";
import { exportSqliteDatabase, importSqliteDatabase } from "@semio-tech/framework";

test("ZIP full member headers and decompressed data expose shared editable relationships", async () => {
  expect(ZIP_SQLITE_SCHEMA).toBe(await Bun.file(new URL("../🗄️.sql", import.meta.url)).text());
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
  await expect(zipSnapshotToSqliteDatabase(snapshot, { signal: controller.signal, onProgress: event => { if (event.completed >= 256) controller.abort(); } })).rejects.toHaveProperty("kind",canceledKind);
});

import subsetFixture from "../🧫️fixtures/🚦️subsets.json";

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
  await expect(zipSnapshotValidateSqliteSubset(snapshot,subsetFixture.acceptedDialects[1]!,database,{signal:controller.signal,onProgress:event=>{if(event.completed===256){reached=true;controller.abort();}}})).rejects.toHaveProperty("kind",canceledKind);
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

import Ajv2020 from "ajv/dist/2020.js";
import Ajv from "ajv";
import structuredPatches from "../../../../🧬️schema/🧬️mutations/🧫️fixtures/🩹️structured/🔣️.json";

import nativeControl from "../🧫️fixtures/🚦️native.json";


test("ZIP native control corpus matches independent exact semantic row admission",async()=>{
  expect(nativeControl["schema"]).toEqual("zip.native.control/v1");expect(nativeControl["copyBytes"]).toEqual(100000);expect(nativeControl["copyCancelAfter"]).toEqual(65536);expect(nativeControl["collectionItems"]).toEqual(1024);expect(nativeControl["cancelAfter"]).toEqual(256);expect(nativeControl["maximumBytes"]).toEqual(16777216);expect(nativeControl["tinyBytes"]).toEqual(1024);expect(nativeControl["fixtureRows"]).toEqual(30);
  const snapshot=parseZipSnapshot(fixture),database=await zipSnapshotToSqliteDatabase(snapshot,{maxRows:nativeControl.fixtureRows});
  expect(database.tables.reduce((n,t)=>n+t.rows.length,0)).toBe(nativeControl.fixtureRows);
  await expect(zipSnapshotToSqliteDatabase(snapshot,{maxRows:nativeControl.fixtureRows-1})).rejects.toThrow();
  const independent=Database.deserialize(await exportSqliteDatabase(database));
  try{
    const actual=independent.query("SELECT (SELECT count(*) FROM zip_archive)+(SELECT count(*) FROM zip_entry)+(SELECT count(*) FROM zip_entry_byte)+(SELECT count(*) FROM zip_local_header)+(SELECT count(*) FROM zip_local_extra_field)+(SELECT count(*) FROM zip_local_extra_field_byte)+(SELECT count(*) FROM zip_local_legacy_name_byte)+(SELECT count(*) FROM zip_central_header)+(SELECT count(*) FROM zip_central_extra_field)+(SELECT count(*) FROM zip_central_extra_field_byte)+(SELECT count(*) FROM zip_central_legacy_name_byte)+(SELECT count(*) FROM zip_central_legacy_comment_byte) AS n").get() as {n:number};
    expect(actual.n).toBe(nativeControl.fixtureRows);
    expect(independent.query("PRAGMA foreign_key_check").all()).toEqual([]);
  }finally{independent.close();}
});

test("ZIP structured mutation corpus preserves operation fields and optional presence independently",()=>{
  
  
  const db=new Database(":memory:");
  try{
    db.run("CREATE TABLE patch_case (ordinal INTEGER PRIMARY KEY, patch TEXT NOT NULL, source TEXT NOT NULL)");
    const insert=db.prepare("INSERT INTO patch_case VALUES (?, ?, ?)");
    for(const [ordinal,sample] of structuredPatches.cases.entries()) insert.run(ordinal,JSON.stringify(sample.patch),sample.source);
    const actual=db.query("SELECT ordinal,json_extract(patch,'$.operation') AS operation,json_extract(patch,'$.path') AS path,json_type(patch,'$.index') AS index_type,json_extract(patch,'$.index') AS item_index,json_extract(patch,'$.from') AS source_path,json_extract(patch,'$.key') AS member_key,json_extract(patch,'$.value') AS value,source FROM patch_case ORDER BY ordinal").all();
    expect(actual).toEqual([
      {ordinal:0,operation:"set",path:"/schema",index_type:null,item_index:null,source_path:null,member_key:null,value:"Grüße",source:structuredPatches.cases[0]!.source},
      {ordinal:1,operation:"insert",path:"/entries/0",index_type:"integer",item_index:2,source_path:null,member_key:null,value:'{"name":"a","data":[1,2]}',source:structuredPatches.cases[1]!.source},
      {ordinal:2,operation:"remove",path:"/entries/1",index_type:null,item_index:null,source_path:null,member_key:null,value:null,source:structuredPatches.cases[2]!.source},
      {ordinal:3,operation:"move",path:"/entries/0",index_type:"integer",item_index:1,source_path:"/entries/1",member_key:null,value:null,source:structuredPatches.cases[3]!.source},
      {ordinal:4,operation:"rename",path:"/metadata",index_type:null,item_index:null,source_path:null,member_key:"with space",value:null,source:structuredPatches.cases[4]!.source},
    ]);
    for(const patch of [{operation:"set",path:"/schema"},{operation:"remove",path:"/entries/1",value:1},{operation:"move",path:"/entries/0",from:"/entries/1",index:-1}]){
      const invalid=structuredClone(structuredPatches);
      invalid.cases[0]!.patch=patch as typeof invalid.cases[0]["patch"];
      
    }
  }finally{db.close();}
});

import NormSemanticAjv from "ajv/dist/2020.js";
import normSemanticContract from "../🧫️fixtures/🎛️semantic.json";

import {zipSnapshotToSqliteDatabase as normSemanticProject,zipSnapshotFromSqliteDatabase as normSemanticRestore} from "../🟦️.ts";
import {exportSqliteDatabase as normSemanticExport,importSqliteDatabase as normSemanticImport} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
import {independentSqliteExtent as normIndependentExtent} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧪️tests/🔮️semantic-extent/🟦️.ts";
test("ZIP complete independent cells preserve copied limits and required empty metadata",async()=>{
 expect(normSemanticContract["schemaBytes"]).toEqual(3963);expect(normSemanticContract["tableWidths"]).toEqual({"zip_archive":4,"zip_central_extra_field":4,"zip_central_extra_field_byte":4,"zip_central_header":11,"zip_central_legacy_comment_byte":4,"zip_central_legacy_name_byte":4,"zip_entry":6,"zip_entry_byte":4,"zip_local_extra_field":4,"zip_local_extra_field_byte":4,"zip_local_header":6,"zip_local_legacy_name_byte":4});expect(normSemanticContract["cases"]).toEqual([{"id":"full","rows":30,"valueBytes":1151},{"id":"emptyEntries","rows":1,"valueBytes":36},{"id":"absentEntryMetadata","rows":11,"valueBytes":528}]);
 for(const item of normSemanticContract.cases){const source=parseZipSnapshot(item.id==="absentEntryMetadata"?{...fixture,entries:fixture.entries.map(({metadata,...entry})=>entry)}:fixture);if(item.id==="emptyEntries"){source.entries=[];}const database=await normSemanticProject(source),bytes=await normSemanticExport(database);expect(normIndependentExtent(bytes)).toEqual({rows:item.rows,valueBytes:item.valueBytes,schemaBytes:normSemanticContract.schemaBytes,tableWidths:normSemanticContract.tableWidths});const limits={maxRows:item.rows,maxValueBytes:item.valueBytes,maxSchemaBytes:normSemanticContract.schemaBytes,maxTables:12,maxColumns:11};expect(await normSemanticProject(source,limits)).toEqual(database);expect(await normSemanticRestore(await normSemanticImport(bytes),limits)).toEqual(source);
  for(const short of[{...limits,maxRows:item.rows-1},{...limits,maxValueBytes:item.valueBytes-1},{...limits,maxSchemaBytes:limits.maxSchemaBytes-1},{...limits,maxTables:11},{...limits,maxColumns:10}]){await expect(normSemanticProject(source,short)).rejects.toThrow();await expect(normSemanticRestore(database,short)).rejects.toThrow();}
 }console.log("[DEBUG] ZIP complete independent SQLite cells and metadata preserve full and empty semantic limits");
});
