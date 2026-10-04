#!/usr/bin/env bun
/** 🖨️ `@semio-tech/print` router: `bun ./📜️script.ts fonts|generate viz|preview-generated|test`. */
import { BundleScript, ScriptRouter } from "../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
import { PrintFontProvisioningCommand } from "../../🎮️commands/🔤print-font-provisioning/🟦️.ts";
import { PrintPipelineVerificationCommand } from "../../🎮️commands/🧪️print-pipeline-verification/🟦️.ts";
import { PrintTokenPreviewScript } from "../../🔨️modules/🎨print-design-token-paints/📜️script.ts";
import { generateVizArtifacts } from "../../🔨️modules/📊️visualization-gallery/🟦️.ts";

//#region 🖨️RouterAdapters
class FontsScript extends BundleScript {
  async run(): Promise<void> {
    await new PrintFontProvisioningCommand(this.root, this.repoRoot).run();
  }
}

class GenerateScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length !== 1 || segments[0] !== "viz") throw new Error("Expected generate viz");
    for (const path of generateVizArtifacts(this.repoRoot)) console.log(`[print-generate] ${path}`);
  }
}

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments[0] === "native-grammar") {
      const { tmpdir } = await import("node:os");
      const { join } = await import("node:path");
      const workDir = process.env.PRINT_NATIVE_GRAMMAR_WORK_DIR ?? join(tmpdir(), "semio-print-native-grammar");
      if (segments.includes("--families-only")) {
        const { compileNativeFamilyContainment } = await import("../../🧪️tests/🖼️family-containment/🟦️.ts");
        await compileNativeFamilyContainment(workDir);
        return;
      }
      const { compileNativeGrammar } = await import("../../🧪️tests/🧬️native-chart-grammar/🟦️.ts");
      await compileNativeGrammar(this.repoRoot, workDir, !segments.includes("--grammar-only"));
      return;
    }
    await new PrintPipelineVerificationCommand(this.root, this.repoRoot).run(segments);
  }
}
//#endregion 🖨️RouterAdapters

const router = new ScriptRouter(import.meta.dir)
  .register("fonts", FontsScript)
  .register("generate", GenerateScript)
  .register("preview-generated", PrintTokenPreviewScript)
  .register("test", TestScript);

if (import.meta.main) await runScriptMain(router, { defaultCommand: "test" });
