import { type RustMutationMetadataFacts, inspectRustMutationMetadataFacts, inspectRustStructure } from "../🏛️rust-structure/🟦️.ts";
import { type CargoProviderBinding, projectCargoProviderManifest, resolveCargoProviderBinding } from "../🚚️cargo-provider/🟦️.ts";
import { type RustModuleContext, type RustModuleGraph, inspectRustModuleGraph } from "../🌐️rust-module-graph/🟦️.ts";

/** 🪪️ One frozen local provider identity required by metadata source proof. */
export interface MutationMetadataProviderIdentity { readonly role: "lower-contract" | "os-facade" | "metadata-derive"; readonly manifestLocator: string; readonly packageName: string; readonly libraryName: string; readonly procMacro: boolean; }

/** 📍️ FND21's already-proven wrapped declaration origin without a root-script dependency. */
export interface MutationMetadataSourceOrigin { readonly sourcePath: string; readonly declarationName: string; readonly modulePath: readonly string[]; }

/** 📥️ Bounded caller-owned evidence used to prove one metadata provider route. */
export interface MutationMetadataSourceInput { readonly repositoryRoot: string; readonly consumerManifestLocator: string; readonly origin: MutationMetadataSourceOrigin; readonly files: readonly string[]; readonly readSource: (path: string) => string | undefined; readonly checkCancellation?: () => void; }

/** 🧾️ Exact accepted/rejected metadata source evidence; this does not activate policy. */
export interface MutationMetadataSourceProof { readonly accepted: boolean; readonly diagnostics: readonly string[]; readonly consumerContext: RustModuleContext | null; readonly deriveProvider: MutationMetadataProviderIdentity | null; readonly contractProvider: MutationMetadataProviderIdentity | null; readonly facadeProvider: MutationMetadataProviderIdentity | null; }

const MUTATION_METADATA_PROVIDERS: readonly MutationMetadataProviderIdentity[] = [
  { role: "lower-contract", manifestLocator: "🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/Cargo.toml", packageName: "semio-framework-replication", libraryName: "protocol", procMacro: false },
  { role: "os-facade", manifestLocator: "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/Cargo.toml", packageName: "semio-framework-os-kernel", libraryName: "semio_framework_os_kernel", procMacro: false },
  { role: "metadata-derive", manifestLocator: "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/📦️packages/🦀️rust/Cargo.toml", packageName: "semio-framework-os-kernel-dsl-derive", libraryName: "dsl_derive", procMacro: true },
];

function mutationMetadataProvider(binding: CargoProviderBinding): MutationMetadataProviderIdentity | null {
  return MUTATION_METADATA_PROVIDERS.find((provider) => provider.manifestLocator === binding.providerManifestLocator && provider.packageName === binding.packageName && provider.libraryName === binding.libraryName && provider.procMacro === binding.procMacro) ?? null;
}

function mutationMetadataPath(path: string): readonly string[] | null {
  const parts = (path.startsWith("::") ? path.slice(2) : path).split("::");
  return parts.length > 0 && parts.every((part) => /^[A-Za-z_][A-Za-z0-9_]*$/u.test(part)) ? parts : null;
}

function mutationMetadataBindings(root: string, manifest: string, crate: string, cache: Map<string, readonly CargoProviderBinding[]>, readSource?: (path: string) => string | undefined): readonly CargoProviderBinding[] {
  const key = `${root}\0${manifest}\0${crate}`;
  const cached = cache.get(key);
  if (cached !== undefined) return cached;
  const manifestSource = readSource?.(manifest), projected = manifestSource === undefined ? undefined : projectCargoProviderManifest({ locator: manifest, source: manifestSource });
  const dependencyKeys = [...new Set([crate, crate.replaceAll("_", "-"), ...(projected?.dependencies.filter((dependency) => dependency.targetCondition === undefined).map((dependency) => dependency.key) ?? [])])];
  const bindings: CargoProviderBinding[] = [];
  for (const dependencyKey of dependencyKeys) try {
    const binding = resolveCargoProviderBinding({ workspaceRoot: root, consumerManifestLocator: manifest, dependencyKey });
    if (dependencyKey === crate || dependencyKey === crate.replaceAll("_", "-") || binding.externName === crate) bindings.push(binding);
  } catch {}
  const resolved = bindings.filter((binding, index) => bindings.findIndex((other) => other.providerManifestLocator === binding.providerManifestLocator && other.dependencyKey === binding.dependencyKey) === index);
  cache.set(key, resolved);
  return resolved;
}

function mutationMetadataAliasPaths(facts: RustMutationMetadataFacts, modulePath: readonly string[], path: readonly string[], publicOnly: boolean): readonly string[][] {
  const [root, ...tail] = path; if (root === undefined) return [];
  const scoped = facts.crateAliases.filter((alias) => alias.modulePath.join("\0") === modulePath.join("\0") && !alias.conditional && (!publicOnly || alias.kind === "reexport"));
  return [...scoped.filter((alias) => alias.alias === root).map((alias) => [...(mutationMetadataPath(alias.source) ?? []), ...tail]), ...scoped.filter((alias) => alias.alias === "*").map((alias) => [...(mutationMetadataPath(alias.source) ?? []), root, ...tail])].filter((candidate) => candidate.length > 0);
}

function mutationMetadataExternal(root: string, manifest: string, facts: RustMutationMetadataFacts, modulePath: readonly string[], path: string, cache: Map<string, readonly CargoProviderBinding[]>, readSource: (path: string) => string | undefined, publicOnly = false, terminalOverride?: string): readonly { readonly binding: CargoProviderBinding; readonly terminal: string }[] {
  const direct = mutationMetadataPath(path);
  if (!direct) return [];
  const resolve = (candidate: readonly string[], scope: readonly string[], depth: number, requirePublic: boolean): readonly { readonly binding: CargoProviderBinding; readonly terminal: string }[] => {
    if (depth > 16 || candidate.length === 0) return [];
    let target: readonly string[] | null = null, rest = candidate;
    if (candidate[0] === "crate") { target = []; rest = candidate.slice(1); }
    else if (candidate[0] === "self") { target = scope; rest = candidate.slice(1); }
    else if (candidate[0] === "super") { let index = 0, parent = [...scope]; while (candidate[index] === "super") { if (parent.length === 0) return []; parent.pop(); index += 1; } target = parent; rest = candidate.slice(index); }
    if (target === null) {
      const bindings = mutationMetadataBindings(root, manifest, candidate[0]!, cache, readSource);
      if (bindings.length > 0) return bindings.map((binding) => ({ binding, terminal: terminalOverride ?? candidate.at(-1)! }));
      const aliases = mutationMetadataAliasPaths(facts, scope, candidate, requirePublic);
      const transformed = aliases.filter((alias) => alias.join("\0") !== candidate.join("\0"));
      if (transformed.length > 0) return transformed.flatMap((alias) => resolve(alias, scope, depth + 1, requirePublic));
      return [];
    }
    if (rest.length === 0) return [];
    const bindings = mutationMetadataBindings(root, manifest, rest[0]!, cache, readSource);
    if (bindings.length > 0) return bindings.map((binding) => ({ binding, terminal: terminalOverride ?? rest.at(-1)! }));
    const nextScope = [...target, ...rest.slice(0, -1)], aliases = mutationMetadataAliasPaths(facts, nextScope, [rest.at(-1)!], requirePublic || nextScope.join("\0") !== scope.join("\0"));
    return aliases.flatMap((alias) => resolve(alias, [...target, ...nextScope], depth + 1, true));
  };
  return resolve(direct, modulePath, 0, publicOnly);
}

function mutationMetadataFacadeRoutes(input: MutationMetadataSourceInput, graph: RustModuleGraph, binding: CargoProviderBinding, expected: MutationMetadataProviderIdentity, seen: ReadonlySet<string>, cache: Map<string, readonly CargoProviderBinding[]>): readonly ({ readonly provider: MutationMetadataProviderIdentity; readonly facade: MutationMetadataProviderIdentity | null } | null)[] {
  const root = (graph.contexts.get(binding.librarySourceLocator) ?? []).filter((context) => context.crateRoot === binding.librarySourceLocator && context.manifestPath === binding.providerManifestLocator && context.modulePath.length === 0);
  if (root.length !== 1) return [];
  const walked = new Set<string>(), symbols = new Set<string>(), factsBySource = new Map<string, RustMutationMetadataFacts>();
  const walk = (sourcePath: string, context: RustModuleContext, path: readonly string[], depth: number): readonly ({ readonly provider: MutationMetadataProviderIdentity; readonly facade: MutationMetadataProviderIdentity | null } | null)[] => {
    input.checkCancellation?.();
    if (depth > 32 || path.length === 0) return [];
    const key = `${sourcePath}\0${context.modulePath.join("\0")}\0${path.join("\0")}`;
    if (walked.has(key)) return [];
    const symbol = `${sourcePath}\0${context.modulePath.join("\0")}\0${path[0]!}\0${path.at(-1)!}`;
    if (symbols.has(symbol)) return [];
    walked.add(key);
    symbols.add(symbol);
    const source = input.readSource(sourcePath);
    if (source === undefined) return [null];
    const facts = factsBySource.get(sourcePath) ?? inspectRustMutationMetadataFacts(source);
    factsBySource.set(sourcePath, facts);
    const aliases = mutationMetadataAliasPaths(facts, context.sourceScope, path, true), localName = (candidate: readonly string[]): boolean => { const first = candidate[0], base = first === "crate" ? [] : first === "self" ? context.modulePath : context.modulePath; const rest = first === "crate" || first === "self" ? candidate.slice(1) : candidate; return rest.length > 1 && graph.targets.has(`${binding.librarySourceLocator}\0${[...base, rest[0]!].join("::")}`); }, external = [...mutationMetadataBindings(input.repositoryRoot, binding.providerManifestLocator, path[0]!, cache, input.readSource).map((candidate) => ({ candidate, terminal: path.at(-1)! })), ...aliases.flatMap((path) => ["crate", "self", "super"].includes(path[0] ?? "") || localName(path) ? [] : mutationMetadataBindings(input.repositoryRoot, binding.providerManifestLocator, path[0]!, cache, input.readSource).map((candidate) => ({ candidate, terminal: path.at(-1)! })))].flatMap(({ candidate, terminal }) => { const route = mutationMetadataRoute(input, graph, candidate, terminal, expected, cache, seen, true); return route === undefined ? [] : [route]; });
    const local = [path, ...aliases].flatMap((candidate) => {
      let base = context.modulePath, rest = candidate;
      if (candidate[0] === "crate") { base = []; rest = candidate.slice(1); }
      else if (candidate[0] === "self") { rest = candidate.slice(1); }
      else if (candidate[0] === "super") { let index = 0; base = [...context.modulePath]; while (candidate[index] === "super") { if (base.length === 0) return []; base = base.slice(0, -1); index += 1; } rest = candidate.slice(index); }
      if (rest.length < 2) return [];
      const modulePath = [...base, rest[0]!], target = graph.targets.get(`${binding.librarySourceLocator}\0${modulePath.join("::")}`);
      const next = target === undefined ? [] : (graph.contexts.get(target) ?? []).filter((item) => item.crateRoot === binding.librarySourceLocator && item.manifestPath === binding.providerManifestLocator && item.modulePath.join("\0") === modulePath.join("\0"));
      return next.length === 1 ? walk(target!, next[0]!, rest.slice(1), depth + 1) : [];
    });
    return [...external, ...local];
  };
  return walk(binding.librarySourceLocator, root[0]!, ["MutationLeaf"], 0);
}

function mutationMetadataRoute(input: MutationMetadataSourceInput, graph: RustModuleGraph, binding: CargoProviderBinding, terminal: string, expected: MutationMetadataProviderIdentity, cache: Map<string, readonly CargoProviderBinding[]>, seen: ReadonlySet<string> = new Set(), facadeHop = false): { readonly provider: MutationMetadataProviderIdentity; readonly facade: MutationMetadataProviderIdentity | null } | null | undefined {
  input.checkCancellation?.();
  const provider = mutationMetadataProvider(binding);
  if (provider?.role === expected.role && terminal === "MutationLeaf") return { provider, facade: null };
  if (facadeHop && provider !== null && provider.role !== "os-facade") return undefined;
  if (provider?.role !== "os-facade" || expected.role === "os-facade" || terminal !== "MutationLeaf" || seen.has(binding.providerManifestLocator)) return null;
  const facadeRoutes = mutationMetadataFacadeRoutes(input, graph, binding, expected, new Set([...seen, binding.providerManifestLocator]), cache), routes = mutationMetadataResolvedRoutes(facadeRoutes);
  return routes?.length === 1 ? { provider: routes[0]!.provider, facade: provider } : null;
}

function mutationMetadataResolvedRoutes(routes: readonly ({ readonly provider: MutationMetadataProviderIdentity; readonly facade: MutationMetadataProviderIdentity | null } | null | undefined)[]): readonly { readonly provider: MutationMetadataProviderIdentity; readonly facade: MutationMetadataProviderIdentity | null }[] | null {
  if (routes.length === 0 || routes.some((route) => route === null || route === undefined)) return null;
  return routes.filter((route, index): route is { readonly provider: MutationMetadataProviderIdentity; readonly facade: MutationMetadataProviderIdentity | null } => route !== null && route !== undefined && routes.findIndex((other) => other !== null && other !== undefined && other.provider.manifestLocator === route.provider.manifestLocator && other.facade?.manifestLocator === route.facade?.manifestLocator) === index);
}

/** 🧬️ Proves exact derive and lower-trait providers for one already-wrapped consumer declaration. */
export function inspectMutationMetadataSource(input: MutationMetadataSourceInput): MutationMetadataSourceProof {
  const reject = (diagnostic: string, consumerContext: RustModuleContext | null = null): MutationMetadataSourceProof => ({ accepted: false, diagnostics: [diagnostic], consumerContext, deriveProvider: null, contractProvider: null, facadeProvider: null });
  input.checkCancellation?.();
  const graph = inspectRustModuleGraph(input.files, input.readSource, { strictManifests: true, checkCancellation: input.checkCancellation }), bindingCache = new Map<string, readonly CargoProviderBinding[]>();
  const contexts = (graph.contexts.get(input.origin.sourcePath) ?? []).filter((context) => context.manifestPath === input.consumerManifestLocator && context.sourceScope.join("\0") === input.origin.modulePath.join("\0"));
  if (contexts.length !== 1) return reject("wrapped declaration has no unique consumer module context");
  const consumerContext = contexts[0]!, source = input.readSource(input.origin.sourcePath);
  if (source === undefined) return reject("wrapped declaration source is unavailable from the consumer inventory", consumerContext);
  const sourceFacts = inspectRustMutationMetadataFacts(source), rootSource = input.readSource(consumerContext.crateRoot), rootAliases = rootSource === undefined || consumerContext.crateRoot === input.origin.sourcePath ? [] : inspectRustMutationMetadataFacts(rootSource).crateAliases.filter((item) => item.modulePath.length === 0 && (item.kind === "extern" || item.kind === "self")), facts: RustMutationMetadataFacts = { ...sourceFacts, crateAliases: [...sourceFacts.crateAliases, ...rootAliases] }, declaration = facts.declarations.filter((item) => item.name === input.origin.declarationName && item.modulePath.join("\0") === input.origin.modulePath.join("\0") && item.visibility === "pub" && !item.conditional);
  if (declaration.length !== 1) return reject("wrapped declaration is absent, conditional, private, or ambiguous", consumerContext);
  const item = declaration[0]!;
  if (item.mutationLeaf.state !== "valid") return reject("wrapped declaration has no unconditional valid mutation_leaf contract", consumerContext);
  const deriveRoutes = mutationMetadataResolvedRoutes(item.derives.flatMap((path) => mutationMetadataExternal(input.repositoryRoot, input.consumerManifestLocator, facts, item.modulePath, path, bindingCache, input.readSource).filter((route) => route.terminal === "MutationLeaf").map((route) => mutationMetadataRoute(input, graph, route.binding, route.terminal, MUTATION_METADATA_PROVIDERS[2]!, bindingCache))));
  if (deriveRoutes?.length !== 1) return reject("metadata derive route is unresolved, ambiguous, or not canonical", consumerContext);
  const contractRoutes = mutationMetadataResolvedRoutes(item.mutationLeaf.contracts.flatMap((path) => mutationMetadataExternal(input.repositoryRoot, input.consumerManifestLocator, facts, item.modulePath, path, bindingCache, input.readSource, false, "MutationLeaf").map((route) => mutationMetadataRoute(input, graph, route.binding, route.terminal, MUTATION_METADATA_PROVIDERS[0]!, bindingCache))));
  if (contractRoutes?.length !== 1) return reject("mutation_leaf contract route is unresolved, ambiguous, or not canonical", consumerContext);
  const manual = inspectRustStructure(source).impls.filter((impl) => impl.selfType.split("::").at(-1) === item.name && impl.traitPath !== null).flatMap((impl) => mutationMetadataExternal(input.repositoryRoot, input.consumerManifestLocator, facts, item.modulePath, impl.traitPath!, bindingCache, input.readSource).map((route) => mutationMetadataRoute(input, graph, route.binding, route.terminal, MUTATION_METADATA_PROVIDERS[0]!, bindingCache))).some((route) => route !== null);
  if (manual) return reject("wrapped declaration manually implements the genuine MutationLeaf trait", consumerContext);
  const facades = contractRoutes.map((route) => route.facade).filter((provider): provider is MutationMetadataProviderIdentity => provider !== null);
  if (facades.length > 1) return reject("mutation_leaf contract has ambiguous facade routes", consumerContext);
  return { accepted: true, diagnostics: [], consumerContext, deriveProvider: deriveRoutes[0]!.provider, contractProvider: contractRoutes[0]!.provider, facadeProvider: facades[0] ?? null };
}
