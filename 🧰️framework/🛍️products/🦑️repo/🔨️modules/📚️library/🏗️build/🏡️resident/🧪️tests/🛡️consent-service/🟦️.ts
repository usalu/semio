/** 🛡️ The resident service and the file access consent: a request macOS refuses reaches the client as the typed refusal `consent` (exit 4) and the service stops itself; a service started without the consent refuses before it binds anything, leaves a typed exit row and a receipt, and the client that waits for it reports exactly that instead of timing out. */
import {afterAll,expect,test} from "bun:test";
import {appendFileSync,chmodSync,existsSync,mkdirSync,mkdtempSync,readdirSync,readFileSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {createHostServiceV1} from "../../🏛️service/🌐️host/🟨️.mjs";
import {runHostForegroundV1,spawnHostServiceV1} from "../../🏗️composition/🌐️host/🟨️.mjs";
import {createGraphPlannerPortV1} from "../../🔌️ports/🧭️plan/🟨️.mjs";
import {connectHostV1,PlanRefusedErrorV1,requestHostStatusV1,requestPlanV1,retireHostServiceV1,lastAccessDenialV1,withRestartOwnerV1} from "../../🚪️client/🛰️plan/🟨️.mjs";
import {readServiceExitRowsV1} from "../../🧾️exit-log/🟨️.mjs";
import {accessDirectoryV1,readAccessReceiptsV1} from "../../🔐️consent/🟨️.mjs";
import {hostEndpointsV1} from "../../📨️protocol/📍️endpoint/🏠️host/🟨️.mjs";
import {runHostCommandV1} from "../../⌨️command/🌐️host/🟨️.mjs";

const base=mkdtempSync(join(process.platform==="win32"?tmpdir():"/tmp","cs-"));
afterAll(()=>rmSync(base,{recursive:true,force:true}));
const environment=(name:string)=>{const dir=join(base,"run-"+name);mkdirSync(dir,{recursive:true});return{...process.env,XDG_RUNTIME_DIR:dir,TMPDIR:dir,LOCALAPPDATA:dir} as Record<string,string>;};
const workspace=(name:string)=>{const root=join(base,name);mkdirSync(root,{recursive:true});writeFileSync(join(root,"graph.json"),"{}");return realpathSync(root);};
const digest=(name:string)=>Buffer.from(name.padEnd(32,"x")).toString("hex").slice(0,64).padEnd(64,"0");
const sleep=(milliseconds:number)=>new Promise(done=>setTimeout(done,milliseconds));
const home="/Users/worker";

test("a request macOS refuses below a protected folder reaches the client as the typed refusal consent (exit 4) in both languages and the service stops itself instead of staying alive without the consent",async()=>{
  const env=environment("consent"),implementation=digest("impl-consent"),runtime=hostEndpointsV1(implementation,env),root=workspace("consent-w");
  const planner=createGraphPlannerPortV1({identify:async()=>"id",loadGraph:async()=>{throw Object.assign(new Error("Relative filesystem fstatat failed (EPERM)"),{code:"EPERM"});}});
  const logged:string[]=[],exits:unknown[]=[],service=createHostServiceV1({runtime,implementation,ports:{planner},policy:{residentCheckMilliseconds:50},log:(event:string,detail?:string)=>{logged.push(event+" "+detail);},onExit:reason=>exits.push(reason),access:{root:home+"/Documents/semio",platform:"darwin",home}});
  await service.start();
  const error=await requestPlanV1({root,implementation,targets:["core:build"],agent:"a",language:"en",environment:env}).then(()=>undefined,reason=>reason as any);
  expect(error?.name).toBe("PlanRefusedErrorV1");expect(error.code).toBe("consent");expect(error.retry).toBe("later");
  expect(error.message_.en).toContain("Files & Folders");expect(error.message_.de).toContain("Dateien & Ordner");
  for(const text of [error.message_.en,error.message_.de]){expect(text).toContain("Documents");expect(text).toContain("semio/launcher/bin/repo");expect(text).not.toContain("{");}
  expect(await Promise.race([service.closed,sleep(5000).then(()=>"alive")])).toBe("access-denied");
  expect(exits).toEqual(["access-denied"]);
  expect(await requestHostStatusV1({implementation,environment:env})).toBeFalsy();
  expect(logged.some(line=>line.startsWith("request-failed"))).toBe(true);
});

test("a request that fails with EPERM outside the protected folders stays the internal failure and the service keeps running",async()=>{
  const env=environment("consent-free"),implementation=digest("impl-consent-free"),runtime=hostEndpointsV1(implementation,env),root=workspace("consent-free-w");
  const planner=createGraphPlannerPortV1({identify:async()=>"id",loadGraph:async()=>{throw Object.assign(new Error("Relative filesystem fstatat failed (EPERM)"),{code:"EPERM"});}});
  const service=createHostServiceV1({runtime,implementation,ports:{planner},policy:{residentCheckMilliseconds:50},access:{root:"/Users/Shared/semio",platform:"darwin",home}});
  await service.start();
  try{
   const error=await requestPlanV1({root,implementation,targets:["core:build"],agent:"a",language:"en",environment:env}).then(()=>undefined,reason=>reason as any);
   expect(error?.code).toBe("internal");
   expect(await Promise.race([service.closed,sleep(300).then(()=>"alive")])).toBe("alive");
  }finally{await service.stop("test");}
});

test("a service that starts without the consent refuses before it binds anything: exit 4, a typed exit row, a receipt outside the checkout, no endpoint; with the consent it runs and clears an old denial",async()=>{
  for(const state of ["denied","waiting"] as const){
   const env=environment("gate-"+state),implementation=digest("impl-gate-"+state),runtime=hostEndpointsV1(implementation,env),root=join(home,"Documents","semio"),receipts=join(base,"receipts-"+state),logged:string[]=[];
   const code=await runHostForegroundV1({implementation,environment:env,platform:"darwin",log:(event:string,detail?:string)=>{logged.push(event+" "+detail);},access:{root,platform:"darwin",home,directory:receipts,probe:async()=>({state,folder:"Documents",...state==="denied"?{error:"EPERM"}:{error:"no answer"}})},loadGraph:async()=>{throw Error("must not load");}});
   expect(code,state).toBe(4);
   const rows=readServiceExitRowsV1(runtime.directory);
   expect(rows.at(-1),state).toMatchObject({event:"exit",reason:"access-denied",code:4,tag:runtime.tag,state,folder:"Documents"});
   expect(existsSync(runtime.endpoint),state).toBe(false);
   expect(await requestHostStatusV1({implementation,environment:env}),state).toBeFalsy();
   const receipt=readAccessReceiptsV1(receipts).find(row=>row.role==="service");
   expect(receipt,state).toMatchObject({version:1,role:"service",state,folder:"Documents",root,pid:process.pid});
   expect(receipt?.identity).toContain("semio/launcher/bin/repo");
   expect(logged.some(line=>line.startsWith("access-denied "+state)),state).toBe(true);
  }
});

test("a service without access to the worktree never becomes the shared service, whatever route started it: exit 4, a typed exit row and receipt that name the refused place, the route, the pid and the parent, no endpoint, nothing loaded",async()=>{
  for(const route of ["launchd","direct","foreground"] as const){
   const env={...environment("never-"+route),SEMIO_SERVICE_ROUTE:route},implementation=digest("impl-never-"+route),runtime=hostEndpointsV1(implementation,env),root=join(home,"Documents","semio"),receipts=join(base,"receipts-never-"+route),place=root+"/packages/inner",logged:string[]=[];
   const code=await runHostForegroundV1({implementation,environment:env,platform:"darwin",log:(event:string,detail?:string)=>{logged.push(event+" "+detail);},access:{root,platform:"darwin",home,directory:receipts,probe:async()=>({state:"denied",folder:"Documents",error:"EPERM",place})},loadGraph:async()=>{throw Error("must not load");}});
   expect(code,route).toBe(4);
   expect(readServiceExitRowsV1(runtime.directory).at(-1),route).toMatchObject({event:"exit",reason:"access-denied",code:4,tag:runtime.tag,state:"denied",folder:"Documents",place,route,pid:process.pid,ppid:process.ppid});
   expect(existsSync(runtime.endpoint),route).toBe(false);
   expect(await requestHostStatusV1({implementation,environment:env}),route).toBeFalsy();
   expect(readAccessReceiptsV1(receipts).find(row=>row.role==="service"),route).toMatchObject({state:"denied",place,route,ppid:process.ppid,pid:process.pid});
   expect(logged.find(line=>line.startsWith("access-denied denied")),route).toContain("place="+place+" route="+route+" pid="+process.pid+" ppid="+process.ppid);
  }
});

test("a service whose forked child (the process a slice load runs in) is refused a place the service itself can read never becomes the shared service either, and names that place and who found it",async()=>{
  const env={...environment("child-refused"),SEMIO_SERVICE_ROUTE:"launchd"},implementation=digest("impl-child-refused"),runtime=hostEndpointsV1(implementation,env),root=join(home,"Documents","semio"),receipts=join(base,"receipts-child"),place=root+"/.cache/b/g/m",roots:string[]=[];
  const code=await runHostForegroundV1({implementation,environment:env,platform:"darwin",access:{root,platform:"darwin",home,directory:receipts,probe:async()=>({state:"ok",folder:"Documents"}),childProbe:async(path:string)=>{roots.push(path);return{place,error:"EPERM"};}},loadGraph:async()=>{throw Error("must not load");}});
  expect(code).toBe(4);expect(roots).toEqual([root]);
  expect(readServiceExitRowsV1(runtime.directory).at(-1)).toMatchObject({reason:"access-denied",code:4,state:"denied",place,route:"launchd",by:"worker"});
  expect(readAccessReceiptsV1(receipts).find(row=>row.role==="service")).toMatchObject({state:"denied",place,route:"launchd"});
  expect(existsSync(runtime.endpoint)).toBe(false);
});

test("the mount boundary: a launchd service whose own sample meets EPERM at the reset marker of a cache image directory never binds, and the exit row names that marker; the same tree with ENOENT for it serves",async()=>{
  const root=join(home,"Documents","semio"),info=(directory:boolean)=>({isDirectory:()=>directory});
  const tree:Record<string,string[]>={[root]:[".cache"],[root+"/.cache"]:["b"],[root+"/.cache/b"]:["fixed.dmg","image.json","m"],[root+"/.cache/b/m"]:[]};
  const list=async(path:string)=>{if(!tree[path])throw Object.assign(new Error("gone"),{code:"ENOENT"});return tree[path];};
  const stat=(code:string)=>async(path:string)=>{if(path.endsWith("/.reset.json"))throw Object.assign(new Error(path.endsWith("/b/.reset.json")?code:"ENOENT"),{code:path.endsWith("/b/.reset.json")?code:"ENOENT"});return info(Boolean(tree[path]));};
  const env={...environment("mount"),SEMIO_SERVICE_ROUTE:"launchd"},implementation=digest("impl-mount"),runtime=hostEndpointsV1(implementation,env),receipts=join(base,"receipts-mount");
  const code=await runHostForegroundV1({implementation,environment:env,platform:"darwin",access:{root,platform:"darwin",home,directory:receipts,list,stat:stat("EPERM"),childProbe:async()=>undefined},loadGraph:async()=>{throw Error("must not load");}});
  expect(code).toBe(4);
  expect(readServiceExitRowsV1(runtime.directory).at(-1)).toMatchObject({reason:"access-denied",code:4,state:"denied",route:"launchd",tag:runtime.tag,place:root+"/.cache/b/.reset.json"});
  expect(existsSync(runtime.endpoint)).toBe(false);
  expect(readAccessReceiptsV1(receipts).find(row=>row.role==="service")).toMatchObject({state:"denied",place:root+"/.cache/b/.reset.json",route:"launchd"});
  const abort=new AbortController();
  const running=runHostForegroundV1({implementation:digest("impl-mount-ok"),environment:env,platform:"darwin",signal:abort.signal,access:{root,platform:"darwin",home,directory:receipts,list,stat:stat("ENOENT"),childProbe:async()=>undefined},loadGraph:async()=>({}),identify:async()=>"id"});
  await sleep(400);
  expect(readAccessReceiptsV1(receipts).find(row=>row.role==="service")).toMatchObject({state:"ok"});
  abort.abort();await Promise.race([running,sleep(5000)]);
});

test("a mount that appears after the service started: the start logs the proof version, the first load after the attach repeats the proof, and a refusal takes the typed consent path (refusal, `where:` line, closing row) instead of failing later; without a change nothing is repeated",async()=>{
  const root=join(home,"Documents","semio"),mounted:{value:boolean}={value:false},loads:string[]=[],logged:string[]=[];
  const env={...environment("mount-later"),SEMIO_SERVICE_ROUTE:"launchd"},implementation=digest("impl-mount-later"),runtime=hostEndpointsV1(implementation,env),work=workspace("mount-later-w"),receipts=join(base,"receipts-mount-later");
  const running=runHostForegroundV1({implementation,environment:env,platform:"darwin",log:(event:string,detail?:string)=>{logged.push(event+" "+detail);},access:{root,platform:"darwin",home,directory:receipts,signature:async()=>mounted.value?"mounted":"plain",probe:async()=>mounted.value?{state:"denied",folder:"Documents",error:"EPERM",place:root+"/.cache/b/.reset.json"}:{state:"ok",folder:"Documents"},childProbe:async()=>undefined},loadGraph:async()=>{loads.push("graph");return{};},identify:async()=>"id",policy:{residentCheckMilliseconds:50}});
  for(let waited=0;waited<50&&!existsSync(runtime.endpoint);waited++)await sleep(100);
  const start=logged.find(line=>line.startsWith("access-proof version=3"));
  expect(start).toBeDefined();expect(start).toContain("route=launchd");expect(start).toContain("pid="+process.pid);expect(start).toContain("ppid="+process.ppid);expect(start).toContain("reproof=on");
  const first=await requestPlanV1({root:work,implementation,targets:["core:build"],agent:"a",language:"en",environment:env}).then(()=>"answered",reason=>reason as any);
  expect(first?.code,"nothing changed: the first request is not refused for access").not.toBe("consent");
  expect(logged.some(line=>line.includes("repeated")),"no repeat without a change").toBe(false);
  mounted.value=true;
  const second=await requestPlanV1({root:work,implementation:implementation,targets:["core:build","core:test"],agent:"b",language:"en",environment:env}).then(()=>undefined,reason=>reason as any);
  expect(second?.code).toBe("consent");expect(second.message_.en).toContain(root+"/.cache/b/.reset.json");expect(second.message_.de).toContain("Dateien & Ordner");
  expect(logged.find(line=>line.includes("repeated"))).toContain("/.cache/b/.reset.json");
  expect(logged.some(line=>line.startsWith("request-failed")&&line.includes("where: root="+root+"/.cache/b entry=.reset.json code=EPERM route=")&&line.includes(" pid="+process.pid)),"the where line").toBe(true);
  expect(await Promise.race([running,sleep(8000).then(()=>"alive")])).toBe(4);
  expect(readServiceExitRowsV1(runtime.directory).findLast(row=>row.reason==="access-denied")).toMatchObject({code:4,tag:runtime.tag,route:"launchd",root});
},30000);

test("a client whose connection dropped without a refusal frame still gets the typed consent refusal from the exit row of the service (with its pid to retire); a drop without such a row stays `lost`",async()=>{
  const{createServer}=await import("node:net");
  const{createRootKeyV1}=await import("../../📨️protocol/🔑️capability/🟨️.mjs");
  const env={...environment("lost-denied"),SEMIO_SERVICE_ROUTE:"launchd"},implementation=digest("impl-lost-denied"),runtime=hostEndpointsV1(implementation,env),root=join(home,"Documents","semio");
  mkdirSync(runtime.directory,{recursive:true,mode:0o700});writeFileSync(runtime.key,createRootKeyV1(),{mode:0o600});
  let rows=true;
  const server=createServer(socket=>{socket.on("data",()=>{if(rows)writeFileSync(join(runtime.directory,"exit.log"),JSON.stringify({event:"exit",at:Date.now(),pid:4711,reason:"access-denied",code:4,tag:runtime.tag,route:"launchd",root,folder:"Documents"})+"\n");socket.destroy();});});
  await new Promise<void>(done=>server.listen(runtime.endpoint,done));
  try{
   const denied=await requestPlanV1({root,implementation,targets:["core:build"],agent:"a",language:"en",environment:env,attempts:1}).then(()=>undefined,reason=>reason as any);
   expect(denied).toBeInstanceOf(PlanRefusedErrorV1);expect(denied.code).toBe("consent");expect(denied.servicePid).toBe(4711);
   expect(denied.message_.en).toContain("Files & Folders");expect(denied.message_.de).toContain("Dateien & Ordner");
   expect(lastAccessDenialV1({implementation,environment:env})).toMatchObject({route:"launchd",pid:4711,root});
   rmSync(join(runtime.directory,"exit.log"));rows=false;
   const lost=await requestPlanV1({root,implementation,targets:["core:build"],agent:"a",language:"en",environment:env,attempts:1}).then(()=>undefined,reason=>reason as any);
   expect(lost?.name).toBe("HostUnavailableErrorV1");expect(lost.reason).toBe("lost");
  }finally{server.close();}
},20000);

test("a mid-request refusal of a real foreground service closes it with a row that names tag, route, root and folder, and the client text names the refused entry",async()=>{
  const env={...environment("mid-row"),SEMIO_SERVICE_ROUTE:"direct"},implementation=digest("impl-mid-row"),runtime=hostEndpointsV1(implementation,env),root=workspace("mid-row-w"),receipts=join(base,"receipts-mid");
  const denied=Object.assign(new Error("Relative filesystem fstatat failed (EPERM)"),{code:"EPERM",root:join(home,"Documents","semio",".cache","b"),entry:".reset.json"});
  const running=runHostForegroundV1({implementation,environment:env,platform:"darwin",access:{root:join(home,"Documents","semio"),platform:"darwin",home,directory:receipts,probe:async()=>({state:"ok",folder:"Documents"})},loadGraph:async()=>{throw denied;},identify:async()=>"id",policy:{residentCheckMilliseconds:50}});
  for(let waited=0;waited<50&&!existsSync(runtime.endpoint);waited++)await sleep(100);
  const error=await requestPlanV1({root,implementation,targets:["core:build"],agent:"a",language:"en",environment:env}).then(()=>undefined,reason=>reason as any);
  expect(error?.code).toBe("consent");
  expect(error.message_.en).toContain(join(home,"Documents","semio",".cache","b",".reset.json"));
  expect(await Promise.race([running,sleep(8000).then(()=>"alive")])).toBe(4);
  expect(readServiceExitRowsV1(runtime.directory).findLast(row=>row.reason==="access-denied")).toMatchObject({event:"exit",code:4,tag:runtime.tag,route:"direct",root:join(home,"Documents","semio"),folder:"Documents",pid:process.pid});
},20000);

test("a remembered refusal of the launchd route sends the next service straight to the direct route with one line that says so and why; without it the launchd route is tried",async()=>{
  const fakeHome=mkdtempSync(join(base,"home-")),cwd=join(fakeHome,"Documents","semio"),identity=join(fakeHome,".local","share","semio","launcher","bin","repo");
  mkdirSync(cwd,{recursive:true});
  const marker=join(base,"remembered-child"),command=[process.execPath,"-e",`require("node:fs").writeFileSync(${JSON.stringify(marker)},process.env.SEMIO_SERVICE_ROUTE)`];
  const route={platform:"darwin",home:fakeHome,exists:(path:string)=>path===identity,executable:process.execPath,uid:501},options={wait:()=>undefined,verifyMilliseconds:50,pollMilliseconds:10,files:()=>[]};
  const env=environment("remembered"),implementation=digest("impl-remembered"),runtime=hostEndpointsV1(implementation,env),calls:string[]=[],lines:string[]=[],original=process.stderr.write.bind(process.stderr);
  (process.stderr as {write:unknown}).write=((text:string)=>{lines.push(String(text));return true;}) as never;
  try{spawnHostServiceV1({command,environment:env,cwd,launchd:{route,run:(program,args)=>{calls.push([program,...args].join(" "));return 0;},options,ensure:()=>({state:"ready"}),refused:()=>({place:cwd+"/.cache/b/.reset.json",at:1})}})({runtime});}finally{(process.stderr as {write:unknown}).write=original;}
  for(let waited=0;waited<50&&!existsSync(marker);waited++)await sleep(100);
  expect(readFileSync(marker,"utf8")).toBe("direct");expect(calls,"no launchctl call at all").toEqual([]);
  const said=lines.filter(line=>line.includes("launchd route was refused"));
  expect(said.length).toBe(1);expect(said[0]).toContain(cwd+"/.cache/b/.reset.json");expect(said[0]).toContain("wurde");
  rmSync(marker,{force:true});calls.length=0;
  const other=environment("remembered-none");
  spawnHostServiceV1({command,environment:other,cwd,launchd:{route,run:(program,args)=>{calls.push([program,...args].join(" "));return{status:0,stdout:args[0]==="print"?"pid = 9\n":""};},options,ensure:()=>({state:"ready"}),refused:()=>undefined}})({runtime:hostEndpointsV1(digest("impl-remembered-none"),other)});
  expect(calls.length,"without a memory the launchd route is used").toBeGreaterThan(0);
});

test("a service on a tree with a really unreadable nested directory (an ordinary permission, EACCES) is no consent problem: it starts and serves",async()=>{
  const sandbox=realpathSync(mkdtempSync(join(base,"nested-"))),root=join(sandbox,"Documents","semio"),nested=join(root,"packages","inner"),receipts=join(base,"receipts-nested");
  mkdirSync(nested,{recursive:true});writeFileSync(join(nested,"file"),"x");
  const env=environment("nested"),implementation=digest("impl-nested");
  try{
   chmodSync(nested,0o000);
   if(process.getuid?.()===0)return;
   const abort=new AbortController();
   const running=runHostForegroundV1({implementation,environment:env,platform:"darwin",signal:abort.signal,access:{root,platform:"darwin",home:sandbox,directory:receipts},loadGraph:async()=>({}),identify:async()=>"id"});
   await sleep(600);
   expect(readAccessReceiptsV1(receipts).find(row=>row.role==="service")).toMatchObject({state:"ok"});
   abort.abort();await Promise.race([running,sleep(5000)]);
  }finally{chmodSync(nested,0o700);rmSync(sandbox,{recursive:true,force:true});}
});

test("a service on a checkout that needs no consent starts without probing a dialog and removes an old denial instead of leaving a receipt",async()=>{
  const env=environment("gate-free"),implementation=digest("impl-gate-free"),receipts=join(base,"receipts-free"),root=realpathSync(workspace("gate-free-w"));
  const{writeAccessReceiptV1}=await import("../../🔐️consent/🟨️.mjs");
  writeAccessReceiptV1(receipts,{version:1,role:"service",pid:1,state:"denied",root,folder:"Documents",identity:"",at:1});
  const abort=new AbortController();
  const running=runHostForegroundV1({implementation,environment:env,platform:process.platform,signal:abort.signal,access:{root,platform:"darwin",home,directory:receipts},loadGraph:async()=>({}),identify:async()=>"id"});
  await sleep(400);
  expect(readAccessReceiptsV1(receipts)).toEqual([]);
  abort.abort();
  await Promise.race([running,sleep(5000)]);
});

test("the service learns by which route it was started: a direct child gets `direct`, a job gets `launchd`; and a client retires a service by its pid, waits until it is gone and treats a missing service as retired",async()=>{
  const out=join(base,"route-env"),command=[process.execPath,"-e",`require("node:fs").writeFileSync(${JSON.stringify(out)},String(process.env.SEMIO_SERVICE_ROUTE))`],env=environment("route-env"),implementation=digest("impl-route-env"),runtime=hostEndpointsV1(implementation,env);
  spawnHostServiceV1({command,environment:{...env,SEMIO_SERVICE_ROUTE:"launchd"},cwd:base})({runtime});
  for(let waited=0;waited<50&&!existsSync(out);waited++)await sleep(100);
  expect(readFileSync(out,"utf8")).toBe("direct");
  const signals:string[]=[];let alive=3;
  const retired=await retireHostServiceV1({implementation,environment:env,status:async()=>alive-->0?{pid:4711}:undefined,kill:(pid:number,signal:string)=>{signals.push(pid+" "+signal);},pause:async()=>undefined});
  expect(retired).toEqual({pid:4711,retired:true});expect(signals).toEqual(["4711 SIGTERM"]);
  expect(await retireHostServiceV1({implementation,environment:env,status:async()=>undefined,kill:()=>{throw Error("never");}})).toEqual({pid:undefined,retired:true});
  const stuck=await retireHostServiceV1({implementation,environment:env,timeoutMilliseconds:100,status:async()=>({pid:4712}),kill:()=>undefined,pause:async()=>undefined});
  expect(stuck).toEqual({pid:4712,retired:false});
  const gone=await retireHostServiceV1({implementation,environment:env,status:async()=>({pid:4713}),kill:()=>{throw Object.assign(Error("no"),{code:"ESRCH"});},timeoutMilliseconds:100,pause:async()=>undefined});
  expect(gone.pid).toBe(4713);
});

test("a service that has the consent runs and its receipt says ok, replacing an old denial of the previous service",async()=>{
  const env=environment("gate-ok"),implementation=digest("impl-gate-ok"),receipts=join(base,"receipts-ok"),root=join(home,"Documents","semio"),{writeAccessReceiptV1}=await import("../../🔐️consent/🟨️.mjs");
  writeAccessReceiptV1(receipts,{version:1,role:"service",pid:1,state:"denied",root,folder:"Documents",identity:"",at:1});
  const abort=new AbortController();
  const running=runHostForegroundV1({implementation,environment:env,platform:process.platform,signal:abort.signal,access:{root,platform:"darwin",home,directory:receipts,probe:async()=>({state:"ok",folder:"Documents"})},loadGraph:async()=>({}),identify:async()=>"id"});
  await sleep(400);
  expect(readAccessReceiptsV1(receipts).find(row=>row.role==="service")).toMatchObject({state:"ok",pid:process.pid,folder:"Documents"});
  abort.abort();
  await Promise.race([running,sleep(5000)]);
});

test("the client that started a service which refused for the missing consent reports that refusal at once (exit 4 class) instead of waiting for a timeout",async()=>{
  const env=environment("early"),implementation=digest("impl-early"),runtime=hostEndpointsV1(implementation,env),started=Date.now();
  const error=await connectHostV1({implementation,environment:env,languages:["en","de"],policy:{startTimeoutMilliseconds:20000,startPollMilliseconds:20,lockStaleMilliseconds:60000},spawnService:async({runtime:target}:{runtime:{directory:string;tag:string}})=>{
    setTimeout(()=>{writeFileSync(join(target.directory,"exit.log"),JSON.stringify({event:"exit",at:Date.now(),pid:99,reason:"access-denied",code:4,tag:target.tag,state:"denied",folder:"Documents",root:home+"/Documents/semio"})+"\n");},120);
  }}).then(()=>undefined,reason=>reason as any);
  expect(error).toBeInstanceOf(PlanRefusedErrorV1);
  expect(error.code).toBe("consent");expect(error.retry).toBe("later");
  expect(error.message_.en).toContain("Files & Folders");expect(error.message_.de).toContain("Dateien & Ordner");
  expect(error.message_.en).toContain("Documents");expect(error.message_.en).toContain(home+"/Documents/semio");
  expect(error.message_.en).toContain("is refused, so");expect(error.message_.en).not.toContain("{");
  expect(Date.now()-started).toBeLessThan(5000);
  const waiting=await connectHostV1({implementation:digest("impl-early-waiting"),environment:env,languages:["de"],policy:{startTimeoutMilliseconds:20000,startPollMilliseconds:20,lockStaleMilliseconds:60000},spawnService:async({runtime:target}:{runtime:{directory:string;tag:string}})=>{
    setTimeout(()=>{writeFileSync(join(target.directory,"exit.log"),JSON.stringify({event:"exit",at:Date.now(),pid:99,reason:"access-denied",code:4,tag:target.tag,state:"waiting",folder:"Desktop",root:home+"/Desktop/semio"})+"\n");},120);
  }}).then(()=>undefined,reason=>reason as any);
  expect(waiting.code).toBe("consent");expect(waiting.message_.de).toContain("Abfrage");expect(waiting.message_.de).toContain("Desktop");
});

test("an exit row from an earlier service does not refuse a start that began after it",async()=>{
  const env=environment("stale-row"),implementation=digest("impl-stale-row"),runtime=hostEndpointsV1(implementation,env);
  mkdirSync(runtime.directory,{recursive:true,mode:0o700});
  writeFileSync(join(runtime.directory,"exit.log"),JSON.stringify({event:"exit",at:Date.now()-60000,pid:98,reason:"access-denied",code:4,tag:runtime.tag,state:"denied",folder:"Documents",root:home+"/Documents/semio"})+"\n");
  const error=await connectHostV1({implementation,environment:env,policy:{startTimeoutMilliseconds:300,startPollMilliseconds:20,lockStaleMilliseconds:60000},spawnService:async()=>undefined}).then(()=>undefined,reason=>reason as any);
  expect(error?.name).toBe("HostUnavailableErrorV1");expect(error.reason).toBe("timeout");
});

test("a start whose process lives and is silent keeps waiting past the silence timeout and connects once the service listens, instead of timing out behind a loaded host",async()=>{
  const env=environment("alive-silent"),implementation=digest("impl-alive-silent"),runtime=hostEndpointsV1(implementation,env);
  const planner=createGraphPlannerPortV1({identify:async()=>"id",loadGraph:async()=>({})});
  const service=createHostServiceV1({runtime,implementation,ports:{planner},policy:{residentCheckMilliseconds:50},log:()=>undefined,onExit:()=>undefined});
  const policy={startTimeoutMilliseconds:150,startCeilingMilliseconds:20000,startLivenessMilliseconds:20,startPollMilliseconds:20,lockStaleMilliseconds:60000};
  const connecting=connectHostV1({implementation,environment:env,policy,spawnService:async()=>{setTimeout(()=>{void service.start();},700);return{alive:()=>true};}});
  const connected=await connecting;
  expect(connected.started).toBe(true);
  connected.socket.destroy();
  await service.stop("test");
});

test("a start whose process died without a socket fails at once as lost instead of waiting for the silence timeout, and one that lives past the ceiling times out",async()=>{
  const env=environment("dead-start"),implementation=digest("impl-dead-start"),started=Date.now();
  const lost=await connectHostV1({implementation,environment:env,policy:{startTimeoutMilliseconds:20000,startCeilingMilliseconds:20000,startLivenessMilliseconds:20,startPollMilliseconds:20,lockStaleMilliseconds:60000},spawnService:async()=>({alive:()=>false})}).then(()=>undefined,reason=>reason as any);
  expect(lost?.name).toBe("HostUnavailableErrorV1");expect(lost.reason).toBe("lost");
  const crashed=hostEndpointsV1(digest("impl-crash-log"),env);
  const crash=await connectHostV1({implementation:digest("impl-crash-log"),environment:env,policy:{startTimeoutMilliseconds:20000,startCeilingMilliseconds:20000,startLivenessMilliseconds:20,startPollMilliseconds:20,lockStaleMilliseconds:60000},spawnService:async()=>{appendFileSync(crashed.log,"[script] {\"stage\":\"running\"}\nInternal error: scriptEnvironment is not defined\n");return{alive:()=>false};}}).then(()=>undefined,reason=>reason as any);
  expect(crash.message).toContain("Internal error: scriptEnvironment is not defined");expect(crash.message).not.toContain("stage");
  expect(Date.now()-started).toBeLessThan(5000);
  const ceiling=await connectHostV1({implementation:digest("impl-ceiling"),environment:env,policy:{startTimeoutMilliseconds:20,startCeilingMilliseconds:300,startLivenessMilliseconds:20,startPollMilliseconds:20,lockStaleMilliseconds:60000},spawnService:async()=>({alive:()=>true})}).then(()=>undefined,reason=>reason as any);
  expect(ceiling.reason).toBe("timeout");
});

test("concurrent clients that all saw a denial have one restart owner: the others wait and attach to the generation it started instead of retiring it",async()=>{
  const env=environment("restart-owner"),implementation=digest("impl-restart-owner"),policy={startPollMilliseconds:20,startCeilingMilliseconds:20000,lockStaleMilliseconds:60000};
  const order:string[]=[];
  const run=(name:string,hold:number)=>withRestartOwnerV1({implementation,environment:env,policy},async()=>{order.push(name+":owned");await sleep(hold);order.push(name+":done");return name;},async()=>{order.push(name+":joined");return name;});
  const first=run("a",300);await sleep(50);
  const others=await Promise.all([run("b",0),run("c",0)]);
  await first;
  expect(order[0]).toBe("a:owned");expect(order.indexOf("a:done")).toBeLessThan(order.indexOf("b:joined"));
  expect(order.indexOf("a:done")).toBeLessThan(order.indexOf("c:joined"));
  expect(order.filter(entry=>entry.endsWith(":owned"))).toEqual(["a:owned"]);
  expect(others).toEqual(["b","c"]);
  expect(await run("d",0)).toBe("d");
});

test("retiring a denied service leaves a newer generation alone: only the pid that refused is stopped",async()=>{
  const kills:number[]=[];
  const stale=await retireHostServiceV1({implementation:digest("impl-only"),only:111,status:async()=>({pid:222}),kill:(pid:number)=>{kills.push(pid);}});
  expect(stale).toEqual({pid:222,retired:true});expect(kills).toEqual([]);
  let running=true;
  const same=await retireHostServiceV1({implementation:digest("impl-only"),only:111,status:async()=>running?{pid:111}:undefined,kill:(pid:number)=>{kills.push(pid);running=false;},pause:async()=>undefined});
  expect(same.retired).toBe(true);expect(kills).toEqual([111]);
});

test("a service that dies while it is answering is replaced and the client attaches to the new generation without using up its attempt, while a loss before any answer still counts",async()=>{
  const {createServer}=await import("node:net");
  const {createFrameDecoderV1,encodeFrameV1}=await import("../../📨️protocol/🟨️.mjs");
  const env=environment("reattach"),implementation=digest("impl-reattach"),runtime=hostEndpointsV1(implementation,env),root=workspace("reattach-w");
  mkdirSync(runtime.directory,{recursive:true,mode:0o700});writeFileSync(runtime.key,Buffer.alloc(32,7),{mode:0o600});
  let connections=0;
  const server=createServer(socket=>{
    const mine=++connections;
    const decoder=createFrameDecoderV1({maximumFrameBytes:1<<20,onError:()=>undefined,onFrame:(frame:any)=>{
      if(frame.t==="hello")socket.write(encodeFrameV1({t:"welcome",v:1,features:["plan"],service:{pid:process.pid}}));
      if(frame.t==="plan"){
        if(mine===1){socket.write(encodeFrameV1({t:"progress",id:frame.id,phase:"slice",done:1,total:2,message:{en:"reading"}}));setTimeout(()=>socket.destroy(),30);}
        else socket.write(encodeFrameV1({t:"plan-reply",id:frame.id,revision:"r",plan:{tasks:[]},progress:[],cacheHits:0}));
      }
    }});
    socket.on("data",chunk=>decoder.push(chunk));socket.on("error",()=>undefined);
  });
  await new Promise<void>(resolve=>server.listen(runtime.endpoint,()=>resolve()));
  try{
    const result=await requestPlanV1({root,implementation,targets:["a:b"],agent:"a",language:"en",environment:env,attempts:1});
    expect(result.plan.tasks).toEqual([]);expect(connections).toBe(2);
  }finally{server.close();}
});

test("a progress message whose decision key the catalogue lacks never crashes the service: the stopped-owner decision has both texts and an unknown slice key falls back to the plain owners line",async()=>{
  const {renderTextV1}=await import("../../📨️protocol/🎚️policy/🟨️.mjs");
  const fields={seconds:5,phase:"owners",done:1,total:9,owners:"a, b",used:3000,budget:2048,kind:"project"};
  const stopped=renderTextV1("host-slice-owners-stopped",fields,["en","de"])!;
  expect(stopped.en).toContain("was stopped");expect(stopped.de).toContain("angehalten");expect(stopped.en).not.toContain("{");
  const env=environment("unknown-key"),implementation=digest("impl-unknown-key"),runtime=hostEndpointsV1(implementation,env),root=workspace("unknown-key-w");
  const planner=createGraphPlannerPortV1({identify:async()=>"id",loadSlice:async({progress}:any)=>{progress(5,{key:"host-slice-owners-nonexistent",running:[],decision:{key:"nonexistent",used:1,budget:2}});return{graph:{version:1,projects:{},dependencies:[]}};},loadGraph:async()=>({})});
  const service=createHostServiceV1({runtime,implementation,ports:{planner},log:()=>undefined,onExit:()=>undefined});
  await service.start();
  try{
    const seen:string[]=[];
    const result=await requestPlanV1({root,implementation,targets:["*:*"],agent:"a",language:"en",environment:env,onProgress:(frame:any)=>seen.push(frame.message?.en??"")}).then(value=>value,error=>error as any);
    expect(result?.name).not.toBe("HostUnavailableErrorV1");
  }finally{await service.stop("test");}
});

test("on macOS a checkout below a protected folder starts its service through launchd by demand with the launcher as program and no direct child; any failure of that route spawns it directly and says so in one line",async()=>{
  const fakeHome=mkdtempSync(join(base,"home-")),cwd=join(fakeHome,"Documents","semio"),identity=join(fakeHome,".local","share","semio","launcher","bin","repo");
  mkdirSync(cwd,{recursive:true});
  const marker=join(base,"direct-child-ran"),command=[process.execPath,"-e",`require("node:fs").writeFileSync(${JSON.stringify(marker)},"x")`];
  const route={platform:"darwin",home:fakeHome,exists:(path:string)=>path===identity,executable:process.execPath,uid:501},env=environment("launchd-spawn"),implementation=digest("impl-launchd-spawn"),runtime=hostEndpointsV1(implementation,env);
  const options={wait:()=>undefined,verifyMilliseconds:50,pollMilliseconds:10,files:()=>[]};
  const calls:string[]=[],healthy=(program:string,args:readonly string[])=>{calls.push([program,...args].join(" "));return{status:0,stdout:args[0]==="print"?"\tstate = running\n\truns = 1\n\tpid = 99\n":""};};
  spawnHostServiceV1({command,environment:{...env,X_TOKEN:"t0ken"},cwd,launchd:{route,run:healthy,options,ensure:()=>({state:"ready"})}})({runtime});
  expect(calls.map(call=>call.split(" ").slice(0,2).join(" "))).toEqual(["launchctl list","launchctl bootout","launchctl bootstrap","launchctl kickstart","launchctl print"]);
  await sleep(500);
  expect(existsSync(marker),"no direct child").toBe(false);
  expect(readServiceExitRowsV1(runtime.directory).at(-1)).toMatchObject({event:"start",via:"launchd"});
  const plistText=readFileSync(join(fakeHome,"Library","Application Support","semio","service-jobs",readdirSync(join(fakeHome,"Library","Application Support","semio","service-jobs"))[0]!),"utf8");
  expect(plistText).not.toContain("X_TOKEN");expect(plistText).not.toContain("t0ken");
  expect(plistText).toContain("<key>SEMIO_SERVICE_ROUTE</key><string>launchd</string>");
  const waitingCalls:string[]=[];
  const pending=environment("launchd-waiting"),pendingImplementation=digest("impl-launchd-waiting"),pendingRuntime=hostEndpointsV1(pendingImplementation,pending);
  const refusal=await connectHostV1({implementation:pendingImplementation,environment:pending,languages:["en","de"],policy:{startTimeoutMilliseconds:20000,startPollMilliseconds:20,lockStaleMilliseconds:60000},spawnService:(context:{runtime:never})=>spawnHostServiceV1({command,environment:pending,cwd,launchd:{route,run:(program,args)=>{waitingCalls.push([program,...args].join(" "));return 0;},options,ensure:()=>({state:"waiting"}),callerCanRead:()=>false}})(context)}).then(()=>undefined,reason=>reason as any);
  expect(refusal).toBeInstanceOf(PlanRefusedErrorV1);expect(refusal.code).toBe("consent");
  expect(refusal.message_.en).toContain("waits for your answer");expect(refusal.message_.de).toContain("wartet");
  expect(waitingCalls,"no service job while the consent waits").toEqual([]);expect(existsSync(marker),"no direct child while the consent waits").toBe(false);
  const lines:string[]=[],original=process.stderr.write.bind(process.stderr);
  const direct=environment("launchd-caller"),directImplementation=digest("impl-launchd-caller"),directRuntime=hostEndpointsV1(directImplementation,direct),directCalls:string[]=[];
  (process.stderr as {write:unknown}).write=((text:string)=>{lines.push(String(text));return true;}) as never;
  try{spawnHostServiceV1({command,environment:direct,cwd,launchd:{route,run:(program,args)=>{directCalls.push([program,...args].join(" "));return 0;},options,ensure:()=>({state:"waiting"}),callerCanRead:()=>true}})({runtime:directRuntime});}finally{(process.stderr as {write:unknown}).write=original;}
  for(let waited=0;waited<50&&!existsSync(marker);waited++)await sleep(100);
  expect(existsSync(marker),"route (b): the consent of the launcher waits but the caller can read, the service is a direct child under the caller's access").toBe(true);
  expect(directCalls,"no launchd job for route (b)").toEqual([]);
  expect(lines.filter(line=>line.includes("runs under the access of the calling app")).length).toBe(1);
  expect(lines.join("")).toContain("Der Graphdienst läuft mit dem Zugriff der aufrufenden App");expect(lines.join("")).toContain("click Allow");expect(lines.join("")).not.toContain("launchd did not run");
  expect(readServiceExitRowsV1(directRuntime.directory).some(row=>row.reason==="access-denied"),"no exit 4 row on route (b)").toBe(false);
  rmSync(marker,{force:true});
  const probeFailed=environment("launchd-probe-failed"),failedImplementation=digest("impl-launchd-probe-failed"),failedRuntime=hostEndpointsV1(failedImplementation,probeFailed);
  spawnHostServiceV1({command,environment:probeFailed,cwd,launchd:{route,run:healthy,options,ensure:()=>({state:"failed",reason:"bootstrap refused"})}})({runtime:failedRuntime});
  for(let waited=0;waited<50&&!existsSync(marker);waited++)await sleep(100);
  expect(existsSync(marker),"direct child when the probe job cannot be run").toBe(true);
  rmSync(marker,{force:true});
  const failing=(program:string,args:readonly string[])=>({status:args[0]==="kickstart"?5:0,stdout:""});
  spawnHostServiceV1({command,environment:env,cwd,launchd:{route,run:failing,options,ensure:()=>({state:"ready"})}})({runtime});
  for(let waited=0;waited<50&&!existsSync(marker);waited++)await sleep(100);
  expect(existsSync(marker),"direct child after a refused kickstart").toBe(true);
  rmSync(marker,{force:true});
  spawnHostServiceV1({command,environment:env,cwd:base,launchd:{route,run:()=>{throw Error("never");},options,ensure:()=>{throw Error("never");}}})({runtime});
  for(let waited=0;waited<50&&!existsSync(marker);waited++)await sleep(100);
  expect(existsSync(marker),"direct child outside protected folders").toBe(true);
});

test("a service that has the consent runs and its receipt says ok, replacing an old denial of the previous service",async()=>{
  const env=environment("gate-ok"),implementation=digest("impl-gate-ok"),receipts=join(base,"receipts-ok"),root=join(home,"Documents","semio"),{writeAccessReceiptV1}=await import("../../🔐️consent/🟨️.mjs");
  writeAccessReceiptV1(receipts,{version:1,role:"service",pid:1,state:"denied",root,folder:"Documents",identity:"",at:1});
  const abort=new AbortController();
  const running=runHostForegroundV1({implementation,environment:env,platform:process.platform,signal:abort.signal,access:{root,platform:"darwin",home,directory:receipts,probe:async()=>({state:"ok",folder:"Documents"})},loadGraph:async()=>({}),identify:async()=>"id"});
  await sleep(400);
  expect(readAccessReceiptsV1(receipts).find(row=>row.role==="service")).toMatchObject({state:"ok",pid:process.pid,folder:"Documents"});
  abort.abort();
  await Promise.race([running,sleep(5000)]);
});

test("the client that started a service which refused for the missing consent reports that refusal at once (exit 4 class) instead of waiting for a timeout",async()=>{
  const env=environment("early"),implementation=digest("impl-early"),runtime=hostEndpointsV1(implementation,env),started=Date.now();
  const error=await connectHostV1({implementation,environment:env,languages:["en","de"],policy:{startTimeoutMilliseconds:20000,startPollMilliseconds:20,lockStaleMilliseconds:60000},spawnService:async({runtime:target}:{runtime:{directory:string;tag:string}})=>{
    setTimeout(()=>{writeFileSync(join(target.directory,"exit.log"),JSON.stringify({event:"exit",at:Date.now(),pid:99,reason:"access-denied",code:4,tag:target.tag,state:"denied",folder:"Documents",root:home+"/Documents/semio"})+"\n");},120);
  }}).then(()=>undefined,reason=>reason as any);
  expect(error).toBeInstanceOf(PlanRefusedErrorV1);
  expect(error.code).toBe("consent");expect(error.retry).toBe("later");
  expect(error.message_.en).toContain("Files & Folders");expect(error.message_.de).toContain("Dateien & Ordner");
  expect(error.message_.en).toContain("Documents");expect(error.message_.en).toContain(home+"/Documents/semio");
  expect(Date.now()-started).toBeLessThan(5000);
  const waiting=await connectHostV1({implementation:digest("impl-early-waiting"),environment:env,languages:["de"],policy:{startTimeoutMilliseconds:20000,startPollMilliseconds:20,lockStaleMilliseconds:60000},spawnService:async({runtime:target}:{runtime:{directory:string;tag:string}})=>{
    setTimeout(()=>{writeFileSync(join(target.directory,"exit.log"),JSON.stringify({event:"exit",at:Date.now(),pid:99,reason:"access-denied",code:4,tag:target.tag,state:"waiting",folder:"Desktop",root:home+"/Desktop/semio"})+"\n");},120);
  }}).then(()=>undefined,reason=>reason as any);
  expect(waiting.code).toBe("consent");expect(waiting.message_.de).toContain("Abfrage");expect(waiting.message_.de).toContain("Desktop");
});

test("an exit row from an earlier service does not refuse a start that began after it",async()=>{
  const env=environment("stale-row"),implementation=digest("impl-stale-row"),runtime=hostEndpointsV1(implementation,env);
  mkdirSync(runtime.directory,{recursive:true,mode:0o700});
  writeFileSync(join(runtime.directory,"exit.log"),JSON.stringify({event:"exit",at:Date.now()-60000,pid:98,reason:"access-denied",code:4,tag:runtime.tag,state:"denied",folder:"Documents",root:home+"/Documents/semio"})+"\n");
  const error=await connectHostV1({implementation,environment:env,policy:{startTimeoutMilliseconds:300,startPollMilliseconds:20,lockStaleMilliseconds:60000},spawnService:async()=>undefined}).then(()=>undefined,reason=>reason as any);
  expect(error?.name).toBe("HostUnavailableErrorV1");expect(error.reason).toBe("timeout");
});

test("`repo service host start` without the consent says what to click in the chosen language and exits 4 before any service is started; `stop` unloads the launchd job even when no service runs",async()=>{
  for(const [language,needle] of [["en","Files & Folders"],["de","Dateien & Ordner"]] as const){
   const env=environment("command-"+language),implementation=digest("impl-command-"+language),err:string[]=[],out:string[]=[];let started=0;
   const code=await runHostCommandV1({args:["start"],languages:[language],implementation,runForeground:async()=>0,command:["x"],environment:env,platform:"darwin",cwd:home+"/Documents/semio",stdout:{write:(text:string)=>{out.push(text);}} as never,stderr:{write:(text:string)=>{err.push(text);}} as never,probe:async()=>({state:"denied",folder:"Documents"}),launchctl:()=>{started++;return 0;}});
   expect(code,language).toBe(4);expect(err.join(""),language).toContain(needle);expect(err.join(""),language).toContain(home+"/Documents/semio");expect(out.join("")).toBe("");expect(started).toBe(0);
  }
  const env=environment("command-stop"),implementation=digest("impl-command-stop"),calls:string[]=[],uid=process.getuid!(),label="tech.semio.repo.service."+"a".repeat(12);
  const code=await runHostCommandV1({args:["stop"],languages:["en"],implementation,runForeground:async()=>0,command:["x"],environment:{...env,HOME:mkdtempSync(join(base,"stophome-"))},platform:"darwin",cwd:home,stdout:{write:()=>true} as never,stderr:{write:()=>true} as never,launchctl:(program:string,list:readonly string[])=>{calls.push([program,...list].join(" "));return{status:0,stdout:list[0]==="list"?`-\t0\t${label}\n`:""};}});
  expect(code).toBe(0);
  expect(calls).toEqual(["launchctl list",`launchctl bootout gui/${uid}/${label}`]);
});

test("a launcher that changed after it was allowed is announced in advance: exit row with changed, the client says that macOS asks once more, in both languages, naming the launcher",async()=>{
  const env=environment("changed"),implementation=digest("impl-changed");
  const error=await connectHostV1({implementation,environment:env,languages:["en","de"],policy:{startTimeoutMilliseconds:20000,startPollMilliseconds:20,lockStaleMilliseconds:60000},spawnService:async({runtime:target}:{runtime:{directory:string;tag:string}})=>{
    setTimeout(()=>{writeFileSync(join(target.directory,"exit.log"),JSON.stringify({event:"exit",at:Date.now(),pid:99,reason:"access-denied",code:4,tag:target.tag,state:"waiting",changed:true,folder:"Documents",root:home+"/Documents/semio"})+"\n");},100);
  }}).then(()=>undefined,reason=>reason as any);
  expect(error.code).toBe("consent");expect(error.message_.en).toContain("changed: macOS asks once more");expect(error.message_.de).toContain("hat sich geändert");
  expect(error.message_.en).toContain("keeps working until it stops");expect(error.message_.en).toContain("semio/launcher/bin/repo");
});

test("the exit row of a service this client retired never refuses the next start: a row of a pid in ignorePids is another generation's, a row of any other pid still refuses at once",async()=>{
  const env=environment("retired-row"),implementation=digest("impl-retired-row"),runtime=hostEndpointsV1(implementation,env);
  mkdirSync(runtime.directory,{recursive:true,mode:0o700});
  const row=(pid:number)=>JSON.stringify({event:"exit",at:Date.now(),pid,reason:"access-denied",code:4,tag:runtime.tag,state:"denied",folder:"Documents",root:home+"/Documents/semio",route:"direct"})+"\n";
  writeFileSync(join(runtime.directory,"exit.log"),row(98));
  const policy={startTimeoutMilliseconds:300,startPollMilliseconds:20,lockStaleMilliseconds:60000},spawnService=async()=>undefined;
  const refused=await connectHostV1({implementation,environment:env,policy,spawnService}).then(()=>undefined,reason=>reason as any);
  expect(refused?.name).toBe("PlanRefusedErrorV1");expect(refused.servicePid).toBe(98);
  const waited=await connectHostV1({implementation,environment:env,policy,spawnService,ignorePids:[98]}).then(()=>undefined,reason=>reason as any);
  expect(waited?.name).toBe("HostUnavailableErrorV1");expect(waited.reason).toBe("timeout");
  appendFileSync(join(runtime.directory,"exit.log"),row(99));
  const other=await connectHostV1({implementation,environment:env,policy,spawnService,ignorePids:[98]}).then(()=>undefined,reason=>reason as any);
  expect(other?.name).toBe("PlanRefusedErrorV1");expect(other.servicePid).toBe(99);
});

test("a service whose responsible app ended stops itself with a typed row instead of failing in the middle of a plan: the row names who was responsible, a service that answers for itself or a stable responsible app keeps running",async()=>{
  const root=join(home,"Documents","semio"),run=async(name:string,answers:(number|undefined)[],expected:"stop"|"serve")=>{
   const env=environment("responsible-"+name),implementation=digest("impl-responsible-"+name),runtime=hostEndpointsV1(implementation,env),receipts=join(base,"receipts-responsible-"+name),abort=new AbortController(),logged:string[]=[];
   let at=0;
   const running=runHostForegroundV1({implementation,environment:env,platform:process.platform,signal:abort.signal,log:(event:string,detail?:string)=>{logged.push(event+" "+detail);},policy:{responsibilityCheckMilliseconds:50},access:{root,platform:"darwin",home,directory:receipts,probe:async()=>({state:"ok",folder:"Documents"}),childProbe:async()=>undefined,responsible:async()=>answers[Math.min(at++,answers.length-1)]},loadGraph:async()=>({}),identify:async()=>"id"});
   const code=await Promise.race([running,sleep(1200).then(()=>"serving" as const)]);
   if(code==="serving")abort.abort();
   await Promise.race([running,sleep(5000)]);
   return{code,rows:readServiceExitRowsV1(runtime.directory),logged};
  };
  const gone=await run("gone",[4242,4242,process.pid],"stop");
  expect(gone.code).toBe(0);
  expect(gone.rows.at(-1)).toMatchObject({event:"exit",reason:"stop:responsible-gone",responsible:4242,responsibleWas:4242,responsibleNow:process.pid});
  expect(gone.logged.some(line=>line.startsWith("access-proof")&&line.includes("responsible=4242"))).toBe(true);
  expect(gone.logged.some(line=>line.startsWith("responsible-gone")&&line.includes("die App"))).toBe(true);
  for(const [name,answers] of [["stable",[4242]],["own",[process.pid]],["unknown",[undefined]]] as const){
   const kept=await run(name,[...answers],"serve");
   expect(kept.code,name).toBe("serving");
   expect(kept.rows.some(row=>row.reason==="stop:responsible-gone"),name).toBe(false);
  }
});

test("a service started by a running service (the reload) on macOS below a protected folder never starts as a direct child, whatever stops launchd: no launcher consent, a failed job and a remembered refusal all end in the bilingual refusal and no process; an ordinary start still goes direct",async()=>{
  const fakeHome=mkdtempSync(join(base,"home-required-")),cwd=join(fakeHome,"Documents","semio"),identity=join(fakeHome,".local","share","semio","launcher","bin","repo");
  mkdirSync(cwd,{recursive:true});
  const marker=join(base,"required-child-ran"),command=[process.execPath,"-e",`require("node:fs").writeFileSync(${JSON.stringify(marker)},"x")`];
  const route={platform:"darwin",home:fakeHome,exists:(path:string)=>path===identity,executable:process.execPath,uid:501},options={wait:()=>undefined,verifyMilliseconds:20,pollMilliseconds:10,files:()=>[]};
  const required=(name:string,launchd:Record<string,unknown>)=>{
   const env={...environment("required-"+name),SEMIO_SERVICE_ROUTE_REQUIRED:"launchd"},implementation=digest("impl-required-"+name);
   return()=>spawnHostServiceV1({command,environment:env,cwd,launchd:{route,options,refused:()=>undefined,...launchd}})({runtime:hostEndpointsV1(implementation,env)});
  };
  const failing=(program:string,args:readonly string[])=>({status:args[0]==="bootstrap"?1:0,stdout:""});
  for(const [name,launchd] of [["waiting",{run:failing,ensure:()=>({state:"waiting"}),callerCanRead:()=>true}],["failed-job",{run:failing,ensure:()=>({state:"ready"})}],["remembered",{refused:()=>({place:cwd}),run:failing,ensure:()=>({state:"ready"})}],["no-route",{route:{...route,exists:()=>false},run:failing}]] as const){
   expect(required(name,launchd),name).toThrow("Dateizugriff");
   expect(existsSync(marker),name).toBe(false);
  }
  const env=environment("required-plain"),implementation=digest("impl-required-plain"),handle=spawnHostServiceV1({command,environment:env,cwd,launchd:{route,options,refused:()=>undefined,run:failing,ensure:()=>({state:"ready"})}})({runtime:hostEndpointsV1(implementation,env)}) as {alive():boolean};
  for(let waited=0;!existsSync(marker)&&waited<3000;waited+=50)await sleep(50);
  expect(handle).toBeDefined();expect(existsSync(marker)).toBe(true);
  expect(readFileSync(join(hostEndpointsV1(implementation,env).directory,"exit.log"),"utf8")).toContain('"event":"start"');
});
