import{afterAll,expect,test}from"bun:test";
import{mkdtempSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{startRemoteCacheServerV1}from"../../../🌐️remote-cache/🖥️server/🟦️.ts";
import{remoteCacheFetchV1}from"../../../🌐️remote-cache/🛰️fleet/🟦️.ts";
import{graphFleetClientV1}from"../../🛰️exchange/🟦️.ts";
import{graphFleetSettingsV1}from"../../🧶️nx/🟦️.ts";
import{conformanceFleetPortV1,conformanceStateV1}from"../🟦️.ts";
import{recordHoldsV1,resolveStampV1}from"../../../🕸️graph/📥️inference/🔖️stamps/🧮️conformance/🟨️.mjs";

/** 🔖️ Contract: conformance records travel through the fleet cache keyed by inferrer, closure key and platform; a machine takes a record only after it checks against its own closure key, and an unreachable or foreign cache costs nothing but the local evaluation. */
const scratch:string[]=[];
const temp=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`conformance-${label}-`));scratch.push(path);return path;};
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});

async function world(){
 const server=await startRemoteCacheServerV1({directory:temp("cache"),host:"127.0.0.1",port:0,maximumBytes:2**28,maximumEntryBytes:2**26,maximumAgeDays:30,defaultInstance:"main",tokens:{read:"r",write:"w"},validation:"off"});
 const environment={SEMIO_REMOTE_CACHE_ENDPOINT:server.url,SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_REMOTE_CACHE_TOKEN:"w",SEMIO_REMOTE_CACHE_INSTANCE:"main"};
 const client=(label:string)=>{const root=temp(label);return graphFleetClientV1(graphFleetSettingsV1(root,environment)!,{fetch:remoteCacheFetchV1(environment,{root})});};
 return{server,client};
}

test("one machine computes and publishes a record, another takes it without evaluating, a record of another key or platform is not found",async()=>{
 const {server,client}=await world();
 try{
  const toy=temp("toy");
  const provider={entries:["a.mjs"],schemas:[],fixtures:{numbers:"f.json"},runner:"toy"};
  await Bun.write(join(toy,"a.mjs"),"export const a = 1;\n");await Bun.write(join(toy,"f.json"),'{"n":[1,2]}');
  let evaluations=0;const runners={toy:async(fixture:any)=>{evaluations++;return{cases:{n:fixture.numbers.n},covered:{}};}};
  const declared="semio.graph-inferrer.toy/1",a=conformanceFleetPortV1({client:client("a"),owner:"g1-m03"}),b=conformanceFleetPortV1({client:client("b"),owner:"g1-m04"});
  const first=await resolveStampV1({id:"toy",declared,root:toy,directory:temp("store-a"),runners,provider,fleet:a});
  expect(first.status).toBe("computed");
  await Bun.sleep(300);
  const published=await b.get("toy",first.key);
  expect(recordHoldsV1(published,{id:"toy",key:first.key,declared})).toBe(true);
  const before=evaluations,second=await resolveStampV1({id:"toy",declared,root:toy,directory:temp("store-b"),runners,provider,fleet:b});
  expect(second.status).toBe("fleet");expect(evaluations).toBe(before);expect(second.stamp).toBe(first.stamp);
  expect(await b.get("toy","f".repeat(64))).toBeUndefined();
  expect(await conformanceFleetPortV1({client:client("c"),owner:"x",platform:"linux-x64"}).get("toy",first.key)).toBeUndefined();
  expect(conformanceStateV1("toy","k").frame.platform).not.toBe("");
  expect(conformanceStateV1("toy","k").digest).not.toBe(conformanceStateV1("toy","k2").digest);
 }finally{await server.stop(true);}
},60000);

test("an unreachable cache answers nothing and publishes nothing, without throwing",async()=>{
 const environment={SEMIO_REMOTE_CACHE_ENDPOINT:"http://127.0.0.1:1",SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_REMOTE_CACHE_TOKEN:"w",SEMIO_REMOTE_CACHE_INSTANCE:"main"};
 const root=temp("offline"),client=graphFleetClientV1(graphFleetSettingsV1(root,environment)!,{fetch:remoteCacheFetchV1(environment,{root})}),port=conformanceFleetPortV1({client,owner:"g1-m03"});
 expect(await port.get("toy","a".repeat(64))).toBeUndefined();
 expect(await port.put("toy","a".repeat(64),{version:1})).toBe(false);
},60000);
