import {RustUnitScript} from "../../🧪️tests/🔬️script-commands/🟦️.ts";
import { receiveScriptProcessInvocation, ScriptRouter } from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";

const router=new ScriptRouter(import.meta.dirname).register("unit",RustUnitScript);
/** 🏃️ Program of the sibling router: the former top-level body of its `📜️script.ts`, kept verbatim. */
export async function runScriptProgram(): Promise<void> {
  await receiveScriptProcessInvocation(process.env,original=>router.run(process.argv.slice(2),original));
}
