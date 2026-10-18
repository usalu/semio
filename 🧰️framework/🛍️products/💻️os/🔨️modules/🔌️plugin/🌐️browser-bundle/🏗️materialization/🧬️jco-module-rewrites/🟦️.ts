/** 🧬️ Jco module rewrites of the plugin web materialization: the async result lifting and the versioned host shim and core asset urls of a freshly transpiled jco module. */
import { readFileSync, writeFileSync } from "node:fs";
import ts from "typescript";
import { PLUGIN_HOST_SHIM_FILE } from "../🧾️contract/🟦️.ts";

//#region 🧬️JcoAsyncResultLifting
const JCO_INDIRECT_RESULT_MEMORY_GUARD = "if (!ctx.memory) {\n      _debugLog('missing memory despite indirect param usage'";
const JCO_RESOLVED_RESULT_MEMORY_GUARD = "if (!memory) {\n      _debugLog('missing memory despite indirect param usage'";
const JCO_TASK_RETURN_DIRECT_VALUES = 16;

function jcoResultField(node: ts.Node | undefined, name: string): ts.Expression {
  if (node && ts.isObjectLiteralExpression(node)) {
    for (const field of node.properties) {
      if (ts.isPropertyAssignment(field) && ((ts.isIdentifier(field.name) || ts.isStringLiteral(field.name)) && field.name.text === name)) return field.initializer;
    }
  }
  throw new Error(`jco task-return metadata is missing ${name}`);
}

function jcoResultEntries(node: ts.Node | undefined): readonly ts.Expression[] {
  if (node && ts.isArrayLiteralExpression(node)) return node.elements;
  throw new Error("jco task-return metadata must be an array");
}

function jcoResultFlatCount(node: ts.Expression): number {
  if (node.kind === ts.SyntaxKind.NullKeyword) return 0;
  if (ts.isIdentifier(node)) {
    if (/^_liftFlatString/.test(node.text)) return 2;
    if (/^_liftFlat(Bool|Char|[SU](8|16|32|64)|Float(32|64))$/.test(node.text)) return 1;
  }
  if (ts.isCallExpression(node) && ts.isIdentifier(node.expression)) {
    const meta = node.arguments[0];
    switch (node.expression.text) {
      case "_liftFlatList": return 2;
      case "_liftFlatEnum":
      case "_liftFlatOwn":
      case "_liftFlatBorrow": return 1;
      case "_liftFlatFlags": return Math.ceil(Number(jcoResultField(meta, "size32").getText()) / 4);
      case "_liftFlatRecord": return jcoResultEntries(jcoResultField(meta, "fieldMetas")).reduce((sum, field) => sum + jcoResultFlatCount(jcoResultEntries(field)[1]!), 0);
      case "_liftFlatTuple": return jcoResultEntries(jcoResultField(meta, "elemLiftFns")).reduce((sum, field) => sum + jcoResultFlatCount(jcoResultEntries(field)[0]!), 0);
      case "_liftFlatVariant":
      case "_liftFlatOption":
      case "_liftFlatResult": return 1 + Math.max(0, ...jcoResultEntries(jcoResultField(meta, "caseMetas")).map((field) => jcoResultFlatCount(jcoResultEntries(field)[1]!)));
    }
  }
  throw new Error(`unsupported jco task-return lift: ${node.getText()}`);
}

/** 🧬️ Derives callback directness from each generated result's canonical flattened shape; memory presence alone does not distinguish direct pointer/length values from an indirect return record. */
export function rewriteJcoAsyncResultLifting(source: string): string {
  source = source.replace(JCO_INDIRECT_RESULT_MEMORY_GUARD, JCO_RESOLVED_RESULT_MEMORY_GUARD);
  const parsed = ts.createSourceFile("component.js", source, ts.ScriptTarget.Latest, true, ts.ScriptKind.JS);
  const edits: { start: number; end: number; value: string }[] = [];
  const visit = (node: ts.Node): void => {
    if (ts.isCallExpression(node) && ts.isPropertyAccessExpression(node.expression) && ts.isIdentifier(node.expression.expression) && node.expression.expression.text === "taskReturn" && node.expression.name.text === "bind") {
      const context = node.arguments[node.arguments.length - 1];
      const count = jcoResultEntries(jcoResultField(context, "liftFns")).reduce((sum, field) => sum + jcoResultFlatCount(field), 0);
      const direct = jcoResultField(context, "useDirectParams");
      edits.push({ start: direct.getStart(parsed), end: direct.end, value: String(count <= JCO_TASK_RETURN_DIRECT_VALUES) });
    }
    ts.forEachChild(node, visit);
  };
  visit(parsed);
  for (const edit of edits.sort((left, right) => right.start - left.start)) source = source.slice(0, edit.start) + edit.value + source.slice(edit.end);
  return source;
}

/** 💾️ Applies {@link rewriteJcoAsyncResultLifting} to one freshly transpiled jco module. */
export function rewriteJcoAsyncResultLiftingAt(modulePath: string): void {
  const source = readFileSync(modulePath, "utf8");
  const rewritten = rewriteJcoAsyncResultLifting(source);
  if (rewritten !== source) writeFileSync(modulePath, rewritten);
}
//#endregion 🧬️JcoAsyncResultLifting

//#region 🧊️JcoComponentAssetVersioning
const JCO_HOST_SHIM_URL_HELPER = `function __semioActivationHostUrl() {
  const url = new URL("./🟨️.js", import.meta.url);
  const source = new URL(import.meta.url);
  for (const key of ["actor", "activation", "v"]) {
    const value = source.searchParams.get(key);
    if (value !== null) url.searchParams.set(key, value);
  }
  return url;
}`;

function rewriteJcoHostShimImports(source: string): string {
  const parsed = ts.createSourceFile("component.js", source, ts.ScriptTarget.Latest, true, ts.ScriptKind.JS);
  const edits: { start: number; end: number; value: string }[] = [];
  for (const statement of parsed.statements) {
    if (!ts.isImportDeclaration(statement) || !ts.isStringLiteral(statement.moduleSpecifier) || statement.moduleSpecifier.text !== `./${PLUGIN_HOST_SHIM_FILE}`) continue;
    const bindings = statement.importClause?.namedBindings;
    if (statement.importClause?.name || !bindings || !ts.isNamedImports(bindings)) throw new Error("unsupported jco host shim import shape");
    const fields = bindings.elements.map((field) => field.propertyName ? `${field.propertyName.getText(parsed)}: ${field.name.text}` : field.name.text).join(", ");
    edits.push({ start: statement.getStart(parsed), end: statement.end, value: `const { ${fields} } = await import(__semioActivationHostUrl().href);` });
  }
  for (const edit of edits.reverse()) source = source.slice(0, edit.start) + edit.value + source.slice(edit.end);
  return edits.length === 0 ? source : `${JCO_HOST_SHIM_URL_HELPER}\n\n${source}`;
}

const JCO_COMPONENT_ASSET_URL = /new URL\((['"])(\.\/[^'"]+\.core\d*\.wasm)\1,\s*import\.meta\.url\)/g;
const JCO_COMPONENT_ASSET_URL_HELPER = `function __semioVersionedComponentAssetUrl(path) {
  const url = new URL(path, import.meta.url);
  const rebuildVersion = new URL(import.meta.url).searchParams.get("v");
  if (rebuildVersion) url.searchParams.set("v", rebuildVersion);
  return url;
}`;

/** 🪪️ Preserves activation identity for host imports and rebuild identity for extracted core Wasm. */
export function rewriteJcoComponentAssetUrls(source: string): string {
  source = rewriteJcoHostShimImports(source);
  if (source.includes("function __semioVersionedComponentAssetUrl(path)")) return source;
  const rewritten = source.replace(JCO_COMPONENT_ASSET_URL, (_match, quote, assetPath) => `__semioVersionedComponentAssetUrl(${quote}${assetPath}${quote})`);
  return rewritten === source ? source : `${JCO_COMPONENT_ASSET_URL_HELPER}\n\n${rewritten}`;
}

/** 💾️ Applies {@link rewriteJcoComponentAssetUrls} to one freshly transpiled jco module. */
export function rewriteJcoComponentAssetUrlsAt(modulePath: string): void {
  const source = readFileSync(modulePath, "utf8");
  const rewritten = rewriteJcoComponentAssetUrls(source);
  if (rewritten !== source) writeFileSync(modulePath, rewritten);
}
//#endregion 🧊️JcoComponentAssetVersioning
