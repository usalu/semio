import {afterAll,expect,test} from "bun:test";
import {createHash} from "node:crypto";
import {mkdirSync,mkdtempSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {fleetCacheLocalPathV1,remoteCacheFetchV1} from "../../🛰️fleet/🟦️.ts";
import {fleetSignedFetchV1,readFleetOriginKeyV1,syncFleetIdentityV1} from "../../🛰️fleet/🪪️identity/🟦️.ts";
import {publishFleetCacheFactsV1} from "../../🛰️fleet/📣️publication/🟦️.ts";
import {loadFleetSignerV1} from "../../🛰️fleet/🪪️identity/🟦️.ts";
import {startRemoteCacheServerV1,type RemoteCacheServerV1} from "../../🖥️server/🟦️.ts";
import {createSealedFetchV1} from "../🟦️.ts";

const scratch:string[]=[],servers:RemoteCacheServerV1[]=[];
const directory=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`sealed-${label}-`));scratch.push(path);return path;};
afterAll(async()=>{for(const server of servers)await server.stop().catch(()=>{});for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const sha=(value:Uint8Array|string)=>createHash("sha256").update(value).digest("hex");
function machine(store:string,group:string,origin:string):string{
 const root=directory(origin);mkdirSync(join(root,".cache","f"),{recursive:true});
 writeFileSync(join(root,".cache","f","machine.json"),JSON.stringify({version:1,originId:origin,groupId:group,storeRoot:store,key:"k"}));
 mkdirSync(join(store,"v1","origins",origin),{recursive:true});return root;
}
async function group(inGroup:"sealed"|"trusted-lan"|undefined){
 const store=directory("store"),server=machine(store,"g1","g1-server"),member=machine(store,"g1","g1-m01");
 for(const root of[server,member])expect((await syncFleetIdentityV1({root})).error).toBeUndefined();
 const running=await startRemoteCacheServerV1({directory:directory("cache"),host:"127.0.0.1",port:0,maximumBytes:2**28,maximumEntryBytes:2**24,maximumAgeDays:30,defaultInstance:"semio",tokens:{},fleet:{storeRoot:store,group:"g1",registryTtlMilliseconds:50,root:server,...(inGroup===undefined?{}:{inGroup})}});
 servers.push(running);
 return {store,server,member,running,endpoint:`http://127.0.0.1:${running.port}`};
}

test("with the in-group policy sealed a member's plain request is refused with the reason, its sealed request is served, and the health route stays open; on a trusted LAN both work",async()=>{
 const secure=await group("sealed"),lan=await group("trusted-lan"),bytes=new TextEncoder().encode("a blob of the group"),hash=sha(bytes);
 for(const which of[secure,lan]){
  const plain=fleetSignedFetchV1({root:which.member,audience:"cache"}),sealed=fleetSignedFetchV1({root:which.member,audience:"cache",fetch:createSealedFetchV1({group:"g1",serverKey:()=>readFleetOriginKeyV1([which.store],"g1-server")})});
  expect((await sealed(`${which.endpoint}/semio/cas/${hash}`,{method:"PUT",body:bytes})).status).toBe(200);
  expect((await sealed(`${which.endpoint}/semio/cas/${hash}`)).status).toBe(200);
  expect((await fetch(`${which.endpoint}/healthz`)).status).toBe(200);
  const direct=await plain(`${which.endpoint}/semio/cas/${hash}`);
  if(which===secure){expect(direct.status).toBe(403);expect(await direct.text()).toContain("sealed transport required");}
  else expect(direct.status).toBe(200);
 }
 const status=await (await fleetSignedFetchV1({root:secure.member,audience:"cache",fetch:createSealedFetchV1({group:"g1",serverKey:()=>readFleetOriginKeyV1([secure.store],"g1-server")})})(`${secure.endpoint}/status`)).json() as {fleet:{transport:{crossGroup:string;inGroup:string;sealedRoute:boolean}}};
 expect(status.fleet.transport).toEqual({crossGroup:"sealed",inGroup:"sealed",sealedRoute:true});
});

test("the resolver of every cache client seals by itself when the published facts say so: the same call that is plain on a trusted LAN reaches a sealed group",async()=>{
 const secure=await group("sealed"),bytes=new TextEncoder().encode("read through the resolver"),hash=sha(bytes);
 const sealed=fleetSignedFetchV1({root:secure.member,audience:"cache",fetch:createSealedFetchV1({group:"g1",serverKey:()=>readFleetOriginKeyV1([secure.store],"g1-server")})});
 expect((await sealed(`${secure.endpoint}/semio/cas/${hash}`,{method:"PUT",body:bytes})).status).toBe(200);
 const admin=loadFleetSignerV1(secure.server,"admin")!;
 await publishFleetCacheFactsV1({admin,storeRoot:secure.store,directory:"/d",group:"g1",instance:"semio",port:secure.running.port,endpoints:[secure.endpoint],transport:"sealed"});
 const published=(await Bun.file(join(secure.store,"v1","fleet","cache.json")).json()) as {cache:{transport:string}};
 expect(published.cache.transport).toBe("sealed");
 writeFileSync(fleetCacheLocalPathV1(secure.member),JSON.stringify({version:1,fetchedAtMs:Date.now(),checkedAtMs:Date.now(),selectedEndpoint:secure.endpoint,facts:await Bun.file(join(secure.store,"v1","fleet","cache.json")).json()}));
 const client=remoteCacheFetchV1({SEMIO_WORKSPACE_ROOT:secure.member},{root:secure.member}),got=await client(`${secure.endpoint}/semio/cas/${hash}`);
 expect(got.status).toBe(200);expect(sha(new Uint8Array(await got.arrayBuffer()))).toBe(hash);
 expect((await fetch(`${secure.endpoint}/semio/cas/${hash}`)).status,"and a caller without any credential is still refused").toBe(401);
});
