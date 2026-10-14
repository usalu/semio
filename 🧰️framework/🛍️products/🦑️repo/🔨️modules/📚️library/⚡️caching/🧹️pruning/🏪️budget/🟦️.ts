import {cargoDirectories} from "../../🦀️cargo/🗂️directories/🟦️.ts";
import {scriptCompilerAdmissionControlV1} from "../../../../../../../🔨️modules/🏃️process/🏗️build/🎟️capacity/🟦️.ts";
import {join,resolve} from "node:path";
import {mkdir,writeFile,statfs} from "node:fs/promises";
import {withCargoCacheMaintenanceV1,scanCargoBuildUnits,deleteUnit,type CacheUnit} from "../🟦️.ts";
import {assertCompilerDescriptorCustodyV1} from "../📉️pressure/📂️descriptors/🟦️.ts";
import {acquirePhysicalFileLocksV1} from "../../../../../../../🔨️modules/🏃️process/🔒️leases/📁️physical/🟦️.ts";
import {checkResourceLeaseControl,type ResourceLeaseControl} from "../../../../../../../🔨️modules/🏃️process/🔒️leases/🟦️.ts";
import {captureOwnedProcess} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import {BundleScript,type ScriptInvocation} from "../../../../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import {validateJsonSchemaSubset} from "../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

/** ⚖️ Defines a bounded retirement operation; recency orders only genuinely inactive custody. */
export type CargoRetirementPolicyV1=Readonly<{version:1;retainedBytes:number;targetAvailableBytes:number;maximumDeletionBytes:number}>;
/** 🧮️ Exposes unmet retained bytes independently from observed physical disk headroom. */
export type CargoRetirementPlanV1=Readonly<{units:readonly CacheUnit[];bytes:number;remainingBytes:number;unmetBytes:number}>;
/** 📜️ The authored operation limit remains separate from the strict compiler backing. */
export const CARGO_RETIREMENT_POLICY_V1=schema.definitions.Default.const;

/** 🥇️ Plans oldest-first eviction without letting age override byte limits or live ownership. */
export function planInactiveCargoRetirementV1(units:readonly CacheUnit[],policy:CargoRetirementPolicyV1,availableBytes:number):CargoRetirementPlanV1{
 if(validateJsonSchemaSubset(schema,policy).length||!Number.isSafeInteger(availableBytes)||availableBytes<0)throw Error("Complete Cargo retirement policy required");const total=units.reduce((sum,row)=>sum+row.bytes,0),required=Math.max(total-policy.retainedBytes,policy.targetAvailableBytes-availableBytes,0),selected:CacheUnit[]=[];let bytes=0;
 for(const row of [...units].sort((a,b)=>a.recencyMs-b.recencyMs||a.path.localeCompare(b.path))){if(!Number.isSafeInteger(row.bytes)||row.bytes<0)throw Error("Invalid Cargo retirement unit");if(bytes>=required)break;if(row.lockHeld||row.bytes>policy.maximumDeletionBytes-bytes)continue;selected.push(row);bytes+=row.bytes;}
 return{units:selected,bytes,remainingBytes:total-bytes,unmetBytes:Math.max(0,required-bytes)};
}

/** ♻️ Retires only inactive owned compiler units after complete process, descriptor and physical-lock custody. */
export async function retireInactiveCargoStoreV1(options:Readonly<{workspace:string;buildDirectory:string;relatedDirectories:readonly string[];leaseDirectory:string;artifactDirectory:string;invocation:ScriptInvocation;control:ResourceLeaseControl;policy:CargoRetirementPolicyV1;preview:boolean}>):Promise<void>{
 const {control,policy}=options;if(validateJsonSchemaSubset(schema,policy).length)throw Error("Complete Cargo retirement policy required");checkResourceLeaseControl(control);const root=resolve(options.buildDirectory),output=resolve(options.artifactDirectory);await mkdir(output,{recursive:true});
 const available=async()=>{checkResourceLeaseControl(control);const fs=await statfs(root,{bigint:true}),value=Number(fs.bavail*fs.bsize);if(!Number.isSafeInteger(value)||value<0)throw Error("Actual physical headroom required");return value;};
 const capture=async(command:string,args:string[],role:string)=>{checkResourceLeaseControl(control);const remaining=control.remainingMilliseconds();if(remaining===null)throw Error("Finite original maintenance authority required");return captureOwnedProcess(command,args,{invocation:options.invocation,cwd:options.workspace,env:process.env,budgetMs:Math.floor(remaining),maxOutputBytes:16777216,stdoutPath:join(output,role+".stdout"),stderrPath:join(output,role+".stderr")});};
 await withCargoCacheMaintenanceV1({buildDirectory:root,relatedDirectories:options.relatedDirectories,leaseDirectory:options.leaseDirectory,control},async()=>{
  const processes=await capture("ps",["-axo","pid,comm"],"processes");if(processes.reason!=="exit"||processes.status!==0||processes.signal!==null||processes.stderr.trim())throw Error("Complete current process inventory required");if(processes.stdout.split("\n").some(row=>/(?:^|\s|\/)(?:rustc|cargo|cargo-nextest|clang|clang\+\+|ld)$/u.test(row.trim())))throw Error("Current external compiler ownership prevents retirement");
  const originalAvailableBytes=await available(),units=await scanCargoBuildUnits(root,control),plan=planInactiveCargoRetirementV1(units,policy,originalAvailableBytes);await writeFile(join(output,"plan.json"),JSON.stringify({policy,originalAvailableBytes,plan}));console.log("[DEBUG] Bounded inactive Cargo retirement plan "+JSON.stringify({preview:options.preview,units:units.length,totalBytes:plan.remainingBytes+plan.bytes,plannedBytes:plan.bytes,selected:plan.units.length,unmetBytes:plan.unmetBytes,originalAvailableBytes}));if(options.preview||!plan.units.length)return;
  const paths=[...new Set(plan.units.map(unit=>join(root,unit.path.slice(0,unit.path.search(/\/(?:build|incremental)\//u)),".cargo-build-lock")))].sort(),guard=await acquirePhysicalFileLocksV1(paths,control);
  try{
   guard.check();const opened=await capture("lsof",["-nP","-Fpn"],"descriptors");for(const directory of [root,...options.relatedDirectories])assertCompilerDescriptorCustodyV1(opened,resolve(directory),{pid:guard.pid,paths:guard.paths.filter(path=>path.startsWith(resolve(directory)+"/"))});console.log("[DEBUG] Actual original physical compiler locks and complete descriptors held "+JSON.stringify({pid:guard.pid,profiles:paths.length,inventoryBytes:Buffer.byteLength(opened.stdout)}));
   const current=new Map((await scanCargoBuildUnits(root,control)).map(unit=>[unit.path,unit]));for(const unit of plan.units){const row=current.get(unit.path);if(!row||row.bytes!==unit.bytes||row.recencyMs!==unit.recencyMs||row.lockHeld)throw Error("Inactive Cargo unit changed under physical custody");}
   let bytes=0,count=0;for(const unit of plan.units){guard.check();if(unit.lockHeld||bytes+unit.bytes>policy.maximumDeletionBytes)throw Error("Inactive retirement ownership changed");await deleteUnit(root,unit,control);bytes+=unit.bytes;count++;if(count%25===0)console.log("[DEBUG] Actual bounded Cargo retirement progress "+JSON.stringify({count,logicalBytes:bytes,availableBytes:await available()}));}
   guard.check();console.log("[DEBUG] Bounded Cargo retirement completed "+JSON.stringify({count,logicalBytes:bytes,originalAvailableBytes,availableBytes:await available(),retainedLogicalBytes:plan.remainingBytes,unmetLogicalBytes:plan.unmetBytes}));
  }finally{guard.close();}
 });
}

/** 🏪️ Maintains the authored inactive byte budget under the actual repository caller. */
export class InactiveCargoRetirementScript extends BundleScript {
 async run(args:string[]):Promise<void>{
  if(args.length!==1||!["preview","retire"].includes(args[0]!))throw Error("Expected cache-retire-inactive preview|retire");const artifactDirectory=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!artifactDirectory)throw Error("Explicit original maintenance artifact directory required");const directories=cargoDirectories(this.repoRoot,{}),control=scriptCompilerAdmissionControlV1(this.invocation,"cache:inactive-cargo-budget",600000);
  await retireInactiveCargoStoreV1({workspace:this.repoRoot,buildDirectory:directories.build,relatedDirectories:[directories.target],leaseDirectory:join(this.repoRoot,".cache/b/l/m/data"),artifactDirectory,invocation:this.invocation,control,policy:CARGO_RETIREMENT_POLICY_V1 as CargoRetirementPolicyV1,preview:args[0]==="preview"});
 }
}
