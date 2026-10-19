import{withHistoricalCargoImageMaintenanceV1}from"./📜️history/🟦️.ts";
import{withCompilerWriterCustodyV1}from"../../💾️storage/🔒️enforcement/🏗️compiler/📏️capacity/🟦️.ts";
import{repositoryCompilerLayoutV1}from"../../💾️storage/🔒️enforcement/🏗️compiler/📍️layout/🟦️.ts";
import{openRepositoryCachePartitionV1}from"../../💾️storage/🔒️enforcement/📦️budget/📥️opening/🟦️.ts";
import{repositoryCompilerReservationV1}from"../../💾️storage/🔒️enforcement/🏗️compiler/🟦️.ts";
import{repositoryStrictStorageV1}from"../../💾️storage/🔒️enforcement/🟦️.ts";
import{readBuildMode}from"@semio-tech/process/build-mode";
import {scriptCompilerAdmissionControlV1} from "@semio-tech/process/compiler-capacity";
import {BundleScript,type ScriptInvocation} from "@semio-tech/process/routing";
import{join}from"node:path";
import{retireInactiveCargoStoreV1,CARGO_RETIREMENT_POLICY_V1,type CargoRetirementPolicyV1}from"./⚙️execution/🟦️.ts";
export{retireInactiveCargoStoreV1,CARGO_RETIREMENT_POLICY_V1,planInactiveCargoRetirementV1,type CargoRetirementPolicyV1,type CargoRetirementPlanV1}from"./⚙️execution/🟦️.ts";

/** 🏪️ Maintains the authored inactive byte budget under the actual repository caller. */
export class InactiveCargoRetirementScript extends BundleScript {
 async run(args:string[]):Promise<void>{
  if(args.length!==1||!["preview","retire","preview-historical","retire-historical"].includes(args[0]!))throw Error("Expected cache-retire-inactive preview|retire|preview-historical|retire-historical");const artifactDirectory=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!artifactDirectory)throw Error("Explicit original maintenance artifact directory required");const control=scriptCompilerAdmissionControlV1(this.invocation,"cache:inactive-cargo-budget",600000),mode=readBuildMode(Object.getOwnPropertyDescriptor(this.invocation.capabilities,"buildMode")?.value),leaseDirectory=join(this.repoRoot,".cache/b/l/m/data");if(args[0]!.endsWith("-historical")){await withHistoricalCargoImageMaintenanceV1(this.repoRoot,this.invocation,control,{version:1,mode,preview:args[0]==="preview-historical"},(image,buildDirectory,targetDirectory,heldControl)=>withCompilerWriterCustodyV1(image,leaseDirectory,heldControl,{version:1,wait:false},()=>retireInactiveCargoStoreV1({workspace:this.repoRoot,buildDirectory,relatedDirectories:[targetDirectory],leaseDirectory,artifactDirectory,invocation:this.invocation,control:heldControl,policy:CARGO_RETIREMENT_POLICY_V1 as CargoRetirementPolicyV1,preview:args[0]==="preview-historical",wait:false})));return;}const image=await openRepositoryCachePartitionV1(this.repoRoot,"rust",control);
  try{const declaration=repositoryStrictStorageV1(this.repoRoot),storage=await repositoryCompilerReservationV1(image,{version:1,root:image.root,leaseDirectory,capacityBytes:declaration.capacityBytes,maximumWorkspaceBytes:declaration.maximumWorkspaceBytes,maximumArtifactBytes:declaration.maximumArtifactBytes},mode,control),layout=repositoryCompilerLayoutV1(image.root,mode);await withCompilerWriterCustodyV1(image,leaseDirectory,control,{version:1,wait:true},()=>retireInactiveCargoStoreV1({workspace:this.repoRoot,buildDirectory:layout.buildDirectory,relatedDirectories:[layout.targetDirectory],leaseDirectory,artifactDirectory,invocation:this.invocation,control,policy:CARGO_RETIREMENT_POLICY_V1 as CargoRetirementPolicyV1,preview:args[0]==="preview",wait:true,compilerStorage:storage,compilerMode:mode}));}finally{await image.close();}
 }
}
