import { BundleScript, TEST_LEVELS, resolveTestLevel } from "../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts";
import { verifyPrintPipelineLong, verifyPrintPipelineQuick } from "./🧪️tests/🟦️test.ts";

//#region 🧪️PrintPipelineVerification
/** 🧪️ Verifies pure print transformations and, at long level, every template PDF output. */
export class PrintPipelineVerificationCommand extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { level } = resolveTestLevel(segments);
    verifyPrintPipelineQuick();
    if (TEST_LEVELS.indexOf(level) >= TEST_LEVELS.indexOf("long")) await verifyPrintPipelineLong();
  }
}
//#endregion 🧪️PrintPipelineVerification
