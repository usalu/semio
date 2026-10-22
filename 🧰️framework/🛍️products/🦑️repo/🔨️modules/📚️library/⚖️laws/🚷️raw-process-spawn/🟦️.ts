import { readFileSync } from "node:fs";
import { join } from "node:path";
import { gitSyncV1, maskCommentsV1, readArgumentV1, shellCommandsV1, toolGlobMatchesV1, TOOL_REGISTRY_V1, type ToolRegistryV1 } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧰️tools/🟦️.ts";
import { validateJsonSchemaSubset } from "../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import type { BreachRecord } from "../../🚨️policy/🔖️breach/🟦️.ts";
import allowlistDocument from "./📒️allowlist/🔣️.json" with { type: "json" };
import allowlistSchema from "./🧬️schema/🔣️.json" with { type: "json" };

/** 🌐️ The languages whose process starts the law reads. */
export type ProcessSpawnLanguageV1 = "ts" | "rust" | "go" | "python";
/** 🚀️ One place in a source file that starts a process: `spawn` is a process primitive, `shell` a command line handed to a shell, `funnel` a call of a process funnel such as `runCmd`. */
export type ProcessSiteV1 = Readonly<{ path: string; line: number; language: ProcessSpawnLanguageV1; callee: string; form: "spawn" | "shell" | "funnel"; program: string; scope: "prod" | "test" }>;
type CalleeV1 = Readonly<{ name: string; language: ProcessSpawnLanguageV1; shape: "command" | "array" | "shell-string" | "template" | "optionsCmd"; imported?: boolean }>;
type FunnelCalleeV1 = Readonly<{ name: string; language: ProcessSpawnLanguageV1; argument: number }>;
/** 🏛️ One module (or glob of modules) that implements the process port itself and may start processes. */
export type ProcessInterfaceV1 = Readonly<{ glob: string; kind: string; reason: Readonly<{ en: string; de: string }> }>;
/** 🪪️ One documented exception: a file that starts `program` (`<dynamic>` for a program that comes from data) `count` times although it is not an interface module, with the class and the reason it stays. */
export type ProcessExceptionV1 = Readonly<{ path: string; program: string; count: number; class: string; reason: Readonly<{ en: string; de: string }> }>;
/** 📒️ The language-agnostic allowlist document: what is scanned, which calls are process primitives, which modules are the interface and which sites are documented exceptions. */
export type ProcessAllowlistV1 = Readonly<{ version: 1; roots: readonly string[]; processModules: readonly string[]; extraCallees: readonly CalleeV1[]; commandFunnels: readonly FunnelCalleeV1[]; interfaces: readonly ProcessInterfaceV1[]; exceptions: readonly ProcessExceptionV1[] }>;
/** 🚩️ One violated rule: `raw-spawn` (a process start outside the interface modules), `unregistered-command` (a literal program the tool registry does not know) or `allowlist` (the allowlist document is incomplete). */
export type ProcessFindingV1 = Readonly<{ rule: "raw-spawn" | "unregistered-command" | "allowlist"; path: string; line: number; program: string; detail: string }>;
/** 🧾️ A production site with the way the law accounts for it. */
export type ProcessVerdictRowV1 = ProcessSiteV1 & Readonly<{ disposition: string }>;

/** 🏠️ The directory of this law, relative to the repository root. */
export const PROCESS_SPAWN_LAW_DIRECTORY_V1 = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🚷️raw-process-spawn";
const TOOLS_MODULE = "🧰️framework/🔨️modules/🏃️process/🧰️tools/**";

/** 📖️ Validates an allowlist document against its schema. */
export function readProcessAllowlistV1(document: unknown): ProcessAllowlistV1 {
  const errors = validateJsonSchemaSubset(allowlistSchema, document);
  if (errors.length) throw new Error(`process-spawn-allowlist-invalid:${errors.slice(0, 5).join(";")}`);
  return document as ProcessAllowlistV1;
}

/** 📚️ The committed allowlist. */
export const PROCESS_ALLOWLIST_V1: ProcessAllowlistV1 = readProcessAllowlistV1(allowlistDocument);

const IDENTIFIER = /^[A-Za-z_$][\w$]*$/u;
const STRING = /^(?:"((?:[^"\\\n]|\\.)*)"|'((?:[^'\\\n]|\\.)*)'|`([^`$]*)`)$/u;
const literalOf = (expression: string): string | undefined => { const match = STRING.exec(expression.trim()); return match === null ? undefined : match[1] ?? match[2] ?? match[3]; };
const escapeRegex = (text: string): string => text.replace(/[.*+?^${}()|[\]\\]/gu, "\\$&");

function maskPython(text: string, blankStrings: boolean): string {
  const out = text.split("");
  const blank = (from: number, to: number): void => { for (let index = from; index < to; index++) if (out[index] !== "\n") out[index] = " "; };
  let index = 0;
  while (index < text.length) {
    const char = text[index]!;
    if (char === "#") { let end = text.indexOf("\n", index); if (end < 0) end = text.length; blank(index, end); index = end; continue; }
    if (char === "\"" || char === "'") {
      if (text.startsWith(char.repeat(3), index)) { const end = text.indexOf(char.repeat(3), index + 3), stop = end < 0 ? text.length : end + 3; blank(index, stop); index = stop; continue; }
      let stop = index + 1;
      while (stop < text.length) { if (text[stop] === "\\") { stop += 2; continue; } if (text[stop] === char || text[stop] === "\n" && char !== "\"") { stop++; break; } stop++; }
      if (blankStrings) blank(index + 1, stop - 1);
      index = stop; continue;
    }
    index++;
  }
  return out.join("");
}

const mask = (text: string, language: ProcessSpawnLanguageV1, blankStrings: boolean): string => language === "python" ? maskPython(text, blankStrings) : maskCommentsV1(text, language, blankStrings);

function resolveProgram(expression: string, masked: string, language: ProcessSpawnLanguageV1, selfs: readonly string[]): string | undefined {
  const trimmed = expression.trim().replace(/^&/u, "").replace(/\.to_string\(\)$|\.as_str\(\)$|\.into\(\)$/u, "");
  const direct = literalOf(trimmed);
  if (direct !== undefined) return direct;
  if (selfs.includes(trimmed)) return "<self>";
  if (!IDENTIFIER.test(trimmed)) return undefined;
  const name = escapeRegex(trimmed);
  const declaration = language === "rust" ? new RegExp(`\\b(?:const|static|let)\\s+(?:mut\\s+)?${name}\\s*(?::[^=]+)?=\\s*([^;\\n]+)`, "u")
    : language === "go" ? new RegExp(`\\b(?:const|var)?\\s*${name}\\s*(?::=|=)\\s*([^\\n]+)`, "u")
    : language === "python" ? new RegExp(`^\\s*${name}\\s*=\\s*([^\\n]+)`, "mu")
    : new RegExp(`\\b(?:const|let|var)\\s+${name}\\s*(?::[^=]+)?=\\s*([^;\\n]+)`, "u");
  const found = declaration.exec(masked);
  if (found === null) return undefined;
  const right = found[1]!.trim();
  if (selfs.includes(right)) return "<self>";
  return literalOf(right.replace(/\s+as\s+const$/u, "").replace(/[;, ]+$/u, ""));
}

function processImports(text: string, modules: readonly string[]): Map<string, string> {
  const names = new Map<string, string>();
  for (const module of modules) {
    const specifier = `(?:from\\s+["']${escapeRegex(module)}["']|require\\(\\s*["']${escapeRegex(module)}["']\\s*\\))`;
    for (const match of text.matchAll(new RegExp(`import\\s+(?:type\\s+)?([^;]*?)\\s+${specifier}`, "gus"))) {
      const clause = match[1]!;
      const namespace = /\*\s+as\s+([\w$]+)/u.exec(clause);
      if (namespace !== null) names.set(namespace[1]!, "*");
      const lead = /^\s*([\w$]+)\s*(?:,|$)/u.exec(clause);
      if (lead !== null && !clause.trim().startsWith("{") && !clause.trim().startsWith("*")) names.set(lead[1]!, "*");
      const brace = /\{([^}]*)\}/u.exec(clause);
      if (brace !== null) for (const part of brace[1]!.split(",")) { const bits = part.trim().split(/\s+as\s+/u); if (bits[0]) names.set(bits[bits.length - 1]!.trim(), bits[0].trim()); }
    }
    for (const match of text.matchAll(new RegExp(`(?:const|let|var)\\s+(\\{[^}]*\\}|[\\w$]+)\\s*=\\s*require\\(\\s*["']${escapeRegex(module)}["']\\s*\\)`, "gu"))) {
      const target = match[1]!;
      if (target.startsWith("{")) { for (const part of target.replace(/[{}]/gu, "").split(",")) { const bits = part.trim().split(/\s*:\s*/u); if (bits[0]) names.set(bits[bits.length - 1]!.trim(), bits[0].trim()); } } else names.set(target, "*");
    }
    if (new RegExp(`await\\s+import\\(\\s*["']${escapeRegex(module)}["']\\s*\\)`, "u").test(text)) names.set("<dynamic-import>", "*");
  }
  return names;
}

const isDefinition = (code: string, start: number): boolean => /\b(?:function\*?|func|fn|def)\s+$/u.test(code.slice(Math.max(0, start - 16), start));
const lineStarts = (text: string): number[] => { const starts = [0]; for (let index = 0; index < text.length; index++) if (text[index] === "\n") starts.push(index + 1); return starts; };
const lineOf = (starts: readonly number[], offset: number): number => { let low = 0, high = starts.length - 1; while (low < high) { const middle = (low + high + 1) >> 1; if (starts[middle]! <= offset) low = middle; else high = middle - 1; } return low + 1; };

/** 🔌️ The process primitives of the registry plus the ones the allowlist adds. */
function calleesOf(registry: ToolRegistryV1, allowlist: ProcessAllowlistV1): readonly CalleeV1[] { return [...registry.scan.callees, ...allowlist.extraCallees]; }

function languageOf(path: string, registry: ToolRegistryV1): ProcessSpawnLanguageV1 | undefined {
  const name = path.split("/").pop()!, extension = name.includes(".") ? "." + name.split(".").pop()! : "";
  for (const language of ["ts", "rust", "go", "python"] as const) if (registry.scan.extensions[language].includes(extension)) return language;
  return undefined;
}

const scopeOf = (path: string, registry: ToolRegistryV1): "prod" | "test" => path.split("/").some(segment => registry.scan.testSegments.includes(segment)) || /\.(?:test|spec)\.[a-z]+$/u.test(path) || /_test\.go$/u.test(path) ? "test" : "prod";

/** 🔎️ Finds every process start of a source file whatever the program: the node and runtime primitives, `Command::new`, `exec.Command`, `subprocess`, shell strings and the process funnels; the program is the literal word, `<self>` for the running executable or `<dynamic>`. */
export function scanProcessStartsV1(path: string, text: string, registry: ToolRegistryV1 = TOOL_REGISTRY_V1, allowlist: ProcessAllowlistV1 = PROCESS_ALLOWLIST_V1): readonly ProcessSiteV1[] {
  const language = languageOf(path, registry);
  if (language === undefined) return [];
  const callees = calleesOf(registry, allowlist).filter(callee => callee.language === language), funnels = allowlist.commandFunnels.filter(funnel => funnel.language === language);
  const needles = [...callees.map(callee => callee.shape === "template" ? "$`" : callee.name.split(".").pop()!.split("::")[0]!), ...funnels.map(funnel => funnel.name)];
  if (!needles.some(needle => text.includes(needle))) return [];
  const scope = scopeOf(path, registry), masked = mask(text, language, false), code = mask(text, language, true), starts = lineStarts(text), selfs = registry.scan.selfExpressions;
  const testAt = language === "rust" ? text.indexOf("#[cfg(test)]") : -1, sites: ProcessSiteV1[] = [];
  const add = (callee: string, form: ProcessSiteV1["form"], program: string, offset: number): void => { sites.push({ path, line: lineOf(starts, offset), language, callee, form, program, scope: testAt >= 0 && offset >= testAt ? "test" : scope }); };
  const imported = language === "ts" ? processImports(text, allowlist.processModules) : new Map<string, string>();
  const rustProcess = language === "rust" && (/\b(?:std|tokio|async_std)::process\b/u.test(text) || /use\s+std::process::\{?[^;]*\bCommand\b/u.test(text));
  const rustClap = language === "rust" && /\bclap\b/u.test(text) && !rustProcess;
  for (const callee of callees) {
    const name = callee.name, escaped = escapeRegex(name), qualified = name.includes(".") || name.includes("::");
    let matches: RegExpExecArray[] = [];
    if (callee.shape === "template") {
      const bare = /import\s*\{[^}]*(?<![\w$])\$(?![\w$])[^}]*\}\s*from\s*["']bun["']/u.test(text);
      for (const match of code.matchAll(bare ? /(?<![\w$])(?:Bun\.)?\$`/gu : /(?<![\w$])Bun\.\$`/gu)) {
        const end = masked.indexOf("`", match.index + match[0].length), body = masked.slice(match.index + match[0].length, end < 0 ? undefined : end), words = shellCommandsV1(body);
        for (const word of words) add(name, "shell", word, match.index);
        if (words.length === 0) add(name, "shell", "<dynamic>", match.index);
      }
      continue;
    }
    if (language === "ts" && !qualified && callee.imported !== false) {
      const direct = [...imported].filter(([, original]) => original === name).map(([local]) => local), spaces = [...imported].filter(([local, original]) => original === "*" && local !== "<dynamic-import>").map(([local]) => local);
      if (direct.length === 0 && !imported.size) continue;
      if (direct.length === 0 && ![...imported.values()].includes("*")) continue;
      if (direct.length > 0) matches.push(...code.matchAll(new RegExp(`(?<![\\w$.])(?:${direct.map(escapeRegex).join("|")})\\s*\\(`, "gu")));
      if (spaces.length > 0) matches.push(...code.matchAll(new RegExp(`(?<![\\w$])(?:${spaces.map(escapeRegex).join("|")})\\.${escaped}\\s*\\(`, "gu")));
      if (imported.has("<dynamic-import>")) matches.push(...code.matchAll(new RegExp(`(?<![\\w$])[\\w$]+\\.${escaped}\\s*\\(`, "gu")));
    } else if (language === "rust" && name === "Command::new") {
      matches = rustClap ? [] : [...code.matchAll(rustProcess ? /(?<![\w$])(?:(?:std|tokio)::process::)?Command::new\s*\(/gu : /(?<![\w$:])(?:std::process|tokio::process)::Command::new\s*\(/gu)];
    } else {
      matches = [...code.matchAll(new RegExp(`(?<![\\w$${qualified ? "" : "."}])${escaped}\\s*\\(`, "gu"))];
      if (!qualified && language !== "ts") matches.push(...code.matchAll(new RegExp(`(?<![\\w$])[\\w$]+\\.${escaped}\\s*\\(`, "gu")));
    }
    for (const match of matches) {
      if (isDefinition(code, match.index)) continue;
      const first = readArgumentV1(masked, match.index + match[0].length);
      if (language === "rust" && masked[first.end] !== ")") continue;
      let expression = first.text, rest = masked.slice(first.end, first.end + 400);
      if (callee.shape === "array" || callee.shape === "optionsCmd") {
        const options = /^\{[\s\S]*?\bcmd\s*:\s*\[/u.exec(first.text);
        if (options !== null) expression = readArgumentV1(first.text, options[0].length).text;
        else if (first.text.startsWith("[")) expression = readArgumentV1(first.text, 1).text;
        else if (language === "python") {
          const direct = literalOf(first.text);
          if (direct === undefined) add(name, "spawn", "<dynamic>", match.index);
          else for (const word of shellCommandsV1(direct)) add(name, "shell", word, match.index);
          continue;
        } else { add(name, "spawn", "<dynamic>", match.index); continue; }
      }
      if (language === "go" && name === "exec.CommandContext") { const second = readArgumentV1(masked, first.end + 1); expression = second.text; rest = masked.slice(second.end, second.end + 400); }
      if (callee.shape === "shell-string") {
        let line = literalOf(first.text);
        if (line === undefined && first.text.startsWith("`")) { const cut = first.text.indexOf("${"); line = first.text.slice(1, cut < 0 ? first.text.length - 1 : cut); }
        if (line === undefined) { add(name, "shell", "<dynamic>", match.index); continue; }
        const words = shellCommandsV1(line);
        for (const word of words) add(name, "shell", word, match.index);
        if (words.length === 0) add(name, "shell", "<dynamic>", match.index);
        continue;
      }
      const value = resolveProgram(expression, masked, language, selfs);
      if (value === undefined) { add(name, "spawn", "<dynamic>", match.index); continue; }
      if (registry.scan.shellWrappers.includes(value.split("/").pop()!.split("\\").pop()!)) {
        const flag = /["'](?:-c|-lc|\/c|\/k|-Command|-command)["']\s*,\s*(?:"((?:[^"\\\n]|\\.)*)"|'((?:[^'\\\n]|\\.)*)'|`([^`]*)`)/u.exec(rest);
        if (flag !== null) { const words = shellCommandsV1(flag[1] ?? flag[2] ?? flag[3] ?? ""); for (const word of words) add(name, "shell", word, match.index); if (words.length > 0) continue; }
      }
      add(name, "spawn", value, match.index);
    }
  }
  for (const funnel of funnels) {
    for (const match of code.matchAll(new RegExp(`(?<![\\w$.])${escapeRegex(funnel.name)}\\s*\\(`, "gu"))) {
      if (isDefinition(code, match.index)) continue;
      const args: string[] = [];
      let position = match.index + match[0].length;
      for (let index = 0; index <= funnel.argument; index++) { const argument = readArgumentV1(masked, position); args.push(argument.text); position = argument.end + 1; }
      if (args.length <= funnel.argument) continue;
      add(funnel.name, "funnel", resolveProgram(args[funnel.argument]!, masked, language, selfs) ?? "<dynamic>", match.index);
    }
  }
  return sites.sort((a, b) => a.line - b.line || (a.callee < b.callee ? -1 : a.callee > b.callee ? 1 : 0));
}

/** 🧭️ The name of a program without directories and `.exe` / `.cmd` / `.bat` suffixes. */
export const processCommandNameV1 = (program: string): string => program.replace(/\\/gu, "/").split("/").pop()!.replace(/\.(?:exe|cmd|bat)$/iu, "");

/** 🧱️ Every glob of the modules that implement the process port: the tools module, each tool's adapter, adapter scope and installer, the registered process funnels and the interfaces of the allowlist. */
export function processInterfaceGlobsV1(registry: ToolRegistryV1, allowlist: ProcessAllowlistV1): readonly string[] {
  const globs = [TOOLS_MODULE];
  for (const tool of registry.tools) globs.push(...tool.adapterScope, tool.adapter, ...(tool.provision.installer === undefined ? [] : [tool.provision.installer]));
  globs.push(...(registry.funnels ?? []).map(funnel => funnel.path), ...allowlist.interfaces.map(row => row.glob));
  return globs;
}

/** ⚖️ Judges the sites: a literal program the registry does not know needs a registry row or an allowlist exception; a process start outside the interface modules (and outside the adapter of its tool) needs an exception; funnel calls with a registered, running or data-driven program are the port working as designed. Exceptions are consumed one per site; a row with unused budget is stale. */
export function judgeProcessStartsV1(sites: readonly ProcessSiteV1[], registry: ToolRegistryV1 = TOOL_REGISTRY_V1, allowlist: ProcessAllowlistV1 = PROCESS_ALLOWLIST_V1): Readonly<{ rows: readonly ProcessVerdictRowV1[]; findings: readonly ProcessFindingV1[]; stale: readonly string[] }> {
  const owner = new Map<string, number>(), scopes = registry.tools.map(tool => [tool.adapterScope, tool.adapter, TOOLS_MODULE, ...(tool.provision.installer === undefined ? [] : [tool.provision.installer])].flat()), interfaces = processInterfaceGlobsV1(registry, allowlist);
  registry.tools.forEach((tool, index) => { for (const command of tool.commands) owner.set(command, index); });
  const budget = new Map<string, number>(), classes = new Map<string, string>();
  for (const row of allowlist.exceptions) { const key = `${row.path}\0${row.program}`; budget.set(key, (budget.get(key) ?? 0) + row.count); classes.set(key, row.class); }
  const left = new Map(budget), rows: ProcessVerdictRowV1[] = [], findings: ProcessFindingV1[] = [], roots = allowlist.roots.map(root => root.replace(/\/+$/u, "") + "/");
  for (const site of sites) {
    if (site.scope !== "prod" || !roots.some(root => site.path.startsWith(root))) continue;
    const literal = site.program !== "<self>" && site.program !== "<dynamic>", name = literal ? processCommandNameV1(site.program) : "", index = literal ? owner.get(name) : undefined;
    const insideInterface = interfaces.some(glob => toolGlobMatchesV1(glob, site.path));
    let disposition = "", rule: "raw-spawn" | "unregistered-command" | undefined;
    if (literal && index === undefined) rule = "unregistered-command";
    else if (site.form === "funnel") disposition = "funnel";
    else if (index !== undefined && scopes[index]!.some(glob => toolGlobMatchesV1(glob, site.path))) disposition = "adapter";
    else if (index === undefined && insideInterface) disposition = "interface";
    else rule = "raw-spawn";
    if (rule !== undefined) {
      const key = `${site.path}\0${site.program}`, remaining = left.get(key) ?? 0;
      if (remaining > 0) { left.set(key, remaining - 1); disposition = `exception:${classes.get(key)}`; }
      else { findings.push({ rule, path: site.path, line: site.line, program: site.program, detail: `${site.callee} ${site.form} of ${site.program} outside the interface modules and without an allowlist row` }); disposition = `FINDING:${rule}`; }
    }
    rows.push({ ...site, disposition });
  }
  const stale = [...left].filter(([, remaining]) => remaining > 0).map(([key, remaining]) => { const [path, program] = key.split("\0"); return `${path} ${program} ${budget.get(key)! - remaining}/${budget.get(key)}`; }).sort();
  return { rows, findings, stale };
}

/** 🩺️ Checks the allowlist document itself: every exception and interface has an English and a German reason and every interface without a wildcard names a listed file. */
export function processAllowlistProblemsV1(files: ReadonlySet<string>, allowlist: ProcessAllowlistV1 = PROCESS_ALLOWLIST_V1): readonly ProcessFindingV1[] {
  const problems: ProcessFindingV1[] = [], found = (path: string, detail: string): void => { problems.push({ rule: "allowlist", path, line: 0, program: "", detail }); };
  for (const row of allowlist.exceptions) if (!row.reason.en || !row.reason.de) found(row.path, `exception ${row.program} has no English and German reason`);
  for (const row of allowlist.interfaces) { if (!row.reason.en || !row.reason.de) found(row.glob, "interface has no English and German reason"); if (!row.glob.includes("*") && !files.has(row.glob)) found(row.glob, "interface matches no file"); }
  return problems;
}

/** 🗒️ Lists the files git knows (tracked and not ignored) below the roots of the allowlist, the set the law reads. */
export function listProcessSpawnFilesV1(root: string, allowlist: ProcessAllowlistV1 = PROCESS_ALLOWLIST_V1): readonly string[] {
  const result = gitSyncV1(["ls-files", "-co", "--exclude-standard", "-z", "--", ...allowlist.roots], { cwd: root, maxBuffer: 512 * 1024 * 1024 });
  if (result.status !== 0) throw new Error(`git-ls-files-failed:${result.stderr.trim()}`);
  return result.stdout.split("\0").filter(Boolean);
}

/** 🔍️ Scans the listed files (comments never count; files over 4 MB, dot files and the registry's ignored directories are skipped). */
export function scanRepositoryProcessStartsV1(root: string, files: readonly string[], registry: ToolRegistryV1 = TOOL_REGISTRY_V1, allowlist: ProcessAllowlistV1 = PROCESS_ALLOWLIST_V1, read: (path: string) => string | undefined = path => { try { return readFileSync(join(root, path), "utf8"); } catch { return undefined; } }): Readonly<{ sites: readonly ProcessSiteV1[]; scanned: number }> {
  const sites: ProcessSiteV1[] = [];
  let scanned = 0;
  for (const path of files) {
    const segments = path.split("/");
    if (languageOf(path, registry) === undefined || segments.some(segment => registry.scan.ignoredSegments.includes(segment)) || segments[segments.length - 1]!.startsWith(".")) continue;
    const text = read(path);
    if (text === undefined || text.length > 4_000_000) continue;
    scanned++;
    sites.push(...scanProcessStartsV1(path, text, registry, allowlist));
  }
  return { sites, scanned };
}

/** 🪫️ Fails when a repository file starts a process outside the interface modules without a documented exception, or names a program the tool registry does not know without one; each finding names the file, the program and the repair. */
export function policyRawProcessSpawnBreaches(repoRoot: string): BreachRecord[] {
  const files = listProcessSpawnFilesV1(repoRoot), scan = scanRepositoryProcessStartsV1(repoRoot, files);
  return [...judgeProcessStartsV1(scan.sites).findings, ...processAllowlistProblemsV1(new Set(files))].map(finding => ({
    id: `raw-process-spawn-${finding.rule}-${finding.program}-${finding.path}`,
    summary: `${finding.rule} ${finding.path}:${finding.line} ${finding.program}: ${finding.detail}`,
    kind: "repo/raw-process-spawn",
    scope: finding.path,
    priority: "high" as const,
    autofixable: false,
    reason: "Programs stay swappable only while every process start goes through the tool registry or the runtime port; a raw start pins the repository to one external program.",
    solution: "Call runToolV1 / runToolSyncV1 with a registered tool id, add the program to the tool registry (repo tools list), or document the operating-system primitive in the allowlist with an English and a German reason.",
  }));
}
