import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { addWorkspaceMembers, insertBuildOwnerRows, manifestRowKey, patchCargoLock, patchRouterPackages, readManifestDependencyRows, rebaseManifestRow, relativePosix, renderCrateManifest } from "../📝️render/🟦️.ts";

/** 📦️ Result of scaffolding the crate boundaries of one subset. */
export type SubsetCrateScaffold = { created: string[]; skipped: string[]; updated: string[]; notes: string[] };

/** 📦️ The crates every new subset starts with: a model crate (schema plus every io codec of its types), a viewer and an editor. */
export const SUBSET_CRATE_COMPONENTS: readonly { readonly slug: string; readonly directory: string; readonly mirror: boolean }[] = [
  { slug: "model", directory: "🧬️schema", mirror: true },
  { slug: "viewer", directory: "👁️viewer", mirror: false },
  { slug: "editor", directory: "✏️editor", mirror: false },
];

function findArtifactRoot(subsetRel: string, artifactsDirName: string): string | null {
  const segments = subsetRel.split("/");
  const index = segments.indexOf(artifactsDirName);
  return index >= 0 && segments.length > index + 1 ? segments.slice(0, index + 2).join("/") : null;
}

/** 📦️ Writes one crate manifest per subset component when the artifact owns a Cargo workspace, and registers the crates in it. */
export function scaffoldSubsetCrates(repoRoot: string, subsetRel: string, artifactsDirName: string, dryRun: boolean): SubsetCrateScaffold {
  const result: SubsetCrateScaffold = { created: [], skipped: [], updated: [], notes: [] };
  const artifactRel = findArtifactRoot(subsetRel, artifactsDirName);
  if (!artifactRel) { result.notes.push("subset is not below an artifact; crate boundaries not scaffolded"); return result; }
  const artifactDir = resolve(repoRoot, artifactRel);
  const workspaceManifest = join(artifactDir, "Cargo.toml");
  const packageDir = join(artifactDir, "📦️packages/🦀️rust");
  const packageManifest = join(packageDir, "Cargo.toml");
  if (!existsSync(workspaceManifest) || !existsSync(packageManifest)) { result.notes.push(`artifact ${artifactRel} has no Cargo workspace; crate boundaries not scaffolded`); return result; }
  const packageText = readFileSync(packageManifest, "utf8");
  const artifactCrate = /^name\s*=\s*"([^"]+)"/mu.exec(packageText)?.[1];
  if (!artifactCrate) { result.notes.push("artifact package has no name"); return result; }
  const rows = readManifestDependencyRows(packageText);
  const subsetName = subsetRel.split("/").pop()!;
  const standardName = subsetRel.split("/").slice(-3)[0] ?? "";
  const suffix = [standardName, subsetName].map((part) => part.replace(/^[^\p{L}\p{N}]+/u, "").replace(/[^A-Za-z0-9]+/gu, "-").replace(/^-|-$/gu, "").toLowerCase()).filter(Boolean).join("-");
  const crates = SUBSET_CRATE_COMPONENTS.map((component) => ({ ...component, name: `${artifactCrate}-${suffix}-${component.slug}`, packageDir: join(resolve(repoRoot, subsetRel), component.directory, "📦️packages/🦀️rust") }));
  const model = crates[0]!;
  const write = (absolute: string, content: string): void => {
    const rel = relativePosix(repoRoot, absolute);
    if (existsSync(absolute)) { result.skipped.push(rel); return; }
    result.created.push(rel);
    if (dryRun) return;
    mkdirSync(dirname(absolute), { recursive: true });
    writeFileSync(absolute, content, { flag: "wx" });
  };
  for (const crate of crates) {
    const siblingRows = crate === model ? [] : [`${model.name} = { path = "${relativePosix(crate.packageDir, model.packageDir)}" }`];
    const sibling = new Set(siblingRows.map(manifestRowKey));
    write(join(crate.packageDir, "Cargo.toml"), renderCrateManifest({
      name: crate.name,
      description: `${crate.slug} crate of ${artifactCrate} ${suffix}`,
      workspaceRel: relativePosix(crate.packageDir, artifactDir),
      libPath: crate.mirror ? "🦀️.rs" : "../../🦀️.rs",
      dependencyRows: [...rows.dependencies.filter((row) => !sibling.has(manifestRowKey(row))).map((row) => rebaseManifestRow(row, packageDir, crate.packageDir)), ...siblingRows],
      devDependencyRows: rows.devDependencies.map((row) => rebaseManifestRow(row, packageDir, crate.packageDir)),
      targetSections: rows.targetSections.map((section) => rebaseManifestRow(section, packageDir, crate.packageDir)),
    }));
    if (crate.mirror) write(join(crate.packageDir, "🦀️.rs"), `//! 🧬️ Model crate of ${subsetRel.split("/").slice(-4).join("/")}: the schema and every io codec of its types, because impls of framework traits must live with the type they implement.\n\n#[path = "../../🦀️.rs"]\npub mod schema;\n\n#[path = "../../../🚪️io/🦀️.rs"]\npub mod io;\n`);
  }
  if (dryRun) return result;
  const members = crates.map((crate) => relativePosix(artifactDir, crate.packageDir));
  const workspaceText = readFileSync(workspaceManifest, "utf8");
  const nextWorkspace = addWorkspaceMembers(workspaceText, members);
  if (nextWorkspace !== workspaceText) { writeFileSync(workspaceManifest, nextWorkspace); result.updated.push(relativePosix(repoRoot, workspaceManifest)); }
  const missingRows = crates.filter((crate) => !new RegExp(`^${crate.name.replace(/[.*+?^${}()|[\]\\]/gu, "\\$&")}\\s*=`, "mu").test(packageText)).map((crate) => `${crate.name} = { path = "${relativePosix(packageDir, crate.packageDir)}" }`);
  if (missingRows.length) {
    const at = packageText.search(/^\[dependencies\]\s*$/mu);
    if (at >= 0) {
      const lineEnd = packageText.indexOf("\n", at) + 1;
      writeFileSync(packageManifest, packageText.slice(0, lineEnd) + missingRows.join("\n") + "\n" + packageText.slice(lineEnd));
      result.updated.push(relativePosix(repoRoot, packageManifest));
    }
  }
  const routerPath = join(packageDir, "📜️script.ts");
  if (existsSync(routerPath)) {
    const router = readFileSync(routerPath, "utf8");
    const patched = patchRouterPackages(router, artifactCrate, crates.map((crate) => crate.name));
    if (patched !== router) { writeFileSync(routerPath, patched); result.updated.push(relativePosix(repoRoot, routerPath)); }
  }
  const ownersPath = resolve(repoRoot, "📋️build-owners.json");
  if (existsSync(ownersPath)) {
    const raw = readFileSync(ownersPath, "utf8");
    const { text, added } = insertBuildOwnerRows(raw, crates.map((crate) => ({ name: crate.name, root: relativePosix(repoRoot, crate.packageDir) })));
    if (added.length) { writeFileSync(ownersPath, text); result.updated.push("📋️build-owners.json"); }
  }
  const lockPath = join(artifactDir, "Cargo.lock");
  if (existsSync(lockPath)) {
    const lock = readFileSync(lockPath, "utf8");
    const patched = patchCargoLock(lock, artifactCrate, crates.map((crate) => ({ name: crate.name, dependsOn: crate === model ? [] : [model.name] })));
    if (patched !== lock) { writeFileSync(lockPath, patched); result.updated.push(relativePosix(repoRoot, lockPath)); }
  }
  return result;
}
