/** 🧹️ Contract of the sweep of stale scratch: the declared area `scratch` is collected by the cache collector exactly as the independent Python oracle of pid-named units says (owner gone and old enough goes; live, young or foreign names stay), and the directory the scratch module creates lies in the home the area declares.
 * @see ./🥒️.feature
 * @see ../🧶️units/🐍️.py
 * @see ../../../../../../../../../🔨️modules/🏃️process/🧹️scratch/🧪️tests/🟦️.ts */
import {afterAll,beforeAll,expect,test} from "bun:test";
import {mkdirSync,mkdtempSync,readdirSync,rmSync,utimesSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {basename,dirname,join,relative} from "node:path";
import {runToolSyncV1} from "../../../../../../../../🔨️modules/🏃️process/🧰️tools/🟦️.ts";
import scratchProtocol from "../../../../../../../../🔨️modules/🏃️process/🧹️scratch/🔣️.json" with {type:"json"};
import {createScratchDirectoryV1,removeScratchDirectoryV1,scratchHomeV1} from "../../../../../../../../🔨️modules/🏃️process/🧹️scratch/🟦️.ts";
import {cacheLedgerV1} from "../../../../⚡️caching/💾️storage/🧾️ledger/🟦️.ts";
import {cacheRunnerPortsV1,collectCacheV1,measureCacheTreeV1} from "../../🏃️runner/🟦️.ts";
import cases from "./🧫️fixtures/🔣️.json" with {type:"json"};

const ledger=cacheLedgerV1(),area=ledger.areas.find(row=>row.id===scratchProtocol.area)!,oracle=join(import.meta.dir,"..","🧶️units","🐍️.py"),now=Date.now();
let workspace="";
const base=():string=>join(workspace,scratchProtocol.root,scratchProtocol.directory);
const ports=(alive:(pid:number)=>boolean)=>cacheRunnerPortsV1({signal:new AbortController().signal,now:()=>now,progress:async()=>{},exclusive:async()=>({release:()=>{}}),alive});
const output=()=>({write:(_line:string)=>{},language:"en" as const});
beforeAll(()=>{workspace=mkdtempSync(join(tmpdir(),"cgc-scratch-"));mkdirSync(base(),{recursive:true});});
afterAll(()=>{rmSync(workspace,{recursive:true,force:true});});

test("the scratch module and the ledger declare the same home and the same unit pattern",()=>{
 expect(area).toMatchObject({root:"native",kind:"tree",directory:scratchProtocol.directory,collect:{custody:"pid",units:scratchProtocol.units}});
 expect(ledger.roots.native.directory).toBe(scratchProtocol.root);
 const root=mkdtempSync(join(tmpdir(),"cgc-home-"));
 try{
  writeFileSync(join(root,"📋️project.json"),"{}");writeFileSync(join(root,"package.json"),"{}");
  const directory=createScratchDirectoryV1("sweep",{root});
  expect(dirname(directory)).toBe(join(root,ledger.roots.native.directory,area.directory!));expect(new RegExp(area.collect!.units!,"u").test(basename(directory))).toBe(true);
  removeScratchDirectoryV1(directory);expect(scratchHomeV1({root})).toBe(dirname(directory));
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("collecting the declared scratch area removes exactly what the independent oracle removes, and a dry run removes nothing",async()=>{
 const live=new Set(cases.livePids),alive=(pid:number)=>live.has(pid);
 for(const entry of cases.entries){const directory=join(base(),entry.name);mkdirSync(directory,{recursive:true});const file=join(directory,"payload.bin");writeFileSync(file,"x".repeat(entry.bytes));const time=(now-entry.ageMs)/1000;utimesSync(file,time,time);utimesSync(directory,time,time);}
 const measured=await Promise.all(cases.entries.map(async entry=>[entry.name,await measureCacheTreeV1(join(base(),entry.name),ports(alive),1000)] as const));
 const file=join(workspace,"cases.json");
 writeFileSync(file,JSON.stringify({cases:[{now,units:area.collect!.units,unusedAgeMs:area.collect!.unusedAgeMs,quietMs:area.collect!.quietMs,budgetBytes:area.budget!.bytes,entries:cases.entries.map((entry,index)=>({name:entry.name,bytes:measured[index]![1].bytes,recencyMs:measured[index]![1].recencyMs,alive:live.has(Number(entry.name.split("-")[0]))}))}]}));
 const run=runToolSyncV1("python",["-I",oracle,file],{encoding:"utf8",maxBuffer:1<<26});expect(run.status,run.stderr).toBe(0);
 const expected=(JSON.parse(run.stdout) as string[][])[0]!.sort();
 expect(expected).toEqual(cases.entries.filter(entry=>entry.expect==="removed").map(entry=>entry.name).sort());
 const before=readdirSync(base()).sort(),planned=await collectCacheV1(workspace,ledger,{dryRun:true,only:new Set([area.id])},ports(alive),output(),undefined as never);
 expect(readdirSync(base()).sort(),"a dry run removes nothing").toEqual(before);expect(planned.removed.map(row=>row.name).sort()).toEqual(expected);
 const result=await collectCacheV1(workspace,ledger,{dryRun:false,only:new Set([area.id])},ports(alive),output(),undefined as never);
 expect(result.failed).toEqual([]);expect(before.filter(name=>!readdirSync(base()).includes(name)).sort()).toEqual(expected);
 expect(relative(workspace,base())).toBe(join(scratchProtocol.root,scratchProtocol.directory));
});
