/** ⌨️ Closed native frontend recovery before JavaScript image or graph admission. */
import {captureBoundedSourceRevisionV1} from "../../📥️inputs/🔏️revision/🟨️.mjs";
import {join} from "node:path";
import {lstatSync,statfsSync} from "node:fs";
import {machinePathV1} from "../../../../../../../🔨️modules/🏃️process/🏗️build/🏠️machine-state/🟦️.ts";
import {IMAGE_RECOVERY_VOLUME_ENV_V1,meetsImageRecoveryPressureV1,readImageRecoveryVolumeV1} from "../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/♻️reset/🧭️volume/🟦️.ts";
import imagePolicy from "../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/🔣️policy.json" with {type:"json"};
import diskRecoverySchema from "../../🌡️host-capacity/🧹️disk-recovery/🧬️schema/🔣️.json" with {type:"json"};
import type {RepositoryCachePartitionIdV1} from "../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import {validateJsonSchemaSubset} from "../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {requireIssuedResourceControlInvocationV1,type ScriptInvocation} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {checkResourceLeaseControl,type ResourceLeaseControl} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";

const protocol=schema.$defs.Protocol.const;
type ElasticImageProfileV1="graph"|"rust"|"products";
type NativeImageRecoveryTargetV1=Readonly<{operation:"reclaim";profile:RepositoryCachePartitionIdV1}>|Readonly<{operation:"reclaim-wait";profile:"rust"}>|Readonly<{operation:"regrow";profile:ElasticImageProfileV1}>;
export type NativeRepoRecoveryPlanV1=Readonly<{kind:"frontend";target:"build"|"install";owner:string}>|Readonly<{kind:"maintenance";target:"preview"|"retire";owner:string}>|Readonly<{kind:"cache-reset";target:"graph"|"nx"|"rust"|"products";owner:string}>|Readonly<{kind:"cache-image";target:NativeImageRecoveryTargetV1;owner:string}>;
export type NativeRepoRecoveryContextV1=Readonly<{root:string;repoRoot:string;invocation:ScriptInvocation;control:ResourceLeaseControl}>;
export type NativeRepoRecoveryPortsV1=Readonly<{build?:(context:NativeRepoRecoveryContextV1)=>Promise<void>;install?:(context:NativeRepoRecoveryContextV1)=>Promise<void>;child?:(root:string,args:readonly string[],original:ScriptInvocation,phase:ResourceLeaseControl)=>Promise<void>;maintenance?:(context:NativeRepoRecoveryContextV1,operation:"preview"|"retire")=>Promise<void>;reset?:(context:NativeRepoRecoveryContextV1,profile:"graph"|"nx"|"rust"|"products")=>Promise<void>;image?:(context:NativeRepoRecoveryContextV1,operation:NativeImageRecoveryTargetV1)=>Promise<void>}>;

/** 🎯️ Selects only fixed authored recovery commands, retaining ordinary graph semantics otherwise. */
export function readNativeRepoRecoveryPlanV1(root:string,args:readonly string[]):NativeRepoRecoveryPlanV1|undefined {
 if(args[0]!=="run")return undefined;const target=args[1]?.split(":");if(target?.length!==2)return undefined;
 const maintenance=target[0]===protocol.maintenance.project&&Object.hasOwn(protocol.maintenance.targets,target[1]!),frontend=target[0]===protocol.project&&protocol.targets.includes(target[1]!),reset=target[0]===protocol.reset.project&&Object.hasOwn(protocol.reset.targets,target[1]!);const image=target[0]===protocol.reset.project&&Object.hasOwn(protocol.reset.imageTargets,target[1]!);if(!maintenance&&!frontend&&!reset&&!image)return undefined;
 let parallel=false;for(let index=2;index<args.length;index++){if(!parallel&&/^--parallel=[1-9][0-9]*$/u.test(args[index]!)&&!validateJsonSchemaSubset(schema.$defs.CapacityParallelJobs,Number(args[index]!.slice(11))).length){parallel=true;continue;}if(args[index]==="--output"&&args[index+1]==="stream"){index++;continue;}if(!protocol.options.includes(args[index]!)&&!(frontend&&target[1]==="build"&&protocol.buildOnlyOptions.includes(args[index]!)))return undefined;}
 const owner=join(root,reset||image?protocol.reset.owner:maintenance?protocol.maintenance.owner:protocol.owner),captured=captureBoundedSourceRevisionV1(join(owner,"📋️project.json"),protocol.maximumOwnerBytes,true),manifest=JSON.parse(captured.text!);
 if(validateJsonSchemaSubset(reset||image?schema.$defs.ResetOwner:maintenance?schema.$defs.MaintenanceOwner:schema.$defs.Owner,manifest).length)throw Error("Exact authored native recovery owner and dependencies required");
 if(image){
  const targetName=target[1] as keyof typeof protocol.reset.imageTargets,operation=protocol.reset.imageTargets[targetName],profile=targetName.slice(("cache-"+operation+"-").length);
  const declared=(manifest as {targets:Record<string,{options?:{command?:unknown}}>}).targets[targetName];
  const command="repo ./📜️script.ts "+operation+" "+profile+(operation==="regrow"?" --pressed":"");
  if(validateJsonSchemaSubset(schema.$defs.ImageOwnerTarget,declared).length||declared?.options?.command!==command)throw Error("Exact authored Native image maintenance target required");
  const selected:NativeImageRecoveryTargetV1|undefined=operation==="reclaim-wait"?{operation,profile:"rust"}:operation==="reclaim"?{operation,profile:profile as RepositoryCachePartitionIdV1}:operation==="regrow"?{operation,profile:profile as ElasticImageProfileV1}:undefined;
  if(selected===undefined)throw Error("Exact authored Native image maintenance target required");
  return Object.freeze({kind:"cache-image",target:Object.freeze(selected),owner});
 }
 return reset?Object.freeze({kind:"cache-reset",target:protocol.reset.targets[target[1] as keyof typeof protocol.reset.targets] as "graph"|"nx"|"rust"|"products",owner:join(root,protocol.reset.scriptRoot)}):maintenance?Object.freeze({kind:"maintenance",target:protocol.maintenance.targets[target[1] as keyof typeof protocol.maintenance.targets] as "preview"|"retire",owner:join(root,protocol.maintenance.scriptRoot)}):Object.freeze({kind:"frontend",target:target[1] as "build"|"install",owner});
}

/** 💽️ Lets an exact fixed image owner reclaim in place, or recover automatically under verified pressure. */
export function admitsNativeImageDiskRecoveryV1(root:string,plan:NativeRepoRecoveryPlanV1|undefined,environment:Readonly<Record<string,string|undefined>>,platform:string=process.platform,observe=observeImageRecoveryDeviceV1):boolean{
 if(platform!=="darwin"||plan?.kind!=="cache-image")return false;
 const child=environment[diskRecoverySchema.definitions.Policy.const.environmentChild],wire=environment[IMAGE_RECOVERY_VOLUME_ENV_V1];
 const waiting=plan.target.operation==="reclaim-wait",manual=child===undefined&&wire===undefined&&(waiting||plan.target.operation==="reclaim");
 if(waiting&&!manual)return false;
 if(!manual&&child!=="1")return false;
 try{
  const declaration=imagePolicy.partitions[plan.target.profile];
  if(!declaration)return false;
  const device=observe(machinePathV1(root,declaration.directory));
  if(device===undefined)return false;
  if(manual)return !waiting||device.availableBytes<Math.min(device.totalBytes,imagePolicy.elastic.rust.reclaimTightHostFreeBytes);
  const recovery=readImageRecoveryVolumeV1(wire);
  return recovery!==undefined&&meetsImageRecoveryPressureV1(device,recovery);
 }catch{return false;}
}
function observeImageRecoveryDeviceV1(directory:string):Readonly<{device:number;availableBytes:number;totalBytes:number}>|undefined{
 const root=lstatSync(directory),file=lstatSync(join(directory,"fixed.dmg"));
 if(!root.isDirectory()||root.isSymbolicLink()||!file.isFile()||file.isSymbolicLink()||file.nlink!==1||file.dev!==root.dev)return undefined;
 const volume=statfsSync(directory);
 return{device:file.dev,availableBytes:volume.bavail*volume.bsize,totalBytes:volume.blocks*volume.bsize};
}

/** 🧬️ Closes the installation's authored build dependency before the Original is issued. */
export function nativeRepoRecoveryBuildDescendantsV1(root:string,args:readonly string[]):readonly (readonly string[])[] {
 const plan=readNativeRepoRecoveryPlanV1(root,args);return plan?.kind==="frontend"&&plan.target==="install"?[protocol.buildChild]:[];
}

/** 🔌️ Supplies the recovery ports of one selected plan: the owners are injected, so this module never imports the layers that run above it. */
export type NativeRepoRecoveryPortSourceV1=NativeRepoRecoveryPortsV1|((plan:NativeRepoRecoveryPlanV1)=>Promise<NativeRepoRecoveryPortsV1>);

/** 🛡️ Executes the existing owner beneath the exact genuine incoming Original and phase. */
export async function runNativeRepoRecoveryV1(root:string,args:readonly string[],original:ScriptInvocation,phase:ResourceLeaseControl,source:NativeRepoRecoveryPortSourceV1):Promise<boolean> {
 requireIssuedResourceControlInvocationV1(phase,original);checkResourceLeaseControl(phase);const plan=readNativeRepoRecoveryPlanV1(root,args);if(!plan)return false;
 const ports=typeof source==="function"?await source(plan):source;
 checkResourceLeaseControl(phase);const context=Object.freeze({root:plan.owner,repoRoot:root,invocation:original,control:phase});
 if(plan.kind==="cache-reset"){if(!ports.reset)throw Error("Exact inactive repository cache reset owner port required");await ports.reset(context,plan.target);}
 else if(plan.kind==="cache-image"){if(!ports.image)throw Error("Exact Native image owner port required");await ports.image(context,plan.target);}
 else if(plan.kind==="maintenance"){if(!ports.maintenance)throw Error("Exact maintenance owner port required");await ports.maintenance(context,plan.target);}
 else if(plan.target==="install"){if(!ports.child||!ports.install)throw Error("Exact native installation ports required");await ports.child(root,protocol.buildChild,original,phase);checkResourceLeaseControl(phase);await ports.install(context);}else{if(!ports.build)throw Error("Exact native build port required");await ports.build(context);}
 checkResourceLeaseControl(phase);return true;
}
