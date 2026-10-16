import { BundleScript } from "../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import{createIssuedResourceControlV1}from"../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import{withPrintExecutionV1}from"../../📦️artifacts/🎛️operation/🟦️.ts";
import { stagePrintFonts } from "../../🔨️modules/🔤print-font-catalog/🟦️.ts";

//#region 🔤️PrintFontProvisioning
/** 🔤️ Ensures the canonical print TTF catalog is present locally. */
export class PrintFontProvisioningCommand extends BundleScript {
  async run(): Promise<void> {
    const phaseControl=createIssuedResourceControlV1(this.invocation,"print:fonts:prepare",0),provisioning=await withPrintExecutionV1({workspace:this.repoRoot,invocation:this.invocation,phaseControl,kind:"fonts"},stagePrintFonts);
    console.log(`print: staged ${provisioning.files} authored fonts`);
  }
}
//#endregion 🔤️PrintFontProvisioning
