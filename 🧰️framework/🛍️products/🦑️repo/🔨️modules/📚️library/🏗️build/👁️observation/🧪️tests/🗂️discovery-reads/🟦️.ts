/** 🗂️ Contract: inside one round of observation every owner that walks the same tree reads each directory once and asks each existence or type question once, the facts recorded are the ones every walk would have recorded without the shared answers, a new round and a forgotten round read the disk anew, and the counts and answers equal an independent Python model. */
import fs,{mkdirSync,mkdtempSync,realpathSync,rmSync,writeFileSync}from"node:fs";
import{afterAll,expect,test}from"bun:test";
import{tmpdir}from"node:os";
import{dirname,join}from"node:path";
import{testPythonV1}from"../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import{captureObservedSourceInputsV1,forgetRoundReadsV1,observedFs,observedTypeScriptHost,withObservationRoundV1,withObservedWorldV1}from"../../🟨️.mjs";
import fixture from"./🔣️.json";

const python=testPythonV1(),oracle=join(import.meta.dir,"🐍️.py");
type Scenario=typeof fixture.scenarios[number];
const scratch:string[]=[];
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});

const build=(tree:readonly string[])=>{
 const base=realpathSync(mkdtempSync(join(tmpdir(),"discovery-reads-")));scratch.push(base);
 for(const entry of tree){if(entry.endsWith("/"))mkdirSync(join(base,entry),{recursive:true});else{mkdirSync(dirname(join(base,entry)),{recursive:true});writeFileSync(join(base,entry),"x\n");}}
 return base;
};
const counted=()=>{
 const counts={readdir:0,exists:0,stat:0};
 const port=new Proxy(fs,{get:(target,key)=>key==="readdirSync"?(...rest:unknown[])=>{counts.readdir++;return(target.readdirSync as Function)(...rest);}:key==="existsSync"?(...rest:unknown[])=>{counts.exists++;return(target.existsSync as Function)(...rest);}:key==="statSync"?(...rest:unknown[])=>{counts.stat++;return(target.statSync as Function)(...rest);}:(target as never)[key as never]});
 return{counts,port};
};
const walk=(base:string)=>{
 const pending=[base],seen:string[]=[];
 while(pending.length){const directory=pending.pop()!;for(const entry of observedFs.readdirSync(directory,{withFileTypes:true})){seen.push(entry.name);if(entry.isDirectory())pending.push(join(directory,entry.name));}}
 return seen.sort();
};
const ask=(base:string,scenario:Scenario)=>{
 const host=observedTypeScriptHost({fileExists:(path:string)=>fs.statSync(path,{throwIfNoEntry:false})?.isFile()===true,directoryExists:(path:string)=>fs.statSync(path,{throwIfNoEntry:false})?.isDirectory()===true} as never);
 return scenario.asks.map(([kind,path])=>kind==="exists"?observedFs.existsSync(join(base,path)):kind==="fileExists"?host.fileExists(join(base,path)):host.directoryExists(join(base,path)));
};
const expected=(()=>{
 const run=Bun.spawnSync([python!,"-I",oracle],{stdin:Buffer.from(JSON.stringify(fixture))});
 expect(run.exitCode).toBe(0);return JSON.parse(run.stdout.toString()).expected as {listings:number;answers:boolean[]}[];
})();

fixture.scenarios.forEach((scenario,index)=>{
 test(scenario.name,async()=>{
  const base=build(scenario.tree),{counts,port}=counted(),model=expected[index]!;
  const first=await withObservedWorldV1(port as never,()=>captureObservedSourceInputsV1(()=>withObservationRoundV1(()=>{
   const walks=Array.from({length:scenario.walks},()=>walk(base));
   const answers=Array.from({length:scenario.askRepeats},()=>ask(base,scenario));
   return{walks,answers};
  }),undefined,false));
  expect(counts.readdir).toBe(model.listings);
  for(const walked of first.value.walks)expect(walked).toEqual(first.value.walks[0]);
  for(const answers of first.value.answers)expect(answers).toEqual(model.answers);
  const rows=new Set(first.observations.map(row=>row.kind+"\0"+row.path));
  expect(rows.size).toBe(first.observations.length);
  const before={...counts};
  await withObservedWorldV1(port as never,()=>withObservationRoundV1(()=>{walk(base);}));
  expect(counts.readdir-before.readdir).toBe(model.listings);
  const mid={...counts};
  await withObservedWorldV1(port as never,()=>withObservationRoundV1(()=>{walk(base);forgetRoundReadsV1();walk(base);}));
  expect(counts.readdir-mid.readdir).toBe(2*model.listings);
 });
});

test("a path that appears after a forgotten round is read as present",async()=>{
 const base=build(["a/"]),{port}=counted();
 await withObservedWorldV1(port as never,()=>withObservationRoundV1(()=>{
  expect(observedFs.existsSync(join(base,"later.ts"))).toBe(false);
  writeFileSync(join(base,"later.ts"),"x\n");
  expect(observedFs.existsSync(join(base,"later.ts"))).toBe(false);
  forgetRoundReadsV1();
  expect(observedFs.existsSync(join(base,"later.ts"))).toBe(true);
 }));
});
