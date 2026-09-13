//#region 🔌️Adapters
import Ajv from "ajv/dist/2020";
import { describe, expect, test } from "bun:test";
import { lstatSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { join, relative, resolve, sep } from "node:path";
import { tmpdir } from "node:os";
import ts from "typescript";
import { classifyPackageSource, classifyPackageSourceDisposition, clearDiscoveryCache, discoverPackageProblems, fixedFilenameContractIdsForPath, fixedSourceDispositionDecision, loadCatalogTaxonomy, type PackageGlueGrammarSpec, type PackageSourceRole, type PackageSourceDisposition } from "../../../🔍️discovery/🟦️.ts";
import { fixedContractScopeSpecificityRank, inventoryTaxonomy, inventoryTaxonomyWithCapturedSourceRead, type FixedContractScopeKind } from "../../🟦️.ts";
//#endregion 🔌️Adapters

//#region 🧬️Contract
const vectors = JSON.parse(readFileSync(new URL("../../🧫️fixtures/📦️package-boundary-classification/🔣️.json", import.meta.url), "utf8")) as {
  schemaVersion: 1;
  glueRoleCases: readonly { id: string; analyzer: PackageGlueGrammarSpec["analyzer"]; maxDelegationStatements: number; content: string; expectedRole: PackageSourceRole; expectedEvidence: string; oracle: "typescript-compiler" | "rust-syn" | "go-parser" | "python-parser" | "c-compiler"; dispositionValidator?: PackageSourceDisposition["validator"]; expectedDispositionRole?: PackageSourceRole; expectedSemanticDiagnostics?: readonly number[]; expectedExecuteBinding?: "import" | "local" }[];
  fixedScriptCases: readonly { id: string; placement: "root" | "domain" | "package"; manifest: "present" | "absent" | "not-applicable"; path: string; content: string; expectedDispositionRole: "tool-metadata" | "unresolved"; expectedFinding: "fixed-source-disposition-unresolved" | null; expectedSemanticDiagnostics?: readonly number[] }[];
  fixedScriptReadFailureCases: readonly { id: string; path: string; content: string; expectedFixedContractId: "root-script"; expectedFileKind: null; expectedPackageRole: "configuration" | "not-package"; expectedViolations: readonly ["fixed-source-content-unreadable", "path-read-failed"] }[];
  manifestPresenceCases: readonly { id: string; manifest: "present" | "absent"; content: string; expectedProblem: "package-implementation" | null }[];
  scopeSpecificityOrder: readonly FixedContractScopeKind[];
};
const schema = JSON.parse(readFileSync(new URL("../../🧬️schema/📦️package-boundary-classification/🔣️.json", import.meta.url), "utf8"));
const repoRoot = resolve(import.meta.dir, "../../../../../../../..");
const validator = new Ajv({ strict: true, allErrors: true });
const validateVectors = validator.compile(schema);

function fixedScriptControlRoot(prefix: string): string {
  const ticketGenerated = resolve(repoRoot, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/KIND-ONLY-BASENAMES-ACROSS-THE-TAXONOMY-TREE/🗑️generated");
  const base = resolve(process.env.SEMIO_TEST_ARTIFACT_DIR?.trim() || join(ticketGenerated, "sol-fixed-script-semantic-enforcement/test-artifacts"));
  const ticketRelative = relative(ticketGenerated, base);
  if (ticketRelative === ".." || ticketRelative.startsWith(`..${sep}`)) throw new Error("SEMIO_TEST_ARTIFACT_DIR must remain inside the active ticket generated directory");
  let current = repoRoot;
  for (const segment of relative(repoRoot, base).split(sep).filter(Boolean)) {
    current = join(current, segment);
    try {
      const entry = lstatSync(current);
      if (entry.isSymbolicLink() || !entry.isDirectory()) throw new Error(`Test artifact ancestor is not a plain directory: ${current}`);
    } catch (error) {
      if (!(error instanceof Error && "code" in error && error.code === "ENOENT")) throw error;
      mkdirSync(current);
      const entry = lstatSync(current);
      if (entry.isSymbolicLink() || !entry.isDirectory()) throw new Error(`Test artifact ancestor is not a plain directory: ${current}`);
    }
  }
  return mkdtempSync(join(base, prefix));
}

function ecmaOracle(content: string, analyzer: "typescript" | "javascript", maximum: number): PackageSourceRole {
  const scriptKind = analyzer === "typescript" ? ts.ScriptKind.TSX : ts.ScriptKind.JSX;
  const source = ts.createSourceFile(analyzer === "typescript" ? "🟦️.tsx" : "🟨️.jsx", content, ts.ScriptTarget.Latest, true, scriptKind);
  if ((source as ts.SourceFile & { parseDiagnostics?: readonly ts.Diagnostic[] }).parseDiagnostics?.length) return "unresolved";
  const imports = new Set<string>();
  for (const statement of source.statements) if (ts.isImportDeclaration(statement) && statement.importClause) {
    if (statement.importClause.name) imports.add(statement.importClause.name.text);
    const bindings = statement.importClause.namedBindings;
    if (bindings && ts.isNamespaceImport(bindings)) imports.add(bindings.name.text);
    else if (bindings) for (const element of bindings.elements) imports.add(element.name.text);
  }
  const root = (expression: ts.Expression): string | null => {
    if (ts.isIdentifier(expression)) return expression.text;
    if (ts.isPropertyAccessExpression(expression) || ts.isElementAccessExpression(expression)) return root(expression.expression);
    if (ts.isAwaitExpression(expression) || ts.isParenthesizedExpression(expression)) return root(expression.expression);
    return null;
  };
  const delegatedExpression = (expression: ts.Expression): boolean => {
    if (ts.isAwaitExpression(expression) || ts.isParenthesizedExpression(expression)) return delegatedExpression(expression.expression);
    if (ts.isCallExpression(expression) || ts.isNewExpression(expression)) return imports.has(root(expression.expression) ?? "");
    if (ts.isJsxElement(expression)) return ts.isIdentifier(expression.openingElement.tagName) && imports.has(expression.openingElement.tagName.text);
    if (ts.isJsxSelfClosingElement(expression)) return ts.isIdentifier(expression.tagName) && imports.has(expression.tagName.text);
    return ts.isIdentifier(expression) && imports.has(expression.text);
  };
  const delegatedBody = (body: ts.ConciseBody | undefined): boolean => {
    if (!body) return false;
    if (!ts.isBlock(body)) return delegatedExpression(body);
    return body.statements.length > 0 && body.statements.length <= maximum && body.statements.every((statement) =>
      ts.isReturnStatement(statement) && statement.expression !== undefined && delegatedExpression(statement.expression)
      || ts.isExpressionStatement(statement) && delegatedExpression(statement.expression));
  };
  let role: PackageSourceRole = "declaration";
  for (const statement of source.statements) {
    if (ts.isImportDeclaration(statement) || ts.isExportDeclaration(statement)) continue;
    if (ts.isTypeAliasDeclaration(statement) || ts.isInterfaceDeclaration(statement) || ts.isEnumDeclaration(statement) || ts.isClassDeclaration(statement)) return "implementation";
    if (ts.isFunctionDeclaration(statement)) {
      if (!delegatedBody(statement.body)) return "implementation";
      const name = statement.name?.text ?? "";
      role = /^(?:main|start|bootstrap)$/iu.test(name) ? "bootstrap" : "thin-delegation";
      continue;
    }
    if (ts.isVariableStatement(statement)) {
      const exported = statement.modifiers?.some((modifier) => modifier.kind === ts.SyntaxKind.ExportKeyword);
      const authored = statement.declarationList.declarations.some((declaration) => {
        const value = declaration.initializer;
        if (!value || ts.isStringLiteral(value) || ts.isNumericLiteral(value) || value.kind === ts.SyntaxKind.TrueKeyword || value.kind === ts.SyntaxKind.FalseKeyword || ts.isArrayLiteralExpression(value) || ts.isObjectLiteralExpression(value)) return false;
        if (ts.isArrowFunction(value) || ts.isFunctionExpression(value)) return !delegatedBody(value.body);
        return !delegatedExpression(value);
      });
      if (exported || authored) return "implementation";
      role = "thin-delegation";
      continue;
    }
    if (ts.isExportAssignment(statement)) {
      if (!delegatedExpression(statement.expression) && !ts.isObjectLiteralExpression(statement.expression)) return "implementation";
      role = "thin-delegation";
      continue;
    }
    if (ts.isExpressionStatement(statement)) {
      const expression = ts.isAwaitExpression(statement.expression) ? statement.expression.expression : statement.expression;
      const name = ts.isCallExpression(expression) ? expression.expression.getText(source) : "";
      if (/^(?:register|mount|provide|bind)/iu.test(name)) role = "registration";
      else if (delegatedExpression(statement.expression)) role = "thin-delegation";
      else return "implementation";
      continue;
    }
    return "implementation";
  }
  return role;
}

function ecmaDispositionOracle(content: string, analyzer: "typescript" | "javascript", validator: PackageSourceDisposition["validator"], maximum: number): PackageSourceRole {
  const role = ecmaOracle(content, analyzer, maximum);
  if (validator === "package-glue") return role;
  const source = ts.createSourceFile(analyzer === "typescript" ? "🟦️.tsx" : "🟨️.jsx", content, ts.ScriptTarget.Latest, true, analyzer === "typescript" ? ts.ScriptKind.TSX : ts.ScriptKind.JSX);
  const calls = (node: ts.Node): readonly string[] => {
    const names: string[] = [];
    const visit = (child: ts.Node): void => {
      if (ts.isCallExpression(child)) names.push(child.expression.getText(source).split(".").at(-1)!);
      ts.forEachChild(child, visit);
    };
    visit(node);
    return names;
  };
  if (validator === "command-router") {
    type Binding = { readonly kind: "pending" | "import-value" | "import-type" | "class" | "parameter" | "data" | "finite" | "receipt" | "module" | "closure" | "router"; readonly imported?: string; readonly module?: string; readonly closure?: ts.ArrowFunction; readonly scope?: Scope };
    class Scope {
      readonly bindings = new Map<string, Binding>();
      constructor(readonly parent?: Scope) {}
      define(name: string, binding: Binding): boolean { if (this.bindings.has(name)) return false; this.bindings.set(name, binding); return true; }
      initialize(name: string, binding: Binding): boolean { if (this.bindings.get(name)?.kind !== "pending") return false; this.bindings.set(name, binding); return true; }
      resolve(name: string): Binding | undefined { return this.bindings.get(name) ?? this.parent?.resolve(name); }
    }
    type Value = "invalid" | "data" | "finite" | "receipt" | "module" | "closure" | "router" | "error" | "console" | "process";
    const top = new Scope(), classes = new Set<string>(), wired = new Set<string>();
    let terminals = 0;
    for (const statement of source.statements) if (ts.isImportDeclaration(statement) && statement.importClause) {
      const clause = statement.importClause, module = ts.isStringLiteral(statement.moduleSpecifier) ? statement.moduleSpecifier.text : "";
      if (clause.name && !top.define(clause.name.text, { kind: clause.isTypeOnly ? "import-type" : "import-value", imported: "default", module })) return "unresolved";
      const bindings = clause.namedBindings;
      if (bindings && ts.isNamespaceImport(bindings)) {
        if (!top.define(bindings.name.text, { kind: clause.isTypeOnly ? "import-type" : "import-value", imported: "*", module })) return "unresolved";
      } else if (bindings) for (const element of bindings.elements) {
        const runtime = !clause.isTypeOnly && !element.isTypeOnly;
        if (!top.define(element.name.text, { kind: runtime ? "import-value" : "import-type", imported: element.propertyName?.text ?? element.name.text, module })) return "unresolved";
      }
      if ((module === "node:fs" || module === "node:fs/promises") && (!clause.isTypeOnly || bindings && ts.isNamedImports(bindings) && bindings.elements.some((element) => !element.isTypeOnly))) return "unresolved";
    }
    const unwrap = (expression: ts.Expression): ts.Expression => ts.isAwaitExpression(expression) || ts.isParenthesizedExpression(expression) || ts.isNonNullExpression(expression) || ts.isAsExpression(expression) || ts.isTypeAssertionExpression(expression) ? unwrap(expression.expression) : expression;
    const intrinsic = (scope: Scope, name: string): Value => scope.resolve(name) ? "invalid" : name === "console" ? "console" : name === "process" ? "process" : name === "Error" ? "error" : "invalid";
    const identifier = (expression: ts.Expression): string | null => ts.isIdentifier(unwrap(expression)) ? (unwrap(expression) as ts.Identifier).text : null;
    const importedRoot = (expression: ts.Expression, scope: Scope): Binding | undefined => {
      let value = unwrap(expression);
      while (ts.isPropertyAccessExpression(value) || ts.isElementAccessExpression(value)) value = unwrap(value.expression);
      const binding = ts.isIdentifier(value) ? scope.resolve(value.text) : undefined;
      return binding?.kind === "import-value" ? binding : undefined;
    };
    let value: (expression: ts.Expression, scope: Scope) => Value;
    let block: (statements: readonly ts.Statement[], scope: Scope, guarded?: boolean, closure?: boolean) => boolean;
    const argument = (expression: ts.Expression, scope: Scope): boolean => ["data", "finite", "receipt", "closure", "router"].includes(value(expression, scope));
    const status = (expression: ts.Expression, scope: Scope): boolean => {
      const item = unwrap(expression);
      return ts.isBinaryExpression(item) && item.operatorToken.kind === ts.SyntaxKind.PlusToken ? status(item.left, scope) && status(item.right, scope) : argument(item, scope);
    };
    const patterns = (name: ts.BindingName): readonly string[] => ts.isIdentifier(name) ? [name.text] : name.elements.flatMap((element) => ts.isBindingElement(element) ? patterns(element.name) : []);
    const patternDefaults = (name: ts.BindingName): boolean => !ts.isIdentifier(name) && name.elements.some((element) => ts.isBindingElement(element) && (element.initializer !== undefined || patternDefaults(element.name)));
    const closure = (expression: ts.ArrowFunction, scope: Scope): boolean => {
      if (expression.parameters.some((parameter) => parameter.initializer || !ts.isIdentifier(parameter.name) || ts.canHaveDecorators(parameter) && (ts.getDecorators(parameter)?.length ?? 0) > 0)) return false;
      const nested = new Scope(scope);
      for (const parameter of expression.parameters) if (!nested.define((parameter.name as ts.Identifier).text, { kind: "parameter" })) return false;
      return ts.isBlock(expression.body) ? block(expression.body.statements, nested, false, true) : value(expression.body, nested) === "receipt";
    };
    const collection = (expression: ts.CallExpression, scope: Scope): Value => {
      if (!ts.isPropertyAccessExpression(expression.expression)) return "invalid";
      const member = expression.expression.name.text, receiver = expression.expression.expression, receiverValue = value(receiver, scope);
      if (member === "at" || member === "includes") return ["data", "finite", "receipt"].includes(receiverValue) && expression.arguments.every((row) => argument(row, scope)) ? "data" : "invalid";
      if (member === "test") return ts.isRegularExpressionLiteral(unwrap(receiver)) && expression.arguments.length === 1 && argument(expression.arguments[0]!, scope) ? "data" : "invalid";
      const callback = expression.arguments[0];
      if (member === "some") {
        if (!["data", "finite", "receipt"].includes(receiverValue) || expression.arguments.length !== 1 || !callback || !ts.isArrowFunction(callback) || callback.parameters.length !== 1 || !ts.isIdentifier(callback.parameters[0]!.name) || callback.parameters[0]!.initializer || ts.isBlock(callback.body)) return "invalid";
        const nested = new Scope(scope); nested.define(callback.parameters[0]!.name.text, { kind: "parameter" });
        return value(callback.body, nested) === "data" ? "data" : "invalid";
      }
      if (member !== "map" && member !== "flatMap" || receiverValue !== "finite" || expression.arguments.length !== 1 || !callback || !ts.isArrowFunction(callback) || callback.parameters.length !== 1 || !ts.isIdentifier(callback.parameters[0]!.name) || callback.parameters[0]!.initializer) return "invalid";
      const nested = new Scope(scope); nested.define(callback.parameters[0]!.name.text, { kind: "parameter" });
      return (ts.isBlock(callback.body) ? block(callback.body.statements, nested, false, true) : argument(callback.body, nested)) ? "finite" : "invalid";
    };
    const registrationValue = (expression: ts.Expression, scope: Scope): boolean => {
      const item = unwrap(expression);
      if (ts.isStringLiteralLike(item) || ts.isNumericLiteral(item) || ts.isRegularExpressionLiteral(item) || [ts.SyntaxKind.TrueKeyword, ts.SyntaxKind.FalseKeyword, ts.SyntaxKind.NullKeyword].includes(item.kind) || ts.isMetaProperty(item)) return true;
      if (ts.isIdentifier(item)) {
        if (item.text === "undefined") return true;
        const binding = scope.resolve(item.text);
        return binding?.kind === "class" || binding?.kind === "data" || binding?.kind === "finite" || binding?.kind === "import-value";
      }
      if (ts.isPropertyAccessExpression(item)) return value(item, scope) === "data";
      if (ts.isElementAccessExpression(item)) return item.argumentExpression !== undefined && registrationValue(item.expression, scope) && registrationValue(item.argumentExpression, scope);
      if (ts.isArrayLiteralExpression(item)) return item.elements.every((row) => ts.isSpreadElement(row) ? registrationValue(row.expression, scope) : registrationValue(row, scope));
      if (ts.isObjectLiteralExpression(item)) return item.properties.every((row) => ts.isSpreadAssignment(row) ? registrationValue(row.expression, scope) : ts.isShorthandPropertyAssignment(row) ? registrationValue(row.name, scope) : ts.isPropertyAssignment(row) && (!ts.isComputedPropertyName(row.name) || registrationValue(row.name.expression, scope)) && registrationValue(row.initializer, scope));
      if (ts.isTemplateExpression(item)) return item.templateSpans.every((row) => registrationValue(row.expression, scope));
      if (ts.isNoSubstitutionTemplateLiteral(item)) return true;
      if (ts.isConditionalExpression(item)) return registrationValue(item.condition, scope) && registrationValue(item.whenTrue, scope) && registrationValue(item.whenFalse, scope);
      if (ts.isBinaryExpression(item) && [ts.SyntaxKind.QuestionQuestionToken, ts.SyntaxKind.BarBarToken, ts.SyntaxKind.AmpersandAmpersandToken, ts.SyntaxKind.EqualsEqualsEqualsToken, ts.SyntaxKind.ExclamationEqualsEqualsToken, ts.SyntaxKind.LessThanToken, ts.SyntaxKind.LessThanEqualsToken, ts.SyntaxKind.GreaterThanToken, ts.SyntaxKind.GreaterThanEqualsToken].includes(item.operatorToken.kind)) return registrationValue(item.left, scope) && registrationValue(item.right, scope);
      if (ts.isCallExpression(item)) {
        const binding = importedRoot(item.expression, scope);
        return (binding?.imported === "dirname" || binding?.imported === "fileURLToPath") && item.arguments.length === 1 && registrationValue(item.arguments[0]!, scope);
      }
      return false;
    };
    value = (expression: ts.Expression, scope: Scope): Value => {
      const item = unwrap(expression);
      if (ts.isStringLiteralLike(item) || ts.isNumericLiteral(item) || ts.isRegularExpressionLiteral(item) || [ts.SyntaxKind.TrueKeyword, ts.SyntaxKind.FalseKeyword, ts.SyntaxKind.NullKeyword].includes(item.kind) || ts.isMetaProperty(item)) return "data";
      if (ts.isIdentifier(item)) {
        if (item.text === "undefined" || item.text === "this") return "data";
        const binding = scope.resolve(item.text);
        if (!binding) return intrinsic(scope, item.text);
        if (binding.kind === "pending" || binding.kind === "import-type") return "invalid";
        return binding.kind === "import-value" || binding.kind === "class" || binding.kind === "parameter" || binding.kind === "data" ? "data" : binding.kind;
      }
      if (item.kind === ts.SyntaxKind.ThisKeyword) return "data";
      if (ts.isTemplateExpression(item)) return item.templateSpans.every((row) => argument(row.expression, scope)) ? "data" : "invalid";
      if (ts.isNoSubstitutionTemplateLiteral(item)) return "data";
      if (ts.isArrayLiteralExpression(item)) return item.elements.every((row) => ts.isSpreadElement(row) ? argument(row.expression, scope) : argument(row, scope)) ? "finite" : "invalid";
      if (ts.isObjectLiteralExpression(item)) return item.properties.every((row) => ts.isSpreadAssignment(row) ? argument(row.expression, scope) : ts.isShorthandPropertyAssignment(row) ? row.objectAssignmentInitializer === undefined && argument(row.name, scope) : ts.isPropertyAssignment(row) && (!ts.isComputedPropertyName(row.name) || argument(row.name.expression, scope)) && argument(row.initializer, scope)) ? "data" : "invalid";
      if (ts.isConditionalExpression(item)) return argument(item.condition, scope) && argument(item.whenTrue, scope) && argument(item.whenFalse, scope) ? "data" : "invalid";
      if (ts.isBinaryExpression(item)) return [ts.SyntaxKind.QuestionQuestionToken, ts.SyntaxKind.BarBarToken, ts.SyntaxKind.AmpersandAmpersandToken, ts.SyntaxKind.EqualsEqualsEqualsToken, ts.SyntaxKind.ExclamationEqualsEqualsToken, ts.SyntaxKind.EqualsEqualsToken, ts.SyntaxKind.ExclamationEqualsToken, ts.SyntaxKind.LessThanToken, ts.SyntaxKind.LessThanEqualsToken, ts.SyntaxKind.GreaterThanToken, ts.SyntaxKind.GreaterThanEqualsToken].includes(item.operatorToken.kind) && argument(item.left, scope) && argument(item.right, scope) ? "data" : "invalid";
      if (ts.isPrefixUnaryExpression(item)) return item.operator === ts.SyntaxKind.ExclamationToken && argument(item.operand, scope) ? "data" : "invalid";
      if (ts.isArrowFunction(item)) return closure(item, scope) ? "closure" : "invalid";
      if (ts.isPropertyAccessExpression(item)) {
        if (ts.isMetaProperty(item.expression)) return "data";
        const base = value(item.expression, scope);
        return ["data", "finite", "receipt", "console", "process", "router"].includes(base) ? base === "finite" ? "finite" : "data" : "invalid";
      }
      if (ts.isElementAccessExpression(item)) return item.argumentExpression && argument(item.argumentExpression, scope) && ["data", "finite", "receipt"].includes(value(item.expression, scope)) ? "data" : "invalid";
      if (ts.isNewExpression(item)) {
        const name = identifier(item.expression), binding = name ? scope.resolve(name) : undefined;
        if (name === "Error" && !binding && item.arguments?.every((row) => status(row, scope)) !== false) return "error";
        if (binding?.kind !== "import-value" || item.arguments?.every((row) => argument(row, scope)) === false) return "invalid";
        return binding.imported === "ScriptRouter" ? item.arguments?.every((row) => registrationValue(row, scope)) !== false ? "router" : "invalid" : "receipt";
      }
      if (!ts.isCallExpression(item)) return "invalid";
      if (item.expression.kind === ts.SyntaxKind.ImportKeyword) return item.arguments.length === 1 && ts.isStringLiteral(item.arguments[0]!) && /(?:^|\/)🧪️tests(?:\/|$)|(?:^|\/)🧪️(?:\/|$)/u.test(item.arguments[0]!.text) ? "module" : "invalid";
      if (ts.isIdentifier(item.expression)) {
        const binding = scope.resolve(item.expression.text);
        return (binding?.kind === "import-value" || binding?.kind === "closure") && item.arguments.every((row) => argument(row, scope)) ? "receipt" : "invalid";
      }
      if (!ts.isPropertyAccessExpression(item.expression)) return "invalid";
      const member = item.expression.name.text, receiver = value(item.expression.expression, scope);
      if (receiver === "console" && (member === "log" || member === "error")) return item.arguments.every((row) => status(row, scope)) ? "receipt" : "invalid";
      if (receiver === "process" && member === "exit") return item.arguments.length === 1 && argument(item.arguments[0]!, scope) ? "receipt" : "invalid";
      if (["at", "includes", "some", "map", "flatMap", "test"].includes(member)) return collection(item, scope);
      if (member === "run" && ts.isNewExpression(unwrap(item.expression.expression))) {
        const instance = unwrap(item.expression.expression) as ts.NewExpression, binding = scope.resolve(identifier(instance.expression) ?? "");
        if (binding?.kind === "import-value" && item.arguments.every((row) => argument(row, scope))) return "receipt";
      }
      return importedRoot(item.expression, scope) && item.arguments.every((row) => argument(row, scope)) ? "receipt" : "invalid";
    };
    const define = (declaration: ts.VariableDeclaration, scope: Scope): boolean => {
      if (!declaration.initializer || patternDefaults(declaration.name)) return false;
      const names = patterns(declaration.name), result = value(declaration.initializer, scope);
      if (result === "invalid" || result === "console" || result === "process" || result === "error") return false;
      if (result === "module") return !ts.isIdentifier(declaration.name) && names.every((name) => scope.initialize(name, { kind: "import-value", imported: name }));
      if (result === "closure") return ts.isIdentifier(declaration.name) && scope.initialize(declaration.name.text, { kind: "closure", closure: declaration.initializer as ts.ArrowFunction, scope });
      const kind: Binding["kind"] = result === "finite" ? "finite" : result === "receipt" ? "receipt" : result === "router" ? "router" : "data";
      return names.every((name) => scope.initialize(name, { kind }));
    };
    const prebind = (statements: readonly ts.Statement[], scope: Scope): boolean => {
      for (const statement of statements) {
        if (ts.isClassDeclaration(statement)) {
          if (!statement.name || !scope.define(statement.name.text, { kind: "pending" })) return false;
          continue;
        }
        if (!ts.isVariableStatement(statement) || (statement.declarationList.flags & (ts.NodeFlags.Const | ts.NodeFlags.Let)) === 0) continue;
        for (const declaration of statement.declarationList.declarations) for (const name of patterns(declaration.name)) if (!scope.define(name, { kind: "pending" })) return false;
      }
      return true;
    };
    block = (statements: readonly ts.Statement[], scope: Scope, guarded = false, insideClosure = false): boolean => {
      if (statements.length === 0 || statements.length > maximum || !prebind(statements, scope)) return false;
      for (const statement of statements) {
        if (ts.isVariableStatement(statement)) {
          if ((statement.declarationList.flags & ts.NodeFlags.Const) === 0 || !statement.declarationList.declarations.every((row) => define(row, scope))) return false;
          continue;
        }
        if (ts.isExpressionStatement(statement)) {
          if (!["receipt", "module"].includes(value(statement.expression, scope))) return false;
          continue;
        }
        if (ts.isThrowStatement(statement)) {
          if (!guarded || !statement.expression || value(statement.expression, scope) !== "error") return false;
          continue;
        }
        if (ts.isReturnStatement(statement)) {
          if ((!guarded && !insideClosure) || statement.expression && !argument(statement.expression, scope)) return false;
          continue;
        }
        if (ts.isBlock(statement)) {
          if (!block(statement.statements, new Scope(scope), guarded, insideClosure)) return false;
          continue;
        }
        if (ts.isIfStatement(statement)) {
          if (!argument(statement.expression, scope)) return false;
          const branch = (row: ts.Statement): boolean => block(ts.isBlock(row) ? row.statements : [row], new Scope(scope), true, insideClosure);
          if (!branch(statement.thenStatement) || statement.elseStatement && !branch(statement.elseStatement)) return false;
          continue;
        }
        if (ts.isForOfStatement(statement)) {
          if (!ts.isVariableDeclarationList(statement.initializer) || (statement.initializer.flags & ts.NodeFlags.Const) === 0 || statement.initializer.declarations.length !== 1 || !ts.isIdentifier(statement.initializer.declarations[0]!.name) || value(statement.expression, scope) !== "finite") return false;
          const nested = new Scope(scope); nested.define(statement.initializer.declarations[0]!.name.text, { kind: "parameter" });
          if (!block(ts.isBlock(statement.statement) ? statement.statement.statements : [statement.statement], nested, false, insideClosure)) return false;
          continue;
        }
        return false;
      }
      return true;
    };
    const router = (expression: ts.Expression, scope: Scope): boolean => {
      const item = unwrap(expression);
      if (ts.isIdentifier(item)) return scope.resolve(item.text)?.kind === "router";
      if (ts.isNewExpression(item)) {
        const binding = scope.resolve(identifier(item.expression) ?? "");
        if (binding?.kind !== "import-value" || binding.imported !== "ScriptRouter") return false;
      } else if (ts.isCallExpression(item) && ts.isPropertyAccessExpression(item.expression) && item.expression.name.text === "register") {
        if (!router(item.expression.expression, scope)) return false;
      } else return false;
      const inspect = (argument: ts.Expression): boolean => {
        const row = unwrap(argument);
        if (ts.isIdentifier(row)) {
          const binding = scope.resolve(row.text);
          if (binding?.kind === "class") wired.add(row.text);
          return binding?.kind === "class" || binding?.kind === "data" || binding?.kind === "finite" || binding?.kind === "import-value" || row.text === "undefined";
        }
        if (ts.isArrayLiteralExpression(row)) return row.elements.every((element) => !ts.isSpreadElement(element) && inspect(element));
        if (ts.isObjectLiteralExpression(row)) return row.properties.every((property) => ts.isSpreadAssignment(property) ? inspect(property.expression) : ts.isShorthandPropertyAssignment(property) ? inspect(property.name) : ts.isPropertyAssignment(property) && (!ts.isComputedPropertyName(property.name) || registrationValue(property.name.expression, scope)) && inspect(property.initializer));
        return registrationValue(row, scope);
      };
      return item.arguments?.every(inspect) !== false;
    };
    const terminal = (statement: ts.Statement, scope: Scope, mainGuard = false): boolean => {
      if (!ts.isExpressionStatement(statement)) return false;
      const expression = unwrap(statement.expression);
      if (!ts.isCallExpression(expression)) return false;
      const callee = expression.expression;
      if (mainGuard && ts.isPropertyAccessExpression(callee) && callee.name.text === "run" && ts.isIdentifier(callee.expression) && scope.resolve(callee.expression.text)?.kind === "router") {
        const argument = expression.arguments.length === 1 ? unwrap(expression.arguments[0]!) : undefined;
        if (!argument || !ts.isCallExpression(argument) || !ts.isPropertyAccessExpression(argument.expression) || argument.expression.name.text !== "slice" || !ts.isPropertyAccessExpression(argument.expression.expression) || argument.expression.expression.name.text !== "argv" || !ts.isIdentifier(argument.expression.expression.expression) || intrinsic(scope, argument.expression.expression.expression.text) !== "process" || argument.arguments.length !== 1 || !ts.isNumericLiteral(argument.arguments[0]!) || argument.arguments[0]!.getText(source) !== "2") return false;
        terminals++;
        return true;
      }
      if (!ts.isIdentifier(callee)) return false;
      const binding = scope.resolve(expression.expression.text);
      if (binding?.kind !== "import-value" || !["runBundleScriptMain", "runWorkspaceScriptMain", "runPolicyOnlyMain", "runArtifactRustPackageMain", "runArtifactTypeScriptPackageMain"].includes(binding.imported ?? "")) return false;
      if (!expression.arguments.every((row) => router(row, scope) || ["data", "finite"].includes(value(row, scope)))) return false;
      terminals++;
      return true;
    };
    const environmentDefault = (statement: ts.Statement, scope: Scope): boolean => {
      if (!ts.isExpressionStatement(statement) || !ts.isBinaryExpression(statement.expression) || statement.expression.operatorToken.kind !== ts.SyntaxKind.QuestionQuestionEqualsToken) return false;
      const { left, right } = statement.expression;
      return ts.isPropertyAccessExpression(left) && /^[A-Z][A-Z0-9_]*$/u.test(left.name.text) && ts.isPropertyAccessExpression(left.expression) && left.expression.name.text === "env" && ts.isIdentifier(left.expression.expression) && intrinsic(scope, left.expression.expression.text) === "process" && (ts.isStringLiteral(right) || ts.isNumericLiteral(right));
    };
    const environmentGuard = (statement: ts.IfStatement, scope: Scope): boolean => {
      if (!ts.isBinaryExpression(statement.expression) || statement.expression.operatorToken.kind !== ts.SyntaxKind.EqualsEqualsEqualsToken || !ts.isElementAccessExpression(statement.expression.left) || !ts.isPropertyAccessExpression(statement.expression.left.expression) || statement.expression.left.expression.name.text !== "argv" || !ts.isIdentifier(statement.expression.left.expression.expression) || intrinsic(scope, statement.expression.left.expression.expression.text) !== "process" || !ts.isNumericLiteral(statement.expression.left.argumentExpression) || !ts.isStringLiteral(statement.expression.right) || statement.elseStatement) return false;
      const rows = ts.isBlock(statement.thenStatement) ? statement.thenStatement.statements : [statement.thenStatement];
      if (rows.length !== 1 || !ts.isExpressionStatement(rows[0]!) || !ts.isBinaryExpression(rows[0]!.expression) || rows[0]!.expression.operatorToken.kind !== ts.SyntaxKind.QuestionQuestionEqualsToken || !ts.isPropertyAccessExpression(rows[0]!.expression.left) || !/^[A-Z][A-Z0-9_]*$/u.test(rows[0]!.expression.left.name.text) || !ts.isPropertyAccessExpression(rows[0]!.expression.left.expression) || rows[0]!.expression.left.expression.name.text !== "env" || !ts.isIdentifier(rows[0]!.expression.left.expression.expression) || intrinsic(scope, rows[0]!.expression.left.expression.expression.text) !== "process") return false;
      return ts.isStringLiteral(rows[0]!.expression.right) || ts.isNumericLiteral(rows[0]!.expression.right);
    };
    const importMetaMember = (expression: ts.Expression, member: string): boolean => ts.isPropertyAccessExpression(expression) && !expression.questionDotToken && expression.name.text === member && ts.isMetaProperty(expression.expression) && expression.expression.keywordToken === ts.SyntaxKind.ImportKeyword && expression.expression.name.text === "meta";
    const testDependency = (expression: ts.Expression, scope: Scope): boolean => {
      if (ts.isParenthesizedExpression(expression) || ts.isNonNullExpression(expression) || ts.isAsExpression(expression) || ts.isTypeAssertionExpression(expression)) return testDependency(expression.expression, scope);
      if (ts.isStringLiteral(expression) || ts.isNumericLiteral(expression) || [ts.SyntaxKind.TrueKeyword, ts.SyntaxKind.FalseKeyword, ts.SyntaxKind.NullKeyword].includes(expression.kind)) return true;
      if (ts.isIdentifier(expression)) {
        const binding = scope.resolve(expression.text);
        return binding?.kind === "import-value" && binding.module !== "dynamic-test";
      }
      if (importMetaMember(expression, "dir") || importMetaMember(expression, "url")) return true;
      if (ts.isArrayLiteralExpression(expression)) return expression.elements.every((row) => testDependency(row, scope));
      return ts.isObjectLiteralExpression(expression) && expression.properties.every((row) => ts.isShorthandPropertyAssignment(row) ? row.objectAssignmentInitializer === undefined && testDependency(row.name, scope) : ts.isPropertyAssignment(row) && !ts.isComputedPropertyName(row.name) && testDependency(row.initializer, scope));
    };
    const testRegistration = (statement: ts.IfStatement): boolean => {
      if (statement.elseStatement || !ts.isBlock(statement.thenStatement) || statement.thenStatement.statements.length !== 2) return false;
      const [declaration, invocation] = statement.thenStatement.statements;
      if (!declaration || !ts.isVariableStatement(declaration) || (declaration.declarationList.flags & ts.NodeFlags.Const) === 0 || declaration.declarationList.declarations.length !== 1 || !invocation || !ts.isExpressionStatement(invocation)) return false;
      const binding = declaration.declarationList.declarations[0]!;
      if (!ts.isObjectBindingPattern(binding.name) || binding.name.elements.length !== 1 || !binding.initializer || !ts.isAwaitExpression(binding.initializer)) return false;
      const member = binding.name.elements[0]!;
      if (!ts.isIdentifier(member.name) || member.propertyName && !ts.isIdentifier(member.propertyName) || member.initializer || member.dotDotDotToken || top.resolve(member.name.text)) return false;
      const module = binding.initializer.expression;
      if (!ts.isCallExpression(module) || module.expression.kind !== ts.SyntaxKind.ImportKeyword || module.arguments.length !== 1 || !ts.isStringLiteral(module.arguments[0]!)) return false;
      const literal = module.arguments[0]!, path = literal.text, segments = path.split("/");
      if (!/^\.{1,2}\//u.test(path) || /[\\%?#]/u.test(literal.getText(source))) return false;
      while (segments[0] === "." || segments[0] === "..") segments.shift();
      if (segments.some((segment) => !segment || segment === "." || segment === "..") || !segments.slice(0, -1).some((segment) => segment === "🧪️tests" || segment === "🧪️")) return false;
      if (!ts.isAwaitExpression(invocation.expression) || !ts.isCallExpression(invocation.expression.expression)) return false;
      const call = invocation.expression.expression, nested = new Scope(top);
      nested.define(member.name.text, { kind: "import-value", imported: member.propertyName?.text ?? member.name.text, module: "dynamic-test" });
      return ts.isIdentifier(call.expression) && call.expression.text === member.name.text && call.arguments.length === 3 && importMetaMember(call.arguments[0]!, "vitest") && call.arguments.slice(1).every((argument) => testDependency(argument, nested));
    };
    let environmentDefaults = 0, testRegistrations = 0;
    if (!prebind(source.statements.filter((statement) => !ts.isImportDeclaration(statement)), top)) return "unresolved";
    for (const statement of source.statements) {
      if (ts.isImportDeclaration(statement)) continue;
      if (ts.isClassDeclaration(statement)) {
        if (!statement.name || ts.getModifiers(statement)?.some((modifier) => modifier.kind === ts.SyntaxKind.ExportKeyword) || ts.canHaveDecorators(statement) && (ts.getDecorators(statement)?.length ?? 0) > 0 || statement.members.length !== 1) return "unresolved";
        const base = statement.heritageClauses?.flatMap((clause) => clause.types).map((row) => row.expression);
        const binding = base?.length === 1 && ts.isIdentifier(base[0]!) ? top.resolve(base[0]!.text) : undefined;
        const method = statement.members[0];
        if (binding?.kind !== "import-value" || !["BundleScript", "Script"].includes(binding.imported ?? "") || !method || !ts.isMethodDeclaration(method) || method.name.getText(source) !== "run" || !method.body || ts.canHaveDecorators(method) && (ts.getDecorators(method)?.length ?? 0) > 0 || method.parameters.some((parameter) => parameter.initializer || !ts.isIdentifier(parameter.name) || ts.canHaveDecorators(parameter) && (ts.getDecorators(parameter)?.length ?? 0) > 0) || !top.initialize(statement.name.text, { kind: "class" })) return "unresolved";
        const nested = new Scope(top);
        for (const parameter of method.parameters) if (!nested.define((parameter.name as ts.Identifier).text, { kind: "parameter" })) return "unresolved";
        if (!block(method.body.statements, nested)) return "unresolved";
        classes.add(statement.name.text);
        continue;
      }
      if (ts.isVariableStatement(statement)) {
        if ((statement.declarationList.flags & ts.NodeFlags.Const) === 0) return "unresolved";
        for (const declaration of statement.declarationList.declarations) {
          if (!declaration.initializer) return "unresolved";
          if (router(declaration.initializer, top)) {
            if (!ts.isIdentifier(declaration.name) || !top.initialize(declaration.name.text, { kind: "router" })) return "unresolved";
          } else if (value(declaration.initializer, top) === "module" || !define(declaration, top)) return "unresolved";
        }
        continue;
      }
      if (ts.isIfStatement(statement) && ts.isPropertyAccessExpression(statement.expression) && statement.expression.name.text === "main" && ts.isMetaProperty(statement.expression.expression) && statement.expression.expression.keywordToken === ts.SyntaxKind.ImportKeyword && statement.expression.expression.name.text === "meta") {
        if (statement.elseStatement) return "unresolved";
        const nested = new Scope(top), rows = ts.isBlock(statement.thenStatement) ? statement.thenStatement.statements : [statement.thenStatement];
        if (!prebind(rows, nested)) return "unresolved";
        for (const row of rows) {
          if (ts.isVariableStatement(row) && (row.declarationList.flags & ts.NodeFlags.Const) !== 0 && row.declarationList.declarations.length === 1 && row.declarationList.declarations[0]!.initializer && router(row.declarationList.declarations[0]!.initializer!, nested) && ts.isIdentifier(row.declarationList.declarations[0]!.name)) {
            if (!nested.initialize(row.declarationList.declarations[0]!.name.text, { kind: "router" })) return "unresolved";
          } else if (!terminal(row, nested, true)) return "unresolved";
        }
        continue;
      }
      if (ts.isIfStatement(statement) && importMetaMember(statement.expression, "vitest")) {
        if (testRegistrations++ !== 0 || !testRegistration(statement)) return "unresolved";
        continue;
      }
      if (ts.isIfStatement(statement) && environmentGuard(statement, top) || environmentDefault(statement, top)) {
        if (environmentDefaults++ !== 0 || terminals !== 0) return "unresolved";
        continue;
      }
      if (!terminal(statement, top)) return "unresolved";
    }
    return terminals === 1 && [...classes].every((name) => wired.has(name)) ? "tool-metadata" : "unresolved";
  }
  if (validator !== "tool-config-vitest") return "unresolved";
  const hasModule = source.statements.some((statement) => ts.isImportDeclaration(statement) && ts.isStringLiteral(statement.moduleSpecifier) && statement.moduleSpecifier.text === "vitest/config");
  const declarations = new Map<string, ts.Node>(), conditionals: ts.IfStatement[] = [];
  let root: ts.ExportAssignment | undefined;
  for (const statement of source.statements) {
    if (ts.isImportDeclaration(statement)) continue;
    if (ts.isClassDeclaration(statement) || ts.isInterfaceDeclaration(statement) || ts.isTypeAliasDeclaration(statement) || ts.isEnumDeclaration(statement) || ts.isModuleDeclaration(statement)) return "unresolved";
    if (ts.getModifiers(statement)?.some((modifier) => modifier.kind === ts.SyntaxKind.ExportKeyword)) return "unresolved";
    if (ts.isFunctionDeclaration(statement) && statement.name) { declarations.set(statement.name.text, statement); continue; }
    if (ts.isVariableStatement(statement) && (statement.declarationList.flags & ts.NodeFlags.Const) !== 0 && statement.declarationList.declarations.every((row) => ts.isIdentifier(row.name))) {
      for (const declaration of statement.declarationList.declarations) declarations.set((declaration.name as ts.Identifier).text, statement);
      continue;
    }
    if (ts.isIfStatement(statement)) { conditionals.push(statement); continue; }
    if (ts.isExportAssignment(statement) && !statement.isExportEquals && ts.isCallExpression(statement.expression) && statement.expression.expression.getText(source) === "defineConfig") { if (root) return "unresolved"; root = statement; continue; }
    return "unresolved";
  }
  if (!hasModule || !root || calls(source).some((name) => /^(?:fetch|writeFile\w*|appendFile\w*|rm\w*|unlink\w*|spawn\w*|exec\w*|listen|connect)$/u.test(name))) return "unresolved";
  const identifiers = (node: ts.Node): readonly string[] => {
    const names: string[] = [];
    const visit = (child: ts.Node): void => { if (ts.isIdentifier(child)) names.push(child.text); ts.forEachChild(child, visit); };
    visit(node);
    return names;
  };
  const referenced = new Set(identifiers(root).filter((name) => declarations.has(name)));
  for (let size = -1; size !== referenced.size;) {
    size = referenced.size;
    for (const name of [...referenced]) for (const dependency of identifiers(declarations.get(name)!)) if (declarations.has(dependency)) referenced.add(dependency);
  }
  if ([...declarations.keys()].some((name) => !referenced.has(name))) return "unresolved";
  const conditional = (statement: ts.IfStatement): boolean => !statement.elseStatement && ts.isBlock(statement.thenStatement) && statement.thenStatement.statements.every((row) => {
    if (!ts.isExpressionStatement(row) || !ts.isCallExpression(row.expression) || !ts.isPropertyAccessExpression(row.expression.expression) || row.expression.expression.name.text !== "push") return false;
    const target = row.expression.expression.expression;
    return ts.isIdentifier(target) && referenced.has(target.text);
  });
  return conditionals.every(conditional) ? "tool-metadata" : "unresolved";
}

function ecmaTypeScriptSemanticEvidence(rows: readonly { readonly id: string; readonly content: string }[]): { readonly diagnostics: ReadonlyMap<string, readonly number[]>; readonly executeBindings: ReadonlyMap<string, readonly ("import" | "local")[]> } {
  const options: ts.CompilerOptions = { allowImportingTsExtensions: true, module: ts.ModuleKind.ESNext, moduleResolution: ts.ModuleResolutionKind.Bundler, noEmit: true, skipLibCheck: true, strict: true, target: ts.ScriptTarget.ESNext, types: [] };
  const sources = new Map(rows.map((row) => [`/${row.id}.ts`, row.content])), files = new Map<string, string>([
    ...sources,
    ["/📜️script.ts", "export abstract class Script { abstract run(...args: any[]): unknown }\nexport abstract class BundleScript extends Script {}\nexport declare class ScriptRouter { constructor(...args: any[]); register(...args: any[]): this; run(segments: string[]): Promise<void> }\nexport declare function runBundleScriptMain(...args: any[]): Promise<void>;\nexport declare function runWorkspaceScriptMain(...args: any[]): Promise<void>;\nexport declare function runPolicyOnlyMain(...args: any[]): Promise<void>;\nexport declare function runArtifactRustPackageMain(...args: any[]): Promise<void>;\nexport declare function runArtifactTypeScriptPackageMain(...args: any[]): Promise<void>;\nexport declare function execute(...args: any[]): any;\n"],
    ["/owner/🟦️.ts", "export abstract class Script { abstract run(...args: any[]): unknown }\nexport abstract class BundleScript extends Script {}\nexport declare class ScriptRouter { constructor(...args: any[]); register(...args: any[]): this; run(segments: string[]): Promise<void> }\nexport declare class MaterializeScript {}\nexport declare class SupportScript {}\nexport declare function runBundleScriptMain(...args: any[]): Promise<void>;\nexport declare function runWorkspaceScriptMain(...args: any[]): Promise<void>;\nexport declare function runArtifactRustPackageMain(...args: any[]): Promise<void>;\nexport declare function runArtifactTypeScriptPackageMain(...args: any[]): Promise<void>;\nexport declare function execute(...args: any[]): any;\nexport declare function prepare(...args: any[]): any;\nexport declare function dirname(value: string): string;\nexport declare function fileURLToPath(value: string): string;\n"],
    ["/🧪️tests/🟦️.ts", "export declare function registerTests1(vitest: NonNullable<ImportMeta[\"vitest\"]>, dependencies: Readonly<Record<string, unknown>>, source: { readonly directory: string; readonly url: string }): Promise<void>;\n"],
    ["/node-path.d.ts", "export declare function dirname(value: string): string;\n"],
    ["/node-url.d.ts", "export declare function fileURLToPath(value: string): string;\n"],
    ["/ambient.d.ts", "declare const console: { log(...values: unknown[]): void; error(...values: unknown[]): void };\ndeclare const process: { exit(value?: unknown): void; argv: string[]; env: Record<string, string | undefined> };\ninterface ImportMeta { readonly dir: string; readonly main: boolean; readonly url: string; readonly vitest?: { readonly suite: string } }\n"],
  ]);
  const base = ts.createCompilerHost(options), normalize = (path: string): string => path.replaceAll("\\", "/");
  const host: ts.CompilerHost = {
    ...base,
    fileExists: (path) => files.has(normalize(path)) || base.fileExists(path),
    readFile: (path) => files.get(normalize(path)) ?? base.readFile(path),
    getSourceFile: (path, languageVersion) => {
      const value = files.get(normalize(path));
      return value === undefined ? base.getSourceFile(path, languageVersion) : ts.createSourceFile(path, value, languageVersion, true, ts.ScriptKind.TS);
    },
    resolveModuleNames: (names, containingFile) => names.map((name) => name === "./🧪️tests/🟦️.ts" ? { resolvedFileName: "/🧪️tests/🟦️.ts", extension: ts.Extension.Ts, isExternalLibraryImport: false } : name === "node:path" ? { resolvedFileName: "/node-path.d.ts", extension: ts.Extension.Dts, isExternalLibraryImport: true } : name === "node:url" ? { resolvedFileName: "/node-url.d.ts", extension: ts.Extension.Dts, isExternalLibraryImport: true } : name === "./owner/🟦️.ts" ? { resolvedFileName: "/owner/🟦️.ts", extension: ts.Extension.Ts, isExternalLibraryImport: false } : name === "./📜️script.ts" ? { resolvedFileName: "/📜️script.ts", extension: ts.Extension.Ts, isExternalLibraryImport: false } : ts.resolveModuleName(name, containingFile, options, host).resolvedModule),
    writeFile: () => {},
  };
  const program = ts.createProgram({ rootNames: [...files.keys()], options, host }), checker = program.getTypeChecker(), diagnostics = new Map<string, readonly number[]>(), executeBindings = new Map<string, readonly ("import" | "local")[]>();
  for (const row of rows) {
    const filename = `/${row.id}.ts`;
    const source = program.getSourceFile(filename)!;
    diagnostics.set(row.id, [...new Set(ts.getPreEmitDiagnostics(program, source).map((diagnostic) => diagnostic.code))].sort((left, right) => left - right));
    const bindings = new Set<"import" | "local">();
    const visit = (node: ts.Node): void => {
      if (ts.isIdentifier(node) && node.text === "execute" && ts.isCallExpression(node.parent) && node.parent.expression === node) {
        const symbol = checker.getSymbolAtLocation(node), target = symbol && (symbol.flags & ts.SymbolFlags.Alias) !== 0 ? checker.getAliasedSymbol(symbol) : symbol;
        if (target) bindings.add(target.declarations?.some((declaration) => declaration.getSourceFile() === source) ? "local" : "import");
      }
      ts.forEachChild(node, visit);
    };
    visit(source);
    executeBindings.set(row.id, [...bindings].sort());
  }
  return { diagnostics, executeBindings };
}

function nativeOracle(row: (typeof vectors.glueRoleCases)[number], root: string): PackageSourceRole {
  const extension = row.oracle === "rust-syn" ? "rs" : row.oracle === "go-parser" ? "go" : row.oracle === "python-parser" ? "py" : "c";
  const source = join(root, `source.${extension}`);
  writeFileSync(source, row.content);
  const oracleRoot = resolve(import.meta.dir, "🔮️oracles");
  const rustOraclePackageRoot = join(oracleRoot, "📦️packages/🦀️rust");
  const run = row.oracle === "rust-syn"
    ? Bun.spawnSync(["cargo", "run", "--quiet", "--manifest-path", join(rustOraclePackageRoot, "Cargo.toml"), "--", source, String(row.maxDelegationStatements)], { cwd: rustOraclePackageRoot, env: { ...process.env, CARGO_TARGET_DIR: join(root, "target") }, stdout: "pipe", stderr: "pipe" })
    : row.oracle === "go-parser"
      ? Bun.spawnSync(["go", "run", join(oracleRoot, "🐹️.go"), "--", source, String(row.maxDelegationStatements)], { cwd: root, stdout: "pipe", stderr: "pipe" })
      : row.oracle === "python-parser"
        ? Bun.spawnSync(["python3", join(oracleRoot, "🐍️.py"), source], { cwd: root, stdout: "pipe", stderr: "pipe" })
        : Bun.spawnSync(["cc", "-dM", "-E", "-x", "c", source], { cwd: root, stdout: "pipe", stderr: "pipe" });
  expect(run.exitCode, run.stderr.toString()).toBe(0);
  if (row.oracle === "c-compiler") {
    const macros = run.stdout.toString().split("\n").filter((line) => /^#define\s+(?:COMPUTE\b|SEMIO_)/u.test(line));
    return macros.some((line) => /^#define\s+\w+\s*\(/u.test(line) || /[+*/%]|\s-\s/u.test(line)) ? "implementation" : "declaration";
  }
  return run.stdout.toString() as PackageSourceRole;
}
//#endregion 🧬️Contract

//#region 🧪️Classification
describe("package boundary glue-content classification", () => {
  test("fixture vectors satisfy the independent schema implementation", () => {
    expect(validateVectors(vectors), JSON.stringify(validateVectors.errors)).toBe(true);
    const ids = vectors.glueRoleCases.map((row) => row.id);
    expect(new Set(ids).size).toBe(ids.length);
    expect(new Set(vectors.fixedScriptCases.map((row) => row.id)).size).toBe(vectors.fixedScriptCases.length);
    expect(new Set(vectors.fixedScriptReadFailureCases.map((row) => row.id)).size).toBe(vectors.fixedScriptReadFailureCases.length);
  });

  test("fixed script contracts distinguish filenames from semantic bodies", () => {
    const taxonomy = loadCatalogTaxonomy(), disposition = taxonomy.packageSourceDispositions["root-script"]!;
    const grammar = taxonomy.packageGlueGrammar[disposition.grammarId!]!;
    for (const row of vectors.fixedScriptCases) {
      expect(fixedFilenameContractIdsForPath(row.path, taxonomy), row.id).toContain("root-script");
      expect(classifyPackageSourceDisposition(row.content, disposition, grammar), row.id).toBe(row.expectedDispositionRole);
      expect(ecmaDispositionOracle(row.content, "typescript", "command-router", grammar.maxDelegationStatements), row.id).toBe(row.expectedDispositionRole);
      expect(fixedSourceDispositionDecision("root-script", row.content, taxonomy)?.finding ?? null, row.id).toBe(row.expectedFinding);
    }
    for (const row of vectors.fixedScriptCases.filter((entry) => entry.placement === "package")) {
      const root = fixedScriptControlRoot("package-");
      const packageRoot = join(root, "🧰️framework/🔨️modules/🧭️fixture/📦️packages/🟦️typescript");
      try {
        mkdirSync(packageRoot, { recursive: true });
        if (row.manifest === "present") writeFileSync(join(packageRoot, "package.json"), JSON.stringify({ name: "@semio/fixed-script", semio: { role: "framework", id: row.id } }));
        writeFileSync(join(packageRoot, "📜️script.ts"), row.content);
        clearDiscoveryCache();
        const problem = discoverPackageProblems(root, taxonomy).find((entry) => entry.path.endsWith("/📜️script.ts"));
        expect(problem?.kind ?? null, row.id).toBe(row.expectedFinding ? "package-role-unresolved" : null);
      } finally {
        clearDiscoveryCache();
        rmSync(root, { recursive: true, force: true });
      }
    }
    const semanticRows = vectors.fixedScriptCases.filter((row): row is typeof row & { expectedSemanticDiagnostics: readonly number[] } => row.expectedSemanticDiagnostics !== undefined);
    const semantic = ecmaTypeScriptSemanticEvidence(semanticRows);
    for (const row of semanticRows) expect(semantic.diagnostics.get(row.id), row.id).toEqual(row.expectedSemanticDiagnostics);
    expect(fixedFilenameContractIdsForPath("📜️script.tsx", taxonomy)).not.toContain("root-script");
    expect(fixedSourceDispositionDecision("root-package", "{}", taxonomy)).toBeNull();
    expect(fixedSourceDispositionDecision("root-script", null, taxonomy)?.finding).toBe("fixed-source-content-unreadable");
  }, 30_000);

  test("actual root domain and package scripts receive independent fixed-source findings", () => {
    const taxonomy = loadCatalogTaxonomy(), disposition = taxonomy.packageSourceDispositions["root-script"]!;
    const grammar = taxonomy.packageGlueGrammar[disposition.grammarId!]!;
    const paths = [
      "📜️script.ts",
      "♻️mit-bestand/🧺️demonstrator/📜️script.ts",
      "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts",
      "🧰️framework/🛍️products/📓️print/🔨️modules/🔤print-font-catalog/📜️script.ts",
      "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/📜️script.ts",
    ] as const;
    for (const path of paths) {
      const content = readFileSync(join(repoRoot, path), "utf8");
      const expectedRole = ecmaDispositionOracle(content, "typescript", disposition.validator, grammar.maxDelegationStatements);
      const expectedFinding = expectedRole === "tool-metadata" ? null : "fixed-source-disposition-unresolved";
      const progress: string[] = [];
      const inventory = inventoryTaxonomy({ repoRoot, scope: path, workers: 1, progress: (row) => progress.push(`${row.operation}:${row.phase}`) });
      const entry = inventory.entries.find((row) => row.sourcePath === path)!;
      expect(entry.fixedContractId).toBe("root-script");
      expect(entry.fileKind).toBeNull();
      expect(entry.packageRole).toBe(path.includes("/📦️packages/") ? "configuration" : "not-package");
      expect(entry.violations.some((row) => row.code === "fixed-source-disposition-unresolved")).toBe(expectedFinding !== null);
      expect(fixedSourceDispositionDecision("root-script", content, taxonomy)?.finding ?? null).toBe(expectedFinding);
      expect(progress.some((row) => row.startsWith("inventory:"))).toBe(true);
    }
  }, 120_000);

  test("fixed-source inventory keeps cancellation fail-fast", () => {
    const control = fixedScriptControlRoot("cancellation-"), cancelFile = join(control, "cancel");
    try {
      writeFileSync(cancelFile, "cancel\n");
      expect(() => inventoryTaxonomy({ repoRoot, scope: "📜️script.ts", workers: 1, cancelFile })).toThrow("Taxonomy operation cancelled");
    } finally {
      rmSync(control, { recursive: true, force: true });
    }
  });

  test("captured fixed-source read failure preserves structural authority and reports both diagnostics", () => {
    for (const row of vectors.fixedScriptReadFailureCases) {
      const control = fixedScriptControlRoot("read-failure-"), source = join(control, row.path);
      const ticketDir = relative(repoRoot, control).split(sep).join("/"), sourcePath = relative(repoRoot, source).split(sep).join("/");
      try {
        writeFileSync(source, row.content);
        const inventory = inventoryTaxonomyWithCapturedSourceRead({ repoRoot, scope: sourcePath, ticketDir, workers: 1 }, (path) => {
          if (path === source) throw new Error("injected captured-source read failure");
          return readFileSync(path);
        });
        const entry = inventory.entries.find((candidate) => candidate.sourcePath === sourcePath)!;
        expect(entry.fixedContractId, row.id).toBe(row.expectedFixedContractId);
        expect(entry.fileKind, row.id).toBe(row.expectedFileKind);
        expect(entry.packageRole, row.id).toBe(row.expectedPackageRole);
        expect(entry.violations.map((violation) => violation.code).filter((code) => row.expectedViolations.includes(code as typeof row.expectedViolations[number])), row.id).toEqual(row.expectedViolations);
      } finally {
        rmSync(control, { recursive: true, force: true });
      }
    }
  }, 30_000);

  for (const row of vectors.glueRoleCases) test(row.id, () => {
    const grammar = { analyzer: row.analyzer, allowedRoles: ["declaration", "registration", "bootstrap", "thin-delegation"] as const, maxDelegationStatements: row.maxDelegationStatements };
    const decision = classifyPackageSource(row.content, grammar);
    expect(decision).toEqual({ role: row.expectedRole, evidence: row.expectedEvidence });
    if (row.dispositionValidator) {
      const disposition = { contractKind: "fixed", disposition: row.dispositionValidator === "package-glue" ? "adapter-source" : "tool-metadata", validator: row.dispositionValidator, authority: "portable fixture", verification: "independent parser" } as const;
      expect(classifyPackageSourceDisposition(row.content, disposition, grammar)).toBe(row.expectedDispositionRole);
      if (row.oracle === "typescript-compiler") expect(ecmaDispositionOracle(row.content, row.analyzer as "typescript" | "javascript", row.dispositionValidator, row.maxDelegationStatements)).toBe(row.expectedDispositionRole);
    }
    if (row.oracle === "typescript-compiler") expect(ecmaOracle(row.content, row.analyzer as "typescript" | "javascript", row.maxDelegationStatements)).toBe(row.expectedRole);
    else {
      const root = mkdtempSync(join(tmpdir(), "semio-package-body-"));
      try { expect(nativeOracle(row, root)).toBe(row.expectedRole); }
      finally { rmSync(root, { recursive: true, force: true }); }
    }
  }, row.oracle === "typescript-compiler" ? 5_000 : 30_000);

  test("runtime value imports, type-only imports, and later lexical bindings follow TypeScript semantics", () => {
    const rows = vectors.glueRoleCases.filter((row): row is typeof row & { expectedSemanticDiagnostics: readonly number[] } => row.expectedSemanticDiagnostics !== undefined);
    const evidence = ecmaTypeScriptSemanticEvidence(rows);
    for (const row of rows) {
      expect(evidence.diagnostics.get(row.id), row.id).toEqual(row.expectedSemanticDiagnostics);
      if (row.expectedExecuteBinding) expect(evidence.executeBindings.get(row.id)).toEqual([row.expectedExecuteBinding]);
    }
  }, 20_000);

  test("exact filename and placement authority still inspect package body ownership", () => {
    const root = mkdtempSync(join(tmpdir(), "semio-package-boundary-"));
    const packageRoot = join(root, "🧰️framework/🔨️modules/🎫️ticket/📦️packages/🟦️typescript");
    const rustPackageRoot = join(root, "🧰️framework/🔨️modules/🎫️ticket/📦️packages/🦀️rust");
    const write = (path: string, content: string) => {
      const target = join(packageRoot, path);
      mkdirSync(resolve(target, ".."), { recursive: true });
      writeFileSync(target, content);
    };
    const writeRust = (path: string, content: string) => {
      const target = join(rustPackageRoot, path);
      mkdirSync(resolve(target, ".."), { recursive: true });
      writeFileSync(target, content);
    };
    try {
      write("package.json", JSON.stringify({ name: "@semio/fixture", semio: { role: "framework", id: "package-body-fixture" } }));
      write("app/api/thin/route.ts", 'export { handleTicket as GET } from "../../../../🎫️ticket/🟦️.ts";\n');
      write("app/api/body/route.ts", 'export async function POST(request: Request) { const body = await request.json(); return Response.json({ id: crypto.randomUUID(), body }); }\n');
      write("🟦️.ts", "export interface Ticket { readonly id: string }\n");
      writeRust("Cargo.toml", '[package]\nname = "package-body-rust-fixture"\nversion = "0.0.0"\n[package.metadata.semio]\nrole = "framework"\nid = "package-body-rust-fixture"\n');
      writeRust("📜️script.ts", 'import { runBundleScriptMain } from "@semio/process";\nawait runBundleScriptMain(import.meta);\n');
      clearDiscoveryCache();
      const roles = discoverPackageProblems(root, loadCatalogTaxonomy()).filter((row) => row.kind === "package-implementation" || row.kind === "package-role-unresolved");
      expect(roles.map((row) => row.path).sort()).toEqual([
        "🧰️framework/🔨️modules/🎫️ticket/📦️packages/🟦️typescript/app/api/body/route.ts",
        "🧰️framework/🔨️modules/🎫️ticket/📦️packages/🟦️typescript/🟦️.ts",
      ].sort());
      expect(roles.every((row) => row.kind === "package-implementation" && row.message.includes(": "))).toBe(true);
      writeRust("📜️script.ts", 'import { runBundleScriptMain } from "@semio/process";\nclass Domain { compute() { return 42; } }\nawait runBundleScriptMain(import.meta);\n');
      clearDiscoveryCache();
      expect(discoverPackageProblems(root, loadCatalogTaxonomy()).some((row) => row.kind === "package-implementation" && row.path.endsWith("/🦀️rust/📜️script.ts"))).toBe(true);
    } finally {
      clearDiscoveryCache();
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("manifest presence never controls package-body ownership inspection", () => {
    for (const row of vectors.manifestPresenceCases) {
      const root = mkdtempSync(join(tmpdir(), "semio-manifest-presence-"));
      const packageRoot = join(root, "🧰️framework/🔨️modules/🎫️ticket/📦️packages/🟦️typescript");
      try {
        mkdirSync(packageRoot, { recursive: true });
        if (row.manifest === "present") writeFileSync(join(packageRoot, "package.json"), JSON.stringify({ name: "@semio/fixture", semio: { role: "framework", id: row.id } }));
        writeFileSync(join(packageRoot, "🟦️.ts"), row.content);
        clearDiscoveryCache();
        const problems = discoverPackageProblems(root, loadCatalogTaxonomy());
        const body = problems.find((problem) => problem.path.endsWith("/🟦️typescript/🟦️.ts"));
        expect(body?.kind ?? null).toBe(row.expectedProblem);
        expect(ecmaOracle(row.content, "typescript", 32)).toBe(row.expectedProblem ? "implementation" : "declaration");
        if (row.manifest === "absent") expect(problems.some((problem) => problem.kind === "manifest-without-marker")).toBe(false);
      } finally {
        clearDiscoveryCache();
        rmSync(root, { recursive: true, force: true });
      }
    }
  });

  test("a rejected fixed command router cannot fall through to generic package glue", () => {
    const root = mkdtempSync(join(tmpdir(), "semio-command-router-fallback-"));
    const packageRoot = join(root, "🧰️framework/🔨️modules/🎫️ticket/📦️packages/🟦️typescript");
    try {
      mkdirSync(packageRoot, { recursive: true });
      writeFileSync(join(packageRoot, "package.json"), JSON.stringify({ name: "@semio/fixture", semio: { role: "framework", id: "command-router-fallback" } }));
      writeFileSync(join(packageRoot, "📜️script.ts"), 'import type { runArtifactRustPackageMain } from "./owner/🟦️.ts";\nawait runArtifactRustPackageMain(import.meta.dir, "fixture");\n');
      clearDiscoveryCache();
      const rejected = discoverPackageProblems(root, loadCatalogTaxonomy()).find((row) => row.path.endsWith("/📜️script.ts"));
      expect(rejected?.kind).toBe("package-role-unresolved");
      expect(rejected?.message).toContain("command-router validator rejected the source body");
      writeFileSync(join(packageRoot, "📜️script.ts"), 'import type { execute } from "./owner/🟦️.ts";\nawait execute();\n');
      clearDiscoveryCache();
      const rejectedDelegate = discoverPackageProblems(root, loadCatalogTaxonomy()).find((row) => row.path.endsWith("/📜️script.ts"));
      expect(rejectedDelegate?.kind).toBe("package-role-unresolved");
      expect(rejectedDelegate?.message).toContain("command-router validator rejected the source body");
      writeFileSync(join(packageRoot, "📜️script.ts"), 'import { runArtifactRustPackageMain } from "./owner/🟦️.ts";\nawait runArtifactRustPackageMain(import.meta.dir, "fixture");\n');
      clearDiscoveryCache();
      expect(discoverPackageProblems(root, loadCatalogTaxonomy()).some((row) => row.path.endsWith("/📜️script.ts"))).toBe(false);
      const pluginRouter = readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/📜️script.ts"), "utf8");
      writeFileSync(join(packageRoot, "📜️script.ts"), pluginRouter);
      clearDiscoveryCache();
      expect(discoverPackageProblems(root, loadCatalogTaxonomy()).some((row) => row.path.endsWith("/📜️script.ts"))).toBe(false);
    } finally {
      clearDiscoveryCache();
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("live tool entries distinguish configuration and orchestration from hidden package bodies", () => {
    const taxonomy = loadCatalogTaxonomy();
    const classify = (path: string, dispositionId: "vitest-config-entry" | "root-script") => {
      const disposition = taxonomy.packageSourceDispositions[dispositionId]!;
      const grammar = taxonomy.packageGlueGrammar[disposition.grammarId ?? "typescript"]!;
      return classifyPackageSourceDisposition(readFileSync(join(repoRoot, path), "utf8"), disposition, grammar);
    };
    expect(classify("🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts", "vitest-config-entry")).toBe("tool-metadata");
    expect(classify("🧰️framework/🛍️products/💻️os/🎚️config/📦️packages/🦀️rust/📜️script.ts", "root-script")).toBe("tool-metadata");
    expect(classify("🧰️framework/🔨️modules/🔀️dispatch/📦️packages/🦀️rust/📜️script.ts", "root-script")).toBe("tool-metadata");
    expect(classify("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/📜️script.ts", "root-script")).toBe("tool-metadata");
    expect(classify("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts", "root-script")).toBe("unresolved");
  });

  test("scope-kind specificity is strictly increasing in the documented, narrowest-wins order", () => {
    const ranks = vectors.scopeSpecificityOrder.map((kind) => fixedContractScopeSpecificityRank(kind));
    for (let index = 1; index < ranks.length; index++) expect(ranks[index]).toBeGreaterThan(ranks[index - 1]);
  });

  test("sibling-fixed-filename-contract outranks package-root (the package.json/tsconfig.json ambiguity fix)", () => {
    expect(fixedContractScopeSpecificityRank("sibling-fixed-filename-contract")).toBeGreaterThan(fixedContractScopeSpecificityRank("package-root"));
  });

  test("the focused policy command is registered from package through both launch authorities", () => {
    const libraryPackageRoot = join(repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript");
    const project = JSON.parse(readFileSync(join(libraryPackageRoot, "📋️project.json"), "utf8"));
    const libraryPackage = JSON.parse(readFileSync(join(libraryPackageRoot, "package.json"), "utf8"));
    const workspacePackage = JSON.parse(readFileSync(join(repoRoot, "package.json"), "utf8"));
    const command = "bun nx run @semio-tech/repo-lib:test-package-body-policy";
    expect(project.targets["test-package-body-policy"].options.command).toBe("bun ./📜️script.ts test package-body-policy");
    expect(libraryPackage.scripts["test-package-body-policy"]).toBe("nx run @semio-tech/repo-lib:test-package-body-policy");
    expect(workspacePackage.scripts["test:repo-lib:package-body-policy"]).toBe(command);
    for (const path of [".vscode/🧩️launch.seed.jsonc", ".vscode/launch.json"]) {
      const launch = Bun.JSONC.parse(readFileSync(join(repoRoot, path), "utf8")) as { configurations: readonly { command?: string }[] };
      expect(launch.configurations.filter((entry) => entry.command === command)).toHaveLength(1);
    }
  });

  test("the remaining package-purity fixture stays registered as frozen history", () => {
    const contract = loadCatalogTaxonomy().frozenCoordinateEvidenceContracts["remaining-package-purity-history-v1"];
    expect(contract?.path).toBe("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🧼️remaining-package-purity-authority/🔣️.json");
    const fixture = JSON.parse(readFileSync(join(repoRoot, contract!.path), "utf8"));
    expect(fixture.authority).toBe("remaining-package-purity-owner-projection");
    expect(fixture.decisionState).toBe("non-authoritative-concurrent-source-byte-drift");
  });
});
//#endregion 🧪️Classification
