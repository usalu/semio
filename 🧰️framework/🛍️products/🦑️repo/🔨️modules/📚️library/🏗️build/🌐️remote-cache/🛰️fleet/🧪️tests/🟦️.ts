import {afterAll,expect,test} from "bun:test";
import Ajv from "ajv";
import {spawnSync} from "node:child_process";
import {existsSync,mkdirSync,mkdtempSync,readdirSync,readFileSync,rmSync,statSync,symlinkSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {fileURLToPath} from "node:url";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import contractSchema from "../../../🏃️execution/🧬️schema/🌐️remote-cache.json" with {type:"json"};
import {fleetCacheFactsPathV1,fleetCacheLocalPathV1,parseFleetCacheFactsV1,readFleetExecutionV1,readLocalFleetCacheFactsV1,refreshFleetCacheFactsV1,remoteCacheFetchV1,requireFleetCacheShapeV1,resolveAuthorisedRemoteCacheV1,resolveRemoteCacheV1,type FleetCacheFactsV1} from "../🟦️.ts";
import {fleetCacheEndpointsV1,publishFleetCacheFactsV1,setFleetExecutionV1} from "../📣️publication/🟦️.ts";
import {syncFleetIdentityV1} from "../🪪️identity/🟦️.ts";
import {nxRemoteCacheEnvironmentV1} from "../../🧶️nx/🟦️.ts";
import {routeNxThroughFlightV1} from "../../🧶️nx/🚦️single-flight/🟦️.ts";
import {remoteCacheStatusV1,serveRemoteCacheV1,rotateRemoteCacheV1,type RemoteCacheCommandContextV1} from "../../⌨️command/🟦️.ts";
import {bazelRemoteCacheToolV1} from "../../../🔧️tools/🏗️bazel/🌐️remote-cache/🟦️.ts";
import {startRemoteCacheServerV1,type RemoteCacheServerV1} from "../../🖥️server/🟦️.ts";

const ajv=new Ajv({strict:true,allowUnionTypes:true}),validator=(definition:string)=>ajv.compile({$schema:schema.$schema,definitions:schema.definitions,$ref:`#/definitions/${definition}`}),statusValidator=ajv.compile({$schema:contractSchema.$schema,definitions:contractSchema.definitions,$ref:"#/definitions/Status"}),scratch:string[]=[],servers:RemoteCacheServerV1[]=[];
const directory=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`fleet-${label}-`));scratch.push(path);return path;};
afterAll(async()=>{for(const server of servers)await server.stop();for(const path of scratch)rmSync(path,{recursive:true,force:true});});

const machine=(root:string,storeRoot:string,origin:string="g1-m01")=>{mkdirSync(join(root,".cache","f"),{recursive:true});writeFileSync(join(root,".cache","f","machine.json"),JSON.stringify({version:1,originId:origin,groupId:"g1",storeRoot,key:"k"}));mkdirSync(join(storeRoot,"v1","origins",origin),{recursive:true});};
const ADMIN={role:"admin" as const,originId:"g1-server"};
const publication=(storeRoot:string,cache:string,endpoints:readonly string[],now:number=Date.now())=>({admin:ADMIN,storeRoot,directory:cache,group:"g1",instance:"semio",endpoints,port:6090,host:"h1",now});
const mode=(path:string)=>statSync(path).mode&0o777;
const posix=process.platform!=="win32";

async function serving(cache:string,store:string,extra:Partial<Parameters<typeof startRemoteCacheServerV1>[0]>={}):Promise<RemoteCacheServerV1>{
 const server=await startRemoteCacheServerV1({directory:cache,host:"127.0.0.1",port:0,maximumBytes:2**26,maximumEntryBytes:2**24,maximumAgeDays:30,defaultInstance:"semio",tokens:{},validation:"off",fleet:{storeRoot:store,group:"g1",networks:["10.77.0.0/16"],registryTtlMilliseconds:200},...extra});
 servers.push(server);return server;
}

const sample:FleetCacheFactsV1={version:1,group:"g1",generation:1,publishedAtMs:1,publisher:{directory:"/var/lib/semio-remote-cache",port:6090,host:"h1"},cache:{endpoints:["http://10.0.0.5:6090"],instance:"semio",access:"read-write",protocols:["bazel-http","nx-self-hosted"],protocolVersion:1},auth:{scheme:"semio-fleet-v1",roles:["cache-read","cache-write","submit","worker","admin"]}};
const formerFormat={...Object.fromEntries(Object.entries(sample).filter(([name])=>name!=="auth")),credential:{scope:"fleet",id:"0123456789abcdef",token:"A".repeat(43),rotatedAtMs:1}};

test("the fleet schema agrees with an independent JSON-schema implementation, and the former format with a shared token is no longer valid",()=>{
 const withExecution={...sample,execution:{endpoint:"http://10.0.0.5:6091",protocolVersion:2}};
 for(const value of[sample,withExecution]){expect(validator("Facts")(value),JSON.stringify(validator("Facts").errors)).toBe(true);expect(()=>requireFleetCacheShapeV1("Facts",value,"x")).not.toThrow();}
 const broken:unknown[]=[{...sample,extra:1},{...sample,auth:{...sample.auth,scheme:"bearer"}},{...sample,cache:{...sample.cache,endpoints:[]}},{...sample,cache:{...sample.cache,access:"read-only"}},{...sample,execution:{endpoint:"ftp://x"}},{...sample,generation:0},{...sample,credential:{scope:"fleet",id:"0123456789abcdef",token:"A".repeat(43),rotatedAtMs:1}},formerFormat];
 for(const value of broken){expect(validator("Facts")(value),JSON.stringify(value).slice(0,100)).toBe(false);expect(()=>requireFleetCacheShapeV1("Facts",value,"x")).toThrow();}
 expect(()=>parseFleetCacheFactsV1(JSON.stringify(formerFormat))).toThrow();
 const copy={version:1,fetchedAtMs:1,checkedAtMs:2,selectedEndpoint:null,facts:sample};
 expect(validator("LocalCopy")(copy)).toBe(true);expect(validator("LocalCopy")({...copy,extra:1})).toBe(false);expect(()=>requireFleetCacheShapeV1("LocalCopy",{...copy,extra:1},"x")).toThrow();
 expect(validator("LocalCopy")({...copy,facts:formerFormat}),"a local copy of the former format").toBe(false);
});

test("publishing writes the facts without any secret, once, keeps the execution fact, raises the generation with the content, and the admin key takes over from any earlier publisher",async()=>{
 const store=directory("store"),cache=directory("cache");
 const first=await publishFleetCacheFactsV1(publication(store,cache,["http://10.0.0.5:6090"],1000));
 expect(first.changed).toBe(true);expect(first.facts.generation).toBe(1);expect(first.facts.execution).toBeUndefined();
 const path=fleetCacheFactsPathV1(store);expect(path).toBe(join(store,"v1","fleet","cache.json"));
 const text=readFileSync(path,"utf8");expect(parseFleetCacheFactsV1(text)).toEqual(first.facts);expect(validator("Facts")(JSON.parse(text))).toBe(true);
 expect(text.toLowerCase()).not.toContain("token");expect(text).not.toContain("credential");expect(existsSync(join(cache,"fleet","credential.json")),"no credential file is minted").toBe(false);
 expect((await publishFleetCacheFactsV1(publication(store,cache,["http://10.0.0.5:6090"],2000))).changed,"an unchanged publication writes nothing").toBe(false);
 expect(readFileSync(path,"utf8")).toContain('"publishedAtMs": 1000');
 const executed=await setFleetExecutionV1(store,{endpoint:"http://10.0.0.5:6091",protocolVersion:2},3000);expect(executed.execution).toEqual({endpoint:"http://10.0.0.5:6091",protocolVersion:2});
 const again=await publishFleetCacheFactsV1(publication(store,cache,["http://10.0.0.5:6090"],4000));expect(again.changed).toBe(false);expect(again.facts.execution).toEqual(executed.execution);
 const moved=await publishFleetCacheFactsV1(publication(store,cache,["http://10.0.0.6:6090","http://10.0.0.5:6090"],5000));expect(moved.changed).toBe(true);expect(moved.facts.execution).toEqual(executed.execution);expect(moved.facts.generation).toBeGreaterThan(first.facts.generation);
 expect((await setFleetExecutionV1(store,null,6000)).execution).toBeUndefined();
 await expect(setFleetExecutionV1(directory("empty"),null)).rejects.toThrow("Published fleet cache facts required");
 const theirs=await publishFleetCacheFactsV1({admin:ADMIN,storeRoot:store,directory:directory("other"),group:"g1",instance:"semio",endpoints:["http://10.0.0.9:6091"],port:6091,host:"scratch",now:Date.now()+11*60000});
 expect(theirs.facts.generation,"a stale publisher of another identity takes over above the generation in place").toBeGreaterThan(moved.facts.generation);
 const back=await publishFleetCacheFactsV1({...publication(store,cache,["http://10.0.0.5:6090"]),now:Date.now()+1000});expect(back.facts.generation,"the admin key takes over at once").toBeGreaterThan(theirs.facts.generation);
});

test("facts of the former format with the shared token are never read by a client and are replaced by the admin-signed publication",async()=>{
 const store=directory("store"),cache=directory("cache"),root=directory("client");machine(root,store);
 mkdirSync(join(store,"v1","fleet"),{recursive:true});
 writeFileSync(fleetCacheFactsPathV1(store),JSON.stringify({...formerFormat,publishedAtMs:Date.now(),publisher:{directory:"/elsewhere",port:6090,host:"old"}}));
 const refreshed=await refreshFleetCacheFactsV1({root,force:true,probeMilliseconds:100});
 expect(refreshed.state,"a client of the new code reads nothing from the old format").toBe("absent");
 expect(resolveRemoteCacheV1({SEMIO_WORKSPACE_ROOT:root},{root})).toBeUndefined();
 const found=await resolveAuthorisedRemoteCacheV1({SEMIO_WORKSPACE_ROOT:root},{root,probeMilliseconds:100});
 expect(found).toMatchObject({resolution:undefined,authorised:false});
 const own=await publishFleetCacheFactsV1({...publication(store,cache,["http://10.0.0.5:6090"]),directory:"/elsewhere",port:6090,host:"old",now:Date.now()+1000});
 expect(own.facts.auth.scheme,"the same publisher rewrites its former facts in the new format").toBe("semio-fleet-v1");expect(JSON.stringify(own.facts)).not.toContain("credential");
});

test("the resolver takes the environment first, completes it from the facts, pairs a signature only with an endpoint the facts name, and never puts a secret in the environment",async()=>{
 const store=directory("store"),root=directory("resolve");machine(root,store);
 const copy={version:1 as const,fetchedAtMs:Date.now(),checkedAtMs:Date.now(),selectedEndpoint:"http://10.0.0.6:6090",facts:{...sample,cache:{...sample.cache,endpoints:["http://10.0.0.5:6090","http://10.0.0.6:6090"]}}};
 expect(resolveRemoteCacheV1({},{root,copy:undefined})).toBeUndefined();
 const unregistered=resolveRemoteCacheV1({},{root,copy})!;
 expect(unregistered.contract).toMatchObject({endpoint:"http://10.0.0.6:6090",instance:"semio",access:"read-write",upload:true});
 expect(unregistered,"no identity yet: nothing to authenticate with").toMatchObject({endpointFrom:"fleet-facts",tokenFrom:"none",token:undefined,generation:1});
 expect((await syncFleetIdentityV1({root})).error).toBeUndefined();
 const bare=resolveRemoteCacheV1({},{root,copy})!;
 expect(bare).toMatchObject({tokenFrom:"fleet-identity",token:undefined});expect(JSON.stringify(bare.environment)).not.toMatch(/"[A-Za-z0-9_-]{43,}"/u);expect(bare.environment.SEMIO_REMOTE_CACHE_TOKEN).toBeUndefined();
 const explicit=resolveRemoteCacheV1({SEMIO_REMOTE_CACHE_ENDPOINT:"http://10.0.0.5:6090",SEMIO_REMOTE_CACHE_INSTANCE:"mine",SEMIO_REMOTE_CACHE_ACCESS:"read-only",SEMIO_REMOTE_CACHE_TOKEN:"explicit-token-1234567"},{root,copy})!;
 expect(explicit.contract).toMatchObject({endpoint:"http://10.0.0.5:6090",instance:sample.cache.instance,access:"read-only"});expect(explicit).toMatchObject({endpointFrom:"environment",tokenFrom:"environment",token:"explicit-token-1234567",instanceFrom:"fleet-facts",instanceRefused:"mine"});
 const kept=resolveRemoteCacheV1({SEMIO_REMOTE_CACHE_ENDPOINT:"http://10.0.0.5:6090",SEMIO_REMOTE_CACHE_INSTANCE:"mine",SEMIO_REMOTE_CACHE_ENDPOINT_OVERRIDE:"1"},{root,copy})!;expect(kept.contract.instance,"only the operator's override keeps another instance").toBe("mine");expect(kept).toMatchObject({instanceFrom:"environment",instanceRefused:null});
 const leftover=resolveRemoteCacheV1({SEMIO_REMOTE_CACHE_INSTANCE:"semio-leftover"},{root,copy})!;expect(leftover,"an instance of the environment that the healthy facts contradict is a leftover: the facts win and the refusal is reported").toMatchObject({instanceFrom:"fleet-facts",instanceRefused:"semio-leftover"});expect(leftover.contract.instance).toBe(sample.cache.instance);expect(leftover.environment.SEMIO_REMOTE_CACHE_INSTANCE).toBe(sample.cache.instance);
 expect(resolveRemoteCacheV1({SEMIO_REMOTE_CACHE_INSTANCE:sample.cache.instance},{root,copy})).toMatchObject({instanceFrom:"environment",instanceRefused:null});
 const named=resolveRemoteCacheV1({SEMIO_REMOTE_CACHE_ENDPOINT:"http://10.0.0.5:6090"},{root,copy})!;expect(named).toMatchObject({endpointFrom:"environment",tokenFrom:"fleet-identity"});
 const stale=resolveRemoteCacheV1({SEMIO_REMOTE_CACHE_ENDPOINT:"http://evil.example:6099"},{root,copy})!;expect(stale,"a pointer the healthy facts do not name is a leftover: the facts win").toMatchObject({endpointFrom:"fleet-facts",tokenFrom:"fleet-identity"});expect(stale.contract.endpoint).toBe("http://10.0.0.6:6090");
 const foreign=resolveRemoteCacheV1({SEMIO_REMOTE_CACHE_ENDPOINT:"http://evil.example:6099",SEMIO_REMOTE_CACHE_ENDPOINT_OVERRIDE:"1"},{root,copy})!;expect(foreign).toMatchObject({endpointFrom:"environment"});expect(foreign.contract.endpoint).toBe("http://evil.example:6099");expect(foreign.tokenFrom,"no credential for an endpoint the facts do not name").toBe("none");
 const seen:Record<string,string|null>[]=[],spy=(async(input:RequestInfo|URL,init?:RequestInit)=>{seen.push({url:String(input),authorization:new Headers(init?.headers).get("authorization")});return new Response("");}) as typeof fetch;
 const call=remoteCacheFetchV1({SEMIO_WORKSPACE_ROOT:root},{root,base:spy});
 const facts={...sample.cache,endpoints:["http://10.0.0.5:6090"]};writeFileSync(fleetCacheLocalPathV1(root),JSON.stringify({...copy,facts:{...sample,cache:facts}}));
 await call("http://10.0.0.5:6090/semio/ac/"+"0".repeat(64));await call("http://evil.example:6090/semio/ac/"+"0".repeat(64));await call("https://static.example/download.tgz");
 expect(seen[0]!.authorization,"the fleet endpoint receives a signature").toStartWith("Semio-Fleet o=g1-m01 r=cache-read a=cache");
 expect(seen[1]!.authorization,"a foreign host receives nothing").toBeNull();expect(seen[2]!.authorization).toBeNull();
 const withToken=remoteCacheFetchV1({SEMIO_WORKSPACE_ROOT:root,SEMIO_REMOTE_CACHE_ENDPOINT:"http://ci.example:6090",SEMIO_REMOTE_CACHE_ENDPOINT_OVERRIDE:"1",SEMIO_REMOTE_CACHE_TOKEN:"explicit-token-1234567"},{root,base:spy});
 const withoutOverride=remoteCacheFetchV1({SEMIO_WORKSPACE_ROOT:root,SEMIO_REMOTE_CACHE_ENDPOINT:"http://ci.example:6090",SEMIO_REMOTE_CACHE_TOKEN:"explicit-token-1234567"},{root,base:spy});
 await withToken("http://ci.example:6090/semio/ac/x");await withToken("http://other.example/x");
 expect(seen[3]!.authorization,"an operator token goes to its own endpoint").toBe("Bearer explicit-token-1234567");expect(seen[4]!.authorization,"and nowhere else").toBeNull();
 await withoutOverride("http://ci.example:6090/semio/ac/x");expect(seen[5]!.authorization,"a leftover pointer that the healthy facts contradict receives nothing").toBeNull();
 expect(readFleetExecutionV1({},{root,copy})).toBeUndefined();
 expect(readFleetExecutionV1({},{root,copy:{...copy,facts:{...copy.facts,execution:{endpoint:"http://10.0.0.5:6091",protocolVersion:1}}}})).toEqual({endpoint:"http://10.0.0.5:6091",protocolVersion:1});
});

test("a machine is authorised by its own signature with no token anywhere, and keeps its last known copy while the store is gone",async()=>{
 const store=directory("store"),cache=directory("cache"),root=directory("worker"),other=directory("other");machine(root,store);machine(other,store,"g1-m02");
 const server=await serving(cache,store),endpoint=server.url;
 await publishFleetCacheFactsV1(publication(store,cache,["http://192.0.2.1:6090",endpoint]));
 const environment={SEMIO_WORKSPACE_ROOT:root};
 expect(resolveRemoteCacheV1(environment,{root}),"nothing known before the first refresh").toBeUndefined();
 const found=await resolveAuthorisedRemoteCacheV1(environment,{root,probeMilliseconds:300});
 expect(found.authorised,`status ${found.status}`).toBe(true);expect(found.status).toBe(404);expect(found.refresh?.state).toBe("updated");
 expect(found.resolution).toMatchObject({endpointFrom:"fleet-facts",tokenFrom:"fleet-identity"});expect(found.resolution?.contract.endpoint,"skips the unreachable endpoint").toBe(endpoint);
 if(posix){expect(mode(fleetCacheLocalPathV1(root))).toBe(0o600);expect(mode(join(root,".cache","f","identity","cache-write.key"))).toBe(0o600);}
 const signed=remoteCacheFetchV1({...environment,...found.resolution!.environment},{root}),url=`${endpoint}/semio/v1/cache/42`;
 expect((await signed(url,{method:"PUT",body:"hello"})).status).toBe(200);
 const code=async(...headers:string[])=>{const child=Bun.spawn(["curl","-s","-o","/dev/null","-w","%{http_code}",...headers,url],{stdout:"pipe"});const output=await new Response(child.stdout).text();await child.exited;return output;};
 expect(await code(),"an independent client without a signature").toBe("401");expect(await code("-H","Authorization: Bearer "+"A".repeat(43)),"an independent client with a bearer token").toBe("401");
 const status=await (await signed(`${endpoint}/status`)).json() as {entries:number;fleet:{origins:Record<string,number>}};expect(status.entries).toBe(1);expect(status.fleet.origins.ok).toBeGreaterThanOrEqual(1);
 const strangers=await resolveAuthorisedRemoteCacheV1({SEMIO_WORKSPACE_ROOT:other},{root:other,probeMilliseconds:300});expect(strangers.authorised,"a second machine joins by itself, with no step of its own").toBe(true);
 writeFileSync(join(store,"v1","origins","g1-m02","retired.json"),JSON.stringify({version:1,originId:"g1-m02",by:"g1-m01",hlc:"1",wallMs:Date.now(),reason:"gone",retired:true}));
 await Bun.sleep(600);
 const revoked=remoteCacheFetchV1({SEMIO_WORKSPACE_ROOT:other},{root:other});expect((await revoked(url)).status,"a retired origin is refused at once").toBe(401);
 expect((await signed(url)).status,"the others are not affected").toBe(200);
 rmSync(store,{recursive:true,force:true});
 const gone=await refreshFleetCacheFactsV1({root,force:true,probeMilliseconds:300});expect(gone.state).toBe("stale");expect(gone.copy?.facts.generation).toBeGreaterThanOrEqual(1);
 expect(resolveRemoteCacheV1(environment,{root})?.tokenFrom,"the last known value still serves").toBe("fleet-identity");
});

test("the cut-over: a machine on the new code against a server on the previous code, and the previous code against the new server, both end local and neither fails or hangs",async()=>{
 const store=directory("store"),root=directory("new-code");machine(root,store);
 const requests:string[]=[];
 const previous=Bun.serve({hostname:"127.0.0.1",port:0,fetch:request=>{requests.push(`${request.method} ${new URL(request.url).pathname} ${request.headers.get("authorization")?.split(" ")[0]??"-"}`);const given=request.headers.get("authorization");return new Response(given===`Bearer ${"A".repeat(43)}`?"ok":"Unauthorized",{status:given===`Bearer ${"A".repeat(43)}`?404:401,headers:{"www-authenticate":"Bearer"}});}});
 try{
  const endpoint=`http://127.0.0.1:${previous.port}`;
  await publishFleetCacheFactsV1(publication(store,directory("cache"),[endpoint]));
  const started=performance.now(),found=await resolveAuthorisedRemoteCacheV1({SEMIO_WORKSPACE_ROOT:root},{root,probeMilliseconds:500});
  expect(performance.now()-started,"bounded").toBeLessThan(5000);
  expect(found.authorised,"the previous server does not know signatures: refused").toBe(false);expect(found.status).toBe(401);
  const warned:string[]=[],environment=await nxRemoteCacheEnvironmentV1({SEMIO_WORKSPACE_ROOT:root},{root,probeMilliseconds:500,warn:text=>warned.push(text)});
  expect(environment,"Nx stays local").toEqual({NX_SELF_HOSTED_REMOTE_CACHE_SERVER:undefined,NX_SELF_HOSTED_REMOTE_CACHE_ACCESS_TOKEN:undefined});expect(warned.length,"one bilingual warning").toBe(1);
  expect(requests.some(line=>line.includes("Semio-Fleet")),"what it sent was a signature, not a secret").toBe(true);expect(requests.some(line=>line.endsWith("Bearer"))).toBe(false);
 }finally{previous.stop(true);}
 const cache=directory("cache2"),server=await serving(cache,store);
 const bearer=await fetch(`${server.url}/semio/v1/cache/x`,{headers:{authorization:`Bearer ${"A".repeat(43)}`}});await bearer.body?.cancel();
 expect(bearer.status,"the previous code sends its shared token: refused, it builds locally").toBe(401);
 const stale=directory("old-client");machine(stale,store);
 writeFileSync(fleetCacheLocalPathV1(stale),JSON.stringify({version:1,fetchedAtMs:Date.now(),checkedAtMs:Date.now(),selectedEndpoint:server.url,facts:formerFormat}));
 expect(readLocalFleetCacheFactsV1(stale),"a local copy of the former format is dropped by the new code").toBeUndefined();
});

test("a refresh is skipped within the TTL, reports a missing store as absent and a stalled store as stale within the timeout",async()=>{
 const store=directory("store"),cache=directory("cache"),root=directory("worker");machine(root,store);
 expect((await refreshFleetCacheFactsV1({root,probeMilliseconds:100})).state,"no facts yet").toBe("absent");
 await publishFleetCacheFactsV1(publication(store,cache,["http://127.0.0.1:1"]));
 const now=Date.now(),first=await refreshFleetCacheFactsV1({root,probeMilliseconds:100,now});expect(first.state).toBe("updated");expect(first.copy?.selectedEndpoint).toBeNull();
 expect((await refreshFleetCacheFactsV1({root,probeMilliseconds:100,now:now+1000})).state).toBe("fresh");
 expect((await refreshFleetCacheFactsV1({root,probeMilliseconds:100,now:now+31000})).state).toBe("checked");
 expect(readLocalFleetCacheFactsV1(root,now+8*86400000),"a copy older than seven days is not trusted").toBeUndefined();
 if(!posix)return;
 const facts=fleetCacheFactsPathV1(store);rmSync(facts);
 expect(spawnSync("mkfifo",[facts]).status,"mkfifo").toBe(0);
 const started=performance.now(),stalled=await refreshFleetCacheFactsV1({root,force:true,timeoutMilliseconds:300});
 expect(performance.now()-started).toBeLessThan(2000);expect(stalled.state).toBe("stale");expect(stalled.error).toContain("took longer");expect(stalled.copy?.facts.generation).toBe(1);
 const fresh=directory("worker2");machine(fresh,store);
 expect((await refreshFleetCacheFactsV1({root:fresh,timeoutMilliseconds:300})).state,"a stalled store with nothing known").toBe("absent");
 const opener=Bun.spawn(["sh","-c",`exec 3<>"${facts}"; sleep 0`]);await opener.exited;
});

test("only the holder of the group server's admin key publishes the facts, and endpoint discovery lists the private addresses",async()=>{
 const store=directory("store"),cache=directory("cache");
 await expect(publishFleetCacheFactsV1({...publication(store,cache,["http://10.0.0.5:6090"]),admin:{role:"cache-write" as never,originId:"g1-server"}})).rejects.toThrow("admin signing key");
 await expect(publishFleetCacheFactsV1({...publication(store,cache,["http://10.0.0.5:6090"]),admin:{role:"admin",originId:"g1-m03"}})).rejects.toThrow("admin signing key");
 expect(existsSync(fleetCacheFactsPathV1(store)),"nothing was written").toBe(false);
 const interfaces={en0:[{address:"192.168.1.9",family:"IPv4",internal:false},{address:"169.254.3.3",family:"IPv4",internal:false}],lo0:[{address:"127.0.0.1",family:"IPv4",internal:true}],en1:[{address:"10.0.0.4",family:"IPv4",internal:false},{address:"fe80::1",family:"IPv6",internal:false}],docker:[{address:"203.0.113.4",family:"IPv4",internal:false}]} as never;
 expect(fleetCacheEndpointsV1(6090,["http://given:6090"],["http://topology.internal:6090"],interfaces)).toEqual(["http://given:6090","http://10.0.0.4:6090","http://192.168.1.9:6090","http://topology.internal:6090"]);
 expect(fleetCacheEndpointsV1(6090,[],[],{} as never)).toEqual(["http://127.0.0.1:6090"]);
});

test("the serve command never publishes the facts; it trusts only the registered origins, and rotation replaces a machine's keys",async()=>{
 const root=directory("server-root"),real=directory("real-store"),cache=directory("cache");machine(root,real,"g1-server");
 const member=directory("member-root");machine(member,real,"g1-m05");
 const written:string[]=[],controller=new AbortController(),context=(signal:AbortSignal=controller.signal,at:string=root):RemoteCacheCommandContextV1=>({root:at,environment:{SEMIO_FLEET_NETWORK:"10.77.0.0/16"},signal,progress:async()=>{},write:line=>written.push(line),tools:async()=>[]});
 const running=serveRemoteCacheV1(["--host","127.0.0.1","--port","0","--directory",cache,"--lang","en"],context());
 for(let wait=0;wait<100&&!written.some(line=>line.startsWith("[remote-cache] listening "));wait++)await Bun.sleep(50);
 const url=written.find(line=>line.startsWith("[remote-cache] listening "))!.slice("[remote-cache] listening ".length);
 expect(existsSync(fleetCacheFactsPathV1(real)),"a served cache publishes nothing: only the supervised sidecar does").toBe(false);
 const call=async(headers:Record<string,string>={})=>{const response=await fetch(`${url}/status`,{headers});await response.body?.cancel();return response.status;};
 expect(await call()).toBe(401);expect(await call({authorization:"Bearer wrong-token-wrong-token"})).toBe(401);
 expect((await syncFleetIdentityV1({root:member})).error).toBeUndefined();
 const signed=remoteCacheFetchV1({SEMIO_WORKSPACE_ROOT:member,SEMIO_REMOTE_CACHE_ENDPOINT:url},{root:member});
 const published=await publishFleetCacheFactsV1({admin:ADMIN,storeRoot:directory("facts"),directory:cache,group:"g1",instance:"main",port:1,endpoints:[url],transport:"sealed"});
 writeFileSync(fleetCacheLocalPathV1(member),JSON.stringify({version:1,fetchedAtMs:Date.now(),checkedAtMs:Date.now(),selectedEndpoint:url,facts:published.facts}));
 expect((await signed(`${url}/status`)).status,"a member of the group store").toBe(200);
 const before=readFileSync(join(member,".cache","f","identity","cache-read.key"),"utf8");
 const rotate=await rotateRemoteCacheV1(["--lang","en"],context(controller.signal,member));expect(rotate.exitCode).toBe(0);
 expect(readFileSync(join(member,".cache","f","identity","cache-read.key"),"utf8")).not.toBe(before);expect(written.join("\n")).toContain("rotated to sequence 2");
 let accepted=0;for(let wait=0;wait<140&&accepted!==200;wait++){accepted=(await signed(`${url}/status`)).status;if(accepted!==200)await Bun.sleep(50);}
 expect(accepted,"the new keys are accepted once the server has re-read the store (within its 5 s window)").toBe(200);
 expect((await rotateRemoteCacheV1(["--lang","en"],{...context(),root:directory("not-joined")})).exitCode,"a directory that is no joined machine").toBe(1);
 controller.abort();expect((await running).exitCode).toBe(0);
},20000);

const nxModules=process.env.SEMIO_TEST_NX_NODE_MODULES,nxPackage=process.env.SEMIO_TEST_NX_PACKAGE;

test.skipIf(!nxModules||!nxPackage||!posix)("one real Nx 23 task is uploaded from one checkout through the signing front and fetched by another checkout with a cold local cache, with no token anywhere",async()=>{
 const store=directory("store"),cache=directory("cache"),checkouts=[directory("machine-a"),directory("machine-b")],server=await serving(cache,store);
 await publishFleetCacheFactsV1(publication(store,cache,[server.url]));
 const count=async(kind:"puts"|"hits"|"misses")=>((await (await remoteCacheFetchV1({SEMIO_WORKSPACE_ROOT:checkouts[0]!},{root:checkouts[0]!})(`${server.url}/status`)).json()) as {counters:{nx:Record<string,number>}}).counters.nx[kind]!;
 for(const [index,root] of checkouts.entries()){
  machine(root,store,`g1-m0${index+1}`);
  mkdirSync(join(root,"libs/a"),{recursive:true});mkdirSync(join(root,"node_modules"));for(const entry of readdirSync(nxModules!))if(entry!=="nx")symlinkSync(join(nxModules!,entry),join(root,"node_modules",entry));symlinkSync(nxPackage!,join(root,"node_modules","nx"));
  writeFileSync(join(root,"nx.json"),JSON.stringify({cacheDirectory:".nx-cache",useDaemonProcess:false,analytics:false,targetDefaults:{build:{cache:true,inputs:["{projectRoot}/**/*"],outputs:["{projectRoot}/out"]}}}));
  writeFileSync(join(root,"libs/a/project.json"),JSON.stringify({name:"a",root:"libs/a",targets:{build:{executor:"nx:run-commands",options:{command:"mkdir -p out && echo built-a > out/a.txt",cwd:"libs/a"}}}}));
  writeFileSync(join(root,"libs/a/src.txt"),"a\n");
 }
 const run=async(root:string)=>{
  await resolveAuthorisedRemoteCacheV1({SEMIO_WORKSPACE_ROOT:root},{root,probeMilliseconds:2000});
  const environment={SEMIO_WORKSPACE_ROOT:root,PATH:process.env.PATH!},routed=await nxRemoteCacheEnvironmentV1(environment,{root,probeMilliseconds:2000});
  expect(routed.NX_SELF_HOSTED_REMOTE_CACHE_ACCESS_TOKEN,"no token for Nx itself").toBeUndefined();
  const flight=(await routeNxThroughFlightV1({...environment,...routed}))!;
  try{
   const child=Bun.spawn(["node",join(root,"node_modules/nx/dist/bin/nx.js"),"run-many","-t","build"],{cwd:root,stdout:"pipe",stderr:"pipe",env:{PATH:process.env.PATH!,HOME:process.env.HOME!,NODE_OPTIONS:"--preserve-symlinks --preserve-symlinks-main",NX_DAEMON:"false",NX_NO_CLOUD:"true",NX_TUI:"false",CI:"1",...flight.environment()}});
   const [out,err,status]=await Promise.all([new Response(child.stdout).text(),new Response(child.stderr).text(),child.exited]);return {out,err,status};
  }finally{await flight.stop();}
 };
 const first=await run(checkouts[0]!);expect(first.status,first.err+first.out).toBe(0);expect(first.out).toContain("0/1 hit");
 expect(await count("puts"),"the first checkout uploaded its task").toBe(1);
 const second=await run(checkouts[1]!);expect(second.status,second.err+second.out).toBe(0);expect(second.out).toContain("1/1 hit (100%)");expect(second.out).toContain("remote cache");
 expect(readFileSync(join(checkouts[1]!,"libs/a/out/a.txt"),"utf8")).toBe("built-a\n");
 expect(await count("hits"),"the second checkout was served by the server").toBeGreaterThanOrEqual(1);expect(await count("puts"),"nothing was uploaded twice").toBe(1);
},240000);

test("a refresh ignores facts whose endpoints all fail and keeps the previous good local copy",async()=>{
 const store=directory("store"),cache=directory("cache"),dead=directory("dead"),root=directory("worker");machine(root,store);
 const server=await serving(cache,store);
 await publishFleetCacheFactsV1(publication(store,cache,[server.url]));
 const now=Date.now(),good=await refreshFleetCacheFactsV1({root,probeMilliseconds:300,now});expect(good.state).toBe("updated");expect(good.copy?.selectedEndpoint).toBe(server.url);
 rmSync(fleetCacheFactsPathV1(store));
 await publishFleetCacheFactsV1({admin:ADMIN,storeRoot:store,directory:dead,group:"g1",instance:"semio",endpoints:["http://127.0.0.1:1"],port:6191,host:"scratch"});
 const ignored=await refreshFleetCacheFactsV1({root,force:true,probeMilliseconds:300});
 expect(ignored.state).toBe("stale");expect(ignored.error).toContain("previous good copy");expect(ignored.copy?.facts.cache.endpoints).toEqual([server.url]);
 expect(resolveRemoteCacheV1({SEMIO_WORKSPACE_ROOT:root},{root})?.contract.endpoint,"the dead endpoint is not adopted").toBe(server.url);
});

test("round trip: facts written by the publisher are read by the cache resolver, the execution reader and the scheduler fact setter; a stale pointer to another port of the cache machine is superseded; `status` validates against its schema with the signed scheme reports the server's own numbers, and renders Bazel through the signing proxy",async()=>{
 const store=directory("store"),cache=directory("cache"),root=directory("client");machine(root,store);
 const server=await serving(cache,store),endpoint=server.url;
 await publishFleetCacheFactsV1(publication(store,cache,[endpoint]));
 await setFleetExecutionV1(store,{endpoint:"http://192.0.2.7:8980",protocolVersion:1});
 const text=readFileSync(fleetCacheFactsPathV1(store),"utf8"),facts=parseFleetCacheFactsV1(text);
 expect(facts.execution,"the scheduler's fact survives a later publication").toEqual({endpoint:"http://192.0.2.7:8980",protocolVersion:1});
 await publishFleetCacheFactsV1(publication(store,cache,[endpoint]));
 expect(parseFleetCacheFactsV1(readFileSync(fleetCacheFactsPathV1(store),"utf8")).execution).toEqual(facts.execution);
 expect((await refreshFleetCacheFactsV1({root,force:true,probeMilliseconds:300})).state).toBe("updated");
 expect(readFleetExecutionV1({SEMIO_WORKSPACE_ROOT:root},{root})).toEqual({endpoint:"http://192.0.2.7:8980",protocolVersion:1});
 const stale={SEMIO_WORKSPACE_ROOT:root,SEMIO_REMOTE_CACHE_ENDPOINT:`http://${new URL(endpoint).hostname}:6090`};
 const resolved=resolveRemoteCacheV1(stale,{root})!;
 expect(resolved,"a leftover pointer on the same machine is superseded by the facts").toMatchObject({endpointFrom:"fleet-facts",tokenFrom:"fleet-identity"});expect(resolved.contract.endpoint).toBe(endpoint);
 const lines:string[]=[],context:RemoteCacheCommandContextV1={root,environment:{SEMIO_WORKSPACE_ROOT:root,...stale},signal:new AbortController().signal,progress:async()=>{},write:line=>lines.push(line),tools:async()=>[bazelRemoteCacheToolV1()]};
 const status=await remoteCacheStatusV1(["--lang","en"],context) as {remote:{authorised:boolean;entries:number|null;bytes:number|null;maximumBytes:number|null;hits:number|null;misses:number|null};server:{fleet:{transport:{inGroup:string}}}|null;serverFault:null;resolution:{tokenFrom:string};renderings:{bazel:{applied:string[]}}};
 expect(status.remote.authorised).toBe(true);expect(status.resolution.tokenFrom).toBe("fleet-identity");
 expect(statusValidator(status),JSON.stringify(statusValidator.errors)).toBe(true);
 expect(status.serverFault).toBeNull();expect(status.server,"the signed /status request is answered").not.toBeNull();expect(status.server!.fleet.transport.inGroup).toBeDefined();
 expect(status.remote).toMatchObject({entries:0,bytes:0,maximumBytes:2**26});expect(typeof status.remote.hits).toBe("number");expect(typeof status.remote.misses).toBe("number");
 expect(status.renderings.bazel.applied).toContain("--remote_cache=http://127.0.0.1:<proxy>/semio");expect(JSON.stringify(status.renderings.bazel)).not.toContain("credential_helper");
});

test("status names why the server gave no numbers: an unsigned machine is denied, a dead endpoint is unreachable, a foreign document is malformed",async()=>{
 const store=directory("store"),cache=directory("cache"),root=directory("client"),bare=directory("bare");machine(root,store);
 const server=await serving(cache,store),endpoint=server.url;
 await publishFleetCacheFactsV1(publication(store,cache,[endpoint]));
 await refreshFleetCacheFactsV1({root,force:true,probeMilliseconds:300});
 const context=(workspace:string,environment:Record<string,string>):RemoteCacheCommandContextV1=>({root:workspace,environment:{SEMIO_WORKSPACE_ROOT:workspace,...environment},signal:new AbortController().signal,progress:async()=>{},write:()=>{},tools:async()=>[bazelRemoteCacheToolV1()]});
 type Row={server:unknown;serverFault:{reason:string;status:number|null;message:string}|null;remote:{entries:number|null;hits:number|null}};
 const signed=await remoteCacheStatusV1([],context(root,{SEMIO_REMOTE_CACHE_ENDPOINT:endpoint})) as Row;
 expect(signed.serverFault).toBeNull();expect(signed.remote.entries).toBe(0);
 const denied=await remoteCacheStatusV1([],context(bare,{SEMIO_REMOTE_CACHE_ENDPOINT:endpoint})) as Row;
 expect(statusValidator(denied),JSON.stringify(statusValidator.errors)).toBe(true);
 expect(denied.server).toBeNull();expect(denied.serverFault).toMatchObject({reason:"denied",status:401});expect(denied.serverFault!.message).toContain("HTTP 401");expect(denied.serverFault!.message).toContain("HTTP 401");expect(denied.remote).toMatchObject({entries:null,hits:null});
 expect(denied.serverFault!.message).toMatch(/refused[\s\S]*abgelehnt/u);
 const dead=await remoteCacheStatusV1([],context(bare,{SEMIO_REMOTE_CACHE_ENDPOINT:"http://127.0.0.1:1",SEMIO_REMOTE_CACHE_TIMEOUT_SECONDS:"2"})) as Row;
 expect(dead.serverFault).toMatchObject({reason:"unreachable",status:null});
 const stub=Bun.serve({port:0,hostname:"127.0.0.1",fetch:request=>new URL(request.url).pathname==="/status"?Response.json({version:1,entries:"many"}):new Response("",{status:404})});
 try{
  const odd=await remoteCacheStatusV1([],context(bare,{SEMIO_REMOTE_CACHE_ENDPOINT:`http://127.0.0.1:${stub.port}`})) as Row;
  expect(odd.server).toBeNull();expect(odd.serverFault).toMatchObject({reason:"malformed",status:200});
 }finally{await stub.stop(true);}
});

test("a healthy verdict of the fleet facts routes Nx at once and the probe runs behind it, so a hanging cache host costs the build nothing",async()=>{
 const hanging=Bun.serve({port:0,fetch:()=>new Promise<Response>(()=>{})}),url=`http://127.0.0.1:${hanging.port}`,root=directory("verdict-root"),warnings:string[]=[];
 try{
  mkdirSync(join(root,".cache","f"),{recursive:true});
  const facts={...sample,cache:{...sample.cache,endpoints:[url],instance:"main"}},write=(checkedAtMs:number,selectedEndpoint:string|null)=>writeFileSync(fleetCacheLocalPathV1(root),JSON.stringify({version:1,fetchedAtMs:checkedAtMs,checkedAtMs,selectedEndpoint,facts}));
  const {nxCachedVerdictV1,NX_VERDICT_MILLISECONDS_V1}=await import("../../🧶️nx/🟦️.ts"),now=Date.now();
  expect(nxCachedVerdictV1(undefined,now)).toBe(false);expect(nxCachedVerdictV1({selectedEndpoint:null,checkedAtMs:now},now)).toBe(false);
  expect(nxCachedVerdictV1({selectedEndpoint:url,checkedAtMs:now-NX_VERDICT_MILLISECONDS_V1-1},now)).toBe(false);expect(nxCachedVerdictV1({selectedEndpoint:url,checkedAtMs:now-1000},now)).toBe(true);
  write(Date.now(),url);
  const started=performance.now(),routed=await nxRemoteCacheEnvironmentV1({SEMIO_WORKSPACE_ROOT:root},{root,probeMilliseconds:5000,retries:1,warn:text=>warnings.push(text)});
  expect(performance.now()-started,"no probe on the critical path").toBeLessThan(400);
  expect(routed.NX_SELF_HOSTED_REMOTE_CACHE_SERVER).toBe(`${url}/main`);
  const again=performance.now();expect(await nxRemoteCacheEnvironmentV1({SEMIO_WORKSPACE_ROOT:root},{root,probeMilliseconds:5000,retries:1})).toEqual(routed);expect(performance.now()-again).toBeLessThan(400);
 }finally{await hanging.stop(true);}
});

test("synchronous resolution never opens a stalled store or delays process exit when the local copy is older than the refresh TTL",()=>{
 if(!posix)return;
 const store=directory("stalled-store"),root=directory("sync-client");machine(root,store);
 mkdirSync(join(store,"v1","fleet"),{recursive:true});
 expect(spawnSync("mkfifo",[fleetCacheFactsPathV1(store)]).status).toBe(0);
 const now=Date.now();
 writeFileSync(fleetCacheLocalPathV1(root),JSON.stringify({version:1,fetchedAtMs:now-120000,checkedAtMs:now-120000,selectedEndpoint:"http://10.0.0.5:6090",facts:sample}));
 const modulePath=fileURLToPath(new URL("../🟦️.ts",import.meta.url));
 const started=performance.now();
 const child=spawnSync(process.execPath,["-e",`import {resolveRemoteCacheV1} from ${JSON.stringify(modulePath)}; const r=resolveRemoteCacheV1({SEMIO_WORKSPACE_ROOT:${JSON.stringify(root)}},{root:${JSON.stringify(root)}}); if(r?.contract.endpoint!=="http://10.0.0.5:6090")process.exit(2);`],{timeout:3000,encoding:"utf8"});
 expect(child.status,child.stderr).toBe(0);
 expect(performance.now()-started).toBeLessThan(2500);
});
