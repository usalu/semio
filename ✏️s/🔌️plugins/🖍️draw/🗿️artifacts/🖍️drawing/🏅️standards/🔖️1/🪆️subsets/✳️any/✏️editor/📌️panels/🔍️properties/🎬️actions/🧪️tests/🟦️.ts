import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv2020 from "ajv/dist/2020";
import fixture from "../🧫️fixtures/🔣️.json";
import schema from "../🧬️schema/🔣️.json";
import {drawingSelectionActionAvailable} from "../🟦️.ts";

test("inspector selection actions match neutral cases and independent relational eligibility",()=>{
 const validate=new Ajv2020({strict:true}).compile(schema);expect(validate(fixture)).toBe(true);
 expect(validate({...fixture,cases:[{...fixture.cases[0]!,selection:{...fixture.cases[0]!.selection,count:-1}}]})).toBe(false);
 const db=new Database(":memory:");
 try{
  db.exec("CREATE TABLE rules(action TEXT PRIMARY KEY,minimum INTEGER,parent INTEGER,shape INTEGER,ungroup INTEGER); INSERT INTO rules VALUES ('group',1,1,0,0),('ungroup',1,0,0,1),('toPath',1,0,1,0),('duplicate',1,1,0,0),('delete',1,1,0,0),('bringForward',1,1,0,0),('sendBackward',1,1,0,0),('bringToFront',1,1,0,0),('sendToBack',1,1,0,0),('alignLeft',2,0,0,0),('alignCenter',2,0,0,0),('alignRight',2,0,0,0),('alignTop',2,0,0,0),('alignMiddle',2,0,0,0),('alignBottom',2,0,0,0),('distributeHorizontal',3,0,0,0),('distributeVertical',3,0,0,0)");
  for(const row of fixture.cases){
   const selection=row.selection,actual=fixture.actions.filter(action=>drawingSelectionActionAvailable(selection,action)).sort();
   const reference=(db.query("SELECT action FROM rules WHERE (CASE WHEN minimum>1 THEN ?7 ELSE ?1 END)>=minimum AND ?6=1 AND (minimum=1 OR ?8=1) AND ?2=0 AND (?3=1 OR parent=0) AND (?4=1 OR shape=0) AND (?5=1 OR ungroup=0) ORDER BY action").all(selection.count,Number(selection.locked),Number(selection.sameParent),Number(selection.convertibleShapes),Number(selection.ungroupableGroups),Number(selection.complete),selection.arrangementCount,Number(selection.arrangeable)) as {action:string}[]).map(item=>item.action);
   expect(actual).toEqual([...row.expected].sort());expect(actual).toEqual(reference);expect(drawingSelectionActionAvailable(selection,"unknown")).toBe(false);
  }
 }finally{db.close();}
 console.log(`[DEBUG] Inspector selection action eligibility matched ${fixture.cases.length} neutral cases and independent SQLite rules`);
});
