import { createHash } from "node:crypto";
import { parserTypescriptV1 } from "../../🏗️build/🚀️bootstrap/📦️parsers/🟨️.mjs";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
/** 🧬️ Identifies immutable parser implementation bytes independently of any workspace data. */
export const sourceHash = createHash("sha256").update(readFileSync(fileURLToPath(import.meta.url))).digest("hex");
const facts = new Map(), limit = 16 * 1024 * 1024;
let bytes = 0, hits = 0, misses = 0;

/** 🔗️ Returns immutable content-derived runtime module facts behind an owned string interface.
 * @param {string} path
 * @param {string} source
 * @returns {string[]}
 */
function sourceImports(path, source, scope = "all") {
  const key = createHash("sha256").update(JSON.stringify([path, source, scope])).digest("hex"), previous = facts.get(key);
  if (previous) { hits++; facts.delete(key); facts.set(key, previous); return [...previous.imports]; }
  misses++;
  const imports = new Set();
  if (!/\.json$/.test(path) && !(scope === "command" && /\.d\.[cm]?ts$/.test(path))) {
    const compiler = parserTypescriptV1(), add = node => { if (node && compiler.isStringLiteralLike(node)) imports.add(node.text); };
    const visit = node => {
      if (scope !== "all" && compiler.isIfStatement(node) && compiler.isPropertyAccessExpression(node.expression) && node.expression.name.text === "vitest" && compiler.isMetaProperty(node.expression.expression) && node.expression.expression.keywordToken === compiler.SyntaxKind.ImportKeyword) { if (node.elseStatement) visit(node.elseStatement); return; }
      if (compiler.isImportTypeNode(node)) { if(scope !== "command" && compiler.isLiteralTypeNode(node.argument))add(node.argument.literal); return; }
      if (compiler.isImportDeclaration(node)) {
        const clause = node.importClause, bindings = clause?.namedBindings;
        if (scope !== "command" || !clause?.isTypeOnly && !(bindings && compiler.isNamedImports(bindings) && !clause.name && bindings.elements.length && bindings.elements.every(element => element.isTypeOnly))) add(node.moduleSpecifier);
        return;
      }
      if (compiler.isExportDeclaration(node)) { const clause = node.exportClause; if (scope !== "command" || !node.isTypeOnly && !(clause && compiler.isNamedExports(clause) && clause.elements.length && clause.elements.every(element => element.isTypeOnly))) add(node.moduleSpecifier); return; }
      if (compiler.isImportEqualsDeclaration(node)) { if ((scope !== "command" || !node.isTypeOnly) && compiler.isExternalModuleReference(node.moduleReference)) add(node.moduleReference.expression); return; }
      if (compiler.isCallExpression(node)) {
        const expression = node.expression;
        if (expression.kind === compiler.SyntaxKind.ImportKeyword || compiler.isIdentifier(expression) && expression.text === "require" || compiler.isCallExpression(expression) && compiler.isIdentifier(expression.expression) && expression.expression.text === "createRequire") add(node.arguments[0]);
      }
      compiler.forEachChild(node, visit);
    };
    visit(compiler.createSourceFile(path, source, compiler.ScriptTarget.Latest, false));
  }
  const result = Object.freeze([...imports]), size = Buffer.byteLength(key) + Buffer.byteLength(JSON.stringify(result)) + 192;
  if (size <= limit) {
    while (bytes + size > limit) { const oldest = facts.keys().next().value; bytes -= facts.get(oldest).bytes; facts.delete(oldest); }
    facts.set(key, { imports: result, bytes: size }); bytes += size;
  }
  return [...result];
}


/** 🧭️ Projects canonical runtime module facts onto relative command source imports. */
export function commandSourceImports(path, source) { return sourceImports(path, source, "command").filter(specifier => specifier.startsWith(".")); }

/** 🚀️ Retains runtime package and relative imports without erased declaration dependencies. */
export function runtimeModuleSourceImports(path, source) { return sourceImports(path, source, "command"); }

/** 🔗️ Projects runtime and optional test module facts for project dependency ownership. */
export function moduleSourceImports(path, source) { return sourceImports(path, source); }

/** 🏗️ Keeps typed and runtime production imports outside command and test ownership. */
export function productionModuleSourceImports(path, source) {
  if(path.replaceAll("\\", "/").split("/").some(segment=>segment === "📜️script.ts" || segment === "🧪️tests") || /(?:^|\/)vitest\.config\.[cm]?[jt]s$/.test(path))return [];
  return sourceImports(path,source,"production");
}

/** 🔗️ Retains the literal paths a script resolves against itself with `new URL("…", import.meta.url)`, which name the test files it runs and the data it reads. */
export function commandUrlReferences(path, source) {
  if (!/new\s+URL\s*\(|\b(?:join|resolve)\s*\(/u.test(source) || !/import\.meta\.(?:url|dir|dirname)\b/u.test(source) || /\.json$|\.d\.[cm]?ts$/u.test(path)) return [];
  const key = createHash("sha256").update(JSON.stringify([path, source, "urls-nested"])).digest("hex"), previous = facts.get(key);
  if (previous) { hits++; facts.delete(key); facts.set(key, previous); return [...previous.imports]; }
  misses++;
  const compiler = parserTypescriptV1(), urls = new Set(), isImportMetaUrl = node => compiler.isPropertyAccessExpression(node) && node.name.text === "url" && compiler.isMetaProperty(node.expression) && node.expression.keywordToken === compiler.SyntaxKind.ImportKeyword, mentionsImportMetaUrl = node => isImportMetaUrl(node) || compiler.forEachChild(node, child => mentionsImportMetaUrl(child) || undefined) === true, calleeName = node => compiler.isIdentifier(node) ? node.text : compiler.isPropertyAccessExpression(node) ? node.name.text : "", isJoin = node => ["join", "resolve"].includes(calleeName(node)), isModuleDirectory = node => compiler.isPropertyAccessExpression(node) && compiler.isMetaProperty(node.expression) && node.expression.keywordToken === compiler.SyntaxKind.ImportKeyword && ["dir", "dirname"].includes(node.name.text) || compiler.isCallExpression(node) && calleeName(node.expression) === "dirname" && node.arguments.length === 1 && compiler.isCallExpression(node.arguments[0]) && calleeName(node.arguments[0].expression) === "fileURLToPath" && node.arguments[0].arguments.length === 1 && isImportMetaUrl(node.arguments[0].arguments[0]), visit = node => {
    if (compiler.isNewExpression(node) && compiler.isIdentifier(node.expression) && node.expression.text === "URL" && node.arguments?.length >= 2 && compiler.isStringLiteralLike(node.arguments[0]) && mentionsImportMetaUrl(node.arguments[1])) urls.add(node.arguments[0].text);
    else if (compiler.isCallExpression(node) && isJoin(node.expression) && node.arguments.length >= 2 && isModuleDirectory(node.arguments[0]) && node.arguments.slice(1).every(compiler.isStringLiteralLike)) urls.add(`./${node.arguments.slice(1).map(argument => argument.text).join("/")}`);
    compiler.forEachChild(node, visit);
  };
  visit(compiler.createSourceFile(path, source, compiler.ScriptTarget.Latest, false));
  const result = Object.freeze([...urls]), size = Buffer.byteLength(key) + Buffer.byteLength(JSON.stringify(result)) + 192;
  if (size <= limit) {
    while (bytes + size > limit) { const oldest = facts.keys().next().value; bytes -= facts.get(oldest).bytes; facts.delete(oldest); }
    facts.set(key, { imports: result, bytes: size }); bytes += size;
  }
  return [...result];
}

/** 📊️ Reports bounded immutable parser fact storage without source authority. */
export function commandSourceParseStats() { return { hits, misses, bytes, limit }; }
