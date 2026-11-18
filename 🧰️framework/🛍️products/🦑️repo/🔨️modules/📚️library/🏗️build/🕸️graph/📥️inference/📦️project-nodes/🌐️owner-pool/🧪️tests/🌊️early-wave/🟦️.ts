import{afterAll,expect,test}from"bun:test";
import{mkdtempSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{createHash}from"node:crypto";
import{startRemoteCacheServerV1}from"../../../../../../🌐️remote-cache/🖥️server/🟦️.ts";
import{createProjectFleetPortV1,type ProjectRecordInfoV1}from"../../../../../../🪪️graph-state/📦️project-records/🟦️.ts";
import{graphFleetClientV1}from"../../../../../../🪪️graph-state/🛰️exchange/🟦️.ts";
import{graphFleetSettingsV1}from"../../../../../../🪪️graph-state/🧶️nx/🟦️.ts";
import{remoteCacheFetchV1}from"../../../../../../🌐️remote-cache/🛰️fleet/🟦️.ts";
import{OWNER_POOL_POLICY_V1 as P,ownerPoolAcceptsRevisionV1,ownerPoolIdV1,ownerPoolRepublishV1,ownerPoolWhyIdleV1,publishOwnerPoolV1,readOwnerPoolPointerV1,type OwnerPoolManifestV1,type OwnerPoolUnitV1}from"../../🟦️.ts";
import{createOwnerPoolPublisherV1}from"../../📣️publisher/🟦️.ts";
import{createOwnerPoolStatusV1}from"../../📊️status/🟦️.ts";
import{runOwnerPoolV1}from"../../🏃️runner/🟨️.mjs";
import fixture from"./🔣️.json"with{type:"json"};

const scratch:string[]=[];
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const temp=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`owner-pool-early-${label}-`));scratch.push(path);return path;};
const sha=(text:string)=>createHash("sha256").update(text).digest("hex");
const pause=(milliseconds:number)=>new Promise<void>(done=>setTimeout(done,milliseconds));
const driverPath=join(import.meta.dir,"..","🧫️fixtures","🧮️driver.mjs");
async function world(){
 const cache=temp("cache"),server=await startRemoteCacheServerV1({directory:cache,host:"127.0.0.1",port:0,maximumBytes:2**28,maximumEntryBytes:2**26,maximumAgeDays:30,defaultInstance:"main",tokens:{read:"r",write:"w"},validation:"off"});
 return{server,environment:{SEMIO_REMOTE_CACHE_ENDPOINT:server.url,SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_REMOTE_CACHE_TOKEN:"w",SEMIO_REMOTE_CACHE_INSTANCE:"main"}};
}
const unitsOf=(count:number,burn=400):OwnerPoolUnitV1[]=>Array.from({length:count},(_all,index)=>({name:`owner-${String(index).padStart(4,"0")}`,kind:"k",expectedMilliseconds:burn,json:{name:`owner-${index}`}}));
const manifestOf=(units:readonly OwnerPoolUnitV1[],tag:string,revision:string,config:object):OwnerPoolManifestV1=>{
 const partial={version:1 as const,identity:"identity-"+tag,globals:{projectAuthority:"a",registryDigest:"r",commandRevision:"c",late:false},publisher:"p:"+tag};
 return{head:{...partial,revision,id:ownerPoolIdV1(partial,units),createdMs:Date.now()},session:Buffer.from(JSON.stringify(config)),units};
};

test("the idle reason, the revision rule and the republish rule equal the authored language-agnostic cases",()=>{
 for(const row of fixture.why)expect(ownerPoolWhyIdleV1({waves:row.waves,open:row.open,children:row.children,targetChildren:row.targetChildren,targetReason:row.targetReason,now:fixture.now}),row.name).toBe(row.expect);
 for(const row of fixture.accepts)expect(ownerPoolAcceptsRevisionV1(row.wave??undefined,row.mine??undefined)).toBe(row.expect);
 for(const row of fixture.republish)expect(ownerPoolRepublishV1({published:row.published??undefined,count:row.count,sinceMs:row.sinceMs}),JSON.stringify(row)).toBe(row.expect);
});

test("a reader publishes its wave when the unit list is known, before any owner is taken, and again as the open owners shrink; an unchanged list is not published twice",async()=>{
 const {server,environment}=await world();
 try{
  const root=temp("pub"),published:OwnerPoolManifestV1[]=[],clock={at:1000000};
  let open=OwnerPoolUnitOf(500);
  function OwnerPoolUnitOf(count:number){return unitsOf(count);}
  const publisher=createOwnerPoolPublisherV1({workspaceRoot:root,environment:{...process.env,...environment},identity:"id",globals:{projectAuthority:"a",registryDigest:"r",commandRevision:"c",late:false},session:()=>({tag:"s"}),units:()=>open,now:()=>clock.at,publish:async input=>{published.push(input.manifest);return true;}});
  publisher.announce(3);await pause(30);
  expect(published.length,"below the threshold nothing is announced").toBe(0);
  publisher.announce(500);await pause(50);
  expect(published.length,"announced at once, before any miss").toBe(1);
  expect(published[0]!.units.length).toBe(500);expect(typeof published[0]!.head.revision).toBe("string");
  publisher.learn(480);await pause(30);expect(published.length,"a shrink below the share is no new wave").toBe(1);
  open=unitsOf(300);publisher.learn(300);await pause(30);expect(published.length,"too soon after the last wave").toBe(1);
  clock.at+=P.republishMinimumMilliseconds;publisher.learn(300);await pause(50);
  expect(published.length,"republished once the open owners shrank and the interval passed").toBe(2);expect(published[1]!.units.length).toBe(300);
  publisher.settle(300);await pause(50);expect(published.length,"the same list is the same wave").toBe(2);
  open=unitsOf(120);publisher.settle(120);await pause(50);expect(published.length,"the settled list is published at once").toBe(3);
  publisher.miss(Number.MAX_SAFE_INTEGER);await pause(30);expect(published.length).toBe(3);
  expect(new Set(published.map(manifest=>manifest.head.id)).size).toBe(3);
 }finally{await server.stop();}
});

test("a pool whose waves all carry another revision says why instead of a bare idle, asks for a read, and starts at once when a wave of its revision is published by any machine",async()=>{
 const {server,environment}=await world();
 try{
  const root=temp("pool"),client=graphFleetClientV1(graphFleetSettingsV1(root,environment)!,{fetch:remoteCacheFetchV1(environment,{root})});
  const old=manifestOf(unitsOf(6),"old","revision-old",{burnMilliseconds:10,revision:"revision-new"});
  expect(await publishOwnerPoolV1({client,owner:"o",manifest:old})).toBe(true);
  const store=temp("store"),status=createOwnerPoolStatusV1({store,origin:"g1-early",role:"puller"}),asked:{revision:string|undefined;waves:number}[]=[];
  const run=()=>runOwnerPoolV1({root,environment:{...process.env,...environment},signal:AbortSignal.timeout(30000),capacity:async()=>({children:2,reason:null}),once:true,driver:driverPath,priority:0,targetPollMilliseconds:100,pollMilliseconds:300,status,revision:"revision-new",starved:async(info:{revision:string|undefined;waves:number})=>{asked.push(info);}});
  await run();
  const first=status.current();
  expect(first.state).toBe("idle");expect(first.reason).toBe("no-current-wave");expect(first.reasonText?.en).toContain("1 known waves");expect(first.reasonText?.de).toContain("1 bekannten Wellen");expect(first.reasonText?.en).not.toBe(first.reasonText?.de);
  expect(asked.length,"the server is asked to start a read").toBeGreaterThanOrEqual(1);expect(asked[0]!.revision).toBe("revision-new");
  const fresh=manifestOf(unitsOf(12),"fresh","revision-new",{burnMilliseconds:10,revision:"revision-new"});
  expect(await publishOwnerPoolV1({client,owner:"another-origin",manifest:fresh})).toBe(true);
  const before=asked.length;
  await run();
  expect(asked.length,"a wave of the current revision exists: nobody is asked").toBe(before);
  const reader=createProjectFleetPortV1({root:temp("reader"),environment,owner:"r:1"})!,stored:boolean[]=[];
  for(const unit of fresh.units){const info:ProjectRecordInfoV1={cachePath:join(root,"np",unit.name+".json"),fleet:`libs/${unit.name}`,keyText:JSON.stringify(["rev",null,[unit.name]]),exportRecord:()=>({digest:sha(unit.name),authority:[],observations:[{kind:"file",path:`${root}/${unit.name}/a.ts`,fact:{sha256:"x",size:1,mtimeMs:1,ctimeMs:1,ino:1}}],image:JSON.stringify({name:unit.name})}),async import(){return true;}};stored.push((await reader.claim(info)).state==="stored");}
  expect(stored.every(Boolean),"every owner of the fresh wave is stored").toBe(true);
  const pointer=await readOwnerPoolPointerV1(client);expect(pointer==="fault"||pointer===undefined?0:pointer.waves.length).toBe(2);
 }finally{await server.stop();}
},120000);
