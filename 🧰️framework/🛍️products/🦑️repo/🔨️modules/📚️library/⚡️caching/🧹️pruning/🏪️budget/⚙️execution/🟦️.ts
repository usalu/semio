import{cacheScanTimingEnabledV1}from"../../⏱️measurement/🟦️.ts";
import{describeCacheRecencyChangeV1}from"../../⏱️recency/🟦️.ts";
import{requireCompilerWriterCustodyV1,withCompilerWriterCustodyV1}from"../../../💾️storage/🔒️enforcement/🏗️compiler/📏️capacity/🟦️.ts";
import{requireHistoricalCargoMaintenanceV1}from"../📜️history/🟦️.ts";
import{repositoryCompilerLayoutV1}from"../../../💾️storage/🔒️enforcement/🏗️compiler/📍️layout/🟦️.ts";
import{requireNativeStorageReservationV1,type NativeStorageReservationV1}from"../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🟦️.ts";
import {join,resolve} from "node:path";
import {mkdir,mkdtemp,writeFile,statfs} from "node:fs/promises";
import {withCargoCacheMaintenanceV1,scanCargoBuildUnits,scanCargoIncrementalUnits,deleteUnit,type CacheUnit} from "../../🟦️.ts";
import {assertNativeCacheDescriptorCustodyV1} from "../../../../../../../../🔨️modules/🏃️process/📦️artifacts/💾️cache/📂️descriptors/🟦️.ts";
import {acquirePhysicalFileLocksV1} from "../../../../../../../../🔨️modules/🏃️process/🔒️leases/📁️physical/🟦️.ts";
import {checkResourceLeaseControl,type ResourceLeaseControl} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import {captureOwnedProcess} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import {validateJsonSchemaSubset} from "../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from ".././🧬️schema/🔣️.json" with {type:"json"};

import{withIssuedResourceOperationV1,type ScriptInvocation}from"../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";

/** ⚖️ Defines a bounded retirement operation; recency orders only genuinely inactive custody. */
export type CargoRetirementPolicyV1=Readonly<{version:1;retainedBytes:number;targetAvailableBytes:number;maximumDeletionBytes:number}>;
/** 🧮️ Exposes unmet retained bytes independently from observed physical disk headroom. */
export type CargoRetirementPlanV1=Readonly<{units:readonly CacheUnit[];bytes:number;remainingBytes:number;unmetBytes:number}>;
/** 📜️ The authored operation limit remains separate from the strict compiler backing. */
export const CARGO_RETIREMENT_POLICY_V1=Object.freeze({...schema.definitions.Default.const}) as CargoRetirementPolicyV1;

/** 🥇️ Plans oldest-first eviction without letting age override byte limits or live ownership. */
export function planInactiveCargoRetirementV1(units:readonly CacheUnit[],policy:CargoRetirementPolicyV1,availableBytes:number):CargoRetirementPlanV1{
 if(validateJsonSchemaSubset(schema,policy).length||!Number.isSafeInteger(availableBytes)||availableBytes<0)throw Error("Complete Cargo retirement policy required");const total=units.reduce((sum,row)=>sum+row.bytes,0),required=Math.max(total-policy.retainedBytes,policy.targetAvailableBytes-availableBytes,0),selected:CacheUnit[]=[];let bytes=0;
 for(const row of [...units].sort((a,b)=>a.recencyMs-b.recencyMs||a.path.localeCompare(b.path))){if(!Number.isSafeInteger(row.bytes)||row.bytes<0)throw Error("Invalid Cargo retirement unit");if(bytes>=required)break;if(row.lockHeld||row.bytes>policy.maximumDeletionBytes-bytes)continue;selected.push(row);bytes+=row.bytes;}
 return{units:selected,bytes,remainingBytes:total-bytes,unmetBytes:Math.max(0,required-bytes)};
}

/** ♻️ Retires only inactive owned compiler units after complete process, descriptor and physical-lock custody. */
type CargoRetirementRequestV1=Readonly<{workspace:string;buildDirectory:string;relatedDirectories:readonly string[];leaseDirectory:string;artifactDirectory:string;invocation:ScriptInvocation;control:ResourceLeaseControl;policy:CargoRetirementPolicyV1;preview:boolean;wait:boolean;compilerStorage?:NativeStorageReservationV1;compilerMode?:"prototype"|"release"}>;
/** 🛟️ Registers authenticated historical work before effects so image custody drains every accepted child. */
export async function retireInactiveCargoStoreV1(options:CargoRetirementRequestV1):Promise<Readonly<{units:number;logicalBytes:number;availableBytes:number}>>{
 options=Object.freeze({...options,relatedDirectories:Object.freeze([...options.relatedDirectories]),policy:Object.freeze({...options.policy})});
 if(options.compilerStorage){const storage=requireNativeStorageReservationV1(options.compilerStorage,options.control);requireCompilerWriterCustodyV1(storage.backing.root,options.control);return withCompilerWriterCustodyV1({root:storage.backing.root,maximumPhysicalBytes:storage.backing.maximumPhysicalBytes,recheck:storage.recheck},options.leaseDirectory,options.control,{version:1,wait:false},()=>retireInactiveCargoStoreOwnedV1(options));}const image=requireHistoricalCargoMaintenanceV1(options);requireCompilerWriterCustodyV1(image.root,options.control);return withCompilerWriterCustodyV1(image,options.leaseDirectory,options.control,{version:1,wait:false},()=>withIssuedResourceOperationV1(options.control,()=>retireInactiveCargoStoreOwnedV1(options)));
}
async function retireInactiveCargoStoreOwnedV1(options:CargoRetirementRequestV1):Promise<Readonly<{units:number;logicalBytes:number;availableBytes:number}>>{
 const {control,policy}=options;if(validateJsonSchemaSubset(schema,policy).length)throw Error("Complete Cargo retirement policy required");checkResourceLeaseControl(control);const root=resolve(options.buildDirectory),parent=resolve(options.artifactDirectory);if(options.compilerStorage){requireNativeStorageReservationV1(options.compilerStorage,control);requireCompilerWriterCustodyV1(options.compilerStorage.backing.root,control);if(options.compilerStorage.allocation.kind!=="compiler-workspace"||!options.compilerMode)throw Error("Actual compact compiler retirement owner required");const layout=repositoryCompilerLayoutV1(options.compilerStorage.backing.root,options.compilerMode);if(root!==layout.buildDirectory||options.relatedDirectories.length!==1||options.relatedDirectories[0]!==layout.targetDirectory)throw Error("Original compact compiler retirement roots required");await options.compilerStorage.recheck();}else{if(options.compilerMode)throw Error("Actual compact compiler reservation required");requireHistoricalCargoMaintenanceV1(options);}await mkdir(parent,{recursive:true});const output=await mkdtemp(join(parent,"turn-"));
 const available=async()=>{checkResourceLeaseControl(control);const fs=await statfs(root,{bigint:true}),value=Number(fs.bavail*fs.bsize);if(!Number.isSafeInteger(value)||value<0)throw Error("Actual physical headroom required");return value;};
 const timing=cacheScanTimingEnabledV1(process.env),scan=async(phase:"initial"|"recensus")=>[...await scanCargoBuildUnits(root,control,undefined,true,timing?phase:undefined),...(await scanCargoIncrementalUnits(root,control,undefined,true,timing?phase:undefined)).units];
 const capture=async(command:string,args:string[],role:string)=>{checkResourceLeaseControl(control);const remaining=control.remainingMilliseconds();if(remaining===null)throw Error("Finite original maintenance authority required");return captureOwnedProcess(command,args,{invocation:options.invocation,cwd:options.workspace,env:process.env,budgetMs:Math.floor(remaining),maxOutputBytes:16777216,stdoutPath:join(output,role+".stdout"),stderrPath:join(output,role+".stderr")});};
 return withCargoCacheMaintenanceV1({buildDirectory:root,relatedDirectories:options.relatedDirectories,leaseDirectory:options.leaseDirectory,control,wait:options.wait},async()=>{
  const processes=await capture("ps",["-axo","pid,comm"],"processes");if(processes.reason!=="exit"||processes.status!==0||processes.signal!==null||processes.stderr.trim())throw Error("Complete current process inventory required");const compilerProcesses=processes.stdout.split("\n").filter(row=>/(?:^|\s|\/)(?:rustc|cargo|cargo-nextest|clang|clang\+\+|ld)$/u.test(row.trim()));
  const originalAvailableBytes=await available(),units=await scan("initial"),plan=planInactiveCargoRetirementV1(units,policy,originalAvailableBytes);await writeFile(join(output,"plan.json"),JSON.stringify({policy,originalAvailableBytes,plan}));if(options.preview||!plan.units.length)return{units:0,logicalBytes:0,availableBytes:originalAvailableBytes};
  const paths=[...new Set(plan.units.map(unit=>join(root,unit.path.slice(0,unit.path.search(/\/(?:build|incremental)\//u)),".cargo-build-lock")))].sort(),guard=await acquirePhysicalFileLocksV1(paths,control,{version:1,wait:options.wait});
  try{
   guard.check();const opened=await capture("lsof",["-nP","-Fpn"],"descriptors");for(const directory of [root,...options.relatedDirectories])assertNativeCacheDescriptorCustodyV1(opened,resolve(directory),{pid:guard.pid,paths:guard.paths.filter(path=>path.startsWith(resolve(directory)+"/"))},"newline-pn");
   const current=new Map((await scan("recensus")).map(unit=>[unit.path,unit]));for(const unit of plan.units){const row=current.get(unit.path);if(!row||row.bytes!==unit.bytes||row.recencyMs!==unit.recencyMs||row.lockHeld){throw Error("Inactive Cargo unit changed under physical custody");}}
   let bytes=0,count=0;for(const unit of plan.units){guard.check();if(unit.lockHeld||bytes+unit.bytes>policy.maximumDeletionBytes)throw Error("Inactive retirement ownership changed");await deleteUnit(root,unit,control);bytes+=unit.bytes;count++;}
   guard.check();return{units:count,logicalBytes:bytes,availableBytes:await available()};
  }finally{guard.close();}
 });
}

