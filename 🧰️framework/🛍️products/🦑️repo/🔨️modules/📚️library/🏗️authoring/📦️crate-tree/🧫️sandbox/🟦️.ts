import { execFileSync } from "node:child_process";
import { existsSync, mkdirSync, readdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";
import { rustTokens } from "../../../../../../../🔨️modules/📚️compiler/📖️syntax/🦀️rust/🟦️.ts";
import { carveWrites } from "../🟦️.ts";
import { allItems, crateIdent, crateRelativeChains, implHeader, measureRustCrate, readCargoLib, type ArtifactCratePlan } from "../🗺️plan/🟦️.ts";
import { classifyPackageSource, loadCatalogTaxonomy } from "../../../🔍️discovery/🟦️.ts";
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
  oracle: Record<string, { original: number; candidate: number; onlyOriginal: number; onlyCandidate: number; sampleOnlyCandidate: string[]; sampleOnlyOriginal: string[] }> | null;
  layout: Record<string, { union: number; combined: number; onlyCombined: number; sampleOnlyCombined: string[] }> | null;
};

export function mountedOutsideDirectories(repoRoot: string, pluginDir: string): Set<string> {
  const found = new Set<string>();
  const walk = (directory: string): void => {
    for (const entry of readdirSync(directory, { withFileTypes: true })) {
      const path = join(directory, entry.name);
      if (entry.isDirectory()) { if (!/^(node_modules|target|dist)$/u.test(entry.name)) walk(path); continue; }
      if (!entry.name.endsWith(".rs")) continue;
      const sourceText = readFileSync(path, "utf8");
      for (const match of sourceText.matchAll(/include(?:_str|_bytes)?!\(\s*"([^"]+)"\s*\)/gu)) {
        const target = resolve(directory, match[1]!);
        const relative = relativePosix(repoRoot, dirname(target));
        if (!relative.startsWith("..") && !`${relative}/`.startsWith(`${pluginDir}/`) && existsSync(dirname(target))) found.add(relative);
      }
      for (const match of sourceText.matchAll(/#\[path\s*=\s*"([^"]+)"\s*\]/gu)) {
        const target = resolve(directory, match[1]!);
        const relative = relativePosix(repoRoot, dirname(target));
        if (!relative.startsWith("..") && !`${relative}/`.startsWith(`${pluginDir}/`) && existsSync(dirname(target))) found.add(relative);
      }
    }
  };
  walk(resolve(repoRoot, pluginDir));
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

/** 🔗️ Whether a diagnostic is about anything but an external crate that the sandbox does not link. */
export const external = (line: string): boolean => !line.includes("unlinked crate");

export const normalizeCombined = (line: string): string => line.replace(/crate::__c_[a-z0-9_]+/gu, "crate").replace(/__c_[a-z0-9_]+(::)?/gu, "");

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
  for (const crate of crateInfos) {
    const declared = new Set(Object.keys({ ...((Bun.TOML.parse(readFileSync(join(sandboxRoot, plan.crates.find((entry) => entry.name === crate.name)!.packageDir, "Cargo.toml"), "utf8")) as { dependencies?: object }).dependencies ?? {}) }).map((key) => key.replaceAll("-", "_")));
    const aliases = [...crate.rootText.matchAll(/extern crate ([A-Za-z0-9_]+)(?: as ([A-Za-z0-9_]+))?;/gu)].map((match) => match[2] ?? match[1]!);
    const used = new Set([...rustcUnresolvedCrates(crate.rootFile, crate.name, []).unresolved, ...rustcUnresolvedCrates(crate.rootFile, crate.name, ["--test"]).unresolved]);
    unresolvedCrates[crate.name] = [...used].filter((name) => !declared.has(name) && !aliases.includes(name));
  }
  const report: CarveVerification = {
    census,
    moduleTreeProblems: compareModuleTrees(original, carved).problems,
    visibility: checkCrossCrateVisibility(sandboxRoot, crateInfos, crateRelativeChains),
    orphan: checkOrphanRules(crateInfos.map((crate) => ({ name: crate.name, sources: new Map([...crate.sourceFiles].filter(([path]) => crate.lib.has(path))) })), implHeader, allItems),
    glue: crateInfos.map((crate) => { const verdict = classifyPackageSource(crate.rootText, grammar); return verdict.role === "declaration" ? "" : `${crate.name}: ${verdict.role} (${verdict.evidence})`; }).filter(Boolean),
    unresolvedCrates,
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
      const strip = (all: string[]): string[] => { const list = all.filter(external); return [...new Set(flags.includes("--test") ? list.map((line) => line.split("\t").slice(0, 2).join("\t")) : list)].filter((line) => !line.includes("aborting due to")); };
      const delta = diffDiagnostics(strip(before.diagnostics), strip(after.diagnostics));
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
      const delta = diffDiagnostics(strip(baseline.diagnostics, true), strip(carvedRun.diagnostics, true));
      report.layout[flags.join(" ") || "native"] = { union: baseline.diagnostics.length, combined: carvedRun.diagnostics.length, onlyCombined: delta.onlyCandidate.length, sampleOnlyCombined: delta.onlyCandidate.slice(0, 40).map((line) => { const at = carvedRun.diagnostics.findIndex((entry) => strip([entry], true)[0] === line); return at >= 0 ? `${line}  @ ${carvedRun.locations[at] ?? ""}` : line; }) };
    }
    if (!options.keep) rmSync(combinedRoot, { recursive: true, force: true });
  }
  return report;
}

/** 🧾️ Result of the layout differential of an applied carve: diagnostics only the mounted crates have, per cfg. */
export type AppliedLayoutReport = { crates: number; layout: Record<string, { base: number; combined: number; onlyCombined: number; sampleOnlyCombined: string[] }> };

/** 🧾️ Gate for an applied carve: mounts the crates of the working tree as modules of one synthetic crate and compares the rustc resolution diagnostics (lib and test) with the pre-carve tree of `baseRef`. Zero `onlyCombined` means the carve introduced no unresolved name. */
export async function verifyAppliedLayout(repoRoot: string, artifactDir: string, sandboxRoot: string, baseRef = "HEAD", keep = false): Promise<AppliedLayoutReport> {
  const pluginDir = artifactDir.split("/").length >= 3 ? artifactDir.split("/").slice(0, 3).join("/") : artifactDir;
  const named = [...new Bun.Glob("**/📦️packages/🦀️rust/Cargo.toml").scanSync({ cwd: join(repoRoot, artifactDir) })]
    .filter((manifest) => !manifest.startsWith("🔮️oracles") && !manifest.includes("🏭️bridge/"))
    .map((manifest) => { const path = join(artifactDir, manifest); return { path, name: (Bun.TOML.parse(readFileSync(join(repoRoot, path), "utf8")) as { package: { name: string } }).package.name }; });
  const facadeDirectory = `${artifactDir}/📦️packages/🦀️rust`;
  const facadeName = named.find((entry) => dirname(entry.path) === facadeDirectory)?.name;
  if (!facadeName) throw new Error(`verifyAppliedLayout: ${artifactDir} has no artifact package`);
  const crates = named.filter((entry) => entry.name === facadeName || entry.name.startsWith(`${facadeName}-`)).map((entry) => ({ dir: dirname(entry.path), name: entry.name, entry: readCargoLib(join(repoRoot, entry.path)).entry }));
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
  buildSandbox(repoRoot, combined, directories);
  const crateModules = new Map(carved.map((crate) => [crateIdent(crate.name), `__c_${crateIdent(crate.name)}`]));
  const relativeEntry = (crate: { entry: string }): string => relative(repoRoot, crate.entry).replaceAll("\\", "/");
  const owned = [...carved.map((crate) => ({ module: crateModules.get(crateIdent(crate.name))!, paths: measured.get(crate.name)!, root: relativeEntry(crate) })), { module: `__c_${crateIdent(facade.name)}`, paths: measured.get(facade.name)!, root: relativeEntry(facade) }];
  const mounts: string[] = [];
  const rewritten = new Set<string>();
  for (const entry of owned) {
    mounts.push(`#[path = ${JSON.stringify(entry.root)}]\npub mod ${entry.module};`);
    for (const path of new Set([...entry.paths, entry.root])) {
      if (!path.endsWith(".rs") || rewritten.has(path)) continue;
      rewritten.add(path);
      const absolute = join(combined, path);
      if (existsSync(absolute)) writeFileSync(absolute, rewriteForCombined(readFileSync(absolute, "utf8"), entry.module, crateModules));
    }
  }
  writeFileSync(join(combined, "__combined.rs"), `${mounts.join("\n")}\n`);
  const prefixes = ["semio_", "serde", "protocol::", "store::", "dsl::", "pack::", "value_derive::", "framework_schema::", "tokio"];
  patchSandboxSources(combined, prefixes);
  buildSandbox(repoRoot, base, directories.filter((directory) => directory !== pluginDir));
  mkdirSync(base, { recursive: true });
  execFileSync("sh", ["-c", `cd '${repoRoot}' && git -c core.quotepath=false archive ${baseRef} '${pluginDir}' | tar -x -C '${base}'`]);
  const rewriteTree = (directory: string): void => {
    for (const entry of readdirSync(directory, { withFileTypes: true })) {
      const path = join(directory, entry.name);
      if (entry.isDirectory()) rewriteTree(path);
      else if (entry.name.endsWith(".rs")) writeFileSync(path, rewriteForCombined(readFileSync(path, "utf8"), "__c_union", new Map()));
    }
  };
  rewriteTree(join(base, pluginDir));
  patchSandboxSources(base, prefixes);
  writeFileSync(join(base, "__combined.rs"), `#[path = ${JSON.stringify(relativeEntry(facade))}]\npub mod __c_union;\n`);
  const report: AppliedLayoutReport = { crates: carved.length, layout: {} };
  for (const flags of [[], ["--test"]] as string[][]) {
    const baseline = runResolutionOracle(base, join(base, "__combined.rs"), "combined", flags);
    const carvedRun = runResolutionOracle(combined, join(combined, "__combined.rs"), "combined", flags);
    const strip = (list: string[]): string[] => [...new Set(list.filter(external).map(normalizeCombined).map((line) => line.split("\t").slice(0, 2).join("\t")))].filter((line) => !line.includes("aborting due to"));
    const delta = diffDiagnostics(strip(baseline.diagnostics), strip(carvedRun.diagnostics));
    report.layout[flags.join(" ") || "native"] = { base: baseline.diagnostics.length, combined: carvedRun.diagnostics.length, onlyCombined: delta.onlyCandidate.length, sampleOnlyCombined: delta.onlyCandidate.slice(0, 12).map((line) => { const at = carvedRun.diagnostics.findIndex((entry) => strip([entry])[0] === line); return at >= 0 ? `${line}  @ ${carvedRun.locations[at] ?? ""}` : line; }) };
  }
  if (!keep) {
    rmSync(combined, { recursive: true, force: true });
    rmSync(base, { recursive: true, force: true });
  }
  return report;
}
