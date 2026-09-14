//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

//#endregion 🧲️Header

//#region 🔌️Adapters
import { spawnSync } from "node:child_process";
import { mkdirSync, rmSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { tmpdir } from "node:os";
import { defineTestAdapter, type AdapterContext } from "../../../🧪️test/📦️packages/🟦️typescript/🟦️.ts";
//#endregion 🔌️Adapters

//#region 🌿️Git
/** 🌿️ The oracle: the machine's own git, invoked directly. No provider code is on this path. */
function git(root: string, args: readonly string[]): { stdout: string; stderr: string; status: number } {
  const outcome = spawnSync("git", [...args], { cwd: root, encoding: "utf8" });
  return { stdout: outcome.stdout ?? "", stderr: outcome.stderr ?? "", status: outcome.status ?? 127 };
}

/**
 * 🧪️ A throwaway repository for one scenario. Deliberately not under `ctx.workDir`: that directory
 * lives inside this repository's own cache, where `git rev-parse` would answer from the surrounding
 * checkout and the not-a-repository scenario could never fail.
 */
function scratchRepo(ctx: AdapterContext, initialised: boolean): string {
  const root = join(tmpdir(), `semio-providers-typescript-${ctx.scenario.id}`);
  rmSync(root, { recursive: true, force: true });
  mkdirSync(root, { recursive: true });
  if (!initialised) return root;
  for (const args of [
    ["init", "-b", "main"],
    ["config", "user.email", "test@test.com"],
    ["config", "user.name", "Test"],
    ["config", "commit.gpgsign", "false"],
  ]) {
    const outcome = git(root, args);
    if (outcome.status !== 0) throw new Error(`git ${args.join(" ")} failed: ${outcome.stderr.trim()}`);
  }
  writeFileSync(join(root, "file.txt"), "hello\n");
  for (const args of [
    ["add", "-A"],
    ["commit", "-m", "initial"],
  ]) {
    const outcome = git(root, args);
    if (outcome.status !== 0) throw new Error(`git ${args.join(" ")} failed: ${outcome.stderr.trim()}`);
  }
  return root;
}

function isObjectName(value: string): boolean {
  return /^[0-9a-fA-F]{40}$/u.test(value);
}

function stagedFiles(root: string): string[] {
  const outcome = git(root, ["diff", "--cached", "--name-only"]);
  if (outcome.status !== 0) throw new Error(`git diff --cached --name-only failed: ${outcome.stderr.trim()}`);
  return outcome.stdout.replace(/\r\n/gu, "\n").split("\n").filter((line) => line !== "");
}

function head(root: string): string {
  const outcome = git(root, ["rev-parse", "HEAD"]);
  if (outcome.status !== 0) throw new Error(`git rev-parse HEAD failed: ${outcome.stderr.trim()}`);
  return outcome.stdout.trim();
}
//#endregion 🌿️Git

//#region 🧭️Adapter
/** 🟦️ The git CLI oracle for the version control case. */
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "a-fresh-repository-reports-its-branch-and-head": {
      oracle: (ctx) => {
        const root = scratchRepo(ctx, true);
        const branch = git(root, ["rev-parse", "--abbrev-ref", "HEAD"]);
        if (branch.status !== 0) throw new Error(`git rev-parse --abbrev-ref HEAD failed: ${branch.stderr.trim()}`);
        return { projection: { branch: branch.stdout.trim(), headIsAnObjectName: isObjectName(head(root)), bothReadsAgree: head(root) === head(root), kind: "git" } };
      },
    },
    "staging-lists-the-added-paths": {
      oracle: (ctx) => {
        const root = scratchRepo(ctx, true);
        const before = stagedFiles(root);
        writeFileSync(join(root, "a.txt"), "a\n");
        writeFileSync(join(root, "b.txt"), "b\n");
        const staged = git(root, ["add", "-A"]);
        if (staged.status !== 0) throw new Error(`git add -A failed: ${staged.stderr.trim()}`);
        return { projection: { before, after: stagedFiles(root).sort() } };
      },
    },
    "a-checkpoint-commits-and-advances-head": {
      oracle: (ctx) => {
        const root = scratchRepo(ctx, true);
        const before = head(root);
        writeFileSync(join(root, "file2.txt"), "world\n");
        // 🧭️The provider stages everything first only when nothing is staged yet, which is the state
        // this scenario starts in — so the oracle stages, then commits, exactly once.
        const added = git(root, ["add", "-A"]);
        if (added.status !== 0) throw new Error(`git add -A failed: ${added.stderr.trim()}`);
        const committed = git(root, ["commit", "-m", "add file2"]);
        if (committed.status !== 0) throw new Error(`git commit failed: ${committed.stderr.trim()}`);
        const checkpoint = head(root);
        return { projection: { headAdvanced: checkpoint !== before, checkpointIsAnObjectName: isObjectName(checkpoint), checkpointIsHead: checkpoint === head(root), stagedAfterCheckpoint: stagedFiles(root) } };
      },
    },
    "reading-outside-a-repository-is-an-error": {
      oracle: (ctx) => {
        const root = scratchRepo(ctx, false);
        return { projection: { branchFailed: git(root, ["rev-parse", "--abbrev-ref", "HEAD"]).status !== 0, checkpointFailed: git(root, ["rev-parse", "HEAD"]).status !== 0 } };
      },
    },
  },
});
//#endregion 🧭️Adapter
