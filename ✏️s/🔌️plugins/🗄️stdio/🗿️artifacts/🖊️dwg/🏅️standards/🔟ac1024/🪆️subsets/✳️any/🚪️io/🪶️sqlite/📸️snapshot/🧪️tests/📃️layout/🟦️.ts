import { expect,test } from "bun:test";
import { Database } from "bun:sqlite";
import type { DwgLayout } from "../../../../../🧬️schema/🟦️.ts";
import { DwgProjection } from "../../🔢️number/🟦️.ts";
import { DwgReader } from "../../🫳️reader/🟦️.ts";
import { dwgProjectLayout,dwgReconstructLayout } from "../../📃️layout/🟦️.ts";
import { DWG_SQLITE_SCHEMA } from "../../🟦️.ts";
import { exportSqliteDatabase,importSqliteDatabase } from "../../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
import { dwgLayoutFixture as layout } from "../🧰️support/📃️layout/🟦️.ts";
const max=18446744073709551615n;
test("DWG paper layout named vectors, enums and optional handles are queryable and exact",async()=>{
  const p=await DwgProjection.create(DWG_SQLITE_SCHEMA);await p.insert("dwg_document",["DWG.SQLite.semantic","AC1024",0n,0n]);await p.insert("dwg_drawing",[1n]);await p.insert("dwg_object",[1n,0n,0n,0n,0n,"","object",null,null,null,null,"layout"]);await dwgProjectLayout(p,1n,layout);
  const n=Database.deserialize(await exportSqliteDatabase(await p.finish()));
  const read=async()=>{const r=await DwgReader.create(await importSqliteDatabase(n.serialize()),DWG_SQLITE_SCHEMA);await r.one("dwg_document");await r.one("dwg_drawing");await r.list("dwg_object",1,1n,2);const result=await dwgReconstructLayout(r,1n);await r.finish();return result;};
  try{
    expect(n.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(n.query("PRAGMA foreign_key_check").all()).toEqual([]);expect(await read()).toEqual(layout);
    n.run("UPDATE dwg_layout SET plot_area='layout',standard_scale='custom',named_ucs_handle_high=4294967295,named_ucs_handle_low=4294967295");const edited=await read();expect(edited.plotArea).toBe("layout");expect(edited.standardScale).toBe("custom");expect(edited.namedUcsHandle).toBe(max);
    n.run("UPDATE dwg_layout_coordinate SET ordinal=0 WHERE id=2");expect(n.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});await expect(read()).rejects.toThrow();
    console.log("[DEBUG] DWG named paper vectors, optional word pairs and plot enums independently verified");
  }finally{n.close();}
},20000);
