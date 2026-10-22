import { receiveScriptProcessInvocation } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { ScriptRouter } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { PrepareScript, FingerprintScript } from "./🟦️.ts";
import { isMainModuleV1 } from "../../../../../../../../🔨️modules/⚙️runtime/🏁️main/🟦️.ts";


if (isMainModuleV1(import.meta.url)) await receiveScriptProcessInvocation(process.env, original => (new ScriptRouter(import.meta.dirname).register("prepare", PrepareScript).register("fingerprint", FingerprintScript)).run(process.argv.slice(2), original));