import { receiveScriptProcessInvocation } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { Script, ScriptRouter } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";

/** 🏁️ Completes the default production graph after the build tool has built or restored its deliverables. */
class CompleteScript extends Script {
  run(args: string[]): void {
    if (args.length) throw new Error("Select a build-<variant>-react-release target to build a specific production variant");
    console.log("Production build complete: s/react/release");
  }
}

if (import.meta.main) await receiveScriptProcessInvocation(process.env, original => (new ScriptRouter(import.meta.dir).register("complete", CompleteScript)).run(process.argv.slice(2), original));
