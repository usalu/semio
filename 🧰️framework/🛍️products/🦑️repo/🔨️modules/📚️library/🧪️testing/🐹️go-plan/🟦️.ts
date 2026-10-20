import { existsSync, lstatSync, mkdirSync, mkdtempSync, readdirSync, readFileSync, realpathSync, rmSync, writeFileSync } from "node:fs";
import { basename, dirname, isAbsolute, join, relative, resolve, sep } from "node:path";
import { createHash } from "node:crypto";
import { tmpdir } from "node:os";
import { canonicalGoLanguageDirectoryV1, readCanonicalGoLayoutV1 as canonicalGoLayout, type CanonicalGoLayout } from "../../🔍️discovery/🐹️go-layout/🟦️.ts";
import { runRepositoryTestCommand } from "../🎛️repository-command/🟦️.ts";
import { type CompilerAdmissionControl, requestedCompilerAdmissionControlV1 } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { buildBudgetMs } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/⏱️budget/🟦️.ts";
import { readBuildMode } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎚️mode/🟦️.ts";
import { getWorkspaceRoot } from "../../🗂️workspaces/🟦️.ts";
import { repoToolCacheEnv } from "../../🏃️process/🌿️environment/🟦️.ts";

export type CanonicalGoPlan = Readonly<{
  packages: readonly string[];
  replacements: Readonly<Record<string, string>>;
  sourceReplacements: Readonly<Record<string, string>>;
  testReplacements: Readonly<Record<string, string>>;
}>;

const GO_LANGUAGE_DIRECTORY = canonicalGoLanguageDirectoryV1();
const GO_HOST_ADAPTER_PACKAGE = "adapter";

/** 🧭️Reads the package clause that opens one Go source, skipping leading whitespace, line comments and block comments. */
function goPackageClause(source: string): string | null {
  let rest = source.replace(/^\uFEFF/u, "");
  for (;;) {
    rest = rest.replace(/^\s+/u, "");
    if (rest.startsWith("//")) {
      const end = rest.indexOf("\n");
      rest = end < 0 ? "" : rest.slice(end + 1);
    } else if (rest.startsWith("/*")) {
      const end = rest.indexOf("*/");
      rest = end < 0 ? "" : rest.slice(end + 2);
    } else break;
  }
  return /^package\s+([\p{L}_][\p{L}\p{N}_]*)/u.exec(rest)?.[1] ?? null;
}

/** 🧪️Resolves the language-neutral owner whose in-package cases target the Go module rooted at `<owner>/<packages>/<language>`. */
function packageModuleOwner(root: string, layout: CanonicalGoLayout): string | null {
  const packages = dirname(root);
  return basename(root) === layout.languageDirectory && basename(packages) === layout.packagesDirectory ? dirname(packages) : null;
}

/**
 * 🧩️Projects one Go module into virtual compiler inputs: registered member directories become sources of their parent package and `🧪️tests/<case>` leaves become its tests.
 * A module rooted at `<owner>/📦️packages/🐹️go` additionally adopts the owner's `🧪️tests/<case>/🐹️.go` leaves that declare its own package (or its `_test` twin), so white-box cases live at the language-neutral owner.
 * @see https://pkg.go.dev/cmd/go#hdr-Compile_packages_and_dependencies
 */
function canonicalGoModulePlan(moduleRoot: string, layout: CanonicalGoLayout): CanonicalGoPlan {
  const root = realpathSync(moduleRoot);
  const sourceReplacements: Record<string, string> = {};
  const testReplacements: Record<string, string> = {};
  const packages = new Set<string>();
  const map = (owner: string, source: string, test: boolean): void => {
    const ownerRelative = relative(root, owner).split(sep).join("/");
    if (ownerRelative === ".." || ownerRelative.startsWith("..")) throw new Error(`Go input owner escapes its module: ${owner}`);
    const id = createHash("sha256").update(relative(root, source).split(sep).join("/")).digest("hex").slice(0, 16);
    const virtual = join(owner, `zz_semio_${id}${test ? "_test" : ""}.go`);
    if (existsSync(virtual)) throw new Error(`Go input overlay collides with an authored file: ${virtual}`);
    (test ? testReplacements : sourceReplacements)[virtual] = source;
    packages.add(ownerRelative ? `./${ownerRelative}` : ".");
  };
  const walk = (directory: string): void => {
    if (directory !== root && existsSync(join(directory, "go.mod"))) return;
    const entries = readdirSync(directory, { withFileTypes: true }).sort((left, right) => left.name.localeCompare(right.name));
    if (entries.some((entry) => entry.isFile() && !entry.isSymbolicLink() && entry.name.endsWith(".go"))) {
      const ownerRelative = relative(root, directory).split(sep).join("/");
      packages.add(ownerRelative ? `./${ownerRelative}` : ".");
    }
    for (const entry of entries) {
      if (!entry.isDirectory() || entry.isSymbolicLink()) continue;
      const path = join(directory, entry.name);
      if (existsSync(join(path, "go.mod"))) continue;
      if (layout.sourceDirectoryNames.includes(entry.name)) {
        const source = join(path, layout.implementationFilename);
        if (existsSync(source)) {
          const info = lstatSync(source);
          if (info.isFile() && !info.isSymbolicLink()) map(directory, source, false);
        }
        continue;
      }
      if (entry.name === layout.testsDirectory) {
        for (const testCase of readdirSync(path, { withFileTypes: true }).sort((left, right) => left.name.localeCompare(right.name))) {
          if (!testCase.isDirectory() || testCase.isSymbolicLink()) continue;
          const source = join(path, testCase.name, layout.implementationFilename);
          if (!existsSync(source)) continue;
          const info = lstatSync(source);
          if (!info.isFile() || info.isSymbolicLink()) continue;
          if (goPackageClause(readFileSync(source, "utf8")) === GO_HOST_ADAPTER_PACKAGE) continue;
          map(directory, source, true);
        }
        continue;
      }
      if (entry.name === ".git" || entry.name === "node_modules" || entry.name === "target" || entry.name === "vendor" || layout.opaqueDirectoryNames.includes(entry.name)) continue;
      walk(path);
    }
  };
  walk(root);
  const owner = packageModuleOwner(root, layout), subjectEntry = join(root, layout.implementationFilename);
  if (owner && existsSync(subjectEntry) && lstatSync(subjectEntry).isFile()) {
    const subject = goPackageClause(readFileSync(subjectEntry, "utf8")), cases = join(owner, layout.testsDirectory);
    if (subject && existsSync(cases) && lstatSync(cases).isDirectory() && !lstatSync(cases).isSymbolicLink()) {
      for (const testCase of readdirSync(cases, { withFileTypes: true }).sort((left, right) => left.name.localeCompare(right.name))) {
        if (!testCase.isDirectory() || testCase.isSymbolicLink()) continue;
        const source = join(cases, testCase.name, layout.implementationFilename);
        if (!existsSync(source) || !lstatSync(source).isFile() || lstatSync(source).isSymbolicLink()) continue;
        const clause = goPackageClause(readFileSync(source, "utf8"));
        if (clause === subject || clause === `${subject}_test`) map(root, source, true);
      }
    }
  }
  const sorted = (values: Record<string, string>): Readonly<Record<string, string>> => Object.fromEntries(Object.entries(values).sort(([left], [right]) => left.localeCompare(right)));
  const sources = sorted(sourceReplacements), tests = sorted(testReplacements);
  return { packages: [...packages].sort(), replacements: sorted({ ...sources, ...tests }), sourceReplacements: sources, testReplacements: tests };
}

/** 🔗️Resolves filesystem-backed Go module replacements without admitting registry dependencies. */
function localGoReplacementRoots(moduleRoot: string): readonly string[] {
  const goMod = join(moduleRoot, "go.mod");
  if (!existsSync(goMod)) return [];
  const replacements: string[] = [];
  let block = false;
  for (const raw of readFileSync(goMod, "utf8").split(/\r?\n/u)) {
    const line = raw.replace(/\/\/.*$/u, "").trim();
    if (!line) continue;
    if (/^replace\s*\($/u.test(line)) {
      block = true;
      continue;
    }
    if (block && line === ")") {
      block = false;
      continue;
    }
    const directive = block ? line : line.startsWith("replace ") ? line.slice("replace ".length).trim() : "";
    if (!directive) continue;
    const parts = directive.split(/\s+/u);
    const arrow = parts.indexOf("=>");
    const replacement = arrow < 0 ? undefined : parts[arrow + 1];
    if (!replacement || (!isAbsolute(replacement) && !replacement.startsWith("."))) continue;
    const candidate = resolve(moduleRoot, replacement);
    if (!existsSync(join(candidate, "go.mod"))) continue;
    replacements.push(realpathSync(candidate));
  }
  return [...new Set(replacements)].sort();
}

/** 🧭️Rejects Go compiler-input layouts whose names are not single path segments or overlap. */
function requireValidCanonicalGoLayout(layout: CanonicalGoLayout): void {
  const validSegment = (name: string): boolean => !!name && !name.includes("/") && !name.includes("\\");
  const validNames = (names: readonly string[]): boolean => names.length > 0 && new Set(names).size === names.length && names.every((name) => !!name && !name.includes("/") && !name.includes("\\"));
  if (!layout.testsDirectory || layout.testsDirectory.includes("/") || layout.testsDirectory.includes("\\") || !layout.implementationFilename.endsWith(".go") || layout.implementationFilename.includes("/") || layout.implementationFilename.includes("\\") || !validSegment(layout.packagesDirectory) || !validSegment(layout.languageDirectory) || !validNames(layout.opaqueDirectoryNames) || !validNames(layout.sourceDirectoryNames) || layout.opaqueDirectoryNames.includes(layout.testsDirectory) || layout.sourceDirectoryNames.includes(layout.testsDirectory) || layout.sourceDirectoryNames.some((name) => layout.opaqueDirectoryNames.includes(name))) throw new Error("Invalid canonical Go compiler-input layout.");
}

/** 🐹️Maps domain-owned Go sources and cases, including local module replacements, to virtual compiler inputs. */
export function canonicalGoPlan(moduleRoot: string, layout: CanonicalGoLayout = canonicalGoLayout()): CanonicalGoPlan {
  requireValidCanonicalGoLayout(layout);
  const root = realpathSync(moduleRoot);
  const primary = canonicalGoModulePlan(root, layout);
  const sourceReplacements: Record<string, string> = { ...primary.sourceReplacements };
  const visited = new Set<string>([root]);
  const addDependencies = (owner: string): void => {
    for (const dependency of localGoReplacementRoots(owner)) {
      if (visited.has(dependency)) continue;
      visited.add(dependency);
      const plan = canonicalGoModulePlan(dependency, layout);
      for (const [virtual, source] of Object.entries(plan.sourceReplacements)) {
        const existing = sourceReplacements[virtual];
        if (existing && existing !== source) throw new Error(`Go input overlay collision: ${virtual}`);
        sourceReplacements[virtual] = source;
      }
      addDependencies(dependency);
    }
  };
  addDependencies(root);
  const sorted = (values: Record<string, string>): Readonly<Record<string, string>> => Object.fromEntries(Object.entries(values).sort(([left], [right]) => left.localeCompare(right)));
  const sources = sorted(sourceReplacements);
  return { packages: primary.packages, replacements: sorted({ ...sources, ...primary.testReplacements }), sourceReplacements: sources, testReplacements: primary.testReplacements };
}

/** 🧱️Merges the canonical source overlays of the given Go modules and their local replacements into one compiler overlay file for hosts that compile workspace modules directly. */
export function writeCanonicalGoWorkspaceOverlay(file: string, moduleRoots: readonly string[], layout: CanonicalGoLayout = canonicalGoLayout()): string {
  requireValidCanonicalGoLayout(layout);
  const replace: Record<string, string> = {};
  const visited = new Set<string>();
  const add = (moduleRoot: string): void => {
    if (!existsSync(join(moduleRoot, "go.mod"))) return;
    const root = realpathSync(moduleRoot);
    if (visited.has(root)) return;
    visited.add(root);
    for (const [virtual, source] of Object.entries(canonicalGoModulePlan(root, layout).sourceReplacements)) {
      const existing = replace[virtual];
      if (existing && existing !== source) throw new Error(`Go input overlay collision: ${virtual}`);
      replace[virtual] = source;
    }
    for (const dependency of localGoReplacementRoots(root)) add(dependency);
  };
  for (const root of moduleRoots) add(root);
  mkdirSync(dirname(file), { recursive: true });
  writeFileSync(file, `${JSON.stringify({ Replace: Object.fromEntries(Object.entries(replace).sort(([left], [right]) => left.localeCompare(right))) }, null, 2)}\n`);
  return file;
}

/** 🧪️Runs canonical Go cases through the standard toolchain without authored legacy filenames. */
export async function runCanonicalGoTests(moduleRoot: string, args: string[], opts: { env?: NodeJS.ProcessEnv; budgetMs?: number; packages?: readonly string[]; signal?: AbortSignal } = {}): Promise<void> {
  const plan = canonicalGoPlan(moduleRoot);
  const packages = opts.packages ?? plan.packages;
  if (packages.length === 0) throw new Error(`No canonical Go inputs found below ${realpathSync(moduleRoot)}`);
  for (const selected of packages) if (!plan.packages.includes(selected)) throw new Error(`Go compiler package is outside the canonical plan: ${selected}`);
  const inheritedOwner = opts.env?.SEMIO_GO_OVERLAY_OWNER;
  const owner = inheritedOwner ? realpathSync(inheritedOwner) : mkdtempSync(join(tmpdir(), "semio-go-tests-"));
  const temporary = mkdtempSync(join(owner, "run-"));
  const overlay = join(temporary, "overlay.json");
  const cleanup = (): void => rmSync(inheritedOwner ? temporary : owner, { recursive: true, force: true });
  writeFileSync(overlay, `${JSON.stringify({ Replace: plan.replacements }, null, 2)}\n`);
  process.once("exit", cleanup);
  try {
    await runRepositoryTestCommand("go", ["test", `-overlay=${overlay}`, ...args, ...packages], { cwd: moduleRoot, env: { ...(opts.env ?? process.env), SEMIO_GO_OVERLAY_OWNER: owner }, budgetMs: opts.budgetMs, signal: opts.signal, throwOnFailure: true });
  } finally {
    process.off("exit", cleanup);
    cleanup();
  }
}

/** 🏗️Builds domain-owned Go sources through the same standard-toolchain projection used by tests. */
export async function runCanonicalGoBuild(moduleRoot: string, args: string[], opts: { control: CompilerAdmissionControl; env?: NodeJS.ProcessEnv; budgetMs?: number }): Promise<void> {
  const control = requestedCompilerAdmissionControlV1(opts.control, opts.budgetMs ?? buildBudgetMs());
  const mode = readBuildMode((opts.env ?? process.env).SEMIO_BUILD_MODE), root = getWorkspaceRoot(), environment = repoToolCacheEnv(root, opts.env ?? process.env), plan = canonicalGoPlan(moduleRoot);
  const captureDirectory = environment.SEMIO_TEST_ARTIFACT_DIR ? resolve(environment.SEMIO_TEST_ARTIFACT_DIR) : resolve(moduleRoot,"dist");
  mkdirSync(captureDirectory,{recursive:true});
  const temporary = mkdtempSync(join(captureDirectory,"go-overlay-")), overlay = join(temporary,"overlay.json");
  writeFileSync(overlay, JSON.stringify({Replace:plan.replacements}));
  try {
    const { buildGoArtifactsV1 } = await import("../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🐹️go-build/🟦️.ts");
    await buildGoArtifactsV1([`-overlay=${overlay}`,...args],{version:1,mode,cwd:resolve(moduleRoot),captureDirectory,leaseDirectory:join(root,".cache","b","l","m","data"),budgetMs:opts.budgetMs??buildBudgetMs()},{environment,control});
  } finally { rmSync(temporary,{recursive:true,force:true}); }
}
