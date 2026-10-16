/** 🧭️ `generate` command family: derived config, taxonomy census and plugin-glue inventory.
 * @see 📜️script.ts
 */
import { runNestedCargoPackageAdapter } from "../../📽️projection/🧩️package-adapter/📦️publication/🟦️.ts";
import { runCmd } from "../../📦️packages/🟦️typescript/🟦️.ts";
import { Script } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { checkDerivedConfig, writeDerivedConfig } from "../../⚡️caching/📦️artifacts/📝️derived-config/🟦️.ts";
import { buildSemanticCensus, renderSemanticCensusJson, renderSemanticCensusMarkdown, renderSemanticDuplicatesJson, renderSemanticDuplicatesMarkdown } from "../../🔍️discovery/🟦️.ts";
import { taxonomyTicketDirectory } from "../../🧹️normalization/🎮️command-contract/🟦️.ts";
import { existsSync, readdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { policyListTopLevelSubsetDirs } from "../../⚖️laws/🧩️plugin-consistency/🪆️subset-conformance/🟦️.ts";
import { taxonomyOption } from "../../🧹️normalization/🔣️taxonomy/🏷️filenames/🟦️.ts";

//#region 🔖️GenerateScript



export class GenerateScript extends Script {
  async run(segments: string[]): Promise<void> {
    if (segments[0] === "config") {
      await this.generateConfig(segments.slice(1));
      return;
    }
    if (segments[0] === "taxonomy") {
      this.generateTaxonomy(segments.slice(1));
      return;
    }
    if (segments[0] === "plugin-glue") {
      this.generatePluginGlue(segments.slice(1));
      return;
    }
    if (segments[0] === "scale-fixture") {
      runCmd("bun", ["repo", "run", "@semio-tech/framework-os-dev:generate-scale-fixture", ...segments.slice(1)], { cwd: this.root });
      return;
    }
    console.error("[generate] usage: bun ./📜️script.ts generate <config [--check]|taxonomy|plugin-glue|scale-fixture>");
    process.exit(1);
  }

  /** 📝️ Writes (or with `--check` only verifies) every checked-in file that derives from one canonical source: the devcontainer
   * editor block, the nextest scopes and the dependabot manifests.
   * @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📝️derived-config/🟦️.ts */
  private async generateConfig(args: string[]): Promise<void> {
    const changed = args.includes("--check") ? [] : await writeDerivedConfig(this.root);
    for (const path of changed) console.log(`[generate config] wrote ${path}`);
    const problems = checkDerivedConfig(this.root);
    for (const problem of problems) console.error(`[generate config] ${problem}`);
    if (problems.length) process.exit(1);
    console.log("[generate config] derived configuration is current");
  }

  /** 🧩️ Writes deterministic census or duplicate evidence with its Markdown companion. */
  private generateTaxonomy(args: string[]): void {
    const operation = args[0];
    if (operation === "package-adapter") {
      runNestedCargoPackageAdapter(this.root, args[1] ?? "preview");
      return;
    }
    if (operation !== "census" && operation !== "duplicates") throw new Error(`[generate taxonomy] expected census or duplicates, got ${JSON.stringify(operation)}.`);
    const ticketId = taxonomyOption(args, "--ticket");
    if (!ticketId) throw new Error(`[generate taxonomy ${operation}] --ticket <ticket-id> is required.`);
    const ticketDir = taxonomyTicketDirectory(this.root, ticketId);
    const census = buildSemanticCensus(this.root);
    if (operation === "census") {
      writeFileSync(join(ticketDir, "📊️semantic-census.json"), renderSemanticCensusJson(census));
      writeFileSync(join(ticketDir, "📓️semantic-census.md"), renderSemanticCensusMarkdown(census));
    } else {
      writeFileSync(join(ticketDir, "📊️semantic-duplicates.json"), renderSemanticDuplicatesJson(census));
      writeFileSync(join(ticketDir, "📓️semantic-duplicates.md"), renderSemanticDuplicatesMarkdown(census));
    }
    console.log(`[generate taxonomy ${operation}] ${census.records.length} components, ${census.problems.length} findings -> ${ticketDir}`);
  }

  /** 🧬️ Emit deterministic #[path] wiring comments for subset/example modules (dry by default). */
  private generatePluginGlue(args: string[]): void {
    const dry = args.includes("--dry") || args.includes("--dry-run") || args.length === 0;
    const pluginFilter = args.find((a) => !a.startsWith("--"));
    const pluginsRoot = join(this.root, "✏️s/🔌️plugins");
    const plugins = readdirSync(pluginsRoot, { withFileTypes: true }).filter((d) => d.isDirectory()).map((d) => d.name);
    let emitted = 0;
    for (const plugin of plugins) {
      if (pluginFilter && plugin !== pluginFilter && !plugin.includes(pluginFilter)) continue;
      const owned = policyListTopLevelSubsetDirs(this.root).filter((s) => s.split("/")[2] === plugin);
      const lines: string[] = [
        `// GENERATED-BY: bun ./📜️script.ts generate plugin-glue ${plugin}`,
        `// subsets=${owned.length} (registration body still hand-authored until serializer wave; this command validates discoverability)`,
      ];
      for (const sub of owned) {
        lines.push(`// subset: ${sub}`);
        emitted += 1;
      }
      const glue = join(pluginsRoot, plugin, "📦️packages/🦀️rust/🦀️.rs");
      if (!existsSync(glue)) {
        console.warn(`[generate plugin-glue] skip ${plugin}: missing glue.rs`);
        continue;
      }
      if (dry) {
        console.log(`[generate plugin-glue] dry ${plugin}: ${owned.length} subsets`);
      } else {
        const inv = join(pluginsRoot, plugin, "📦️packages/🦀️rust/🤖️generated-subset-inventory.txt");
        writeFileSync(inv, lines.join("\n") + "\n");
        console.log(`[generate plugin-glue] wrote ${inv}`);
      }
    }
    console.log(`[generate plugin-glue] ${dry ? "dry-run" : "wrote"} inventory for ${emitted} subset refs`);
  }
}
//#endregion 🔖️GenerateScript
