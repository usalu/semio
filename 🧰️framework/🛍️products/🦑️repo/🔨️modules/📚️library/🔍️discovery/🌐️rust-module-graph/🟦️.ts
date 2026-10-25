import { type RustCompileReference, type RustStructuralVisibility, rustAttributes, rustFindTopLevel, rustIdentifierSymbol, rustMetadataAttributes, rustPathAttributes, rustStringValue, rustTokenPairs, rustTokenText, rustTokens, rustVisibility } from "../../../../../../🔨️modules/📚️compiler/📖️syntax/🦀️rust/📦️packages/🟦️typescript/🟦️.ts";
import { posix } from "node:path";
import type { RustModuleGraphFact, RustModuleGraphFacts, RustModuleIncludeFact, RustModuleMetadataProblem, RustModuleScopeFact, RustModuleScopeProof, RustModuleUseFact } from "../🦀️rust-fact-contracts/🟦️.ts";
import { cargoProviderTomlParser } from "../🚚️cargo-provider/🟦️.ts";

/** 📦️ Cargo entry-point facts shared by module consumers and physical path authority. */
export interface RustCargoManifestFacts {
  readonly kind: "package" | "workspace" | "invalid";
  readonly crateName: string | null;
  readonly libPath: string | null;
  readonly dependencies: readonly string[];
  readonly targetPaths: readonly string[];
  readonly valid: boolean;
}

/** 🎯️ Every explicitly-pathed non-library Cargo target (`[[bin]]`/`[[test]]`/`[[bench]]`/`[[example]]`).
 * Each one is a compilation root Cargo owns exactly as it owns `[lib]`, so module membership proved
 * from it is membership in the manifest. */
const RUST_CARGO_TARGET_TABLES = ["bin", "test", "bench", "example"] as const;

/** 🧱️ Collects an array-of-tables' bodies without a TOML parser, for the lexical manifest reader. */
function rustCargoArrayTableBodies(source: string, name: string): string[] {
  const out: string[] = [];
  for (const start of source.matchAll(new RegExp(`^\\s*\\[\\[${name}\\]\\]\\s*$`, "gmu"))) {
    if (start.index === undefined) continue;
    const remainder = source.slice(start.index + start[0].length), end = /^\s*\[{1,2}[^\]]+\]{1,2}\s*$/mu.exec(remainder);
    out.push(end?.index === undefined ? remainder : remainder.slice(0, end.index));
  }
  return out;
}

/** 📋️ Reads the exact package/lib tables, rejecting ambiguous ownership declarations. */
export function inspectRustCargoManifest(source: string, strict = false): RustCargoManifestFacts {
  const acceptableTargetPath = (value: unknown): value is string => typeof value === "string" && !posix.isAbsolute(value) && !/^[A-Za-z]:/u.test(value) && !value.includes("\\");
  if (strict) {
    try {
      const parsed = cargoProviderTomlParser.parse(source) as { package?: { name?: unknown }; lib?: { name?: unknown; path?: unknown }; dependencies?: Record<string, unknown> } & Record<string, unknown>;
      if (parsed.package === undefined && parsed.workspace !== null && typeof parsed.workspace === "object" && !Array.isArray(parsed.workspace) && parsed.lib === undefined && RUST_CARGO_TARGET_TABLES.every((table) => parsed[table] === undefined)) return { kind: "workspace", crateName: null, libPath: null, dependencies: [], targetPaths: [], valid: true };
      const packageName = typeof parsed.package?.name === "string" && /^[A-Za-z0-9_-]+$/u.test(parsed.package.name) ? parsed.package.name : null;
      const libName = parsed.lib?.name === undefined ? null : typeof parsed.lib.name === "string" && /^[A-Za-z0-9_-]+$/u.test(parsed.lib.name) ? parsed.lib.name : undefined;
      const libPath = parsed.lib?.path === undefined ? null : typeof parsed.lib.path === "string" ? parsed.lib.path : undefined;
      const valid = packageName !== null && libName !== undefined && libPath !== undefined && (libPath === null || !posix.isAbsolute(libPath) && !/^[A-Za-z]:/u.test(libPath) && !libPath.includes("\\"));
      const targetPaths = RUST_CARGO_TARGET_TABLES.flatMap((table) => (Array.isArray(parsed[table]) ? (parsed[table] as { path?: unknown }[]) : [])).map((row) => row.path).filter(acceptableTargetPath);
      return { kind: valid ? "package" : "invalid", crateName: libName ?? packageName, libPath: libPath ?? null, dependencies: Object.keys(parsed.dependencies ?? {}).map((name) => name.replaceAll("-", "_")).sort(), targetPaths, valid };
    } catch { return { kind: "invalid", crateName: null, libPath: null, dependencies: [], targetPaths: [], valid: false }; }
  }
  let valid = true;
  const section = (name: string): string | null => {
    const matches = [...source.matchAll(new RegExp(`^\\s*\\[${name}\\]\\s*$`, "gmu"))];
    if (matches.length > 1) valid = false;
    const start = matches[0];
    if (!start || start.index === undefined) return null;
    const remainder = source.slice(start.index + start[0].length), end = /^\s*\[[^\]]+\]\s*$/mu.exec(remainder);
    return end?.index === undefined ? remainder : remainder.slice(0, end.index);
  };
  const value = (body: string | null, key: string, pattern: string): string | null => {
    if (body === null) return null;
    const candidates = [...body.matchAll(new RegExp(`^\\s*${key}\\s*=`, "gmu"))], values = [...body.matchAll(new RegExp(`^\\s*${key}\\s*=\\s*"(${pattern})"\\s*$`, "gmu"))];
    if (candidates.length > 1 || candidates.length !== values.length) valid = false;
    return values[0]?.[1] ?? null;
  };
  const packageSection = section("package"), libSection = section("lib"), dependencySection = section("dependencies");
  const packageName = value(packageSection, "name", "[A-Za-z0-9_-]+"), libName = value(libSection, "name", "[A-Za-z0-9_-]+"), libPath = value(libSection, "path", "[^\"\\\\]+"), crateName = libName ?? packageName;
  if (!packageName || libPath !== null && (posix.isAbsolute(libPath) || /^[A-Za-z]:/u.test(libPath))) valid = false;
  const dependencies = [...(dependencySection ?? "").matchAll(/^\s*([A-Za-z0-9_-]+)\s*=/gmu)].map((match) => match[1]!.replaceAll("-", "_")).sort();
  const targetPaths = RUST_CARGO_TARGET_TABLES.flatMap((table) => rustCargoArrayTableBodies(source, table)).map((body) => /^\s*path\s*=\s*"([^"\\]+)"\s*$/mu.exec(body)?.[1]).filter(acceptableTargetPath);
  return { kind: valid ? "package" : "invalid", crateName, libPath, dependencies, targetPaths, valid };
}

/** 🧬️ One proven lexical module context and its Cargo ownership, if declared. */
export type RustModuleMount = Readonly<{ kind: "root" } | { kind: "module"; from: string; modulePath: readonly string[]; sourceScope: readonly string[]; declarationOffset: number; inline: boolean; visibility: RustStructuralVisibility; macroUse?: true } | { kind: "include"; from: string; modulePath: readonly string[]; sourceScope: readonly string[]; line: number; path: string }>;

/** 📍️ Retains each actual source origin without collapsing alternate mounts. */
export interface RustModuleContext {
  readonly crateRoot: string;
  readonly manifestPath: string | null;
  readonly modulePath: readonly string[];
  readonly sourceScope: readonly string[];
  readonly moduleBase: string;
  readonly sourceChain: readonly string[];
  readonly mount: RustModuleMount;
}

/** 🕸️ Mounted Rust sources and dependency names, with explicit manifest provenance. */
export interface RustModuleGraph {
  readonly participations: readonly RustModuleParticipation[];
  readonly targets: ReadonlyMap<string, string>;
  readonly ambiguousTargets: ReadonlySet<string>;
  readonly unresolvedTargets: ReadonlySet<string>;
  readonly contexts: ReadonlyMap<string, readonly RustModuleContext[]>;
  readonly namedCrates: ReadonlyMap<string, readonly string[]>;
  readonly dependencies: ReadonlyMap<string, readonly string[]>;
  readonly invalidManifests: ReadonlySet<string>;
}

/** 🚦️ A retained authored origin is distinct from permission to resolve its module namespace. */
export type RustModuleParticipationReason = "invalid-manifest" | "unavailable-manifest" | "nonportable-target" | "unavailable-source" | "unresolved-scope" | "unresolved-target" | "conflicting-target" | "cyclic-target";

/** 📸️ Raw foreign targets and malformed manifests never become invented source locators. */
export type RustModuleParticipation = Readonly<{ target: Readonly<{ kind: "manifest"; path: string }>; state: "denied"; reason: "invalid-manifest" | "unavailable-manifest" } | { target: Readonly<{ kind: "source"; path: string } | { kind: "unresolved"; rawPath: string }>; context: RustModuleContext } & ({ state: "admitted" } | { state: "denied"; reason: RustModuleParticipationReason })>;

/** 🦀️ Builds only file-membership-proven module edges; conventional roots never confer manifest authority. */
export function inspectRustModuleGraph(files: readonly string[], readSource: (path: string) => string | undefined, options: Readonly<{ conventionalRoots?: boolean; strictManifests?: boolean; checkCancellation?: () => void; compileReferences?: ReadonlyMap<string, readonly RustCompileReference[]> }> = {}): RustModuleGraph {
  const compare = (left: string, right: string): number => Buffer.from(left).compare(Buffer.from(right));
  const sourceFiles = new Set(files.filter((path) => path.endsWith(".rs"))), sources = new Map<string, string | undefined>(), factsBySource = new Map<string, RustModuleGraphFacts>();
  const targets = new Map<string, string>(), ambiguousTargets = new Set<string>(), unresolvedTargets = new Set<string>(), contexts = new Map<string, RustModuleContext[]>(), namedCrates = new Map<string, string[]>(), dependencies = new Map<string, readonly string[]>(), invalidManifests = new Set<string>();
  const origins: { target: Readonly<{ kind: "source"; path: string } | { kind: "unresolved"; rawPath: string }>; context: RustModuleContext; reason?: RustModuleParticipationReason }[] = [], manifestReasons = new Map<string, "invalid-manifest" | "unavailable-manifest">(), deniedKeys = new Map<string, RustModuleParticipationReason>();
  const precedence: readonly RustModuleParticipationReason[] = ["invalid-manifest", "unavailable-manifest", "nonportable-target", "unresolved-scope", "conflicting-target", "cyclic-target", "unresolved-target", "unavailable-source"];
  const read = (path: string): string | undefined => { if (!sources.has(path)) sources.set(path, readSource(path)); return sources.get(path); };
  const sourceFacts = (path: string): RustModuleGraphFacts => {
    if (!factsBySource.has(path)) { const source = read(path); factsBySource.set(path, source === undefined ? { modules: [], uses: [], includes: [], scopes: [] } : inspectRustModuleGraphFacts(source)); }
    return factsBySource.get(path)!;
  };
  const deny = (crateRoot: string, modulePath: readonly string[], reason: RustModuleParticipationReason): void => {
    const key = crateRoot + "\0" + modulePath.join("::"), prior = deniedKeys.get(key);
    unresolvedTargets.add(key);
    if (!prior || precedence.indexOf(reason) < precedence.indexOf(prior)) deniedKeys.set(key, reason);
  };
  const manifestRoots = files.filter((path) => path === "Cargo.toml" || path.endsWith("/Cargo.toml")).sort(compare).flatMap((manifest) => {
    options.checkCancellation?.();
    const source = read(manifest), facts = inspectRustCargoManifest(source ?? "", options.strictManifests === true);
    if (!facts.valid) { invalidManifests.add(manifest); manifestReasons.set(manifest, source === undefined ? "unavailable-manifest" : "invalid-manifest"); }
    if (facts.kind === "workspace" || !facts.valid && facts.crateName === null && facts.libPath === null) return [];
    const entry = posix.normalize(posix.join(posix.dirname(manifest), facts.libPath ?? "src/lib.rs"));
    if (entry.startsWith("../") || posix.isAbsolute(entry) || /^[A-Za-z]:/u.test(entry)) return [];
    return [{ path: entry, manifestPath: manifest, crateName: facts.crateName, dependencies: facts.dependencies }, ...facts.targetPaths.map((target) => posix.normalize(posix.join(posix.dirname(manifest), target))).filter((path) => path !== entry && !path.startsWith("../")).map((path) => ({ path, manifestPath: manifest, crateName: null, dependencies: facts.dependencies }))];
  });
  const manifestOwns = (manifest: string, path: string): boolean => posix.dirname(manifest) === "." || path.startsWith(posix.dirname(manifest) + "/");
  const conventionalRoots = options.conventionalRoots ? [...sourceFiles].filter((path) => /(?:^|\/)(?:lib|main)\.rs$/u.test(path) && !manifestRoots.some((root) => root.path === path) && ![...invalidManifests].some((manifest) => manifestOwns(manifest, path))).map((path) => ({ path, manifestPath: null, crateName: null, dependencies: [] as string[] })) : [];
  for (const root of [...manifestRoots, ...conventionalRoots].sort((left, right) => compare(left.path, right.path))) {
    const crateRoot = root.path, pending: { sourcePath: string; context: RustModuleContext }[] = [], seen = new Set<string>();
    if (root.crateName) namedCrates.set(root.crateName.replaceAll("-", "_"), [...(namedCrates.get(root.crateName.replaceAll("-", "_")) ?? []), crateRoot]);
    dependencies.set(crateRoot, root.dependencies);
    const observe = (path: string, context: RustModuleContext, reason?: RustModuleParticipationReason): void => {
      const identity = path + "\0" + JSON.stringify(context);
      if (seen.has(identity)) return;
      seen.add(identity);
      const localReason = reason ?? (!sourceFiles.has(path) || read(path) === undefined ? "unavailable-source" : rustModuleScopeProof(sourceFacts(path), context.sourceScope).state === "unresolved" || path !== crateRoot && sourceFacts(path).scopes.some((scope) => scope.crateCapabilities?.length) ? "unresolved-scope" : undefined);
      origins.push({ target: { kind: "source", path }, context, ...(localReason ? { reason: localReason } : {}) });
      if (localReason) deny(crateRoot, context.modulePath, localReason);
      else contexts.set(path, [...(contexts.get(path) ?? []), context]);
      if (sourceFiles.has(path) && read(path) !== undefined && reason !== "cyclic-target") pending.push({ sourcePath: path, context });
    };
    observe(crateRoot, { crateRoot, manifestPath: root.manifestPath, modulePath: [], sourceScope: [], moduleBase: posix.dirname(crateRoot), sourceChain: [crateRoot], mount: { kind: "root" } });
    for (let index = 0; index < pending.length; index++) {
      options.checkCancellation?.();
      const { sourcePath, context } = pending[index]!;
      for (const module of sourceFacts(sourcePath).modules.filter((fact) => fact.modulePath.length === context.sourceScope.length + 1 && fact.modulePath.slice(0, -1).join("::") === context.sourceScope.join("::"))) {
        const modulePath = [...context.modulePath, module.name], key = crateRoot + "\0" + modulePath.join("::");
        const mount: RustModuleMount = { kind: "module", from: sourcePath, modulePath: context.modulePath, sourceScope: context.sourceScope, declarationOffset: module.declarationOffset, inline: module.inline, visibility: module.visibility, ...(module.macroUse ? { macroUse: true } : {}) };
        if (module.unresolved) deny(crateRoot, modulePath, "unresolved-target");
        const paths = module.unresolved?.code === "ambiguous-path" ? module.unresolved.paths : [module.pathTarget];
        const candidates = new Set<string>();
        for (const path of paths) {
          if (path !== null && (posix.isAbsolute(path) || /^[A-Za-z]:/u.test(path) || path.includes("\\") || path.includes("\0"))) {
            origins.push({ target: { kind: "unresolved", rawPath: path }, context: { ...context, modulePath, mount }, reason: "nonportable-target" });
            deny(crateRoot, modulePath, "nonportable-target");
            if (root.manifestPath && options.strictManifests) { invalidManifests.add(root.manifestPath); manifestReasons.set(root.manifestPath, "invalid-manifest"); }
            continue;
          }
          const base = path !== null && context.sourceScope.length === 0 ? posix.dirname(sourcePath) : context.moduleBase;
          for (const target of module.inline ? [sourcePath] : path === null ? [posix.join(base, module.name + ".rs"), posix.join(base, module.name, "mod.rs")] : [posix.normalize(posix.join(base, path))]) {
            if (target.startsWith("../") || target === "..") { origins.push({ target: { kind: "unresolved", rawPath: path ?? target }, context: { ...context, modulePath, mount }, reason: "nonportable-target" }); deny(crateRoot, modulePath, "nonportable-target"); }
            else candidates.add(target);
          }
        }
        const matching = [...candidates].filter((target) => sourceFiles.has(target)), selected = matching.length ? matching : [...candidates];
        if (matching.length !== 1) { deny(crateRoot, modulePath, matching.length > 1 ? "conflicting-target" : "unavailable-source"); if (matching.length > 1) ambiguousTargets.add(key); }
        for (const target of selected) {
          const base = module.pathTarget !== null && context.sourceScope.length === 0 ? posix.dirname(sourcePath) : context.moduleBase;
          const moduleBase = module.inline ? posix.normalize(posix.join(base, module.pathTarget ?? module.name)) : module.pathTarget === null && posix.basename(target) === module.name + ".rs" ? posix.join(posix.dirname(target), module.name) : posix.dirname(target);
          const child: RustModuleContext = { crateRoot, manifestPath: root.manifestPath, modulePath, sourceScope: module.inline ? module.modulePath : [], moduleBase, sourceChain: module.inline ? context.sourceChain : [...context.sourceChain, target], mount };
          const cycle = !module.inline && context.sourceChain.includes(target);
          if (cycle && root.manifestPath && options.strictManifests) { invalidManifests.add(root.manifestPath); manifestReasons.set(root.manifestPath, "invalid-manifest"); }
          observe(target, child, cycle ? "cyclic-target" : undefined);
          const prior = targets.get(key);
          if (prior && prior !== target) { ambiguousTargets.add(key); deny(crateRoot, modulePath, "conflicting-target"); }
          if (matching.length === 1 && !unresolvedTargets.has(key)) targets.set(key, target);
        }
      }
      for (const reference of options.compileReferences?.get(sourcePath) ?? []) {
        if (reference.kind !== "include" || reference.base === "generated" || (reference.modulePath ?? []).join("::") !== context.sourceScope.join("::")) continue;
        const mount: RustModuleMount = { kind: "include", from: sourcePath, modulePath: context.modulePath, sourceScope: context.sourceScope, line: reference.line, path: reference.path };
        if (reference.base === "manifest" && !context.manifestPath) { deny(crateRoot, context.modulePath, "unresolved-target"); continue; }
        const rawPath = reference.base ? reference.path.slice(1) : reference.path, base = reference.base === "manifest" ? posix.dirname(context.manifestPath!) : posix.dirname(sourcePath), target = posix.normalize(posix.join(base, rawPath));
        if (posix.isAbsolute(rawPath) || /^[A-Za-z]:/u.test(rawPath) || rawPath.includes("\\") || target.startsWith("../")) { origins.push({ target: { kind: "unresolved", rawPath }, context: { ...context, mount }, reason: "nonportable-target" }); deny(crateRoot, context.modulePath, "nonportable-target"); continue; }
        const child: RustModuleContext = { ...context, sourceScope: [], moduleBase: posix.dirname(target), sourceChain: [...context.sourceChain, target], mount };
        observe(target, child, context.sourceChain.includes(target) ? "cyclic-target" : undefined);
      }
    }
  }
  for (const [name, roots] of namedCrates) namedCrates.set(name, roots.sort(compare));
  if (options.strictManifests) for (const root of manifestRoots) if (invalidManifests.has(root.manifestPath)) deny(root.path, [], manifestReasons.get(root.manifestPath) ?? "invalid-manifest");
  const denial = (crateRoot: string, modulePath: readonly string[]): RustModuleParticipationReason | undefined => {
    const reasons: RustModuleParticipationReason[] = [];
    for (let length = 0; length <= modulePath.length; length++) { const reason = deniedKeys.get(crateRoot + "\0" + modulePath.slice(0, length).join("::")); if (reason) reasons.push(reason); }
    return reasons.sort((a, b) => precedence.indexOf(a) - precedence.indexOf(b))[0];
  };
  for (const key of targets.keys()) { options.checkCancellation?.(); const [crateRoot, path] = key.split("\0"); if (denial(crateRoot!, path ? path.split("::") : [])) targets.delete(key); }
  for (const [path, rows] of contexts) { options.checkCancellation?.(); contexts.set(path, rows.filter((row) => !denial(row.crateRoot, row.modulePath))); }
  const participations: RustModuleParticipation[] = origins.map(({ target, context, reason }) => {
    const refused = target.kind === "unresolved" ? reason : denial(context.crateRoot, context.modulePath) ?? reason;
    return refused ? { target, context, state: "denied", reason: refused } : { target, context, state: "admitted" };
  });
  for (const [path, reason] of manifestReasons) participations.push({ target: { kind: "manifest", path }, state: "denied", reason });
  return { targets, ambiguousTargets, unresolvedTargets, contexts, namedCrates, dependencies, invalidManifests, participations: participations.sort((a, b) => compare(JSON.stringify(a), JSON.stringify(b))) };
}

/** 🔎️ Extracts mounted modules, use items and `include!` expansions with lexical Rust scope, excluding decoys. */
export function inspectRustModuleGraphFacts(source: string): RustModuleGraphFacts {
  const tokens = rustTokens(source);
  const pairs = rustTokenPairs(tokens);
  const modules: RustModuleGraphFact[] = [];
  const uses: RustModuleUseFact[] = [];
  const includes: RustModuleIncludeFact[] = [];
  const scopes: RustModuleScopeFact[] = [];
  const skipItem = (start: number, end: number): number => {
    const boundary = rustFindTopLevel(tokens, pairs, start, end, new Set([";", "{"]));
    if (boundary < 0) return end;
    if (tokens[boundary]!.text === ";") return boundary + 1;
    return (pairs.get(boundary) ?? end - 1) + 1;
  };
  const parseScope = (start: number, end: number, modulePath: readonly string[], inheritedConditional = false, inheritedProblem?: RustModuleMetadataProblem): void => {
    let index = start, scopeConditional = inheritedConditional, problem = inheritedProblem;
    const crateCapabilities: "associated_type_defaults"[] = [];
    while (index + 2 < end && tokens[index]?.text === "#" && tokens[index + 1]?.text === "!" && tokens[index + 2]?.text === "[") {
      const close = pairs.get(index + 2);
      if (close === undefined || close >= end) return;
      const metadata = rustMetadataAttributes(tokens, pairs, { ranges: [[index + 3, close]], next: close + 1 });
      scopeConditional ||= metadata.conditional;
      const featureTokens = tokens.slice(index + 3, close).map((token) => token.text);
      const feature = modulePath.length === 0 && (featureTokens.join(" ") === "feature ( associated_type_defaults )" || featureTokens.join(" ") === "feature ( associated_type_defaults , )");
      if (feature && !crateCapabilities.includes("associated_type_defaults")) crateCapabilities.push("associated_type_defaults");
      const unsupported = metadata.items.filter((item) => !["allow", "warn", "deny", "forbid", "expect", "doc", "rustfmt::skip"].includes(item.name) && !(feature && item.name === "feature")).map((item) => item.name);
      if (unsupported.length) problem = { code: "unsupported-attribute", attributes: [...(problem?.code === "unsupported-attribute" ? problem.attributes : []), ...unsupported] };
      index = close + 1;
    }
    scopes.push({ kind: modulePath.length ? "inline" : "root", modulePath, bodyStartOffset: modulePath.length ? tokens[start - 1]!.start : 0, bodyEndOffset: modulePath.length ? tokens[end]!.end : source.length, ...(crateCapabilities.length ? { crateCapabilities } : {}), ...(problem ? { unresolved: problem } : {}) });
    for (; index < end;) {
      const attributes = rustAttributes(tokens, pairs, index, end), metadata = rustMetadataAttributes(tokens, pairs, attributes), conditional = scopeConditional || metadata.conditional;
      const visibility = rustVisibility(tokens, pairs, attributes.next);
      const keyword = tokens[visibility.next]?.text;
      if (keyword === "use") {
        const boundary = rustFindTopLevel(tokens, pairs, visibility.next + 1, end, new Set([";"]));
        if (boundary < 0) return;
        uses.push({ modulePath, specifier: rustTokenText(tokens, visibility.next + 1, boundary), relation: visibility.value === "private" ? "import" : "reexport", visibility: visibility.value, ...(conditional ? { conditional: true as const } : {}) });
        index = boundary + 1;
        continue;
      }
      if (keyword !== "mod") {
        if (keyword === "include" && tokens[visibility.next + 1]?.text === "!" && (tokens[visibility.next + 2]?.text === "(" || tokens[visibility.next + 2]?.text === "[")) {
          const literal = rustStringValue(tokens[visibility.next + 3]);
          if (literal !== null && tokens[visibility.next + 3]?.kind === "string") includes.push({ modulePath, path: literal, ...(conditional ? { conditional: true as const } : {}) });
        }
        index = skipItem(attributes.next, end);
        continue;
      }
      const name = tokens[visibility.next + 1];
      const boundary = rustFindTopLevel(tokens, pairs, visibility.next + 2, end, new Set([";", "{"]));
      if (!name || name.kind !== "identifier" || boundary < 0) return;
      const inline = tokens[boundary]!.text === "{";
      const symbol = rustIdentifierSymbol(name)!, childPath = [...modulePath, symbol];
      const paths = rustPathAttributes(tokens, pairs, attributes), unsupported = metadata.items.filter((item) => !["allow", "warn", "deny", "forbid", "expect", "doc", "path", "macro_use", "rustfmt::skip"].includes(item.name)).map((item) => item.name);
      const unresolved = unsupported.length ? { code: "unsupported-attribute" as const, attributes: unsupported } : paths.length > 1 ? { code: "ambiguous-path" as const, paths: paths.map((path) => path.path) } : undefined;
      modules.push({ name: symbol, modulePath: childPath, visibility: visibility.value, inline, pathTarget: paths.length === 1 ? paths[0]!.path : null, declarationOffset: tokens[visibility.next]!.start, ...(metadata.items.some((item) => item.name === "macro_use") ? { macroUse: true as const } : {}), ...(conditional ? { conditional: true as const } : {}), ...(unresolved ? { unresolved } : {}) });
      if (!inline) {
        index = boundary + 1;
        continue;
      }
      const close = pairs.get(boundary);
      if (close === undefined) return;
      parseScope(boundary + 1, close, childPath, conditional, unresolved);
      index = close + 1;
    }
  };
  parseScope(0, tokens.length, []);
  return { modules, uses, includes, scopes };
}

/** 🗝️ Selects one complete lexical body chain without inferring absent scope authority. */
export function rustModuleScopeProof(facts: RustModuleGraphFacts, sourceScope: readonly string[]): RustModuleScopeProof {
  const scopes: RustModuleScopeFact[] = [];
  for (let depth = 0; depth <= sourceScope.length; depth++) {
    const modulePath = sourceScope.slice(0, depth), parent = scopes.at(-1);
    const matches = facts.scopes.filter((scope) => scope.kind === (depth ? "inline" : "root") && scope.modulePath.join("\0") === modulePath.join("\0") && Number.isInteger(scope.bodyStartOffset) && Number.isInteger(scope.bodyEndOffset) && scope.bodyStartOffset >= 0 && scope.bodyEndOffset >= scope.bodyStartOffset && (!parent || scope.bodyStartOffset > parent.bodyStartOffset && scope.bodyEndOffset <= parent.bodyEndOffset));
    if (matches.length !== 1) return { state: "unresolved", modulePath, problem: { code: "scope-not-unique", count: matches.length } };
    const scope = matches[0]!;
    if (scope.unresolved) return { state: "unresolved", modulePath, problem: scope.unresolved };
    scopes.push(scope);
  }
  return { state: "resolved", scopes };
}
