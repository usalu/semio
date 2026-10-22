import {validateJsonSchemaSubset} from "../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type: "json"};
import {resolveBuildHelpLanguageV1} from "../../⚙️operations/📖️help/🟦️.ts";
import { isRuntimeExecutableV1 } from "../../../../../../../🔨️modules/⚙️runtime/🏷️identity/🟦️.ts";

/** 🧾️ The closed derivation document: named glob profiles and the rules that select targets and name what they read. */
export type InputPrecisionDocumentV1 = {readonly version: 1; readonly exclusions: readonly string[]; readonly retained: readonly string[]; readonly profiles: Readonly<Record<string, {readonly description: string; readonly reads: readonly InputPrecisionReadV1[]}>>; readonly rules: readonly InputPrecisionRuleV1[]; readonly routes: readonly InputPrecisionRouteV1[]; readonly messages: InputPrecisionMessagesV1};
/** 💬️ The progress texts of the program analysis in every language of the document. */
export type InputPrecisionMessagesV1 = {readonly languages: readonly string[]; readonly texts: Readonly<Record<"deriving" | "progress" | "done", Readonly<Record<string, string>>>>};
/** 🔀️ One owner delegation: the script words a root command forwards to an owner target. */
export type InputPrecisionRouteV1 = {readonly words: readonly string[]; readonly project: string; readonly target: string; readonly root: string};
/** 🎯️ One rule: which targets it selects, which inherited groups it narrows and what the task reads instead. */
export type InputPrecisionRuleV1 = {readonly id: string; readonly description: string; readonly select: {readonly project: string; readonly targets?: readonly string[]; readonly commands?: readonly (readonly string[])[]}; readonly evidence: "traced" | "static" | "bound" | "whole" | "assumed"; readonly replaces: readonly string[]; readonly reads: readonly InputPrecisionReadV1[]};
/** 📖️ The closed union of derivations a rule can name. */
export type InputPrecisionReadV1 =
  | {readonly profile: string}
  | {readonly globs: readonly string[]}
  | {readonly kinds: readonly string[]; readonly under: readonly string[]; readonly named?: boolean}
  | {readonly cargo: {readonly packages?: readonly string[]; readonly directories?: readonly string[]}}
  | {readonly script: {readonly source: string}}
  | {readonly dispatch: {readonly source: string; readonly after: readonly string[]}}
  | {readonly program: {readonly tsconfig: string}}
  | {readonly owner: "route"}
  | {readonly input: string};
/** 🧾️ The decision a narrowed or assumed target records in `metadata.semio.inputPrecision`: which rule bounds its inputs and how well the rule is evidenced. */
export type InputPrecisionDecisionV1 = {readonly version: 1; readonly rule: string; readonly evidence: InputPrecisionRuleV1["evidence"]};
/** 🏷️ The identity the engine selects on: project, target and the script command the target runs. */
export type InputPrecisionTargetV1 = {readonly project: string; readonly target: string; readonly command?: string};
/** 🧰️ The file system, the taxonomy and the dependency analyses the engine consults; every one is injected so the engine owns no I/O. */
export type InputPrecisionHostV1 = {
  readonly exists: (path: string) => boolean;
  readonly readText: (path: string) => string | undefined;
  readonly listDirectory: (path: string) => readonly {readonly name: string; readonly directory: boolean}[];
  readonly kinds: Readonly<Record<string, {readonly emoji: string; readonly extensionChains: readonly string[]}>>;
  readonly importClosure: (entries: readonly string[]) => Promise<readonly string[]>;
  readonly program?: ProgramHostV1;
  readonly cargoManifests: () => readonly string[];
  readonly cargoRoots: (crate: string) => readonly string[];
  readonly cargoSources: (crate: string) => readonly string[] | undefined;
  readonly cargoToolchain: readonly string[];
  readonly cargoIdentity?: (crateDirectory: string) => unknown;
};
/** 📤️ What a rule derives: patterns for the Nx globs, named groups to keep (the retained toolchain identity first), exact files for the command-source group, the groups it drops and what could not be resolved. */
export type InputPrecisionResultV1 = {readonly rule: string; readonly drops: readonly string[]; readonly patterns: readonly string[]; readonly groups: readonly string[]; readonly files: readonly string[]; readonly unresolved: readonly string[]; readonly markers?: readonly unknown[]; readonly reads: readonly {readonly read: string; readonly patterns: number; readonly files: number}[]};

/** 🚫️ Refusal of a derivation that cannot be bounded. */
export class InputPrecisionErrorV1 extends Error {
  constructor(message: string) {
    super(message);
    this.name = "InputPrecisionErrorV1";
  }
}

const WORKSPACE = "{workspaceRoot}/";
const compiledGlobs = new Map<string, RegExp>();
/** 🧷️ Compiles an input glob (`*`, `**`, `?`, `{a,b}`) to the exact expression of the path it selects. */
export function inputGlobRegExpV1(pattern: string): RegExp {
  const known = compiledGlobs.get(pattern);
  if (known) return known;
  let source = "", depth = 0;
  for (let index = 0; index < pattern.length; index++) {
    const character = pattern[index]!;
    if (character === "*") {
      if (pattern[index + 1] !== "*") source += "[^/]*";
      else if (pattern[index + 2] === "/") {
        index += 2;
        source += "(?:.*/)?";
      } else {
        index += 1;
        source += ".*";
      }
    } else if (character === "?") source += "[^/]";
    else if (character === "{") {
      depth++;
      source += "(?:";
    } else if (character === "}" && depth) {
      depth--;
      source += ")";
    } else if (character === "," && depth) source += "|";
    else source += character.replace(/[.+^$()|\\[\]{}]/u, "\\$&");
  }
  const expression = new RegExp("^" + source + "$", "su");
  compiledGlobs.set(pattern, expression);
  return expression;
}
/** 🧷️ Whether a workspace-relative path matches one input glob. */
export const globMatchesV1 = (pattern: string, path: string): boolean => inputGlobRegExpV1(pattern).test(path);
const WHOLE = /^(?:\*\*\/\*|\*\*|\*)$/u;
const REPOSITORY = /^(?:\*\*\/\*|\*\*)$/u;
const bare = (pattern: string): string => pattern.replace(/^!?\{(?:workspaceRoot|projectRoot)\}\//u, "").replace(/^(?:\.\/)+/u, "");
/** 🌐️ Whether a workspace-relative input glob selects every file of the repository. */
export const globSelectsRepositoryV1 = (pattern: string): boolean => REPOSITORY.test(bare(pattern));
/** 🧾️ The decision record of a rule. */
export const inputPrecisionDecisionV1 = (rule: InputPrecisionRuleV1): InputPrecisionDecisionV1 => ({version: 1, rule: rule.id, evidence: rule.evidence});
const fail = (message: string): never => {
  throw new InputPrecisionErrorV1(message);
};
const unique = <T>(values: readonly T[]): T[] => [...new Set(values)];
const uniqueJson = (values: readonly unknown[]): unknown[] => [...new Map(values.map((value) => [JSON.stringify(value), value])).values()];
const bytes = (left: string, right: string): number => (left < right ? -1 : left > right ? 1 : 0);
const sorted = (values: readonly string[]): string[] => [...values].sort(bytes);
const normalize = (path: string): string => {
  const parts: string[] = [];
  for (const part of path.split("/")) {
    if (part === "" || part === ".") continue;
    if (part === "..") parts.pop();
    else parts.push(part);
  }
  return parts.join("/");
};

/** ✅️ Admits a derivation document against its closed schema and refuses whole-repository patterns, unknown profiles and duplicate rules. */
export function readInputPrecisionV1(value: unknown): InputPrecisionDocumentV1 {
  const failures = validateJsonSchemaSubset(schema, value, schema);
  if (failures.length) fail(`Input precision document refused: ${failures.slice(0, 3).join("; ")}`);
  const document = value as InputPrecisionDocumentV1, ids = new Set<string>();
  const patterns = [...Object.values(document.profiles).flatMap((profile) => profile.reads.flatMap((read) => ("globs" in read ? read.globs : []))), ...document.rules.flatMap((rule) => rule.reads.flatMap((read) => ("globs" in read ? read.globs : [])))];
  for (const pattern of patterns) if (WHOLE.test(bare(pattern))) fail(`Input precision pattern selects the whole repository: ${pattern}`);
  for (const rule of document.rules) {
    if (ids.has(rule.id)) fail(`Input precision rule declared twice: ${rule.id}`);
    ids.add(rule.id);
    for (const read of rule.reads) if ("profile" in read && !Object.hasOwn(document.profiles, read.profile)) fail(`Input precision rule ${rule.id} reads unknown profile ${read.profile}`);
    for (const read of rule.reads) if ("profile" in read && document.profiles[read.profile]!.reads.some((inner) => "profile" in inner)) fail(`Input precision profile ${read.profile} nests another profile`);
  }
  return document;
}

/** 🗣️ Chooses the progress language by the shared rule of the CLI help; no language is assumed. */
export const inputPrecisionLanguageV1 = (document: Pick<InputPrecisionDocumentV1, "messages">, environment: Readonly<Record<string, string | undefined>>, osLanguages?: () => readonly string[]): string => resolveBuildHelpLanguageV1(document.messages.languages, [], environment, osLanguages);
/** 💬️ Renders one program analysis event in a language of the document. */
export function inputPrecisionTextV1(document: Pick<InputPrecisionDocumentV1, "messages">, event: ProgramEventV1, language: string): string {
  const key = event.phase === "derive" ? "deriving" : event.phase;
  return (document.messages.texts[key][language] ?? fail(`Input precision message language missing: ${language}`)).replace(/\{(tsconfig|files|derived|reused|seconds)\}/gu, (_, name: keyof ProgramEventV1) => String(event[name]));
}

/** 🔤️ Splits a script command into its words, honoring single and double quotes. */
export function commandTokensV1(command: string): string[] {
  const tokens: string[] = [], pattern = /"([^"]*)"|'([^']*)'|(\S+)/gu;
  for (let match = pattern.exec(command); match; match = pattern.exec(command)) tokens.push(match[1] ?? match[2] ?? match[3]!);
  return tokens;
}

/** 🧭️ The words a `bun <script> <words…>` command passes to its script; `undefined` for anything else. */
export function commandWordsV1(command: string): {readonly script: string; readonly words: readonly string[]} | undefined {
  const tokens = commandTokensV1(command);
  if (!isRuntimeExecutableV1(tokens[0] ?? "")) return undefined;
  const index = tokens.findIndex((token, position) => position > 0 && /script\.ts$/u.test(token));
  return index < 0 ? undefined : {script: tokens[index]!, words: tokens.slice(index + 1)};
}

/** 🎯️ Selects the first rule a target falls under; rules are ordered from the specific to the general. */
export function selectRuleV1(document: InputPrecisionDocumentV1, identity: InputPrecisionTargetV1): InputPrecisionRuleV1 | undefined {
  const parsed = identity.command === undefined ? undefined : commandWordsV1(identity.command), tokens = parsed ? [parsed.script, ...parsed.words] : [];
  return document.rules.find((rule) => {
    if (rule.select.project !== identity.project) return false;
    const byTarget = rule.select.targets?.includes(identity.target) ?? false, byCommand = rule.select.commands?.some((head) => head.length <= tokens.length && head.every((word, index) => tokens[index] === word)) ?? false;
    return rule.select.targets === undefined && rule.select.commands === undefined ? true : byTarget || byCommand;
  });
}

/** 🗂️ Turns taxonomy file kinds into patterns: every file of the kind's extensions, or only the canonical emoji-named one. */
export function kindPatternsV1(read: {readonly kinds: readonly string[]; readonly under: readonly string[]; readonly named?: boolean}, host: Pick<InputPrecisionHostV1, "kinds">): string[] {
  const patterns: string[] = [];
  for (const id of read.kinds) {
    const kind = host.kinds[id] ?? fail(`Unknown taxonomy file kind: ${id}`);
    for (const base of read.under) for (const extension of kind.extensionChains) patterns.push(`${WORKSPACE}${normalize(base) ? `${normalize(base)}/` : ""}**/${read.named ? kind.emoji : "*"}${extension}`);
  }
  return unique(patterns);
}

/** 🦀️ The `[package] name` of a Cargo manifest text. */
export function packageNameV1(manifest: string): string | undefined {
  let inside = false;
  for (const line of manifest.split(/\r?\n/u)) {
    const header = /^\s*\[([^\]]+)\]\s*$/u.exec(line);
    if (header) {
      inside = header[1]!.trim() === "package";
      continue;
    }
    const name = inside ? /^\s*name\s*=\s*"([^"]+)"/u.exec(line) : null;
    if (name) return name[1];
  }
  return undefined;
}

const indexes = new WeakMap<object, Map<string, string>>();
/** 🦀️ Maps every crate name of the declared Cargo workspaces to its directory; hidden trees (tickets, caches, drafts) never own a crate. */
export function crateIndexV1(host: Pick<InputPrecisionHostV1, "readText" | "cargoManifests">): ReadonlyMap<string, string> {
  const known = indexes.get(host);
  if (known) return known;
  const index = new Map<string, string>();
  for (const directory of host.cargoManifests()) {
    if (directory.split("/").some((part) => part.startsWith("."))) continue;
    const name = packageNameV1(host.readText(directory ? `${directory}/Cargo.toml` : "Cargo.toml") ?? "");
    if (name && !index.has(name)) index.set(name, directory);
  }
  indexes.set(host, index);
  return index;
}

/** 🦀️ The sources of the crates in the given directories and every path dependency they compile with. */
export function crateDirectoryPatternsV1(directories: readonly string[], host: Pick<InputPrecisionHostV1, "cargoRoots" | "cargoSources" | "cargoToolchain" | "cargoIdentity">): {patterns: string[]; files: string[]; markers: unknown[]} {
  const patterns: string[] = [], files: string[] = [], markers: unknown[] = [];
  for (const directory of directories) {
    const marker = host.cargoIdentity?.(directory);
    if (marker !== undefined) markers.push(marker);
    for (const root of host.cargoRoots(directory)) {
      const sources = host.cargoSources(root);
      patterns.push(`${WORKSPACE}${root}/Cargo.toml`);
      if (sources) files.push(...sources.map((source) => source.replace(WORKSPACE, "")));
      else patterns.push(`${WORKSPACE}${root}/**/*`, `!${WORKSPACE}${root}/**/target/**`);
    }
  }
  return {patterns: unique([...patterns, ...host.cargoToolchain.map((file) => `${WORKSPACE}${file}`)]), files: unique(files), markers: uniqueJson(markers)};
}

/** 🦀️ The sources of the named crates and every path dependency they compile with. */
export function cratePatternsV1(packages: readonly string[], host: Pick<InputPrecisionHostV1, "readText" | "cargoManifests" | "cargoRoots" | "cargoSources" | "cargoToolchain" | "cargoIdentity">): {patterns: string[]; files: string[]; markers: unknown[]} {
  const index = crateIndexV1(host);
  return crateDirectoryPatternsV1(packages.map((name) => index.get(name) ?? fail(`Unknown Cargo package: ${name}`)), host);
}

/** 🔖️ What one dispatcher branch names: Cargo packages, repository path literals and TypeScript entry files. */
export type BranchFactsV1 = {readonly key: string; readonly subject?: string; readonly crates: readonly string[]; readonly paths: readonly string[]; readonly cargo: boolean};

/** 🪓️ Splits a dispatcher source into its `if (segments[0] === "key")` branches. */
export function dispatchBranchesV1(source: string): BranchFactsV1[] {
  const lines = source.split("\n"), branches: BranchFactsV1[] = [];
  for (let index = 0; index < lines.length; index++) {
    const head = /^(\s*)if \(segments\[0\] === "([^"]+)"(?: && segments\[1\] === "([^"]+)")?/u.exec(lines[index]!);
    if (!head) continue;
    let end = index;
    if (lines[index]!.trimEnd().endsWith("{")) for (end = index + 1; end < lines.length && lines[end] !== `${head[1]}}`; end++);
    branches.push({key: head[2]!, ...(head[3] ? {subject: head[3]} : {}), ...literalFactsV1(lines.slice(index, end + 1).join("\n"))});
  }
  return branches;
}

/** 🔖️ Cargo packages (-p), repository path literals and whether a source text runs Cargo. */
export function literalFactsV1(text: string): {readonly crates: readonly string[]; readonly paths: readonly string[]; readonly cargo: boolean} {
  const literals = [...text.matchAll(/"((?:[^"\\\n]|\\.)*)"/gu)].map((match) => match[1]!);
  const joined = [...text.matchAll(/\bjoin\(([^)]*)\)/gu)].map((match) => [...match[1]!.matchAll(/"((?:[^"\\\n]|\\.)*)"/gu)].map((part) => part[1]!).join("/"));
  const templates = [...text.matchAll(/`((?:[^`\\]|\\.)*)`/gu)].flatMap((match) => match[1]!.split(/\$\{[^}]*\}/u));
  const crates = [...text.matchAll(/"(?:-p|--package)",\s*"([^"]+)"/gu)].map((match) => match[1]!);
  return {crates: unique(crates), paths: unique([...literals, ...joined, ...templates].filter((literal) => literal.includes("/") && !/\s|^https?:|^@|^\/\/|[*?]/u.test(literal))), cargo: /\brunCargo\b|"cargo"/u.test(text)};
}

/** 🔎️ The branches the dispatcher segments select: every branch of the first segment whose second segment, if it names one, matches. */
export function selectedBranchesV1(branches: readonly BranchFactsV1[], segments: readonly string[]): BranchFactsV1[] {
  return branches.filter((branch) => branch.key === segments[0] && (branch.subject === undefined || branch.subject === segments[1]));
}

/** 🔗️ Resolves branch path literals to existing repository paths: literals already rooted at the repository, and suffix literals joined to every directory literal of the branch. */
export function resolveLiteralsV1(paths: readonly string[], host: Pick<InputPrecisionHostV1, "exists">, base = ""): {resolved: string[]; unresolved: string[]} {
  const rooted = paths.filter((path) => !path.startsWith(".")).map((path) => normalize(path)), directories = rooted.filter((path) => host.exists(path) && !/\.[a-z]+$/u.test(path)), resolved = new Set<string>(), unresolved: string[] = [];
  for (const literal of paths) {
    const direct = literal.startsWith(".") ? normalize(`${base}/${literal}`) : normalize(literal), relative = normalize(`${base}/${literal}`);
    if (host.exists(direct) || (base && !literal.startsWith("/") && host.exists(relative))) {
      resolved.add(host.exists(direct) ? direct : relative);
      continue;
    }
    const joined = directories.map((directory) => normalize(`${directory}/${literal}`)).filter((path) => host.exists(path));
    if (joined.length) for (const path of joined) resolved.add(path);
    else if (/^\/|\.(?:ts|tsx|mts|json)$/u.test(literal)) unresolved.push(literal);
  }
  return {resolved: sorted([...resolved].filter(Boolean)), unresolved: sorted(unresolved)};
}

/** 📜️ Parses a JSON document that may carry comments and trailing commas, as `tsconfig.json` does. */
export function parseJsoncV1(text: string): any {
  let output = "", quote = false;
  for (let index = 0; index < text.length; index++) {
    const character = text[index]!;
    if (quote) {
      output += character;
      if (character === "\\") output += text[++index] ?? "";
      else if (character === '"') quote = false;
    } else if (character === '"') {
      quote = true;
      output += character;
    } else if (character === "/" && text[index + 1] === "/") while (index < text.length && text[index] !== "\n") index++;
    else if (character === "/" && text[index + 1] === "*") {
      index += 2;
      while (index < text.length && !(text[index] === "*" && text[index + 1] === "/")) index++;
      index++;
    } else output += character;
  }
  return JSON.parse(output.replace(/,(\s*[}\]])/gu, "$1"));
}

type ProgramConfigV1 = {readonly files: readonly string[]; readonly include: readonly string[] | undefined; readonly exclude: readonly string[] | undefined; readonly chain: readonly string[]; readonly directory: string};
const configuration = (path: string, host: Pick<InputPrecisionHostV1, "readText">, seen: readonly string[] = []): ProgramConfigV1 => {
  if (seen.includes(path)) fail(`TypeScript configuration cycle: ${path}`);
  const text = host.readText(path) ?? fail(`TypeScript configuration missing: ${path}`), json = parseJsoncV1(text), directory = normalize(path.split("/").slice(0, -1).join("/"));
  const parents = typeof json.extends === "string" ? [json.extends] : Array.isArray(json.extends) ? json.extends : [];
  let inherited: ProgramConfigV1 = {files: [], include: undefined, exclude: undefined, chain: [], directory};
  for (const parent of parents) {
    if (typeof parent !== "string" || !parent.startsWith(".")) continue;
    const resolved = normalize(`${directory}/${parent}`), file = resolved.endsWith(".json") ? resolved : `${resolved}.json`, base = configuration(file, host, [...seen, path]);
    const rebase = (list: readonly string[] | undefined) => list?.map((pattern) => normalize(`${base.directory}/${pattern}`));
    inherited = {files: unique([...inherited.files, ...(rebase(base.files) ?? [])]), include: rebase(base.include) ?? inherited.include, exclude: rebase(base.exclude) ?? inherited.exclude, chain: [...inherited.chain, ...base.chain, file], directory};
  }
  const own = (list: unknown) => (Array.isArray(list) ? list.filter((item): item is string => typeof item === "string").map((pattern) => normalize(`${directory}/${pattern}`)) : undefined);
  return {files: unique([...inherited.files, ...(own(json.files) ?? [])]), include: own(json.include) ?? inherited.include, exclude: own(json.exclude) ?? inherited.exclude, chain: inherited.chain, directory};
};

const SOURCE = /\.(?:ts|tsx|mts|cts)$/u;
const SKIPPED = new Set(["node_modules", ".git"]);
const GLOB_CHARACTERS = /[*?{[]/u;
const SEGMENT_EXCLUDE = /^(?:(.*?)\/)?\*\*\/([^*?{}[\]/]+)(\/\*\*)?$/u;
/** 🚫️ Compiles exclude globs into one predicate with the answers of matching each glob and its `/**` extension: literal paths and `prefix/**\/name[/**]` patterns by path segment, everything else by expression. */
export function excludesV1(patterns: readonly string[]): (path: string) => boolean {
  const literals: string[] = [], segments: {prefix: string; name: string; beyond: boolean}[] = [], expressions: RegExp[] = [];
  for (const pattern of patterns) {
    const shape = SEGMENT_EXCLUDE.exec(pattern);
    if (!GLOB_CHARACTERS.test(pattern)) literals.push(pattern);
    else if (shape && !GLOB_CHARACTERS.test(shape[1] ?? "")) segments.push({prefix: shape[1] ? `${shape[1]}/` : "", name: shape[2]!, beyond: shape[3] !== undefined});
    else expressions.push(inputGlobRegExpV1(pattern), inputGlobRegExpV1(`${pattern}/**`));
  }
  return (path) => {
    for (const literal of literals) if (path === literal || path.startsWith(`${literal}/`)) return true;
    for (const rule of segments) {
      if (!path.startsWith(rule.prefix)) continue;
      const parts = path.slice(rule.prefix.length).split("/");
      for (let index = 0; index < parts.length - (rule.beyond ? 1 : 0); index++) if (parts[index] === rule.name) return true;
    }
    return expressions.some((expression) => expression.test(path));
  };
}

/** 🧾️ One answer of the file system or the compiler that a module resolution depended on: a probe is `file` or `directory` existence, the `text` digest of a package manifest, a `realpath`, a `directories` listing or the `digest` of a workspace file. */
export type ProgramFactV1 = {readonly kind: "file" | "directory" | "text" | "realpath" | "directories" | "digest"; readonly path: string; readonly value: boolean | string | null};
/** 📇️ The persisted import analysis of one program: per source file its content digest, what its imports resolved to and the indexes of the facts that resolution consulted. */
export type ProgramReceiptV1 = {readonly version: 1; readonly tsconfig: string; readonly compiler: string; readonly chain: readonly {readonly path: string; readonly sha256: string | null}[]; readonly facts: readonly ProgramFactV1[]; readonly files: Readonly<Record<string, {readonly sha256: string; readonly resolved: readonly string[]; readonly facts: readonly number[]}>>};
/** 📣️ What a program analysis reports while it derives: it starts, advances every few hundred files and finishes with the counts of reused and derived files. */
export type ProgramEventV1 = {readonly phase: "derive" | "progress" | "done"; readonly tsconfig: string; readonly files: number; readonly derived: number; readonly reused: number; readonly seconds: number};
/** 🔷️ The compiler side of a program analysis; the engine only asks questions and compares answers. */
export type ProgramHostV1 = {
  readonly compiler: string;
  readonly imports: (file: string, tsconfig: string) => Promise<{readonly resolved: readonly string[]; readonly facts: readonly ProgramFactV1[]}>;
  readonly fact: (kind: ProgramFactV1["kind"], path: string) => boolean | string | null;
  readonly receipts?: {readonly read: (key: string) => string | undefined; readonly write: (key: string, image: string) => void};
  readonly check?: (phase: string, index: number) => void | Promise<void>;
  readonly report?: (event: ProgramEventV1) => void;
};

const PROGRAM_PHASE = "build:input-precision";
const PROGRAM_REPORT_MS = 5000;
const programs = new WeakMap<object, Map<string, Promise<{patterns: string[]; files: string[]; roots: string[]}>>>();
const sameFact = (left: ProgramFactV1["value"], right: ProgramFactV1["value"]): boolean => left === right;
const readReceipt = (host: ProgramHostV1, key: string, tsconfig: string, chain: ProgramReceiptV1["chain"]): ProgramReceiptV1 | undefined => {
  const image = host.receipts?.read(key);
  if (image === undefined) return undefined;
  let value: unknown;
  try {
    value = JSON.parse(image);
  } catch {
    return undefined;
  }
  if (validateJsonSchemaSubset({$ref: "#/definitions/ProgramReceipt"}, value, schema).length) return undefined;
  const receipt = value as ProgramReceiptV1;
  return receipt.tsconfig === tsconfig && receipt.compiler === host.compiler && receipt.chain.length === chain.length && receipt.chain.every((row, index) => row.path === chain[index]!.path && sameFact(row.sha256, chain[index]!.sha256)) ? receipt : undefined;
};

/** 🔷️ The TypeScript program of a tsconfig: the include globs, the import closure of the files they select, the configuration chain and the package manifests that resolution reads. The import analysis is kept in a receipt keyed by tsconfig and compiler, and a file's entry is reused while its content digest and every fact its resolution consulted still hold, so a warm run derives nothing and an edit re-derives only the files it touches. Progress is reported and cancellation honoured per file; a cancelled or failed run writes no receipt. */
export function programInputsV1(tsconfig: string, host: Pick<InputPrecisionHostV1, "readText" | "exists" | "listDirectory" | "program">): Promise<{patterns: string[]; files: string[]; roots: string[]}> {
  const key = normalize(tsconfig);
  let byConfig = programs.get(host);
  if (!byConfig) programs.set(host, byConfig = new Map());
  let known = byConfig.get(key);
  if (!known) {
    known = deriveProgramV1(key, host);
    byConfig.set(key, known);
    known.catch(() => byConfig!.delete(key));
  }
  return known;
}

async function deriveProgramV1(tsconfig: string, host: Pick<InputPrecisionHostV1, "readText" | "exists" | "listDirectory" | "program">): Promise<{patterns: string[]; files: string[]; roots: string[]}> {
  const program = host.program ?? fail(`TypeScript program analysis missing: ${tsconfig}`);
  const config = configuration(tsconfig, host), include = config.include ?? [`${config.directory ? `${config.directory}/` : ""}**/*`], exclude = config.exclude ?? [];
  const isExcluded = excludesV1(exclude), roots = new Set<string>(config.files.filter((file) => host.exists(file)));
  for (const pattern of include) {
    const stop = pattern.search(/[*?{[]/u), base = stop < 0 ? pattern : pattern.slice(0, pattern.lastIndexOf("/", stop)), pending = [normalize(base)];
    if (stop < 0 && SOURCE.test(pattern) && host.exists(pattern)) roots.add(pattern);
    while (stop >= 0 && pending.length) {
      const directory = pending.pop()!;
      for (const entry of host.listDirectory(directory)) {
        const path = directory ? `${directory}/${entry.name}` : entry.name;
        if (SKIPPED.has(entry.name) || isExcluded(path)) continue;
        if (entry.directory) pending.push(path);
        else if (SOURCE.test(path) && globMatchesV1(pattern, path)) roots.add(path);
      }
    }
  }
  const chain = unique([...config.chain, tsconfig]).map((path) => ({path, sha256: program.fact("digest", path) as string | null})), receiptKey = JSON.stringify(["input-precision-program", 1, tsconfig, program.compiler]);
  const prior = readReceipt(program, receiptKey, tsconfig, chain), known = new Map<string, ProgramFactV1["value"]>();
  const answer = (kind: ProgramFactV1["kind"], path: string): ProgramFactV1["value"] => {
    const id = `${kind}\0${path}`;
    if (!known.has(id)) known.set(id, program.fact(kind, path));
    return known.get(id)!;
  };
  const table: ProgramFactV1[] = [], index = new Map<string, number>(), files: Record<string, {sha256: string; resolved: string[]; facts: number[]}> = {}, seen = new Set<string>(), pending = sorted([...roots]);
  const started = performance.now(), event = (phase: ProgramEventV1["phase"]): ProgramEventV1 => ({phase, tsconfig, files: seen.size, derived, reused, seconds: Math.round((performance.now() - started) / 100) / 10});
  let derived = 0, reused = 0, announced = false, reported = started, visited = 0;
  const intern = (fact: ProgramFactV1): number => {
    const id = `${fact.kind}\0${fact.path}\0${JSON.stringify(fact.value)}`;
    let at = index.get(id);
    if (at === undefined) {
      at = table.length;
      table.push({kind: fact.kind, path: fact.path, value: fact.value});
      index.set(id, at);
    }
    return at;
  };
  const holds = (file: string, entry: NonNullable<ProgramReceiptV1["files"][string]>): boolean => answer("digest", file) === entry.sha256 && entry.facts.every((at) => {
    const fact = prior!.facts[at];
    return fact !== undefined && sameFact(answer(fact.kind, fact.path), fact.value);
  });
  const write = (): void => program.receipts?.write(receiptKey, JSON.stringify({version: 1, tsconfig, compiler: program.compiler, chain, facts: table, files} satisfies ProgramReceiptV1));
  try {
    while (pending.length) {
      const file = pending.pop()!;
      if (seen.has(file)) continue;
      await program.check?.(PROGRAM_PHASE, ++visited);
      if (!SOURCE.test(file)) {
        seen.add(file);
        continue;
      }
      const entry = prior?.files[file];
      let resolved: readonly string[];
      if (entry && holds(file, entry)) {
        reused++;
        resolved = entry.resolved;
        files[file] = {sha256: entry.sha256, resolved: [...entry.resolved], facts: entry.facts.map((at) => intern(prior!.facts[at]!))};
      } else {
        let analysed: {sha256: string; resolved: readonly string[]; facts: readonly ProgramFactV1[]} | undefined;
        for (let attempt = 0; attempt < 3 && !analysed; attempt++) {
          const before = program.fact("digest", file) as string | null;
          if (before === null) break;
          if (!announced) {
            announced = true;
            program.report?.(event("derive"));
          }
          const result = await program.imports(file, tsconfig);
          if (program.fact("digest", file) === before) analysed = {sha256: before, ...result};
        }
        if (!analysed) {
          if (program.fact("digest", file) === null) continue;
          throw new InputPrecisionErrorV1(`TypeScript program file changed while it was analysed: ${file}`);
        }
        derived++;
        resolved = analysed.resolved;
        files[file] = {sha256: analysed.sha256, resolved: [...analysed.resolved], facts: analysed.facts.map(intern)};
      }
      seen.add(file);
      for (const path of resolved) if (!seen.has(path)) pending.push(path);
      if (announced && performance.now() - reported >= PROGRAM_REPORT_MS) {
        reported = performance.now();
        program.report?.(event("progress"));
      }
    }
  } catch (error) {
    if (derived > 0 && prior) for (const [file, entry] of Object.entries(prior.files)) if (!files[file]) files[file] = {sha256: entry.sha256, resolved: [...entry.resolved], facts: entry.facts.map((at) => intern(prior.facts[at]!))};
    if (derived > 0) write();
    throw error;
  }
  if (derived > 0 || !prior || Object.keys(prior.files).length !== Object.keys(files).length) write();
  if (announced) program.report?.(event("done"));
  const closure = [...seen], covered = (path: string) => include.some((pattern) => globMatchesV1(pattern, path)) && !isExcluded(path);
  const manifests = new Set<string>();
  for (const path of closure) {
    const parts = path.split("/");
    for (let depth = parts.length - 1; depth >= 0; depth--) {
      const manifest = [...parts.slice(0, depth), "package.json"].join("/");
      if (host.exists(manifest)) {
        manifests.add(manifest);
        break;
      }
    }
  }
  const patterns = [...include.map((pattern) => `${WORKSPACE}${pattern}`), ...exclude.map((pattern) => `!${WORKSPACE}${pattern}`)];
  return {patterns, files: sorted(unique([...closure.filter((path) => !covered(path)), ...config.chain, tsconfig, ...manifests])), roots: sorted([...roots])};
}

/** 🧮️ Derives the narrowest declared inputs of one target from its rule. */
export async function deriveInputsV1(document: InputPrecisionDocumentV1, rule: InputPrecisionRuleV1, identity: InputPrecisionTargetV1, host: InputPrecisionHostV1): Promise<InputPrecisionResultV1> {
  const patterns: string[] = [], groups: string[] = [], files: string[] = [], markers: unknown[] = [], unresolved: string[] = [], reads: {read: string; patterns: number; files: number}[] = [];
  const parsed = identity.command === undefined ? undefined : commandWordsV1(identity.command), words = parsed ? [parsed.script, ...parsed.words] : [];
  let cargoUnresolved = false, cargoDeclared = false;
  const apply = async (read: InputPrecisionReadV1, label: string): Promise<void> => {
    const before: [number, number] = [patterns.length, files.length];
    if ("profile" in read) {
      for (const inner of document.profiles[read.profile]!.reads) await apply(inner, `${label}${read.profile}/`);
      return;
    }
    let name: string;
    if ("input" in read) {
      groups.push(read.input);
      name = `input:${read.input}`;
    } else if ("globs" in read) {
      patterns.push(...read.globs);
      name = "globs";
    } else if ("kinds" in read) {
      patterns.push(...kindPatternsV1(read, host));
      name = `kinds:${read.kinds.join("+")}`;
    } else if ("cargo" in read) {
      const packages = read.cargo.packages ?? [], directories = read.cargo.directories ?? [];
      if (!packages.length && !directories.length) fail(`Cargo read names no package or directory: ${identity.project}:${identity.target}`);
      for (const crate of [packages.length ? cratePatternsV1(packages, host) : {patterns: [], files: [], markers: []}, directories.length ? crateDirectoryPatternsV1(directories, host) : {patterns: [], files: [], markers: []}]) {
        patterns.push(...crate.patterns);
        files.push(...crate.files);
        markers.push(...crate.markers);
      }
      cargoDeclared = true;
      name = `cargo:${[...packages, ...directories].join("+")}`;
    } else if ("script" in read) {
      const source = normalize(read.script.source), text = host.readText(source) ?? fail(`Script source missing: ${read.script.source}`), facts = literalFactsV1(text);
      const resolved = resolveLiteralsV1(facts.paths, host, source.split("/").slice(0, -1).join("/")), entries = [source, ...resolved.resolved.filter((path) => SOURCE.test(path))];
      const crates = facts.crates.length ? cratePatternsV1(facts.crates, host) : {patterns: [], files: [], markers: []}, crateDirs = resolved.resolved.filter((path) => host.exists(`${path}/Cargo.toml`)), directories = crateDirs.length ? crateDirectoryPatternsV1(crateDirs, host) : {patterns: [], files: [], markers: []};
      patterns.push(...crates.patterns, ...directories.patterns);
      markers.push(...crates.markers, ...directories.markers);
      files.push(...crates.files, ...directories.files, ...resolved.resolved.filter((path) => !SOURCE.test(path) && /\.[a-z]+$/u.test(path)), ...(entries.length ? await host.importClosure(entries) : []));
      unresolved.push(...resolved.unresolved);
      name = `script:${read.script.source}`;
    } else if ("dispatch" in read) {
      const source = normalize(read.dispatch.source), text = host.readText(source) ?? fail(`Dispatch source missing: ${read.dispatch.source}`), branches = selectedBranchesV1(dispatchBranchesV1(text), words.slice(1 + read.dispatch.after.length));
      if (!read.dispatch.after.every((word, index) => words[index + 1] === word)) fail(`Dispatch routing words differ for ${words.join(" ")}: ${read.dispatch.after.join(" ")}`);
      if (!branches.length) fail(`Dispatch source has no branch for ${words.join(" ")}: ${read.dispatch.source}`);
      const resolved = resolveLiteralsV1(branches.flatMap((branch) => branch.paths), host, source.split("/").slice(0, -1).join("/")), crateDirectories = resolved.resolved.filter((path) => host.exists(`${path}/Cargo.toml`));
      const crates = unique(branches.flatMap((branch) => branch.crates)), named = crates.length ? cratePatternsV1(crates, host) : {patterns: [], files: [], markers: []}, directories = crateDirectories.length ? crateDirectoryPatternsV1(crateDirectories, host) : {patterns: [], files: [], markers: []};
      patterns.push(...named.patterns, ...directories.patterns);
      markers.push(...named.markers, ...directories.markers);
      files.push(...named.files, ...directories.files);
      if (!crates.length && !crateDirectories.length && branches.some((branch) => branch.cargo)) cargoUnresolved = true;
      const entries = resolved.resolved.filter((path) => SOURCE.test(path) && host.exists(path));
      files.push(...resolved.resolved.filter((path) => !SOURCE.test(path) && /\.[a-z]+$/u.test(path)), ...(entries.length ? await host.importClosure(entries) : []));
      unresolved.push(...resolved.unresolved);
      name = "dispatch";
    } else if ("program" in read) {
      const program = await programInputsV1(read.program.tsconfig, host);
      patterns.push(...program.patterns);
      files.push(...program.files);
      name = `program:${read.program.tsconfig}`;
    } else {
      const route = document.routes.filter((row) => row.words.length <= words.length - 1 && row.words.every((word, index) => words[index + 1] === word)).sort((left, right) => right.words.length - left.words.length)[0] ?? fail(`No owner route for ${identity.project}:${identity.target}`);
      patterns.push(`${WORKSPACE}${route.root}/**/*`);
      if (host.exists(`${route.root}/Cargo.toml`)) {
        const crate = crateDirectoryPatternsV1([route.root], host);
        patterns.push(...crate.patterns);
        files.push(...crate.files);
        markers.push(...crate.markers);
      }
      const script = `${route.root}/📜️script.ts`;
      if (host.exists(script)) files.push(...(await host.importClosure([script])));
      name = `owner:${route.project}`;
    }
    reads.push({read: label + name, patterns: patterns.length - before[0], files: files.length - before[1]});
  };
  for (const read of rule.reads) await apply(read, "");
  if (cargoUnresolved && !cargoDeclared) fail(`Dispatch branch runs cargo without a literal package and the rule declares none: ${identity.project}:${identity.target}`);
  for (const pattern of patterns) if (!pattern.startsWith("!") && globSelectsRepositoryV1(pattern)) fail(`Input precision derived a pattern that selects the whole repository: ${pattern} (${identity.project}:${identity.target})`);
  const positive = patterns.some((pattern) => !pattern.startsWith("!"));
  return {rule: rule.id, drops: [...rule.replaces], patterns: unique([...patterns, ...(positive ? document.exclusions : [])]), groups: unique([...(rule.replaces.length ? document.retained : []), ...groups]), files: sorted(unique(files)), unresolved: unique(unresolved), ...(markers.length ? {markers: uniqueJson(markers)} : {}), reads};
}
