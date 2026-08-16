#!/usr/bin/env bun
/** 🖨️ `@semio-tech/print` router: `bun ./📜️script.ts generate|fonts|build|watch|test`. */
import { BundleScript, ScriptRouter, runBundleScriptMain } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts";
import { LatexTokenStylesheetGenerationCommand } from "../../🎮️commands/🎨latex-token-stylesheet-generation/🟦️component.ts";
import { PrintFontProvisioningCommand } from "../../🎮️commands/🔤print-font-provisioning/🟦️component.ts";
import { TemplatePdfBuildCommand } from "../../🎮️commands/🖨️template-pdf-build/🟦️component.ts";
import { TemplatePdfWatchCommand } from "../../🎮️commands/👁️template-pdf-watch/🟦️component.ts";
import { PrintPipelineVerificationCommand } from "../../🎮️commands/🧪️print-pipeline-verification/🟦️component.ts";

//#region 🖨️RouterAdapters
class GenerateScript extends BundleScript {
  run(): void {
    new LatexTokenStylesheetGenerationCommand(this.root, this.repoRoot).run();
  }
}

class FontsScript extends BundleScript {
  async run(): Promise<void> {
    await new PrintFontProvisioningCommand(this.root, this.repoRoot).run();
  }
}

class BuildScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await new TemplatePdfBuildCommand(this.root, this.repoRoot).run(segments);
  }
}

class WatchScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await new TemplatePdfWatchCommand(this.root, this.repoRoot).run(segments);
  }
}

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await new PrintPipelineVerificationCommand(this.root, this.repoRoot).run(segments);
  }
}
//#endregion 🖨️RouterAdapters

const router = new ScriptRouter(import.meta.dir)
  .register("generate", GenerateScript)
  .register("fonts", FontsScript)
  .register("build", BuildScript)
  .register("watch", WatchScript)
  .register("test", TestScript);

if (import.meta.main) await runBundleScriptMain(router, import.meta.url, { defaultCommand: "build" });
