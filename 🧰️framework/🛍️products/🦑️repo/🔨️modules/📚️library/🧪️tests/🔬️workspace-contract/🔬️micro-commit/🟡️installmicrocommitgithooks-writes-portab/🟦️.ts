/** 🟡️ The installmicrocommitgithooks writes portable hooks tests of the micro-commit suite. */
import { expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { chmodSync, existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { gitSpawnEnv } from "../../../../🏃️process/🌿️environment/🌳️git/🟦️.ts";
import { gitDirFor } from "../🔵️shared/🟦️.ts";

/** 🟢 Registers the installmicrocommitgithooks writes portable hooks tests of the micro-commit suite. */
export function registerInstallmicrocommitgithooksWritesPortableHooksTests(): void {
  test("installMicroCommitGitHooks writes portable hooks through the native repo boundary", async () => {
    const { installMicroCommitGitHooks, renderMicroCommitGitHook } = await import("../../../../🎛️commands/📝️commit/🪝️hooks/🟦️.ts");
    const { mkdtempSync, rmSync } = await import("node:fs");
    const { tmpdir } = await import("node:os");
    const { join } = await import("node:path");
    const { spawnSync } = await import("node:child_process");
    const root = mkdtempSync(join(tmpdir(), "compose-micro-commit-"));
    try {
      mkdirSync(join(root, ".git"));
      installMicroCommitGitHooks(root);
      const hook = readFileSync(join(root, ".git/hooks/post-commit"), "utf8");
      expect(hook).toContain("compose_micro_commit_wipe");
      expect(hook).not.toContain("\r");
      expect(hook).toContain("native-repo-path");
      expect(renderMicroCommitGitHook("prepare-commit-msg")).toContain('"$CLI" micro-commit prepare-commit-msg "$1" "$2"');
      expect(hook).not.toContain("compose_resolve_bun");
      expect(renderMicroCommitGitHook("post-commit")).toContain("#!/usr/bin/env sh");
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });
  test("prepare-commit-msg lets a manual commit complete when template refresh fails", async () => {
    const { renderMicroCommitGitHook } = await import("../../../../🎛️commands/📝️commit/🪝️hooks/🟦️.ts");
    const root = mkdtempSync(join(tmpdir(), "compose-micro-commit-hook-"));
    try {
      expect(spawnSync("git", ["init"], { cwd: root, encoding: "utf8", env: gitSpawnEnv() }).status).toBe(0);
      expect(spawnSync("git", ["config", "user.email", "hook@example.com"], { cwd: root, encoding: "utf8", env: gitSpawnEnv() }).status).toBe(0);
      expect(spawnSync("git", ["config", "user.name", "Hook"], { cwd: root, encoding: "utf8", env: gitSpawnEnv() }).status).toBe(0);
      expect(spawnSync("git", ["config", "commit.gpgsign", "false"], { cwd: root, encoding: "utf8", env: gitSpawnEnv() }).status).toBe(0);
      const hook = join(root, ".git", "hooks", "prepare-commit-msg");
      const brokenRepo = join(root, "broken-repo");
      writeFileSync(hook, renderMicroCommitGitHook("prepare-commit-msg"));
      writeFileSync(brokenRepo, "#!/usr/bin/env sh\nexit 1\n");
      chmodSync(hook, 0o755);
      chmodSync(brokenRepo, 0o755);
      writeFileSync(join(root, ".git", "compose-micro-commit-active"), "1\n");
      writeFileSync(join(root, "tracked.txt"), "tracked\n");
      expect(spawnSync("git", ["add", "tracked.txt"], { cwd: root, encoding: "utf8", env: gitSpawnEnv() }).status).toBe(0);
      const commit = spawnSync("git", ["commit", "-m", "Manual Commit"], { cwd: root, encoding: "utf8", env: { ...gitSpawnEnv(), REPO_CLI_BIN: brokenRepo } });
      expect(commit.status, commit.stderr).toBe(0);
      expect(spawnSync("git", ["log", "-1", "--format=%s"], { cwd: root, encoding: "utf8", env: gitSpawnEnv() }).stdout?.trim()).toBe("Manual Commit");
      expect(existsSync(join(root, ".git", "compose-micro-commit-active"))).toBe(false);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });
  test("handlePrepareCommitMsg inactive does not clear commit message file", async () => {
    const { handlePrepareCommitMsg } = await import("../../../../🎛️commands/📝️commit/📄️prepared-message/🟦️.ts");
    const { mkdtempSync, writeFileSync, readFileSync, rmSync } = await import("node:fs");
    const { tmpdir } = await import("node:os");
    const { join } = await import("node:path");
    const { spawnSync } = await import("node:child_process");
    const root = mkdtempSync(join(tmpdir(), "compose-micro-commit-"));
    try {
      expect(spawnSync("git", ["init"], { cwd: root, encoding: "utf8" }).status).toBe(0);
      const msgFile = join(root, ".git", "COMMIT_EDITMSG");
      writeFileSync(msgFile, "🐙️ueli manual subject\n", "utf8");
      handlePrepareCommitMsg(root, msgFile, "template");
      expect(readFileSync(msgFile, "utf8")).toContain("manual subject");
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });
  test("wipeAfterCommit clears all GK templates and prepare state", async () => {
    const { wipeAfterCommit, writeMicroCommitTemplates } = await import("../../../../🎛️commands/📝️commit/📄️prepared-message/🟦️.ts");
    const { mkdtempSync, writeFileSync, readFileSync, rmSync, existsSync, readdirSync } = await import("node:fs");
    const { tmpdir } = await import("node:os");
    const { join } = await import("node:path");
    const { spawnSync } = await import("node:child_process");
    const root = mkdtempSync(join(tmpdir(), "compose-micro-commit-wipe-"));
    const prevRoot = process.env.REPO_ROOT;
    const prevGitDir = process.env.GIT_DIR;
    const prevWorkTree = process.env.GIT_WORK_TREE;
    delete process.env.REPO_ROOT;
    delete process.env.GIT_DIR;
    delete process.env.GIT_WORK_TREE;
    try {
      expect(spawnSync("git", ["init"], { cwd: root, encoding: "utf8", env: gitSpawnEnv() }).status).toBe(0);
      spawnSync("git", ["config", "user.email", "u@example.com"], { cwd: root, env: gitSpawnEnv() });
      spawnSync("git", ["config", "user.name", "U"], { cwd: root, env: gitSpawnEnv() });
      const msg = "🐙️ueli🎆️26🌙️08☀️17🚩️001\n\n🎆️test reset\n";
      const gitDir = gitDirFor(root);
      writeMicroCommitTemplates(root, msg);
      writeFileSync(join(gitDir, "gkcommittemplate-099.txt"), "stale numbered", "utf8");
      wipeAfterCommit(root);
      const gkTemplate = join(gitDir, "gkcommittemplate.txt");
      expect(existsSync(gkTemplate)).toBe(true);
      expect(readFileSync(gkTemplate, "utf8")).toBe("");
      const gkLeft = readdirSync(gitDir).filter((n) => n.startsWith("gkcommittemplate"));
      expect(gkLeft).toEqual(["gkcommittemplate.txt"]);
      expect(existsSync(join(gitDir, "compose-micro-commit-active"))).toBe(false);
      expect(readFileSync(join(gitDir, "COMMIT_EDITMSG"), "utf8")).toBe("");
    } finally {
      if (prevRoot === undefined) delete process.env.REPO_ROOT;
      else process.env.REPO_ROOT = prevRoot;
      if (prevGitDir === undefined) delete process.env.GIT_DIR;
      else process.env.GIT_DIR = prevGitDir;
      if (prevWorkTree === undefined) delete process.env.GIT_WORK_TREE;
      else process.env.GIT_WORK_TREE = prevWorkTree;
      rmSync(root, { recursive: true, force: true });
    }
  });
  test("writeMicroCommitTemplates uses single gkcommittemplate.txt", async () => {
    const { writeMicroCommitTemplates } = await import("../../../../🎛️commands/📝️commit/📄️prepared-message/🟦️.ts");
    const { mkdtempSync, readdirSync, rmSync, readFileSync } = await import("node:fs");
    const { tmpdir } = await import("node:os");
    const { join } = await import("node:path");
    const { spawnSync } = await import("node:child_process");
    const root = mkdtempSync(join(tmpdir(), "compose-micro-commit-tpl-"));
    const prevRoot = process.env.REPO_ROOT;
    const prevGitDir = process.env.GIT_DIR;
    const prevWorkTree = process.env.GIT_WORK_TREE;
    delete process.env.REPO_ROOT;
    delete process.env.GIT_DIR;
    delete process.env.GIT_WORK_TREE;
    try {
      expect(spawnSync("git", ["init"], { cwd: root, encoding: "utf8", env: gitSpawnEnv() }).status).toBe(0);
      const msg = "🐙️ueli🎆️26🌙️08☀️17🚩️001\n\n🎆️bullet\n";
      const gitDir = gitDirFor(root);
      writeMicroCommitTemplates(root, msg);
      const gk = readdirSync(gitDir).filter((n) => n.startsWith("gkcommittemplate"));
      expect(gk).toEqual(["gkcommittemplate.txt"]);
      expect(readFileSync(join(gitDir, "gkcommittemplate.txt"), "utf8")).toContain("🚩️");
    } finally {
      if (prevRoot === undefined) delete process.env.REPO_ROOT;
      else process.env.REPO_ROOT = prevRoot;
      if (prevGitDir === undefined) delete process.env.GIT_DIR;
      else process.env.GIT_DIR = prevGitDir;
      if (prevWorkTree === undefined) delete process.env.GIT_WORK_TREE;
      else process.env.GIT_WORK_TREE = prevWorkTree;
      rmSync(root, { recursive: true, force: true });
    }
  });
  test("shouldRefreshPreparedCommitMessage keeps user edits", async () => {
    const { digestMicroCommitMessage, shouldRefreshPreparedCommitMessage } = await import("../../../../🎛️commands/📝️commit/📄️prepared-message/🟦️.ts");
    const prepared = "line1\nline2\n";
    const digest = digestMicroCommitMessage(prepared);
    expect(shouldRefreshPreparedCommitMessage(prepared, digest)).toBe(true);
    expect(shouldRefreshPreparedCommitMessage("", digest)).toBe(true);
    expect(shouldRefreshPreparedCommitMessage(`${prepared}\nmy edit`, digest)).toBe(false);
  });
}
