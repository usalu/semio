import {afterAll,expect,test} from "bun:test";
import {createHash} from "node:crypto";
import {mkdirSync,mkdtempSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {encodeProtobufV1,REAPI_V1} from "../../../../🌐️remote-execution/📡️wire/📦️protobuf/🟦️.ts";
import {fleetContentHashV1,fleetSignedFetchV1,loadFleetSignerV1,syncFleetIdentityV1,fleetTargetV1} from "../../../🛰️fleet/🪪️identity/🟦️.ts";
import {createRemoteCacheHandlerV1,startRemoteCacheServerV1,type RemoteCacheServerV1} from "../../🟦️.ts";
import {RemoteCacheStoreV1} from "../../../💾️store/🟦️.ts";

const scratch:string[]=[],servers:RemoteCacheServerV1[]=[];
const directory=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`rcache-acc-${label}-`));scratch.push(path);return path;};
afterAll(async()=>{for(const server of servers)await server.stop();for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const sha=(value:Uint8Array|string)=>createHash("sha256").update(value).digest("hex");

function machine(store:string,origin:string):string{
 const root=directory(origin);mkdirSync(join(root,".cache","f"),{recursive:true});
 writeFileSync(join(root,".cache","f","machine.json"),JSON.stringify({version:1,originId:origin,groupId:"g1",storeRoot:store,key:"k"}));
 mkdirSync(join(store,"v1","origins",origin),{recursive:true});return root;
}
const result=(outputs:readonly string[])=>encodeProtobufV1(REAPI_V1.actionResult,{output_files:outputs.map((hash,index)=>({path:`o${index}`,digest:{hash,size_bytes:3}})),exit_code:0});

async function fleetServer(extra:Partial<Parameters<typeof startRemoteCacheServerV1>[0]>={}){
 const store=directory("group"),a=machine(store,"g1-m01"),b=machine(store,"g1-m02"),server=machine(store,"g1-server");
 for(const root of[a,b,server])expect((await syncFleetIdentityV1({root})).error).toBeUndefined();
 const running=await startRemoteCacheServerV1({directory:directory("cache"),host:"127.0.0.1",port:0,maximumBytes:2**26,maximumEntryBytes:2**24,maximumAgeDays:30,defaultInstance:"semio",tokens:{},validation:"structure",fleet:{storeRoot:store,group:"g1",networks:["10.77.0.0/16"]},...extra});
 servers.push(running);
 const base=`http://127.0.0.1:${running.port}/semio`;
 return {store,a,b,server,running,base,signed:(root:string)=>fleetSignedFetchV1({root,audience:"cache"})};
}

test("attack: unsigned, old bearer credentials and unknown machines get 401 on every route, and nothing is stored",async()=>{
 const fleet=await fleetServer(),key=sha("k1"),stranger=machine(directory("elsewhere"),"g1-m77");await syncFleetIdentityV1({root:stranger});
 for(const [method,path] of[["GET",`/ac/${key}`],["GET",`/cas/${key}`],["HEAD",`/cas/${key}`],["GET","/v1/cache/abc"],["PUT",`/ac/${key}`],["PUT","/v1/cache/abc"],["POST","/v1/slots"],["POST",`/claim/ac/${key}`]]){
  const response=await fetch(fleet.base+path,{method,...(method==="PUT"||method==="POST"?{body:"x"}:{})});expect(response.status,`${method} ${path}`).toBe(401);
 }
 expect((await fetch(`http://127.0.0.1:${fleet.running.port}/status`)).status).toBe(401);
 const bearer=await fetch(fleet.base+`/ac/${key}`,{headers:{authorization:"Bearer "+"A".repeat(43)}});expect(bearer.status,"the pre-signature fleet credential").toBe(401);
 const put=await fetch(fleet.base+`/ac/${key}`,{method:"PUT",headers:{authorization:"Bearer "+"A".repeat(43)},body:result([sha("x")])});expect(put.status).toBe(401);
 expect((await fleet.signed(stranger)(fleet.base+`/ac/${key}`)).status,"an origin the group store does not know").toBe(401);
 expect((await fleet.signed(fleet.a)(fleet.base+`/ac/${key}`)).status,"a registered machine: authorised, entry absent").toBe(404);
 expect(fleet.running.store.usage().entries).toBe(0);
 const health=await fetch(`http://127.0.0.1:${fleet.running.port}/healthz`);expect(health.status).toBe(200);
});

test("roles: a read key cannot write, a write key can, and a replayed or retargeted signature is refused",async()=>{
 const fleet=await fleetServer(),key=sha("k2"),body=result([sha("out")]),target=`/semio/ac/${key}`;
 const reader=loadFleetSignerV1(fleet.a,"cache-read")!,writer=loadFleetSignerV1(fleet.a,"cache-write")!;
 const signedPut=(signer:typeof reader,path:string,bytes:Uint8Array)=>fetch(`http://127.0.0.1:${fleet.running.port}${path}`,{method:"PUT",headers:signer.sign({audience:"cache",method:"PUT",target:path,content:fleetContentHashV1(bytes)}),body:bytes});
 expect((await signedPut(reader,target,body)).status,"a cache-read key used for a write").toBe(403);
 const headers=writer.sign({audience:"cache",method:"PUT",target,content:fleetContentHashV1(body)}),send=(extra:Record<string,string>={},bytes:Uint8Array=body,path=target)=>fetch(`http://127.0.0.1:${fleet.running.port}${path}`,{method:"PUT",headers:{...headers,...extra},body:bytes});
 expect((await send()).status).toBe(200);
 expect((await send()).status,"the same captured request replayed").toBe(401);
 const otherKey=sha("k2b");expect((await send({},body,`/semio/ac/${otherKey}`)).status,"the captured signature on another key").toBe(401);
 const tampered=result([sha("evil")]);
 const fresh=writer.sign({audience:"cache",method:"PUT",target:`/semio/ac/${sha("k2c")}`,content:fleetContentHashV1(body)});
 expect((await fetch(`http://127.0.0.1:${fleet.running.port}/semio/ac/${sha("k2c")}`,{method:"PUT",headers:fresh,body:tampered})).status,"body replaced in flight").toBe(400);
 const unbound=writer.sign({audience:"cache",method:"PUT",target:`/semio/ac/${sha("k2d")}`});
 expect((await fetch(`http://127.0.0.1:${fleet.running.port}/semio/ac/${sha("k2d")}`,{method:"PUT",headers:unbound,body})).status,"an action result without a bound hash").toBe(400);
 expect(fleet.running.store.usage().entries).toBe(1);
});

test("provenance: a different origin cannot replace a unit with different content, the divergence is recorded with both digests, identical content is accepted, the first uploader stays",async()=>{
 const fleet=await fleetServer(),key=sha("unit"),first=result([sha("one")]),second=result([sha("two")]),same=result([sha("one")]);
 const put=(root:string,bytes:Uint8Array)=>fleet.signed(root)(`${fleet.base}/ac/${key}`,{method:"PUT",body:bytes});
 expect((await put(fleet.a,first)).status).toBe(200);
 expect((await put(fleet.b,second)).status,"another origin, other content").toBe(409);
 expect((await put(fleet.b,same)).status,"another origin, same content").toBe(200);
 const got=await fleet.signed(fleet.b)(`${fleet.base}/ac/${key}`);expect(new Uint8Array(await got.arrayBuffer())).toEqual(first);
 expect((await put(fleet.a,second)).status,"the first uploader may update its own entry").toBe(200);
 const status=await (await fleet.signed(fleet.a)(`http://127.0.0.1:${fleet.running.port}/status`)).json() as {divergences:{count:number;recent:{keptOrigin:string;refusedOrigin:string;key:string;keptSha256:string;refusedSha256:string}[]};fleet:{origins:Record<string,number>;denied:Record<string,number>}};
 expect(status.divergences.count).toBe(1);expect(status.divergences.recent[0]).toMatchObject({keptOrigin:"g1-m01",refusedOrigin:"g1-m02",key});
 expect(status.divergences.recent[0]!.keptSha256).not.toBe(status.divergences.recent[0]!.refusedSha256);
 expect(status.fleet.origins.ok).toBe(3);expect(JSON.stringify(status)).not.toContain("publicKey");
 expect(fleet.running.store.usage().entries).toBe(1);
});

test("claims belong to the origin that made them: another origin cannot renew or release them with the same owner name",async()=>{
 const fleet=await fleetServer(),key=sha("claimed"),route=`${fleet.base}/claim/ac/${key}`,post=(root:string,path:string,body:unknown)=>fleet.signed(root)(route+path,{method:"POST",body:JSON.stringify(body)});
 const granted=await post(fleet.a,"",{owner:"build-1",ttlMilliseconds:30000,expectedMilliseconds:1000});expect(granted.status).toBe(200);expect(await granted.json()).toMatchObject({state:"granted",owner:"build-1"});
 const held=await post(fleet.b,"",{owner:"build-1",ttlMilliseconds:30000});expect(held.status,"the same owner name from another origin is another owner").toBe(409);
 const body=await held.json() as {state:string;by:string};expect(body.state).toBe("held");expect(body.by).toBe("g1-m01");
 expect((await post(fleet.b,"/renew",{owner:"build-1",ttlMilliseconds:30000})).status).toBe(409);
 expect(await (await post(fleet.b,"/release",{owner:"build-1"})).json()).toEqual({state:"absent"});
 expect((await post(fleet.a,"/renew",{owner:"build-1",ttlMilliseconds:30000})).status).toBe(200);
 expect(await (await post(fleet.a,"/release",{owner:"build-1"})).json()).toEqual({state:"released"});
});

test("the network gate refuses a peer outside the fleet CIDR before any credential is read, operator tokens work from the machine only, and a wildcard bind is refused",async()=>{
 const store=directory("gate"),root=machine(store,"g1-m01");await syncFleetIdentityV1({root});
 const cache=directory("gate-cache"),opened=await RemoteCacheStoreV1.open({directory:cache,maximumBytes:2**26,maximumEntryBytes:2**24,maximumAgeMilliseconds:0});
 const {createFleetGateV1,}=await import("../../../🛰️fleet/🪪️identity/🛂️gate/🟦️.ts"),{createFleetRegistryV1,createFleetVerifierV1}=await import("../../../🛰️fleet/🪪️identity/🟦️.ts");
 const registry=createFleetRegistryV1({storeRoot:store,group:"g1",ttlMilliseconds:0}),verifier=createFleetVerifierV1({registry,audience:"cache",adminOrigin:"g1-server",startedAtMs:0});
 const gate=createFleetGateV1({verifier,networks:["192.168.178.0/24"],tokens:{read:"r".repeat(32),write:"w".repeat(32)}});
 const handler=createRemoteCacheHandlerV1({store:opened,tokens:{},gate,defaultInstance:"semio",validation:"off"});
 const signer=loadFleetSignerV1(root,"cache-read")!,target=`/semio/cas/${sha("none")}`,ask=(peer:string|undefined,headers:Record<string,string>)=>handler(new Request("http://x"+target,{headers}),peer);
 const good=()=>signer.sign({audience:"cache",method:"GET",target});
 expect((await ask("203.0.113.9",good())).status,"outside the CIDR, a valid signature").toBe(403);
 expect((await ask(undefined,good())).status,"unknown peer").toBe(403);
 expect((await ask("192.168.178.9",good())).status,"inside the CIDR").toBe(404);
 expect((await ask("192.168.178.9",{authorization:"Bearer "+"r".repeat(32)})).status,"operator token from another machine").toBe(401);
 expect((await ask("127.0.0.1",{authorization:"Bearer "+"r".repeat(32)})).status,"operator token on the machine").toBe(404);
 const put=await handler(new Request("http://x"+target,{method:"PUT",body:"x",headers:{authorization:"Bearer "+"r".repeat(32)}}),"127.0.0.1");expect(put.status,"read token cannot write").toBe(403);
 expect((await handler(new Request("http://x/healthz"),"203.0.113.9")).status,"even the health route is closed to outsiders").toBe(403);
 await opened.close();
 await expect(startRemoteCacheServerV1({directory:directory("wild"),host:"0.0.0.0",port:0,maximumBytes:2**26,maximumEntryBytes:2**24,maximumAgeDays:30,defaultInstance:"semio",tokens:{},fleet:{storeRoot:store,group:"g1",networks:["192.168.178.0/24"]}})).rejects.toThrow("not on a wildcard");
});

test("attack: every signed write binds its body: blobs to their digest, nx archives and results to a signed hash verified while streaming, so a body swapped in flight is refused and removed",async()=>{
 const fleet=await fleetServer(),writer=loadFleetSignerV1(fleet.a,"cache-write")!,port=fleet.running.port;
 const send=(path:string,body:string|Uint8Array,content:string|undefined)=>fetch(`http://127.0.0.1:${port}${path}`,{method:"PUT",headers:writer.sign({audience:"cache",method:"PUT",target:path,...(content===undefined?{}:{content})}),body});
 const good="payload",digest=sha(good),blob=`/semio/cas/${digest}`;
 expect((await send(blob,good,undefined)).status,"unbound blob").toBe(400);
 expect((await send(blob,good,sha("other"))).status,"blob signed for another digest").toBe(400);
 expect((await send(`/semio/cas/${sha("evil")}`,"evil but",sha("evil"))).status,"body does not match the digest it is stored under").toBe(400);
 expect((await send(blob,good,digest)).status).toBe(200);
 expect((await send("/semio/v1/cache/archive1","archive",undefined)).status,"unbound nx archive").toBe(400);
 expect((await send("/semio/v1/cache/archive1","swapped!",sha("archive"))).status,"nx archive swapped in flight").toBe(400);
 expect((await fleet.signed(fleet.a)(`http://127.0.0.1:${port}/semio/v1/cache/archive1`)).status,"the swapped archive was removed").toBe(404);
 expect((await send("/semio/v1/cache/archive2","archive",sha("archive"))).status).toBe(200);
 expect(await (await fleet.signed(fleet.b)(`http://127.0.0.1:${port}/semio/v1/cache/archive2`)).text()).toBe("archive");
});

const unitResult=(manifest:string,outputs:readonly string[])=>encodeProtobufV1(REAPI_V1.actionResult,{output_files:[{path:"unit.json",digest:{hash:manifest,size_bytes:3}},...outputs.map((hash,index)=>({path:`out/o${index}`,digest:{hash,size_bytes:3}}))],exit_code:0});

test("provenance: two builds of one unit whose manifests differ only in their timestamps are the same result, other output bytes are still a divergence",async()=>{
 const fleet=await fleetServer(),key=sha("manifest unit"),put=(root:string,bytes:Uint8Array)=>fleet.signed(root)(`${fleet.base}/ac/${key}`,{method:"PUT",body:bytes});
 expect((await put(fleet.a,unitResult(sha("built at 1"),[sha("rlib"),sha("rmeta")]))).status).toBe(200);
 expect((await put(fleet.b,unitResult(sha("built at 2"),[sha("rmeta"),sha("rlib")]))).status,"another origin, same outputs, other manifest").toBe(200);
 expect((await put(fleet.b,unitResult(sha("built at 3"),[sha("other rlib"),sha("rmeta")]))).status,"another origin, other outputs").toBe(409);
 const status=await (await fleet.signed(fleet.a)(`http://127.0.0.1:${fleet.running.port}/status`)).json() as {divergences:{count:number}};
 expect(status.divergences.count).toBe(1);
});

test("consumption: the status names which origin read the results of which origin, and how many of them were compiled units",async()=>{
 const fleet=await fleetServer(),unit=sha("consumed unit"),record=sha("consumed record");
 const put=(root:string,key:string,bytes:Uint8Array)=>fleet.signed(root)(`${fleet.base}/ac/${key}`,{method:"PUT",body:bytes});
 expect((await put(fleet.a,unit,unitResult(sha("m"),[sha("rlib")]))).status).toBe(200);
 expect((await put(fleet.a,record,result([sha("record.br")]))).status).toBe(200);
 const get=async(root:string,key:string)=>{const answer=await fleet.signed(root)(`${fleet.base}/ac/${key}`);await answer.arrayBuffer();return answer.status;};
 expect(await get(fleet.b,unit)).toBe(200);expect(await get(fleet.b,unit)).toBe(200);expect(await get(fleet.b,record)).toBe(200);expect(await get(fleet.a,unit)).toBe(200);expect(await get(fleet.b,sha("absent"))).toBe(404);
 const head=await fleet.signed(fleet.b)(`${fleet.base}/ac/${unit}`,{method:"HEAD"});expect(head.status).toBe(200);
 const status=await (await fleet.signed(fleet.a)(`http://127.0.0.1:${fleet.running.port}/status`)).json() as {consumption:{since:number;total:number;pairs:Record<string,{hits:number;units:number}>}};
 expect(status.consumption.total).toBe(4);expect(status.consumption.since).toBeGreaterThan(0);
 expect(status.consumption.pairs).toEqual({"g1-m01>g1-m02":{hits:3,units:2},"g1-m01>g1-m01":{hits:1,units:1}});
});
