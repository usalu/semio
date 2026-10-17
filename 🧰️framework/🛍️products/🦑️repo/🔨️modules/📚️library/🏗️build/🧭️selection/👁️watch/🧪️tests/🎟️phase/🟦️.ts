import {withBuildTestInvocationV1} from "../../../../🧪️tests/📥️invocation/🟦️.ts";
import {test,expect} from "bun:test";
import {readFile} from "node:fs/promises";
import {createHash} from "node:crypto";
import {EventEmitter} from "node:events";
import {join,resolve} from "node:path";
import {transform} from "esbuild";
import ts from "typescript";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import * as routing from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {compilerAdmissionBudgetV1} from "../../../../../../../../../🔨️modules/🏃️process/🏗️build/🎟️capacity/🟦️.ts";
import {checkResourceLeaseControl,type ResourceLeaseControl} from "../../../../../../../../../🔨️modules/🏃️process/🔒️leases/🟦️.ts";
import {selectBuildProjectsV1,buildOptionV1} from "../../../🟦️.ts";
import * as watchHelpers from "../../🟦️.ts";
import cachePolicy from "../../../../../⚡️caching/🔣️policy.json";
import ownership from "../../../../../🗂️workspaces/🔣️policy.json";
import schema from "./📋️schema.json";
import corpus from "./🔣️.json";

const source=await readFile(new URL("../../../🟦️.ts",import.meta.url),"utf8"),tree=ts.createSourceFile("watch.ts",source,ts.ScriptTarget.Latest,true),declaration=tree.statements.find(node=>ts.isFunctionDeclaration(node)&&node.name?.text==="watchBuildInvocationV1");
if(!declaration||!ts.isFunctionDeclaration(declaration))throw Error("Actual watch owner required");
const text=declaration.getText(tree).replace(/^export /u,"").replace('await import("./👁️watch/🟦️.ts")','await watchHelpersPort()').replace('await import("../📥️caller/🟦️.ts")','await Promise.resolve(childReaderPort)').replace('await import("../📥️caller/📤️child/🟦️.ts")','await Promise.resolve(childRunnerPort)'),code=ts.transpileModule(text,{compilerOptions:{target:ts.ScriptTarget.ES2022,module:ts.ModuleKind.ESNext}}).outputText;
type Row=typeof corpus.cases[number];
type Observation={wireSha256:string;watchers:number;closed:number;commands:number;finite:boolean;drained:boolean;settledBeforeCleanup:boolean;rejected:boolean;scopeRejected:boolean;primary:boolean;parentActive:boolean;lateCommands:number};
const wait=async(predicate:()=>boolean,milliseconds=500)=>{const deadline=performance.now()+milliseconds;while(!predicate()&&performance.now()<deadline)await new Promise(done=>setTimeout(done,2));return predicate();};

/** 🧫️ Executes the actual watcher body with real issued authority and explicit event/execution custody ports. */
async function observe(row:Row):Promise<Observation>{
 const result:Observation={wireSha256:"",watchers:0,closed:0,commands:0,finite:true,drained:false,settledBeforeCleanup:false,rejected:false,scopeRejected:false,primary:false,parentActive:false,lateCommands:0},cleanup=new AbortController(),stop=new AbortController(),reason=Error("watch actual narrower stop"),primary=row.failure==="undefined"?undefined:Object.freeze(Error("watch actual frozen primary"));let watcherRejected=false,watcherPrimary:unknown;
 try{await withBuildTestInvocationV1(async parent=>{result.wireSha256=createHash("sha256").update(JSON.stringify(routing.readIssuedScriptProcessEnvelopeV1(parent))).digest("hex");
  try{await routing.withScriptInvocationSignalV1(parent,cleanup.signal,async original=>routing.withScriptInvocationSignalV1(parent,new AbortController().signal,async foreign=>{
   const base=routing.createIssuedResourceControlV1(row.phase==="foreign"?foreign:original,"watch:actual-phase",0),issued=routing.createRequestedIssuedResourceControlV1(base,row.phase==="expired"?1:row.stop==="deadline-idle"?40:4000,stop.signal),phase=row.phase==="copied"?{...issued}:issued;
   if(row.phase==="cancelled")stop.abort(reason);if(row.phase==="expired")await new Promise(done=>setTimeout(done,4));
   let emit:((event:string,path:string)=>void)|undefined,settled=false,observed:unknown,childRunning=false;
   const watcher=new EventEmitter() as EventEmitter&{close:()=>void};watcher.close=()=>{result.closed++;};
   const execute=async(signal:AbortSignal)=>{result.commands++;childRunning=true;if(row.stop==="child-error"){stop.abort(reason);await new Promise(done=>setTimeout(done,5));result.drained=true;childRunning=false;throw primary;}if(row.stop==="abort-child"||row.stop==="watch-error")await new Promise<void>(done=>{const finish=()=>{signal.removeEventListener("abort",finish);setTimeout(()=>{result.drained=true;childRunning=false;done();},5);};if(signal.aborted)finish();else signal.addEventListener("abort",finish,{once:true});});else childRunning=false;};
   const bindings={...routing,checkResourceLeaseControl,compilerAdmissionBudgetV1,selectBuildProjectsV1,buildOptionV1,cachePolicy,ownership,
    watchHelpersPort:async()=>{if(row.stop==="after-import")stop.abort(reason);return watchHelpers;},
    childReaderPort:{readOwnedBuildInvocationV1:(_root:string,actual:routing.ScriptInvocation)=>routing.requireIssuedScriptInvocationV1(actual)},
    childRunnerPort:{runOwnedBuildChildInvocationV1:async(_root:string,_args:unknown,actual:routing.ScriptInvocation,control:ResourceLeaseControl,_changes:unknown)=>{routing.requireIssuedResourceControlInvocationV1(control,actual);try{routing.requireIssuedResourceControlRefinementV1(phase,control);const remaining=control.remainingMilliseconds();result.finite=remaining!==null&&remaining<=4000;}catch{result.finite=false;}await execute(control.signal);}},
    watch:(_root:string,_options:unknown,callback:typeof emit)=>{result.watchers++;emit=callback;return watcher;},
    runOwnedCommand:async(_program:string,_args:unknown,_root:string,_command:string,budget:number,options:{signal:AbortSignal})=>{const remaining=issued.remainingMilliseconds();result.finite=remaining!==null&&budget>0&&budget<=remaining+5;await execute(options.signal);}
   },run=new Function(...Object.keys(bindings),code+";return watchBuildInvocationV1;")(...Object.values(bindings)),command=row.child==="owned"?["bun","repo","run","app:build"]:["fixture-command"],watching:Promise<void>=run("/watch-fixture",{projects:{app:{root:"app",targets:{}}},dependencies:[]},["watch","--projects=app","--",...command],original,phase,{initial:row.stop!=="deadline-idle"});
   void watching.then(()=>{settled=true;},error=>{settled=true;result.rejected=true;observed=error;watcherRejected=true;watcherPrimary=error;});
   try{
    await wait(()=>settled||result.watchers>0);
    if(row.stop==="deadline-idle")await wait(()=>settled);else if(row.admitted){emit?.("change","app/main.ts");await wait(()=>result.commands>0);if(row.stop==="watch-error")watcher.emit("error",primary);else stop.abort(reason);await wait(()=>settled);}
    else await new Promise(done=>setTimeout(done,10));
    result.settledBeforeCleanup=settled;result.primary=result.rejected&&observed===primary;const before=result.commands;emit?.("change","app/late.ts");await new Promise(done=>setTimeout(done,220));result.lateCommands=result.commands-before;
   }finally{
    cleanup.abort(Error("watch fixture cleanup"));await watching.catch(()=>{});routing.acknowledgeScriptInvocationSignalStopV1(original);
   }
   if(childRunning)throw Error("Watch fixture child escaped drainage");
  }));}finally{result.parentActive=!parent.control.signal.aborted;routing.checkScriptInvocation(parent);}
 });}catch(error){if(!watcherRejected||!Object.is(error,watcherPrimary))throw error;result.scopeRejected=true;}
 return result;
}

test("watch phase corpus is closed and agrees with an independent relational admission oracle",()=>{
 expect(new Ajv({strict:true}).compile(schema)(corpus)).toBe(true);const db=new Database(":memory:");try{for(const row of corpus.cases){const oracle=db.query("select case when ?='issued' and ?!='after-import' then 1 else 0 end admitted").get(row.phase,row.stop) as {admitted:number};expect(Boolean(oracle.admitted)).toBe(row.admitted);}}finally{db.close();}
});
test("watch API requires the actual phase before options",()=>{expect(declaration.parameters.map(parameter=>parameter.name.getText(tree))).toEqual(corpus.parameters);});
for(const row of corpus.cases)test("actual watch phase: "+row.name,async()=>{
 const observed=await observe(row);
 expect(observed.parentActive).toBe(true);expect(observed.watchers).toBe(row.admitted?1:0);expect(observed.closed).toBe(observed.watchers);expect(observed.commands).toBe(row.admitted&&row.stop!=="deadline-idle"?1:0);expect(observed.settledBeforeCleanup).toBe(true);expect(observed.lateCommands).toBe(0);
 if(!row.admitted)expect(observed.rejected).toBe(true);if(row.finite)expect(observed.finite).toBe(true);if(row.drained)expect(observed.drained).toBe(true);if(row.failure!=="none")expect(observed.primary).toBe(true);
},5000);

test("neutral child execution retains the exact finite phase across projection, progress and return",async()=>{
 const source=await readFile(new URL("../../../../📥️caller/📤️child/🟦️.ts",import.meta.url),"utf8"),tree=ts.createSourceFile("child.ts",source,ts.ScriptTarget.Latest,true),find=(name:string)=>{const row=tree.statements.find(node=>ts.isFunctionDeclaration(node)&&node.name?.text===name);if(!row||!ts.isFunctionDeclaration(row))throw Error("Actual child boundary required");expect(row.parameters.map(value=>value.name.getText(tree))).toEqual(corpus.childParameters);return row;},owner=find("runOwnedBuildChildInvocationV1");find("projectOwnedBuildChildEnvironmentV1");const code=ts.transpileModule(owner.getText(tree).replace(/^export /u,""),{compilerOptions:{target:ts.ScriptTarget.ES2022}}).outputText,db=new Database(":memory:");
 try{for(const row of corpus.childCases){let executed=0,accepted=false,finite=false,samePhase=false;await withBuildTestInvocationV1(async original=>routing.withScriptInvocationSignalV1(original,new AbortController().signal,async foreign=>{
  const stop=new AbortController(),issued=routing.createIssuedResourceControlV1(row.phase==="foreign"?foreign:original,"watch:child-model",row.phase==="expired"?1:4000,stop.signal),phase=row.phase==="copied"?{...issued}:issued,reason=Error("watch child phase stop");if(row.phase==="expired")await new Promise(done=>setTimeout(done,4));if(row.phase==="cancelled")stop.abort(reason);
  const bindings={...routing,checkResourceLeaseControl,compilerAdmissionBudgetV1,projectOwnedBuildChildEnvironmentV1:async(_root:string,_args:unknown,parent:routing.ScriptInvocation,actual:ResourceLeaseControl)=>{samePhase=parent===original&&actual===phase;await Promise.resolve();if(row.phase==="after-project")stop.abort(reason);return{};},advanceScriptInvocation:async(...args:Parameters<typeof routing.advanceScriptInvocation>)=>{await routing.advanceScriptInvocation(...args);if(row.phase==="after-progress")stop.abort(reason);},runOwnedCommand:async(_program:string,_args:unknown,_root:string,_name:string,budget:number,options:{signal:AbortSignal;onProgress:()=>Promise<void>})=>{executed++;const remaining=issued.remainingMilliseconds();finite=options.signal===phase.signal&&remaining!==null&&budget>0&&budget<=remaining+5;await options.onProgress();if(row.phase==="after-run")stop.abort(reason);}},run=new Function(...Object.keys(bindings),code+";return runOwnedBuildChildInvocationV1;")(...Object.values(bindings));try{await run("/child-fixture",["run","app:build"],original,phase);accepted=true;}catch{}routing.checkScriptInvocation(original);
 }));const oracle=db.query("select case when ?='issued' then 1 else 0 end admitted,case when ? in ('issued','after-run') then 1 else 0 end executed").get(row.phase,row.phase) as {admitted:number;executed:number};expect(Boolean(oracle.admitted)).toBe(row.accepted);expect(oracle.executed).toBe(row.executed);expect(accepted).toBe(row.accepted);expect(executed).toBe(row.executed);if(executed){expect(finite).toBe(true);expect(samePhase).toBe(true);}}}finally{db.close();}
});

test("neutral child projection checks actual phase before reads and after awaited authoring",async()=>{
 const source=await readFile(new URL("../../../../📥️caller/📤️child/🟦️.ts",import.meta.url),"utf8"),tree=ts.createSourceFile("projection.ts",source,ts.ScriptTarget.Latest,true),owner=tree.statements.find(node=>ts.isFunctionDeclaration(node)&&node.name?.text==="projectOwnedBuildChildEnvironmentV1");if(!owner)throw Error("Actual child projection required");
 const text=owner.getText(tree).replace(/^export /u,"").replace('await import("../../🧭️selection/👁️watch/🟦️.ts")','await Promise.resolve(watchHelpers)').replace('await import("../🟦️.ts")','await Promise.resolve(callerPort)').replace('await import("../../🧭️invocation/🟦️.ts")','await Promise.resolve(invocationPort)').replace('await import("../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts")','await Promise.resolve(routing)'),code=ts.transpileModule(text,{compilerOptions:{target:ts.ScriptTarget.ES2022}}).outputText,db=new Database(":memory:");
 try{for(const row of corpus.childCases){let reads=0,projected=false;await withBuildTestInvocationV1(async original=>routing.withScriptInvocationSignalV1(original,new AbortController().signal,async foreign=>{
  const stop=new AbortController(),issued=routing.createIssuedResourceControlV1(row.phase==="foreign"?foreign:original,"watch:projection-model",row.phase==="expired"?1:4000,stop.signal),phase=row.phase==="copied"?{...issued}:issued;if(row.phase==="cancelled")stop.abort(Error("projection phase stop"));if(row.phase==="expired")await new Promise(done=>setTimeout(done,4));const caller={tool:"bazel",buildMode:"debug",native:{network:{offline:true},artifactDirectory:"/projection-fixture"}},bindings={routing,watchHelpers,checkResourceLeaseControl,requireIssuedResourceControlInvocationV1:routing.requireIssuedResourceControlInvocationV1,callerPort:{readOwnedBuildInvocationV1:()=>{reads++;return caller;}},invocationPort:{parseBuildInvocationV1:(args:readonly string[])=>({args})},createBuildBootstrapCapabilitiesV1:async()=>{await Promise.resolve();if(row.phase==="after-project")stop.abort(Error("projection authoring phase stop"));return{native:caller.native};}},run=new Function(...Object.keys(bindings),code+";return projectOwnedBuildChildEnvironmentV1;")(...Object.values(bindings));try{const result=await run("/projection-fixture",["run","app:build"],original,phase,{files:["app/main.ts"],projects:["app"]}),wire=JSON.parse(result.SEMIO_SCRIPT_PROCESS_INVOCATION),before=routing.readIssuedScriptProcessEnvelopeV1(original);expect(wire.policy).toEqual(before.policy);expect(wire.deadlineEpochMilliseconds).toBe(before.deadlineEpochMilliseconds);expect(wire.capabilities.native).toEqual(caller.native);projected=true;}catch{}routing.checkScriptInvocation(original);
 }));const oracle=db.query("select case when ? in ('issued','after-progress','after-run') then 1 else 0 end projected").get(row.phase) as {projected:number};expect(Boolean(oracle.projected)).toBe(row.projected);expect(projected).toBe(row.projected);expect(reads).toBe(row.projected||row.phase==="after-project"?1:0);}}finally{db.close();}
});

test("declared Rust receiver issues one genuine phase and carries it through either branch",async()=>{
 const source=await readFile(new URL("../../../../🕸️graph/🦀️rust/📥️intent/🏃️routing/🟦️.ts",import.meta.url),"utf8"),tree=ts.createSourceFile("rust-routing.ts",source,ts.ScriptTarget.Latest,true),owner=tree.statements.find(node=>ts.isFunctionDeclaration(node)&&node.name?.text==="runDeclaredRustBuildV1");if(!owner)throw Error("Actual declared Rust receiver required");const text=owner.getText(tree).replace(/^export /u,"").replace('await import("../../../../../⚡️caching/📦️artifacts/🏗️native-build/🟦️.ts")','await Promise.resolve(fallbackPort)').replace('await import("../../../../../🏃️process/🦀️cargo/📥️dependencies/🟦️.ts")','await Promise.resolve(networkPort)').replace('await import("../../../../📥️caller/📤️child/🟦️.ts")','await Promise.resolve(childPort)'),implementations=[ts.transpileModule(text,{compilerOptions:{target:ts.ScriptTarget.ES2022}}).outputText,(await transform(text,{loader:"ts",target:"es2022"})).code],receipts:unknown[][]=[];
 for(const code of implementations){const rows:unknown[]=[];for(const branch of corpus.receiverCases)await withBuildTestInvocationV1(async original=>{
  let issued=0,consumed=0,actual:ResourceLeaseControl|undefined;const carry=(parent:routing.ScriptInvocation,phase:ResourceLeaseControl)=>{expect(parent).toBe(original);if(!actual)throw Error("Receiver issuance required before consumption");expect(phase).toBe(actual);routing.requireIssuedResourceControlInvocationV1(phase,original);checkResourceLeaseControl(phase);consumed++;},project="@semio-tech/repo-fixture-rs",bindings={...routing,checkResourceLeaseControl,join,resolve,createIssuedResourceControlV1:(parent:routing.ScriptInvocation,command:string,maximum:number)=>{expect(maximum).toBe(0);issued++;actual=routing.createIssuedResourceControlV1(parent,command,maximum);return actual;},lstatSync:()=>({isFile:()=>true,isSymbolicLink:()=>false,nlink:1,size:1}),readFileSync:()=>JSON.stringify({name:project,targets:{}}),readRustBuildIntentV1:()=>({manifest:"Cargo.toml"}),declaredRustBuildFallbackV1:()=>branch==="fallback"?{adapters:[]}:undefined,rustBuildIntentArgumentsV1:()=>[],declaredRustBuildArgumentsV1:()=>["run",project+":build"],verifyCapturedCargoArtifactsV1:()=>{},fallbackPort:{buildRepositoryCargoArtifacts:async(_manifest:string,_args:unknown,_root:string,options:{invocation:routing.ScriptInvocation;control:ResourceLeaseControl})=>carry(options.invocation,options.control)},networkPort:{repositoryCargoNetworkV1:()=>({offline:true})},childPort:{runOwnedBuildChildInvocationV1:async(_root:string,_args:unknown,parent:routing.ScriptInvocation,phase:ResourceLeaseControl)=>carry(parent,phase)}},run=new Function(...Object.keys(bindings),code+";return runDeclaredRustBuildV1;")(...Object.values(bindings));await run("/rust-fixture",project,"/rust-fixture",original);expect(issued).toBe(1);expect(consumed).toBe(1);rows.push({branch,issued,consumed});routing.checkScriptInvocation(original);
 });receipts.push(rows);}expect(receipts[0]).toEqual(receipts[1]);
});
