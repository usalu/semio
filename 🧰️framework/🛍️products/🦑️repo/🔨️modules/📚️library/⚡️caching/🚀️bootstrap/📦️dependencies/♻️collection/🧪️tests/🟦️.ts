import {test,expect} from "bun:test";
import {spawnSync} from "node:child_process";
import {existsSync,mkdirSync,mkdtempSync,readdirSync,rmSync,symlinkSync,unlinkSync,utimesSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import Ajv from "ajv";
import {acquireConcurrentPinsV1}from "../../🚦️concurrent/💾️storage/🟦️.ts";
import {concurrentPinResourceV1,concurrentProtocolV1}from "../../🚦️concurrent/🟦️.ts";
import {acquireResourceLease}from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import {danglingDependencyEntryV1,discardDependencyGenerationsV1,planDependencyGenerationCollectionV1,collectUnpinnedDependencyGenerationsV1,collectDependencyGenerationsV1,dependencyGenerationCollectionPolicyV1 as policy,holdDependencyGenerationsV1,planDependencyGenerationRetirementV1,readBoundDependencyGenerationsV1,readHeldDependencyGenerationsV1} from "../🟦️.ts";
import schema from "../🧬️schema/🔣️.json";
import {javascriptDependencyBindingsV1} from "../../💾️storage/🟦️.ts";
import cases from "./🔣️.json";

const ajv=new Ajv({strict:false}).addSchema(schema),validate=(name:string,value:unknown)=>ajv.getSchema(schema.$id+"#/$defs/"+name)!(value);
const K=(c:string)=>c.repeat(20),HOUR=3600;

test("the plan equals the python oracle on every case and satisfies its schema",()=>{
 const oracle=spawnSync("python3",["-I",join(import.meta.dirname,"🐍️.py")],{input:JSON.stringify({policy,cases:cases.cases}),encoding:"utf8"});expect(oracle.status,oracle.stderr).toBe(0);
 const expected=JSON.parse(oracle.stdout);
 cases.cases.forEach((row,index)=>{expect(validate("Plan",{generations:row.generations,current:row.current,retained:row.retained,held:row.held,bound:row.bound,pressure:row.pressure})).toBe(true);const plan=planDependencyGenerationRetirementV1(row);expect(plan,row.name).toEqual(expected[index]);expect(validate("Retirement",plan)).toBe(true);});
});

const store=()=>{
 const root=mkdtempSync(join(tmpdir(),"dgc-")),make=(key:string,ready:boolean,ageSeconds:number)=>{const directory=join(root,"j",key);mkdirSync(join(directory,"node_modules"),{recursive:true});writeFileSync(join(directory,"node_modules","x.js"),"1");if(ready)writeFileSync(join(directory,"bindings.json"),"{}");const at=new Date(Date.now()-ageSeconds*1000);utimesSync(join(directory,"node_modules"),at,at);if(ready)utimesSync(join(directory,"bindings.json"),at,at);utimesSync(directory,at,at);};
 mkdirSync(join(root,"pointer"),{recursive:true});writeFileSync(join(root,"pointer","s.pointer.json"),JSON.stringify({version:1,purpose:"dependency-generation-pointer",scope:".",generation:K("a")+"0123",retired:[{generation:K("b")}]}));
 return{root,make};
};

test("retires unfinished and superseded generations that no live process holds, while other processes use the store",async()=>{
 const{root,make}=store();
 try{
  make(K("a"),true,99999);make(K("b"),true,99999);make(K("c"),true,99999);make(K("d"),true,99999);make(K("e"),false,99999);make(K("f"),false,60);make(K("1"),true,30);
  holdDependencyGenerationsV1(root,[K("d")],process.pid);
  expect(readHeldDependencyGenerationsV1(root)).toEqual([K("d")]);
  const dry=(await collectDependencyGenerationsV1(root,{freeFraction:()=>1,dryRun:true}));expect(dry.retired.map(row=>row.key)).toEqual([K("c"),K("e")]);expect(readdirSync(join(root,"j")).length).toBe(7);
  const done=(await collectDependencyGenerationsV1(root,{freeFraction:()=>1,dryRun:false}));
  expect(done.retired).toEqual([{key:K("c"),reason:"superseded"},{key:K("e"),reason:"unfinished"}]);
  expect(readdirSync(join(root,"j")).sort()).toEqual([K("1"),K("a"),K("b"),K("d"),K("f")].sort());
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("a hold of a dead process protects nothing and is removed; the hold file satisfies its schema",async()=>{
 const{root,make}=store();
 try{
  make(K("c"),true,99999);
  holdDependencyGenerationsV1(root,[K("c")],424242);
  const file=join(root,policy.holdDirectory,K("c")+".424242.json");expect(existsSync(file)).toBe(true);
  expect(validate("Hold",JSON.parse(require("node:fs").readFileSync(file,"utf8")))).toBe(true);
  expect(readHeldDependencyGenerationsV1(root,pid=>pid===process.pid)).toEqual([]);expect(existsSync(file)).toBe(false);
  expect((await collectDependencyGenerationsV1(root,{freeFraction:()=>1,dryRun:false,alive:()=>false})).retired.map(row=>row.key)).toEqual([K("c")]);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("nothing is retired when no pointer can be read, and a cancellation stops between generations",async()=>{
 const{root,make}=store();
 try{
  make(K("c"),true,99999);make(K("e"),false,99999);
  const stop=new AbortController();stop.abort();
  await expect(collectDependencyGenerationsV1(root,{freeFraction:()=>1,dryRun:false,signal:stop.signal})).rejects.toThrow();
  expect(readdirSync(join(root,"j")).length).toBe(2);
  rmSync(join(root,"pointer"),{recursive:true});
  const refused=(await collectDependencyGenerationsV1(root,{freeFraction:()=>1,dryRun:false}));expect(refused.retired).toEqual([]);expect(refused.refused).toContain("pointer");expect(readdirSync(join(root,"j")).length).toBe(2);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("an installer holds its generation from the first byte, so a live unfinished install of any age is never retired",async()=>{
 const{root,make}=store(),workspace=mkdtempSync(join(tmpdir(),"dgw-"));
 try{
  const generation="9".repeat(64);
  make("9".repeat(20),false,99999);make(K("e"),false,99999);
  javascriptDependencyBindingsV1(workspace,root,"scope",["../shared","local"],generation);
  expect(readHeldDependencyGenerationsV1(root)).toEqual(["9".repeat(20)]);
  expect((await collectDependencyGenerationsV1(root,{freeFraction:()=>1,dryRun:false})).retired.map(row=>row.key)).toEqual([K("e")]);
  expect(existsSync(join(root,"j","9".repeat(20)))).toBe(true);
  expect((await collectDependencyGenerationsV1(root,{freeFraction:()=>1,dryRun:false,alive:()=>false})).retired.map(row=>row.key)).toEqual(["9".repeat(20)]);
 }finally{rmSync(root,{recursive:true,force:true});rmSync(workspace,{recursive:true,force:true});}
});

test("a generation that a checkout still binds survives without pointer and without hold, as a stage of an older binary leaves it",async()=>{
 const{root,make}=store(),workspace=mkdtempSync(join(tmpdir(),"dgb-"));
 try{
  make(K("c"),true,99999);make(K("7"),true,99999);make(K("8"),true,99999);
  const link=join(workspace,"node_modules"),plain=join(workspace,"plain");mkdirSync(plain);symlinkSync(join(root,"j",K("7"),"node_modules"),link);
  writeFileSync(join(root,"j",K("7"),policy.readyMarker),JSON.stringify({bindings:[{sourceDirectory:link,targetDirectory:join(root,"j",K("7"),"node_modules")}]}));
  writeFileSync(join(root,"j",K("8"),policy.readyMarker),JSON.stringify({bindings:[{sourceDirectory:plain,targetDirectory:join(root,"j",K("8"),"node_modules")}]}));
  const old=new Date(Date.now()-99999000);utimesSync(join(root,"j",K("7")),old,old);utimesSync(join(root,"j",K("8")),old,old);utimesSync(join(root,"j",K("7"),policy.readyMarker),old,old);utimesSync(join(root,"j",K("8"),policy.readyMarker),old,old);
  expect(readBoundDependencyGenerationsV1(root)).toEqual([K("7")]);
  expect((await collectDependencyGenerationsV1(root,{freeFraction:()=>1,dryRun:false})).retired.map(row=>row.key)).toEqual([K("8"),K("c")]);
  expect(readdirSync(join(root,"j"))).toEqual([K("7")]);
  unlinkSync(link);
  expect((await collectDependencyGenerationsV1(root,{freeFraction:()=>1,dryRun:false})).retired.map(row=>row.key)).toEqual([K("7")]);
 }finally{rmSync(root,{recursive:true,force:true});rmSync(workspace,{recursive:true,force:true});}
});

test("nothing is retired when a binding record cannot be read, and a recently written unfinished install is not dead",async()=>{
 const{root,make}=store();
 try{
  make(K("c"),true,99999);make(K("e"),false,99999);
  writeFileSync(join(root,"j",K("c"),policy.readyMarker),"{not json");
  const refused=(await collectDependencyGenerationsV1(root,{freeFraction:()=>1,dryRun:false}));expect(refused.retired).toEqual([]);expect(refused.refused).toContain("binding");
  writeFileSync(join(root,"j",K("c"),policy.readyMarker),"{}");
  const old=new Date(Date.now()-99999000);utimesSync(join(root,"j",K("c"),policy.readyMarker),old,old);utimesSync(join(root,"j",K("c")),old,old);
  writeFileSync(join(root,"j",K("e"),"node_modules","y.js"),"2");utimesSync(join(root,"j",K("e")),old,old);
  expect((await collectDependencyGenerationsV1(root,{freeFraction:()=>1,dryRun:false})).retired.map(row=>row.key)).toEqual([K("c")]);
  expect(existsSync(join(root,"j",K("e")))).toBe(true);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("a live shared pin lease keeps a generation, an unreadable lease registry retires nothing, and the pins are released afterwards",async()=>{
 const{root,make}=store();
 try{
  make(K("c"),true,99999);make(K("d"),true,99999);
  const released:string[]=[],claim=(pinned:Set<string>)=>async(key:string)=>pinned.has(key)?null:{release:()=>{released.push(key);}};
  const done=await collectUnpinnedDependencyGenerationsV1(root,{freeFraction:()=>1,dryRun:false,acquire:claim(new Set([K("c")]))});
  expect(done.retired.map(row=>row.key)).toEqual([K("d")]);expect(readdirSync(join(root,"j")).sort()).toEqual([K("c")]);expect(released).toEqual([K("d")]);
  make(K("e"),true,99999);released.length=0;
  const broken=await collectUnpinnedDependencyGenerationsV1(root,{freeFraction:()=>1,dryRun:false,acquire:async key=>{if(key===K("e"))throw Error("registry unreadable");return{release:()=>{released.push(key);}};}});
  expect(broken.retired).toEqual([]);expect(broken.refused).toContain("lease");expect(readdirSync(join(root,"j")).sort()).toEqual([K("c"),K("e")]);
  const dry=await collectUnpinnedDependencyGenerationsV1(root,{freeFraction:()=>1,dryRun:true,acquire:claim(new Set())});expect(dry.retired.map(row=>row.key)).toEqual([K("c"),K("e")]);expect(readdirSync(join(root,"j")).length).toBe(2);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("real pin leases: a reader of an older binary that holds the shared pin keeps its generation, and it goes once the pin is released",async()=>{
 const{root,make}=store(),directory=mkdtempSync(join(tmpdir(),"dgl-")),workspace=mkdtempSync(join(tmpdir(),"dgw-")),signal=new AbortController().signal;
 try{
  make(K("c"),true,99999);make(K("d"),true,99999);
  const reader=await acquireConcurrentPinsV1(directory,workspace,[K("c")+"0".repeat(44)],undefined,signal);
  const acquire=async(key:string)=>{try{return await acquireResourceLease({directory,resource:concurrentPinResourceV1(workspace,key),mode:"exclusive",signal,timeoutMs:0});}catch(error){if(/timed out/iu.test(String((error as Error).message)))return null;throw error;}};
  expect((await collectUnpinnedDependencyGenerationsV1(root,{freeFraction:()=>1,dryRun:false,acquire})).retired.map(row=>row.key)).toEqual([K("d")]);
  expect(readdirSync(join(root,"j"))).toEqual([K("c")]);
  for(const lease of reader)lease.release();
  expect((await collectUnpinnedDependencyGenerationsV1(root,{freeFraction:()=>1,dryRun:false,acquire})).retired.map(row=>row.key)).toEqual([K("c")]);
 }finally{rmSync(root,{recursive:true,force:true});rmSync(directory,{recursive:true,force:true});rmSync(workspace,{recursive:true,force:true});}
});

test("a bound node_modules with a dangling top-level link is found like the python walk finds it, hidden entries and real packages are not",()=>{
 const directory=mkdtempSync(join(tmpdir(),"dgd-"));
 try{
  mkdirSync(join(directory,"typescript"));mkdirSync(join(directory,"@scope","pkg"),{recursive:true});symlinkSync(join(directory,"typescript"),join(directory,"alias"));symlinkSync(join(directory,"missing-target"),join(directory,".bin-gone"));
  const walk=()=>spawnSync("python3",["-I","-c","import os,sys\nd=sys.argv[1]\nprint(sorted(n for n in os.listdir(d) if not n.startswith('.') and not os.path.exists(os.path.join(d,n))))",directory],{encoding:"utf8"}).stdout.trim();
  expect(danglingDependencyEntryV1(directory)).toBeUndefined();expect(walk()).toBe("[]");
  symlinkSync(join(directory,"gone"),join(directory,"zod"));
  expect(danglingDependencyEntryV1(directory)).toBe("zod");expect(walk()).toBe("['zod']");
  rmSync(join(directory,"typescript"),{recursive:true});
  expect(danglingDependencyEntryV1(directory)).toBe("alias");expect(walk()).toBe("['alias', 'zod']");
 }finally{rmSync(directory,{recursive:true,force:true});}
});

test("discarding removes every synchronization receipt and the ready marker of each current generation, so the next synchronization rebuilds; it never throws on an empty store",()=>{
 const{root,make}=store();
 try{
  make(K("a"),true,99999);make(K("b"),true,99999);
  mkdirSync(join(root,"generation"));writeFileSync(join(root,"generation","1.x.sync.json"),"{}");writeFileSync(join(root,"generation","1.x.receipt.json"),"{}");
  expect(discardDependencyGenerationsV1(root)).toEqual({receipts:1,generations:1});
  expect(existsSync(join(root,"j",K("a"),policy.readyMarker))).toBe(false);expect(existsSync(join(root,"j",K("b"),policy.readyMarker))).toBe(true);expect(existsSync(join(root,"generation","1.x.receipt.json"))).toBe(true);
  expect(discardDependencyGenerationsV1(join(root,"nothing"))).toEqual({receipts:0,generations:0});
 }finally{rmSync(root,{recursive:true,force:true});}
});

const fat=(root:string,key:string,packages:number,ageSeconds=99999)=>{
 const directory=join(root,"j",key);
 for(let index=0;index<packages;index++){mkdirSync(join(directory,"node_modules","p"+index,"lib"),{recursive:true});for(let file=0;file<20;file++)writeFileSync(join(directory,"node_modules","p"+index,"lib","f"+file+".js"),"1");}
 const at=new Date(Date.now()-ageSeconds*1000);for(const path of [join(directory,"node_modules"),directory])utimesSync(path,at,at);
};
const entriesOf=(path:string):number=>Number(spawnSync("python3",["-I","-c","import os,sys\nprint(sum(len(d)+len(f) for _,d,f in os.walk(sys.argv[1])))",path],{encoding:"utf8"}).stdout.trim());
const setPointer=(root:string,retired:readonly {generation:string;atMs:number}[],current=K("a"))=>writeFileSync(join(root,"pointer","s.pointer.json"),JSON.stringify({version:1,purpose:"dependency-generation-pointer",scope:".",generation:current+"0123",retired}));

test("the policy equals the retention of the concurrent storage protocol it replaces for the retained generations",()=>{
 const retention=concurrentProtocolV1.retention;
 expect(policy.retainedGraceSeconds*1000).toBe(retention.graceMilliseconds);expect(policy.rollbackSeconds*1000).toBe(retention.rollbackMilliseconds);expect(policy.pressureFreeFraction).toBe(retention.pressureFreeFraction);
});

test("a retained generation goes after its retention, the newest of a scope after the rollback time, and under pressure at once",()=>{
 const{root,make}=store(),now=Date.now(),H=3600000;
 try{
  for(const c of ["b","c","d","e"])make(K(c),true,99999);
  setPointer(root,[{generation:K("b"),atMs:now-7*H},{generation:K("c"),atMs:now-8*H},{generation:K("d"),atMs:now-30*H}]);
  const plan=(freeFraction:number)=>planDependencyGenerationCollectionV1(root,{freeFraction:()=>freeFraction,now:()=>now}).retire.map(row=>row.key);
  expect(plan(1)).toEqual([K("c"),K("d"),K("e")]);
  expect(plan(0.1)).toEqual([K("b"),K("c"),K("d"),K("e")]);
  setPointer(root,[{generation:K("b"),atMs:now-25*H}]);expect(plan(1)).toEqual([K("b"),K("c"),K("d"),K("e")]);
  expect(planDependencyGenerationCollectionV1(root,{freeFraction:()=>0.1,now:()=>now}).pressure).toBe(true);expect(planDependencyGenerationCollectionV1(root,{freeFraction:()=>0.25,now:()=>now}).pressure).toBe(false);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("a store of any size is paged through in bounded batches with progress and never deferred wholesale",async()=>{
 const{root,make}=store();
 try{
  for(const c of ["c","d","e"]){make(K(c),true,99999);fat(root,K(c),250);}
  const total=entriesOf(join(root,"j")),rows:number[]=[];
  const result=await collectDependencyGenerationsV1(root,{freeFraction:()=>1,dryRun:false,batchEntries:128,progress:row=>{rows.push(row.entries);}});
  expect(result.retired.map(row=>row.key)).toEqual([K("c"),K("d"),K("e")]);expect(result.more).toBe(false);expect(result.finished).toEqual([]);
  expect(result.entries).toBeGreaterThan(4096*3);expect(result.entries).toBe(total-3);
  expect(rows.length).toBeGreaterThan(Math.floor(result.entries/128));expect(rows).toEqual([...rows].sort((left,right)=>left-right));
  expect(readdirSync(join(root,"j"))).toEqual([]);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("a cancelled removal leaves a tombstone that the next run finishes first, entry for entry",async()=>{
 const{root,make}=store();
 try{
  make(K("c"),true,99999);fat(root,K("c"),60);
  const total=entriesOf(join(root,"j")),stop=new AbortController();let last=0,ticks=0;
  await expect(collectDependencyGenerationsV1(root,{freeFraction:()=>1,dryRun:false,batchEntries:128,signal:stop.signal,progress:row=>{last=row.entries;if(++ticks===3)stop.abort();}})).rejects.toThrow();
  expect(readdirSync(join(root,"j"))).toEqual([policy.tombstonePrefix+K("c")]);expect(last).toBe(384);
  expect(planDependencyGenerationCollectionV1(root,{freeFraction:()=>1}).tombstones).toEqual([K("c")]);
  const dry=await collectDependencyGenerationsV1(root,{freeFraction:()=>1,dryRun:true});expect(dry.finished).toEqual([K("c")]);expect(readdirSync(join(root,"j")).length).toBe(1);
  const again=await collectDependencyGenerationsV1(root,{freeFraction:()=>1,dryRun:false,batchEntries:128});
  expect(again.finished).toEqual([K("c")]);expect(again.retired).toEqual([]);expect(again.entries+last+1).toBe(total);expect(readdirSync(join(root,"j"))).toEqual([]);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("a time budget stops between batches without losing anything, says more follow, and the next run resumes",async()=>{
 const{root,make}=store();
 try{
  make(K("c"),true,99999);make(K("d"),true,99999);fat(root,K("c"),40);fat(root,K("d"),40);
  let time=0;const first=await collectDependencyGenerationsV1(root,{freeFraction:()=>1,dryRun:false,batchEntries:64,maximumMilliseconds:25,clock:()=>time,progress:()=>{time+=10;}});
  expect(first.more).toBe(true);expect(first.retired).toEqual([]);expect(readdirSync(join(root,"j")).sort()).toEqual([policy.tombstonePrefix+K("c"),K("d")]);
  const second=await collectDependencyGenerationsV1(root,{freeFraction:()=>1,dryRun:false});
  expect(second.more).toBe(false);expect(second.finished).toEqual([K("c")]);expect(second.retired.map(row=>row.key)).toEqual([K("d")]);expect(readdirSync(join(root,"j"))).toEqual([]);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("under pressure the retired generations of a pointer are collected, a held one is not, and the finished tree is gone on the python side too",async()=>{
 const{root,make}=store(),now=Date.now();
 try{
  make(K("a"),true,99999);make(K("b"),true,99999);make(K("c"),true,99999);
  setPointer(root,[{generation:K("b"),atMs:now-1000},{generation:K("c"),atMs:now-2000}]);holdDependencyGenerationsV1(root,[K("c")],process.pid);
  expect((await collectDependencyGenerationsV1(root,{freeFraction:()=>1,dryRun:false})).retired).toEqual([]);
  const done=await collectDependencyGenerationsV1(root,{freeFraction:()=>0.05,dryRun:false});
  expect(done.pressure).toBe(true);expect(done.retired.map(row=>row.key)).toEqual([K("b")]);
  const gone=spawnSync("python3",["-I","-c","import os,sys\nprint(sorted(os.listdir(sys.argv[1])))",join(root,"j")],{encoding:"utf8"}).stdout.trim();
  expect(gone).toBe("['"+K("a")+"', '"+K("c")+"']");
 }finally{rmSync(root,{recursive:true,force:true});}
});
