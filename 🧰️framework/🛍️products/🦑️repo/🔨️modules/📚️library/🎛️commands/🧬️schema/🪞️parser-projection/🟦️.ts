/** 🪞️ Projects the JSON Schema exports of a scope into the other formats its module provides: the GraphQL type, the protobuf message, the TypeScript type and its typed `parse<Export>` entry point.
 *
 * Execution contract: every export must exist in every format the scope provides. The normative JSON Schema already states the
 * contract, so each projection is derivable from it, and a derived projection is the only kind that cannot drift from the contract
 * it projects. It never invents a Rust type: a Rust entity that exists nowhere is reported, because the Rust file is the crate's real code.
 *
 * `closing` limits the TypeScript pass to the parsers existing code already calls but never declared (and what those need), so a call
 * can never dangle. `replace` regenerates the named parsers that exist but are wrong (for example a typed map an earlier guard left untyped),
 * and `restate` regenerates every parser the audit finds drifted for the named reasons. A regenerated parser keeps its place in the file.
 * A parser is emitted only when every parser it calls is declared on disk or emitted in the same pass, and a parser never resolves to
 * itself. A parser refuses an unknown property exactly where its schema states `additionalProperties: false` (through `<prefix>Closed`,
 * which a file receives with its first such parser), and nowhere else. The freshness gate (`check`) fails on a write the closing projection
 * would still make, a call that dangles, a parser that only calls itself, a schema-typed map read as an opaque object, a parser that
 * accepts an unknown property its schema refuses, any other parser that differs from its projection, and a call of `<prefix>Closed` that
 * no helper block defines. The only parsers left out are the ones in the registry of hand-written parsers no schema can own yet
 * (`🧱️unowned`), and the gate fails on a listed parser that no longer drifts.
 *
 * A leaf split into component files keeps its parsers with the components: a new parser is written to the `🚪️parsers` folder next to the
 * file that declares its type (to the leaf when the leaf declares it), and the audit reads every file of the leaf's module tree below its
 * `//#region 🚪️Parsers` marker, so a parser is checked wherever it lives and a second definition of one is a drift (`duplicate`). The leaf
 * re-exports nothing: whoever needs a parser, a sibling document or hand-written code, imports it from the file that holds it. The parser
 * folders import the guard helpers of the leaf, so the leaf must never import a folder: a parser stays in the leaf when its type is declared
 * there, when the leaf's own code or a parser that stays calls it, when its leaf is named by a package specifier, and when it sits in a cycle
 * of calls between folders. `--distribute <leaf folder>` moves the fresh parsers of the leaves below a folder into the component folders,
 * verbatim and in leaf order.
 *
 * A property whose schema states no structure while its declared type instantiates a generic of the codebase (`NamedTripleDiff<string, D, T>`) has no projection: the opaque read cannot produce that type, so the parser is refused (the schema must state the structure, or the parser is listed in `🧱️unowned`) rather than emitted with a cast.
 *
 * Documented limits: a schema property named like an array index (`"0"`, `"1"`) cannot keep its document order in a JavaScript object, so
 * its export is refused; integers beyond 2^53 stay exact, other floats print in JavaScript form (`1.0` as `1`); the tracked and untracked
 * files are listed through git, a tree that is no work tree is walked instead.
 * @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧬️schema/🟦️.ts
 */
import { existsSync } from "node:fs";
import { dirname, relative } from "node:path";
import { join } from "node:path";
import { EXPORT_NAME, codePointOrder, escapeRegExp, jsonString, jsonValue, type Dict } from "./🔡️json-text/🟦️.ts";
import { declaredOptionalFields, declares, declaresParser, definesParser, existingHelper, importsParser, parserRange } from "./📖️declared-types/🟦️.ts";
import { Context, findOwner, parserKey, parserKeyParts, pointed, renderParser, type Owner, type ParserKey } from "./🧾️parser-render/🟦️.ts";
import { renderGraphql, renderProto, renderTsType, type Taken, type TypeScope } from "./📐️type-render/🟦️.ts";
import { PARSER_REGION } from "./🚪️guard-helpers/🟦️.ts";
import { LeafWriter } from "./✍️leaf-writer/🟦️.ts";
import { ParserFiles, ParserHomes, ownedParserNames, pinnedParsers } from "./📍️parser-files/🟦️.ts";
import { loadUnowned, unownedKey, type UnownedParser } from "./🧱️unowned/🟦️.ts";
import { FORMATS, LEAF, SchemaCatalog, SourceStore, buildOwnerIndex, exportsOf, facetOf, helperPrefix, leafPath, portable, readDocument } from "./🗂️documents/🟦️.ts";

//#region 🔖️Options
const TS = "🟦️typescript";
const JSON_SCHEMA = "🔣️jsonschema";
const RUST = "🦀️rust";

/** 🔎️ Why a projected parser no longer equals what the current schema and renderers produce: `self-call` only delegates to itself and never returns, `untyped-map` reads a schema-typed map as an opaque object, `open` equals the projection without the refusal of unknown properties that its schema requires, `unrenderable` has no projection at all, `duplicate` is a second projected definition of a parser another file of the same leaf already defines, `stale` differs in any other way. */
export type DriftReason = "self-call" | "untyped-map" | "open" | "stale" | "unrenderable" | "duplicate";

/** 🔎️ The drift reasons `restate` can regenerate. */
export type RestateReason = Exclude<DriftReason, "unrenderable" | "duplicate">;

const RESTATE_REASONS: readonly RestateReason[] = ["self-call", "untyped-map", "open", "stale"];

/** 🧭️ How one projection run is scoped and what it may write. */
export interface ProjectionOptions {
  readonly root: string;
  readonly mode: "report" | "apply";
  readonly under: string;
  readonly formats: ReadonlySet<string>;
  readonly closing: boolean;
  readonly replace: ReadonlySet<string>;
  readonly restate: ReadonlySet<RestateReason>;
  readonly distribute?: readonly string[];
  readonly unowned?: readonly UnownedParser[];
  readonly catalog?: SchemaCatalog;
  readonly signal?: AbortSignal;
  readonly onProgress?: (module: string) => void;
}

/** 📊️ What one projection run did or would do. */
export interface ProjectionReport {
  readonly mode: string;
  readonly under: string;
  readonly closing: boolean;
  readonly added: readonly (readonly [string, number])[];
  readonly filesTouched: number;
  readonly refused: readonly string[];
  readonly rustAbsent: readonly string[];
  readonly touched: readonly string[];
}

/** 🧭️ The formats projected when none is named. */
export const DEFAULT_FORMATS: readonly string[] = ["🔗️graphql", "🛰️protobuf", TS];

/** 🧭️ The default scope: every plugin and app contract. */
export const DEFAULT_SCOPE = "✏️s";

/** 🧭️ The scopes the freshness gate covers. */
export const CHECKED_SCOPES: readonly string[] = ["✏️s", "🌎️hub"];

const VALUE_FLAGS = new Set(["--under", "--formats", "--replace", "--restate", "--distribute"]);
const SWITCH_FLAGS = new Set(["--closing", "--all"]);

/** 🔤️ Reads `<report|apply|check> [--under <path>] [--formats a,b] [--closing] [--replace A,B] [--restate open,stale,… | all] [--distribute <leaf folder>,…] [--all]`; a mistyped flag is refused. */
export function parseProjectionArguments(root: string, argv: readonly string[]): ProjectionOptions & { readonly command: string; readonly all: boolean; readonly distribute: readonly string[] } {
  const named = argv[0] !== undefined && !argv[0].startsWith("--");
  const command = named ? argv[0]! : "report";
  if (!["report", "apply", "check"].includes(command)) throw new Error(`[schema parsers] unknown command ${JSON.stringify(command)} (expected report | apply | check).`);
  for (let at = named ? 1 : 0; at < argv.length; at += 1) {
    const token = argv[at]!;
    if (VALUE_FLAGS.has(token)) {
      const found = argv[at + 1];
      if (found === undefined || found.startsWith("--")) throw new Error(`[schema parsers] ${token} requires a value.`);
      at += 1;
    } else if (!SWITCH_FLAGS.has(token)) throw new Error(`[schema parsers] unknown argument ${JSON.stringify(token)}.`);
  }
  const value = (flag: string): string | undefined => {
    const at = argv.indexOf(flag);
    return at < 0 ? undefined : argv[at + 1];
  };
  const formats = new Set((value("--formats") ?? DEFAULT_FORMATS.join(",")).split(","));
  for (const format of formats) if (!DEFAULT_FORMATS.includes(format)) throw new Error(`[schema parsers] unknown format ${JSON.stringify(format)} (expected ${DEFAULT_FORMATS.join(" | ")}).`);
  const restated = (value("--restate") ?? "").split(",").filter((reason) => reason !== "");
  const restate = new Set<RestateReason>(restated.includes("all") ? RESTATE_REASONS : (restated as RestateReason[]));
  for (const reason of restate) if (!RESTATE_REASONS.includes(reason)) throw new Error(`[schema parsers] unknown restate reason ${JSON.stringify(reason)} (expected ${RESTATE_REASONS.join(" | ")} | all).`);
  return { command, root, mode: command === "apply" ? "apply" : "report", under: value("--under") ?? DEFAULT_SCOPE, formats, closing: argv.includes("--closing"), replace: new Set(value("--replace")?.split(",") ?? []), restate, distribute: (value("--distribute") ?? "").split(",").filter((folder) => folder !== ""), all: argv.includes("--all") };
}
//#endregion 🔖️Options

//#region 🔖️Plans
interface Entry {
  readonly type: string | undefined;
  readonly typed: boolean;
  readonly parser: string | undefined;
  readonly local: ReadonlySet<string>;
  readonly foreign: ReadonlySet<ParserKey>;
}

interface Plan {
  readonly entries: Map<string, Entry>;
  readonly exports: ReadonlySet<string>;
  readonly helper: string;
}

class Counter {
  private readonly counts = new Map<string, number>();

  add(key: string): void {
    this.counts.set(key, (this.counts.get(key) ?? 0) + 1);
  }

  clear(): void {
    this.counts.clear();
  }

  entries(): [string, number][] {
    return [...this.counts];
  }
}

const sortedKeys = (keys: Iterable<ParserKey>): ParserKey[] => [...keys].sort(codePointOrder);

/** 🧭️ Decides which parsers are emitted: one is emitted only when every parser it calls is declared on disk or emitted in the same pass, so a call can never dangle. */
function settle(root: string, plans: ReadonlyMap<string, Plan>, closing: boolean, sources: SourceStore, refused: string[], replaces: (target: string, name: string) => boolean): Set<ParserKey> {
  const candidates = new Map<ParserKey, Entry>();
  for (const [target, plan] of plans) for (const [name, entry] of plan.entries) if (entry.parser !== undefined && entry.typed) candidates.set(parserKey(target, name), entry);
  let emit = new Set(candidates.keys());
  if (closing) {
    const needed = new Set<ParserKey>();
    const stack: ParserKey[] = [];
    for (const [target, plan] of plans) {
      const source = sources.parsers(target);
      const called = new Set([...source.matchAll(/\bparse([A-Z][A-Za-z0-9]*)\(/gu)].map((match) => match[1]!));
      for (const name of plan.exports) if (replaces(target, name)) called.add(name);
      for (const name of [...called].sort(codePointOrder)) {
        if (plan.exports.has(name) && ((!definesParser(source, name) && !importsParser(source, name)) || replaces(target, name))) stack.push(parserKey(target, name));
      }
    }
    while (stack.length > 0) {
      const key = stack.pop()!;
      if (needed.has(key)) continue;
      const [owner, name] = parserKeyParts(key);
      const entry = candidates.get(key);
      if (entry === undefined) {
        refused.push(`dangling parse${name} ${portable(root, owner)} has no renderable parser`);
        continue;
      }
      needed.add(key);
      const text = sources.parsers(owner);
      for (const callee of [...entry.local].sort(codePointOrder)) if (!definesParser(text, callee) && !importsParser(text, callee)) stack.push(parserKey(owner, callee));
      for (const foreignKey of sortedKeys(entry.foreign)) {
        const [foreignOwner, callee] = parserKeyParts(foreignKey);
        if (!definesParser(sources.parsers(foreignOwner), callee)) stack.push(foreignKey);
      }
    }
    emit = needed;
  }
  const available = (owner: string, name: string): boolean => emit.has(parserKey(owner, name)) || definesParser(sources.parsers(owner), name) || importsParser(sources.parsers(owner), name);
  let changed = true;
  while (changed) {
    changed = false;
    for (const key of sortedKeys(emit)) {
      const entry = candidates.get(key)!;
      const [owner] = parserKeyParts(key);
      const missing = [
        ...[...entry.local].sort(codePointOrder).filter((name) => !available(owner, name)).map((name) => `parse${name}`),
        ...sortedKeys(entry.foreign).map(parserKeyParts).filter(([foreignOwner, name]) => !available(foreignOwner, name)).map(([, name]) => `parse${name}`),
      ];
      if (missing.length > 0) {
        refused.push(`parser ${parserKeyParts(key)[1]} ${portable(root, owner)} needs ${missing.join(", ")}`);
        emit.delete(key);
        changed = true;
      }
    }
  }
  return emit;
}
//#endregion 🔖️Plans

//#region 🔖️Projection
const rendererOf = (format: string, name: string, definition: Dict, taken: Taken, scope: TypeScope): string | undefined => (format === "🔗️graphql" ? renderGraphql(name, definition, taken) : format === "🛰️protobuf" ? renderProto(name, definition, taken) : renderTsType(name, definition, scope));

/** 🧭️ The module specifier under which the leaf at `from` names the leaf at `to`: the package export for a framework leaf, the relative path otherwise. */
function leafSpecifier(from: string, to: string, owner: Owner): string | undefined {
  if (owner.shared) return owner.specifier;
  const path = relative(dirname(from), to).split("\\").join("/");
  return path.startsWith(".") ? path : `./${path}`;
}

/** 🧭️ What a TypeScript type may follow from one document: an export of this document is named, a pointer into this document is rendered in place, and a whole other document, one of its exports or a pointer into it is named or rendered through `import("<specifier>")`, so the type needs no import statement. */
function typeScope(index: ReadonlyMap<string, Owner>, document: Dict | undefined, leaf: string, receiver: string, qualifier?: string): TypeScope {
  const documentId = typeof document?.$id === "string" ? document.$id : undefined;
  const named = (name: string): string => (qualifier === undefined ? name : `import(${jsonString(qualifier, false)}).${name}`);
  return {
    type(reference: string) {
      const hash = reference.indexOf("#");
      const location = hash < 0 ? reference : reference.slice(0, hash);
      const pointer = hash < 0 ? "" : reference.slice(hash + 1);
      const foreign = location !== "" && location !== documentId;
      const owner = foreign ? findOwner(index, location, leaf) : undefined;
      const last = pointer === "" ? owner?.root : pointer.split("/").at(-1);
      if (foreign && owner === undefined) return last !== undefined && EXPORT_NAME.test(last) && pointer !== "" ? last : undefined;
      const exported = last !== undefined && EXPORT_NAME.test(last) && (owner === undefined ? pointer.startsWith("/") : owner.exports.has(last));
      if (owner !== undefined) {
        const specifier = leafSpecifier(receiver, owner.ts, owner);
        if (specifier === undefined) return undefined;
        if (exported) return `import(${jsonString(specifier, false)}).${last}`;
        if (!pointer.startsWith("/")) return undefined;
        const target = pointed(pointer, owner.load());
        return target === undefined ? undefined : { node: target, scope: typeScope(index, owner.load(), owner.ts, receiver, specifier) };
      }
      if (exported) return named(last!);
      if (!pointer.startsWith("/")) return undefined;
      const target = pointed(pointer, document);
      return target === undefined ? undefined : { node: target, scope: this };
    },
  };
}

/** 🪞️ Projects the exports of every schema module under `options.under`; `report` only plans, `apply` writes the leaves. */
export function projectParsers(options: ProjectionOptions): ProjectionReport {
  const { root, mode, under, formats, closing, replace } = options;
  const catalog = options.catalog ?? new SchemaCatalog(root);
  const restated = new Set<ParserKey>();
  if (options.restate.size > 0) {
    const unowned = new Set((options.unowned ?? loadUnowned().parsers).map((row) => unownedKey(row.path, row.name)));
    for (const row of auditParsers({ root, under, catalog, signal: options.signal })) {
      if (row.reason !== "unrenderable" && row.reason !== "duplicate" && options.restate.has(row.reason) && !unowned.has(unownedKey(row.path, row.name))) restated.add(parserKey(join(root, row.leaf), row.name));
    }
  }
  const replaces = (target: string, name: string): boolean => replace.has(name) || restated.has(parserKey(target, name));
  const index: ReadonlyMap<string, Owner> = buildOwnerIndex(root, catalog);
  const specifiers = new Map([...index.values()].filter((owner) => owner.specifier !== undefined).map((owner) => [owner.ts, owner.specifier!] as const));
  const sources = new SourceStore();
  const promised = new Set<ParserKey>();
  const added = new Counter();
  const refused: string[] = [];
  const rustAbsent: string[] = [];
  const touched = new Set<string>();
  const plans = new Map<string, Plan>();
  const modules = catalog.modules(under);
  if (modules.length === 0) throw new Error(`[schema parsers] no schema module under ${JSON.stringify(under)}.`);
  const documents = new Map<string, Dict | undefined>();
  const load = (path: string): Dict | undefined => {
    if (!documents.has(path)) documents.set(path, readDocument(root, path));
    return documents.get(path);
  };
  for (const scanPass of [0, 1]) {
    const dry = scanPass === 0;
    plans.clear();
    added.clear();
    refused.length = 0;
    rustAbsent.length = 0;
    touched.clear();
    for (const module of modules) {
      options.signal?.throwIfAborted();
      options.onProgress?.(module);
      const provided = FORMATS.filter((format) => existsSync(`${root}/${module}/${LEAF[format as keyof typeof LEAF]}`));
      const moduleDocumentPaths = catalog.documents(module);
      const siblings = moduleDocumentPaths.map((other) => leafPath(root, module, facetOf(module, other), TS));
      for (const path of moduleDocumentPaths) {
        const document = load(path);
        if (document === undefined) continue;
        const facet = facetOf(module, path);
        const exports = exportsOf(document);
        if (exports.size === 0) continue;
        const scope = typeScope(index, document, leafPath(root, module, facet, TS), leafPath(root, module, facet, TS));
        for (const format of provided) {
          if (format === JSON_SCHEMA) continue;
          const target = leafPath(root, module, facet, format as keyof typeof LEAF);
          if (!existsSync(target)) continue;
          const relativeTarget = portable(root, target);
          const source = sources.parsers(target);
          const appended: string[] = [];
          const helper = existingHelper(source) ?? helperPrefix(document.$id);
          let plan: Plan | undefined;
          if (format === TS) {
            plan = plans.get(target);
            if (plan === undefined) {
              plan = { entries: new Map(), exports: new Set(exports.keys()), helper };
              plans.set(target, plan);
            }
          }
          for (const [name, definition] of exports) {
            const restricted = definition["x-semio-formats"];
            if (Array.isArray(restricted) && !restricted.includes(format)) continue;
            let present = declares(format, sources.declarations(target), name);
            let typeText: string | undefined;
            if (!present && formats.has(format) && !(closing && format === TS)) {
              const taken: Taken = (other) => exports.has(other) || declares(format, sources.declarations(target), other) || declares(format, appended.join("\n"), other);
              const rendered = rendererOf(format, name, definition, taken, scope);
              if (rendered === undefined) {
                refused.push(`${format} ${name} ${relativeTarget}`);
                continue;
              }
              typeText = rendered;
              if (format !== TS) {
                appended.push(rendered);
                added.add(format);
              }
              present = true;
            } else if (!present && format === TS && closing) {
              typeText = renderTsType(name, definition, scope);
              present = typeText !== undefined;
            } else if (!present && format === RUST) {
              rustAbsent.push(`${name} ${relativeTarget}`);
              continue;
            }
            if (format === TS && (!declaresParser(source, name) || replaces(target, name))) {
              const context = new Context(helper, typeof document.$id === "string" ? document.$id : undefined, index, target, siblings, sources, promised, true, document);
              const rendered = present ? renderParser(name, definition, context, declaredOptionalFields(sources.declarations(target), name)) : undefined;
              if (rendered === undefined && present) refused.push(`parser ${name} ${relativeTarget}`);
              const local = new Set(context.local);
              local.delete(name);
              plan!.entries.set(name, { type: typeText, typed: present, parser: rendered, local, foreign: new Set(context.foreign) });
            }
          }
          if (format === TS || appended.length === 0 || mode !== "apply" || dry) continue;
          sources.write(target, `${source.trimEnd()}\n\n${appended.join("\n").trimEnd()}\n`);
          touched.add(relativeTarget);
        }
      }
    }
    for (const [owner, plan] of plans) for (const [name, entry] of plan.entries) if (entry.parser !== undefined) promised.add(parserKey(owner, name));
  }
  const emit = settle(root, plans, closing || replace.size > 0 || restated.size > 0, sources, refused, replaces);
  const spread = options.distribute ?? [];
  const drifted = new Set<ParserKey>();
  if (spread.length > 0) for (const row of auditParsers({ root, under, catalog, signal: options.signal })) drifted.add(parserKey(join(root, row.leaf), row.name));
  const kept = new Map<string, string[]>();
  const fresh = new Map<string, string[]>();
  for (const [target, plan] of plans) {
    const keep = [...emit].map(parserKeyParts).filter(([owner]) => owner === target).map(([, name]) => name).sort(codePointOrder);
    kept.set(target, keep);
    const leaf = sources.read(target);
    const region = leaf.indexOf(PARSER_REGION);
    const distributes = spread.some((folder) => portable(root, target).startsWith(`${folder}/`));
    fresh.set(target, !distributes || region < 0 ? [] : ownedParserNames(leaf.slice(region)).filter((name) => plan.exports.has(name) && !keep.includes(name) && !replaces(target, name) && !drifted.has(parserKey(target, name))));
  }
  const pins = new Map<string, ReadonlySet<string>>();
  for (const [target, plan] of plans) {
    const files = new ParserFiles(sources, target);
    const names = [...plan.exports].filter((name) => kept.get(target)!.includes(name) || files.definingFile(name) !== undefined).sort(codePointOrder);
    pins.set(target, pinnedParsers(sources, target, names, new Map(kept.get(target)!.map((name) => [name, plan.entries.get(name)!.local] as const)), new Set(fresh.get(target))));
  }
  const homes = new ParserHomes(sources, pins, new Set(specifiers.keys()), new Map([...fresh].map(([target, names]) => [target, new Set(names)] as const)));
  for (const [target, plan] of plans) {
    const keep = kept.get(target)!;
    const distributes = spread.some((folder) => portable(root, target).startsWith(`${folder}/`));
    if (keep.length === 0 && !distributes) continue;
    const writer = new LeafWriter(sources, root, target, plan.helper, mode === "apply", specifiers, homes);
    for (const name of keep) {
      const entry = plan.entries.get(name)!;
      if (entry.type !== undefined && !declares(TS, sources.declarations(target), name)) {
        writer.addType(entry.type);
        added.add(TS);
      }
      const closedCall = entry.parser!.includes(`${plan.helper}Closed(`);
      if (replaces(target, name) && definesParser(sources.parsers(target), name)) {
        added.add("restated");
        writer.restate(name, entry.parser!, entry.foreign, closedCall);
      } else {
        writer.add(name, entry.parser!, entry.foreign, closedCall);
        added.add("parser");
      }
    }
    if (distributes) writer.distribute(fresh.get(target)!);
    for (const file of writer.finish()) touched.add(file);
  }
  return { mode, under, closing, added: added.entries(), filesTouched: touched.size, refused: [...refused], rustAbsent: [...rustAbsent], touched: [...touched].sort(codePointOrder) };
}

//#endregion 🔖️Projection

//#region 🔖️Audit
/** 🔎️ A projected parser that no longer equals what the current schema and renderers produce: `path` is the file that holds it, `leaf` the leaf of its schema document. */
export interface ParserDrift {
  readonly path: string;
  readonly leaf: string;
  readonly name: string;
  readonly reason: DriftReason;
  readonly actual: string;
  readonly expected: string | undefined;
}

/** 🔎️ What one audit found: the drifted parsers, and the leaves whose parsers call `<prefix>Closed` without a helper block that defines it. */
export interface ParserAudit {
  readonly drift: ParserDrift[];
  readonly helperless: string[];
}

const mapConversions = (text: string): number => text.split("Object.entries(").length - 1;

const SELF_CALL = /^export function parse(\w+)\(value: unknown, at = "\$"\): \1 \{\n  return parse\1\(value[^\n]*\n\}\n/gmu;

/** 🔎️ Re-renders every parser below a parser region in the files of each leaf's module tree (the leaf and its parser folders) and lists the ones whose text differs from the projection of the current schema, the ones defined in two files, and the leaves that call `<prefix>Closed` without defining it, or that hold parser regions without a helper block. */
export function auditLeaves(options: Pick<ProjectionOptions, "root" | "under" | "catalog" | "signal" | "onProgress">): ParserAudit {
  const { root, under } = options;
  const catalog = options.catalog ?? new SchemaCatalog(root);
  const index: ReadonlyMap<string, Owner> = buildOwnerIndex(root, catalog);
  const sources = new SourceStore();
  const promised = new Set<ParserKey>();
  const drift: ParserDrift[] = [];
  const helperless: string[] = [];
  const audited = new Set<string>();
  for (const module of catalog.modules(under)) {
    options.signal?.throwIfAborted();
    options.onProgress?.(module);
    const paths = catalog.documents(module);
    const siblings = paths.map((other) => leafPath(root, module, facetOf(module, other), TS));
    for (const path of paths) {
      const document = readDocument(root, path);
      const target = leafPath(root, module, facetOf(module, path), TS);
      if (document === undefined || !existsSync(target)) continue;
      const regions = sources.owned(target).map((file) => ({ file, source: sources.read(file) })).map((row) => ({ ...row, region: row.source.indexOf(PARSER_REGION) })).filter((row) => row.region >= 0);
      if (regions.length === 0) continue;
      const source = sources.read(target);
      const helper = existingHelper(source) ?? helperPrefix(document.$id);
      const relativeTarget = portable(root, target);
      if (!audited.has(target)) {
        audited.add(target);
        const callsClosed = regions.some((row) => row.source.includes(`${helper}Closed(`)) && !new RegExp(`^export const ${escapeRegExp(helper)}Closed\\b`, "m").test(source);
        if (callsClosed || !source.includes(PARSER_REGION)) helperless.push(relativeTarget);
      }
      for (const [name, definition] of exportsOf(document)) {
        const [first, ...extra] = regions.flatMap((row) => {
          const range = parserRange(row.source, name);
          return range === undefined || range[0] < row.region ? [] : [{ path: portable(root, row.file), text: row.source.slice(range[0], range[1]) }];
        });
        for (const row of extra) drift.push({ path: row.path, leaf: relativeTarget, name, reason: "duplicate", actual: row.text, expected: undefined });
        if (first === undefined) continue;
        const context = (closed: boolean): Context => new Context(helper, typeof document.$id === "string" ? document.$id : undefined, index, target, siblings, sources, promised, closed, document);
        const rendered = renderParser(name, definition, context(true), declaredOptionalFields(sources.declarations(target), name));
        if (rendered === first.text) continue;
        const open = rendered === undefined ? undefined : renderParser(name, definition, context(false), declaredOptionalFields(sources.declarations(target), name));
        const reason: DriftReason = rendered === undefined ? "unrenderable" : open === first.text ? "open" : mapConversions(rendered) > mapConversions(first.text) ? "untyped-map" : "stale";
        drift.push({ path: first.path, leaf: relativeTarget, name, reason, actual: first.text, expected: rendered });
      }
      for (const row of regions) {
        for (const call of row.source.matchAll(SELF_CALL)) {
          if (call.index < row.region) continue;
          const file = portable(root, row.file);
          const already = drift.findIndex((entry) => entry.path === file && entry.name === call[1]);
          if (already >= 0) drift[already] = { ...drift[already]!, reason: "self-call" };
          else drift.push({ path: file, leaf: relativeTarget, name: call[1]!, reason: "self-call", actual: call[0], expected: undefined });
        }
      }
    }
  }
  return { drift, helperless };
}

/** 🔎️ The drifted parsers of a scope; see `auditLeaves`. */
export const auditParsers = (options: Pick<ProjectionOptions, "root" | "under" | "catalog" | "signal" | "onProgress">): ParserDrift[] => auditLeaves(options).drift;

/** 🚦️ The freshness verdict of the projection over a scope. */
export interface ParserCheck {
  readonly pending: readonly string[];
  readonly dangling: readonly string[];
  readonly helperless: readonly string[];
  readonly drift: readonly ParserDrift[];
  readonly orphans: readonly UnownedParser[];
  readonly unowned: number;
  readonly ok: boolean;
}

/** 🚦️ Freshness gate over the checked scopes: the closing projection must write nothing and leave no call dangling, every parser must equal the projection of its schema (the listed unowned parsers excepted), every call of `<prefix>Closed` needs its helper, and a listed parser must still drift. */
export function checkParsers(options: Pick<ProjectionOptions, "root" | "signal" | "onProgress" | "unowned"> & { readonly scopes?: readonly string[] }): ParserCheck {
  const pending: string[] = [];
  const dangling: string[] = [];
  const helperless: string[] = [];
  const found: ParserDrift[] = [];
  const catalog = new SchemaCatalog(options.root);
  const requested = options.scopes ?? CHECKED_SCOPES;
  const scopes = requested.filter((scope) => catalog.modules(scope).length > 0);
  if (scopes.length === 0) throw new Error(`[schema parsers] no schema module under ${JSON.stringify(requested)}.`);
  const registry = options.unowned ?? loadUnowned().parsers;
  for (const under of scopes) {
    const closing = projectParsers({ ...options, catalog, under, mode: "report", formats: new Set([TS]), closing: true, replace: new Set(), restate: new Set(), unowned: registry });
    pending.push(...closing.touched);
    dangling.push(...closing.refused.filter((row) => row.startsWith("dangling ") || /^parser \S+ \S+ needs /u.test(row)));
    const audit = auditLeaves({ ...options, catalog, under });
    found.push(...audit.drift);
    helperless.push(...audit.helperless);
  }
  const listed = new Set(registry.map((row) => unownedKey(row.path, row.name)));
  const drifted = new Set(found.map((row) => unownedKey(row.path, row.name)));
  const drift = found.filter((row) => !listed.has(unownedKey(row.path, row.name)));
  const orphans = registry.filter((row) => scopes.some((scope) => row.path === scope || row.path.startsWith(`${scope}/`)) && !drifted.has(unownedKey(row.path, row.name)));
  return { pending, dangling, helperless, drift, orphans, unowned: found.length - drift.length, ok: pending.length === 0 && dangling.length === 0 && helperless.length === 0 && drift.length === 0 && orphans.length === 0 };
}

/** 🚦️ The verdict as the gate prints it. */
export function renderParserCheck(check: ParserCheck): string {
  const lines = [
    ...check.pending.map((path) => `PENDING ${path}`),
    ...check.dangling.map((row) => `DANGLING ${row}`),
    ...check.helperless.map((path) => `HELPERLESS ${path}`),
    ...check.drift.map((row) => `DRIFT ${row.reason} ${row.name} ${row.path}`),
    ...check.orphans.map((row) => `ORPHAN ${row.name} ${row.path}`),
  ];
  lines.push(`[schema parsers check] ${check.ok ? "fresh" : "stale"}: pending=${check.pending.length} dangling=${check.dangling.length} helperless=${check.helperless.length} drift=${check.drift.length} orphans=${check.orphans.length} unowned=${check.unowned}`);
  return `${lines.join("\n")}\n`;
}
//#endregion 🔖️Audit

//#region 🔖️Report
/** 🧾️ The report as the command prints it: one JSON summary line, then the first refusals, the Rust entities that exist nowhere and every touched leaf. */
export function renderProjectionReport(report: ProjectionReport): string {
  const summary = { mode: report.mode, under: report.under, closing: report.closing, added: Object.fromEntries(report.added), filesTouched: report.filesTouched, refused: report.refused.length, rustAbsent: report.rustAbsent.length };
  const lines = [jsonValue(summary, false)];
  for (const row of report.refused.slice(0, 60)) lines.push(`REFUSED ${row}`);
  for (const row of report.rustAbsent.slice(0, 30)) lines.push(`RUST-ABSENT ${row}`);
  for (const path of report.touched) lines.push(`TOUCHED ${path}`);
  return `${lines.join("\n")}\n`;
}
//#endregion 🔖️Report
