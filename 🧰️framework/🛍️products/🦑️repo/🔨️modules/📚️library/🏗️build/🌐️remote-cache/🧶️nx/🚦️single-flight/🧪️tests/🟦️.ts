import {afterAll,expect,test} from "bun:test";
import {createHash} from "node:crypto";
import {existsSync,mkdirSync,mkdtempSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import Ajv from "ajv";
import {startRemoteCacheServerV1} from "../../../🖥️server/🟦️.ts";
import {announceNxTaskFinishV1,NX_FLIGHT_POLICY_V1,nxFlightTextV1,nxFlightTimingV1,routeNxThroughFlightV1,startNxFlightV1,type NxFlightV1} from "../🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};

const scratch:string[]=[],stops:Array<()=>Promise<void>>=[],TOKEN="write-token",SLOT="a".repeat(64);
const directory=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`nx-flight-${label}-`));scratch.push(path);return path;};
afterAll(async()=>{for(const stop of stops)await stop().catch(()=>{});for(const path of scratch)rmSync(path,{recursive:true,force:true});});

async function serve(tokens:{read?:string;write?:string}={write:TOKEN}){
 const server=await startRemoteCacheServerV1({directory:directory("store"),host:"127.0.0.1",port:0,maximumBytes:2**27,maximumEntryBytes:2**25,maximumAgeDays:30,defaultInstance:"main",tokens});
 stops.push(()=>server.stop());return server;
}
async function front(upstream:string,extra:Partial<Parameters<typeof startNxFlightV1>[0]>={}):Promise<NxFlightV1>{
 const flight=await startNxFlightV1({upstream,instance:"main",token:TOKEN,timing:{awaitSliceMilliseconds:300,waitBudgetMilliseconds:5000,claimTtlMilliseconds:1000,heartbeatMilliseconds:300,claimTimeoutMilliseconds:1000,upstreamTimeoutMilliseconds:5000},...extra});
 stops.push(()=>flight.stop());return flight;
}
const key=(label:string)=>createHash("sha256").update(label).digest("hex").slice(0,40);
const authorized=(flight:NxFlightV1)=>({authorization:`Bearer ${flight.token}`});
const get=(flight:NxFlightV1,id:string,signal?:AbortSignal)=>fetch(`${flight.url}/v1/cache/${id}`,{headers:authorized(flight),...(signal?{signal}:{})});
const put=(flight:NxFlightV1,id:string,body:string)=>fetch(`${flight.url}/v1/cache/${id}`,{method:"PUT",headers:{...authorized(flight),"content-type":"application/octet-stream"},body});
const sleep=(milliseconds:number)=>new Promise(resolve=>setTimeout(resolve,milliseconds));

test("the policy and the finish message are closed by the schema (Ajv as the independent validator)",()=>{
 const ajv=new Ajv({strict:false}),validate=ajv.compile({$ref:"#/definitions/FinishRequest",definitions:schema.definitions});
 expect(validate({hash:"abc123",project:"p",target:"build",outcome:"succeeded",slot:SLOT})).toBe(true);
 expect(validate({hash:"ABC",project:"p",target:"build",outcome:"succeeded"})).toBe(false);
 expect(validate({hash:"abc",project:"p",target:"build",outcome:"done"})).toBe(false);
 expect(validate({hash:"abc",project:"p",target:"build",outcome:"failed",extra:1})).toBe(false);
 expect(NX_FLIGHT_POLICY_V1.claimTtlMilliseconds).toBeGreaterThan(NX_FLIGHT_POLICY_V1.heartbeatMilliseconds*2);
 expect(nxFlightTimingV1({SEMIO_NX_FLIGHT_WAIT_SECONDS:"7"}).waitBudgetMilliseconds).toBe(7000);
 expect(nxFlightTimingV1({SEMIO_NX_FLIGHT_WAIT_SECONDS:"-1"}).waitBudgetMilliseconds).toBe(NX_FLIGHT_POLICY_V1.waitBudgetMilliseconds);
 for(const language of["en","de"] as const)expect(nxFlightTextV1(language,{kind:"waiting",key:"k1",holder:"m2"})).toContain("m2");
});

test("a hit and a miss pass through, a front refuses a caller without its per-run token",async()=>{
 const server=await serve(),flight=await front(server.url),id=key("pass");
 expect((await fetch(`${flight.url}/v1/cache/${id}`)).status).toBe(401);
 expect((await fetch(`${flight.url}/v1/cache/${id}`,{headers:{authorization:`Bearer ${TOKEN}`}})).status).toBe(401);
 const miss=await get(flight,id);expect(miss.status).toBe(404);await miss.text();
 expect((await put(flight,id,"payload")).status).toBe(200);
 const hit=await get(flight,id);expect(hit.status).toBe(200);expect(await hit.text()).toBe("payload");
 expect(flight.counters()).toMatchObject({hits:1,misses:1,claimed:1,puts:1,released:1});
 expect(server.claims.summary().active).toBe(0);
});

test("two builds ask for one missing task: one computes, the other receives its upload and computes nothing",async()=>{
 const server=await serve(),one=await front(server.url,{owner:"machine-one:1:aa"}),two=await front(server.url,{owner:"machine-two:2:bb"}),id=key("race");
 const first=await get(one,id);expect(first.status).toBe(404);await first.text();
 const waiting=get(two,id);await sleep(500);
 expect(server.claims.summary()).toMatchObject({active:1});
 const stored=put(one,id,"computed once");
 const answer=await waiting;expect(answer.status).toBe(200);expect(await answer.text()).toBe("computed once");
 expect((await stored).status).toBe(200);
 expect(two.counters()).toMatchObject({waited:1,served:1,claimed:0,duplicated:0});
 expect(one.counters()).toMatchObject({claimed:1,puts:1});
 expect(server.claims.summary().active).toBe(0);
});

test("many concurrent askers of one task: exactly one is told to compute", async()=>{
 const server=await serve(),fronts=await Promise.all(Array.from({length:6},(_,index)=>front(server.url,{owner:`m${index}:1:cc`}))),id=key("many");
 const answers=await Promise.all(fronts.map((flight,index)=>(async()=>{await sleep(index*20);const response=await get(flight,id);if(response.status===404){await response.text();await sleep(400);await put(flight,id,"the one result");return "computed";}return await response.text();})()));
 expect(answers.filter(row=>row==="computed")).toHaveLength(1);
 expect(answers.filter(row=>row==="the one result")).toHaveLength(5);
});

test("a task that fails releases its claim at once through the executor's finish message and the waiter takes over",async()=>{
 const server=await serve(),one=await front(server.url,{owner:"m-one:1:dd"}),two=await front(server.url,{owner:"m-two:2:ee"}),id=key("failed");
 await (await get(one,id)).text();
 const waiting=get(two,id);await sleep(400);
 const begin=Date.now();
 await announceNxTaskFinishV1(one.environment(),{hash:id,project:"p",target:"build",succeeded:false});
 const answer=await waiting;expect(answer.status).toBe(404);await answer.text();
 expect(Date.now()-begin).toBeLessThan(2000);
 expect(two.counters()).toMatchObject({waited:1,claimed:1,taken:1,served:0});
});

test("a holder that dies without releasing loses its lease and the waiter takes over after the lease lapsed",async()=>{
 const server=await serve(),id=key("lapse");
 const dead=await front(server.url,{owner:"m-dead:1:ff",timing:{heartbeatMilliseconds:60000,claimTtlMilliseconds:1000,awaitSliceMilliseconds:300,waitBudgetMilliseconds:8000,claimTimeoutMilliseconds:1000,upstreamTimeoutMilliseconds:5000}});
 await (await get(dead,id)).text();
 const live=await front(server.url,{owner:"m-live:2:gg"}),begin=Date.now(),answer=await get(live,id);
 expect(answer.status).toBe(404);await answer.text();
 expect(Date.now()-begin).toBeGreaterThan(600);expect(Date.now()-begin).toBeLessThan(4500);
 expect(live.counters()).toMatchObject({waited:1,claimed:1,taken:1});
});

test("a holder that stays alive but never finishes does not block longer than the wait budget",async()=>{
 const server=await serve(),one=await front(server.url,{owner:"m-slow:1:hh"}),events:string[]=[],two=await front(server.url,{owner:"m-wait:2:ii",timing:{waitBudgetMilliseconds:900,awaitSliceMilliseconds:300,claimTtlMilliseconds:1000,heartbeatMilliseconds:300,claimTimeoutMilliseconds:1000,upstreamTimeoutMilliseconds:5000},report:event=>events.push(event.kind)}),id=key("budget");
 await (await get(one,id)).text();
 const begin=Date.now(),answer=await get(two,id);expect(answer.status).toBe(404);await answer.text();
 expect(Date.now()-begin).toBeLessThan(3000);
 expect(two.counters()).toMatchObject({waited:1,duplicated:1,claimed:0});expect(events).toEqual(["waiting","duplicated"]);
});

test("an unreachable cache server never hangs a miss",async()=>{
 const dead=await serve();const origin=dead.url;await dead.stop();
 const flight=await front(origin,{timing:{claimTimeoutMilliseconds:300,upstreamTimeoutMilliseconds:500,awaitSliceMilliseconds:200,waitBudgetMilliseconds:1000,claimTtlMilliseconds:1000,heartbeatMilliseconds:300}}),begin=Date.now();
 const answer=await get(flight,key("down"));expect(answer.status).toBe(404);await answer.text();
 expect(Date.now()-begin).toBeLessThan(2500);
 expect((await put(flight,key("down"),"x")).status).toBe(502);
});

test("a cache that refuses claims (read-only token, old server) is a plain cache and says so once",async()=>{
 const server=await serve({read:"r",write:TOKEN}),events:string[]=[],flight=await startNxFlightV1({upstream:server.url,instance:"main",token:"r",report:event=>events.push(event.kind),timing:{claimTimeoutMilliseconds:500}});stops.push(()=>flight.stop());
 for(const label of["ro1","ro2"]){const answer=await get(flight,key(label));expect(answer.status).toBe(404);await answer.text();}
 expect(events).toEqual(["unavailable"]);expect(flight.counters()).toMatchObject({unclaimed:2,claimed:0});
});

test("the upload carries the slot the executor announced and nothing else about the task leaks upstream",async()=>{
 const seen:Array<Record<string,string>>=[];
 const upstream=await startNxFlightV1({upstream:"http://127.0.0.1:1",instance:"main",token:"t",fetch:async(input,init)=>{
  const url=String(input);
  if(url.includes("/claim/"))return Response.json({state:"granted",owner:"x",untilMilliseconds:1,ttlMilliseconds:1000});
  if(init?.method==="PUT"){seen.push(init.headers as Record<string,string>);await new Response(init.body as ReadableStream).arrayBuffer();return new Response("Stored",{status:200});}
  return new Response("no",{status:404});
 }});stops.push(()=>upstream.stop());
 const id=key("slot");await (await get(upstream,id)).text();
 await announceNxTaskFinishV1(upstream.environment(),{hash:id,project:"p",target:"build",slot:SLOT,succeeded:true});
 expect((await put(upstream,id,"archive")).status).toBe(200);
 expect(seen[0]!["x-semio-slot"]).toBe(SLOT);expect(seen[0]!.authorization).toBe("Bearer t");
 const other=key("slot2");await (await get(upstream,other)).text();
 await announceNxTaskFinishV1(upstream.environment(),{hash:other,project:"p",target:"build",slot:SLOT,succeeded:false});
 await put(upstream,other,"archive");expect(seen[1]!["x-semio-slot"]).toBeUndefined();
});

test("an upload of a guarded task is dropped before it reaches the cache",async()=>{
 const server=await serve(),flight=await front(server.url),id=key("guarded");
 await (await get(flight,id)).text();
 await announceNxTaskFinishV1(flight.environment(),{hash:id,project:"p",target:"wasm",guarded:true,succeeded:true});
 expect((await put(flight,id,"receipt")).status).toBe(200);
 const again=await get(flight,id);expect(again.status).toBe(404);await again.text();
 expect(flight.counters()).toMatchObject({guarded:1,puts:0});
});

test("routing re-points the Nx environment at a front and leaves it alone when the switch is off or no remote is set",async()=>{
 const server=await serve(),environment={NX_SELF_HOSTED_REMOTE_CACHE_SERVER:`${server.url}/main`,NX_SELF_HOSTED_REMOTE_CACHE_ACCESS_TOKEN:TOKEN};
 expect(await routeNxThroughFlightV1({})).toBeUndefined();
 expect(await routeNxThroughFlightV1({...environment,SEMIO_NX_FLIGHT:"off"})).toBeUndefined();
 expect(await routeNxThroughFlightV1({NX_SELF_HOSTED_REMOTE_CACHE_SERVER:"not a url"})).toBeUndefined();
 expect(await routeNxThroughFlightV1({NX_SELF_HOSTED_REMOTE_CACHE_SERVER:`${server.url}/a/b`})).toBeUndefined();
 const routed=await routeNxThroughFlightV1(environment);stops.push(()=>routed!.stop());
 const env=routed!.environment();
 expect(env.NX_SELF_HOSTED_REMOTE_CACHE_SERVER).toBe(routed!.url);expect(env.NX_SELF_HOSTED_REMOTE_CACHE_ACCESS_TOKEN).not.toBe(TOKEN);expect(env.SEMIO_NX_FLIGHT_URL).toBe(routed!.origin);
 expect(JSON.stringify(env)).not.toContain(TOKEN);
});

test("a front that is stopped releases every claim it holds so nobody waits for a dead build",async()=>{
 const server=await serve(),one=await front(server.url,{owner:"m-stop:1:jj"}),id=key("stop");
 await (await get(one,id)).text();expect(server.claims.summary().active).toBe(1);
 await one.stop();expect(server.claims.summary().active).toBe(0);
});

const nativeModule=["/Users/ueli/.bun/install/cache/@nx/nx-darwin-arm64@23.2.0@@@1/nx.darwin-arm64.node"].find(path=>existsSync(path)),driver=join(import.meta.dir,"🧰️nx-native-client/🟨️.mjs");
const nativeClient=(mode:"retrieve"|"store",flight:NxFlightV1,hash:string,cache:string)=>{
 const child=Bun.spawn([process.execPath,driver,nativeModule!,mode,hash,cache],{env:{PATH:process.env.PATH??"",...flight.environment()},stdout:"pipe",stderr:"pipe"});
 return (async()=>{const [out,err]=await Promise.all([new Response(child.stdout).text(),new Response(child.stderr).text()]);await child.exited;const line=out.trim().split("\n").at(-1);if(!line)throw Error("native client printed nothing: "+err);return JSON.parse(line) as {ok:boolean;found?:boolean;stored?:boolean;milliseconds?:number;error?:string};})();
};
test.skipIf(nativeModule===undefined||process.platform!=="darwin"||process.arch!=="arm64")("Nx's own native HTTP cache client (the third-party oracle) misses, waits through a held task and receives the holder's archive through the front",async()=>{
 const server=await serve(),one=await front(server.url,{owner:"m-native-one:1:kk"}),two=await front(server.url,{owner:"m-native-two:2:ll"}),id="48"+String(Date.now()).slice(-12),cache=directory("native-cache"),outputs=join(cache,id);
 mkdirSync(join(outputs,"outputs"),{recursive:true});writeFileSync(join(cache,`${id}.commit`),"0");writeFileSync(join(outputs,"terminalOutput"),"built\n");writeFileSync(join(outputs,"outputs","artifact.txt"),"x".repeat(5000));
 const holder=await nativeClient("retrieve",one,id,directory("native-miss"));
 expect(holder).toMatchObject({ok:true,found:false});
 const began=performance.now(),pending=nativeClient("retrieve",two,id,directory("native-wait"));await sleep(900);
 expect(server.claims.summary().active).toBe(1);
 const stored=await nativeClient("store",one,id,cache);expect(stored).toMatchObject({ok:true,stored:true});
 const received=await pending;expect(received).toMatchObject({ok:true,found:true});expect(performance.now()-began).toBeGreaterThanOrEqual(900);expect(received.milliseconds).toBeGreaterThan(400);
 expect(two.counters()).toMatchObject({served:1,claimed:0});expect(one.counters().puts).toBe(1);
});

test("a hanging cache costs one request timeout in total and every later task goes local at once until the server answers its health check again",async()=>{
 let mode:"hang"|"ok"="hang",cacheRequests=0,health=0;
 const upstream=Bun.serve({port:0,fetch:request=>{
  const path=new URL(request.url).pathname;
  if(path==="/healthz"){health++;return mode==="ok"?new Response('{"status":"ok"}'):new Promise<Response>(()=>{});}
  cacheRequests++;return mode==="ok"?new Response("Not found",{status:404}):new Promise<Response>(()=>{});
 }});
 stops.push(async()=>{await upstream.stop(true);});
 const flight=await front(`http://127.0.0.1:${upstream.port}`,{timing:{requestTimeoutMilliseconds:300,probeTimeoutMilliseconds:200,breakerMilliseconds:600,claimTimeoutMilliseconds:100}});
 const started=performance.now(),first=await get(flight,key("down-0"));
 expect(first.status).toBe(404);expect(performance.now()-started).toBeGreaterThanOrEqual(250);
 const later=performance.now();
 for(let task=1;task<=8;task++)expect((await get(flight,key("down-"+task))).status).toBe(404);
 expect(performance.now()-later,"eight more tasks pay nothing").toBeLessThan(250);
 expect(cacheRequests,"only the first request reached the hanging server").toBe(1);
 expect((await put(flight,key("down-put"),"x")).status,"an upload does not wait either").toBe(502);
 await sleep(700);mode="ok";
 const recovered=await get(flight,key("up-0"));
 expect(recovered.status).toBe(404);expect(health).toBeGreaterThanOrEqual(1);expect(cacheRequests,"after the health check answered, requests reach the server again").toBeGreaterThan(1);
 const fresh=await front(`http://127.0.0.1:${upstream.port}`,{timing:{requestTimeoutMilliseconds:300,probeTimeoutMilliseconds:200,breakerMilliseconds:600,claimTimeoutMilliseconds:100}});
 mode="hang";
 const burst=performance.now(),answers=await Promise.all(Array.from({length:8},(_,task)=>get(fresh,key("burst-"+task))));
 expect(answers.every(answer=>answer.status===404)).toBe(true);expect(performance.now()-burst,"eight simultaneous tasks pay one timeout together").toBeLessThan(900);
});
