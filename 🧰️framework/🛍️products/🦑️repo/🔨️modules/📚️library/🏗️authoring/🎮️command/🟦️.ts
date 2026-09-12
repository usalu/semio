import { existsSync, readdirSync, statSync } from "node:fs";
import { join, relative } from "node:path";
import { loadCatalogTaxonomy as loadTaxonomy, subsetIdForDirectoryName } from "../../🔍️discovery/🟦️.ts";
import { Script } from "../../🏃️process/🧭️routing/🟦️.ts";
import { policyStripEmoji } from "../../🧹️normalization/🧬️mutation/🪪️identity/🟦️.ts";
import { newScaffoldArtifactTree, newScaffoldStandardTree, newScaffoldSubsetTree } from "../🗿️artifact-tree/🟦️.ts";
import { newScaffoldMutationTree } from "../🧬️mutation-tree/🟦️.ts";

export function newResolveChildDir(parentAbs: string, wantStripped: string): string | undefined {
  if (!existsSync(parentAbs)) return undefined;
  for (const name of readdirSync(parentAbs)) {
    if (!statSync(join(parentAbs, name)).isDirectory()) continue;
    if (policyStripEmoji(name) === wantStripped) return name;
  }
  return undefined;
}

/** 🚪️ Scaffolds `${ioRel}` per the corrected design.md §1 shape: root leaves, then one dir per
 * `ioSemanticCollectionDirNames` member DIRECTLY under `${ioRel}` (native codec, unsplit — `import`/
 * `export` express direction and exist only for FOREIGN dialects, which this generic scaffolder
 * cannot know in advance and therefore never creates) — `representationDirs` children for
 * 📸️snapshot/🔺️diff, a wildcard-slug-ready empty facet marker for 🧬️mutations/💡️inferences (their real
/**
 * 🚪️ `new artifact|standard|subset` CLI — registered as `bun ./📜️script.ts new <kind> …` via
 * `ScriptRouter`. Existing path segments (plugin/artifact-kind/standard) are resolved the same
 * emoji-tolerant way `new surface` resolves them; the final, NEW segment is taken literally (it must
 * already carry the right emoji prefix and, for standard/subset, the taxonomy's dir prefix).
 */
export class CleanMechanismNewScript extends Script {
  run(segments: string[]): void {
    const kind = segments[0];
    if (kind !== "subset" && kind !== "standard" && kind !== "artifact" && kind !== "mutation") {
      console.error("usage: bun ./📜️script.ts new artifact <plugin> <new-artifact-dir>");
      console.error("   or: bun ./📜️script.ts new standard <plugin> <artifact-kind> <new-standard-dir>");
      console.error("   or: bun ./📜️script.ts new subset <plugin> <artifact-kind> <standard> <new-subset-dir> [--dry-run]");
      console.error("   or: bun ./📜️script.ts new mutation <owner-mutation-root> <emoji-semantic-name> [--composite] [--text] [--binary] [--typescript] [--graphql] [--protobuf] [--json-schema] [--dry-run]");
      process.exit(1);
      return;
    }
    const taxonomy = loadTaxonomy();
    const rest = segments.slice(1);
    const flags = new Set(rest.filter((arg) => arg.startsWith("--")));
    const knownFlags = new Set(["--dry-run", "--composite", "--text", "--binary", "--typescript", "--graphql", "--protobuf", "--json-schema"]);
    for (const flag of flags) if (!knownFlags.has(flag)) throw new Error(`new ${kind}: unknown option ${flag}`);
    const dryRun = flags.has("--dry-run");
    const positional = rest.filter((a) => !a.startsWith("--"));
    const repoRoot = this.root;
    const pluginsRoot = join(repoRoot, "✏️s/🔌️plugins");

    const resolveExisting = (parentAbs: string, arg: string, label: string): string => {
      const dir = newResolveChildDir(parentAbs, policyStripEmoji(arg));
      if (!dir) throw new Error(`new ${kind}: no ${label} "${arg}" under ${relative(repoRoot, parentAbs)}`);
      return dir;
    };

    try {
      if (kind === "mutation") {
        if (positional.length !== 2) throw new Error("usage: bun ./📜️script.ts new mutation <owner-mutation-root> <emoji-semantic-name> [options]");
        const [owner, name] = positional as [string, string];
        const result = newScaffoldMutationTree(
          repoRoot,
          owner,
          name,
          {
            composite: flags.has("--composite"),
            text: flags.has("--text"),
            binary: flags.has("--binary"),
            typescript: flags.has("--typescript"),
            graphql: flags.has("--graphql"),
            protobuf: flags.has("--protobuf"),
            jsonSchema: flags.has("--json-schema"),
          },
          dryRun,
        );
        this.report(`${owner}/${name}`, result.created, result.skipped, dryRun);
        for (const edit of result.updated) console.log(`  ${dryRun ? "~ (dry-run)" : "~"} ${edit}`);
        return;
      }
      if (kind === "artifact") {
        if (positional.length !== 2) throw new Error("usage: bun ./📜️script.ts new artifact <plugin> <new-artifact-dir>");
        const [pluginArg, newDir] = positional as [string, string];
        const pluginDir = resolveExisting(pluginsRoot, pluginArg, "plugin");
        const artifactsAbs = join(pluginsRoot, pluginDir, taxonomy.artifactsDirName);
        const artRel = relative(repoRoot, join(artifactsAbs, newDir)).replaceAll("\\", "/");
        const { created, skipped } = newScaffoldArtifactTree(repoRoot, artRel, dryRun);
        this.report(artRel, created, skipped, dryRun);
        return;
      }
      if (kind === "standard") {
        if (positional.length !== 3) throw new Error("usage: bun ./📜️script.ts new standard <plugin> <artifact-kind> <new-standard-dir>");
        const [pluginArg, artArg, newDir] = positional as [string, string, string];
        const pluginDir = resolveExisting(pluginsRoot, pluginArg, "plugin");
        const artifactsAbs = join(pluginsRoot, pluginDir, taxonomy.artifactsDirName);
        const artDir = resolveExisting(artifactsAbs, artArg, "artifact kind");
        const standardsAbs = join(artifactsAbs, artDir, taxonomy.standardsDirName);
        if (taxonomy.standardDirPrefix && !newDir.startsWith(taxonomy.standardDirPrefix)) throw new Error(`new standard: "${newDir}" must start with standardDirPrefix "${taxonomy.standardDirPrefix}"`);
        const standardRel = relative(repoRoot, join(standardsAbs, newDir)).replaceAll("\\", "/");
        const { created, skipped } = newScaffoldStandardTree(repoRoot, standardRel, dryRun);
        this.report(standardRel, created, skipped, dryRun);
        return;
      }
      if (positional.length !== 4) throw new Error("usage: bun ./📜️script.ts new subset <plugin> <artifact-kind> <standard> <new-subset-dir>");
      const [pluginArg, artArg, stdArg, newDir] = positional as [string, string, string, string];
      const pluginDir = resolveExisting(pluginsRoot, pluginArg, "plugin");
      const artifactsAbs = join(pluginsRoot, pluginDir, taxonomy.artifactsDirName);
      const artDir = resolveExisting(artifactsAbs, artArg, "artifact kind");
      const standardsAbs = join(artifactsAbs, artDir, taxonomy.standardsDirName);
      const standardDir = resolveExisting(standardsAbs, stdArg, "standard");
      const subsetsAbs = join(standardsAbs, standardDir, taxonomy.subsetsDirName);
      const subsetsRel = relative(repoRoot, subsetsAbs).replaceAll("\\", "/");
      if (subsetIdForDirectoryName(subsetsRel, newDir, taxonomy) === null) throw new Error(`new subset: "${newDir}" is not a registered semantic directory for "${subsetsRel}"`);
      const subsetRel = relative(repoRoot, join(subsetsAbs, newDir)).replaceAll("\\", "/");
      const { created, skipped } = newScaffoldSubsetTree(repoRoot, subsetRel, taxonomy, dryRun);
      this.report(subsetRel, created, skipped, dryRun);
    } catch (error) {
      console.error((error as Error).message);
      process.exit(1);
    }
  }

  private report(rel: string, created: string[], skipped: string[], dryRun: boolean): void {
    const verb = dryRun ? "would create" : "created";
    console.log(`new: ${rel} — ${verb} ${created.length} file(s), ${skipped.length} already present.`);
    for (const p of created) console.log(`  ${dryRun ? "+ (dry-run)" : "+"} ${p}`);
  }
}
