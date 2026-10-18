import {test,expect} from "bun:test";
import Ajv from "ajv/dist/2020";
import {Database} from "bun:sqlite";
import {readFileSync} from "node:fs";
import schema from "./🔣️schema.json";
import law from "../🧫️fixtures/🧾️transaction-command-close/🔣️.json";

test("transaction fixture close retains its declared caller grant independently of quotes",()=>{
 expect(new Ajv({strict:false}).compile(schema)(law)).toBe(true);
 const original=law.callerGrant;
 const db=new Database(":memory:");
 try{
  expect(db.query("select json_extract(?, '$.callerGrant') as grant").get(JSON.stringify(law))).toEqual({grant:JSON.stringify(original)});
  for(const row of law.cases){
   const quote=[31,47,59,3],actual=[...original];
   if(row.probe){actual[0]=row.probe.items;if(row.probe.axis==="depth-minus-one")actual[4]=quote[3]-1;if(row.probe.axis==="release-minus-one")actual[3]=quote[2]-1;}
   expect(actual[1]).toBe(original[1]);expect(actual[2]).toBe(original[2]);
   expect(original).toEqual(law.callerGrant);
  }
 }finally{db.close()}
 const source=readFileSync(new URL("./🦀️.rs",import.meta.url),"utf8");
 const close=readFileSync(new URL("../🧬️mutation-fixtures-job-close/🦀️.rs",import.meta.url),"utf8");
 expect(source.includes("caller_grant: [usize; 5]")).toBe(true);
 expect(source.includes("turn(&mut job,&mut cx,original,&mut released)")).toBe(true);
 expect(source.includes("retire_completion(external,&mut cx,original)")).toBe(true);
 expect(close.includes("fn job_grant")).toBe(false);
 expect(close.includes("plugin_demand_grant")).toBe(false);
});
