import {Database} from "bun:sqlite";
import {expect,test} from "bun:test";
import Ajv from "ajv";
import Ajv2020 from "ajv/dist/2020";
import recipeSchema from "../../../✨️derive/🧬️schema/🔣️.json";
import schema from "../🧬️schema/🔣️.json";
import law from "./🧫️fixtures/🔣️.json";

test("joint infrared emissivity events preserve both SQL columns and distinct original inverses", () => {
  expect(new Ajv2020({strict:true}).compile(recipeSchema)(schema["x-semio-energy-operation"])).toBe(true);
  const valid=new Ajv({strict:false,strictNumbers:true}).compile(schema);
  const db=new Database(":memory:");
  db.run("CREATE TABLE glazing(id INTEGER PRIMARY KEY,front REAL,back REAL)");
  db.run("INSERT INTO glazing VALUES(?,?,?)",law.original.id,...law.originalValue);
  for(const example of law.cases){
    const p=example.payload;
    const accepted=valid({mutation:"changeGlazingMaterialInfraredEmissivity",...p});
    const original=db.query<{front:number,back:number},[number]>("SELECT front,back FROM glazing WHERE id=?").get(p.id);
    expect(!original?"missing":!accepted?"invalid":original.front===p.newInfraredEmissivityFront&&original.back===p.newInfraredEmissivityBack?"no-op":"changed").toBe(example.outcome);
    if(example.outcome==="changed"){
      db.run("BEGIN");
      db.run("UPDATE glazing SET front=?,back=? WHERE id=?",p.newInfraredEmissivityFront,p.newInfraredEmissivityBack,p.id);
      expect(db.query("SELECT front,back FROM glazing WHERE id=?").get(p.id)).toEqual({front:example.after![0],back:example.after![1]});
      db.run("UPDATE glazing SET front=?,back=? WHERE id=?",original!.front,original!.back,p.id);
      expect(db.query("SELECT front,back FROM glazing WHERE id=?").get(p.id)).toEqual({front:law.originalValue[0],back:law.originalValue[1]});
      db.run("ROLLBACK");
    }
  }
  db.close();
});
