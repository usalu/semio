import {randomBytes} from "node:crypto";
import {copyFileSync,mkdirSync,readdirSync,rmSync} from "node:fs";
import {availableParallelism,constants,totalmem} from "node:os";
import {dirname,join,resolve} from "node:path";
import {acquireResourceLease,resourceLeasePathV1,type ResourceLease,type ResourceLeaseControl} from "../../../../../../../🔨️modules/🏃️process/🔒️leases/🟦️.ts";
import {captureOwnedCommandV1,runOwnedCommand,OwnedCommandFailure} from "../../../../../../../🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts";
import type {BuildMode} from "../../../../../../../🔨️modules/🏃️process/🏗️build/🎚️mode/🟦️.ts";
import {acquireCompilerCapacityLeaseV1,scriptCompilerAdmissionControlV1} from "../../../../../../../🔨️modules/🏃️process/🏗️build/🎟️capacity/🟦️.ts";
import {cargoArtifactArgumentsV1,cargoConfigurationArgumentsV1} from "../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🟦️.ts";
import {acquireCargoBuildLeaseV1,cargoRequestedProfileV1} from "../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🔒️lease/🟦️.ts";
import {ADMISSION_POLICY_V1,buildSchedulerV1,readUnitHistoryV1,startAdmissionBrokerV1,unitHistoryPathV1} from "../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎟️admission/🟦️.ts";
import {createJobserverV1,defaultJobserverStyleV1} from "../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎰️jobserver/🟦️.ts";
import {memoryCountersV1} from "../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📈️telemetry/🟦️.ts";
import {captureCargoCompilationGraphV1,actualCargoCompilerPlanningPortV1} from "../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🕸️units/📥️capture/🟦️.ts";
import {admitCargoSourceBoundariesV1,actualCargoSourceAdmissionPortV1} from "../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🎟️admission/🟦️.ts";
import {requireNativeStorageReservationV1,type NativeStorageReservationV1} from "../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🟦️.ts";
import {requireNativeStoragePathV1} from "../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts";
import {validateJsonSchemaSubset} from "../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import type {ScriptInvocation} from "../../../../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import {withRepositoryCompilerStoreV1} from "../../../⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/🟦️.ts";
import {repositoryCargoNetworkV1,withRepositoryCargoInputsV1,withRepositoryCargoResolutionV1} from "../📥️dependencies/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import authored from "./🔣️policy.json" with {type:"json"};

/** 🧾️ One admitted ad-hoc Cargo command: its explicit store, build configuration, core share and closed subcommand class. */
export type RepositoryCargoLaunchV1=Readonly<{version:1;store:"shared"|"isolated";configuration:BuildMode;parallelism:number;kind:"compile"|"resolve"|"lock";cargo:readonly string[]}>;
/** 🧺️ Names one invocation's private target, build and temporary namespaces inside the issued compiler store. */
export type RepositoryCargoIsolatedStoreV1=Readonly<{root:string;target:string;build:string;temporary:string}>;
/** 📐️ The authored launcher policy. */
export type RepositoryCargoLaunchPolicyV1=Readonly<typeof authored>;

/** 🛂️ Validates one value against a named launcher schema definition. */
function admit<T>(definition:keyof typeof schema.definitions,value:unknown):T{const errors=validateJsonSchemaSubset(schema.definitions[definition],value,schema);if(errors.length)throw Error(`Invalid repository Cargo launch ${definition}: ${errors.join("; ")}`);return value as T;}

/** 📜️ The schema-admitted launcher policy. */
export const REPOSITORY_CARGO_LAUNCH_POLICY_V1=Object.freeze(admit<RepositoryCargoLaunchPolicyV1>("Policy",authored));
const policy=REPOSITORY_CARGO_LAUNCH_POLICY_V1;

/** 🧭️ Parses `--store=shared|isolated --configuration=prototype|release [--parallelism=<cores>] -- <subcommand> <args…>` without consuming any Cargo argument.
 * @see ./🧬️schema/🔣️.json */
export function readRepositoryCargoLaunchV1(argv:readonly string[],environment:Readonly<Record<string,string|undefined>>):RepositoryCargoLaunchV1{
 const delimiter=argv.indexOf(policy.delimiter);if(delimiter<0)throw Error("Repository Cargo launch requires `--` before the Cargo subcommand");
 const head=argv.slice(0,delimiter),cargo=argv.slice(delimiter+1),selected=new Map<string,string>(),names=Object.values(policy.options);
 for(let index=0;index<head.length;index++){const argument=head[index]!,name=names.find(option=>argument===option||argument.startsWith(option+"="));if(!name||selected.has(name))throw Error(`Unknown or repeated repository Cargo launch option: ${argument}`);const value=argument===name?head[++index]:argument.slice(name.length+1);if(!value||value.startsWith("-"))throw Error(`Repository Cargo launch option ${name} requires a value`);selected.set(name,value);}
 const store=selected.get(policy.options.store),configuration=selected.get(policy.options.configuration),parallelism=selected.get(policy.options.parallelism);
 if(store===undefined||configuration===undefined)throw Error(`Repository Cargo launch requires explicit ${policy.options.store}=${policy.stores.join("|")} and ${policy.options.configuration}=${policy.configurations.join("|")}`);
 if(environment.SEMIO_BUILD_MODE!==undefined&&environment.SEMIO_BUILD_MODE!==configuration)throw Error("Repository Cargo launch configuration conflicts with SEMIO_BUILD_MODE");
 if(parallelism!==undefined&&!/^[1-9][0-9]{0,3}$/u.test(parallelism))throw Error("Repository Cargo launch parallelism must be a positive integer");
 const kinds=Object.keys(policy.subcommands) as RepositoryCargoLaunchV1["kind"][],kind=kinds.find(key=>policy.subcommands[key].includes(cargo[0]??""));
 if(!kind)throw Error(`Repository Cargo launch admits only ${kinds.flatMap(key=>policy.subcommands[key]).join(", ")} as its first Cargo argument`);
 if(cargo.length>policy.maximumArguments||cargo.some(argument=>argument.length>policy.maximumArgumentLength||argument.includes("\0")))throw Error("Repository Cargo launch arguments exceed their declared bounds");
 return Object.freeze(admit<RepositoryCargoLaunchV1>("Request",{version:1,store,configuration,parallelism:parallelism===undefined?availableParallelism():Number(parallelism),kind,cargo:Object.freeze([...cargo])}));
}

/** 🔏️ Holds a kernel-released exclusive claim on one private store, removes it on every settlement and first sweeps stores whose owner died. */
export async function withRepositoryCargoIsolatedStoreV1<T>(options:Readonly<{storage:NativeStorageReservationV1;base:string;control:ResourceLeaseControl}>,body:(store:RepositoryCargoIsolatedStoreV1)=>Promise<T>):Promise<T>{
 requireNativeStorageReservationV1(options.storage,options.control);await options.storage.recheck();
 const directory=requireNativeStoragePathV1(options.storage.backing.root,join(options.base,policy.isolatedDirectory)),pattern=new RegExp(policy.isolatedNoncePattern,"u");mkdirSync(directory,{recursive:true});
 const claim=(nonce:string)=>acquireResourceLease({directory,resource:nonce,mode:"exclusive",signal:options.control.signal,control:options.control,timeoutMs:0});
 const retire=(nonce:string,lease:ResourceLease)=>{try{rmSync(join(directory,nonce),{recursive:true,force:true});}finally{lease.release();rmSync(resourceLeasePathV1(directory,nonce),{force:true});}};
 for(const nonce of readdirSync(directory).filter(name=>pattern.test(name))){let lease:ResourceLease;try{lease=await claim(nonce);}catch(error){if(error instanceof Error&&error.message.startsWith("Resource lease timed out: "))continue;throw error;}retire(nonce,lease);process.stderr.write(`[repo cargo] swept isolated store ${nonce} of a settled owner\n`);}
 const nonce=randomBytes(6).toString("hex"),lease=await claim(nonce),root=join(directory,nonce),store=Object.freeze({root,target:join(root,"target"),build:join(root,"build"),temporary:join(root,"tmp")});
 try{for(const path of[store.target,store.build,store.temporary]){requireNativeStoragePathV1(options.storage.backing.root,path);mkdirSync(path,{recursive:true});}return await body(store);}finally{retire(nonce,lease);}
}

/** 📍️ Selects the manifest Cargo itself would read: `--manifest-path` or the working directory's `Cargo.toml`. */
function cargoManifestV1(args:readonly string[],cwd:string):string{
 const delimiter=args.indexOf("--"),head=delimiter<0?args:args.slice(0,delimiter),index=head.indexOf("--manifest-path"),inline=head.find(argument=>argument.startsWith("--manifest-path=")),manifest=index>=0?head[index+1]:inline?.slice("--manifest-path=".length)??"Cargo.toml";
 if(!manifest)throw Error("Original compiler manifest selection required");return resolve(cwd,manifest);
}

/** 🎯️ The compilation target a broker pass is keyed by: the first `--target`, else the host. */
function cargoTargetV1(args:readonly string[]):string{const delimiter=args.indexOf("--"),head=delimiter<0?args:args.slice(0,delimiter),index=head.indexOf("--target"),inline=head.find(argument=>argument.startsWith("--target="));return(index>=0?head[index+1]:inline?.slice("--target=".length))??"host";}

/** 🦀️ Runs one admitted Cargo command inside the issued compiler-workspace reservation and returns Cargo's own exit status. Compilers share the machine through
 * the host capacity gate, a jobserver of exactly the admitted cores and the memory-admitting rustc broker sized to the same share of physical memory.
 * @see ../../../⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/🚪️ingress/🟦️.ts
 * @see ../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🟦️.ts */
export async function launchRepositoryCargoV1(invocation:ScriptInvocation,workspace:string,request:RepositoryCargoLaunchV1,cwd:string,environment:Readonly<Record<string,string|undefined>>,options:Readonly<{maximumMilliseconds:number;output:"stream"|"capture"}>):Promise<number>{
 request=admit<RepositoryCargoLaunchV1>("Request",request);
 const args=request.cargo,label="cargo:"+args[0],admission=scriptCompilerAdmissionControlV1(invocation,label,options.maximumMilliseconds);let publishedAtMs=0;
 const control:ResourceLeaseControl={...admission,publish:async progress=>{const now=Date.now();if(now-publishedAtMs<policy.progressMilliseconds)return;publishedAtMs=now;await admission.publish(progress);}},network=repositoryCargoNetworkV1(invocation,workspace),configuration=join(workspace,".cargo/config.toml");
 const execute=async(command:readonly string[],env:Readonly<Record<string,string|undefined>>):Promise<number>=>{
  const remaining=control.remainingMilliseconds();if(remaining!==null&&remaining<1)throw Error("Original repository Cargo deadline exhausted");
  try{if(options.output==="capture"){await captureOwnedCommandV1("cargo",command,cwd,{control,env,output:{maximumBytes:16777216,maximumLines:65536}});return 0;}await runOwnedCommand("cargo",[...command],cwd,label,remaining===null?0:Math.floor(remaining),{env,signal:control.signal,onProgress:line=>{process.stderr.write(line+"\n");}});return 0;}
  catch(error){if(!(error instanceof OwnedCommandFailure))throw error;return error.outcome.status??128+(constants.signals[error.outcome.signal as keyof typeof constants.signals]??0);}
 };
 return withRepositoryCompilerStoreV1(workspace,request.configuration,control,{...environment,SEMIO_BUILD_MODE:request.configuration},async store=>{
  const admitted=async(compilerArgs:readonly string[],env:Readonly<Record<string,string|undefined>>,selected:RepositoryCargoIsolatedStoreV1|Readonly<{target:string;build:string}>):Promise<number>=>{
   const lease=await acquireCargoBuildLeaseV1({directory:join(workspace,".cache/b/l/m/data"),buildDirectory:selected.build,targetDirectory:selected.target,args:compilerArgs,signal:control.signal,control});
   try{
    if(env.SEMIO_COMPILER_CAPACITY_HELD==="true")return await execute(compilerArgs,env);
    const capacity=await acquireCompilerCapacityLeaseV1({directory:join(workspace,policy.capacityLeaseDirectory),control,environment:env,requestedParallelism:request.parallelism});
    try{
     const scheduler=(await buildSchedulerV1({cwd:workspace,targetDirectory:store.environment.CARGO_TARGET_DIR!,buildDirectory:store.buildDirectory,environment:store.environment,control,storage:store.storage,sourceBoundary:store.sourceBoundary})).get("semio-build-scheduler")!,tokens=capacity.parallelism;
     const history=await readUnitHistoryV1(unitHistoryPathV1(join(store.root,"components"))),jobserverDirectory=join(store.environment.TMPDIR!,policy.jobserverDirectoryPrefix+randomBytes(6).toString("hex")),jobserver=await createJobserverV1({version:1,tokens,style:defaultJobserverStyleV1()},jobserverDirectory);
     try{
      const broker=await startAdmissionBrokerV1({physicalBytes:ADMISSION_POLICY_V1.reserveBytes+Math.floor((totalmem()-ADMISSION_POLICY_V1.reserveBytes)*tokens/availableParallelism()),history});let previous:Readonly<{atMs:number;bytes:number}>|undefined;
      const sampler=setInterval(()=>{void memoryCountersV1().then(counters=>{const atMs=Date.now(),rate=previous&&counters.swappedOutBytes!==null?Math.max(0,(counters.swappedOutBytes-previous.bytes)*1000/Math.max(1,atMs-previous.atMs)):null;if(counters.swappedOutBytes!==null)previous={atMs,bytes:counters.swappedOutBytes};broker.observe({nonRustcBytes:0,swapOutBytesPerSecond:rate});},error=>{clearInterval(sampler);process.stderr.write(`[repo cargo] memory sampling stopped: ${error instanceof Error?error.message:String(error)}\n`);});},policy.swapSampleMilliseconds);
      try{broker.registerPass({id:policy.pass,profile:cargoRequestedProfileV1(compilerArgs)!,target:cargoTargetV1(compilerArgs)});process.stderr.write(`[repo cargo] ${request.store} store, ${tokens} of ${availableParallelism()} cores admitted, rustc memory budget ${(broker.stats().budgetBytes/1024**3).toFixed(1)} GiB\n`);return await execute(compilerArgs,{...env,...jobserver.environment(),RUSTC_WRAPPER:scheduler,[ADMISSION_POLICY_V1.brokerEnvironment]:broker.endpoint,[ADMISSION_POLICY_V1.passEnvironment]:policy.pass,SEMIO_COMPILER_CAPACITY_HELD:"true",SEMIO_COMPILER_PARALLELISM:String(tokens)});}
      finally{clearInterval(sampler);await broker.close();}
     }finally{jobserver.close();rmSync(jobserverDirectory,{recursive:true,force:true});}
    }finally{capacity.release();}
   }finally{lease.release();}
  };
  const run=async(selected:RepositoryCargoIsolatedStoreV1|Readonly<{target:string;build:string;temporary:string}>):Promise<number>=>{
   const inputs={workspace,manifest:cargoManifestV1(args,cwd),args,network,control,storage:store.storage,environment:{...store.environment,CARGO_TARGET_DIR:selected.target,CARGO_BUILD_BUILD_DIR:selected.build,TMPDIR:selected.temporary,TMP:selected.temporary,TEMP:selected.temporary}};
   if(request.kind==="lock")return withRepositoryCargoResolutionV1(inputs,env=>execute(cargoConfigurationArgumentsV1(args,configuration,selected.build),env));
   return withRepositoryCargoInputsV1(inputs,async env=>{
    if(request.kind==="resolve")return execute(cargoConfigurationArgumentsV1(args,configuration,selected.build),env);
    const generated=env.SEMIO_TYPEGEN_OUT?join(store.captureDirectory,"typegen-"+randomBytes(4).toString("hex")+".ts"):undefined,compilerEnvironment={...env,...(generated?{SEMIO_TYPEGEN_OUT:generated}:{})},compilerArgs=cargoArtifactArgumentsV1(args,{mode:request.configuration,compilerConfiguration:configuration,buildDirectory:selected.build});
    const graph=await captureCargoCompilationGraphV1({workspaceRoot:workspace,cwd,program:"cargo",prefix:[],args:compilerArgs,environment:compilerEnvironment,maximumOutputBytes:16777216},control,actualCargoCompilerPlanningPortV1());await admitCargoSourceBoundariesV1(graph,workspace,store.sourceBoundary,control,actualCargoSourceAdmissionPortV1(compilerEnvironment,cwd,store.storage));await store.storage.recheck();
    try{const status=await admitted(compilerArgs,compilerEnvironment,selected);if(generated&&status===0)copyFileSync(generated,env.SEMIO_TYPEGEN_OUT!);return status;}finally{if(generated)rmSync(generated,{force:true});}
   });
  };
  return request.store==="shared"?run({target:store.environment.CARGO_TARGET_DIR!,build:store.buildDirectory,temporary:store.environment.TMPDIR!}):withRepositoryCargoIsolatedStoreV1({storage:store.storage,base:dirname(store.buildDirectory),control},run);
 });
}
