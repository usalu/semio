import { expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { mkdirSync, mkdtempSync, realpathSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { isRepoErrorV1, renderRepoErrorV1 } from "../../../../../🏗️build/🗣️language/🚨️error/🟦️.ts";
import { readRustSourceClosureV1 } from "../../🟦️.ts";
import { generatedModuleOwnersV1, generatedPrerequisiteArgumentsV1, missingMountedModulesV1 } from "../🟦️.ts";

const write = (root: string, path: string, text: string) => { mkdirSync(dirname(join(root, path)), { recursive: true }); writeFileSync(join(root, path), text); };
const workspace = () => {
  const root = realpathSync(mkdtempSync(join(tmpdir(), "generated-owner-")));
  write(root, "assets/📦️packages/📋️project.json", JSON.stringify({ name: "@t/assets", targets: { build: { outputs: ["{projectRoot}/../icons/🤖️generated/🪪️icon-name/🦀️.rs"] } } }));
  write(root, "assets/icons/schema/🦀️.rs", '#[path = "../🤖️generated/🪪️icon-name/🦀️.rs"] mod icon_name;');
  write(root, "ui/📦️packages/📋️project.json", JSON.stringify({ name: "@t/tokens", targets: { generate: { outputs: [] } } }));
  write(root, "ui/styling/🦀️.rs", '#[path = "🤖️generated/tokens.rs"] mod generated;');
  write(root, "plain/🦀️.rs", '#[path = "absent.rs"] mod absent;');
  write(root, ".gitignore", "🤖️generated/\n");
  spawnSync("git", ["init", "-q"], { cwd: root });
  return root;
};
const read = (root: string) => async (path: string) => { try { return new Uint8Array(await Bun.file(path).arrayBuffer()); } catch { return null; } };

test("a missing generated module names its owning target in English and German", async () => {
  const root = workspace();
  try {
    const failure = await readRustSourceClosureV1(["assets/icons/schema/🦀️.rs"], root, [], read(root)).catch((error: unknown) => error);
    expect(isRepoErrorV1(failure)).toBe(true);
    for (const language of ["en", "de"]) expect(renderRepoErrorV1(failure as never, language)).toContain("@t/assets:build");
    const ungenerated = await readRustSourceClosureV1(["plain/🦀️.rs"], root, [], read(root)).catch((error: unknown) => error);
    expect(isRepoErrorV1(ungenerated) && ungenerated.key).toMatch(/source.unresolved-module$/u);
  } finally { rmSync(root, { recursive: true, force: true }); }
});

test("generate prerequisites cover every missing generated mount and nothing else", () => {
  const root = workspace();
  try {
    expect(missingMountedModulesV1(root)).toHaveLength(3);
    expect(generatedModuleOwnersV1(root, join(root, "ui/styling/🤖️generated/tokens.rs"))).toEqual(["@t/tokens:generate"]);
    expect(generatedPrerequisiteArgumentsV1(root)).toEqual([["run", "@t/assets:build"], ["run", "@t/tokens:generate"]]);
    write(root, "assets/icons/🤖️generated/🪪️icon-name/🦀️.rs", "");
    expect(generatedPrerequisiteArgumentsV1(root)).toEqual([["run", "@t/tokens:generate"]]);
  } finally { rmSync(root, { recursive: true, force: true }); }
});
