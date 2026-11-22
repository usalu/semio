import {afterAll,expect,test} from "bun:test";
import {createHash} from "node:crypto";
import {mkdirSync,mkdtempSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {acquireRemoteCacheSigningProxyV1,startRemoteCacheSigningProxyV1} from "../🟦️.ts";
import {refreshFleetCacheFactsV1,remoteCacheFetchV1,resolveRemoteCacheV1} from "../../🛰️fleet/🟦️.ts";
import {publishFleetCacheFactsV1} from "../../🛰️fleet/📣️publication/🟦️.ts";
import {startRemoteCacheServerV1,type RemoteCacheServerV1} from "../../🖥️server/🟦️.ts";
import {bazelRemoteCacheV1,startBazelRemoteCacheProxyV1} from "../../../🔧️tools/🏗️bazel/🌐️remote-cache/🟦️.ts";

const scratch:string[]=[],stoppers:(()=>Promise<unknown>)[]=[];
const directory=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`signing-proxy-${label}-`));scratch.push(path);return path;};
afterAll(async()=>{for(const stop of stoppers)await stop().catch(()=>{});for(const path of scratch)rmSync(path,{recursive:true,force:true});});

const machine=(root:string,storeRoot:string,origin:string)=>{mkdirSync(join(root,".cache","f"),{recursive:true});writeFileSync(join(root,".cache","f","machine.json"),JSON.stringify({version:1,originId:origin,groupId:"g1",storeRoot,key:"k"}));mkdirSync(join(storeRoot,"v1","origins",origin),{recursive:true});};
const sha=(bytes:Uint8Array|string)=>createHash("sha256").update(bytes).digest("hex");

async function fleet(){
 const store=directory("store"),cache=directory("cache"),root=directory("client"),bare=directory("bare");machine(root,store,"g1-m01");
 const server:RemoteCacheServerV1=await startRemoteCacheServerV1({directory:cache,host:"127.0.0.1",port:0,maximumBytes:2**26,maximumEntryBytes:2**24,maximumAgeDays:30,defaultInstance:"semio",tokens:{},validation:"off",fleet:{storeRoot:store,group:"g1",networks:["10.77.0.0/16"],registryTtlMilliseconds:200}});
 stoppers.push(()=>server.stop());
 await publishFleetCacheFactsV1({admin:{role:"admin",originId:"g1-server"},storeRoot:store,directory:cache,group:"g1",instance:"semio",endpoints:[server.url],port:6090,host:"h1",now:Date.now()});
 await refreshFleetCacheFactsV1({root,force:true,probeMilliseconds:300});
 const environment={SEMIO_WORKSPACE_ROOT:root},resolved=resolveRemoteCacheV1(environment,{root})!;
 return {server,root,bare,environment,resolved,signed:remoteCacheFetchV1({...environment,...resolved.environment},{root})};
}

test("Bazel-style unsigned PUT and GET on /cas and /ac reach a server that requires signatures, through the proxy and only through it",async()=>{
 const {server,signed}=await fleet(),proxy=await startRemoteCacheSigningProxyV1({upstream:server.url,fetch:signed,timeoutMilliseconds:5000});stoppers.push(()=>proxy.stop());
 const blob=new TextEncoder().encode("bazel output blob "+"x".repeat(5000)),digest=sha(blob),action=sha("action");
 expect((await fetch(`${server.url}/semio/cas/${digest}`,{method:"PUT",body:blob})).status,"the server refuses an unsigned writer").toBe(401);
 expect((await fetch(`${proxy.url}/semio/cas/${digest}`,{method:"PUT",body:blob})).status).toBe(200);
 const read=await fetch(`${proxy.url}/semio/cas/${digest}`);expect(read.status).toBe(200);expect(new Uint8Array(await read.arrayBuffer())).toEqual(blob);
 expect((await fetch(`${proxy.url}/semio/cas/${digest}`,{method:"HEAD"})).status).toBe(200);
 expect((await fetch(`${proxy.url}/semio/cas/${sha("none")}`)).status).toBe(404);
 const result=new TextEncoder().encode("result-"+digest);
 expect((await fetch(`${proxy.url}/semio/ac/${action}`,{method:"PUT",body:result})).status).toBe(200);
 expect(new Uint8Array(await (await fetch(`${proxy.url}/semio/ac/${action}`)).arrayBuffer())).toEqual(result);
 expect((await fetch(`${server.url}/semio/cas/${digest}`)).status,"the server still refuses unsigned readers").toBe(401);
 expect(proxy.stats()).toMatchObject({requests:6,answeredLocally:0,refused:0});expect(proxy.stats().forwarded).toBe(6);
});

test("the proxy is no open relay: only the cache routes, only GET, HEAD and PUT, only loopback hosts",async()=>{
 const {server,signed}=await fleet(),proxy=await startRemoteCacheSigningProxyV1({upstream:server.url,fetch:signed,timeoutMilliseconds:5000});stoppers.push(()=>proxy.stop());
 expect((await fetch(`${proxy.url}/status`)).status).toBe(404);
 expect((await fetch(`${proxy.url}/semio/v1/cache/12345`)).status).toBe(404);
 expect((await fetch(`${proxy.url}/semio/cas/${sha("a")}`,{method:"POST",body:"x"})).status).toBe(405);
 expect((await fetch(`${proxy.url}/semio/cas/${sha("a")}`,{method:"DELETE"})).status).toBe(405);
 expect((await fetch(`${proxy.url}/semio/cas/not-a-digest`)).status).toBe(404);
 const rebinding=Bun.spawn(["curl","-s","-o","/dev/null","-w","%{http_code}","-H","Host: attacker.example",`${proxy.url}/semio/cas/${sha("a")}`],{stdout:"pipe"});
 expect(await new Response(rebinding.stdout).text()).toBe("404");await rebinding.exited;
 expect((await fetch(`${proxy.url}/healthz`)).status).toBe(200);
 expect(proxy.stats()).toMatchObject({forwarded:0,refused:6});
 const small=await startRemoteCacheSigningProxyV1({upstream:server.url,fetch:signed,maximumBodyBytes:16});stoppers.push(()=>small.stop());
 expect((await fetch(`${small.url}/semio/cas/${sha("a")}`,{method:"PUT",body:"x".repeat(64)})).status).toBe(413);
});

test("a fleet cache that is down, silent or refusing costs the client one bounded miss, then nothing, and is tried again after the window",async()=>{
 const refused=await startRemoteCacheSigningProxyV1({upstream:"http://127.0.0.1:1",fetch,timeoutMilliseconds:500,downMilliseconds:300});stoppers.push(()=>refused.stop());
 const at=Date.now();expect((await fetch(`${refused.url}/semio/ac/${sha("a")}`)).status).toBe(404);expect(Date.now()-at).toBeLessThan(2000);
 let silent=true;const hang=Bun.serve({port:0,hostname:"127.0.0.1",fetch:async request=>{if(silent)await Bun.sleep(5000);return new Response(request.method==="GET"?"late":"",{status:request.method==="GET"?200:404});}});stoppers.push(()=>hang.stop(true));
 const proxy=await startRemoteCacheSigningProxyV1({upstream:`http://127.0.0.1:${hang.port}`,fetch,timeoutMilliseconds:300,downMilliseconds:400});stoppers.push(()=>proxy.stop());
 const first=Date.now(),miss=await fetch(`${proxy.url}/semio/cas/${sha("a")}`);expect(miss.status).toBe(404);expect(Date.now()-first,"bounded by the timeout").toBeLessThan(1500);
 const second=Date.now(),put=await fetch(`${proxy.url}/semio/cas/${sha("a")}`,{method:"PUT",body:"x"});expect(put.status,"unavailable, which clients do not retry").toBe(507);expect(Date.now()-second,"answered without asking the upstream").toBeLessThan(150);
 const head=Date.now();expect((await fetch(`${proxy.url}/semio/ac/${sha("b")}`,{method:"HEAD"})).status).toBe(404);expect(Date.now()-head).toBeLessThan(150);
 expect(proxy.stats()).toMatchObject({forwarded:0,answeredLocally:3});
 silent=false;await Bun.sleep(450);
 expect(await (await fetch(`${proxy.url}/semio/cas/${sha("a")}`)).text(),"the window is over: the upstream is asked again").toBe("late");
 const denying=Bun.serve({port:0,hostname:"127.0.0.1",fetch:()=>new Response("no",{status:401})});stoppers.push(()=>denying.stop(true));
 const denied=await startRemoteCacheSigningProxyV1({upstream:`http://127.0.0.1:${denying.port}`,fetch});stoppers.push(()=>denied.stop());
 expect((await fetch(`${denied.url}/semio/cas/${sha("a")}`)).status,"a refused signature is a miss for the client, never an error").toBe(404);
});

test("a proxy is shared by the holders of one upstream while it is healthy and ends with the last holder; one that cannot start answers nothing",async()=>{
 const options={upstream:"http://127.0.0.1:1",fetch};
 const one=(await acquireRemoteCacheSigningProxyV1(options))!,two=(await acquireRemoteCacheSigningProxyV1(options))!;
 expect(two.url).toBe(one.url);expect((await fetch(one.url+"/healthz")).status).toBe(200);
 await one.release();await one.release();expect((await fetch(two.url+"/healthz")).status,"still held by the second holder").toBe(200);
 await two.release();await expect(fetch(two.url+"/healthz")).rejects.toThrow();
 const again=(await acquireRemoteCacheSigningProxyV1(options))!;expect((await fetch(again.url+"/healthz")).status).toBe(200);await again.release();
 const taken=Bun.serve({port:0,hostname:"127.0.0.1",fetch:()=>new Response("")});stoppers.push(()=>taken.stop(true));
 expect(await acquireRemoteCacheSigningProxyV1({upstream:"http://127.0.0.1:2",fetch,port:taken.port})).toBeUndefined();
});

test("the Bazel backend starts the proxy for a machine with a fleet identity, renders Bazel without credentials through it and builds locally when there is none",async()=>{
 const {server,root,bare,environment,resolved}=await fleet(),env={...environment};
 const started=await startBazelRemoteCacheProxyV1(env);
 try{
  expect(started.url).toMatch(/^http:\/\/127\.0\.0\.1:[0-9]+$/u);
  const rendering=bazelRemoteCacheV1(resolved.contract,resolved.environment,{helperDirectory:join(root,"tmp"),proxy:started.url!});
  expect(rendering.flags).toContain(`--remote_cache=${started.url}/semio`);expect(rendering.flags.some(flag=>flag.startsWith("--credential_helper"))).toBe(false);
  expect(rendering.helper).toBeUndefined();expect(rendering.environment).toEqual({});expect(rendering.flags).toContain("--remote_retries=3");expect(rendering.rendering.applied).toEqual([...rendering.flags]);
  const blob="through bazel flags",digest=sha(blob);
  expect((await fetch(`${started.url}/semio/cas/${digest}`,{method:"PUT",body:blob})).status).toBe(200);
  expect(await (await fetch(`${server.url}/semio/cas/${digest}`,{headers:{}})).status,"the server itself still wants signatures").toBe(401);
  const second=await startBazelRemoteCacheProxyV1(env);expect(second.url,"a healthy proxy is reused").toBe(started.url);await second.stop();
  expect((await fetch(`${started.url}/healthz`)).status).toBe(200);
 }finally{await started.stop();}
 await expect(fetch(`${started.url}/healthz`)).rejects.toThrow();
 expect((await startBazelRemoteCacheProxyV1(env,{skip:true})).url).toBeUndefined();
 expect((await startBazelRemoteCacheProxyV1({SEMIO_WORKSPACE_ROOT:bare})).url,"no identity and no endpoint: no proxy").toBeUndefined();
 const local=bazelRemoteCacheV1(resolved.contract,resolved.environment,{helperDirectory:join(root,"tmp")});
 expect(local.flags).toEqual([]);expect(local.rendering.applied).toEqual([]);
});
