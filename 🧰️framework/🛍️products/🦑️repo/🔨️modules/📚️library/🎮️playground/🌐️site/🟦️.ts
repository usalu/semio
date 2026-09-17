/** @emoji 🌐️ Builds CDN-ready standalone playground sites from plugin `📜️script.ts build`. */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { BundleScript, type ScriptRouter } from "../../🏃️process/🧭️routing/🟦️.ts";
import { runBun } from "../../🟦️.ts";

const DISTRIBUTION_BUILD_SCRIPT = "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚚️distribution/📜️script.ts";

/** @emoji 🎮️ Playground variant ids declared on the crate next to this package's `📜️script.ts`. */
export function playgroundVariantsFromCrateManifest(crateManifestPath: string): readonly string[] {
  const text = readFileSync(crateManifestPath, "utf8");
  const variants: string[] = [];
  for (const block of text.split("[[package.metadata.semio.playground]]").slice(1)) {
    const variant = block.match(/^variant\s*=\s*"([^"]+)"/m)?.[1];
    if (!variant) throw new Error(`Invalid playground block in ${crateManifestPath}`);
    variants.push(variant);
  }
  return variants;
}

/** @emoji 🏗️ Registers `build [<variant>|all]` to publish the react release distribution for this plugin crate. */
export function registerPlaygroundSiteBuildCommands(router: ScriptRouter): void {
  class BuildScript extends BundleScript {
    async run(segments: string[]): Promise<void> {
      const manifestPath = join(this.root, "Cargo.toml");
      const variants = [...playgroundVariantsFromCrateManifest(manifestPath)];
      if (variants.length === 0) throw new Error("This crate declares no [[package.metadata.semio.playground]] rows");
      const mode = segments[0] ?? (variants.length === 1 ? variants[0] : undefined);
      if (!mode) throw new Error(`Usage: build <${variants.join("|")}|all>`);
      const selected = mode === "all" ? variants : variants.includes(mode) ? [mode] : undefined;
      if (!selected) throw new Error(`Unknown playground variant ${JSON.stringify(mode)} — expected ${variants.join(", ")} or all`);
      const buildScript = join(this.repoRoot, DISTRIBUTION_BUILD_SCRIPT);
      for (const variant of selected) {
        runBun([buildScript, "build", variant, "react", "release"], this.repoRoot);
      }
    }
  }
  router.register("build", BuildScript);
}
