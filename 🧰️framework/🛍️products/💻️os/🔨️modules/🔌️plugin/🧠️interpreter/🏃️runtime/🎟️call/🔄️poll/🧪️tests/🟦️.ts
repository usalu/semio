import{test,expect}from "bun:test";
import Ajv from "ajv";
import{Database}from "bun:sqlite";
/** 🧮️ The independent relational oracle enforces the guest one-event poll admission. */
test("original poll input preserves borrowed events and refuses an event batch",async()=>{
 const fixture=await Bun.file(new URL("🧫️fixtures/🔣️.json",import.meta.url)).json(),schema=await Bun.file(new URL("../🧬️schema/🔣️.json",import.meta.url)).json();
 expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);
 const db=new Database(":memory:");try{for(const row of fixture.cases){expect((db.query("SELECT json_array_length(?) <= 1 AS admitted").get(JSON.stringify(row.events))as{admitted:number}).admitted===1).toBe(row.valid);if(row.valid){const input={events:row.events,command_page:null,cold_pair_page:null,budget:fixture.budget};expect(JSON.parse(JSON.stringify(input))).toEqual(input);}}}finally{db.close()}
});
