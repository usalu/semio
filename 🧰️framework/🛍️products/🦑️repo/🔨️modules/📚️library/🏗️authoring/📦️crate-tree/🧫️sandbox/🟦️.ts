import { execFileSync, spawnSync } from "node:child_process";
import { cpSync, existsSync, mkdirSync, readdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join, posix, relative, resolve } from "node:path";
import { rustTokens } from "@semio-tech/compiler-syntax-rust-ts";
import { carveWrites } from "../🟦️.ts";
import { allItems } from "../🗺️plan/🧰️tree-helpers/🟦️.ts";
import { crateIdent } from "../🗺️plan/🏷️naming/🟦️.ts";
import { crateRelativeChains, implHeader } from "../🗺️plan/🧭️module-chains/🟦️.ts";
import { measureRustCrate } from "../🗺️plan/📏️measure/🟦️.ts";
import { readCargoLib } from "../🗺️plan/📦️cargo-lib/🟦️.ts";
import { type ArtifactCratePlan } from "../🗺️plan/🧾️model/🟦️.ts";
import { classifyPackageSource } from "../../../🔍️discovery/🏷️package-source-classification/🟦️.ts";
import { loadCatalogTaxonomy } from "../../../🔍️discovery/🔐️taxonomy-authority/🟦️.ts";
import { resolveFixturePlaceholders } from "../🧪️test-closure/🟦️.ts";
import { relativePosix } from "../📝️render/🟦️.ts";
import { buildSandbox, checkCrossCrateVisibility, checkOrphanRules, compareModuleTrees, diffDiagnostics, patchSandboxSources, resolveCrateModules, runResolutionOracle, rustcUnresolvedCrates } from "../🔬️verify/🟦️.ts";

/** 🧫️ Everything a carve is checked against inside a sandbox copy of the plugin. */
export type CarveVerification = {
  census: { name: string; lib: string; test: string; withinHardBudget: boolean; withinTargetBudget: boolean }[];
  moduleTreeProblems: string[];
  visibility: unknown[];
  orphan: unknown[];
  glue: string[];
  unresolvedCrates: Record<string, string[]>;
  inheritedUnresolvedCrates: string[];
  oracle: Record<string, { original: number; candidate: number; onlyOriginal: number; onlyCandidate: number; sampleOnlyCandidate: string[]; sampleOnlyOriginal: string[] }> | null;
  layout: Record<string, { union: number; combined: number; onlyCombined: number; sampleOnlyCombined: string[] }> | null;
};

export function mountedOutsideDirectories(repoRoot: string, pluginDir: string, scanRoot = repoRoot): Set<string> {
  const found = new Set<string>();
  const walk = (directory: string): void => {
    for (const entry of readdirSync(directory, { withFileTypes: true })) {
      const path = join(directory, entry.name);
      if (entry.isDirectory()) { if (!/^(node_modules|target|dist)$/u.test(entry.name)) walk(path); continue; }
      if (!entry.name.endsWith(".rs")) continue;
      const sourceText = readFileSync(path, "utf8");
      for (const match of sourceText.matchAll(/include(?:_str|_bytes)?!\(\s*"([^"]+)"\s*\)/gu)) {
        const target = resolve(directory, match[1]!);
        const relative = relativePosix(scanRoot, dirname(target));
        if (!relative.startsWith("..") && !`${relative}/`.startsWith(`${pluginDir}/`) && existsSync(resolve(repoRoot, relative))) found.add(relative);
      }
      for (const match of sourceText.matchAll(/#\[path\s*=\s*"([^"]+)"\s*\]/gu)) {
        const target = resolve(directory, match[1]!);
        const relative = relativePosix(scanRoot, dirname(target));
        if (!relative.startsWith("..") && !`${relative}/`.startsWith(`${pluginDir}/`) && existsSync(resolve(repoRoot, relative))) found.add(relative);
      }
    }
  };
  walk(resolve(scanRoot, pluginDir));
  return found;
}

/** 🧵️ Rewrites one source file of a carved crate so that all carved crates can be mounted as modules of one synthetic crate: `crate::` becomes `crate::<module>::` and a dependency crate's name becomes the path of its module. */
export function rewriteForCombined(text: string, module: string, crateModules: ReadonlyMap<string, string>): string {
  const tokens = rustTokens(text);
  const edits: { start: number; end: number; text: string }[] = [];
  for (let k = 0; k + 1 < tokens.length; k += 1) {
    const token = tokens[k]!;
    const previous = tokens[k - 1]?.text;
    if (tokens[k + 1]!.text !== "::" || previous === "::" || previous === "." || previous === "$") continue;
    if (token.text === "crate") edits.push({ start: token.start, end: token.end, text: `crate::${module}` });
    else if (token.kind === "identifier" && crateModules.has(token.text)) edits.push({ start: token.start, end: token.end, text: `crate::${crateModules.get(token.text)}` });
  }
  let out = text;
  for (const edit of edits.sort((left, right) => right.start - left.start)) out = out.slice(0, edit.start) + edit.text + out.slice(edit.end);
  return out;
}

/** 🔗️ Whether a diagnostic is about anything but an external crate that the sandbox does not link (an associated type looked up on a generic parameter bound by such a crate's trait is one of them: the base crate aborts at resolution before it type-checks, a carved crate with no unresolved item header does not): a path through the extern prelude (`::semio_ext::x`) names such a crate in the 2021 edition, and rustc reports it as "cannot find `semio_ext` in the crate root". */
export const external = (line: string): boolean => !line.includes("unlinked crate") && !/\tcannot find `[A-Za-z0-9_]+` in the crate root\t/u.test(line) && !/^E0432\tunresolved import `semio_[A-Za-z0-9_]+`(?:\t|$)/u.test(line) && !/^E0220\tassociated type `[A-Za-z0-9_]+` not found for `[A-Z][A-Za-z0-9_]*`(?:\t|$)/u.test(line);

/** 📍️ Drops the file and label of the diagnostics about unresolved derive helper attributes and derive macros (the sandbox links no macro crate, so every derive reports them): an item that moves to another file keeps its count but changes its location. The label of a type or value that is not found in scope is dropped too: a type that an unlinked macro would have generated is reported with or without a suggestion depending on the imports around it, which the carve legitimately changes. */
export const placeless = (line: string): string => {
  if (/^[^\t]*\tcannot find (?:attribute|derive macro) /u.test(line)) return line.split("\t").slice(0, 2).join("\t");
  return /^[^\t]*\tcannot find (?:type|struct, variant or union type|value|function) `[^`]+` in this scope\t/u.test(line) ? line.split("\t").slice(0, 3).join("\t") : line;
};

/** 🧩️ The names a stripped diagnostic reports as unresolved (an import, a type, a value, a function or the last segment of a path, `cannot find `Name` in `module``), as the last segment of each backticked path; empty for any other diagnostic. */
export function unresolvedSubjects(line: string): string[] {
  const inPath = /^E0433\tcannot find `([^`]+)` in `[^`]+`/u.exec(line);
  if (inPath) return [inPath[1]!];
  const match = /^E0(?:412|422|425|432|433)\t(?:unresolved imports? |cannot find (?:type|value|struct, variant or union type|function|trait|macro) )(.*)$/u.exec(line);
  if (!match) return [];
  const subject = match[1]!.split(": ")[0]!.split(" in this scope")[0]!.split(" in module")[0]!;
  return [...subject.matchAll(/`([^`]+)`/gu)].map((entry) => entry[1]!.split("::").pop()!);
}

/** 🧩️ Drops the carved diagnostics that only re-word an unresolved name the base tree already fails to resolve: a name an unexpanded macro defines is reported as `unresolved import` in one layout and as `cannot find .. in this scope` in the other, so the two spellings must not count as a regression. */
export function dropBaseUnresolvedNames(baseLines: readonly string[], carvedLines: readonly string[]): string[] {
  const known = new Set(baseLines.flatMap(unresolvedSubjects));
  return carvedLines.filter((line) => { const subjects = unresolvedSubjects(line); return !subjects.length || !subjects.every((name) => known.has(name)); });
}

export const normalizeCombined = (line: string): string => line.replace(/crate::__c_[a-z0-9_]+/gu, "crate").replace(/__c_[a-z0-9_]+(::)?/gu, "").replace(/<sandbox>(?:\/\.\.)+/gu, "<sandbox>/..");

/** 🪞️ Rewrites the imports of the combined layout that name a crate without a following path: `use <crate> as name;` and `use crate as name;`, which the token rewrite of `rewriteForCombined` leaves alone. */
export function rewriteCrateAliases(text: string, module: string, crateModules: ReadonlyMap<string, string>): string {
  let out = text.replace(/(\buse\s+)crate(\s+as\b)/gu, `$1crate::${module}$2`);
  for (const [ident, mounted] of crateModules) out = out.replace(new RegExp(`(\\buse\\s+)${ident}(\\s+as\\b)`, "gu"), `$1crate::${mounted}$2`);
  return out;
}

/** 🔗️ A path dependency of an artifact crate that lives in the same plugin outside the artifact, with the extern identifiers the dependents use for it. */
export type SiblingCrate = Readonly<{ name: string; dir: string; entry: string; idents: readonly string[] }>;

/** 🔗️ Collects the path dependencies of the given crate manifests that live outside the artifact: plugin-local ones transitively, any other repository crate only as a direct dependency. These are the sibling crates an artifact crate may now depend on instead of mounting their sources; the caller keeps the ones whose sources the base tree compiled in-crate. */
export function siblingPathCrates(repoRoot: string, pluginDir: string, artifactDir: string, manifests: readonly string[], workspaceDir: string = artifactDir): SiblingCrate[] {
  const found = new Map<string, { name: string; entry: string; idents: Set<string> }>();
  const inherited = (Bun.TOML.parse(readFileSync(join(repoRoot, workspaceDir, "Cargo.toml"), "utf8")) as { workspace?: { dependencies?: Record<string, { path?: string }> } }).workspace?.dependencies ?? {};
  const queue = [...manifests];
  const seen = new Set(queue);
  while (queue.length) {
    const manifest = queue.pop()!;
    const parsed = Bun.TOML.parse(readFileSync(join(repoRoot, manifest), "utf8")) as Record<string, unknown>;
    const tables: Record<string, { path?: string; package?: string; workspace?: boolean }>[] = [];
    for (const section of ["dependencies", "dev-dependencies"]) if (parsed[section]) tables.push(parsed[section] as typeof tables[number]);
    for (const target of Object.values((parsed.target ?? {}) as Record<string, Record<string, unknown>>)) for (const section of ["dependencies", "dev-dependencies"]) if (target[section]) tables.push(target[section] as typeof tables[number]);
    for (const table of tables) for (const [key, spec] of Object.entries(table)) {
      if (typeof spec !== "object" || spec === null) continue;
      const viaWorkspace = spec.workspace === true && typeof inherited[key]?.path === "string";
      if (typeof spec.path !== "string" && !viaWorkspace) continue;
      const directory = relativePosix(repoRoot, viaWorkspace ? resolve(repoRoot, workspaceDir, inherited[key]!.path!) : resolve(repoRoot, dirname(manifest), spec.path!));
      const next = `${directory}/Cargo.toml`;
      const local = directory.startsWith(`${pluginDir}/`);
      if (directory.startsWith(`${artifactDir}/`) || directory.startsWith("..") || !existsSync(join(repoRoot, next))) continue;
      const lib = readCargoLib(join(repoRoot, next));
      const row = found.get(directory) ?? { name: lib.name, entry: lib.entry, idents: new Set<string>() };
      row.idents.add(crateIdent(lib.name));
      row.idents.add(key.replaceAll("-", "_"));
      found.set(directory, row);
      if (local && !seen.has(next)) { seen.add(next); queue.push(next); }
    }
  }
  return [...found].map(([dir, row]) => ({ name: row.name, dir, entry: row.entry, idents: [...row.idents] }));
}

/** 🧫️ Applies a planned carve to a sandbox copy of the plugin and runs every static check and, optionally, the full-resolution differential of the test rewrites. */
export async function verifyCarveInSandbox(plan: ArtifactCratePlan, sandboxRoot: string, options: { oracle: boolean; keep?: boolean }): Promise<CarveVerification> {
  const repoRoot = plan.repoRoot;
  const pluginDir = plan.packageDir.split("/").slice(0, 3).join("/");
  const outside = new Set<string>();
  const { writes, rootLib, rootTest } = await carveWrites(plan);
  const closures = [plan.rootMeasure?.lib, plan.rootMeasure?.test, rootLib, rootTest, ...plan.crates.flatMap((crate) => [crate.lib, crate.test])];
  for (const closure of closures) for (const path of closure?.paths ?? []) if (!path.startsWith(`${pluginDir}/`)) outside.add(path);
  const directories = [pluginDir, ...new Set([...[...outside].map((path) => dirname(path)).filter((directory) => existsSync(join(repoRoot, directory))), ...mountedOutsideDirectories(repoRoot, pluginDir)])];
  buildSandbox(repoRoot, sandboxRoot, directories);
  const applyWrites = (root: string): void => {
  for (const write of writes) {
    const target = join(root, write.path);
    if (write.action === "delete") rmSync(target, { force: true });
    else { mkdirSync(dirname(target), { recursive: true }); writeFileSync(target, write.content); }
  }
  };
  applyWrites(sandboxRoot);
  const census: CarveVerification["census"] = [];
  const hard = { files: 32, bytes: 262144, lines: 4000 };
  const target = { files: 28, bytes: 230000, lines: 3500 };
  const within = (m: { files: number; bytes: number; lines: number }, b: typeof hard): boolean => m.files <= b.files && m.bytes <= b.bytes && m.lines <= b.lines;
  const rowFor = async (name: string, packageDir: string): Promise<void> => {
    const manifest = join(sandboxRoot, packageDir);
    const carvedEntry = join(manifest, "🦀️.rs");
    const entry = existsSync(carvedEntry) ? carvedEntry : readCargoLib(join(manifest, "Cargo.toml")).entry;
    const lib = await measureRustCrate(sandboxRoot, entry, false, new Map(), manifest);
    const test = await measureRustCrate(sandboxRoot, entry, true, new Map(), manifest);
    census.push({ name, lib: `${lib.files}/${lib.bytes}/${lib.lines}`, test: `${test.files}/${test.bytes}/${test.lines}`, withinHardBudget: within(lib, hard) && within(test, hard), withinTargetBudget: within(lib, target) && within(test, target) });
  };
  for (const crate of plan.crates) await rowFor(crate.name, crate.packageDir);
  await rowFor(plan.artifactCrate, plan.packageDir);
  const rootAbs = resolve(repoRoot, plan.rootFile);
  const overlay = new Map<string, string>([...plan.overlay.edits, ...plan.overlay.creates, [rootAbs, plan.tree.source]]);
  const original = resolveCrateModules(repoRoot, rootAbs, (path) => overlay.get(path) ?? (plan.overlay.deletes.has(path) ? null : existsSync(path) ? readFileSync(path, "utf8") : null));
  const carved = [resolveCrateModules(sandboxRoot, join(sandboxRoot, plan.rootFile)), ...plan.crates.map((crate) => resolveCrateModules(sandboxRoot, join(sandboxRoot, crate.packageDir, "🦀️.rs")))];
  const crateInfos = plan.crates.map((crate) => {
    const rootFile = join(sandboxRoot, crate.packageDir, "🦀️.rs");
    const sourceFiles = new Map<string, string>();
    for (const path of [...(crate.lib?.paths ?? []), ...(crate.test?.paths ?? [])]) { const absolute = join(sandboxRoot, path); if (path.endsWith(".rs") && existsSync(absolute)) sourceFiles.set(path, readFileSync(absolute, "utf8")); }
    return { name: crate.name, unitPaths: crate.attach.map((decl) => [...decl.eff]), rootFile, rootText: readFileSync(rootFile, "utf8"), sourceFiles, lib: new Set(crate.lib?.sourcePaths ?? []) };
  });
  const grammar = loadCatalogTaxonomy().packageGlueGrammar.rust!;
  const unresolvedCrates: Record<string, string[]> = {};
  const originalDeclared = new Set(Object.keys((Bun.TOML.parse(readFileSync(resolve(repoRoot, plan.packageDir, "Cargo.toml"), "utf8")) as { dependencies?: object }).dependencies ?? {}).map((key) => key.replaceAll("-", "_")));
  const inheritedUnresolvedCrates = [...new Set([...plan.tree.source.matchAll(/extern crate ([A-Za-z0-9_]+)(?: as [A-Za-z0-9_]+)?;/gu)].map((match) => match[1]!))].filter((name) => !originalDeclared.has(name)).sort();
  for (const crate of crateInfos) {
    const declared = new Set(Object.keys({ ...((Bun.TOML.parse(readFileSync(join(sandboxRoot, plan.crates.find((entry) => entry.name === crate.name)!.packageDir, "Cargo.toml"), "utf8")) as { dependencies?: object }).dependencies ?? {}) }).map((key) => key.replaceAll("-", "_")));
    const aliases = [...crate.rootText.matchAll(/extern crate ([A-Za-z0-9_]+)(?: as ([A-Za-z0-9_]+))?;/gu)].map((match) => match[2] ?? match[1]!);
    const used = new Set([...rustcUnresolvedCrates(crate.rootFile, crate.name, []).unresolved, ...rustcUnresolvedCrates(crate.rootFile, crate.name, ["--test"]).unresolved]);
    unresolvedCrates[crate.name] = [...used].filter((name) => !declared.has(name) && !aliases.includes(name) && !inheritedUnresolvedCrates.includes(name));
  }
  const report: CarveVerification = {
    census,
    moduleTreeProblems: compareModuleTrees(original, carved).problems,
    visibility: checkCrossCrateVisibility(sandboxRoot, crateInfos, crateRelativeChains),
    orphan: checkOrphanRules(crateInfos.map((crate) => ({ name: crate.name, sources: new Map([...crate.sourceFiles].filter(([path]) => crate.lib.has(path))) })), implHeader, allItems),
    glue: crateInfos.map((crate) => { const verdict = classifyPackageSource(crate.rootText, grammar); return verdict.role === "declaration" ? "" : `${crate.name}: ${verdict.role} (${verdict.evidence})`; }).filter(Boolean),
    unresolvedCrates,
    inheritedUnresolvedCrates,
    oracle: null,
    layout: null,
  };
  if (options.oracle) {
    const rustName = plan.artifactCrate;
    const unionRoot = `${sandboxRoot}-union`;
    const originalRoot = `${sandboxRoot}-original`;
    buildSandbox(repoRoot, originalRoot, directories);
    buildSandbox(repoRoot, unionRoot, directories);
    writeFileSync(join(unionRoot, plan.rootFile), plan.tree.source);
    for (const [absolute, text] of [...plan.overlay.edits, ...plan.overlay.creates]) { const to = join(unionRoot, relativePosix(repoRoot, absolute)); mkdirSync(dirname(to), { recursive: true }); writeFileSync(to, resolveFixturePlaceholders(text, repoRoot, dirname(to))); }
    for (const absolute of plan.overlay.deletes) rmSync(join(unionRoot, relativePosix(repoRoot, absolute)), { force: true });
    const prefixes = ["semio_", "serde", "protocol::", "store::", "dsl::", "pack::", "value_derive::", "framework_schema::", "tokio"];
    patchSandboxSources(originalRoot, prefixes);
    patchSandboxSources(unionRoot, prefixes);
    report.oracle = {};
    const unionDiagnostics = new Map<string, string[]>();
    for (const flags of [[], ["--test"]] as string[][]) {
      const before = runResolutionOracle(originalRoot, join(originalRoot, plan.rootFile), rustName, flags);
      const after = runResolutionOracle(unionRoot, join(unionRoot, plan.rootFile), rustName, flags);
      unionDiagnostics.set(flags.join(" "), after.diagnostics);
      const strip = (all: string[]): string[] => { const list = all.filter(external).map(placeless); return [...new Set(flags.includes("--test") ? list.map((line) => line.split("\t").slice(0, 2).join("\t")) : list)].filter((line) => !line.includes("aborting due to")); };
      const delta = diffDiagnostics(strip(before.diagnostics), dropBaseUnresolvedNames(strip(before.diagnostics), strip(after.diagnostics)));
      report.oracle[flags.join(" ") || "native"] = { original: before.diagnostics.length, candidate: after.diagnostics.length, onlyOriginal: delta.onlyOriginal.length, onlyCandidate: delta.onlyCandidate.length, sampleOnlyCandidate: delta.onlyCandidate.slice(0, 12).map((line) => { const at = after.diagnostics.findIndex((entry) => strip([entry])[0] === line); return at >= 0 ? `${line}  @ ${after.locations[at] ?? ""}` : line; }), sampleOnlyOriginal: delta.onlyOriginal.slice(0, 12) };
    }

    const combinedRoot = `${sandboxRoot}-combined`;
    buildSandbox(repoRoot, combinedRoot, directories);
    applyWrites(combinedRoot);
    const crateModules = new Map<string, string>(plan.crates.map((crate) => [crateIdent(crate.name), `__c_${crateIdent(crate.name)}`]));
    const facadeModule = `__c_${crateIdent(plan.artifactCrate)}`;
    const mounts: string[] = [];
    const owned: { module: string; paths: readonly string[]; root: string }[] = [
      ...plan.crates.map((crate) => ({ module: crateModules.get(crateIdent(crate.name))!, paths: [...(crate.lib?.sourcePaths ?? []), ...(crate.test?.sourcePaths ?? [])], root: join(crate.packageDir, "🦀️.rs") })),
      { module: facadeModule, paths: [...rootLib.sourcePaths, ...rootTest.sourcePaths], root: plan.rootFile },
    ];
    const rewritten = new Set<string>();
    for (const entry of owned) {
      mounts.push(`#[path = ${JSON.stringify(entry.root)}]\npub mod ${entry.module};`);
      for (const path of new Set([...entry.paths, entry.root])) {
        if (!path.endsWith(".rs") || rewritten.has(path)) continue;
        rewritten.add(path);
        const absolute = join(combinedRoot, path);
        if (!existsSync(absolute)) continue;
        writeFileSync(absolute, rewriteForCombined(readFileSync(absolute, "utf8"), entry.module, crateModules));
      }
    }
    writeFileSync(join(combinedRoot, "__combined.rs"), `${mounts.join("\n")}\n`);
    patchSandboxSources(combinedRoot, prefixes);
    const rewriteTree = (directory: string, module: string): void => {
      for (const entry of readdirSync(directory, { withFileTypes: true })) {
        const path = join(directory, entry.name);
        if (entry.isDirectory()) rewriteTree(path, module);
        else if (entry.name.endsWith(".rs")) writeFileSync(path, rewriteForCombined(readFileSync(path, "utf8"), module, new Map()));
      }
    };
    rewriteTree(join(originalRoot, pluginDir), "__c_union");
    writeFileSync(join(originalRoot, "__combined.rs"), `#[path = ${JSON.stringify(plan.rootFile)}]\npub mod __c_union;\n`);
    void unionDiagnostics;
    report.layout = {};
    for (const flags of [[], ["--test"]] as string[][]) {
      const baseline = runResolutionOracle(originalRoot, join(originalRoot, "__combined.rs"), "combined", flags);
      const carvedRun = runResolutionOracle(combinedRoot, join(combinedRoot, "__combined.rs"), "combined", flags);
      const strip = (list: string[], normalize: boolean): string[] => [...new Set(list.filter(external).map((line) => (normalize ? normalizeCombined(line) : line)).map((line) => line.split("\t").slice(0, 2).join("\t")))].filter((line) => !line.includes("aborting due to"));
      const delta = diffDiagnostics(strip(baseline.diagnostics, true), dropBaseUnresolvedNames(strip(baseline.diagnostics, true), strip(carvedRun.diagnostics, true)));
      report.layout[flags.join(" ") || "native"] = { union: baseline.diagnostics.length, combined: carvedRun.diagnostics.length, onlyCombined: delta.onlyCandidate.length, sampleOnlyCombined: delta.onlyCandidate.slice(0, 40).map((line) => { const at = carvedRun.diagnostics.findIndex((entry) => strip([entry], true)[0] === line); return at >= 0 ? `${line}  @ ${carvedRun.locations[at] ?? ""}` : line; }) };
    }
    if (!options.keep) rmSync(combinedRoot, { recursive: true, force: true });
  }
  return report;
}

/** 🧱️ A Cargo package of one artifact tree: the packages below `📦️packages/🦀️rust` of the artifact, minus the oracle and bridge packages, with the absolute library entry resolved against `root`. */
export type ArtifactCrate = Readonly<{ dir: string; name: string; entry: string }>;

/** 📋️ Lists the library packages of an artifact tree under `root` (a package that declares no `[lib]` and has no `src/lib.rs`, such as a binary-only tool, is not one); the same rule reads the working tree and the pre-carve archive, so both sides of the layout differential see the crates they own. */
export function artifactCrates(root: string, artifactDir: string): ArtifactCrate[] {
  const found: ArtifactCrate[] = [];
  for (const manifest of new Bun.Glob("**/📦️packages/🦀️rust/Cargo.toml").scanSync({ cwd: join(root, artifactDir) })) {
    if (manifest.startsWith("🔮️oracles") || manifest.includes("🏭️bridge/")) continue;
    const path = join(artifactDir, manifest);
    if (!/^\[package\]/mu.test(readFileSync(join(root, path), "utf8"))) continue;
    const lib = readCargoLib(join(root, path));
    if ((Bun.TOML.parse(readFileSync(join(root, path), "utf8")) as { lib?: unknown }).lib === undefined && !existsSync(lib.entry)) continue;
    found.push({ dir: dirname(path), ...lib });
  }
  return found;
}

/** 🚧️ Whether a rustc message is a tree-structure error (a syntax error, a misplaced inner attribute or a module file that does not exist): rustc stops at it before it resolves a single name, so a tree that has one cannot serve as a baseline. */
export const treeError = (message: string): boolean => /^(an inner attribute is not permitted|couldn't find file|file not found for module|circular modules|couldn't read|expected |unexpected |this file contains an unclosed|unclosed delimiter|mismatched closing delimiter)/u.test(message);

/** 🔝️ Moves the single-line inner attributes (`#![..]`) and the inner doc lines (`//!`) that sit behind an item to the head of the file, in their order: rustc refuses an inner attribute after a `use` or `extern crate`. Items are never moved, a file without a misplaced line comes back unchanged. */
export function hoistInnerAttributes(text: string): string {
  const lines = text.split("\n");
  let first = -1;
  let block = false;
  for (let index = 0; index < lines.length && first < 0; index += 1) {
    const line = lines[index]!.trim();
    if (block) { block = !line.includes("*/"); continue; }
    if (line.startsWith("/*")) { block = !line.includes("*/"); continue; }
    if (line === "" || (line.startsWith("//") && !line.startsWith("///")) || line.startsWith("#![")) continue;
    first = index;
  }
  if (first < 0) return text;
  const item = /^(?:pub(?:\([^)]*\))?\s+)?(?:mod|fn|const|static|struct|enum|trait|impl|type|macro_rules!|unsafe|async)\b|^#\[(?:path|derive|cfg_attr|macro_use)/u;
  const lifted: number[] = [];
  let stop = false;
  for (let index = first; index < lines.length; index += 1) {
    const line = lines[index]!;
    if (!stop && item.test(line)) stop = true;
    if (/^#!\[.*\]\s*$/u.test(line) && index < 400) lifted.push(index);
    else if (!stop && line.startsWith("//!")) lifted.push(index);
  }
  if (!lifted.length) return text;
  const skip = new Set(lifted);
  const rest = lines.filter((_, index) => !skip.has(index));
  return [...rest.slice(0, first), ...lifted.map((index) => lines[index]!), ...rest.slice(first)].join("\n");
}

/** 🪝️ The repository paths of the module files that `text` (the source of `file`) mounts with a flat `#[path = "<relative>"]` attribute: rustc stops at the first missing module file, so the base healing copies the whole mounted subtree at once instead of one file per round. A mount inside an inline module is resolved against the wrong directory and is skipped by the caller when the file does not exist. */
export function mountedPathFiles(file: string, text: string): string[] {
  return [...text.matchAll(/#\[path\s*=\s*"([^"]+\.rs)"\]/gu)].map((match) => posix.normalize(posix.join(posix.dirname(file), match[1]!)));
}

/** 🧩️ One crate of the synthetic layout: the module that mounts it, the repository paths of its sources and its library root. */
export type OwnedCrate = Readonly<{ module: string; paths: readonly string[]; root: string }>;

/** 🪢️ Mounts every crate as a module of `__combined.rs` below `root` and rewrites each source file for the owner that reaches it first. The pre-carve archive and the carved tree go through this one function, so both sides of the differential see their crates the same way. */
export function mountCrates(root: string, owned: readonly OwnedCrate[], crateModules: ReadonlyMap<string, string>): void {
  const mounts: string[] = [];
  const rewritten = new Set<string>();
  for (const entry of owned) {
    mounts.push(`#[path = ${JSON.stringify(entry.root)}]\npub mod ${entry.module};`);
    for (const path of new Set([...entry.paths, entry.root])) {
      if (!path.endsWith(".rs") || rewritten.has(path)) continue;
      rewritten.add(path);
      const absolute = join(root, path);
      if (existsSync(absolute)) writeFileSync(absolute, rewriteCrateAliases(rewriteForCombined(readFileSync(absolute, "utf8"), entry.module, crateModules), entry.module, crateModules));
    }
  }
  writeFileSync(join(root, "__combined.rs"), `${mounts.join("\n")}\n`);
}

/** 🧼️ Removes from `sandbox` the files of `artifactDir` that the pre-carve tree of `baseRef` does not have and that git does not ignore: the sandbox starts as a copy of the working tree, so a crate the carve added would otherwise stay and be taken for a hand-built crate of the base. Ignored files (generated sources) stay, because the base compiles against them as well. */
export function pruneNonBaseFiles(repoRoot: string, sandbox: string, artifactDir: string, baseRef: string): string[] {
  const archived = new Set(execFileSync("git", ["-c", "core.quotepath=false", "-c", "core.fsmonitor=false", "ls-tree", "-r", "--name-only", "-z", baseRef, "--", artifactDir], { cwd: repoRoot, encoding: "utf8", maxBuffer: 1 << 28 }).split("\0").filter(Boolean));
  const extra: string[] = [];
  const walk = (directory: string): void => {
    for (const entry of readdirSync(join(sandbox, directory), { withFileTypes: true })) {
      const path = `${directory}/${entry.name}`;
      if (entry.isDirectory()) walk(path);
      else if (!archived.has(path)) extra.push(path);
    }
  };
  if (existsSync(join(sandbox, artifactDir))) walk(artifactDir);
  if (!extra.length) return [];
  const ignored = new Set(spawnSync("git", ["-c", "core.quotepath=false", "-c", "core.fsmonitor=false", "check-ignore", "-z", "--stdin"], { cwd: repoRoot, input: `${extra.join("\0")}\0`, encoding: "utf8", maxBuffer: 1 << 28 }).stdout.split("\0").filter(Boolean));
  const removed = extra.filter((path) => !ignored.has(path));
  for (const path of removed) rmSync(join(sandbox, path), { force: true });
  return removed;
}

/** 🧾️ Result of the layout differential of an applied carve: diagnostics only the mounted crates have, per cfg. */
export type AppliedLayoutReport = { crates: number; baseCrates: number; siblings: string[]; healedBase: string[]; layout: Record<string, { base: number; combined: number; onlyCombined: number; sampleOnlyCombined: string[] }> };

/** 🧾️ Gate for an applied carve: mounts the crates of the working tree as modules of one synthetic crate and compares the rustc resolution diagnostics (lib and test) with the pre-carve tree of `baseRef`. Zero `onlyCombined` means the carve introduced no unresolved name. */
export async function verifyAppliedLayout(repoRoot: string, artifactDir: string, sandboxRoot: string, baseRef = "HEAD", keep = false, sampleLimit = 12): Promise<AppliedLayoutReport> {
  const baseDirectory = baseRef.startsWith("dir:") ? resolve(baseRef.slice("dir:".length)) : null;
  const pluginDir = artifactDir.split("/").length >= 3 ? artifactDir.split("/").slice(0, 3).join("/") : artifactDir;
  const named = artifactCrates(repoRoot, artifactDir);
  const facadeDirectory = `${artifactDir}/📦️packages/🦀️rust`;
  const facadeName = named.find((entry) => entry.dir === facadeDirectory)?.name;
  if (!facadeName) throw new Error(`verifyAppliedLayout: ${artifactDir} has no artifact package`);
  const crates = named.filter((entry) => entry.name === facadeName || entry.name.startsWith(`${facadeName}-`));
  const facade = crates.find((crate) => crate.dir === facadeDirectory)!;
  const carved = crates.filter((crate) => crate !== facade);
  const outside = new Set<string>();
  const measured = new Map<string, string[]>();
  for (const crate of crates) {
    const lib = await measureRustCrate(repoRoot, crate.entry, false, new Map(), join(repoRoot, crate.dir));
    const test = await measureRustCrate(repoRoot, crate.entry, true, new Map(), join(repoRoot, crate.dir));
    measured.set(crate.name, [...lib.sourcePaths, ...test.sourcePaths]);
    for (const path of [...lib.paths, ...test.paths]) if (!path.startsWith(`${pluginDir}/`)) outside.add(path);
  }
  const directories = [pluginDir, ...new Set([...[...outside].map((path) => dirname(path)).filter((directory) => existsSync(join(repoRoot, directory))), ...mountedOutsideDirectories(repoRoot, pluginDir)])];
  const combined = `${sandboxRoot}-combined`;
  const base = `${sandboxRoot}-base`;
  const prefixes = ["semio_", "serde", "protocol::", "store::", "dsl::", "pack::", "value_derive::", "framework_schema::", "tokio"];
  const relativeEntry = (crate: { entry: string }): string => relative(repoRoot, crate.entry).replaceAll("\\", "/");
  const closureFiles = (root: string, entry: string): string[] => [entry, ...resolveCrateModules(root, join(root, entry), (file) => (existsSync(file) ? readFileSync(file, "utf8") : "")).flatMap((module) => (module.file !== null && module.file !== "unresolved" ? [module.file] : []))];
  const baseCarved = (): ArtifactCrate[] => artifactCrates(base, artifactDir).filter((crate) => crate.dir !== facadeDirectory && crate.name.startsWith(`${facadeName}-`));
  const baseEntry = (crate: ArtifactCrate): string => relative(base, crate.entry).replaceAll("\\", "/");
  const prepareBase = async (healed: ReadonlyMap<string, string>): Promise<Set<string>> => {
    rmSync(base, { recursive: true, force: true });
    buildSandbox(repoRoot, base, directories);
    mkdirSync(base, { recursive: true });
    if (baseDirectory) { rmSync(join(base, artifactDir), { recursive: true, force: true }); cpSync(join(baseDirectory, artifactDir), join(base, artifactDir), { recursive: true }); }
    else execFileSync("sh", ["-c", `cd '${repoRoot}' && git -c core.quotepath=false archive ${baseRef} '${pluginDir}' | tar -x -C '${base}'`]);
    if (!baseDirectory) pruneNonBaseFiles(repoRoot, base, artifactDir, baseRef);
    for (const directory of mountedOutsideDirectories(repoRoot, pluginDir, base)) {
      if (existsSync(join(base, directory))) continue;
      mkdirSync(dirname(join(base, directory)), { recursive: true });
      cpSync(resolve(repoRoot, directory), join(base, directory), { recursive: true, filter: (path) => !/\/(node_modules|target|dist)(\/|$)/u.test(path) });
    }
    for (const [path, text] of healed) { mkdirSync(dirname(join(base, path)), { recursive: true }); writeFileSync(join(base, path), text); }
    const carvedAtBase = baseCarved();
    const roots: ArtifactCrate[] = [...carvedAtBase, { dir: facadeDirectory, name: facade.name, entry: join(base, relativeEntry(facade)) }];
    const files = new Set(roots.flatMap((crate) => closureFiles(base, baseEntry(crate))));
    const modules = new Map<string, string>(carvedAtBase.map((crate) => [crateIdent(crate.name), `__c_${crateIdent(crate.name)}`] as const));
    const baseOwned: OwnedCrate[] = [];
    for (const crate of roots) {
      let paths: string[];
      try {
        const lib = await measureRustCrate(base, crate.entry, false, new Map(), join(base, crate.dir));
        const test = await measureRustCrate(base, crate.entry, true, new Map(), join(base, crate.dir));
        paths = [...lib.sourcePaths, ...test.sourcePaths];
      } catch { paths = closureFiles(base, baseEntry(crate)); }
      baseOwned.push({ module: `__c_${crateIdent(crate.name)}`, paths, root: baseEntry(crate) });
    }
    mountCrates(base, baseOwned, modules);
    patchSandboxSources(base, prefixes);
    return files;
  };
  const baseFiles = await prepareBase(new Map());
  const workspaceDir = relativePosix(repoRoot, resolve(repoRoot, facadeDirectory, (Bun.TOML.parse(readFileSync(join(repoRoot, facadeDirectory, "Cargo.toml"), "utf8")) as { package?: { workspace?: string } }).package?.workspace ?? "../.."));
  const siblings = siblingPathCrates(repoRoot, pluginDir, artifactDir, crates.map((crate) => `${crate.dir}/Cargo.toml`), workspaceDir).filter((sibling) => { const roots = [dirname(relative(repoRoot, sibling.entry)), dirname(dirname(sibling.dir))].map((root) => `${root.replaceAll("\\", "/")}/`); return [...baseFiles].some((file) => roots.some((root) => file.startsWith(root))); }).map((sibling) => ({ ...sibling, files: closureFiles(repoRoot, relative(repoRoot, sibling.entry).replaceAll("\\", "/")) })).filter((sibling) => sibling.files.some((file) => baseFiles.has(file)));
  buildSandbox(repoRoot, combined, [...new Set([...directories, ...siblings.flatMap((sibling) => sibling.files.filter((file) => !file.startsWith(`${pluginDir}/`)).map((file) => dirname(file)))])]);
  const crateModules = new Map<string, string>([...carved.map((crate) => [crateIdent(crate.name), `__c_${crateIdent(crate.name)}`] as const), ...siblings.flatMap((sibling) => sibling.idents.map((ident) => [ident, `__c_${crateIdent(sibling.name)}`] as const))]);
  const owned = [...carved.map((crate) => ({ module: crateModules.get(crateIdent(crate.name))!, paths: measured.get(crate.name)!, root: relativeEntry(crate) })), { module: `__c_${crateIdent(facade.name)}`, paths: measured.get(facade.name)!, root: relativeEntry(facade) }, ...siblings.map((sibling) => ({ module: `__c_${crateIdent(sibling.name)}`, paths: sibling.files, root: relative(repoRoot, sibling.entry).replaceAll("\\", "/") }))];
  mountCrates(combined, owned, crateModules);
  patchSandboxSources(combined, prefixes);
  const declaresFeature = (feature: string): boolean => [...siblings.map((sibling) => sibling.dir), ...crates.map((crate) => crate.dir)].some((dir) => Object.hasOwn((Bun.TOML.parse(readFileSync(join(repoRoot, dir, "Cargo.toml"), "utf8")) as { features?: Record<string, unknown> }).features ?? {}, feature));
  const testingFlags = ["testing", "testkit"].filter(declaresFeature).flatMap((feature) => ["--cfg", `feature="${feature}"`]);
  const flagSets = [[], ["--test", ...testingFlags]] as string[][];
  const archivedAt = (file: string): boolean => { if (baseDirectory) return existsSync(join(baseDirectory, file)); try { execFileSync("git", ["-c", "core.quotepath=false", "cat-file", "-e", `${baseRef}:${file}`], { cwd: repoRoot, stdio: "pipe" }); return true; } catch { return false; } };
  const healed = new Map<string, string>();
  for (let round = 0; round < 64; round += 1) {
    const fresh = new Map<string, string>();
    for (const flags of flagSets) for (const diagnostic of runResolutionOracle(base, join(base, "__combined.rs"), "combined", flags).diagnostics) {
      const [, message = "", file = ""] = diagnostic.split("\t");
      const missing = /^couldn't find file `([^`]+)`/u.exec(message)?.[1]?.replace(`${base}/`, "").replace("<sandbox>/", "");
      if (missing && !healed.has(missing) && existsSync(join(repoRoot, missing)) && !existsSync(join(base, missing)) && !archivedAt(missing)) fresh.set(missing, readFileSync(join(repoRoot, missing), "utf8"));
      if (!treeError(message) || !file || healed.has(file) || !existsSync(join(repoRoot, file))) continue;
      const current = readFileSync(join(repoRoot, file), "utf8");
      let archived: string | null = null;
      if (baseDirectory) archived = existsSync(join(baseDirectory, file)) ? readFileSync(join(baseDirectory, file), "utf8") : null;
      else try { archived = execFileSync("git", ["-c", "core.quotepath=false", "show", `${baseRef}:${file}`], { cwd: repoRoot, encoding: "utf8", maxBuffer: 1 << 28 }); } catch { archived = null; }
      if (current === archived) continue;
      if (archived !== null && /^(an inner attribute is not permitted|expected outer doc comment)/u.test(message)) {
        if (hoistInnerAttributes(current) === current && hoistInnerAttributes(archived) !== archived) fresh.set(file, hoistInnerAttributes(archived));
        continue;
      }
      fresh.set(file, current);
    }
    if (!fresh.size) break;
    for (const pending = [...fresh.keys()]; pending.length;) {
      const file = pending.pop()!;
      for (const mounted of mountedPathFiles(file, fresh.get(file)!)) {
        if (fresh.has(mounted) || healed.has(mounted) || !existsSync(join(repoRoot, mounted)) || existsSync(join(base, mounted)) || archivedAt(mounted)) continue;
        fresh.set(mounted, readFileSync(join(repoRoot, mounted), "utf8"));
        pending.push(mounted);
      }
    }
    for (const [file, text] of fresh) healed.set(file, text);
    await prepareBase(healed);
  }
  const report: AppliedLayoutReport = { crates: carved.length, baseCrates: baseCarved().length, siblings: siblings.map((sibling) => sibling.name), healedBase: [...healed.keys()], layout: {} };
  for (const flags of flagSets) {
    const baseline = runResolutionOracle(base, join(base, "__combined.rs"), "combined", flags);
    const carvedRun = runResolutionOracle(combined, join(combined, "__combined.rs"), "combined", flags);
    const strip = (list: string[]): string[] => [...new Set(list.filter(external).map(normalizeCombined).map((line) => line.split("\t").slice(0, 2).join("\t")))].filter((line) => !line.includes("aborting due to"));
    const baselineLines = strip(baseline.diagnostics);
    const delta = diffDiagnostics(baselineLines, dropBaseUnresolvedNames(baselineLines, strip(carvedRun.diagnostics)));
    report.layout[flags.includes("--test") ? "--test" : "native"] = { base: baseline.diagnostics.length, combined: carvedRun.diagnostics.length, onlyCombined: delta.onlyCandidate.length, sampleOnlyCombined: delta.onlyCandidate.slice(0, sampleLimit).map((line) => { const at = carvedRun.diagnostics.findIndex((entry) => strip([entry])[0] === line); return at >= 0 ? `${line}  @ ${carvedRun.locations[at] ?? ""}` : line; }) };
  }
  if (!keep) {
    rmSync(combined, { recursive: true, force: true });
    rmSync(base, { recursive: true, force: true });
  }
  return report;
}
