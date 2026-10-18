import { normalizeRelative, emojiFold } from "../🛣️path/🟦️.ts";
import { type TaxonomyReferenceAdapter, requireTaxonomyValue } from "../📜️plan-contracts/🟦️.ts";

export interface ReferenceToken {
  readonly adapter: TaxonomyReferenceAdapter;
  readonly structuredLocation: string;
  readonly start: number;
  readonly end: number;
  readonly value: string;
  readonly targetValues?: readonly string[];
  readonly physicalTargets?: readonly string[];
  readonly physicalInterpretation?: "rust-finite-manifest-targets";
  readonly rewriteKind?: "rust-mod" | "rust-path-join" | "python-entrypoint" | "path-prefix" | "artifact-catalog-glob" | "artifact-catalog-prose" | "exact-owner-reference";
  readonly rewriteData?: Readonly<Record<string, string>>;
  readonly unsupportedReason?: string;
}

export let indexedLineContent = "";

export let indexedLineStarts: readonly number[] = [0];

export function lineLocation(content: string, start: number, label: string): string {
  if (indexedLineContent !== content) {
    const starts = [0];
    for (let index = content.indexOf("\n"); index >= 0; index = content.indexOf("\n", index + 1)) starts.push(index + 1);
    indexedLineContent = content;
    indexedLineStarts = starts;
  }
  let low = 0;
  let high = indexedLineStarts.length;
  while (low < high) {
    const middle = (low + high) >>> 1;
    if (requireTaxonomyValue(indexedLineStarts[middle],"indexedLineStarts[middle]") <= start) low = middle + 1;
    else high = middle;
  }
  const line = Math.max(1, low);
  const column = start - requireTaxonomyValue(indexedLineStarts[line - 1],"indexedLineStarts[line - 1]") + 1;
  return `${label}:${line}:${column}@${start}`;
}

export function regexTokens(content: string, adapter: TaxonomyReferenceAdapter, label: string, patterns: readonly RegExp[]): ReferenceToken[] {
  const rows: ReferenceToken[] = [];
  for (const pattern of patterns) {
    for (const match of content.matchAll(pattern)) {
      const value = match[1];
      if (typeof value !== "string" || match.index === undefined) continue;
      const relativeIndex = match[0].indexOf(value);
      const start = match.index + relativeIndex;
      rows.push({ adapter, structuredLocation: lineLocation(content, start, label), start, end: start + value.length, value });
    }
  }
  return rows;
}

function argumentTokens(content: string, fragment: string, fragmentStart: number, adapter: TaxonomyReferenceAdapter, label: string): ReferenceToken[] {
  const rows: ReferenceToken[] = [];
  for (const match of fragment.matchAll(/"([^"]+)"|'([^']+)'|([^\s()[\],;]+)/gu)) {
    if (match.index === undefined) continue;
    const value = match[1] ?? match[2] ?? match[3];
    if (!value || /^(?:=>|PUBLIC|PRIVATE|INTERFACE|EXCLUDE_FROM_ALL)$/u.test(value)) continue;
    const inner = match[0].indexOf(value);
    const start = fragmentStart + match.index + inner;
    rows.push({ adapter, structuredLocation: lineLocation(content, start, label), start, end: start + value.length, value });
  }
  return rows;
}

const EMBEDDED_ARGUMENT_WORD = /[^\s"'`]+/gu;

const EMBEDDED_ESCAPED_ARGUMENT_WORD = /\\(?:u[\dA-Fa-f]{4}|[^])|[^\s"'`\\]+/gu;

const EMBEDDED_ARGUMENT_PATH = /(?:\.\.?\/|(?<![^\s"'`([,;=:@])\/)[^\s\\"'`()\],;]+/gu;

/** 🧩️ Splits one command-like string into argv words, each yielding its path candidate plus the `./`, `../` and word-boundary `/`
 * fragments inside it. A `/` that continues a word (`${workspaceFolder}/x`, `a/b`) is a separator, never a root. With
 * `escaped` the value is a raw JSON string body: every escape sequence (`\"`, `\t`, `\u00e9`, ...) delimits words, so no candidate spans one. */
export function embeddedArgumentTokens(content: string, value: string, valueStart: number, adapter: TaxonomyReferenceAdapter, label: string, escaped: boolean): ReferenceToken[] {
  if (!/\s|(?:^|\s)--?[\w-]+=|\$\{(?:workspaceFolder|workspaceRoot)\}/u.test(value)) return [];
  const rows: ReferenceToken[] = [];
  for (const match of value.matchAll(escaped ? EMBEDDED_ESCAPED_ARGUMENT_WORD : EMBEDDED_ARGUMENT_WORD)) {
    if (match.index === undefined || match[0].startsWith("\\")) continue;
    for (const fragment of match[0].matchAll(EMBEDDED_ARGUMENT_PATH)) {
      const start = valueStart + match.index + fragment.index!;
      rows.push({ adapter, structuredLocation: lineLocation(content, start, label), start, end: start + fragment[0].length, value: fragment[0] });
    }
    let candidate = match[0].replace(/^[[(]+|[\]),;]+$/gu, "");
    let offset = match[0].indexOf(candidate);
    const assignment = candidate.match(/^--?[\w-]+=(.+)$/u);
    if (assignment) {
      offset += candidate.indexOf(requireTaxonomyValue(assignment[1],"assignment[1]"));
      candidate = requireTaxonomyValue(assignment[1],"required argument value");
    }
    const workspace = candidate.match(/^\$\{(?:workspaceFolder|workspaceRoot)\}\/(.+)$/u);
    if (workspace) {
      offset += candidate.indexOf(requireTaxonomyValue(workspace[1],"workspace[1]"));
      candidate = requireTaxonomyValue(workspace[1],"required argument value");
    }
    if (!candidate || /^(?:bun|node|python|python3|go|cargo|nx|run|test|build)$/u.test(candidate)) continue;
    const start = valueStart + match.index + offset;
    rows.push({ adapter, structuredLocation: lineLocation(content, start, label), start, end: start + candidate.length, value: candidate });
  }
  return [...new Map(rows.map((entry) => [`${entry.start}\u0000${entry.end}\u0000${entry.value}`, entry])).values()].sort((left, right) => left.start - right.start || left.value.localeCompare(right.value));
}

function artifactRootForPath(path: string): string | null {
  const segments = normalizeRelative(path).split("/");
  const index = segments.findIndex((segment) => emojiFold(segment) === emojiFold("🗿️artifacts"));
  if (index >= 0 && index + 1 < segments.length) return segments.slice(0, index + 2).join("/");
  const standards = segments.findIndex((segment) => emojiFold(segment) === emojiFold("🏅️standards"));
  return standards > 0 ? segments.slice(0, standards).join("/") : null;
}

const TOML_TABLE_HEADER = /^\s*\[([^\]]+)\]\s*$/gmu;

export function jsonTokens(path: string, content: string, adapter: "json" | "jsonc"): ReferenceToken[] {
  const rows: ReferenceToken[] = [];
  let ordinal = 0;
  const embeddedArgv = /(?:^|\/)(?:launch(?:\.seed)?\.jsonc?|tasks\.json|project\.json|package\.json)$/iu.test(path);
  for (const match of content.matchAll(/"((?:\\.|[^"\\])*)"/g)) {
    if (match.index === undefined) continue;
    const tail = content.slice(match.index + match[0].length).match(/^\s*/)?.[0].length ?? 0;
    const key = content[match.index + match[0].length + tail] === ":";
    let value: string;
    try {
      value = JSON.parse(match[0]) as string;
    } catch {
      continue;
    }
    const raw = requireTaxonomyValue(match[1],"match[1]");
    const start = match.index + 1;
    if (raw === value) rows.push({ adapter, structuredLocation: `${key ? "/@key" : "/@value"}[${ordinal++}]@${start}`, start, end: start + raw.length, value });
    const workspaceGlob = !key && raw === value ? value.match(/^\{workspaceRoot\}\/(.+?)(\/\*\*\/\*[^/]*)$/u) : null;
    if (workspaceGlob) rows.push({ adapter, structuredLocation: `${key ? "/@key" : "/@value"}[${Math.max(0, ordinal - 1)}]/workspace-glob@${start}`, start, end: start + raw.length, value, targetValues: [requireTaxonomyValue(workspaceGlob[1],"workspaceGlob[1]")], rewriteKind: "path-prefix", rewriteData: { prefix: "{workspaceRoot}/", suffix: requireTaxonomyValue(workspaceGlob[2],"workspace suffix") } });
    if (!key && raw !== value && /^\{workspaceRoot\}\/.+\/\*\*/u.test(value)) rows.push({ adapter, structuredLocation: `/@value[${Math.max(0, ordinal - 1)}]/workspace-glob@${start}`, start, end: start + raw.length, value: raw, rewriteKind: "path-prefix", unsupportedReason: "Escaped workspace projection glob has no proven decoded-to-raw offset map" });
    /** 🧷️ An Nx `{workspaceRoot}/<path>` value with no glob wildcard names one concrete file; `{projectRoot}/…` is left alone since it only ever appears as a `**\/*` glob in this repo. */
    const workspaceFile = !key && raw === value ? value.match(/^\{workspaceRoot\}\/([^*]+)$/u) : null;
    if (workspaceFile) rows.push({ adapter, structuredLocation: `${key ? "/@key" : "/@value"}[${Math.max(0, ordinal - 1)}]/workspace-file@${start}`, start, end: start + raw.length, value, targetValues: [requireTaxonomyValue(workspaceFile[1],"workspaceFile[1]")], rewriteKind: "path-prefix", rewriteData: { prefix: "{workspaceRoot}/", suffix: "" } });
    if (!key && raw !== value && /^\{workspaceRoot\}\/[^*]+$/u.test(value)) rows.push({ adapter, structuredLocation: `/@value[${Math.max(0, ordinal - 1)}]/workspace-file@${start}`, start, end: start + raw.length, value: raw, rewriteKind: "path-prefix", unsupportedReason: "Escaped workspace projection file reference has no proven decoded-to-raw offset map" });
    if (!key && embeddedArgv) rows.push(...embeddedArgumentTokens(content, raw, start, adapter, "embedded-argv", true));
  }
  return rows;
}

export function tomlTokens(path: string, content: string): ReferenceToken[] {
  const adapter: TaxonomyReferenceAdapter = "toml";
  const rows: ReferenceToken[] = [], headers = [...content.matchAll(TOML_TABLE_HEADER)];
  for (const match of content.matchAll(/"([^"\r\n]+)"|'([^'\r\n]+)'/gu)) {
    if (match.index === undefined) continue;
    const value = requireTaxonomyValue(match[1] ?? match[2],"quoted TOML value");
    const start = match.index + match[0].indexOf(value);
    const entrypoint = value.match(/^([A-Za-z_]\w*(?:\.[A-Za-z_]\w*)+):([A-Za-z_]\w*)$/u);
    let low = 0, high = headers.length;
    while (low < high) {
      const middle = (low + high) >>> 1;
      if (headers[middle]!.index! + headers[middle]![0].length <= start) low = middle + 1;
      else high = middle;
    }
    const section = low > 0 ? headers[low - 1]![1] : undefined;
    const lineStart = content.lastIndexOf("\n", start - 1) + 1;
    const key = content.slice(lineStart, start).match(/^\s*([A-Za-z0-9_.-]+)\s*=\s*["']/u)?.[1];
    const label = section && key ? `${section}.${key}` : "toml-string";
    rows.push(entrypoint
      ? { adapter, structuredLocation: lineLocation(content, start, "python-entrypoint"), start, end: start + value.length, value, targetValues: [requireTaxonomyValue(entrypoint[1],"entrypoint target")], rewriteKind: "python-entrypoint", rewriteData: { suffix: `:${entrypoint[2]}` } }
      : { adapter, structuredLocation: lineLocation(content, start, label), start, end: start + value.length, value });
  }
  return rows;
}

export function pythonTokens(_path: string, content: string): ReferenceToken[] {
  const rows = regexTokens(content, "python", "python-reference", [/^\s*from\s+([\w.]+)\s+import\s+/gmu, /^\s*import\s+([\w.]+)(?:\s+as\s+\w+)?\s*$/gmu, /\b(?:open|Path|joinpath|files|read_text|read_bytes)\s*\(\s*["']([^"']+)["']/gu, /__file__[^\r\n]*?\/\s*["']([^"']+)["']/gu]);
  return rows;
}

/** 🥒️ Finds single-backtick inline-code spans inside a Gherkin `Feature:`'s free-form description prose — the lines between the `Feature:` header and the first tag/keyword that opens a `Background`/`Scenario`/`Rule`, never inside a step. Mirrors markdownSourceCoordinateSpans's per-line backtick-run discipline (escaped backticks, longest-matching same-length nested runs, no span crossing a line or a blank-line paragraph break) so a description path reads with the same rigor as a frozen Markdown one, but stays live and rewritable here — see 📓️goal-gherkin-report.md in this ticket. */
function gherkinDescriptionInlineCodeSpans(content: string): readonly Readonly<{ start: number; end: number }>[] {
  const feature = content.match(/^Feature:[^\r\n]*(?:\r\n|\r|\n)?/mu);
  if (!feature || feature.index === undefined) return [];
  const descriptionStart = feature.index + feature[0].length;
  const boundary = content.slice(descriptionStart).match(/^[ \t]*(?:@\S|Background:|Scenario(?: Outline| Template)?:|Rule:)/mu);
  const descriptionEnd = boundary?.index === undefined ? content.length : descriptionStart + boundary.index;
  const rows: { start: number; end: number }[] = [];
  let inline = 0;
  for (const match of content.slice(descriptionStart, descriptionEnd).matchAll(/[^\r\n]*(?:\r\n|\r|\n|$)/gu)) {
    const line = match[0].replace(/(?:\r\n|\r|\n)$/u, ""), offset = descriptionStart + match.index!;
    if (!line.trim()) { inline = 0; continue; }
    const runs = [...line.matchAll(/`+/gu)];
    for (let index = 0; index < runs.length; index++) {
      const run = requireTaxonomyValue(runs[index],"runs[index]"), start = run.index!;
      if (inline) { if (requireTaxonomyValue(run[0],"run[0]").length === inline) inline = 0; continue; }
      if ((line.slice(0, start).match(/\\+$/u)?.[0].length ?? 0) % 2) continue;
      const close = runs.findIndex((candidate, candidateIndex) => candidateIndex > index && candidate[0].length === requireTaxonomyValue(run[0],"run[0]").length);
      if (close < 0) { inline = requireTaxonomyValue(run[0],"run[0]").length; continue; }
      if (requireTaxonomyValue(run[0],"run[0]").length === 1) rows.push({ start: offset + start + 1, end: offset + requireTaxonomyValue(runs[close],"runs[close]").index! });
      index = close;
    }
  }
  return rows;
}

export function gherkinTokens(_path: string, content: string): readonly ReferenceToken[] {
  const rows: ReferenceToken[] = [];
  for (const span of gherkinDescriptionInlineCodeSpans(content)) {
    if (rows.some((token) => token.start <= span.start && token.end >= span.end)) continue;
    const value = content.slice(span.start, span.end);
    rows.push({ adapter: "gherkin", structuredLocation: lineLocation(content, span.start, "gherkin-description-inline-code"), start: span.start, end: span.end, value });
  }
  return rows;
}

export function goTokens(path: string, content: string): ReferenceToken[] {
  const rows: ReferenceToken[] = [];
  if (path.toLowerCase().endsWith(".go")) {
    rows.push(...regexTokens(content, "go", "go-import", [/^\s*(?:[\w.]+\s+)?"([^"]+)"\s*$/gmu]));
    for (const match of content.matchAll(/^\s*\/\/go:(?:embed|generate)\s+([^\r\n]+)$/gmu)) if (match.index !== undefined) rows.push(...argumentTokens(content, requireTaxonomyValue(match[1],"match[1]"), match.index + match[0].indexOf(requireTaxonomyValue(match[1],"match[1]")), "go", "go-directive"));
    return rows;
  }
  for (const match of content.matchAll(/\buse\s*\(([\s\S]*?)\)/gu)) if (match.index !== undefined) rows.push(...argumentTokens(content, requireTaxonomyValue(match[1],"match[1]"), match.index + match[0].indexOf(requireTaxonomyValue(match[1],"match[1]")), "go", "go-work-use"));
  for (const match of content.matchAll(/^\s*use\s+([^\r\n(][^\r\n]*)$/gmu)) if (match.index !== undefined) rows.push(...argumentTokens(content, requireTaxonomyValue(match[1],"match[1]"), match.index + match[0].indexOf(requireTaxonomyValue(match[1],"match[1]")), "go", "go-work-use"));
  for (const match of content.matchAll(/^\s*replace\s+[^\r\n=]+=>\s*([^\s]+).*$/gmu)) if (match.index !== undefined) rows.push(...argumentTokens(content, requireTaxonomyValue(match[1],"match[1]"), match.index + match[0].indexOf(requireTaxonomyValue(match[1],"match[1]")), "go", "go-mod-replace"));
  return rows;
}

export function cmakeTokens(content: string): ReferenceToken[] {
  const rows: ReferenceToken[] = [];
  for (const match of content.matchAll(/\b(?:add_subdirectory|add_executable|add_library|target_sources|include|configure_file|set)\s*\(([\s\S]*?)\)/giu)) if (match.index !== undefined) rows.push(...argumentTokens(content, requireTaxonomyValue(match[1],"match[1]"), match.index + match[0].indexOf(requireTaxonomyValue(match[1],"match[1]")), "native", "cmake-argument"));
  return rows;
}

export function htmlTokens(content: string, adapter: "xml" | "markdown"): ReferenceToken[] {
  return regexTokens(content, adapter, "html-attribute", [/<(?:a|img|script|link|source|video|audio|form)\b[^>]*\b(?:href|src|srcset|poster|data|action)\s*=\s*["']([^"']+)["'][^>]*>/giu]);
}

/** 🔗 Collects exact inline destinations with monotonic shared lookaheads. */
export function markdownInlineTokens(content: string): ReferenceToken[] {
  const rows: ReferenceToken[] = [], delimiterPattern = /[)\s]/gu, titlePattern = /\S/gu;
  let cursor = 0, delimiter = -1, titleStart = -1, titleEnd = -1;
  while (cursor < content.length) {
    const open = content.indexOf("[", cursor);
    if (open < 0) break;
    const close = content.indexOf("]", open + 1);
    if (close < 0) break;
    cursor = close + 1;
    if (content[cursor] !== "(") continue;
    const start = cursor + 1;
    if (delimiter < start) {
      delimiterPattern.lastIndex = start;
      delimiter = delimiterPattern.exec(content)?.index ?? content.length;
    }
    if (delimiter === start || delimiter === content.length) continue;
    let end = delimiter;
    if (content[end] !== ")") {
      if (titleStart <= delimiter) {
        titlePattern.lastIndex = delimiter;
        titleStart = titlePattern.exec(content)?.index ?? content.length;
      }
      if (content[titleStart] !== '"') continue;
      if (titleEnd <= titleStart) {
        const next = content.indexOf('"', titleStart + 1);
        titleEnd = next < 0 ? content.length : next;
      }
      if (content[titleEnd + 1] !== ")") continue;
      end = titleEnd + 1;
    }
    rows.push({ adapter: "markdown", structuredLocation: lineLocation(content, start, "markdown-link"), start, end: delimiter, value: content.slice(start, delimiter) });
    cursor = end + 1;
  }
  return rows;
}

const MARKDOWN_WHITESPACE_RUN = /\s*/uy, MARKDOWN_TEXT_RUN = /\S+/uy, MARKDOWN_LINE_TERMINATOR = /[\n\r\u2028\u2029]/gu;

/** 🔖️ Link reference definitions (`[label]: destination`) exactly as `regexTokens` reads `/^\s*\[[^\]]+\]:\s*(\S+)/gmu`, in one linear pass: that
 * pattern's unbounded `\s*` and `[^\]]+` rescan to the next `]` from every line start, which is quadratic on long documents. A line start
 * whose whitespace run ends at the same position as an earlier failed one fails identically, and once no `]` follows a `[`, none ever will. */
export function markdownReferenceDefinitionTokens(content: string): ReferenceToken[] {
  const rows: ReferenceToken[] = [], nextLineStart = (from: number): number => {
    MARKDOWN_LINE_TERMINATOR.lastIndex = from;
    const terminator = MARKDOWN_LINE_TERMINATOR.exec(content);
    return terminator ? terminator.index + 1 : -1;
  };
  for (let lineStart = 0; lineStart >= 0 && lineStart <= content.length;) {
    MARKDOWN_WHITESPACE_RUN.lastIndex = lineStart;
    const label = lineStart + MARKDOWN_WHITESPACE_RUN.exec(content)![0].length;
    if (content[label] !== "[" || label + 1 >= content.length || content[label + 1] === "]") { lineStart = nextLineStart(Math.max(label, lineStart)); continue; }
    const close = content.indexOf("]", label + 1);
    if (close < 0) break;
    if (content[close + 1] !== ":") { lineStart = nextLineStart(label); continue; }
    MARKDOWN_WHITESPACE_RUN.lastIndex = close + 2;
    const destination = close + 2 + MARKDOWN_WHITESPACE_RUN.exec(content)![0].length;
    MARKDOWN_TEXT_RUN.lastIndex = destination;
    const text = MARKDOWN_TEXT_RUN.exec(content);
    if (!text) { lineStart = nextLineStart(label); continue; }
    const whole = content.slice(lineStart, destination + text[0].length), start = lineStart + whole.indexOf(text[0]);
    rows.push({ adapter: "markdown", structuredLocation: lineLocation(content, start, "markdown-link"), start, end: start + text[0].length, value: text[0] });
    lineStart = nextLineStart(destination + text[0].length);
  }
  return rows;
}
