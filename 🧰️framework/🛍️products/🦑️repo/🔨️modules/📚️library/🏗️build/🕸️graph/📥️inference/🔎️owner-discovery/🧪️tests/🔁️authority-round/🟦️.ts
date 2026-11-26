import {expect,test} from "bun:test";
import * as fs from "node:fs";
import {join} from "node:path";
import Ajv from "ajv";
import {testPythonV1} from "../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {testScratchRootV1} from "../../../../../../../../../../🔨️modules/🏃️process/🧪️testing/🏖️sandbox/🟦️.ts";
import fixture from "./🔣️.json";
import schema from "./🧬️schema/🔣️.json";

const inference=await import("../../../🟨️.mjs"),observation=await import("../../../../../👁️observation/🟨️.mjs"),discovery=await import("../../🟨️.mjs"),manifests=await import("../../../🦀️native-manifests/🟨️.mjs");

function workspace(){
 const root=fs.mkdtempSync(join(testScratchRootV1("authority-round"),"native-")),crates:Record<string,string[]>={core:[]};
 for(let index=0;index<fixture.chain;index++)crates["c"+index]=[...Array.from({length:fixture.fan},(_,step)=>index+1+step).filter(next=>next<fixture.chain).map(next=>"c"+next),"core"];
 crates[fixture.cycle[0]!]=[fixture.cycle[1]!];crates[fixture.cycle[1]!]=[fixture.cycle[0]!];
 for(const [name,children] of Object.entries(crates)){
  fs.mkdirSync(join(root,"crates",name),{recursive:true});
  const dev=name===fixture.devFrom?`\n[dev-dependencies]\n${fixture.devTo}={path="../${fixture.devTo}"}\n`:"";
  fs.writeFileSync(join(root,"crates",name,"Cargo.toml"),`[package]\nname="${name}"\nversion="0.1.0"\n\n[dependencies]\n${children.map(child=>`${child}={path="../${child}"}`).join("\n")}\n${dev}`);
 }
 return {root,names:Object.keys(crates)};
}

class ProbeMap extends Map<string,never>{rounds=new Set<boolean>();override get(key:string){this.rounds.add(observation.observationRoundV1()!==undefined);return super.get(key);}}

test("the assembly authority equals an independent closure digest and walks the closure inside one validation round",()=>{
 expect(new Ajv({strict:true}).validate(schema,fixture)).toBe(true);
 expect(inference).toBeDefined();
 const {root,names}=workspace(),projects:Record<string,string>={"crates/core":"core-project"},python=testPythonV1();
 const oracle=Bun.spawnSync([python!,"-I",join(import.meta.dir,"🐍️.py")],{stdin:Buffer.from(JSON.stringify({root,crates:names,projects}))});
 expect(oracle.exitCode,String(oracle.stderr)).toBe(0);
 const expected=JSON.parse(String(oracle.stdout)) as Record<string,string>,byRoot=new Map(Object.entries(projects)),inside=new ProbeMap(),outside=new ProbeMap(),closures=new Map(),others=new Map();
 for(const name of names)expect(discovery.projectAssemblyAuthorityV1(root,"crates/"+name,byRoot,undefined,inside,closures),name).toBe(expected[name]);
 for(const name of names)manifests.nativeDependencyRoots("crates/"+name,root,true,outside,others);
 expect([...inside.rounds]).toEqual([true]);
 expect([...outside.rounds]).toEqual([false]);
});
