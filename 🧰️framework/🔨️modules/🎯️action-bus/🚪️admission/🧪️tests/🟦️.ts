import { test, expect } from "bun:test";
import { Database } from "bun:sqlite";
import { readFileSync } from "node:fs";
import { semioSchemaAjvV1 } from "../../../🧬️schema/🔮️oracles/✅️validator/🟦️.ts";
import { validateJsonSchemaSubset } from "../../../🧬️schema/✅️validator/🟦️.ts";
const source=readFileSync(new URL("./🧫️fixtures/🔣️.json",import.meta.url),"utf8"),vectors=JSON.parse(source),schema=JSON.parse(readFileSync(new URL("../🧬️schema/🔣️.json",import.meta.url),"utf8"));
/** 🧮️ Portable five-axis admission law is checked independently by SQLite rather than native authority emulation. */
test("original factory schema and each physical-axis decision agree with independent SQLite",()=>{
 expect(validateJsonSchemaSubset(schema,vectors)).toEqual([]);expect(semioSchemaAjvV1({strict:true}).compile(schema)(vectors)).toBe(true);
 const sql=new Database(":memory:");try{const decision=sql.query("SELECT ?1=1 AND ?5>0 AND ?1<=?6 AND ?2<=?7 AND ?3<=?8 AND ?4<=?9 AND ?5<=?10 AS admitted");for(const row of vectors.admissionVectors){const actual=row.demand[0]===1&&row.demand[4]>0&&row.demand.every((value:number,axis:number)=>value<=row.grant[axis]);const independent=Boolean((decision.get(...row.demand,...row.grant)as{admitted:number}).admitted);expect(actual).toBe(row.expected);expect(actual).toBe(independent);}}
 finally{sql.close();}expect(vectors.candidateSource).toBe("callerOutputBeforeReceipt");expect(vectors.receiptSource).toBe("measuredOriginalPhysicalEffects");console.log("[DEBUG] original factory neutral decisions agree for exact, zero-copy and every denied physical currency");
});

test("scoped nested owner quotes debit atomically and agree with independent SQLite",()=>{
 const vectors=JSON.parse(readFileSync(new URL("./🧫️fixtures/🧰️scope/🔣️.json",import.meta.url),"utf8"));const schema=JSON.parse(readFileSync(new URL("../🧬️schema/🧰️scope/🔣️.json",import.meta.url),"utf8"));expect(validateJsonSchemaSubset(schema,vectors)).toEqual([]);expect(semioSchemaAjvV1({strict:true}).compile(schema)(vectors)).toBe(true);
 const db=new Database(":memory:");try{const query=db.query("SELECT ?1<=?5 AND ?2<=?6 AND ?3<=?7 AND ?4<=?8 AS accepted");for(const row of vectors.cases){let remaining=[...row.quote];for(const[index,owner]of row.owners.entries()){const accepted=owner.every((value:number,axis:number)=>value<=remaining[axis]);expect(Boolean((query.get(...owner,...remaining)as{accepted:number}).accepted)).toBe(accepted);expect(accepted).toBe(row.accepted[index]);if(accepted)remaining=remaining.map((value:number,axis:number)=>axis===3?value:value-owner[axis]);}expect(remaining).toEqual(row.remaining);}}finally{db.close()}
 console.log("[DEBUG] Seven scope quotes preserve atomic refusal, nested exhaustion and structural depth against SQLite");
});
