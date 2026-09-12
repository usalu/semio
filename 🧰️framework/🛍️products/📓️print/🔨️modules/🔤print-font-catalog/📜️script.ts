import { BundleScript, ScriptRouter } from "../../../🦑️repo/🔨️modules/📚️library/🏃️process/🧭️routing/🟦️.ts";
import { stagePrintFonts } from "./🟦️.ts";

class BuildScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length) throw new Error("Print font preparation accepts no arguments");
    const result = await stagePrintFonts(this.repoRoot);
    console.log(`Print fonts staged: ${result.total} authored TTF assets`);
  }
}

const router = new ScriptRouter(import.meta.dir).register("build", BuildScript);
if (import.meta.main) await router.run(process.argv.slice(2));
