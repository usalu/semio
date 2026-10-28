import {Database} from "bun:sqlite";
import {expect,test} from "bun:test";
import Ajv from "ajv";
import Ajv2020 from "ajv/dist/2020";
import schema from "../🧬️schema/🔣️.json";
import recipeSchema from "../../../✨️derive/🧬️schema/🔣️.json";
import law from "./🧫️fixtures/🔣️.json";

test("fixed native daily profiles preserve exact schema length and SQL hourly inverse values", () => {
  const valid=new Ajv({strict:false,strictNumbers:true}).compile(schema);
  expect(new Ajv2020({strict:true}).compile(recipeSchema)(schema["x-semio-energy-operation"])).toBe(true);
  for(const length of law.invalidLengths)expect(valid({mutation:"replaceDailyScheduleHourlyValues",id:7,newHourlyValues:Array(length).fill(1)})).toBe(false);
  for(const value of [NaN,Infinity])expect(valid({mutation:"replaceDailyScheduleHourlyValues",id:7,newHourlyValues:Array(24).fill(value)})).toBe(false);
  const db=new Database(":memory:");
  db.run("CREATE TABLE hours(schedule INTEGER, rank INTEGER, value REAL, PRIMARY KEY(schedule,rank))");
  for(const example of law.cases){
    expect(valid({mutation:"replaceDailyScheduleHourlyValues",id:7,newHourlyValues:example.value})).toBe(true);
    db.run("DELETE FROM hours");
    example.row.hourly_values.forEach((value,rank)=>db.run("INSERT INTO hours VALUES(?,?,?)",example.row.id,rank,value));
    const original=db.query<{value:number},[number]>("SELECT value FROM hours WHERE schedule=? ORDER BY rank").all(7).map(row=>row.value);
    expect(original.length===0?"missing":JSON.stringify(original)===JSON.stringify(example.value)?"no-op":"changed").toBe(example.outcome);
    if(example.outcome==="changed"){
      example.value.forEach((value,rank)=>db.run("UPDATE hours SET value=? WHERE schedule=? AND rank=?",value,7,rank));
      expect(db.query<{value:number},[]>("SELECT value FROM hours WHERE schedule=7 ORDER BY rank").all().map(row=>row.value)).toEqual(example.after);
      original.forEach((value,rank)=>db.run("UPDATE hours SET value=? WHERE schedule=? AND rank=?",value,7,rank));
      expect(db.query<{value:number},[]>("SELECT value FROM hours WHERE schedule=7 ORDER BY rank").all().map(row=>row.value)).toEqual(example.inverse);
    }
  }
  db.close();
});
