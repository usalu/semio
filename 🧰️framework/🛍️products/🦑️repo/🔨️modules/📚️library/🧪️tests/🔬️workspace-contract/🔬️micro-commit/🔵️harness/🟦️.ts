/** 🔵️ The shared declarations of the micro-commit suite tests. */
import { spawnSync } from "node:child_process";
import { join } from "node:path";
import { gitSpawnEnv } from "../../../../🏃️process/🌿️environment/🌳️git/🟦️.ts";

export function gitDirFor(root: string): string {
  const rel = spawnSync("git", ["rev-parse", "--git-dir"], { cwd: root, encoding: "utf8", env: gitSpawnEnv() }).stdout?.trim() ?? ".git";
  return rel.startsWith("/") ? rel : join(root, rel);
}
