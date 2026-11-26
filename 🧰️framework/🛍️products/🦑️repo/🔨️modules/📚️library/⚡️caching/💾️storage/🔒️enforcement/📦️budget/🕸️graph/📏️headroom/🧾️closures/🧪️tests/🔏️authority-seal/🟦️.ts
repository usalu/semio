/** 🔏️ Contract: a graph authority receipt that carries the seal verdict of its writer (`selected/<hash>.authority.json` with `seal`) is a closed authority of the closure proof, in the read worker and in the receipt program of the headroom, exactly as one without it; a seal that departs from the authority schema is refused. */
import{expect,test}from"bun:test";
import{readFileSync}from"node:fs";
import{validateJsonSchemaSubset}from"@semio-tech/schema-validator-ts";
import Ajv from"ajv";
import authoritySchema from"../../../../../../../../../🏗️build/🧬️schema/📥️authority.json";

const hex="a".repeat(64),seal={version:1,mode:"quiet",rounds:0,sealedWithLaterChanges:0,laterChangedOwners:[] as string[],membershipMoved:false,admissionMoved:false,owners:{workspace:hex}};
const authority=(extra:{seal?:object;observations?:readonly unknown[]}={})=>({version:4,module:{implementation:hex,publication:hex},membership:hex,graph:hex,observations:[],...extra});
const source=(path:string)=>readFileSync(new URL(path,import.meta.url),"utf8");

test("the authority schema admits an authority with and without its seal and refuses a malformed seal, with the validator of the worker and an independent one",()=>{
 const ajv=new Ajv({strict:false});
 for(const[value,accepted]of[[authority(),true],[authority({seal}),true],[authority({seal:{...seal,mode:"other"}}),false],[authority({seal:{...seal,rounds:-1}}),false],[authority({seal:{version:1}}),false]]as const){
  expect(ajv.validate(authoritySchema,value),JSON.stringify(value.seal)).toBe(accepted);
  expect(validateJsonSchemaSubset(authoritySchema,value,authoritySchema).length===0,JSON.stringify(value.seal)).toBe(accepted);
  if("seal"in value)expect(validateJsonSchemaSubset(authoritySchema.properties.seal,value.seal,authoritySchema).length===0).toBe(accepted);
 }
});

test("the authority observation kinds are exactly the kinds of the observation schema its writer emits",()=>{
 const observed=JSON.parse(source("../../../../../../../../../🏗️build/👁️observation/🧬️schema/🔣️.json")) as{$defs:{observation:{properties:{kind:{enum:string[]}}}}};
 expect([...authoritySchema.definitions.ObservationKind.enum].sort()).toEqual([...observed.$defs.observation.properties.kind.enum].sort());
 for(const kind of["stamp","closure","registry"])expect(validateJsonSchemaSubset(authoritySchema,authority({observations:[{kind,path:"p",fact:true},0]}),authoritySchema)).toEqual([]);
});

test("the read worker and the headroom receipt program both accept the optional seal key beside the closed authority keys",()=>{
 expect(source("../../👷️closure-read-worker/🟨️.mjs")).toContain("shape(value,\"graph,membership,module,observations,seal,version\")&&validateJsonSchemaSubset(authoritySchema.properties.seal,value.seal,authoritySchema).length===0");
 expect(source("../../../🟦️.ts")).toContain("shape({...value,observations:[]},'graph,membership,module,observations,seal,version')&&shape(value.seal,'admissionMoved,laterChangedOwners,membershipMoved,mode,owners,rounds,sealedWithLaterChanges,version')&&value.seal.version===1");
});

test("a retained reference without an inventoried closure is dangling only when its leaf is provably absent",()=>{
 const text=source("../../🟦️.ts");
 expect(text).toContain("danglingReferencesV1(proof);");
 expect(text).not.toContain("for(const reference of proof.references)if(!proof.candidates.has(reference))throw Error(\"Complete retained closure inventory required\");");
 expect(text).toContain("missingLeaf(proof,join(proof.image.root,hit[1]===\"source\"?\"build-input-closures\":\"build-lock-closures\",hit[2]+\".json\"));proof.references.delete(reference);");
});
