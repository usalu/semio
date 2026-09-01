import ts from "typescript";

//#region 🧬️FactTypes
type Structure = { readonly form: "object" | "union" | "reference" | "enum" | "class" | "unresolved"; readonly members: readonly string[]; readonly unresolved: string | null };
type Declaration = { readonly kind: "type" | "interface" | "enum" | "class" | "variable"; readonly name: string; readonly exported: boolean; readonly modulePath: readonly string[]; readonly span: { readonly start: number; readonly end: number }; readonly structure: Structure };
type Alias = { readonly relation: "import" | "reexport"; readonly typeOnly: boolean; readonly imported: string; readonly local: string; readonly moduleSpecifier: string; readonly modulePath: readonly string[]; readonly span: { readonly start: number; readonly end: number } };
type Diagnostic = { readonly code: string; readonly span: { readonly start: number; readonly end: number } };
export type Facts = { readonly completeness: "complete" | "incomplete"; readonly declarations: readonly Declaration[]; readonly aliases: readonly Alias[]; readonly diagnostics: readonly Diagnostic[] };
export type Vector = { readonly id: string; readonly language: "ts" | "tsx"; readonly sourceLines: readonly string[]; readonly expected: Facts };
//#endregion 🧬️FactTypes

//#region 🧪️TypeScriptCompilerOracle
const exported = (node: ts.Node): boolean => ts.canHaveModifiers(node) && !!ts.getModifiers(node)?.some((modifier) => modifier.kind === ts.SyntaxKind.ExportKeyword);
const span = (node: ts.Node, source: ts.SourceFile): { readonly start: number; readonly end: number } => ({ start: node.getStart(source, false), end: node.end });
const namedMembers = (members: readonly ts.TypeElement[] | readonly ts.ClassElement[] | readonly ts.EnumMember[] | readonly ts.ObjectLiteralElementLike[], source: ts.SourceFile): { readonly members: readonly string[]; readonly computed: boolean; readonly spreads: readonly ts.SpreadAssignment[] } => {
  const values: string[] = [], computed = members.some((member) => member.name !== undefined && ts.isComputedPropertyName(member.name)), spreads = members.filter(ts.isSpreadAssignment);
  for (const member of members) if (member.name !== undefined && !ts.isComputedPropertyName(member.name)) values.push(member.name.getText(source));
  return { members: values, computed, spreads };
};
const typeStructure = (node: ts.TypeNode, source: ts.SourceFile): Structure => {
  if (ts.isTypeLiteralNode(node)) { const members = namedMembers(node.members, source); return { form: "object", members: members.members, unresolved: members.computed ? "computed-property" : null }; }
  if (ts.isUnionTypeNode(node)) return { form: "union", members: node.types.map((member) => member.getText(source)), unresolved: node.types.some((member) => ts.isConditionalTypeNode(member) || ts.isMappedTypeNode(member) || (ts.isParenthesizedTypeNode(member) && (ts.isConditionalTypeNode(member.type) || ts.isMappedTypeNode(member.type)))) ? "conditional-or-mapped-union-member" : null };
  if (ts.isTypeReferenceNode(node)) { const value = node.getText(source); return value ? { form: "reference", members: [value], unresolved: null } : { form: "unresolved", members: [], unresolved: "unsupported-type" }; }
  if (ts.isConditionalTypeNode(node)) return { form: "unresolved", members: [], unresolved: "conditional" };
  if (ts.isMappedTypeNode(node)) return { form: "unresolved", members: [], unresolved: "mapped" };
  return { form: "unresolved", members: [], unresolved: "unsupported-type" };
};
const expressionStructure = (node: ts.Expression, source: ts.SourceFile): Structure => {
  const value = ts.isAsExpression(node) || ts.isTypeAssertionExpression(node) || ts.isSatisfiesExpression(node) ? node.expression : node;
  if (ts.isObjectLiteralExpression(value)) { const members = namedMembers(value.properties, source); return { form: "object", members: members.members, unresolved: members.spreads.length ? "object-spread" : members.computed ? "computed-property" : null }; }
  if (ts.isNoSubstitutionTemplateLiteral(value)) return { form: "unresolved", members: [], unresolved: "initializer:template-literal" };
  if (ts.isTemplateExpression(value)) return { form: "unresolved", members: [], unresolved: "initializer:template-interpolation" };
  if (ts.isStringLiteral(value)) return { form: "unresolved", members: [], unresolved: "initializer:string-literal" };
  if (ts.isRegularExpressionLiteral(value)) return { form: "unresolved", members: [], unresolved: "initializer:regex-literal" };
  if (ts.isJsxElement(value) || ts.isJsxSelfClosingElement(value) || ts.isJsxFragment(value)) return { form: "unresolved", members: [], unresolved: "initializer:jsx" };
  return { form: "unresolved", members: [], unresolved: "initializer:expression" };
};
/** 🧪️ Projects declaration facts independently through the test-only TypeScript compiler. */
export const compilerFacts = (sourceText: string, language: Vector["language"]): Facts => {
  const source = ts.createSourceFile(language === "tsx" ? "virtual.tsx" : "virtual.ts", sourceText, ts.ScriptTarget.Latest, true, language === "tsx" ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
  const declarations: Declaration[] = [], aliases: Alias[] = [], diagnostics: Diagnostic[] = [];
  const diagnose = (code: Diagnostic["code"], node: ts.Node): void => { diagnostics.push({ code, span: span(node, source) }); };
  const alias = (relation: Alias["relation"], typeOnly: boolean, element: ts.ImportSpecifier | ts.ExportSpecifier, moduleSpecifier: string, modulePath: readonly string[]): void => { aliases.push({ relation, typeOnly, imported: element.propertyName?.text ?? element.name.text, local: element.name.text, moduleSpecifier, modulePath, span: span(element, source) }); };
  const typeDiagnostics = (node: ts.TypeNode, direct = true): void => {
    if (ts.isUnionTypeNode(node)) { for (const member of node.types) typeDiagnostics(ts.isParenthesizedTypeNode(member) ? member.type : member, true); return; }
    if (ts.isConditionalTypeNode(node)) { diagnose("unresolved-conditional-type", node); return; }
    if (ts.isMappedTypeNode(node)) { diagnose("unresolved-mapped-type", node); return; }
    if (ts.isTypeLiteralNode(node)) { if (namedMembers(node.members, source).computed) diagnose("unresolved-computed-property", node); for (const member of node.members) if (ts.isPropertySignature(member) && member.type) typeDiagnostics(member.type, false); return; }
    if (ts.isTypeReferenceNode(node)) { for (const argument of node.typeArguments ?? []) typeDiagnostics(argument, false); return; }
    if (direct && node.getStart(source, false) !== node.end) diagnose("unsupported-type-node", node);
  };
  const visit = (statements: readonly ts.Statement[], modulePath: readonly string[]): void => {
    for (const statement of statements) {
      if (ts.isModuleDeclaration(statement)) {
        const body = statement.body;
        if (body && ts.isModuleBlock(body)) visit(body.statements, [...modulePath, statement.name.getText(source)]);
        else if (body && ts.isModuleDeclaration(body)) visit([body], [...modulePath, statement.name.getText(source)]);
        else diagnose("unsupported-ambient-module-body", statement);
      } else if (ts.isImportEqualsDeclaration(statement)) diagnose("unsupported-import-equals", statement);
      else if (ts.isImportDeclaration(statement)) {
        if (!statement.importClause?.namedBindings || !ts.isNamedImports(statement.importClause.namedBindings) || !ts.isStringLiteral(statement.moduleSpecifier)) diagnose("unsupported-default-or-namespace-import", statement);
        else {
          if (statement.importClause.name) diagnose("unsupported-default-or-namespace-import", statement.importClause.name);
          for (const element of statement.importClause.namedBindings.elements) alias("import", statement.importClause.isTypeOnly || element.isTypeOnly, element, statement.moduleSpecifier.text, modulePath);
        }
      } else if (ts.isExportDeclaration(statement)) {
        if (!statement.exportClause || !ts.isNamedExports(statement.exportClause) || !statement.moduleSpecifier || !ts.isStringLiteral(statement.moduleSpecifier)) diagnose("unsupported-export-star", statement);
        else for (const element of statement.exportClause.elements) alias("reexport", statement.isTypeOnly || element.isTypeOnly, element, statement.moduleSpecifier.text, modulePath);
      } else if (ts.isTypeAliasDeclaration(statement)) {
        const structure = typeStructure(statement.type, source);
        declarations.push({ kind: "type", name: statement.name.text, exported: exported(statement), modulePath, span: span(statement, source), structure });
        typeDiagnostics(statement.type);
      } else if (ts.isInterfaceDeclaration(statement)) { const members = namedMembers(statement.members, source), heritage = statement.heritageClauses?.length ? "heritage" : members.computed ? "computed-property" : null; declarations.push({ kind: "interface", name: statement.name.text, exported: exported(statement), modulePath, span: span(statement, source), structure: { form: "object", members: members.members, unresolved: heritage } }); for (const clause of statement.heritageClauses ?? []) diagnose("unresolved-heritage", clause); if (members.computed) diagnose("unresolved-computed-property", statement); for (const member of statement.members) if (ts.isPropertySignature(member) && member.type) typeDiagnostics(member.type, false); }
      else if (ts.isEnumDeclaration(statement)) { const members = namedMembers(statement.members, source); declarations.push({ kind: "enum", name: statement.name.text, exported: exported(statement), modulePath, span: span(statement, source), structure: { form: "enum", members: members.members, unresolved: members.computed ? "computed-property" : null } }); if (members.computed) diagnose("unresolved-computed-property", statement); }
      else if (ts.isClassDeclaration(statement) && statement.name) { const members = namedMembers(statement.members, source), bodies = statement.members.flatMap((member) => { const body = (ts.isMethodDeclaration(member) || ts.isConstructorDeclaration(member) || ts.isGetAccessorDeclaration(member) || ts.isSetAccessorDeclaration(member) || ts.isClassStaticBlockDeclaration(member)) ? member.body : undefined; return body && body.statements.length > 0 ? [body] : []; }), unresolved = bodies.length ? "class-member-body" : statement.heritageClauses?.length ? "heritage" : members.computed ? "computed-property" : null; declarations.push({ kind: "class", name: statement.name.text, exported: exported(statement), modulePath, span: span(statement, source), structure: { form: "class", members: members.members, unresolved } }); for (const clause of statement.heritageClauses ?? []) diagnose("unresolved-heritage", clause); for (const body of bodies) diagnose("unsupported-class-member-body", body); if (members.computed) diagnose("unresolved-computed-property", statement); for (const member of statement.members) if (ts.isPropertyDeclaration(member) && member.type) typeDiagnostics(member.type, false); }
      else if (ts.isClassDeclaration(statement)) diagnose("unsupported-anonymous-default-class", statement);
      else if (ts.isVariableStatement(statement)) for (const declaration of statement.declarationList.declarations) {
        if (!ts.isIdentifier(declaration.name)) { diagnose("unsupported-binding-pattern", declaration); continue; }
        const structure = declaration.initializer ? expressionStructure(declaration.initializer, source) : { form: "unresolved" as const, members: [], unresolved: "initializer:absent" };
        declarations.push({ kind: "variable", name: declaration.name.text, exported: exported(statement), modulePath, span: span(declaration, source), structure });
        if (structure.unresolved === "computed-property") diagnose("unresolved-computed-property", declaration.initializer!);
        if (structure.unresolved === "object-spread" && declaration.initializer) {
          const initializer = declaration.initializer, value = ts.isAsExpression(initializer) || ts.isTypeAssertionExpression(initializer) || ts.isSatisfiesExpression(initializer) ? initializer.expression : initializer;
          if (ts.isObjectLiteralExpression(value)) for (const member of value.properties) if (ts.isSpreadAssignment(member)) diagnose("unresolved-object-spread", member);
        }
        if (structure.unresolved === "initializer:jsx") diagnose("unresolved-jsx", declaration.initializer!);
        if (structure.unresolved === "initializer:expression" || structure.unresolved === "initializer:absent" || structure.unresolved === "initializer:template-interpolation") diagnose("unresolved-expression", declaration);
      } else if (ts.isFunctionDeclaration(statement)) diagnose("unsupported-function-local", statement);
      else if (!ts.isEmptyStatement(statement)) diagnose("unsupported-module-statement", statement);
    }
  };
  visit(source.statements, []);
  for (const diagnostic of (source as ts.SourceFile & { readonly parseDiagnostics: readonly ts.Diagnostic[] }).parseDiagnostics) diagnostics.push({ code: "parse-error", span: { start: diagnostic.start ?? 0, end: (diagnostic.start ?? 0) + (diagnostic.length ?? 0) } });
  return { completeness: diagnostics.length === 0 ? "complete" : "incomplete", declarations, aliases, diagnostics };
};
//#endregion 🧪️TypeScriptCompilerOracle

//#region 🧪️MalformedSourceOracle
/** 🚧️ Preserves independent compiler syntax-error codes and exact raw-source coordinates. */
export function compilerParseDiagnostics(sourceText: string, language: Vector["language"]): readonly { readonly code: number; readonly start: number; readonly length: number }[] {
  const source = ts.createSourceFile(language === "tsx" ? "virtual.tsx" : "virtual.ts", sourceText, ts.ScriptTarget.Latest, true, language === "tsx" ? ts.ScriptKind.TSX : ts.ScriptKind.TS) as ts.SourceFile & { readonly parseDiagnostics: readonly ts.Diagnostic[] };
  return source.parseDiagnostics.map((diagnostic) => ({ code: diagnostic.code, start: diagnostic.start ?? -1, length: diagnostic.length ?? -1 }));
}
//#endregion 🧪️MalformedSourceOracle

//#region 🧪️StrictSourceTypes
/** 🔎️ Checks one exact virtual source with strict compiler options and no emitted files. */
export function strictSourceDiagnostics(sourceText: string, filePath: string): readonly { readonly code: number; readonly start: number; readonly end: number; readonly message: string }[] {
  const options: ts.CompilerOptions = { target: ts.ScriptTarget.ESNext, module: ts.ModuleKind.ESNext, moduleResolution: ts.ModuleResolutionKind.Bundler, strict: true, noEmit: true, skipLibCheck: true, allowImportingTsExtensions: true, esModuleInterop: true, types: [] };
  const host = ts.createCompilerHost(options), original = host.getSourceFile;
  host.getSourceFile = (path, version, onError, fresh) => path === filePath ? ts.createSourceFile(path, sourceText, version, true, ts.ScriptKind.TS) : original(path, version, onError, fresh);
  const program = ts.createProgram([filePath], options, host), source = program.getSourceFile(filePath);
  if (!source) throw new Error("Strict declaration source was not loaded");
  return [...program.getSyntacticDiagnostics(source), ...program.getSemanticDiagnostics(source)].map((diagnostic) => ({ code: diagnostic.code, start: diagnostic.start ?? 0, end: (diagnostic.start ?? 0) + (diagnostic.length ?? 0), message: ts.flattenDiagnosticMessageText(diagnostic.messageText, "\n") }));
}
//#endregion 🧪️StrictSourceTypes
