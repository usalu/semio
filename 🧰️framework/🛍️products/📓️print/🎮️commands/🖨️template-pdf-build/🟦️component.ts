import { BundleScript } from "../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts";
import { buildRegisteredPrintTemplates } from "../../🔨️modules/🖨️tectonic-template-compilation/🟦️component.ts";

//#region 🖨️TemplatePdfBuild
/** 🖨️ Builds requested registered print templates as light and dark PDFs. */
export class TemplatePdfBuildCommand extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await buildRegisteredPrintTemplates(segments);
  }
}
//#endregion 🖨️TemplatePdfBuild
