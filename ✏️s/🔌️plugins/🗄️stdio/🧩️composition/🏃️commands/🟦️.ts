import { BundleScript } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { buildStdioComposition, checkStdioComposition, testStdioComposition } from "../../🏗️build/🟦️.ts";

/** 📦️ Builds the Stdio composition package. */
export class StdioCompositionBuildScript extends BundleScript {
  async run(): Promise<void> {
    await buildStdioComposition(this.root);
  }
}

/** 🔎️ Checks the Stdio composition package. */
export class StdioCompositionCheckScript extends BundleScript {
  async run(): Promise<void> {
    await checkStdioComposition(this.root);
  }
}

/** 🧪️ Builds and consumes the Stdio composition package. */
export class StdioCompositionTestScript extends BundleScript {
  async run(): Promise<void> {
    await testStdioComposition(this.root);
  }
}
