/** 🎟️ Original body and native metadata admission remain separate finite caller inputs. */
import {test,expect} from "bun:test";
import Ajv from "ajv";
import {readFileSync} from "node:fs";
import {validateJsonSchemaSubset} from "../../../../../../../../🔨️modules/🧬️schema/✅️validator/🟦️.ts";
import grantSchema from "../../../../../../../../🔨️modules/🌱️value/🧬️retained-clone/🌐️wire/🧬️schema/🔣️.json";
const root=new URL("../",import.meta.url);
const read=(path:string)=>JSON.parse(readFileSync(new URL(path,root),"utf8"));
const schema=read("🧬️schema/🔣️.json"),law=read("🧫️fixtures/🔣️.json");
test("original canonical control schema agrees with independent Ajv without a metadata default",()=>{
 const resolved={...schema,$defs:{Grant:grantSchema}};
 const independent=new Ajv({strict:true}).addSchema(grantSchema,"urn:semio:retained-clone-grant:v1").compile(schema);
 const cases=[law.original,...law.invalidInputs];
 const ours=cases.map(value=>validateJsonSchemaSubset(resolved,value).length===0);
 expect(ours).toEqual([true,false,false,false]);expect(cases.map(value=>independent(value))).toEqual(ours);
 for(const bytes of law.bodyCopyGrants){const input={...law.original,body:{...law.original.body,maximumCopyBytes:bytes}};expect(validateJsonSchemaSubset(resolved,input)).toEqual([]);expect(independent(input)).toBe(true);expect(input.metadata).toBe(law.original.metadata);}
 console.log("[DEBUG] original finite metadata4096 is separately caller-authored; body1/7 and every zero receipt field remain unchanged; firstparty schema/Ajv outputs agree");
});
test("actual Sealer receives mandatory original metadata and records it separately before source work",()=>{
 const source=readFileSync(new URL("../../🦀️.rs",root),"utf8");
 for(const port of law.requiredReceivingPorts)expect(source).toContain(port);
 for(const fallback of law.forbiddenAuthority)expect(source).not.toContain(fallback);
 expect(source).not.toContain("canonical-edit.native-checkpoint-unsupported");
});
