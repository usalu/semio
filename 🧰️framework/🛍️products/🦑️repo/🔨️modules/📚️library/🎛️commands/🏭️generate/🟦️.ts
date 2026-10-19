/** 🏭️ `generate` command family: derived config, taxonomy census and plugin-glue inventory.
 * @see 📜️script.ts
 */
import { Script } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { existsSync, readdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

//#region 🔖️GenerateScript



export class GenerateScript extends Script {
  async run(segments: string[]): Promise<void> {
    if (segments[0] === "config") {
      await this.generateConfig(segments.slice(1));
      return;
    }
    if (segments[0] === "taxonomy") {
      await this.generateTaxonomy(segments.slice(1));
      return;
    }
    if (segments[0] === "plugin-glue") {
      await this.generatePluginGlue(segments.slice(1));
      return;
    }
    if (segments[0] === "scale-fixture") {
      const { runCmd } = await import("../../🏃️process/🟦️.ts");
      runCmd("bun", ["repo", "run", "@semio-tech/framework-os-dev:generate-scale-fixture", ...segments.slice(1)], { cwd: this.root });
      return;
    }
    console.error("[generate] usage: bun ./📜️script.ts generate <config [--check]|taxonomy|plugin-glue|scale-fixture>");
    process.exit(1);
  }

  /** 📝️ Writes (or with `--check` only verifies) every checked-in file that derives from one canonical source: the nextest
   * scope configurations and dependency update manifests.
   * @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📝️derived-config/🟦️.ts */
  private async generateConfig(args: string[]): Promise<void> {
    const { checkDerivedConfig, writeDerivedConfig } = await import("../../⚡️caching/📦️artifacts/📝️derived-config/🟦️.ts");
    const changed = args.includes("--check") ? [] : await writeDerivedConfig(this.root);
    for (const path of changed) console.log(`[generate config] wrote ${path}`);
    const problems = checkDerivedConfig(this.root);
    for (const problem of problems) console.error(`[generate config] ${problem}`);
    if (problems.length) process.exit(1);
    console.log("[generate config] derived configuration is current");
  }

  /** 🧩️ Writes deterministic census or duplicate evidence with its Markdown companion. */
  private async generateTaxonomy(args: string[]): Promise<void> {
    const { runNestedCargoPackageAdapter } = await import("../../📽️projection/🧩️package-adapter/📦️publication/🟦️.ts");
    const { buildSemanticCensus, renderSemanticCensusJson, renderSemanticCensusMarkdown, renderSemanticDuplicatesJson, renderSemanticDuplicatesMarkdown } = await import("../../🔍️discovery/🧩️semantic-census/🟦️.ts");
    const { taxonomyTicketDirectory } = await import("../../🧹️normalization/🎮️command-contract/🟦️.ts");
    const { taxonomyOption } = await import("../../🧹️normalization/🔣️taxonomy/🏷️filenames/🟦️.ts");
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
  private async generatePluginGlue(args: string[]): Promise<void> {
    const { policyListTopLevelSubsetDirs } = await import("../../⚖️laws/🧩️plugin-consistency/🪆️subset-conformance/🟦️.ts");
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
