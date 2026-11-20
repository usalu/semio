import { existsSync, readdirSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { gitSyncV1 } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧰️tools/🟦️.ts";
import type { BreachRecord } from "../../🚨️policy/🔖️breach/🟦️.ts";

/** 🔑️ One object key that a JSON object repeats: `path` locates the repeated key as `$.a.b[2].c`, `line` is the 1-based line of the later occurrence and `key` the decoded key. @see ./🧬️schema/🔣️.json */
export type JsonDuplicateKeyV1 = Readonly<{ path: string; line: number; key: string }>;

type FrameV1 = { array: boolean; keys: Set<string>; key: string; index: number; expectKey: boolean };

const IDENTIFIER = /^[A-Za-z_][A-Za-z0-9_]*$/u;
const segment = (key: string): string => (IDENTIFIER.test(key) ? `.${key}` : `[${JSON.stringify(key)}]`);
const decode = (token: string): string => {
  try {
    return JSON.parse(token) as string;
  } catch {
    return token.slice(1, -1);
  }
};

/** 🔎️ Streams over JSON text once, without building a value, and reports every object key that repeats inside one object; `//` and block comments, a byte order mark and syntax errors are tolerated so JSON with comments never throws. Keys compare after unescaping, so a key and its escaped spelling are one key. */
export function findJsonDuplicateKeysV1(text: string): readonly JsonDuplicateKeyV1[] {
  const found: { path: string; offset: number; key: string }[] = [];
  const frames: FrameV1[] = [];
  const structural = /["{}[\],/]/gu;
  const string = /"(?:[^"\\]|\\[\s\S])*"/gyu;
  let index = text.charCodeAt(0) === 0xfeff ? 1 : 0;
  while (index < text.length) {
    structural.lastIndex = index;
    const hit = structural.exec(text);
    if (hit === null) break;
    index = hit.index;
    const char = text[index]!;
    if (char === "/") {
      const block = text[index + 1] === "*";
      if (!block && text[index + 1] !== "/") {
        index++;
        continue;
      }
      const end = block ? text.indexOf("*/", index + 2) : text.indexOf("\n", index);
      index = end < 0 ? text.length : block ? end + 2 : end;
      continue;
    }
    if (char === '"') {
      string.lastIndex = index;
      const token = string.exec(text);
      if (token === null) break;
      const top = frames.at(-1);
      if (top && !top.array && top.expectKey) {
        const key = decode(token[0]);
        if (top.keys.has(key)) found.push({ path: `$${frames.slice(0, -1).map((frame) => (frame.array ? `[${frame.index}]` : segment(frame.key))).join("")}${segment(key)}`, offset: index, key });
        top.keys.add(key);
        top.key = key;
        top.expectKey = false;
      }
      index += token[0].length;
      continue;
    }
    if (char === "{") frames.push({ array: false, keys: new Set(), key: "", index: 0, expectKey: true });
    else if (char === "[") frames.push({ array: true, keys: new Set(), key: "", index: 0, expectKey: false });
    else if (char === "}" || char === "]") frames.pop();
    else if (char === ",") {
      const top = frames.at(-1);
      if (top?.array) top.index++;
      else if (top) top.expectKey = true;
    }
    index++;
  }
  let line = 1;
  let counted = 0;
  return found.map(({ path, offset, key }) => {
    for (let at = text.indexOf("\n", counted); at >= 0 && at < offset; at = text.indexOf("\n", counted)) {
      line++;
      counted = at + 1;
    }
    return { path, line, key };
  });
}

/** 🗂️ The JSON files of the repository: the tracked files from git, or a walk that skips dot directories and installed packages where no git is available. */
export function listJsonFilesV1(root: string): readonly string[] {
  if (existsSync(join(root, ".git"))) {
    const listed = gitSyncV1(["-c", "core.quotepath=false", "ls-files", "-z", "--", "*.json"], { cwd: root, maxBuffer: 512 * 1024 * 1024 });
    if (listed.status !== 0) throw Error(`git-ls-files-failed:${listed.stderr.trim()}`);
    return listed.stdout.split("\0").filter((path) => path !== "" && existsSync(join(root, path)));
  }
  const files: string[] = [];
  const walk = (relative: string): void => {
    for (const entry of readdirSync(join(root, relative), { withFileTypes: true })) {
      if (entry.name.startsWith(".") || entry.name === "node_modules") continue;
      const path = relative === "" ? entry.name : `${relative}/${entry.name}`;
      if (entry.isDirectory()) walk(path);
      else if (entry.isFile() && path.endsWith(".json")) files.push(path);
    }
  };
  walk("");
  return files.sort();
}

/** 🚩️ Every repeated object key of every JSON file below `root`, keyed by repository-relative path. */
export function jsonDuplicateKeyReportV1(root: string, files: readonly string[] = listJsonFilesV1(root)): ReadonlyMap<string, readonly JsonDuplicateKeyV1[]> {
  const report = new Map<string, readonly JsonDuplicateKeyV1[]>();
  for (const path of files) {
    let text: string;
    try {
      text = readFileSync(join(root, path), "utf8");
    } catch {
      continue;
    }
    if (!text.includes('"')) continue;
    const duplicates = findJsonDuplicateKeysV1(text);
    if (duplicates.length > 0) report.set(path, duplicates);
  }
  return report;
}

/** 🚨️ A repeated key is a high-priority breach: JSON readers keep only the last value, so the earlier one is silently shadowed. */
export function policyJsonDuplicateKeyBreaches(repoRoot: string): BreachRecord[] {
  return [...jsonDuplicateKeyReportV1(repoRoot)].flatMap(([path, duplicates]) =>
    duplicates.map((duplicate) => ({
      id: `json-duplicate-key-${path}-${duplicate.path}`,
      summary: `${path}:${duplicate.line} repeats the key ${JSON.stringify(duplicate.key)} at ${duplicate.path}`,
      kind: "repo/json-duplicate-key",
      scope: path,
      line: duplicate.line,
      priority: "high" as const,
      autofixable: false,
      reason: "JSON parsers keep the last value of a repeated key and drop the earlier ones silently, so a copied entry can shadow real data without any error.",
      solution: "Delete or rename one of the repeated keys, keeping the value that is meant to win.",
    })),
  );
}
