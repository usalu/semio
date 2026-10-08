import {readFileSync,mkdirSync,mkdtempSync,writeFileSync,rmSync} from "node:fs";
import {isAbsolute,join} from "node:path";
import {validateJsonSchemaSubset} from "../../../🧬️schema/✅️validator/🟦️.ts";
import {captureOwnedProcess,type OwnedProcessCaptureResult} from "../../📥️capture/🟦️.ts";
import {readProcessOwnerContextV1,type ProcessOwnerContextV1} from "../../📋️context/🟦️.ts";
import {readCargoTestPolicyV1,type CargoTestPolicyV1} from "../../🧪️testing/🦀️cargo/🟦️.ts";
import {readVitestPolicyV1,type VitestPolicyV1} from "../../🧪️testing/🧪️vitest/🟦️.ts";
import {readCargoArtifactBuildPolicyV1,type CargoArtifactBuildPolicyV1} from "../../📦️artifacts/🏗️native-build/🟦️.ts";

/** 🎛️ Declares exact executable ownership and required semantic preparation. */
export type CommandRequestV1=Readonly<{version:1;cwd:string;command:string;args:readonly string[];manifests:readonly string[];preparation:"none"|"required"}>;
/** 📋️ Composes the existing defining policies with bounded evidence custody. */
export type CommandPolicyV1=Readonly<{version:1;context:ProcessOwnerContextV1;budgetMs:number;maximumOutputBytes:number;artifactDirectory:string;retainArtifacts:boolean;cargoPolicies:readonly CargoTestPolicyV1[];vitestPolicy:VitestPolicyV1|null;cargoArtifactPolicy:CargoArtifactBuildPolicyV1|null}>;
/** 📊️ Reports command admission, queue waits and completion to its caller. */
export type CommandProgressV1=Readonly<{phase:"started"|"running"|"completed";elapsedMs:number}>;
/** 🔌️ Holds required caller-owned semantic custody for the entire child operation. */
export type CommandPreparationV1=Readonly<{version:1;cwd:string;manifests:readonly string[];release:()=>Promise<void>}>;
/** 🛑️ Supplies observable execution and optional declared host preparation. */
export type CommandOperationV1=Readonly<{environment:Readonly<Record<string,string|undefined>>;cancelled:()=>boolean;onProgress:(event:CommandProgressV1)=>void;prepare?:(request:CommandRequestV1,operation:CommandOperationV1)=>Promise<CommandPreparationV1>}>;
/** 📥️ Returns bounded output and the exact completed execution receipt. */
export type CommandResultV1=OwnedProcessCaptureResult&Readonly<{receiptPath:string}>;
const requestSchema=JSON.parse(readFileSync(new URL("./📥️request/🧬️schema/🔣️.json",import.meta.url),"utf8")),policySchema=JSON.parse(readFileSync(new URL("./📋️policy/🧬️schema/🔣️.json",import.meta.url),"utf8"));

/** 🔐️ Admits variable requests without discovering a workspace or package catalogue. */
export function admitCommandRequestV1(value:unknown):CommandRequestV1{
 if(validateJsonSchemaSubset(requestSchema,value).length)throw Error("Invalid command request");
 const request=value as CommandRequestV1;if(!isAbsolute(request.cwd)||request.manifests.some(path=>!isAbsolute(path))||new Set(request.manifests).size!==request.manifests.length)throw Error("Command custody requires absolute unique paths");return request;
}
function admitPolicy(policy:CommandPolicyV1,request:CommandRequestV1):void{
 if(validateJsonSchemaSubset(policySchema,policy).length||!isAbsolute(policy.artifactDirectory))throw Error("Invalid command policy");
 readProcessOwnerContextV1({SEMIO_PROCESS_OWNER_CONTEXT:JSON.stringify(policy.context)},request.cwd);
 const paths=policy.cargoPolicies.map(value=>readCargoTestPolicyV1({SEMIO_CARGO_TEST_POLICY:JSON.stringify(value)}).manifestPath);
 if(paths.length!==new Set(paths).size||JSON.stringify(paths)!==JSON.stringify(request.manifests))throw Error("Missing or foreign Cargo custody");
 if(policy.vitestPolicy)readVitestPolicyV1({SEMIO_VITEST_POLICY:JSON.stringify(policy.vitestPolicy)},request.cwd);
 if(policy.cargoArtifactPolicy)readCargoArtifactBuildPolicyV1({SEMIO_CARGO_ARTIFACT_POLICY:JSON.stringify(policy.cargoArtifactPolicy)},request.cwd);
 if(request.manifests.length&&!policy.cargoArtifactPolicy)throw Error("Missing Cargo custody for compiler artifacts");
}

/** ▶️ Executes one exact request with mandatory controls and completed artifact evidence. */
export async function executeCommandV1(value:CommandRequestV1,policy:CommandPolicyV1,operation:CommandOperationV1):Promise<CommandResultV1>{
 const request=admitCommandRequestV1(value);admitPolicy(policy,request);if(request.preparation==="required"&&!operation.prepare)throw Error("Required command preparation port missing");
 let preparation:CommandPreparationV1|undefined;
 try{
  if(request.preparation==="required"){preparation=await operation.prepare!(request,operation);if(preparation.version!==1||preparation.cwd!==request.cwd||JSON.stringify(preparation.manifests)!==JSON.stringify(request.manifests))throw Error("Required command preparation custody mismatch");}
  mkdirSync(policy.artifactDirectory,{recursive:true});const directory=mkdtempSync(join(policy.artifactDirectory,"command-")),started=Date.now(),receiptPath=join(policy.artifactDirectory,`${directory.split(/[\\/]/u).at(-1)}-receipt.json`),environment:Record<string,string|undefined>={...operation.environment,SEMIO_TEST_ARTIFACT_DIR:policy.artifactDirectory,SEMIO_PROCESS_OWNER_CONTEXT:JSON.stringify(policy.context),SEMIO_CARGO_TEST_POLICIES:JSON.stringify(policy.cargoPolicies)};
  delete environment.SEMIO_CARGO_TEST_POLICY;delete environment.SEMIO_VITEST_POLICY;delete environment.SEMIO_CARGO_ARTIFACT_POLICY;
  if(policy.cargoPolicies[0])environment.SEMIO_CARGO_TEST_POLICY=JSON.stringify(policy.cargoPolicies[0]);if(policy.vitestPolicy)environment.SEMIO_VITEST_POLICY=JSON.stringify(policy.vitestPolicy);if(policy.cargoArtifactPolicy)environment.SEMIO_CARGO_ARTIFACT_POLICY=JSON.stringify(policy.cargoArtifactPolicy);
  operation.onProgress({phase:"started",elapsedMs:0});const progress=setInterval(()=>operation.onProgress({phase:"running",elapsedMs:Date.now()-started}),10000);let result:OwnedProcessCaptureResult;
  try{result=await captureOwnedProcess(request.command,[...request.args],{cwd:request.cwd,env:environment,budgetMs:policy.budgetMs,maxOutputBytes:policy.maximumOutputBytes,stdoutPath:join(directory,"stdout.txt"),stderrPath:join(directory,"stderr.txt"),cancelled:operation.cancelled});}finally{clearInterval(progress);}
  writeFileSync(receiptPath,JSON.stringify({version:1,request,policy,startedAtMs:started,completedAtMs:Date.now(),status:result.status,signal:result.signal,reason:result.reason}));operation.onProgress({phase:"completed",elapsedMs:Date.now()-started});const completed={...result,receiptPath};if(!policy.retainArtifacts)rmSync(directory,{recursive:true,force:true});return completed;
 }finally{await preparation?.release();}
}
