import {afterEach,expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import {createHash} from "node:crypto";
import {brotliCompressSync} from "node:zlib";
import {existsSync,mkdirSync,mkdtempSync,readFileSync,rmSync,statSync,truncateSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join,relative,sep} from "node:path";
import Ajv from "ajv";
import {createNodeFilePortV1,readGenerationManifestV1,readGenerationPointerV1} from "../../../🟨️.mjs";
import {CLOSURE_POLICY_V1} from "../../../../🩹️closure-repair/🟨️.mjs";
import {loadGenerationGraphV1} from "../../../🔌️load/🟨️.mjs";
import {applyOps,createMockHost,ringFiles,seed} from "../../../🧪️tests/🏃️world/🟦️.ts";
import {startRemoteCacheServerV1,type RemoteCacheServerV1} from "../../../../../🌐️remote-cache/🖥️server/🟦️.ts";
import {httpRemoteExecutionContentV1} from "../../../../../🌐️remote-execution/🗄️content/🟦️.ts";
import {REAPI_V1,decodeProtobufV1,encodeProtobufV1} from "../../../../../🌐️remote-execution/📡️wire/📦️protobuf/🟦️.ts";
import {remoteGenerationExchangeV1,workspaceClosurePortV1,type ClosurePortV1} from "../../🟦️.ts";
import {closureEntryV1,encodeClosureMemoV1,inventoryFragmentClosuresV1,readClosureMemoV1} from "../../🧲️closure-shards/🟦️.ts";
import transportSchema from "../../🧬️schema/🔣️.json" with {type:"json"};
import currentCases from "./🔣️.json";
import currentSchema from "./🧬️schema/🔣️.json";
import boundedCases from "./📏️bounded.json";
import transportCases from "./🕓️transport.json";
import boundedSchema from "./🧬️schema/📏️bounded.json";

/** 🧲️ Contract (one real cache server on a free port with a temporary store, never a production identity): the build input closure leaves a generation names travel with it. An upload sends each named leaf the cache misses once, a restore fetches the named leaves this machine lacks and verifies them against their names, and a leaf nobody holds is reported with the owners that name it (they are stale) without failing the restore. */
const sha=(bytes:Uint8Array|string)=>createHash("sha256").update(bytes).digest("hex"),files=createNodeFilePortV1(),directories:string[]=[],servers:RemoteCacheServerV1[]=[],platform="darwin-arm64";
const scratch=()=>{const path=mkdtempSync(join(tmpdir(),"graph-closures-"));directories.push(path);return path;};
afterEach(async()=>{for(const server of servers.splice(0))await server.stop();for(const path of directories.splice(0))rmSync(path,{recursive:true,force:true});});
async function serve():Promise<RemoteCacheServerV1>{const server=await startRemoteCacheServerV1({directory:scratch(),host:"127.0.0.1",port:0,maximumBytes:2**28,maximumEntryBytes:2**26,maximumAgeDays:30,defaultInstance:"main",tokens:{},validation:"complete",maximumActionResultBytes:4194304});servers.push(server);return server;}
const environment=(server:RemoteCacheServerV1)=>({SEMIO_REMOTE_CACHE_ENDPOINT:server.url,SEMIO_REMOTE_CACHE_ACCESS:"read-write"});
const NAMESPACE="build-input-closures" as const;
function port(base:string):ClosurePortV1{
 const directory=(namespace:string)=>join(base,namespace);
 return{directory,size:(namespace,key)=>{try{const row=statSync(join(directory(namespace),key+".json"));return row.isFile()?row.size:undefined;}catch{return undefined;}},read:async(namespace,key)=>{try{return readFileSync(join(directory(namespace),key+".json"));}catch{return undefined;}},put:async(namespace,key,bytes)=>{mkdirSync(directory(namespace),{recursive:true});writeFileSync(join(directory(namespace),key+".json"),bytes);}};
}
const leaf=(text:string)=>{const bytes=Buffer.from(JSON.stringify({files:[text]}));return{bytes,key:sha(bytes)};};
const tokens=(keys:Record<string,string>)=>Object.fromEntries(Object.entries(keys).map(([name,key])=>[`pkg/${name}/project.json`,JSON.stringify({name,package:"SEMIO_SOURCE_CLOSURE_"+key,uses:[]})]));
async function produce(server:RemoteCacheServerV1,workspace:string,closures:ClosurePortV1,named:Record<string,string>){
 await seed(workspace,{...ringFiles(6),...tokens(named)});
 const mock=createMockHost(workspace);
 (mock.host as any).exchange=remoteGenerationExchangeV1({environment:environment(server),workspace,identity:()=>mock.host.identity(),platform,closures});
 await loadGenerationGraphV1(mock.host);
 return mock;
}
const consumer=(server:RemoteCacheServerV1,workspace:string,closures:ClosurePortV1,identity:string)=>remoteGenerationExchangeV1({environment:environment(server),workspace,identity:async()=>identity,platform,closures})!;

test("workspace closure verification hashes bounded chunks and reports an unknown entry kind safely",async()=>{
 const workspace=scratch(),store=workspaceClosurePortV1(workspace)!;
 const directory=store.directory(NAMESPACE),inside=relative(workspace,directory);
 expect(inside.startsWith(".cache"+sep)).toBe(true);
 const bytes=Buffer.alloc(2**17+11,41),key=sha(bytes),path=join(directory,key+".json");
 const verify=store.verify!;
 await expect(verify(NAMESPACE,key)).resolves.toBe(false);
 await expect(verify(NAMESPACE,"../elsewhere")).resolves.toBeUndefined();
 mkdirSync(directory,{recursive:true});
 writeFileSync(path,bytes);
 await expect(verify(NAMESPACE,key)).resolves.toBe(true);
 bytes[bytes.length-1]=42;
 writeFileSync(path,bytes);
 await expect(verify(NAMESPACE,key)).resolves.toBe(false);
 truncateSync(path,CLOSURE_POLICY_V1.maximumLeafBytes+1);
 await expect(verify(NAMESPACE,key)).resolves.toBe(false);
 const reason=Error("cancel closure proof"),signal=AbortSignal.abort(reason);
 await expect(verify(NAMESPACE,key,signal)).rejects.toBe(reason);
});

test("a closure port with a shared reader stats every leaf under the lease and answers the same size as the plain stat",async()=>{
 const workspace=scratch(),plain=workspaceClosurePortV1(workspace)!,runs:string[]=[],reader={run:async<T>(body:()=>T|Promise<T>):Promise<T>=>{runs.push("run");return body();},close:async()=>undefined},leased=workspaceClosurePortV1(workspace,reader)!;
 const bytes=Buffer.alloc(33,7),key=sha(bytes),directory=plain.directory(NAMESPACE);
 expect(plain.sizeLeased).toBeUndefined();
 await expect(leased.sizeLeased!(NAMESPACE,key)).resolves.toBeUndefined();
 mkdirSync(directory,{recursive:true});writeFileSync(join(directory,key+".json"),bytes);
 await expect(leased.sizeLeased!(NAMESPACE,key)).resolves.toBe(33);
 expect(leased.size(NAMESPACE,key)).toBe(33);expect(plain.size(NAMESPACE,key)).toBe(33);expect(runs).toEqual(["run","run"]);
});

test("an upload sends each named leaf once and a restore fetches the leaves this machine lacks, verified by name",async()=>{
 const server=await serve(),one=leaf("a.ts"),two=leaf("b.ts"),mother=scratch(),motherLeaves=port(scratch());
 await motherLeaves.put(NAMESPACE,one.key,one.bytes);await motherLeaves.put(NAMESPACE,two.key,two.bytes);
 const mock=await produce(server,mother,motherLeaves,{q0:one.key,q1:two.key,q2:one.key}),content=httpRemoteExecutionContentV1({endpoint:server.url});
 expect(await content.has("main",one.key)).toBe(true);expect(await content.has("main",two.key)).toBe(true);
 const child=scratch(),childLeaves=port(scratch()),outcome=await consumer(server,child,childLeaves,await mock.host.identity()).restore({root:join(child,".gen"),fs:files});
 expect(outcome.status).toBe("restored");expect(outcome.closures).toEqual({named:2,sent:0,fetched:2,missing:[]});
 expect(existsSync(join(childLeaves.directory(NAMESPACE),one.key+".json"))).toBe(true);expect(sha(readFileSync(join(childLeaves.directory(NAMESPACE),two.key+".json")))).toBe(two.key);
 expect(outcome.stale).toBe(0);
});

test("a leaf that is on this machine is not fetched and a leaf the fleet lacks is reported with the owners that name it, the restore still completes with those owners stale",async()=>{
 const server=await serve(),held=leaf("held.ts"),lost=leaf("lost.ts"),mother=scratch(),motherLeaves=port(scratch());
 await motherLeaves.put(NAMESPACE,held.key,held.bytes);
 const mock=await produce(server,mother,motherLeaves,{q0:held.key,q1:lost.key,q3:lost.key});
 const content=httpRemoteExecutionContentV1({endpoint:server.url});expect(await content.has("main",lost.key)).toBe(false);
 const child=scratch(),childLeaves=port(scratch());await childLeaves.put(NAMESPACE,held.key,held.bytes);
 const outcome=await consumer(server,child,childLeaves,await mock.host.identity()).restore({root:join(child,".gen"),fs:files});
 expect(outcome.status).toBe("restored");
 expect(outcome.closures).toEqual({named:2,sent:0,fetched:0,missing:[{key:lost.key,namespace:NAMESPACE,projects:["q1","q3"]}]});
 expect(outcome.stale).toBe(2);expect(existsSync(join(childLeaves.directory(NAMESPACE),lost.key+".json"))).toBe(false);
});

test("the leaf is taken from the fleet cache by the exchange and an absent leaf is nothing, not an error",async()=>{
 const server=await serve(),good=leaf("good.ts"),content=httpRemoteExecutionContentV1({endpoint:server.url}),exchange=remoteGenerationExchangeV1({environment:environment(server),workspace:scratch(),identity:async()=>sha("x"),platform,closures:port(scratch())})!;
 await content.write("main",{hash:good.key,size:good.bytes.length},good.bytes);
 expect(Buffer.from((await exchange.leaf({namespace:NAMESPACE,key:good.key}))!).equals(good.bytes)).toBe(true);
 expect(await exchange.leaf({namespace:NAMESPACE,key:sha("absent")})).toBeUndefined();
});

test("an exact leaf fetch cancels oversized bytes before the request deadline",async()=>{
 const one=leaf("strict.ts"),chunk=new Uint8Array(1048576),stopped=Promise.withResolvers<void>();
 const receiver=Bun.serve({hostname:"127.0.0.1",port:0,fetch(){
  let left=CLOSURE_POLICY_V1.maximumLeafBytes+1;
  return new Response(new ReadableStream({
   pull(controller){if(left>0){const size=Math.min(left,chunk.length);controller.enqueue(chunk.subarray(0,size));left-=size;}},
   cancel(){stopped.resolve();}
  }));
 }});
 try{
  const signal=AbortSignal.timeout(1500),workspace=scratch();
  const exchange=remoteGenerationExchangeV1({environment:{SEMIO_REMOTE_CACHE_ENDPOINT:receiver.url.origin},workspace,identity:async()=>sha("x"),platform,closures:port(scratch())})!;
  expect(await exchange.leaf({namespace:NAMESPACE,key:one.key,signal})).toBeUndefined();
  expect(signal.aborted).toBe(false);
  await Promise.race([stopped.promise,Bun.sleep(1000).then(()=>{throw Error("oversized exact leaf download was not cancelled");})]);
 }finally{await receiver.stop(true);}
});

test("a graph exchange probes the same cache after a transient restore transport failure",async()=>{
 using oracle=new Database(":memory:");
 const server=await serve(),mother=scratch(),mock=await produce(server,mother,port(scratch()),{}),root=join(scratch(),".gen"),identity=await mock.host.identity();
 const source=join(mother,".gen"),pointer=readGenerationPointerV1((await files.read(join(source,"current.json")))!);
 const manifest=readGenerationManifestV1((await files.read(join(source,"m",pointer.manifest+".json")))!);
 let unavailable=true,clock=0,requests=0;
 const proxy=Bun.serve({hostname:"127.0.0.1",port:0,fetch(request){
  requests++;
  return unavailable?new Response(null,{status:503}):fetch(new URL(new URL(request.url).pathname,server.url));
 }});
 try{
  const exchange=remoteGenerationExchangeV1({environment:{SEMIO_REMOTE_CACHE_ENDPOINT:proxy.url.origin},workspace:scratch(),identity:async()=>identity,platform,closures:port(scratch()),now:()=>clock})!;
  const request={root,fs:files,implementation:manifest.implementation,requireComplete:true as const};
  expect((await exchange.restore(request)).status).toBe("failed");
  unavailable=false;
  const initial=requests,observed=[];
  for(const offset of transportCases.observations){
   clock=offset;
   const look=await exchange.look(request);
   observed.push(Number(look.state==="complete"&&look.candidate!==undefined));
  }
  const expected=oracle.query("select value >= ?1 as available from json_each(?2) order by key").values(transportCases.retryMilliseconds,JSON.stringify(transportCases.observations)).flat();
  expect<unknown>(observed).toEqual(expected);
  expect(requests-initial).toBe(1);
 }finally{await proxy.stop(true);}
});

test("whole current fetch follows the independent policy model across head and closure changes",async()=>{
 expect(new Ajv({strict:true}).validate(currentSchema,currentCases)).toBe(true);
 using oracle=new Database(":memory:");
 const verdict=oracle.query(`SELECT CASE
   WHEN $head <> 'complete' OR $closure = 'unknown' THEN 'GENERATION_NONE_SEALED'
   WHEN $closure = 'missing' THEN 'closure-missing'
   WHEN $closure = 'cancelled' THEN 'ABORT_ERR'
   ELSE 'served' END AS status,
   ($head = 'complete' AND $closure = 'available') AS sealed`);
 const server=await serve(),one=leaf("current.ts"),mother=scratch(),motherLeaves=port(scratch());
 await motherLeaves.put(NAMESPACE,one.key,one.bytes);
 const published=await produce(server,mother,motherLeaves,{q0:one.key});
 let selectedHead="complete",closure="available",looked=false,cancel:()=>void;
 const receiver=Bun.serve({hostname:"127.0.0.1",port:0,async fetch(request){
  const path=new URL(request.url).pathname;
  if(path.endsWith("/cas/"+one.key)){
   if(closure==="cancelled")cancel();
   if(closure==="missing")return new Response(null,{status:404});
  }
  const response=await fetch(new URL(path,server.url));
  if(!looked||selectedHead==="complete"||!path.includes("/ac/")||!response.ok)return response;
  const result=decodeProtobufV1(REAPI_V1.actionResult,new Uint8Array(await response.arrayBuffer()));
  const note=JSON.parse(Buffer.from(result.stdout_raw as Uint8Array).toString("utf8"));
  if(selectedHead==="partial")note.stale=1;
  if(selectedHead==="other-code")note.implementation="0".repeat(64);
  if(selectedHead==="manifest-mismatch")note.owners++;
  return new Response(encodeProtobufV1(REAPI_V1.actionResult,{...result,stdout_raw:Buffer.from(JSON.stringify(note))}));
 }});
 try{
  const identity=await published.host.identity();
  for(const row of currentCases){
   const controller=new AbortController(),child=scratch(),childLeaves=port(scratch()),reader=createMockHost(child,{signal:controller.signal}),root=join(child,".gen"),pointer=join(root,"current.json"),effects:string[]=[];
   await seed(child,{...ringFiles(6),...tokens({q0:one.key})});
   ({selectedHead,closure}=row);looked=false;
   cancel=()=>controller.abort(Object.assign(Error("cancelled"),{code:"ABORT_ERR"}));
   if(closure==="unknown")childLeaves.size=()=>{throw Error("closure store unavailable");};
   const exchange=remoteGenerationExchangeV1({environment:{SEMIO_REMOTE_CACHE_ENDPOINT:receiver.url.origin},workspace:child,identity:async()=>identity,platform,closures:childLeaves})!;
   Object.assign(reader.host,{exchange:{...exchange,async look(options:Parameters<typeof exchange.look>[0]){const found=await exchange.look(options);expect(found,row.name).toMatchObject({state:"complete",ahead:true});looked=true;return found;}},settleClosure:async()=>{effects.push("settle");return"re-derived";},fleetRead:async()=>{effects.push("fleet");}});
   const result=await loadGenerationGraphV1(reader.host,{policy:"current",lean:true}).then((graph):{status:string;graph?:unknown;key?:unknown}=>({status:"served",graph:graph.graph}),(error):{status:string;graph?:unknown;key?:unknown}=>({status:error.code,key:error.missingClosure}));
   expect(effects,row.name).toEqual([]);expect(reader.log,row.name).toEqual([]);
   expect<unknown>({status:result.status,sealed:Number(existsSync(pointer))},row.name).toEqual(verdict.get({$head:selectedHead,$closure:closure}));
   if(result.status==="closure-missing")expect(result.key).toBe(one.key);
   if(result.status==="served")expect(Object.keys((result.graph as {projects:object}).projects).sort()).toEqual(["p0","p1","p2","p3","p4","p5","q0"]);
  }
 }finally{await receiver.stop(true);}
});

test("a later upload of the same generation lineage asks the cache nothing about the leaves it already sent and still lists them in its head, so a reader after the second upload gets them",async()=>{
 const server=await serve(),one=leaf("a.ts"),mother=scratch(),motherLeaves=port(scratch());
 await motherLeaves.put(NAMESPACE,one.key,one.bytes);
 const mock=await produce(server,mother,motherLeaves,{q0:one.key});
 const sentFile=join(mother,".gen")+".closures-sent.json";expect(JSON.parse(readFileSync(sentFile,"utf8")).keys).toEqual([one.key]);
 await Bun.sleep(30);await applyOps(mother,[{op:"write",path:"pkg/p3/src/index.ts",content:"import \"@syn/p1\";\nexport const edited=1;\n"}]);
 const again=createMockHost(mother);(again.host as any).exchange=remoteGenerationExchangeV1({environment:environment(server),workspace:mother,identity:()=>again.host.identity(),platform,closures:motherLeaves});
 await loadGenerationGraphV1(again.host);
 expect(JSON.parse(readFileSync(sentFile,"utf8")).keys).toEqual([one.key]);
 const child=scratch(),childLeaves=port(scratch()),outcome=await consumer(server,child,childLeaves,await mock.host.identity()).restore({root:join(child,".gen"),fs:files});
 expect(outcome.status).toBe("restored");expect(outcome.closures).toEqual({named:1,sent:0,fetched:1,missing:[]});
});

test("strict restore verifies installed closure bytes and cancels an oversized remote stream",async()=>{
 expect(new Ajv({strict:true}).validate(boundedSchema,boundedCases)).toBe(true);
 using oracle=new Database(":memory:");
 const decide=oracle.query(`SELECT ($local='valid' OR ($remote='valid' AND $write='succeed')) AS sealed,
   ($local<>'valid') AS fetched, ($local<>'valid' AND $remote='valid') AS written`);
 const server=await serve(),one=leaf("strict.ts"),wrong=Buffer.alloc(one.bytes.length,49),mother=scratch(),motherLeaves=port(scratch());
 await motherLeaves.put(NAMESPACE,one.key,one.bytes);
 const mock=await produce(server,mother,motherLeaves,{q0:one.key}),identity=await mock.host.identity();
 const pointer=readGenerationPointerV1((await files.read(join(mother,".gen","current.json")))!);
 const manifest=readGenerationManifestV1((await files.read(join(mother,".gen","m",pointer.manifest+".json")))!);
 let selected="missing",requests=0,oversizedCancelled=()=>{};
 const chunk=new Uint8Array(1048576);
 const receiver=Bun.serve({hostname:"127.0.0.1",port:0,fetch(request){
  const path=new URL(request.url).pathname;
  if(!path.endsWith("/cas/"+one.key))return fetch(new URL(path,server.url));
  requests++;
  if(selected==="missing")return new Response(null,{status:404});
  if(selected==="corrupt")return new Response(Uint8Array.from(wrong));
  if(selected==="valid")return new Response(Uint8Array.from(one.bytes));
  let left=CLOSURE_POLICY_V1.maximumLeafBytes+1;
  return new Response(new ReadableStream({
   pull(controller){if(left>0){const size=Math.min(left,chunk.length);controller.enqueue(chunk.subarray(0,size));left-=size;}},
   cancel(){oversizedCancelled();},
  }));
 }});
 try{
  for(const row of boundedCases){
   const root=join(scratch(),".gen"),local=port(scratch()),signal=AbortSignal.timeout(5000);
   if(row.local!=="absent")await local.put(NAMESPACE,one.key,row.local==="valid"?one.bytes:wrong);
   let writes=0;
   const closures={...local,async put(...args:Parameters<typeof local.put>){
    writes++;
    if(row.write==="refuse")throw Error("immutable leaf");
    if(row.write==="succeed")await local.put(...args);
   }};
   selected=row.remote;
   requests=0;
   const stopped=Promise.withResolvers<void>();oversizedCancelled=stopped.resolve;
   const exchange=remoteGenerationExchangeV1({environment:{SEMIO_REMOTE_CACHE_ENDPOINT:receiver.url.origin},workspace:root,identity:async()=>identity,platform,closures})!;
   const outcome=await exchange.restore({root,fs:files,signal,implementation:manifest.implementation,requireComplete:true});
   const expected=decide.get({$local:row.local,$remote:row.remote,$write:row.write}) as {sealed:number;fetched:number;written:number};
   expect(outcome.status,JSON.stringify(row)).toBe(expected.sealed?"restored":"incomplete");
   expect({sealed:Number(existsSync(join(root,"current.json"))),fetched:requests,written:writes}).toEqual(expected);
   if(expected.sealed){
    const installed=(await closures.read(NAMESPACE,one.key))!;
    expect(Buffer.from(await crypto.subtle.digest("SHA-256",Uint8Array.from(installed))).toString("hex")).toBe(one.key);
   }
   if(row.remote==="oversized"){
    await Promise.race([stopped.promise,Bun.sleep(1000).then(()=>{throw Error("oversized closure download was not cancelled");})]);
    expect(signal.aborted).toBe(false);
   }
  }
 }finally{await receiver.stop(true);}
});

test("a bounded closure memo retains a verified fragment inventory across publications",async()=>{
 const closure=leaf("shared input"),entry=closureEntryV1(NAMESPACE,closure.key);
 const compressed=brotliCompressSync(Buffer.from(JSON.stringify(tokens({owner:closure.key}))));
 const fragment=sha(compressed),memo=readClosureMemoV1();
 let reads=0;
 const first=await inventoryFragmentClosuresV1([fragment],async()=>{reads++;return compressed;},undefined,memo);
 expect(first.get(entry)?.fragments.has(fragment)).toBe(true);
 expect(reads).toBe(1);
 const encoded=encodeClosureMemoV1([fragment],memo);
 expect(encoded).toBeDefined();
 const validator=new Ajv({strict:true}).compile(transportSchema.definitions.ClosureMemo);
 const payload=JSON.parse(encoded!);
 expect(validator(payload)).toBe(true);
 expect(payload.entries).toEqual([[fragment,[entry]]]);expect(payload.sha256).toBe(sha(JSON.stringify(payload.entries)));
 const recovered=readClosureMemoV1(Buffer.from(encoded!));
 const second=await inventoryFragmentClosuresV1([fragment],async()=>{reads++;return undefined;},undefined,recovered);
 expect(second).toEqual(first);
 expect(reads).toBe(1);
 const memoText=(entries:unknown)=>JSON.stringify({version:1,sha256:sha(JSON.stringify(entries)),entries});
 const duplicate=memoText([[fragment,[entry,entry]]]);
 expect(readClosureMemoV1(Buffer.from(duplicate)).size).toBe(0);
 const unsorted=[entry,closureEntryV1(NAMESPACE,leaf("other input").key)].sort().reverse();
 const outOfOrder=JSON.parse(memoText([[fragment,unsorted]]));
 expect(validator(outOfOrder)).toBe(true);
 expect(readClosureMemoV1(Buffer.from(JSON.stringify(outOfOrder))).size).toBe(0);
 expect(encodeClosureMemoV1([sha("absent fragment")],memo)).toBeUndefined();
});
