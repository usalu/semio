import { expect,test } from "bun:test";
import { Database } from "bun:sqlite";
import type { DwgLogicalDrawing,DwgLogicalObjectBody,DwgTableControlBody } from "../../../../🟦️.ts";
import { DwgProjection } from "../../../🪶️sqlite/🔢️number/🟦️.ts";
import { DwgReader } from "../../../🪶️sqlite/🫳️reader/🟦️.ts";
import { dwgProjectDrawing,dwgReconstructDrawing } from "../../../🪶️sqlite/✏️drawing/🟦️.ts";
import { dwgProjectDictionary,dwgReconstructDictionary,dwgProjectControl,dwgReconstructControl,dwgProjectXRecord,dwgReconstructXRecord } from "../../../🪶️sqlite/🗃️tables/🟦️.ts";
import { DWG_SQLITE_SCHEMA } from "../../../🪶️sqlite/🟦️.ts";
import { exportSqliteDatabase,importSqliteDatabase } from "../../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";

import { dwgDrawingBodies as bodies,dwgDrawingFixture as drawing } from "../../../🧫️fixtures/🪶️sqlite/✏️drawing/🟦️.ts";
const max=18446744073709551615n;
async function projectBody(p:DwgProjection,id:bigint,body:DwgLogicalObjectBody){switch(body.kind){case"tableControl":return dwgProjectControl(p,id,body.value);case"dictionary":return dwgProjectDictionary(p,id,body.value);case"xrecord":return dwgProjectXRecord(p,id,body.value);default:throw new Error("test covers authored table variants");}}
async function reconstructBody(reader:DwgReader,id:bigint,kind:string):Promise<DwgLogicalObjectBody>{switch(kind){case"table_control":return{kind:"tableControl",value:await dwgReconstructControl(reader,id)};case"dictionary":return{kind:"dictionary",value:await dwgReconstructDictionary(reader,id)};case"xrecord":return{kind:"xrecord",value:await dwgReconstructXRecord(reader,id)};default:throw new Error("unknown test body");}}
async function read(native:Database){const reader=await DwgReader.create(await importSqliteDatabase(native.serialize()),DWG_SQLITE_SCHEMA);await reader.one("dwg_document");const restored=await dwgReconstructDrawing(reader,reconstructBody);await reader.finish();return restored;}
test("DWG ordered drawing and nine native symbol controls preserve exact independent SQLite entities",async()=>{
  const p=await DwgProjection.create(DWG_SQLITE_SCHEMA);await p.insert("dwg_document",["DWG.SQLite.semantic","AC1018",0n,0n]);await dwgProjectDrawing(p,drawing,projectBody);
  const native=Database.deserialize(await exportSqliteDatabase(await p.finish()));
  try{
    expect(native.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(native.query("PRAGMA foreign_key_check").all()).toEqual([]);
    expect(native.query("SELECT kind FROM dwg_table_control ORDER BY id").all()).toEqual(["block","layer","text_style","linetype","view","ucs","viewport","registered_application","dimension_style"].map(kind=>({kind})));
    expect(await read(native)).toEqual(drawing);
    native.run("UPDATE dwg_dictionary_entry SET name='edited' WHERE ordinal=1");native.run("UPDATE dwg_table_control_entry SET handle_high=4294967295,handle_low=4294967295 WHERE control_id=2 AND ordinal=0");
    const edited=await read(native);expect(edited.objects[9]!.body).toEqual({kind:"dictionary",value:{cloningFlag:65535,hardOwner:true,defaultEntryHandle:0n,entries:[{name:"same",handle:max},{name:"edited",handle:0n}]}});
    expect(edited.objects[1]!.body).toEqual({kind:"tableControl",value:{kind:"layer",value:{entryHandles:[{handle:max},{handle:0n},{handle:max}]}}});
    native.run("UPDATE dwg_object_reactor_handle SET ordinal=0 WHERE id=2");expect(native.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});await expect(read(native)).rejects.toThrow();
    console.log("[DEBUG] DWG drawing orders, mixed owners, dictionary and nine symbol controls independently verified");
  }finally{native.close();}
},20000);
