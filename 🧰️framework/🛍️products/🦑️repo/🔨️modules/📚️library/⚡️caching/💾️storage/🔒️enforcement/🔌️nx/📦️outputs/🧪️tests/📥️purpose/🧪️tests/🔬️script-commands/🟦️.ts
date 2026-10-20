import { receiveScriptProcessInvocation } from "../../../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runOutputNegativePurposeV1, type OutputNegativeModeV1 } from "../../🟦️.ts";


/** 🏃️ Program of the sibling router: the former top-level body of its `📜️script.ts`, kept verbatim. */
export async function runScriptProgram(): Promise<void> {
  const mode=process.argv[2];if(process.argv.length!==3||mode!=="actual"&&mode!=="ledger-model")throw new Error("Closed output negative worker required");await receiveScriptProcessInvocation(process.env,original=>runOutputNegativePurposeV1(original,mode as OutputNegativeModeV1));
}
