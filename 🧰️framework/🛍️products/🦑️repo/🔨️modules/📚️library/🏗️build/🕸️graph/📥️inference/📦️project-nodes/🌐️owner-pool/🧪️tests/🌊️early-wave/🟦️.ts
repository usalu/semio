import{afterAll,expect,test}from"bun:test";
import{Database}from"bun:sqlite";
import Ajv from"ajv";
import{mkdtempSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{createHash}from"node:crypto";
import{startRemoteCacheServerV1}from"../../../../../../🌐️remote-cache/🖥️server/🟦️.ts";
import{createProjectFleetPortV1,type ProjectRecordInfoV1}from"../../../../../../🪪️graph-state/📦️project-records/🟦️.ts";
import{graphFleetClientV1,graphRefusalOfV1}from"../../../../../../🪪️graph-state/🛰️exchange/🟦️.ts";
import{graphFleetSettingsV1}from"../../../../../../🪪️graph-state/🧶️nx/🟦️.ts";
import{remoteCacheFetchV1}from"../../../../../../🌐️remote-cache/🛰️fleet/🟦️.ts";
import{OWNER_POOL_POLICY_V1 as P,ownerPoolAcceptsRevisionV1,ownerPoolIdV1,ownerPoolRepublishV1,ownerPoolWhyIdleV1,publishOwnerPoolV1,readOwnerPoolPointerV1,type OwnerPoolManifestV1,type OwnerPoolPublishRefusalV1,type OwnerPoolUnitV1}from"../../🟦️.ts";
import{createOwnerPoolPublisherV1}from"../../📣️publisher/🟦️.ts";
import{createOwnerPoolStatusV1}from"../../📊️status/🟦️.ts";
import{runOwnerPoolV1}from"../../🏃️runner/🟨️.mjs";
import fixture from"./🔣️.json"with{type:"json"};
import breakerCases from"./🌐️breaker.json"with{type:"json"};
import breakerSchema from"./🌐️breaker.schema.json"with{type:"json"};

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
 const partial={version:1 as const,revision,identity:"identity-"+tag,globals:{registryDigest:"r",commandRevision:"c",late:false},publisher:"p:"+tag};
 return{head:{...partial,id:ownerPoolIdV1(partial,units),createdMs:Date.now()},session:Buffer.from(JSON.stringify(config)),units};
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
  const publisher=createOwnerPoolPublisherV1({workspaceRoot:root,environment:{...process.env,...environment},identity:"id",globals:{registryDigest:"r",commandRevision:"c",late:false},session:()=>({tag:"s"}),units:()=>open,now:()=>clock.at,publish:async input=>{published.push(input.manifest);return true;}});
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

test("a refused fleet offer records the bounded cache failure without response text",async()=>{
 const {server,environment}=await world(),root=temp("refusal"),db=new Database(":memory:");
 try{
  for(const row of fixture.publishRefusals){
   const said:string[]=[],privateText="Private response body must stay local";
   const publisher=createOwnerPoolPublisherV1({
    workspaceRoot:root,environment,identity:row.stage,globals:{registryDigest:"r",commandRevision:"c",late:false},
    session:()=>({}),units:()=>unitsOf(1),threshold:1,say:line=>said.push(line),
    publish:input=>publishOwnerPoolV1({...input,client:{...input.client,async putResult(key,bytes,slot,expected){
     if((expected===undefined)===(row.stage==="manifest"))return {stored:false,refusal:graphRefusalOfV1(row.status,privateText)};
     return input.client.putResult(key,bytes,slot,expected);
    }}}),
   });
   publisher.miss(1);
   expect(await publisher.published()).toBe(false);
   const prefix="owner pool refusal ";
   const line=said.find(value=>value.startsWith(prefix));
   expect(line).toBeDefined();
   const oracle=db.query(`select json_extract(?1,'$.stage') stage,
    case json_extract(?1,'$.status') when 507 then 'capacity' else 'denied' end code,
    json_extract(?1,'$.status') status`).get(JSON.stringify(row));
   expect(JSON.parse(line!.slice(prefix.length))).toMatchObject(oracle as object);
   expect(said.join("\n")).not.toContain(privateText);
  }
 }finally{db.close();await server.stop();}
});

test("a transient first offer is retried during discovery after one cooldown",async()=>{
 const {server,environment}=await world(),root=temp("retry"),db=new Database(":memory:"),rule=fixture.firstOfferRetry;
 try{
  expect(P.faultRetryMilliseconds).toBe(rule.retryMilliseconds);
  for(const status of rule.statuses){
   const clock={at:0};let offers=0;
   const publisher=createOwnerPoolPublisherV1({
    workspaceRoot:root,environment,identity:"early-"+status,globals:{registryDigest:"r",commandRevision:"c",late:false},
    session:()=>({}),units:()=>unitsOf(8),threshold:1,now:()=>clock.at,
    publish:input=>{
     if(++offers>1)return Promise.resolve(true);
     return publishOwnerPoolV1({...input,client:{...input.client,putResult:async()=>({stored:false,refusal:graphRefusalOfV1(status)})}});
    },
   });
   publisher.announce(8);
   expect(await publisher.published()).toBe(false);
   for(const elapsed of rule.elapsed){
    clock.at=elapsed;
    publisher.learn(8);publisher.miss(8);publisher.learn(8);
    const oracle=db.query("select case when ?1=507 and ?2>=?3 then 2 else 1 end offers").get(status,elapsed,rule.retryMilliseconds) as {offers:number};
    await publisher.published();
    expect(offers).toBe(oracle.offers);
   }
  }
 }finally{db.close();await server.stop();}
});

test("optional HEAD does not block an accepted manifest from publishing its pointer",async()=>{
 const database=new Database(":memory:");
 try{for(const row of breakerCases){
  const directory=temp(row.id),manifest=manifestOf(unitsOf(1),row.id,"a".repeat(64),{fixture:true}),refusals:OwnerPoolPublishRefusalV1[]=[],methods:string[]=[];
  let headTimedOut=false;
  const client=graphFleetClientV1({endpoint:"https://fixture.invalid",instance:"test",token:"fixture",upload:true,stateDirectory:directory},{fetch:(async(input,init)=>{
   const kind=new URL(String(input)).pathname.split("/")[2],method=init?.method;
   methods.push(method+" "+kind);
   if(method==="HEAD"&&row.headTimeout&&!headTimedOut){headTimedOut=true;throw new DOMException("advisory deadline","TimeoutError");}
   if(method==="PUT"&&kind===row.requiredTimeout)throw new DOMException("required deadline","TimeoutError");
   return new Response(null,{status:method==="PUT"?200:404});
  }) as typeof fetch});
  const stored=await publishOwnerPoolV1({client,owner:"fixture",manifest,onRefused:refusal=>refusals.push(refusal)});
  const expected=database.query("select ?1 is null as stored, ?1 is null as usable, ?1 is null as pointerRead, case when ?1 is null then null else 'no-answer' end as refusal").get(row.requiredTimeout) as {stored:number;usable:number;pointerRead:number;refusal:string|null};
  expect<unknown>({stored,usable:client.usable(),pointerRead:methods.includes("GET ac"),refusal:refusals[0]?.code??null},row.id).toEqual({stored:Boolean(expected.stored),usable:Boolean(expected.usable),pointerRead:Boolean(expected.pointerRead),refusal:expected.refusal});
  expect(methods.filter(method=>method==="PUT ac")).toHaveLength(row.requiredTimeout==="cas"?0:row.requiredTimeout==="ac"?1:2);
  if(row.headTimeout)expect(headTimedOut).toBe(true);
  if(row.requiredTimeout!==null)expect(refusals).toMatchObject([{stage:"manifest",code:"no-answer",status:null,transient:true}]);
 }}finally{database.close();}
});

test("a 507 of the cache server names the exhausted resource with its numbers in English and German in the logged refusal",async()=>{
 const detail="The store holds 52.67 GiB of its 15.3 GiB cap in force (role cache-host, 36.7 GiB free on the partition, hard reserve 74.08 GiB); largest holders: unit-cache 25.98 GiB / Der Speicher hält 52.67 GiB seiner wirksamen Grenze von 15.3 GiB (Rolle cache-host, 36.7 GiB frei auf der Partition, harte Reserve 74.08 GiB); größte Halter: unit-cache 25.98 GiB",refusals:OwnerPoolPublishRefusalV1[]=[];
 const client=graphFleetClientV1({endpoint:"https://fixture.invalid",instance:"test",token:"fixture",upload:true,stateDirectory:temp("capacity-detail")},{fetch:(async(_input,init)=>init?.method==="PUT"?new Response("Insufficient storage: "+detail,{status:507}):new Response(null,{status:404})) as typeof fetch});
 const stored=await publishOwnerPoolV1({client,owner:"fixture",manifest:manifestOf(unitsOf(1),"capacity-detail","a".repeat(64),{fixture:true}),onRefused:refusal=>refusals.push(refusal)});
 expect(stored).toBe(false);
 expect(refusals[0]).toMatchObject({stage:"manifest",code:"capacity",status:507,transient:true});
 expect(refusals[0]?.detail).toContain("hard reserve 74.08 GiB");expect(refusals[0]?.detail).toContain("harte Reserve 74.08 GiB");
});

test("manifest timeout log identifies the required write and its effective deadline",async()=>{
 expect(new Ajv({strict:true}).validate(breakerSchema,breakerCases)).toBe(true);
 const database=new Database(":memory:"),privateText="fixture-private-token";
 try{for(const row of breakerCases){
  const directory=temp("provenance-"+row.id),lines:string[]=[],prefix="owner pool refusal ";
  let failedBytes:number|undefined,headTimedOut=false;
  const environment={SEMIO_REMOTE_CACHE_ENDPOINT:"https://fixture.invalid",SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_REMOTE_CACHE_TOKEN:privateText,SEMIO_REMOTE_CACHE_INSTANCE:"main"};
  const client=graphFleetClientV1({endpoint:environment.SEMIO_REMOTE_CACHE_ENDPOINT,instance:"main",token:privateText,upload:true,stateDirectory:directory},{fetch:(async(input,init)=>{
   const kind=new URL(String(input)).pathname.split("/")[2],method=init?.method;
   if(method==="HEAD"&&row.headTimeout&&!headTimedOut){headTimedOut=true;throw new DOMException(privateText,"TimeoutError");}
   if(method==="PUT"&&kind===row.requiredTimeout&&failedBytes===undefined){
    const body=init?.body;
    if(!ArrayBuffer.isView(body))throw Error("Encoded upload body required");
    failedBytes=body.byteLength;
    throw new DOMException(privateText,"TimeoutError");
   }
   return new Response(null,{status:method==="PUT"?200:404});
  }) as typeof fetch});
  const units=unitsOf(1),publisher=createOwnerPoolPublisherV1({workspaceRoot:directory,environment,identity:row.id,globals:{registryDigest:"b",commandRevision:"c",late:false},session:()=>({fixture:true}),units:()=>units,say:line=>lines.push(line),publish:input=>publishOwnerPoolV1({...input,client})});
  expect(await publisher.ask(units,true)).toBe(row.requiredTimeout===null);
  const refusals=lines.filter(line=>line.startsWith(prefix));
  if(row.requiredTimeout===null){expect(headTimedOut).toBe(true);expect(refusals).toEqual([]);continue;}
  expect(failedBytes).toBeGreaterThan(0);
  expect(refusals).toHaveLength(1);
  const timeout=database.query(`select case ?1 when 'cas' then 'cas-put' else 'action-result-put' end request,
   case ?1 when 'cas' then 20000+(?2*1000+2097151)/2097152 else 6000 end deadlineMilliseconds`).get(row.requiredTimeout,failedBytes!);
  expect(JSON.parse(refusals[0]!.slice(prefix.length))).toEqual({wave:expect.stringMatching(/^[a-f0-9]{12}$/u),stage:"manifest",code:"no-answer",status:null,transient:true,timeout});
  expect(lines.join("\n")).not.toContain(privateText);
 }}finally{database.close();}
});
