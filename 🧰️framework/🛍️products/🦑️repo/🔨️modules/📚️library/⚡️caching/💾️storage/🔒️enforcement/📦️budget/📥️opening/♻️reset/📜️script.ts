import {readImageRecoveryVolumeV1,IMAGE_RECOVERY_VOLUME_ENV_V1} from "./🧭️volume/🟦️.ts";
import {reclaimImageIfQuiescentV1,relayoutRepositoryPartitionIfQuiescentV1,reclaimRepositoryRustImageWithWaitV1,regrowElasticImageOutcomeV1,resetInactiveRepositoryCacheNamespaceV1,type RepositoryCacheResetProfileV1} from "./🟦️.ts";
import type {RepositoryCachePartitionIdV1} from "../../🟦️.ts";
import {BundleScript,ScriptRouter,runScriptMain,receiveScriptProcessInvocation,requireIssuedResourceControlInvocationV1,withRequestedIssuedResourceControlV1,createIssuedResourceControlV1,advanceScriptInvocation,type ScriptInvocation} from "../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {checkResourceLeaseControl,type ResourceLeaseControl} from "../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import {readNativeCallerAuthorityV1} from "../../../../../📦️artifacts/📋️native-orchestration/📥️invocation/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import diskRecoverySchema from "../../../../../../🏗️build/🌡️host-capacity/🧹️disk-recovery/🧬️schema/🔣️.json" with {type:"json"};
import { isMainModuleV1 } from "../../../../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🏁️main/🟦️.ts";
/** 🎯️ Accepts one declared cache profile without caller-controlled paths or options. */
export function readInactiveRepositoryCacheResetProfileV1(args:readonly string[]):RepositoryCacheResetProfileV1{if(args.length!==1||!schema.$defs.Profile.enum.includes(args[0] as RepositoryCacheResetProfileV1))throw Error("Exactly one declared repository cache reset profile required");return args[0] as RepositoryCacheResetProfileV1;}
/** ♻️ Runs one fixed recomputable inactive repository namespace operation. */
class InactiveRepositoryCacheResetScript extends BundleScript {async run(args:string[]):Promise<void>{const profile=readInactiveRepositoryCacheResetProfileV1(args);await runInactiveRepositoryCacheResetV1({root:this.root,repoRoot:this.repoRoot,invocation:this.invocation,control:createIssuedResourceControlV1(this.invocation,"cache:reset-inactive-"+profile,schema.$defs.Protocol.const.maximumMilliseconds)},profile);}}
/** 🛡️ Receives the same issued caller phase without entering the generic build graph or cache. */
export function runInactiveRepositoryCacheResetV1(context:Readonly<{root:string;repoRoot:string;invocation:ScriptInvocation;control:ResourceLeaseControl}>,profile:RepositoryCacheResetProfileV1):Promise<void>{readInactiveRepositoryCacheResetProfileV1([profile]);requireIssuedResourceControlInvocationV1(context.control,context.invocation);checkResourceLeaseControl(context.control);if(readNativeCallerAuthorityV1(context.invocation.capabilities).repositoryRoot!==context.repoRoot)throw Error("Exact repository cache reset owner required");return withRequestedIssuedResourceControlV1(context.control,schema.$defs.Protocol.const.maximumMilliseconds,async phase=>{const result=await resetInactiveRepositoryCacheNamespaceV1(context.invocation,phase,profile);console.log(JSON.stringify(result));});}
/** 🌱️ Reads one declared elastic profile and the `--pressed` flag of the gate, nothing else. */
export function readElasticRegrowArgumentsV1(args:readonly string[]):Readonly<{profile:"graph"|"products"|"rust";pressed:boolean}>{
 if(args.length<1||args.length>2||!(["graph","products","rust"] as readonly string[]).includes(args[0])||(args.length===2&&args[1]!=="--pressed"))throw Error("One elastic repository cache profile and optionally --pressed required");
 return Object.freeze({profile:args[0] as "graph"|"products"|"rust",pressed:args.length===2});
}
/** 🌱️ Retires one idle elastic image that the volume needs back and prints one JSON line with the outcome; a command that holds the image mapped ends it silently with `held`. */
class ElasticRepositoryCacheRegrowScript extends BundleScript {async run(args:string[]):Promise<void>{const{profile,pressed}=readElasticRegrowArgumentsV1(args);await runRepositoryElasticRegrowV1({repoRoot:this.repoRoot,invocation:this.invocation,control:createIssuedResourceControlV1(this.invocation,"cache:regrow-"+profile,schema.$defs.Protocol.const.maximumMilliseconds)},profile,pressed);}}
type ImageOwnerContextV1=Readonly<{repoRoot:string;invocation:ScriptInvocation;control:ResourceLeaseControl}>;
/** 🌱️ Retains the same Original for a fixed elastic image maintenance target. */
export function runRepositoryElasticRegrowV1(context:ImageOwnerContextV1,profile:"graph"|"products"|"rust",pressed:boolean):Promise<void>{
 readElasticRegrowArgumentsV1([profile,...(pressed?["--pressed"]:[])]);
 const{invocation,control}=context;
 requireIssuedResourceControlInvocationV1(control,invocation);checkResourceLeaseControl(control);
 if(readNativeCallerAuthorityV1(invocation.capabilities).repositoryRoot!==context.repoRoot)throw Error("Exact repository cache regrow owner required");
 return withRequestedIssuedResourceControlV1(control,schema.$defs.Protocol.const.maximumMilliseconds,async phase=>{
  const outcome=await regrowElasticImageOutcomeV1(invocation,phase,profile,pressed,readImageRecoveryVolumeV1(process.env[IMAGE_RECOVERY_VOLUME_ENV_V1]));
  console.log(JSON.stringify({version:1,kind:"repository-cache-regrow",partition:profile,outcome}));
 });
}
/** ♻️ Reads one declared partition, nothing else. */
export function readImageReclaimArgumentsV1(args:readonly string[]):RepositoryCachePartitionIdV1{
 if(args.length!==1||!(schema.$defs.Partition.enum as readonly string[]).includes(args[0]))throw Error("Exactly one declared repository cache partition required");
 return args[0] as RepositoryCachePartitionIdV1;
}
/** ♻️ Returns the blocks of one idle bloated image to the volume in place and prints one JSON line with the outcome (`reclaimed` with the host allocation before and after, `skipped` with the reason observed by the operation); the image is never recreated and never stays attached. */
class ImageReclaimScript extends BundleScript {async run(args:string[]):Promise<void>{const profile=readImageReclaimArgumentsV1(args);await runRepositoryImageReclaimV1({repoRoot:this.repoRoot,invocation:this.invocation,control:createIssuedResourceControlV1(this.invocation,"cache:reclaim-"+profile,schema.$defs.Protocol.const.maximumMilliseconds)},profile);}}
/** ♻️ Retains the same Original for the exact registered in-place reclaim target. */
export function runRepositoryImageReclaimV1(context:ImageOwnerContextV1,profile:RepositoryCachePartitionIdV1):Promise<void>{
 readImageReclaimArgumentsV1([profile]);
 const{invocation,control}=context;
 requireIssuedResourceControlInvocationV1(control,invocation);checkResourceLeaseControl(control);
 if(readNativeCallerAuthorityV1(invocation.capabilities).repositoryRoot!==context.repoRoot)throw Error("Exact repository cache reclaim owner required");
 return withRequestedIssuedResourceControlV1(control,schema.$defs.Protocol.const.maximumMilliseconds,async phase=>{
  const result=await reclaimImageIfQuiescentV1(invocation,phase,profile,undefined,readImageRecoveryVolumeV1(process.env[IMAGE_RECOVERY_VOLUME_ENV_V1]));
  console.log(JSON.stringify(result));
 });
}
/** 🚚️ Splits one declared partition into its images and prints one JSON line with the outcome (`busy` while a build holds the partition, `unmounted`, `disabled`, `space`, `refused` or `done` with the moved bytes); the files of every sub-area keep their logical paths. */
class ImageRelayoutScript extends BundleScript {async run(args:string[]):Promise<void>{const profile=readImageReclaimArgumentsV1(args);await runRepositoryImageRelayoutV1({repoRoot:this.repoRoot,invocation:this.invocation,control:createIssuedResourceControlV1(this.invocation,"cache:relayout-"+profile,schema.$defs.Protocol.const.maximumMilliseconds)},profile);}}
/** 🚚️ Retains the same Original for the exact registered re-layout target. */
export function runRepositoryImageRelayoutV1(context:ImageOwnerContextV1,profile:RepositoryCachePartitionIdV1):Promise<void>{
 readImageReclaimArgumentsV1([profile]);
 const{invocation,control}=context;
 requireIssuedResourceControlInvocationV1(control,invocation);checkResourceLeaseControl(control);
 if(readNativeCallerAuthorityV1(invocation.capabilities).repositoryRoot!==context.repoRoot)throw Error("Exact repository cache relayout owner required");
 return withRequestedIssuedResourceControlV1(control,schema.$defs.Protocol.const.maximumMilliseconds,async phase=>{
  const result=await relayoutRepositoryPartitionIfQuiescentV1(invocation,phase,profile,(done,total)=>{void advanceScriptInvocation(invocation,`cache:relayout-${profile}-${done}-${total}`,"running").catch(()=>undefined);});
  console.log(JSON.stringify({version:1,kind:"repository-cache-relayout",partition:profile,...result}));
 });
}
/** ⌛️ Routes a separately authored wait for the exact Rust partition. */
class RustImageReclaimWaitScript extends BundleScript {async run(args:string[]):Promise<void>{
 if(args.length!==1||args[0]!==schema.$defs.RustReclaimWait.const.partition)throw Error("Exactly the Rust partition is admitted for a reclaim wait");
 await runRepositoryRustImageReclaimWaitV1({repoRoot:this.repoRoot,invocation:this.invocation,control:createIssuedResourceControlV1(this.invocation,"cache:reclaim-wait-rust",schema.$defs.Protocol.const.maximumMilliseconds)});
}}
/** 🎟️ Uses the operator's existing Original for the bounded exclusive reclaim request. */
export function runRepositoryRustImageReclaimWaitV1(context:ImageOwnerContextV1):Promise<void>{
 const{invocation,control}=context;
 requireIssuedResourceControlInvocationV1(control,invocation);checkResourceLeaseControl(control);
 if(process.env[diskRecoverySchema.definitions.Policy.const.environmentChild]!==undefined||process.env[IMAGE_RECOVERY_VOLUME_ENV_V1]!==undefined||readNativeCallerAuthorityV1(invocation.capabilities).repositoryRoot!==context.repoRoot)throw Error("Exact operator Rust reclaim owner required");
 return withRequestedIssuedResourceControlV1(control,schema.$defs.Protocol.const.maximumMilliseconds,async phase=>{
  const result=await reclaimRepositoryRustImageWithWaitV1(invocation,phase,(done,total)=>{void advanceScriptInvocation(invocation,`cache:reclaim-wait-rust-${done}-${total}`,"running").catch(()=>undefined);});
  console.log(JSON.stringify(result));
 });
}
const router=new ScriptRouter(import.meta.dirname).register("reset",InactiveRepositoryCacheResetScript).register("regrow",ElasticRepositoryCacheRegrowScript).register("reclaim",ImageReclaimScript).register("relayout",ImageRelayoutScript).register("reclaim-wait",RustImageReclaimWaitScript);
if(isMainModuleV1(import.meta.url))await receiveScriptProcessInvocation(process.env,invocation=>runScriptMain(router,{invocation}));
