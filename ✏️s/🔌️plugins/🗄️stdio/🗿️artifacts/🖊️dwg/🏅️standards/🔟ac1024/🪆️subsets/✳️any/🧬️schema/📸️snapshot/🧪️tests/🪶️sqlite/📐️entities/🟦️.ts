import { expect,test } from "bun:test";
import { Database } from "bun:sqlite";
import type { DwgEntityBody,DwgEntityCommon,DwgLinearDimensionEntity,DwgViewportEntity } from "../../../../🟦️.ts";
import { DwgProjection } from "../../../🪶️sqlite/🔢️number/🟦️.ts";
import { DwgReader } from "../../../🪶️sqlite/🫳️reader/🟦️.ts";
import { dwgProjectEntity,dwgReconstructEntity } from "../../../🪶️sqlite/📐️entities/🟦️.ts";
import { DWG_SQLITE_SCHEMA } from "../../../🪶️sqlite/🟦️.ts";
import fixture from "../../../🧫️fixtures/🪶️sqlite/📐️entities/🔣️.json";
import { exportSqliteDatabase,importSqliteDatabase } from "../../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
import { dwgEntityFixtures as entities } from "../../../🧫️fixtures/🪶️sqlite/📐️entities/🟦️.ts";
test("DWG all nineteen native entity variants preserve exact geometry, presence, colors and references",async()=>{
  const p=await DwgProjection.create(DWG_SQLITE_SCHEMA);await p.insert("dwg_document",["DWG.SQLite.semantic","AC1024",0n,0n]);await p.insert("dwg_drawing",[1n]);for(const[index,entity]of entities.entries()){const id=await p.insert("dwg_object",[1n,BigInt(index),0n,BigInt(index),0n,"","entity",null,null,null,null,"entity"]);await dwgProjectEntity(p,id,entity);}
  const n=Database.deserialize(await exportSqliteDatabase(await p.finish()));const read=async()=>{const r=await DwgReader.create(await importSqliteDatabase(n.serialize()),DWG_SQLITE_SCHEMA);await r.one("dwg_document");await r.one("dwg_drawing");const result:DwgEntityBody[]=[];for(const object of await r.list("dwg_object",1,1n,2))result.push(await dwgReconstructEntity(r,object.rowid));await r.finish();return result;};
  try{
    expect(n.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(n.query("PRAGMA foreign_key_check").all()).toEqual([]);expect(await read()).toEqual(entities);expect(n.query("SELECT kind FROM dwg_entity ORDER BY id").all()).toEqual(fixture.kinds.map(kind=>({kind})));expect(n.query("SELECT value AS flag FROM dwg_viewport_status_flag ORDER BY ordinal").all()).toEqual([...fixture.viewportFlags,fixture.viewportFlags[0]!].map(flag=>({flag})));
    n.run(fixture.independentEdit);const edited=await read();const text=edited[11]!,poly=edited[2]!;if(text.kind!=="text"||poly.kind!=="lwPolyline")throw new Error("kind");expect(text.value.value).toBe("edited");expect(poly.value.vertices[1]!.vertexId).toBe(17);
    n.run("UPDATE dwg_text_entity SET alignment_present=0");const noAlignment=await read();const missing=noAlignment[11]!;if(missing.kind!=="text")throw new Error("kind");expect(missing.value.alignment).toBeUndefined();
    n.run("UPDATE dwg_entity_line_end_coordinate SET ordinal=0 WHERE id=2");expect(n.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});await expect(read()).rejects.toThrow();console.log("[DEBUG] DWG nineteen explicit entities, twenty-two viewport flags and exact optional IEEE components independently verified");
  }finally{n.close();}
},30000);
