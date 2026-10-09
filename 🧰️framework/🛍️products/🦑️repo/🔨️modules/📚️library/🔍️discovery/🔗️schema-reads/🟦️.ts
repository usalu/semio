import { createHash } from "node:crypto";
import { posix } from "node:path";
import { ecmaProgram, ecmaStringValue, ecmaTokens, type EcmaExpression, type EcmaStatement } from "../../../../../../🔨️modules/📚️compiler/📖️syntax/🟨️ecma/🟦️.ts";
import type { SchemaReadIssue, SchemaReadProgress, SchemaSourceOrigin, SchemaValidationRead, SchemaValidationReadIndex, SchemaValidationReadReport } from "./🧬️schema/🟦️.ts";
export type { SchemaReadIssue, SchemaReadProgress, SchemaSourceOrigin, SchemaValidationRead, SchemaValidationReadIndex, SchemaValidationReadReport } from "./🧬️schema/🟦️.ts";

interface ReadDocument { readonly value: unknown; readonly origin: SchemaSourceOrigin; readonly dependencies?: readonly ReadDocument[]; tainted: boolean; }
interface ReadValue {
  readonly kind: "unknown" | "constant" | "json" | "text" | "url" | "namespace" | "constructor" | "ajv" | "validator" | "helper" | "promise";
  readonly name?: string;
  readonly data?: unknown;
  readonly document?: ReadDocument;
  readonly origin?: SchemaSourceOrigin;
  readonly schema?: ReadValue;
  readonly node?: EcmaExpression | EcmaStatement;
  readonly scope?: ReadScope;
  readonly registry?: Map<string, ReadValue>;
  readonly authority?: { tainted: boolean };
  readonly settled?: ReadValue;
}
interface ReadBinding { value: ReadValue; readonly imported: boolean; }
class ReadScope {
  readonly bindings = new Map<string, ReadBinding>();
  constructor(readonly parent?: ReadScope, readonly functionBoundary = !parent) {}
  binding(name: string): ReadBinding | undefined { return this.bindings.get(name) ?? this.parent?.binding(name); }
  functionScope(): ReadScope { return this.functionBoundary ? this : this.parent!.functionScope(); }
}

const unresolvedValue: ReadValue = { kind: "unknown" }, sha = (text: string): string => createHash("sha256").update(text).digest("hex");

class SchemaReadInspector {
  readonly reads: SchemaValidationRead[] = [];
  readonly unresolved: SchemaReadIssue[] = [];
  private readonly documents = new Map<string, ReadDocument | null>();
  private readonly helpers = new Set<EcmaExpression | EcmaStatement>();
  private readonly deferred: ReadValue[] = [];
  private readonly emitted = new Set<string>();
  private conditionalDepth = 0;
  readonly readerHash: string;
  constructor(readonly readerPath: string, readonly source: string, readonly read: (path: string) => string | undefined, readonly checkCancellation?: () => void) { this.readerHash = sha(source); }
  private issue(node: { readonly start: number; readonly end: number }, reason: string): void {
    const key = `${node.start}:${node.end}:${reason}`;
    if (this.emitted.has(key)) return;
    this.emitted.add(key); this.unresolved.push({ start: node.start, end: node.end, reason });
  }
  private global(name: string, scope: ReadScope): ReadValue {
    const binding = scope.binding(name);
    if (binding) return binding.value;
    return ["JSON", "Bun", "URL", "import"].includes(name) ? { kind: "namespace", name } : unresolvedValue;
  }
  private document(path: string): ReadValue {
    let document = this.documents.get(path);
    if (document === undefined) {
      const text = this.read(path);
      try { document = text === undefined ? null : { value: JSON.parse(text), origin: { path, hash: sha(text), selector: [], start: 0, end: text.length }, tainted: false }; } catch { document = null; }
      this.documents.set(path, document);
    }
    return document ? { kind: "json", document, origin: document.origin } : unresolvedValue;
  }
  private constant(value: ReadValue): unknown {
    if (value.kind === "constant") return value.data;
    if (value.kind !== "json" || this.tainted(value.document!)) return undefined;
    let data = value.document!.value;
    for (const key of value.origin!.selector) {
      if (!data || typeof data !== "object" || key === "*") return undefined;
      data = (data as Record<string, unknown>)[key];
    }
    return data;
  }
  private tainted(document: ReadDocument, visited = new Set<ReadDocument>()): boolean {
    this.checkCancellation?.();
    if (visited.has(document)) return false;
    visited.add(document);
    return document.tainted || (document.dependencies ?? []).some(child => this.tainted(child, visited));
  }
  private inline(node: EcmaExpression, data: unknown, dependencies: readonly ReadDocument[]): ReadValue {
    const origin: SchemaSourceOrigin = { path: null, hash: this.readerHash, selector: [], start: node.start, end: node.end };
    return { kind: "json", origin, document: { value: data, origin, dependencies, tainted: false }, node };
  }
  private fork(scope: ReadScope, uncertain = false): ReadScope {
    const parent = scope.parent ? this.fork(scope.parent, uncertain) : undefined, result = new ReadScope(parent, scope.functionBoundary);
    for (const [name, binding] of scope.bindings) result.bindings.set(name, { ...binding, value: uncertain && !binding.imported ? unresolvedValue : binding.value });
    return result;
  }
  private merge(scope: ReadScope, left: ReadScope, right: ReadScope): void {
    for (const [name, binding] of scope.bindings) if (left.binding(name)?.value !== right.binding(name)?.value) binding.value = unresolvedValue; else binding.value = left.binding(name)!.value;
    if (scope.parent) this.merge(scope.parent, left.parent!, right.parent!);
  }
  private returns(row: EcmaStatement): boolean { return row.kind === "return" || (row.body ?? []).some(child => this.returns(child)) || !!row.then && this.returns(row.then) || !!row.otherwise && this.returns(row.otherwise); }
  private literal(node: EcmaExpression): ReadValue {
    const text = node.value ?? "";
    if (text.startsWith("\"") || text.startsWith("'")) { const data = ecmaStringValue({ kind: "string", text, start: node.start, end: node.end }, this.checkCancellation); return data === null ? unresolvedValue : { kind: "constant", data }; }
    if (["true", "false", "null"].includes(text)) return { kind: "constant", data: JSON.parse(text) };
    const data = Number(text.replaceAll("_", ""));
    return text && Number.isFinite(data) ? { kind: "constant", data } : unresolvedValue;
  }
  private member(value: ReadValue, property: string): ReadValue {
    if (value.kind === "unknown" && (value.document || value.authority)) return value;
    if (value.kind === "json") return { ...value, origin: { ...value.origin!, selector: [...value.origin!.selector, property] } };
    if (value.kind === "namespace") {
      if (["ajv", "ajv/dist/2020", "ajv/dist/2020.js"].includes(value.name!) && ["default", "Ajv"].includes(property)) return { kind: "constructor" };
      return { kind: "namespace", name: `${value.name}.${property}` };
    }
    const data = this.constant(value);
    return data && typeof data === "object" && Object.hasOwn(data, property) ? { kind: "constant", data: (data as Record<string, unknown>)[property] } : unresolvedValue;
  }
  private imported(module: string, name: string): ReadValue {
    if (["ajv", "ajv/dist/2020", "ajv/dist/2020.js"].includes(module)) return name === "*" ? { kind: "namespace", name: module } : ["default", "Ajv"].includes(name) ? { kind: "constructor" } : unresolvedValue;
    if (["node:fs", "fs"].includes(module)) return { kind: "namespace", name: name === "*" ? "fs" : `fs.${name}` };
    if (module.endsWith(".json") && module.startsWith(".")) return this.document(posix.normalize(posix.join(posix.dirname(this.readerPath), module)));
    return unresolvedValue;
  }
  private schemaReference(schema: ReadValue, registry: ReadonlyMap<string, ReadValue>): ReadValue {
    const data = this.constant(schema);
    const reference = data && typeof data === "object" && !Array.isArray(data) ? (data as Record<string, unknown>).$ref : undefined;
    if (typeof reference !== "string") return schema;
    const [id, fragment] = reference.split("#"), root = registry.get(id!);
    if (!root) return schema;
    if (!fragment) return root;
    if (!fragment.startsWith("/")) return unresolvedValue;
    try { return fragment.slice(1).split("/").map(key => decodeURIComponent(key).replaceAll("~1", "/").replaceAll("~0", "~")).reduce((value, key) => this.member(value, key), root); } catch { return unresolvedValue; }
  }
  private validate(node: EcmaExpression, schema: ReadValue, value: ReadValue): ReadValue {
    const data = this.constant(schema);
    if (!data || typeof data !== "object" || Array.isArray(data) || schema.document && this.tainted(schema.document)) { this.issue(node, "schema-origin-unresolved"); return unresolvedValue; }
    const origin = schema.origin ?? { path: null, hash: this.readerHash, selector: [], start: schema.node?.start ?? node.start, end: schema.node?.end ?? node.end };
    const input = value.kind === "json" && !this.tainted(value.document!) ? value.origin! : null;
    const resolution = input ? input.selector.length ? "projected" : "whole" : "unresolved";
    const row: SchemaValidationRead = { schema: origin, value: input, callStart: node.start, callEnd: node.end, resolution, ...(origin.path === null ? { inlineSchema: data as Record<string, unknown> } : {}) };
    const key = JSON.stringify(row);
    if (!this.emitted.has(key)) { this.emitted.add(key); this.reads.push(row); }
    if (!input) this.issue(node, "value-origin-unresolved");
    return unresolvedValue;
  }
  private template(node: EcmaExpression, scope: ReadScope): ReadValue {
    const token = ecmaTokens(this.source.slice(node.start, node.end), node.start, this.checkCancellation)[0];
    if (token?.kind !== "template") return unresolvedValue;
    let cursor = node.start + 1, data = "", resolved = true;
    const chunk = (start: number, end: number): string | null => ecmaStringValue({ kind: "string", start, end, text: `"${this.source.slice(start, end).replaceAll('"', '\\"').replaceAll("\r\n", "\n").replaceAll("\r", "\n").replaceAll("\n", "\\n")}"` }, this.checkCancellation);
    for (const [index, span] of (token.expressions ?? []).entries()) {
      const prefix = chunk(cursor, span.start - 2), value = this.constant(this.evaluate(node.expressions![index]!, scope));
      if (prefix === null || !["string", "number", "boolean"].includes(typeof value)) resolved = false;
      else data += prefix + String(value);
      cursor = span.end + 1;
    }
    const suffix = chunk(cursor, node.end - 1);
    return !resolved || suffix === null ? unresolvedValue : { kind: "constant", data: data + suffix };
  }
  private invoke(helper: ReadValue, args: readonly ReadValue[]): ReadValue {
    const node = helper.node!;
    if (this.helpers.has(node)) { this.issue(node, "recursive-helper-unresolved"); return unresolvedValue; }
    this.helpers.add(node);
    try {
      const scope = new ReadScope(helper.scope, true);
      for (const [index, pattern] of (node.parameters ?? []).entries()) {
        this.patternEffects(pattern, scope);
        for (const name of pattern.names) scope.bindings.set(name, { value: !pattern.destructured && !pattern.defaults ? args[index] ?? unresolvedValue : unresolvedValue, imported: false });
      }
      const result = Array.isArray(node.body) ? this.statements(node.body, scope) : node.body ? this.evaluate(node.body as EcmaExpression, scope) : unresolvedValue;
      return node.async ? { kind: "promise", settled: result } : result;
    } finally { this.helpers.delete(node); }
  }
  private call(node: EcmaExpression, scope: ReadScope): ReadValue {
    const calleeNode = node.callee!, reference = this.memberReference(calleeNode, scope), owner = reference?.owner, property = reference?.property;
    const callee = reference ? typeof property === "string" ? this.member(owner!, property) : { kind: "unknown", document: owner?.document, authority: owner?.authority } as ReadValue : this.evaluate(calleeNode, scope);
    const args = (node.arguments ?? []).map(arg => this.evaluate(arg, scope));
    if (reference && typeof property === "string") {
      if (owner?.kind === "ajv") {
        if (owner.authority!.tainted) { this.issue(node, "validator-api-mutated"); return unresolvedValue; }
        if (property === "addSchema" && args[0]) {
          if (this.conditionalDepth) { owner.authority!.tainted = true; this.issue(node, "conditional-validator-registry-unresolved"); return unresolvedValue; }
          const data = this.constant(args[0]); if (data && typeof data === "object" && typeof (data as Record<string, unknown>).$id === "string") owner.registry!.set((data as Record<string, unknown>).$id as string, args[0]); return owner;
        }
        if (property === "compile") return { kind: "validator", schema: this.schemaReference(args[0] ?? unresolvedValue, owner.registry!) };
        if (property === "getSchema") { const data = this.constant(args[0] ?? unresolvedValue); return { kind: "validator", schema: typeof data === "string" ? this.schemaReference({ kind: "constant", data: { $ref: data } }, owner.registry!) : unresolvedValue }; }
        if (property === "validate") {
          const schema = args[0] ?? unresolvedValue, data = this.constant(schema);
          return this.validate(node, this.schemaReference(typeof data === "string" ? { kind: "constant", data: { $ref: data } } : schema, owner.registry!), args[1] ?? unresolvedValue);
        }
        this.issue(node, "validator-method-unresolved");
      }
      if (property === "compile") { this.issue(node, "validator-owner-unresolved"); return unresolvedValue; }
      if (owner?.kind === "namespace" && owner.name === "JSON" && property === "parse" && args[0]?.kind === "text") return this.document(args[0].name!);
      if (owner?.kind === "namespace" && owner.name === "Bun" && property === "file" && args[0]?.kind === "url") return { kind: "text", name: args[0].name };
      if (owner?.kind === "text" && property === "json") return { kind: "promise", settled: this.document(owner.name!) };
    }
    if (callee.kind === "validator") return this.validate(node, callee.schema!, args[0] ?? unresolvedValue);
    if (callee.kind === "helper") return this.invoke(callee, args);
    if (callee.kind === "namespace" && callee.name === "fs.readFileSync" && args[0]?.kind === "url") return { kind: "text", name: args[0].name };
    for (const value of args) {
      if (value.kind === "helper") this.deferred.push(value);
      else this.invalidate(value);
    }
    this.invalidate(owner);
    return unresolvedValue;
  }
  private memberReference(node: EcmaExpression, scope: ReadScope): { readonly owner: ReadValue; readonly property: unknown } | undefined {
    if (node.kind !== "member") return undefined;
    const owner = this.evaluate(node.object!, scope), property = typeof node.property === "string" ? node.property : node.property ? this.constant(this.evaluate(node.property, scope)) : undefined;
    return { owner, property };
  }
  private reference(node: EcmaExpression, scope: ReadScope): ReadValue | undefined { return this.memberReference(node, scope)?.owner; }
  private invalidate(value: ReadValue | undefined): void {
    if (value?.document) value.document.tainted = true;
    if (value?.authority) value.authority.tainted = true;
  }
  private patternEffects(pattern: { readonly start: number; readonly end: number; readonly defaults: boolean }, scope: ReadScope): void {
    if (!pattern.defaults) return;
    this.issue(pattern, "pattern-default-effects-unresolved");
    for (let current: ReadScope | undefined = scope; current; current = current.parent) for (const binding of current.bindings.values()) { this.checkCancellation?.(); this.invalidate(binding.value); binding.value = unresolvedValue; }
  }
  private hoist(rows: readonly EcmaStatement[], scope: ReadScope): void {
    for (const row of rows) {
      this.checkCancellation?.();
      if (row.kind === "var") for (const declaration of row.declarations ?? []) for (const name of declaration.pattern.names) if (!scope.bindings.has(name)) scope.bindings.set(name, { value: unresolvedValue, imported: false });
      if (["function", "class"].includes(row.kind)) continue;
      this.hoist([...(row.body ?? []), ...(row.then ? [row.then] : []), ...(row.otherwise ? [row.otherwise] : []), ...(row.statement ? [row.statement] : [])], scope);
    }
  }
  private evaluate(node: EcmaExpression, scope: ReadScope): ReadValue {
    this.checkCancellation?.();
    if (node.kind === "identifier") return this.global(node.name!, scope);
    if (node.kind === "literal") return this.literal(node);
    if (["parenthesized", "nonnull", "assertion"].includes(node.kind)) return this.evaluate(node.object!, scope);
    if (node.kind === "unary") {
      if (node.operator === "delete" && node.object?.kind === "member") { this.invalidate(this.reference(node.object, scope)); return unresolvedValue; }
      const value = this.evaluate(node.object!, scope);
      return node.operator === "await" ? value.kind === "promise" ? value.settled! : value : unresolvedValue;
    }
    if (node.kind === "member") { const reference = this.memberReference(node, scope)!; return typeof reference.property === "string" ? this.member(reference.owner, reference.property) : { kind: "unknown", document: reference.owner.document, authority: reference.owner.authority }; }
    if (["arrow", "function"].includes(node.kind)) return { kind: "helper", node, scope };
    if (node.kind === "template") return this.template(node, scope);
    if (node.kind === "new") {
      const callee = this.evaluate(node.callee!, scope), args = (node.arguments ?? []).map(arg => this.evaluate(arg, scope));
      if (callee.kind === "constructor") { const options = this.constant(args[0] ?? unresolvedValue) as Record<string, unknown> | undefined; return { kind: "ajv", registry: new Map(), authority: { tainted: !!args.length && (!options || ["useDefaults", "removeAdditional", "coerceTypes"].some(key => !!options[key])) } }; }
      if (callee.kind === "namespace" && callee.name === "URL" && args[1]?.kind === "namespace" && args[1].name === "import.meta.url") { const data = this.constant(args[0] ?? unresolvedValue); return typeof data === "string" && !/^[a-z][a-z\d+.-]*:|[?#]/iu.test(data) ? { kind: "url", name: posix.normalize(posix.join(posix.dirname(this.readerPath), data)) } : unresolvedValue; }
      for (const value of args) { if (value.kind === "helper") this.deferred.push(value); else this.invalidate(value); }
      this.invalidate(callee); this.issue(node, "constructor-unresolved");
      return unresolvedValue;
    }
    if (node.kind === "spread") { this.evaluate(node.object!, scope); this.issue(node, "spread-construction-unresolved"); return unresolvedValue; }
    if (node.kind === "call") return this.call(node, scope);
    if (node.kind === "assignment") {
      const owner = node.left ? this.reference(node.left, scope) : undefined, value = this.evaluate(node.right!, scope);
      if (node.left?.kind === "identifier") { const binding = scope.binding(node.left.name!); if (binding) binding.value = node.operator === "=" ? value : unresolvedValue; }
      this.invalidate(owner);
      return value;
    }
    if (node.kind === "object") {
      const data: Record<string, unknown> = {}, dependencies: ReadDocument[] = []; let resolved = true;
      for (const property of node.properties ?? []) {
        const key = property.key?.kind === "identifier" && !property.computed ? property.key.name : property.key ? this.constant(this.evaluate(property.key, scope)) : undefined, evaluated = this.evaluate(property.value, scope), value = this.constant(evaluated);
        if (typeof key !== "string" || value === undefined) resolved = false;
        else data[key] = value;
        if (evaluated.document) dependencies.push(evaluated.document);
      }
      return resolved ? this.inline(node, data, dependencies) : unresolvedValue;
    }
    if (node.kind === "array") { const values = (node.elements ?? []).map(element => this.evaluate(element, scope)), data = values.map(value => this.constant(value)); return data.every(value => value !== undefined) ? this.inline(node, data, values.flatMap(value => value.document ? [value.document] : [])) : unresolvedValue; }
    if (node.kind === "binary") {
      const left = this.constant(this.evaluate(node.left!, scope));
      if (["&&", "||", "??"].includes(node.operator!)) {
        const skipped = this.fork(scope), evaluated = this.fork(scope); this.conditionalDepth++;
        try { this.evaluate(node.right!, evaluated); } finally { this.conditionalDepth--; }
        this.merge(scope, skipped, evaluated); return unresolvedValue;
      }
      const right = this.constant(this.evaluate(node.right!, scope)); return node.operator === "+" && typeof left === "string" && typeof right === "string" ? { kind: "constant", data: left + right } : unresolvedValue;
    }
    if (node.kind === "conditional") {
      this.evaluate(node.condition!, scope); const left = this.fork(scope), right = this.fork(scope); this.conditionalDepth++;
      try { this.evaluate(node.whenTrue!, left); this.evaluate(node.whenFalse!, right); } finally { this.conditionalDepth--; }
      this.merge(scope, left, right);
    }
    return unresolvedValue;
  }
  statements(rows: readonly EcmaStatement[], scope = new ReadScope()): ReadValue {
    if (scope.functionBoundary) this.hoist(rows, scope);
    for (const row of rows) {
      for (const declaration of row.declarations ?? []) for (const name of declaration.pattern.names) {
        const target = row.kind === "var" ? scope.functionScope() : scope;
        if (row.kind !== "var" || !target.bindings.has(name)) target.bindings.set(name, { value: unresolvedValue, imported: false });
      }
      for (const binding of row.imports ?? []) scope.bindings.set(binding.local, { value: unresolvedValue, imported: true });
      if (row.kind === "function") scope.bindings.set(row.name!, { value: { kind: "helper", node: row, scope }, imported: false });
    }
    for (const row of rows) {
      this.checkCancellation?.();
      if (row.kind === "import") for (const binding of row.imports ?? []) scope.bindings.get(binding.local)!.value = binding.runtime ? this.imported(binding.module, binding.imported) : unresolvedValue;
      else if (["const", "let", "var"].includes(row.kind)) for (const declaration of row.declarations ?? []) {
        const value = this.evaluate(declaration.initializer, scope);
        this.patternEffects(declaration.pattern, scope);
        for (const name of declaration.pattern.names) (row.kind === "var" ? scope.functionScope() : scope).bindings.get(name)!.value = declaration.pattern.destructured ? declaration.pattern.objectBindings ? this.member(value, declaration.pattern.objectBindings.find(binding => binding.local === name)!.imported) : unresolvedValue : value;
      }
      else if (row.kind === "return") return row.expression ? this.evaluate(row.expression, scope) : unresolvedValue;
      else if (row.kind === "expression" || row.kind === "throw") this.evaluate(row.expression!, scope);
      else if (row.kind === "block") this.statements(row.body!, new ReadScope(scope));
      else if (row.kind === "export") this.statements([row.statement!], scope);
      else if (row.kind === "if") {
        this.evaluate(row.expression!, scope); const left = this.fork(scope), right = this.fork(scope);
        this.conditionalDepth++;
        try { this.statements([row.then!], left); if (row.otherwise) this.statements([row.otherwise], right); } finally { this.conditionalDepth--; }
        this.merge(scope, left, right);
        if (this.returns(row.then!) || row.otherwise && this.returns(row.otherwise)) { this.issue(row, "conditional-return-unresolved"); return unresolvedValue; }
      }
      else if (row.kind === "for") {
        const value = this.evaluate(row.iterable!, scope), child = new ReadScope(scope);
        for (const name of row.initializer!.names) child.bindings.set(name, { value: value.kind === "json" ? this.member(value, "*") : unresolvedValue, imported: false });
        this.statements([row.statement!], child);
      } else if (row.kind === "function") this.deferred.push(scope.bindings.get(row.name!)!.value);
      else if (row.kind === "class") this.issue(row, "class-body-unresolved");
    }
    return unresolvedValue;
  }
  finish(): void {
    const inspected = new Set<EcmaExpression | EcmaStatement>();
    for (const helper of this.deferred) {
      this.checkCancellation?.();
      if (inspected.has(helper.node!)) continue;
      inspected.add(helper.node!);
      this.invoke({ ...helper, scope: this.fork(helper.scope!, true) }, []);
    }
  }
}

/** 🧬️ Inspects original bindings through caller-supplied source access; unresolved syntax never proves absence. */
export function inspectSchemaValidationReads(readerPath: string, source: string, read: (path: string) => string | undefined, checkCancellation?: () => void): SchemaValidationReadReport {
  const program = ecmaProgram(source, checkCancellation), inspector = new SchemaReadInspector(readerPath, source, read, checkCancellation);
  if (program) { inspector.statements(program); inspector.finish(); }
  else inspector.unresolved.push({ start: 0, end: source.length, reason: "unsupported-or-malformed-syntax" });
  return { readerPath, readerHash: inspector.readerHash, completeSyntax: program !== null, reads: inspector.reads, unresolved: inspector.unresolved };
}

/** 🎛️ Caller-owned progress and cancellation for original source inspection. */
export interface SchemaReadObserver { readonly onProgress?: (progress: SchemaReadProgress) => void; readonly checkCancellation?: () => void; }

/** 🗂️ Inspects every supplied JavaScript or TypeScript source without treating incomplete evidence as absence. */
export async function inspectSchemaValidationReadIndex(paths: readonly string[], read: (path: string) => string | undefined, observer: SchemaReadObserver = {}): Promise<SchemaValidationReadIndex> {
  observer.checkCancellation?.();
  const sources = [...new Set(paths.filter(path => /\.(?:[cm]?[jt]s|[jt]sx)$/u.test(path)))].sort((left, right) => Buffer.from(left).compare(Buffer.from(right))), reports: SchemaValidationReadReport[] = [], unavailablePaths: string[] = [], contents = new Map<string, string | undefined>();
  const original = (path: string): string | undefined => { observer.checkCancellation?.(); if (!contents.has(path)) contents.set(path, read(path)); return contents.get(path); };
  observer.onProgress?.({ phase: "inspect", completed: 0, total: sources.length, path: null });
  for (const [index, path] of sources.entries()) {
    observer.checkCancellation?.();
    const source = original(path);
    if (source === undefined) {
      unavailablePaths.push(path);
      reports.push({ readerPath: path, readerHash: null, completeSyntax: false, reads: [], unresolved: [{ start: 0, end: 0, reason: "source-unavailable" }] });
    } else reports.push(inspectSchemaValidationReads(path, source, original, observer.checkCancellation));
    observer.onProgress?.({ phase: "inspect", completed: index + 1, total: sources.length, path });
    if ((index + 1) % 16 === 0) await new Promise<void>(accept => setImmediate(accept));
  }
  observer.checkCancellation?.();
  return { reports, unavailablePaths };
}
