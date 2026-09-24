import { Script, ScriptRouter } from "../../../../../🦑️repo/🔨️modules/📚️library/🏃️process/🧭️routing/🟦️.ts";

/** 🏁️ Completes the default production graph after Nx has built or restored its deliverables. */
class CompleteScript extends Script {
  run(args: string[]): void {
    if (args.length) throw new Error("Select a build-<variant>-react-release target to build a specific production variant");
    console.log("Nx production build complete: s/react/release");
  }
}

if (import.meta.main) await new ScriptRouter(import.meta.dir).register("complete", CompleteScript).run(process.argv.slice(2));
