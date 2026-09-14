import { BundleScript } from "../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { generateVizArtifacts } from "../../🔨️modules/📊️visualization-gallery/🟦️.ts";

//#region 📊️VizCatalogGeneration
/** 📊️ Emits the generated visualization catalogue packages and the per-section gallery documents. */
export class VizCatalogGenerationCommand extends BundleScript {
  run(): void {
    const written = generateVizArtifacts(this.repoRoot);
    console.log(`print: wrote ${written.length} visualization catalogue artifacts`);
  }
}
//#endregion 📊️VizCatalogGeneration
