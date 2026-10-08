import {expect,test} from "bun:test";
import Ajv from "ajv";
import {readFileSync} from "node:fs";
import {resolve} from "node:path";
import {mutationInputDefs} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🛂️manifest/🟦️.ts";
import valueSchema from "../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🧬️schema/🔣️.json";
import fixture from "./🧫️fixtures/🔣️.json";
const root=resolve(import.meta.dir,"../..");
function bits(number:number){const data=new DataView(new ArrayBuffer(8));data.setFloat64(0,number,false);return data.getBigUint64(0,false).toString(16).padStart(16,"0");}
test("PDF native binary64 intent controls admit canonical numeric transport",()=>{
 for(const row of fixture.cases){
  const values=Array.isArray(row.value)?row.value:[row.value];const expected=Array.isArray(row.bits)?row.bits:[row.bits];expect(values.map(bits)).toEqual(expected);
  const word=Array.isArray(row.value)?expected.map(bits=>({bits})):{bits:expected[0]};
  const schema=JSON.parse(readFileSync(resolve(root,"🧬️mutations",row.leaf,"🧬️schema/🔣️.json"),"utf8"));
  const ajv=new Ajv({strict:false});ajv.addSchema(valueSchema);const validate=ajv.compile(schema);
  expect(validate({...row.mutation,[row.field]:word})).toBe(true);expect(validate({...row.mutation,[row.field]:row.value})).toBe(true);
  const inputs=mutationInputDefs(schema,id=>id===valueSchema.$id?valueSchema:undefined);const input=inputs.find(input=>input.id===`/${row.field}`);expect(input?.schema.kind).toBe(row.kind);
  if(row.kind==="vector")expect((input!.schema as any).dims).toBe(row.dimensions);
  console.log("[DEBUG] PDF numeric intent preserves independent IEEE754 words and canonical input",row.leaf,row.field,expected);
 }
});
