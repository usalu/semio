import { posix } from "node:path";
import { rustSourceTargetProblem, rustSourceTargets, type RustSourceInputInventory, type RustSourceInputProblem } from "../🟦️.ts";
import { projectCargoProviderManifest, cargoProviderTomlParser, inspectRustModuleGraphFacts, rustModuleScopeProof, type CargoProviderManifestProjection, type CargoProviderTomlParser, type RustModuleContext, type RustModuleGraph, type RustModuleGraphFacts } from "../../../../🔍️discovery/🟦️.ts";
import { rustTokens, rustTokenPairs, rustIdentifierSymbol } from "../../../../../../../../🔨️modules/📚️compiler/📖️syntax/🦀️rust/🟦️.ts";

export type RustBindingSpan = Readonly<{ start: number; end: number }>;
export type RustBindingProblemKind = RustSourceInputProblem | "missing-manifest" | "invalid-manifest" | "missing-workspace-authority" | "unsupported-dependency-authority" | "external-provider-unproven" | "provider-package-mismatch" | "unproven-library-identity" | "provider-source-unproven" | "unknown-extern-root" | "ambiguous-extern-root" | "local-route-unproven" | "unsupported-use-tree" | "unproven-path-namespace" | "unproven-macro-output" | "orphan-context" | "unproven-module-mount" | "unproven-attribute-output" | "unsupported-edition-namespace" | "unproven-generic-scope" | "unsupported-extern-declaration" | "unproven-source-scope";
export type RustBindingProblem = Readonly<{ kind: RustBindingProblemKind; path: string; detail: string; syntaxPath?: string; span?: RustBindingSpan; crateRoot?: string; manifestPath?: string; modulePath?: readonly string[]; blockScope?: readonly number[] }>;
export type RustExternProvider = Readonly<{ externName: string; dependencyKey: string; dependencyKind: "normal" | "development" | "build"; manifestPath: string; librarySource: string; packageName: string; libraryName: string; workspaceInherited: boolean; targetCondition?: string; optional: boolean }>;
export type RustBindingEdition = "2015" | "2018" | "2021" | "2024";
export type RustExternAuthority = Readonly<{ status: "complete" | "unproven"; edition: RustBindingEdition | null; providers: readonly RustExternProvider[]; problems: readonly RustBindingProblem[] }>;
export type RustImportFact = Readonly<{ kind: "import" | "extern"; path: readonly string[]; alias?: string; glob: boolean; absolute: boolean; modulePath: readonly string[]; blockScope: readonly number[]; span: RustBindingSpan; characterSpan: RustBindingSpan; conditions: readonly string[] }>;
export type RustNamespaceDeclaration = Readonly<{ name: string; kind: "module" | "type"; modulePath: readonly string[]; blockScope: readonly number[]; span: RustBindingSpan }>;
export type RustBindingFacts = Readonly<{ graphFacts: RustModuleGraphFacts; declarations: readonly RustNamespaceDeclaration[]; imports: readonly RustImportFact[]; problems: readonly RustBindingProblem[] }>;
export type RustImportBinding = Readonly<{ from: string; to: string; librarySource: string; crateRoot: string; consumerManifest: string; modulePath: readonly string[]; fact: RustImportFact; aliasRoute: readonly RustImportFact[]; provider: RustExternProvider }>;

const localRoot = (text: string): boolean => ["self", "super", "crate"].includes(text);

/** 🦀️ Projects each graph-proven compilation context's authored extern identities without package-name inference. */
export function rustExternProviders(input: Readonly<{ context: RustModuleContext; compileKind: "library" | "test" | "build"; files: ReadonlyMap<string, string>; parser?: CargoProviderTomlParser; sourceFiles: ReadonlySet<string>; inventory: RustSourceInputInventory; checkCancellation?: () => void }>): RustExternAuthority {
  const providers: RustExternProvider[] = [], problems: RustBindingProblem[] = [], projections = new Map<string, CargoProviderManifestProjection | null>(), documents = new Map<string, unknown>();
  let edition: RustBindingEdition | null = null;
  const problem = (kind: RustBindingProblemKind, path: string, detail: string): void => { problems.push({ kind, path, detail, crateRoot: input.context.crateRoot, ...(input.context.manifestPath ? { manifestPath: input.context.manifestPath } : {}) }); };
  const target = (base: string, rawPath: string, declaration: string): string | null => {
    try {
      if (!rawPath || /[\\\0]/u.test(rawPath)) throw Error("Cargo source path requires a portable authored spelling");
      const resolved = rustSourceTargets(posix.join(base, "Cargo.toml"), [{ kind: "include", path: rawPath, line: 1 }])[0]!;
      for (const directory of resolved.directories) {
        input.checkCancellation?.();
        const failure = rustSourceTargetProblem(directory, true, input.inventory, input.sourceFiles);
        if (failure) { problem(failure === "unexpected-input-kind" ? "non-directory-ancestor" : failure, directory, declaration); return null; }
      }
      return resolved.to;
    } catch (error) { problem("unsupported-dependency-authority", posix.join(base, "Cargo.toml"), error instanceof Error ? error.message : String(error)); return null; }
  };
  const read = (path: string): CargoProviderManifestProjection | null => {
    input.checkCancellation?.();
    if (projections.has(path)) return projections.get(path)!;
    const physicalProblem = rustSourceTargetProblem(path, false, input.inventory, input.sourceFiles);
    if (physicalProblem) { problem(physicalProblem, path, "Manifest authority is not a captured physical file"); projections.set(path, null); return null; }
    const source = input.files.get(path);
    if (source === undefined) { problem("missing-manifest", path, "Authored manifest is absent from captured authority"); projections.set(path, null); return null; }
    try { const value = projectCargoProviderManifest({ locator: path, source }, { parse: text => { const parsed = (input.parser ?? cargoProviderTomlParser).parse(text); documents.set(path, parsed); return parsed; } }); projections.set(path, value); return value; }
    catch (error) { problem("invalid-manifest", path, error instanceof Error ? error.message : String(error)); projections.set(path, null); return null; }
  };
  const consumerPath = input.context.manifestPath;
  if (!consumerPath) { problem("orphan-context", input.context.crateRoot, "Compilation context has no physical manifest"); return { status: problems.length ? "unproven" : "complete", edition, providers, problems }; }
  const consumer = read(consumerPath);
  if (!consumer) return { status: problems.length ? "unproven" : "complete", edition, providers, problems };
  if (!consumer.package) { problem("invalid-manifest", consumerPath, "Compilation consumer requires an authored package"); return { status: problems.length ? "unproven" : "complete", edition, providers, problems }; }
  let workspace: CargoProviderManifestProjection | null = null;
  if (consumer.package?.workspaceLocator !== undefined) {
    const path = target(posix.dirname(consumerPath), consumer.package.workspaceLocator + "/Cargo.toml", "package.workspace");
    workspace = path ? read(path) : null;
  } else {
    let directory = posix.dirname(consumerPath);
    for (;;) {
      const path = posix.join(directory, "Cargo.toml");
      if (input.files.has(path)) { const candidate = read(path); if (candidate?.workspaceDeclared) { workspace = candidate; break; } }
      if (directory === ".") break;
      directory = posix.dirname(directory);
    }
  }
  const record = (value: unknown): Readonly<Record<string, unknown>> | undefined => typeof value === "object" && value !== null && !Array.isArray(value) ? value as Readonly<Record<string, unknown>> : undefined;
  const packageTable = record(record(documents.get(consumerPath))?.package);
  let declaredEdition: unknown = packageTable?.edition ?? "2015";
  if (typeof declaredEdition === "object") {
    const inheritance = record(declaredEdition);
    declaredEdition = inheritance?.workspace === true && Object.keys(inheritance).length === 1 && workspace ? record(record(record(documents.get(workspace.locator))?.workspace)?.package)?.edition : undefined;
  }
  if (typeof declaredEdition === "string" && ["2015", "2018", "2021", "2024"].includes(declaredEdition)) edition = declaredEdition as RustBindingEdition;
  else problem("invalid-manifest", consumerPath, "Edition requires a valid explicit string or authored workspace.package authority");
  const declarations = input.compileKind === "build" ? [{ kind: "build" as const, rows: consumer.buildDependencies }] : [{ kind: "normal" as const, rows: consumer.dependencies }, ...(input.compileKind === "test" ? [{ kind: "development" as const, rows: consumer.developmentDependencies }] : [])];
  for (const group of declarations) for (const declared of group.rows) {
    input.checkCancellation?.();
    let dependency = declared, base = posix.dirname(consumerPath);
    if (declared.workspaceInherited) {
      if (!workspace?.workspaceDeclared) { problem("missing-workspace-authority", consumerPath, declared.key); continue; }
      const inherited = workspace.workspaceDependencies.filter(row => row.key === declared.key && !row.targetCondition);
      if (inherited.length !== 1) { problem("missing-workspace-authority", workspace.locator, declared.key); continue; }
      dependency = inherited[0]!;
      base = posix.dirname(workspace.locator);
    }
    if (Object.keys(dependency.unsupported).some(key => !["optional", "features", "default-features"].includes(key)) || dependency.workspaceInherited) { problem("unsupported-dependency-authority", consumerPath, declared.key); continue; }
    if (!dependency.localPath) { problem("external-provider-unproven", consumerPath, declared.key); continue; }
    const providerPath = target(base, dependency.localPath + "/Cargo.toml", declared.key);
    if (!providerPath) continue;
    const directory = posix.dirname(providerPath), provider = read(providerPath);
    if (!provider) continue;
    if (!provider.package || provider.package.name !== (dependency.packageOverride ?? dependency.key)) { problem("provider-package-mismatch", providerPath, declared.key); continue; }
    const library = provider.library;
    if (!library?.name || !library.path || !/^[A-Za-z_][A-Za-z0-9_]*$/u.test(library.name)) { problem("unproven-library-identity", providerPath, declared.key); continue; }
    const librarySource = target(directory, library.path, declared.key);
    if (!librarySource) continue;
    if (!input.sourceFiles.has(librarySource)) { problem("provider-source-unproven", providerPath, declared.key); continue; }
    const physicalProblem = rustSourceTargetProblem(librarySource, false, input.inventory, input.sourceFiles);
    if (physicalProblem) { problem(physicalProblem, librarySource, declared.key); continue; }
    const externName = dependency.packageOverride === undefined ? library.name : declared.key.replaceAll("-", "_");
    providers.push({ externName, dependencyKey: declared.key, dependencyKind: group.kind, manifestPath: providerPath, librarySource, packageName: provider.package.name, libraryName: library.name, workspaceInherited: declared.workspaceInherited, ...(declared.targetCondition ? { targetCondition: declared.targetCondition } : {}), optional: declared.unsupported.optional === true || dependency.unsupported.optional === true });
  }
  return { status: problems.length ? "unproven" : "complete", edition, providers, problems };
}

/** 🔎️ Captures import leaves in every lexical block while retaining uncertain path and macro obligations. */
export function inspectRustBindingFacts(source: string, options: Readonly<{ checkCancellation?: () => void; onProgress?: (tokensVisited: number) => void }> = {}): RustBindingFacts {
  options.checkCancellation?.();
  const tokens = rustTokens(source), pairs = rustTokenPairs(tokens), imports: RustImportFact[] = [], declarations: RustNamespaceDeclaration[] = [], problems: RustBindingProblem[] = [];
  const byteOffsets: number[] = [0];
  let characterOffset = 0, byteOffset = 0;
  for (const character of source) { if (character.length === 2) byteOffsets[characterOffset + 1] = byteOffset; characterOffset += character.length; byteOffset += Buffer.byteLength(character); byteOffsets[characterOffset] = byteOffset; if (characterOffset % 4096 === 0) options.checkCancellation?.(); }
  const span = (start: number, end: number): RustBindingSpan => ({ start: byteOffsets[start]!, end: byteOffsets[end]! });
  const add = (start: number, end: number, path: string[], alias: string | undefined, glob: boolean, absolute: boolean, kind: RustImportFact["kind"], modulePath: string[], blockScope: number[], conditions: string[]): void => {
    imports.push({ kind, path, ...(alias === undefined ? {} : { alias }), glob, absolute, modulePath: [...modulePath], blockScope: [...blockScope], span: span(tokens[start]!.start, tokens[end]!.end), characterSpan: { start: tokens[start]!.start, end: tokens[end]!.end }, conditions: [...conditions] });
  };
  const tree = (start: number, end: number, prefix: string[], absolute: boolean, origin: number, modulePath: string[], blockScope: number[], conditions: string[]): boolean => {
    let index = start, path = [...prefix];
    if (tokens[index]?.text === "::") { absolute = true; index++; }
    while (index < end) {
      const token = tokens[index]!;
      if (token.text === "{") {
        const close = pairs.get(index);
        if (close === undefined || close >= end) return false;
        let branch = index + 1;
        for (let cursor = branch; cursor <= close; cursor++) {
          if (cursor === close || tokens[cursor]?.text === ",") { if (cursor > branch && !tree(branch, cursor, path, absolute, origin, modulePath, blockScope, conditions)) return false; branch = cursor + 1; }
          else if (tokens[cursor]?.text === "{") { const nested = pairs.get(cursor); if (nested === undefined) return false; cursor = nested; }
        }
        return close + 1 === end;
      }
      if (token.text === "*") { add(origin, index, path, undefined, true, absolute, "import", modulePath, blockScope, conditions); return index + 1 === end; }
      if (token.kind !== "identifier") return false;
      if (!(token.text === "self" && path.length)) path.push(rustIdentifierSymbol(token)!);
      index++;
      if (tokens[index]?.text === "as") {
        if (tokens[index + 1]?.kind !== "identifier" || index + 2 !== end) return false;
        add(origin, index + 1, path, rustIdentifierSymbol(tokens[index + 1])!, false, absolute, "import", modulePath, blockScope, conditions); return true;
      }
      if (index === end) { add(origin, index - 1, path, undefined, false, absolute, "import", modulePath, blockScope, conditions); return true; }
      if (tokens[index]?.text !== "::") return false;
      index++;
    }
    return false;
  };
  const visit = (start: number, end: number, modulePath: string[], blockScope: number[], inherited: readonly string[] = []): void => {
    let pending: string[] = [];
    for (let index = start; index < end; index++) {
      if (index % 256 === 0) { options.checkCancellation?.(); options.onProgress?.(index); }
      const token = tokens[index]!;
      if (token.text === "fn" && tokens[index + 2]?.text === "<" || token.text === "impl" && tokens[index + 1]?.text === "<" || ["struct", "enum", "trait", "type"].includes(token.text) && tokens[index + 2]?.text === "<") problems.push({ kind: "unproven-generic-scope", path: "", detail: "Generic type parameter shadowing requires namespace evidence", span: span(token.start, token.end) });
      const attributeOpen = token.text === "#" ? tokens[index + 1]?.text === "[" ? index + 1 : tokens[index + 1]?.text === "!" && tokens[index + 2]?.text === "[" ? index + 2 : undefined : undefined;
      if (attributeOpen !== undefined) {
        const close = pairs.get(attributeOpen);
        if (close !== undefined) {
          const attribute = source.slice(token.start, tokens[close]!.end), name = tokens[attributeOpen + 1]?.text;
          if (!["cfg", "path", "allow", "warn", "deny", "forbid", "doc", "inline", "cold", "must_use", "repr"].includes(name ?? "")) problems.push({ kind: "unproven-attribute-output", path: name ?? "", detail: attribute, span: span(token.start, tokens[close]!.end), modulePath: [...modulePath], blockScope: [...blockScope] });
          pending.push(attribute); index = close; continue;
        }
      }
      if (token.kind === "identifier" && tokens[index + 1]?.text === "!") {
        const open = token.text === "macro_rules" ? index + 3 : index + 2, close = pairs.get(open);
        problems.push({ kind: "unproven-macro-output", path: token.text, detail: "Opaque macro token trees require expansion evidence", span: span(token.start, close === undefined ? token.end : tokens[close]!.end), modulePath: [...modulePath], blockScope: [...blockScope] });
        if (close !== undefined) index = close;
        continue;
      }
      if (token.text === "use") {
        let close = index + 1;
        while (close < end && tokens[close]?.text !== ";") close++;
        if (close === end || !tree(index + 1, close, [], false, index + 1, modulePath, blockScope, [...inherited, ...pending])) problems.push({ kind: "unsupported-use-tree", path: "", detail: source.slice(token.start, tokens[Math.min(close, end - 1)]!.end), span: span(token.start, tokens[Math.min(close, end - 1)]!.end) });
        pending = []; index = close; continue;
      }
      if (token.text === "extern" && tokens[index + 1]?.text === "crate") {
        const root = tokens[index + 2], renamed = tokens[index + 3]?.text === "as";
        if (root?.kind === "identifier" && (!renamed || tokens[index + 4]?.kind === "identifier") && tokens[index + (renamed ? 5 : 3)]?.text === ";") { add(index + 2, index + (renamed ? 4 : 2), [rustIdentifierSymbol(root)!], renamed ? rustIdentifierSymbol(tokens[index + 4])! : undefined, false, false, "extern", modulePath, blockScope, [...inherited, ...pending]); index += renamed ? 5 : 3; } else problems.push({ kind: "unsupported-extern-declaration", path: "", detail: "Malformed extern crate item", span: span(token.start, root?.end ?? token.end) });
        pending = []; continue;
      }
      if (["mod", "type", "struct", "enum", "trait", "union"].includes(token.text) && tokens[index + 1]?.kind === "identifier") declarations.push({ name: rustIdentifierSymbol(tokens[index + 1])!, kind: token.text === "mod" ? "module" : "type", modulePath: [...modulePath], blockScope: [...blockScope], span: span(token.start, tokens[index + 1]!.end) });
      if (token.text === "mod" && tokens[index + 1]?.kind === "identifier" && tokens[index + 2]?.text === "{") {
        const close = pairs.get(index + 2);
        if (close !== undefined) { visit(index + 3, close, [...modulePath, rustIdentifierSymbol(tokens[index + 1])!], [], [...inherited, ...pending]); index = close; pending = []; continue; }
      }
      if (token.text === "{") {
        const close = pairs.get(index);
        if (close !== undefined) { visit(index + 1, close, modulePath, [...blockScope, token.start], [...inherited, ...pending]); index = close; pending = []; continue; }
      }
      if (token.kind === "identifier" && tokens[index + 1]?.text === "::" && tokens[index - 1]?.text !== "::") {
        let close = index;
        while (tokens[close + 1]?.text === "::" && tokens[close + 2]?.kind === "identifier") close += 2;
        const macro = tokens[close + 1]?.text === "!";
        problems.push({ kind: macro ? "unproven-macro-output" : "unproven-path-namespace", path: tokens.slice(index, close + 1).map(row => row.text).join(""), detail: "Qualified syntax requires namespace or expansion evidence", span: span(token.start, tokens[close]!.end), ...(macro ? { modulePath: [...modulePath], blockScope: [...blockScope] } : {}) });
        index = macro ? pairs.get(close + 2) ?? close : close;
      }
      if (token.text === ";") pending = [];
    }
  };
  const graphFacts = inspectRustModuleGraphFacts(source);
  for (const module of graphFacts.modules) if (module.unresolved) problems.push({ kind: "unproven-module-mount", path: module.modulePath.join("::"), detail: module.unresolved.code, span: span(module.declarationOffset, module.declarationOffset) });
  visit(0, tokens.length, [], []);
  options.checkCancellation?.(); options.onProgress?.(tokens.length);
  return { graphFacts, declarations, imports, problems };
}

/** 🏷️ Proves compiler markers against the captured namespace and refuses imported or opaque producers. */
export function rustCompilerAttributeOriginsClosed(input: Readonly<{ context: RustModuleContext; graph: RustModuleGraph; sources: ReadonlyMap<string, string>; attributes: readonly ("test" | "doc")[] }>): boolean {
  const names = new Set(["test", "doc", ...input.attributes]), facts = new Map<string, RustBindingFacts>(), visiting = new Set<string>(), results = new Map<string, boolean>();
  const origins = [...input.graph.contexts].flatMap(([path, contexts]) => contexts.filter(context => context.crateRoot === input.context.crateRoot && context.manifestPath === input.context.manifestPath).map(context => ({ path, context })));
  const read = (path: string): RustBindingFacts | undefined => {
    const source = input.sources.get(path);
    if (source === undefined) return undefined;
    if (!facts.has(path)) facts.set(path, inspectRustBindingFacts(source));
    return facts.get(path);
  };
  if (!input.context.manifestPath || input.graph.invalidManifests.has(input.context.manifestPath) || !origins.length || input.graph.participations.some(row => row.state === "denied" && "context" in row && row.context.crateRoot === input.context.crateRoot && row.context.manifestPath === input.context.manifestPath && row.context.mount.kind === "module" && row.context.mount.macroUse)) return false;
  for (const { path, context } of origins) {
    const observed = read(path);
    if (!observed || context.mount.kind === "module" && context.mount.macroUse || observed.imports.some(row => row.kind === "extern" && row.conditions.some(condition => /^#\[(?:r#)?(?:macro_use(?:\(|\])|cfg_attr\()/u.test(condition.replace(/\s/gu, ""))))) return false;
  }
  const close = (modulePath: readonly string[]): boolean => {
    const key = `${input.context.crateRoot}\0${modulePath.join("::")}`;
    if (results.has(key)) return results.get(key)!;
    if (visiting.has(key) || input.graph.ambiguousTargets.has(key) || input.graph.unresolvedTargets.has(key)) return false;
    const matches = origins.filter(row => row.context.modulePath.join("\0") === modulePath.join("\0"));
    if (!matches.length || input.graph.participations.some(row => row.state === "denied" && "context" in row && row.context.crateRoot === input.context.crateRoot && row.context.manifestPath === input.context.manifestPath && row.context.modulePath.join("\0") === modulePath.join("\0"))) return false;
    visiting.add(key);
    let admitted = true;
    for (const { path, context } of matches) {
      const observed = read(path)!;
      if (rustModuleScopeProof(observed.graphFacts, context.sourceScope).state !== "resolved" || observed.problems.some(row => row.kind === "unsupported-use-tree" || row.kind === "unsupported-extern-declaration")) { admitted = false; break; }
      const scope = context.sourceScope.join("\0");
      for (const row of observed.problems.filter(row => row.blockScope?.length === 0 && row.modulePath?.join("\0") === scope)) {
        if (row.kind === "unproven-attribute-output" && !(row.path === "test" && /^#\[\s*test\s*\]$/u.test(row.detail))) admitted = false;
        if (row.kind === "unproven-macro-output" && row.path !== "macro_rules") {
          const includes = observed.graphFacts.includes.filter(item => item.modulePath.join("\0") === scope);
          if (row.path !== "include" || !includes.length || includes.some(item => !origins.some(origin => origin.context.mount.kind === "include" && origin.context.mount.from === path && origin.context.mount.path === item.path && origin.context.modulePath.join("\0") === modulePath.join("\0")))) admitted = false;
        }
      }
      for (const row of observed.imports.filter(row => row.blockScope.length === 0 && row.modulePath.join("\0") === scope)) {
        if (!row.glob) { if (names.has((row.alias ?? row.path.at(-1)) as "test" | "doc") || observed.graphFacts.includes.length && (row.alias ?? row.path.at(-1)) === "include") admitted = false; continue; }
        const target = [...modulePath], route = [...row.path];
        if (route[0] === "crate") { target.length = 0; route.shift(); }
        else if (route[0] === "self") route.shift();
        else if (route[0] === "super") { while (route[0] === "super") { if (!target.length) admitted = false; target.pop(); route.shift(); } }
        else { admitted = false; continue; }
        if (!close([...target, ...route])) admitted = false;
      }
      if (!admitted) break;
    }
    visiting.delete(key); results.set(key, admitted);
    return admitted;
  };
  return close(input.context.modulePath);
}

/** 🧭️ Binds imported external roots separately in every physical module-graph compilation context. */
export function resolveRustImportBindings(input: Readonly<{ from: string; facts: RustBindingFacts; contexts: readonly Readonly<{ context: RustModuleContext; authority: RustExternAuthority }>[]; rootFacts?: ReadonlyMap<string, RustBindingFacts>; checkCancellation?: () => void }>): Readonly<{ status: "complete" | "unproven"; bindings: readonly RustImportBinding[]; problems: readonly RustBindingProblem[] }> {
  const bindings: RustImportBinding[] = [], problems: RustBindingProblem[] = input.facts.problems.map(problem => ({ ...problem, syntaxPath: problem.path, path: input.from }));
  const authorityProblems = new Map<string, RustBindingProblem>();
  for (const { authority } of input.contexts) for (const problem of authority.problems) authorityProblems.set(JSON.stringify(problem), problem);
  problems.push(...authorityProblems.values());
  if (!input.contexts.length) problems.push({ kind: "orphan-context", path: input.from, detail: "No source mount context" });
  for (const fact of input.facts.imports) if (!input.contexts.some(row => row.context.sourceScope.join("::") === fact.modulePath.join("::"))) problems.push({ kind: "orphan-context", path: input.from, detail: "Import scope has no physical source mount", span: fact.span });
  for (const { context, authority } of input.contexts) {
    input.checkCancellation?.();
    const scope = rustModuleScopeProof(input.facts.graphFacts, context.sourceScope);
    if (scope.state === "unresolved") { problems.push({ kind: "unproven-source-scope", path: input.from, detail: `${scope.modulePath.join("::")}: ${scope.problem.code}`, crateRoot: context.crateRoot, ...(context.manifestPath ? { manifestPath: context.manifestPath } : {}) }); continue; }
    const scoped = input.facts.imports.filter(row => row.modulePath.join("::") === context.sourceScope.join("::"));
    type Alias = Readonly<{ providers: readonly RustExternProvider[]; route: readonly RustImportFact[] }>;
    const aliases = new Map<string, Alias>(), rootAliases = new Map<string, Alias>();
    const shadowed = (fact: RustImportFact): boolean => !fact.absolute && input.facts.declarations.some(row => row.name === fact.path[0] && row.modulePath.join("::") === fact.modulePath.join("::") && row.blockScope.length <= fact.blockScope.length && row.blockScope.every((scope, index) => scope === fact.blockScope[index]));
    const key = (blocks: readonly number[], root: string): string => blocks.join("/") + "/" + root;
    const selected = (fact: RustImportFact): Alias => {
      const root = fact.path[0]!;
      if (!fact.absolute) {
        for (let length = fact.blockScope.length; length >= 0; length--) { const known = aliases.get(key(fact.blockScope.slice(0, length), root)); if (known) return known; }
        const known = rootAliases.get(root); if (known) return known;
      }
      return { providers: authority.providers.filter(provider => provider.externName === root), route: [] };
    };
    const roots = input.rootFacts?.get(context.crateRoot) ?? (input.contexts.some(row => row.context.crateRoot === context.crateRoot && row.context.manifestPath === context.manifestPath && row.context.modulePath.length === 0 && row.context.sourceScope.length === 0) ? input.facts : undefined);
    if (roots) {
      const scope = rustModuleScopeProof(roots.graphFacts, []);
      if (scope.state === "unresolved") { problems.push({ kind: "unproven-source-scope", path: context.crateRoot, detail: scope.problem.code, crateRoot: context.crateRoot, ...(context.manifestPath ? { manifestPath: context.manifestPath } : {}) }); continue; }
    }
    if (roots && input.from !== context.crateRoot) for (const problem of roots.problems) {
      const owned = { ...problem, path: context.crateRoot, syntaxPath: problem.path, crateRoot: context.crateRoot };
      if (!problems.some(previous => JSON.stringify(previous) === JSON.stringify(owned))) problems.push(owned);
    }
    for (const fact of roots?.imports ?? []) if (fact.kind === "extern" && fact.modulePath.length === 0 && fact.blockScope.length === 0 && fact.alias && !localRoot(fact.path[0]!)) rootAliases.set(fact.alias, { providers: authority.providers.filter(provider => provider.externName === fact.path[0]), route: [fact] });
    for (let pass = 0; pass <= scoped.length; pass++) {
      let changed = false;
      for (const fact of scoped) {
        input.checkCancellation?.();
        const alias = fact.alias ?? (fact.glob ? undefined : fact.path.at(-1));
        if (!alias || alias === "_" || fact.path.length !== 1 || localRoot(fact.path[0]!) || shadowed(fact) || aliases.has(key(fact.blockScope, alias))) continue;
        const binding = selected(fact);
        if (binding.providers.length) { aliases.set(key(fact.blockScope, alias), { providers: binding.providers, route: [...binding.route, fact] }); changed = true; }
      }
      if (!changed) break;
    }
    for (const fact of scoped) {
      input.checkCancellation?.();
      const root = fact.path[0]!;
      if (authority.edition === null || authority.edition === "2015") { problems.push({ kind: "unsupported-edition-namespace", path: input.from, detail: "External-prelude import proof requires an authored modern edition", span: fact.span, crateRoot: context.crateRoot }); continue; }
      if (localRoot(root) || shadowed(fact)) { problems.push({ kind: "local-route-unproven", path: input.from, detail: fact.path.join("::"), span: fact.span, crateRoot: context.crateRoot, ...(context.manifestPath ? { manifestPath: context.manifestPath } : {}) }); continue; }
      const binding = selected(fact), providers = binding.providers;
      const identities = new Set(providers.map(provider => provider.manifestPath));
      if (!providers.length || identities.size > 1) { problems.push({ kind: providers.length ? "ambiguous-extern-root" : "unknown-extern-root", path: input.from, detail: root, span: fact.span, crateRoot: context.crateRoot, ...(context.manifestPath ? { manifestPath: context.manifestPath } : {}) }); continue; }
      for (const provider of providers) bindings.push({ from: input.from, to: provider.manifestPath, librarySource: provider.librarySource, crateRoot: context.crateRoot, consumerManifest: context.manifestPath!, modulePath: [...context.modulePath], fact, aliasRoute: binding.route, provider });

    }
  }
  return { status: problems.length ? "unproven" : "complete", bindings, problems };
}
