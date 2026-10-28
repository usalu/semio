import {Database} from "bun:sqlite";
import {expect,test} from "bun:test";
import Ajv from "ajv";
import Ajv2020 from "ajv/dist/2020";
import recipeSchema from "../../../✨️derive/🧬️schema/🔣️.json";
import schema from "../🧬️schema/🔣️.json";
import law from "./🧫️fixtures/🔣️.json";
import wide from "./🧫️fixtures/🌊️wide.json";

test("original layer permutation matches SQL material multiplicities and preserves exact event inverses", () => {
  expect(new Ajv2020({strict:true}).compile(recipeSchema)(schema["x-semio-energy-operation"])).toBe(true);
  const valid=new Ajv({strict:false}).compile(schema);
  const db=new Database(":memory:");
  db.run("CREATE TABLE layers(source INTEGER,rank INTEGER,id INTEGER,PRIMARY KEY(source,rank))");
  for(const fixture of [law,wide])for(const example of fixture.cases){
    expect(valid({mutation:"reorderConstructionLayers",...example.payload})).toBe(true);
    db.run("DELETE FROM layers");
    const original=fixture.existingRows[0].layer_material_ids;
    original.forEach((id,rank)=>db.run("INSERT INTO layers VALUES(?,?,?)",0,rank,id));
    example.payload.newLayerMaterialIds.forEach((id,rank)=>db.run("INSERT INTO layers VALUES(?,?,?)",1,rank,id));
    const held=db.query<{id:number,count:number},[]>("SELECT id,COUNT(*) AS count FROM layers WHERE source=0 GROUP BY id ORDER BY id").all();
    const wanted=db.query<{id:number,count:number},[]>("SELECT id,COUNT(*) AS count FROM layers WHERE source=1 GROUP BY id ORDER BY id").all();
    const sameCounts=JSON.stringify(held)===JSON.stringify(wanted);
    const sameOrder=JSON.stringify(original)===JSON.stringify(example.payload.newLayerMaterialIds);
    expect(example.payload.id!==fixture.existingRows[0].id?"missing":!sameCounts?"invalid-context":sameOrder?"no-op":"changed").toBe(example.outcome);
    if(example.outcome==="changed"){
      const inverse=db.query<{id:number},[]>("SELECT id FROM layers WHERE source=0 ORDER BY rank").all().map(row=>row.id);
      expect(inverse).toEqual(original);
      db.run("DELETE FROM layers WHERE source=0");
      db.run("INSERT INTO layers SELECT 0,rank,id FROM layers WHERE source=1");
      expect(db.query<{id:number},[]>("SELECT id FROM layers WHERE source=0 ORDER BY rank").all().map(row=>row.id)).toEqual(example.payload.newLayerMaterialIds);
      db.run("DELETE FROM layers WHERE source=0");
      inverse.forEach((id,rank)=>db.run("INSERT INTO layers VALUES(?,?,?)",0,rank,id));
      expect(db.query<{id:number},[]>("SELECT id FROM layers WHERE source=0 ORDER BY rank").all().map(row=>row.id)).toEqual(original);
    }
  }
  db.close();
});
