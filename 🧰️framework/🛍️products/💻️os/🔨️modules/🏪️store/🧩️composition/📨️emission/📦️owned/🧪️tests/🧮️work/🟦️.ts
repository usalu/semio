import {expect,test} from "bun:test";
import Ajv from "ajv/dist/2020.js";
import cases from "./🔣️.json";
import schema from "./🧬️schema.json";

test("raw child work preserves independent delegate and refusal values",()=>{
 const validate=new Ajv().compile(schema);expect(validate(cases)).toBe(true);
 const results=cases.events.map(event=>event.admitted?event.delegateWork:0);
 expect(results).toEqual(cases.events.map(event=>event.expectedWork));
 expect(results.reduce((sum,value)=>sum+value,0)).toBe(cases.expectedTotal);
});
