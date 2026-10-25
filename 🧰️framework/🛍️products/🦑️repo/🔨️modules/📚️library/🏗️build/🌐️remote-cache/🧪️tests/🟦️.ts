import {afterAll,expect,test} from "bun:test";
import {testPythonV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import Ajv from "ajv";
import {createHash} from "node:crypto";
import {spawn,spawnSync} from "node:child_process";
import {cpSync,existsSync,mkdirSync,mkdtempSync,readFileSync,rmSync,symlinkSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import schema from "../../🏃️execution/🧬️schema/🌐️remote-cache.json" with {type:"json"};
import policy from "../../🔣️policy.json" with {type:"json"};
import fixture from "./🧫️fixtures/🔣️.json" with {type:"json"};
import wire from "./🧫️fixtures/📡️wire.json" with {type:"json"};
import bounds from "./🧫️fixtures/💾️store.json" with {type:"json"};
import {readRemoteCachePolicyV1,resolveRemoteCacheContractV1,requireRemoteCacheShapeV1,type RemoteCacheContractV1} from "../🟦️.ts";
import {bazelRemoteCacheV1} from "../../🔧️tools/🏗️bazel/🌐️remote-cache/🟦️.ts";
import {nxRemoteCacheV1} from "../../../⚡️caching/🚀️bootstrap/🛠️tools/🌐️remote-cache/🟦️.ts";
import {RemoteCacheStoreV1,type RemoteCacheStoreEventV1} from "../💾️store/🟦️.ts";
import {startRemoteCacheServerV1,parseRemoteCachePathV1,type RemoteCacheServerV1} from "../🖥️server/🟦️.ts";
import {collectRemoteCacheV1,parseRemoteCacheArgumentsV1,remoteCacheLanguagesV1,remoteCacheStatusV1,remoteCacheTextV1,runRemoteCacheCommandV1,serveRemoteCacheV1,type RemoteCacheCommandContextV1} from "../⌨️command/🟦️.ts";
import texts from "../📖️texts/🔣️.json" with {type:"json"};
import {bazelRemoteCacheToolV1} from "../../🔧️tools/🏗️bazel/🌐️remote-cache/🟦️.ts";
import {nxRemoteCacheToolV1} from "../../../⚡️caching/🚀️bootstrap/🛠️tools/🌐️remote-cache/🟦️.ts";

const ajv=new Ajv({strict:true,allowUnionTypes:true}),validator=(definition:string)=>ajv.compile({$schema:schema.$schema,definitions:schema.definitions,$ref:`#/definitions/${definition}`}),python=testPythonV1(),scratch:string[]=[];
const directory=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`rcache-${label}-`));scratch.push(path);return path;};
const digest=(value:string)=>createHash("sha256").update(value).digest("hex");

type Step={name:string;request:{method:string;path:string;headers?:Record<string,string>;body?:string};response:{status:number;body?:string;headers?:Record<string,string>}};
type Scenario={name:string;server:{tokens:{read?:string;write?:string};maximumBytes?:number;maximumEntryBytes?:number;readOnly?:boolean;validation?:"off"|"structure"|"complete"};steps:Step[]};
type Answer={status:number;body:string;headers:Record<string,string>};

/** 🏃️ Runs a client process without blocking the event loop that serves it. */
function client(command:string,args:string[],options:{cwd?:string;env?:Record<string,string>;input?:string;timeout:number}):Promise<{status:number|null;stdout:string;stderr:string}>{
 return new Promise((resolve,reject)=>{
  const child=spawn(command,args,{cwd:options.cwd,env:options.env,stdio:["pipe","pipe","pipe"]}),out:Buffer[]=[],err:Buffer[]=[],timer=setTimeout(()=>child.kill("SIGKILL"),options.timeout);
  child.stdout.on("data",chunk=>out.push(chunk));child.stderr.on("data",chunk=>err.push(chunk));child.on("error",reject);
  child.on("close",status=>{clearTimeout(timer);resolve({status,stdout:Buffer.concat(out).toString(),stderr:Buffer.concat(err).toString()});});
  child.stdin.end(options.input??"");
 });
}

async function serve(options:Partial<Parameters<typeof startRemoteCacheServerV1>[0]>={}):Promise<RemoteCacheServerV1>{
 return startRemoteCacheServerV1({directory:directory("store"),host:"127.0.0.1",port:0,maximumBytes:2**26,maximumEntryBytes:2**24,maximumAgeDays:30,defaultInstance:"default",tokens:{},validation:"off",...options});
}

test("the policy, the fixtures and the contract schema agree against an independent JSON-schema implementation",()=>{
 expect(validator("Policy")(policy.remoteCache),JSON.stringify(validator("Policy").errors)).toBe(true);
 expect(readRemoteCachePolicyV1()).toEqual(policy.remoteCache);
 expect(()=>readRemoteCachePolicyV1({...policy.remoteCache,defaults:{...policy.remoteCache.defaults,access:"write"}})).toThrow("Closed remote cache policy required");
 expect(policy.remoteCache.defaults.endpoint,"the committed default keeps the remote cache off").toBeNull();
 expect(policy.remoteCache.defaults.access).toBe("read-only");
 const names=Object.values(policy.remoteCache.environment);expect(new Set(names).size).toBe(names.length);
 for(const row of fixture.contract.accepted){
  const contract=resolveRemoteCacheContractV1(row.environment);
  expect(contract,row.name).toEqual((row.contract as RemoteCacheContractV1|null)??undefined);
  if(contract)expect(validator("Contract")(contract),row.name+JSON.stringify(validator("Contract").errors)).toBe(true);
 }
 for(const row of fixture.contract.refused)expect(()=>resolveRemoteCacheContractV1(row.environment),row.name).toThrow(fixture.contract.message);
});

test("every schema pattern accepts and refuses the fixture strings like Python's re module",()=>{
 const rows=Object.entries(fixture.patterns).map(([definition,row])=>({definition,pattern:(schema.definitions as unknown as Record<string,{pattern:string}>)[definition]!.pattern,accepted:row.accepted,refused:row.refused}));
 for(const row of rows){
  const validate=validator(row.definition);
  for(const value of row.accepted)expect(validate(value),`${row.definition} accepts ${value.slice(0,20)}`).toBe(true);
  for(const value of row.refused)expect(validate(value),`${row.definition} refuses ${value.slice(0,20)}`).toBe(false);
  for(const value of row.accepted)expect(()=>requireRemoteCacheShapeV1(row.definition as keyof typeof schema.definitions,value,"x"),row.definition).not.toThrow();
  for(const value of row.refused)expect(()=>requireRemoteCacheShapeV1(row.definition as keyof typeof schema.definitions,value,"x"),row.definition).toThrow();
 }
 if(!python)return;
 const result=Bun.spawnSync([python,"-I","-c","import json,re,sys\nrows=json.loads(sys.stdin.read())\nprint(json.dumps([[re.fullmatch(r['pattern'],v) is not None for v in r['accepted']+r['refused']] for r in rows]))"],{stdin:new TextEncoder().encode(JSON.stringify(rows)),stdout:"pipe",stderr:"pipe"});
 expect(result.exitCode,result.stderr.toString()).toBe(0);
 const expected=rows.map(row=>[...row.accepted.map(()=>true),...row.refused.map(()=>false)]),actual=JSON.parse(result.stdout.toString()) as boolean[][];
 rows.forEach((row,index)=>{if(row.definition==="Endpoint"||row.definition==="Token")expect(actual[index],row.definition).toEqual(expected[index]!);else expect(actual[index]!.slice(0,row.accepted.length),row.definition).toEqual(expected[index]!.slice(0,row.accepted.length));});
});

test("Bazel renders the neutral contract into exactly the fixture flags, environment and credential helper",()=>{
 for(const row of fixture.bazel){
  const contract=resolveRemoteCacheContractV1(row.environment)!,options={helperDirectory:row.directory,platform:row.platform as NodeJS.Platform};
  if("refused" in row){expect(()=>bazelRemoteCacheV1(contract,row.environment,options),row.name).toThrow(row.refused);continue;}
  const rendering=bazelRemoteCacheV1(contract,row.environment,options);
  expect(rendering.flags,row.name).toEqual(row.flags);expect(rendering.environment,row.name).toEqual(row.tokenEnvironment as Record<string,string>);expect(rendering.helper??null,row.name).toEqual(row.helper);
  expect(rendering.rendering.uploadEnforcedBy as string,row.name).toBe(row.uploadEnforcedBy);expect(rendering.rendering.applied,row.name).toEqual(row.flags);
  expect(JSON.stringify(rendering.flags),row.name+" never puts the token in a flag").not.toContain("tok-123");
  expect(rendering.flags.filter(flag=>flag.startsWith("--remote_header")),row.name).toEqual([]);
 }
});

test("the generated credential helper answers Bazel's get request with a valid bearer header and never contains the token",()=>{
 if(process.platform==="win32")return;
 const row=fixture.bazel.find(candidate=>candidate.helper&&candidate.platform==="linux")!,contract=resolveRemoteCacheContractV1(row.environment)!,folder=directory("helper"),rendering=bazelRemoteCacheV1(contract,row.environment,{helperDirectory:folder,platform:"linux"}),helper=rendering.helper!;
 writeFileSync(helper.path,helper.content,{mode:helper.mode});
 expect(helper.content).not.toContain("tok-123");
 const result=spawnSync(helper.path,["get"],{input:JSON.stringify({uri:"https://cache.example.test:8443/team-a/ac/x"}),env:{PATH:process.env.PATH,CI_CACHE_TOKEN:"tok-123"},encoding:"utf8"});
 expect(result.status).toBe(0);expect(JSON.parse(result.stdout)).toEqual({headers:{Authorization:["Bearer tok-123"]}});
 if(python){const parsed=Bun.spawnSync([python,"-I","-c","import json,sys\nprint(json.dumps(json.loads(sys.stdin.read())))"],{stdin:new TextEncoder().encode(result.stdout),stdout:"pipe"});expect(JSON.parse(parsed.stdout.toString())).toEqual({headers:{Authorization:["Bearer tok-123"]}});}
});

test("Nx renders the neutral contract into exactly the fixture environment and reports what it cannot honour",()=>{
 for(const row of fixture.nx){
  const rendering=nxRemoteCacheV1(resolveRemoteCacheContractV1(row.environment),row.environment);
  expect(rendering.environment,row.name).toEqual(Object.fromEntries(Object.entries(row.variables).map(([name,value])=>[name,value??undefined])));expect(rendering.rendering.applied,row.name).toEqual(row.applied);
  expect(rendering.rendering.unsupported as readonly string[],row.name).toEqual(row.unsupported);expect(rendering.rendering.uploadEnforcedBy as string,row.name).toBe(row.uploadEnforcedBy);expect(rendering.rendering.mechanism).toBe("nx-environment");
  expect(validator("Rendering")(rendering.rendering),row.name+JSON.stringify(validator("Rendering").errors)).toBe(true);
  expect(JSON.stringify(rendering.rendering),row.name).not.toContain("tok-123");
 }
});

test("the route parser maps both protocols and every namespace to one address and refuses everything else",()=>{
 const key=digest("hello");
 expect(parseRemoteCachePathV1(`/ac/${key}`,"default")).toEqual({namespace:"default",kind:"ac",key});
 expect(parseRemoteCachePathV1(`/team-a/cas/${key}`,"default")).toEqual({namespace:"team-a",kind:"cas",key});
 expect(parseRemoteCachePathV1("/v1/cache/123","default")).toEqual({namespace:"default",kind:"nx",key:"123"});
 expect(parseRemoteCachePathV1("/team-a/v1/cache/123","default")).toEqual({namespace:"team-a",kind:"nx",key:"123"});
 for(const path of ["/","/ac","/ac/","/cas/x","/v1/cache","/v1/cache/A","/v1/cache/a/b","/Team/ac/"+key,"/ac/"+key+"/x","/a/b/c/d","/healthz/ac/"+key,"/../ac/"+key,"/%2e%2e/ac/"+key])expect(parseRemoteCachePathV1(path,"default"),path).toBeUndefined();
});

async function typescriptClient(url:string,scenario:Scenario):Promise<Answer[]>{
 const rows=[];
 for(const {request} of scenario.steps){
  const response=await fetch(url+request.path,{method:request.method,...(request.headers?{headers:request.headers}:{}),...(request.body!==undefined?{body:request.body}:{})});
  rows.push({status:response.status,body:await response.text(),headers:Object.fromEntries(response.headers.entries())});
 }
 return rows;
}

async function pythonClient(url:string,scenario:Scenario):Promise<Answer[]>{
 const program="import http.client,json,sys,urllib.parse\nrequest=json.loads(sys.stdin.read())\nparts=urllib.parse.urlsplit(request['url'])\nout=[]\nfor step in request['steps']:\n  c=http.client.HTTPConnection(parts.hostname,parts.port,timeout=30)\n  body=step.get('body')\n  try:\n    c.request(step['method'],step['path'],body=None if body is None else body.encode(),headers=step.get('headers',{}))\n    r=c.getresponse()\n    out.append({'status':r.status,'body':r.read().decode(),'headers':{k.lower():v for k,v in r.getheaders()}})\n  except OSError as e:\n    out.append({'status':-1,'body':repr(e),'headers':{}})\n  c.close()\nprint(json.dumps(out))";
 const result=await client(python!,["-I","-c",program],{input:JSON.stringify({url,steps:scenario.steps.map(step=>step.request)}),timeout:60000});
 expect(result.status,result.stderr).toBe(0);return JSON.parse(result.stdout);
}

test("the wire fixture holds against the TypeScript client and against Python's http.client, both bit for bit",async()=>{
 expect(wire.digests.hello).toBe(digest("hello"));expect(wire.digests.world).toBe(digest("world"));expect(wire.digests.empty).toBe(digest(""));
 for(const scenario of wire.scenarios as unknown as Scenario[])for(const who of python?["typescript","python"]:["typescript"]){
  const server=await serve({tokens:scenario.server.tokens,...(scenario.server.maximumBytes?{maximumBytes:scenario.server.maximumBytes}:{}),...(scenario.server.maximumEntryBytes?{maximumEntryBytes:scenario.server.maximumEntryBytes}:{}),...(scenario.server.readOnly?{readOnly:true}:{}),...(scenario.server.validation?{validation:scenario.server.validation}:{})});
  try{
   const rows=who==="typescript"?await typescriptClient(server.url,scenario):await pythonClient(server.url,scenario);
   scenario.steps.forEach((step,index)=>{
    const row=rows[index]!,label=`${who}: ${scenario.name}: ${step.name}`;
    expect(row.status,label).toBe(step.response.status);
    if(step.response.body!==undefined)expect(row.body,label).toBe(step.response.body);
    for(const [name,value] of Object.entries(step.response.headers??{}))expect(row.headers[name],label+" "+name).toBe(value);
   });
  }finally{await server.stop();}
 }
},60000);

test("a restart keeps stored values, drops partial writes and a single store answers the whole status document",async()=>{
 const folder=directory("restart"),first=await serve({directory:folder}),key=digest("hello");
 expect((await fetch(first.url+"/cas/"+key,{method:"PUT",body:"hello"})).status).toBe(200);
 expect((await fetch(first.url+"/v1/cache/12345",{method:"PUT",body:"archive"})).status).toBe(200);
 const status=await (await fetch(first.url+"/status")).json() as {entries:number;bytes:number;counters:Record<string,{puts:number}>};
 expect(status.entries).toBe(2);expect(status.bytes).toBe(12);expect(status.counters.cas!.puts).toBe(1);expect(status.counters.nx!.puts).toBe(1);
 await first.stop();
 mkdirSync(join(folder,"v1",".tmp"),{recursive:true});writeFileSync(join(folder,"v1",".tmp","partial"),"half");
 const second=await serve({directory:folder});
 try{
  expect(second.store.usage().entries).toBe(2);expect(existsSync(join(folder,"v1",".tmp","partial"))).toBe(false);
  expect(await (await fetch(second.url+"/cas/"+key)).text()).toBe("hello");
 }finally{await second.stop();}
});

function modelSurvivors(max:number,ops:{put?:string;size?:number;read?:string}[]):string[]{
 const entries=new Map<string,number>();let total=0;
 const touch=(name:string)=>{const size=entries.get(name)!;entries.delete(name);entries.set(name,size);};
 for(const op of ops){
  if(op.read!==undefined){if(entries.has(op.read))touch(op.read);continue;}
  const name=op.put!,size=op.size!;
  if(total+size>max){
   const target=Math.floor(max*9/10)-size;
   for(const [victim,victimSize] of [...entries]){if(total<=target)break;entries.delete(victim);total-=victimSize;}
  }
  total-=entries.get(name)??0;entries.delete(name);entries.set(name,size);total+=size;
 }
 return [...entries.keys()].sort();
}

test("the bounded store evicts least recently used values exactly like an independent model and never exceeds its bound",async()=>{
 for(const row of bounds.cases){
  const store=await RemoteCacheStoreV1.open({directory:directory("bound"),maximumBytes:row.maximumBytes,maximumEntryBytes:row.maximumEntryBytes,maximumAgeMilliseconds:0}),keys=bounds.keys as Record<string,string>;
  for(const op of row.ops as {put?:string;size?:number;read?:string}[]){
   if(op.read!==undefined){await store.read("default","ac",keys[op.read]!);continue;}
   const body=new Uint8Array(op.size!).fill(97);
   expect(await store.put("default","ac",keys[op.put!]!,(async function*(){yield body;})(),{mode:"replace",declaredLength:op.size}),row.name).toMatch(/created|replaced/u);
   expect(store.usage().bytes,row.name).toBeLessThanOrEqual(row.maximumBytes);
  }
  await store.collect();
  const survivors=Object.keys(keys).filter(name=>store.has("default","ac",keys[name]!)).sort();
  expect(survivors,row.name).toEqual(row.survivors);expect(survivors,row.name+" against the model").toEqual(modelSurvivors(row.maximumBytes,row.ops as {put?:string;size?:number;read?:string}[]));
  await store.close();
 }
});

test("concurrent writers never exceed the bound, never expose a partial value and agree on the winner of a write-once key",async()=>{
 const server=await serve({maximumBytes:2**20,maximumEntryBytes:1024}),small=await serve({maximumBytes:4096,maximumEntryBytes:1024});
 try{
  const writes=await Promise.all(Array.from({length:64},(_,index)=>fetch(`${server.url}/v1/cache/key${index%8}`,{method:"PUT",body:String(index).padStart(512,"x")})));
  const statuses=writes.map(response=>response.status);
  expect(statuses.filter(status=>status===200).length).toBe(8);expect(statuses.filter(status=>status===409).length).toBe(56);
  for(let index=0;index<8;index++){const body=await (await fetch(`${server.url}/v1/cache/key${index}`)).text();expect(body.length).toBe(512);}
  const blobs=await Promise.all(Array.from({length:40},(_,index)=>{const value=`blob-${index}-`.padEnd(900,"y");return fetch(`${small.url}/cas/${digest(value)}`,{method:"PUT",body:value});}));
  expect(blobs.every(response=>response.status===200||response.status===507)).toBe(true);expect(blobs.some(response=>response.status===200)).toBe(true);
  await small.store.collect();expect(small.store.usage().bytes).toBeLessThanOrEqual(4096);expect(small.store.usage().reservedBytes).toBe(0);
 }finally{await server.stop();await small.stop();}
});

test("a multi-megabyte blob streams through with a verified digest equal to Node's and Python's",async()=>{
 const server=await serve({maximumEntryBytes:2**24}),bytes=new Uint8Array(6*1024*1024).map((_,index)=>(index*31+7)&255),expected=createHash("sha256").update(bytes).digest("hex");
 try{
  if(python){const result=Bun.spawnSync([python,"-I","-c","import hashlib,sys\nprint(hashlib.sha256(bytes(((i*31+7)&255) for i in range(256))*(6*1024*4)).hexdigest())"],{stdout:"pipe"});expect(result.stdout.toString().trim()).toBe(expected);}
  expect((await fetch(`${server.url}/cas/${expected}`,{method:"PUT",body:bytes})).status).toBe(200);
  const back=new Uint8Array(await (await fetch(`${server.url}/cas/${expected}`)).arrayBuffer());expect(createHash("sha256").update(back).digest("hex")).toBe(expected);expect(back.byteLength).toBe(bytes.byteLength);
  const flipped=bytes.slice();flipped[100]^=1;expect((await fetch(`${server.url}/cas/${expected.replace(/^./u,"f")}`,{method:"PUT",body:flipped})).status).toBe(400);
 }finally{await server.stop();}
},60000);

test("opening a store publishes scan progress and stops when cancelled; a server stops with its signal and refuses open listening beyond loopback",async()=>{
 const folder=directory("progress"),seed=await RemoteCacheStoreV1.open({directory:folder,maximumBytes:2**26,maximumEntryBytes:2**20,maximumAgeMilliseconds:0});
 for(let index=0;index<600;index++)await seed.put("default","nx",`key${index}`,(async function*(){yield new Uint8Array(8);})(),{mode:"keep",declaredLength:8});
 await seed.close();
 const events:RemoteCacheStoreEventV1[]=[],reopened=await RemoteCacheStoreV1.open({directory:folder,maximumBytes:2**26,maximumEntryBytes:2**20,maximumAgeMilliseconds:0,onEvent:event=>events.push(event)});
 expect(events.filter(event=>event.stage==="scan").length).toBeGreaterThanOrEqual(3);expect(events.at(-1)).toEqual({stage:"ready",entries:600,bytes:4800});await reopened.close();
 const controller=new AbortController();controller.abort();
 await expect(RemoteCacheStoreV1.open({directory:folder,maximumBytes:2**26,maximumEntryBytes:2**20,maximumAgeMilliseconds:0,signal:controller.signal})).rejects.toThrow();
 const live=new AbortController(),server=await serve({signal:live.signal});
 expect((await fetch(server.url+"/healthz")).status).toBe(200);live.abort();await server.stopped;
 await expect(fetch(server.url+"/healthz")).rejects.toThrow();
 await expect(serve({host:"0.0.0.0"})).rejects.toThrow("A token is required to listen beyond loopback");
 const guarded=await serve({host:"0.0.0.0",tokens:{write:"secret-token"}});await guarded.stop();
 await expect(serve({defaultInstance:"cas"})).rejects.toThrow("Valid default instance required");
 await expect(RemoteCacheStoreV1.open({directory:directory("bad"),maximumBytes:10,maximumEntryBytes:20,maximumAgeMilliseconds:0})).rejects.toThrow("Finite remote cache bounds required");
},60000);

test("idle entries older than the maximum age are dropped when the store collects",async()=>{
 const folder=directory("age"),store=await RemoteCacheStoreV1.open({directory:folder,maximumBytes:2**20,maximumEntryBytes:2**10,maximumAgeMilliseconds:50});
 await store.put("default","nx","old",(async function*(){yield new Uint8Array(4);})(),{mode:"keep",declaredLength:4});
 await new Promise(resolve=>setTimeout(resolve,120));
 await store.put("default","nx","new",(async function*(){yield new Uint8Array(4);})(),{mode:"keep",declaredLength:4});
 await store.collect();expect(store.has("default","nx","old")).toBe(false);expect(store.has("default","nx","new")).toBe(true);await store.close();
});

test("the command words parse like the catalogue says and refuse everything else, in both languages",()=>{
 expect(parseRemoteCacheArgumentsV1(["serve","--port","0","--host=127.0.0.1","--directory","cache","--max-bytes=1048576","--read-only","--tls-certificate","c.pem","--tls-key=k.pem"])).toMatchObject({operation:"serve",values:{"--port":"0","--host":"127.0.0.1","--directory":"cache","--max-bytes":"1048576","--tls-certificate":"c.pem","--tls-key":"k.pem"}});
 expect(parseRemoteCacheArgumentsV1(["gc","--dry-run","--json"]).flags).toEqual(new Set(["--dry-run","--json"]));
 for(const bad of [[],["clear"],["remote","status"],["serve","--port"],["serve","--port","--host"],["serve","--unknown","1"],["serve","--port="],["serve","--dry-run"],["gc","--read-only"],["status","--max-bytes","1"],["gc","--dry-run","--dry-run"],["serve","--read-only=true"]])expect(()=>parseRemoteCacheArgumentsV1(bad),bad.join(" ")).toThrow();
 let message="";try{parseRemoteCacheArgumentsV1(["serve","--bogus"]);}catch(error){message=(error as Error).message;}
 expect(message).toContain("Unknown option for remote-cache serve: --bogus");expect(message).toContain("Unbekannte Option für remote-cache serve: --bogus");
 expect(remoteCacheLanguagesV1("de",{})).toEqual(["de"]);expect(remoteCacheLanguagesV1(undefined,{SEMIO_LANGUAGE:"en"})).toEqual(["en"]);expect(remoteCacheLanguagesV1(undefined,{LANG:"de_DE.UTF-8"})).toEqual(["de"]);
 expect(remoteCacheLanguagesV1(undefined,{LANG:"C"}),"no default language: both are printed").toEqual(["en","de"]);
 expect(()=>remoteCacheLanguagesV1("fr",{})).toThrow("Unbekannte Sprache");
 expect(remoteCacheTextV1("de","serve.stopped")).toBe("Der Remote-Cache wurde beendet.");
});

test("every message exists in English and German with the same placeholders",()=>{
 const names=(value:string)=>[...value.matchAll(/\{([a-z]+)\}/gu)].map(match=>match[1]).sort();
 for(const [key,row] of Object.entries(texts.messages as Record<string,{en:string;de:string}>)){
  expect(Object.keys(row).sort(),key).toEqual(["de","en"]);expect(names(row.de),key).toEqual(names(row.en));
  if(key!=="gc.mode.apply")expect(row.en.length>0&&row.de.length>0&&row.en!==row.de||key.startsWith("serve.transport"),key).toBe(true);
 }
});

const toolsFor=async()=>[nxRemoteCacheToolV1(),bazelRemoteCacheToolV1()];

test("remote-cache status reports the contract, both renderings, a probe per protocol, the server and the store, redacts the token, and obeys the contract's token and cancellation",async()=>{
 const folder=directory("status-store"),server=await serve({directory:folder,tokens:{read:"read-only-token-9",write:"write-token-7"}}),written:string[]=[];
 const context=(environment:Record<string,string>,signal:AbortSignal=new AbortController().signal):RemoteCacheCommandContextV1=>({root:directory("root"),environment,signal,progress:async()=>{},write:line=>written.push(line),tools:toolsFor});
 try{
  expect((await fetch(server.url+"/v1/cache/777",{method:"PUT",body:"payload",headers:{authorization:"Bearer write-token-7"}})).status).toBe(200);
  const on={SEMIO_REMOTE_CACHE_ENDPOINT:server.url,SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_REMOTE_CACHE_TOKEN:"write-token-7"};
  const off=await remoteCacheStatusV1([],context({}));
  expect(off).toEqual({version:1,configured:false,contract:null,effectiveUpload:false,token:null,renderings:null,probes:null,server:null,store:null});expect(validator("Status")(off),JSON.stringify(validator("Status").errors)).toBe(true);
  const status=await remoteCacheStatusV1(["--directory",folder],context(on)) as {configured:boolean;probes:{protocol:string;reachable:boolean;authorized:boolean;status:number}[];server:{readOnly:boolean;validation:string;entries:number};store:{entries:number;bytes:number}};
  expect(validator("Status")(status),JSON.stringify(validator("Status").errors)).toBe(true);
  expect(status).toMatchObject({configured:true,effectiveUpload:true,token:{present:true},renderings:{nx:{mechanism:"nx-environment"},bazel:{mechanism:"bazel-flags"}},server:{readOnly:false,validation:"off",entries:1},store:{entries:1,bytes:7}});
  expect(status.probes.map(probe=>probe.protocol)).toEqual(["nx-self-hosted","bazel-http"]);for(const probe of status.probes)expect(probe).toMatchObject({reachable:true,authorized:true,status:404});
  expect(JSON.stringify(status),"never prints the token").not.toContain("write-token-7");
  const denied=await remoteCacheStatusV1([],context({...on,SEMIO_REMOTE_CACHE_TOKEN:"wrong-token"})) as {probes:{authorized:boolean;status:number}[];server:null};
  for(const probe of denied.probes)expect(probe).toMatchObject({authorized:false,status:401});expect(denied.server).toBeNull();
  const custom=await remoteCacheStatusV1([],context({...on,SEMIO_REMOTE_CACHE_TOKEN_ENVIRONMENT:"CI_CACHE_TOKEN",CI_CACHE_TOKEN:"read-only-token-9"})) as {token:{environment:string};probes:{authorized:boolean}[];server:{entries:number}};
  expect(custom.token.environment).toBe("CI_CACHE_TOKEN");for(const probe of custom.probes)expect(probe.authorized).toBe(true);expect(custom.server.entries).toBe(1);
  const unreachable=await remoteCacheStatusV1([],context({SEMIO_REMOTE_CACHE_ENDPOINT:"http://127.0.0.1:1",SEMIO_REMOTE_CACHE_TIMEOUT_SECONDS:"2"})) as {probes:{reachable:boolean;status:null}[]};
  for(const probe of unreachable.probes)expect(probe).toMatchObject({reachable:false,status:null});
  const cancelled=new AbortController();cancelled.abort();await expect(remoteCacheStatusV1([],context(on,cancelled.signal))).rejects.toThrow();
  expect(await runRemoteCacheCommandV1(["status"],context({}))).toEqual({exitCode:0,result:expect.objectContaining({configured:false})});
  await expect(runRemoteCacheCommandV1(["remote","clear"],context(on))).rejects.toThrow("Expected repo remote-cache serve|status|gc.");
 }finally{await server.stop();}
},60000);

async function listening(lines:string[]):Promise<string>{
 return new Promise<string>((resolve,reject)=>{const started=Date.now(),timer=setInterval(()=>{const line=lines.find(candidate=>candidate.startsWith("[remote-cache] listening "));if(line){clearInterval(timer);resolve(line.slice(25));}else if(Date.now()-started>20000){clearInterval(timer);reject(Error("no listening line"));}},20);});
}

test("remote-cache serve opens the store, announces its address in the chosen language, publishes progress and stops when cancelled",async()=>{
 const stages:string[]=[],lines:string[]=[],controller=new AbortController(),folder=directory("serve");
 const context:RemoteCacheCommandContextV1={root:folder,environment:{SEMIO_REMOTE_CACHE_SERVER_WRITE_TOKEN:"write-token-7"},signal:controller.signal,progress:async stage=>{stages.push(stage);},write:line=>lines.push(line),tools:toolsFor};
 const running=serveRemoteCacheV1(["--port","0","--directory","store","--max-bytes","1048576","--lang","en"],context);
 const address=await listening(lines);
 expect(address).toMatch(/^http:\/\/127\.0\.0\.1:[0-9]+$/u);
 expect((await fetch(address+"/healthz")).status).toBe(200);expect((await fetch(address+"/status")).status).toBe(401);
 expect((await fetch(address+"/v1/cache/abc",{method:"PUT",body:"x",headers:{authorization:"Bearer write-token-7"}})).status).toBe(200);
 controller.abort();expect(await running).toEqual({exitCode:0});
 expect(stages).toEqual(expect.arrayContaining(["remote-cache:open","remote-cache:scan","remote-cache:ready","remote-cache:serving"]));
 expect(lines.some(line=>line.startsWith("Serving the remote cache on "+address+" (read-write, plain HTTP, result validation complete)"))).toBe(true);
 expect(lines.some(line=>line.includes("Clients set SEMIO_REMOTE_CACHE_ENDPOINT="+address))).toBe(true);expect(lines.some(line=>line.includes("No token is set"))).toBe(false);
 expect(lines.slice(-2)).toEqual(["[remote-cache] stopped","The remote cache stopped."]);
 await expect(fetch(address+"/healthz")).rejects.toThrow();
 const german:string[]=[],second=new AbortController(),again=serveRemoteCacheV1(["--port","0","--directory","store","--lang","de","--read-only"],{...context,environment:{},signal:second.signal,write:line=>german.push(line)});
 await listening(german);second.abort();await again;
 expect(german.some(line=>line.startsWith("Der Remote-Cache läuft auf ")&&line.includes("nur lesend")&&line.includes("einfaches HTTP"))).toBe(true);expect(german.some(line=>line.includes("Kein Token gesetzt"))).toBe(true);expect(german.at(-1)).toBe("Der Remote-Cache wurde beendet.");
 const both:string[]=[],third=new AbortController(),neutral=serveRemoteCacheV1(["--port","0","--directory","store"],{...context,environment:{LANG:"C"},signal:third.signal,write:line=>both.push(line)});
 await listening(both);third.abort();await neutral;
 expect(both.filter(line=>line==="The remote cache stopped."||line==="Der Remote-Cache wurde beendet.").length,"without a language both are printed").toBe(2);
 expect(both.some(line=>line.includes("result validation complete"))&&both.some(line=>line.includes("Ergebnisprüfung vollständig")),"every part of a line follows its language").toBe(true);
 const quiet=new AbortController();
 await expect(serveRemoteCacheV1(["--port","70000"],context)).rejects.toThrow("--port must be a whole number from 0 to 65535");
 await expect(serveRemoteCacheV1(["--host","0.0.0.0","--port","0"],{...context,environment:{},signal:quiet.signal})).rejects.toThrow("A token is required to listen beyond loopback.");
 await expect(serveRemoteCacheV1(["--tls-certificate","only.pem"],context)).rejects.toThrow("TLS needs both a certificate and a key.");
 await expect(serveRemoteCacheV1(["--tls-certificate","missing.pem","--tls-key","missing.key"],context)).rejects.toThrow("TLS file required");
 const holder=new AbortController(),held:string[]=[],first=serveRemoteCacheV1(["--port","0","--directory","busy"],{...context,signal:holder.signal,write:line=>held.push(line)});
 await listening(held);await expect(serveRemoteCacheV1(["--port","0","--directory","busy"],context)).rejects.toThrow("Ein anderer Remote-Cache-Server benutzt");
 holder.abort();await first;
},90000);

test("remote-cache gc plans first, then removes exactly the planned entries, in the chosen language, and reports JSON",async()=>{
 const folder=directory("gc"),store=await RemoteCacheStoreV1.open({directory:folder,maximumBytes:2**22,maximumEntryBytes:2**20,maximumAgeMilliseconds:0}),lines:string[]=[];
 for(let index=0;index<12;index++){await store.put("main","nx",`key${index}`,(async function*(){yield new Uint8Array(100_000).fill(index);})(),{mode:"keep",declaredLength:100_000});}
 await store.close();
 const context=(write:(line:string)=>void,environment:Record<string,string>={}):RemoteCacheCommandContextV1=>({root:directory("root"),environment,signal:new AbortController().signal,progress:async()=>{},write,tools:toolsFor});
 expect(await collectRemoteCacheV1(["--directory",join(folder,"missing")],context(line=>lines.push(line),{SEMIO_LANGUAGE:"en"}))).toEqual({exitCode:0});expect(lines.at(-1)).toContain("does not exist; nothing to collect.");
 const plan=await collectRemoteCacheV1(["--directory",folder,"--max-bytes","1048576","--dry-run","--json"],context(()=>{})) as {exitCode:number;result:{dryRun:boolean;scanned:{entries:number};removed:{entries:number;budget:number;bytes:number};remaining:{entries:number}}};
 expect(plan.result).toMatchObject({dryRun:true,scanned:{entries:12},removed:{budget:plan.result.removed.entries},remaining:{entries:12-plan.result.removed.entries}});expect(plan.result.removed.entries).toBeGreaterThan(0);
 const mixed:string[]=[];await collectRemoteCacheV1(["--directory",folder,"--dry-run"],context(line=>mixed.push(line),{LANG:"C"}));
 expect(mixed.some(line=>line.includes("(plan only)"))&&mixed.some(line=>line.includes("(nur Plan)"))).toBe(true);
 const store2=await RemoteCacheStoreV1.open({directory:folder,maximumBytes:2**30,maximumEntryBytes:2**20,maximumAgeMilliseconds:0});expect(store2.usage().entries,"a plan removes nothing").toBe(12);await store2.close();
 const german:string[]=[],applied=await collectRemoteCacheV1(["--directory",folder,"--max-bytes","1048576","--lang","de"],context(line=>german.push(line)));
 expect(applied).toEqual({exitCode:0});expect(german.at(-1)).toContain(`${plan.result.removed.entries} von 12 Einträgen entfernt`);expect(german[0]).toContain("Sammle");
 const store3=await RemoteCacheStoreV1.open({directory:folder,maximumBytes:2**30,maximumEntryBytes:2**20,maximumAgeMilliseconds:0});expect(store3.usage().entries).toBe(plan.result.remaining.entries);expect(store3.usage().bytes).toBeLessThanOrEqual(1048576*0.9);
 expect(store3.has("main","nx","key11"),"the newest entry survives").toBe(true);expect(store3.has("main","nx","key0"),"the oldest entry goes first").toBe(false);await store3.close();
 const cancelling=new AbortController();cancelling.abort();
 expect(await collectRemoteCacheV1(["--directory",folder,"--dry-run"],{...context(()=>{},{SEMIO_LANGUAGE:"en"}),signal:cancelling.signal})).toEqual({exitCode:130});
});

test("the public JSON of the remote cache is schema-first: status, gc results and the message catalogue validate against the contract schema, and drifted documents are refused",async()=>{
 const server=await serve({tokens:{}}),status=await (await fetch(server.url+"/status")).json() as Record<string,any>;
 try{
  expect(validator("ServerStatus")(status),JSON.stringify(validator("ServerStatus").errors)).toBe(true);requireRemoteCacheShapeV1("ServerStatus",status,"x");
  const drifted:[string,(value:Record<string,any>)=>void][]=[["extra key",value=>{value.extra=1;}],["missing counter",value=>{delete value.counters.nx;}],["negative count",value=>{value.counters.ac.hits=-1;}],["extra counter field",value=>{value.counters.cas.errors=0;}],["unknown validation",value=>{value.validation="weird";}],["protocol order",value=>{value.protocols.reverse();}],["start time",value=>{value.startedAt="yesterday";}],["version",value=>{value.version=2;}]];
  for(const [name,mutate] of drifted){const copy=structuredClone(status);mutate(copy);expect(validator("ServerStatus")(copy),name).toBe(false);expect(()=>requireRemoteCacheShapeV1("ServerStatus",copy,"x"),name).toThrow();}
  expect(validator("TextCatalogue")(texts),JSON.stringify(validator("TextCatalogue").errors)).toBe(true);
  for(const bad of [{...texts,languages:["de","en"]},{...texts,messages:{...texts.messages,"x.y":{en:"only"}}},{...texts,messages:{...texts.messages,"Bad Key":{en:"a",de:"b"}}},{...texts,extra:true}])expect(validator("TextCatalogue")(bad)).toBe(false);
  const folder=directory("json"),store=await RemoteCacheStoreV1.open({directory:folder,maximumBytes:2**22,maximumEntryBytes:2**20,maximumAgeMilliseconds:0});
  for(let index=0;index<6;index++)await store.put("main","nx",`k${index}`,(async function*(){yield new Uint8Array(1000);})(),{mode:"keep",declaredLength:1000});await store.close();
  const context:RemoteCacheCommandContextV1={root:directory("root"),environment:{},signal:new AbortController().signal,progress:async()=>{},write:()=>{},tools:toolsFor};
  const results:unknown[]=[];
  for(const args of [["--directory",join(folder,"none"),"--json"],["--directory",folder,"--json","--dry-run"],["--directory",folder,"--json","--max-bytes","1048576"]]){const outcome=await collectRemoteCacheV1(args,context) as {result:unknown};results.push(outcome.result);expect(validator("GcResult")(outcome.result),JSON.stringify(validator("GcResult").errors)).toBe(true);}
  expect(results[0]).toMatchObject({present:false});expect(results[1]).toMatchObject({dryRun:true,scanned:{entries:6}});
  for(const bad of [{...(results[0] as object),present:true},{...(results[1] as object),extra:1},{version:1,dryRun:true,directory:"x"}])expect(validator("GcResult")(bad)).toBe(false);
 }finally{await server.stop();}
 for(const bad of [{maximumBytes:0,maximumEntryBytes:1,maximumAgeMilliseconds:0},{maximumBytes:10,maximumEntryBytes:1.5,maximumAgeMilliseconds:0},{maximumBytes:10,maximumEntryBytes:1,maximumAgeMilliseconds:-1},{maximumBytes:10,maximumEntryBytes:1,maximumAgeMilliseconds:0,extra:1},{maximumBytes:10,maximumEntryBytes:1}]){
  expect(validator("StoreBounds")(bad)).toBe(false);if(!("extra" in bad))await expect(RemoteCacheStoreV1.open({directory:directory("bounds"),...bad} as never)).rejects.toThrow("Finite remote cache bounds required");
 }
 expect(validator("StoreBounds")({maximumBytes:10,maximumEntryBytes:10,maximumAgeMilliseconds:0})).toBe(true);
});
const bazelBinary=process.env.SEMIO_TEST_BAZEL_BINARY;

test.skipIf(!bazelBinary)("a real Bazel client builds against the server, hits the remote cache after bazel clean, honours read-only and authenticates through the credential helper without writing the token to disk",async()=>{
 const token="rcache-sentinel-4711-token",base=directory("bazel"),workspace=join(base,"ws"),output=join(base,"ob"),second=join(base,"ws2"),outputTwo=join(base,"ob2"),events:{method:string;path:string;status:number}[]=[];
 mkdirSync(workspace,{recursive:true});
 writeFileSync(join(workspace,"MODULE.bazel"),'module(name="scratch",version="0.0.0")\n');writeFileSync(join(workspace,"in.txt"),"hello\n");
 writeFileSync(join(workspace,"BUILD.bazel"),'genrule(name="a",srcs=["in.txt"],outs=["a.txt"],cmd="cat $(SRCS) > $@ && echo a >> $@")\ngenrule(name="b",srcs=[":a"],outs=["b.txt"],cmd="cat $(SRCS) > $@ && echo b >> $@")\n');
 const server=await serve({tokens:{read:"read-only-token-9",write:token},onEvent:event=>{if(event.stage==="request")events.push(event);}});
 const run=async(environment:Record<string,string>,command:string[],extra:string[]=[],place:{cwd:string;output:string}={cwd:workspace,output})=>{
  const contract=resolveRemoteCacheContractV1(environment)!,rendering=bazelRemoteCacheV1(contract,environment,{helperDirectory:base});
  if(rendering.helper)writeFileSync(rendering.helper.path,rendering.helper.content,{mode:rendering.helper.mode});
  return client(bazelBinary!,[`--output_user_root=${join(base,"root")}`,`--output_base=${place.output}`,`--install_base=${join(base,"install")}`,"--nosystem_rc","--nohome_rc","--max_idle_secs=15",...command,"--lockfile_mode=off","--incompatible_autoload_externally=",...(command[0]==="build"?[...rendering.flags,...extra]:[])],{cwd:place.cwd,timeout:300000,env:{PATH:process.env.PATH!,HOME:process.env.HOME!,LANG:"en_US.UTF-8",LC_ALL:"en_US.UTF-8",...rendering.environment}});
 };
 const rw={SEMIO_REMOTE_CACHE_ENDPOINT:server.url,SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_REMOTE_CACHE_TOKEN:token};
 try{
  const cold=await run(rw,["build","//:b"]);expect(cold.status,cold.stderr).toBe(0);expect(cold.stderr).not.toContain("remote cache hit");
  expect(events.some(event=>event.method==="PUT"&&event.path.startsWith("/main/ac/")&&event.status===200)).toBe(true);
  expect((await run(rw,["clean"])).status).toBe(0);
  const warm=await run(rw,["build","//:b"]);expect(warm.status,warm.stderr).toBe(0);expect(warm.stderr).toMatch(/2 remote cache hit/u);
  const produced=/Target \/\/:b up-to-date:\s+(\S+)/u.exec(warm.stderr)?.[1];expect(produced).toBeDefined();expect(readFileSync(join(workspace,produced!),"utf8")).toBe("hello\na\nb\n");
  const status=await (await fetch(server.url+"/status",{headers:{authorization:"Bearer "+token}})).json() as {counters:{ac:{hits:number};cas:{hits:number}}};
  expect(status.counters.ac.hits).toBeGreaterThanOrEqual(2);
  expect((await run(rw,["shutdown"])).status).toBe(0);
  cpSync(workspace,second,{recursive:true});
  const fresh=await run(rw,["build","//:b"],[],{cwd:second,output:outputTwo});expect(fresh.status,fresh.stderr).toBe(0);
  const summary=/INFO: [0-9]+ processes?: ([^\n]*?)\./u.exec(fresh.stderr);expect(summary,fresh.stderr).not.toBeNull();
  const kinds=summary![1]!.split(", ").map(part=>/^([0-9]+) (.+)$/u.exec(part.trim())!).filter(part=>part[2]!=="internal");
  expect(kinds.map(part=>part[2]),"a second output base in another directory runs nothing: every process is a remote cache hit").toEqual(["remote cache hit"]);expect(Number(kinds[0]![1])).toBe(2);
  expect(readFileSync(join(second,/Target \/\/:b up-to-date:\s+(\S+)/u.exec(fresh.stderr)![1]!),"utf8")).toBe("hello\na\nb\n");
  expect((await run(rw,["shutdown"],[],{cwd:second,output:outputTwo})).status).toBe(0);
  expect((await run(rw,["clean"])).status).toBe(0);
  const anonymous=await run({SEMIO_REMOTE_CACHE_ENDPOINT:server.url},["build","//:b"]);expect(anonymous.status,anonymous.stderr).toBe(0);expect(anonymous.stderr).not.toMatch(/remote cache hit/u);
  expect(events.some(event=>event.status===401)).toBe(true);
  const before=events.filter(event=>event.method==="PUT").length;
  expect((await run({SEMIO_REMOTE_CACHE_ENDPOINT:server.url,SEMIO_REMOTE_CACHE_TOKEN:"read-only-token-9",SEMIO_REMOTE_CACHE_INSTANCE:"fresh"},["clean"])).status).toBe(0);
  const readonly=await run({SEMIO_REMOTE_CACHE_ENDPOINT:server.url,SEMIO_REMOTE_CACHE_TOKEN:"read-only-token-9",SEMIO_REMOTE_CACHE_INSTANCE:"fresh"},["build","//:b"]);expect(readonly.status,readonly.stderr).toBe(0);
  expect(events.filter(event=>event.method==="PUT").length,"read-only never pushes").toBe(before);
  const leaked=spawnSync("grep",["-rl",token,base],{encoding:"utf8"});expect(leaked.stdout.trim(),"the token appears in no file below the scratch workspace and output base").toBe("");
 }finally{await run(rw,["shutdown"]);await server.stop();}
},900000);

const nxBinary=process.env.SEMIO_TEST_NX_BIN;

test.skipIf(!nxBinary)("a real Nx 23 client stores archives, hits the remote cache after the local cache is gone, is rejected on a wrong token and is read-only with a read token",async()=>{
 const token="rcache-sentinel-4711-token",base=directory("nx"),events:{method:string;path:string;status:number}[]=[];
 mkdirSync(join(base,"libs/a"),{recursive:true});mkdirSync(join(base,"libs/b"),{recursive:true});symlinkSync(join(dirname(nxBinary!),"../../.."),join(base,"node_modules"),"junction");
 writeFileSync(join(base,"nx.json"),JSON.stringify({cacheDirectory:".nx-cache",useDaemonProcess:false,analytics:false,targetDefaults:{build:{cache:true,inputs:["{projectRoot}/**/*"],outputs:["{projectRoot}/out"]}}}));
 writeFileSync(join(base,"libs/a/project.json"),JSON.stringify({name:"a",root:"libs/a",targets:{build:{executor:"nx:run-commands",options:{command:"mkdir -p out && echo built-a > out/a.txt",cwd:"libs/a"}}}}));
 writeFileSync(join(base,"libs/b/project.json"),JSON.stringify({name:"b",root:"libs/b",implicitDependencies:["a"],targets:{build:{dependsOn:["^build"],executor:"nx:run-commands",options:{command:"mkdir -p out && echo built-b > out/b.txt",cwd:"libs/b"}}}}));
 writeFileSync(join(base,"libs/a/src.txt"),"a\n");writeFileSync(join(base,"libs/b/src.txt"),"b\n");
 const server=await serve({tokens:{read:"read-only-token-9",write:token},onEvent:event=>{if(event.stage==="request")events.push(event);}});
 const run=(environment:Record<string,string>)=>{
  const rendering=nxRemoteCacheV1(resolveRemoteCacheContractV1(environment),environment);
  return client("node",[nxBinary!,"run-many","-t","build"],{cwd:base,timeout:240000,env:{PATH:process.env.PATH!,HOME:process.env.HOME!,NX_DAEMON:"false",NX_NO_CLOUD:"true",NX_TUI:"false",CI:"1",...Object.fromEntries(Object.entries(rendering.environment).filter((entry):entry is [string,string]=>entry[1]!==undefined))}});
 };
 const reset=()=>{rmSync(join(base,".nx-cache"),{recursive:true,force:true});rmSync(join(base,".nx"),{recursive:true,force:true});rmSync(join(base,"libs/a/out"),{recursive:true,force:true});rmSync(join(base,"libs/b/out"),{recursive:true,force:true});};
 const rw={SEMIO_REMOTE_CACHE_ENDPOINT:server.url,SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_REMOTE_CACHE_TOKEN:token};
 try{
  const cold=await run(rw);expect(cold.status,cold.stderr).toBe(0);expect(cold.stdout).toContain("0/2 hit");
  expect(events.filter(event=>event.method==="PUT"&&event.path.startsWith("/main/v1/cache/")&&event.status===200).length).toBe(2);
  reset();const warm=await run(rw);expect(warm.status,warm.stderr).toBe(0);expect(warm.stdout).toContain("2/2 hit (100%)");expect(readFileSync(join(base,"libs/b/out/b.txt"),"utf8")).toBe("built-b\n");
  reset();const wrong=await run({...rw,SEMIO_REMOTE_CACHE_TOKEN:"wrong-token"});expect(wrong.status).not.toBe(0);expect(wrong.stderr+wrong.stdout).toContain("Unauthorized");
  reset();const readOnly=await run({SEMIO_REMOTE_CACHE_ENDPOINT:server.url,SEMIO_REMOTE_CACHE_TOKEN:"read-only-token-9"});expect(readOnly.status,readOnly.stderr).toBe(0);expect(readOnly.stdout).toContain("2/2 hit (100%)");
  writeFileSync(join(base,"libs/a/src.txt"),"a changed\n");
  const refused=await run({SEMIO_REMOTE_CACHE_ENDPOINT:server.url,SEMIO_REMOTE_CACHE_TOKEN:"read-only-token-9"});expect(refused.status,refused.stderr).toBe(0);
  expect(events.some(event=>event.method==="PUT"&&event.status===403),"a read-only token is refused on write").toBe(true);
  const leaked=spawnSync("grep",["-rl",token,join(base,".nx-cache")],{encoding:"utf8"});expect(leaked.stdout.trim()).toBe("");
 }finally{await server.stop();}
},900000);

afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
