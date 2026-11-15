import {afterAll,expect,test} from "bun:test";
import Ajv from "ajv";
import {createHash} from "node:crypto";
import {mkdirSync,mkdtempSync,readFileSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {createFleetRegistryV1,createFleetVerifierV1,fleetPeerAllowedV1,fleetSignedFetchV1,loadFleetSignerV1,readFleetOriginKeyV1,syncFleetIdentityV1} from "../../🛰️fleet/🪪️identity/🟦️.ts";
import {createSealedFetchV1} from "../../🔐️sealed/🟦️.ts";
import {createFleetGateV1} from "../../🛰️fleet/🪪️identity/🛂️gate/🟦️.ts";
import {startRemoteCacheServerV1,type RemoteCacheServerV1} from "../../🖥️server/🟦️.ts";
import {graphFleetClientV1} from "../../../🪪️graph-state/🛰️exchange/🟦️.ts";
import {publishServedV1,publishWantedV1,readServedV1,readWantedV1} from "../../../🪪️graph-state/🙋️wanted/🟦️.ts";
import {createFleetPeerReadThroughV1,directPeerTransportV1,forwardableRequestersV1,fleetPeerEndpointsV1,relayedPeerTransportV1,requireFleetPeerShapeV1,servePeerExchangeV1,type FleetPeerTransportV1} from "../🟦️.ts";
import {spawnSync} from "node:child_process";
import {testPythonV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import other from "./🧫️fixtures/🌍️.json" with {type:"json"};
import vectors from "./🧫️fixtures/🔣️.json" with {type:"json"};

const scratch:string[]=[],servers:RemoteCacheServerV1[]=[],controllers:AbortController[]=[];
const directory=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`peers-${label}-`));scratch.push(path);return path;};
afterAll(async()=>{for(const controller of controllers)controller.abort();for(const server of servers)await server.stop().catch(()=>{});for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const sha=(value:Uint8Array|string)=>createHash("sha256").update(value).digest("hex");

function machine(store:string,group:string,origin:string):string{
 const root=directory(origin);mkdirSync(join(root,".cache","f"),{recursive:true});
 writeFileSync(join(root,".cache","f","machine.json"),JSON.stringify({version:1,originId:origin,groupId:group,storeRoot:store,key:"k"}));
 mkdirSync(join(store,"v1","origins",origin),{recursive:true});return root;
}
type Prepared={id:string;store:string;server:string;member:string};
type Group=Prepared&{port:number;running:RemoteCacheServerV1;endpoint:string};
async function prepare(id:string):Promise<Prepared>{
 const store=directory(`${id}-store`),server=machine(store,id,`${id}-server`),member=machine(store,id,`${id}-m01`);
 for(const root of[server,member])expect((await syncFleetIdentityV1({root})).error).toBeUndefined();
 return {id,store,server,member};
}
async function start(own:Prepared,peers:Readonly<Record<string,string>>,transports:readonly FleetPeerTransportV1[]):Promise<Group>{
 const through=createFleetPeerReadThroughV1({transports,timeoutMilliseconds:1500,missMilliseconds:20,pauseMilliseconds:60000});
 const running=await startRemoteCacheServerV1({directory:directory(`${own.id}-cache`),host:"127.0.0.1",port:0,maximumBytes:2**28,maximumEntryBytes:2**24,maximumAgeDays:30,defaultInstance:"semio",tokens:{},fleet:{storeRoot:own.store,group:own.id,peerStores:peers,registryTtlMilliseconds:50,root:own.server},peers:through});
 servers.push(running);
 return {...own,port:running.port,running,endpoint:`http://127.0.0.1:${running.port}`};
}
const sealedTo=(peerStore:string,peer:string,base?:typeof fetch)=>createSealedFetchV1({group:peer,serverKey:()=>readFleetOriginKeyV1([peerStore],`${peer}-server`),...(base===undefined?{}:{fetch:base})});
const direct=(own:Prepared,peer:string,peerStore:string,endpoint:()=>string|undefined,base?:typeof fetch)=>directPeerTransportV1({group:peer,endpoints:async()=>{const found=endpoint();return found===undefined?[]:[found];},fetch:fleetSignedFetchV1({root:own.server,audience:"cache",roleOf:()=>"cache-read",fetch:sealedTo(peerStore,peer,base)}),maximumBytes:2**24});
async function pair(viaA?:()=>string|undefined,base?:typeof fetch){
 const one=await prepare("g1"),two=await prepare("g2"),late:{a?:Group;b?:Group}={};
 const a=await start(one,{g2:two.store},[direct(one,"g2",two.store,()=>late.b?.endpoint)]);late.a=a;
 const b=await start(two,{g1:one.store},[direct(two,"g1",one.store,()=>viaA?.()??late.a?.endpoint,base)]);late.b=b;
 return {a,b};
}
const signed=(root:string,audience:"cache"="cache")=>fleetSignedFetchV1({root,audience});
const across=(root:string,answering:Group)=>fleetSignedFetchV1({root,audience:"cache",fetch:sealedTo(answering.store,answering.id)});

async function statusOf(group:Group){const response=await signed(group.member)(`${group.endpoint}/status`);expect(response.status).toBe(200);return await response.json() as {counters:Record<"ac"|"cas"|"nx",{hits:number;misses:number;puts:number}>;peers?:{hits:number;misses:number;errors:number;skipped:number}};}
const put=(group:Group,bytes:Uint8Array)=>signed(group.member)(`${group.endpoint}/semio/cas/${sha(bytes)}`,{method:"PUT",body:bytes});

test("a blob uploaded in group A is a hit for a machine of group B exactly once through the server of B and local afterwards, with no network check in the way",async()=>{
 const {a,b}=await pair();
 const registryA=createFleetRegistryV1({storeRoot:a.store,group:"g1",peerStores:{g2:b.store}});
 const gateOpen=createFleetGateV1({verifier:createFleetVerifierV1({registry:registryA,audience:"cache",adminOrigin:"g1-server"}),home:"g1"}),gateCidr=createFleetGateV1({verifier:undefined,networks:["10.0.1.0/24"]});
 expect(fleetPeerAllowedV1("10.0.2.7",["10.0.1.0/24"]),"the former CIDR gate refuses a machine of the other network").toBe(false);
 expect(gateCidr.network("10.0.2.7")).toBe(false);expect(gateOpen.network("10.0.2.7"),"without the optional policy the address is not a criterion").toBe(true);
 const bytes=new TextEncoder().encode("blob of group one ".repeat(500)),hash=sha(bytes);
 expect((await put(a,bytes)).status).toBe(200);
 expect((await signed(b.member)(`${b.endpoint}/semio/cas/${hash}`,{method:"HEAD"})).status,"a HEAD answers for the own store only").toBe(404);
 const first=await signed(b.member)(`${b.endpoint}/semio/cas/${hash}`);expect(first.status).toBe(200);expect(sha(new Uint8Array(await first.arrayBuffer()))).toBe(hash);
 const afterFirst=await statusOf(a);expect(afterFirst.counters.cas.hits,"A served the blob once").toBe(1);
 expect(b.running.store.usage().entries,"B stores the hit").toBeGreaterThanOrEqual(1);
 expect(await b.running.store.has("semio","cas",hash)).toBe(true);
 const second=await signed(b.member)(`${b.endpoint}/semio/cas/${hash}`);expect(second.status).toBe(200);expect(sha(new Uint8Array(await second.arrayBuffer()))).toBe(hash);
 expect((await statusOf(a)).counters.cas.hits,"the second read is local, A is not asked again").toBe(1);
 const status=await statusOf(b);expect(status.peers?.hits).toBe(1);expect(status.counters.cas.hits).toBe(2);
 expect((await signed(b.member)(`${b.endpoint}/semio/cas/${sha("absent")}`)).status).toBe(404);
});

test("a request without a signature, from a foreign fleet, or from a peer group that writes is refused, and uploads stay local",async()=>{
 const {a,b}=await pair();
 const bytes=new TextEncoder().encode("only for group one"),hash=sha(bytes);expect((await put(a,bytes)).status).toBe(200);
 expect((await fetch(`${a.endpoint}/semio/cas/${hash}`)).status).toBe(401);
 expect((await fetch(`${a.endpoint}/semio/cas/${hash}`,{headers:{authorization:"Bearer "+"A".repeat(43)}})).status).toBe(401);
 const foreignStore=directory("g9-store"),foreign=machine(foreignStore,"g9","g9-m01");await syncFleetIdentityV1({root:foreign});
 expect((await signed(foreign)(`${a.endpoint}/semio/cas/${hash}`)).status,"an origin of a fleet that no topology names").toBe(401);
 expect((await signed(foreign)(`${b.endpoint}/semio/cas/${hash}`)).status).toBe(401);
 expect((await signed(b.member)(`${a.endpoint}/semio/cas/${hash}`)).status,"a machine of group two reading the cache of group one without sealing is refused, whatever it signs").toBe(403);
 expect((await across(b.member,a)(`${a.endpoint}/semio/cas/${hash}`)).status,"sealed, a machine of group two may read the cache of group one").toBe(200);
 const written=new TextEncoder().encode("intruder");
 expect((await across(b.member,a)(`${a.endpoint}/semio/cas/${sha(written)}`,{method:"PUT",body:written})).status,"but it may not write there").toBe(403);
 expect((await across(b.member,a)(`${a.endpoint}/semio/claim/ac/${sha("k")}`,{method:"POST",body:JSON.stringify({owner:"g2-m01",ttlMilliseconds:1000})})).status,"nor claim work there").toBe(403);
 expect(await a.running.store.has("semio","cas",sha(written))).toBe(false);
});

test("when the link is down the group keeps working on its own, quickly and without an error",async()=>{
 const {a,b}=await pair();
 await a.running.stop();
 const own=new TextEncoder().encode("local to group two"),started=performance.now();
 expect((await put(b,own)).status).toBe(200);
 for(let round=0;round<4;round++)expect((await signed(b.member)(`${b.endpoint}/semio/cas/${sha("missing"+round)}`)).status).toBe(404);
 expect((await signed(b.member)(`${b.endpoint}/semio/cas/${sha(own)}`)).status).toBe(200);
 expect(performance.now()-started,"a dead peer costs no long wait").toBeLessThan(6000);
 const status=await statusOf(b);expect(status.peers?.errors).toBeGreaterThanOrEqual(1);expect(status.peers?.skipped,"after three failures the peer is left out for a while").toBeGreaterThanOrEqual(1);
});

test("a wanted request of group two is answered by the server of group one, and the answer comes back, through the caches alone",async()=>{
 const {a,b}=await pair();
 const client=(group:Group,root:string)=>graphFleetClientV1({endpoint:group.endpoint,instance:"semio",token:undefined,upload:true,stateDirectory:directory("breaker")},{fetch:signed(root)});
 const clientA=client(a,a.member),clientB=client(b,b.member);
 expect(await readWantedV1({client:clientA,requester:"g2-m01"}),"nothing asked yet").toBeUndefined();
 await Bun.sleep(30);
 expect(await publishWantedV1({client:clientB,requester:"g2-m01",platform:"linux-x64",graphKey:sha("state-1"),projects:["p-a","p-b"]})).toBe(true);
 const seen=await readWantedV1({client:clientA,requester:"g2-m01"});
 expect(seen?.projects).toEqual(["p-a","p-b"]);expect(seen?.platform).toBe("linux-x64");
 expect(await readServedV1({client:clientB,requester:"g2-m01"})).toBeUndefined();
 await Bun.sleep(30);
 expect(await publishServedV1({client:clientA,server:"g1-m01",request:seen!,projects:["p-a"],generation:7})).toBe(true);
 const served=await readServedV1({client:clientB,requester:"g2-m01"});
 expect(served?.server).toBe("g1-m01");expect(served?.projects).toEqual(["p-a"]);expect(served?.generation).toBe(7);
 await Bun.sleep(30);
 expect(await publishWantedV1({client:clientB,requester:"g2-m01",platform:"linux-x64",graphKey:sha("state-2"),projects:["p-c"]})).toBe(true);
 expect((await readWantedV1({client:clientA,requester:"g2-m01"}))?.graphKey,"a newer request is seen at once: the mutable entries are not kept by the reader").toBe(sha("state-2"));
});

test("the relayed route gives the same bytes without any route between the groups, and a silent peer is a plain miss",async()=>{
 const one=await prepare("g1"),two=await prepare("g2");
 const a=await start(one,{g2:two.store},[]);
 const b=await start(two,{g1:one.store},[relayedPeerTransportV1({group:"g1",ownGroup:"g2",ownStore:two.store,peerStore:one.store,pollMilliseconds:20,maximumBytes:2**24})]);
 const bytes=new TextEncoder().encode("through the stores ".repeat(2000)),hash=sha(bytes);expect((await put(a,bytes)).status).toBe(200);
 expect((await signed(b.member)(`${b.endpoint}/semio/cas/${hash}`)).status,"nobody answers the request file yet: a miss after the bound, not an error").toBe(404);
 const controller=new AbortController();controllers.push(controller);
 const own=fleetSignedFetchV1({root:a.server,audience:"cache",roleOf:()=>"cache-read"});
 void servePeerExchangeV1({ownGroup:"g1",ownStore:one.store,peers:{g2:two.store},signal:controller.signal,pollMilliseconds:20,answer:async path=>{const response=await own(a.endpoint+path);return response.status===200?{status:200,bytes:new Uint8Array(await response.arrayBuffer())}:{status:response.status};}});
 await Bun.sleep(1700);
 const reached=await signed(b.member)(`${b.endpoint}/semio/cas/${hash}`);expect(reached.status).toBe(200);expect(sha(new Uint8Array(await reached.arrayBuffer()))).toBe(hash);
 expect(await b.running.store.has("semio","cas",hash)).toBe(true);
 expect((await signed(b.member)(`${b.endpoint}/semio/cas/${sha("nothing")}`)).status).toBe(404);
 controller.abort();
});

test("the peer folder of the topology and the attested facts give the endpoints of a peer, and only readable cache paths ever travel",async()=>{
 const store=directory("facts");mkdirSync(join(store,"v1","fleet"),{recursive:true});
 writeFileSync(join(store,"v1","fleet","cache.json"),JSON.stringify({version:1,group:"g2",generation:1,publishedAtMs:1,publisher:{directory:"/d",port:1,host:"h"},cache:{endpoints:["http://10.0.2.1:7001"],instance:"semio",access:"read-write",protocols:["bazel-http","nx-self-hosted"],protocolVersion:1},auth:{scheme:"semio-fleet-v1",roles:["cache-read","cache-write","submit","worker","admin"]}}));
 expect(await fleetPeerEndpointsV1(store,["http://peer.example:7001","http://10.0.2.1:7001"])).toEqual(["http://10.0.2.1:7001","http://peer.example:7001"]);
 expect(await fleetPeerEndpointsV1(undefined,["http://peer.example:7001"])).toEqual(["http://peer.example:7001"]);
 expect(await fleetPeerEndpointsV1(directory("empty"))).toEqual([]);
 const through=createFleetPeerReadThroughV1({transports:[]});
 for(const bad of["/status","/semio/cas/zz","/../etc/passwd","/semio/claim/ac/"+sha("x")])expect(await through.fetch(bad),bad).toBeUndefined();
});

test("the exchange schema agrees with an independent JSON-schema implementation on every vector",()=>{
 const ajv=new Ajv({strict:true,allowUnionTypes:true}),check=(definition:string)=>ajv.compile({$schema:schema.$schema,definitions:schema.definitions,$ref:`#/definitions/${definition}`});
 for(const [definition,rows] of Object.entries(vectors) as [string,{valid:unknown[];invalid:unknown[]}][]){
  for(const value of rows.valid){expect(check(definition)(value),`${definition} ${JSON.stringify(value)}`).toBe(true);expect(()=>requireFleetPeerShapeV1(definition as "Path",value,"x")).not.toThrow();}
  for(const value of rows.invalid){expect(check(definition)(value),`${definition} ${JSON.stringify(value)}`).toBe(false);expect(()=>requireFleetPeerShapeV1(definition as "Path",value,"x")).toThrow();}
 }
});

test("a peer origin is admitted only for a named group, with its own group in its registration, and a topology pin outranks first sight",async()=>{
 const own=directory("own-store"),peer=directory("peer-store"),root=machine(peer,"g2","g2-m01");await syncFleetIdentityV1({root});
 const unnamed=createFleetRegistryV1({storeRoot:own,group:"g1"});await unnamed.refresh(true);await unnamed.learn("g2-m01");expect(unnamed.key("g2-m01","cache-read")).toBeUndefined();
 const named=createFleetRegistryV1({storeRoot:own,group:"g1",peerStores:{g2:peer}});await named.refresh(true);expect(named.status("g2-m01").status).toBe("ok");expect(named.groupOf("g2-m01")).toBe("g2");expect(named.key("g2-m01","cache-read")).toBeDefined();
 const wrong=createFleetRegistryV1({storeRoot:own,group:"g1",peerStores:{g3:peer}});await wrong.refresh(true);expect(wrong.key("g2-m01","cache-read"),"the registration names another group than the folder").toBeUndefined();
 const registration=JSON.parse(readFileSync(join(peer,"v1","origins","g2-m01","fleet-auth.json"),"utf8")) as {origin:string};
 const pinned=createFleetRegistryV1({storeRoot:own,group:"g1",peerStores:{g2:peer},pinned:{"g2-m01":registration.origin}});await pinned.refresh(true);expect(pinned.status("g2-m01").status).toBe("ok");
 const forged=createFleetRegistryV1({storeRoot:own,group:"g1",peerStores:{g2:peer},pinned:{"g2-m01":"A".repeat(60)}});await forged.refresh(true);expect(forged.status("g2-m01").status).toBe("conflict");
});

test("a machine of a peer group is forwardable only when nobody in its own group works in its platform class",async()=>{
 const own=directory("fw-own"),peer=directory("fw-peer"),now=1_000_000;
 const attest=(store:string,id:string,platform:string,age:number)=>{mkdirSync(join(store,"v1","origins",id),{recursive:true});writeFileSync(join(store,"v1","origins",id,"attest.json"),JSON.stringify({platform,wallMs:now-age}));};
 attest(peer,"g2-server","linux",0);attest(peer,"g2-m01","windows",1000);attest(peer,"g2-m02","macos",1000);attest(peer,"g2-m03","linux",1000);attest(peer,"g2-m04","macos",900_000);
 attest(own,"g3-m01","windows",1000);attest(own,"g3-m02","windows",1000);attest(own,"g3-m03","linux",1000);
 expect(await forwardableRequestersV1({ownStore:own,peers:{g2:peer},groups:["g3"],now:()=>now,windowMilliseconds:600_000}),"g2-m02 and g2-m03 share a class (posix), the stale g2-m04 does not count, g2-m01 is alone on windows; g3 is read from the relay copy").toEqual(["g2-m01","g3-m03"]);
});

test("nothing in the fleet code names a home network address or one of the three service ports, and the topology declares the ports and the optional network policy",()=>{
 const base=join(import.meta.dirname,"..","..",".."),files=["🌐️remote-execution/🛰️sidecar/🗄️cache/🟦️.ts","🌐️remote-execution/🛰️sidecar/🗓️scheduler/🟦️.ts","🌐️remote-execution/🛰️sidecar/🧰️shared/🟦️.ts","🌐️remote-cache/🛰️fleet/📣️publication/🟦️.ts","🌐️remote-cache/🧭️peers/🟦️.ts","🌐️remote-cache/🖥️server/🟦️.ts"];
 for(const file of files)expect(readFileSync(join(base,file),"utf8"),file).not.toMatch(/192\.168\.178|\b(6094|8980|8981)\b/u);
 const policy=JSON.parse(readFileSync(join(base,"🔣️policy.json"),"utf8")) as {remoteCache:{server:Record<string,unknown>}};
 expect(policy.remoteCache.server.fleetPort,"the cache port of the fleet is a fact of the topology").toBeUndefined();
});

test("the scheduler of group one takes work from a machine of group two by its signature from any address, never as a worker, and refuses a signature from outside the fleet",async()=>{
 const one=await prepare("g1"),two=await prepare("g2"),stranger=await prepare("g9");
 const registry=createFleetRegistryV1({storeRoot:one.store,group:"g1",peerStores:{g2:two.store},registryTtlMilliseconds:0} as never),verifier=createFleetVerifierV1({registry,audience:"execution",adminOrigin:"g1-server"}),gate=createFleetGateV1({verifier,home:"g1"});
 await registry.refresh(true);
 const ask=(root:string,role:"submit"|"worker",peer:string,sealed=true)=>{
  const target="/build.v2.Execution/Execute",headers=loadFleetSignerV1(root,role)!.sign({audience:"execution",method:"POST",target});
  return gate.check({method:"POST",target,authorization:headers.authorization!,content:headers["x-semio-content-sha256"]!,peer,roles:["submit","worker"],tokens:[],sealed});
 };
 const own=await ask(one.member,"worker","10.0.1.20"),foreign=await ask(two.member,"submit","10.0.2.77"),foreignWorker=await ask(two.member,"worker","10.0.2.77"),outsider=await ask(stranger.member,"submit","10.0.2.78");
 const plain=await ask(two.member,"submit","10.0.2.77",false),ownPlain=await ask(one.member,"worker","10.0.1.20",false);
 expect(plain.ok?"ok":(plain as {status:number}).status,"a peer group's submit outside the sealed exchange is refused").toBe(403);
 expect(ownPlain.ok,"the own group on a trusted LAN may stay plain").toBe(true);
 expect(own.ok,"a worker of the own group").toBe(true);
 expect(foreign.ok&&foreign.access?.group,"a submitter of the peer group, from an address the CIDR of the group would refuse").toBe("g2");
 expect(foreignWorker.ok?"ok":(foreignWorker as {status:number}).status).toBe(403);
 expect(outsider.ok?"ok":(outsider as {status:number}).status).toBe(401);
});

test("an independent Python implementation agrees with the schema on every vector, with SHA-256 on the answer bodies and with the CIDR policy on every address",()=>{
 for(const [peer,network,expected] of other.cidr as [string,string,boolean][])expect(fleetPeerAllowedV1(peer,[network]),`${peer} in ${network}`).toBe(expected);
 for(const [text,digest] of other.sha256 as [string,string][])expect(sha(text)).toBe(digest);
 const python=testPythonV1();
 if(python!==undefined){const run=spawnSync(python,["-I",join(import.meta.dirname,"🐍️oracle","🐍️.py"),"check"],{encoding:"utf8"});expect(run.status,run.stdout+run.stderr).toBe(0);}
});

type Tap={port:number;sent:Buffer[];received:Buffer[];stop():void};
function tap(target:number,change?:(direction:"sent"|"received",chunk:Buffer,count:number)=>Buffer):Tap{
 const sent:Buffer[]=[],received:Buffer[]=[],counts={sent:0,received:0},listener=Bun.listen<{up?:Promise<Awaited<ReturnType<typeof Bun.connect>>>}>({hostname:"127.0.0.1",port:0,socket:{
  open(client){
   client.data={};
   client.data.up=Bun.connect({hostname:"127.0.0.1",port:target,socket:{data(_up,chunk){const buffer=Buffer.from(chunk);received.push(buffer);client.write(change?change("received",buffer,counts.received++)??buffer:buffer);},close(){client.end();},error(){client.end();}}});
  },
  async data(client,chunk){const buffer=Buffer.from(chunk);sent.push(buffer);(await client.data.up!).write(change?change("sent",buffer,counts.sent++)??buffer:buffer);},
  async close(client){(await client.data.up?.catch(()=>undefined))?.end();},
  error(){},
 }});
 return {port:listener.port,sent,received,stop:()=>listener.stop(true)};
}
const seen=(parts:readonly Buffer[],needle:string|Buffer)=>Buffer.concat(parts).includes(needle);

test("law: a passive observer of the direct link between two groups sees no blob byte, no digest, no path, no project name, no credential and no origin; an in-group request on the trusted LAN is plain and the same observer reads it",async()=>{
 const one=await prepare("g1"),holder:{tap?:Tap}={};
 const probe=await prepare("g2");void probe;
 const {a,b}=await (async()=>{
  const two=await prepare("g2"),late:{a?:Group;b?:Group}={};
  const a=await start(one,{g2:two.store},[direct(one,"g2",two.store,()=>late.b?.endpoint)]);late.a=a;
  holder.tap=tap(a.port);
  const b=await start(two,{g1:one.store},[direct(two,"g1",one.store,()=>`http://127.0.0.1:${holder.tap!.port}`)]);late.b=b;
  return {a,b};
 })();
 const project="🦀️project-alpha-confidential",bytes=new TextEncoder().encode(`${project} payload `.repeat(300)),hash=sha(bytes);
 expect((await put(a,bytes)).status).toBe(200);
 const reached=await signed(b.member)(`${b.endpoint}/semio/cas/${hash}`);expect(reached.status).toBe(200);expect(sha(new Uint8Array(await reached.arrayBuffer()))).toBe(hash);
 const wire=[...holder.tap!.sent,...holder.tap!.received],text=Buffer.concat(wire).toString("latin1");
 expect(wire.length,"the observer did see the link").toBeGreaterThan(0);expect(text).toContain("POST /v1/fleet/sealed");
 for(const forbidden of[project,"payload",hash,hash.slice(0,16),"cas/","semio/","Semio-Fleet","authorization","g2-server","g1-server","cache-read","x-semio-hop","x-semio-content"])expect(seen(wire,forbidden),`the observer saw: ${forbidden}`).toBe(false);
 expect(seen(wire,Buffer.from(bytes.subarray(0,64))),"no blob bytes").toBe(false);
 expect(Buffer.concat(wire).length,"the blob did travel").toBeGreaterThan(bytes.length);
 const plainTap=tap(a.port),plainFetch=await signed(a.member)(`http://127.0.0.1:${plainTap.port}/semio/cas/${hash}`);expect(plainFetch.status).toBe(200);await plainFetch.arrayBuffer();
 expect(seen([...plainTap.sent,...plainTap.received],project),"control: the same observer reads a plain in-group request").toBe(true);
 expect(seen(plainTap.sent,"Semio-Fleet"),"control: and the credentials").toBe(true);
 plainTap.stop();holder.tap!.stop();
});

test("law: a message changed on the link, in either direction, is refused and nothing of it is kept, and a recorded message replayed is refused and answers nothing",async()=>{
 for(const direction of["sent","received"] as const){
  const one=await prepare("g1"),two=await prepare("g2"),late:{a?:Group;b?:Group}={},holder:{tap?:Tap}={};
  const a=await start(one,{g2:two.store},[]);late.a=a;
  let flipped=false;
  holder.tap=tap(a.port,(way,chunk)=>{if(way!==direction||flipped||chunk.length<400)return chunk;flipped=true;const copy=Buffer.from(chunk);copy[copy.length-9]=copy[copy.length-9]!^1;return copy;});
  const b=await start(two,{g1:one.store},[direct(two,"g1",one.store,()=>`http://127.0.0.1:${holder.tap!.port}`)]);late.b=b;
  const bytes=new TextEncoder().encode(`changed on the link ${direction} `.repeat(200)),hash=sha(bytes);
  expect((await put(a,bytes)).status).toBe(200);
  expect((await signed(b.member)(`${b.endpoint}/semio/cas/${hash}`)).status,`a byte flipped ${direction}`).toBe(404);
  expect(flipped).toBe(true);
  expect(await b.running.store.has("semio","cas",hash),"nothing of a refused exchange is stored").toBe(false);
  expect((await statusOf(b)).peers?.errors).toBeGreaterThanOrEqual(1);
  holder.tap.stop();
 }
 const one=await prepare("g1"),two=await prepare("g2"),late:{a?:Group;b?:Group}={},holder:{tap?:Tap}={};
 const a=await start(one,{g2:two.store},[]);late.a=a;holder.tap=tap(a.port);
 const b=await start(two,{g1:one.store},[direct(two,"g1",one.store,()=>`http://127.0.0.1:${holder.tap!.port}`)]);late.b=b;
 const bytes=new TextEncoder().encode("replayed ".repeat(100)),hash=sha(bytes);expect((await put(a,bytes)).status).toBe(200);
 expect((await signed(b.member)(`${b.endpoint}/semio/cas/${hash}`)).status).toBe(200);
 const before=(await statusOf(a)).counters.cas.hits,recorded=Buffer.concat(holder.tap.sent);
 expect(recorded.toString("latin1")).toContain("POST /v1/fleet/sealed");
 const socket=await Bun.connect({hostname:"127.0.0.1",port:a.port,socket:{data(_s,chunk){answer.push(Buffer.from(chunk));},close(){},error(){}}}),answer:Buffer[]=[];
 socket.write(recorded.subarray(0,recorded.indexOf("\r\n\r\n")+4+Number(/content-length: (\d+)/iu.exec(recorded.toString("latin1"))![1])));
 await Bun.sleep(300);socket.end();
 expect(Buffer.concat(answer).toString("latin1").split("\r\n")[0],"the recorded message again").toContain("401");
 expect((await statusOf(a)).counters.cas.hits,"and nothing was served for it").toBe(before);
 holder.tap.stop();
});
