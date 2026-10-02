#!/usr/bin/env bun
import { createChromiumSvgVideoRuntimeV1 } from "../../../🖌️raster/🎥️video/🖋️svg-export/🌐️browser/🟦️.ts";
import { runBudgetedTestCommand } from "../../../🏃️process/🧪️testing/🎛️execution/🟦️.ts";
import { testLevelBudgetMs } from "../../../🏃️process/🧪️testing/🎚️budget/🟦️.ts";
/** 🧬️ Routes deterministic catalog, metabolism, and animated logo tasks. */
import { resolve } from "node:path";

import { BundleScript, ScriptRouter } from "../../../🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
import { renderCatalogArtifacts } from "../../🔣️icons/🏗️builder/📽️projection/🟦️.ts";
import { renderMetabolismArtifacts } from "../../🌱️metabolism/🏗️builder/📽️projection/🟦️.ts";
import { assetOutputManifest, checkAssetArtifacts, previewAssetArtifacts, publishAssetArtifacts, writeAssetArtifacts } from "../../🏗️builder/📦️publication/🟦️.ts";
import { exportLogoAnimation, generateLogoAnimation } from "../../🪧️logos/🏗️builder/🎞️animation/🟦️.ts";

class GenerateCatalogScript extends BundleScript {
  run(segments: string[]): void {
    const target = (segments[0] ?? "all").toLowerCase();
    if (!["js", "net", "py", "rust", "all"].includes(target)) throw new Error(`Unknown catalog generate target: ${target}`);
    const artifacts = renderCatalogArtifacts(target);
    writeAssetArtifacts(artifacts);
    console.log(`[asset] wrote ${artifacts.length} catalog artifacts → ${target}`);
  }
}

/** 🥽️ Proves schema-owned mesh transport without a concrete product dependency. */
class MeshContractScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw Error("test-mesh-contract accepts no arguments");
    await runBudgetedTestCommand(process.execPath, ["test", resolve(this.root, "../../🥽️mesh/🧪️tests/🧩️suite/🟦️.ts")], { cwd: this.repoRoot , budgetMs: testLevelBudgetMs()});
  }
}

/** 🗺️ Verifies the caller-owned tile transport contract without any concrete product. */
class TileProxyContractScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw Error("test-tile-proxy-contract accepts no arguments");
    await runBudgetedTestCommand(process.execPath, ["test", resolve(this.root, "../../🗺️tile-proxy/🧪️tests/🟦️.ts")], { cwd: this.repoRoot, budgetMs: testLevelBudgetMs() });
  }
}

/** 🧭️ Verifies explicit asset providers and actual transport after concrete owners are removed. */
class AssetDispatchContractScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw Error("test-dispatch-contract accepts no arguments");
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
    const artifactRoot = process.env.SEMIO_TEST_ARTIFACTS_DIR;
    if (!artifactRoot) throw new Error("Logo video export requires caller-owned SEMIO_TEST_ARTIFACTS_DIR");
    const runtime = await createChromiumSvgVideoRuntimeV1();
    try {
      await exportLogoAnimation({ artifactRoot, openBrowser: runtime.openBrowser, encoderExecutable: "ffmpeg" });
    } finally {
      await runtime.close();
    }
  }
}

class BuildScript extends BundleScript {
  run(): void {
    const count = publishAssetArtifacts();
    console.log(`@semio-tech/assets wrote ${count} deterministic outputs.`);
  }
}

class PreviewGeneratedScript extends BundleScript {
  run(): void {
    process.stdout.write(previewAssetArtifacts(this.repoRoot));
  }
}

class CheckGeneratedScript extends BundleScript {
  run(): void {
    checkAssetArtifacts();
    console.log(`@semio-tech/assets ${assetOutputManifest().length} deterministic outputs are fresh.`);
  }
}

const router = new ScriptRouter(import.meta.dir)
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

if (import.meta.main) await runScriptMain(router, { defaultCommand: "build" });
