//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

//#endregion 🧲️Header

//#region 🔌️Adapters
import { readFileSync } from "node:fs";
import { defineTestAdapter, type AdapterContext } from "../../../🧪️test/📦️packages/🟦️typescript/🟦️.ts";
//#endregion 🔌️Adapters

//#region 🛡️Policy
/**
 * 🛡️ A second implementation of the blocking policy, written from the rules stated in `🥒️.feature`
 * and in `🧬️schema/🔣️.json` rather than from the Rust source. It deliberately uses a different
 * strategy — the platform's regular expression engine where the subject hand-rolls a scanner — so a
 * shared mistake in one scanner cannot hide behind the other.
 */
const BLOCKED_GIT_VERBS = ["add", "branch", "checkout", "cherry-pick", "clone", "commit", "config", "fetch", "init", "merge", "mv", "pull", "push", "rebase", "remote", "reset", "restore", "revert", "rm", "stash", "switch", "tag", "clean"] as const;

const BLOCKED_TOOL_PATTERNS = new Set(["rm", "rmdir", "mv", "cp", ...BLOCKED_GIT_VERBS.filter((verb) => verb !== "clean").map((verb) => `git ${verb}`), "git clean"]);

const CONCURRENCY_NOTE = "; other developers and agents may be editing the same files concurrently";

const ALLOWED_PREFIXES = ["grep ", "rg ", "ripgrep ", "echo ", "printf ", "ls ", "pwd", "cat ", "sed ", "awk "];

const SHELL_INTERPRETERS = new Set(["bash", "sh", "zsh", "fish", "ksh", "csh", "tcsh", "dash"]);

const SCRIPT_INTERPRETERS = new Set(["python", "python3", "python2", "node", "nodejs", "perl", "ruby", "php", "lua", "tclsh", "groovy", "scala"]);

const VERB_ALTERNATION = BLOCKED_GIT_VERBS.join("|");

const KILL_LSOF_PORT = /\bkill\b[\s\S]*\$\(\s*lsof\b[\s\S]*-t[\s\S]*-i\s*:\s*\d+[\s\S]*\)/iu;

const INLINE_GIT_VERB = new RegExp(String.raw`\bgit\s+(${VERB_ALTERNATION})\b`, "iu");

const INLINE_GIT_LIST = new RegExp(String.raw`['"]\s*git\s*['"]\s*,\s*['"]\s*(${VERB_ALTERNATION})\s*['"]`, "iu");

/** ✂️ Splits at every `;`, `&&`, `|` and `||` that is not inside a quoted run. */
function splitCommandSegments(command: string): string[] {
  const trimmed = command.trim();
  if (trimmed === "") return [];
  const segments: string[] = [];
  let current = "";
  let quote = "";
  let index = 0;
  const flush = (): void => {
    const segment = current.trim();
    if (segment !== "") segments.push(segment);
    current = "";
  };
  while (index < trimmed.length) {
    const character = trimmed[index] as string;
    if (quote !== "") {
      current += character;
      if (character === quote) quote = "";
      index += 1;
      continue;
    }
    if (character === "'" || character === '"') {
      quote = character;
      current += character;
      index += 1;
      continue;
    }
    if (character === ";") {
      flush();
      index += 1;
      continue;
    }
    if (character === "&" && trimmed[index + 1] === "&") {
      flush();
      index += 2;
      continue;
    }
    if (character === "|") {
      flush();
      index += trimmed[index + 1] === "|" ? 2 : 1;
      continue;
    }
    current += character;
    index += 1;
  }
  flush();
  return segments;
}

/** 🌿️ Scans arbitrary inline code for the shell form and the list form of a blocked git call. */
function containsBlockedGitInCode(code: string): string | null {
  const shellForm = INLINE_GIT_VERB.exec(code);
  if (shellForm !== null) return `blocked: ${shellForm[0].trim().toLowerCase()}`;
  if (INLINE_GIT_LIST.test(code)) return "blocked: git (list form) in inline code";
  return null;
}

/** 🐙️ The reason one segment is refused, or `null`. */
function segmentRefusal(segment: string): string | null {
  const trimmed = segment.trim();
  if (trimmed === "") return null;
  const lower = trimmed.toLowerCase();
  if (KILL_LSOF_PORT.test(lower)) return "blocked: kill $(lsof -t -i:PORT); this can match PID 1 in containers and terminate the devcontainer, stopping all running work";
  if (ALLOWED_PREFIXES.some((prefix) => lower.startsWith(prefix))) return null;
  let tokens = lower.split(/\s+/u).filter((token) => token !== "");
  for (;;) {
    const first = tokens[0];
    if (first === undefined) return null;
    if (first.includes("=") || first === "env" || first === "command" || first === "sudo") {
      tokens = tokens.slice(1);
      continue;
    }
    if (SHELL_INTERPRETERS.has(first) || first === "xargs") {
      const joined = tokens.join(" ");
      const at = joined.indexOf("git ");
      return at < 0 ? null : segmentRefusal(joined.slice(at));
    }
    if (SCRIPT_INTERPRETERS.has(first)) return containsBlockedGitInCode(tokens.slice(1).join(" "));
    break;
  }
  const program = (tokens[0] as string).endsWith("/git") ? "git" : (tokens[0] as string);
  if (program !== "git") return null;
  let verbIndex = 1;
  while (verbIndex < tokens.length && (tokens[verbIndex] as string).startsWith("-")) {
    verbIndex += 1;
    const previous = tokens[verbIndex - 1] as string;
    if (verbIndex < tokens.length && !(tokens[verbIndex] as string).startsWith("-") && (previous === "-c" || previous === "-C")) verbIndex += 1;
  }
  if (verbIndex >= tokens.length) return null;
  const verb = (tokens[verbIndex] as string).replace(/^["']+|["']+$/gu, "");
  if (!(BLOCKED_GIT_VERBS as readonly string[]).includes(verb)) return null;
  if (verb === "clean" && !lower.includes("-fd") && !lower.includes("-df")) return null;
  return `blocked: git ${verb}`;
}

/** ✔️ Whether an invocation is refused, and why. */
function isToolBlocked(tool: string, args: string): string | null {
  for (const segment of splitCommandSegments(args)) {
    const reason = segmentRefusal(segment);
    if (reason !== null) return `${reason}${CONCURRENCY_NOTE}`;
  }
  const name = tool.trim().toLowerCase();
  if (BLOCKED_TOOL_PATTERNS.has(name)) return `blocked: ${name}${CONCURRENCY_NOTE}`;
  return null;
}
//#endregion 🛡️Policy

//#region 🧭️Adapter
type Invocation = { id: string; tool: string; args: string; blocked: boolean };
type Vectors = { invocations: Invocation[]; segments: string[]; inlineCode: string[] };

function vectors(ctx: AdapterContext): Vectors {
  return JSON.parse(readFileSync(ctx.fixture("local://🛡️invocations.json"), "utf8")) as Vectors;
}

function verdict(reason: string | null): { blocked: boolean; reason: string } {
  return { blocked: reason !== null, reason: reason ?? "" };
}

/** 🟦️ The independently written blocking policy oracle. */
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "every-invocation-is-judged-the-same-way": {
      oracle: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).invocations.map((entry) => [entry.id, verdict(isToolBlocked(entry.tool, entry.args))])) }),
    },
    "the-verdict-matches-the-pinned-specification": {
      oracle: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).invocations.map((entry) => [entry.id, (isToolBlocked(entry.tool, entry.args) !== null) === entry.blocked])) }),
    },
    "a-command-splits-into-the-same-segments": {
      oracle: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).segments.map((command) => [command, splitCommandSegments(command)])) }),
    },
    "inline-code-is-scanned-the-same-way": {
      oracle: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).inlineCode.map((code) => [code, verdict(containsBlockedGitInCode(code))])) }),
    },
    "refusing-is-idempotent-and-order-free": {
      oracle: (ctx) => {
        const projection: Record<string, unknown> = {};
        for (const entry of vectors(ctx).invocations) {
          if (isToolBlocked(entry.tool, entry.args) === null || entry.args === "") continue;
          const appended = isToolBlocked(entry.tool, `${entry.args} && echo done`);
          const prepended = isToolBlocked(entry.tool, `echo start && ${entry.args}`);
          projection[entry.id] = { appendedStillRefused: appended !== null, prependedStillRefused: prepended !== null, appendedReason: appended ?? "", prependedReason: prepended ?? "" };
        }
        return { projection };
      },
    },
  },
});
//#endregion 🧭️Adapter
