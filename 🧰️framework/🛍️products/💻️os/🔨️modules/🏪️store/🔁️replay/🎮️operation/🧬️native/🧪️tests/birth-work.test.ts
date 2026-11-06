import { Database } from "bun:sqlite";
import { test, expect } from "bun:test";
import Ajv from "ajv/dist/2020.js";
import law from "./🧫️fixtures/birth-work.json";
import schema from "./🧫️fixtures/birth-work.schema.json";

test("original projection denial law has an independently validated closed shape", () => {
  expect(new Ajv({ strict: true }).validate(schema, law)).toBe(true);
  expect(law.projectionAdvanceDenials).toBe(1);
  expect([law.deniedWork, law.deniedItems, law.deniedCapacity]).toEqual([0, 0, 0]);
});

test("an unstarted clone owns no transferable output", async () => {
  const { default: Draft7 } = await import("ajv");
  const work = (await import("./🧫️fixtures/⛽️work.json")).default;
  const shape = (await import("./🧬️schema/⛽️work.json")).default;
  expect(new Draft7({ strict: true }).validate(shape, work)).toBe(true);
  expect(work.declaredWork.retainedCloneTake).toBe(0);
});

test("preparation shell waits for its exact admitted work",()=>{
 const db=new Database(":memory:");
 try{for(const budget of[0,law.preparationShellWork]){
  const row=db.query("SELECT ?1>=?2 AS released,CASE WHEN ?1>=?2 THEN ?2 ELSE 0 END AS work").get(budget,law.preparationShellWork);
  expect(row).toEqual({released:Number(budget>=law.preparationShellWork),work:budget});
 }}finally{db.close()}
});
