import { expect,test } from "bun:test";
import { Database } from "bun:sqlite";
import fixture from "../../../🧫️fixtures/🪶️sqlite/📄️document/🔣️.json";
import { DWG_SQLITE_SCHEMA } from "../../../🪶️sqlite/🟦️.ts";
import { dwgProjectDocument,dwgReconstructDocument,type DwgDocumentState } from "../../../🪶️sqlite/📄️document/🟦️.ts";
import { DwgProjection } from "../../../🪶️sqlite/🔢️number/🟦️.ts";
import { DwgReader } from "../../../🪶️sqlite/🫳️reader/🟦️.ts";
import { exportSqliteDatabase,importSqliteDatabase,type SqliteDatabase } from "../../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";

import { dwgDocumentFixture as input } from "../../../🧫️fixtures/🪶️sqlite/📄️document/🟦️.ts";
async function project(value:DwgDocumentState,options={}){const p=await DwgProjection.create(DWG_SQLITE_SCHEMA,options);await dwgProjectDocument(p,value);return p.finish();}
async function reconstruct(database:SqliteDatabase){const r=await DwgReader.create(database,DWG_SQLITE_SCHEMA);const result=await dwgReconstructDocument(r);await r.finish();return result;}

test("DWG actual native document entities preserve all fields, duplicates and full unsigned64",async()=>{
  const oracle=Database.deserialize(await exportSqliteDatabase(await project(input)));
  try{
    expect(oracle.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(oracle.query("PRAGMA foreign_key_check").all()).toEqual([]);
    expect(oracle.query(fixture.query).get()).toEqual({schema:fixture.schema,version:fixture.version,title:fixture.summaryTitle,total_editing_time_high:4294967295,total_editing_time_low:4294967295});
    expect(await reconstruct(await importSqliteDatabase(oracle.serialize()))).toEqual(input);
    oracle.run("UPDATE dwg_summary SET title='Independent SQLite',total_editing_time_low=4294967294");oracle.run("UPDATE dwg_preview_palette_entry SET alpha=19 WHERE ordinal=0");
    const expected=structuredClone(input);expected.summary.title="Independent SQLite";expected.summary.totalEditingTime--;expected.preview.palette[0]!.alpha=19;
    expect(await reconstruct(await importSqliteDatabase(oracle.serialize()))).toEqual(expected);
    console.log("[DEBUG] DWG TypeScript typed document SQLite oracle preserved unsigned64 metadata and duplicate entities");
  }finally{oracle.close();}
},30000);

test("DWG document ordered ownership, native widths and preallocation bounds reject",async()=>{
  const oracle=Database.deserialize(await exportSqliteDatabase(await project(input)));
  try{oracle.run("UPDATE dwg_custom_property SET ordinal=0 WHERE ordinal=1");expect(oracle.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});await expect(reconstruct(await importSqliteDatabase(oracle.serialize()))).rejects.toThrow();}finally{oracle.close();}
  await expect(project({...input,codepage:65536})).rejects.toThrow();await expect(project({...input,summary:{...input.summary,totalEditingTime:18446744073709551616n}})).rejects.toThrow();await expect(project(input,{maxValueBytes:8})).rejects.toThrow();
},30000);
