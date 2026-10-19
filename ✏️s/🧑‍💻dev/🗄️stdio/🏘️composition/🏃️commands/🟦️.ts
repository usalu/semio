import { BundleScript } from "@semio-tech/process/routing";
import { buildStdioComposition, checkStdioComposition } from "../🏗️build/🟦️.ts";

/** 📦️ Builds the Stdio composition package. */
export class StdioCompositionBuildScript extends BundleScript {
  async run(): Promise<void> {
    await buildStdioComposition(this.root, this.invocation);
  }
}

/** 🔎️ Checks the Stdio composition package. */
export class StdioCompositionCheckScript extends BundleScript {
  async run(): Promise<void> {
    await checkStdioComposition(this.root, this.invocation);
  }
}

/** 🧪️ Builds and consumes the Stdio composition package. */
export class StdioCompositionTestScript extends BundleScript {
  async run(): Promise<void> {
    const { testStdioComposition } = await import("../../🧪️tests/🧩️composition-consumption/🟦️.ts");
    const { testStdioOfficeSchemaContracts } = await import("../../🧪️tests/📚️office-schema-contract/🟦️.ts");
    await testStdioComposition(this.root, this.invocation);
    await testStdioOfficeSchemaContracts(this.invocation);
  }
}
