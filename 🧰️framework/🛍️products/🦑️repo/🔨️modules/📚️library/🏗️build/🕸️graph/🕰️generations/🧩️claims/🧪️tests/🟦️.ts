import{expect,test}from"bun:test";
import Ajv2020 from"ajv";
import{mkdirSync,mkdtempSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{dirname,join}from"node:path";
import{fileURLToPath}from"node:url";
import{testPythonV1}from"../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import{CLAIM_MESSAGES_V1,CLAIMS_POLICY_V1,chunkOfV1,createClaimRegistryV1,createFileClaimLeasePortV1,prepareStaleProjectsV1,readClaimV1,renderClaimMessageV1,slotCountV1,type ClaimLeasePortV1,type ClaimV1}from"../🟨️.mjs";
import{createNodeFilePortV1}from"../../🟨️.mjs";
import schema from"../🧬️schema/🔣️.json";
import fixture from"./🔣️.json";

const python=testPythonV1(),oracle=join(dirname(fileURLToPath(import.meta.url)),"🐍️oracle/🐍️.py");
const model=(command:string,input:unknown)=>{const run=Bun.spawnSync([python!,"-I",oracle,command],{stdin:Buffer.from(JSON.stringify(input))});expect(run.stderr.toString()).toBe("");expect(run.exitCode).toBe(0);return JSON.parse(run.stdout.toString());};
const IMPLEMENTATION="a".repeat(64);

test("the fixture, the policy and the texts follow the schema file",()=>{
 expect(new Ajv2020({strict:false}).validate({$ref:"#/definitions/Fixture",definitions:schema.definitions},fixture)).toBe(true);
 expect(CLAIMS_POLICY_V1).toEqual(schema.definitions.Policy.const as typeof CLAIMS_POLICY_V1);
 expect(CLAIM_MESSAGES_V1).toEqual(schema.definitions.Messages.const as typeof CLAIM_MESSAGES_V1);
 expect(Object.keys(CLAIM_MESSAGES_V1.en)).toEqual(Object.keys(CLAIM_MESSAGES_V1.de));
 for(const key of Object.keys(CLAIM_MESSAGES_V1.en) as (keyof typeof CLAIM_MESSAGES_V1.en)[]){
  const fields=Object.fromEntries([...new Set([...CLAIM_MESSAGES_V1.en[key].matchAll(/\{([a-z]+)\}/gu),...CLAIM_MESSAGES_V1.de[key].matchAll(/\{([a-z]+)\}/gu)].map(match=>match[1]!))].map(name=>[name,7]));
  for(const language of["en","de"] as const)expect(renderClaimMessageV1(key,fields,language)).not.toMatch(/[{}]/u);
 }
 expect(()=>renderClaimMessageV1("nothing" as never,{},"en")).toThrow();
});

for(const row of fixture.chunks)test("the chunk equals the Python model and the stated law: "+row.id,()=>{
 const actual=chunkOfV1({wanted:row.wanted,claims:row.claims as never,base:row.base,implementation:row.implementation,chunk:row.chunk,tail:row.tail,...(row as {stamps?:Record<string,string>}).stamps?{stamps:(row as {stamps:Record<string,string>}).stamps}:{},...(row as {nowMs?:number}).nowMs!==undefined?{nowMs:(row as {nowMs:number}).nowMs}:{}});
 expect(actual).toEqual(model("chunk",row));
 if("expected" in row)expect(actual).toEqual(row.expected as never);
 expect(actual.take).toEqual(actual.open.slice(0,actual.take.length));
 expect(actual.open.length-actual.take.length===0||actual.take.length===row.chunk).toBe(true);
});

for(const row of fixture.slots)test("the slot count equals the Python model: "+row.id,()=>{
 const slots=slotCountV1({memoryMegabytes:row.memoryMegabytes,environment:row.environment});
 expect({slots}).toEqual(model("slots",row));
 expect(slots).toBeGreaterThanOrEqual(1);expect(slots).toBeLessThanOrEqual(CLAIMS_POLICY_V1.maximumSlots);
});

test("the slot count never decreases with memory and is 2 on 16 GB",()=>{
 let last=0;for(let gb=0;gb<=256;gb++){const slots=slotCountV1({memoryMegabytes:gb*1024});expect(slots).toBeGreaterThanOrEqual(last);last=slots;}
 expect(slotCountV1({memoryMegabytes:16384})).toBe(2);expect(slotCountV1({memoryMegabytes:8192})).toBe(1);expect(slotCountV1({memoryMegabytes:32768})).toBe(4);
});

const valid:ClaimV1={version:1,token:"0".repeat(24),base:3,implementation:IMPLEMENTATION,owners:["a","b"],state:"running",startedMs:5};
test("a valid claim follows the schema and is admitted by both implementations",()=>{
 expect(new Ajv2020({strict:false}).validate({$ref:"#/definitions/Claim",definitions:schema.definitions},valid)).toBe(true);
 expect(readClaimV1(valid)).toEqual(valid);expect(model("claim",{claim:valid})).toEqual({accepted:true});
 expect(readClaimV1({...valid,endedMs:9,state:"done"}).endedMs).toBe(9);
});
for(const row of fixture.refusals)test("a malformed claim is refused by both implementations: "+row.id,()=>{
 expect(()=>readClaimV1(row.claim)).toThrow();
 expect(model("claim",{claim:row.claim})).toEqual({accepted:false});
});

for(const[label,config]of[
 ["three overlapping commands, chunk 2, one crash",{participants:[["a","b","c","d"],["b","c","e","f"],["a","f"]],chunk:2,tail:0,crashes:1}],
 ["two identical commands, chunk 1, two crashes",{participants:[["a","b","c"],["a","b","c"]],chunk:1,tail:0,crashes:2}],
 ["four commands, disjoint and shared, chunk 2 with tail 1",{participants:[["a","b","c","d","e"],["d","e","f"],["g"],["a","g"]],chunk:2,tail:1,crashes:1}],
]as const)test("every interleaving keeps the claims disjoint and finishes every surviving command: "+label,()=>{
 const result=model("explore",config);
 expect(result.violationCount).toBe(0);expect(result.terminals).toBeGreaterThan(0);expect(result.states).toBeGreaterThan(50);
});
test("the explorer detects a protocol that ignores running claims",()=>{
 const result=model("explore",{participants:[["a","b","c","d"],["b","c","e","f"],["a","f"]],chunk:2,tail:0,crashes:1,broken:"ignore-running"});
 expect(result.violationCount).toBeGreaterThan(0);expect(result.violations[0][0]).toBe("overlap");
});

const tick=(milliseconds:number)=>new Promise<void>(resolve=>setTimeout(resolve,milliseconds));
function deferred(){let resolve!:()=>void,reject!:(error:unknown)=>void;const promise=new Promise<void>((done,fail)=>{resolve=done;reject=fail;});return{promise,resolve,reject};}
function scenario(){
 const root=mkdtempSync(join(tmpdir(),"claims-")),leases=join(root,"leases"),fs=createNodeFilePortV1(),log:string[]=[];
 const command=(name:string,options:{wait?:(owners:string[])=>Promise<void>}={})=>{
  const held=new Set<{release():void}>(),port=createFileClaimLeasePortV1(leases),lease:ClaimLeasePortV1&{die():void}={
   async tryHold(resource){const hold=await port.tryHold(resource);if(!hold)return undefined;const wrapped={release(){held.delete(wrapped);hold.release();}};held.add(wrapped);return wrapped;},
   async hold(resource,o){const hold=await port.hold(resource,o);const wrapped={release(){held.delete(wrapped);hold.release();}};held.add(wrapped);return wrapped;},
   wait:(resource,o)=>port.wait(resource,o),
   die(){for(const hold of[...held])hold.release();},
  };
  const registry=createClaimRegistryV1({root:join(root,"generations"),fs,lease});
  const prefetch=(wanted:string[],extra:Record<string,unknown>={})=>registry.prefetch({base:4,implementation:IMPLEMENTATION,wanted,slots:4,run:async(owners:string[])=>{log.push(name+":"+owners.join(","));await(options.wait?.(owners)??tick(5));},...extra} as never);
  return{registry,lease,prefetch};
 };
 return{root,log,command,done:()=>rmSync(root,{recursive:true,force:true})};
}
const computed=(log:string[])=>log.flatMap(line=>line.split(":")[1]!.split(","));

test("three commands with overlapping stale owners compute every owner exactly once and each finishes",async()=>{
 const s=scenario();
 try{
  const names=Array.from({length:40},(_,at)=>"p"+String(at).padStart(2,"0")),a=s.command("A"),b=s.command("B"),c=s.command("C");
  const results=await Promise.all([a.prefetch(names.slice(0,30)),b.prefetch(names.slice(10,40)),c.prefetch(names)]);
  const owners=computed(s.log);expect(owners.sort()).toEqual(names);expect(new Set(owners).size).toBe(owners.length);
  expect(results.reduce((sum,row)=>sum+row.computed,0)).toBe(40);
  const claims=await a.registry.list();expect(claims.every(claim=>claim.state==="done")).toBe(true);expect(claims.flatMap(claim=>[...claim.owners]).sort()).toEqual(names);
 }finally{s.done();}
});

test("a request whose stale owners a peer already computed joins it and computes nothing",async()=>{
 const s=scenario();
 try{
  const a=s.command("A"),b=s.command("B"),names=["x1","x2","x3"];
  await a.prefetch(names);const later=await b.prefetch(["x2","x3"]);
  expect(later).toEqual({computed:0,served:false});expect(s.log).toEqual(["A:x1,x2,x3"]);
 }finally{s.done();}
});

test("a request with a running peer waits for it and then computes only what is left",async()=>{
 const s=scenario();
 try{
  const gate=deferred(),say:string[]=[],a=s.command("A",{wait:()=>gate.promise}),b=s.command("B");
  const first=a.prefetch(["m1","m2"]);await tick(60);
  const second=b.prefetch(["m1","m2","m3"],{say:(key:string)=>say.push(key),slots:4});
  await tick(120);expect(s.log).toEqual(["A:m1,m2","B:m3"]);expect(say).toContain("joined");
  gate.resolve();await Promise.all([first,second]);
  expect(computed(s.log).sort()).toEqual(["m1","m2","m3"]);
 }finally{s.done();}
});

test("disjoint stale sets compute at the same time when slots allow and one after the other when they do not",async()=>{
 const s=scenario();
 try{
  const gate=deferred(),a=s.command("A",{wait:()=>gate.promise}),b=s.command("B",{wait:()=>gate.promise});
  const first=a.prefetch(["a1","a2"]),second=b.prefetch(["b1","b2"]);await tick(150);
  expect(s.log.sort()).toEqual(["A:a1,a2","B:b1,b2"]);gate.resolve();await Promise.all([first,second]);
 }finally{s.done();}
 const t=scenario();
 try{
  const gate=deferred(),a=t.command("A",{wait:()=>gate.promise}),b=t.command("B",{wait:()=>gate.promise});
  const first=a.prefetch(["a1","a2"],{slots:1}),second=b.prefetch(["b1","b2"],{slots:1});await tick(150);
  expect(t.log).toHaveLength(1);gate.resolve();await Promise.all([first,second]);expect(t.log).toHaveLength(2);
 }finally{t.done();}
});

test("a crashed command's chunk is taken over by a joiner",async()=>{
 const s=scenario();
 try{
  const hang=deferred(),a=s.command("A",{wait:()=>hang.promise}),b=s.command("B"),abort=new AbortController();
  const lost=a.prefetch(["k1","k2","k3"],{signal:abort.signal}).catch(()=>undefined);await tick(60);
  expect(s.log).toEqual(["A:k1,k2,k3"]);
  const joined=b.prefetch(["k1","k2","k3","k4"]);await tick(120);expect(s.log).toEqual(["A:k1,k2,k3","B:k4"]);
  a.lease.die();
  const result=await joined;expect(result.computed).toBe(4);expect(s.log).toEqual(["A:k1,k2,k3","B:k4","B:k1,k2,k3"]);
  abort.abort(new Error("done"));hang.reject(new Error("dead"));await lost;
 }finally{s.done();}
});

test("a failing chunk is marked failed, reported to its command and left to the assembly for the others",async()=>{
 const s=scenario();
 try{
  const a=s.command("A",{wait:async()=>{throw new Error("inference exploded");}}),b=s.command("B");
  await expect(a.prefetch(["f1","f2"])).rejects.toThrow("inference exploded");
  expect((await a.registry.list()).map(claim=>claim.state)).toEqual(["failed"]);
  expect(await b.prefetch(["f1","f2","f3"])).toEqual({computed:1,served:false});expect(s.log).toEqual(["A:f1,f2","B:f3"]);
 }finally{s.done();}
});

test("a generation published by a peer ends the prefetch at once",async()=>{
 const s=scenario();
 try{
  const a=s.command("A"),say:[string,unknown][]=[];let polls=0;
  const result=await a.prefetch(["s1","s2"],{served:async()=>++polls>1?9:undefined,say:(key:string,fields:unknown)=>say.push([key,fields])});
  expect(result.served).toBe(true);expect(say.at(-1)).toEqual(["served",{generation:9}]);expect(computed(s.log)).toEqual(["s1","s2"]);
 }finally{s.done();}
});

test("cancellation while waiting for a peer rejects and leaves the peer's claim alone",async()=>{
 const s=scenario();
 try{
  const gate=deferred(),a=s.command("A",{wait:()=>gate.promise}),b=s.command("B"),abort=new AbortController();
  const first=a.prefetch(["c1"]);await tick(60);
  const waiting=b.prefetch(["c1","c2"],{signal:abort.signal});await tick(80);abort.abort(new Error("cancelled"));
  await expect(waiting).rejects.toThrow("cancelled");
  expect((await a.registry.list()).map(claim=>claim.state).sort()).toEqual(["done","running"]);
  gate.resolve();await first;
 }finally{s.done();}
});

test("big deltas are cut into chunks of the policy size with a short tail folded in",async()=>{
 const s=scenario();
 try{
  const names=Array.from({length:300},(_,at)=>"q"+String(at).padStart(3,"0")),a=s.command("A");
  await a.prefetch(names);expect(s.log.map(line=>line.split(":")[1]!.split(",").length)).toEqual([128,128,44]);
 }finally{s.done();}
});

test("the sweep removes finished claims and keeps the claim of a live command",async()=>{
 const s=scenario();
 try{
  const gate=deferred(),a=s.command("A",{wait:()=>gate.promise}),b=s.command("B");
  await b.prefetch(["w1"]);const live=a.prefetch(["w2"]);await tick(60);
  await b.registry.sweep();expect((await b.registry.list()).map(claim=>[...claim.owners].join())).toEqual(["w2"]);
  gate.resolve();await live;
 }finally{s.done();}
});

test("a prefetch through the guard never fails its command and still rethrows a cancellation",async()=>{
 const s=scenario();
 try{
  const a=s.command("A",{wait:async()=>{throw new Error("inference\nexploded");}}),said:[string,Record<string,string|number>][]=[],abort=new AbortController();
  const result=await prepareStaleProjectsV1({registry:a.registry,base:4,implementation:IMPLEMENTATION,wanted:["g1"],slots:2,run:async owners=>{s.log.push("A:"+owners.join(","));throw new Error("inference\nexploded");},say:(key,fields)=>said.push([key,fields])});
  expect(result).toEqual({computed:0,served:false,failed:true});expect(said.at(-1)).toEqual(["failed",{reason:"inference exploded"}]);
  expect(renderClaimMessageV1("failed",said.at(-1)![1],"de")).toContain("inference exploded");
  abort.abort(new Error("cancelled"));
  await expect(prepareStaleProjectsV1({registry:a.registry,base:4,implementation:IMPLEMENTATION,wanted:["g2"],signal:abort.signal,run:async()=>{throw new Error("late");}})).rejects.toThrow("cancelled");
 }finally{s.done();}
});

function clocked(){
 const root=mkdtempSync(join(tmpdir(),"claims-clock-")),leases=join(root,"leases"),fs=createNodeFilePortV1(),clock={at:1_000_000},log:string[]=[];
 const registry=createClaimRegistryV1({root:join(root,"generations"),fs,lease:createFileClaimLeasePortV1(leases),now:()=>clock.at});
 const prefetch=(wanted:string[],extra:Record<string,unknown>={},fail=false)=>registry.prefetch({base:4,implementation:IMPLEMENTATION,wanted,slots:4,run:async(owners:string[])=>{log.push(owners.join(","));if(fail)throw new Error("ENOENT renamed mid-run");},...extra} as never);
 return{root,registry,clock,log,prefetch,done:()=>rmSync(root,{recursive:true,force:true})};
}

test("Law: a failed chunk is retaken by the next command after the retry time and by none before it, so one churn error never disables the prefetch of its owners for the rest of the plan",async()=>{
 const s=clocked();
 try{
  await expect(s.prefetch(["f1","f2"],{},true)).rejects.toThrow("ENOENT");
  expect(s.log).toEqual(["f1,f2"]);
  s.clock.at+=CLAIMS_POLICY_V1.failedRetryMilliseconds-1;
  expect(await s.prefetch(["f1","f2"]),"inside the retry time the failed claim covers its owners").toEqual({computed:0,served:false});
  expect(s.log).toEqual(["f1,f2"]);
  s.clock.at+=1;
  expect(await s.prefetch(["f1","f2"]),"after it a joiner computes them again").toEqual({computed:2,served:false});
  expect(s.log).toEqual(["f1,f2","f1,f2"]);
 }finally{s.done();}
});

test("Law: an owner edited after its prefetch is prepared again by the next command, while the unedited owners of the same claim stay prepared",async()=>{
 const s=clocked(),first={e1:"1".repeat(16),e2:"2".repeat(16),e3:"3".repeat(16)};
 try{
  await s.prefetch(["e1","e2","e3"],{stamps:first});
  expect(await s.prefetch(["e1","e2","e3"],{stamps:first}),"same content: nothing to do").toEqual({computed:0,served:false});
  const edited={...first,e2:"9".repeat(16)};
  expect(await s.prefetch(["e1","e2","e3"],{stamps:edited}),"e2 was edited").toEqual({computed:1,served:false});
  expect(s.log).toEqual(["e1,e2,e3","e2"]);
  const claims=await s.registry.list();
  expect(claims.map(claim=>claim.stamps)).toEqual(expect.arrayContaining([first,{e2:"9".repeat(16)}]));
 }finally{s.done();}
});

test("Law: a claim table beyond its bound is never silently truncated: the command answers complete and leaves the owners to the assembly instead of computing them twice",async()=>{
 const s=clocked(),claims=join(s.root,"generations",CLAIMS_POLICY_V1.directory);
 try{
  mkdirSync(claims,{recursive:true});
  for(let at=0;at<CLAIMS_POLICY_V1.maximumClaims+2;at++)writeFileSync(join(claims,at.toString(16).padStart(24,"0")+".json"),JSON.stringify({version:1,token:at.toString(16).padStart(24,"0"),base:4,implementation:IMPLEMENTATION,owners:["z"+at],state:"done",startedMs:1,endedMs:2}));
  const result=await s.prefetch(["fresh-owner"]);
  expect(result,"complete without computing").toEqual({computed:0,served:false});expect(s.log).toEqual([]);
 }finally{s.done();}
});

test("Law: the content stamp of an owner follows its files only: an edit changes the stamp of its owner, a file of a sibling owner changes nothing, an owner without a root is never equal to content",async()=>{
 const{ownerContentStampsV1}=await import("../../../🟦️.ts") as typeof import("../../../🟦️.ts");
 const root=mkdtempSync(join(tmpdir(),"claims-stamps-"));
 try{
  for(const file of["a/p.ts","a/q.ts","b/r.ts","c/s.ts"]){mkdirSync(join(root,file,".."),{recursive:true});writeFileSync(join(root,file),file);}
  const files=["a/p.ts","a/q.ts","b/r.ts","c/s.ts"],roots={A:"a",B:"b",C:"c"},owners=["A","B","C","N"];
  const before=await ownerContentStampsV1(root,files,roots,owners);
  expect(before.N).toBe("-");expect(Object.values(before).filter(value=>value!=="-").every(value=>/^[a-f0-9]{16}$/u.test(value))).toBe(true);
  expect(await ownerContentStampsV1(root,files,roots,owners),"stable without an edit").toEqual(before);
  writeFileSync(join(root,"a/q.ts"),"edited, longer");
  const after=await ownerContentStampsV1(root,files,roots,owners);
  expect(after.A).not.toBe(before.A);expect(after.B).toBe(before.B);expect(after.C).toBe(before.C);
  rmSync(join(root,"b/r.ts"));
  expect((await ownerContentStampsV1(root,files,roots,owners)).B,"a vanished file is a change too").not.toBe(before.B);
 }finally{rmSync(root,{recursive:true,force:true});}
},60000);
