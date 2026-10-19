import { observedFs, observedRequireResolve, registerSourceAnalysisV1, observedSourceAnalysisV1, observedSourceAnalysisAsyncV1 } from "../../🏗️build/👁️observation/🟨️.mjs";
import { dirname, isAbsolute, relative, resolve } from "node:path";
import { createRequire, isBuiltin } from "node:module";
import { createHash } from "node:crypto";
import { fileURLToPath } from "node:url";

const { existsSync, readFileSync, realpathSync } = observedFs;

const require = createRequire(import.meta.url);
const pathPattern = /^(?![/\\])(?![A-Za-z]:)(?!.*(?:^|\/)\.\.(?:\/|$))(?!.*\\).+$/;
const packagePattern = /^(?:@[^/]+\/)?[^/]+$/;
let importProvider;

/** 🔏️ Binds declared source semantics to the current parser, schema and compiler bytes. */
function sourceImportProviderV1() {
  if (!importProvider) {
    const compiler = observedRequireResolve(fileURLToPath(import.meta.url), "typescript");
    const revision = createHash("sha256").update(readFileSync(fileURLToPath(import.meta.url))).update(readFileSync(new URL("./🔣️schema.json", import.meta.url))).update(readFileSync(compiler)).digest("hex");
    importProvider = registerSourceAnalysisV1({ id: "build:declared-source-imports:v1", revision, analyze: parseRuntimeImports });
  }
  return importProvider;
}

/** 🔣️ Validates the owned source-input contract without exposing compiler types. */
export function sourceInputContract(value) {
  const keys = ["entries", "generated", "externalDependencies"];
  if (!value || typeof value !== "object" || Array.isArray(value) || Object.keys(value).some(key => !keys.includes(key))) throw new Error("Invalid source input contract");
  for (const key of keys) {
    const pattern = key === "externalDependencies" ? packagePattern : pathPattern;
    if (!Array.isArray(value[key]) || (key === "entries" && !value[key].length) || value[key].some(item => typeof item !== "string" || !pattern.test(item)) || new Set(value[key]).size !== value[key].length) throw new Error(`Invalid source input ${key}`);
  }
  return value;
}

/** 📥️ Reads a schema-owned module graph declaration. */
export function readSourceInputContract(path) {
  return sourceInputContract(JSON.parse(readFileSync(path, "utf8")));
}

/** 🔗️ Collects module imports conservatively while excluding explicit type-only declarations. */
function parseRuntimeImports(path, bytes) {
  const compiler = require("typescript"), text = bytes.toString("utf8");
  const tree = compiler.createSourceFile(path, text, compiler.ScriptTarget.Latest, true);
  const imports = new Set(), loaders = new Set(["require"]);
  const factory = node => compiler.isCallExpression(node) && compiler.isIdentifier(node.expression) && node.expression.text === "createRequire";
  const declarations = node => {
    if (compiler.isVariableDeclaration(node) && compiler.isIdentifier(node.name) && node.initializer && factory(node.initializer)) loaders.add(node.name.text);
    compiler.forEachChild(node, declarations);
  };
  declarations(tree);
  const add = node => {
    if (!node || !compiler.isStringLiteralLike(node)) throw new Error(`Source imports must be literal in ${path}`);
    imports.add(node.text);
  };
  const visit = node => {
    if (compiler.isImportTypeNode(node)) return;
    if (compiler.isImportDeclaration(node)) { if (!node.importClause?.isTypeOnly) add(node.moduleSpecifier); return; }
    if (compiler.isExportDeclaration(node)) { if (!node.isTypeOnly && node.moduleSpecifier) add(node.moduleSpecifier); return; }
    if (compiler.isImportEqualsDeclaration(node)) { if (!node.isTypeOnly && compiler.isExternalModuleReference(node.moduleReference)) add(node.moduleReference.expression); return; }
    if (compiler.isCallExpression(node)) {
      const expression = node.expression;
      if (expression.kind === compiler.SyntaxKind.ImportKeyword || compiler.isIdentifier(expression) && loaders.has(expression.text) || factory(expression) || compiler.isPropertyAccessExpression(expression) && compiler.isIdentifier(expression.expression) && loaders.has(expression.expression.text) && expression.name.text === "resolve") add(node.arguments[0]);
    }
    compiler.forEachChild(node, visit);
  };
  visit(tree);
  return [...imports];
}

/** 📖️ Reuses only a content-addressed import image while reading the current source bytes. */
export function sourceRuntimeImportsV1(path) {
  return /\.(?:json|d\.[cm]?ts)$/.test(path) ? [] : observedSourceAnalysisV1(sourceImportProviderV1(), path);
}

/** 📦️ Parses current admitted bytes without performing another physical Source read. */
export function sourceRuntimeImportsFromBytesV1(path, bytes) {
  if (typeof path !== "string" || !(bytes instanceof Uint8Array)) throw new Error("Source path and admitted bytes required");
  return /\.(?:json|d\.[cm]?ts)$/.test(path) ? [] : parseRuntimeImports(path, Buffer.from(bytes.buffer, bytes.byteOffset, bytes.byteLength));
}

/** 🌊️ Reads each source asynchronously before admitting its registered import image. */
async function runtimeImportsAsyncV1(path) {
  return /\.(?:json|d\.[cm]?ts)$/.test(path) ? [] : observedSourceAnalysisAsyncV1(sourceImportProviderV1(), path);
}

function inside(workspace, path) {
  const local = relative(workspace, path);
  if (isAbsolute(local) || local === ".." || local.startsWith("../") || local.startsWith("..\\")) throw new Error(`Source import escapes workspace: ${path}`);
  if (existsSync(path)) {
    const physical = relative(realpathSync(workspace), realpathSync(path));
    if (isAbsolute(physical) || physical === ".." || physical.startsWith("../") || physical.startsWith("..\\")) throw new Error(`Source import escapes workspace through a link: ${path}`);
  }
  return path;
}

function generatedSource(path, required) {
  if (!existsSync(path)) { if (required) throw new Error(`Generated source is missing: ${path}`); return; }
  if (sourceRuntimeImportsV1(path).length) throw new Error(`Generated source boundary contains runtime imports: ${path}`);
}

/** 🧱️ Ensures generated data boundaries remain self-contained before the compiler consumes them. */
export function assertGeneratedSources(value, workspace) {
  const contract = sourceInputContract(value);
  for (const path of contract.generated) generatedSource(inside(workspace, resolve(workspace, path)), true);
}

/** 🕸️ Resolves relative module sources; generated leaves are hashed from their producer's outputs. */
export function relativeSourceInputs(value, workspace) {
  const contract = sourceInputContract(value), generated = new Set(contract.generated.map(path => inside(workspace, resolve(workspace, path)))), files = new Set();
  for (const path of generated) generatedSource(path, false);
  const visit = path => {
    inside(workspace, path);
    if (generated.has(path) || files.has(path)) return;
    files.add(path);
    for (const specifier of sourceRuntimeImportsV1(path)) {
      if (isBuiltin(specifier)) continue;
      if (!specifier.startsWith(".")) {
        const name = specifier.startsWith("@") ? specifier.split("/").slice(0, 2).join("/") : specifier.split("/")[0];
        if (!contract.externalDependencies.includes(name)) throw new Error(`Undeclared external source import ${specifier} in ${path}`);
        continue;
      }
      const candidate = inside(workspace, resolve(dirname(path), specifier));
      visit(generated.has(candidate) ? candidate : observedRequireResolve(path, specifier));
    }
  };
  for (const path of contract.entries) visit(resolve(workspace, path));
  return { files: [...files].sort(), externalDependencies: [...contract.externalDependencies] };
}

/** ⏳️ Traverses bounded source batches through the caller's original progress and cancellation control.
 * @param {unknown} value
 * @param {string} workspace
 * @param {(index: number) => void | Promise<void>} [advance]
 */
export async function relativeSourceInputsAsyncV1(value, workspace, advance = async () => {}) {
  const contract = sourceInputContract(value), generated = new Set(contract.generated.map(path => inside(workspace, resolve(workspace, path)))), files = new Set();
  let index = 0;
  const checkpoint = async () => { if (index++ % 16 === 0) await advance(index); };
  await checkpoint();
  for (const path of generated) {
    await checkpoint();
    if (existsSync(path) && (await runtimeImportsAsyncV1(path)).length) throw new Error(`Generated source boundary contains runtime imports: ${path}`);
  }
  const pending = contract.entries.map(path => resolve(workspace, path));
  while (pending.length) {
    await checkpoint();
    const path = inside(workspace, pending.pop());
    if (generated.has(path) || files.has(path)) continue;
    files.add(path);
    for (const specifier of await runtimeImportsAsyncV1(path)) {
      if (isBuiltin(specifier)) continue;
      if (!specifier.startsWith(".")) {
        const name = specifier.startsWith("@") ? specifier.split("/").slice(0, 2).join("/") : specifier.split("/")[0];
        if (!contract.externalDependencies.includes(name)) throw new Error(`Undeclared external source import ${specifier} in ${path}`);
        continue;
      }
      const candidate = inside(workspace, resolve(dirname(path), specifier));
      pending.push(generated.has(candidate) ? candidate : observedRequireResolve(path, specifier));
    }
  }
  await advance(index);
  return { files: [...files].sort(), externalDependencies: [...contract.externalDependencies] };
}
