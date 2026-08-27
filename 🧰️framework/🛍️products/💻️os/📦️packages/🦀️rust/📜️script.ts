#!/usr/bin/env bun
/** 🦀️ `@semio-tech/framework-os-kernel` task router. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runCargo } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts";
import { runNestedCargoPackageAdapter } from "../../../../../📜️script.ts";

//#region 🔎️ScalarWireSource
class ScalarWireSourceScript extends BundleScript {
  async run(): Promise<void> {
    const { testScalarRecordWireFixture } = await import("../../🔨️modules/🎒️pack/🔎️scalar-witness/📜️script.ts");
    testScalarRecordWireFixture();
  }
}
//#endregion 🔎️ScalarWireSource

class CheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runCargo(["check", "--manifest-path", "Cargo.toml", ...segments], this.root);
  }
}

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runCargo(["test", "--manifest-path", "Cargo.toml", "--lib", ...segments], this.root);
  }
}

//#region 🧩️JCO Package Adapter
class GenerateJcoPackageAdapterScript extends BundleScript {
  run(): void { runNestedCargoPackageAdapter(this.repoRoot, "generate"); }
}
class PreviewGeneratedScript extends BundleScript {
  run(): void { runNestedCargoPackageAdapter(this.repoRoot, "preview"); }
}
class CheckJcoPackageAdapterScript extends BundleScript {
  run(): void { runNestedCargoPackageAdapter(this.repoRoot, "check"); }
}
//#endregion 🧩️JCO Package Adapter

const router = new ScriptRouter(import.meta.dir).register("check", CheckScript).register("test", TestScript).register("test-scalar-wire-source", ScalarWireSourceScript).register("generate-jco-package-adapter", GenerateJcoPackageAdapterScript).register("preview-generated", PreviewGeneratedScript).register("check-jco-package-adapter", CheckJcoPackageAdapterScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "check" });
