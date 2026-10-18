import { join } from "node:path";
import { existsSync, readFileSync } from "node:fs";
import type { Contributor } from "../👤️contributor/🟦️.ts";
import { getRepoMetaDir } from "../../../🗂️workspaces/🧬️metadata/🟦️.ts";

export type CommitLevel = "prepare-only" | "prepare-and-tag" | "prepare-and-tag-and-squash" | "prepare-and-tag-and-squash-and-push";

export type CommitSteps = { tag: boolean; squash: boolean; push: boolean };

/** 🧭️Parses explicit `ct` / `cs` / `cp` step flags from argv segments. */
export function parseCommitSteps(segments: string[]): CommitSteps {
  const token = segments.join(" ").toLowerCase();
  return {
    tag: /\b(ct|ctag|tag!|\+tag)\b/.test(token),
    squash: /\b(cs|csquash|squash!|\+squash)\b/.test(token),
    push: /\b(cp|cpush|push!|\+push)\b/.test(token),
  };
}

function commitStepsFromLevel(level: CommitLevel): CommitSteps {
  switch (level) {
    case "prepare-and-tag":
      return { tag: true, squash: false, push: false };
    case "prepare-and-tag-and-squash":
      return { tag: true, squash: true, push: false };
    case "prepare-and-tag-and-squash-and-push":
      return { tag: true, squash: true, push: true };
    default:
      return { tag: false, squash: false, push: false };
  }
}

export function loadCommitSteps(root: string, contributor: Contributor, segments: string[]): CommitSteps {
  const explicit = parseCommitSteps(segments);
  if (explicit.tag || explicit.squash || explicit.push) return explicit;
  const path = join(getRepoMetaDir(root), "🧑️‍💻️devs", contributor.alias, "commit.json");
  if (existsSync(path)) {
    const j = JSON.parse(readFileSync(path, "utf8")) as { level?: string };
    const allowed: CommitLevel[] = ["prepare-only", "prepare-and-tag", "prepare-and-tag-and-squash", "prepare-and-tag-and-squash-and-push"];
    if (allowed.includes(j.level as CommitLevel)) return commitStepsFromLevel(j.level as CommitLevel);
  }
  return { tag: false, squash: false, push: false };
}

export function isCommitPrepareOnly(segments: string[]): boolean {
  const token = segments.join(" ").toLowerCase();
  if (/\b(c\.|cprepare|prepare!|\+prepare)\b/.test(token)) return true;
  const steps = parseCommitSteps(segments);
  return !steps.tag && !steps.squash && !steps.push;
}
