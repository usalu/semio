import type { PackageGlueGrammarSpec, PackageSourceDisposition } from "../📜️taxonomy-contracts/🟦️.ts";
import { ecmaCommandRouterModule } from "../🛣️ecma-command-routes/🟦️.ts";
import type { Taxonomy } from "../📑️taxonomy-document/🟦️.ts";

export type PackageSourceRole = "declaration" | "registration" | "bootstrap" | "thin-delegation" | "tool-metadata" | "implementation" | "unresolved";

export interface PackageSourceDecision {
  readonly role: PackageSourceRole;
  readonly evidence: string;
}

interface PackageFunctionBody {
  readonly name: string;
  readonly body: string;
  readonly start: number;
  readonly end: number;
}

/** 🫥️ Preserves program structure while blanking comments and literal payloads. */
function packageStructuralSource(content: string, analyzer?: PackageGlueGrammarSpec["analyzer"]): { readonly source: string; readonly balanced: boolean } {
  let source = "", index = 0, balanced = true;
  const blank = (value: string): string => value.replace(/[^\r\n]/gu, " ");
  while (index < content.length) {
    const char = content[index]!, next = content[index + 1];
    if (char === "/" && next === "/") {
      const end = content.indexOf("\n", index + 2), stop = end < 0 ? content.length : end;
      source += blank(content.slice(index, stop)); index = stop; continue;
    }
    if (char === "/" && next === "*") {
      const end = content.indexOf("*/", index + 2), stop = end < 0 ? content.length : end + 2;
      if (end < 0) balanced = false;
      source += blank(content.slice(index, stop)); index = stop; continue;
    }
    if (char === "#" && index === 0 && next === "!" && content[index + 2] !== "[") {
      const end = content.indexOf("\n", index + 2), stop = end < 0 ? content.length : end;
      source += blank(content.slice(index, stop)); index = stop; continue;
    }
    if (char === "#" && analyzer === "python" && (index === 0 || content[index - 1] === "\n")) {
      const end = content.indexOf("\n", index + 1), stop = end < 0 ? content.length : end;
      source += blank(content.slice(index, stop)); index = stop; continue;
    }
    if (char === "'" && analyzer === "rust") {
      const lifetime = /^'[A-Za-z_][A-Za-z0-9_]*/u.exec(content.slice(index));
      if (lifetime && content[index + lifetime[0].length] !== "'") { source += lifetime[0]; index += lifetime[0].length; continue; }
    }
    if (char === '"' || char === "'" || char === "`") {
      const quote = char, raw = quote === "`" && analyzer === "go";
      let end = index + 1, escaped = false;
      for (; end < content.length; end++) {
        const value = content[end]!;
        if (!escaped && value === quote) { end++; break; }
        escaped = !raw && !escaped && value === "\\";
        if (value !== "\\") escaped = false;
      }
      if (end > content.length || content[end - 1] !== quote) balanced = false;
      source += quote + blank(content.slice(index + 1, Math.max(index + 1, end - 1))) + (content[end - 1] === quote ? quote : "");
      index = end; continue;
    }
    source += char; index++;
  }
  const stack: string[] = [], pairs: Readonly<Record<string, string>> = { ")": "(", "]": "[", "}": "{" };
  for (const char of source) {
    if (char === "(" || char === "[" || char === "{") stack.push(char);
    else if (pairs[char] && stack.pop() !== pairs[char]) balanced = false;
  }
  return { source, balanced: balanced && stack.length === 0 };
}

/** 🧱️ Finds brace-delimited functions without treating nested blocks as separate bodies. */
function packageFunctionBodies(source: string, opener: RegExp): readonly PackageFunctionBody[] {
  const rows: PackageFunctionBody[] = [];
  for (const match of source.matchAll(opener)) {
    const brace = (match.index ?? 0) + match[0].lastIndexOf("{");
    let depth = 1, end = brace + 1;
    for (; end < source.length && depth > 0; end++) {
      if (source[end] === "{") depth++;
      else if (source[end] === "}") depth--;
    }
    if (depth === 0) rows.push({ name: match[1] ?? match[2] ?? "", body: source.slice(brace + 1, end - 1), start: match.index ?? 0, end });
  }
  return rows;
}

function packageWithoutRanges(source: string, ranges: readonly { readonly start: number; readonly end: number }[]): string {
  const chars = source.split("");
  for (const range of ranges) for (let index = range.start; index < range.end; index++) if (chars[index] !== "\n") chars[index] = " ";
  return chars.join("");
}

function packageStatements(body: string): readonly string[] {
  const rows: string[] = [];
  let start = 0, depth = 0;
  for (let index = 0; index < body.length; index++) {
    const char = body[index];
    if (char === "(" || char === "[" || char === "{") depth++;
    else if (char === ")" || char === "]" || char === "}") depth--;
    else if ((char === ";" || char === "\n") && depth === 0) {
      if (char === "\n" && /=>\s*$/u.test(body.slice(start, index))) continue;
      const row = body.slice(start, index).trim(); if (row) rows.push(row); start = index + 1;
    }
  }
  const row = body.slice(start).trim(); if (row) rows.push(row);
  return rows;
}

function ecmaImportedBindings(source: string): ReadonlySet<string> {
  const bindings = new Set<string>();
  for (const match of source.matchAll(/\bimport\s+(?:type\s+)?([^;\n]+?)\s+from\s+["']/gu)) {
    const clause = match[1]!.replace(/^\*\s+as\s+/u, "").replace(/[{}]/gu, "");
    for (const part of clause.split(",")) {
      const name = part.trim().split(/\s+as\s+/u).at(-1);
      if (name && /^[A-Za-z_$][\w$]*$/u.test(name)) bindings.add(name);
    }
  }
  return bindings;
}

function ecmaCallRoot(expression: string): string | null {
  const match = /^(?:return\s+)?(?:await\s+)?(?:new\s+)?([A-Za-z_$][\w$]*)(?:\??\.[A-Za-z_$][\w$]*)*\s*\(/u.exec(expression.trim());
  return match?.[1] ?? null;
}

function ecmaDelegatedBody(body: string, imports: ReadonlySet<string>, max: number): boolean {
  const rows = packageStatements(body);
  if (rows.length === 0 || rows.length > max || /\b(?:if|for|while|switch|try|throw|const|let|var|class|function)\b|(?<![=!<>])=(?!=|>)/u.test(body)) return false;
  return rows.every((row) => {
    const jsx = /^(?:return\s+)?<([A-Z][\w$]*)\b[\s\S]*\/?\s*>$/u.exec(row);
    if (jsx) return imports.has(jsx[1]!);
    const root = ecmaCallRoot(row);
    return root !== null && imports.has(root) && !/[+*/%]|\s-\s/u.test(row);
  });
}

/** 📇️ A CommonJS module that only re-exports another module (`module.exports = require(…)` or `Object.assign(module.exports, require(…))`): the require form of an `export … from`. */
function ecmaCommonJsReexport(row: string): boolean {
  return /^(?:module\.exports\s*=\s*|Object\.assign\(\s*module\.exports\s*,\s*)require\(\s*(["'`])[^"'`]*\1\s*\)\s*\)?$/u.test(row.trim());
}

function ecmaDataModule(source: string): boolean {
  const functions = packageFunctionBodies(source, /\b(?:export\s+)?(?:default\s+)?(?:async\s+)?function\s+([A-Za-z_$][\w$]*)[^{}]*\{/gu);
  if (functions.length > 0 || !/\bexport\s+default\b/u.test(source) || /\b(?:class|namespace|enum|if|for|while|switch|try|throw)\b|=>|\bexport\s+(?:const|let|var)\b/u.test(source)) return false;
  const rows = packageStatements(source);
  return rows.length > 0 && rows.every((row) => /^(?:import\b|export\s+(?:type\b|interface\b|\*|\{|default\b)|(?:export\s+)?(?:const|let)\s+[A-Za-z_$][\w$]*(?:\s*:[^=]+)?\s*=)/su.test(row));
}

function classifyEcmaPackageSource(content: string, grammar: PackageGlueGrammarSpec): PackageSourceDecision {
  const structural = packageStructuralSource(content, grammar.analyzer), source = structural.source.trim();
  if (/\b(?:class|namespace|enum)\b/u.test(source)) return { role: "implementation", evidence: "runtime type or namespace declaration" };
  if (/\b(?:export\s+)?(?:declare\s+)?(?:interface|type)\s+[A-Za-z_$][\w$]*/u.test(source.replace(/\b(?:export|import)\s*(?:type\s*)?\{[^}]*\}/gu, ""))) return { role: "implementation", evidence: "domain type declaration is owned inside the package" };
  if (!structural.balanced) return { role: "unresolved", evidence: "unbalanced lexical structure" };
  if (!source) return { role: "declaration", evidence: "trivia-only module" };
  const imports = ecmaImportedBindings(content);
  const functions = packageFunctionBodies(source, /\b(?:export\s+)?(?:default\s+)?(?:async\s+)?function\s+([A-Za-z_$][\w$]*)[^{}]*\{|\b(?:export\s+)?(?:const|let)\s+([A-Za-z_$][\w$]*)[^=;]*=\s*(?:async\s*)?\([^)]*\)\s*=>\s*\{/gu);
  if (functions.some((row) => !ecmaDelegatedBody(row.body, imports, grammar.maxDelegationStatements))) return { role: "implementation", evidence: "function body performs non-delegating work" };
  let rest = packageWithoutRanges(source, functions);
  rest = rest
    .replace(/\bimport\s*[\s\S]*?\s*from\s*["'][\s\S]*?["'](?:\s*with\s*\{[^{}]*\})?\s*;?/gu, "")
    .replace(/\bimport\s*["'][\s\S]*?["'](?:\s*with\s*\{[^{}]*\})?\s*;?/gu, "")
    .replace(/\bexport\s*(?:type(?:\s+|(?=\{))[^;]+|\*\s*(?:as\s+[A-Za-z_$][\w$]*\s+)?from\s*["'][\s\S]*?["'](?:\s*with\s*\{[^{}]*\})?|\{[^}]*\}(?:\s*from\s*["'][\s\S]*?["'](?:\s*with\s*\{[^{}]*\})?)?)\s*;?/gu, "")
    .replace(/\b(?:export\s+)?(?:declare\s+)?type\s+[A-Za-z_$][\w$]*[\s\S]*?;/gu, "")
    .replace(/\b(?:export\s+)?(?:declare\s+)?interface\s+[A-Za-z_$][\w$]*[^{}]*\{[^{}]*\}\s*;?/gu, "")
    .trim();
  if (!rest) return { role: functions.length > 0 ? "thin-delegation" : "declaration", evidence: functions.length > 0 ? "functions only forward to imported owners" : "import, export, or type wiring only" };
  if (ecmaDataModule(source)) return { role: "thin-delegation", evidence: "data-only default export module" };
  const rows = packageStatements(rest.replace(/^export\s+default\s+/gmu, ""));
  if (rows.length > 0 && rows.length <= grammar.maxDelegationStatements && rows.every((row) => {
    if (/^[A-Za-z_$][\w$]*$/u.test(row) || ecmaCommonJsReexport(row)) return true;
    const root = ecmaCallRoot(row);
    return root !== null && imports.has(root) && !/[+*/%]|\s-\s/u.test(row);
  })) {
    const registration = rows.some((row) => /^(?:await\s+)?(?:register|mount|provide|bind)[A-Za-z_$]*\s*\(/iu.test(row));
    return { role: registration ? "registration" : "thin-delegation", evidence: registration ? "registration calls only" : "top-level delegation to imported owners" };
  }
  if (/\b(?:function|const|let|var)\b|=>|(?<![=!<>])=(?!=|>)/u.test(rest)) return { role: "implementation", evidence: "runtime declaration or computation remains" };
  return { role: "unresolved", evidence: "unsupported ECMAScript package form" };
}

function classifyRustPackageSource(content: string, grammar: PackageGlueGrammarSpec): PackageSourceDecision {
  const structural = packageStructuralSource(content, grammar.analyzer), source = structural.source.trim();
  if (/\b(?:struct|enum|trait|union|impl|const|static)\b|\bmacro_rules\s*!/u.test(source)) return { role: "implementation", evidence: "authored Rust item declaration" };
  const aliases = source.replace(/#\[(?:cfg|cfg_attr)\([^\]]*\)\]\s*(?:pub\s+)?type\s+[A-Za-z_]\w*[^;]*;/gu, "");
  if (/\btype\s+[A-Za-z_]\w*/u.test(aliases)) return { role: "implementation", evidence: "domain type alias is owned inside the package" };
  if (!structural.balanced) return { role: "unresolved", evidence: "unbalanced lexical structure" };
  if (!source) return { role: "declaration", evidence: "trivia-only crate" };
  const functions = packageFunctionBodies(source, /\bfn\s+([A-Za-z_]\w*)[^{}]*\{/gu);
  if (functions.length > 0 && (source.match(/#\[proc_macro(?:_attribute|_derive)?(?:\([^\]]*\))?\]/gu) ?? []).length === functions.length && functions.every((row) => !/\b(?:let|if|for|while|match|loop|unsafe)\b|[+*/%]|\s-\s/u.test(row.body))) return { role: "registration", evidence: "compiler registration functions only delegate to semantic owner" };
  const delegated = (body: string): boolean => {
    const rows = packageStatements(body);
    return rows.length > 0 && rows.length <= grammar.maxDelegationStatements && rows.every((row) => /^(?:return\s+)?(?:[A-Za-z_]\w*::)*[A-Za-z_]\w*!?\s*[({][\s\S]*[)}]$/u.test(row) && !/\b(?:let|if|for|while|match|loop|unsafe)\b|[+*/%]|\s-\s/u.test(row));
  };
  if (functions.some((row) => !delegated(row.body))) return { role: "implementation", evidence: "Rust function body performs non-delegating work" };
  let rest = packageWithoutRanges(source, functions)
    .replace(/#!?\[[^\]]*\]/gu, "")
    .replace(/\b(?:pub(?:\([^)]*\))?\s+)?(?:unsafe\s+)?extern\s+crate\s+[^;]+;/gu, "")
    .replace(/\b(?:pub(?:\([^)]*\))?\s+)?use\s+[^;]+;/gu, "")
    .replace(/\b(?:pub(?:\([^)]*\))?\s+)?(?:mod|type)\s+[^;{}]+;/gu, "")
    .replace(/\binclude!?\s*\([^;]+;/gu, "")
    .trim();
  let previous = "";
  while (rest !== previous) {
    previous = rest;
    rest = rest.replace(/\b(?:pub\s+)?mod\s+[A-Za-z_]\w*\s*\{(?:\s*(?:\b(?:pub\s+)?(?:use|mod|type)\s+[^;{}]+;|#!?\[[^\]]*\]))*\s*\}/gu, "").trim();
  }
  const macros = [...rest.matchAll(/(?:^|;)\s*((?:[A-Za-z_]\w*::)*[A-Za-z_]\w*)!\s*[({][\s\S]*[)}]\s*;?/gu)];
  const macroRest = macros.reduce((value, row) => value.replace(row[0], ""), rest).trim();
  if (macroRest) return { role: "unresolved", evidence: "unsupported Rust package form" };
  if (macros.length > 0) {
    const registration = macros.map((row) => {
      const path = row[1]!, body = /!\s*[({]\s*([\s\S]*?)\s*[)}]\s*;?$/u.exec(row[0])?.[1]?.trim() ?? "";
      if (path === "semio_framework_plugin::plugin_exports" || path === "semio_framework_plugin::extension_exports") return /^(?:[A-Za-z_]\w*(?:::[A-Za-z_]\w*)?)(?:\s*,\s*[A-Za-z_]\w*(?:::[A-Za-z_]\w*)?)*$/u.test(body) ? "admitted" : "implementation";
      if (path === "inventory::submit") return /^[A-Za-z_]\w*(?:::[A-Za-z_]\w*)*::new\s*\(\s*&?[A-Za-z_]\w*(?:::[A-Za-z_]\w*)*\s*\)$/u.test(body) ? "admitted" : "implementation";
      return "unresolved";
    });
    if (registration.includes("implementation")) return { role: "implementation", evidence: "registration macro contains an unowned executable body" };
    if (registration.includes("unresolved")) return { role: "unresolved", evidence: "unregistered macro invocation" };
    return { role: "registration", evidence: "registration macro invocations only" };
  }
  if (functions.length > 0) return { role: functions.every((row) => row.name === "main" || row.name === "start" || row.name === "bootstrap") ? "bootstrap" : "thin-delegation", evidence: "Rust functions only delegate to mounted owners" };
  return { role: "declaration", evidence: "module, include, re-export, or type wiring only" };
}

function classifyGoPackageSource(content: string, grammar: PackageGlueGrammarSpec): PackageSourceDecision {
  const structural = packageStructuralSource(content, grammar.analyzer), source = structural.source.trim();
  if (!structural.balanced) return { role: "unresolved", evidence: "unbalanced lexical structure" };
  if (!source) return { role: "unresolved", evidence: "missing Go package declaration" };
  const functions = packageFunctionBodies(source, /\bfunc\s+([A-Za-z_]\w*)\s*\([^)]*\)[^{]*\{/gu);
  const delegated = (body: string): boolean => {
    const rows = packageStatements(body);
    return rows.length > 0 && rows.length <= grammar.maxDelegationStatements && rows.every((row) => /^(?:return\s+)?[A-Za-z_]\w*(?:\.[A-Za-z_]\w*)+\s*\([^{}]*\)$/u.test(row) && !/\b(?:go|defer|if|for|switch|select|var|const)\b|:=|[+*/%]|\s-\s/u.test(row));
  };
  if (functions.some((row) => row.name !== "main" && row.name !== "init" || !delegated(row.body))) return { role: "implementation", evidence: "Go function body performs non-bootstrap work" };
  let rest = packageWithoutRanges(source, functions)
    .replace(/^\s*package\s+[A-Za-z_]\w*/mu, "")
    .replace(/\bimport\s*(?:[A-Za-z_.]\w*\s*)?(?:\([^)]*\)|"[^"]*"|'[^']*')/gu, "")
    .trim();
  if (/\b(?:func|type|var|const)\b/u.test(rest)) return { role: "implementation", evidence: "Go method, type or runtime declaration remains" };
  if (rest) return { role: "unresolved", evidence: "unsupported Go package form" };
  return functions.length > 0 ? { role: "bootstrap", evidence: "main or init only delegates to imported owner" } : { role: "declaration", evidence: "package and import declarations only" };
}

/** 🧾️ Removes unescaped TeX comments while retaining statement boundaries. */
function texStructuralSource(content: string): string {
  let source = "", index = 0;
  while (index < content.length) {
    if (content[index] !== "%") { source += content[index]!; index++; continue; }
    let escapes = 0;
    for (let cursor = index - 1; cursor >= 0 && content[cursor] === "\\"; cursor--) escapes++;
    if (escapes % 2 === 1) { source += content[index]!; index++; continue; }
    const end = content.indexOf("\n", index);
    if (end < 0) break;
    source += "\n"; index = end + 1;
  }
  return source;
}

/** 🪶️ Accepts only a native package identity that delegates to one canonical semantic macro leaf. */
function classifyTexPackageSource(content: string, grammar: PackageGlueGrammarSpec): PackageSourceDecision {
  const source = texStructuralSource(content).trim();
  const match = /^\\NeedsTeXFormat\s*\{\s*LaTeX2e\s*\}\s*\\ProvidesPackage\s*\{\s*([A-Za-z0-9]+(?:-[A-Za-z0-9]+)*)\s*\}(?:\s*\[[A-Za-z0-9][A-Za-z0-9 ./,:;()+-]*\])?\s*\\input\s*\{\s*([^{}]+?)\s*\}\s*(?:\\endinput\s*)?$/u.exec(source);
  if (match) {
    const target = match[2]!, segments = target.split("/");
    const statements = source.match(/\\(?:NeedsTeXFormat|ProvidesPackage|input|endinput)\b/gu)?.length ?? 0;
    const literal = target === target.normalize("NFC") && !/[\u0000-\u0020\u007f\\^%#{}~$&]/u.test(target) && segments.every((segment) => segment !== "." && segment !== ".." && /^[\p{L}\p{N}\p{M}\p{Extended_Pictographic}\p{Regional_Indicator}\u200d._-]+$/u.test(segment));
    if (statements <= grammar.maxDelegationStatements && literal && !target.startsWith("/") && /(?:^|\/)📐️\.tex$/u.test(target)) return { role: "thin-delegation", evidence: "native TeX package identity delegates to one canonical semantic macro leaf" };
  }
  if (/\\(?:def|edef|gdef|xdef|newcommand|renewcommand|providecommand|NewDocumentCommand|RenewDocumentCommand|cs_new|cs_set|RequirePackage)\b/u.test(source)) return { role: "implementation", evidence: "TeX runtime definition or package composition remains in the fixed shim" };
  return { role: "unresolved", evidence: "unsupported TeX package shim form" };
}

/** 🎨️ Strips CSS comments and string/url bodies' delimiters-free text so statement splitting sees only structure; `balanced` is false for an unterminated comment, string or block. */
function cssStructuralSource(content: string): { readonly source: string; readonly balanced: boolean } {
  let out = "", depth = 0;
  for (let index = 0; index < content.length; index++) {
    const char = content[index]!;
    if (char === "/" && content[index + 1] === "*") {
      const end = content.indexOf("*/", index + 2);
      if (end < 0) return { source: out, balanced: false };
      index = end + 1;
      out += " ";
      continue;
    }
    if (char === '"' || char === "'") {
      let end = index + 1;
      while (end < content.length && content[end] !== char) end += content[end] === "\\" ? 2 : 1;
      if (end >= content.length) return { source: out, balanced: false };
      out += content.slice(index, end + 1);
      index = end;
      continue;
    }
    if (char === "{") depth++;
    if (char === "}") depth--;
    out += char;
  }
  return { source: out, balanced: depth === 0 };
}

/** 🎨️ A stylesheet is package glue only when it wires other stylesheets: `@import` of a literal path or url() and the Tailwind `@source` path registration; every rule, declaration or other at-rule is authored implementation. */
function classifyCssPackageSource(content: string, grammar: PackageGlueGrammarSpec): PackageSourceDecision {
  const structural = cssStructuralSource(content), source = structural.source.trim();
  if (!structural.balanced) return { role: "unresolved", evidence: "unbalanced lexical structure" };
  if (!source) return { role: "declaration", evidence: "trivia-only stylesheet" };
  const statements = source.split(";").map((row) => row.trim()).filter(Boolean);
  if (statements.some((row) => row.includes("{") || row.includes("}"))) return { role: "implementation", evidence: "authored stylesheet rule" };
  const wiring = (row: string): boolean => /^@import\s+(?:url\(\s*(["']?)[^"')\s]+\1\s*\)|(["'])[^"']+\2)(?:\s+[^;{}]*)?$/u.test(row) || /^@source\s+(?:not\s+)?(["'])[^"']+\1$/u.test(row);
  if (statements.length <= grammar.maxDelegationStatements && statements.every(wiring)) return { role: "declaration", evidence: "stylesheet import and source wiring only" };
  if (statements.some((row) => !row.startsWith("@"))) return { role: "implementation", evidence: "authored stylesheet declaration" };
  return { role: "unresolved", evidence: "unsupported stylesheet package form" };
}

/** 📚️ Requires the declared grammar at semantic projection boundaries before inspecting source. */
export function requiredPackageGlueGrammar(taxonomy: Taxonomy, id: string): PackageGlueGrammarSpec { const grammar = taxonomy.packageGlueGrammar[id]; if (!grammar) throw new Error("Package source grammar required: " + id); return grammar; }

/** 🧪️ Returns one shared, justified decision for discovery and normalization. */
export function classifyPackageSource(content: string, grammar: PackageGlueGrammarSpec): PackageSourceDecision {
  if (grammar.analyzer === "rust") return classifyRustPackageSource(content, grammar);
  if (grammar.analyzer === "typescript" || grammar.analyzer === "javascript") return classifyEcmaPackageSource(content, grammar);
  if (grammar.analyzer === "go") return classifyGoPackageSource(content, grammar);
  if (grammar.analyzer === "tex") return classifyTexPackageSource(content, grammar);
  if (grammar.analyzer === "css") return classifyCssPackageSource(content, grammar);
  const structural = packageStructuralSource(content, grammar.analyzer), source = structural.source.trim();
  if (!structural.balanced) return { role: "unresolved", evidence: "unbalanced lexical structure" };
  if (!source) return { role: "declaration", evidence: "trivia-only source" };
  if (grammar.analyzer === "python") {
    if (/^(?:\s*(?:from\s+\S+\s+import\s+[^\n]+|import\s+[^\n]+|__all__\s*=\s*\[[^\]]*\])\s*)+$/u.test(source)) return { role: "declaration", evidence: "Python import or export declarations only" };
    return /^(?:async\s+)?def\s|^class\s/mu.test(source) ? { role: "implementation", evidence: "Python function or class declaration" } : { role: "unresolved", evidence: "unsupported Python package form" };
  }
  if (grammar.analyzer === "c-cpp") {
    if (/^\s*#\s*define\s+[A-Za-z_]\w*\s*\([^\n]*\)/mu.test(source)) return { role: "implementation", evidence: "C or C++ function-like macro definition" };
    if (/\b(?:class|struct|union|enum)\s+\w+[^;{]*\{|\w+\s*\([^;{}]*\)\s*\{/u.test(source)) return { role: "implementation", evidence: "C or C++ definition" };
    return /^(?:\s*(?:#\s*(?:include|pragma)\b[^\n]*|(?:using|typedef|extern)\b[^;]*;)\s*)+$/su.test(source) ? { role: "declaration", evidence: "C or C++ declarations only" } : { role: "unresolved", evidence: "unsupported C or C++ package form" };
  }
  if (/\b(?:class|record|struct|interface|enum)\b|\b(?:public|private|protected|internal)\s+(?:static\s+)?\w+[<\w, >]*\s+\w+\s*\(/u.test(source)) return { role: "implementation", evidence: ".NET type or method definition" };
  const rest = source.replace(/^(?:global\s+)?using\s+[^;]+;/gmu, "").replace(/^namespace\s+[\w.]+\s*;?$/gmu, "").trim();
  return rest ? { role: "unresolved", evidence: "unsupported .NET package form" } : { role: "declaration", evidence: ".NET namespace or using declarations only" };
}

function packageToolMetadata(content: string, validator: PackageSourceDisposition["validator"], grammar: PackageGlueGrammarSpec): boolean {
  const structural = packageStructuralSource(content, grammar.analyzer), source = structural.source;
  if (validator === "command-router") {
    return (grammar.analyzer === "typescript" || grammar.analyzer === "javascript") && ecmaCommandRouterModule(content, grammar);
  }
  if (!structural.balanced) return false;
  if (/\bexport\s+(?:interface|type|enum|class|const|let|var|function)\b/u.test(source)) return false;
  if (validator === "pytest-configuration") return /\bpytest\b/u.test(source) && !/\b(?:def|class)\b/u.test(source);
  const configuration = (modulePattern: RegExp): boolean => {
    if (!modulePattern.test(content) || !/\bexport\s+default\s+defineConfig\s*\(/u.test(source) || /\b(?:class|interface|type|enum|namespace)\b/u.test(source) || /\b(?:fetch|writeFile\w*|appendFile\w*|rm\w*|unlink\w*|spawn\w*|exec\w*|listen|connect)\s*\(/u.test(source)) return false;
    const statements = packageStatements(source);
    const declarations = new Map<string, string>();
    const conditionals: string[] = [];
    let root = "";
    for (const statement of statements) {
      if (/^import\b/u.test(statement)) continue;
      if (/^export\s+default\s+defineConfig\s*\(/u.test(statement)) { if (root) return false; root = statement; continue; }
      const declaration = /^(?:const\s+([A-Za-z_$][\w$]*)|function\s+([A-Za-z_$][\w$]*))/u.exec(statement);
      if (declaration) { declarations.set(declaration[1] ?? declaration[2]!, statement); continue; }
      if (/^if\s*\([\s\S]*\)\s*\{\s*[A-Za-z_$][\w$]*\.push\s*\([\s\S]*\)\s*;?\s*\}$/u.test(statement)) { conditionals.push(statement); continue; }
      return false;
    }
    if (!root) return false;
    const referenced = new Set<string>(), identifiers = (value: string): readonly string[] => [...value.matchAll(/\b[A-Za-z_$][\w$]*\b/gu)].map((row) => row[0]);
    for (const name of identifiers(root)) if (declarations.has(name)) referenced.add(name);
    for (let size = -1; size !== referenced.size;) {
      size = referenced.size;
      for (const name of [...referenced]) for (const dependency of identifiers(declarations.get(name)!)) if (declarations.has(dependency)) referenced.add(dependency);
    }
    if ([...declarations.keys()].some((name) => !referenced.has(name))) return false;
    return conditionals.every((statement) => {
      const target = /^if\s*\([\s\S]*\)\s*\{\s*([A-Za-z_$][\w$]*)\.push/u.exec(statement)?.[1];
      return target !== undefined && referenced.has(target);
    });
  };
  if (validator === "eslint-configuration") return ecmaDataModule(source) && /\beslint\b/iu.test(content);
  if (validator === "vitest-configuration" || validator === "tool-config-vitest") return configuration(/from\s+["']vitest\/config["']/u);
  const token: Readonly<Record<string, RegExp>> = {
    "tool-config-tailwind": /\b(?:tailwind|content|theme|plugins)\b/iu,
    "tool-config-postcss": /\b(?:postcss|plugins)\b/iu,
    "tool-config-eslint": /\beslint\b/iu,
    "tool-config-dependency-cruiser": /\b(?:dependency|forbidden|options)\b/iu,
  };
  return ecmaDataModule(source) && (token[validator]?.test(content) ?? false);
}

/** 💊️ Classifies an explicitly dispositioned source-format fixed/configurable package entry. */
export function classifyPackageSourceDisposition(content: string, disposition: PackageSourceDisposition, grammar: PackageGlueGrammarSpec): PackageSourceRole {
  if (disposition.validator === "package-glue") return classifyPackageSource(content, grammar).role;
  return packageToolMetadata(content, disposition.validator, grammar) ? "tool-metadata" : "unresolved";
}

export interface FixedSourceDispositionDecision {
  readonly contractId: string;
  readonly role: PackageSourceRole | null;
  readonly finding: "fixed-source-content-unreadable" | "fixed-source-disposition-unresolved" | null;
}
