import { existsSync, realpathSync, statSync } from "node:fs";
import { dirname, isAbsolute, join, relative, sep } from "node:path";
import { loadCatalogTaxonomy as loadTaxonomy, subsetIdForDirectoryName } from "../../🔍️discovery/🟦️.ts";
import { Script } from "../../🏃️process/🧭️routing/🟦️.ts";
import { newScaffoldArtifactTree, newScaffoldStandardTree, newScaffoldSubsetTree } from "../🗿️artifact-tree/🟦️.ts";
import { newScaffoldMutationTree } from "../🧬️mutation-tree/🟦️.ts";

/** 🛂️ Resolves a scaffold destination from the caller's exact physical owner and canonical leaf. */
export function newScaffoldDestinationV1(repoRoot: string, kind: "artifact" | "standard" | "subset", owner: string, newDirectory: string): string {
  if (!owner || /^[A-Za-z]:/u.test(owner) || owner.startsWith("/") || /[\\\u0000-\u001f]/u.test(owner) || owner.split("/").some((part) => !part || part === "." || part === "..")) throw Error("Scaffold owner must be an exact workspace-relative path");
  if (!newDirectory || /[/:\\\u0000-\u001f]/u.test(newDirectory) || newDirectory === "." || newDirectory === "..") throw Error("Scaffold directory must be one canonical leaf");
  const root = realpathSync(repoRoot), ownerPath = join(root, owner);
  if (!existsSync(ownerPath) || !statSync(ownerPath).isDirectory()) throw Error(`Scaffold owner is missing: ${owner}`);
  const physical = relative(root, realpathSync(ownerPath));
  if (!physical || isAbsolute(physical) || physical === ".." || physical.startsWith(`..${sep}`)) throw Error("Scaffold owner must be physically inside the workspace");
  const taxonomy = loadTaxonomy();
  const collection = kind === "artifact" ? taxonomy.artifactsDirName : kind === "standard" ? taxonomy.standardsDirName : taxonomy.subsetsDirName;
  if (kind === "standard" && taxonomy.standardDirPrefix && !newDirectory.startsWith(taxonomy.standardDirPrefix)) throw Error(`Standard directory must start with ${taxonomy.standardDirPrefix}`);
  const collectionRelative = `${owner}/${collection}`;
  if (kind === "subset" && subsetIdForDirectoryName(collectionRelative, newDirectory, taxonomy) === null) throw Error(`Subset directory is not registered for ${collectionRelative}`);
  const destination = `${collectionRelative}/${newDirectory}`;
  let existing = join(root, destination);
  while (!existsSync(existing)) existing = dirname(existing);
  const physicalParent = relative(root, realpathSync(existing));
  if (isAbsolute(physicalParent) || physicalParent === ".." || physicalParent.startsWith(`..${sep}`)) throw Error("Scaffold destination must be physically inside the workspace");
  return destination;
}

/** 🏗️ Scaffolds an exact caller-owned artifact, standard, subset, or mutation directory. */
export class CleanMechanismNewScript extends Script {
  run(segments: string[]): void {
    const kind = segments[0];
    if (kind !== "subset" && kind !== "standard" && kind !== "artifact" && kind !== "mutation") {
      console.error("usage: bun ./📜️script.ts new artifact <owner-root> <new-artifact-dir>");
      console.error("   or: bun ./📜️script.ts new standard <artifact-root> <new-standard-dir>");
      console.error("   or: bun ./📜️script.ts new subset <standard-root> <new-subset-dir> [--dry-run]");
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
      if (positional.length !== 2) throw new Error(`new ${kind}: expected an exact owner path and a new directory`);
      const [owner, newDirectory] = positional as [string, string];
      const destination = newScaffoldDestinationV1(repoRoot, kind, owner, newDirectory);
      const result = kind === "artifact" ? newScaffoldArtifactTree(repoRoot, destination, dryRun) : kind === "standard" ? newScaffoldStandardTree(repoRoot, destination, dryRun) : newScaffoldSubsetTree(repoRoot, destination, taxonomy, dryRun);
      this.report(destination, result.created, result.skipped, dryRun);
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
