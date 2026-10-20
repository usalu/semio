import { receiveScriptProcessInvocation } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { ScriptRouter } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { PrepareScript, FingerprintScript } from "./🟦️.ts";


if (import.meta.main) await receiveScriptProcessInvocation(process.env, original => (new ScriptRouter(import.meta.dir).register("prepare", PrepareScript).register("fingerprint", FingerprintScript)).run(process.argv.slice(2), original));