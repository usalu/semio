import { BundleScript } from "../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { watchRegisteredPrintTemplates } from "../../🔨️modules/🖨️tectonic-template-compilation/🟦️.ts";

//#region 👁️TemplatePdfWatch
/** 👁️ Watches print inputs and rebuilds requested template PDFs. */
export class TemplatePdfWatchCommand extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await watchRegisteredPrintTemplates(segments);
  }
}
//#endregion 👁️TemplatePdfWatch
