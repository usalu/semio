import{readInactiveCargoRetirementOperationV1,retireInactiveCargoStoreV1,CARGO_RETIREMENT_POLICY_V1,type CargoRetirementPolicyV1}from"./🟦️.ts";
import{withHistoricalCargoImageMaintenanceV1}from"./📜️history/🟦️.ts";
import{withCompilerWriterCustodyV1}from"../../💾️storage/🔒️enforcement/🏗️compiler/📏️capacity/🟦️.ts";
import{repositoryCompilerLayoutV1}from"../../💾️storage/🔒️enforcement/🏗️compiler/📍️layout/🟦️.ts";
import{openRepositoryCachePartitionV1}from"../../💾️storage/🔒️enforcement/📦️budget/📥️opening/🟦️.ts";
import{repositoryCompilerReservationV1}from"../../💾️storage/🔒️enforcement/🏗️compiler/🟦️.ts";
import{repositoryStrictStorageV1}from"../../💾️storage/🔒️enforcement/🟦️.ts";
import{withRepositoryCacheStoreV1}from"../../💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🟦️.ts";
import{BundleScript,ScriptRouter,runScriptMain,receiveScriptProcessInvocation,requireIssuedResourceControlInvocationV1,withRequestedIssuedResourceControlV1,type ScriptInvocation}from"../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import{readBuildMode}from"../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎚️mode/🟦️.ts";
import{scriptCompilerAdmissionControlV1}from"../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import{checkResourceLeaseControl,type ResourceLeaseControl}from"../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import{dirname,join}from"node:path";
import{fileURLToPath}from"node:url";
import{mkdtemp}from"node:fs/promises";
import schema from"./🧬️schema/🔣️.json"with{type:"json"};
import { isMainModuleV1 } from "../../../../../../../🔨️modules/⚙️runtime/🏁️main/🟦️.ts";
const runtime=schema.definitions.Runtime.const;

/** 🏪️ Maintains the authored inactive byte budget under the actual repository caller. */
export class InactiveCargoRetirementScript extends BundleScript {
 async run(args:string[],provided?:ResourceLeaseControl):Promise<void>{
  const operation=readInactiveCargoRetirementOperationV1(args),control=provided??scriptCompilerAdmissionControlV1(this.invocation,"cache:inactive-cargo-budget",runtime.maximumMilliseconds),configured=process.env.SEMIO_TEST_ARTIFACT_DIR;
  requireIssuedResourceControlInvocationV1(control,this.invocation);checkResourceLeaseControl(control);
  if(configured!==undefined&&!configured)throw Error("Explicit original maintenance artifact directory required");
  const mode=readBuildMode(Object.getOwnPropertyDescriptor(this.invocation.capabilities,"buildMode")?.value),leaseDirectory=join(this.repoRoot,".cache/b/l/m/data");
  if(operation.endsWith("-historical")){if(configured===undefined)throw Error("Explicit original maintenance artifact directory required");await withHistoricalCargoImageMaintenanceV1(this.repoRoot,this.invocation,control,{version:1,mode,preview:operation==="preview-historical"},(image,buildDirectory,targetDirectory,heldControl)=>withCompilerWriterCustodyV1(image,leaseDirectory,heldControl,{version:1,wait:false},()=>retireInactiveCargoStoreV1({workspace:this.repoRoot,buildDirectory,relatedDirectories:[targetDirectory],leaseDirectory,artifactDirectory:configured,invocation:this.invocation,control:heldControl,policy:CARGO_RETIREMENT_POLICY_V1 as CargoRetirementPolicyV1,preview:operation==="preview-historical",wait:false})));return;}
  const image=await openRepositoryCachePartitionV1(this.repoRoot,"rust",control);
  try{const declaration=repositoryStrictStorageV1(this.repoRoot),storage=await repositoryCompilerReservationV1(image,{version:1,root:image.root,leaseDirectory,capacityBytes:declaration.capacityBytes,maximumWorkspaceBytes:declaration.maximumWorkspaceBytes,maximumArtifactBytes:declaration.maximumArtifactBytes},mode,control),layout=repositoryCompilerLayoutV1(image.root,mode),result=await withCompilerWriterCustodyV1(image,leaseDirectory,control,{version:1,wait:true},writer=>this.withArtifacts(writer,configured,async(artifactDirectory,actual)=>({artifactDirectory,...await retireInactiveCargoStoreV1({workspace:this.repoRoot,buildDirectory:layout.buildDirectory,relatedDirectories:[layout.targetDirectory],leaseDirectory,artifactDirectory,invocation:this.invocation,control:actual,policy:CARGO_RETIREMENT_POLICY_V1 as CargoRetirementPolicyV1,preview:operation==="preview",wait:true,compilerStorage:storage,compilerMode:mode})})));console.log(JSON.stringify({version:1,operation,mode,root:image.root,buildDirectory:layout.buildDirectory,targetDirectory:layout.targetDirectory,...result}));}finally{await image.close();}
 }
 private withArtifacts<T>(control:ResourceLeaseControl,configured:string|undefined,body:(directory:string,actual:ResourceLeaseControl)=>Promise<T>):Promise<T>{
  if(configured!==undefined)return body(configured,control);
  return withRepositoryCacheStoreV1(this.repoRoot,"diagnostics",this.invocation,control,async(diagnostics,phase)=>{await diagnostics.recheck();const artifacts=await mkdtemp(join(diagnostics.root,"tmp","retire-")),value=await body(artifacts,phase);await diagnostics.recheck();return value;});
 }
}

/** 🪪️ Receives an existing authenticated phase for only the declared current-store maintenance owner. */
export function runInactiveCompilerMaintenanceV1(context:Readonly<{root:string;repoRoot:string;invocation:ScriptInvocation;control:ResourceLeaseControl}>,operation:"preview"|"retire"):Promise<void>{requireIssuedResourceControlInvocationV1(context.control,context.invocation);checkResourceLeaseControl(context.control);return withRequestedIssuedResourceControlV1(context.control,runtime.maximumMilliseconds,phase=>new InactiveCargoRetirementScript(context.root,context.repoRoot,context.invocation).run([operation],phase));}

const root=dirname(fileURLToPath(import.meta.url)),router=new ScriptRouter(root).register("cache-retire-inactive",InactiveCargoRetirementScript);
if(isMainModuleV1(import.meta.url))await receiveScriptProcessInvocation(process.env,invocation=>runScriptMain(router,{invocation}));
