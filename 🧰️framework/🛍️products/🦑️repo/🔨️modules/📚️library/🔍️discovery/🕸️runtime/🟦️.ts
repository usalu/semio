import type { RuntimeGraphEdgeV1, RuntimeGraphFindingV1, RuntimeGraphEvidenceV1, RuntimeDynamicImportOwnerV1, RuntimeResourceReadInputV1, RuntimeResourceReadOwnerV1 } from "./🧬️schema/🟦️.ts";
export type { RuntimeGraphEdgeV1, RuntimeGraphFindingV1, RuntimeGraphEvidenceV1, RuntimeDynamicImportOwnerV1, RuntimeResourceReadInputV1, RuntimeResourceReadOwnerV1 } from "./🧬️schema/🟦️.ts";
import { posix } from "node:path";
import { createHash } from "node:crypto";
import { scanRegistryCompilerImports } from "../../🟦️.ts";
import { ecmaTokens, ecmaStringValue } from "../../../../../../🔨️modules/📚️compiler/📖️syntax/🟨️ecma/🟦️.ts";
import { rustAttributes, rustFindTopLevel, rustStringValue, rustTokenPairs, rustTokenSegments, rustTokens, rustVisibility, type RustToken } from "../../../../../../🔨️modules/📚️compiler/📖️syntax/🦀️rust/🟦️.ts";

export interface RuntimeGraphContextV1 {
  readonly read: (path: string) => string | undefined;
  readonly features?: readonly string[];
  readonly cfg?: readonly string[];
  readonly manifestDirectory?: string;
  readonly rootDirectory?: string;
  readonly workingDirectory?: string;
  readonly environment?: Readonly<Record<string, string>>;
  readonly aliases?: Readonly<Record<string, string>>;
  readonly generated?: Readonly<Record<string, readonly string[]>>;
  readonly assetOwners?: Readonly<Record<string, string>>;
  readonly moduleUrls?: Readonly<Record<string, string>>;
  readonly resourceMounts?: Readonly<Record<string, string>>;
  readonly dynamicImports?: Readonly<Record<string, readonly RuntimeDynamicImportOwnerV1[]>>;
  readonly resourceReads?: Readonly<Record<string, readonly RuntimeResourceReadOwnerV1[]>>;
  readonly directoryInputs?: Readonly<Record<string, string>>;
  readonly resourceDigest?: (path: string, kind: RuntimeResourceReadInputV1["kind"]) => string | undefined;
  readonly compiledInputs?: readonly string[];
  readonly fixtureCollections?: readonly string[];
  readonly productionTests?: "excluded";
  readonly checkCancellation?: () => void;
}
type Reference = Readonly<{ path: string | null; kind: RuntimeGraphEdgeV1["kind"]; base?: string; expression?: string; sourceSha256?: string; callsiteIndex?: number; callsiteOffset?: number }>;

/** 🧭️ Exact collection ancestry keeps Fixture-named domain types outside test input ownership. */
export function runtimeFixturePathV1(path: string, collections: readonly string[] = ["🧫️fixtures", "🧫️examples", "🧪️fixtures", "🧪️examples"]): boolean {
  const segments = posix.normalize(path).split("/");
  return segments.some((segment,index) => collections.includes(segment) && segments[index-1] !== "🔨️modules");
}

/** 🦀️ Parses conditional Rust module/resource edges using the owned compiler token model. */
export function runtimeRustReferencesV1(source: string, path: string, context: RuntimeGraphContextV1, moduleBase = posix.dirname(path)): readonly Reference[] {
  const tokens = rustTokens(source), pairs = rustTokenPairs(tokens), references: Reference[] = [];
  const sourceSha256 = createHash("sha256").update(source).digest("hex");
  const macros = new Map<string, readonly [number, number]>(), expanded = new Set<string>();
  const filesystem = new Set<string>(), readers = new Set<string>(), fileHandles = new Set<string>();
  const operations = ["read", "read_to_string", "read_dir"];
  for (let index = 0; index < tokens.length; index++) {
    if (tokens[index]?.text !== "use") continue;
    const end = rustFindTopLevel(tokens, pairs, index + 1, tokens.length, new Set([";"]));
    if (end < 0 || !tokens.slice(index + 1, end).some(token => token.text === "fs") || !tokens.slice(index + 1, end).some(token => token.text === "std")) continue;
    if (tokens.slice(index + 1, end).some(token => token.text === "*")) { for (const operation of operations) readers.add(operation); fileHandles.add("File"); }
    for (let cursor = index + 1; cursor < end; cursor++) {
      const name = tokens[cursor]?.text;
      if (name === "fs") filesystem.add(tokens[cursor + 1]?.text === "as" ? tokens[cursor + 2]!.text : name);
      if (name === "File") fileHandles.add(tokens[cursor + 1]?.text === "as" ? tokens[cursor + 2]!.text : name);
      if (operations.includes(name!)) readers.add(tokens[cursor + 1]?.text === "as" ? tokens[cursor + 2]!.text : name!);
    }
  }
  for (let index = 0; index < tokens.length; index++) if (tokens[index]?.text === "macro_rules" && tokens[index + 1]?.text === "!") {
    const body = rustFindTopLevel(tokens, pairs, index + 3, tokens.length, new Set(["{"]));
    if (body >= 0 && pairs.has(body)) macros.set(tokens[index + 2]!.text, [body + 1, pairs.get(body)!]);
  }
  const cfg = new Set(context.cfg ?? []), features = new Set(context.features ?? []);
  const predicate = (start: number, end: number): boolean => {
    const name = tokens[start]?.text;
    if (start + 1 === end) return name !== "test" && cfg.has(name!);
    if (tokens[start + 1]?.text === "=") return name === "feature" ? features.has(rustStringValue(tokens[start + 2])!) : cfg.has(`${name}=${JSON.stringify(rustStringValue(tokens[start + 2]))}`);
    const close = pairs.get(start + 1);
    if (close === undefined || close + 1 !== end) throw Error("Unresolved Rust cfg predicate");
    const values = rustTokenSegments(tokens, pairs, start + 2, close, ",").map(([a, b]) => predicate(a, b));
    if (name === "not" && values.length === 1) return !values[0];
    if (name === "all") return values.every(Boolean);
    if (name === "any") return values.some(Boolean);
    throw Error("Unsupported Rust cfg predicate");
  };
  const expression = (start: number, end: number): string | null => {
    if (end === start + 1) return rustStringValue(tokens[start]);
    const name = tokens[start]?.text, open = start + 2, close = pairs.get(open);
    if (tokens[start + 1]?.text !== "!" || close === undefined || close + 1 !== end) return null;
    const parts = rustTokenSegments(tokens, pairs, open + 1, close, ",");
    if (name === "concat") { const values = parts.map(([a, b]) => expression(a, b)); return values.every(value => value !== null) ? values.join("") : null; }
    if (name === "env" && parts.length === 1) { const key = expression(...parts[0]!); return key === "CARGO_MANIFEST_DIR" ? context.manifestDirectory ?? null : key ? context.environment?.[key] ?? null : null; }
    return null;
  };
  const visit = (start: number, end: number, base: string): void => {
    while (tokens[start]?.text === "#" && tokens[start + 1]?.text === "!" && tokens[start + 2]?.text === "[") {
      const close = pairs.get(start + 2);
      if (close === undefined) throw Error("Unresolved Rust inner attribute");
      if (tokens[start + 3]?.text === "cfg" && !predicate(start + 5, close - 1)) return;
      start = close + 1;
    }
    for (let index = start; index < end;) {
      context.checkCancellation?.();
      const attributes = rustAttributes(tokens, pairs, index, end);
      let enabled = true, mount: string | null = null;
      const attribute = (a: number, b: number): void => {
        const name = tokens[a]?.text;
        if (name === "test") enabled = false;
        if (name === "cfg") enabled &&= predicate(a + 2, b - 1);
        if (name === "path") mount = rustStringValue(tokens[a + 2]);
        if (name === "cfg_attr") {
          const parts = rustTokenSegments(tokens, pairs, a + 2, b - 1, ",");
          if (parts.length && predicate(...parts[0]!)) for (const [x, y] of parts.slice(1)) attribute(x, y);
        }
      };
      for (const [a, b] of attributes.ranges) attribute(a, b);
      if (!enabled) {
        const boundary = rustFindTopLevel(tokens, pairs, attributes.next, end, new Set(["{", ";"]));
        if (boundary < 0) break;
        index = tokens[boundary]?.text === "{" ? (pairs.get(boundary) ?? end) + 1 : boundary + 1;
        continue;
      }
      index = attributes.next;
      const visible = rustVisibility(tokens, pairs, index), head = tokens[visible.next]?.text;
      if (head === "mod" && tokens[visible.next + 1]?.kind === "identifier") {
        const name = tokens[visible.next + 1]!.text, boundary = visible.next + 2;
        if (tokens[boundary]?.text === ";") {
          if (mount !== null) references.push({ path: posix.normalize(posix.join(base, mount)), kind: "module", base: "resolved" });
          else references.push({ path: posix.join(base, name + ".rs"), kind: "module", base: posix.join(base, name, "mod.rs") });
          index = boundary + 1; continue;
        }
        if (tokens[boundary]?.text === "{") { const close = pairs.get(boundary)!; visit(boundary + 1, close, posix.join(base, mount ?? name)); index = close + 1; continue; }
      }
      const name = tokens[index]?.text;
      const filesystemNamespace = filesystem.has(tokens[index - 2]?.text ?? "") || tokens[index - 2]?.text === "fs" && tokens[index - 4]?.text === "std";
      if (tokens[index + 1]?.text === "(" && (readers.has(name!) || operations.includes(name!) && tokens[index - 1]?.text === "::" && filesystemNamespace || name === "open" && tokens[index - 1]?.text === "::" && (fileHandles.has(tokens[index - 2]?.text ?? "") || tokens[index - 2]?.text === "File" && (filesystem.has(tokens[index - 4]?.text ?? "") || tokens[index - 4]?.text === "fs" && tokens[index - 6]?.text === "std")))) {
        const close = pairs.get(index + 1);
        if (close !== undefined) {
          const args = rustTokenSegments(tokens, pairs, index + 2, close, ",");
          references.push({ path: args[0] ? expression(...args[0]) : null, kind: "resource", base: "cwd", expression: `Filesystem ${name} requires actual read input ownership`, sourceSha256, callsiteOffset: tokens[index]!.start });
        }
      }
      if (["include", "include_str", "include_bytes"].includes(name!) && tokens[index + 1]?.text === "!") {
        const close = pairs.get(index + 2);
        if (close !== undefined) { references.push({ path: expression(index + 3, close), kind: name === "include" ? "module" : "resource" }); index = close + 1; continue; }
      }
      if (name === "macro_rules") { const body = rustFindTopLevel(tokens, pairs, index + 1, end, new Set(["{"])); if (body >= 0) { index = (pairs.get(body) ?? end) + 1; continue; } }
      if (tokens[index + 1]?.text === "!" && tokens[index]?.kind === "identifier" && ["(","[","{"].includes(tokens[index + 2]?.text ?? "") && !["if","while","return","break","yield","let","else","match","in"].includes(name!)) {
        const definition = macros.get(name!);
        const builtin = ["vec", "format", "format_args", "print", "println", "eprint", "eprintln", "assert", "assert_eq", "assert_ne", "debug_assert", "debug_assert_eq", "debug_assert_ne", "matches", "env", "option_env", "concat", "stringify", "line", "file", "column", "todo", "unreachable", "panic", "write", "writeln", "compile_error"].includes(name!);
        let arms = 0;
        if (definition) for (let cursor = definition[0]; cursor < definition[1]; cursor++) { if (tokens[cursor]?.text === "=>") arms++; const close = pairs.get(cursor); if (close !== undefined && close > cursor) cursor = close; }
        const captured = definition && tokens.slice(...definition).some(token => token.text === "$");
        if (definition && arms === 1 && !captured && !expanded.has(name!)) { expanded.add(name!); visit(...definition, base); }
        else if (!builtin && !expanded.has(name!)) references.push({ path: null, kind: "module", expression: `${name}! requires actual expansion/input provenance` });
        if (!builtin) { const close = pairs.get(index + 2); if (close !== undefined) { index = close + 1; continue; } }
      }
      const close = pairs.get(index);
      if (close !== undefined && close > index) { visit(index + 1, close, base); index = close + 1; }
      else index++;
    }
  };
  visit(0, tokens.length, moduleBase);
  return references;
}

/** 🟦️ Bun compiler imports include literal dynamic/require edges; owned tokens add physical resources and unresolved calls. */
export function runtimeEcmaReferencesV1(source: string, path: string, context: RuntimeGraphContextV1 = { read: () => undefined }): readonly Reference[] {
  const physicalSource = source;
  const language = path.endsWith("tsx") ? "tsx" : /\.[mc]?ts$/u.test(path) ? "ts" : path.endsWith("jsx") ? "jsx" : "js";
  if (context.productionTests === "excluded") source = new Bun.Transpiler({ loader: language, define: { "import.meta.vitest": "undefined" }, deadCodeElimination: true }).transformSync(source);
  const references: Reference[] = scanRegistryCompilerImports(source, language).map(row => {
    const decoded = Buffer.from(row.path, "latin1").toString("utf8");
    return { path: decoded !== row.path && !decoded.includes("\ufffd") && (source.includes(decoded) || physicalSource.includes(decoded)) ? decoded : row.path, kind: "import" };
  });
  const tokens = ecmaTokens(source, 0, context.checkCancellation);
  const sourceSha256 = createHash("sha256").update(source).digest("hex");
  let callsiteIndex = 0;
  const readers = new Set(["readFileSync", "readFile"]);
  for (let index = 0; index < tokens.length; index++) {
    if (readers.has(tokens[index]!.text) && ["as", ":"].includes(tokens[index + 1]?.text ?? "") && tokens[index + 2]?.kind === "identifier") readers.add(tokens[index + 2]!.text);
  }
  const literal = (index: number): string | null => ecmaStringValue(tokens[index], context.checkCancellation);
  for (let index = 0; index < tokens.length; index++) {
    const name = tokens[index]?.text;
    if ((name === "import" || name === "require") && tokens[index + 1]?.text === "(") {
      const value = tokens[index + 3]?.text === ")" ? literal(index + 2) : null;
      if (value === null) {
        let depth=0,end=index+2;
        for(;end<tokens.length;end++){const token=tokens[end]!.text;if(["(","[","{"].includes(token))depth++;else if([")","]","}"].includes(token)){if(depth===0)break;depth--;}}
        references.push({path:null,kind:"import",sourceSha256,callsiteIndex:callsiteIndex++,expression:name+"("+tokens.slice(index+2,end).map(token=>token.text).join("")+")"});
      }
      else if (!references.some(reference => reference.kind === "import" && reference.path === value)) references.push({ path: value, kind: "import" });
    }
    if (name === "URL" && tokens[index - 1]?.text === "new" && tokens[index + 1]?.text === "(") {
      let depth = 0, comma = -1, close = -1;
      for (let cursor = index + 2; cursor < tokens.length; cursor++) {
        const text = tokens[cursor]!.text;
        if (["(", "[", "{"].includes(text)) depth++;
        else if ([")", "]", "}"].includes(text)) { if (depth === 0) { close = cursor; break; } depth--; }
        else if (text === "," && depth === 0 && comma < 0) comma = cursor;
      }
      if (comma >= 0 && tokens.slice(comma + 1, close).map(token => token.text).join("") === "import.meta.url") references.push({ path: comma === index + 3 ? literal(index + 2) : null, kind: "resource", base:"module-url" });
    }
    if ((readers.has(name!) || name === "file") && tokens[index + 1]?.text === "(") {
      if (name === "file" && tokens[index - 2]?.text !== "Bun") continue;
      references.push({ path: [",", ")"].includes(tokens[index + 3]?.text ?? "") ? literal(index + 2) : null, kind: "resource", base: "cwd" });
    }
  }
  return references;
}

/** 🕸️ Traverses resolved runtime roots, rejects fixture ancestry and retains every unresolved resource as a finding. */
export function inspectRuntimeGraphV1(roots: readonly string[], context: RuntimeGraphContextV1): RuntimeGraphEvidenceV1 {
  const absolute = (path: string): boolean => path.startsWith("/") || /^[a-z]:\//iu.test(path);
  const canonical = (path: string): string => posix.normalize(context.rootDirectory && absolute(path) ? posix.relative(context.rootDirectory, path) : path);
  const nodes = new Set<string>(), edges: RuntimeGraphEdgeV1[] = [], findings: RuntimeGraphFindingV1[] = [], pending = roots.map(path => ({ path: canonical(path), base: posix.dirname(canonical(path)) }));
  const finding = (path: string, detail: string, code: RuntimeGraphFindingV1["code"] = "runtime-unresolved-edge"): void => { findings.push({ code, path, detail }); };
  for (let cursor = 0; cursor < pending.length; cursor++) {
    context.checkCancellation?.();
    const { path, base } = pending[cursor]!;
    if (nodes.has(path)) continue;
    nodes.add(path);
    if (runtimeFixturePathV1(path, context.fixtureCollections)) { finding(path, "Runtime source or resource belongs to a fixture collection", "runtime-fixture-edge"); continue; }
    const directorySha256 = context.directoryInputs?.[path];
    if (directorySha256 !== undefined) { if (!/^[0-9a-f]{64}$/u.test(directorySha256) || context.resourceDigest?.(path, "directory") !== directorySha256) finding(path, "Current directory metadata differs from its exact captured input", "runtime-input-mismatch"); continue; }
    const source = context.read(path);
    if (source === undefined) { finding(path, "Source/resource is unavailable"); continue; }
    if (!/\.(?:rs|[cm]?[jt]sx?)$/u.test(path)) continue;
    let references: readonly Reference[];
    try { references = path.endsWith(".rs") ? runtimeRustReferencesV1(source, path, context, base) : runtimeEcmaReferencesV1(source, path, context); }
    catch (error) { finding(path, String(error)); continue; }
    const resolvedReferences: Reference[] = [];
    for (const reference of references) {
      const reads = reference.path === null && reference.kind === "resource" && reference.base === "cwd" && reference.callsiteOffset !== undefined ? context.resourceReads?.[path]?.filter(owner => Number.isSafeInteger(owner.callsiteOffset) && owner.callsiteOffset === reference.callsiteOffset) : undefined;
      if (reads?.length) {
        for (const owner of reads) {
          if (owner.sourceSha256 !== reference.sourceSha256) { finding(path, "Declared filesystem callsite differs from exact reader source", "runtime-input-mismatch"); resolvedReferences.push(reference); continue; }
          for (const input of owner.inputs) {
            const target = canonical(input.path), bytes = input.kind === "file" && !context.resourceDigest ? context.read(target) : undefined;
            const actual = context.resourceDigest ? context.resourceDigest(target, input.kind) : bytes === undefined ? undefined : createHash("sha256").update(bytes).digest("hex");
            if (!/^[0-9a-f]{64}$/u.test(input.sha256) || actual !== input.sha256 || input.kind === "directory" && context.directoryInputs?.[target] !== input.sha256) finding(target, "Declared filesystem input differs from exact current bytes or directory metadata", "runtime-input-mismatch");
            resolvedReferences.push({ path: target, kind: "resource", base: "resolved" });
          }
        }
        continue;
      }
      const owners = reference.path === null && reference.kind === "import" ? context.dynamicImports?.[path]?.filter(owner => Number.isSafeInteger(owner.callsiteIndex) && owner.callsiteIndex === reference.callsiteIndex) : undefined;
      if (!owners?.length) { resolvedReferences.push(reference); continue; }
      for (const owner of owners) {
        if (owner.sourceSha256 !== reference.sourceSha256) { finding(path, "Declared dynamic callsite differs from exact active caller source", "runtime-input-mismatch"); resolvedReferences.push(reference); continue; }
        const bytes = context.read(owner.path);
        if (bytes === undefined || createHash("sha256").update(bytes).digest("hex") !== owner.sha256) finding(owner.path, "Declared dynamic module digest differs from actual owned bytes", "runtime-input-mismatch");
        resolvedReferences.push({ path: owner.path, kind: "import", base: "resolved" });
      }
    }
    for (const reference of resolvedReferences) {
      if (reference.path === null) { finding(path, reference.expression ?? `Computed ${reference.kind} has no declared resolved owner`); continue; }
      let target = reference.path;
      if (reference.base === "module-url" && context.moduleUrls?.[path]) {
        const baseUrl = new URL(context.moduleUrls[path]!), url = new URL(target,baseUrl);
        if (url.origin!==baseUrl.origin) { finding(path,"Module URL resource leaves its declared actual browser realm"); continue; }
        const route = decodeURIComponent(url.pathname), mounted = Object.entries(context.resourceMounts ?? {}).filter(([prefix])=>route===prefix||route.startsWith(prefix+"/")).sort(([a],[b])=>b.length-a.length)[0];
        if (!mounted) { finding(path,`Module URL resource ${route} has no actual browser mount owner`); continue; }
        target = posix.join(mounted[1],route.slice(mounted[0].length));
      } else if (reference.base === "cwd") {
        if (!absolute(target) && context.workingDirectory === undefined) { finding(path, "Filesystem resource has no declared actual process cwd"); continue; }
        target = absolute(target) ? target : posix.normalize(posix.join(context.workingDirectory!, target));
      }
      else if (reference.base && reference.base !== "resolved" && reference.base !== "module-url") target = context.read(target) !== undefined ? target : reference.base;
      else if (reference.base !== "resolved") {
        if (reference.kind === "import" && !target.startsWith(".")) {
          const mapped = context.aliases?.[target];
          if (mapped) target = mapped;
          else if (target.startsWith("node:") || target === "bun" || target === "bun:test") continue;
          else { finding(path, `Package/virtual import ${target} has no resolved owner`); continue; }
        } else target = absolute(target) ? target : posix.normalize(posix.join(posix.dirname(path), target));
      }
      target = canonical(target);
      if (reference.kind === "resource" && context.assetOwners?.[target]) target = context.assetOwners[target]!;
      const candidates = reference.kind === "import" ? [target, ...[".ts", ".tsx", ".js", "/index.ts", "/index.js"].map(suffix => target + suffix)] : [target];
      const resolved = candidates.find(candidate => context.directoryInputs?.[canonical(candidate)] !== undefined || context.read(candidate) !== undefined) ?? target;
      edges.push({ from: path, to: resolved, kind: reference.kind });
      for (const origin of context.generated?.[resolved] ?? []) { edges.push({ from: resolved, to: origin, kind: "generated" }); pending.push({ path: origin, base: posix.dirname(origin) }); }
      const childBase = reference.kind === "module" && reference.base && reference.base !== "resolved" && resolved.endsWith(".rs") && !resolved.endsWith("/mod.rs") ? resolved.slice(0, -3) : posix.dirname(resolved);
      pending.push({ path: resolved, base: childBase });
    }
  }
  if (context.compiledInputs) {
    const compiled = new Set(context.compiledInputs);
    for (const path of nodes) if (!compiled.has(path)) finding(path, "Source graph input is absent from actual compiler/bundler roster", "runtime-input-mismatch");
    for (const path of compiled) if (!nodes.has(path)) finding(path, "Compiler/bundler input is absent from source graph", "runtime-input-mismatch");
  }
  return { roots, nodes: [...nodes].sort(), edges, findings: [...new Map(findings.map(finding => [JSON.stringify(finding), finding])).values()] };
}
