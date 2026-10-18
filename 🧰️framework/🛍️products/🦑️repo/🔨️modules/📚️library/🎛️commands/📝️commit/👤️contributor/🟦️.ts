import { join } from "node:path";
import { existsSync, readdirSync, readFileSync } from "node:fs";
import { git, gitEmail } from "../🌳️git/🟦️.ts";
import { normalizeEmojiPresentation } from "../🔣️emoji-presentation/🟦️.ts";
import { getRepoMetaDir } from "../../../🗂️workspaces/🧬️metadata/🟦️.ts";

export type MicroCommitLevel = "prepare-only" | "prepare-and-commit" | "prepare-and-commit-and-push";

export type Contributor = { alias: string; emoji: string; name: string; email: string; emails?: string[] };

export type CurrentBranch = { ok: true; name: string } | { ok: false; error: string };

export function currentBranch(root: string): CurrentBranch {
  const result = git(root, ["branch", "--show-current"]);
  if (!result.ok) return { ok: false, error: result.out || "git branch --show-current failed" };
  return { ok: true, name: result.out };
}

export function branchValidationError(command: "micro-commit" | "commit", branch: CurrentBranch): string | null {
  if (!branch.ok) return `${command}: cannot read current branch: ${branch.error}`;
  if (!branch.name) return `${command}: detached HEAD; switch to a branch containing ⛳️wip or 🏗️dev`;
  const normalizedBranch = normalizeEmojiPresentation(branch.name);
  const wipMarker = normalizeEmojiPresentation("⛳️wip");
  const devMarker = normalizeEmojiPresentation("🏗️dev");
  if (!normalizedBranch.includes(wipMarker) && !normalizedBranch.includes(devMarker)) {
    return `${command}: current branch "${branch.name}" must contain ⛳️wip or 🏗️dev`;
  }
  return null;
}

export function branchAllowed(root: string): boolean {
  return branchValidationError("micro-commit", currentBranch(root)) === null;
}

export function findContributor(root: string): Contributor | null {
  const email = gitEmail(root).toLowerCase();
  const dir = join(getRepoMetaDir(root), "🧑️‍💻️devs");
  if (!existsSync(dir)) return null;
  if (email) {
    for (const name of readdirSync(dir, { withFileTypes: true })) {
      if (!name.isDirectory()) continue;
      const path = join(dir, name.name, "🧑️‍💻️contributor.json");
      if (!existsSync(path)) continue;
      const c = JSON.parse(readFileSync(path, "utf8")) as Contributor & { emails?: string[] };
      const emails = [c.email, ...(c.emails ?? [])].filter((e): e is string => typeof e === "string" && e.length > 0).map((e) => e.toLowerCase());
      if (emails.includes(email)) return c;
    }
  }
  const branchName = git(root, ["branch", "--show-current"]).out;
  if (branchName) {
    for (const name of readdirSync(dir, { withFileTypes: true })) {
      if (!name.isDirectory()) continue;
      const path = join(dir, name.name, "🧑️‍💻️contributor.json");
      if (!existsSync(path)) continue;
      const c = JSON.parse(readFileSync(path, "utf8")) as Contributor & { emails?: string[] };
      if (branchName.includes(c.alias) || (c.emoji && branchName.includes(c.emoji))) {
        if (!email && c.email) {
          git(root, ["config", "--local", "user.email", c.email]);
          if (c.name) git(root, ["config", "--local", "user.name", c.name]);
        }
        return c;
      }
    }
  }
  return null;
}

export function loadLevel(root: string, contributor: Contributor, segments: string[]): MicroCommitLevel {
  const token = segments.join(" ").toLowerCase();
  if (/\b(gp|gpush|push!|\+push)\b/.test(token)) return "prepare-and-commit-and-push";
  if (/\b(gc|commit!|\+commit)\b/.test(token)) return "prepare-and-commit";
  if (/\b(g\.|gprepare|prepare!|\+prepare)\b/.test(token)) return "prepare-only";
  const path = join(getRepoMetaDir(root), "🧑️‍💻️devs", contributor.alias, "micro-commit.json");
  if (existsSync(path)) {
    const j = JSON.parse(readFileSync(path, "utf8")) as { level?: string };
    if (j.level === "prepare-and-commit" || j.level === "prepare-and-commit-and-push" || j.level === "prepare-only") {
      return j.level;
    }
  }
  return "prepare-only";
}
