import pLimit from "p-limit";
import {cargoPreparationProgramSourcesV1,assertCargoPreparationObservationCurrentV1} from "../../🛠️preparation/🧾️custody/🟦️.ts";
import {runOwnedCommand} from "@semio-tech/process/owned-execution";
import ts from "typescript";
import {transformSync} from "esbuild";
import {test,expect} from "bun:test";
import Ajv from "ajv";
import {existsSync,mkdirSync,mkdtempSync,readFileSync,writeFileSync} from "node:fs";
import {join} from "node:path";

import {setTimeout as delay} from "node:timers/promises";
import fixture from "../../🧫️fixtures/📥️original-control/🔣️.json";
import schema from "../../🧫️fixtures/📥️original-control/🧬️schema.json";
import {withPreparedCargoDependencyPairV1} from "../../🛠️preparation/🟦️.ts";
import type {CargoPreparationControlV1,CargoWorkspaceProgressV1} from "../../🛠️preparation/🎛️control/🟦️.ts";
import {repositoryCargoPreparationStorageV1} from "../../../../⚡️caching/📦️artifacts/📋️native-orchestration/📥️invocation/🛠️preparation/🟦️.ts";
import {cargoWorkspaceForManifest,prepareCargoOwnersControlledV1} from "../../🟦️.ts";
import {acquireQueuedResourceLease,type ResourceLeaseControl} from "@semio-tech/process/leases";
import {scriptInvocationBudget,createIssuedResourceControlV1,type ScriptInvocation} from "@semio-tech/process/routing";
import {withBuildTestInvocationV1} from "@semio-tech/process/testing/invocation";

type Hooks=Readonly<{advance?:(progress:CargoWorkspaceProgressV1)=>Promise<void>;childBudget?:(requested:number)=>number}>;
const output=():string=>{const directory=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!directory)throw Error("Original preparation fixture artifact directory required");mkdirSync(directory,{recursive:true});return directory;};
const storageIn=(directory:string)=>({...repositoryCargoPreparationStorageV1(process.cwd()),directory:join(directory,"cp"),leaseDirectory:join(directory,"cl")});
const preparationControl=(lease:ResourceLeaseControl,hooks:Hooks={}):CargoPreparationControlV1=>({signal:lease.signal,leaseControl:lease,childBudget:hooks.childBudget??(requested=>Math.min(requested,lease.remainingMilliseconds()??requested)),childOptions:()=>({signal:lease.signal,output:{maximumBytes:4096,maximumLines:20},onProgress:()=>{}}),async advance(progress){await hooks.advance?.(progress);await lease.yieldContinuation();}});
const resourceOf=(root:string)=>`cargo-preparation:${root}`;


for(const mode of fixture.cases)test("original preparation authority closes only its own "+mode+" operation",()=>withBuildTestInvocationV1(async original=>{
 expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);
 const directory=output(),root=mkdtempSync(join(directory,"oc-")),storage=storageIn(directory),controller=new AbortController(),reason=Error("original "+mode);
 const put=(path:string,source:string)=>{mkdirSync(join(root,path,".."),{recursive:true});writeFileSync(join(root,path),source);};
 put("Cargo.toml",'[workspace]\nmembers=["p"]\n[workspace.metadata.semio.repository]\nschema-version=1\nowner-manifests=[]\nmember-manifests=["p/Cargo.toml"]\nexclude-patterns=[]\n');
 put("p/Cargo.toml",'[package]\nname="controlled"\nversion="0.1.0"\n[lib]\npath="🦀️.rs"\n[package.metadata.semio.preparation]\nscript="../📜️script.ts"\ncommand=["publish"]\n');put("p/🦀️.rs","");
 put("📜️script.ts",'import {writeFileSync} from "node:fs";writeFileSync('+JSON.stringify(join(root,"ready"))+',String(process.pid));setInterval(()=>{},1000);');
 const lease=createIssuedResourceControlV1(original,"test:original-preparation",fixture.parentMilliseconds,controller.signal),budgets:number[]=[],owner=cargoWorkspaceForManifest(root,"Cargo.toml");
 let count=0,timer:ReturnType<typeof setInterval>|ReturnType<typeof setTimeout>|undefined,holder:Awaited<ReturnType<typeof acquireQueuedResourceLease>>|undefined,refusal:unknown;
 const control=preparationControl(lease,{childBudget:requested=>{if(mode==="budget")throw reason;const budget=scriptInvocationBudget(original,mode==="deadline"?100:requested);budgets.push(budget);return budget;},async advance(){count++;if(mode==="blocked-progress")return new Promise<void>(()=>{});if(mode==="physical"&&count===fixture.physicalCancelAfter)controller.abort(reason);}});
 try{
  if(mode==="queued"||mode==="blocked-progress"){holder=await acquireQueuedResourceLease({directory:storage.leaseDirectory,resource:resourceOf(root),mode:"exclusive",owner:"independent-holder",signal:new AbortController().signal});timer=setTimeout(()=>controller.abort(reason),fixture.queueMilliseconds);await withPreparedCargoDependencyPairV1(storage,root,"Cargo.toml",lease,async()=>{throw Error("Cancelled queue reached dependency child");},async()=>"fixture-cargo",control);}
  else{if(mode==="child")timer=setInterval(()=>{if(existsSync(join(root,"ready")))controller.abort(reason);},10);await prepareCargoOwnersControlledV1(root,owner,control);}
 }catch(error){refusal=error;}finally{if(timer){clearTimeout(timer);clearInterval(timer);}}
 expect(refusal).toBeDefined();if(mode!=="deadline")expect(refusal).toBe(reason);else expect(String(refusal)).toContain("timeout");
 expect(existsSync(join(root,"done"))).toBe(false);
 if(mode==="child"||mode==="deadline"){expect(existsSync(join(root,"ready"))).toBe(true);const pid=Number(readFileSync(join(root,"ready"),"utf8"));let alive=false;try{process.kill(pid,0);alive=true;}catch{}expect(alive).toBe(false);expect(budgets.every(value=>value>0&&value<=fixture.parentMilliseconds)).toBe(true);}
 else expect(existsSync(join(root,"ready"))).toBe(false);
 if(holder){let acquired=false;const subsequent=acquireQueuedResourceLease({directory:storage.leaseDirectory,resource:resourceOf(root),mode:"exclusive",owner:"next-owner",signal:new AbortController().signal}).then(value=>{acquired=true;return value;});await delay(30);expect(acquired).toBe(false);holder.release();(await subsequent).release();}
 else{const subsequent=await acquireQueuedResourceLease({directory:storage.leaseDirectory,resource:resourceOf(root),mode:"exclusive",owner:"next-owner",signal:new AbortController().signal});subsequent.release();}
 expect(await referenceQueueCancellation()).toEqual(fixture.expected);
}),15000);
test("actual artifact wrappers and repository consumer forward the identical original invocation",async()=>{
 const library=new URL("../../../../🟦️.ts",import.meta.url),wrapper=new URL("../../../../⚡️caching/📦️artifacts/🦀️rust/🟦️.ts",import.meta.url),definitions=(path:URL)=>{const syntax=ts.createSourceFile(path.pathname,readFileSync(path,"utf8"),ts.ScriptTarget.Latest,true);return(name:string)=>syntax.statements.find((node:any)=>node.name?.text===name)!.getText(syntax).replace(/^export /,"");},artifact=definitions(wrapper),repository=definitions(library);
 for(const compile of [(source:string)=>new Bun.Transpiler({loader:"ts"}).transformSync(source),(source:string)=>ts.transpileModule(source,{compilerOptions:{target:ts.ScriptTarget.ES2022,module:ts.ModuleKind.ESNext}}).outputText,(source:string)=>transformSync(source,{loader:"ts",target:"es2022"}).code]){
  const controller=new AbortController(),invocation:ScriptInvocation={policy:{version:1,owner:"artifact-forwarding",maximumElapsedMilliseconds:8000},capabilities:{},control:{signal:controller.signal,remainingMilliseconds:()=>8000,publish(){},async yieldContinuation(){}}},events:string[]=[];
  const bindings={scriptInvocationBudget,checkScriptInvocation:(value:ScriptInvocation)=>expect(value).toBe(invocation),getWorkspaceRoot:()=>"/repo",cargoRepositoryPackageSelections:()=>[{manifest:"owner/Cargo.toml",name:"law-owner"}],cargoWorkspaceForManifest:()=>({directory:"owner"}),relative:()=>"owner/Cargo.toml",join,resolve:(...parts:string[])=>parts.join("/"),repositoryCargoPreparationStorageV1:()=>({version:1,directory:"/repo/output"}),prepareCargoWorkspaceInvocationControlledV1:async(value:ScriptInvocation)=>{expect(value).toBe(invocation);events.push("prepare");},repositoryCargoTestPolicyV1:()=>({assertionBudgets:{quick:100}}),TEST_LEVELS:["quick"],packageTestBudgetMs:()=>100,runCargoTestsV1:async(request:any)=>{expect(request.signal).toBe(controller.signal);expect(request.remainingMilliseconds()).toBe(8000);events.push("assertions");}};
  const consume=new Function(...Object.keys(bindings),compile(repository("runRepositoryCargoTests"))+";return runRepositoryCargoTests;")(...Object.values(bindings));
  const body=artifact("runArtifactRustPackagesTests").replace('await import("../../../🟦️.ts")','repository');
  const packages=new Function("repository","resolveTestLevel","validateNativeCargoArguments","startNativeProgress",compile(body)+";return runArtifactRustPackagesTests;")({runRepositoryCargoTests:consume},(args:string[])=>({rest:args}),()=>{},()=>()=>{});
  const single=new Function("runArtifactRustPackagesTests",compile(artifact("runArtifactRustTests"))+";return runArtifactRustTests;")(packages);
  await single("law-owner","/repo",["--lib"],invocation,["protocol-laws"]);expect(events).toEqual(["prepare","assertions"]);
 }
 console.log("[DEBUG] actual Rust artifact wrappers and Cargo consumer preserve original object through Bun, TypeScript and esbuild implementations");
});
for(const cancelled of fixture.dependencyCases)test("original preparation custody bounds and closes dependency "+cancelled,()=>withBuildTestInvocationV1(async original=>{
 const output=process.env.SEMIO_TEST_ARTIFACT_DIR!;const root=mkdtempSync(join(output,"dc-")),storage=storageIn(output),controller=new AbortController(),reason=Error("original dependency "+cancelled),started=performance.now(),budgets:number[]=[],steps:string[]=[];
 writeFileSync(join(root,"Cargo.toml"),'[workspace]\nmembers=["p"]\n[workspace.metadata.semio.repository]\nschema-version=1\nowner-manifests=[]\nmember-manifests=["p/Cargo.toml"]\nexclude-patterns=[]\n');mkdirSync(join(root,"p"));writeFileSync(join(root,"p/Cargo.toml"),'[package]\nname="dependency-owner"\nversion="0.1.0"\n[lib]\npath="🦀️.rs"\n');writeFileSync(join(root,"p/🦀️.rs"),"");
 const lease=createIssuedResourceControlV1(original,"test:dependency-custody",8000,controller.signal),control=preparationControl(lease,{childBudget:requested=>scriptInvocationBudget(original,requested)});
 let pid=0,completed=false,refusal:unknown;
 try{await withPreparedCargoDependencyPairV1(storage,root,"Cargo.toml",lease,async(args,cwd,signal)=>{steps.push(args[0]);expect(signal).toBe(lease.signal);const budget=control.childBudget(1000);budgets.push(budget);const code=args[0]==="update"?'require("node:fs").writeFileSync("Cargo.lock","version = 4\\n");':'';await runOwnedCommand("node",["-e",code+'console.log("[DEBUG] "+process.pid);'+(args[0]===cancelled?'setInterval(()=>{},1000)':'')],cwd,"original dependency fixture",budget,{signal,stdout:"stderr",output:{maximumBytes:4096,maximumLines:20},onLine:line=>{if(args[0]===cancelled){pid=Number(line.slice("[DEBUG] ".length));controller.abort(reason);}},onProgress:()=>lease.yieldContinuation()});},async()=>"fixture-cargo",control);completed=true;}catch(error){refusal=error;}
 expect(refusal).toBe(reason);expect(completed).toBe(fixture.expected.completed);expect(pid).toBeGreaterThan(0);let alive=false;try{process.kill(pid,0);alive=true;}catch{}expect(alive).toBe(false);expect(steps).toEqual(cancelled==="update"?["update"]:["update","fetch"]);expect(budgets.every(value=>value>0&&value<=1000)).toBe(true);
 const next=await acquireQueuedResourceLease({directory:storage.leaseDirectory,resource:`cargo-preparation:${root}`,mode:"exclusive",owner:"next-dependency-owner",signal:new AbortController().signal});next.release();console.log("[DEBUG] original dependency custody "+JSON.stringify({cancelled,steps,budgets,closed:true,completed,nextOwner:true}));
}),15000);
test("controlled resolver retains exact Unicode closure and refuses changed or cancelled custody",()=>withBuildTestInvocationV1(async original=>{
 const output=process.env.SEMIO_TEST_ARTIFACT_DIR!,root=mkdtempSync(join(output,"rc-")),entry=join(root,"📜️script.ts"),source='import {value as a} from "./ä.ts";import {value as b} from "./\\u00e4.ts";export {value} from "./é.ts";console.log(a,b);';writeFileSync(entry,source);for(const name of["ä.ts","é.ts"])writeFileSync(join(root,name),"export const value=1;\n");
 const controller=new AbortController(),lease=createIssuedResourceControlV1(original,"test:resolver-custody",30000,controller.signal),progress:string[]=[],control=preparationControl(lease,{childBudget:requested=>scriptInvocationBudget(original,requested),async advance(row){progress.push(row.operation);}});
 const program=await cargoPreparationProgramSourcesV1(root,entry,control),syntax=ts.createSourceFile("entry.ts",source,ts.ScriptTarget.Latest,true),oracle=syntax.statements.filter(ts.isImportDeclaration).map(row=>(row.moduleSpecifier as import("typescript").StringLiteral).text).concat(syntax.statements.filter(ts.isExportDeclaration).map(row=>(row.moduleSpecifier as import("typescript").StringLiteral).text));expect(program.resolutions.filter(edge=>edge.source===entry).map(edge=>edge.specifier)).toEqual(oracle);
 const receipt={...program,version:1,root,script:entry,command:["publish"],outputs:[]};await assertCargoPreparationObservationCurrentV1(receipt,control);expect(progress).toContain("observation");writeFileSync(join(root,"ä.ts"),"export const value=2;\n");await expect(assertCargoPreparationObservationCurrentV1(receipt,control)).rejects.toThrow("custody changed");
 const cancelled=new AbortController(),reason=Error("original resolver child cancellation"),cancelControl=preparationControl(createIssuedResourceControlV1(original,"test:resolver-cancel",30000,cancelled.signal),{childBudget:()=>{setTimeout(()=>cancelled.abort(reason),0);return 1000;},async advance(){cancelled.signal.throwIfAborted();}});await expect(cargoPreparationProgramSourcesV1(root,entry,cancelControl)).rejects.toBe(reason);console.log("[DEBUG] controlled resolver "+JSON.stringify({unicodeSpecifiers:oracle,sourceChangeRefused:true,originalCancellation:true,progress:progress.length}));
}),45000);
test("actual original finite process deadline cancels its queued preparation without releasing a peer holder",()=>withBuildTestInvocationV1(async original=>{
 const output=process.env.SEMIO_TEST_ARTIFACT_DIR!,root=mkdtempSync(join(output,"qd-")),storage=storageIn(output),resource=`cargo-preparation:${root}`,holder=await acquireQueuedResourceLease({directory:storage.leaseDirectory,resource,mode:"exclusive",owner:"finite-peer-holder",signal:new AbortController().signal});let originalReason:unknown,refusal:unknown,ran=false;
 try{let lease:ResourceLeaseControl|undefined;try{lease=createIssuedResourceControlV1(original,"test:queued-deadline",fixture.queuedDeadlineMilliseconds);await withPreparedCargoDependencyPairV1(storage,root,"Cargo.toml",lease!,async()=>{ran=true;},async()=>"fixture-cargo",preparationControl(lease!));}catch(error){refusal=error;}expect(String(refusal)).toContain("deadline");expect(refusal).toBe(lease!.signal.reason);expect(ran).toBe(false);let acquired=false;const next=acquireQueuedResourceLease({directory:storage.leaseDirectory,resource,mode:"exclusive",owner:"finite-next-owner",signal:new AbortController().signal}).then(value=>{acquired=true;return value;});await delay(30);expect(acquired).toBe(false);holder.release();(await next).release();console.log("[DEBUG] finite original queued deadline "+JSON.stringify({milliseconds:fixture.queuedDeadlineMilliseconds,originalReason:true,peerRetained:true,nextOwner:true}));}finally{holder.release();}
}),15000);
async function referenceQueueCancellation():Promise<Readonly<{completed:boolean;originalReason:boolean;nextOwner:boolean}>> {
 const limit=pLimit(1),signal=new AbortController(),reason=Error("independent scheduler cancellation");let release!:()=>void,completed=false,nextOwner=false;
 const holder=limit(()=>new Promise<void>(done=>{release=done;}));await new Promise<void>(done=>setImmediate(done));const pending=limit(()=>{signal.signal.throwIfAborted();completed=true;});pending.catch(()=>{});
 const cancelled=new Promise<unknown>(done=>signal.signal.addEventListener("abort",()=>done(signal.signal.reason),{once:true}));signal.abort(reason);const originalReason=await Promise.race([pending,cancelled]);expect(limit.activeCount).toBe(1);const next=limit(()=>{nextOwner=true;});await new Promise<void>(done=>setImmediate(done));expect(nextOwner).toBe(false);release();await holder;await expect(pending).rejects.toBe(reason);await next;return{completed,originalReason:originalReason===reason,nextOwner};
}