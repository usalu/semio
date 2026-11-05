import {expect,test} from "bun:test";
import Ajv from "ajv/dist/2020.js";
import cases from "./🔣️.json";
import schema from "./🧬️schema.json";
test("generation observation preserves absent and existing original states",()=>{
 expect(new Ajv().compile(schema)(cases)).toBe(true);
 for(const row of cases.rows)expect(row.state?.generation??0).toBe(row.expected);
 expect(cases.constructRegistry).toBe(false);
});
