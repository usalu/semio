import { BundleScript } from "../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { writePrintLatexTokenStylesheet } from "../../🔨️modules/🎨print-design-token-paints/🟦️.ts";

//#region 🎨️LatexTokenStylesheetGeneration
/** 🎨️ Writes the canonical LaTeX stylesheet from the framework design-token document. */
export class LatexTokenStylesheetGenerationCommand extends BundleScript {
  run(): void {
    writePrintLatexTokenStylesheet();
    console.log("print: wrote latex/semio-tokens.sty");
  }
}
//#endregion 🎨️LatexTokenStylesheetGeneration
