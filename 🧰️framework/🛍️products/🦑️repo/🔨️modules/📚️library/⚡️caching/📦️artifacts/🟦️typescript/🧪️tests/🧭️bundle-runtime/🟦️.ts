/** 🧭 Contract: an artifact package whose sources import text through an import attribute is bundled by the repository runtime, because esbuild refuses `with { type: "text" }`. */
import { expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";

const root = resolve(import.meta.dirname, "../../../../../../../../../..");
const git = (...args: string[]): string[] => spawnSync("git", args, { cwd: root, encoding: "utf8" }).stdout.split("\0").filter(Boolean);

test("every artifact package that imports text through an attribute selects the runtime bundler", () => {
  const scripts = git("ls-files", "-z", "--", "*/📦️packages/🟦️typescript/📜️script.ts").filter(path => readFileSync(resolve(root, path), "utf8").includes("runArtifactTypeScriptPackageMain("));
  expect(scripts.length).toBeGreaterThan(10);
  const users = git("grep", "-lzE", "-e", "with *\\{ *type: *\"text\" *\\}", "--", "*.ts");
  const offenders = scripts.filter(script => {
    const owner = dirname(dirname(dirname(script))) + "/";
    return users.some(path => path.startsWith(owner)) && !/bundleRuntime: *"runtime"/u.test(readFileSync(resolve(root, script), "utf8"));
  });
  expect(offenders).toEqual([]);
}, 60_000);
