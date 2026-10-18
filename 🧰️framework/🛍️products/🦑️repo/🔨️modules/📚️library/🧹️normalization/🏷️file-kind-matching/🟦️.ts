import { TaxonomyCancellationError } from "../🏃️operation/🟦️.ts";
import { type LoadedTaxonomy } from "../🔣️taxonomy/📚️loaded/🟦️.ts";
import { type FileKindSpec } from "../🔣️taxonomy/📐️kind-specs/🟦️.ts";
import { normalizeRelative, splitLeadingEmoji, sourceRelative, SEGMENTER, isEmojiGrapheme, emojiFold } from "../🛣️path/🟦️.ts";
import { assertLexicalInputOutsideOpaque } from "../📁️input/🟦️.ts";
import { existsSync } from "node:fs";
import { semanticMemberPatternEmojiAdmitted } from "../../🔍️discovery/📂️semantic-directory-kinds/🟦️.ts";
import { basename, isAbsolute, relative, resolve, sep } from "node:path";
import { type TaxonomySeverity, type TaxonomyViolation, requireTaxonomyValue } from "../📜️plan-contracts/🟦️.ts";

export function absolutePath(repoRoot: string, path: string): string {
  const root = resolve(repoRoot);
  const result = resolve(root, ...sourceRelative(path).split("/").filter(Boolean));
  const rel = relative(root, result);
  if (rel === ".." || rel.startsWith(`..${sep}`) || rel.startsWith("..") || rel.startsWith("..\\") || isAbsolute(rel)) throw new Error(`Path escapes repository root: ${path}`);
  return result;
}

export function isExcluded(path: string, taxonomy: Pick<LoadedTaxonomy, "exclusions">): boolean {
  const normalized = normalizeRelative(path);
  return taxonomy.exclusions.some((entry) => normalized === entry.path || normalized.startsWith(`${entry.path}/`));
}

export function isProperScopeAncestor(path: string, scope?: string): boolean {
  if (!scope) return false;
  const normalizedScope = normalizeRelative(scope);
  const normalizedPath = normalizeRelative(path);
  return normalizedPath !== normalizedScope && normalizedScope.startsWith(`${normalizedPath}/`);
}

export function splitLeadingEmojiIdentity(value: string): { sequence: string; first: string; rest: string } {
  let sequence = "", first = "";
  for (const { segment } of SEGMENTER.segment(value.normalize("NFC"))) {
    if (!isEmojiGrapheme(segment)) break;
    if (!first) first = segment;
    sequence += segment;
  }
  return { sequence, first, rest: value.slice(sequence.length) };
}

/** 🗂️ Directory kinds by folded emoji and by locale-folded id, semantic member kinds by folded member name, and the member kinds carrying a slug pattern, each in declaration order, built once per loaded taxonomy. */
interface DirectoryKindIndex {
  readonly kindsByEmoji: ReadonlyMap<string, readonly LoadedTaxonomy["directoryKinds"][number][]>;
  readonly memberKindsByName: ReadonlyMap<string, readonly (readonly [string, LoadedTaxonomy["schema"]["semanticDirectoryMemberKinds"][string]])[]>;
  readonly kindsByFoldedId: ReadonlyMap<string, readonly LoadedTaxonomy["directoryKinds"][number][]>;
  readonly patternMemberKinds: readonly (readonly [string, LoadedTaxonomy["schema"]["semanticDirectoryMemberKinds"][string], RegExp])[];
}

const directoryKindIndexes = new WeakMap<LoadedTaxonomy, DirectoryKindIndex>();

/** 🧮️ Indexes {@link matchDirectoryKind}'s emoji and member-name comparisons so each directory costs O(1) lookups instead of a scan of every kind and member name. */
export function directoryKindIndex(taxonomy: LoadedTaxonomy): DirectoryKindIndex {
  const cached = directoryKindIndexes.get(taxonomy);
  if (cached) return cached;
  const kindsByEmoji = new Map<string, LoadedTaxonomy["directoryKinds"][number][]>(), kindsByFoldedId = new Map<string, LoadedTaxonomy["directoryKinds"][number][]>(), memberKindsByName = new Map<string, (readonly [string, LoadedTaxonomy["schema"]["semanticDirectoryMemberKinds"][string]])[]>();
  for (const kind of taxonomy.directoryKinds) {
    const key = emojiFold(kind.emoji), rows = kindsByEmoji.get(key);
    if (rows) rows.push(kind);
    else kindsByEmoji.set(key, [kind]);
    const id = kind.id.normalize("NFC").toLocaleLowerCase("und"), named = kindsByFoldedId.get(id);
    if (named) named.push(kind);
    else kindsByFoldedId.set(id, [kind]);
  }
  const patternMemberKinds: (readonly [string, LoadedTaxonomy["schema"]["semanticDirectoryMemberKinds"][string], RegExp])[] = [];
  for (const entry of Object.entries(taxonomy.schema.semanticDirectoryMemberKinds)) {
    for (const key of new Set(entry[1].memberNames.map(emojiFold))) {
      const rows = memberKindsByName.get(key);
      if (rows) rows.push(entry);
      else memberKindsByName.set(key, [entry]);
    }
    if (entry[1].memberSlugPattern !== undefined) patternMemberKinds.push([entry[0], entry[1], new RegExp(entry[1].memberSlugPattern, "u")]);
  }
  const index = { kindsByEmoji, memberKindsByName, kindsByFoldedId, patternMemberKinds };
  directoryKindIndexes.set(taxonomy, index);
  return index;
}

/** 🧷️ Matches one directory name to its structural kind, then its exact member registry, then its member slug pattern, each below the nearest owner scope; kinds matching at the same nearest distance are reported as ambiguous. */
export function matchDirectoryKind(name: string, taxonomy: LoadedTaxonomy, parentKindId?: string, ancestorKindIds: readonly string[] = []): { kind: { readonly id: string; readonly emoji: string } | null; slug: string; ambiguous: readonly string[] } {
  const normalized = name.normalize("NFC");
  const identity = splitLeadingEmojiIdentity(normalized);
  const leading = { emoji: identity.first, rest: identity.rest };
  const contextAllows = (kind: LoadedTaxonomy["directoryKinds"][number]): boolean => (kind.parentKindIds?.length ?? 0) === 0 || (parentKindId !== undefined && kind.parentKindIds?.includes(parentKindId) === true);
  const index = directoryKindIndex(taxonomy);
  if (leading.emoji) {
    const global = (index.kindsByEmoji.get(emojiFold(leading.emoji)) ?? []).filter((kind) => (leading.rest.length === 0 && kind.allowEmojiOnly) || kind.slugRegex.test(leading.rest));
    const named = index.kindsByFoldedId.get(leading.rest.toLocaleLowerCase("und")) ?? [];
    const exact = global.filter((kind) => contextAllows(kind) && named.includes(kind));
    if (exact.length === 1) return { kind: requireTaxonomyValue(exact[0],"single directory match"), slug: leading.rest, ambiguous: [] };
    if (exact.length > 1) return { kind: null, slug: leading.rest, ambiguous: exact.map((entry) => entry.id) };
    const contextual = parentKindId === undefined ? [] : global.filter((kind) => kind.parentKindIds?.includes(parentKindId) === true);
    const ordinary = contextual.length > 0 ? contextual : global.filter((kind) => (kind.parentKindIds?.length ?? 0) === 0);
    if (ordinary.length === 1) return { kind: requireTaxonomyValue(ordinary[0],"single directory match"), slug: leading.rest, ambiguous: [] };
    const contexts = [parentKindId, ...ancestorKindIds].filter((kindId, index, rows): kindId is string => Boolean(kindId) && rows.indexOf(kindId) === index);
    const overlays = (index.memberKindsByName.get(emojiFold(`${identity.first}${identity.rest}`)) ?? [])
      .map(([id, spec]) => ({ id, distance: contexts.findIndex((kindId) => spec.ownerKindIds.includes(kindId)) }))
      .filter((entry) => entry.distance >= 0)
      .sort((left, right) => left.distance - right.distance || left.id.localeCompare(right.id));
    if (overlays.length > 0) {
      const nearest = overlays.filter((entry) => entry.distance === requireTaxonomyValue(overlays[0],"overlays[0]").distance);
      if (nearest.length === 1) return { kind: { id: requireTaxonomyValue(nearest[0],"nearest[0]").id, emoji: leading.emoji }, slug: leading.rest, ambiguous: [] };
      return { kind: null, slug: leading.rest, ambiguous: nearest.map((entry) => entry.id) };
    }
    if (index.patternMemberKinds.length > 0 && semanticMemberPatternEmojiAdmitted(leading.emoji)) {
      const patterned = index.patternMemberKinds
        .filter(([, , regex]) => regex.test(leading.rest))
        .map(([id, spec]) => ({ id, distance: contexts.findIndex((kindId) => spec.ownerKindIds.includes(kindId)) }))
        .filter((entry) => entry.distance >= 0)
        .sort((left, right) => left.distance - right.distance || left.id.localeCompare(right.id));
      if (patterned.length > 0) {
        const nearest = patterned.filter((entry) => entry.distance === requireTaxonomyValue(patterned[0],"patterned[0]").distance);
        if (nearest.length === 1) return { kind: { id: requireTaxonomyValue(nearest[0],"nearest[0]").id, emoji: leading.emoji }, slug: leading.rest, ambiguous: [] };
        return { kind: null, slug: leading.rest, ambiguous: nearest.map((entry) => entry.id) };
      }
    }
    return { kind: null, slug: leading.rest, ambiguous: ordinary.map((entry) => entry.id) };
  }
  const exact = (index.kindsByFoldedId.get(normalized.toLocaleLowerCase("und")) ?? []).filter((kind) => contextAllows(kind) && kind.inferWithoutEmoji !== false);
  if (exact.length === 1) return { kind: requireTaxonomyValue(exact[0],"single directory match"), slug: normalized, ambiguous: [] };
  if (exact.length > 1) return { kind: null, slug: normalized, ambiguous: exact.map((entry) => entry.id) };
  const matching = taxonomy.directoryKinds.filter((kind) => kind.inferWithoutEmoji !== false && kind.slugRegex.test(normalized));
  const contextual = parentKindId === undefined ? [] : matching.filter((kind) => kind.parentKindIds?.includes(parentKindId) === true);
  const matches = contextual.length > 0 ? contextual : matching.filter((kind) => (kind.parentKindIds?.length ?? 0) === 0);
  return { kind: matches.length === 1 ? matches[0] ?? null : null, slug: normalized, ambiguous: matches.map((entry) => entry.id) };
}

export function resolveFileKind(
  path: string,
  taxonomy: LoadedTaxonomy,
  parentKindId: string | undefined,
  ancestorKindIds: readonly string[],
  forcedId?: string,
  contentKindId?: string,
): { kind: (FileKindSpec & { readonly id: string }) | null; extension: string; stem: string; ambiguous: readonly string[] } {
  const name = basename(path);
  const normalized = name.normalize("NFC");
  const folded = normalized.toLocaleLowerCase("und");
  const scoped = Object.entries(taxonomy.schema.scopedFileKinds)
    .flatMap(([id, spec]) => {
      if (!taxonomy.pathMatcher.matches(path, spec.pathPattern) || spec.parentDirectoryKindId !== undefined && spec.parentDirectoryKindId !== parentKindId || !new RegExp(spec.sourceFilenamePattern, "u").test(normalized)) return [];
      const extensions = spec.extensionChains.filter((chain) => folded.endsWith(chain.toLocaleLowerCase("und"))).sort((left, right) => right.length - left.length || left.localeCompare(right));
      return extensions.length > 0 ? [{ id, spec, extension: requireTaxonomyValue(extensions[0],"scoped extension") }] : [];
    })
    .sort((left, right) => left.id.localeCompare(right.id));
  if (scoped.length > 1) return { kind: null, extension: "", stem: normalized, ambiguous: scoped.map(({ id }) => `scoped:${id}`) };
  if (scoped.length === 1) {
    const selected = requireTaxonomyValue(scoped[0],"scoped[0]");
    const kind = { id: `scoped:${selected.id}`, emoji: selected.spec.emoji, extensionChains: selected.spec.extensionChains, role: selected.spec.role };
    const withoutExtension = normalized.slice(0, -selected.extension.length);
    const leading = splitLeadingEmoji(withoutExtension);
    return { kind, extension: selected.extension, stem: leading.emoji && emojiFold(leading.emoji) === emojiFold(kind.emoji) ? leading.rest : withoutExtension, ambiguous: [] };
  }
  const forced = forcedId ? taxonomy.fileKinds.find((kind) => kind.id === forcedId) : undefined;
  if (forced) {
    const extensions = forced.extensionChains.filter((chain) => normalized.endsWith(chain)).sort((left, right) => right.length - left.length || left.localeCompare(right));
    if (extensions.length > 0) {
      const extension = requireTaxonomyValue(extensions[0],"extensions[0]");
      const withoutExtension = normalized.slice(0, -extension.length);
      const leading = splitLeadingEmoji(withoutExtension);
      return { kind: forced, extension, stem: leading.emoji && emojiFold(leading.emoji) === emojiFold(forced.emoji) ? leading.rest : withoutExtension, ambiguous: [] };
    }
  }
  const extensionRows = Object.entries(taxonomy.schema.fileKindResolutionRules)
    .filter(([, rule]) => normalized.endsWith(rule.extensionChain))
    .sort((left, right) => right[1].extensionChain.length - left[1].extensionChain.length || left[0].localeCompare(right[0]));
  const longest = extensionRows[0]?.[1].extensionChain.length ?? 0;
  const candidates = extensionRows
    .filter(([, rule]) => rule.extensionChain.length === longest)
    .filter(([, rule]) => !rule.filenamePattern || new RegExp(rule.filenamePattern, "u").test(normalized))
    .filter(([, rule]) => !rule.pathPattern || taxonomy.pathMatcher.matches(path, rule.pathPattern))
    .filter(([, rule]) => !rule.parentKindIds || (parentKindId !== undefined && rule.parentKindIds.includes(parentKindId)))
    .filter(([, rule]) => !rule.ancestorKindIds || rule.ancestorKindIds.some((kindId) => ancestorKindIds.includes(kindId)))
    .map(([id, rule]) => ({ id, rule, predicates: Number(Boolean(rule.filenamePattern)) + Number(Boolean(rule.pathPattern)) + Number(Boolean(rule.parentKindIds)) + Number(Boolean(rule.ancestorKindIds)) }))
    .sort((left, right) => right.rule.priority - left.rule.priority || right.predicates - left.predicates || left.id.localeCompare(right.id));
  if (candidates.length === 0) {
    const contentKind = contentKindId ? taxonomy.fileKinds.find((kind) => kind.id === contentKindId) : undefined;
    if (!contentKind) return { kind: null, extension: "", stem: normalized, ambiguous: [] };
    const extension = requireTaxonomyValue([...contentKind.extensionChains].sort((left, right) => left.length - right.length || left.localeCompare(right))[0],"[...contentKind.extensionChains].sort((left, right) => left.length - right.length || left.localeCompare(right))[0]");
    const leading = splitLeadingEmoji(normalized);
    const stem = (leading.emoji && emojiFold(leading.emoji) === emojiFold(contentKind.emoji) ? leading.rest : normalized).trim().replace(/[. ]+$/u, "");
    return { kind: contentKind, extension, stem, ambiguous: [] };
  }
  const top = candidates.filter((entry) => entry.rule.priority === requireTaxonomyValue(candidates[0],"candidates[0]").rule.priority && entry.predicates === requireTaxonomyValue(candidates[0],"candidates[0]").predicates);
  const kindIds = [...new Set(top.map((entry) => entry.rule.fileKindId))];
  const extension = requireTaxonomyValue(top[0],"top[0]").rule.extensionChain;
  const withoutExtension = normalized.slice(0, normalized.length - extension.length);
  if (kindIds.length !== 1) return { kind: null, extension, stem: withoutExtension, ambiguous: top.map((entry) => `${entry.id}:${entry.rule.fileKindId}`) };
  const selected = taxonomy.fileKinds.find((kind) => kind.id === kindIds[0]);
  if (!selected) return { kind: null, extension, stem: withoutExtension, ambiguous: kindIds };
  const leading = splitLeadingEmoji(withoutExtension);
  const stem = leading.emoji && emojiFold(leading.emoji) === emojiFold(selected.emoji) ? leading.rest : withoutExtension;
  return { kind: selected, extension, stem, ambiguous: [] };
}

export interface ContentKindHint {
  readonly kindId: string | null;
  readonly violation?: TaxonomyViolation;
}

function shebangCommand(line: string): string | null {
  const raw = line.startsWith("#!") ? line.slice(2).trim() : "";
  if (!raw) return null;
  const tokens = raw.split(/\s+/u).filter(Boolean);
  let command = tokens.shift() ?? "";
  if (basename(command).toLocaleLowerCase("und") === "env") {
    while (tokens[0]?.startsWith("-") || /^[A-Za-z_][A-Za-z0-9_]*=/u.test(tokens[0] ?? "")) tokens.shift();
    command = tokens.shift() ?? "";
  }
  return command ? basename(command).replace(/\.exe$/iu, "").toLocaleLowerCase("und") : null;
}

function typescriptSyntax(text: string): boolean {
  return /\b(?:interface|namespace|enum)\s+[A-Za-z_$]|\btype\s+[A-Za-z_$][\w$]*\s*=|\b(?:const|let|var)\s+[A-Za-z_$][\w$]*\s*:\s*[^=]|\b(?:satisfies|as\s+const)\b/u.test(text);
}

export function extensionlessContentKind(path: string, bytes: Uint8Array | undefined, taxonomy: LoadedTaxonomy): ContentKindHint {
  const name = basename(path);
  if (name.includes(".") || !bytes) return { kindId: null };
  if (bytes.includes(0)) return { kindId: "binary" };
  let text: string;
  try {
    text = new TextDecoder("utf-8", { fatal: true }).decode(bytes);
  } catch {
    return { kindId: "binary" };
  }
  if (/[\u0001-\u0008\u000B\u000C\u000E-\u001F]/u.test(text)) return { kindId: null, violation: violation("content-kind-ambiguous", path, "Extensionless content contains non-text control bytes without a binary signature") };
  if (text.startsWith("#!")) {
    const command = shebangCommand(text.split(/\r?\n/u, 1)[0] ?? "");
    const kindId = command && /^(?:ba|da|z|k|fi)?sh$/u.test(command) ? "shell"
      : command && /^python(?:\d+(?:\.\d+)*)?$/u.test(command) ? "python-source"
        : command && /^(?:pwsh|powershell)$/u.test(command) ? "powershell"
          : command && /^(?:node|nodejs)$/u.test(command) ? (typescriptSyntax(text) ? "typescript-source" : "javascript-source")
            : command && /^(?:bun|deno|tsx|ts-node)$/u.test(command) ? (typescriptSyntax(text) ? "typescript-source" : "javascript-source")
              : null;
    if (!kindId) return { kindId: null, violation: violation("shebang-kind-unresolved", path, `Extensionless shebang interpreter is unknown or contradictory: ${command ?? "missing"}`) };
    if (!taxonomy.schema.fileKinds[kindId]) return { kindId: null, violation: violation("shebang-kind-unregistered", path, `Shebang resolved to unregistered file kind ${kindId}`) };
    return { kindId };
  }
  if (!taxonomy.schema.fileKinds["plain-text"]) return { kindId: null, violation: violation("text-kind-unregistered", path, "Extensionless UTF-8 content requires registered plain-text kind") };
  return { kindId: "plain-text" };
}

export function ownerId(path: string): string {
  const parts = path.split("/");
  if (parts[0] === ".🧬semio" && parts[1] === "🦑️repo" && parts[2] === "🎫️tickets" && parts.length >= 7) return parts.slice(0, 7).join("/");
  if (parts[0] === "✏️s" && (parts[1] === "🔌️plugins" || parts[1] === "🔨️modules") && parts[2]) return parts.slice(0, 3).join("/");
  if (parts[0] === "🧰️framework" && (parts[1] === "🛍️products" || parts[1] === "🔨️modules") && parts[2]) return parts.slice(0, 3).join("/");
  if ((parts[0] === "🌎️hub" || parts[0] === "♻️mit-bestand") && parts[1]) return parts.slice(0, 2).join("/");
  return parts[0] ?? "";
}

export function areaId(path: string): string {
  const first = path.split("/")[0] ?? "";
  if (first === "✏️s") return path.split("/").slice(0, 2).join("/");
  return first;
}

/** 🚨️ One violation. Path and message are copied into fresh strings: JavaScriptCore substrings and template ropes keep their whole source
 * string alive, so a message quoting a token would otherwise retain the entire file it came from for the life of the report (the UTF-16
 * round trip is exact for every code unit, lone surrogates included). */
export function violation(code: string, path: string, message: string, severity: TaxonomySeverity = "error"): TaxonomyViolation {
  return { code, severity, path: Buffer.from(path, "utf16le").toString("utf16le"), message: Buffer.from(message, "utf16le").toString("utf16le") };
}

export function stableViolations(rows: readonly TaxonomyViolation[]): readonly TaxonomyViolation[] {
  return [...new Map(rows.map((entry) => [`${entry.path}\u0000${entry.code}\u0000${entry.severity}\u0000${entry.message}`, entry])).values()].sort((a, b) => a.path.localeCompare(b.path) || a.code.localeCompare(b.code) || a.message.localeCompare(b.message));
}

export function checkCancellation(repoRoot: string, cancelFile?: string): void {
  if (!cancelFile) return;
  const path = assertLexicalInputOutsideOpaque(repoRoot, cancelFile, "cancelFile", true);
  if (existsSync(path)) throw new TaxonomyCancellationError();
}

export function cancellationRequested(repoRoot: string, cancelFile?: string): boolean {
  if (!cancelFile) return false;
  return existsSync(assertLexicalInputOutsideOpaque(repoRoot, cancelFile, "cancelFile", true));
}
