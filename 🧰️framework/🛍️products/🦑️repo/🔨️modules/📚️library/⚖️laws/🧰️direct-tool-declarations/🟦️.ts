import { readFileSync, statSync } from "node:fs";
import { join } from "node:path";
import { gitSyncV1 } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧰️tools/🟦️.ts";
import { validateJsonSchemaSubset } from "../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import patternsDocument from "./🔣️.json" with { type: "json" };
import patternsSchema from "./🧬️schema/🔣️.json" with { type: "json" };

/** 🗣️ A text in both declared languages. */
export type DirectToolTextV1 = Readonly<{ en: string; de: string }>;
/** 🧰️ One banned tool family: the id findings report and the command names it owns. */
export type DirectToolFamilyV1 = Readonly<{ id: string; commands: readonly string[] }>;
/** 🔎️ Inline-span rule of the Markdown kind: which lead-ins make a span an instruction. */
export type DirectToolInlineRuleV1 = Readonly<{ verbs: readonly string[]; modals: readonly string[]; negations: readonly string[]; verbGap: number; negationWindow: number; repoPayload: boolean }>;
/** 🗂️ One file kind: the globs it owns and how its commands are extracted. */
export type DirectToolKindV1 = Readonly<{
  id: string; extractor: "markdown" | "script" | "dockerfile" | "yaml" | "json" | "toml" | "bazel"; files: readonly string[];
  fenceLanguages?: readonly string[]; inlineSpans?: boolean; inline?: DirectToolInlineRuleV1; commentPrefixes?: readonly string[]; keys?: readonly string[];
  stringKeys?: readonly string[]; spacedKeys?: readonly string[]; argvKeys?: readonly string[]; mapKeys?: readonly string[]; proseFiles?: readonly string[]; proseKeys?: readonly string[];
}>;
/** 🚫️ A path set the law never reads. */
export type DirectToolExcludeV1 = Readonly<{ id: string; paths: readonly string[]; reason: DirectToolTextV1 }>;
/** ✅️ A path set where some tools in some contexts are the point of the file. */
export type DirectToolAllowV1 = Readonly<{ id: string; paths: readonly string[]; tools: readonly string[]; contexts: readonly string[]; reason: DirectToolTextV1 }>;
/** 📋️ The pattern document every implementation of the law reads. */
export type DirectToolPatternsV1 = Readonly<{
  version: 1; id: string; summary: DirectToolTextV1; enforced: readonly DirectToolFamilyV1[]; observed: readonly DirectToolFamilyV1[]; wrappers: readonly string[];
  wrapperValueOptions: Readonly<Record<string, readonly string[]>>; shells: readonly string[]; shellCommandFlags: readonly string[]; separators: readonly string[]; prompts: readonly string[];
  payloadAfterDoubleDash: readonly (readonly string[])[]; kinds: readonly DirectToolKindV1[]; exclude: readonly DirectToolExcludeV1[]; allow: readonly DirectToolAllowV1[];
}>;
/** 🔬️ One finding: a banned tool started by name in a declarative file. */
export type DirectToolFindingV1 = Readonly<{ path: string; line: number; tool: string; word: string; command: string; kind: string; context: string; allowed: string | null }>;
type Row = readonly [number, string, string];
type Word = readonly ["word" | "sep", string];

/** 📖️ The pattern document of the repository, validated against its schema. */
export const DIRECT_TOOL_PATTERNS_V1: DirectToolPatternsV1 = (() => {
  const errors = validateJsonSchemaSubset(patternsSchema, patternsDocument);
  if (errors.length > 0) throw new Error(`direct-tool-patterns-invalid:${errors.slice(0, 5).join(";")}`);
  return patternsDocument as unknown as DirectToolPatternsV1;
})();

/** 🔤️ Converts a glob (`**` spans directories, `*` and `?` stay inside one segment) into an anchored expression. */
export function directToolGlobV1(pattern: string): RegExp {
  let out = "^";
  for (let i = 0; i < pattern.length; i++) {
    const c = pattern[i]!;
    if (c === "*") {
      if (pattern.startsWith("**/", i)) { out += "(?:.*/)?"; i += 2; continue; }
      if (pattern.startsWith("**", i)) { out += ".*"; i += 1; continue; }
      out += "[^/]*";
    } else if (c === "?") out += "[^/]";
    else out += c.replace(/[.*+?^${}()|[\]\\]/gu, "\\$&");
  }
  return new RegExp(out + "$", "su");
}

const WORD = "[\\p{L}\\p{N}_]";
const escape = (text: string): string => text.replace(/[.*+?^${}()|[\]\\]/gu, "\\$&");
const isSpace = (c: string): boolean => /\s/u.test(c);
const lines = (text: string): string[] => text.split("\n");
const lineAt = (text: string, end: number): number => { let count = 1; for (let i = 0; i < end; i++) if (text[i] === "\n") count++; return count; };

/** ⚖️ The law over one pattern document: command extraction per file kind, the banned families and the exclusions. */
export function createDirectToolLawV1(patterns: DirectToolPatternsV1 = DIRECT_TOOL_PATTERNS_V1) {
  const enforced = new Map<string, string>(), observed = new Map<string, string>();
  for (const row of patterns.enforced) for (const command of row.commands) enforced.set(command, row.id);
  for (const row of patterns.observed) for (const command of row.commands) observed.set(command, row.id);
  const wrappers = new Set(patterns.wrappers), shells = new Set(patterns.shells), shellFlags = new Set(patterns.shellCommandFlags.map(flag => flag.toLowerCase()));
  const separators = [...patterns.separators].sort((a, b) => b.length - a.length), payload = patterns.payloadAfterDoubleDash;
  const kinds = patterns.kinds.map(kind => ({ kind, regexes: kind.files.map(directToolGlobV1) }));
  const excludes = patterns.exclude.map(row => ({ row, regexes: row.paths.map(directToolGlobV1) }));
  const allows = patterns.allow.map(row => ({ row, regexes: row.paths.map(directToolGlobV1) }));
  const inlineRule = patterns.kinds.find(kind => kind.inline !== undefined)!.inline!;
  const jsonKind = patterns.kinds.find(kind => kind.id === "json-config")!;
  const inlineVerbs = new RegExp(`(?:^|[.:;!?)]\\s*|^\\s*(?:[-*+]|\\d+[.)])\\s*|(?<!${WORD})(?:${inlineRule.modals.map(escape).join("|")})\\s+)(${inlineRule.verbs.map(escape).join("|")})(?!${WORD})[^\`.;]{0,${inlineRule.verbGap}}$`, "u");
  const inlineNegations = inlineRule.negations.map(word => new RegExp(`(?<!${WORD})${escape(word)}(?!${WORD})`, "u"));
  const proseFiles = (jsonKind.proseFiles ?? []).map(directToolGlobV1), proseKeys = new Set(jsonKind.proseKeys ?? []);

  const toolOf = (word: string): readonly [string, "enforced" | "observed"] | undefined => {
    let base = word.replaceAll("\\", "/").split("/").pop()!;
    const lowered = base.toLowerCase();
    for (const suffix of [".exe", ".cmd", ".bat", ".ps1"]) if (lowered.endsWith(suffix)) { base = base.slice(0, base.length - suffix.length); break; }
    const hard = enforced.get(base);
    if (hard !== undefined) return [hard, "enforced"];
    const soft = observed.get(base);
    return soft === undefined ? undefined : [soft, "observed"];
  };

  const splitWords = (text: string): Word[] => {
    const out: Word[] = [];
    let buf = "", quote: string | null = null, i = 0;
    const flush = (): void => { if (buf.length > 0) { out.push(["word", buf]); buf = ""; } };
    while (i < text.length) {
      const c = text[i]!;
      if (quote !== null) {
        if (c === "\\" && quote === '"' && i + 1 < text.length) { buf += text.slice(i, i + 2); i += 2; continue; }
        if (text.startsWith("$(", i) && quote === '"') { flush(); out.push(["sep", "$("]); i += 2; continue; }
        if (c === quote) quote = null;
        buf += c; i++; continue;
      }
      if (c === "'" || c === '"') { quote = c; buf += c; i++; continue; }
      if (isSpace(c)) { flush(); i++; continue; }
      if (c === "#" && buf.length === 0 && (i === 0 || isSpace(text[i - 1]!))) break;
      let hit: string | null = null;
      for (const separator of separators) if (text.startsWith(separator, i)) { hit = separator; break; }
      if (hit === "|" && !((i === 0 || isSpace(text[i - 1]!)) || (i + 1 >= text.length || isSpace(text[i + 1]!)))) hit = null;
      if (hit !== null && !(hit === "{" && (i + 1 >= text.length || !isSpace(text[i + 1]!))) && !(hit === "&" && i + 1 < text.length && text[i + 1] === ">") && !(hit === "(" && buf.length > 0)) { flush(); out.push(["sep", hit]); i += hit.length; continue; }
      buf += c; i++;
    }
    flush();
    return out;
  };

  const commands = (text: string): Array<readonly [string, string, string]> => {
    const results: Array<readonly [string, string, string]> = [];
    const words = splitWords(text);
    let position = true, index = 0;
    while (index < words.length) {
      const [kind, value] = words[index]!;
      if (kind === "sep") { position = true; index++; continue; }
      if (!position) { index++; continue; }
      const bare = value.replace(/^['"]+|['"]+$/gu, "");
      if (/^[A-Za-z_][A-Za-z0-9_]*=/u.test(value)) { index++; continue; }
      if (/^(\$env:|\$)[A-Za-z_][A-Za-z0-9_]*=/u.test(value)) { index++; continue; }
      const base = bare.replaceAll("\\", "/").split("/").pop()!;
      if (wrappers.has(base) || bare === "&" || bare === ".") {
        const options = patterns.wrapperValueOptions[base] ?? [];
        index++;
        while (index < words.length && words[index]![0] === "word") {
          const next = words[index]![1];
          if (options.includes(next) && index + 1 < words.length) { index += 2; continue; }
          if (next.startsWith("-") && base in patterns.wrapperValueOptions) { index++; continue; }
          if (base === "env" && /^[A-Za-z_][A-Za-z0-9_]*=/u.test(next)) { index++; continue; }
          break;
        }
        continue;
      }
      const tool = toolOf(bare);
      if (tool !== undefined) results.push([tool[0], tool[1], bare]);
      else if (shells.has(base)) {
        for (let j = index + 1; j < words.length && words[j]![0] === "word"; j++) {
          if (shellFlags.has(words[j]![1].toLowerCase()) && j + 1 < words.length && words[j + 1]![0] === "word") {
            let inner = words[j + 1]![1];
            if (inner.length >= 2 && (inner[0] === "'" || inner[0] === '"') && inner[inner.length - 1] === inner[0]) inner = inner.slice(1, -1);
            results.push(...commands(inner));
            break;
          }
        }
      } else if (base === "repo") {
        const rest: string[] = [];
        for (let j = index + 1; j < words.length && words[j]![0] === "word"; j++) rest.push(words[j]![1]);
        const boundary = rest.indexOf("--");
        if (boundary >= 0) {
          const head = rest.slice(0, boundary).filter(word => !word.startsWith("-"));
          for (const verb of payload) if (verb.every((word, at) => head[at] === word) && head.length >= verb.length) { results.push(...commands(rest.slice(boundary + 1).join(" "))); break; }
        }
      }
      position = false;
      index++;
    }
    return results;
  };

  const promptStrip = (line: string): string => {
    const stripped = line.replace(/^\s+/u, "");
    for (const prompt of patterns.prompts) if (prompt !== "# " && stripped.startsWith(prompt)) return stripped.slice(prompt.length);
    return stripped;
  };

  const inlineFlagged = (rule: DirectToolInlineRuleV1, before: string, content: string): boolean => {
    const first = content.split(/\s+/u).find(word => word.length > 0)!;
    if (rule.repoPayload && first === "repo") return true;
    const lead = before.toLowerCase(), match = inlineVerbs.exec(lead);
    if (match === null) return false;
    const window = lead.slice(Math.max(0, match.index - rule.negationWindow), match.index + match[0].length);
    return !inlineNegations.some(negation => negation.test(window));
  };

  const walkJson = (kind: DirectToolKindV1, path: string, document: unknown, text: string): Row[] => {
    const stringKeys = new Set(kind.stringKeys ?? []), spacedKeys = new Set(kind.spacedKeys ?? []), argvKeys = new Set(kind.argvKeys ?? []), mapKeys = new Set(kind.mapKeys ?? []);
    const proseFile = proseFiles.some(regex => regex.test(path)), rows: Row[] = [];
    let cursor = 0, counted = 0, line = 1;
    const lineOf = (value: string): number => {
      const at = text.indexOf(JSON.stringify(value), cursor);
      if (at >= 0) cursor = at + 1;
      for (; counted < cursor; counted++) if (text.charCodeAt(counted) === 10) line++;
      return line;
    };
    const walk = (node: unknown, key: string | null): void => {
      if (Array.isArray(node)) {
        if (key !== null && argvKeys.has(key) && node.length > 0 && node.every(item => typeof item === "string")) { rows.push([lineOf(node[0] as string), (node as string[]).join(" "), "json"]); return; }
        for (const item of node) walk(item, key);
      } else if (typeof node === "object" && node !== null) {
        for (const [k, v] of Object.entries(node)) {
          if (mapKeys.has(k) && typeof v === "object" && v !== null && !Array.isArray(v)) { for (const value of Object.values(v)) if (typeof value === "string") rows.push([lineOf(value), value, "json"]); continue; }
          walk(v, k);
        }
      } else if (typeof node === "string" && key !== null) {
        if (stringKeys.has(key) || (spacedKeys.has(key) && /\s/u.test(node.trim()))) rows.push([lineOf(node), node, "json"]);
        else if (proseFile && proseKeys.has(key)) for (const span of node.matchAll(/`([^`\n]+)`/gu)) {
          const content = promptStrip(span[1]!);
          if (/\s/u.test(content.trim()) && inlineFlagged(inlineRule, node.slice(0, span.index), content)) rows.push([lineOf(node), content, "inline"]);
        }
      }
    };
    walk(document, null);
    return rows;
  };

  const extract = (kind: DirectToolKindV1, path: string, text: string): Row[] => {
    const found: Row[] = [], all = lines(text);
    if (kind.extractor === "markdown") {
      let fence: string | null = null;
      const languages = new Set(kind.fenceLanguages ?? []);
      all.forEach((line, at) => {
        const match = /^\s*(```+|~~~+)\s*([\w+-]*)/u.exec(line);
        if (match !== null) { fence = fence === null ? match[2]!.toLowerCase() : null; return; }
        if (fence !== null) { if (languages.has(fence)) found.push([at + 1, promptStrip(line), "fence"]); }
        else if (kind.inlineSpans === true) for (const span of line.matchAll(/`([^`\n]+)`/gu)) {
          const content = promptStrip(span[1]!);
          if (/\s/u.test(content.trim()) && inlineFlagged(kind.inline!, line.slice(0, span.index), content)) found.push([at + 1, content, "inline"]);
        }
      });
    } else if (kind.extractor === "script") {
      const prefixes = kind.commentPrefixes ?? [];
      let pending = "", start = 0;
      all.forEach((line, at) => {
        let stripped = line.trim(), number = at + 1;
        if (stripped.length === 0 || prefixes.some(prefix => stripped.startsWith(prefix))) return;
        if (path.endsWith(".ps1") || path.endsWith(".psm1")) stripped = stripped.replace(/^&\s+/u, "& ");
        if (pending.length > 0) { stripped = pending + " " + stripped; number = start; }
        if (stripped.endsWith("\\") || (stripped.endsWith("`") && path.endsWith(".ps1"))) { pending = stripped.slice(0, -1).trimEnd(); start = start || number; return; }
        pending = ""; start = 0;
        found.push([number, stripped, "line"]);
      });
    } else if (kind.extractor === "dockerfile") {
      let pending = "", start = 0;
      all.forEach((line, at) => {
        let stripped = line.trim();
        if (stripped.length === 0 || stripped.startsWith("#")) return;
        if (pending.length > 0) stripped = pending + " " + stripped; else start = at + 1;
        if (stripped.endsWith("\\")) { pending = stripped.slice(0, -1).trimEnd(); return; }
        pending = "";
        const match = /^(RUN|CMD|ENTRYPOINT|HEALTHCHECK\s+(?:--\S+\s+)*CMD)\s+(.*)$/iu.exec(stripped);
        if (match === null) return;
        let body = match[2]!.trim();
        if (body.startsWith("[")) { try { body = (JSON.parse(body) as string[]).join(" "); } catch { body = match[2]!.trim(); } }
        found.push([start, body, "line"]);
      });
    } else if (kind.extractor === "yaml") {
      const keys = new Set(kind.keys ?? []);
      for (let i = 0; i < all.length; i++) {
        const match = /^(\s*)(?:-\s+)?([A-Za-z_][\w-]*):\s*(.*)$/u.exec(all[i]!);
        if (match === null || !keys.has(match[2]!)) continue;
        const indent = match[1]!.length;
        let value = match[3]!.trim();
        if (value[0] === "|" || value[0] === ">") {
          let j = i + 1;
          while (j < all.length && (all[j]!.trim().length === 0 || all[j]!.length - all[j]!.trimStart().length > indent)) { if (all[j]!.trim().length > 0 && !all[j]!.trim().startsWith("#")) found.push([j + 1, all[j]!.trim(), "line"]); j++; }
          i = j - 1;
          continue;
        }
        if (value.length > 0 && !value.startsWith("#")) {
          if (value.startsWith("[")) { try { value = (JSON.parse(value) as string[]).join(" "); } catch { value = match[3]!.trim(); } }
          found.push([i + 1, (value[0] === "'" || value[0] === '"') && value[value.length - 1] === value[0] ? value.replace(/^['"]+|['"]+$/gu, "") : value, "line"]);
        }
      }
    } else if (kind.extractor === "toml") {
      const keys = new Set(kind.keys ?? []);
      let alias = false;
      all.forEach((line, at) => {
        const header = /^\s*\[([^\]]+)\]/u.exec(line);
        if (header !== null) { alias = header[1]!.trim() === "alias"; return; }
        const match = /^\s*([A-Za-z_][\w.-]*)\s*=\s*(.+)$/u.exec(line);
        if (match === null || !(keys.has(match[1]!) || alias)) return;
        let value = match[2]!.trim();
        if (value[0] === "[") value = [...value.matchAll(/"([^"]*)"|'([^']*)'/gu)].map(word => word[1] ?? word[2] ?? "").join(" ");
        else if (value[0] === "'" || value[0] === '"') value = value.slice(1, value.indexOf(value[0]!, 1));
        found.push([at + 1, value, "line"]);
      });
    } else if (kind.extractor === "bazel") {
      const keys = (kind.keys ?? []).map(escape).join("|");
      for (const match of text.matchAll(new RegExp(`(?:${keys})\\s*=\\s*("""|'''|"|')([\\s\\S]*?)\\1`, "gu"))) found.push([lineAt(text, match.index), match[2]!, "line"]);
      for (const match of text.matchAll(/execute\(\s*\[\s*("[^"]*"|'[^']*')/gu)) found.push([lineAt(text, match.index), match[1]!.slice(1, -1) + " x", "line"]);
    } else if (kind.extractor === "json") {
      let document: unknown;
      try { document = JSON.parse(text); } catch { return found; }
      found.push(...walkJson(kind, path, document, text));
    }
    return found;
  };

  const kindOf = (path: string): DirectToolKindV1 | undefined => kinds.find(row => row.regexes.some(regex => regex.test(path)))?.kind;
  const excluded = (path: string): string | null => excludes.find(row => row.regexes.some(regex => regex.test(path)))?.row.id ?? null;
  const allowed = (path: string, tool: string, context = "line"): string | null => allows.find(row => (row.row.tools.includes(tool) || row.row.tools.includes("*")) && row.row.contexts.includes(context) && row.regexes.some(regex => regex.test(path)))?.row.id ?? null;
  const scanText = (path: string, text: string): DirectToolFindingV1[] => {
    const kind = kindOf(path);
    if (kind === undefined) return [];
    const findings: DirectToolFindingV1[] = [];
    for (const [line, command, context] of extract(kind, path, text)) for (const [tool, cls, word] of commands(command)) {
      if (cls !== "enforced") continue;
      findings.push({ path, line, tool, word, command: command.trim().slice(0, 160), kind: kind.id, context, allowed: allowed(path, tool, context) });
    }
    return findings;
  };
  return { commands: (text: string) => commands(text), scanText, kindOf, excluded, allowed };
}

/** 🧾️ Lists the files of the repository the law reads: tracked and untracked, not ignored. */
export function listDirectToolFilesV1(root: string): readonly string[] {
  const result = gitSyncV1(["ls-files", "-co", "--exclude-standard", "-z"], { cwd: root, maxBuffer: 512 * 1024 * 1024 });
  if (result.status !== 0) throw new Error(`git-ls-files-failed:${result.stderr.trim()}`);
  return result.stdout.split("\0").filter(Boolean);
}

/** 🔧️ Scans files below a root; an empty file list is refused. */
export function scanDirectToolDeclarationsV1(root: string, files: readonly string[], law = createDirectToolLawV1()): Readonly<{ scanned: number; findings: readonly DirectToolFindingV1[] }> {
  if (files.length === 0) throw new Error("direct-tool-declarations-empty-scope");
  const findings: DirectToolFindingV1[] = [];
  let scanned = 0;
  for (const path of files) {
    if (law.excluded(path) !== null || law.kindOf(path) === undefined) continue;
    const full = join(root, path);
    try { const stat = statSync(full); if (!stat.isFile() || stat.size > 8_000_000) continue; } catch { continue; }
    let text: string;
    try { text = new TextDecoder("utf-8", { fatal: true }).decode(readFileSync(full)); } catch { continue; }
    scanned++;
    findings.push(...law.scanText(path, text.replaceAll("\r\n", "\n")));
  }
  return { scanned, findings };
}
