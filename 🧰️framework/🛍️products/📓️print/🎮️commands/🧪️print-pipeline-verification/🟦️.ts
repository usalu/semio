import { verifyVisualizationCoverage } from "../../🔨️modules/📊️visualization-gallery/🟦️.ts";
import { verifyPrintVisualizationBuild } from "./🧪️tests/🟦️.ts";
import { BundleScript, TEST_LEVELS, resolveTestLevel } from "../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { verifyPrintPipelineLong, verifyPrintPipelineQuick } from "./🧪️tests/🟦️.ts";

//#region 🧪️PrintPipelineVerification
/** 🧪️ Verifies pure print transformations and, at long level, every template PDF output. */
export class PrintPipelineVerificationCommand extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments[0] === "viz") {
      const mode = segments[1] ?? "coverage";
      if (!["quick", "coverage", "full"].includes(mode)) throw new Error(`unknown viz test mode: ${mode}`);
      verifyPrintPipelineQuick();
      verifyVisualizationCoverage();
      if (mode === "full") await verifyPrintVisualizationBuild();
      return;
    }
    const { level } = resolveTestLevel(segments);
    verifyPrintPipelineQuick();
    if (TEST_LEVELS.indexOf(level) >= TEST_LEVELS.indexOf("long")) await verifyPrintPipelineLong();
  }
}
//#endregion 🧪️PrintPipelineVerification
