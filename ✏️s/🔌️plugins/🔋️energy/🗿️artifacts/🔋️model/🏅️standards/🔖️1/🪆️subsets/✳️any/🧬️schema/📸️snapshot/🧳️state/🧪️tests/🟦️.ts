import assert from "node:assert/strict";
import {readFileSync} from "node:fs";
import Ajv from "ajv";
import {applyPatch,type Operation} from "fast-json-patch";
import schema from "../🧬️schema/🔣️.json";
import fixture from "../🧫️fixtures/🔣️.json";
type Json=null|boolean|number|string|Json[]|{[key:string]:Json};
function coldReference(root:Json,operation:{op:string;path:string;value?:Json}):Json{
 const path=operation.path.split("/").slice(1).map(part=>part.replaceAll("~1","/").replaceAll("~0","~"));
 const visit=(original:Json,index:number):Json=>{
  assert(original!==null&&typeof original==="object");const key=path[index]!;
  if(Array.isArray(original)){const rows=original.slice(),rank=Number(key);assert(Number.isSafeInteger(rank)&&rank>=0);if(index===path.length-1){if(operation.op==="add")rows.splice(rank,0,operation.value!);else if(operation.op==="remove")rows.splice(rank,1);else{assert(operation.op==="replace"&&rank<rows.length);rows[rank]=operation.value!;}}else rows[rank]=visit(rows[rank]!,index+1);return rows;}
  const record={...original};if(index===path.length-1){assert(operation.op==="replace"&&Object.hasOwn(original,key));record[key]=operation.value!;}else record[key]=visit(record[key]!,index+1);return record;
 };return visit(root,0);
}
/** 🧫️ Explicit cold reference arithmetic validates logical sparse intent independently of native storage claims. */
export function verifyEnergyRetainedStateOracle():void{
 const apply=readFileSync(new URL("../🔺️diff/🚚️apply/🦀️.rs",import.meta.url),"utf8").replace(/\s+/g,"");
 assert(apply.includes("post:&StateZone,body:usize,g:RetainedCloneGrant,_:&mutNativeDecodeControl<'_>"),"Zone apply requires original logical body independently of physical grant");
 assert(apply.includes("preflight_demand(self,d,s,post,body)"));
 assert(apply.includes("self.fields.stage_demand(d,s,body)"));
 assert(apply.includes("self.fields.stage(d,s,body,zone_child(g))"));
 const native=readFileSync(new URL("./🦀️.rs",import.meta.url),"utf8").replace(/\s+/g,"");
 assert(native.includes("preflight_retained_diff(&mutowner,patch.borrow(),original.borrow(),post,4096,g,&mutcontrol)"),"Zone native calls retain their original4096 quote policy");
 const validate=new Ajv({strict:true}).compile(schema);assert(validate(fixture),JSON.stringify(validate.errors));assert(!validate({...fixture,authority:"structure"}));assert(!validate({...fixture,snapshotFields:["schema","model","structure","zones"]}));
 for(const law of fixture.cases){
  let actual:Json=law.before as Json;for(const op of law.forward)actual=coldReference(actual,op as {op:string;path:string;value?:Json});assert.deepEqual(actual,law.after);
  const independent=applyPatch(structuredClone(law.before),law.forward as Operation[],true,false).newDocument;assert.deepEqual(independent,law.after);
  let restored:Json=actual;for(const op of law.inverse)restored=coldReference(restored,op as {op:string;path:string;value?:Json});assert.deepEqual(restored,law.before);
  assert.deepEqual(applyPatch(structuredClone(independent),law.inverse as Operation[],true,false).newDocument,law.before);
 }
 const zone=new Ajv({strict:true}).compile({type:"object",additionalProperties:false,properties:{volume_m3:{type:"number",exclusiveMinimum:0},multiplier:{type:"integer",minimum:1}}});
 for(const refusal of fixture.refusals){const patch=refusal.patch;const admitted=(!Object.hasOwn(patch,"volume_m3")||typeof patch.volume_m3==="number"&&Number.isFinite(patch.volume_m3)&&patch.volume_m3>0)&&(!Object.hasOwn(patch,"multiplier")||typeof patch.multiplier==="number"&&patch.multiplier>0);assert.equal(admitted,refusal.admitted);assert.equal(zone(patch),refusal.admitted);}
 console.log(`[DEBUG] Energy immutable state ${fixture.cases.length} language-neutral forward/inverse intents and ${fixture.refusals.length} independent semantic refusals match independent RFC6902 and Ajv; physical authority proof remains native`);
}
