import { verifyVisualizationCoverage } from "../../🔨️modules/📊️visualization-gallery/🟦️.ts";
import { verifyPrintVisualizationBuild } from "./🧪️tests/🖨️pipeline/🟦️.ts";
import { BundleScript, TEST_LEVELS, resolveTestLevel } from "../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { verifyPrintMacroStagingNative, verifyPrintPipelineLong, verifyPrintPipelineQuick } from "./🧪️tests/🖨️pipeline/🟦️.ts";

//#region 🧪️PrintPipelineVerification
/** 🧪️ Verifies pure print transformations and, at long level, every template PDF output. */
export class PrintPipelineVerificationCommand extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments[0] === "macro") {
      if (segments.length !== 1) throw new Error("print macro staging test accepts no additional segments");
      await verifyPrintMacroStagingNative();
      return;
    }
    if (segments[0] === "viz") {
      const mode = segments[1] ?? "coverage";
      if (!["quick", "coverage", "full"].includes(mode)) throw new Error(`unknown viz test mode: ${mode}`);
      await verifyPrintPipelineQuick();
      verifyVisualizationCoverage();
      if (mode === "full") await verifyPrintVisualizationBuild();
      return;
    }
    const { level } = resolveTestLevel(segments);
    await verifyPrintPipelineQuick();
    if (TEST_LEVELS.indexOf(level) >= TEST_LEVELS.indexOf("long")) await verifyPrintPipelineLong();
  }
}
//#endregion 🧪️PrintPipelineVerification
