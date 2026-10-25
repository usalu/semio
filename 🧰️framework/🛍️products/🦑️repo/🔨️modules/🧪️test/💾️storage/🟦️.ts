import{requireIssuedResourceControlInvocationV1,requireIssuedResourceControlRefinementV1}from"../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {AsyncLocalStorage} from "node:async_hooks";
import {createHash,randomBytes} from "node:crypto";
import {mkdirSync,lstatSync,readdirSync,readFileSync,statfsSync,writeFileSync,rmSync} from "node:fs";
import {join,resolve,basename,dirname} from "node:path";
import {withRepositoryCacheStoreV1,type RepositoryCacheStoreV1} from "../../📚️library/📦️packages/🟦️typescript/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🟦️.ts";
import {requireNativeStoragePathV1} from "../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts";
import {acquireResourceLease,acquireQueuedResourceLease,checkResourceLeaseControl,type ResourceLeaseControl} from "../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import type {ScriptInvocation} from "../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {validateJsonSchemaSubset} from "../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import retentionPolicy from "./🔣️policy.json" with {type:"json"};

type TestArtifactScopeV1={readonly workspace:string;readonly root:string;readonly store:RepositoryCacheStoreV1;readonly access:"shared"|"exclusive";readonly phaseControl:ResourceLeaseControl;live:boolean};
const scopes=new AsyncLocalStorage<TestArtifactScopeV1>();

/** 🧮️ Maps unbounded run identities to one fixed finite reader-lock set. */
export function repositoryTestRunShardV1(task:string):number{if(!/^[a-f0-9]{24}$/u.test(task))throw Error("Original test task identity required");return parseInt(task.slice(0,2),16)%schema.definitions.Protocol.const.runLeaseShards;}
function testRunLeaseV1(path:string):string{return "repository-test-run-shard:"+repositoryTestRunShardV1(basename(dirname(path)));}

/** 🧪️ Holds original finite diagnostics and task-reader custody until the command settles. */
export async function withRepositoryTestArtifactsV1<T>(workspace:string,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl,body:(actualPhase:ResourceLeaseControl)=>Promise<T>,access:"shared"|"exclusive"="shared"):Promise<T>{
 requireIssuedResourceControlInvocationV1(phaseControl,invocation);checkResourceLeaseControl(phaseControl);workspace=resolve(workspace);const parent=scopes.getStore();if(parent?.workspace===workspace&&parent.store.invocation===invocation){repositoryTestArtifactRootV1(workspace);if(access==="exclusive"&&parent.access!==access)throw Error("Test artifact reader cannot upgrade original custody");requireIssuedResourceControlRefinementV1(parent.phaseControl,phaseControl);const nested={...parent,phaseControl,live:true};try{const value=await scopes.run(nested,()=>body(phaseControl));checkResourceLeaseControl(phaseControl);return value;}finally{nested.live=false;}}
 return withRepositoryCacheStoreV1(workspace,"diagnostics",invocation,phaseControl,async(store,actualPhase)=>{const namespace=process.env.SEMIO_TEST_OUTPUT_SCOPE??"repository",identity=createHash("sha256").update(namespace).digest("hex").slice(0,24),task=requireNativeStoragePathV1(store.root,join(store.root,"tests",identity)),root=access==="exclusive"?task:requireNativeStoragePathV1(store.root,join(task,randomBytes(8).toString("hex"))),directory=join(workspace,".cache/b/l/m/data"),lease=await acquireResourceLease({directory,resource:"repository-test-task-shard:"+repositoryTestRunShardV1(identity),mode:access,signal:actualPhase.signal,control:actualPhase});const scope={workspace,root,store,access,phaseControl:actualPhase,live:true};let reader:Awaited<ReturnType<typeof acquireResourceLease>>|undefined;
  try{await store.recheck();if(access==="shared")reader=await acquireResourceLease({directory,resource:testRunLeaseV1(root),mode:"shared",signal:actualPhase.signal,control:actualPhase});const publisher=await acquireResourceLease({directory,resource:"repository-test-retention:"+join(store.root,"tests"),mode:"exclusive",signal:actualPhase.signal,control:actualPhase});try{mkdirSync(root,{recursive:true});if(reader)writeFileSync(requireNativeStoragePathV1(root,join(root,"run.json")),JSON.stringify({version:1,task:identity,run:basename(root)})+"\n",{flag:"wx"});}finally{publisher.release();}await retainRepositoryTestArtifactsV1(workspace,invocation,actualPhase,{...retentionPolicy,version:1});return await scopes.run(scope,()=>body(actualPhase));}finally{scope.live=false;reader?.release();lease.release();if(access==="shared")await retainRepositoryTestArtifactsV1(workspace,invocation,actualPhase,{...retentionPolicy,version:1}).catch(()=>undefined);}
 });
}

/** 📂️ Refuses ambient output roots and released callbacks before resolving any test write. */
export function repositoryTestArtifactRootV1(workspace:string):string{
 const scope=scopes.getStore();if(!scope||scope.workspace!==resolve(workspace)||validateJsonSchemaSubset(schema,{version:1,partition:"diagnostics",directory:"tests",live:scope.live}).length)throw Error("Original finite test artifact callback required");checkResourceLeaseControl(scope.store.control);checkResourceLeaseControl(scope.phaseControl);return requireNativeStoragePathV1(scope.store.root,scope.root);
}

/** 🛣️ Admits every child under the original fixed diagnostics filesystem. */
export function repositoryTestArtifactPathV1(workspace:string,...parts:string[]):string{const root=repositoryTestArtifactRootV1(workspace);return requireNativeStoragePathV1(root,join(root,...parts));}

/** 🧹️ Requires exclusive task custody before any output retirement. */
export function requireRepositoryTestArtifactRetirementV1(workspace:string):void{repositoryTestArtifactRootV1(workspace);if(scopes.getStore()?.access!=="exclusive")throw Error("Exclusive original test artifact retirement required");}

/** 🎛️ Confines child scratch and framework cache locations to the retained diagnostics owner. */
export function repositoryTestArtifactEnvironmentV1(workspace:string,base:NodeJS.ProcessEnv=process.env):NodeJS.ProcessEnv{const root=repositoryTestArtifactRootV1(workspace),temporary=repositoryTestArtifactPathV1(workspace,"tmp"),cache=repositoryTestArtifactPathV1(workspace,"cache");mkdirSync(temporary,{recursive:true});mkdirSync(cache,{recursive:true});return {...base,SEMIO_TEST_ARTIFACT_DIR:root,SEMIO_DIAGNOSTIC_DIRECTORY:temporary,TMPDIR:temporary,TMP:temporary,TEMP:temporary,XDG_CACHE_HOME:cache,PIP_CACHE_DIR:join(cache,"pip"),PYTHONPYCACHEPREFIX:join(cache,"py"),GOCACHE:join(cache,"go"),GOMODCACHE:join(cache,"gomod"),DOTNET_CLI_HOME:join(cache,"dotnet"),NUGET_PACKAGES:join(cache,"nuget")};}

/** 🦑️ Applies the closed finite run retention policy under original reader custody, then relieves the diagnostics image: above its high-water mark the oldest runs that have ended (no live task or run reader lease, older than the minimum age) are retired until the low-water mark is met. With `dryRun` nothing is removed and the answer names what would go. */
export async function retainRepositoryTestArtifactsV1(workspace:string,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl,policy:Readonly<{version:1;maximumRetainedBytes:number;maximumRuns:number}>,tasks?:readonly string[],options:Readonly<{dryRun?:boolean;nowMs?:number;volume?:()=>Readonly<{total:number;free:number}>}>={}):Promise<{removed:string[];held:string[];releasedBytes:number;retainedRuns:number;freedBytes:number}>{
 requireIssuedResourceControlInvocationV1(phaseControl,invocation);checkResourceLeaseControl(phaseControl);if(validateJsonSchemaSubset(schema.definitions.Retention,policy).length||tasks?.some(task=>!/^[a-f0-9]{24}$/u.test(task)))throw Error("Closed finite test retention required");
 return withRepositoryCacheStoreV1(workspace,"diagnostics",invocation,phaseControl,async(store,actualPhase)=>{
  const root=requireNativeStoragePathV1(store.root,join(store.root,"tests")),directory=join(workspace,".cache/b/l/m/data"),dryRun=options.dryRun===true,rules=schema.definitions.Protocol.const,writer=await acquireQueuedResourceLease({directory,resource:"repository-test-retention:"+root,mode:"exclusive",owner:randomBytes(8).toString("hex"),signal:actualPhase.signal,control:actualPhase}),removed:string[]=[],held:string[]=[],rows:{path:string;stamp:number}[]=[],kept:{path:string;allocated:number;stamp:number}[]=[];let bytes=0,runs=0,retained=0,freed=0;
  const custody=async<R>(path:string,body:()=>Promise<R>):Promise<R|undefined>=>{
   let taskReader:Awaited<ReturnType<typeof acquireResourceLease>>|undefined,reader:Awaited<ReturnType<typeof acquireResourceLease>>|undefined;
   try{taskReader=await acquireResourceLease({directory,resource:"repository-test-task-shard:"+repositoryTestRunShardV1(basename(dirname(path))),mode:"exclusive",signal:actualPhase.signal,control:actualPhase,timeoutMs:0});reader=await acquireResourceLease({directory,resource:testRunLeaseV1(path),mode:"exclusive",signal:actualPhase.signal,control:actualPhase,timeoutMs:0});}catch(error){taskReader?.release();if(!String(error).includes("Resource lease timed out"))throw error;return undefined;}
   try{return await body();}finally{reader.release();taskReader?.release();}
  };
  try{
   await store.recheck();mkdirSync(root,{recursive:true});const device=lstatSync(root).dev;
   const count=(path:string,seen=new Set<string>()):number=>{checkResourceLeaseControl(actualPhase);const status=lstatSync(path),key=status.dev+":"+status.ino;if(status.isSymbolicLink())return 0;if(status.dev!==device)throw Error("Original test run filesystem required");if(seen.has(key))return 0;seen.add(key);if(status.isFile())return status.blocks*512;if(!status.isDirectory())throw Error("Original test run file or directory required");return status.blocks*512+readdirSync(path).reduce((sum,name)=>sum+count(join(path,name),seen),0);};
   for(const task of tasks??readdirSync(root).filter(name=>!rules.reservedChildren.includes(name))){
    const parent=requireNativeStoragePathV1(root,join(root,task)),status=lstatSync(parent);if(!/^[a-f0-9]{24}$/u.test(task)||!status.isDirectory()||status.isSymbolicLink())throw Error("Original test task inventory required");
    for(const run of readdirSync(parent)){const path=requireNativeStoragePathV1(root,join(parent,run)),status=lstatSync(path);if(!/^[a-f0-9]{16}$/u.test(run)||!status.isDirectory()||status.isSymbolicLink())throw Error("Original test run inventory required");rows.push({path,stamp:status.mtimeMs});runs++;}
   }
   const receiptOf=(path:string):boolean=>{const receipt=requireNativeStoragePathV1(root,join(path,"run.json"));let file;try{file=lstatSync(receipt);}catch(error){if((error as NodeJS.ErrnoException).code!=="ENOENT")throw error;return false;}
    if(!file.isFile()||file.isSymbolicLink()||file.nlink!==1||file.size>1024)throw Error("Original test run receipt required");const value=JSON.parse(readFileSync(receipt,"utf8"));if(validateJsonSchemaSubset(schema.definitions.Run,value).length||value.task!==basename(join(path,".."))||value.run!==basename(path))throw Error("Original test run receipt identity required");return true;};
   const retire=async(path:string,allocated:number):Promise<void>=>{if(!dryRun){await store.recheck();rmSync(path,{recursive:true});}runs--;freed+=allocated;removed.push(path);};
   for(const row of rows.sort((a,b)=>b.stamp-a.stamp||a.path.localeCompare(b.path))){
    const outcome=await custody(row.path,async()=>{
     await store.recheck();if(!receiptOf(row.path)){bytes+=count(row.path);held.push(row.path);return;}
     const allocated=count(row.path);if(bytes+allocated>policy.maximumRetainedBytes||retained>=policy.maximumRuns)await retire(row.path,allocated);else{bytes+=allocated;retained++;kept.push({path:row.path,allocated,stamp:row.stamp});}
    });
    if(outcome===undefined&&!held.includes(row.path)&&!removed.includes(row.path)&&!kept.some(item=>item.path===row.path))held.push(row.path);
   }
   const fs=statfsSync(store.root),image=options.volume?.()??{total:fs.blocks*fs.bsize,free:fs.bavail*fs.bsize},total=image.total,used=total-image.free-(dryRun?freed:0),now=options.nowMs??Date.now();
   let need=used>Math.floor(total*rules.highWaterShare)?used-Math.floor(total*rules.lowWaterShare):0;
   for(const row of kept.sort((a,b)=>a.stamp-b.stamp||a.path.localeCompare(b.path))){
    if(need<=0)break;checkResourceLeaseControl(actualPhase);if(now-row.stamp<rules.minimumAgeMilliseconds)continue;
    const done=await custody(row.path,async()=>{if(!receiptOf(row.path))return false;await retire(row.path,row.allocated);return true;});
    if(done){need-=row.allocated;bytes-=row.allocated;retained--;}else if(done===undefined)held.push(row.path);
   }
   return {removed,held,releasedBytes:bytes,retainedRuns:runs,freedBytes:freed};
  }finally{writer.release();}
 });
}

/** 💬️ The one-line answer of `repo cache gc` for the finite test run store, in the selected language. */
export function repositoryTestRetentionReportV1(language:"en"|"de",outcome:Readonly<{removed:readonly string[];held:readonly string[];retainedRuns:number;freedBytes:number}>,dryRun:boolean):string{
 const catalogue=schema.definitions.Messages.const[dryRun?"would":"removed"][language],bytes=outcome.freedBytes>=1073741824?(outcome.freedBytes/1073741824).toFixed(1)+" GiB":(outcome.freedBytes/1048576).toFixed(0)+" MiB";
 return catalogue.replace("{count}",String(outcome.removed.length)).replace("{bytes}",bytes).replace("{held}",String(outcome.held.length)).replace("{kept}",String(outcome.retainedRuns));
}
