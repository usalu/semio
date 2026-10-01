import { expect,test } from "bun:test";
import { Database } from "bun:sqlite";
import type { DwgAssoc2dConstraintGroup,DwgConstraintNode,DwgGeometricConstraint,DwgConstraintGeometry } from "../../../../🟦️.ts";
import { DwgProjection } from "../../../🪶️sqlite/🔢️number/🟦️.ts";
import { DwgReader } from "../../../🪶️sqlite/🫳️reader/🟦️.ts";
import { dwgProjectConstraints,dwgReconstructConstraints } from "../../../🪶️sqlite/📏️constraints/🟦️.ts";
import { DWG_SQLITE_SCHEMA } from "../../../🪶️sqlite/🟦️.ts";
import fixture from "../../../🧫️fixtures/🪶️sqlite/🎭️bodies/🔣️.json";
import { exportSqliteDatabase,importSqliteDatabase } from "../../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
import { dwgConstraintFixture as group,dwgConstraintGeometryFixture as geometry,dwgGeometricConstraintFixture as geometric } from "../../../🧫️fixtures/🪶️sqlite/📏️constraints/🟦️.ts";
const max=18446744073709551615n;
test("DWG fourteen typed constraint nodes preserve presence, IEEE geometry and graph references",async()=>{
  const p=await DwgProjection.create(DWG_SQLITE_SCHEMA);await p.insert("dwg_document",["DWG.SQLite.semantic","AC1024",0n,0n]);await p.insert("dwg_drawing",[1n]);await p.insert("dwg_object",[1n,0n,0n,0n,0n,"","object",null,null,null,null,"assoc_2d_constraint_group"]);await dwgProjectConstraints(p,1n,group);
  const n=Database.deserialize(await exportSqliteDatabase(await p.finish()));const read=async()=>{const r=await DwgReader.create(await importSqliteDatabase(n.serialize()),DWG_SQLITE_SCHEMA);await r.one("dwg_document");await r.one("dwg_drawing");await r.list("dwg_object",1,1n,2);const result=await dwgReconstructConstraints(r,1n);await r.finish();return result;};
  try{
    expect(n.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(n.query("PRAGMA foreign_key_check").all()).toEqual([]);expect(await read()).toEqual(group);expect(n.query("SELECT kind FROM dwg_constraint_node ORDER BY ordinal").all()).toEqual(fixture.constraintKinds.map(kind=>({kind})));
    n.run("UPDATE dwg_constrained_implicit_point SET point_present=0; UPDATE dwg_distance_constraint SET direction_present=0");const edited=await read();expect(edited.nodes[0]).toEqual({kind:"constrainedImplicitPoint",value:{geometry,point:undefined,pointKind:255,pointIndex:-2147483648,curveNodeId:2147483647}});expect(edited.nodes[4]).toEqual({kind:"distanceConstraint",value:{explicit:{geometric,valueDependencyHandle:max,dimensionDependencyHandle:0n},directionKind:255,direction:undefined}});
    n.run("UPDATE dwg_constraint_coordinate SET vector='point' WHERE id=1");expect(n.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});await expect(read()).rejects.toThrow();console.log("[DEBUG] DWG all fourteen constraint tags, absent/empty points and explicit dependencies independently verified");
  }finally{n.close();}
},20000);
