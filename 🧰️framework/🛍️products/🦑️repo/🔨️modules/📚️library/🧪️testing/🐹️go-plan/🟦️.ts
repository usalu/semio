import { requireIssuedScriptInvocationV1, requireIssuedResourceControlInvocationV1, advanceScriptInvocation, createRequestedIssuedResourceControlV1, scriptResourceProcessEnvironmentV1, type ScriptInvocation } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { scriptCompilerAdmissionControlV1, compilerAdmissionBudgetV1 } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { checkResourceLeaseControl, type ResourceLeaseControl } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import { testLevelBudgetMs } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import { withRepositoryTestArtifactsV1, repositoryTestArtifactRootV1, repositoryTestArtifactPathV1, repositoryTestArtifactEnvironmentV1 } from "../../../🧪️test/📦️packages/🟦️typescript/💾️storage/🟦️.ts";
import overlaySchema from "./💾️overlay/🧬️schema/🔣️.json" with {type:"json"};
const GO_OVERLAY_PROTOCOL=overlaySchema.definitions.Protocol.const;
import { existsSync, lstatSync, mkdirSync, mkdtempSync, readdirSync, readFileSync, realpathSync, rmSync, writeFileSync } from "node:fs";
import { basename, dirname, isAbsolute, join, relative, resolve, sep } from "node:path";
import { createHash } from "node:crypto";
import { canonicalGoLanguageDirectoryV1, readCanonicalGoLayoutV1 as canonicalGoLayout, type CanonicalGoLayout } from "../../🔍️discovery/🐹️go-layout/🟦️.ts";
import { runOwnedCommand, OwnedCommandFailure } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { prepareTestCommandV1 } from "../../../../../../🔨️modules/🏃️process/🧪️testing/🚦️protocol/🟦️.ts";
import { type CompilerAdmissionControl, requestedCompilerAdmissionControlV1 } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { buildBudgetMs } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/⏱️budget/🟦️.ts";
import { readBuildMode } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎚️mode/🟦️.ts";
import { pinnedGoEnvironmentV1 } from "../../⚡️caching/🚀️bootstrap/🛠️tools/🧰️native/🟦️.ts";
import { systemNativeToolPortV1 } from "../../⚡️caching/🚀️bootstrap/🛠️tools/🧰️native/🔌️port/🟦️.ts";
import { repositoryCachePartitionRootV1 } from "../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🟦️.ts";
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

/** 📂️ Validates the complete future overlay extent and existing no-follow ancestry before creation. */
function canonicalGoOverlayParentV1(root:string,inherited:string|undefined):string{
 const owner=inherited??root;if(!isAbsolute(root)||resolve(root)!==root||!isAbsolute(owner)||resolve(owner)!==owner)throw Error("Canonical issued Go overlay paths required");const coordinate=relative(root,owner);if(isAbsolute(coordinate)||coordinate===".."||coordinate.startsWith(".."+sep))throw Error("Go overlay owner must remain under issued artifacts");const longest=join(owner,...(inherited===undefined?[GO_OVERLAY_PROTOCOL.ownerPrefix+"x".repeat(GO_OVERLAY_PROTOCOL.randomSuffixUnits)]:[]),GO_OVERLAY_PROTOCOL.runPrefix+"x".repeat(GO_OVERLAY_PROTOCOL.randomSuffixUnits),GO_OVERLAY_PROTOCOL.overlayFile);if(longest.length>GO_OVERLAY_PROTOCOL.maximumPathUnits)throw Error("Bounded Go overlay paths required");for(let path=owner;;){const row=lstatSync(path);if(!row.isDirectory()||row.isSymbolicLink())throw Error("No-follow Go overlay directory required");const parent=dirname(path);if(parent===path)break;path=parent;}return owner;
}

/** 🎛️ Retains issued Go scratch and test reporting through one settled owned command. */
async function runCanonicalGoCommandV1(invocation:ScriptInvocation,control:ResourceLeaseControl,workspace:string,moduleRoot:string,args:string[],environment:NodeJS.ProcessEnv):Promise<void>{
 requireIssuedResourceControlInvocationV1(control,invocation);checkResourceLeaseControl(control);const temporary=repositoryTestArtifactPathV1(workspace,"tmp");repositoryTestArtifactPathV1(workspace,"tmp","junit-"+process.pid+"-"+Number.MAX_SAFE_INTEGER+".xml");for(const [key,parts]of[["TMPDIR",["tmp"]],["TMP",["tmp"]],["TEMP",["tmp"]],["GOCACHE",["cache","go"]],["GOMODCACHE",["cache","gomod"]]]as const){const target=repositoryTestArtifactPathV1(workspace,...parts);if(environment[key]!==target)throw Error("Issued Go scratch environment required");for(let directory=dirname(target);;){const row=lstatSync(directory);if(!row.isDirectory()||row.isSymbolicLink())throw Error("No-follow Go scratch directory required");const parent=dirname(directory);if(parent===directory)break;directory=parent;}mkdirSync(target,{recursive:true});const row=lstatSync(target);if(!row.isDirectory()||row.isSymbolicLink())throw Error("No-follow Go scratch directory required");}
 const prepared=prepareTestCommandV1("go",args,environment,temporary),started=performance.now();let failed=false,primary:unknown,code:number|null=null,signal:string|null=null,reason:string|undefined;
 try{checkResourceLeaseControl(control);await runOwnedCommand("go",[...prepared.args],moduleRoot,"go:test",compilerAdmissionBudgetV1(control),{control,signal:control.signal,env:{...environment,...prepared.environment},onProgress:async()=>{checkResourceLeaseControl(control);await advanceScriptInvocation(invocation,"go:test","running");checkResourceLeaseControl(control);}});code=0;checkResourceLeaseControl(control);}catch(error){failed=true;primary=error;if(error instanceof OwnedCommandFailure){code=error.outcome.status;signal=error.outcome.signal;}else reason="Go command did not complete";}
 try{prepared.finish({code,signal,durationMs:performance.now()-started,...(reason===undefined?{}:{reason})});}catch(error){if(!failed){failed=true;primary=error;}}if(failed)throw primary;
}

/** 🧪️ Runs canonical Go cases under the caller's actual Original and issued artifact lifetime. */
export async function runCanonicalGoTests(moduleRoot:string,args:string[],invocation:ScriptInvocation,opts:{env?:NodeJS.ProcessEnv;budgetMs?:number;packages?:readonly string[];signal?:AbortSignal}={}):Promise<void>{
 requireIssuedScriptInvocationV1(invocation);const environment=Object.freeze({...opts.env??process.env}),options=Object.freeze({budgetMs:opts.budgetMs,packages:opts.packages?[...opts.packages]:undefined,signal:opts.signal}),arguments_=[...args];if(options.signal?.aborted)throw Error("go cancelled");const workspace=getWorkspaceRoot(),phase=createRequestedIssuedResourceControlV1(scriptCompilerAdmissionControlV1(invocation,"go:test",options.budgetMs??testLevelBudgetMs()),0,options.signal);
 await withRepositoryTestArtifactsV1(workspace,invocation,phase,async actual=>{
  checkResourceLeaseControl(actual);const pinned=await pinnedGoEnvironmentV1(environment,repositoryCachePartitionRootV1(workspace,"tools"),systemNativeToolPortV1(workspace,environment.CARGO_NET_OFFLINE==="true"),actual.signal),plan=canonicalGoPlan(moduleRoot),packages=options.packages??plan.packages;if(!packages.length)throw Error("No canonical Go inputs found below "+moduleRoot);for(const selected of packages)if(!plan.packages.includes(selected))throw Error("Go compiler package is outside the canonical plan: "+selected);const root=repositoryTestArtifactRootV1(workspace),inherited=environment.SEMIO_GO_OVERLAY_OWNER,parent=canonicalGoOverlayParentV1(root,inherited),issued=repositoryTestArtifactEnvironmentV1(workspace,pinned);let owner:string|undefined,temporary:string|undefined,registered=false,failed=false;
  const cleanup=()=>{const path=inherited===undefined?owner:temporary;if(path)rmSync(path,{recursive:true,force:true});},exitCleanup=()=>{try{cleanup();}catch{}};
  try{checkResourceLeaseControl(actual);owner=inherited===undefined?mkdtempSync(join(parent,GO_OVERLAY_PROTOCOL.ownerPrefix)):parent;temporary=mkdtempSync(join(owner,GO_OVERLAY_PROTOCOL.runPrefix));const overlay=join(temporary,GO_OVERLAY_PROTOCOL.overlayFile);process.once("exit",exitCleanup);registered=true;writeFileSync(overlay,JSON.stringify({Replace:plan.replacements})+"\n");checkResourceLeaseControl(actual);await runCanonicalGoCommandV1(invocation,actual,workspace,moduleRoot,["test","-overlay="+overlay,...arguments_,...packages],scriptResourceProcessEnvironmentV1(actual,{...issued,SEMIO_GO_OVERLAY_OWNER:undefined}));checkResourceLeaseControl(actual);}catch(error){failed=true;throw error;}finally{if(registered)process.off("exit",exitCleanup);try{cleanup();}catch(error){if(!failed)throw error;}}
 });
}

/** 🏗️Builds domain-owned Go sources through the same standard-toolchain projection used by tests. */
export async function runCanonicalGoBuild(moduleRoot: string, args: string[], opts: { control: CompilerAdmissionControl; env?: NodeJS.ProcessEnv; budgetMs?: number }): Promise<void> {
  const control = requestedCompilerAdmissionControlV1(opts.control, opts.budgetMs ?? buildBudgetMs());
  const mode = readBuildMode((opts.env ?? process.env).SEMIO_BUILD_MODE), root = getWorkspaceRoot(), environment = await pinnedGoEnvironmentV1(repoToolCacheEnv(root, opts.env ?? process.env), repositoryCachePartitionRootV1(root, "tools"), systemNativeToolPortV1(root, (opts.env ?? process.env).CARGO_NET_OFFLINE === "true"), control.signal), plan = canonicalGoPlan(moduleRoot);
  const captureDirectory = environment.SEMIO_TEST_ARTIFACT_DIR ? resolve(environment.SEMIO_TEST_ARTIFACT_DIR) : resolve(moduleRoot,"dist");
  mkdirSync(captureDirectory,{recursive:true});
  const temporary = mkdtempSync(join(captureDirectory,"go-overlay-")), overlay = join(temporary,"overlay.json");
  writeFileSync(overlay, JSON.stringify({Replace:plan.replacements}));
  try {
    const { buildGoArtifactsV1 } = await import("../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🐹️go-build/🟦️.ts");
    await buildGoArtifactsV1([`-overlay=${overlay}`,...args],{version:1,mode,cwd:resolve(moduleRoot),captureDirectory,leaseDirectory:join(root,".cache","b","l","m","data"),budgetMs:opts.budgetMs??buildBudgetMs()},{environment,control});
  } finally { rmSync(temporary,{recursive:true,force:true}); }
}
