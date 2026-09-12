#!/usr/bin/env bun
/** @emoji 🧬️ Routes deterministic catalog, metabolism, and animated logo tasks. */
import { BundleScript, ScriptRouter, runBundleScriptMain } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
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
    await exportLogoAnimation(this.repoRoot);
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
  .register("generate-metabolism", GenerateMetabolismScript)
  .register("generate-logo", GenerateLogoScript)
  .register("export-logo", ExportLogoScript)
  .register("build", BuildScript)
  .register("preview-generated", PreviewGeneratedScript)
  .register("check-generated", CheckGeneratedScript);

if (import.meta.main) await runBundleScriptMain(router, import.meta.url, { defaultCommand: "build" });
