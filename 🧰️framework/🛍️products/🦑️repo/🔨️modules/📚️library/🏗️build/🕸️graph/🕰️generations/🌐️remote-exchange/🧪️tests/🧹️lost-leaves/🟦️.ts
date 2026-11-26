import {afterEach,expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import {createHash} from "node:crypto";
import {mkdtempSync,readFileSync,rmSync,statSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {createFrontierV1,createGenerationStoreV1,createNodeFilePortV1} from "../../../🟨️.mjs";
import {loadGenerationGraphV1} from "../../../🔌️load/🟨️.mjs";
import {applyOps,createMockHost,ringFiles,seed} from "../../../🧪️tests/🏃️world/🟦️.ts";
import {startRemoteCacheServerV1,type RemoteCacheServerV1} from "../../../../../🌐️remote-cache/🖥️server/🟦️.ts";
import {httpRemoteExecutionContentV1} from "../../../../../🌐️remote-execution/🗄️content/🟦️.ts";
import {REAPI_V1,decodeProtobufV1} from "../../../../../🌐️remote-execution/📡️wire/📦️protobuf/🟦️.ts";
import {generationExchangeKeyV1,publishGenerationV1,remoteGenerationExchangeV1,type ClosurePortV1} from "../../🟦️.ts";
import exchangeSchema from "../../🧬️schema/🔣️.json";

/** 🧹️ A validating replacement cache can repair evicted, memoized closure leaves during the same publication. */
const sha=(bytes:Uint8Array|string)=>createHash("sha256").update(bytes).digest("hex"),files=createNodeFilePortV1(),directories:string[]=[],servers:RemoteCacheServerV1[]=[],platform="darwin-arm64",NAMESPACE="build-input-closures" as const;
const scratch=()=>{const path=mkdtempSync(join(tmpdir(),"graph-lost-"));directories.push(path);return path;};
afterEach(async()=>{for(const server of servers.splice(0))await server.stop();for(const path of directories.splice(0))rmSync(path,{recursive:true,force:true});});
async function serve(tokens:{read?:string;write?:string}={}):Promise<RemoteCacheServerV1>{const server=await startRemoteCacheServerV1({directory:scratch(),host:"127.0.0.1",port:0,maximumBytes:2**28,maximumEntryBytes:2**26,maximumAgeDays:30,defaultInstance:"main",tokens,validation:"complete",maximumActionResultBytes:4194304});servers.push(server);return server;}
const environment=(server:RemoteCacheServerV1)=>({SEMIO_REMOTE_CACHE_ENDPOINT:server.url,SEMIO_REMOTE_CACHE_ACCESS:"read-write"});
const port=(base:string):ClosurePortV1=>{const directory=(namespace:string)=>join(base,namespace);return{directory,size:(namespace,key)=>{try{const row=statSync(join(directory(namespace),key+".json"));return row.isFile()?row.size:undefined;}catch{return undefined;}},read:async(namespace,key)=>{try{return readFileSync(join(directory(namespace),key+".json"));}catch{return undefined;}},put:async(namespace,key,bytes)=>{const{mkdirSync,writeFileSync}=await import("node:fs");mkdirSync(directory(namespace),{recursive:true});writeFileSync(join(directory(namespace),key+".json"),bytes);}} as ClosurePortV1;};

test("a replacement cache repairs missing memoized closures during one publication",async()=>{
 const first=await serve(),mother=scratch(),leaves=port(scratch()),payloads=["lost-a.ts","lost-b.ts","cached.ts"].map(file=>Buffer.from(JSON.stringify({files:[file]}))),keys=payloads.map(sha);
 for(const [index,key] of keys.entries())await leaves.put(NAMESPACE,key,payloads[index]!);
 await seed(mother,{...ringFiles(6),...Object.fromEntries(keys.map((key,index)=>["pkg/q"+index+"/project.json",JSON.stringify({name:"q"+index,package:"SEMIO_SOURCE_CLOSURE_"+key,uses:[]})]))});
 const mock=createMockHost(mother),identity=await mock.host.identity();
 const exchange=(access:{SEMIO_REMOTE_CACHE_ENDPOINT:string;SEMIO_REMOTE_CACHE_ACCESS:string})=>remoteGenerationExchangeV1({environment:access,workspace:mother,identity:async()=>identity,platform,closures:leaves})!;
 (mock.host as any).exchange=exchange(environment(first));
 await loadGenerationGraphV1(mock.host);
 const root=join(mother,".gen"),sentFile=root+".closures-sent.json";
 expect(JSON.parse(readFileSync(sentFile,"utf8")).keys).toEqual([...keys].sort());
 const second=await serve(),resident=keys.at(-1)!,content=httpRemoteExecutionContentV1({endpoint:second.url}),requests:{method:string;path:string;status:number}[]=[],actionBodies:string[]=[],trees=new Set<string>();
 await content.write("main",{hash:resident,size:payloads.at(-1)!.length},payloads.at(-1)!);
 const proxy=Bun.serve({hostname:"127.0.0.1",port:0,async fetch(request){
  const path=new URL(request.url).pathname;
  if(request.method==="PUT"&&path.includes("/ac/")){
   const bytes=new Uint8Array(await request.clone().arrayBuffer());actionBodies.push(sha(bytes));
   for(const row of decodeProtobufV1(REAPI_V1.actionResult,bytes).output_directories)trees.add(row.tree_digest.hash);
  }
  const response=await fetch(new Request(new URL(path,second.url),request));
  requests.push({method:request.method,path,status:response.status});return response;
 }});
 try{
  const accepted=await exchange({...environment(second),SEMIO_REMOTE_CACHE_ENDPOINT:proxy.url.origin}).publish({root,fs:files,stale:0});
  useOracle(keys,resident,requests,trees);
  expect(actionBodies).toEqual([actionBodies[0]!,actionBodies[0]!]);
  expect(accepted.status).toBe("uploaded");
  expect(accepted.closures?.sent).toBe(2);
  expect(JSON.parse(readFileSync(sentFile,"utf8")).keys).toEqual([...keys].sort());
  for(const key of keys)expect(await content.has("main",key)).toBe(true);
 }finally{await proxy.stop(true);}
});

function useOracle(keys:readonly string[],resident:string,requests:readonly {method:string;path:string;status:number}[],trees:ReadonlySet<string>){
 using db=new Database(":memory:");
 db.exec("CREATE TABLE requests (method TEXT, path TEXT, status INTEGER)");
 const put=db.query("INSERT INTO requests VALUES (?,?,?)");
 for(const row of requests)put.run(row.method,row.path,row.status);
 const actions=db.query("SELECT status FROM requests WHERE method='PUT' AND path LIKE '%/ac/%' ORDER BY rowid").values();
 expect(actions).toEqual([[400],[200]]);
 const repeated=db.query("SELECT path FROM requests WHERE method='PUT' GROUP BY path HAVING COUNT(*)>1 AND path LIKE '%/cas/%'").values();
 expect(repeated).toEqual([]);
 const puts=db.query("SELECT path FROM requests WHERE method='PUT' AND path LIKE '%/cas/%'").values().flat();
 for(const key of keys)expect(puts.includes("/main/cas/"+key)).toBe(key!==resident);
 const firstRefusal=db.query("SELECT rowid FROM requests WHERE method='PUT' AND path LIKE '%/ac/%' ORDER BY rowid").get() as {rowid:number};
 const early=db.query("SELECT path FROM requests WHERE method='HEAD' AND rowid<?").values(firstRefusal.rowid).flat();
 for(const key of keys)expect(early.includes("/main/cas/"+key)).toBe(false);
 const inspected=db.query("SELECT path FROM requests WHERE method='HEAD' AND rowid>?").values(firstRefusal.rowid).flat();
 for(const path of inspected)expect([...keys,...trees].some(key=>path==="/main/cas/"+key)).toBe(true);
}

test("an authenticated HTTP cache that loses a closure Tree before commit qualifies the same head on retry",async()=>{
 const server=await serve({read:"graph-reader",write:"graph-writer"}),workspace=scratch(),root=join(workspace,".gen"),leaves=port(scratch()),payload=Buffer.from('{"files":["tree.ts"]}'),leaf=sha(payload);
 await leaves.put(NAMESPACE,leaf,payload);
 await seed(workspace,{"pkg/a/project.json":JSON.stringify({name:"a",package:"SEMIO_SOURCE_CLOSURE_"+leaf,uses:[]})});
 const mock=createMockHost(workspace),identity=await mock.host.identity(),key=generationExchangeKeyV1(identity,platform,"local");
 await loadGenerationGraphV1(mock.host);
 const statuses:number[]=[],attempts:string[]=[];
 const proxy=Bun.serve({hostname:"127.0.0.1",port:0,async fetch(request){
  const path=new URL(request.url).pathname,isHead=request.method==="PUT"&&path.endsWith("/ac/"+key);
  if(isHead){
   const bytes=new Uint8Array(await request.clone().arrayBuffer());
   attempts.push(sha(bytes));
   if(attempts.length===1){
    const result=decodeProtobufV1(REAPI_V1.actionResult,bytes);
    await server.store.remove("main","cas",result.output_directories[0].tree_digest.hash);
   }
  }
  const response=await fetch(new Request(new URL(path,server.url),request));
  if(isHead)statuses.push(response.status);
  return response;
 }});
 try{
  const content=httpRemoteExecutionContentV1({endpoint:proxy.url.origin,token:"graph-writer"});
  const result=await publishGenerationV1({content,instance:"main",key,root,workspace,platform,files,closures:leaves,stale:0});
  expect(result).toMatchObject({status:"uploaded",closures:{named:1,sent:1,missing:[]}});
  expect(statuses).toEqual([400,200]);
  expect(attempts).toEqual([attempts[0],attempts[0]]);
 }finally{await proxy.stop(true);}
});


test("closure recovery is scoped to the missing memoized digest and preserves the prior head on failure",async()=>{
 const server=await serve(),workspace=scratch(),leaves=port(scratch()),root=join(workspace,".gen"),sentFile=root+".closures-sent.json";
 const payloads=["cached-a.ts","cached-b.ts"].map(file=>Buffer.from(JSON.stringify({files:[file]}))),keys=payloads.map(sha);
 for(const [index,key] of keys.entries())await leaves.put(NAMESPACE,key,payloads[index]!);
 await seed(workspace,{...ringFiles(6),...Object.fromEntries(keys.map((key,index)=>["pkg/q"+index+"/project.json",JSON.stringify({name:"q"+index,package:"SEMIO_SOURCE_CLOSURE_"+key,uses:[]})]))});
 const mock=createMockHost(workspace),identity=await mock.host.identity(),head=generationExchangeKeyV1(identity,platform,"local");
 (mock.host as any).exchange=remoteGenerationExchangeV1({environment:environment(server),workspace,identity:async()=>identity,platform,closures:leaves});
 await loadGenerationGraphV1(mock.host);
 const content=httpRemoteExecutionContentV1({endpoint:server.url}),prior=await content.readActionResult("main",head),memo=readFileSync(sentFile);
 expect(prior).toBeDefined();
 await Bun.sleep(20);
 await applyOps(workspace,[{op:"write",path:"pkg/p3/src/index.ts",content:"export const newer = 9;"}]);
 await loadGenerationGraphV1(createMockHost(workspace).host);
 let mode="",actions=0,reads=0,controller=new AbortController();
 const refused=(hash:string,status=400)=>new Response("Invalid action result: blob "+hash+" is not in the content store",{status});
 const proxy=Bun.serve({hostname:"127.0.0.1",port:0,async fetch(request){
  const path=new URL(request.url).pathname;
  if(request.method==="PUT"&&path.includes("/ac/")){
   actions++;
   const bytes=new Uint8Array(await request.arrayBuffer()),action=decodeProtobufV1(REAPI_V1.actionResult,bytes);
   if(mode==="document"){
    const document=(action.output_files as {path:string;digest:{hash:string}}[]).find(row=>row.path.startsWith("m/"))!;
    return refused(document.digest.hash);
   }
   if(mode==="malformed")return new Response("Invalid action result: malformed wire",{status:400});
   return refused(keys[0]!,mode==="forbidden"?403:400);
  }
  if(mode==="cancelled"&&request.method==="HEAD"&&path.endsWith("/"+keys[1]))controller.abort(Error("cancelled during closure probe"));
  return fetch(new Request(new URL(path,server.url),request));
 }});
 using db=new Database(":memory:");
 const model=db.query(`SELECT CASE WHEN $mode='refused again' THEN 2 ELSE 1 END AS actions,
  CASE WHEN $mode IN ('refused again','corrupt') THEN 1 ELSE 0 END AS reads,
  CASE WHEN $mode IN ('refused again','corrupt','cancelled') THEN 1 ELSE 2 END AS memo`);
 try{
  for(mode of ["document","malformed","forbidden","corrupt","refused again","cancelled"]){
   writeFileSync(sentFile,memo);actions=0;reads=0;controller=new AbortController();
   const publisher=remoteGenerationExchangeV1({environment:{...environment(server),SEMIO_REMOTE_CACHE_ENDPOINT:proxy.url.origin},workspace,identity:async()=>identity,platform,closures:{...leaves,async read(namespace,key,maximumBytes){
    reads++;
    expect(maximumBytes).toBe(payloads[0]!.length);
    return mode==="corrupt"?Buffer.alloc(maximumBytes!,0):leaves.read(namespace,key,maximumBytes);
   }}})!;
   expect((await publisher.publish({root,fs:files,stale:0,signal:controller.signal})).status,mode).toBe("failed");
   expect<unknown>({actions,reads,memo:JSON.parse(readFileSync(sentFile,"utf8")).keys.length},mode).toEqual(model.get({$mode:mode}));
   expect(await content.readActionResult("main",head),mode).toEqual(prior);
  }
 }finally{await proxy.stop(true);}
});

test("a head with a missing global closure is partial and can be upgraded for the same sealed generation",async()=>{
 using oracle=new Database(":memory:");
 const model=oracle.query(`SELECT CASE WHEN $held=1 THEN 'complete' ELSE 'partial' END AS state,
  CASE WHEN $held=1 THEN 'restored' ELSE 'incomplete' END AS restore`);
 const server=await serve();
 for(const count of [0,1]){
  const bytes=Buffer.from(JSON.stringify({files:["global"+count+".ts"]})),key=sha(bytes),token="SEMIO_SOURCE_CLOSURE_"+key;
  const workspace=scratch(),root=join(workspace,".gen"),leaves=port(scratch()),identity=sha("global"+count),implementation=sha("global-implementation");
  const store=createGenerationStoreV1({root,fs:files,holds:async()=>true,resources:{facts:"facts",builder:"builder"}});
  const frontier=createFrontierV1({directories:[],members:[],capturedAt:0});
  const owners=Array.from({length:count},()=>({name:"p",root:"p",kind:"project" as const,node:{name:"p",root:"p",targets:{}},edges:[],targets:[],rows:[]}));
  await store.refresh({identity,implementation,policy:"fresh",membership:async()=>({digest:sha("membership"),frontier,changes:[]}),compute:async()=>({owners,env:{digest:sha("env"),rows:[]},global:{version:1,externalNodes:{shared:{name:"shared",data:{closure:token}}},edges:[],unavailable:[]},replaceTags:[]})});
  const writer=remoteGenerationExchangeV1({environment:environment(server),workspace,identity:async()=>identity,platform,closures:leaves})!;
  const child=scratch(),reader=remoteGenerationExchangeV1({environment:environment(server),workspace:child,identity:async()=>identity,platform,closures:port(scratch())})!;
  const options={root:join(child,".gen"),fs:files,implementation,requireComplete:true as const};
  expect((await writer.publish({root,fs:files,signal:AbortSignal.abort(),stale:0})).status).toBe("failed");
  let manifest:string|undefined;
  for(const held of [0,1]){
   if(held)await leaves.put(NAMESPACE,key,bytes);
   const published=await writer.publish({root,fs:files,stale:0,ifAbsent:true});
   expect(published.status).toBe("uploaded");
   expect(published.closures).toEqual({named:1,sent:held,fetched:0,missing:held?[]:[{key,namespace:NAMESPACE,projects:[]}]});
   if(held)expect(published.head?.manifest).toBe(manifest);
   else manifest=published.head?.manifest;
   const expected=model.get({$held:held}) as {state:string;restore:string};
   expect<unknown>((await reader.look(options)).state).toBe(expected.state);
   expect<unknown>((await reader.restore(options)).status).toBe(expected.restore);
   expect((await writer.publish({root,fs:files,stale:0,ifAbsent:true})).status).toBe("current");
  }
 }
});

test("the closure entry limit maintains partial truth even when missing reports are truncated",async()=>{
 using oracle=new Database(":memory:");
 const model=oracle.query(`SELECT MIN($named,$limit) AS entries, MIN(MAX($named-$limit,0),$reports) AS reported,
  CASE WHEN $named<=$limit THEN 0 WHEN $global=1 THEN NULL ELSE 2 END AS stale`);
 const {maximumClosures,maximumClosureReports}=exchangeSchema.definitions.Policy.const;
 const overflow=maximumClosureReports+1,keys=Array.from({length:maximumClosures+overflow},(_,index)=>index.toString(16).padStart(64,"0"));
 for(const {excess,global} of [{excess:0,global:true},{excess:overflow,global:true},{excess:overflow,global:false}]){
  const workspace=scratch(),root=join(workspace,".gen"),identity=sha("limit-"+excess+"-"+global),implementation=sha("limit-implementation"),key=generationExchangeKeyV1(identity,platform,"local");
  const store=createGenerationStoreV1({root,fs:files,holds:async()=>true,resources:{facts:"facts",builder:"builder"}}),frontier=createFrontierV1({directories:[],members:[],capturedAt:0});
  const named=maximumClosures+excess,tokens=keys.slice(0,named).map(value=>"SEMIO_SOURCE_CLOSURE_"+value);
  const owners=global?[]:["p","q"].map((name,index)=>({name,root:name,kind:"project" as const,node:{name,root:name,targets:{},data:{tokens:index===0?tokens.slice(0,-1):tokens.slice(-1)}},edges:[],targets:[],rows:[]}));
  const externalNodes=global?{shared:{name:"shared",data:{tokens}}}:{};
  await store.refresh({identity,implementation,policy:"fresh",membership:async()=>({digest:sha("membership"),frontier,changes:[]}),compute:async()=>({owners,env:{digest:sha("env"),rows:[]},global:{version:1,externalNodes,edges:[],unavailable:[]},replaceTags:[]})});
  let action:Uint8Array|undefined;
  const trees=new Map<string,Uint8Array>();
  const content={...httpRemoteExecutionContentV1({endpoint:"http://127.0.0.1:1"}),readActionResult:async()=>action,missing:async(_instance:string,rows:readonly {hash:string;size:number;path?:string}[])=>rows.filter(row=>row.path?.startsWith("c/")),write:async(_instance:string,row:{hash:string},bytes:Uint8Array)=>{trees.set(row.hash,bytes);},writeActionResult:async(_instance:string,_key:string,bytes:Uint8Array)=>{action=bytes;}};
  const local=port(scratch()),closures={...local,size:()=>1};
  const published=await publishGenerationV1({content,instance:"main",key,root,workspace,platform,files,closures,stale:0});
  expect(published.status).toBe("uploaded");
  expect(published.closures?.named).toBe(named);
  const result=decodeProtobufV1(REAPI_V1.actionResult,action!),note=JSON.parse(Buffer.from(result.stdout_raw as Uint8Array).toString("utf8"));
  const entries=result.output_directories.reduce((count:number,row:{tree_digest:{hash:string}})=>count+decodeProtobufV1(REAPI_V1.tree,trees.get(row.tree_digest.hash)!).root.files.length,0);
  expect<unknown>({entries,reported:published.closures?.missing.length,stale:note.stale??null}).toEqual(model.get({$named:named,$limit:maximumClosures,$reports:maximumClosureReports,$global:Number(global)}));
  expect(published.closures?.missing.map(row=>row.key)).toEqual(keys.slice(maximumClosures,named).slice(0,maximumClosureReports));
 }
});
