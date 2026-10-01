import { expect,test } from "bun:test";
import { Database } from "bun:sqlite";
import type { DwgXRecordValue,DwgComplexColor } from "../../../../🟦️.ts";
import { DwgProjection } from "../../../🪶️sqlite/🔢️number/🟦️.ts";
import { DwgReader } from "../../../🪶️sqlite/🫳️reader/🟦️.ts";
import { dwgProjectValues,dwgReconstructValues } from "../../../🪶️sqlite/🏷️xrecord/🟦️.ts";
import { dwgProjectColor,dwgReconstructColor } from "../../../🪶️sqlite/🎨️color/🟦️.ts";
import { exportSqliteDatabase,importSqliteDatabase } from "../../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
import { DWG_SQLITE_SCHEMA } from "../../../🪶️sqlite/🟦️.ts";
import fixture from "../../../🧫️fixtures/🪶️sqlite/🏷️xrecord/🔣️.json";

const values:DwgXRecordValue[]=[{kind:"string",groupCode:-32768,value:"same\0text"},{kind:"real",groupCode:40,value:{bits:0x7ff0000000000001n}},{kind:"boolean",groupCode:290,value:true},{kind:"integer8",groupCode:280,value:-128},{kind:"integer16",groupCode:70,value:-32768},{kind:"integer32",groupCode:90,value:-2147483648},{kind:"integer64",groupCode:160,value:-9223372036854775808n},{kind:"point3d",groupCode:10,value:[{bits:0x8000000000000000n},{bits:0xfff0000000000000n},{bits:0x7ff8123456789abcn}]},{kind:"binary",groupCode:310,octets:[0,255,17,0]},{kind:"handle",groupCode:5,value:18446744073709551615n},{kind:"objectId",groupCode:330,absoluteValue:9007199254740993n}];
const colors:DwgComplexColor[]=[{index:65535,value:{kind:"none"}},{index:1,value:{kind:"byLayer"},name:""},{index:2,value:{kind:"byBlock"},bookName:""},{index:3,value:{kind:"byColor",red:0,green:255,blue:17},name:"RGB",bookName:"book"},{index:4,value:{kind:"byAci",index:65535}},{index:5,value:{kind:"byPen",index:255}},{index:6,value:{kind:"foreground"}},{index:7,value:{kind:"layerOff"}},{index:8,value:{kind:"layerFrozen"}}];

async function project(){
  const p=await DwgProjection.create(DWG_SQLITE_SCHEMA);
  await p.insert("dwg_document",["DWG.SQLite.semantic","AC1024",0n,0n]);await p.insert("dwg_drawing",[1n]);
  for(let index=0;index<colors.length;index++){
    const id=await p.insert("dwg_object",[1n,BigInt(index),0n,BigInt(index),0n,"","object",null,null,null,null,index===0?"xrecord":"visual_style"]);
    if(index===0){await p.insert("dwg_xrecord",[0n],id);await dwgProjectValues(p,undefined,id,values);const extended=await p.insert("dwg_extended_entity_data",[id,0n,4294967295n,4294967295n]);await dwgProjectValues(p,extended,undefined,[{kind:"string",groupCode:1000,value:"extended"}]);}
    await p.insert("dwg_visual_style",["",0n,0n,0n],id);await p.insert("dwg_visual_style_face",[0n,"inherit",0n,"set",0n,"disable",0n,"enable",{bits:0n},"set",{bits:0n},"inherit","set"],id);
    await dwgProjectColor(p,"dwg_visual_face_monochrome_color",id,colors[index]!);
  }
  return p.finish();
}
async function read(native:Database){
  const reader=await DwgReader.create(await importSqliteDatabase(native.serialize()),DWG_SQLITE_SCHEMA);
  await reader.one("dwg_document");await reader.one("dwg_drawing");
  const objects=await reader.list("dwg_object",1,1n,2);await reader.component("dwg_xrecord",1n);
  const extended=await reader.list("dwg_extended_entity_data",1,1n,2);expect(await dwgReconstructValues(reader,extended[0]!.rowid,undefined)).toEqual([{kind:"string",groupCode:1000,value:"extended"}]);
  const result=await dwgReconstructValues(reader,undefined,1n);const cs:DwgComplexColor[]=[];
  for(const object of objects){await reader.component("dwg_visual_style",object.rowid);await reader.component("dwg_visual_style_face",object.rowid);cs.push(await dwgReconstructColor(reader,"dwg_visual_face_monochrome_color",object.rowid));}
  await reader.finish();return{values:result,colors:cs};
}
test("DWG eleven XRecord and nine color variants are relational and exact through SQLite",async()=>{
  const native=Database.deserialize(await exportSqliteDatabase(await project()));
  try{
    expect(native.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(native.query("PRAGMA foreign_key_check").all()).toEqual([]);
    expect(native.query("SELECT kind,count(*) AS count FROM dwg_xrecord_value GROUP BY kind ORDER BY kind").all()).toHaveLength(11);
    expect(native.query("SELECT DISTINCT kind FROM dwg_xrecord_value ORDER BY kind").all()).toEqual([...fixture.valueKinds].sort().map(kind=>({kind})));
    expect(native.query("SELECT kind FROM dwg_visual_face_monochrome_color ORDER BY id").all()).toEqual(fixture.colorKinds.map(kind=>({kind})));
    expect(native.query("SELECT value AS octet FROM dwg_xrecord_binary_octet ORDER BY ordinal").all()).toEqual([{octet:0},{octet:255},{octet:17},{octet:0}]);
    expect(await read(native)).toEqual({values,colors});
    native.run("UPDATE dwg_xrecord_binary_octet SET value=99 WHERE ordinal=2");native.run("UPDATE dwg_visual_face_monochrome_color SET green=18 WHERE id=4");
    const edited=await read(native);expect(edited.values[8]).toEqual({kind:"binary",groupCode:310,octets:[0,255,99,0]});expect(edited.colors[3]!.value).toEqual({kind:"byColor",red:0,green:18,blue:17});
    console.log("[DEBUG] DWG explicit XRecord/color SQL queries, edits and IEEE/u64 identities verified");
  }finally{native.close();}
},30000);
test("DWG XRecord ownership and incoherent typed fields reject after SQLite editing",async()=>{
  const native=Database.deserialize(await exportSqliteDatabase(await project()));
  try{
    native.run("UPDATE dwg_xrecord_value SET ordinal=0 WHERE id=2");expect(native.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});await expect(read(native)).rejects.toThrow();
    native.run("UPDATE dwg_xrecord_value SET ordinal=1 WHERE id=2");native.run("INSERT INTO dwg_xrecord_binary_octet (value_id,ordinal,value) VALUES(1,0,17)");expect(native.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});await expect(read(native)).rejects.toThrow();
    await expect(dwgProjectValues(await DwgProjection.create(DWG_SQLITE_SCHEMA),1n,1n,values)).rejects.toThrow();
  }finally{native.close();}
},30000);
