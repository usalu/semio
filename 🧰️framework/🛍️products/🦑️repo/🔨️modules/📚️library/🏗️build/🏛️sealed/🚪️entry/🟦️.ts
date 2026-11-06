import{createHash}from"node:crypto";
import{writeSync}from"node:fs";
import{fileURLToPath}from"node:url";
import{buildSourceClosureStreamLoaderV1}from"../../📥️source/🔏️closure/🟦️.ts";
import type{CapturedBuildCommandProofV1}from"../../📥️source/🔏️closure/🚪️command/🟦️.ts";
import{sealedPolicyV1}from"../🟦️.ts";
import{adoptPartitionPinV1,releasePartitionPinV1}from"../🔀️partition/📌️retention/🟦️.ts";
/** 🏷️ The first argument that starts the front door instead of a verb kernel. */
export const sealedDoorRoleV1="--semio-role=door";
/** 🔥️ Declares the storage image needed by the authenticated resident warm manifest. */
export const sealedWarmStorageV1=()=>import("../../🔧️tools/💾️storage/🟦️.ts");
const checkV1=(proof:CapturedBuildCommandProofV1)=>sealedPolicyV1.check.argument+proof.sha256;

/** 🧪️ Checks the authenticated door and its loader before promotion. */
async function answerCheckV1(proof:CapturedBuildCommandProofV1):Promise<void>{
 await import("../../🚪️door/🏃️main/🟦️.ts");
 const loaderSha256=createHash("sha256").update(buildSourceClosureStreamLoaderV1).digest("hex"),matches=loaderSha256===proof.loaderSha256;
 process.stdout.write(JSON.stringify({check:"loaded",matches,loaderSha256,...(matches?{}:{loader:buildSourceClosureStreamLoaderV1})})+"\n");
}
/** 🚪️ Runs the front door from the sealed snapshot: the entry script of the next restart is the one the user started, so a restart starts the next snapshot from the store; the supervisor of the door needs no other knowledge. */
async function runDoorV1(argv:readonly string[],proof:CapturedBuildCommandProofV1,ready:number):Promise<void>{
 const inherited=process.env.SEMIO_SEALED_PARTITION_PIN,adopted=inherited?adoptPartitionPinV1(proof.root,inherited):undefined;
 if(adopted)process.env.SEMIO_SEALED_PARTITION_PIN=adopted;
 try{writeSync(ready,"ready\n");}catch{}
 process.env.SEMIO_SEALED_DOOR_DIGEST=proof.sha256;
 process.env[sealedPolicyV1.modeVariable]="sealed";
 process.argv=[process.execPath,fileURLToPath(new URL("../../🚪️door/📜️script.ts",import.meta.url)),...argv];process.execArgv=[];
 const{runBuildDoorMainV1}=await import("../../🚪️door/🏃️main/🟦️.ts");
 try{await runBuildDoorMainV1(proof.root,argv,process.env);}finally{if(adopted)releasePartitionPinV1(proof.root,adopted);}
}
/** 🏛️ Runs the authenticated front door or its promotion check. */
export async function runCapturedBuildSourceV1(args:readonly string[],proof:CapturedBuildCommandProofV1):Promise<void>{
 if(args.length===1&&args[0]===checkV1(proof))return answerCheckV1(proof);
 if(args[0]===sealedDoorRoleV1)return runDoorV1(args.slice(1),proof,sealedPolicyV1.readyDescriptor);
 throw Error("Captured door requires its declared role");
}
