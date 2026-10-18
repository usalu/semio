import {readFileSync} from "node:fs";
import {dirname,resolve} from "node:path";
import {fileURLToPath} from "node:url";
import Ajv from "ajv";
import {toolJobMemberStoreProducerExact,memberStoreProducerSourceDeclarationV1,type ToolJobMemberStoreProducerSources, type MemberProducerSourceRole,type MemberProducerSourceBody} from "../🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import fixtures from "./🧫️fixtures/🔣️.json" with {type:"json"};
import { readModuleTreeSource } from "@semio-tech/framework-compiler-ts/syntax/module-tree";

/** 🧪️ Runs portable current-body producer laws without importing the executable workspace script. */
export function memberStoreProducerSourceSelfTests(check:typeof toolJobMemberStoreProducerExact):number{
 const root=resolve(dirname(fileURLToPath(import.meta.url)),"../../../../../../../../../..");
 const splitRoots=["🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️read/🦀️.rs","🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🌅️bin-05/🔬️unit/🦀️.rs"];
 const read=(path:string)=>({path,source:splitRoots.includes(path)?readModuleTreeSource(resolve(root,path)):readFileSync(resolve(root,path),"utf8")});
 const ajv=new Ajv({strict:true,allErrors:true});ajv.addKeyword({keyword:"x-semio-formats",valid:true});ajv.addSchema(schema);
 const validate=ajv.getSchema(schema.$id)!,validateSources=ajv.getSchema(schema.$id+"#/$defs/ToolJobMemberStoreProducerSources")!;
 if(!validate(memberStoreProducerSourceDeclarationV1))throw Error("[member producer source] defining schema refused current declaration");
 const bodies=Object.fromEntries(Object.entries(memberStoreProducerSourceDeclarationV1.roles).map(([role,ref])=>[role,read(ref.path)])) as Record<MemberProducerSourceRole,MemberProducerSourceBody>;
 const baseline:ToolJobMemberStoreProducerSources={...bodies,extensions:[]};
 if(!validateSources(baseline))throw Error("[member producer source] defining actual body schema refused");
 for(const row of fixtures.cases){
  const candidate=structuredClone(baseline),change=row.change;
  if(change&&"removeRole" in change&&typeof change.removeRole==="string"){delete (candidate as unknown as Record<string,unknown>)[change.removeRole];if(validateSources(candidate))throw Error("[member producer source] independent actual body schema accepted missing role");}
  else if(change&&"from" in change){
   const body=candidate[change.role as MemberProducerSourceRole];
   const occurrence="occurrence" in change?change.occurrence??0:0;
   let at=-1;for(let count=0;count<=occurrence;count++)at=body.source.indexOf(change.from,at+1);
   if(at<0)throw Error("[member producer source] current defining mutation absent: "+row.id);
   (body as {source:string}).source=body.source.slice(0,at)+change.to+body.source.slice(at+change.from.length);
  }
  if(check(candidate)!==row.accepted)throw Error("[member producer source] portable guard mismatch: "+row.id);
 }
 for(const row of fixtures.inputCases){const candidate=structuredClone(baseline),owner=row.path.slice(0,-1).reduce((value,key)=>(value as Record<string,unknown>)[key],candidate as unknown),key=row.path.at(-1);if(key===undefined)throw Error("input case path missing");(owner as Record<string,unknown>)[key]=row.value;if(validateSources(candidate)!==row.accepted||check(candidate)!==row.accepted)throw Error("[member producer source] schema/production input refusal mismatch: "+row.id);}
 let descriptorChecks=0;
 for(const role of Object.keys(memberStoreProducerSourceDeclarationV1.roles)){
  const candidate=structuredClone(memberStoreProducerSourceDeclarationV1);delete (candidate.roles as Record<string,unknown>)[role];
  if(validate(candidate))throw Error("[member producer source] independent Ajv accepted missing role: "+role);descriptorChecks++;
 }
 for(const [role,body] of Object.entries(baseline).filter(([role])=>role!=="extensions")){
  if(role==="storeLaws"||role==="catalogLaws")continue;
  const candidate=structuredClone(baseline);(candidate[role as MemberProducerSourceRole] as {source:string}).source="#[cfg(test)] mod false_proof {"+(body as {source:string}).source+"}";
  if(check(candidate))throw Error("[member producer source] test-only proof falsely admitted: "+role);descriptorChecks++;
 }
 for(const candidate of [{...baseline,authority:true},{...baseline,producer:{...baseline.producer,path:baseline.store.path}},{...baseline,producer:{...baseline.producer,source:""}}])if(validateSources(candidate))throw Error("[member producer source] independent actual input schema admitted extra/foreign/empty owner");
 return fixtures.cases.length+fixtures.inputCases.length+descriptorChecks+5;
}
