import { inspectRustCompileReferences, type RustGeneratedTokenOutput } from "../../../../../../../../🔨️modules/📚️compiler/📖️syntax/🦀️rust/🟦️.ts";
import { inspectRustPathLiterals } from "../../../../../../../../🔨️modules/📚️compiler/📖️syntax/🦀️rust/📁️paths/🟦️.ts";
import { rustRuntimePathDirectionEdges, type RustRuntimePathDirectionEdge } from "../📁️runtime/🟦️.ts";
import { dirname, join, posix, resolve } from "node:path";
import { lstatSync, readFileSync, readdirSync } from "node:fs";
import { mkdir, stat as followedStat, writeFile } from "node:fs/promises";
import { rustSourceDirectionEdges, rustSourceTargets, rustSourceTargetProblem, type RustSourceDirectionEdge, type RustSourceInputNode, type RustSourceInputProblem, type RustSourceTarget } from "../🟦️.ts";
import type { DependencyDirectionRule } from "../../🟦️.ts";
import { inspectRustModuleGraph, inspectRustModuleGraphFacts, projectCargoProviderManifest } from "../../../../🔍️discovery/🟦️.ts";
import { type RustCompileExpansion } from "../../../../../../../../🔨️modules/📚️compiler/📖️syntax/🦀️rust/🟦️.ts";
import { loadDependencyDirectionPolicy } from "../../🚀️bootstrap/🟦️.ts";
import { rustCompilerAttributeOriginsClosed } from "../🔗️binding/🟦️.ts";
import { COMPUTE_OWNERSHIP_CONTRACT_PATH, rustFamilyOwnershipActive, inspectRustFamilyOwnership, readRustFamilyOwnershipContract, type RustFamilyOwnershipProblem } from "../📍️ownership/🟦️.ts";

export type RustSourceInputFailure = Readonly<{ code: RustSourceInputProblem; to: string; kind: RustSourceTarget["reference"]["kind"]; line: number }>;
export type RustSourceDirectionProblem = Readonly<{ code: RustFamilyOwnershipProblem["code"] | RustSourceInputProblem | "unsupported-expression" | "unresolved-target" | "unresolved-template-scope" | "unresolved-generator-origin"; from: string; detail: string; to?: string; kind?: RustSourceTarget["reference"]["kind"]; line?: number; expansion?: RustCompileExpansion }>;
export type RustSourceDirectionReport = Readonly<{ schemaVersion: 1; generatedTokens: readonly Readonly<{ from: string; output: RustGeneratedTokenOutput; manifests: readonly string[] }>[]; files: number; references: number; violations: readonly RustSourceDirectionEdge[]; problems: readonly RustSourceDirectionProblem[]; runtime: Readonly<{ scope: "owner-qualified-literal-first-arguments"; references: number; violations: readonly RustRuntimePathDirectionEdge[] }> }>;

/** 🪪️ Inspects every authored input component with lstat so links cannot conceal another owner. */
export async function inspectRustSourceInputs(root: string, targets: readonly RustSourceTarget[], sources: ReadonlySet<string>, checkCancellation: () => void = () => {}): Promise<readonly RustSourceInputFailure[]> {
  if (!root || root.includes("\0") || /^[A-Za-z]:(?:$|[^\\/])/u.test(root) || root.split(/[\\/]/u).some((part) => part === "." || part === "..")) throw new Error(`Rust source input requires an unnormalized safe root: ${root}`);
  const rootDirectory = resolve(root), ancestors: string[] = [];
  for (let current = rootDirectory; ; current = dirname(current)) {
    ancestors.push(current);
    if (current === dirname(current)) break;
  }
  let inspected = 0;
  for (const current of ancestors.reverse()) {
    if (++inspected % 64 === 0) await new Promise<void>((accept) => setImmediate(accept));
    checkCancellation();
    let info;
    try { info = lstatSync(current); }
    catch (error) {
      if ((error as NodeJS.ErrnoException).code === "ENOENT") throw new Error(`Rust source input has an unavailable root: ${current}`);
      throw error;
    }
    if (info.isSymbolicLink()) throw new Error(`Rust source input refuses a linked root: ${current}`);
    if (!info.isDirectory()) throw new Error(`Rust source input requires a physical root directory kind: ${current}`);
  }
  const nodes = new Map<string, RustSourceInputNode["kind"]>(), absent = new Set<string>(), failures: RustSourceInputFailure[] = [];
  const inspect = async (to: string, directory: boolean): Promise<RustSourceInputProblem | null> => {
    checkCancellation();
    if (to === ".") {
      const info = lstatSync(rootDirectory);
      return info.isSymbolicLink() ? "linked-input" : info.isDirectory() && directory ? null : "unexpected-input-kind";
    }
    const parts = to.split("/");
    if (posix.isAbsolute(to) || /^[A-Za-z]:/u.test(to) || to.includes("\\") || parts.some((part) => part === ".." || part === "." || part === "")) throw new Error(`Rust source input requires a canonical workspace target: ${to}`);
    for (let index = 0; index < parts.length; index++) {
      if (++inspected % 64 === 0) await new Promise<void>((accept) => setImmediate(accept));
      checkCancellation();
      const path = parts.slice(0, index + 1).join("/");
      if (absent.has(path)) break;
      if (!nodes.has(path)) {
        try {
          const info = lstatSync(join(rootDirectory, path));
          if (!info.isSymbolicLink() && !info.isDirectory() && !info.isFile()) throw new Error(`Unsupported authored input type: ${path}`);
          nodes.set(path, info.isSymbolicLink() ? "symlink" : info.isDirectory() ? "directory" : "file");
        } catch (error) {
          if ((error as NodeJS.ErrnoException).code !== "ENOENT") throw error;
          absent.add(path); break;
        }
      }
      if (nodes.get(path)! !== "directory") break;
    }
    return rustSourceTargetProblem(to, directory, nodes, sources);
  };
  for (const { to, reference, directories } of targets) {
    let failure: RustSourceInputFailure | undefined;
    for (const directory of directories) {
      const code = await inspect(directory, true);
      if (code) { failure = { code: code === "unexpected-input-kind" ? "non-directory-ancestor" : code, to: directory, kind: reference.kind, line: reference.line }; break; }
    }
    if (!failure) {
      const code = await inspect(to, reference.directory === true);
      if (code) failure = { code, to, kind: reference.kind, line: reference.line };
    }
    if (failure) failures.push(failure);
  }
  return failures;
}

/** 🧾️ Inventories every present authored Rust owner and returns the complete typed layer verdict. */
export async function inspectRustSourceDirection(root: string): Promise<RustSourceDirectionReport> {
  const { taxonomy, policy } = loadDependencyDirectionPolicy(root);
  const names = ["framework-no-implementation", "repo-no-implementation", "s-modules-no-plugins", ...Object.keys(taxonomy.dependencyDirections.rules)];
  const patterns = (value: string | readonly string[]): readonly string[] => typeof value === "string" ? [value] : value;
  const rules: DependencyDirectionRule[] = policy.forbidden.filter((rule) => names.includes(rule.name) || rule.name.startsWith("plugin-no-extension-or-artifact-")).map((rule) => {
    if (!rule.from.path || !rule.to.path) throw new Error(`Rust source direction requires authored selectors: ${rule.name}`);
    return { name: rule.name, severity: rule.severity, from: { path: patterns(rule.from.path), ...(rule.from.pathNot ? { pathNot: patterns(rule.from.pathNot) } : {}) }, to: { path: patterns(rule.to.path), ...(rule.to.pathNot ? { pathNot: patterns(rule.to.pathNot) } : {}) } };
  });
  if (names.some((name) => !rules.some((rule) => rule.name === name)) || new Set(rules.map((rule) => rule.name)).size !== rules.length || rules.some((rule) => rule.severity !== "error")) throw new Error("Rust source direction requires every declared strict layer rule");
  const ignore: string[] = taxonomy.implementationLeafPolicy.ignoredPathPatterns.map((path: string) => path.replace(/^\*\*\//u, ""));
  const excluded: string[] = Object.values(taxonomy.pathExclusions).map((value) => value.path.replace(/\/$/u, ""));
  const edges: RustSourceDirectionEdge[] = [];
  const runtime = { scope: "owner-qualified-literal-first-arguments" as const, references: 0, violations: [] as RustRuntimePathDirectionEdge[] };
  const ownerRoots = Object.keys(taxonomy.areaLayers);
  let sources = new Map<string, string>();
  const inventory = new Map<string, RustSourceInputNode["kind"]>();
  const rootInput = lstatSync(root);
  if (!rootInput.isDirectory() || rootInput.isSymbolicLink()) throw Error("Rust source census requires a physical root");
  inventory.set(".", "directory");
  const problems: RustSourceDirectionProblem[] = [];
  let files = 0, references = 0, stopped = false;
  const stop = (): void => { stopped = true; };
  process.once("SIGINT", stop); process.once("SIGTERM", stop);
  const check = (): void => { if (stopped) throw new Error("Rust source direction scan canceled"); };
  let inspected = 0;
  const checkpoint = async (): Promise<void> => {
    if (++inspected % 64 === 0) await new Promise<void>((accept) => setImmediate(accept));
    check();
  };
  const walk = async (path: string): Promise<readonly string[]> => {
    await checkpoint();
    if (ignore.some((part) => path === part || path.endsWith(`/${part}`)) || excluded.some((part) => path === part || path.startsWith(`${part}/`))) return [];
    const stat = lstatSync(join(root, path));
    inventory.set(path, stat.isSymbolicLink() ? "symlink" : stat.isDirectory() ? "directory" : "file");
    if (stat.isSymbolicLink()) {
      if (path === COMPUTE_OWNERSHIP_CONTRACT_PATH || path.endsWith(".rs") || path === "Cargo.toml" || path.endsWith("/Cargo.toml") || (await followedStat(join(root, path))).isDirectory()) throw new Error(`Rust source direction cannot inventory linked input: ${path}`);
      return [];
    }
    if (stat.isDirectory()) {
      return readdirSync(join(root, path)).sort().map((entry) => `${path}/${entry}`);
    } else if (stat.isFile() && (path === COMPUTE_OWNERSHIP_CONTRACT_PATH || path.endsWith(".rs") || path === "Cargo.toml" || path.endsWith("/Cargo.toml"))) sources.set(path, readFileSync(join(root, path), "utf8"));
    return [];
  };
  try {
    let pending: readonly string[] = [];
    for (const area of Object.keys(taxonomy.areaLayers)) {
      check();
      let entry;
      try { entry = lstatSync(join(root, area)); } catch (error) { if ((error as NodeJS.ErrnoException).code === "ENOENT") continue; throw error; }
      if (!entry.isDirectory()) throw new Error(`Rust source area requires a physical directory: ${area}`);
      pending = [...pending, area];
    }
    for (const entry of readdirSync(root, { withFileTypes: true })) if ((entry.isFile() || entry.isSymbolicLink()) && (entry.name.endsWith(".rs") || entry.name === "Cargo.toml")) pending = [...pending, entry.name];
    while (pending.length) {
      const batch = pending, children: (readonly string[])[] = new Array(batch.length);
      for (let index = 0; index < batch.length; index++) children[index] = await walk(batch[index]!);
      pending = children.flat();
    }
    sources = new Map([...sources].sort(([left], [right]) => Buffer.from(left).compare(Buffer.from(right))));
    const compileReferences = new Map<string, ReturnType<typeof inspectRustCompileReferences>>(), deferred = new Map<string, RustGeneratedTokenOutput[]>();
    const generatedTokens: { from: string; output: RustGeneratedTokenOutput; manifests: string[] }[] = [];
    for (const [path, source] of sources) {
      await checkpoint();
      if (!path.endsWith(".rs")) continue;
      try { const outputs: RustGeneratedTokenOutput[] = []; compileReferences.set(path, inspectRustCompileReferences(source, output => outputs.push(output))); if (outputs.length) deferred.set(path, outputs); } catch (error) { problems.push({ code: "unsupported-expression", from: path, detail: (error as Error).message }); }
    }
    const graph = inspectRustModuleGraph([...sources.keys()], (path) => sources.get(path), { checkCancellation: check, compileReferences, strictManifests: true });
    if (rustFamilyOwnershipActive({ sources, graph, inventory, checkCancellation: check })) try {
      const bytes = sources.get(COMPUTE_OWNERSHIP_CONTRACT_PATH);
      if (bytes === undefined || inventory.get(COMPUTE_OWNERSHIP_CONTRACT_PATH) !== "file") throw Error("Canonical compute ownership contract is not a captured physical input");
      problems.push(...inspectRustFamilyOwnership({ contract: readRustFamilyOwnershipContract(JSON.parse(bytes)), sources, graph, inventory, checkCancellation: check }));
    } catch (error) { problems.push({ code: "invalid-ownership-contract", from: COMPUTE_OWNERSHIP_CONTRACT_PATH, detail: error instanceof Error ? error.message : String(error) }); }
    const sourcePaths = new Set(sources.keys());
    for (const [path] of sources) {
      await checkpoint();
      if (!path.endsWith(".rs")) continue;
      const refs = compileReferences.get(path) ?? [];
      const pathLiterals = inspectRustPathLiterals(sources.get(path)!, ownerRoots);
      runtime.references += pathLiterals.length;
      runtime.violations.push(...rustRuntimePathDirectionEdges(path, pathLiterals, rules));
      references += refs.length; files++;
      const contexts = graph.contexts.get(path), manifestPaths: string[] = [];
      for (const output of deferred.get(path) ?? []) {
        const owners = contexts?.filter(context => context.sourceScope.length === 0) ?? [], manifests = [...new Set(owners.flatMap(context => context.manifestPath ? [context.manifestPath] : []))];
        const sealed = owners.length > 0 && owners.every(context => context.manifestPath && !graph.invalidManifests.has(context.manifestPath) && context.sourceChain.at(-1) === path && context.mount.kind !== "include") && manifests.every(locator => {
          const source = sources.get(locator); if (source === undefined) return false;
          const projection = projectCargoProviderManifest({ locator, source }), candidates = projection.dependencies.filter(binding => binding.key === "quote");
          return candidates.length === 1 && candidates[0]!.source === "version" && typeof candidates[0]!.version === "string" && !candidates[0]!.targetCondition && !candidates[0]!.workspaceInherited && !candidates[0]!.localPath && !candidates[0]!.packageOverride && Object.keys(candidates[0]!.unsupported).length === 0;
        });
        generatedTokens.push({ from: path, output, manifests });
        if (!sealed) problems.push({ code: "unresolved-generator-origin", from: path, line: output.line, detail: "Token generator requires a captured direct normal quote provider and exclusive lexical scope: " + output.macro });
      }
      const checkedTemplates = new Set<number>();
      let uses: ReturnType<typeof inspectRustModuleGraphFacts>["uses"] | undefined;
      for (const reference of refs) {
        const expansion = reference.expansion;
        if (!expansion || checkedTemplates.has(expansion.definitionOffset)) continue;
        checkedTemplates.add(expansion.definitionOffset);
        const scope = expansion.scope, lexicalPath = scope.kind === "module" ? scope.modulePath : reference.modulePath ?? [];
        const occurrences = refs.filter((ref) => ref.expansion?.definitionOffset === expansion.definitionOffset);
        const coherent = occurrences.every((ref) => {
          const item = ref.expansion!;
          return (ref.modulePath ?? []).join("::") === lexicalPath.join("::") && item.macro === expansion.macro && item.definitionLine === expansion.definitionLine && JSON.stringify(item.scope) === JSON.stringify(scope) && item.definitionOffset < item.templateOffset && item.templateOffset < item.invocationOffset && item.invocationOffset < sources.get(path)!.length && (scope.kind === "module" || scope.startOffset < item.definitionOffset && item.invocationOffset < scope.endOffset);
        });
        const candidates = contexts?.filter((context) => context.sourceScope.join("::") === lexicalPath.join("::")) ?? [];
        uses ??= inspectRustModuleGraphFacts(sources.get(path)!).uses;
        const aliases = uses.some((fact) => fact.specifier.split(/[^\p{L}\p{N}_]/u).includes(expansion.macro));
        const sealed = coherent && !aliases && candidates.length > 0 && candidates.every((context) => {
          if (!context.manifestPath || graph.invalidManifests.has(context.manifestPath) || context.sourceChain[0] !== context.crateRoot || context.sourceChain.at(-1) !== path || context.mount.kind === "include") return false;
          if (scope.kind === "local-block") return scope.startOffset < expansion.definitionOffset && expansion.definitionOffset < expansion.templateOffset && expansion.templateOffset < expansion.invocationOffset && expansion.invocationOffset < scope.endOffset && (!scope.compilerAttributes?.length || rustCompilerAttributeOriginsClosed({ context, graph, sources, attributes: scope.compilerAttributes }));
          const mount = context.mount;
          return mount.kind === "module" && !mount.inline && mount.visibility === "private" && !mount.macroUse && graph.targets.get(`${context.crateRoot}\0${context.modulePath.join("::")}`) === path && context.sourceChain.at(-2) === mount.from && (graph.contexts.get(mount.from) ?? []).some((parent) => parent.crateRoot === context.crateRoot && parent.manifestPath === context.manifestPath && parent.modulePath.join("::") === mount.modulePath.join("::") && parent.sourceScope.join("::") === mount.sourceScope.join("::") && parent.sourceChain.join("\0") === context.sourceChain.slice(0, -1).join("\0"));
        });
        if (!sealed) problems.push({ code: "unresolved-template-scope", from: path, kind: reference.kind, line: reference.line, expansion, detail: `Finite Rust macro requires an exclusive live lexical scope: ${expansion.macro}; definitionOffset=${expansion.definitionOffset}` });
      }
      for (let directory = posix.dirname(path); directory !== "."; directory = posix.dirname(directory)) {
        const manifest = `${directory}/Cargo.toml`;
        if (sources.has(manifest)) { manifestPaths.push(manifest); break; }
      }
      try {
        const ownership = { contexts, manifestPaths: contexts?.some((context) => context.manifestPath) ? [] : manifestPaths };
        edges.push(...rustSourceDirectionEdges(path, refs, rules, ownership));
        for (const failure of await inspectRustSourceInputs(root, rustSourceTargets(path, refs, ownership), sourcePaths, check)) problems.push({ ...failure, from: path, detail: `Authored compile input failed physical census: ${failure.to}` });
      } catch (error) { problems.push({ code: "unresolved-target", from: path, detail: (error as Error).message }); }
      if (files % 250 === 0) console.log(`[rust-source-direction] progress; files=${files}; references=${references}`);
    }
    if (!files) throw new Error("Rust source direction requires a nonempty authored Rust source inventory");
    return { schemaVersion: 1, files, references, violations: edges, problems, runtime, generatedTokens };
  } finally { process.off("SIGINT", stop); process.off("SIGTERM", stop); }
}

/** 🛡️ Rejects compile-time, declared runtime path and physical census boundary failures. */
export async function verifyRustSourceDirection(root: string, reportPath?: string): Promise<void> {
  const report = await inspectRustSourceDirection(root);
  if (reportPath) { await mkdir(join(reportPath, ".."), { recursive: true }); await writeFile(reportPath, `${JSON.stringify(report, null, 2)}\n`); }
  for (const edge of report.violations) console.error(`[rust-source-direction] ${edge.rule}: ${edge.from}:${edge.line} → ${edge.to}; kind=${edge.kind}`);
  for (const edge of report.runtime.violations) console.error(`[rust-source-direction] ${edge.rule}: ${edge.from}:${edge.line} → ${edge.to}; runtime path literal=${edge.call}; context=${edge.context}`);
  for (const problem of report.problems) console.error(`[rust-source-direction] ${problem.code}: ${problem.from}${problem.line ? `:${problem.line}` : ""}: ${problem.detail}`);
  if (report.violations.length || report.runtime.violations.length || report.problems.length) throw new Error(`Rust source direction failed: ${report.violations.length} strict compile-time boundary violations, ${report.runtime.violations.length} strict runtime path boundary violations and ${report.problems.length} source census problems across ${report.files} files and ${report.references} compile-time references`);
  console.log(`[rust-source-direction] passed; files=${report.files}; authoredReferences=${report.references}; scope=all-configurations-and-macro-templates; runtimeReferences=${report.runtime.references}; runtimeScope=${report.runtime.scope}`);
}
