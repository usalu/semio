import {expect,test} from "bun:test";
import {readFileSync} from "node:fs";
import {join,resolve,posix} from "node:path";
import Ajv from "ajv/dist/2020.js";
import Draft7Ajv from "ajv";
import TOML from "@iarna/toml";
import {inspectRustModuleGraph} from "../../../../../🔍️discovery/🟦️.ts";
import {rustExternProviders} from "../../🔗️binding/🟦️.ts";
import {COMPUTE_OWNERSHIP_CONTRACT_PATH,COMPUTE_OWNERSHIP_DECLARATION_PATH,rustFamilyOwnershipActive,inspectRustFamilyOwnership,readRustFamilyOwnershipContract,type RustFamilyOwnershipProblem} from "../🟦️.ts";
const owner=resolve(import.meta.dir,"..");
export const corpus=JSON.parse(readFileSync(join(owner,"🧫️fixtures/🔣️.json"),"utf8")) as {version:1;ownerSource:string;activationCases:readonly {id:string;sources:Record<string,string>;active:boolean;nativeAliases?:Record<string,"provider"|"other">}[];cases:readonly {id:string;source:string;expectedCodes:readonly RustFamilyOwnershipProblem["code"][];nativeCompiles:boolean}[]};
export const declaration=corpus.ownerSource;
export const contract=readRustFamilyOwnershipContract({version:1,ownerManifest:"provider/Cargo.toml",ownerRoot:"provider/lib.rs",declarationSource:"provider/compute.rs",modulePath:["compute"],externName:"semio_framework_2d",packageName:"semio-framework-2d",symbols:["EngineKey","EngineHandle","EngineFault","EngineHandles","EngineCache","EngineRep"],traitName:"Engine"});
export function inputs(row:typeof corpus.cases[number]){
 const dep=row.id==="optional-provider"?'semio-framework-2d={path="../provider",optional=true}':row.id==="conditional-provider"?'[target.\'cfg(windows)\'.dependencies]\nsemio-framework-2d={path="../provider"}':row.id==="missing-provider"?"":row.id==="dependency-alias"?'renamed={package="semio-framework-2d",path="../provider"}':'semio-framework-2d={path="../provider"}';
 const sources=new Map([["provider/Cargo.toml",'[package]\nname="semio-framework-2d"\nversion="0.1.0"\nedition="2021"\n[lib]\nname="semio_framework_2d"\npath="lib.rs"'],["provider/lib.rs",'pub mod compute;'],["provider/compute.rs",declaration],["consumer/Cargo.toml",'[package]\nname="consumer"\nversion="0.1.0"\nedition="2021"\n[lib]\npath="lib.rs"\n[dependencies]\n'+dep+'\nother="1"'],["consumer/lib.rs",row.id==="unmounted"?"":row.source]]);
 if(row.id.startsWith("canonical-root-")){sources.set("provider/compute.rs",declaration+"\n"+row.source);sources.set("consumer/lib.rs","");}
 else if(row.id.startsWith("canonical-")){sources.delete("consumer/Cargo.toml");sources.set("provider/compute.rs",declaration+(row.id==="canonical-parent-imported-macro"?" use other::assert_eq;":"")+' #[path="../consumer/lib.rs"] mod child;');}
 if(row.id==="canonical-ancestor-glob-not-imported")sources.set("provider/lib.rs",'pub mod compute;pub use other::*;');
 if(row.id==="unmounted")sources.set("consumer/orphan.rs",row.source);
 if(row.id==="foreign-mount")sources.set("consumer/lib.rs",row.source+' #[path="../provider/compute.rs"] mod foreign;');
 const inventory=new Map<string,"file"|"directory"|"symlink">([[".","directory"]]);for(const path of sources.keys()){const parts=path.split("/");for(let index=1;index<parts.length;index++)inventory.set(parts.slice(0,index).join("/"),"directory");inventory.set(path,"file");}
 if(row.id==="linked-provider")inventory.set("provider","symlink");
 const graph=inspectRustModuleGraph([...sources.keys()],path=>sources.get(path),{strictManifests:true});
 return {contract,sources,inventory,graph};
}
test("closed family ownership cases and direct provider agree with independent TOML",()=>{
 const data=inputs(corpus.cases[0]!);const context=data.graph.contexts.get("consumer/lib.rs")![0]!;
 const options={context,compileKind:"test" as const,files:data.sources,sourceFiles:new Set(data.sources.keys()),inventory:data.inventory};
 expect(rustExternProviders(options)).toEqual(rustExternProviders({...options,parser:{parse:source=>TOML.parse(source)}}));
});
for(const row of corpus.cases)test(row.id,()=>{
 const actual=inspectRustFamilyOwnership(inputs(row));
 expect([...new Set(actual.map(problem=>problem.code))].sort(),JSON.stringify(actual)).toEqual([...row.expectedCodes].sort());
});
test("family inspection cancellation and closed contract rejection",()=>{
 expect(()=>inspectRustFamilyOwnership({...inputs(corpus.cases[0]!),checkCancellation:()=>{throw Error("cancelled");}})).toThrow("cancelled");
 expect(()=>readRustFamilyOwnershipContract({...contract,unknown:true})).toThrow();
 expect(()=>readRustFamilyOwnershipContract({...contract,ownerRoot:"provider/../lib.rs"})).toThrow();
});

test("captured portable contract agrees with independent closed JSON schema",()=>{
 const root=resolve(import.meta.dir,"../../../../../../../../../..");
 const value=JSON.parse(readFileSync(join(root,COMPUTE_OWNERSHIP_CONTRACT_PATH),"utf8"));
 const validate=new Draft7Ajv({strict:true}).compile(JSON.parse(readFileSync(join(root,COMPUTE_OWNERSHIP_CONTRACT_PATH.replace("📏️ownership/🔣️.json","🧬️schema/📍️binding-origin/🔣️.json")),"utf8")));
 expect(validate(value),JSON.stringify(validate.errors)).toBe(true);expect(readRustFamilyOwnershipContract(value)).toEqual(value);
 for(const row of [{...value,foreign:true},{...value,ownerRoot:"provider/../lib.rs"},{...value,ownerRoot:"/provider/lib.rs"}]){expect(validate(row)).toBe(false);expect(()=>readRustFamilyOwnershipContract(row)).toThrow();}
});

for(const row of corpus.activationCases)test("owner activation "+row.id,()=>{
 const sources=new Map(Object.entries(row.sources).map(([path,source])=>[path==="contract"?COMPUTE_OWNERSHIP_CONTRACT_PATH:path==="owner"?COMPUTE_OWNERSHIP_DECLARATION_PATH:path,source]));
 const inventory=new Map<string,"file"|"directory">([[".","directory"]]);for(const path of sources.keys()){const parts=path.split("/");for(let index=1;index<parts.length;index++)inventory.set(parts.slice(0,index).join("/"),"directory");inventory.set(path,"file");}
 expect(rustFamilyOwnershipActive({sources,inventory,graph:inspectRustModuleGraph([...sources.keys()],path=>sources.get(path),{strictManifests:true})})).toBe(row.active);
});

test("workspace dependency identities agree with independent TOML projection",()=>{
 const record=(value:unknown):Record<string,unknown>=>{expect(typeof value).toBe("object");expect(value).not.toBeNull();expect(Array.isArray(value)).toBe(false);return value as Record<string,unknown>;};
 for(const row of corpus.activationCases.filter(row=>row.id.includes("workspace-"))){
  const manifest=record(TOML.parse(row.sources["consumer/Cargo.toml"]!)),packageTable=record(manifest.package);
  const workspacePath=typeof packageTable.workspace==="string"?posix.join("consumer",packageTable.workspace,"Cargo.toml"):"Cargo.toml";
  const workspace=record(record(TOML.parse(row.sources[workspacePath]!)).workspace),dependency=record(record(workspace.dependencies).e);
  expect(dependency.package==="semio-framework-2d").toBe(row.active);
  expect(record(record(manifest.dependencies).e).workspace).toBe(true);
 }
});
