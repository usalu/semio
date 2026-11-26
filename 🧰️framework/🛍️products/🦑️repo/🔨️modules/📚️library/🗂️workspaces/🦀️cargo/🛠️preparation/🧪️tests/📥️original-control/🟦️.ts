import {registeredToolExecutableV1} from "../../../../../../../../../🔨️modules/🏃️process/🧰️tools/🏃️run/🟦️.ts";
import {expect,test} from "bun:test";
import Ajv from "ajv";
import {mkdtempSync,mkdirSync,writeFileSync,readFileSync,existsSync,rmSync} from "node:fs";
import {join} from "node:path";
import corpus from "./🔣️.json";
import {PreparationScript,SynchronizeScript} from "../../../../../⚡️caching/📦️artifacts/📋️native-orchestration/📥️invocation/🛠️preparation/🧪️tests/🔬️script-commands/🟦️.ts";
import {acquireQueuedResourceLease} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import {createIssuedResourceControlV1,scriptInvocationEnvironment,withHandledScriptInvocationSignalV1,type ScriptInvocation} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {runOwnedCommand,type OwnedCommandOptions} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import {readNativeCallerAuthorityV1,readNativeOwnerCapabilities} from "../../../../../⚡️caching/📦️artifacts/📋️native-orchestration/📥️invocation/🟦️.ts";
import callerSchema from "../../../../../🏗️build/🧬️schema/📥️caller.json";
import schema from "../../../../../⚡️caching/📦️artifacts/📋️native-orchestration/📥️invocation/🧬️schema/🔣️.json";
import {cargoExecutionSchemaV1 as cargoExecutionSchema} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️execution/🟦️.ts";
import {cargoPreparationStorageFromEnvironmentV1} from "../../📦️storage/🟦️.ts";
import {withCompilerWorkspaceSharedWriteV1} from "../../../../../⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/📏️capacity/🟦️.ts";
import {repositoryCargoPreparationStorageV1,withRepositoryCargoPreparationV1} from "../../../../../⚡️caching/📦️artifacts/📋️native-orchestration/📥️invocation/🛠️preparation/🟦️.ts";
import {privateRepositoryWorkspaceV1,testCheckoutRootV1} from "../../../../../🧪️testing/🧫️private-repository/🟦️.ts";

const artifactRoot=process.env.SEMIO_TEST_ARTIFACT_DIR;
if(!artifactRoot)throw Error("Original default preparation artifact declaration required");
const repository=await privateRepositoryWorkspaceV1(),root=repository.root,storage=repositoryCargoPreparationStorageV1(root),base={...process.env,SEMIO_CARGO_PREPARATION_STORAGE:undefined},ceiling=corpus.callerCeilingMilliseconds,
 ownerGrant=(command:string)=>({version:1 as const,buildMode:"prototype" as const,repositoryRoot:root,codeRoot:testCheckoutRootV1(),artifactDirectory:join(root,"target"),command:{kind:"native-owner-command" as const,manifest:"Cargo.toml",workingDirectory:".",program:process.execPath,arguments:[command,"--manifest","Cargo.toml"]},...corpus.native}),
 clear=()=>{for(const name of ["kernel","Cargo.toml","Cargo.lock","target"])rmSync(join(root,name),{recursive:true,force:true});rmSync(storage.directory,{recursive:true,force:true});},
 author=(directory:string)=>{mkdirSync(join(directory,"kernel"));writeFileSync(join(directory,"Cargo.toml"),corpus.workspace);writeFileSync(join(directory,"kernel/Cargo.toml"),corpus.package);writeFileSync(join(directory,"kernel/lib.rs"),"pub fn original() {}\n");},
 issue=<T,>(owner:string,capabilities:(bootstrap:ScriptInvocation["capabilities"])=>ScriptInvocation["capabilities"],body:(original:ScriptInvocation)=>Promise<T>):Promise<T>=>repository.invoke(body,{owner,maximumElapsedMilliseconds:ceiling,arguments:["show","projects"],capabilities});
type BoundedHooksV1={execute?:(command:string,args:string[],cwd:string,label:string,budget:number,options:OwnedCommandOptions)=>void;capture?:(command:string,args:string[],cwd:string,label:string,budget:number,options:OwnedCommandOptions)=>void;captured?:(output:string)=>void;};
const bounded=<T,>(invocation:ScriptInvocation,hooks:BoundedHooksV1,body:(scripts:{prepare:()=>PreparationScript;synchronize:()=>SynchronizeScript})=>Promise<T>):Promise<T>=>{
 const repositoryRoot=readNativeCallerAuthorityV1(invocation.capabilities).repositoryRoot;
 return withRepositoryCargoPreparationV1(repositoryRoot,root,invocation,createIssuedResourceControlV1(invocation,"native:preparation-entry",0),scriptInvocationEnvironment(invocation,base),async(environment,phase,store,storeControl)=>{
  class BoundedPreparation extends PreparationScript{protected override readonly environment=environment;protected override phase(){return phase;}}
  class BoundedSynchronize extends SynchronizeScript{
   protected override readonly environment=environment;
   protected override phase(){return phase;}
   protected override toolInputs():readonly string[]{return [store.toolchain.cargo];}
   protected override execute(command:string,args:string[],cwd:string,label:string,budget:number,options:OwnedCommandOptions):Promise<void>{hooks.execute?.(command,args,cwd,label,budget,options);return withCompilerWorkspaceSharedWriteV1(store.storage,cargoPreparationStorageFromEnvironmentV1(environment).leaseDirectory,storeControl,()=>super.execute(command,args,cwd,label,budget,options));}
   protected override async capture(command:string,args:string[],cwd:string,label:string,budget:number,options:OwnedCommandOptions):Promise<string>{hooks.capture?.(command,args,cwd,label,budget,options);const output=await super.capture(command,args,cwd,label,budget,options);hooks.captured?.(output);return output;}
  }
  return body({prepare:()=>new BoundedPreparation(root,repositoryRoot,invocation),synchronize:()=>new BoundedSynchronize(root,repositoryRoot,invocation)});
 });
};
const progress=(refuse=false)=>{
 const ledger:string[]=[],write=process.stderr.write,patched=((chunk:string|Uint8Array,...rest:unknown[])=>{const line=typeof chunk==="string"?chunk:Buffer.from(chunk).toString("utf8");if(line.startsWith("[script] {")){if(refuse&&line.includes('"command":"cargo:'))throw Error("Original default preparation publication refused");ledger.push("publish:"+(JSON.parse(line.slice(9)) as {command:string}).command);}return(write as (...input:unknown[])=>boolean).call(process.stderr,chunk,...rest);}) as typeof process.stderr.write;
 process.stderr.write=patched;return{ledger,stop:()=>{process.stderr.write=write;}};
};
const oracle='const [row,root,corpus,admission,cargoExecutionSchema,cargo]=JSON.parse(process.argv[1]),fs=require("node:fs"),p=require("node:path"),cp=require("node:child_process"),Ajv=require("ajv"),controller=new AbortController();let commands=0,refusal="";try{if(!new Ajv({strict:true}).addSchema(cargoExecutionSchema).compile(admission.schema)(admission.value))throw Error("Original native caller capabilities required");if(row.mode==="publisher-refused")throw Error("Original default preparation publication refused");if(row.mode==="queued-cancel"){controller.abort(Error("Original default preparation cancelled"));controller.signal.throwIfAborted();}fs.mkdirSync(p.join(root,"kernel"));fs.writeFileSync(p.join(root,"Cargo.toml"),corpus.workspace);fs.writeFileSync(p.join(root,"kernel/Cargo.toml"),corpus.package);fs.writeFileSync(p.join(root,"kernel/lib.rs"),"pub fn original() {}\\n");if(row.command==="synchronize")for(const args of [["update","--workspace","--manifest-path","Cargo.toml"],["fetch","--locked","--manifest-path","Cargo.toml"]]){const child=cp.spawnSync(cargo,args,{cwd:root,timeout:10000,encoding:"utf8"});if(child.status!==0)throw Error(child.stderr);commands++;}const child=cp.spawnSync(cargo,["metadata","--offline","--no-deps","--format-version=1","--manifest-path","Cargo.toml"],{cwd:root,timeout:10000,encoding:"utf8"});if(child.status!==0)throw Error(child.stderr);if(JSON.parse(child.stdout).packages[0].name!=="original-preparation-kernel")throw Error("Independent native scope changed");}catch(error){refusal=error.message;if(!/Original native caller capabilities required|Original default preparation publication refused|Original default preparation cancelled/.test(refusal))throw error;}console.log(JSON.stringify({commands,refusal}));';

test("Default preparation uses the unchanged production native grant with independent Ajv",()=>{
 const admit=new Ajv({strict:true}).addSchema(cargoExecutionSchema).compile(schema),grant=ownerGrant("synchronize");
 expect(admit(grant)).toBe(true);expect<unknown>(readNativeOwnerCapabilities(grant)).toEqual(grant);
 for(const value of [{},{...grant,child:{maximumElapsedMilliseconds:0}},{...grant,codeRoot:undefined},{...grant,transport:{maximumBytes:grant.transport.maximumBytes}},{...grant,transport:{...grant.transport,maximumLines:Number.MAX_SAFE_INTEGER+1}}]){expect(admit(value)).toBe(false);expect(()=>readNativeOwnerCapabilities(value)).toThrow("Original native caller capabilities required");}
 expect(grant.child.maximumElapsedMilliseconds).toBe(3600000);expect(grant.transport).toEqual({maximumBytes:134217728,maximumLines:1000000});
});

for(const row of corpus.cases)test(row.id,async()=>{
 const oracleRoot=mkdtempSync(join(artifactRoot,"do49-")),reason=Error("Original default preparation cancelled"),observed=progress(row.mode==="publisher-refused"),cancel=new AbortController(),holderAbort=new AbortController(),granted=row.mode!=="normal-caller";clear();author(root);
 let commands=0,refusal:unknown,oracleChild:ReturnType<typeof Bun.spawn>|undefined,holder:Awaited<ReturnType<typeof acquireQueuedResourceLease>>|undefined,cancelTimer:ReturnType<typeof setTimeout>|undefined,releaseTimer:ReturnType<typeof setTimeout>|undefined;
 const exercise=async(invocation:ScriptInvocation):Promise<void>=>{
  oracleChild=Bun.spawn([registeredToolExecutableV1("node"),"--eval",oracle,JSON.stringify([row,oracleRoot,corpus,{schema:granted?schema:callerSchema,value:invocation.capabilities},cargoExecutionSchema,registeredToolExecutableV1("cargo")])],{cwd:process.cwd(),stdout:"pipe",stderr:"pipe"});
  const caller=row.mode==="missing-grant"?undefined:readNativeCallerAuthorityV1(invocation.capabilities);
  const hooks:BoundedHooksV1={execute(_command,args,_cwd,_label,budget,options){
   expect(observed.ledger.at(-1)).toBe("publish:cargo:"+args[0]);expect(options.signal).toBeInstanceOf(AbortSignal);expect(options.signal?.aborted).toBe(false);expect(options.output).toEqual(caller!.transport);expect(budget).toBeGreaterThan(0);expect(budget).toBeLessThanOrEqual(Math.min(ceiling,caller!.child.maximumElapsedMilliseconds));expect(Number.isInteger(budget)).toBe(true);commands++;
  }};
  try{await bounded(invocation,hooks,async scripts=>{await(row.command==="prepare"?scripts.prepare():scripts.synchronize()).run(["--manifest","Cargo.toml"]);});}catch(error){refusal=error;if(row.mode==="queued-cancel")throw error;}
 };
 const capabilities=row.mode==="missing-grant"?()=>({}):row.mode==="normal-caller"?(bootstrap:ScriptInvocation["capabilities"])=>({...bootstrap,codeRoot:testCheckoutRootV1()}):()=>ownerGrant(row.command);
 try{
  try{await issue(row.id,capabilities,async original=>{
   if(row.mode!=="queued-cancel")return exercise(original);
   holder=await acquireQueuedResourceLease({directory:storage.leaseDirectory,resource:"cargo-preparation:"+root,mode:"exclusive",owner:"original-holder",signal:holderAbort.signal});cancelTimer=setTimeout(()=>cancel.abort(reason),30);releaseTimer=setTimeout(()=>holder?.release(),300);
   await withHandledScriptInvocationSignalV1(original,cancel.signal,exercise);
  });}catch(error){if(row.mode!=="publisher-refused")throw error;refusal??=error;expect(String(error)).toContain(row.expectedRefusal);}
  const [stdout,stderr,status]=await Promise.all([new Response(oracleChild!.stdout as ReadableStream).text(),new Response(oracleChild!.stderr as ReadableStream).text(),oracleChild!.exited]);expect(status,stderr).toBe(0);expect(JSON.parse(stdout)).toEqual({commands:row.expectedCommands,refusal:row.expectedRefusal});expect(commands).toBe(row.expectedCommands);
  if(row.mode==="normal"||row.mode==="normal-caller"){expect(refusal).toBeUndefined();expect(observed.ledger).toContain("publish:cargo:"+(row.command==="prepare"?"preparation":"synchronization"));expect((Object.fromEntries(Object.entries(Bun.TOML.parse(readFileSync(join(root,"Cargo.toml"),"utf8")))).workspace as {members:string[]}).members).toEqual(["kernel"]);if(row.command==="synchronize")expect(existsSync(join(root,"Cargo.lock"))).toBe(true);}
  else{expect(String(refusal)).toContain(row.expectedRefusal);expect(existsSync(join(root,"Cargo.lock"))).toBe(false);if(row.mode==="queued-cancel")expect(refusal).toBe(reason);}
 }finally{observed.stop();clearTimeout(cancelTimer);clearTimeout(releaseTimer);holder?.release();if(oracleChild&&oracleChild.exitCode===null){oracleChild.kill();await oracleChild.exited;}clear();rmSync(oracleRoot,{recursive:true,force:true});}
},ceiling*2);

test(corpus.recipe.id,async()=>{
 const independentRoot=mkdtempSync(join(artifactRoot,"dn49-")),reason=Error("Original authored recipe cancelled"),cancel=new AbortController(),observed=progress(),marker=join(root,"kernel/started.json");clear();
 for(const directory of [root,independentRoot]){author(directory);writeFileSync(join(directory,"kernel/Cargo.toml"),corpus.package+corpus.recipe.metadata);writeFileSync(join(directory,"kernel/📜️script.ts"),corpus.recipe.program.replace("{DELAY}",String(corpus.recipe.delayMilliseconds)));}
 let abortedAt=0;
 const poll=setInterval(()=>{if(existsSync(marker)&&!cancel.signal.aborted){abortedAt=performance.now();cancel.abort(reason);}},5);
 const node=Bun.spawn([registeredToolExecutableV1("node"),"--eval",'const fs=require("node:fs"),p=require("node:path"),cp=require("node:child_process"),TOML=require("smol-toml"),[root,row]=JSON.parse(process.argv[1]),controller=new AbortController(),reason=Error("Original authored recipe cancelled"),manifest=TOML.parse(fs.readFileSync(p.join(root,"kernel/Cargo.toml"),"utf8")),recipe=manifest.package.metadata.semio.preparation,child=cp.spawn(process.execPath,[p.join(root,"kernel",recipe.script),...recipe.command],{cwd:p.join(root,"kernel"),signal:controller.signal,stdio:"pipe"}),started=performance.now(),poll=setInterval(()=>{if(fs.existsSync(p.join(root,"kernel/started.json")))controller.abort(reason);},5),timer=setTimeout(()=>controller.abort(Error("Original independent recipe timeout")),10000);child.once("error",error=>{if(!controller.signal.aborted)throw error;});child.once("close",()=>{clearInterval(poll);clearTimeout(timer);console.log(JSON.stringify({cancelled:controller.signal.reason===reason,started:fs.existsSync(p.join(root,"kernel/started.json")),published:fs.existsSync(p.join(root,"kernel/forbidden.txt")),elapsedMilliseconds:performance.now()-started}));});',JSON.stringify([independentRoot,corpus.recipe])],{stdout:"pipe",stderr:"pipe"});
 try{
  let refusal:unknown,elapsedMilliseconds=0;
  await issue(corpus.recipe.id,()=>ownerGrant("prepare"),original=>withHandledScriptInvocationSignalV1(original,cancel.signal,async invocation=>{try{await bounded(invocation,{},scripts=>scripts.prepare().run(["--manifest","Cargo.toml"]));}catch(error){refusal=error;elapsedMilliseconds=performance.now()-abortedAt;throw error;}}));
  const [stdout,stderr,code]=await Promise.all([new Response(node.stdout).text(),new Response(node.stderr).text(),node.exited]);expect(code,stderr).toBe(0);const independent=JSON.parse(stdout);expect(independent.cancelled).toBe(true);expect(independent.started).toBe(true);expect(independent.published).toBe(false);expect(independent.elapsedMilliseconds).toBeLessThan(corpus.recipe.maximumCancellationMilliseconds);expect(refusal).toBe(reason);expect(existsSync(marker)).toBe(true);expect(existsSync(join(root,"kernel/forbidden.txt"))).toBe(false);expect(elapsedMilliseconds).toBeLessThan(corpus.recipe.maximumCancellationMilliseconds);expect(observed.ledger).toContain("publish:cargo:preparation");const pid=JSON.parse(readFileSync(marker,"utf8")).pid;let alive=false;try{process.kill(pid,0);alive=true;}catch{}expect(alive).toBe(false);
 }finally{observed.stop();clearInterval(poll);if(node.exitCode===null){node.kill();await node.exited;}clear();rmSync(independentRoot,{recursive:true,force:true});}
},ceiling*2);

test(corpus.identity.id,async()=>{
 const observed=progress();clear();author(root);let calls=0,identity="";
const hooks:BoundedHooksV1={capture(command,args,cwd,_label,budget,options){
  expect(command).toBe(corpus.identity.expectedProgram);expect(args).toEqual(corpus.identity.arguments);expect(cwd).toBe(root);expect(options.signal).toBeInstanceOf(AbortSignal);expect(options.signal?.aborted).toBe(false);expect(options.output).toEqual(corpus.native.transport);expect(budget).toBeGreaterThan(0);expect(budget).toBeLessThanOrEqual(ceiling);expect(Number.isInteger(budget)).toBe(true);expect(observed.ledger.at(-1)).toBe("publish:cargo:identity");calls++;
 },captured(output){identity=output;}};
 const node=Bun.spawn([registeredToolExecutableV1("node"),"--eval",'const cp=require("node:child_process"),row=JSON.parse(process.argv[1]),result=cp.spawnSync(row.expectedProgram,row.arguments,{cwd:process.argv[2],timeout:10000,encoding:"utf8"});if(result.status!==0)throw Error(result.stderr);console.log(JSON.stringify({identity:result.stdout.trim(),calls:1}));',JSON.stringify(corpus.identity),root],{stdout:"pipe",stderr:"pipe"});
 try{
  await issue(corpus.identity.id,()=>ownerGrant("synchronize"),original=>bounded(original,hooks,scripts=>scripts.synchronize().run(["--manifest","Cargo.toml"])));
  const [stdout,stderr,code]=await Promise.all([new Response(node.stdout).text(),new Response(node.stderr).text(),node.exited]);expect(code,stderr).toBe(0);const independent=JSON.parse(stdout);expect(independent.calls).toBe(corpus.identity.expectedCalls);expect(calls).toBe(corpus.identity.expectedCalls);expect(identity.trim()).toBe(independent.identity);
 }finally{observed.stop();if(node.exitCode===null){node.kill();await node.exited;}clear();}
},ceiling*2);
