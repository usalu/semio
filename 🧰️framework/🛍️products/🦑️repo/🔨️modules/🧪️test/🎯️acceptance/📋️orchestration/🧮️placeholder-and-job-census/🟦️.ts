/** 🧮️ The source census of Rust production code: tracked sources, comment stripping, placeholder macros and interactive job declarations. */
import { spawnSync } from "node:child_process";
import { readFileSync } from "node:fs";
import { join } from "node:path";

//#region 🧮️SourceCensus
/** 🗂️ One tracked source file: repo-relative path and whether it is test-only by its path (a `🧪️tests`, `tests`,
 * `🧫️fixtures`, `benches` or `examples` segment). */
export type CensusSource = Readonly<{ path: string; testOnly: boolean }>;

export const TEST_SEGMENT = /(^|\/)(🧪️tests|tests|🧫️fixtures|benches|examples)\//u;

/** 🗂️ Every git-tracked `*.rs` outside the ticket tree, classified. */
export function trackedRustSources(repoRoot: string): CensusSource[] {
  const listed = spawnSync("git", ["ls-files", "-z", "--", "*.rs", ":!.🧬semio"], { cwd: repoRoot, encoding: "utf8", maxBuffer: 1 << 30 });
  if (listed.status !== 0) throw new Error(`git ls-files failed: ${listed.stderr}`);
  return listed.stdout.split("\0").filter(Boolean).map((path) => ({ path, testOnly: TEST_SEGMENT.test(path) }));
}

/** ✂️ The code of one Rust line with line comments and the parts of block comments removed; `inBlock` carries a block
 * comment across lines. String literals are honoured, so a `//` inside a string is code. */
export function rustCodeOfLine(line: string, inBlock: { open: boolean }): string {
  let code = "";
  let inString = false;
  for (let index = 0; index < line.length; index += 1) {
    const char = line[index]!;
    const next = line[index + 1];
    if (inBlock.open) {
      if (char === "*" && next === "/") {
        inBlock.open = false;
        index += 1;
      }
      continue;
    }
    if (inString) {
      code += char;
      if (char === "\\") {
        code += next ?? "";
        index += 1;
      } else if (char === '"') inString = false;
      continue;
    }
    if (char === "/" && next === "/") break;
    if (char === "/" && next === "*") {
      inBlock.open = true;
      index += 1;
      continue;
    }
    if (char === '"') inString = true;
    code += char;
  }
  return code;
}

/** 🚧️ One `unimplemented!(` / `todo!(` placeholder. */
export type PlaceholderHit = Readonly<{ path: string; line: number; macro: "unimplemented" | "todo"; testOnly: boolean; commented: boolean }>;

const PLACEHOLDER = /(unimplemented|todo)!\s*\(/gu;

/** 🚧️ Scans Rust text for placeholder macros, separating code from commented occurrences. */
export function placeholderHitsOfText(path: string, text: string, testOnly: boolean): PlaceholderHit[] {
  const hits: PlaceholderHit[] = [];
  const inBlock = { open: false };
  text.split("\n").forEach((line, index) => {
    const code = rustCodeOfLine(line, inBlock);
    for (const match of line.matchAll(PLACEHOLDER)) hits.push({ path, line: index + 1, macro: match[1] as "unimplemented" | "todo", testOnly, commented: !new RegExp(PLACEHOLDER.source, "u").test(code) });
  });
  return hits;
}

/** 🎛️ One `.action_interactive_job(<"command" | CONST_PATH>, [path::]InteractiveJobClassification::<class>)` declaration. */
export type InteractiveJobDeclaration = Readonly<{ path: string; line: number; command: string; classification: string; testOnly: boolean }>;

const INTERACTIVE_JOB = /\.action_interactive_job\(\s*(?:"([^"]+)"|\*?([A-Za-z_][\w:]*))\s*,\s*(?:[A-Za-z_][\w]*::)*InteractiveJobClassification::(\w+)/gu;

/** 🎛️ Every declared interactive job of Rust text, and how many `.action_interactive_job(` calls it holds. */
export function interactiveJobsOfText(path: string, text: string, testOnly: boolean): { declarations: InteractiveJobDeclaration[]; calls: number; codeCalls: number } {
  const inBlock = { open: false };
  const code = text.split("\n").map((line) => rustCodeOfLine(line, inBlock)).join("\n");
  const declarations = [...code.matchAll(INTERACTIVE_JOB)].map((match) => ({ path, line: code.slice(0, match.index).split("\n").length, command: match[1] ?? match[2]!, classification: match[3]!, testOnly }));
  return { declarations, calls: text.split(".action_interactive_job(").length - 1, codeCalls: code.split(".action_interactive_job(").length - 1 };
}

/** 📊️ The two production censuses over every tracked Rust source, each cross-checked against `git grep` (the oracle):
 * every placeholder line git's own regex engine finds must be one the scanner found, and every file's count of
 * `action_interactive_job(` calls must equal the scanner's. Progress every 2000 files; the signal stops the walk. */
export function runSourceCensus(repoRoot: string, signal: AbortSignal, onProgress: (line: string) => void) {
  const sources = trackedRustSources(repoRoot);
  const placeholders: PlaceholderHit[] = [];
  const jobs: InteractiveJobDeclaration[] = [];
  const callsPerFile = new Map<string, number>();
  const codeCallsPerFile = new Map<string, { calls: number; testOnly: boolean }>();
  for (const [index, source] of sources.entries()) {
    if (signal.aborted) throw new Error("source census cancelled");
    if (index % 2000 === 0) onProgress(`${index}/${sources.length} Rust sources scanned`);
    let text: string;
    try {
      text = readFileSync(join(repoRoot, source.path), "utf8");
    } catch {
      continue;
    }
    if (text.includes("!")) placeholders.push(...placeholderHitsOfText(source.path, text, source.testOnly));
    if (text.includes(".action_interactive_job(")) {
      const found = interactiveJobsOfText(source.path, text, source.testOnly);
      jobs.push(...found.declarations);
      callsPerFile.set(source.path, found.calls);
      codeCallsPerFile.set(source.path, { calls: found.codeCalls, testOnly: source.testOnly });
    }
  }
  const grep = (args: string[]): string => spawnSync("git", ["grep", ...args, "--", "*.rs", ":!.🧬semio"], { cwd: repoRoot, encoding: "utf8", maxBuffer: 1 << 30 }).stdout;
  const oraclePlaceholders = new Set(grep(["-n", "-E", "(unimplemented|todo)![[:space:]]*\\("]).split("\n").filter(Boolean).map((row) => row.split(":").slice(0, 2).join(":")));
  const scannerPlaceholders = new Set(placeholders.map((hit) => `${hit.path}:${hit.line}`));
  const oracleCalls = new Map(grep(["-c", "-F", ".action_interactive_job("]).split("\n").filter(Boolean).map((row) => [row.slice(0, row.lastIndexOf(":")), Number(row.slice(row.lastIndexOf(":") + 1))] as const));
  const placeholderDisagreements = [...oraclePlaceholders].filter((key) => !scannerPlaceholders.has(key)).concat([...scannerPlaceholders].filter((key) => !oraclePlaceholders.has(key)));
  const callDisagreements = [...new Set([...oracleCalls.keys(), ...callsPerFile.keys()])].filter((path) => (oracleCalls.get(path) ?? 0) !== (callsPerFile.get(path) ?? 0));
  const unparsed = [...codeCallsPerFile].filter(([path, entry]) => !entry.testOnly && entry.calls !== jobs.filter((job) => job.path === path).length).map(([path, entry]) => `${path}: ${entry.calls} calls in code, ${jobs.filter((job) => job.path === path).length} classified declarations`);
  onProgress(`${sources.length}/${sources.length} Rust sources scanned`);
  return { files: sources.length, placeholders, jobs, placeholderDisagreements, callDisagreements, unparsed };
}
//#endregion 🧮️SourceCensus
