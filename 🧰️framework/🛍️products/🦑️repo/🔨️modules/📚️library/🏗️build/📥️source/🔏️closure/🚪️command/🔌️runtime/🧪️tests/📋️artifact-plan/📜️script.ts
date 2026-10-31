import {requireCapturedBuildCommandV1,type CapturedBuildCommandProofV1} from "../../../🟦️.ts";
import {withAdmittedCargoArtifactPlanV1} from "../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts";
import {readFile,writeFile} from "node:fs/promises";
import {basename,dirname,isAbsolute,join,relative,sep} from "node:path";
import {receiveScriptProcessInvocation} from "../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {scriptCompilerAdmissionControlV1} from "../../../../../../../../../../../🔨️modules/🏃️process/🏗️build/🎟️capacity/🟦️.ts";
import {verifyCapturedCargoArtifactsV1,withCapturedCargoArtifactsTaskV1,type CargoArtifactPlanV1} from "../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/🟦️.ts";
import {captureOwnedCommandV1} from "../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import {provider} from "../../../../../../../🏃️process/🦀️cargo/🔌️provider/🟦️.ts";
import {withRepositoryRuntimeEntriesV1} from "../../🟦️.ts";
export async function runCapturedBuildSourceV1(args:readonly string[],proof:CapturedBuildCommandProofV1):Promise<void>{
 requireCapturedBuildCommandV1(proof,import.meta.semioCapturedImplementationIdentity,import.meta.semioCapturedLoaderIdentity);
 const [manifest,mode]=args;if(!manifest||!mode)throw Error("Declared physical artifact-plan fixture required");
await receiveScriptProcessInvocation(process.env,async original=>{
 const control=scriptCompilerAdmissionControlV1(original,"test:physical-artifact-plan",120000);let consumed=0,pendingFinished=false,buildScriptOutDir:string|undefined;
 try{await withRepositoryRuntimeEntriesV1(process.cwd(),original,control,async()=>{
  let facts:{identity:string;source:string;configuration:string;manifest:string;buildDirectory:string;plan:CargoArtifactPlanV1["facts"]}|undefined;
  await (mode==="public-provider"?withAdmittedCargoArtifactPlanV1:provider.withArtifactPlan)(manifest,["--package","artifact-plan-subject","--lib","--bin","artifact-plan-subject"],process.cwd(),{producer:"artifact",invocation:original,control,consume:async capture=>{consumed++;await verifyCapturedCargoArtifactsV1(capture);await withCapturedCargoArtifactsTaskV1(capture,async view=>{const binary=[...view.files.values()].find(path=>basename(path)==="artifact-plan-subject"+(process.platform==="win32"?".exe":""));if(!binary||!facts)throw Error("Actual completed product executable required");buildScriptOutDir=(await captureOwnedCommandV1(binary,[],process.cwd(),{control:view.control,output:{maximumBytes:8192,maximumLines:16}})).trim();const path=relative(facts.buildDirectory,buildScriptOutDir);if(!isAbsolute(buildScriptOutDir)||!path||path===".."||path.startsWith(".."+sep)||isAbsolute(path))throw Error("Build script OUT_DIR escaped the intended product workspace");});}},async plan=>{
   facts={identity:plan.identity,source:plan.facts.source,configuration:plan.facts.configuration,plan:plan.facts,...plan.execution};
   if(mode==="source-change")await writeFile(join(dirname(dirname(manifest)),"base/src/lib.rs"),"pub fn total() -> u8 { 8 }\n");
   if(mode==="manifest-change"){const path=join(dirname(dirname(manifest)),"base/Cargo.toml");await writeFile(path,(await readFile(path,"utf8"))+"\n[features]\ndefault = [\"new-selection\"]\nnew-selection = []\n");}
   if(mode==="pending"){void plan.recheck().then(()=>{pendingFinished=true;});return;}
   if(mode==="produce"||mode==="source-change"||mode==="manifest-change")await plan.produce();else await plan.recheck();
  });
  if(!facts)throw Error("Actual owned plan was never entered");if(mode==="pending"&&!pendingFinished)throw Error("Plan operation escaped input/storage scope");console.log(JSON.stringify({mode,...facts,receiverPid:process.pid,originalOwner:original.policy.owner,consumed,pendingFinished,buildScriptOutDir}));
 });}catch(error){console.log(JSON.stringify({mode,refused:true,consumed,message:error instanceof Error?error.message:String(error)}));throw error;}
});
}
