const neutral=(value:any):any=>value instanceof Uint8Array?Array.from(value):Array.isArray(value)?value.map(neutral):value&&typeof value==="object"?Object.fromEntries(Object.entries(value).map(([key,item])=>[key,neutral(item)])):value;
/** 🪪️ Exact source identity, validated independently by JSON Schema. */
import {test,expect} from "bun:test";
import Ajv from "ajv";
import schema from "../../🧬️schema/🔣️.json";
import cases from "../../🧫️fixtures/🔣️.json";
import {sceneIdentityMatches} from "../../🟦️.ts";
import admissionCases from "../../🚦️admission/🧫️fixtures/🔣️.json";
import admissionSchema from "../../🚦️admission/🧬️schema/🔣️.json";
import {sceneAdmission,type SceneAdmission} from "../../🚦️admission/🟦️.ts";
test("query admission borrows only the exact complete live source and distinguishes pending failure and stale owners",()=>{
 const ajv=new Ajv({strict:true,$data:true}).addSchema(schema),validate=ajv.compile(admissionSchema),validateStatus=ajv.compile({$ref:"semio.draw.scene.admission#/definitions/status"});
 const live={type:"object",properties:{live:{const:{$data:"1/captured"}}}},visual={type:"object",properties:{visual:{const:{$data:"1/captured"}}}},flag=(field:string,value:boolean)=>({type:"object",properties:{[field]:{const:value}}});
 const oracles=[["ready",{allOf:[live,visual]}],["pending",{allOf:[live,{not:visual},flag("failed",false),flag("preparing",true)]}],["failed",{allOf:[live,{not:visual},flag("failed",true)]}],["unavailable",{allOf:[live,{not:visual},flag("failed",false),flag("preparing",false)]}],["stale",{not:live}]].map(([status,rule])=>({status,validate:ajv.compile(rule as object)}));
 for(const row of admissionCases.cases){const input={captured:admissionCases.identity,live:row.live===null?null:{...admissionCases.identity,...row.live},visual:row.visual===null?null:{...admissionCases.identity,...row.visual},preparing:row.preparing,failed:row.failed}as SceneAdmission,before=structuredClone(input);expect(validate(input),row.name).toBe(true);const status=sceneAdmission(input);expect(validateStatus(status)).toBe(true);expect(status,row.name).toBe(row.status);
  const independent=oracles.filter(oracle=>oracle.validate(input));expect(independent.map(oracle=>oracle.status),row.name).toEqual([status]);expect(input).toEqual(before);
 }
 console.log(`[DEBUG] ${admissionCases.cases.length} exact source admission relations agree with independent Ajv and canonical JSON identity, including full-width lanes and persistent failure`);
});
test("mounted geometry requires every source identity field",()=>{const validate=new Ajv({strict:true}).compile(schema);for(const row of cases){expect(validate(row.captured)).toBe(true);expect(validate(row.live)).toBe(true);expect(sceneIdentityMatches(row.captured,row.live)).toBe(row.matches);expect(JSON.stringify(row.captured)===JSON.stringify(row.live)).toBe(row.matches);expect(BigInt(row.captured.base)===BigInt(row.live.base)&&BigInt(row.captured.generation)===BigInt(row.live.generation)&&row.captured.instance===row.live.instance&&row.captured.revision.every((byte,index)=>byte===row.live.revision[index])).toBe(row.matches);}});

import invalids from "../../🧫️fixtures/⚠️invalid/🔣️.json";
import type {SceneIdentity} from "../../🟦️.ts";
test("source authority refuses numeric lanes, noncanonical decimals and invalid identity widths",()=>{
 const validate=new Ajv({strict:true}).compile(schema);for(const row of invalids){const value={...cases[0]!.captured,...row.patch}as unknown as SceneIdentity;expect(validate(neutral(value)),row.name).toBe(false);expect(sceneIdentityMatches(value,value),row.name).toBe(false);expect(sceneIdentityMatches(value,cases[0]!.captured),row.name).toBe(false);expect(sceneIdentityMatches(cases[0]!.captured,value),row.name).toBe(false);}
 const sparse={...cases[0]!.captured,revision:Array<number>(32)};expect(validate(sparse)).toBe(false);expect(sceneIdentityMatches(sparse,sparse)).toBe(false);
 console.log("[DEBUG] Exact source identity rejects twenty-two malformed identities and sparse hashes without accepting legacy numeric lanes");
});
test("full-width decimal identity schema independently agrees with BigInt around every unsigned prefix",()=>{
 const validate=new Ajv({strict:true}).compile(schema),maximum=18446744073709551615n,values=new Set<bigint>([0n,1n,9007199254740991n,9007199254740992n,9007199254740993n,maximum-1n,maximum,maximum+1n]);
 for(let width=1n;width<=20n;width++){const magnitude=10n**width;for(const delta of [-1n,0n,1n])values.add(magnitude+delta);}for(let width=0n;width<20n;width++){const unit=10n**width,prefix=maximum/unit*unit;for(const delta of [-unit,-1n,0n,1n,unit])if(prefix+delta>=0n)values.add(prefix+delta);}
 for(const value of values){const decimal=value.toString(),identity={...cases[0]!.captured,base:decimal,generation:decimal},accepted=value<=maximum;expect(validate(identity),decimal).toBe(accepted);expect(sceneIdentityMatches(identity,identity),decimal).toBe(accepted);}
 console.log(`[DEBUG] ${values.size} full-width decimal source lanes matched independent BigInt range checks, including adjacent values above 2^53`);
});
