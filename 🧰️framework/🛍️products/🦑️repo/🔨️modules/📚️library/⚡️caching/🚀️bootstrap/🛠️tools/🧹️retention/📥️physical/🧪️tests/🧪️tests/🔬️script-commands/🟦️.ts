import { receiveScriptProcessInvocation } from "../../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";

/** 🏃️ Receives one closed Bun retention refusal purpose. */
export async function runScriptProgram():Promise<void>{
 const [,,,workspace,purpose]=process.argv;
 if(process.argv[2]!=="negative"||process.argv.length!==5||!workspace||!purpose)throw Error("Closed retention worker purpose required");
 const {runRetentionNegativePurposeV1}=await import("../../📥️negative/🟦️.ts");
 await receiveScriptProcessInvocation(process.env,original=>runRetentionNegativePurposeV1(original,workspace,purpose));
}
