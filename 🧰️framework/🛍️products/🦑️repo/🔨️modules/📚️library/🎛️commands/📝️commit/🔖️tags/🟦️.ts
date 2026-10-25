import { gitRepoRoot } from "../🌳️git/🟦️.ts";
import { spawnCapturedSync } from "../../../🏃️process/🔍️probe/🟦️.ts";
import { gitSpawnEnv } from "../../../🏃️process/🌿️environment/🌳️git/🟦️.ts";
import { BUNDLE_WIP_SUBJECT_RE } from "../🧾️bundle-model/🟦️.ts";
import type { Contributor } from "../👤️contributor/🟦️.ts";
import { pad2 } from "../🔢️counter/🟦️.ts";

/** 🔀️Finds the newest commit whose subject is a bundle/WIP marker (`…🔀️`). */
export function findLastBundleWipCommit(root: string): { sha: string; subject: string } | null {
  root = gitRepoRoot(root);
  const r = spawnCapturedSync("git", ["log", "--format=%H%x00%s%x00", "-n", "500"], {
    cwd: root,
    env: gitSpawnEnv(),
  });
  if (r.status !== 0) return null;
  const parts = r.stdout.toString("utf8").split("\0").filter(Boolean);
  for (let i = 0; i + 1 < parts.length; i += 2) {
    const sha = parts[i]!.trim();
    const subject = parts[i + 1]!.trim();
    if (BUNDLE_WIP_SUBJECT_RE.test(subject)) return { sha, subject };
  }
  return null;
}

/** 🏷️Signed tag name for the micro-commit tip before squash (`…🚩️`). */
export function formatBundleTagName(contributor: Contributor, now = new Date()): string {
  const yy = pad2(now.getFullYear() % 100);
  const mm = pad2(now.getMonth() + 1);
  const dd = pad2(now.getDate());
  return `${contributor.emoji}${contributor.alias}🎆️${yy}🌙️${mm}☀️${dd}🚩️`;
}

/** 📦️Bundle squash commit subject (`…🔀️`). */
export function formatBundleSubject(contributor: Contributor, now = new Date()): string {
  const yy = pad2(now.getFullYear() % 100);
  const mm = pad2(now.getMonth() + 1);
  const dd = pad2(now.getDate());
  return `${contributor.emoji}${contributor.alias}🎆️${yy}🌙️${mm}☀️${dd}🔀️`;
}
