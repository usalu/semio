import { expect,test } from "bun:test";
import { Database } from "bun:sqlite";
import type { DwgTableRecordBody,DwgViewportTableRecord,DwgDimensionStyleTableRecord } from "../../../../../🧬️schema/🟦️.ts";
import { DwgProjection } from "../../🔢️number/🟦️.ts";
import { DwgReader } from "../../🫳️reader/🟦️.ts";
import { dwgProjectRecord,dwgReconstructRecord } from "../../🗃️tables/📇️records/🟦️.ts";
import { DWG_SQLITE_SCHEMA } from "../../🟦️.ts";
import fixture from "../../🧫️fixtures/📇️records/🔣️.json";
import { exportSqliteDatabase,importSqliteDatabase } from "../../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
import { dwgRecordFixtures as records } from "../🧰️support/📇️records/🟦️.ts";
test("DWG all seven symbol-table record variants retain every typed scalar, tuple and optional reference",async()=>{
  const p=await DwgProjection.create(DWG_SQLITE_SCHEMA);await p.insert("dwg_document",["DWG.SQLite.semantic","AC1024",0n,0n]);await p.insert("dwg_drawing",[1n]);for(const[index,record]of records.entries()){const id=await p.insert("dwg_object",[1n,BigInt(index),0n,BigInt(index),0n,"","table_record",null,null,null,null,"table_record"]);await dwgProjectRecord(p,id,record);}
  const n=Database.deserialize(await exportSqliteDatabase(await p.finish()));const read=async()=>{const r=await DwgReader.create(await importSqliteDatabase(n.serialize()),DWG_SQLITE_SCHEMA);await r.one("dwg_document");await r.one("dwg_drawing");const result:DwgTableRecordBody[]=[];for(const object of await r.list("dwg_object",1,1n,2))result.push(await dwgReconstructRecord(r,object.rowid));await r.finish();return result;};
  try{
    expect(n.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(n.query("PRAGMA foreign_key_check").all()).toEqual([]);expect(await read()).toEqual(records);expect(n.query("SELECT kind FROM dwg_table_record ORDER BY id").all()).toEqual(fixture.kinds.map(kind=>({kind})));
    n.run(fixture.independentEdit);const edited=await read();const v=edited[5]!,d=edited[6]!;if(v.kind!=="viewport"||d.kind!=="dimensionStyle")throw new Error("recordkind");expect(v.value.gridMajor).toBe(17);expect(d.value.dimensionPostfix).toBe("edited");
    n.run("UPDATE dwg_linetype_dash SET ordinal=0 WHERE id=2");expect(n.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});await expect(read()).rejects.toThrow();console.log("[DEBUG] DWG seven symbol records preserve native widths, fixed tuples, exact IEEE and full-u64 references");
  }finally{n.close();}
},20000);
