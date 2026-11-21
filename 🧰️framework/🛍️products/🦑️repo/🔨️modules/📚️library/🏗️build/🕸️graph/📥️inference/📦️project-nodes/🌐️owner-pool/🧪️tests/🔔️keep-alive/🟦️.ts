import{afterAll,expect,test}from"bun:test";
import{Database}from"bun:sqlite";
import{deserialize}from"node:v8";
import{graphFleetClientV1,graphRefusalOfV1}from"../../../../../../🪪️graph-state/🛰️exchange/🟦️.ts";
import{graphFleetSettingsV1}from"../../../../../../🪪️graph-state/🧶️nx/🟦️.ts";
import fixture from"./🔣️.json"with{type:"json"};
import{mkdtempSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{startRemoteCacheServerV1}from"../../../../../../🌐️remote-cache/🖥️server/🟦️.ts";
import{OWNER_POOL_POLICY_V1 as P,ownerPoolKeepAliveV1,publishOwnerPoolV1,readOwnerPoolPointerV1,readOwnerPoolV1,type OwnerPoolManifestV1,type OwnerPoolUnitV1}from"../../🟦️.ts";
import{createOwnerPoolPublisherV1}from"../../📣️publisher/🟦️.ts";

const scratch:string[]=[];
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const temp=(label:string)=>{const path=mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir(),`owner-pool-alive-${label}-`));scratch.push(path);return path;};
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

test("small membership changes renew the accepted owner wave without uploading new units",async()=>{
 const server=await startRemoteCacheServerV1({directory:temp("cache"),host:"127.0.0.1",port:0,maximumBytes:2**27,maximumEntryBytes:2**24,maximumAgeDays:30,defaultInstance:"main",tokens:{write:"w"},validation:"off"});
 const sqlite=new Database(":memory:");
 try{
  const root=temp("native"),clock={at:Date.now()},environment={SEMIO_REMOTE_CACHE_ENDPOINT:server.url,SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_REMOTE_CACHE_TOKEN:"w",SEMIO_REMOTE_CACHE_INSTANCE:"main"};
  const observed:OwnerPoolManifestV1[]=[],puts:number[]=[];
  const client=graphFleetClientV1(graphFleetSettingsV1(root,environment)!,{fetch:((url,init)=>{
   if(init?.method==="PUT"&&String(url).includes("/cas/")&&ArrayBuffer.isView(init.body))puts.push(init.body.byteLength);
   return fetch(url,init);
  }) as typeof fetch});
  let open:OwnerPoolUnitV1[]=[],permit=true,sessions=0,accepted:OwnerPoolManifestV1|undefined;
  const publisher=createOwnerPoolPublisherV1({workspaceRoot:root,environment,identity:"native-keepalive",globals:{projectAuthority:"a",registryDigest:"r",commandRevision:"c",late:false},session:()=>({serial:++sessions}),units:()=>open,now:()=>clock.at,publish:input=>{
   observed.push(input.manifest);
   const about=permit?client:{...client,putResult:async()=>({stored:false as const,refusal:graphRefusalOfV1(507)})};
   return publishOwnerPoolV1({...input,client:about,now:()=>clock.at});
  }});
  const expected=sqlite.query(`WITH steps AS (
   SELECT cast(key AS integer) AS n, json_extract(value,'$.mode') AS mode, json_extract(value,'$.count') AS count,
    json_extract(value,'$.stored') AS stored, json_extract(value,'$.renewal') AS renewal FROM json_each(?)
  ) SELECT CASE WHEN s.renewal=0 THEN s.count ELSE (SELECT p.count FROM steps p
   WHERE p.n<s.n AND p.renewal=0 AND p.stored AND p.mode<>'target' ORDER BY p.n DESC LIMIT 1) END AS count
   FROM steps s ORDER BY s.n`).all(JSON.stringify(fixture.steps)) as {count:number}[];
  for(const [index,row] of fixture.steps.entries()){
   if(row.advance==="keepalive")clock.at+=P.republishKeepAliveMilliseconds;
   if(row.advance==="minimum")clock.at+=P.republishMinimumMilliseconds;
   open=unitsOf(row.count);permit=row.stored;
   const beforePuts=puts.length,beforeSessions=sessions;
   const mode=row.mode as "target"|"announce"|"learn"|"settle"|"miss";
   if(mode==="target")expect(await publisher.ask(open,true)).toBe(row.stored);
   else{publisher[mode](row.count);expect(await publisher.published()).toBe(row.stored);}
   const result=observed.at(-1)!;
   expect(observed).toHaveLength(index+1);
   expect(result.units.length,row.mode).toBe(expected[index]!.count);
   expect(result.head.createdMs).toBe(clock.at);
   const added=puts.slice(beforePuts).filter(bytes=>bytes>=fixture.largeBlobBytes);
   if(row.renewal){
    expect(result.head).toEqual({...accepted!.head,createdMs:clock.at});
    expect(result.units).toEqual(accepted!.units);
    expect(accepted!.head.createdMs).toBeLessThan(result.head.createdMs);
    expect(deserialize(result.session)).toEqual(deserialize(accepted!.session));
    expect(sessions).toBe(beforeSessions);
    expect(added).toEqual([]);
   }else if(mode==="announce")expect(added.length).toBeGreaterThan(0);
   if(row.stored){
    const pointer=await readOwnerPoolPointerV1(client,()=>clock.at);
    expect(pointer&&pointer!=="fault"?pointer.waves.find(wave=>wave.id===result.head.id):undefined).toMatchObject({createdMs:clock.at,units:expected[index]!.count});
    expect(deserialize((await readOwnerPoolV1(client,result.head.id))!.session)).toEqual(deserialize(result.session));
    if(mode!=="target")accepted=result;
   }
  }
 }finally{sqlite.close();await server.stop();}
},30000);
