import{afterAll,expect,test}from"bun:test";
import{mkdirSync,mkdtempSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{createHash}from"node:crypto";
import{startRemoteCacheServerV1}from"../../../../🌐️remote-cache/🖥️server/🟦️.ts";
import{createProjectFleetPortV1,type ProjectRecordInfoV1}from"../../🟦️.ts";

/** ✂️ Contract: the records a slice worker finished before the pool cut its load smaller (and stopped it before it could publish them) are on disk, and the next worker offers them to the fleet: another machine takes them, and a record already published is not sent twice.
 * @see ../../../../🧠️memory/✂️cut/🧬️schema/🔣️.json */
const scratch:string[]=[];
const temp=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`after-cut-${label}-`));scratch.push(path);return path;};
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const sha=(text:string)=>createHash("sha256").update(text).digest("hex");
const key=JSON.stringify(["rev",null,[["authority"],"libs/alpha",{name:"alpha"}]]);
const infoOf=(root:string,store:{record?:unknown;imported?:unknown[]}):ProjectRecordInfoV1=>({cachePath:join(root,"np","alpha.json"),fleet:"libs/alpha",keyText:key,exportRecord:()=>store.record,async import(portable){(store.imported??=[]).push(portable);return true;}});

test("a record finished by a worker that was stopped by the cut is offered by the next worker at the same root, taken by another machine, and not sent twice",async()=>{
 const cache=temp("cache"),server=await startRemoteCacheServerV1({directory:cache,host:"127.0.0.1",port:0,maximumBytes:2**28,maximumEntryBytes:2**26,maximumAgeDays:30,defaultInstance:"main",tokens:{read:"r",write:"w"},validation:"off"});
 const environment={SEMIO_REMOTE_CACHE_ENDPOINT:server.url,SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_REMOTE_CACHE_TOKEN:"w",SEMIO_REMOTE_CACHE_INSTANCE:"main"};
 try{
  const rootA=temp("a"),rootB=temp("b");mkdirSync(join(rootA,"np"),{recursive:true});
  const finished={digest:sha("d"),authority:[],observations:[],image:JSON.stringify({name:"alpha",root:"libs/alpha"})},store:{record?:unknown}={record:finished},info=infoOf(rootA,store);
  writeFileSync(info.cachePath,"1");
  const second=createProjectFleetPortV1({root:rootA,environment,owner:"a:2"})!,taker=createProjectFleetPortV1({root:rootB,environment,owner:"b:1"})!;
  second.offer(info);await second.flush();expect(second.stats().offered).toBe(1);
  second.offer(info);await second.flush();expect(second.stats().offered).toBe(1);
  const taken:{imported?:unknown[]}={},gate=await taker.gate(infoOf(rootB,taken));
  expect(gate.seeded).toBe(true);expect(taker.stats().seeded).toBe(1);expect(JSON.stringify(taken.imported)).toContain("alpha");
 }finally{await server.stop();}
},60000);
