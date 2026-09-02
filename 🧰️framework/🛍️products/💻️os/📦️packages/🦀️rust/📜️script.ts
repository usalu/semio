#!/usr/bin/env bun
/** 🦀️ `@semio-tech/framework-os-kernel` task router. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runCargo, resolveTestLevel, runCargoTestBudgeted } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
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

class NativeTestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runCargoTestBudgeted(["semio-framework-os-kernel"], this.repoRoot, ["--lib", "--features", "sync,ureq", ...rest]);
  }
}

class DirectoryRuntimeSourceScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("test-directory-runtime-source accepts no arguments");
    const { testDirectoryRuntimeIdentityFixture } = await import("../../🔨️modules/📇️directory/🔌️client/🪪️runtime/📜️script.ts");
    testDirectoryRuntimeIdentityFixture();
  }
}

class CodecSendSourceScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("test-codec-send-source accepts no arguments");
    const { testNativeCodecSendFixture } = await import("../../🔨️modules/🏪️store/📦️codec/🧵️send/📜️script.ts");
    testNativeCodecSendFixture();
  }
}

class BackboneDetachSourceScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("test-backbone-detach-source accepts no arguments");
    const { testBackboneDetachFixture } = await import("../../🔨️modules/🏪️store/🔗️backbone/✂️detach/📜️script.ts");
    testBackboneDetachFixture();
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

const router = new ScriptRouter(import.meta.dir).register("check", CheckScript).register("test", TestScript).register("test-scalar-wire-source", ScalarWireSourceScript).register("generate-jco-package-adapter", GenerateJcoPackageAdapterScript).register("preview-generated", PreviewGeneratedScript).register("check-jco-package-adapter", CheckJcoPackageAdapterScript).register("test-native", NativeTestScript).register("test-directory-runtime-source", DirectoryRuntimeSourceScript).register("test-codec-send-source", CodecSendSourceScript).register("test-backbone-detach-source", BackboneDetachSourceScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "check" });
