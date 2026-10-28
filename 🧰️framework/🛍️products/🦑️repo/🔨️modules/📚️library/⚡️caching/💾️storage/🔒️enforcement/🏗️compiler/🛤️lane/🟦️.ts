import {readFile} from "node:fs/promises";
import {join} from "node:path";
import {agentSessionAddressV1,currentAgentSessionV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🪪️agent-sessions/🟦️.ts";
import {readNativeStorageReservationPolicyV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🟦️.ts";
import {requireNativeStoragePathV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts";
import {resolveLanguageV1} from "../../../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🗣️language/🟦️.ts";
import {CATALOG_LANGUAGES_V1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🗣️language/📖️catalog/🟦️.ts";
import type {ResourceLeaseControl} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import {UNIT_CACHE_POLICY_V1} from "../../../../🦀️cargo/🗃️unit-cache/🔏️key/🟦️.ts";
import {acquirePooledLaneV1,poolQuotaBytesV1} from "../../../../🦀️cargo/🗃️unit-cache/🧵️lane/🏊️pool/🟦️.ts";
import {laneMessageV1,unitCacheCapacityBytesV1,unitCacheEnvironmentV1,unitCacheRootV1} from "../../../../🦀️cargo/🗃️unit-cache/🚪️launch/🟦️.ts";
import {withLaneRetentionV1} from "../../../../🦀️cargo/🗃️unit-cache/🧵️lane/📤️retention/🟦️.ts";
import {cacheLedgerV1,cacheRootDirectoryV1} from "../../../🧾️ledger/🟦️.ts";
import {machinePolicyV1,resolveMachineTotalV1,splitMachineBudgetV1} from "../../../🧾️ledger/🎚️machine/🟦️.ts";
import type {RepositoryCompilerStoreV1} from "../🟦️.ts";

type Environment=Readonly<Record<string,string|undefined>>;
/** 🛤️ What a caller says about the compile it is about to run: the crates it builds or tests (the affinity of its lane) and whether the lane compiles through the shared unit cache (`false` when its target directory is a private one). */
export type CompilerLaneRequestV1=Readonly<{crates:readonly string[];unitCache?:boolean}>;

/** 💰️ The recorded bytes the idle lanes of the pool may hold on this machine: the lane share of the smaller of the image capacity and the compiler store's quota in the machine disk budget (the total from the environment or the machine file, split by the declared weights). */
export async function compilerLaneQuotaBytesV1(host:string,environment:Environment,capacityBytes:number):Promise<number>{
 try{
  const declared=machinePolicyV1(),fileText=await readFile(join(cacheRootDirectoryV1(host,cacheLedgerV1(),"repository"),declared.file),"utf8").catch(()=>undefined),total=resolveMachineTotalV1({environment,...fileText===undefined?{}:{fileText}},declared).bytes;
  const quota=splitMachineBudgetV1(total,declared.stores.map(store=>({id:store.id,weight:store.weightBasisPoints,floor:store.floorBytes,demand:Number.MAX_SAFE_INTEGER})),true).get(UNIT_CACHE_POLICY_V1.pool.machineStore);
  return poolQuotaBytesV1(capacityBytes,quota??null);
 }catch{return poolQuotaBytesV1(capacityBytes,null);}
}

/** 🧪️ The environment of a Cargo process that compiles in one lane: the lane's target and build directories and, unless switched off, the unit cache wrapper chain with the lane's crates as the focus compiled incrementally in the lane. Pure. */
export function compilerLaneEnvironmentV1(input:Readonly<{environment:Environment;cacheRoot:string;lane:Readonly<{id:string;base:string;target:string;build:string}>;toolchain:string;crates:readonly string[];unitCache:boolean}>):Environment{
 const lane={...input.environment,CARGO_TARGET_DIR:input.lane.target,CARGO_BUILD_BUILD_DIR:input.lane.build};
 if(!input.unitCache||input.environment.SEMIO_UNIT_CACHE==="off")return Object.freeze(lane);
 return Object.freeze({...lane,...unitCacheEnvironmentV1({cacheRoot:input.cacheRoot,lane:input.lane.id,laneRoot:input.lane.base,compiler:input.environment.RUSTC??"rustc",toolchain:input.toolchain,focus:input.crates,features:{buildScripts:true}})});
}

/** 🏊️ Runs `body` on a view of the compiler store whose build and target directories are a lane of the pool (see `acquirePooledLaneV1`) and whose environment routes the compiler through the unit cache, so the caller compiles beside every other Cargo process instead of queueing behind a custody of the shared directory. The view carries the same storage, control and capture directory as the store; the lane's provenance ledger and logs are published into the store's shared build directory and only then is the lane released, when `body` settles (see `withLaneRetentionV1`). */
export async function withCompilerLaneViewV1<T>(input:Readonly<{host:string;store:RepositoryCompilerStoreV1;control:ResourceLeaseControl;request:CompilerLaneRequestV1;register:(view:RepositoryCompilerStoreV1)=>void}>,body:(view:RepositoryCompilerStoreV1,control:ResourceLeaseControl)=>Promise<T>):Promise<T>{
 const {store,control,request}=input,space=readNativeStorageReservationPolicyV1(store.storage,store.control),cacheRoot=unitCacheRootV1(store.root),language=resolveLanguageV1(CATALOG_LANGUAGES_V1,[],store.environment).language,session=currentAgentSessionV1({environment:store.environment});
 await store.storage.recheck();
 const quotaBytes=await compilerLaneQuotaBytesV1(input.host,store.environment,unitCacheCapacityBytesV1(space));
 const lane=await acquirePooledLaneV1(cacheRoot,{crates:request.crates,quotaBytes,owner:{session:agentSessionAddressV1(session),agent:session.agent.id}},{signal:control.signal,report:(kind,parameters)=>{process.stderr.write(laneMessageV1(kind,parameters,language)+"\n");}});
 return withLaneRetentionV1({lane,shared:store.buildDirectory,language,report:line=>{process.stderr.write(line+"\n");}},async()=>{
  for(const path of[lane.paths.target,lane.paths.build])requireNativeStoragePathV1(store.storage.backing.root,path);
  const view=Object.freeze({...store,buildDirectory:lane.paths.build,environment:compilerLaneEnvironmentV1({environment:store.environment,cacheRoot,lane:{id:lane.paths.id,base:lane.paths.base,target:lane.paths.target,build:lane.paths.build},toolchain:store.toolchain.digest,crates:request.crates,unitCache:request.unitCache??true})});
  const refusal=view.environment.SEMIO_UNIT_CACHE_REFUSAL;if(refusal&&refusal!==store.environment.SEMIO_UNIT_CACHE_REFUSAL)process.stderr.write(refusal+"\n");
  input.register(view);
  return body(view,control);
 });
}
