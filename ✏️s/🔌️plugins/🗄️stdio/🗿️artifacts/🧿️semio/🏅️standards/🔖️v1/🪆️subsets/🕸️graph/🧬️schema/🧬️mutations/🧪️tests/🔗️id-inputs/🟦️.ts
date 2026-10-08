import { expect,test } from "bun:test";
import Ajv from "ajv";
import { applyPatch } from "fast-json-patch";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { mutationInputDefs } from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🛂️manifest/🟦️.ts";

const fixture=JSON.parse(readFileSync(resolve(import.meta.dir,"🧫️fixtures/🔣️.json"),"utf8"));
const root=resolve(import.meta.dir,"../..");
test("Semio Graph native object identifiers expose truthful primitive child controls",()=>{
  for(const row of fixture.cases){
    const schema=JSON.parse(readFileSync(resolve(root,row.leaf,"🧬️schema/🔣️.json"),"utf8"));
    const validate=new Ajv({strict:false}).compile({$ref:`#/$defs/${row.definition}`,$defs:schema.$defs});
    expect(validate(row.value)).toBe(true);
    expect(validate(row.value.value)).toBe(false);
    const replacement=applyPatch(structuredClone(row.value),[{op:"replace",path:"/value",value:"Knoten 😀 B"}],true,false).newDocument;
    expect(validate(replacement)).toBe(true);
    const inputs=mutationInputDefs(schema,()=>undefined);
    if(schema.properties.key!==undefined){
      const key=inputs.find(input=>input.id==="/key");
      expect(key?.schema.kind).toBe("string");
      expect((key?.schema as any).minLen).toBe(1);
    }
    const input=inputs.find(input=>input.id===`/${row.field}`);
    expect(input).toBeDefined();
    const owner=row.many?(input!.schema as any).items:input!.schema;
    expect(owner.kind).toBe("object");
    const value=owner.fields.find((field:any)=>field.id==="/value");
    expect(value.schema.kind).toBe("reference");
    expect(value.schema.idType ?? "string").toBe("string");
    expect(Object.keys(value.label)).toEqual(["native","reuse"]);
    for(const label of Object.values(value.label) as {en:string;de:string}[]){
      expect(label.en.length).toBeGreaterThan(0);
      expect(label.de.length).toBeGreaterThan(0);
    }
    console.log("[DEBUG] Semio Graph native object ID preserves Ajv/RFC6902 payload and primitive reference control",row.leaf,row.field,replacement);
  }
});
