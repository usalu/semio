import{afterAll,expect,test}from"bun:test";
import{mkdtempSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{startRemoteCacheServerV1}from"../../../../../../🌐️remote-cache/🖥️server/🟦️.ts";
import{OWNER_POOL_POLICY_V1 as P,ownerPoolKeepAliveV1,type OwnerPoolManifestV1,type OwnerPoolUnitV1}from"../../🟦️.ts";
import{createOwnerPoolPublisherV1}from"../../📣️publisher/🟦️.ts";

const scratch:string[]=[];
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const temp=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`owner-pool-alive-${label}-`));scratch.push(path);return path;};
const pause=(milliseconds:number)=>new Promise<void>(done=>setTimeout(done,milliseconds));
const unitsOf=(count:number):OwnerPoolUnitV1[]=>Array.from({length:count},(_all,index)=>({name:`owner-${String(index).padStart(4,"0")}`,kind:"k",expectedMilliseconds:400,json:{name:`owner-${index}`}}));

test("the keep-alive rule fires once a published wave is older than the declared interval and never before a first publication",()=>{
 expect(ownerPoolKeepAliveV1({published:undefined,sinceMs:P.republishKeepAliveMilliseconds*10})).toBe(false);
 expect(ownerPoolKeepAliveV1({published:500,sinceMs:P.republishKeepAliveMilliseconds-1})).toBe(false);
 expect(ownerPoolKeepAliveV1({published:500,sinceMs:P.republishKeepAliveMilliseconds})).toBe(true);
});

test("a reader whose take pass is long keeps its unchanged wave on the pointer: the same wave is put again each interval, from the take reporter and from the running owners, and not after the read finished",async()=>{
 const cache=temp("cache"),server=await startRemoteCacheServerV1({directory:cache,host:"127.0.0.1",port:0,maximumBytes:2**28,maximumEntryBytes:2**26,maximumAgeDays:30,defaultInstance:"main",tokens:{read:"r",write:"w"},validation:"off"});
 try{
  const environment={...process.env,SEMIO_REMOTE_CACHE_ENDPOINT:server.url,SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_REMOTE_CACHE_TOKEN:"w",SEMIO_REMOTE_CACHE_INSTANCE:"main"},published:OwnerPoolManifestV1[]=[],clock={at:1000000};
  const publisher=createOwnerPoolPublisherV1({workspaceRoot:temp("root"),environment,identity:"id",globals:{projectAuthority:"a",registryDigest:"r",commandRevision:"c",late:false},session:()=>({tag:"s"}),units:()=>unitsOf(500),now:()=>clock.at,publish:async input=>{published.push(input.manifest);return true;}});
  publisher.announce(500);await pause(50);expect(published.length).toBe(1);
  publisher.learn(500);clock.at+=P.republishKeepAliveMilliseconds-1;publisher.learn(500);await pause(50);
  expect(published.length,"inside the interval nothing is put again").toBe(1);
  clock.at+=1;publisher.learn(500);await pause(50);
  expect(published.length,"after the interval the take reporter puts the wave again").toBe(2);
  expect(published[1]!.head.id,"the same wave").toBe(published[0]!.head.id);expect(published[1]!.head.createdMs,"listed from now on").toBeGreaterThan(published[0]!.head.createdMs);
  publisher.learn(500);await pause(30);expect(published.length,"the attempt restarts the interval").toBe(2);
  clock.at+=P.republishKeepAliveMilliseconds;publisher.miss(3);await pause(50);
  expect(published.length,"a running owner counts too").toBe(3);
  publisher.finish({owners:500,seconds:1,consumedByHost:{}});clock.at+=P.republishKeepAliveMilliseconds*3;publisher.learn(500);publisher.miss(3);await pause(50);
  expect(published.length,"a finished read keeps nothing alive").toBe(3);
 }finally{await server.stop();}
},60000);
