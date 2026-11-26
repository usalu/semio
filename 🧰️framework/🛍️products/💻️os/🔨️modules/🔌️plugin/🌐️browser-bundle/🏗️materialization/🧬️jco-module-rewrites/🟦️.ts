/** 🧬️ Jco module rewrites of the plugin web materialization: the async result lifting and the WASI import object of a freshly transpiled jco instantiation factory. */
import { readFileSync, writeFileSync } from "node:fs";
import ts from "typescript";

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

//#region 🏭️JcoComponentFactory
const JCO_FACTORY_EXPORT = "export function instantiate(getCoreModule, imports, instantiateCore, options)";
const JCO_FACTORY_WASI_IMPORT = /imports\[(['"])(wasi:([a-z][a-z0-9-]*)\/([a-z][a-z0-9-]*)(?:@[0-9A-Za-z.+-]+)?)\1\]/g;
const JCO_FACTORY_WASI_EXPORT = "export const wasiImports = ";
const camel = (name: string): string => name.replace(/-([a-z0-9])/g, (_match, letter: string) => letter.toUpperCase());

/** 🧩️ Identifies the pinned jco browser instantiation entry. */
export function isBrowserCoreFactory(source: string): boolean {
  return source.includes(JCO_FACTORY_EXPORT);
}

/** 🏭️ Completes a jco instantiation factory (`--instantiation async`) for the browser: the factory asks its caller for every import, so the module imports the WASI interfaces it names from the preview2 shim (relocated to the vendor directory by `rewritePreview2ShimImports`) and exports them as `wasiImports`, the import object the bridge extends with the activation's own host. The rewrite is idempotent and refuses a module that is not a factory, since only a factory keeps instance state out of module scope. */
export function rewriteJcoComponentFactory(source: string): string {
  if (!isBrowserCoreFactory(source)) throw new Error("jco component module is not an instantiation factory: transpile with --instantiation async");
  if (source.includes(JCO_FACTORY_WASI_EXPORT)) return source;
  const interfaces = new Map<string, { readonly packageName: string; readonly member: string }>();
  for (const match of source.matchAll(JCO_FACTORY_WASI_IMPORT)) interfaces.set(match[2]!, { packageName: match[3]!, member: camel(match[4]!) });
  const packages = new Map<string, Set<string>>();
  for (const { packageName, member } of interfaces.values()) packages.set(packageName, (packages.get(packageName) ?? new Set()).add(member));
  const local = (packageName: string, member: string): string => `__wasi_${camel(packageName)}_${member}`;
  const imports = [...packages].sort(([left], [right]) => left.localeCompare(right)).map(([packageName, members]) => `import { ${[...members].sort().map((member) => `${member} as ${local(packageName, member)}`).join(", ")} } from '@bytecodealliance/preview2-shim/${packageName}';`);
  const entries = [...interfaces].sort(([left], [right]) => left.localeCompare(right)).map(([specifier, { packageName, member }]) => `${JSON.stringify(specifier)}: ${local(packageName, member)}`);
  return `${imports.join("\n")}${imports.length > 0 ? "\n" : ""}${source}\n${JCO_FACTORY_WASI_EXPORT}Object.freeze({ ${entries.join(", ")} });\n`;
}

/** 💾️ Applies {@link rewriteJcoComponentFactory} to one freshly transpiled jco module. */
export function rewriteJcoComponentFactoryAt(modulePath: string): void {
  const source = readFileSync(modulePath, "utf8");
  const rewritten = rewriteJcoComponentFactory(source);
  if (rewritten !== source) writeFileSync(modulePath, rewritten);
}
//#endregion 🏭️JcoComponentFactory
