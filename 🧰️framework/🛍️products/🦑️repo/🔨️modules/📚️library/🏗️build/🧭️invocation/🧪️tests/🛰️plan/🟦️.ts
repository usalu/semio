/** 🛰️ Contract: an invocation is a client of the per-host graph service. Language-agnostic JSON fixtures decide what a reply means as a scoped plan and which commands ask the service; an independent Python oracle recomputes the plans; Ajv (third party) validates every scoped plan against the published schema; a real service over a socket answers a real client, whose handed-over slice is verified by digest. */
import {afterAll,expect,test} from "bun:test";
import {mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {spawnSync} from "node:child_process";
import {fileURLToPath} from "node:url";
import Ajv from "ajv";
import scopeSchema from "../../../🧭️selection/🎯️scope/🧬️schema/🔣️.json" with {type:"json"};
import {createHostServiceV1} from "../../../🏡️resident/🏛️service/🌐️host/🟨️.mjs";
import {createGraphPlannerPortV1} from "../../../🏡️resident/🔌️ports/🧭️plan/🟨️.mjs";
import {HostUnavailableErrorV1,PlanRefusedErrorV1,connectHostV1,requestPlanV1} from "../../../🏡️resident/🚪️client/🛰️plan/🟨️.mjs";
import {hostEndpointsV1} from "../../../🏡️resident/📨️protocol/📍️endpoint/🏠️host/🟨️.mjs";
import {HOST_POLICY_V1} from "../../../🏡️resident/📨️protocol/🎚️policy/🟨️.mjs";
import {appendFileSync} from "node:fs";
import {recordPlanSuccessV1} from "../../🏃️run/🟦️.ts";
import {createCasStoreV1} from "../../../🗄️cas/🗃️store/🟦️.ts";
import {createTaskRecorderV1} from "../../../🕸️graph/🧮️derived-nodes/🎯️task-state/🟨️.mjs";
import {workspaceIdV1} from "../../../🏡️resident/📨️protocol/📍️endpoint/🏠️host/🟨️.mjs";
import {isRepoErrorV1,renderRepoErrorV1,repoErrorDataV1} from "../../../../../../../../🔨️modules/🏃️process/🏗️build/🗣️language/🚨️error/🟦️.ts";
import repoErrorSchema from "../../../../../../../../🔨️modules/🏃️process/🏗️build/🗣️language/🚨️error/🧬️schema/🔣️.json" with {type:"json"};
import {reportRepoErrorV1} from "../../../../../../../../🔨️modules/🏃️process/🏗️build/🗣️language/📣️progress/🟦️.ts";
import {PLAN_HANDOFF_V1,invocationPlanRequestV1,planHandoffEnvironmentV1,readSliceV1,requestInvocationPlanV1,scopedPlanOfReplyV1,takePlannedGraphV1} from "../../🛰️plan/🟦️.ts";

const here=fileURLToPath(new URL(".",import.meta.url)),root=realpathSync(fileURLToPath(new URL("../../../../../../../../../",import.meta.url))),fixture=JSON.parse(readFileSync(join(here,"🔣️.json"),"utf8"));
const base=mkdtempSync(join(tmpdir(),"semio-client-plan-"));
const runBase=mkdtempSync(join(process.platform==="win32"?tmpdir():"/tmp","cp-"));
afterAll(()=>{rmSync(base,{recursive:true,force:true});rmSync(runBase,{recursive:true,force:true});});
const pair=(id:string)=>{const split=id.lastIndexOf(":");return{project:id.slice(0,split),target:id.slice(split+1)};};

test("a reply means the scoped plan the fixture and the independent Python oracle state",()=>{
 const oracle=JSON.parse(spawnSync(process.env.SEMIO_PYTHON??"python3",["-I",join(here,"🐍️oracle.py"),join(here,"🔣️.json")],{encoding:"utf8"}).stdout) as unknown[];
 const ajv=new Ajv({strict:false}),validate=ajv.compile({...scopeSchema,$ref:"#/definitions/ScopedPlan"});
 fixture.scoped.forEach((row:any,index:number)=>{
  expect(oracle[index],row.name).toEqual(row.expected);
  const request={arguments:["run"],tool:"nx",targets:row.targets};
  if(row.expected===null){expect(()=>scopedPlanOfReplyV1(request,row.tasks),row.name).toThrow("outside the plan");return;}
  const plan=scopedPlanOfReplyV1(request,row.tasks);
  expect(validate(plan),JSON.stringify(validate.errors)).toBe(true);
  expect(plan.projects).toEqual(row.expected.projects);
  expect(plan.tasks.map((task:any)=>task.project+":"+task.target)).toEqual(row.expected.tasks);
  expect(plan.edges.map((edge:any)=>[edge.from.project+":"+edge.from.target,edge.to.project+":"+edge.to.target])).toEqual(row.expected.edges);
  expect(plan.scope.targets).toEqual(row.expected.scope.map(pair));
 });
});

test("a plan that breaks the closed schema is a typed refusal that names the violated rule, the path and the counts in both languages",()=>{
 const tasks=[{id:"p".repeat(300)+":build",dependsOn:[] as string[],stale:true}],request={arguments:["run"],tool:"nx",targets:[tasks[0]!.id]};
 let caught:unknown;try{scopedPlanOfReplyV1(request,tasks);}catch(error){caught=error;}
 expect(isRepoErrorV1(caught),String(caught)).toBe(true);
 const english=renderRepoErrorV1(caught as never,"en"),german=renderRepoErrorV1(caught as never,"de");
 expect(english).toContain("at most 256 character(s)");expect(english).toContain("/scope/targets/0/project");expect(english).toContain("1 tasks");expect(german).toContain("1 Aufgaben");
});

test("the commands that plan a task graph ask the service, nothing else does",async()=>{
 for(const row of fixture.requests){
  const request=await invocationPlanRequestV1(root,row.args,{SEMIO_BUILD_TOOL:"nx"});
  if(row.expected===null)expect(request,row.args.join(" ")).toBeUndefined();
  else expect(JSON.parse(JSON.stringify(request)),row.args.join(" ")).toMatchObject(row.expected);
 }
 expect(await invocationPlanRequestV1(root,["run","repo-task-scope:test"],{SEMIO_BUILD_REUSE_GRAPH:"true"})).toBeUndefined();
 expect(await invocationPlanRequestV1(root,["run","repo-task-scope:test"],{[PLAN_HANDOFF_V1]:"{}"})).toBeUndefined();
});

const graphOf=(names:readonly string[])=>({version:1,projects:Object.fromEntries(names.map(name=>[name,{name,root:"p/"+name,targets:{build:{execution:{kind:"noop"},dependsOn:names.indexOf(name)>0?["^build"]:[]}}}])),dependencies:names.slice(1).map((name,index)=>({source:name,target:names[index]!}))});

test("a real client gets its plan and verified graph slice from a real service, and a corrupted slice is refused",async()=>{
 const runtimeDirectory=join(runBase,"run");mkdirSync(runtimeDirectory,{recursive:true});
 const environment={...process.env,XDG_RUNTIME_DIR:runtimeDirectory,TMPDIR:runtimeDirectory,LOCALAPPDATA:runtimeDirectory,SEMIO_BUILD_TOOL:"nx"} as Record<string,string>,implementation="a".repeat(64),runtime=hostEndpointsV1(implementation,environment);
 const loads:string[][]=[];
 const planner=createGraphPlannerPortV1({loadSlice:async({arguments:invocation})=>{loads.push([...invocation]);return{graph:graphOf(["core","util","app"]),generationPin:"generation-1"};}});
 const service=createHostServiceV1({runtime,implementation,ports:{planner}});
 await service.start();
 try{
  const progress:string[]=[];
  const plan=await requestInvocationPlanV1({root,request:{arguments:["run","app:build"],tool:"nx",targets:["app:build"]},environment,agent:"agent-1",write:line=>progress.push(line),ports:{requestPlanV1:options=>requestPlanV1({...options,implementation})}});
  expect(plan.scoped.tasks.map(task=>task.project)).toEqual(["core","util","app"]);
  expect(plan.scoped.edges).toHaveLength(2);
  expect(plan.stale).toEqual(["core:build","util:build","app:build"]);
  expect(loads).toEqual([["run","app:build"]]);
  const graph=await takePlannedGraphVia(plan);
  expect(Object.keys(graph.projects)).toEqual(["core","util","app"]);
  const document=readSliceV1(plan.handoff);expect(document.generationPin).toBe("generation-1");
  const other=await requestInvocationPlanV1({root,request:{arguments:["run","app:build"],tool:"nx",targets:["app:build"]},environment,agent:"agent-2",write:()=>undefined,ports:{requestPlanV1:options=>requestPlanV1({...options,implementation})}});
  expect(other.generationKey).toBe(plan.generationKey);expect(other.handoff.hash).toBe(plan.handoff.hash);
  writeFileSync(plan.handoff.path,readFileSync(plan.handoff.path,"utf8").replace("core","c0re"));
  expect(()=>readSliceV1(plan.handoff)).toThrow("does not match its digest");
 }finally{await service.stop("test");}
});

async function takePlannedGraphVia(plan:Awaited<ReturnType<typeof requestInvocationPlanV1>>){return takePlannedGraphV1({...planHandoffEnvironmentV1(plan)});}

test("without a handed-over plan the executing process refuses, in both languages",async()=>{
 await expect(takePlannedGraphV1({})).rejects.toThrow("Ein Plan des Host-Dienstes");
});

test("the start timeout applies only to absent progress: a service that keeps writing its log is waited for, a silent one times out",async()=>{
 const directory=join(runBase,"slow");mkdirSync(directory,{recursive:true});
 const environment={...process.env,XDG_RUNTIME_DIR:directory,TMPDIR:directory,LOCALAPPDATA:directory} as Record<string,string>,implementation="c".repeat(64),runtime=hostEndpointsV1(implementation,environment),policy={...HOST_POLICY_V1,startTimeoutMilliseconds:200,startPollMilliseconds:20};
 const service=createHostServiceV1({runtime,implementation,ports:{planner:createGraphPlannerPortV1({loadSlice:async()=>({graph:graphOf(["a"])})})}});
 try{
  const{socket}=await connectHostV1({implementation,environment,policy,spawnService:()=>{const timer=setInterval(()=>appendFileSync(runtime.log,"booting\n"),50);setTimeout(()=>{clearInterval(timer);void service.start();},700);}});
  socket.destroy();
 }finally{await service.stop("test");}
 const silent={...environment,XDG_RUNTIME_DIR:join(directory,"silent")};mkdirSync(silent.XDG_RUNTIME_DIR!,{recursive:true});
 await expect(connectHostV1({implementation:"d".repeat(64),environment:silent,policy,spawnService:()=>undefined})).rejects.toMatchObject({reason:"timeout"});
});

test("a successful run records the keys of its plan, a dry run, a failed command class and an empty plan record nothing",async()=>{
 const workspace=mkdtempSync(join(base,"ws-")),cas=createCasStoreV1({root:join(base,"cas-record")}),keys={"a:build":"a".repeat(64),"b:build":"b".repeat(64)},recorder=createTaskRecorderV1({directory:join(workspace,".cache/b/g/m/task-keys"),cas}),ask=()=>recorder.upToDate(workspaceIdV1(workspace),new Map(Object.entries(keys)));
 expect(await recordPlanSuccessV1({root:workspace,args:["run","a:build","--dry-run"],keys,cas})).toBe(0);expect(await recordPlanSuccessV1({root:workspace,args:["exec","a"],keys,cas})).toBe(0);expect(await recordPlanSuccessV1({root:workspace,args:["run","a:build"],keys:{},cas})).toBe(0);
 expect([...await ask()]).toEqual([]);
 expect(await recordPlanSuccessV1({root:workspace,args:["run","a:build"],keys,cas})).toBe(2);
 expect([...await ask()].sort()).toEqual(["a:build","b:build"]);
});

test("a refused or unreachable plan ends the command as a localized refusal with a failing exit status, in both languages",async()=>{
 const ajv=new Ajv({strict:true});
 for(const row of fixture.refusals){
  const failure=row.error==="PlanRefusedErrorV1"?new PlanRefusedErrorV1(row.frame):new HostUnavailableErrorV1(row.reason);
  for(const language of["en","de"]){
   const caught=await requestInvocationPlanV1({root,request:{arguments:["run","app:build","--lang="+language],tool:"nx",targets:["app:build"]},environment:{SEMIO_BUILD_TOOL:"nx"},agent:"agent-1",write:()=>undefined,ports:{requestPlanV1:async()=>{throw failure;},spawnHostServiceV1:()=>undefined,probeConsentV1:(()=>{let asked=0;return async()=>({state:asked++===0?"ok":"denied",folder:"Documents"});})()}}).then(()=>undefined,(error:unknown)=>error);
   expect(isRepoErrorV1(caught),row.name).toBe(true);
   expect(ajv.validate(repoErrorSchema,repoErrorDataV1(caught as never)),JSON.stringify(ajv.errors)).toBe(true);
   const lines:string[]=[],codes:number[]=[];
   expect(reportRepoErrorV1(caught,{args:["--lang="+language],environment:{},write:line=>lines.push(line),setExitCode:code=>codes.push(code)}),row.name).toBe(true);
   expect(lines,row.name).toEqual([row.expected[language]]);
   expect(codes,row.name).toEqual([row.expected.exit]);
   expect((caught as {code:string}).code,row.name).toBe(row.expected.code);
  }
 }
});

test("a checkout macOS keeps from being read ends the command before any service is started or asked, with exit status 4 and the click to make in both languages",async()=>{
 const ajv=new Ajv({strict:true});
 for(const state of["denied","waiting"]){
  for(const language of["en","de"]){
   let asked=0,started=0;const receipts:unknown[]=[];
   const caught=await requestInvocationPlanV1({root,request:{arguments:["run","app:build","--lang="+language],tool:"nx",targets:["app:build"]},environment:{SEMIO_BUILD_TOOL:"nx"},agent:"agent-1",write:()=>undefined,ports:{probeConsentV1:async()=>({state,folder:"Documents"}),writeAccessReceiptV1:(_directory:string,receipt:unknown)=>{receipts.push(receipt);},requestPlanV1:async()=>{asked++;throw Error("must not be asked");},spawnHostServiceV1:()=>{started++;}}}).then(()=>undefined,(error:unknown)=>error);
   expect(isRepoErrorV1(caught),state).toBe(true);
   expect(ajv.validate(repoErrorSchema,repoErrorDataV1(caught as never)),JSON.stringify(ajv.errors)).toBe(true);
   const lines:string[]=[],codes:number[]=[];
   expect(reportRepoErrorV1(caught,{args:["--lang="+language],environment:{},write:line=>lines.push(line),setExitCode:code=>codes.push(code)})).toBe(true);
   expect(codes,state+language).toEqual([4]);
   expect(lines.join("\n"),state+language).toContain(language==="en"?"Files & Folders":"Dateien & Ordner");
   expect(lines.join("\n")).toContain("Documents");expect(lines.join("\n")).toContain(root);expect(lines.join("\n")).toContain("repo");
   expect((caught as {code:string}).code).toBe("consent");
   expect([asked,started],state).toEqual([0,0]);
   expect(receipts,state).toEqual([expect.objectContaining({version:1,role:"client",state,folder:"Documents",root,pid:process.pid})]);
  }
 }
});

test("a checkout that can be read goes on to the service, a checkout that needs no consent is not even probed for a dialog",async()=>{
 for(const state of["ok","unprotected"]){
  let asked=0;const written:unknown[]=[];
  await requestInvocationPlanV1({root,request:{arguments:["run","app:build"],tool:"nx",targets:["app:build"]},environment:{SEMIO_BUILD_TOOL:"nx"},agent:"agent-1",write:()=>undefined,ports:{probeConsentV1:async()=>({state,folder:state==="ok"?"Documents":""}),writeAccessReceiptV1:(_directory:string,receipt:unknown)=>{written.push(receipt);},requestPlanV1:async()=>{asked++;throw new HostUnavailableErrorV1("down");},spawnHostServiceV1:()=>undefined}}).then(()=>undefined,()=>undefined);
  expect(asked,state).toBe(1);expect(written.length,state).toBe(state==="ok"?1:0);
 }
});

test("a service that is denied after all is retired and replaced once by a service under the access of the caller, with one line; exit 4 only when the caller itself cannot read the checkout, a second denial with a readable checkout is a failure of its own",async()=>{
 const denied=(text="macOS denied the resident service access")=>new PlanRefusedErrorV1({t:"refused",code:"consent",retry:"later",message:{en:text,de:"macOS hat dem Dienst den Zugriff verweigert"}});
 const failure=new HostUnavailableErrorV1("down");
 const run=async(probes:string[],asks:(Error|"ok")[],language="en")=>{
  const lines:string[]=[],environments:NodeJS.ProcessEnv[]=[],invalidated:string[]=[],order:string[]=[],retired:string[]=[];let asked=0;
  const caught=await requestInvocationPlanV1({root,request:{arguments:["run","app:build","--lang="+language],tool:"nx",targets:["app:build"]},environment:{SEMIO_BUILD_TOOL:"nx"},agent:"agent-1",write:line=>lines.push(line),ports:{probeConsentV1:async()=>{order.push("probe");return{state:probes.shift()??"ok",folder:"Documents"};},writeAccessReceiptV1:()=>undefined,invalidateLauncherMarkerV1:path=>{invalidated.push(path);order.push("invalidate");},retireHostServiceV1:async(options:{implementation:string})=>{retired.push(options.implementation);order.push("retire");return{pid:7,retired:true};},spawnHostServiceV1:(spawnOptions:{environment:NodeJS.ProcessEnv})=>{environments.push(spawnOptions.environment);return undefined;},requestPlanV1:async(askOptions:{spawnService:unknown})=>{void askOptions.spawnService;order.push("ask");const next=asks[asked++]!;if(next==="ok")throw failure;throw next;}}}).then(()=>undefined,(error:unknown)=>error);
  return{caught,lines,environments,invalidated,asked,order,retired};
 };
 const second=await run(["ok","ok"],[denied(),"ok"]);
 expect(second.asked,"the request is asked a second time").toBe(2);
 expect(second.order,"the denied service is retired before the second start").toEqual(["probe","ask","probe","retire","invalidate","ask"]);
 expect(second.retired.length).toBe(1);expect(second.retired[0]).toMatch(/^[0-9a-f]{64}$/u);
 expect(second.invalidated).toEqual([root]);
 expect(second.lines.filter(line=>line.includes("started again under the access of the calling app")).length).toBe(1);
 expect(second.lines.join("\n")).toContain("wird erneut mit dem Zugriff der aufrufenden App gestartet");
 expect(second.environments.length).toBe(2);expect(second.environments[0]!.SEMIO_SERVICE_LAUNCHD).toBeUndefined();expect(second.environments[1]!.SEMIO_SERVICE_LAUNCHD).toBe("off");
 expect((second.caught as {key?:string}|undefined)?.key,"the second attempt ended in the unavailable stub, not in a consent refusal").not.toBe("graph.consent-denied");
 const readable=await run(["ok","ok","ok"],[denied("service text with where"),denied("service text with where")]);
 expect(readable.asked,"only one replacement").toBe(2);expect(readable.retired.length).toBe(1);
 expect((readable.caught as {code:string}).code,"the caller reads the checkout: this is no consent of the caller").toBe("failed");
 expect((readable.caught as {key:string}).key).toBe("build.graph.service-denied");
 const lines:string[]=[],codes:number[]=[];
 expect(reportRepoErrorV1(readable.caught,{args:["--lang=en"],environment:{},write:line=>lines.push(line),setExitCode:code=>codes.push(code)})).toBe(true);
 expect(codes,"not exit 4").not.toEqual([4]);expect(codes[0]).toBeGreaterThan(0);
 expect(lines.join("\n")).toContain("service text with where");expect(lines.join("\n")).toContain("although this process can read it");
 const german:string[]=[];reportRepoErrorV1(readable.caught,{args:["--lang=de"],environment:{},write:line=>german.push(line),setExitCode:()=>undefined});
 expect(german.join("\n")).toContain("obwohl dieser Prozess");
 const final=await run(["ok","ok","denied"],[denied(),denied()]);
 expect(final.asked).toBe(2);expect((final.caught as {code:string}).code,"the caller cannot read either after the second denial: that is the real exit 4").toBe("consent");
 const blind=await run(["ok","denied"],[denied(),"ok"]);
 expect(blind.asked,"the caller cannot read either: no second attempt").toBe(1);expect(blind.invalidated).toEqual([]);expect(blind.retired).toEqual([]);
 expect((blind.caught as {code:string}).code).toBe("consent");
 const exit:string[]=[],codes2:number[]=[];
 expect(reportRepoErrorV1(blind.caught,{args:["--lang=de"],environment:{},write:line=>exit.push(line),setExitCode:code=>codes2.push(code)})).toBe(true);expect(codes2).toEqual([4]);
 const other=await run(["ok"],[failure]);
 expect(other.asked,"only a consent refusal triggers the replacement").toBe(1);expect(other.invalidated).toEqual([]);expect(other.retired).toEqual([]);
});

test("the refusal is remembered for the boot after the denied service is retired (its closing row exists by then): for a launchd service and for a service that left no row, not for a direct or foreground one",async()=>{
 const denied=()=>new PlanRefusedErrorV1({t:"refused",code:"consent",retry:"later",message:{en:"macOS denied the resident service access",de:"macOS hat dem Dienst den Zugriff verweigert"}});
 const run=async(row:{route?:string;place?:string}|undefined)=>{
  const order:string[]=[],remembered:{root:string;place:string}[]=[],invalidated:string[]=[];let asked=0;
  await requestInvocationPlanV1({root,request:{arguments:["run","app:build"],tool:"nx",targets:["app:build"]},environment:{SEMIO_BUILD_TOOL:"nx"},agent:"agent-1",write:()=>undefined,ports:{probeConsentV1:async()=>({state:"ok",folder:"Documents"}),writeAccessReceiptV1:()=>undefined,invalidateLauncherMarkerV1:(forRoot:string)=>{invalidated.push(forRoot);},retireHostServiceV1:async()=>{order.push("retire");return{};},lastAccessDenialV1:()=>{order.push("read-row");return row;},rememberLaunchdRefusalV1:(forRoot:string,options:{place:string})=>{order.push("remember");remembered.push({root:forRoot,place:options.place});},spawnHostServiceV1:()=>undefined,requestPlanV1:async()=>{order.push("ask");if(asked++===0)throw denied();throw new HostUnavailableErrorV1("down");}}}).then(()=>undefined,()=>undefined);
  return{order,remembered,asked,invalidated};
 };
 const launchd=await run({route:"launchd",place:root+"/.cache/b/.reset.json"});
 expect(launchd.remembered).toEqual([{root,place:root+"/.cache/b/.reset.json"}]);
 expect(launchd.order,"the row is read after the retire, the memory is written before the second start").toEqual(["ask","retire","read-row","remember","ask"]);
 expect(launchd.invalidated,"the launcher proof is forgotten when the launcher route was denied").toEqual([root]);
 const direct=await run({route:"direct",place:"x"}),foreground=await run({route:"foreground"});
 expect(direct.remembered).toEqual([]);expect(foreground.remembered).toEqual([]);
 expect(direct.invalidated,"a denied direct service says nothing about the launcher").toEqual([]);expect(foreground.invalidated).toEqual([]);
 const none=await run(undefined);
 expect(none.remembered,"no row: the service lacked access although the caller has it, so the next start goes direct").toEqual([{root,place:root}]);expect(none.asked).toBe(2);expect(none.invalidated).toEqual([root]);
});

test("the second start ignores the exit rows of the services it retired: the denied pid and the pid the retire stopped are handed to the client, the first start hands none",async()=>{
 const denied=Object.assign(new PlanRefusedErrorV1({t:"refused",code:"consent",retry:"later",message:{en:"macOS denied the resident service access",de:"macOS hat dem Dienst den Zugriff verweigert"}}),{servicePid:5});
 const ignored:(readonly number[]|undefined)[]=[];let asked=0;
 await requestInvocationPlanV1({root,request:{arguments:["run","app:build"],tool:"nx",targets:["app:build"]},environment:{SEMIO_BUILD_TOOL:"nx"},agent:"agent-1",write:()=>undefined,ports:{probeConsentV1:async()=>({state:"ok",folder:"Documents"}),writeAccessReceiptV1:()=>undefined,invalidateLauncherMarkerV1:()=>undefined,rememberLaunchdRefusalV1:()=>undefined,retireHostServiceV1:async()=>({pid:7,retired:true}),lastAccessDenialV1:()=>({route:"direct",pid:5}),spawnHostServiceV1:()=>undefined,requestPlanV1:async(options:{ignorePids?:readonly number[]})=>{ignored.push(options.ignorePids);if(asked++===0)throw denied;throw new HostUnavailableErrorV1("down");}}}).then(()=>undefined,()=>undefined);
 expect(ignored).toEqual([undefined,[5,7]]);
});
