import {afterAll,expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import pointerRace from "./📍️pointer-race.json" with {type:"json"};
import {ownerPoolIdV1,ownerPoolStateV1,publishOwnerPoolV1,readOwnerPoolPointerV1,readOwnerPoolV1,type OwnerPoolManifestV1} from "../../../../🕸️graph/📥️inference/📦️project-nodes/🌐️owner-pool/🟦️.ts";
import {createHash} from "node:crypto";
import {mkdirSync,mkdtempSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {encodeProtobufV1,REAPI_V1} from "../../../../🌐️remote-execution/📡️wire/📦️protobuf/🟦️.ts";
import {fleetSignedFetchV1,syncFleetIdentityV1} from "../../../🛰️fleet/🪪️identity/🟦️.ts";
import {startRemoteCacheServerV1,type RemoteCacheServerV1} from "../../🟦️.ts";
import {fetchGraphEntryV1,graphEntryKeyV1,graphFleetClientV1,publishGraphEntryV1} from "../../../../🪪️graph-state/🛰️exchange/🟦️.ts";
import {readServedV1,publishServedV1,servedStateV1} from "../../../../🪪️graph-state/🙋️wanted/🟦️.ts";

/** 🧭️ A mutable head (the answer served to a requester, the table of owner costs, the pointer of the owner pool) is replaced by compare-and-swap, whoever wrote the old value: the newest writer wins, a stale writer is told so and nobody is counted as a divergence; a result that is no head stays first-write-wins. */
const scratch:string[]=[],servers:RemoteCacheServerV1[]=[];
const directory=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`rcache-head-${label}-`));scratch.push(path);return path;};
afterAll(async()=>{for(const server of servers)await server.stop();for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const sha=(value:Uint8Array|string)=>createHash("sha256").update(value).digest("hex");
function machine(store:string,origin:string):string{
 const root=directory(origin);mkdirSync(join(root,".cache","f"),{recursive:true});
 writeFileSync(join(root,".cache","f","machine.json"),JSON.stringify({version:1,originId:origin,groupId:"g1",storeRoot:store,key:"k"}));
 mkdirSync(join(store,"v1","origins",origin),{recursive:true});return root;
}
const result=(seed:string)=>encodeProtobufV1(REAPI_V1.actionResult,{output_files:[{path:"head.br",digest:{hash:sha(seed),size_bytes:3}}],exit_code:0});
async function fleet(){
 const store=directory("group"),a=machine(store,"g1-m01"),b=machine(store,"g1-m02"),server=machine(store,"g1-server");
 for(const root of[a,b,server])expect((await syncFleetIdentityV1({root})).error).toBeUndefined();
 const running=await startRemoteCacheServerV1({directory:directory("cache"),host:"127.0.0.1",port:0,maximumBytes:2**26,maximumEntryBytes:2**24,maximumAgeDays:30,defaultInstance:"semio",tokens:{},validation:"structure",fleet:{storeRoot:store,group:"g1",networks:["10.77.0.0/16"]}});
 servers.push(running);
 return{a,b,running,base:`http://127.0.0.1:${running.port}/semio`,signed:(root:string)=>fleetSignedFetchV1({root,audience:"cache"}),status:async()=>await (await fleetSignedFetchV1({root:a,audience:"cache"})(`http://127.0.0.1:${running.port}/status`)).json() as {divergences:{count:number}}};
}

test("a head is replaced by the other origin only with the digest of what it holds; a stale or wrong expectation is refused and is no divergence",async()=>{
 const world=await fleet(),key=sha("head"),put=(root:string,bytes:Uint8Array,expect?:string)=>world.signed(root)(`${world.base}/ac/${key}`,{method:"PUT",body:bytes,headers:expect===undefined?{}:{"x-semio-expect":expect}});
 const one=result("one"),two=result("two"),three=result("three");
 expect((await put(world.a,one,"none")).status,"first writer of an empty head").toBe(200);
 expect((await put(world.b,two,"none")).status,"expects an empty head but it holds a value").toBe(409);
 expect((await put(world.b,two,sha(one))).status,"another origin with the digest of the held value").toBe(200);
 expect((await put(world.a,three,sha(one))).status,"the first writer is now stale").toBe(409);
 expect((await put(world.a,three,sha(two))).status).toBe(200);
 const got=await world.signed(world.b)(`${world.base}/ac/${key}`);expect(new Uint8Array(await got.arrayBuffer())).toEqual(three);
 expect((await world.status()).divergences.count,"heads never count as divergences").toBe(0);
 const plain=sha("plain");
 expect((await world.signed(world.a)(`${world.base}/ac/${plain}`,{method:"PUT",body:one})).status).toBe(200);
 expect((await world.signed(world.b)(`${world.base}/ac/${plain}`,{method:"PUT",body:two})).status,"without an expectation the first write still wins").toBe(409);
 expect((await world.status()).divergences.count).toBe(1);
});

test("two machines publishing the answer served to one requester both succeed and the newest answer is read",async()=>{
 const world=await fleet(),settings=(root:string)=>({endpoint:`http://127.0.0.1:${world.running.port}`,instance:"semio",token:undefined,upload:true,stateDirectory:join(root,"state")});
 const clientOf=(root:string)=>graphFleetClientV1(settings(root),{fetch:world.signed(root)});
 const request={version:1 as const,requester:"g1-m05",platform:"win32-x64",projects:["p"],atMs:1} as never;
 expect(await publishServedV1({client:clientOf(world.a),server:"g1-server",request,projects:["p"],generation:1,now:()=>10})).toBe(true);
 expect(await publishServedV1({client:clientOf(world.b),server:"g1-m02",request,projects:["p","q"],generation:2,now:()=>20})).toBe(true);
 const read=await readServedV1({client:clientOf(world.a),requester:"g1-m05"});
 expect(read?.server).toBe("g1-m02");expect(read?.projects).toEqual(["p","q"]);
 expect((await world.status()).divergences.count).toBe(0);
 expect(await fetchGraphEntryV1(clientOf(world.b),servedStateV1("g1-m05"))).not.toBe("fault");
 expect(graphEntryKeyV1(servedStateV1("g1-m05"))).toHaveLength(64);
});

test("a publisher retries its merge when another machine changes the exact pointer it read",async()=>{
 const world=await fleet(),db=new Database(":memory:"),now=Date.now();
 try{
  const clientOf=(root:string)=>graphFleetClientV1({endpoint:world.running.url,instance:"semio",token:undefined,upload:true,stateDirectory:join(root,"state")},{fetch:world.signed(root)});
  const client=clientOf(world.a),competitor=clientOf(world.b);
  const pointerKey=graphEntryKeyV1(ownerPoolStateV1("pointer"));
  const [initial,ours,theirs]=pointerRace.waves.map((wave):OwnerPoolManifestV1=>({
   head:{version:1,id:sha(wave.publisher),createdMs:now+wave.offset,publisher:wave.publisher,identity:"fixture",globals:{projectAuthority:"a",registryDigest:"r",commandRevision:"t",late:false}},
   session:new Uint8Array(0),
   units:Array.from({length:wave.units},(_,index)=>({name:String(index),kind:"fixture",expectedMilliseconds:1000,json:{}})),
  }));
  expect(await publishOwnerPoolV1({client,owner:"initial",manifest:initial!})).toBe(true);
  let intervened=false;
  const racing={...client,async getResult(key:string,signal?:AbortSignal){
   const old=await client.getResult(key,signal);
   if(key===pointerKey&&!intervened){
    intervened=true;
    expect(await publishOwnerPoolV1({client:competitor,owner:"theirs",manifest:theirs!})).toBe(true);
   }
   return old;
  }};
  expect(await publishOwnerPoolV1({client:racing,owner:"ours",manifest:ours!})).toBe(true);
  expect(intervened).toBe(true);
  const oracle=db.query(`select json_extract(value,'$.units') units from json_each(?1) order by json_extract(value,'$.offset')`),pointer=await readOwnerPoolPointerV1(client);
  expect(pointer&&pointer!=="fault"?pointer.waves.map(wave=>wave.units):pointer).toEqual(oracle.all(JSON.stringify(pointerRace.waves)).map(row=>(row as {units:number}).units));
 }finally{db.close();}
});

test("concurrent signed head replacements of the same result admit one writer",async()=>{
 const world=await fleet(),fixture=pointerRace.samePrior,key=sha("simultaneous-head"),store=world.running.store;
 const put=(root:string,bytes:Uint8Array,expect:string)=>world.signed(root)(`${world.base}/ac/${key}`,{method:"PUT",body:bytes,headers:{"x-semio-expect":expect}});
 const original=result(fixture.base),prior=sha(original);
 expect((await put(world.a,original,"none")).status).toBe(200);
 const write=store.put.bind(store);
 let release!:()=>void,entered!:()=>void,writes=0;
 const held=new Promise<void>(done=>{release=done;}),first=new Promise<void>(done=>{entered=done;});
 store.put=async(...args:Parameters<typeof write>)=>{
  if(args[1]==="ac"&&args[2]===key&&++writes===1){entered();await held;}
  return write(...args);
 };
 const db=new Database(":memory:");
 try{
  const left=put(world.a,result(fixture.writers[0]!),prior);
  await first;
  const right=put(world.b,result(fixture.writers[1]!),prior);
  await Promise.race([right,new Promise(resolve=>setTimeout(resolve,250))]);
  release();
  const codes=(await Promise.all([left,right])).map(response=>response.status).sort();
  const expected=db.query(`select value status from json_each(?1) order by value`).all(JSON.stringify(fixture.statuses));
  expect(codes).toEqual(expected.map(row=>(row as {status:number}).status));
 }finally{release();store.put=write;db.close();}
});

test("same declared wave refreshes through another signed publisher without replacing its verified manifest",async()=>{
 const world=await fleet(),now=Date.now(),rootA=world.a,rootB=world.b;
 const clientOf=(root:string)=>graphFleetClientV1({endpoint:world.running.url,instance:"semio",token:undefined,upload:true,stateDirectory:join(root,"state")},{fetch:world.signed(root)});
 const first=clientOf(rootA),second=clientOf(rootB),units=[{name:"owner",kind:"test",expectedMilliseconds:10,json:{name:"owner"}}];
 const partial={version:1 as const,revision:"fixture-revision",identity:"current",globals:{projectAuthority:"p",registryDigest:"r",commandRevision:"c",late:false}};
 const id=ownerPoolIdV1(partial,units),a={...partial,id,createdMs:now-1000,publisher:"a"},b={...a,createdMs:now,publisher:"b"};
 expect(await publishOwnerPoolV1({client:first,owner:"a",manifest:{head:a,session:Buffer.from("same"),units}})).toBe(true);
 expect(await publishOwnerPoolV1({client:second,owner:"b",manifest:{head:b,session:Buffer.from("same"),units}})).toBe(true);
 const pointer=await readOwnerPoolPointerV1(first);
 expect(pointer&&pointer!=="fault"?pointer.waves:undefined).toEqual([{id,createdMs:now,units:1}]);
 expect((await readOwnerPoolV1(first,id))?.head.publisher).toBe("a");
 const newer={...partial,revision:"changed-record-revision"};
 const newId=ownerPoolIdV1(newer,units);
 expect(newId).not.toBe(id);
 expect(await publishOwnerPoolV1({client:second,owner:"b",manifest:{head:{...b,...newer},session:Buffer.from("same"),units}})).toBe(false);
 expect(await publishOwnerPoolV1({client:second,owner:"b",manifest:{head:{...b,...newer,id:newId},session:Buffer.from("newer"),units}})).toBe(true);
 expect((await readOwnerPoolV1(first,newId))?.head.revision).toBe(newer.revision);
});
