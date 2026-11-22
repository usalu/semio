import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {fileURLToPath} from "node:url";
import {closeBuildOwnerAdmissionV1,readBuildGraphSelectionV1,type BuildOwnerRegistryV1} from "../../../🚪️admission/🟦️.ts";
import {evictOwnerFactsReferencingV1,observeOwnerFactsMemoizedV1,selectedAdmissionFactsV1} from "../../🟦️.ts";
import {loadedGraphAdmissionFormatV1} from "../../../🔐️authority/🟦️.ts";
import fixture from "./🔣️.json" with {type:"json"};

interface Row {targets:Record<string,string[]>;dependencies?:string[];files?:string[]}

const rows=():Record<string,Row>=>{
 const all:Record<string,Row>={...fixture.rows as Record<string,Row>};
 for(let index=0;index<fixture.unrelated;index++){const name="u"+String(index).padStart(4,"0");all[name]={targets:{build:["^build"]},dependencies:index+1<fixture.unrelated?["u"+String(index+1).padStart(4,"0")]:[],files:[name+"/src/file.ts"]};}
 return all;
};
const registry=(all:Record<string,Row>):BuildOwnerRegistryV1=>({version:1,owners:Object.keys(all).map(name=>({name,root:name,manifest:name+"/📋️project.json",packages:[]})),catalogProviders:[]});
const oracle=(direct:boolean)=>{
 const python=process.platform==="win32"?"python":"python3",result=spawnSync(python,["-I",fileURLToPath(new URL("./🐍️.py",import.meta.url))],{input:JSON.stringify([{...fixture,name:"case",direct}]),encoding:"utf8"});
 expect(result.status,result.stderr).toBe(0);
 return JSON.parse(result.stdout).case as {owners:string[];registry:number};
};
const selection=(direct:boolean)=>readBuildGraphSelectionV1({kind:"tasks",tasks:[fixture.target],...(direct?{excludeDependencies:true}:{})});
const facts=(all:Record<string,Row>,name:string)=>{const known=all[name]!;return {project:{name,targets:Object.fromEntries(Object.entries(known.targets).map(([target,dependsOn])=>[target,{dependsOn}]))},files:known.files??[],dependencies:known.dependencies??[]};};

test("a leaf target admits and observes only its dependency closure of a registry of many unrelated owners",async()=>{
 const all=rows(),expected=oracle(false);
 expect(expected.registry).toBe(fixture.unrelated+Object.keys(fixture.rows).length);
 const calls:string[]=[],admission=await closeBuildOwnerAdmissionV1(selection(false),registry(all),async owner=>{calls.push(owner.name);return facts(all,owner.name);});
 expect(admission.owners).toEqual(expected.owners);
 expect([...new Set(calls)].sort()).toEqual(expected.owners);
 expect(expected.owners).toEqual(["app","core","lib"]);
 expect(calls.length).toBeLessThan(expected.registry/100);
});

test("a direct target observes its own owner only",async()=>{
 const all=rows(),expected=oracle(true),calls:string[]=[];
 await closeBuildOwnerAdmissionV1(selection(true),registry(all),async owner=>{calls.push(owner.name);return facts(all,owner.name);});
 expect(calls).toEqual(expected.owners);
 expect(calls).toEqual(["app"]);
});

test("churn outside the closure keeps every owner fact, churn inside re-observes only the owners that read it",async()=>{
 const all=rows(),scope="closure-first-scope";
 let observed:string[]=[],rowsOf:unknown[]=[];
 const observation={captureObservedSourceInputsV1:async(observe:()=>Promise<unknown>)=>({value:await observe(),observations:rowsOf}),retainObservedRowsV1:()=>{}};
 const read=()=>closeBuildOwnerAdmissionV1(selection(false),registry(all),async owner=>observeOwnerFactsMemoizedV1({
  key:scope+"\0"+loadedGraphAdmissionFormatV1+"\0"+owner.name,hold:async()=>true,observation:observation as never,
  observe:async()=>{observed.push(owner.name);const value=facts(all,owner.name);rowsOf=value.files.map(path=>({kind:"exists",path:"/w/"+path}));return value;},
 }) as never);
 await read();
 expect(observed.sort()).toEqual(["app","core","lib"]);
 expect(selectedAdmissionFactsV1.entries(scope).map(entry=>entry.suffix).sort()).toEqual(["app","core","lib"]);
 observed=[];
 expect(evictOwnerFactsReferencingV1(scope,["/w/"+fixture.outside])).toBe(0);
 await read();
 expect(observed).toEqual([]);
 expect(evictOwnerFactsReferencingV1(scope,["/w/"+fixture.inside])).toBe(2);
 observed=[];
 await read();
 expect(observed.sort()).toEqual(["core","lib"]);
});

test("the closure trace names how many owners each edge kind admitted and the owner that admitted the most",async()=>{
 const all=rows(),expected=oracle(false),traces:{owners:number;kinds:Record<string,number>;top:{owner:string;admitted:number}[]}[]=[];
 const admission=await closeBuildOwnerAdmissionV1(selection(false),registry(all),async owner=>facts(all,owner.name),undefined,undefined,summary=>{traces.push(summary);});
 const last=traces.at(-1)!;
 expect(admission.owners).toEqual(expected.owners);
 expect(last.owners).toBe(expected.owners.length);
 expect(Object.values(last.kinds).reduce((sum,count)=>sum+count,0)).toBe(expected.owners.length);
 expect(last.kinds.selection).toBe(1);
 expect(last.top.reduce((sum,row)=>sum+row.admitted,0)).toBe(expected.owners.length);
});
