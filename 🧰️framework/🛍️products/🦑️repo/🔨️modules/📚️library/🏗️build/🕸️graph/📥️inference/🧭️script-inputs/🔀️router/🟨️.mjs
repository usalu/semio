/** 🔀️ The module specifiers a router script needs to run one of its registered commands: the router names its commands with `.register("name", Command)` and `.registerLazy("name", load)`, and the script runs exactly one of them, so what only the other commands use (their imports, their dynamic loads, their bodies) cannot change the output of the selected one; every construct this reading does not fully understand answers `undefined`, which keeps the whole closure. See `./🧪️tests/🟦️.ts` for the law and `./🧪️tests/🐍️oracle/🐍️.py` for the independent oracle. */
const REGISTRATIONS = new Set(["register", "registerLazy"]);

/** 🧭️ Reads the selected command of a router script.
 * @param {any} compiler the TypeScript compiler module
 * @param {string} path
 * @param {string} text
 * @param {string} command
 * @returns {ReadonlySet<string> | undefined} the specifiers of the imports the command needs, or nothing when the script is not a router this reading understands
 */
export function routerCommandSpecifiersV1(compiler, path, text, command) {
  const ts = compiler, file = ts.createSourceFile(path, text, ts.ScriptTarget.Latest, true);
  if (file.parseDiagnostics?.length) return undefined;
  const bindings = new Map(), kept = new Set(), declarations = new Map(), roots = [], classes = [], skipped = new Set(), commands = new Map();
  let understood = true;
  const registration = node => ts.isCallExpression(node) && ts.isPropertyAccessExpression(node.expression) && REGISTRATIONS.has(node.expression.name.text);
  const survey = node => {
    if (registration(node)) {
      if (node.arguments.length < 2 || !ts.isStringLiteralLike(node.arguments[0])) understood = false;
      else { const name = node.arguments[0].text; commands.set(name, (commands.get(name) ?? 0) + 1); if (name !== command) skipped.add(node.arguments[1]); }
    }
    if (ts.isIdentifier(node) && (node.text === "eval" || node.text === "Function")) understood = false;
    ts.forEachChild(node, survey);
  };
  survey(file);
  if (!understood || commands.get(command) !== 1) return undefined;
  const exported = node => (ts.getCombinedModifierFlags(node) & ts.ModifierFlags.Export) !== 0;
  for (const statement of file.statements) {
    if (ts.isImportDeclaration(statement) && ts.isStringLiteral(statement.moduleSpecifier)) {
      const specifier = statement.moduleSpecifier.text, clause = statement.importClause;
      if (!clause) { kept.add(specifier); continue; }
      if (clause.name) bindings.set(clause.name.text, specifier);
      const named = clause.namedBindings;
      if (named && ts.isNamespaceImport(named)) bindings.set(named.name.text, specifier);
      else if (named) for (const element of named.elements) bindings.set(element.name.text, specifier);
    } else if (ts.isImportEqualsDeclaration(statement)) return undefined;
    else if (ts.isExportDeclaration(statement)) {
      if (statement.moduleSpecifier && ts.isStringLiteral(statement.moduleSpecifier)) kept.add(statement.moduleSpecifier.text);
      else roots.push(statement);
    } else if (ts.isClassDeclaration(statement) && statement.name) { declarations.set(statement.name.text, [...(declarations.get(statement.name.text) ?? []), statement]); classes.push(statement); if (exported(statement)) roots.push(statement); }
    else if (ts.isFunctionDeclaration(statement) && statement.name) { declarations.set(statement.name.text, [...(declarations.get(statement.name.text) ?? []), statement]); if (exported(statement)) roots.push(statement); }
    else if (ts.isInterfaceDeclaration(statement) || ts.isTypeAliasDeclaration(statement)) declarations.set(statement.name.text, [...(declarations.get(statement.name.text) ?? []), statement]);
    else roots.push(statement);
  }
  const reachable = new Set(), queue = [], dynamic = new Set(), needed = new Set();
  const note = name => {
    if (reachable.has(name)) return;
    if (declarations.has(name)) { reachable.add(name); queue.push(name); }
    else if (bindings.has(name)) needed.add(bindings.get(name));
  };
  const memberName = node => node.name && !ts.isComputedPropertyName(node.name);
  const scan = node => {
    if (!node || skipped.has(node)) return;
    if (ts.isIdentifier(node)) { note(node.text); return; }
    if (ts.isPropertyAccessExpression(node)) { scan(node.expression); return; }
    if (ts.isQualifiedName(node)) { scan(node.left); return; }
    if (ts.isImportDeclaration(node)) return;
    if (ts.isPropertyAssignment(node) || ts.isPropertyDeclaration(node) || ts.isMethodDeclaration(node) || ts.isGetAccessor(node) || ts.isSetAccessor(node) || ts.isPropertySignature(node) || ts.isMethodSignature(node) || ts.isEnumMember(node)) {
      if (!memberName(node)) scan(node.name);
      ts.forEachChild(node, child => { if (child !== node.name) scan(child); });
      return;
    }
    if (ts.isCallExpression(node)) {
      const callee = node.expression, first = node.arguments[0];
      if ((callee.kind === ts.SyntaxKind.ImportKeyword || ts.isIdentifier(callee) && callee.text === "require" || ts.isCallExpression(callee) && ts.isIdentifier(callee.expression) && callee.expression.text === "createRequire") && first && ts.isStringLiteralLike(first)) dynamic.add(first.text);
    }
    ts.forEachChild(node, scan);
  };
  const evaluation = declaration => {
    for (const clause of declaration.heritageClauses ?? []) scan(clause);
    for (const modifier of ts.canHaveDecorators?.(declaration) ? ts.getDecorators(declaration) ?? [] : []) scan(modifier);
    for (const member of declaration.members) {
      if (member.name && ts.isComputedPropertyName(member.name)) scan(member.name);
      for (const decorator of ts.canHaveDecorators?.(member) ? ts.getDecorators(member) ?? [] : []) scan(decorator);
      const isStatic = (ts.getModifiers?.(member) ?? []).some(modifier => modifier.kind === ts.SyntaxKind.StaticKeyword);
      if (ts.isClassStaticBlockDeclaration(member) || isStatic && ts.isPropertyDeclaration(member)) scan(member);
    }
  };
  for (const root of roots) scan(root);
  for (const declaration of classes) evaluation(declaration);
  while (queue.length) for (const declaration of declarations.get(queue.shift())) scan(declaration);
  return new Set([...kept, ...needed, ...dynamic]);
}

/** 🗣️ Names the registered command a bound launcher command line runs: the argument right behind the script, unless it is an option or absent. */
export function routerCommandWordV1(bound) {
  if (!bound?.script) return undefined;
  const script = bound.script.replaceAll("\\", "/"), index = bound.arguments.indexOf(script), word = index < 0 ? undefined : bound.arguments[index + 1];
  return typeof word === "string" && word !== "" && !word.startsWith("-") ? word : undefined;
}
