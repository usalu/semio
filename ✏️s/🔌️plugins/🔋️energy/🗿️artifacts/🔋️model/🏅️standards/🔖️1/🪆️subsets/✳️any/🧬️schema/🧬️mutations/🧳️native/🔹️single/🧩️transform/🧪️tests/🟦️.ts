import {Database} from "bun:sqlite";
import {expect,test} from "bun:test";
import Ajv from "ajv";
import Ajv2020 from "ajv/dist/2020";
import {fileURLToPath} from "node:url";
import recipeSchema from "../../../../✨️derive/🧬️schema/🔣️.json";

test("both monthly ground sources preserve one-based slots and exact SQL inverse values", async () => {
  const root=fileURLToPath(new URL("../../../../",import.meta.url));
  const validateRecipe=new Ajv2020({strict:true}).compile(recipeSchema);
  let count=0;
  for await(const path of new Bun.Glob("*/*/🧪️native/🧫️fixtures/🔣️.json").scan({cwd:root,absolute:true})){
    const law=await Bun.file(path).json();
    if(law.contract!=="semio.energy.original-monthly-ground/v1")continue;
    const schema=await Bun.file(new URL("../../🧬️schema/🔣️.json",`file://${path}`)).json();
    expect(validateRecipe(schema["x-semio-energy-operation"]),JSON.stringify(validateRecipe.errors)).toBe(true);
    const valid=new Ajv({strict:false,strictNumbers:true}).compile(schema);
    const db=new Database(":memory:");
    db.run("CREATE TABLE ground(month INTEGER PRIMARY KEY CHECK(month BETWEEN 1 AND 12),value REAL)");
    law.ground[law.field].forEach((value:number,index:number)=>db.run("INSERT INTO ground VALUES(?,?)",index+1,value));
    for(const example of law.cases){
      const p=example.payload;
      const accepted=valid({...p,mutation:schema.properties.mutation.const});
      const original=db.query<{value:number},[number]>("SELECT value FROM ground WHERE month=?").get(p.month);
      expect(!accepted?"invalid":original?.value===p.newTemperatureC?"no-op":"changed").toBe(example.outcome);
      if(example.outcome==="changed"){
        db.run("BEGIN");
        db.run("UPDATE ground SET value=? WHERE month=?",p.newTemperatureC,p.month);
        expect(db.query<{value:number},[]>("SELECT value FROM ground ORDER BY month").all().map(row=>row.value)).toEqual(example.after);
        expect(original!.value).toBe(example.inverse);
        db.run("UPDATE ground SET value=? WHERE month=?",original!.value,p.month);
        expect(db.query<{value:number},[]>("SELECT value FROM ground ORDER BY month").all().map(row=>row.value)).toEqual(law.ground[law.field]);
        db.run("ROLLBACK");
      }
    }
    db.close();
    count++;
  }
  expect(count).toBe(2);
});
