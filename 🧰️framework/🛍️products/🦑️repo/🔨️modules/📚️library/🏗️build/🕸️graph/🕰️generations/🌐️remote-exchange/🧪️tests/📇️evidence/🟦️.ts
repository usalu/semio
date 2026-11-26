import {afterEach,expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import {mkdirSync,mkdtempSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import Ajv from "ajv";
import {fleetCacheLocalPathV1,remoteCacheFetchV1} from "../../../../../🌐️remote-cache/🛰️fleet/🟦️.ts";
import {loadFleetSignerV1,syncFleetIdentityV1} from "../../../../../🌐️remote-cache/🛰️fleet/🪪️identity/🟦️.ts";
import {publishFleetCacheFactsV1} from "../../../../../🌐️remote-cache/🛰️fleet/📣️publication/🟦️.ts";
import {startRemoteCacheServerV1,type RemoteCacheServerV1} from "../../../../../🌐️remote-cache/🖥️server/🟦️.ts";
import {httpRemoteExecutionContentV1} from "../../../../../🌐️remote-execution/🗄️content/🟦️.ts";
import {REAPI_V1,decodeProtobufV1,encodeProtobufV1} from "../../../../../🌐️remote-execution/📡️wire/📦️protobuf/🟦️.ts";
import {createNodeFilePortV1} from "../../../🟨️.mjs";
import {loadGenerationGraphV1} from "../../../🔌️load/🟨️.mjs";
import {createGenerationUploaderV1,localGenerationUploadSourceV1} from "../../../📡️wave-upload/🟦️.ts";
import {createMockHost,ringFiles,seed} from "../../../🧪️tests/🏃️world/🟦️.ts";
import {generationExchangeKeyV1,remoteGenerationExchangeV1,remoteGenerationLookV1,restoreGenerationV1} from "../../🟦️.ts";
import fixture from "./🔣️.json";
import schema from "./🧬️schema/🔣️.json";

const directories:string[]=[],servers:RemoteCacheServerV1[]=[],platform="darwin-arm64",group="g1",instance="semio";
const scratch=()=>{const path=mkdtempSync(join(tmpdir(),"graph-head-evidence-"));directories.push(path);return path;};
afterEach(async()=>{for(const server of servers.splice(0))await server.stop();for(const path of directories.splice(0))rmSync(path,{recursive:true,force:true});});

async function machine(store:string,originId:string){
 const root=scratch();
 mkdirSync(join(root,".cache","f"),{recursive:true});
 writeFileSync(join(root,".cache","f","machine.json"),JSON.stringify({version:1,originId,groupId:group,storeRoot:store,key:"private-test-key"}));
 mkdirSync(join(store,"v1","origins",originId),{recursive:true});
 expect((await syncFleetIdentityV1({root})).error).toBeUndefined();
 return root;
}

test("authenticated publish and restore report the same immutable neutral manifest across receiver generation renumbering",async()=>{
 expect(new Ajv({strict:true}).validate(schema,fixture)).toBe(true);
 using oracle=new Database(":memory:");
 const ranking=oracle.query<{ordinary:string;complete:string},Record<string,string|number>>(`WITH candidates(origin,age,stale) AS (VALUES ($complete,$age,0),($partial,0,$stale))
  SELECT (SELECT origin FROM candidates ORDER BY stale+age/60000.0,age,origin LIMIT 1) AS ordinary,
         (SELECT origin FROM candidates WHERE stale=0 ORDER BY age,origin LIMIT 1) AS complete`);
 const exact=oracle.query("SELECT json_extract(?1,'$.generation')=json_extract(?2,'$.generation') AND json_extract(?1,'$.stale')=0 AND json_extract(?1,'$.implementation')=json_extract(?2,'$.implementation') AND coalesce(json_extract(?1,'$.scope'),'all')=json_extract(?2,'$.scope') AND json_extract(?1,'$.origin')=json_extract(?2,'$.origin') AND json_extract(?1,'$.manifest')=json_extract(?2,'$.manifest') AS holds");
 for(const row of fixture.exchanges){
  const store=scratch(),server=await machine(store,"g1-server"),producer=await machine(store,row.producer),consumer=await machine(store,row.consumer),partial=await machine(store,row.fresherPartial.origin),cache=scratch(),language=row.language as "en"|"de";
  const running=await startRemoteCacheServerV1({directory:cache,host:"127.0.0.1",port:0,maximumBytes:2**28,maximumEntryBytes:2**24,maximumAgeDays:30,defaultInstance:instance,tokens:{},validation:"complete",fleet:{group,root:server,storeRoot:store,inGroup:"sealed"}});
  servers.push(running);
  const endpoint=running.url,admin=loadFleetSignerV1(server,"admin")!;
  await publishFleetCacheFactsV1({admin,storeRoot:store,directory:cache,group,instance,port:running.port,transport:"sealed"});
  const facts=await Bun.file(join(store,"v1","fleet","cache.json")).json();
  for(const root of [producer,consumer,partial]){
   writeFileSync(fleetCacheLocalPathV1(root),JSON.stringify({version:1,fetchedAtMs:Date.now(),checkedAtMs:Date.now(),selectedEndpoint:endpoint,candidates:[endpoint],selectedExecution:null,facts}));
   await seed(root,ringFiles(6));
  }
  const a=createMockHost(producer,{language}),b=createMockHost(consumer,{language}),identity=await a.host.identity(),scope=row.scope==="all"?undefined:row.scope;
  const exchangeAt=(root:string)=>remoteGenerationExchangeV1({environment:{},workspace:root,identity:async()=>identity,platform,scope})!;
  const aExchange=exchangeAt(producer);
  Object.assign(a.host,{exchange:aExchange});
  const waveLines:string[]=[];
  if(scope===undefined){
   await loadGenerationGraphV1(createMockHost(producer).host);
   const uploader=createGenerationUploaderV1({source:localGenerationUploadSourceV1({workspace:producer,root:join(producer,".gen"),environment:{},platform}),language,say:line=>waveLines.push(line)});
   uploader.wave({generation:0,done:6,total:6});uploader.tick();await uploader.idle();uploader.stop();
  }else await loadGenerationGraphV1(a.host);
  const key=generationExchangeKeyV1(identity,platform,row.producer,scope),fetch=remoteCacheFetchV1({}, {root:consumer});
  expect((await globalThis.fetch(`${endpoint}/${instance}/ac/${key}`)).status).toBe(401);
  const content=httpRemoteExecutionContentV1({endpoint,fetch}),action=decodeProtobufV1(REAPI_V1.actionResult,(await content.readActionResult(instance,key))!);
  const note=JSON.parse(Buffer.from(action.stdout_raw as Uint8Array).toString("utf8"));
  const payload=(await content.read(instance,{hash:note.manifest,size:0}))!;
  const digest=Buffer.from(await crypto.subtle.digest("SHA-256",Uint8Array.from(payload))).toString("hex");
  const producerGeneration=JSON.parse(Buffer.from(payload).toString("utf8")).generation;
  expect({manifest:note.manifest,origin:note.origin,scope:note.scope??"all",producerGeneration:note.generation}).toEqual({manifest:digest,origin:row.producer,scope:row.scope,producerGeneration});
  const author=httpRemoteExecutionContentV1({endpoint,fetch:remoteCacheFetchV1({}, {root:producer})});
 const partialAuthor=httpRemoteExecutionContentV1({endpoint,fetch:remoteCacheFetchV1({}, {root:partial})});
 const fs=createNodeFilePortV1(),completeAt=Date.now()-row.fresherPartial.olderCompleteMilliseconds;
 await author.writeActionResult(instance,key,encodeProtobufV1(REAPI_V1.actionResult,{...action,stdout_raw:Buffer.from(JSON.stringify({...note,stateMs:completeAt}))}));
 const partialKey=generationExchangeKeyV1(identity,platform,row.fresherPartial.origin,scope);
 await partialAuthor.writeActionResult(instance,partialKey,encodeProtobufV1(REAPI_V1.actionResult,{...action,stdout_raw:Buffer.from(JSON.stringify({...note,origin:row.fresherPartial.origin,stateMs:Date.now(),stale:row.fresherPartial.stale}))}));
 const expected=ranking.get({$complete:row.producer,$age:row.fresherPartial.olderCompleteMilliseconds,$partial:row.fresherPartial.origin,$stale:row.fresherPartial.stale});
 const exchange=exchangeAt(consumer),look={root:join(consumer,".gen"),fs,implementation:note.implementation};
 expect((await exchange.look(look)).origin).toBe(expected?.ordinary);
 const complete=await exchange.look({...look,requireComplete:true});
 const actionReads:string[]=[],blobReads:string[]=[];
 const observed={...content,readActionResult:async(...args:Parameters<typeof content.readActionResult>)=>{actionReads.push(args[1]);return content.readActionResult(...args);},read:async(...args:Parameters<typeof content.read>)=>{blobReads.push(args[1].hash);return content.read(...args);}};
 const location={content:observed,instance,key:generationExchangeKeyV1(identity,platform,row.consumer,scope),keys:[key,partialKey],root:look.root,workspace:consumer,platform,identity,scope,files:fs,implementation:note.implementation};
 expect(await remoteGenerationLookV1({...location,requireComplete:true})).toMatchObject({state:"complete",ahead:true,candidate:complete.candidate});
 expect(actionReads.sort()).toEqual([key,partialKey].sort());
 expect(blobReads).toEqual([]);
 const expectedHead={origin:row.producer,manifest:digest,generation:producerGeneration,scope:row.scope,identity,platform,implementation:note.implementation};
 expect(complete).toMatchObject({state:"complete",ahead:true,origin:expected?.complete,candidate:expectedHead});
 expect(await exchange.look({...look,implementation:undefined,requireComplete:true})).toEqual({state:"none",ahead:false,held:0});
 const strict=await exchange.restore({...look,root:join(consumer,"isolated"),expectedHead:complete.candidate!});
 expect(strict).toMatchObject({status:"restored",head:{origin:row.producer,manifest:digest,generation:producerGeneration,scope:row.scope}});
 const writeNotes=async(change:object)=>author.writeActionResult(instance,key,encodeProtobufV1(REAPI_V1.actionResult,{...action,stdout_raw:Buffer.from(JSON.stringify({...note,stateMs:completeAt,...change}))}));
 const sealedRoot=join(consumer,"isolated"),pointer=join(sealedRoot,"current.json"),before=await Bun.file(pointer).text();
 expect(exact.get(JSON.stringify(note),JSON.stringify(expectedHead))).toEqual({holds:1});
 const changes={generation:producerGeneration+1,stale:1,implementation:"f".repeat(64),scope:row.scope==="all"?"f".repeat(64):"all",origin:row.fresherPartial.origin,manifest:"f".repeat(64)};
 const writeAlternate=async(stale:number)=>partialAuthor.writeActionResult(instance,partialKey,encodeProtobufV1(REAPI_V1.actionResult,{...action,stdout_raw:Buffer.from(JSON.stringify({...note,origin:row.fresherPartial.origin,stateMs:Date.now(),stale}))}));
 await writeAlternate(0);
 for(const field of fixture.pinDrift as (keyof typeof changes)[]){
  const changed={[field]:changes[field]};
  expect(exact.get(JSON.stringify({...note,...changed}),JSON.stringify(expectedHead)),field).toEqual({holds:0});
  await writeNotes(changed);
  actionReads.length=0;
  const emptyRoot=join(consumer,`pinned-drift-${field}`);
  const outcome=await restoreGenerationV1({...location,root:emptyRoot,expectedHead:complete.candidate!});
  expect(actionReads,field).toEqual([key]);
  expect(blobReads,field).toEqual([]);
  expect(outcome).toMatchObject({status:"incomplete",reason:"expected generation head changed"});
  expect(await Bun.file(join(emptyRoot,"current.json")).exists()).toBe(false);
  const existing=await exchange.restore({...look,root:sealedRoot,expectedHead:complete.candidate!});
  expect(existing.status).not.toBe("restored");
  expect(await Bun.file(pointer).text()).toBe(before);
 }
 await writeNotes({stale:1});
 await writeAlternate(row.fresherPartial.stale);
 expect((await exchange.look({...look,requireComplete:true})).candidate).toBeUndefined();
 await writeNotes({});
 const wrongScope=remoteGenerationExchangeV1({environment:{},workspace:consumer,identity:async()=>identity,platform,scope:scope===undefined?"f".repeat(64):undefined})!;
 const wrongScopeRoot=join(consumer,"wrong-scope");
 expect(await wrongScope.restore({...look,root:wrongScopeRoot,expectedHead:complete.candidate!})).toMatchObject({status:"foreign"});
 expect(await Bun.file(join(wrongScopeRoot,"current.json")).exists()).toBe(false);
 Object.assign(b.host,{exchange});
  const read=await loadGenerationGraphV1(b.host,{policy:"current",lean:true});
  expect(Object.keys((read.graph as {projects:object}).projects)).toHaveLength(6);
  const receiverGeneration=(await Bun.file(join(consumer,".gen","current.json")).json()).generation;
  expect(receiverGeneration).toBeGreaterThan(producerGeneration);
  const fields=`origin=${row.producer} producerGeneration=${producerGeneration} scope=${row.scope} manifest=${digest}`;
  expect(scope===undefined?waveLines:a.said.map(entry=>entry.text)).toContain(`${language==="en"?"graph: published fleet head":"Graph: Flottenstand veröffentlicht"} ${fields}`);
  expect(b.said.map(entry=>entry.text)).toContain(`${language==="en"?"graph: restored fleet head":"Graph: Flottenstand wiederhergestellt"} ${fields}`);
   for(const [index,changed] of [{origin:row.consumer},{generation:producerGeneration+2}].entries()){
   await author.writeActionResult(instance,key,encodeProtobufV1(REAPI_V1.actionResult,{...action,stdout_raw:Buffer.from(JSON.stringify({...note,...changed}))}));
   const outcome=await exchangeAt(consumer).restore({root:join(consumer,`other-${index}`),fs});
   expect(outcome.status).toBe("restored");
   expect(outcome.head).toBeUndefined();
   const strictRoot=join(consumer,`strict-${index}`);
   const refused=await exchange.restore({...look,root:strictRoot,requireComplete:true});
   expect(refused.status).toBe(index===0?"foreign":"incomplete");
   expect(await Bun.file(join(strictRoot,"current.json")).exists()).toBe(false);
  }
 }
});
