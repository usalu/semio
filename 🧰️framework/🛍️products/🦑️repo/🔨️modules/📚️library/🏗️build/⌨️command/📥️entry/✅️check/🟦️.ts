import {createHash} from "node:crypto";
import {buildSourceClosureStreamLoaderV1} from "../../../📥️source/🔏️closure/🟦️.ts";
import type {CapturedBuildCommandProofV1} from "../../../📥️source/🔏️closure/🚪️command/🟦️.ts";
import policy from "../../../🏛️sealed/🔣️.json" with {type:"json"};

/** 🧪️ Validates the loaded partition against the loader that authenticated its captured source. */
export function answerKernelCheckV1(args:readonly string[],proof:CapturedBuildCommandProofV1):boolean{
 if(args.length!==1||args[0]!==policy.check.argument+proof.sha256)return false;
 const loaderSha256=createHash("sha256").update(buildSourceClosureStreamLoaderV1).digest("hex"),matches=loaderSha256===proof.loaderSha256;
 process.stdout.write(JSON.stringify({check:"loaded",role:"kernel",matches,loaderSha256,...(matches?{}:{loader:buildSourceClosureStreamLoaderV1})})+"\n");return true;
}
