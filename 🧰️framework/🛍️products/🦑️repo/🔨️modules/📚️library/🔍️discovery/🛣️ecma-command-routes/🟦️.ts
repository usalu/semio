import { ecmaProgram, type EcmaPattern, type EcmaExpression, type EcmaStatement } from "@semio-tech/compiler-syntax-ecma-ts";
import type { PackageGlueGrammarSpec } from "../📜️taxonomy-contracts/🟦️.ts";

interface EcmaRouteBinding {
  readonly kind: "pending" | "import-value" | "import-type" | "class" | "parameter" | "data" | "finite" | "receipt" | "module" | "closure" | "router";
  readonly imported?: string;
  readonly module?: string;
  readonly closure?: EcmaExpression;
  readonly scope?: EcmaRouteScope;
}

/** 🧭️ Carries one lexical binding environment without pooling sibling scopes. */
class EcmaRouteScope {
  readonly bindings = new Map<string, EcmaRouteBinding>();
  readonly parent?: EcmaRouteScope;
  constructor(parent?: EcmaRouteScope) { this.parent = parent; }
  define(name: string, binding: EcmaRouteBinding): boolean {
    if (this.bindings.has(name)) return false;
    this.bindings.set(name, binding);
    return true;
  }
  initialize(name: string, binding: EcmaRouteBinding): boolean {
    if (this.bindings.get(name)?.kind !== "pending") return false;
    this.bindings.set(name, binding);
    return true;
  }
  resolve(name: string): EcmaRouteBinding | undefined {
    return this.bindings.get(name) ?? this.parent?.resolve(name);
  }
}

const ECMA_ROUTE_OWNED_PROGRAM = "runScriptProgram";

type EcmaRouteValue = "invalid" | "data" | "finite" | "receipt" | "module" | "closure" | "router" | "error" | "console" | "process";

interface EcmaRouteValidation {
  terminals: number;
  readonly classes: Set<string>;
  readonly wiredClasses: Set<string>;
}

function ecmaRouteUnwrap(expression: EcmaExpression): EcmaExpression {
  return ["unary", "parenthesized", "nonnull"].includes(expression.kind) && (expression.kind !== "unary" || expression.operator === "await") ? ecmaRouteUnwrap(expression.object!) : expression;
}

function ecmaRouteIdentifier(expression: EcmaExpression): string | null {
  const value = ecmaRouteUnwrap(expression);
  return value.kind === "identifier" ? value.name! : null;
}

function ecmaRouteIntrinsic(scope: EcmaRouteScope, name: string): EcmaRouteValue {
  if (scope.resolve(name)) return "invalid";
  if (name === "console") return "console";
  if (name === "process") return "process";
  return name === "Error" ? "error" : "invalid";
}

function ecmaRouteImportedRoot(expression: EcmaExpression, scope: EcmaRouteScope): EcmaRouteBinding | undefined {
  let value = ecmaRouteUnwrap(expression);
  while (value.kind === "member") value = ecmaRouteUnwrap(value.object!);
  const name = ecmaRouteIdentifier(value);
  const binding = name ? scope.resolve(name) : undefined;
  return binding?.kind === "import-value" ? binding : undefined;
}

function ecmaRouteArgument(expression: EcmaExpression, scope: EcmaRouteScope): boolean {
  return ["data", "finite", "receipt", "closure", "router"].includes(ecmaRouteValue(expression, scope));
}

function ecmaRouteClosure(expression: EcmaExpression, scope: EcmaRouteScope): boolean {
  if (expression.kind !== "arrow" || expression.parameters?.some((pattern) => pattern.defaults || pattern.destructured)) return false;
  const nested = new EcmaRouteScope(scope);
  for (const pattern of expression.parameters ?? []) for (const name of pattern.names) if (!nested.define(name, { kind: "parameter" })) return false;
  if (Array.isArray(expression.body)) return ecmaRouteBlock(expression.body, nested, { terminals: 0, classes: new Set(), wiredClasses: new Set() }, true, true);
  const value = ecmaRouteValue(expression.body as EcmaExpression, nested);
  return value === "receipt";
}

function ecmaRouteCollectionCall(expression: EcmaExpression, scope: EcmaRouteScope): EcmaRouteValue {
  const callee = ecmaRouteUnwrap(expression.callee!), receiver = callee.object!, member = typeof callee.property === "string" ? callee.property : "";
  const receiverValue = ecmaRouteValue(receiver, scope), args = expression.arguments ?? [];
  if (member === "at" || member === "includes") return args.every((argument) => ecmaRouteArgument(argument, scope)) && ["data", "finite", "receipt"].includes(receiverValue) ? "data" : "invalid";
  if (member === "test") return ecmaRouteUnwrap(receiver).kind === "regex" && args.length === 1 && ecmaRouteArgument(args[0]!, scope) ? "data" : "invalid";
  if (member === "some") {
    if (!["data", "finite", "receipt"].includes(receiverValue) || args.length !== 1 || args[0]!.kind !== "arrow") return "invalid";
    const arrow = args[0]!, pattern = arrow.parameters?.[0];
    if (!pattern || arrow.parameters!.length !== 1 || pattern.defaults || pattern.destructured || Array.isArray(arrow.body)) return "invalid";
    const nested = new EcmaRouteScope(scope); nested.define(pattern.names[0]!, { kind: "parameter" });
    return ecmaRouteValue(arrow.body as EcmaExpression, nested) === "data" ? "data" : "invalid";
  }
  if (member !== "map" && member !== "flatMap" || receiverValue !== "finite" || args.length !== 1 || args[0]!.kind !== "arrow") return "invalid";
  const arrow = args[0]!, pattern = arrow.parameters?.[0];
  if (!pattern || arrow.parameters!.length !== 1 || pattern.defaults || pattern.destructured) return "invalid";
  const nested = new EcmaRouteScope(scope); nested.define(pattern.names[0]!, { kind: "parameter" });
  if (Array.isArray(arrow.body)) {
    if (!ecmaRouteBlock(arrow.body, nested, { terminals: 0, classes: new Set(), wiredClasses: new Set() }, false, true)) return "invalid";
  } else if (!ecmaRouteArgument(arrow.body as EcmaExpression, nested)) return "invalid";
  return "finite";
}

function ecmaRouteRegistrationValue(expression: EcmaExpression, scope: EcmaRouteScope): boolean {
  const value = ecmaRouteUnwrap(expression);
  if (value.kind === "literal" || value.kind === "regex") return true;
  if (value.kind === "identifier") {
    if (value.name === "this" || value.name === "undefined") return true;
    const binding = scope.resolve(value.name!);
    return binding?.kind === "class" || binding?.kind === "data" || binding?.kind === "finite" || binding?.kind === "import-value";
  }
  if (value.kind === "member") return (typeof value.property === "string" || ecmaRouteRegistrationValue(value.property!, scope)) && ecmaRouteValue(value, scope) === "data";
  if (value.kind === "template") return value.expressions!.every((row) => ecmaRouteRegistrationValue(row, scope));
  if (value.kind === "array") return value.elements!.every((row) => row.kind === "spread" ? ecmaRouteRegistrationValue(row.object!, scope) : ecmaRouteRegistrationValue(row, scope));
  if (value.kind === "object") return value.properties!.every((row) => (!row.computed || ecmaRouteRegistrationValue(row.key!, scope)) && ecmaRouteRegistrationValue(row.value.kind === "spread" ? row.value.object! : row.value, scope));
  if (value.kind === "binary") return ["??", "||", "&&", "===", "!==", "==", "!=", "<", "<=", ">", ">="].includes(value.operator!) && ecmaRouteRegistrationValue(value.left!, scope) && ecmaRouteRegistrationValue(value.right!, scope);
  if (value.kind === "conditional") return ecmaRouteRegistrationValue(value.condition!, scope) && ecmaRouteRegistrationValue(value.whenTrue!, scope) && ecmaRouteRegistrationValue(value.whenFalse!, scope);
  if (value.kind === "call") {
    const binding = ecmaRouteImportedRoot(value.callee!, scope);
    return (binding?.imported === "dirname" || binding?.imported === "fileURLToPath") && (value.arguments ?? []).length === 1 && ecmaRouteRegistrationValue(value.arguments![0]!, scope);
  }
  return false;
}

function ecmaRouteValue(expression: EcmaExpression, scope: EcmaRouteScope): EcmaRouteValue {
  if (expression.kind === "unary") {
    if (expression.operator === "await") return ecmaRouteValue(expression.object!, scope);
    return expression.operator === "!" && ecmaRouteArgument(expression.object!, scope) ? "data" : "invalid";
  }
  const value = ecmaRouteUnwrap(expression);
  if (value.kind === "literal" || value.kind === "regex") return "data";
  if (value.kind === "identifier") {
    if (value.name === "this" || value.name === "undefined") return "data";
    const binding = scope.resolve(value.name!);
    if (!binding) return ecmaRouteIntrinsic(scope, value.name!);
    if (binding.kind === "pending" || binding.kind === "import-type") return "invalid";
    if (binding.kind === "import-value" || binding.kind === "class" || binding.kind === "parameter" || binding.kind === "data") return "data";
    return binding.kind;
  }
  if (value.kind === "template") return value.expressions!.every((row) => ecmaRouteArgument(row, scope)) ? "data" : "invalid";
  if (value.kind === "array") return value.elements!.every((row) => row.kind === "spread" ? ecmaRouteArgument(row.object!, scope) : ecmaRouteArgument(row, scope)) ? "finite" : "invalid";
  if (value.kind === "object") return value.properties!.every((row) => (!row.computed || ecmaRouteArgument(row.key!, scope)) && (row.value.kind === "spread" ? ecmaRouteArgument(row.value.object!, scope) : ecmaRouteArgument(row.value, scope))) ? "data" : "invalid";
  if (value.kind === "conditional") return ecmaRouteArgument(value.condition!, scope) && ecmaRouteArgument(value.whenTrue!, scope) && ecmaRouteArgument(value.whenFalse!, scope) ? "data" : "invalid";
  if (value.kind === "binary") return ["??", "||", "&&", "===", "!==", "==", "!=", "<", "<=", ">", ">="].includes(value.operator!) && ecmaRouteArgument(value.left!, scope) && ecmaRouteArgument(value.right!, scope) ? "data" : "invalid";
  if (value.kind === "arrow") return ecmaRouteClosure(value, scope) ? "closure" : "invalid";
  if (value.kind === "member") {
    if (ecmaRouteIdentifier(value.object!) === "import" && value.property === "meta" && !scope.resolve("import")) return "data";
    const base = ecmaRouteValue(value.object!, scope);
    if (base === "invalid" || base === "module" || base === "closure" || base === "error") return "invalid";
    if (typeof value.property !== "string" && !ecmaRouteArgument(value.property!, scope)) return "invalid";
    return base === "finite" ? "finite" : "data";
  }
  if (value.kind === "new") {
    const name = ecmaRouteIdentifier(value.callee!), binding = name ? scope.resolve(name) : undefined;
    if (name === "Error" && !binding && value.arguments!.every((row) => ecmaRouteStatus(row, scope))) return "error";
    if (binding?.kind !== "import-value" || !value.arguments!.every((row) => ecmaRouteArgument(row, scope))) return "invalid";
    if (binding.imported === "ScriptRouter") return value.arguments!.every((row) => ecmaRouteRegistrationValue(row, scope)) ? "router" : "invalid";
    return "receipt";
  }
  if (value.kind !== "call") return "invalid";
  const callee = ecmaRouteUnwrap(value.callee!);
  if (callee.kind === "identifier" && callee.name === "import" && !scope.resolve("import")) {
    const argument = value.arguments?.[0];
    return value.arguments?.length === 1 && argument?.kind === "literal" && /(?:^|\/)🧪️tests(?:\/|$)|(?:^|\/)🧪️(?:\/|$)/u.test(argument.value!.slice(1, -1)) ? "module" : "invalid";
  }
  if (callee.kind === "identifier") {
    const binding = scope.resolve(callee.name!);
    if (binding?.kind === "closure") return value.arguments!.every((row) => ecmaRouteArgument(row, scope)) ? "receipt" : "invalid";
    return binding?.kind === "import-value" && value.arguments!.every((row) => ecmaRouteArgument(row, scope)) ? "receipt" : "invalid";
  }
  if (callee.kind !== "member") return "invalid";
  const member = typeof callee.property === "string" ? callee.property : "";
  if (!member) return "invalid";
  const receiver = ecmaRouteValue(callee.object!, scope);
  if (receiver === "console" && (member === "log" || member === "error")) return value.arguments!.every((row) => ecmaRouteStatus(row, scope)) ? "receipt" : "invalid";
  if (receiver === "process" && member === "exit") return value.arguments!.length === 1 && ecmaRouteArgument(value.arguments![0]!, scope) ? "receipt" : "invalid";
  if (["at", "includes", "some", "map", "flatMap", "test"].includes(member)) return ecmaRouteCollectionCall(value, scope);
  if (member === "run") {
    const instance = ecmaRouteUnwrap(callee.object!), constructor = instance.kind === "new" ? ecmaRouteIdentifier(instance.callee!) : null, binding = constructor ? scope.resolve(constructor) : undefined;
    if (binding?.kind === "import-value" && value.arguments!.every((row) => ecmaRouteArgument(row, scope))) return "receipt";
  }
  const imported = ecmaRouteImportedRoot(callee, scope);
  return imported && value.arguments!.every((row) => ecmaRouteArgument(row, scope)) ? "receipt" : "invalid";
}

function ecmaRouteStatus(expression: EcmaExpression, scope: EcmaRouteScope): boolean {
  const value = ecmaRouteUnwrap(expression);
  if (value.kind === "binary" && value.operator === "+") return ecmaRouteStatus(value.left!, scope) && ecmaRouteStatus(value.right!, scope);
  return ecmaRouteArgument(value, scope);
}

function ecmaRouteDefine(pattern: EcmaPattern, initializer: EcmaExpression, scope: EcmaRouteScope): boolean {
  if (pattern.defaults) return false;
  const value = ecmaRouteValue(initializer, scope);
  if (value === "invalid" || value === "console" || value === "process" || value === "error") return false;
  if (value === "module") return pattern.destructured && pattern.names.every((name) => scope.initialize(name, { kind: "import-value", imported: name, module: "dynamic-test" }));
  if (value === "closure") return !pattern.destructured && pattern.names.length === 1 && scope.initialize(pattern.names[0]!, { kind: "closure", closure: initializer, scope });
  const kind: EcmaRouteBinding["kind"] = value === "finite" ? "finite" : value === "receipt" ? "receipt" : value === "router" ? "router" : "data";
  return pattern.names.every((name) => scope.initialize(name, { kind }));
}

function ecmaRoutePrebind(rows: readonly EcmaStatement[], scope: EcmaRouteScope): boolean {
  for (const row of rows) {
    if (row.kind === "class") {
      if (!row.name || !scope.define(row.name, { kind: "pending" })) return false;
      continue;
    }
    if (row.kind !== "const" && row.kind !== "let") continue;
    for (const declaration of row.declarations ?? []) for (const name of declaration.pattern.names) if (!scope.define(name, { kind: "pending" })) return false;
  }
  return true;
}

function ecmaRouteEnvironment(statement: EcmaStatement, scope: EcmaRouteScope): boolean {
  if (statement.kind !== "expression" || statement.expression?.kind !== "assignment" || statement.expression.operator !== "??=") return false;
  const left = statement.expression.left!, right = statement.expression.right!;
  if (right.kind !== "literal" || !/^["'\d]/u.test(right.value ?? "") || left.kind !== "member" || typeof left.property !== "string" || !/^[A-Z][A-Z0-9_]*$/u.test(left.property)) return false;
  const env = ecmaRouteUnwrap(left.object!);
  return env.kind === "member" && env.property === "env" && ecmaRouteIntrinsic(scope, ecmaRouteIdentifier(env.object!) ?? "") === "process";
}

function ecmaRouteEnvironmentGuard(expression: EcmaExpression, scope: EcmaRouteScope): boolean {
  const value = ecmaRouteUnwrap(expression);
  if (value.kind !== "binary" || value.operator !== "===" || value.right?.kind !== "literal") return false;
  const left = ecmaRouteUnwrap(value.left!);
  if (left.kind !== "member" || typeof left.property === "string" || left.property?.kind !== "literal") return false;
  const argv = ecmaRouteUnwrap(left.object!);
  return argv.kind === "member" && argv.property === "argv" && ecmaRouteIntrinsic(scope, ecmaRouteIdentifier(argv.object!) ?? "") === "process";
}

function ecmaRouteImportMetaMember(expression: EcmaExpression, member: string, scope: EcmaRouteScope): boolean {
  const value = ecmaRouteUnwrap(expression);
  if (value.kind !== "member" || value.property !== member || value.optional) return false;
  const meta = ecmaRouteUnwrap(value.object!);
  return meta.kind === "member" && meta.property === "meta" && !meta.optional && ecmaRouteIdentifier(meta.object!) === "import" && !scope.resolve("import");
}

function ecmaRouteBlock(rows: readonly EcmaStatement[], scope: EcmaRouteScope, validation: EcmaRouteValidation, guarded = false, closure = false, maximum = 64): boolean {
  if (rows.length === 0 || rows.length > maximum || !ecmaRoutePrebind(rows, scope)) return false;
  for (const row of rows) {
    if (row.kind === "const") {
      for (const declaration of row.declarations ?? []) if (!ecmaRouteDefine(declaration.pattern, declaration.initializer, scope)) return false;
      continue;
    }
    if (row.kind === "let" || row.kind === "var" || row.kind === "class" || row.kind === "import") return false;
    if (row.kind === "expression") {
      const result = ecmaRouteValue(row.expression!, scope);
      if (!["receipt", "module"].includes(result)) return false;
      continue;
    }
    if (row.kind === "throw") {
      if (!guarded || ecmaRouteValue(row.expression!, scope) !== "error") return false;
      continue;
    }
    if (row.kind === "return") {
      if ((!guarded && !closure) || row.expression && ecmaRouteValue(row.expression, scope) !== "receipt" && !ecmaRouteArgument(row.expression, scope)) return false;
      continue;
    }
    if (row.kind === "block") {
      if (!ecmaRouteBlock(row.body!, new EcmaRouteScope(scope), validation, guarded, closure, maximum)) return false;
      continue;
    }
    if (row.kind === "if") {
      if (!ecmaRouteArgument(row.expression!, scope)) return false;
      const branch = (statement: EcmaStatement): boolean => ecmaRouteBlock(statement.kind === "block" ? statement.body! : [statement], new EcmaRouteScope(scope), validation, true, closure, maximum);
      if (!branch(row.then!) || row.otherwise && !branch(row.otherwise)) return false;
      continue;
    }
    if (row.kind === "for") {
      if (row.initializer!.defaults || row.initializer!.destructured || row.initializer!.names.length !== 1) return false;
      const iterable = ecmaRouteValue(row.iterable!, scope);
      if (iterable !== "finite") return false;
      const nested = new EcmaRouteScope(scope); nested.define(row.initializer!.names[0]!, { kind: "parameter" });
      if (!ecmaRouteBlock(row.statement!.kind === "block" ? row.statement!.body! : [row.statement!], nested, validation, false, closure, maximum)) return false;
      continue;
    }
    return false;
  }
  return true;
}

function ecmaRouteRouter(expression: EcmaExpression, scope: EcmaRouteScope, validation: EcmaRouteValidation): boolean {
  const value = ecmaRouteUnwrap(expression);
  if (value.kind === "identifier") return scope.resolve(value.name!)?.kind === "router";
  if (value.kind === "new") {
    const name = ecmaRouteIdentifier(value.callee!), binding = name ? scope.resolve(name) : undefined;
    if (binding?.kind !== "import-value" || binding.imported !== "ScriptRouter") return false;
  } else if (value.kind === "call") {
    const callee = ecmaRouteUnwrap(value.callee!);
    if (callee.kind !== "member" || callee.property !== "register" || !ecmaRouteRouter(callee.object!, scope, validation)) return false;
  } else return false;
  const inspect = (argument: EcmaExpression): boolean => {
    const item = ecmaRouteUnwrap(argument);
    if (item.kind === "identifier") {
      const binding = scope.resolve(item.name!);
      if (binding?.kind === "class") validation.wiredClasses.add(item.name!);
      return binding?.kind === "class" || binding?.kind === "data" || binding?.kind === "finite" || binding?.kind === "import-value" || item.name === "this" || item.name === "undefined";
    }
    if (item.kind === "array") return item.elements!.every((row) => row.kind !== "spread" && inspect(row));
    if (item.kind === "object") return item.properties!.every((row) => (!row.computed || ecmaRouteArgument(row.key!, scope)) && inspect(row.value));
    return ecmaRouteRegistrationValue(item, scope);
  };
  return (value.arguments ?? []).every(inspect);
}

/** 🐣️ Accepts only the process-birth wrapper whose single issued parameter reaches exactly one validated terminal. */
function ecmaRouteReceivedTerminal(call: EcmaExpression, scope: EcmaRouteScope, validation: EcmaRouteValidation, mainGuard: boolean): boolean {
  const [environment, body] = call.arguments ?? [], arrow = body ? ecmaRouteUnwrap(body) : undefined, root = environment ? ecmaRouteUnwrap(environment) : undefined;
  if (call.arguments?.length !== 2 || arrow?.kind !== "arrow" || Array.isArray(arrow.body) || arrow.parameters?.length !== 1 || root?.kind !== "member" || root.property !== "env" || ecmaRouteIntrinsic(scope, ecmaRouteIdentifier(root.object!) ?? "") !== "process") return false;
  const pattern = arrow.parameters[0]!, nested = new EcmaRouteScope(scope);
  if (pattern.defaults || pattern.destructured || pattern.names.length !== 1 || !nested.define(pattern.names[0]!, { kind: "parameter" })) return false;
  return ecmaRouteTerminal({ kind: "expression", expression: arrow.body as EcmaExpression } as EcmaStatement, nested, validation, mainGuard, pattern.names[0]!);
}

/** 🪺 Accepts the bare entry of a program that owns its process birth: one argument-free call of the program imported from a sibling module. */
function ecmaRouteOwnedProgram(call: EcmaExpression, scope: EcmaRouteScope): boolean {
  const name = ecmaRouteIdentifier(call.callee!), binding = name ? scope.resolve(name) : undefined;
  return binding?.kind === "import-value" && binding.imported === ECMA_ROUTE_OWNED_PROGRAM && /^\.{1,2}\/(?:[^/]+\/)*🟦️\.ts$/u.test(binding.module ?? "") && call.arguments?.length === 0;
}

function ecmaRouteTerminal(statement: EcmaStatement, scope: EcmaRouteScope, validation: EcmaRouteValidation, mainGuard = false, issued?: string): boolean {
  if (statement.kind !== "expression") return false;
  const expression = ecmaRouteUnwrap(statement.expression!);
  if (expression.kind !== "call") return false;
  if (mainGuard && issued === undefined && ecmaRouteOwnedProgram(expression, scope)) {
    validation.terminals++;
    return true;
  }
  const callee = ecmaRouteUnwrap(expression.callee!), received = ecmaRouteIdentifier(expression.callee!), receiver = received ? scope.resolve(received) : undefined;
  if (receiver?.kind === "import-value" && receiver.imported === "receiveScriptProcessInvocation") return issued === undefined && ecmaRouteReceivedTerminal(expression, scope, validation, mainGuard);
  if (mainGuard && callee.kind === "member" && callee.property === "run" && (scope.resolve(ecmaRouteIdentifier(callee.object!) ?? "")?.kind === "router" || ecmaRouteRouter(callee.object!, scope, validation))) {
    const only = expression.arguments?.length === (issued === undefined ? 1 : 2), last = issued === undefined ? undefined : expression.arguments?.[1] ? ecmaRouteUnwrap(expression.arguments[1]) : undefined;
    if (issued !== undefined && (last?.kind !== "identifier" || last.name !== issued)) return false;
    const argument = only ? ecmaRouteUnwrap(expression.arguments![0]!) : undefined;
    const slice = argument?.kind === "call" ? ecmaRouteUnwrap(argument.callee!) : undefined;
    const argv = slice?.kind === "member" && slice.property === "slice" ? ecmaRouteUnwrap(slice.object!) : undefined;
    if (argv?.kind !== "member" || argv.property !== "argv" || ecmaRouteIntrinsic(scope, ecmaRouteIdentifier(argv.object!) ?? "") !== "process" || argument?.arguments?.length !== 1 || argument.arguments[0]?.kind !== "literal" || argument.arguments[0].value !== "2") return false;
    validation.terminals++;
    return true;
  }
  const name = ecmaRouteIdentifier(expression.callee!), binding = name ? scope.resolve(name) : undefined;
  if (binding?.kind !== "import-value" || !["runScriptMain", "runRepoScriptMain", "runWorkspaceScriptMain", "runPolicyOnlyMain", "runArtifactRustPackageMain", "runArtifactTypeScriptPackageMain"].includes(binding.imported ?? "")) return false;
  if (!expression.arguments!.every((argument) => ecmaRouteRouter(argument, scope, validation) || ["data", "finite"].includes(ecmaRouteValue(argument, scope)))) return false;
  validation.terminals++;
  return true;
}

function ecmaRouteTestMetadata(expression: EcmaExpression, member: string, scope: EcmaRouteScope): boolean {
  const meta = expression.object;
  return expression.kind === "member" && expression.property === member && !expression.optional && meta?.kind === "member" && meta.property === "meta" && !meta.optional && meta.object?.kind === "identifier" && meta.object.name === "import" && !scope.resolve("import");
}

function ecmaRouteTestDependency(expression: EcmaExpression, scope: EcmaRouteScope): boolean {
  if (expression.kind === "parenthesized" || expression.kind === "nonnull") return ecmaRouteTestDependency(expression.object!, scope);
  if (expression.kind === "literal") return expression.value !== "undefined";
  if (expression.kind === "identifier") {
    const binding = scope.resolve(expression.name!);
    return binding?.kind === "import-value" && binding.module !== "dynamic-test";
  }
  if (expression.kind === "member") return ["dir", "url"].some((member) => ecmaRouteTestMetadata(expression, member, scope));
  if (expression.kind === "array") return expression.elements!.every((row) => ecmaRouteTestDependency(row, scope));
  return expression.kind === "object" && expression.properties!.every((row) => !row.computed && ecmaRouteTestDependency(row.value, scope));
}

function ecmaRouteTestRegistration(statement: EcmaStatement, scope: EcmaRouteScope): boolean {
  if (statement.otherwise || statement.then?.kind !== "block" || statement.then.body?.length !== 2) return false;
  const [declaration, invocation] = statement.then.body;
  if (declaration?.kind !== "const" || declaration.declarations?.length !== 1 || invocation?.kind !== "expression") return false;
  const binding = declaration.declarations[0]!, members = binding.pattern.objectBindings;
  if (binding.pattern.defaults || members?.length !== 1 || scope.resolve(members[0]!.local) || binding.initializer.kind !== "unary" || binding.initializer.operator !== "await") return false;
  const module = binding.initializer.object!;
  if (module.kind !== "call" || module.callee?.kind !== "identifier" || module.callee.name !== "import" || scope.resolve("import") || module.arguments?.length !== 1) return false;
  const literal = module.arguments[0]!;
  if (literal.kind !== "literal" || !/^["']/u.test(literal.value ?? "")) return false;
  const path = literal.value!.slice(1, -1), segments = path.split("/");
  if (!/^\.{1,2}\//u.test(path) || /[\\%?#]/u.test(path)) return false;
  while (segments[0] === "." || segments[0] === "..") segments.shift();
  if (segments.some((segment) => !segment || segment === "." || segment === "..") || !segments.slice(0, -1).some((segment) => segment === "🧪️tests" || segment === "🧪️")) return false;
  const expression = invocation.expression!;
  if (expression.kind !== "unary" || expression.operator !== "await" || expression.object?.kind !== "call") return false;
  const call = expression.object, nested = new EcmaRouteScope(scope), local = members[0]!.local;
  if (!nested.define(local, { kind: "import-value", imported: members[0]!.imported, module: "dynamic-test" }) || call.callee?.kind !== "identifier" || call.callee.name !== local || call.arguments?.length !== 3 || !ecmaRouteTestMetadata(call.arguments[0]!, "vitest", nested)) return false;
  return call.arguments.slice(1).every((argument) => ecmaRouteTestDependency(argument, nested));
}

/** 🚦️ Recognizes only value-bound, lexically scoped command routing. */
export function ecmaCommandRouterModule(content: string, grammar: PackageGlueGrammarSpec): boolean {
  const program = ecmaProgram(content);
  if (!program) return false;
  const scope = new EcmaRouteScope(), validation: EcmaRouteValidation = { terminals: 0, classes: new Set(), wiredClasses: new Set() };
  let environmentDefaults = 0, testRegistrations = 0;
  for (const row of program) if (row.kind === "import") for (const binding of row.imports ?? []) if (!scope.define(binding.local, { kind: binding.runtime ? "import-value" : "import-type", imported: binding.imported, module: binding.module })) return false;
  if (!ecmaRoutePrebind(program.filter((row) => row.kind !== "import"), scope)) return false;
  for (const row of program) {
    if (row.kind === "import") continue;
    if (row.kind === "class") {
      const base = ecmaRouteIdentifier(row.base!), binding = base ? scope.resolve(base) : undefined;
      if (!row.name || binding?.kind !== "import-value" || !["BundleScript", "Script"].includes(binding.imported ?? "") || !scope.initialize(row.name, { kind: "class" })) return false;
      const method = row.body?.[0];
      if (!method || method.parameters?.some((pattern) => pattern.defaults || pattern.destructured)) return false;
      const methodScope = new EcmaRouteScope(scope);
      for (const pattern of method.parameters ?? []) for (const name of pattern.names) if (!methodScope.define(name, { kind: "parameter" })) return false;
      if (!ecmaRouteBlock(method.body ?? [], methodScope, validation, false, false, grammar.maxDelegationStatements)) return false;
      validation.classes.add(row.name);
      continue;
    }
    if (row.kind === "const") {
      for (const declaration of row.declarations ?? []) {
        if (ecmaRouteRouter(declaration.initializer, scope, validation)) {
          if (declaration.pattern.destructured || declaration.pattern.defaults || declaration.pattern.names.length !== 1 || !scope.initialize(declaration.pattern.names[0]!, { kind: "router" })) return false;
        } else if (ecmaRouteValue(declaration.initializer, scope) === "module" || !ecmaRouteDefine(declaration.pattern, declaration.initializer, scope)) return false;
      }
      continue;
    }
    if (row.kind === "if") {
      const condition = ecmaRouteUnwrap(row.expression!);
      if (ecmaRouteTestMetadata(row.expression!, "vitest", scope)) {
        if (testRegistrations++ !== 0 || !ecmaRouteTestRegistration(row, scope)) return false;
        continue;
      }
      if (condition.kind === "member" && ecmaRouteImportMetaMember(condition, "main", scope)) {
        const nested = new EcmaRouteScope(scope), rows = row.then!.kind === "block" ? row.then!.body! : [row.then!];
        if (!ecmaRoutePrebind(rows, nested)) return false;
        for (const statement of rows) {
          if (statement.kind === "const") {
            for (const declaration of statement.declarations ?? []) {
              if (!ecmaRouteRouter(declaration.initializer, nested, validation) || declaration.pattern.destructured || declaration.pattern.defaults || declaration.pattern.names.length !== 1 || !nested.initialize(declaration.pattern.names[0]!, { kind: "router" })) return false;
            }
          } else if (!ecmaRouteTerminal(statement, nested, validation, true)) return false;
        }
        if (row.otherwise) return false;
        continue;
      }
      if (!ecmaRouteEnvironmentGuard(row.expression!, scope) || row.otherwise) return false;
      const rows = row.then!.kind === "block" ? row.then!.body! : [row.then!];
      if (rows.length !== 1 || !ecmaRouteEnvironment(rows[0]!, scope) || environmentDefaults++ !== 0 || validation.terminals !== 0) return false;
      continue;
    }
    if (ecmaRouteEnvironment(row, scope)) {
      if (environmentDefaults++ !== 0 || validation.terminals !== 0) return false;
      continue;
    }
    if (!ecmaRouteTerminal(row, scope, validation)) return false;
  }
  return validation.terminals === 1 && [...validation.classes].every((name) => validation.wiredClasses.has(name));
}
