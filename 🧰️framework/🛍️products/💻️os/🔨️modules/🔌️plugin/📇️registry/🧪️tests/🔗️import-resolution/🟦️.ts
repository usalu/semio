import { existsSync, readFileSync, statSync } from "node:fs";
import { dirname, join, normalize, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";

const registry = resolve(dirname(fileURLToPath(import.meta.url)), "../..");
const frameworkRoot = resolve(registry, "../../../../..");
const extensions = [".ts", ".tsx", ".mjs", ".js", ".mts"];
const entries = [
  join(registry, "📜️script.ts"),
  join(frameworkRoot, "🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/🧩️component-pass/🟦️.ts"),
  join(frameworkRoot, "🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/🧩️driver-stages/🟦️.ts"),
];
const importPattern = /(?:^|[\n;])\s*(import|export)\s+(type\s+)?(\{[^}]*\}|\*(?:\s+as\s+\w+)?|\w+(?:\s*,\s*\{[^}]*\})?)\s*from\s*['"]([^'"]+)['"]/g;
const dynamicPattern = /import\(\s*['"]([^'"]+)['"]\s*\)/g;
const reExportPattern = /\bexport\s*(?:type\s*)?\{([^}]*)\}/g;
const starPattern = /\bexport\s*\*\s*(?:as\s+(\w+)\s*)?from\s*['"]([^'"]+)['"]/g;
const declarationPattern = /\bexport\s+(?:\/\*[\s\S]*?\*\/\s*)?(?:declare\s+)?(?:async\s+)?(?:abstract\s+)?(?:function\s*\*?\s*|class\s+|type\s+|interface\s+|enum\s+|namespace\s+)(\w+)/g;
const bindingPattern = /\bexport\s+(?:const|let|var)\s+([^;]*)/g;

const sources = new Map<string, string>();
const exported = new Map<string, Set<string>>();

/** 🔗️ Resolves one relative specifier to the module file it names, or null for bare and non-code specifiers. */
function resolveSpecifier(from: string, specifier: string): string | null {
  if (!specifier.startsWith(".") || /\.(json|cjs|css|wasm)$/.test(specifier)) return null;
  const base = normalize(join(dirname(from), specifier));
  const candidates = [base, ...extensions.map(extension => base + extension), ...extensions.map(extension => join(base, "index" + extension))];
  return candidates.find(candidate => existsSync(candidate) && statSync(candidate).isFile()) ?? null;
}

function source(file: string): string {
  let text = sources.get(file);
  if (text === undefined) sources.set(file, text = readFileSync(file, "utf8"));
  return text;
}

function valueNames(block: string): string[] {
  const inner = /\{([^}]*)\}/.exec(block)?.[1] ?? "";
  return inner.split(",").map(part => part.trim()).filter(part => part && !/^type\s+/.test(part)).map(part => part.split(/\s+as\s+/)[0]!.trim());
}

function bindingNames(declaration: string): string[] {
  const body = declaration.trim();
  if (body.startsWith("{")) return body.slice(1, body.indexOf("}")).split(",").map(part => part.trim().split(/\s*:\s*/).pop()!.split(/\s*=/)[0]!.trim()).filter(Boolean);
  return [...body.matchAll(/(?:^|,)\s*(\w+)\s*[:=]/g)].map(match => match[1]!);
}

/** 📤 Collects every name a module exports, following `export *` chains. */
function exportsOf(file: string, visiting = new Set<string>()): Set<string> {
  const known = exported.get(file);
  if (known) return known;
  if (visiting.has(file)) return new Set();
  visiting.add(file);
  const text = source(file), names = new Set<string>();
  for (const match of text.matchAll(declarationPattern)) names.add(match[1]!);
  for (const match of text.matchAll(bindingPattern)) for (const name of bindingNames(match[1]!)) names.add(name);
  for (const match of text.matchAll(reExportPattern)) for (const part of match[1]!.split(",")) { const name = part.trim().replace(/^type\s+/, "").split(/\s+as\s+/).pop()!.trim(); if (name) names.add(name); }
  if (/\bexport\s+default\b/.test(text)) names.add("default");
  for (const match of text.matchAll(starPattern)) {
    if (match[1]) { names.add(match[1]); continue; }
    const target = resolveSpecifier(file, match[2]!);
    if (target) for (const name of exportsOf(target, visiting)) names.add(name);
  }
  exported.set(file, names);
  return names;
}

/** 🧭 Lists every relative module a file imports, re-exports or lazily imports with a literal specifier. */
function dependenciesOf(file: string): string[] {
  const text = source(file), specifiers = [...text.matchAll(importPattern)].map(match => match[4]!);
  specifiers.push(...[...text.matchAll(dynamicPattern)].map(match => match[1]!));
  return specifiers.flatMap(specifier => resolveSpecifier(file, specifier) ?? []);
}

/** 🔎 Names every value import or re-export in the closure of the entries whose target does not export it. */
function unresolvedImports(roots: readonly string[]): string[] {
  const seen = new Set<string>(), queue = roots.filter(existsSync), problems: string[] = [];
  for (let file = queue.pop(); file !== undefined; file = queue.pop()) {
    if (seen.has(file) || !extensions.some(extension => file!.endsWith(extension))) continue;
    seen.add(file);
    queue.push(...dependenciesOf(file));
    for (const match of source(file).matchAll(importPattern)) {
      const target = resolveSpecifier(file, match[4]!);
      if (!target || match[2]) continue;
      const available = exportsOf(target), wanted = match[3]!.includes("{") ? valueNames(match[3]!) : [];
      if (/^\w/.test(match[3]!) && !available.has("default")) wanted.push("default");
      for (const name of wanted) if (!available.has(name)) problems.push(`${file.slice(frameworkRoot.length + 1)} imports ${name} from ${match[4]} which ${target.slice(frameworkRoot.length + 1)} does not export`);
    }
  }
  return problems;
}

describe("plugin registry import closure", () => {
  it("resolves every named import of the registry script, the native build facade and the plugin driver stages", () => {
    expect(entries.every(existsSync)).toBe(true);
    expect(unresolvedImports(entries)).toEqual([]);
  });

  it("reports an import whose target lacks the name", () => {
    const facade = entries[1]!, original = source(facade);
    sources.set(facade, original + '\nexport { missingNativeBuildNameV1 } from "../../../../../📦️artifacts/🏗️native-build/🧩️component-pass/🟦️.ts";\n');
    try {
      expect(unresolvedImports([facade]).some(problem => problem.includes("missingNativeBuildNameV1"))).toBe(true);
    } finally {
      sources.set(facade, original);
    }
  });
});
