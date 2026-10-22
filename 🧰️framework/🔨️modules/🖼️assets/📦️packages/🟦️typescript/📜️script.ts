#!/usr/bin/env repo
import { receiveScriptProcessInvocation } from "../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runBudgetedTestCommand } from "../../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🎛️execution/🟦️.ts";
import { testLevelBudgetMs } from "../../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
/** 🧬️ Routes deterministic catalog, metabolism, and animated logo tasks. */
import { resolve } from "node:path";

import { BundleScript, ScriptRouter } from "../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { renderMetabolismArtifacts } from "../../🌱️metabolism/🏗️builder/📽️projection/🟦️.ts";
import { assetOutputManifest, checkAssetArtifacts, publishAssetArtifacts, writeAssetArtifacts } from "../../🏗️builder/📦️publication/🟦️.ts";
import { generateLogoAnimation } from "../../🪧️logos/🏗️builder/🎞️animation/🟦️.ts";
import { runGenerateCatalogScript, runExportLogoScript, runPreviewGeneratedScript } from "../../🧪️tests/🔬️typescript-package-commands/🟦️.ts";
import { isMainModuleV1 } from "../../../⚙️runtime/🏁️main/🟦️.ts";

class GenerateCatalogScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runGenerateCatalogScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, segments);
  }
}

/** 🥽️ Proves schema-owned mesh transport without a concrete product dependency. */
class MeshContractScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("test-mesh-contract accepts no arguments");
    await runBudgetedTestCommand(process.execPath, ["test", resolve(this.root, "../../🥽️mesh/🧪️tests/🧩️suite/🟦️.ts")], { cwd: this.repoRoot , budgetMs: testLevelBudgetMs()});
  }
}

/** 🗺️ Verifies the caller-owned tile transport contract without any concrete product. */
class TileProxyContractScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("test-tile-proxy-contract accepts no arguments");
    await runBudgetedTestCommand(process.execPath, ["test", resolve(this.root, "../../🗺️tile-proxy/🧪️tests/🟦️.ts")], { cwd: this.repoRoot, budgetMs: testLevelBudgetMs() });
  }
}

/** 🧭️ Verifies explicit asset providers and actual transport after concrete owners are removed. */
class AssetDispatchContractScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("test-dispatch-contract accepts no arguments");
    await runBudgetedTestCommand(process.execPath, ["test", resolve(this.root, "../../🔍️resolver/🧭️dispatch/🧪️tests/🟦️.ts")], { cwd: this.repoRoot, budgetMs: testLevelBudgetMs() });
  }
}

/** 🖋️Proves neutral SVG video export with independent native media observation. */
class SvgVideoContractScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("test-svg-video-contract takes no arguments");
    await runBudgetedTestCommand(process.execPath, ["test", resolve(this.repoRoot, "🧰️framework/🔨️modules/🖌️raster/🎥️video/🖋️svg-export/🧪️tests/🟦️.ts")], { cwd: this.repoRoot, budgetMs: testLevelBudgetMs() });
  }
}

class GenerateMetabolismScript extends BundleScript {
  run(): void {
    const artifacts = renderMetabolismArtifacts();
    writeAssetArtifacts(artifacts);
    console.log(`[asset] wrote ${artifacts.length} metabolism artifacts`);
  }
}

class GenerateLogoScript extends BundleScript {
  run(): void {
    generateLogoAnimation();
  }
}

class ExportLogoScript extends BundleScript {
  async run(): Promise<void> {
    await runExportLogoScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation });
  }
}

class BuildScript extends BundleScript {
  run(): void {
    const count = publishAssetArtifacts();
    console.log(`@semio-tech/assets wrote ${count} deterministic outputs.`);
  }
}

class PreviewGeneratedScript extends BundleScript {
  async run(): Promise<void> {
    await runPreviewGeneratedScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation });
  }
}

class CheckGeneratedScript extends BundleScript {
  run(): void {
    checkAssetArtifacts();
    console.log(`@semio-tech/assets ${assetOutputManifest().length} deterministic outputs are fresh.`);
  }
}

const router = new ScriptRouter(import.meta.dirname)
  .register("generate", GenerateCatalogScript)
  .register("test-mesh-contract", MeshContractScript)
  .register("test-tile-proxy-contract", TileProxyContractScript)
  .register("test-dispatch-contract", AssetDispatchContractScript)
  .register("test-svg-video-contract", SvgVideoContractScript)
  .register("generate-metabolism", GenerateMetabolismScript)
  .register("generate-logo", GenerateLogoScript)
  .register("export-logo", ExportLogoScript)
  .register("build", BuildScript)
  .register("preview-generated", PreviewGeneratedScript)
  .register("check-generated", CheckGeneratedScript);

if (isMainModuleV1(import.meta.url)) await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, defaultCommand: "build" }));
