import{afterAll,expect,test}from"bun:test";
import{mkdirSync,mkdtempSync,readFileSync,rmSync,writeFileSync,existsSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{createHash}from"node:crypto";
import Ajv from"ajv";
import{startRemoteCacheServerV1}from"../../../🌐️remote-cache/🖥️server/🟦️.ts";
import{createProjectFleetPortV1,projectRecordStateV1,runTakeLookV1,type ProjectRecordInfoV1}from"../🟦️.ts";
import{graphFleetClientV1}from"../../🛰️exchange/🟦️.ts";
import{graphFleetSettingsV1}from"../../🧶️nx/🟦️.ts";
import schema from"../🧬️schema/🔣️.json"with{type:"json"};
import{observationRowsHoldV1,withObservedWorldV1}from"../../../👁️observation/🟨️.mjs";

/** 📦️ A project record travels through the fleet cache: stored once under a root-neutral key with one slot per project, taken by a machine at another root, computed once when two machines ask at the same moment, never accepted without the reading machine's own check of every observation. */
const scratch:string[]=[];
const temp=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`project-records-${label}-`));scratch.push(path);return path;};
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const sha=(text:string)=>createHash("sha256").update(text).digest("hex");
const base=(root:string)=>({root,fleet:"libs/alpha",keyText:JSON.stringify(["rev",null,[["authority"],root+"/libs/alpha",{name:"alpha"}]]),platform:"darwin-arm64"});

test("the state is equal at every root and differs when the key or the project differs and not with the platform, and names nothing of the producer's machine or module-load state; each project has its own slot",()=>{
 const a=projectRecordStateV1(base("/Users/Shared/semio")),b=projectRecordStateV1(base("/srv/other-root"));
 expect(a.digest).toBe(b.digest);expect(a.frame.implementation).toBe(b.frame.implementation);
 const edit={...base("/Users/Shared/semio")};
 expect(projectRecordStateV1({...edit,keyText:edit.keyText.replace("alpha","beta")}).digest).not.toBe(a.digest);
  expect(Object.keys(edit).sort()).toEqual(["fleet","keyText","platform","root"]);
 expect(projectRecordStateV1({...edit,platform:"linux-x64"}).digest).toBe(a.digest);expect(projectRecordStateV1({...edit,platform:"win32-x64"}).digest).toBe(a.digest);
 const other=projectRecordStateV1({...edit,fleet:"libs/beta"});
 expect(other.digest).not.toBe(a.digest);expect(other.frame.tooling).not.toBe(a.frame.tooling);
});

async function world(){
 const cache=temp("cache"),server=await startRemoteCacheServerV1({directory:cache,host:"127.0.0.1",port:0,maximumBytes:2**28,maximumEntryBytes:2**26,maximumAgeDays:30,defaultInstance:"main",tokens:{read:"r",write:"w"},validation:"off"});
 const environment={SEMIO_REMOTE_CACHE_ENDPOINT:server.url,SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_REMOTE_CACHE_TOKEN:"w",SEMIO_REMOTE_CACHE_INSTANCE:"main"};
 return{server,environment};
}
const infoOf=(root:string,store:{record?:unknown;imported?:unknown[]}):ProjectRecordInfoV1=>{const b=base(root);return{cachePath:join(root,"np","alpha.json"),fleet:b.fleet,keyText:b.keyText,exportRecord:()=>store.record,async import(portable){(store.imported??=[]).push(portable);return true;}};};

test("gate telemetry distinguishes a take blocked on transport from a normal owner blocked on its claim",async()=>{
 const {server,environment}=await world(),root=temp("gate-times");
 const client=graphFleetClientV1(graphFleetSettingsV1(root,environment)!);
 const metadata=Promise.withResolvers<void>(),claimStarted=Promise.withResolvers<void>(),allowMetadata=Promise.withResolvers<void>(),allowClaim=Promise.withResolvers<void>();
 const validate=new Ajv({strict:false}).compile({...schema,$ref:"#/definitions/GateTimings"});
 const port=createProjectFleetPortV1({root,environment,client:{
  ...client,
  async getResult(...args){metadata.resolve();await allowMetadata.promise;return client.getResult(...args);},
  async claim(...args){claimStarted.resolve();await allowClaim.promise;return client.claim(...args);},
 }})!;
 const taken=runTakeLookV1(()=>port.gate(infoOf(root,{})));
 let inferred:ReturnType<typeof port.gate>|undefined;
 try{
  await metadata.promise;
  const awaitingMetadata=port.stats().gateTimings;
  expect(validate(awaitingMetadata)).toBe(true);
  expect(awaitingMetadata.metadata).toMatchObject({started:1,active:1});
  expect(awaitingMetadata.claim.started).toBe(0);
  allowMetadata.resolve();
  expect((await taken).seeded).toBe(false);
  expect(port.stats().gateTimings.metadata.active).toBe(0);
  inferred=port.gate(infoOf(root,{}));await claimStarted.promise;
  const awaitingClaim=port.stats().gateTimings;
  expect(awaitingClaim.metadata).toMatchObject({started:2,active:0});
  expect(awaitingClaim.claim).toMatchObject({started:1,active:1});
  expect(awaitingClaim.retry.started).toBe(0);
  allowClaim.resolve();await (await inferred).abandon();
  expect(port.stats().gateTimings.claim.active).toBe(0);
 }finally{allowMetadata.resolve();allowClaim.resolve();await Promise.allSettled([taken,inferred]);await server.stop();}
});

test("a record computed at one root is taken by a machine at another root with that machine's paths, and a record nobody has is computed once under a claim",async()=>{
 const {server,environment}=await world();
 try{
  const rootA=temp("a"),rootB=temp("b"),rootC=temp("c"),a=createProjectFleetPortV1({root:rootA,environment,owner:"a:1"})!,b=createProjectFleetPortV1({root:rootB,environment,owner:"b:1"})!,c=createProjectFleetPortV1({root:rootC,environment,owner:"c:1"})!;
  const portableOf=(root:string)=>({digest:sha("d"),authority:[{kind:"file",path:`${root}/impl/a.mjs`,fact:{sha256:"aaa"}}],observations:[{kind:"file",path:`${root}/libs/alpha/src/a.ts`,fact:{sha256:"x",size:1,mtimeMs:1,ctimeMs:1,ino:1}}],image:JSON.stringify({name:"alpha",root:"libs/alpha"})});
  const storeA:{record?:unknown;imported?:unknown[]}={record:portableOf(rootA)},storeB:{record?:unknown;imported?:unknown[]}={},storeC:{record?:unknown;imported?:unknown[]}={};
  const gateA=await a.gate(infoOf(rootA,storeA));expect(gateA.seeded).toBe(false);
  const gateB=b.gate(infoOf(rootB,storeB));
  await Bun.sleep(150);
  gateA.publish();await a.flush();
  const taken=await gateB;expect(taken.seeded).toBe(true);
  const record=storeB.imported![0] as ReturnType<typeof portableOf>;
  expect(record.observations[0]!.path).toBe(`${rootB}/libs/alpha/src/a.ts`);expect(JSON.stringify(record)).not.toContain(rootA);
  expect(b.stats().seeded).toBe(1);expect(b.stats().waited).toBe(1);expect(a.stats().published).toBe(1);
  const late=await c.gate(infoOf(rootC,storeC));expect(late.seeded).toBe(true);expect(JSON.stringify(storeC.imported)).toContain(rootC);
  expect(await c.flush()).toContain("1");
 }finally{await server.stop();}
},60000);

test("a record that does not hold in the taker's tree is replaced by the taker's own, and a record changed locally is offered again",async()=>{
 const {server,environment}=await world();
 try{
  const rootA=temp("ra"),rootB=temp("rb"),rootC=temp("rc"),a=createProjectFleetPortV1({root:rootA,environment,owner:"a:1"})!,b=createProjectFleetPortV1({root:rootB,environment,owner:"b:1"})!,c=createProjectFleetPortV1({root:rootC,environment,owner:"c:1"})!;
  const record=(tag:string)=>({digest:sha("d"),authority:[],observations:[],image:JSON.stringify({name:"alpha",tag})});
  const storeA:{record?:unknown}={record:record("old")},storeB:{record?:unknown;imported?:unknown[]}={record:record("new")},storeC:{record?:unknown;imported?:unknown[]}={};
  const infoA=infoOf(rootA,storeA);mkdirSync(join(rootA,"np"),{recursive:true});writeFileSync(infoA.cachePath,"1");
  a.offer(infoA);await a.flush();expect(a.stats().offered).toBe(1);
  a.offer(infoA);await a.flush();expect(a.stats().offered).toBe(1);
  const gateB=await b.gate(infoOf(rootB,storeB));expect(gateB.seeded).toBe(true);
  gateB.publish();await b.flush();expect(b.stats().published).toBe(1);
  const taken=await c.gate(infoOf(rootC,storeC));expect(taken.seeded).toBe(true);
  expect(JSON.stringify(storeC.imported)).toContain("new");expect(JSON.stringify(storeC.imported)).not.toContain("old");
  storeA.record=record("newer");writeFileSync(infoA.cachePath,"123456");
  a.offer(infoA);await a.flush();expect(a.stats().offered).toBe(2);
 }finally{await server.stop();}
},60000);

test("an unreachable cache or an unusable record ends in computing locally, an offered record is stored once and remembered",async()=>{
 const dead=createProjectFleetPortV1({root:temp("d"),environment:{SEMIO_REMOTE_CACHE_ENDPOINT:"http://127.0.0.1:9",SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_REMOTE_CACHE_TOKEN:"w",SEMIO_REMOTE_CACHE_INSTANCE:"main"}})!;
 const gone=await dead.gate(infoOf("/nowhere",{}));expect(gone.seeded).toBe(false);await gone.abandon();
 const {server,environment}=await world();
 try{
  const root=temp("o"),port=createProjectFleetPortV1({root,environment,owner:"o:1"})!,store:{record?:unknown}={record:{digest:sha("o"),authority:[],observations:[],image:"{}"}},info=infoOf(root,store);
  mkdirSync(join(root,"np"),{recursive:true});
  port.offer(info);await port.flush();expect(port.stats().offered).toBe(1);expect(existsSync(info.cachePath+".fleet")).toBe(true);
  port.offer(info);await port.flush();expect(port.stats().offered).toBe(1);
  const refusing=createProjectFleetPortV1({root:temp("r"),environment,owner:"r:1"})!,refused=await refusing.gate({...infoOf("/x",{}),import:async()=>{throw Error("tampered");}});
  expect(refused.seeded).toBe(false);
 }finally{await server.stop();}
},60000);

test("a seeded record is only a candidate: the seam looks it up again against this tree before it returns it, and a changed opened file refutes it while a file outside the closure does not",()=>{
 const source=readFileSync(new URL("../../../👁️observation/🟨️.mjs",import.meta.url),"utf8"),at=source.indexOf("if(!reused&&port){gate=await lookupLapAsyncV1(\"gate\",()=>port.gate(info))");
 expect(at).toBeGreaterThan(0);
 expect(source.slice(at,at+500)).toContain("if(gate?.seeded)reused=await lookup();");
 expect(source.slice(at-300,at)).toContain("let reused=await lookup();");
 const directory=temp("law"),inside=join(directory,"in.ts"),outside=join(directory,"out.ts");
 writeFileSync(inside,"export const a=1");writeFileSync(outside,"export const b=1");
 const row=(path:string)=>({kind:"file",path,fact:{sha256:sha(readFileSync(path,"utf8")),size:readFileSync(path).length,mtimeMs:1,ctimeMs:1,ino:1}});
 const rows=[row(inside)];
 let refuted=0,kept=0;
 for(let round=0;round<40;round++){
  const target=round%2===0?inside:outside,before=readFileSync(target,"utf8");
  writeFileSync(target,before+"//"+round);
  const holds=withObservedWorldV1(undefined as never,()=>observationRowsHoldV1(rows));
  if(target===inside){expect(holds).toBe(false);refuted++;writeFileSync(inside,"export const a=1");}else{expect(holds).toBe(true);kept++;}
 }
 expect(refuted).toBe(20);expect(kept).toBe(20);
});

test("a burst of offers is not dropped: every record reaches the cache although more than the pending limit is offered at once, and the progress line says how many were taken and inferred",async()=>{
 const {server,client:_c}=await world();
 try{
  const root=temp("burst"),lines:string[]=[],port=createProjectFleetPortV1({root,environment:{...environment0(server),SEMIO_LANGUAGE:"de"},owner:"b:1",say:line=>lines.push(line)})!;
  mkdirSync(join(root,"np"),{recursive:true});
  const total=150;
  for(let index=0;index<total;index++){
   const info={...infoOf(root,{record:{digest:sha("d"+index),authority:[],observations:[],image:JSON.stringify({index})}}),fleet:"libs/p"+index,cachePath:join(root,"np","p"+index+".json")};
   writeFileSync(info.cachePath,String(index));
   await port.offer(info);
  }
  await port.flush();expect(port.stats().offered).toBe(total);
  for(let index=0;index<50;index++){const gate=await port.gate({...infoOf(root,{}),fleet:"libs/p"+index,cachePath:join(root,"np","q"+index+".json")});expect(gate.seeded).toBe(true);}
  expect(lines).toEqual(["Graph: 50 Projekte aus der Flotte übernommen, dazu 0 Closure-Einträge, davon 0 hier nicht gültig und neu berechnet, 0 nicht geteilt, 0 hier berechnet"]);
 }finally{await server.stop();}
},120000);
const environment0=(server:{url:string})=>({SEMIO_REMOTE_CACHE_ENDPOINT:server.url,SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_REMOTE_CACHE_TOKEN:"w",SEMIO_REMOTE_CACHE_INSTANCE:"main"});
