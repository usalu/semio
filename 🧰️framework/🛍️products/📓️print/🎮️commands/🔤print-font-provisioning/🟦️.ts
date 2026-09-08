import { BundleScript } from "../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { stagePrintFonts } from "../../🔨️modules/🔤print-font-catalog/🟦️.ts";

//#region 🔤️PrintFontProvisioning
/** 🔤️ Ensures the canonical print TTF catalog is present locally. */
export class PrintFontProvisioningCommand extends BundleScript {
  async run(): Promise<void> {
    const provisioning = await stagePrintFonts();
    console.log(`print: staged ${provisioning.total} authored fonts`);
  }
}
//#endregion 🔤️PrintFontProvisioning
