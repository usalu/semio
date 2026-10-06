import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv2020 from "ajv/dist/2020";
import policy from "../🧫️fixtures/🔣️.json";
import policySchema from "../🧬️schema/🔣️.json";
import interruption from "../🧫️fixtures/🛑️interruption/🔣️.json";
import interruptionSchema from "../../../../🧬️schema/🧮️geometry/📍️point/🧬️schema/🔣️.json";
test("Draw utility and interruption vectors remain admitted by their owning schemas",()=>{
 const ajv=new Ajv2020({strict:true}),validate=ajv.compile(policySchema);expect(validate(policy)).toBe(true);expect(validate({...policy,utilities:policy.utilities.map(row=>({...row,allowsActionsWhileActive:false}))})).toBe(false);expect(validate({...policy,utilities:policy.utilities.map(()=>policy.utilities[0])})).toBe(false);const validatePoint=ajv.compile(interruptionSchema);for(const row of interruption.cases){expect(validatePoint(row.press)).toBe(true);expect(validatePoint(row.move)).toBe(true);}expect(validatePoint([1])).toBe(false);
 const db=new Database(":memory:");try{const actual=db.query("SELECT json_array_length(json_extract(?1,'$.utilities')) AS utilities, json_array_length(json_extract(?2,'$.cases')) AS interruptions").get(JSON.stringify(policy),JSON.stringify(interruption));expect(actual).toEqual({utilities:12,interruptions:8});expect(interruption.cases.every(row=>policy.utilities.some(utility=>utility.id===row.utility))).toBe(true);}finally{db.close();}
 console.log("[DEBUG] Draw owning schemas admit twelve utility policies and eight interrupted interaction cases with independent SQLite counts");
});
