import { verifyVisualizationCoverage } from "../../🔨️modules/📊️visualization-gallery/🟦️.ts";

//#region 🧪️PrintPipelineVerification
/** 🧪️ Verifies pure print transformations, the platform cases of the print owner, and the rendered gallery. */
export class PrintPipelineVerificationCommand extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments[0] === "macro") {
      if (segments.length !== 1) throw new Error("print macro staging test accepts no additional segments");
      await verifyPrintMacroStagingNative();
      return;
    }
    if (segments[0] === "viz") {
      const mode = segments[1] ?? "coverage";
      if (mode === "fixtures") {
        await regeneratePrintGalleryFixtures(segments.slice(2));
        return;
      }
      if (!["quick", "coverage", "full"].includes(mode)) throw new Error(`unknown viz test mode: ${mode}`);
      await verifyPrintPipelineQuick();
      verifyVisualizationCoverage();
      runPrintPlatformCases(mode === "full" ? "long" : "quick");
      if (mode === "full") await verifyPrintVisualizationBuild();
      return;
    }
    const { level } = resolveTestLevel(segments);
  }
}
//#endregion 🧪️PrintPipelineVerification
