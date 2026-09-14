//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

//#endregion 🧲️Header

//#region 🔌️Adapters
import { countUnifiedLocForFile } from "../../../📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { defineTestAdapter } from "../../../🧪️test/📦️packages/🟦️typescript/🟦️.ts";
//#endregion 🔌️Adapters

//#region 🔬️Reference aggregation
const AGG_MARKUP = "Markup";
const AGG_DATA = "Data";
const AGG_CODE = "Code";
const AGG_TOTAL = "Total";
const CODE = ["TypeScript", "Go", "C#", "Python", "Rust"];
const MARKUP_EXTENSIONS = [".html", ".htm", ".xhtml", ".md", ".markdown", ".mdown", ".mkd", ".mdx", ".mdc", ".svx", ".svxtheme"];
const DATA_EXTENSIONS = [".json", ".jsonc", ".yaml", ".yml", ".toml", ".csv", ".xml", ".ini", ".cfg", ".conf", ".properties", ".editorconfig", ".gitattributes", ".gitmodules"];
const CODE_EXTENSIONS: Record<string, string> = { ".ts": "TypeScript", ".tsx": "TypeScript", ".cts": "TypeScript", ".mts": "TypeScript", ".mtsx": "TypeScript", ".go": "Go", ".cs": "C#", ".py": "Python", ".rs": "Rust" };
const SEPARATOR = String.fromCharCode(1);

type Pair = { added: number; removed: number };
type Row = { loc: number; percent: number; since_prev_loc_percent?: number; wip_percent: number; edited: number; added: number; removed: number };
type Transcript = { logs: Record<string, string>; tracked: Record<string, string[]>; blobs: Record<string, string> };

function extensionOf(path: string): string {
  const base = path.slice(Math.max(path.lastIndexOf("/"), path.lastIndexOf("\\")) + 1);
  const dot = base.lastIndexOf(".");
  return dot < 0 ? "" : base.slice(dot).toLowerCase();
}

function classify(path: string): string | null {
  const extension = extensionOf(path);
  if (extension in CODE_EXTENSIONS) return CODE_EXTENSIONS[extension]!;
  if (MARKUP_EXTENSIONS.includes(extension)) return AGG_MARKUP;
  if (DATA_EXTENSIONS.includes(extension)) return AGG_DATA;
  return null;
}

function skipped(rel: string): boolean {
  const path = rel.replace(/\\/g, "/").replace(/^\.\//, "");
  if (!path || path === ".🧬semio" || path.startsWith(".🧬semio/")) return true;
  return path.split("/").some((segment) => segment && segment !== "." && segment !== ".." && segment.startsWith("."));
}

function unquote(raw: string): string {
  const trimmed = raw.trim();
  if (trimmed.length < 2 || !trimmed.startsWith('"') || !trimmed.endsWith('"')) return raw;
  const inner = trimmed.slice(1, -1);
  const simple: Record<string, number> = { n: 10, t: 9, r: 13, a: 7, b: 8, f: 12, v: 11, '"': 34, "\\": 92 };
  const bytes: number[] = [];
  for (let index = 0; index < inner.length; ) {
    const current = inner[index]!;
    if (current !== "\\") {
      for (const byte of Buffer.from(current, "utf8")) bytes.push(byte);
      index += 1;
      continue;
    }
    index += 1;
    const escape = inner[index]!;
    index += 1;
    if (escape in simple) {
      bytes.push(simple[escape]!);
      continue;
    }
    if (escape >= "0" && escape <= "7") {
      let value = escape.charCodeAt(0) - 48;
      for (let taken = 1; taken < 3 && index < inner.length && inner[index]! >= "0" && inner[index]! <= "7"; taken += 1) {
        value = value * 8 + (inner[index]!.charCodeAt(0) - 48);
        index += 1;
      }
      bytes.push(value & 0xff);
      continue;
    }
    for (const byte of Buffer.from(escape, "utf8")) bytes.push(byte);
  }
  return Buffer.from(bytes).toString("utf8");
}

function resolvePath(field: string): string {
  const collapse = (value: string): string => value.replace(/\/{2,}/g, "/");
  const open = field.indexOf("{");
  const close = field.indexOf("}");
  if (open >= 0 && close > open) {
    const middle = field.slice(open + 1, close);
    const arrow = middle.indexOf(" => ");
    if (arrow >= 0) return unquote(collapse(field.slice(0, open) + middle.slice(arrow + 4) + field.slice(close + 1)));
  }
  const arrow = field.indexOf(" => ");
  return arrow >= 0 ? unquote(field.slice(arrow + 4).trim()) : unquote(field);
}

type Commit = { sha: string; when_unix: number; author: string; author_mail: string; delta: Record<string, Pair> };

function parseNumstat(stdout: string): Commit[] {
  const commits: Commit[] = [];
  let current: Commit | null = null;
  for (const raw of stdout.split("\n")) {
    const line = raw.replace(/\r$/, "");
    if (!line) continue;
    if (line.startsWith("COMMIT")) {
      const fields = line.split("\t");
      if (fields.length < 5) continue;
      if (current) commits.push(current);
      current = { sha: fields[1]!, when_unix: Number.parseInt(fields[4]!, 10), author: fields[2]!, author_mail: fields[3]!, delta: {} };
      continue;
    }
    if (!current) continue;
    const parts = line.split("\t");
    if (parts.length < 3) continue;
    if (parts[0] === "-" || parts[1] === "-") continue;
    const added = Number.parseInt(parts[0]!, 10);
    const removed = Number.parseInt(parts[1]!, 10);
    if (Number.isNaN(added) || Number.isNaN(removed)) continue;
    const path = resolvePath(parts.slice(2).join("\t"));
    if (skipped(path)) continue;
    const bucket = classify(path);
    if (bucket === null) continue;
    const entry = (current.delta[bucket] ??= { added: 0, removed: 0 });
    entry.added += added;
    entry.removed += removed;
  }
  if (current) commits.push(current);
  return commits;
}

function zeroed(): Record<string, Pair> {
  return Object.fromEntries([...CODE, AGG_MARKUP, AGG_DATA].map((bucket) => [bucket, { added: 0, removed: 0 }]));
}

function scanCounts(transcript: Transcript, ref: string): Record<string, number> {
  const counts: Record<string, number> = Object.fromEntries([...CODE, AGG_MARKUP, AGG_DATA].map((bucket) => [bucket, 0]));
  for (const rel of transcript.tracked[ref] ?? []) {
    if (skipped(rel)) continue;
    const bucket = classify(rel);
    if (bucket === null || !(bucket in counts)) continue;
    const body = transcript.blobs[`${ref}${SEPARATOR}${rel}`];
    if (body === undefined) continue;
    counts[bucket] = counts[bucket]! + countUnifiedLocForFile(rel, body);
  }
  return counts;
}

function round2(value: number): number {
  return Math.round(value * 100) / 100;
}

function compose(cumulative: Record<string, Pair>, scan: Record<string, number>, wipDenominator: number): Record<string, Row> {
  const make = (key: string): Row => {
    const pair = cumulative[key] ?? { added: 0, removed: 0 };
    return { loc: scan[key] ?? 0, percent: 0, wip_percent: 0, edited: pair.added + pair.removed, added: pair.added, removed: pair.removed };
  };
  const rows: Record<string, Row> = {};
  const code: Row = { loc: 0, percent: 0, wip_percent: 0, edited: 0, added: 0, removed: 0 };
  for (const language of CODE) {
    const row = make(language);
    rows[language] = row;
    code.loc += row.loc;
    code.added += row.added;
    code.removed += row.removed;
    code.edited += row.edited;
  }
  const markup = make(AGG_MARKUP);
  const data = make(AGG_DATA);
  rows[AGG_MARKUP] = markup;
  rows[AGG_DATA] = data;
  rows[AGG_CODE] = code;
  rows[AGG_TOTAL] = { loc: code.loc + markup.loc + data.loc, percent: 0, wip_percent: 0, edited: code.edited + markup.edited + data.edited, added: code.added + markup.added + data.added, removed: code.removed + markup.removed + data.removed };
  const denominator = rows[AGG_TOTAL]!.loc;
  for (const [key, row] of Object.entries(rows)) row.percent = denominator <= 0 ? 0 : key === AGG_TOTAL ? 100 : Math.round((10000 * row.loc) / denominator) / 100;
  const churn = wipDenominator > 0 ? wipDenominator : rows[AGG_TOTAL]!.edited;
  for (const row of Object.values(rows)) row.wip_percent = churn <= 0 ? 0 : Math.round((10000 * row.edited) / churn) / 100;
  return rows;
}

function sortedRowKeys(rows: Record<string, Row>): string[] {
  return Object.keys(rows)
    .filter((name) => name !== AGG_TOTAL)
    .sort((left, right) => rows[right]!.loc - rows[left]!.loc || left.localeCompare(right))
    .concat(AGG_TOTAL in rows ? [AGG_TOTAL] : []);
}

function markdownTable(title: string, rows: Record<string, Row>): string {
  let out = title === "" ? "" : `### ${title}\n\n`;
  out += "| Category | loc | % | wip% | edited | added | removed |\n| --- | ---: | ---: | ---: | ---: | ---: | ---: |\n";
  for (const name of sortedRowKeys(rows)) {
    const row = rows[name]!;
    out += `| ${name} | ${row.loc} | ${row.percent.toFixed(2)}% | ${row.wip_percent.toFixed(2)}% | ${row.edited} | ${row.added} | ${row.removed} |\n`;
  }
  return out;
}

function cumulativeOf(commits: Commit[]): Record<string, Pair> {
  const cumulative = zeroed();
  for (const commit of commits) {
    for (const [name, pair] of Object.entries(commit.delta)) {
      cumulative[name]!.added += pair.added;
      cumulative[name]!.removed += pair.removed;
    }
  }
  return cumulative;
}

function churnOf(cumulative: Record<string, Pair>): number {
  return Object.values(cumulative).reduce((total, pair) => total + pair.added + pair.removed, 0);
}

function contributorAlias(name: string, email: string): string {
  const combined = email.trim() === "" ? name.trim() : `${name.trim()} <${email.trim()}>`;
  return combined.trim() === "" ? "unknown" : combined;
}

function rfc3339(unix: number): string {
  return new Date(unix * 1000).toISOString().replace(/\.\d{3}Z$/, "Z");
}

function history(transcript: Transcript, commits: Commit[]): unknown[] {
  const running = zeroed();
  const branch = zeroed();
  const entries: { sha: string; date: string; author: string; languages: Record<string, Row> }[] = [];
  for (const commit of commits) {
    for (const [name, pair] of Object.entries(commit.delta)) {
      branch[name]!.added += pair.added;
      branch[name]!.removed += pair.removed;
      running[name]!.added += pair.added;
      running[name]!.removed += pair.removed;
    }
    entries.push({ sha: commit.sha, date: rfc3339(commit.when_unix), author: contributorAlias(commit.author, commit.author_mail), languages: compose(running, scanCounts(transcript, commit.sha), churnOf(branch)) });
  }
  let previous: Record<string, number> | null = null;
  for (const entry of entries) {
    for (const [name, row] of Object.entries(entry.languages)) {
      if (previous !== null) row.since_prev_loc_percent = round2(previous[name] === undefined || previous[name] === 0 ? (row.loc === 0 ? 0 : 100) : (100 * (row.loc - previous[name]!)) / previous[name]!);
    }
    previous = Object.fromEntries(Object.entries(entry.languages).map(([name, row]) => [name, row.loc]));
  }
  return entries;
}
//#endregion 🔬️Reference aggregation

//#region 🧭️Adapter
function read(ctx: { fixtureBytes(uri: string): Uint8Array }): Transcript {
  return JSON.parse(Buffer.from(ctx.fixtureBytes("shared://🎞️git-transcript.json")).toString("utf8")) as Transcript;
}

function snapshot(transcript: Transcript): { snapshot: Record<string, Row> } {
  const commits = parseNumstat(transcript.logs[""] ?? "");
  const cumulative = cumulativeOf(commits);
  return { snapshot: compose(cumulative, scanCounts(transcript, ""), churnOf(cumulative)) };
}

/** 🟦️ The library reference: `countUnifiedLocForFile` is the repository's third implementation of the same unit. */
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "counts-unified-loc-per-tracked-file": {
      oracle: (ctx) => {
        const transcript = read(ctx);
        const rows: Record<string, number> = {};
        for (const path of [...(transcript.tracked[""] ?? [])].sort()) {
          const body = transcript.blobs[`${SEPARATOR}${path}`];
          if (body === undefined) continue;
          rows[path] = countUnifiedLocForFile(path, body);
        }
        return { projection: rows };
      },
    },
    "composes-the-snapshot-table": { oracle: (ctx) => ({ projection: snapshot(read(ctx)) }) },
    "history-stamps-delta-against-previous-row": {
      oracle: (ctx) => {
        const transcript = read(ctx);
        return { projection: history(transcript, parseNumstat(transcript.logs["⛳️wip"] ?? "")) };
      },
    },
    "renders-the-markdown-snapshot-table": {
      oracle: (ctx) => {
        const rows = snapshot(read(ctx)).snapshot;
        return { projection: { markdown: markdownTable("Snapshot", rows), order: sortedRowKeys(rows) } };
      },
    },
  },
});
//#endregion 🧭️Adapter
