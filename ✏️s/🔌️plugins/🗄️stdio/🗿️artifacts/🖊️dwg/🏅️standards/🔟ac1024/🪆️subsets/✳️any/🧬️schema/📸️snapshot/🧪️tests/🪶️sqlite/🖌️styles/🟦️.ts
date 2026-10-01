import { expect,test } from "bun:test";
import { Database } from "bun:sqlite";
import type { DwgComplexColor,DwgMaterial,DwgMaterialMap,DwgMlineStyle,DwgVisualStyle,DwgCellStyle,DwgTableStyle,DwgMLeaderStyle } from "../../../../🟦️.ts";
import { DwgProjection } from "../../../🪶️sqlite/🔢️number/🟦️.ts";
import { DwgReader } from "../../../🪶️sqlite/🫳️reader/🟦️.ts";
import { dwgProjectVisualStyle,dwgReconstructVisualStyle } from "../../../🪶️sqlite/🖌️styles/👁️visual/🟦️.ts";
import { dwgProjectMaterial,dwgReconstructMaterial,dwgProjectMline,dwgReconstructMline } from "../../../🪶️sqlite/🖌️styles/🧱️material/🟦️.ts";
import { dwgProjectTableStyle,dwgReconstructTableStyle } from "../../../🪶️sqlite/🖌️styles/🗃️table/🟦️.ts";
import { dwgProjectMLeader,dwgReconstructMLeader } from "../../../🪶️sqlite/🖌️styles/↗️mleader/🟦️.ts";
import fixture from "../../../🧫️fixtures/🪶️sqlite/🎭️bodies/🔣️.json";
import { DWG_SQLITE_SCHEMA } from "../../../🪶️sqlite/🟦️.ts";
import { exportSqliteDatabase,importSqliteDatabase } from "../../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
import { dwgMaterialFixture as material,dwgMlineFixture as mline,dwgVisualFixture as visual,dwgTableStyleFixture as table,dwgMLeaderFixture as leader } from "../../../🧫️fixtures/🪶️sqlite/🖌️styles/🟦️.ts";
const nan={bits:0x7ff0000000000001n};
async function project(){const p=await DwgProjection.create(DWG_SQLITE_SCHEMA);await p.insert("dwg_document",["DWG.SQLite.semantic","AC1024",0n,0n]);await p.insert("dwg_drawing",[1n]);for(const[index,kind]of["material","mline_style","visual_style"].entries())await p.insert("dwg_object",[1n,BigInt(index),0n,BigInt(index),0n,"","object",null,null,null,null,kind]);await dwgProjectMaterial(p,1n,material);await dwgProjectMline(p,2n,mline);await dwgProjectVisualStyle(p,3n,visual);return p.finish();}
async function read(n:Database){const r=await DwgReader.create(await importSqliteDatabase(n.serialize()),DWG_SQLITE_SCHEMA);await r.one("dwg_document");await r.one("dwg_drawing");await r.list("dwg_object",1,1n,2);const result={material:await dwgReconstructMaterial(r,1n),mline:await dwgReconstructMline(r,2n),visual:await dwgReconstructVisualStyle(r,3n)};await r.finish();return result;}
test("DWG visual operations, material channels/transforms and multiline color entities survive SQLite",async()=>{
  const n=Database.deserialize(await exportSqliteDatabase(await project()));
  try{
    expect(n.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(n.query("PRAGMA foreign_key_check").all()).toEqual([]);expect(await read(n)).toEqual({material,mline,visual});
    expect(n.query("SELECT channel FROM dwg_material_map ORDER BY ordinal").all()).toEqual(fixture.materialMapChannels.map(channel=>({channel})));
    n.run("UPDATE dwg_visual_style_face SET opacity_operation='enable'; UPDATE dwg_material_map SET tiling='clamp' WHERE ordinal=4");const edited=await read(n);expect(edited.visual.properties.faceOpacity).toEqual({value:nan,operation:"enable"});expect(edited.material.bumpMap.tiling).toBe("clamp");
    n.run("UPDATE dwg_material_map_transform_coordinate SET ordinal=0 WHERE id=2");expect(n.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});await expect(read(n)).rejects.toThrow();
    console.log("[DEBUG] DWG visual operations, six material channels and multiline colors independently verified");
  }finally{n.close();}
},20000);

test("DWG table roles/optional edges and multileader component enums roundtrip with independent SQL edits",async()=>{
  const p=await DwgProjection.create(DWG_SQLITE_SCHEMA);await p.insert("dwg_document",["DWG.SQLite.semantic","AC1024",0n,0n]);await p.insert("dwg_drawing",[1n]);for(const[index,kind]of["table_style","mleader_style"].entries())await p.insert("dwg_object",[1n,BigInt(index),0n,BigInt(index),0n,"","object",null,null,null,null,kind]);await dwgProjectTableStyle(p,1n,table);await dwgProjectMLeader(p,2n,leader);
  const n=Database.deserialize(await exportSqliteDatabase(await p.finish()));
  const read=async()=>{const r=await DwgReader.create(await importSqliteDatabase(n.serialize()),DWG_SQLITE_SCHEMA);await r.one("dwg_document");await r.one("dwg_drawing");await r.list("dwg_object",1,1n,2);const result={table:await dwgReconstructTableStyle(r,1n),leader:await dwgReconstructMLeader(r,2n)};await r.finish();return result;};
  try{
    expect(n.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(n.query("PRAGMA foreign_key_check").all()).toEqual([]);expect(await read()).toEqual({table,leader});
    expect(n.query("SELECT role FROM dwg_cell_style ORDER BY ordinal").all()).toEqual(fixture.tableCellRoles.map(role=>({role})));
    const rows=n.query(fixture.independentQuery).all() as{role:string;edge:string;horizontal_spacing:number}[];expect(rows.length).toBe(fixture.expectedBorderCount);expect(rows.every(row=>row.horizontal_spacing===fixture.expectedMarginHorizontalSpacing)).toBe(true);
    n.run("UPDATE dwg_mleader_text_style SET left_attachment='middle'; UPDATE dwg_cell_border SET lineweight=17 WHERE id=1");const edited=await read();expect(edited.leader.text.leftAttachment).toBe("middle");expect(edited.table.table.borders.top!.lineweight).toBe(17);
    n.run("UPDATE dwg_cell_border SET ordinal=0 WHERE id=2");expect(n.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});await expect(read()).rejects.toThrow();
    console.log("[DEBUG] DWG table border joins and multileader component SQL edits independently verified");
  }finally{n.close();}
},20000);
