//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

//#endregion 🧲️Header

//#region 🔌️Adapters
import { spawnSync } from "node:child_process";
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { defineTestAdapter, digest } from "../../../🧪️test/📦️packages/🟦️typescript/🟦️.ts";
//#endregion 🔌️Adapters

//#region 🧪️Oracle repository
type Person = { name: string; email: string };
type Change = { path: string; content?: string; bytes?: number[]; rename?: string; delete?: boolean };
type Recipe = {
  branch: string;
  commits: { message: string; when: number; author: Person; changes: Change[] }[];
  side: { from: number; branch: string; path: string; content: string; message: string; when: number; author: Person; mergeMessage: string; mergeWhen: number; mergeAuthor: Person };
};

/** 🖥️ Runs the real `git` binary and fails loudly — an oracle that silently degrades proves nothing. */
function git(cwd: string, args: string[], author?: Person, when?: number): string {
  const stamp = when === undefined ? {} : { GIT_AUTHOR_DATE: `${when} +0000`, GIT_COMMITTER_DATE: `${when} +0000` };
  const identity = author === undefined ? {} : { GIT_AUTHOR_NAME: author.name, GIT_AUTHOR_EMAIL: author.email, GIT_COMMITTER_NAME: author.name, GIT_COMMITTER_EMAIL: author.email };
  const result = spawnSync("git", args, { cwd, env: { ...process.env, ...identity, ...stamp }, encoding: "buffer", maxBuffer: 64 * 1024 * 1024 });
  if (result.status !== 0) throw new Error(`git ${args.join(" ")} failed: ${result.stderr?.toString("utf8")}`);
  return result.stdout.toString("utf8");
}

/** 🏗️ Rebuilds the recorded repository from its committed recipe, byte for byte and date for date. */
function buildRepository(recipe: Recipe): string {
  const dir = mkdtempSync(join(tmpdir(), "semio-metrics-oracle-"));
  git(dir, ["init", "--quiet", `--initial-branch=${recipe.branch}`]);
  git(dir, ["config", "user.name", recipe.commits[0]!.author.name]);
  git(dir, ["config", "user.email", recipe.commits[0]!.author.email]);
  git(dir, ["config", "core.quotepath", "true"]);
  git(dir, ["config", "diff.renames", "true"]);
  const shas: string[] = [];
  for (const commit of recipe.commits) {
    for (const change of commit.changes) {
      if (change.delete) {
        git(dir, ["rm", "--quiet", "--", change.path]);
        continue;
      }
      const target = join(dir, change.path);
      mkdirSync(dirname(target), { recursive: true });
      if (change.rename !== undefined) git(dir, ["mv", change.rename, change.path]);
      if (change.bytes !== undefined) writeFileSync(target, Buffer.from(change.bytes));
      else writeFileSync(target, change.content ?? "", "utf8");
      git(dir, ["add", "--", change.path]);
    }
    git(dir, ["commit", "--quiet", "-m", commit.message], commit.author, commit.when);
    shas.push(git(dir, ["rev-parse", "HEAD"]).trim());
  }
  git(dir, ["checkout", "--quiet", "-b", recipe.side.branch, shas[recipe.side.from]!]);
  writeFileSync(join(dir, recipe.side.path), recipe.side.content, "utf8");
  git(dir, ["add", "--", recipe.side.path]);
  git(dir, ["commit", "--quiet", "-m", recipe.side.message], recipe.side.author, recipe.side.when);
  git(dir, ["checkout", "--quiet", recipe.branch]);
  git(dir, ["merge", "--quiet", "--no-ff", "-m", recipe.side.mergeMessage, recipe.side.branch], recipe.side.mergeAuthor, recipe.side.mergeWhen);
  return dir;
}

const PRETTY = "COMMIT%x09%H%x09%aN%x09%aE%x09%at";
const TAIL = ["--numstat", "--first-parent", "--reverse", `--pretty=format:${PRETTY}`];

/** 🎞️ The two log streams and the tracked path list, straight from the git binary. */
function record(recipe: Recipe): { noMerges: string; withMerges: string; tracked: string[] } {
  const dir = buildRepository(recipe);
  try {
    return {
      noMerges: git(dir, ["log", "--no-merges", ...TAIL]),
      withMerges: git(dir, ["log", ...TAIL, recipe.branch]),
      tracked: git(dir, ["ls-files", "-z"]).split("\0").filter(Boolean),
    };
  } finally {
    rmSync(dir, { recursive: true, force: true });
  }
}
//#endregion 🧪️Oracle repository

//#region 🔬️Reference parser
const AGG_MARKUP = "Markup";
const AGG_DATA = "Data";
const WEIGHTS = new Set(["TypeScript", "Go", "C#", "Python", "Rust", AGG_MARKUP, AGG_DATA]);
const MARKUP_EXTENSIONS = [".html", ".htm", ".xhtml", ".md", ".markdown", ".mdown", ".mkd", ".mdx", ".mdc", ".svx", ".svxtheme"];
const DATA_EXTENSIONS = [".json", ".jsonc", ".yaml", ".yml", ".toml", ".csv", ".xml", ".ini", ".cfg", ".conf", ".properties", ".editorconfig", ".gitattributes", ".gitmodules"];
const CODE_EXTENSIONS: Record<string, string> = { ".ts": "TypeScript", ".tsx": "TypeScript", ".cts": "TypeScript", ".mts": "TypeScript", ".mtsx": "TypeScript", ".go": "Go", ".cs": "C#", ".py": "Python", ".rs": "Rust" };

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

/** 🔤️ Decodes git's octal-escaped quoting, which is how a `core.quotepath` repository names emoji. */
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

function resolvePath(field: string): { path: string; rename_from?: string } {
  const collapse = (value: string): string => value.replace(/\/{2,}/g, "/");
  const open = field.indexOf("{");
  const close = field.indexOf("}");
  if (open >= 0 && close > open) {
    const middle = field.slice(open + 1, close);
    const arrow = middle.indexOf(" => ");
    if (arrow >= 0) {
      const prefix = field.slice(0, open);
      const suffix = field.slice(close + 1);
      return { path: unquote(collapse(prefix + middle.slice(arrow + 4) + suffix)), rename_from: unquote(collapse(prefix + middle.slice(0, arrow) + suffix)) };
    }
  }
  const arrow = field.indexOf(" => ");
  if (arrow >= 0) return { path: unquote(field.slice(arrow + 4).trim()), rename_from: unquote(field.slice(0, arrow).trim()) };
  return { path: unquote(field) };
}

function skipped(rel: string): boolean {
  const path = rel.replace(/\\/g, "/").replace(/^\.\//, "");
  if (!path || path === ".🧬semio" || path.startsWith(".🧬semio/")) return true;
  return path.split("/").some((segment) => segment && segment !== "." && segment !== ".." && segment.startsWith("."));
}

type FileRow = { path: string; rename_from?: string; added: number; removed: number; binary: boolean; bucket?: string };
type CommitRow = { sha: string; when_unix: number; author: string; author_mail: string; files: FileRow[]; delta: Record<string, { added: number; removed: number }> };

/** 🧩️ The reference numstat parser, written from the specification the feature states. */
function parseNumstat(stdout: string): CommitRow[] {
  const commits: CommitRow[] = [];
  let current: CommitRow | null = null;
  for (const raw of stdout.split("\n")) {
    const line = raw.replace(/\r$/, "");
    if (!line) continue;
    if (line.startsWith("COMMIT")) {
      const fields = line.split("\t");
      if (fields.length < 5) continue;
      if (current) commits.push(current);
      current = { sha: fields[1]!, when_unix: Number.parseInt(fields[4]!, 10), author: fields[2]!, author_mail: fields[3]!, files: [], delta: {} };
      continue;
    }
    if (!current) continue;
    const parts = line.split("\t");
    if (parts.length < 3) continue;
    const binary = parts[0] === "-" || parts[1] === "-";
    const added = binary ? 0 : Number.parseInt(parts[0]!, 10);
    const removed = binary ? 0 : Number.parseInt(parts[1]!, 10);
    if (!binary && (Number.isNaN(added) || Number.isNaN(removed))) continue;
    const resolved = resolvePath(parts.slice(2).join("\t"));
    const language = classify(resolved.path);
    const bucket = binary || skipped(resolved.path) || language === null || !WEIGHTS.has(language) ? undefined : language;
    if (bucket !== undefined) {
      const entry = (current.delta[bucket] ??= { added: 0, removed: 0 });
      entry.added += added;
      entry.removed += removed;
    }
    const row: FileRow = { path: resolved.path, added, removed, binary };
    if (resolved.rename_from !== undefined) row.rename_from = resolved.rename_from;
    if (bucket !== undefined) row.bucket = bucket;
    current.files.push(row);
  }
  if (current) commits.push(current);
  return commits;
}
//#endregion 🔬️Reference parser

//#region 🧭️Adapter
/** 🎞️ Recording the repository is the expensive half, so one run serves every scenario. */
let recorded: { noMerges: string; withMerges: string; tracked: string[] } | null = null;

function live(ctx: { fixtureBytes(uri: string): Uint8Array }): { noMerges: string; withMerges: string; tracked: string[] } {
  if (recorded === null) recorded = record(JSON.parse(Buffer.from(ctx.fixtureBytes("shared://🌱️repository-recipe.json")).toString("utf8")) as Recipe);
  return recorded;
}

/** 🟦️ The git oracle: every answer comes from the real binary, never from the committed transcript. */
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "recorded-transcript-matches-real-git": {
      oracle: (ctx) => {
        const fresh = live(ctx);
        const shas = parseNumstat(fresh.noMerges).map((commit) => commit.sha);
        return { projection: { noMergesDigest: digest(fresh.noMerges), withMergesDigest: digest(fresh.withMerges), commitCount: shas.length, shas, trackedAtHead: fresh.tracked } };
      },
    },
    "parses-recorded-numstat-stream": { oracle: (ctx) => ({ projection: parseNumstat(live(ctx).noMerges) }) },
    "resolves-renames-and-quoted-paths": { oracle: (ctx) => ({ projection: parseNumstat(live(ctx).noMerges).flatMap((commit) => commit.files) }) },
    "merge-commit-carries-no-file-rows": { oracle: (ctx) => ({ projection: parseNumstat(live(ctx).withMerges) }) },
  },
});
//#endregion 🧭️Adapter
