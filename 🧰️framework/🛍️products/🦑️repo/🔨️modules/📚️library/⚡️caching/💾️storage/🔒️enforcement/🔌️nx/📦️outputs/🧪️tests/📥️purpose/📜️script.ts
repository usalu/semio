import{receiveScriptProcessInvocation}from"../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import{runOutputNegativePurposeV1,type OutputNegativeModeV1}from"./🟦️.ts";
const mode=process.argv[2];if(process.argv.length!==3||mode!=="actual"&&mode!=="ledger-model")throw new Error("Closed output negative worker required");await receiveScriptProcessInvocation(process.env,original=>runOutputNegativePurposeV1(original,mode as OutputNegativeModeV1));
