import { receiveScriptProcessInvocation, readIssuedScriptProcessEnvelopeV1 } from "../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {readOwnedBuildInvocationV1} from "../../../../../📥️caller/🟦️.ts";

/** 🏃️ Program of the sibling router: the former top-level body of its `📜️script.ts`, kept verbatim. */
export async function runScriptProgram(): Promise<void> {
  await receiveScriptProcessInvocation(process.env,async original=>{const value=readIssuedScriptProcessEnvelopeV1(original),caller=readOwnedBuildInvocationV1(process.cwd(),original,["run","child:compile"]);let refused=false;try{readOwnedBuildInvocationV1(process.cwd(),original,["run","unrelated:compile"]);}catch{refused=true;}console.log(JSON.stringify({policy:value.policy,deadlineEpochMilliseconds:value.deadlineEpochMilliseconds,native:caller.native,launches:caller.launches,remaining:original.control.remainingMilliseconds(),refused}));});
}
