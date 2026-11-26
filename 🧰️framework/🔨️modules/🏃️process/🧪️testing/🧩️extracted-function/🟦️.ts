import { createRequire } from "node:module";
import { refuseProcessV1 } from "../../📖️texts/🟦️.ts";

/** 🧩️ Evaluates one transpiled production function against explicit ports; every other free name it reaches resolves to the global scope or to a pass-through stub (it calls the last function argument it receives), so an unrelated new dependency of the function cannot break the law that the test states. */
export function evaluateExtractedFunctionV1(code: string, name: string, ports: Record<string, unknown>): any {
  ports = { __importMeta: { url: "file:///extracted-function/", dir: "/extracted-function", dirname: "/extracted-function" }, ...ports };
  const pass = (...values: unknown[]): unknown => { const body = [...values].reverse().find(value => typeof value === "function") as (() => unknown) | undefined; return body ? body() : undefined; };
  const scope = new Proxy(ports, {
    has: (_target, key) => typeof key === "string",
    get: (target, key) => { if (typeof key !== "string") return undefined; if (key in target) return target[key]; if (key in globalThis) return (globalThis as Record<string, unknown>)[key]; return pass; },
    set: (target, key, value) => { target[key as string] = value; return true; },
  });
  const found = new Function("scope", "with(scope){" + code.replaceAll("import.meta", "__importMeta") + ";return " + name + ";}")(scope);
  if (found === pass) throw refuseProcessV1("missing", "testing.extracted-function-required", { name });
  return found;
}

type ProductImportV1 = { specifier: string; kind: "default" | "namespace" | "named"; imported: string };
type ProductPlanV1 = { names: readonly string[]; code: string; bound: Record<string, unknown>; pending: Array<[string, ProductImportV1, string]>; module: string };

function planProductFunctionsV1(ts: typeof import("typescript"), text: string, url: URL, names: readonly string[], ports: Record<string, unknown>): ProductPlanV1 {
  const tree = ts.createSourceFile(url.pathname, text, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
  const declarations = new Map<string, import("typescript").Node>();
  const kinds = new Map<import("typescript").Node, string>();
  const imports = new Map<string, ProductImportV1>();
  const collect = (name: import("typescript").BindingName, declaration: import("typescript").Node): void => {
    if (ts.isIdentifier(name)) declarations.set(name.text, declaration);
    else for (const element of name.elements) if (ts.isBindingElement(element)) collect(element.name, declaration);
  };
  for (const statement of tree.statements) {
    if (ts.isImportDeclaration(statement) && statement.importClause && !statement.importClause.isTypeOnly && ts.isStringLiteral(statement.moduleSpecifier)) {
      const clause = statement.importClause, specifier = statement.moduleSpecifier.text;
      if (clause.name) imports.set(clause.name.text, { specifier, kind: "default", imported: "default" });
      if (clause.namedBindings && ts.isNamespaceImport(clause.namedBindings)) imports.set(clause.namedBindings.name.text, { specifier, kind: "namespace", imported: "*" });
      if (clause.namedBindings && ts.isNamedImports(clause.namedBindings)) for (const element of clause.namedBindings.elements) if (!element.isTypeOnly) imports.set(element.name.text, { specifier, kind: "named", imported: (element.propertyName ?? element.name).text });
    } else if ((ts.isFunctionDeclaration(statement) || ts.isClassDeclaration(statement) || ts.isEnumDeclaration(statement)) && statement.name) declarations.set(statement.name.text, statement);
    else if (ts.isVariableStatement(statement)) for (const declaration of statement.declarationList.declarations) { kinds.set(declaration, statement.declarationList.flags & ts.NodeFlags.Const ? "const" : statement.declarationList.flags & ts.NodeFlags.Let ? "let" : "var"); collect(declaration.name, declaration); }
  }
  const references = (root: import("typescript").Node, into: Set<string>): void => {
    const visit = (node: import("typescript").Node): void => {
      if (ts.isIdentifier(node)) {
        const parent = node.parent;
        const label = (ts.isPropertyAccessExpression(parent) && parent.name === node) || (ts.isPropertyAssignment(parent) && parent.name === node) || (ts.isMethodDeclaration(parent) && parent.name === node) || (ts.isPropertySignature(parent) && parent.name === node);
        if (!label) into.add(node.text);
      }
      ts.forEachChild(node, visit);
    };
    visit(root);
  };
  const included = new Set<import("typescript").Node>(), needed = new Set<string>(), queue = [...names];
  while (queue.length > 0) {
    const name = queue.pop()!;
    if (needed.has(name)) continue;
    needed.add(name);
    if (name in ports) continue;
    const statement = declarations.get(name);
    if (!statement || included.has(statement)) continue;
    included.add(statement);
    const found = new Set<string>();
    references(statement, found);
    queue.push(...found);
  }
  for (const name of names) if (!(name in ports) && !declarations.has(name)) throw refuseProcessV1("missing", "testing.product-function-required", { name, module: url.href });
  const declaratorText = (node: import("typescript").VariableDeclaration): string => {
    const edits: Array<[number, number, string]> = [];
    const visit = (name: import("typescript").BindingName): void => {
      if (ts.isIdentifier(name)) return;
      for (const element of name.elements) {
        if (!ts.isBindingElement(element)) continue;
        if (ts.isIdentifier(element.name) && element.name.text in ports) edits.push([element.name.getStart(tree), element.name.getEnd(), (element.propertyName || element.dotDotDotToken ? "" : element.name.text + ": ") + "__ported_" + edits.length]);
        else visit(element.name);
      }
    };
    visit(node.name);
    let text = node.getText(tree);
    const base = node.getStart(tree);
    for (const [start, end, replacement] of edits.sort((left, right) => right[0] - left[0])) text = text.slice(0, start - base) + replacement + text.slice(end - base);
    return text;
  };
  const body = [...included].sort((left, right) => left.pos - right.pos).map(node => kinds.has(node) ? kinds.get(node) + " " + declaratorText(node as import("typescript").VariableDeclaration) + ";" : node.getText(tree).replace(/^export\s+(default\s+)?/u, "")).join("\n");
  const code = ts.transpileModule(body, { compilerOptions: { target: ts.ScriptTarget.ES2022, module: ts.ModuleKind.ESNext } }).outputText.replace(/^export \{\};?\s*$/mu, "").replaceAll("import.meta", "__importMeta");
  const folder = new URL(".", url).pathname.replace(/\/$/u, "");
  const bound: Record<string, unknown> = { __importMeta: { url: url.href, dir: folder, dirname: folder } };
  const pending: Array<[string, ProductImportV1, string]> = [];
  for (const name of needed) {
    if (name in ports) { bound[name] = ports[name]; continue; }
    const row = imports.get(name);
    if (row && !declarations.has(name)) pending.push([name, row, row.specifier.startsWith(".") ? new URL(row.specifier, url).href : row.specifier]);
  }
  for (const name of Object.keys(ports)) if (!(name in bound)) bound[name] = ports[name];
  return { names, code, bound, pending, module: url.href };
}

function bindProductImportV1(plan: ProductPlanV1, name: string, row: ProductImportV1, loaded: any): void {
  if (row.kind === "named" && !(row.imported in loaded)) throw refuseProcessV1("missing", "testing.product-import-required", { name, imported: row.imported, specifier: row.specifier, module: plan.module });
  plan.bound[name] = row.kind === "namespace" ? loaded : loaded[row.imported];
}

function evaluateProductPlanV1(plan: ProductPlanV1): Record<string, any> {
  return new Function(...Object.keys(plan.bound), plan.code + "\nreturn {" + plan.names.join(",") + "};")(...Object.values(plan.bound));
}

function productModuleUrlV1(moduleUrl: string | URL, pathToFileURL: (path: string) => URL): URL {
  return typeof moduleUrl === "string" && !/^[a-z][a-z0-9+.-]*:/iu.test(moduleUrl) ? pathToFileURL(moduleUrl) : new URL(moduleUrl);
}

/** 🧬️ Loads named functions of a product module (a file URL or an absolute path) with every module-level helper and import they reach transitively, resolved against the real module; only names given in `ports` are replaced, and a moved or missing helper fails at load instead of drifting silently. */
export async function loadProductFunctionsV1(moduleUrl: string | URL, names: readonly string[], ports: Record<string, unknown> = {}): Promise<Record<string, any>> {
  const ts = await import("typescript");
  const { readFile } = await import("node:fs/promises");
  const { fileURLToPath, pathToFileURL } = await import("node:url");
  const url = productModuleUrlV1(moduleUrl, pathToFileURL);
  const plan = planProductFunctionsV1(ts, await readFile(fileURLToPath(url), "utf8"), url, names, ports);
  for (const [name, row, specifier] of plan.pending) bindProductImportV1(plan, name, row, await import(specifier));
  return evaluateProductPlanV1(plan);
}

/** 🧬️ The synchronous form of `loadProductFunctionsV1` for harnesses that build their function inside a synchronous port; it resolves the same imports through the module system's synchronous loader. */
export function loadProductFunctionsSyncV1(moduleUrl: string | URL, names: readonly string[], ports: Record<string, unknown> = {}): Record<string, any> {
  const load = createRequire(import.meta.url);
  const ts = load("typescript") as typeof import("typescript");
  const { fileURLToPath, pathToFileURL } = load("node:url") as typeof import("node:url");
  const url = productModuleUrlV1(moduleUrl, pathToFileURL);
  const plan = planProductFunctionsV1(ts, (load("node:fs") as typeof import("node:fs")).readFileSync(fileURLToPath(url), "utf8"), url, names, ports);
  for (const [name, row, specifier] of plan.pending) bindProductImportV1(plan, name, row, load(specifier.startsWith("file:") ? fileURLToPath(specifier) : specifier));
  return evaluateProductPlanV1(plan);
}

/** 🧾️ Evaluates generated source text (a worker body, a host shim, an embedded class) that is itself the product artifact under test, and returns the named top-level binding; only the given ports are visible besides the globals. */
export function evaluateGeneratedSourceV1(text: string, result: string, ports: Record<string, unknown> = {}): any {
  return new Function(...Object.keys(ports), text + ";return " + result + ";")(...Object.values(ports));
}

/** 🧬️ The transpiled declaration closure of named functions of a product module (what `loadProductFunctionsSyncV1` evaluates) without its imports, for harnesses that evaluate the text themselves with `evaluateExtractedFunctionV1`; names in `portNames` are left out so the harness supplies them. */
export function productClosureCodeV1(moduleUrl: string | URL, names: readonly string[], portNames: readonly string[] = []): string {
  const load = createRequire(import.meta.url);
  const ts = load("typescript") as typeof import("typescript");
  const { fileURLToPath, pathToFileURL } = load("node:url") as typeof import("node:url");
  const url = productModuleUrlV1(moduleUrl, pathToFileURL);
  return planProductFunctionsV1(ts, (load("node:fs") as typeof import("node:fs")).readFileSync(fileURLToPath(url), "utf8"), url, names, Object.fromEntries(portNames.map(name => [name, undefined]))).code;
}
