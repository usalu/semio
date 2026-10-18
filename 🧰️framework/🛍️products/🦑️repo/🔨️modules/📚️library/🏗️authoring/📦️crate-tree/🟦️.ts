import { existsSync, mkdirSync, readFileSync, realpathSync, rmSync, writeFileSync } from "node:fs";
import { dirname, isAbsolute, join, relative, resolve, sep } from "node:path";
import { crateIdent } from "./🗺️plan/🏷️naming/🟦️.ts";
import { crateTreeHostCfg, measureRustCrate, readMeasureSources } from "./🗺️plan/📏️measure/🟦️.ts";
import { identifierSet, useLeafNames } from "./🌳️module-tree/🟦️.ts";
import { planArtifactCrates } from "./🗺️plan/🧮️pack/🟦️.ts";
import { readCargoLib } from "./🗺️plan/📦️cargo-lib/🟦️.ts";
import { weaveRootEdits } from "./🗺️plan/🪢️weave/🟦️.ts";
import { type ArtifactCratePlan, type CrateMeasure, type PlannedCrate } from "./🗺️plan/🧾️model/🟦️.ts";
import { buildTestkitCrate, cfgFeatures, declaredNames, externalDependencies, optionalDependencyRow, planTestkitWiring, requiredTestFeatures, scanTestGates, testOnlyGate, testkitFeatureRow, withFeatureRows, withRequiredFeatures, type CrateWiring, type ExternalDependency, type ExternalGateReader, type TestkitCrate } from "./🧰️testkit/🟦️.ts";
import { addWorkspaceMembers, syncBuildOwnerRows, manifestRowKey, rebaseManifestDirPaths, renderMirrorFiles, patchCargoLock, patchRouterPackages, readManifestDependencyRows, rebaseManifestRow, relativePosix, renderCrateManifest, renderMirrorRoot, type ManifestDependencyRows, type MirrorSpec, type MirrorUnit } from "./📝️render/🟦️.ts";
import { demoteDanglingOuterDocs, parseRustSourceTree, rustOuterDocStart } from "./🌳️module-tree/🟦️.ts";
import { FIXTURE_READER_CRATE, FIXTURE_READER_PACKAGE_DIR, resolveFixturePlaceholders } from "./🧪️test-closure/🟦️.ts";

export { planArtifactCrates } from "./🗺️plan/🧮️pack/🟦️.ts";
export type { ArtifactCratePlan, PlannedCrate } from "./🗺️plan/🧾️model/🟦️.ts";
export { CRATE_HARD_BUDGET, CRATE_TARGET_BUDGET, parseCrateBudget } from "./🗺️plan/💰️budget/🟦️.ts";
export { measureFits, measureRustCrate } from "./🗺️plan/📏️measure/🟦️.ts";

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
  const testGated = new Set(plan.tree.items.filter((item) => item.name && testOnlyGate(item.attributeTexts) !== null).map((item) => item.name!));
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

/** 🧰️ Reads the text of a carved source file from the overlay, or from disk when the carve leaves it unchanged. */
function carvedText(repoRoot: string, overlay: ReadonlyMap<string, string>, repoRelative: string): string | null {
  const absolute = resolve(repoRoot, repoRelative);
  const virtual = overlay.get(absolute);
  if (virtual !== undefined) return virtual;
  try { return readFileSync(absolute, "utf8"); } catch { return null; }
}

/** 🧰️ Builds the wiring analysis input of one crate (or of the facade) from its library and test closures, with the closure that each test-support feature adds. */
async function testkitCrateOf(plan: ArtifactCratePlan, overlay: ReadonlyMap<string, string>, spec: Readonly<{ name: string; dependsOn: readonly string[]; entry: string; manifestDir: string; lib: CrateMeasure | null; test: CrateMeasure | null; extra: ReadonlyMap<string, string>; ownedFiles?: ReadonlySet<string> }>): Promise<TestkitCrate> {
  const files = new Map<string, string>();
  for (const path of [...(spec.lib?.paths ?? []), ...(spec.test?.paths ?? [])]) {
    if (!path.endsWith(".rs") || files.has(path)) continue;
    const text = spec.extra.get(path) ?? carvedText(plan.repoRoot, overlay, path);
    if (text !== null) files.set(path, text);
  }
  for (const [path, text] of spec.extra) if (path.endsWith(".rs")) files.set(path, text);
  const base = new Set(spec.lib?.paths ?? []);
  const featureClosure = async (feature: string): Promise<ReadonlySet<string>> => {
    const measure = await measureRustCrate(plan.repoRoot, spec.entry, false, overlay, spec.manifestDir, [...crateTreeHostCfg(), `feature=${feature}`]);
    return new Set(measure.paths.filter((path) => path.endsWith(".rs") && !base.has(path)));
  };
  return buildTestkitCrate({ name: spec.name, ident: crateIdent(spec.name), dependsOn: spec.dependsOn, ownedFiles: spec.ownedFiles, files }, featureClosure);
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
  const mirrors = new Map(plan.crates.map((crate) => [crate.id, renderMirrorFiles(mirrorSpecOf(plan, crate))]));
  const rootText = await carveArtifactRootText(plan);
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
  const wiring = await planCarveWiring(plan, originalManifest, rows, features, mirrors, overlayAbsolute, rootText, rootLib, rootTest);
  for (const crate of plan.crates) {
    const packageDir = resolve(plan.repoRoot, crate.packageDir);
    const siblingRows = crate.dependsOn.map((id) => crateById.get(id)!).map((dep) => `${dep.name} = { path = "${relativePosix(packageDir, resolve(plan.repoRoot, dep.packageDir))}" }`);
    const sibling = new Set(siblingRows.map(manifestRowKey));
    const mine = wiring.crates.get(crate.name)!;
    const devFeatures = new Map<string, Set<string>>();
    for (const dep of crate.dependsOn.map((id) => crateById.get(id)!)) devFeatures.set(dep.name, new Set([...(featureNames.length ? featureNames : []), ...(mine.consumed.get(dep.name) ?? [])]));
    const siblingDevRows = [...devFeatures].filter(([, names]) => names.size).map(([name, names]) => {
      const dep = plan.crates.find((entry) => entry.name === name)!;
      return `${name} = { path = "${relativePosix(packageDir, resolve(plan.repoRoot, dep.packageDir))}", features = [${[...names].sort().map((feature) => JSON.stringify(feature)).join(", ")}] }`;
    });
    const optionalRows = mine.optionalDependencies.map((key) => rebaseManifestRow(optionalDependencyRow(wiring.externals.find((entry) => entry.key === key)!.devRow!), artifactPackageDir, packageDir));
    const manifest = renderCrateManifest({
      name: crate.name,
      description: crate.description,
      workspaceRel: relativePosix(packageDir, plan.artifactDir),
      libPath: "🦀️.rs",
      dependencyRows: [...rows.dependencies.filter((row) => !sibling.has(manifestRowKey(row))).map((row) => rebaseManifestRow(row, artifactPackageDir, packageDir)), ...siblingRows, ...optionalRows],
      devDependencyRows: [...rows.devDependencies.map((row) => rebaseManifestRow(row, artifactPackageDir, packageDir)), ...siblingDevRows],
      targetSections: rows.targetSections.map((section) => rebaseManifestRow(section, artifactPackageDir, packageDir)),
      featureRows: [...features.map((feature) => featureRow(feature, crate.dependsOn.map((id) => crateById.get(id)!.name))), ...mine.features.map(testkitFeatureRow)],
    });
    writes.push({ path: join(crate.packageDir, "Cargo.toml"), action: existsSync(join(packageDir, "Cargo.toml")) ? "update" : "create", content: manifest });
    const mirror = mirrors.get(crate.id)!;
    writes.push({ path: join(crate.packageDir, "🦀️.rs"), action: existsSync(join(packageDir, "🦀️.rs")) ? "update" : "create", content: mirror.root });
    if (mirror.body !== null) writes.push({ path: join(crate.packageDir, "🪞️mirror/🦀️.rs"), action: existsSync(join(packageDir, "🪞️mirror/🦀️.rs")) ? "update" : "create", content: mirror.body });
  }
  writes.push({ path: plan.rootFile, action: "update", content: rootText });
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
  const facade = wiring.crates.get(plan.artifactCrate)!;
  const facadeDevRows = plan.crates.flatMap((crate) => {
    const names = new Set([...(featureNames.length ? featureNames : []), ...(facade.consumed.get(crate.name) ?? [])]);
    return names.size ? [`${crate.name} = { path = "${relativePosix(artifactPackageDir, resolve(plan.repoRoot, crate.packageDir))}", features = [${[...names].sort().map((feature) => JSON.stringify(feature)).join(", ")}] }`] : [];
  });
  const forwardedFacade = forwardFeatures(originalManifest.slice(0, lineEnd) + carvedRows.join("\n") + "\n" + originalManifest.slice(lineEnd), features, plan.crates.map((crate) => crate.name));
  const facadeOptional = facade.optionalDependencies.map((key) => optionalDependencyRow(wiring.externals.find((entry) => entry.key === key)!.devRow!));
  const facadeText = withRequiredFeatures(withFeatureRows(withDevDependencyRows(facadeOptional.length ? insertDependencyRows(forwardedFacade, facadeOptional) : forwardedFacade, facadeDevRows), facade.features), wiring.requiredFeatures);
  writes.push({ path: join(plan.packageDir, "Cargo.toml"), action: "update", content: facadeText });
  const workspaceManifestPath = resolve(plan.artifactDir, "Cargo.toml");
  writes.push({ path: relativePosix(plan.repoRoot, workspaceManifestPath), action: "update", content: addWorkspaceMembers(readFileSync(workspaceManifestPath, "utf8"), plan.crates.map((crate) => relativePosix(plan.artifactDir, resolve(plan.repoRoot, crate.packageDir)))) });
  const routerPath = resolve(artifactPackageDir, "📜️script.ts");
  if (existsSync(routerPath)) writes.push({ path: join(plan.packageDir, "📜️script.ts"), action: "update", content: patchRouterPackages(readFileSync(routerPath, "utf8"), plan.artifactCrate, plan.crates.map((crate) => crate.name)) });
  const lockPath = resolve(plan.artifactDir, "Cargo.lock");
  if (existsSync(lockPath)) {
    const memberNames = new Set<string>();
    for (const member of ((Bun.TOML.parse(readFileSync(workspaceManifestPath, "utf8")) as { workspace?: { members?: string[] } }).workspace?.members ?? [])) {
      try { memberNames.add((Bun.TOML.parse(readFileSync(resolve(plan.artifactDir, member, "Cargo.toml"), "utf8")) as { package: { name: string } }).package.name); } catch { continue; }
    }
    writes.push({ path: relativePosix(plan.repoRoot, lockPath), action: "update", content: patchCargoLock(readFileSync(lockPath, "utf8"), plan.artifactCrate, plan.crates.map((crate) => ({ name: crate.name, dependsOn: crate.dependsOn.map((id) => crateById.get(id)!.name) })), memberNames) });
  }
  for (const owner of owners) {
    if (!writes.some((write) => write.path.endsWith(".rs") && owner.paths.has(write.path) && write.content.includes("semio_framework_test_fixtures"))) continue;
    const manifestPath = relativePosix(plan.repoRoot, join(owner.dir, "Cargo.toml"));
    const at = writes.findIndex((write) => write.path === manifestPath);
    if (at >= 0) writes[at] = { ...writes[at]!, content: withFixtureReaderDevDependency(writes[at]!.content, owner.dir, plan.repoRoot) };
  }
  return { writes, rootLib, rootTest, fileTexts };
}

const EXTERNAL_GATE_CACHE = new Map<string, ReadonlySet<string> | null>();

type CargoDependencySpec = { path?: string; workspace?: boolean };

/** 🔍️ The directory of the dependency `key` of the package manifest at `manifestPath`: the `path` of its row, or, for an inherited row, the `path` of the workspace dependency of the workspace root the package belongs to. */
function dependencyDirectory(manifestPath: string, key: string): string | null {
  const parsed = Bun.TOML.parse(readFileSync(manifestPath, "utf8")) as { package?: { workspace?: string }; dependencies?: Record<string, CargoDependencySpec | string>; "dev-dependencies"?: Record<string, CargoDependencySpec | string> };
  const spec = parsed.dependencies?.[key] ?? parsed["dev-dependencies"]?.[key];
  if (spec === undefined || typeof spec === "string") return null;
  if (typeof spec.path === "string") return resolve(dirname(manifestPath), spec.path);
  if (spec.workspace !== true) return null;
  let cursor = parsed.package?.workspace !== undefined ? resolve(dirname(manifestPath), parsed.package.workspace) : dirname(manifestPath);
  for (;;) {
    const candidate = join(cursor, "Cargo.toml");
    if (existsSync(candidate) && candidate !== manifestPath) {
      const root = Bun.TOML.parse(readFileSync(candidate, "utf8")) as { workspace?: { dependencies?: Record<string, CargoDependencySpec> } };
      const inherited = root.workspace?.dependencies?.[key]?.path;
      if (root.workspace !== undefined) return inherited === undefined ? null : resolve(cursor, inherited);
    }
    const parent = dirname(cursor);
    if (parent === cursor) return null;
    cursor = parent;
  }
}

/** 🔍️ The names a crate declares behind one of its features: the features it implies in the same crate, the dependency features it forwards (followed through the dependency's sources), the files only those features compile, and the exported items under their gates. */
async function featureGatedNames(repoRoot: string, dir: string, feature: string, seen: Set<string>): Promise<Set<string> | null> {
  const key = `${dir}\0${feature}`;
  if (seen.has(key)) return new Set();
  seen.add(key);
  const cached = EXTERNAL_GATE_CACHE.get(key);
  if (cached !== undefined) return cached === null ? null : new Set(cached);
  try {
    const manifestPath = join(dir, "Cargo.toml");
    const table = (Bun.TOML.parse(readFileSync(manifestPath, "utf8")) as { features?: Record<string, string[]> }).features ?? {};
    const implied = new Set<string>([feature]);
    const forwarded: [string, string][] = [];
    for (const name of implied) for (const entry of table[name] ?? []) {
      const slash = /^([A-Za-z0-9_-]+)\??\/([A-Za-z0-9_-]+)$/u.exec(entry);
      if (slash) forwarded.push([slash[1]!, slash[2]!]);
      else if (!entry.startsWith("dep:")) implied.add(entry);
    }
    const entry = readCargoLib(manifestPath).entry;
    const base = await measureRustCrate(repoRoot, entry, false, new Map(), dir);
    const full = await measureRustCrate(repoRoot, entry, false, new Map(), dir, [...crateTreeHostCfg(), ...[...implied].map((name) => `feature=${name}`)]);
    const baseline = new Set(base.paths);
    const names = new Set<string>();
    for (const path of full.paths) {
      if (!path.endsWith(".rs")) continue;
      const text = readFileSync(resolve(repoRoot, path), "utf8");
      if (!baseline.has(path)) for (const name of declaredNames(text)) names.add(name);
      for (const gate of scanTestGates(text, (attribute) => cfgFeatures(attribute).filter((name) => implied.has(name)))) for (const name of gate.names) names.add(name);
    }
    for (const [dependency, forwardedFeature] of forwarded) {
      const target = dependencyDirectory(manifestPath, dependency);
      const inner = target === null ? null : await featureGatedNames(repoRoot, target, forwardedFeature, seen);
      if (inner === null) { EXTERNAL_GATE_CACHE.set(key, null); return null; }
      for (const name of inner) names.add(name);
    }
    EXTERNAL_GATE_CACHE.set(key, names);
    return new Set(names);
  } catch {
    EXTERNAL_GATE_CACHE.set(key, null);
    return null;
  }
}

/** 🔍️ Where an external dependency of the artifact package lives: the `path` of its row, or the `path` of the workspace dependency its row inherits. */
function externalCrateDir(external: ExternalDependency, artifactPackageDir: string, artifactDir: string, workspaceDependencies: Record<string, { path?: string }>): string | null {
  const own = /path\s*=\s*"([^"]*)"/u.exec(external.row ?? "")?.[1];
  if (own !== undefined) return resolve(artifactPackageDir, own);
  const inherited = workspaceDependencies[external.key]?.path;
  return inherited === undefined ? null : resolve(artifactDir, inherited);
}

/** 🔍️ Reads, for every feature that a dev-dependency row adds to an external dependency, the names the dependency declares behind that feature (the features it implies and the dependency features it forwards), so the wiring forwards a feature only when the gated code names something the feature adds. A dependency whose sources cannot be read maps to `null` and its features are forwarded. */
export async function loadExternalGates(repoRoot: string, artifactPackageDir: string, artifactDir: string, externals: readonly ExternalDependency[]): Promise<ExternalGateReader> {
  const workspace = (Bun.TOML.parse(readFileSync(join(artifactDir, "Cargo.toml"), "utf8")) as { workspace?: { dependencies?: Record<string, { path?: string }> } }).workspace?.dependencies ?? {};
  const found = new Map<string, ReadonlySet<string> | null>();
  for (const external of externals) for (const feature of external.devFeatures) {
    const dir = externalCrateDir(external, artifactPackageDir, artifactDir, workspace);
    found.set(`${external.key}\0${feature}`, dir === null ? null : await featureGatedNames(repoRoot, dir, feature, new Set()));
  }
  return (external, feature) => found.get(`${external.key}\0${feature}`) ?? null;
}

/** 🧰️ The test-support wiring of a whole carve: per crate (and for the facade, under the artifact crate's name) the gate features, optional dependencies and consumed features, the external dependencies they refer to, and the `required-features` of the facade's `[[test]]` targets. */
export type CarveWiring = Readonly<{ crates: ReadonlyMap<string, CrateWiring>; externals: readonly ExternalDependency[]; requiredFeatures: ReadonlyMap<string, readonly string[]> }>;

async function planCarveWiring(plan: ArtifactCratePlan, originalManifest: string, rows: ManifestDependencyRows, features: readonly ManifestFeature[], mirrors: ReadonlyMap<string, { root: string; body: string | null }>, overlay: ReadonlyMap<string, string>, rootText: string, rootLib: CrateMeasure, rootTest: CrateMeasure): Promise<CarveWiring> {
  const crateById = new Map(plan.crates.map((crate) => [crate.id, crate]));
  const inputs: TestkitCrate[] = [];
  const full = new Map(overlay);
  for (const crate of plan.crates) {
    const mirror = mirrors.get(crate.id)!;
    full.set(resolve(plan.repoRoot, crate.packageDir, "🦀️.rs"), mirror.root);
    if (mirror.body !== null) full.set(resolve(plan.repoRoot, crate.packageDir, "🪞️mirror/🦀️.rs"), mirror.body);
  }
  for (const crate of plan.crates) {
    const mirror = mirrors.get(crate.id)!;
    const extra = new Map<string, string>([[join(crate.packageDir, "🦀️.rs"), mirror.root]]);
    if (mirror.body !== null) extra.set(join(crate.packageDir, "🪞️mirror/🦀️.rs"), mirror.body);
    inputs.push(await testkitCrateOf(plan, full, { name: crate.name, dependsOn: crate.dependsOn.map((id) => crateById.get(id)!.name), entry: resolve(plan.repoRoot, crate.packageDir, "🦀️.rs"), manifestDir: resolve(plan.repoRoot, crate.packageDir), lib: crate.lib, test: crate.test, extra, ownedFiles: new Set(crate.slices.flatMap((slice) => slice.decls.map((decl) => decl.file ?? decl.declFile)).map((file) => relativePosix(plan.repoRoot, file))) }));
  }
  const artifactPackageDir = resolve(plan.repoRoot, plan.packageDir);
  inputs.push(await testkitCrateOf(plan, full, { name: plan.artifactCrate, dependsOn: plan.crates.map((crate) => crate.name), entry: resolve(plan.repoRoot, plan.rootFile), manifestDir: artifactPackageDir, lib: rootLib, test: rootTest, extra: new Map([[plan.rootFile, rootText]]) }));
  const externals = externalDependencies(originalManifest, rows);
  const crates = planTestkitWiring(inputs, externals, new Set(features.map((feature) => feature.name)), await loadExternalGates(plan.repoRoot, artifactPackageDir, plan.artifactDir, externals));
  const requiredFeatures = new Map<string, readonly string[]>();
  const facadeNames = crates.get(plan.artifactCrate)?.gatedNames ?? new Map<string, ReadonlySet<string>>();
  if (facadeNames.size) {
    const targets = ((Bun.TOML.parse(originalManifest) as { test?: { name?: string; path?: string }[] }).test ?? []).filter((target) => target.name && target.path);
    for (const target of targets) {
      const entry = resolve(artifactPackageDir, target.path!);
      const measure = await measureRustCrate(plan.repoRoot, entry, true, full, artifactPackageDir).catch(() => null);
      if (!measure) continue;
      const tokens = new Set<string>();
      for (const path of measure.paths) if (path.endsWith(".rs")) { const text = carvedText(plan.repoRoot, full, path); if (text !== null) for (const identifier of identifierSet(text)) tokens.add(identifier); }
      const needed = requiredTestFeatures(tokens, facadeNames);
      if (needed.length) requiredFeatures.set(target.name!, needed);
    }
  }
  return { crates, externals, requiredFeatures };
}

/** 🧲️ Adds dependency rows to the `[dependencies]` section of a manifest, after the rows it has. */
function insertDependencyRows(manifestText: string, rows: readonly string[]): string {
  const lines = manifestText.split("\n");
  lines.splice(lines.findIndex((line) => /^\[dependencies\]\s*$/u.test(line)) + 1, 0, ...rows);
  return lines.join("\n");
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

/** 🗂️ Registers the carved crates in `📋️build-owners.json`, re-reading the file at call time because peers edit it concurrently. New rows are inserted, a row at the same root under an older name takes the new name, and the rows of this artifact whose manifest is gone are dropped; the added names are listed beside the backup so a restore removes exactly those rows instead of overwriting the shared file, and the dropped and renamed rows are listed there too so that a restore can bring them back. */
export function registerBuildOwnerRows(repoRoot: string, plan: ArtifactCratePlan, backupDir?: string): string[] {
  const file = resolve(repoRoot, "📋️build-owners.json");
  const scope = { artifactCrate: plan.artifactCrate, artifactDir: relativePosix(repoRoot, plan.artifactDir), exists: (root: string) => existsSync(resolve(repoRoot, root, "Cargo.toml")) };
  const { text, added, renamed, removed } = syncBuildOwnerRows(readFileSync(file, "utf8"), plan.crates.map((crate) => ({ name: crate.name, root: crate.packageDir })), scope);
  if (!added.length && !renamed.length && !removed.length) return added;
  if (backupDir) {
    mkdirSync(dirname(backupDir), { recursive: true });
    writeFileSync(`${backupDir}-owner-rows.json`, JSON.stringify(added, null, 2) + "\n");
    if (renamed.length || removed.length) writeFileSync(`${backupDir}-owner-rows-changed.json`, JSON.stringify({ renamed, removed }, null, 2) + "\n");
  }
  writeFileSync(file, text);
  return added;
}
