import {afterAll,expect,test} from "bun:test";
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
