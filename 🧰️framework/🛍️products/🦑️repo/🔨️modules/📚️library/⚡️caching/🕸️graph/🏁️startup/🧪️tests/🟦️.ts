/** 🪜️ Nx lock-order laws. The real start orchestrator opens the store and the birth before it takes the exclusive start lease; every declared Nx acquisition sequence (`./🧫️fixtures/🔣️.json`) replays against the authored lock order in TypeScript and in the independent Python model `./🐍️.py` with the same violations, and the order before the fix is flagged; the source facts the inert classes rest on (pins and daemon slots are only ever requested by non-waiting try-locks; the namespace is bound inside the toolkit use) hold in the TypeScript compiler's syntax tree and in the Python scan. */
import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {mkdtempSync,readFileSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join,resolve} from "node:path";
import ts from "typescript";
import authored from "../../../../../../../../🔨️modules/🏃️process/🔒️leases/🧭️order/🔣️policy.json" with {type:"json"};
import fixture from "./🧫️fixtures/🔣️.json" with {type:"json"};
import {testPythonV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {checkHoldAndWaitV1,checkLeaseOrderV1,classifyLeaseResourceV1,readLeaseOrderPolicyV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🧭️order/🟦️.ts";
import {withNxDaemonStartupV1,type NxDaemonStartupStepsV1} from "../🟦️.ts";

type Mode="shared"|"exclusive";
type Step=Readonly<{op:"acquire"|"release";resource:string;mode?:Mode;via?:string;waitedMilliseconds?:number}>;
type Pairs=readonly Readonly<{held:string;requested:string}>[];
type Sequence=Readonly<{id:string;after:boolean;steps:readonly Step[];outOfOrder:Pairs;holdAndWait:Pairs}>;
const sequences=fixture.sequences as unknown as readonly Sequence[],policy=readLeaseOrderPolicyV1(authored),library=resolve(import.meta.dir,"../../../.."),pair=(row:{held:string;requested:string})=>({held:row.held,requested:row.requested});

/** 🔁️ The rules of `beginLeaseOrderV1` on a declared sequence: a shared request for a resource the task already holds shared is re-entrant and unchecked; a waited request meets the hold-and-wait rule. */
const replay=(rows:readonly Step[])=>{
 const held:{directory:string;resource:string;mode:Mode;atMs:number;task:number}[]=[],order:{held:string;requested:string}[]=[],wait:{held:string;requested:string}[]=[];
 rows.forEach((step,index)=>{
  if(step.op==="release"){held.splice(held.findIndex(row=>row.resource===step.resource),1);return;}
  const request={directory:fixture.directory,resource:step.resource,mode:step.mode!,nonblocking:false},reentrant=request.mode==="shared"&&held.some(row=>row.mode==="shared"&&row.resource===request.resource);
  const early=reentrant?undefined:checkLeaseOrderV1(policy,held,request),late=checkHoldAndWaitV1(policy,held,request,step.waitedMilliseconds??0);
  if(early)order.push(pair(early));if(late)wait.push(pair(late));
  held.push({directory:request.directory,resource:request.resource,mode:request.mode,atMs:index+1,task:1});
 });
 return {outOfOrder:order,holdAndWait:wait};
};

const model=(()=>{const run=spawnSync(testPythonV1()??"python3",["-I",resolve(import.meta.dir,"./🐍️.py"),resolve(import.meta.dir,"../../../../../../../../🔨️modules/🏃️process/🔒️leases/🧭️order/🔣️policy.json"),resolve(import.meta.dir,"./🧫️fixtures/🔣️.json"),library],{encoding:"utf8"});if(run.status!==0)throw Error("Python lock order model failed: "+run.stderr);return JSON.parse(run.stdout) as {replays:Record<string,{outOfOrder:Pairs;holdAndWait:Pairs}>;pins:readonly Record<string,unknown>[];slots:readonly Record<string,unknown>[];binding:Record<string,{toolUseBeforeBind:boolean}>};})();

for(const row of sequences){
 test(`Nx acquisition sequence ${row.id}: TypeScript and the Python model find the declared violations`,()=>{
  const found=replay(row.steps);
  expect<unknown>(found).toEqual({outOfOrder:row.outOfOrder,holdAndWait:row.holdAndWait});
  expect<unknown>(model.replays[row.id]).toEqual(found);
  const flagged=found.outOfOrder.length+found.holdAndWait.length;
  if(row.after)expect(flagged).toBe(0);else expect(flagged).toBeGreaterThan(0);
 });
}

test("every Nx lease a declared sequence names is classified, and each class is the one its owner declared",()=>{
 const expected:Record<string,string>={"native-storage:":"image","nx:fixed-cache:":"graph","nx:graph-pool:":"graph","nx:graph-namespace:":"graph","nx:daemon-service:":"graph","nx:daemon-startup:":"daemon-startup","nx:daemon-slot:":"daemon-slot","build-graph:":"pin","nx-tooling-provision":"tool-singleton","nx-tooling-activation":"tool-singleton","nx-tooling:":"tool-singleton","nx-sdk-snapshot:":"tool-singleton","nx-authorizer-":"tool-singleton","artifact:":"artifact","nx:put-gate:":"image","nx:put-slot:":"image"};
 const seen=new Set<string>();
 for(const row of sequences)for(const step of row.steps){const prefix=Object.keys(expected).find(name=>step.resource.startsWith(name));expect(prefix,step.resource).toBeDefined();expect(classifyLeaseResourceV1(policy,step.resource).id,step.resource).toBe(expected[prefix!]);seen.add(prefix!);}
 expect([...seen].sort()).toEqual(Object.keys(expected).sort());
});

/** 🧭️ The real `withNxDaemonStartupV1` runs on recording steps: its order is the order the declared sequence assumes. */
const phase=()=>({signal:new AbortController().signal,remainingMilliseconds:()=>null,publish:()=>undefined,yieldContinuation:()=>Promise.resolve()});
const recording=(trace:string[],failBody=false)=>{
 const resources:string[]=[],steps:NxDaemonStartupStepsV1={
  receive:(_environment,run)=>{trace.push("receive");return run({} as never);},
  admit:()=>{trace.push("admit");return phase();},
  store:async(_original,_control,run)=>{trace.push("store");try{return await run(phase());}finally{trace.push("store:close");}},
  birth:async(_original,_phase,_namespace,_identity,_environment,run)=>{trace.push("birth");try{return await run({BIRTH:"1"},phase());}finally{trace.push("birth:close");}},
  lease:async(_phase,resource,run)=>{trace.push("lease");resources.push(resource);try{return await run();}finally{trace.push("lease:close");}}
 };
 return {steps,resources,body:async()=>{trace.push("body");if(failBody)throw Error("body refused");return "started";}};
};
test("a daemon start opens the store and the birth before it takes the exclusive start lease, and nothing is opened while it is held",async()=>{
 const directory=mkdtempSync(join(tmpdir(),"nxl-")),identity="a".repeat(64),trace:string[]=[],run=recording(trace);
 try{
  expect(await withNxDaemonStartupV1(directory,identity,{},run.body,run.steps)).toBe("started");
  expect(trace).toEqual(["receive","admit","store","birth","lease","body","lease:close","birth:close","store:close"]);
  expect(run.resources).toHaveLength(1);expect(run.resources[0]).toMatch(/^nx:daemon-startup:[a-f0-9]{64}$/u);
  expect(classifyLeaseResourceV1(policy,run.resources[0]!).id).toBe("daemon-startup");
  const declared=(id:string)=>[...new Set(sequences.find(row=>row.id===id)!.steps.flatMap(step=>step.via&&["store","birth","lease"].includes(step.via)?[step.via]:[]))];
  expect(declared("client-start")).toEqual(trace.filter(name=>["store","birth","lease"].includes(name)));
  expect(declared("client-start-lease-first")).not.toEqual(declared("client-start"));
  const below=Math.max(...sequences.find(row=>row.id==="client-start")!.steps.flatMap(step=>step.op==="acquire"&&step.via!=="lease"?[classifyLeaseResourceV1(policy,step.resource).rank]:[]));
  expect(classifyLeaseResourceV1(policy,run.resources[0]!).rank).toBeGreaterThan(below);
 }finally{rmSync(directory,{recursive:true,force:true});}
});
test("a refused body or a malformed toolkit unwinds the start in reverse order and never reaches the lease",async()=>{
 const directory=mkdtempSync(join(tmpdir(),"nxl-")),trace:string[]=[],run=recording(trace,true);
 try{
  await expect(withNxDaemonStartupV1(directory,"a".repeat(64),{},run.body,run.steps)).rejects.toThrow("body refused");
  expect(trace.slice(-4)).toEqual(["body","lease:close","birth:close","store:close"]);
  const quiet:string[]=[];await expect(withNxDaemonStartupV1(directory,"toolkit",{},recording(quiet).body,recording(quiet).steps)).rejects.toThrow("toolkit required");expect(quiet).toEqual([]);
 }finally{rmSync(directory,{recursive:true,force:true});}
});

const parse=(path:string)=>ts.createSourceFile(path,readFileSync(path,"utf8"),ts.ScriptTarget.Latest,true);
const callsOf=(node:ts.Node):ts.CallExpression[]=>{const found:ts.CallExpression[]=[];const visit=(child:ts.Node)=>{if(ts.isCallExpression(child))found.push(child);ts.forEachChild(child,visit);};visit(node);return found;};
/** 🔎️ The mode and timeout of every lease call whose `resource` initializer the filter keeps. */
const leaseRows=(source:ts.SourceFile,keep:(resource:string)=>boolean)=>callsOf(source).filter(call=>["acquireResourceLease","acquireQueuedResourceLease"].includes(call.expression.getText(source))).flatMap(call=>{
 const object=call.arguments[0];if(!object||!ts.isObjectLiteralExpression(object))return [];
 const property=(name:string)=>object.properties.find(row=>ts.isPropertyAssignment(row)&&row.name.getText(source)===name) as ts.PropertyAssignment|undefined;
 if(!keep(property("resource")?.initializer.getText(source)??""))return [];
 return [{mode:JSON.parse(property("mode")!.initializer.getText(source)) as Mode,timeoutMs:property("timeoutMs")?Number(property("timeoutMs")!.initializer.getText(source)):null}];
});
const pinFiles=["⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/🟦️.ts","🏗️build/🕸️graph/🟦️.ts"],slotFile="🕸️graph/🏁️startup/🚏️admission/🟦️.ts";
test("a graph snapshot pin is only ever exclusive as a non-waiting try-lock, so nothing waits behind a pin (the compiler's syntax tree agrees with the Python scan)",()=>{
 const rows=pinFiles.flatMap(name=>leaseRows(parse(join(library,name)),resource=>resource.startsWith("resource(")||resource.startsWith("\"build-graph:\"")).map(row=>({file:name,...row})));
 expect(rows).toEqual([{file:pinFiles[0]!,mode:"exclusive",timeoutMs:0},{file:pinFiles[0]!,mode:"shared",timeoutMs:null},{file:pinFiles[1]!,mode:"shared",timeoutMs:null}]);
 expect(model.pins).toEqual(rows);
 expect(rows.filter(row=>row.mode==="exclusive").every(row=>row.timeoutMs===0)).toBe(true);
});
test("a daemon slot is only ever requested by non-waiting try-locks, so nothing waits for a slot (the compiler's syntax tree agrees with the Python scan)",()=>{
 const rows=leaseRows(parse(join(library,"⚡️caching",slotFile)),()=>true);
 expect(rows).toEqual([{mode:"shared",timeoutMs:0},{mode:"exclusive",timeoutMs:0}]);
 expect(model.slots).toEqual(rows);
});

/** 🪆️ Whether every `bind(...)` call of a function sits inside the callback of its `withNxToolingUseV1(...)` call: the toolkit is used before the namespace is bound. */
const bindsInsideToolUse=(source:ts.SourceFile,name:string):boolean=>{
 const scope=(()=>{let found:ts.Node|undefined;const visit=(node:ts.Node)=>{if((ts.isFunctionDeclaration(node)||ts.isMethodDeclaration(node))&&node.name?.getText(source)===name)found=node;ts.forEachChild(node,visit);};visit(source);return found;})();
 if(!scope)throw Error("function not found: "+name);
 const use=callsOf(scope).find(call=>call.expression.getText(source)==="withNxToolingUseV1"),binds=callsOf(scope).filter(call=>call.expression.getText(source)==="bind");
 if(!use||!binds.length)return false;
 const callback=use.arguments.at(-1)!;
 return binds.every(call=>{for(let node:ts.Node|undefined=call;node;node=node.parent)if(node===callback)return true;return false;});
};
const bindSources=[["🕸️graph/🏁️startup/🚦️service/🟦️.ts","withNxDaemonServiceV1","function withNxDaemonServiceV1"],["🕸️graph/🏁️startup/🚦️service/🟦️.ts","withNxDaemonServiceBirthV1","function withNxDaemonServiceBirthV1"],["🕸️graph/🏁️startup/🚦️service/🟦️.ts","withNxDaemonJournalReaderV1","function withNxDaemonJournalReaderV1"],["🚀️bootstrap/📜️script.ts","runWithPreparedToolingStore","async runWithPreparedToolingStore("]] as const;
for(const [file,name,marker] of bindSources)test(`${name} uses the toolkit before it binds the namespace (syntax tree and Python scan)`,()=>{
 expect(bindsInsideToolUse(parse(join(library,"⚡️caching",file)),name)).toBe(true);
 expect(model.binding[marker]).toEqual({toolUseBeforeBind:true});
});
test("the syntax-tree check tells the two orders apart",()=>{
 const make=(text:string)=>ts.createSourceFile("f.ts",text,ts.ScriptTarget.Latest,true);
 expect(bindsInsideToolUse(make("function f(){return withRepositoryNxStorageV1(a,async(bind,p)=>withNxToolingUseV1(x,y,p,async()=>{const n=await bind(1);return n;}));}"),"f")).toBe(true);
 expect(bindsInsideToolUse(make("function f(){return withRepositoryNxStorageV1(a,async(bind,p)=>{const n=await bind(1);return withNxToolingUseV1(x,y,p,async()=>n);});}"),"f")).toBe(false);
});
