import { execFileSync } from "node:child_process";
import { existsSync, readFileSync, realpathSync, statSync, writeFileSync } from "node:fs";
import { dirname, isAbsolute, join, relative, resolve, sep } from "node:path";
import { loadCatalogTaxonomy as loadTaxonomy } from "../../🔍️discovery/🔐️taxonomy-authority/🟦️.ts";
import { subsetIdForDirectoryName } from "../../🔍️discovery/🗂️file-kind-resolution/🟦️.ts";
import { Script } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { newScaffoldArtifactTree, newScaffoldStandardTree, newScaffoldSubsetTree } from "../🗿️artifact-tree/🟦️.ts";
import { assessMutationCrateBudget, newScaffoldMutationTree } from "../🧬️mutation-tree/🟦️.ts";
import { parseCrateBudget, type CrateBudget } from "../📦️crate-tree/🗺️plan/💰️budget/🟦️.ts";

/** 📐️ Reads the crate budgets of `new crates`: `--target=<files>/<bytes>/<lines>` is what every crate aims for, `--hard=<files>/<bytes>/<lines>` is the limit the planner refuses to pack beyond. */
export function crateBudgetFlags(flags: ReadonlySet<string>): { target: CrateBudget | undefined; hard: CrateBudget | undefined } {
  const value = (name: string): string | undefined => [...flags].find((flag) => flag.startsWith(`${name}=`))?.slice(name.length + 1);
  const target = value("--target");
  const hard = value("--hard");
  return { target: target === undefined ? undefined : parseCrateBudget(target), hard: hard === undefined ? undefined : parseCrateBudget(hard) };
}

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

/** 🏗️ Scaffolds an exact caller-owned artifact, standard, subset, or mutation directory, or lays out the crates of an existing artifact. */
export class CleanMechanismNewScript extends Script {
  async run(segments: string[]): Promise<void> {
    const kind = segments[0];
    if (kind !== "subset" && kind !== "standard" && kind !== "artifact" && kind !== "mutation" && kind !== "crates" && kind !== "leaves") {
      console.error("usage: bun ./📜️script.ts new artifact <owner-root> <new-artifact-dir>");
      console.error("   or: bun ./📜️script.ts new standard <artifact-root> <new-standard-dir>");
      console.error("   or: bun ./📜️script.ts new subset <standard-root> <new-subset-dir> [--dry-run]");
      console.error("   or: bun ./📜️script.ts new mutation <owner-mutation-root> <emoji-semantic-name> [--composite] [--text] [--binary] [--typescript] [--graphql] [--protobuf] [--json-schema] [--family-crate] [--dry-run]");
      console.error("   or: bun ./📜️script.ts new crates <artifact-package-dir> [--dry-run] [--explain] [--no-test-rewrites] [--no-test-aggregate] [--inline-root-modules] [--mutation-families=off|on|auto] [--lift=<module-path>,...] [--target=<files>/<bytes>/<lines>] [--hard=<files>/<bytes>/<lines>] [--backup=<dir>] [--verify [--oracle] [--keep] --sandbox=<dir>] | --check-applied [--base=<git-ref>] [--samples=<count>] [--keep] --sandbox=<dir>");
      console.error("   or: bun ./📜️script.ts new leaves <rust-file> [--dry-run] [--json-schema]   (serve typescript/graphql/proto facet companions, and with --json-schema the JSON Schema leaves, from their documents)");
      process.exit(1);
      return;
    }
    const taxonomy = loadTaxonomy();
    const rest = segments.slice(1);
    const flags = new Set(rest.filter((arg) => arg.startsWith("--")));
    const knownFlags = new Set(["--verify", "--check-applied", "--oracle", "--dry-run", "--composite", "--text", "--binary", "--typescript", "--graphql", "--protobuf", "--json-schema", "--rust", "--no-test-rewrites", "--no-test-aggregate", "--inline-root-modules", "--family-crate", "--keep", "--explain", "--unmirror"]);
    for (const flag of flags) if (!knownFlags.has(flag) && !flag.startsWith("--backup=") && !flag.startsWith("--sandbox=") && !flag.startsWith("--base=") && !flag.startsWith("--samples=") && !flag.startsWith("--mutation-families=") && !flag.startsWith("--lift=") && !flag.startsWith("--target=") && !flag.startsWith("--hard=")) throw new Error(`new ${kind}: unknown option ${flag}`);
    const dryRun = flags.has("--dry-run");
    const positional = rest.filter((a) => !a.startsWith("--"));
    const repoRoot = this.root;
    try {
      if (kind === "mutation") {
        if (positional.length !== 2) throw new Error("usage: bun ./📜️script.ts new mutation <owner-mutation-root> <emoji-semantic-name> [options]");
        const [owner, name] = positional as [string, string];
        const budget = await assessMutationCrateBudget(repoRoot, owner);
        if (budget?.verdict === "refused") throw new Error(`new mutation: ${budget.crate} would leave the source gate (lib ${budget.lib.files} files / ${budget.lib.bytes} bytes / ${budget.lib.lines} lines, test ${budget.test.files} / ${budget.test.bytes} / ${budget.test.lines}); put the leaf in a family crate or run \`bun ./📜️script.ts new crates <artifact-package-dir>\` first`);
        if (budget?.verdict === "near-limit" || budget?.verdict === "already-over") console.warn(`new mutation: ${budget.crate} is ${budget.verdict === "already-over" ? "already over" : "next to"} the source gate (lib ${budget.lib.files} files / ${budget.lib.lines} lines, test ${budget.test.files} files); plan a carve with \`bun ./📜️script.ts new crates\``);
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
            familyCrate: flags.has("--family-crate"),
          },
          dryRun,
        );
        this.report(`${owner}/${name}`, result.created, result.skipped, dryRun);
        for (const edit of result.updated) console.log(`  ${dryRun ? "~ (dry-run)" : "~"} ${edit}`);
        return;
      }
      if (kind === "leaves") {
        if (positional.length !== 1) throw new Error("usage: bun ./📜️script.ts new leaves <rust-file> [--dry-run] [--json-schema] [--rust]");
        const { externalizeFacetLeaves } = await import("../📦️crate-tree/🧾️facet-leaves/🟦️.ts");
        const absolute = join(repoRoot, positional[0]!);
        const result = externalizeFacetLeaves(readFileSync(absolute, "utf8"), absolute, repoRoot, { json: flags.has("--json-schema"), rust: flags.has("--rust") });
        if (!result) { console.log(`new leaves: ${positional[0]} has no ArtifactSchemaDescriptor companion to serve.`); return; }
        if (!dryRun) writeFileSync(absolute, result.text);
        console.log(`new leaves: ${positional[0]} — ${dryRun ? "would serve" : "serving"} ${result.leaves.length} companion(s) from their documents; install a SchemaLeafReader (FileSystemSchemaLeaves) in every host that resolves them.`);
        for (const leaf of result.leaves) console.log(`  ${leaf.scope}/${leaf.export}/${leaf.format} <- ${leaf.path}`);
        return;
      }
      if (kind === "crates") {
        if (positional.length !== 1) throw new Error("usage: bun ./📜️script.ts new crates <artifact-package-dir> [--dry-run] [--no-test-rewrites] [--backup=<dir>]");
        await this.crates(positional[0]!, flags, dryRun);
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

  private async crates(packageDir: string, flags: ReadonlySet<string>, dryRun: boolean): Promise<void> {
    const { applyCarveWrites, carveWrites, planArtifactCrates, registerBuildOwnerRows, resolveCarveBackupDir } = await import("../📦️crate-tree/🟦️.ts");
    if (flags.has("--check-applied")) {
      const sandbox = [...flags].find((flag) => flag.startsWith("--sandbox="))?.slice("--sandbox=".length);
      if (!sandbox) throw new Error("new crates --check-applied needs --sandbox=<dir> (a directory outside the repository)");
      const { verifyAppliedLayout } = await import("../📦️crate-tree/🧫️sandbox/🟦️.ts");
      const { workspaceAdmissionProblems } = await import("../📦️crate-tree/📝️render/🟦️.ts");
      const { sharedMountFiles } = await import("../📦️crate-tree/🔬️verify/🟦️.ts");
      const base = [...flags].find((flag) => flag.startsWith("--base="))?.slice("--base=".length) ?? "HEAD";
      const artifactDir = dirname(dirname(packageDir));
      const samples = Number([...flags].find((flag) => flag.startsWith("--samples="))?.slice("--samples=".length) ?? 12);
      if (!Number.isInteger(samples) || samples < 1) throw new Error("new crates --samples expects a positive integer");
      const report = await verifyAppliedLayout(this.root, artifactDir, resolve(sandbox), base, flags.has("--keep"), samples);
      const packageManifest = Bun.TOML.parse(readFileSync(join(this.root, packageDir, "Cargo.toml"), "utf8")) as { package?: { workspace?: string } };
      const admission = workspaceAdmissionProblems(readFileSync(join(this.root, packageDir, packageManifest.package?.workspace ?? "../..", "Cargo.toml"), "utf8"));
      const pluginDir = artifactDir.split("/").slice(0, 3).join("/");
      const manifests = execFileSync("git", ["-c", "core.quotepath=false", "-c", "core.fsmonitor=false", "ls-files", "-co", "--exclude-standard", "-z", "--", pluginDir], { cwd: this.root, maxBuffer: 1 << 28 }).toString("utf8").split("\0").filter((path) => path.endsWith("/Cargo.toml"));
      const reached = sharedMountFiles(this.root, manifests, (manifest) => manifest.startsWith(`${artifactDir}/`));
      const shared = reached.filter((entry) => entry.file.startsWith(`${pluginDir}/`));
      const sharedForeign = reached.filter((entry) => !entry.file.startsWith(`${pluginDir}/`)).map((entry) => entry.file);
      console.log(JSON.stringify({ base, ...report, admission, shared, sharedForeign }, null, 2));
      if (admission.length || shared.length || Object.values(report.layout).some((row) => row.onlyCombined > 0)) { process.exit(1); return; }
      return;
    }
    if (flags.has("--unmirror")) { await this.unmirror(packageDir, !dryRun); return; }
    const mode = [...flags].find((flag) => flag.startsWith("--mutation-families="))?.slice("--mutation-families=".length) ?? "auto";
    if (mode !== "off" && mode !== "on" && mode !== "auto") throw new Error(`--mutation-families expects off, on or auto, got ${JSON.stringify(mode)}`);
    const lift = [...flags].find((flag) => flag.startsWith("--lift="))?.slice("--lift=".length).split(",").filter(Boolean);
    const plan = await planArtifactCrates(this.root, packageDir, { mutationFamilies: mode, lift, ...crateBudgetFlags(flags), inlineRootModules: flags.has("--inline-root-modules"), tests: flags.has("--no-test-rewrites") ? undefined : { fixtures: true, aggregate: !flags.has("--no-test-aggregate"), hoist: true, detach: true } });
    for (const crate of plan.crates) console.log(`crate ${crate.name}  ${crate.packageDir}  lib ${crate.lib?.files}/${crate.lib?.bytes}/${crate.lib?.lines}  test ${crate.test?.files}/${crate.test?.bytes}/${crate.test?.lines}  deps [${crate.dependsOn.map((id) => plan.crates.find((entry) => entry.id === id)?.name).join(", ")}]`);
    for (const note of plan.overlay.notes) console.log(`note: ${note}`);
    if (plan.problems.length) {
      for (const problem of plan.problems) console.error(`problem: ${problem}`);
      if (flags.has("--explain")) for (const blocker of plan.blockers) {
        console.error(`blocker: ${blocker.slices} slices, ${blocker.files} files, ${blocker.bytes} bytes, ${blocker.lines} lines; coherence [${blocker.coherence.join("; ")}]; cycles [${blocker.cycles.join(", ")}]`);
        for (const row of blocker.groupReport) console.error(`  ${row}`);
      }
      process.exit(1);
      return;
    }
    if (flags.has("--verify")) {
      const sandbox = [...flags].find((flag) => flag.startsWith("--sandbox="))?.slice("--sandbox=".length);
      if (!sandbox) throw new Error("new crates --verify needs --sandbox=<dir> (a directory outside the repository)");
      const { verifyCarveInSandbox } = await import("../📦️crate-tree/🧫️sandbox/🟦️.ts");
      const report = await verifyCarveInSandbox(plan, resolve(sandbox), { oracle: flags.has("--oracle"), keep: flags.has("--keep") });
      console.log(JSON.stringify(report, null, 2));
      const failed = report.census.some((row) => !row.withinHardBudget) || report.moduleTreeProblems.length > 0 || report.visibility.length > 0 || report.orphan.length > 0 || report.glue.length > 0 || Object.values(report.unresolvedCrates).some((list) => list.length > 0) || Object.values(report.oracle ?? {}).some((row) => row.onlyCandidate > 0) || Object.values(report.layout ?? {}).some((row) => row.onlyCombined > 0);
      if (failed) { process.exit(1); return; }
      return;
    }
    const { writes } = await carveWrites(plan);
    for (const write of writes) console.log(`  ${dryRun ? "(dry-run) " : ""}${write.action === "create" ? "+" : write.action === "delete" ? "-" : "~"} ${write.path}`);
    if (dryRun) return;
    const backupFlag = [...flags].find((flag) => flag.startsWith("--backup="))?.slice("--backup=".length);
    const backup = backupFlag ? resolveCarveBackupDir(this.root, backupFlag) : undefined;
    applyCarveWrites(this.root, writes, backup);
    const owners = registerBuildOwnerRows(this.root, plan, backup);
    console.log(`owner rows: ${owners.length ? owners.join(", ") : "none added"}`);
    console.log(`new crates: ${plan.artifactCrate} — wrote ${writes.length} file(s) for ${plan.crates.length} crate(s).`);
    await this.unmirror(packageDir, true);
  }

  private async unmirror(packageDir: string, apply: boolean): Promise<void> {
    const { runUnmirror } = await import("../📦️crate-tree/🪞️unmirror/🏃️run/🟦️.ts");
    const result = runUnmirror({ repoRoot: this.root, artifacts: [dirname(dirname(packageDir))], apply, log: (line) => console.log(`  ${line}`) });
    const report = result.report;
    console.log(`new crates --unmirror: ${report.edits.size} file(s), ${result.manifests.size} manifest(s), ${report.removedLeaves} mirror re-export(s) removed, ${report.repointedLeaves + report.repointedChains} path(s) re-pointed, ${report.addedImports} import(s) added, ${report.removedModules} module(s) removed${apply ? `, ${result.written} written` : " (dry-run)"}.`);
    for (const line of [...report.unresolvedMirrors.map((entry) => `unresolved mirror re-export ${entry}`), ...report.multiMounted.map((entry) => `file mounted more than once, not edited ${entry}`), ...result.cycles.map((entry) => `dependency cycle ${entry}`), ...result.regressions.map((entry) => `audit regression ${entry}`), ...result.conflicts.map((entry) => `file changed while planning ${entry}`)]) console.error(`problem: ${line}`);
    if (result.cycles.length || result.regressions.length || result.conflicts.length || report.unresolvedMirrors.length) process.exit(1);
  }

  private report(rel: string, created: string[], skipped: string[], dryRun: boolean): void {
    const verb = dryRun ? "would create" : "created";
    console.log(`new: ${rel} — ${verb} ${created.length} file(s), ${skipped.length} already present.`);
    for (const p of created) console.log(`  ${dryRun ? "+ (dry-run)" : "+"} ${p}`);
  }
}
