import { BundleScript } from "../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts";
import { provisionPrintFonts } from "../../🔨️modules/🔤print-font-catalog/🟦️component.ts";

//#region 🔤️PrintFontProvisioning
/** 🔤️ Ensures the canonical print TTF catalog is present locally. */
export class PrintFontProvisioningCommand extends BundleScript {
  async run(): Promise<void> {
    const provisioning = await provisionPrintFonts();
    console.log(`print: fonts ready under print/asset/font (${provisioning.downloaded} downloaded, ${provisioning.total} total)`);
  }
}
//#endregion 🔤️PrintFontProvisioning
