import { createHash } from "node:crypto";
import { createRequire } from "node:module";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
const require = createRequire(import.meta.url);
/** 🧬️ Identifies immutable parser implementation bytes independently of any workspace data. */
export const sourceHash = createHash("sha256").update(readFileSync(fileURLToPath(import.meta.url))).digest("hex");
const facts = new Map(), limit = 16 * 1024 * 1024;
let bytes = 0, hits = 0, misses = 0;

/** 🔗️ Returns immutable content-derived command facts behind an owned string interface.
 * @param {string} path
 * @param {string} source
 * @returns {string[]}
 */
export function commandSourceImports(path, source) {
  const key = createHash("sha256").update(JSON.stringify([path, source])).digest("hex"), previous = facts.get(key);
  if (previous) { hits++; facts.delete(key); facts.set(key, previous); return [...previous.imports]; }
  misses++;
  const imports = new Set();
  if (!/\.(?:json|d\.[cm]?ts)$/.test(path)) {
    const compiler = require("typescript"), add = node => { if (node && compiler.isStringLiteralLike(node) && node.text.startsWith(".")) imports.add(node.text); };
    const visit = node => {
      if (compiler.isImportTypeNode(node)) return;
      if (compiler.isImportDeclaration(node)) {
        const clause = node.importClause, bindings = clause?.namedBindings;
        if (!clause?.isTypeOnly && !(bindings && compiler.isNamedImports(bindings) && !clause.name && bindings.elements.length && bindings.elements.every(element => element.isTypeOnly))) add(node.moduleSpecifier);
        return;
      }
      if (compiler.isExportDeclaration(node)) { if (!node.isTypeOnly) add(node.moduleSpecifier); return; }
      if (compiler.isImportEqualsDeclaration(node)) { if (!node.isTypeOnly && compiler.isExternalModuleReference(node.moduleReference)) add(node.moduleReference.expression); return; }
      if (compiler.isCallExpression(node)) {
        const expression = node.expression;
        if (expression.kind === compiler.SyntaxKind.ImportKeyword || compiler.isIdentifier(expression) && expression.text === "require" || compiler.isCallExpression(expression) && compiler.isIdentifier(expression.expression) && expression.expression.text === "createRequire") add(node.arguments[0]);
      }
      compiler.forEachChild(node, visit);
    };
    visit(compiler.createSourceFile(path, source, compiler.ScriptTarget.Latest, false));
  }
  const result = [...imports], size = Buffer.byteLength(key) + Buffer.byteLength(JSON.stringify(result)) + 192;
  if (size <= limit) {
    while (bytes + size > limit) { const oldest = facts.keys().next().value; bytes -= facts.get(oldest).bytes; facts.delete(oldest); }
    facts.set(key, { imports: result, bytes: size }); bytes += size;
  }
  return [...result];
}


/** 📊️ Reports bounded immutable parser fact storage without source authority. */
export function commandSourceParseStats() { return { hits, misses, bytes, limit }; }
