#!/usr/bin/env bun
/** 🧬️ Routes deterministic catalog, metabolism, and animated logo tasks. */
import { resolve } from "node:path";
import { runTestBudgeted } from "../../../../🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
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
    await runTestBudgeted(process.execPath, ["test", resolve(this.root, "../../🥽️mesh/🧪️tests/🧩️suite/🟦️.ts")], { cwd: this.repoRoot });
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
  .register("test-mesh-contract", MeshContractScript)
  .register("generate-metabolism", GenerateMetabolismScript)
  .register("generate-logo", GenerateLogoScript)
  .register("export-logo", ExportLogoScript)
  .register("build", BuildScript)
  .register("preview-generated", PreviewGeneratedScript)
  .register("check-generated", CheckGeneratedScript);

if (import.meta.main) await runScriptMain(router, { defaultCommand: "build" });
