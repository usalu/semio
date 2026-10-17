import { existsSync, mkdirSync, readFileSync, realpathSync, rmSync, writeFileSync } from "node:fs";
import { dirname, isAbsolute, join, relative, resolve, sep } from "node:path";
import { crateIdent, identifierSet, measureRustCrate, planArtifactCrates, readMeasureSources, useLeafNames, weaveRootEdits, type ArtifactCratePlan, type CrateMeasure, type PlannedCrate } from "./🗺️plan/🟦️.ts";
import { addWorkspaceMembers, insertBuildOwnerRows, manifestRowKey, rebaseManifestDirPaths, renderMirrorFiles, patchCargoLock, patchRouterPackages, readManifestDependencyRows, rebaseManifestRow, relativePosix, renderCrateManifest, renderMirrorRoot, type MirrorSpec, type MirrorUnit } from "./📝️render/🟦️.ts";
import { demoteDanglingOuterDocs, parseRustSourceTree, rustOuterDocStart } from "./🌳️module-tree/🟦️.ts";
import { FIXTURE_READER_CRATE, FIXTURE_READER_PACKAGE_DIR, resolveFixturePlaceholders } from "./🧪️test-closure/🟦️.ts";

export { planArtifactCrates } from "./🗺️plan/🟦️.ts";
export type { ArtifactCratePlan, PlannedCrate } from "./🗺️plan/🟦️.ts";
export { CRATE_HARD_BUDGET, CRATE_TARGET_BUDGET, measureFits, measureRustCrate, parseCrateBudget } from "./🗺️plan/🟦️.ts";

/** 📝️ One file the carve writes, rewrites or deletes. */
export type CarveFileWrite = { readonly path: string; readonly action: "create" | "update" | "delete"; readonly content: string };

/** 🪞️ The mirror spec of one planned crate. */
export function mirrorSpecOf(plan: ArtifactCratePlan, crate: PlannedCrate): MirrorSpec {
  return {
    doc: crate.doc,
    newRootDir: resolve(plan.repoRoot, crate.packageDir),
    innerAttributes: crate.innerAttributes,
    externLines: crate.externLines,
    movedItemsText: crate.movedItemsText,
    itemImports: crate.itemImportLines,
    owned: crate.attach.map((decl): MirrorUnit => ({ path: decl.eff, node: decl.node, sourceText: decl.declText, oldScope: decl.scope, component: decl.isComponent, fileDir: dirname(decl.declFile) })),
    imported: crate.imports,
    wrapperNodes: plan.wrapperNodes,
    gates: plan.gates,
    hoist: crate.hoisted === true,
  };
}

/** 🪞️ The complete mirror root text of one planned crate. */
export function carveCrateRootText(plan: ArtifactCratePlan, crate: PlannedCrate): string {
  return renderMirrorRoot(mirrorSpecOf(plan, crate));
}

function applyEdits(text: string, edits: readonly { start: number; end: number; text: string }[]): string {
  let out = text;
  for (const edit of [...edits].sort((a, b) => b.start - a.start)) out = out.slice(0, edit.text === "" && edit.end > edit.start ? rustOuterDocStart(out, edit.start) : edit.start) + edit.text + out.slice(edit.end);
  return edits.some((edit) => edit.text === "" && edit.end > edit.start) ? demoteDanglingOuterDocs(out) : out;
}

/** 🧾️ Rewrites the artifact root so carved modules and moved items become explicit re-exports. */
export async function carveArtifactRootText(plan: ArtifactCratePlan): Promise<string> {
  const edits = [...weaveRootEdits(plan.rootEdits, plan.wrapperNodes, plan.weave, plan.placement, plan.fileCuts, plan.gates).edits];
  const testGated = new Set(plan.tree.items.filter((item) => item.name && item.attributeTexts.some((attribute) => /cfg\(\s*test\s*\)/u.test(attribute))).map((item) => item.name!));
  for (const unit of plan.movedUnits) {
    const host = plan.crates.find((crate) => crate.id === unit.to);
    const reexport = (path: string): string => [unit.names.filter((name) => !testGated.has(name)), unit.names.filter((name) => testGated.has(name))].map((names, gated) => (names.length ? `${gated ? "#[cfg(test)]\n" : ""}pub use ${path}::{${names.join(", ")}};` : "")).filter(Boolean).join("\n");
    [...unit.blocks].sort((a, b) => a.start - b.start).forEach((block, index) => edits.push({ start: block.start, end: block.end, text: index === 0 && host ? reexport(crateIdent(host.name)) : "" }));
  }
  return pruneRootUses(plan, applyEdits(plan.tree.source, edits));
}

async function pruneRootUses(plan: ArtifactCratePlan, rewritten: string): Promise<string> {
  let text = rewritten;
  const tree = parseRustSourceTree(text);
  const virtualRoot = resolve(plan.repoRoot, plan.rootFile);
  const overlay = new Map<string, string>([...plan.overlay.edits, ...plan.overlay.creates, [virtualRoot, text]]);
  const used = new Set<string>();
  const outside = new Set<string>();
  for (const withTest of [false, true]) {
    const measure = await measureRustCrate(plan.repoRoot, virtualRoot, withTest, overlay, resolve(plan.repoRoot, plan.packageDir));
    for (const [file, body] of readMeasureSources(plan.repoRoot, measure, overlay)) for (const identifier of identifierSet(body)) { used.add(identifier); if (resolve(plan.repoRoot, file) !== virtualRoot) outside.add(identifier); }
  }
  const without = (start: number, end: number): Set<string> => identifierSet(text.slice(0, start) + text.slice(end));
  const edits: { start: number; end: number; text: string }[] = [];
  for (const item of tree.items) {
    if (item.kind === "use" && !item.visibility.startsWith("pub")) {
      const statement = text.slice(item.declStart, item.end);
      const rest = without(item.start, item.end);
      const leaves = useLeafNames(statement);
      const kept = leaves.filter((leaf) => rest.has(leaf) || outside.has(leaf));
      if (kept.length === leaves.length) continue;
      edits.push({ start: item.start, end: item.end, text: kept.length ? prunedUseText(text.slice(item.start, item.end), kept) : "" });
    } else if (item.kind === "extern-crate") {
      const match = /extern\s+crate\s+([A-Za-z0-9_]+)(?:\s+as\s+([A-Za-z0-9_]+))?\s*;/u.exec(text.slice(item.declStart, item.end));
      if (match && !used.has(match[2] ?? match[1]!)) edits.push({ start: item.start, end: item.end, text: "" });
    }
  }
  text = applyEdits(text, edits);
  return text.replace(/\n{3,}/gu, "\n\n");
}

function prunedUseText(statement: string, leaves: readonly string[]): string {
  const open = statement.indexOf("{");
  if (open < 0) return statement;
  const close = statement.lastIndexOf("}");
  const members = statement.slice(open + 1, close).split(",").map((member) => member.trim()).filter(Boolean).filter((member) => leaves.includes(member.split(/\s+as\s+/u).pop()!.trim().split("::").pop()!));
  return `${statement.slice(0, open)}{${members.join(", ")}}${statement.slice(close + 1)}`;
}

/** 🚩️ A feature of a package manifest with the entries it enables, which every carved crate declares again and forwards to the carved crates it depends on, so a consumer that enables the feature on the artifact reaches every crate. */
export type ManifestFeature = Readonly<{ name: string; entries: readonly string[] }>;

/** 🚩️ The features of a package manifest in declaration order; a feature that enables optional dependencies (`testkit = ["dep:serde_json"]`) keeps those entries in every carved crate, which carries the same dependency rows. */
export function forwardableFeatures(manifestText: string): ManifestFeature[] {
  const table = (Bun.TOML.parse(manifestText) as { features?: Record<string, unknown> }).features ?? {};
  return Object.entries(table).filter(([, entries]) => Array.isArray(entries)).map(([name, entries]) => ({ name, entries: (entries as unknown[]).map(String) }));
}

/** 🚩️ The rendered entries of a carved feature: the original entries, then the same feature of every carved crate it forwards to. */
export function featureRow(feature: ManifestFeature, forwardTo: readonly string[]): string {
  return `${feature.name} = [${[...feature.entries, ...forwardTo.map((crate) => `${crate}/${feature.name}`)].map((entry) => JSON.stringify(entry)).join(", ")}]`;
}

/** 🚩️ Extends the features of the artifact package manifest with the carved crates' feature of the same name, keeping the original entries. */
export function forwardFeatures(manifestText: string, features: readonly ManifestFeature[], crateNames: readonly string[]): string {
  let out = manifestText;
  for (const feature of features) out = out.replace(new RegExp(`^${feature.name.replace(/[.*+?^${}()|[\]\\]/gu, "\\$&")}\\s*=\\s*\\[[^\\]]*\\]`, "mu"), () => featureRow(feature, crateNames));
  return out;
}

/** 🧪️ Adds dev-dependency rows to the artifact package manifest (creating its `[dev-dependencies]` section when absent), so the facade tests see the features its carved crates gate their test helpers behind. */
export function withDevDependencyRows(manifestText: string, rows: readonly string[]): string {
  if (!rows.length) return manifestText;
  const at = manifestText.search(/^\[dev-dependencies\]\s*$/mu);
  if (at < 0) return `${manifestText.trimEnd()}\n\n[dev-dependencies]\n${rows.join("\n")}\n`;
  const lineEnd = manifestText.indexOf("\n", at) + 1;
  return manifestText.slice(0, lineEnd) + rows.join("\n") + "\n" + manifestText.slice(lineEnd);
}

/** 🧲️ Adds a dev-dependency row to a manifest at its sorted place inside `[dev-dependencies]`, creating the section when absent and leaving a manifest that already names the dependency unchanged. */
export function insertDevDependencyRow(manifestText: string, row: string): string {
  const key = manifestRowKey(row);
  const lines = manifestText.split("\n");
  const header = lines.findIndex((line) => /^\[dev-dependencies\]\s*$/u.test(line));
  if (header < 0) return `${manifestText.trimEnd()}\n\n[dev-dependencies]\n${row}\n`;
  let end = header + 1;
  while (end < lines.length && !/^\[/u.test(lines[end]!)) end += 1;
  if (lines.slice(header + 1, end).some((line) => manifestRowKey(line) === key)) return manifestText;
  let last = header;
  for (let at = header + 1; at < end; at += 1) {
    const line = lines[at]!;
    if (line.trim() === "" || line.trim().startsWith("#")) continue;
    const existing = manifestRowKey(line);
    if (existing > key) { lines.splice(at, 0, row); return lines.join("\n"); }
    last = at;
  }
  lines.splice(last + 1, 0, row);
  return lines.join("\n");
}

/** 🧲️ Gives a manifest the shared fixture reader as dev-dependency, with the path computed from the manifest's own directory. */
export function withFixtureReaderDevDependency(manifestText: string, manifestDirectory: string, repoRoot: string): string {
  return insertDevDependencyRow(manifestText, `${FIXTURE_READER_CRATE} = { path = "${relativePosix(manifestDirectory, resolve(repoRoot, FIXTURE_READER_PACKAGE_DIR))}" }`);
}

/** 🧱️ Computes every file write of a carve without touching the tree. */
export async function carveWrites(plan: ArtifactCratePlan): Promise<{ writes: CarveFileWrite[]; rootLib: CrateMeasure; rootTest: CrateMeasure; fileTexts: Map<string, string> }> {
  const writes: CarveFileWrite[] = [];
  const packageManifestPath = resolve(plan.repoRoot, plan.packageDir, "Cargo.toml");
  const originalManifest = readFileSync(packageManifestPath, "utf8");
  const rows = readManifestDependencyRows(originalManifest);
  const artifactPackageDir = resolve(plan.repoRoot, plan.packageDir);
  const crateById = new Map(plan.crates.map((crate) => [crate.id, crate]));
  const features = forwardableFeatures(originalManifest);
  const featureNames = features.map((feature) => feature.name);
  for (const crate of plan.crates) {
    const packageDir = resolve(plan.repoRoot, crate.packageDir);
    const siblingRows = crate.dependsOn.map((id) => crateById.get(id)!).map((dep) => `${dep.name} = { path = "${relativePosix(packageDir, resolve(plan.repoRoot, dep.packageDir))}" }`);
    const sibling = new Set(siblingRows.map(manifestRowKey));
    const manifest = renderCrateManifest({
      name: crate.name,
      description: crate.description,
      workspaceRel: relativePosix(packageDir, plan.artifactDir),
      libPath: "🦀️.rs",
      dependencyRows: [...rows.dependencies.filter((row) => !sibling.has(manifestRowKey(row))).map((row) => rebaseManifestRow(row, artifactPackageDir, packageDir)), ...siblingRows],
      devDependencyRows: [...rows.devDependencies.map((row) => rebaseManifestRow(row, artifactPackageDir, packageDir)), ...(featureNames.length ? crate.dependsOn.map((id) => crateById.get(id)!).map((dep) => `${dep.name} = { path = "${relativePosix(packageDir, resolve(plan.repoRoot, dep.packageDir))}", features = [${featureNames.map((feature) => JSON.stringify(feature)).join(", ")}] }`) : [])],
      targetSections: rows.targetSections.map((section) => rebaseManifestRow(section, artifactPackageDir, packageDir)),
      featureRows: features.map((feature) => featureRow(feature, crate.dependsOn.map((id) => crateById.get(id)!.name))),
    });
    writes.push({ path: join(crate.packageDir, "Cargo.toml"), action: existsSync(join(packageDir, "Cargo.toml")) ? "update" : "create", content: manifest });
    const mirror = renderMirrorFiles(mirrorSpecOf(plan, crate));
    writes.push({ path: join(crate.packageDir, "🦀️.rs"), action: existsSync(join(packageDir, "🦀️.rs")) ? "update" : "create", content: mirror.root });
    if (mirror.body !== null) writes.push({ path: join(crate.packageDir, "🪞️mirror/🦀️.rs"), action: existsSync(join(packageDir, "🪞️mirror/🦀️.rs")) ? "update" : "create", content: mirror.body });
  }
  const rootText = await carveArtifactRootText(plan);
  writes.push({ path: plan.rootFile, action: "update", content: rootText });
  const rootAbsolute = resolve(plan.repoRoot, plan.rootFile);
  const baseTexts = new Map<string, string>([...plan.overlay.edits, ...plan.overlay.creates]);
  const cutsByFile = new Map<string, { start: number; end: number; text: string }[]>();
  for (const cut of weaveRootEdits(plan.rootEdits, plan.wrapperNodes, plan.weave, plan.placement, plan.fileCuts, plan.gates).cuts) cutsByFile.set(cut.file, [...(cutsByFile.get(cut.file) ?? []), cut]);
  const fileTexts = new Map<string, string>(baseTexts);
  for (const [file, cuts] of cutsByFile) {
    const base = fileTexts.get(file) ?? readFileSync(file, "utf8");
    fileTexts.set(file, applyEdits(base, cuts));
  }
  for (const append of plan.movedAppends) fileTexts.set(append.file, (fileTexts.get(append.file) ?? readFileSync(append.file, "utf8")) + append.text);
  const overlayAbsolute = new Map<string, string>([...fileTexts, [rootAbsolute, rootText]]);
  const rootLib = await measureRustCrate(plan.repoRoot, rootAbsolute, false, overlayAbsolute, artifactPackageDir);
  const rootTest = await measureRustCrate(plan.repoRoot, rootAbsolute, true, overlayAbsolute, artifactPackageDir);
  const owners: { dir: string; paths: Set<string> }[] = [...plan.crates.map((crate) => ({ dir: resolve(plan.repoRoot, crate.packageDir), paths: new Set([...(crate.lib?.paths ?? []), ...(crate.test?.paths ?? [])]) })), { dir: artifactPackageDir, paths: new Set([...rootLib.paths, ...rootTest.paths]) }];
  for (const [absolute, text] of fileTexts) {
    if (absolute === rootAbsolute) continue;
    const repoRelative = relativePosix(plan.repoRoot, absolute);
    const owner = owners.find((entry) => entry.paths.has(repoRelative));
    const content = owner ? resolveFixturePlaceholders(rebaseManifestDirPaths(text, artifactPackageDir, owner.dir), plan.repoRoot, owner.dir) : text;
    writes.push({ path: repoRelative, action: plan.overlay.creates.has(absolute) ? "create" : "update", content });
  }
  const written = new Set(writes.map((write) => write.path));
  for (const entry of owners) {
    if (entry.dir === artifactPackageDir) continue;
    for (const path of entry.paths) {
      if (!path.endsWith(".rs") || written.has(path)) continue;
      const absolute = resolve(plan.repoRoot, path);
      if (!existsSync(absolute) || plan.overlay.deletes.has(absolute)) continue;
      const original = readFileSync(absolute, "utf8");
      const rebased = rebaseManifestDirPaths(original, artifactPackageDir, entry.dir);
      if (rebased !== original) { writes.push({ path, action: "update", content: rebased }); written.add(path); }
    }
  }
  for (const absolute of plan.overlay.deletes) writes.push({ path: relativePosix(plan.repoRoot, absolute), action: "delete", content: "" });
  const carvedRows = plan.crates.map((crate) => `${crate.name} = { path = "${relativePosix(artifactPackageDir, resolve(plan.repoRoot, crate.packageDir))}" }`);
  const dependenciesAt = originalManifest.search(/^\[dependencies\]\s*$/mu);
  if (dependenciesAt < 0) throw new Error("Artifact package manifest has no [dependencies]");
  const lineEnd = originalManifest.indexOf("\n", dependenciesAt) + 1;
  writes.push({ path: join(plan.packageDir, "Cargo.toml"), action: "update", content: withDevDependencyRows(forwardFeatures(originalManifest.slice(0, lineEnd) + carvedRows.join("\n") + "\n" + originalManifest.slice(lineEnd), features, plan.crates.map((crate) => crate.name)), featureNames.length ? plan.crates.map((crate) => `${crate.name} = { path = "${relativePosix(artifactPackageDir, resolve(plan.repoRoot, crate.packageDir))}", features = [${featureNames.map((feature) => JSON.stringify(feature)).join(", ")}] }`) : []) });
  const workspaceManifestPath = resolve(plan.artifactDir, "Cargo.toml");
  writes.push({ path: relativePosix(plan.repoRoot, workspaceManifestPath), action: "update", content: addWorkspaceMembers(readFileSync(workspaceManifestPath, "utf8"), plan.crates.map((crate) => relativePosix(plan.artifactDir, resolve(plan.repoRoot, crate.packageDir)))) });
  const routerPath = resolve(artifactPackageDir, "📜️script.ts");
  if (existsSync(routerPath)) writes.push({ path: join(plan.packageDir, "📜️script.ts"), action: "update", content: patchRouterPackages(readFileSync(routerPath, "utf8"), plan.artifactCrate, plan.crates.map((crate) => crate.name)) });
  const lockPath = resolve(plan.artifactDir, "Cargo.lock");
  if (existsSync(lockPath)) writes.push({ path: relativePosix(plan.repoRoot, lockPath), action: "update", content: patchCargoLock(readFileSync(lockPath, "utf8"), plan.artifactCrate, plan.crates.map((crate) => ({ name: crate.name, dependsOn: crate.dependsOn.map((id) => crateById.get(id)!.name) }))) });
  for (const owner of owners) {
    if (!writes.some((write) => write.path.endsWith(".rs") && owner.paths.has(write.path) && write.content.includes("semio_framework_test_fixtures"))) continue;
    const manifestPath = relativePosix(plan.repoRoot, join(owner.dir, "Cargo.toml"));
    const at = writes.findIndex((write) => write.path === manifestPath);
    if (at >= 0) writes[at] = { ...writes[at]!, content: withFixtureReaderDevDependency(writes[at]!.content, owner.dir, plan.repoRoot) };
  }
  return { writes, rootLib, rootTest, fileTexts };
}

/** 🧭️ The ticket tree of the repository, the only place inside the repository where a carve backup may live. */
const CARVE_BACKUP_TICKET_TREE = ".🧬semio/🦑️repo/🎫️tickets";

/** 🛡️ Resolves the `--backup=` directory of a carve: an absolute path is used as given and a relative one resolves against the repository root. A directory inside the source tree (the repository outside its ticket folders) is refused, because a backup there would be carved, committed or swept like source. */
export function resolveCarveBackupDir(repoRoot: string, backup: string): string {
  const root = realpathSync(repoRoot);
  const target = resolve(root, backup);
  let existing = target;
  while (!existsSync(existing)) {
    const parent = dirname(existing);
    if (parent === existing) break;
    existing = parent;
  }
  const physical = resolve(realpathSync(existing), relative(existing, target));
  const inside = relative(root, physical);
  const outside = isAbsolute(inside) || inside === ".." || inside.startsWith(`..${sep}`);
  if (outside) return physical;
  const ticketTree = CARVE_BACKUP_TICKET_TREE.split("/").join(sep);
  if (inside === ticketTree || inside.startsWith(`${ticketTree}${sep}`)) return physical;
  throw new Error(`new crates --backup=${backup}: ${physical} is inside the source tree; give an absolute directory outside the repository or one below ${CARVE_BACKUP_TICKET_TREE}`);
}

/** 💾️ Writes the planned files, saving a backup of every file it replaces when a backup directory is given. */
export function applyCarveWrites(repoRoot: string, writes: readonly CarveFileWrite[], backupDir?: string): void {
  for (const write of writes) {
    const absolute = resolve(repoRoot, write.path);
    const backup = backupDir ? join(backupDir, write.path) : null;
    if (backup && existsSync(absolute) && !existsSync(backup)) { mkdirSync(dirname(backup), { recursive: true }); writeFileSync(backup, readFileSync(absolute)); }
    if (write.action === "delete") { rmSync(absolute, { force: true }); continue; }
    mkdirSync(dirname(absolute), { recursive: true });
    writeFileSync(absolute, write.content);
  }
}

/** 🗂️ Registers the carved crates in `📋️build-owners.json`, re-reading the file at call time because peers edit it concurrently; the added names are listed beside the backup so a restore removes exactly those rows instead of overwriting the shared file. */
export function registerBuildOwnerRows(repoRoot: string, plan: ArtifactCratePlan, backupDir?: string): string[] {
  const file = resolve(repoRoot, "📋️build-owners.json");
  const { text, added } = insertBuildOwnerRows(readFileSync(file, "utf8"), plan.crates.map((crate) => ({ name: crate.name, root: crate.packageDir })));
  if (!added.length) return added;
  if (backupDir) { mkdirSync(dirname(backupDir), { recursive: true }); writeFileSync(`${backupDir}-owner-rows.json`, JSON.stringify(added, null, 2) + "\n"); }
  writeFileSync(file, text);
  return added;
}
