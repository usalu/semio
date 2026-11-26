import { expect, test } from "bun:test";
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
  return root;
};
const read = (path: string) => Bun.file(path).arrayBuffer().then(buffer => new Uint8Array(buffer), () => null);

test("a missing generated module names its owning target in English and German", async () => {
  const root = workspace();
  try {
    const failure = await readRustSourceClosureV1(["assets/icons/schema/🦀️.rs"], root, [], read).catch((error: unknown) => error);
    expect(isRepoErrorV1(failure)).toBe(true);
    for (const language of ["en", "de"]) expect(renderRepoErrorV1(failure as never, language)).toContain("@t/assets:build");
    const ungenerated = await readRustSourceClosureV1(["plain/🦀️.rs"], root, [], read).catch((error: unknown) => error);
    expect(isRepoErrorV1(ungenerated) && ungenerated.key).toMatch(/source.unresolved-module$/u);
  } finally { rmSync(root, { recursive: true, force: true }); }
});

test("generate prerequisites cover every missing generated mount and nothing else", async () => {
  const root = workspace();
  try {
    expect(await missingMountedModulesV1(root)).toHaveLength(3);
    expect(await generatedModuleOwnersV1(root, join(root, "ui/styling/🤖️generated/tokens.rs"))).toEqual(["@t/tokens:generate"]);
    expect(await generatedPrerequisiteArgumentsV1(root)).toEqual([["run", "@t/assets:build"], ["run", "@t/tokens:generate"]]);
    write(root, "assets/icons/🤖️generated/🪪️icon-name/🦀️.rs", "");
    expect(await generatedPrerequisiteArgumentsV1(root)).toEqual([["run", "@t/tokens:generate"]]);
  } finally { rmSync(root, { recursive: true, force: true }); }
});

test("generate prerequisites resolve relative to the workspace root and report progress", async () => {
  const root = workspace();
  const stages: string[] = [];
  try {
    const cwd = process.cwd();
    process.chdir(tmpdir());
    try {
      expect(await generatedPrerequisiteArgumentsV1(root, { report: stage => { stages.push(stage); } })).toEqual([["run", "@t/assets:build"], ["run", "@t/tokens:generate"]]);
    } finally { process.chdir(cwd); }
    expect(stages).toEqual(["loading", "running"]);
  } finally { rmSync(root, { recursive: true, force: true }); }
});

test("generate prerequisites stop when the caller cancels the scan", async () => {
  const root = workspace();
  const controller = new AbortController();
  try {
    controller.abort();
    await expect(generatedPrerequisiteArgumentsV1(root, { signal: controller.signal })).rejects.toThrow();
  } finally { rmSync(root, { recursive: true, force: true }); }
});

test("the scan never enters the test scratch root and treats a directory that vanishes mid-walk as empty", async () => {
  const root = workspace();
  try {
    write(root, ".test-scratch/st/x/🦀️.rs", '#[path = "absent-in-scratch.rs"] mod scratch;');
    expect(await missingMountedModulesV1(root)).toHaveLength(3);
    rmSync(join(root, "plain"), { recursive: true, force: true });
    expect(await missingMountedModulesV1(root)).toHaveLength(2);
  } finally { rmSync(root, { recursive: true, force: true }); }
});

test("repeated scans of one root follow every edit made between them", async () => {
  const root = workspace();
  try {
    for (let round = 0; round < 3; round++) {
      expect(await generatedPrerequisiteArgumentsV1(root)).toEqual([["run", "@t/assets:build"], ["run", "@t/tokens:generate"]]);
      write(root, "assets/icons/🤖️generated/🪪️icon-name/🦀️.rs", "");
      expect(await generatedPrerequisiteArgumentsV1(root)).toEqual([["run", "@t/tokens:generate"]]);
      rmSync(join(root, "assets/icons/🤖️generated"), { recursive: true });
      write(root, "ui/📦️packages/📋️project.json", JSON.stringify({ name: "@t/tokens", targets: { generate: { outputs: [`{projectRoot}/../styling/🤖️generated/tokens.rs`] }, other: {} } }));
      write(root, "ui/styling/🤖️generated/tokens.rs", "");
      expect(await generatedPrerequisiteArgumentsV1(root)).toEqual([["run", "@t/assets:build"]]);
      rmSync(join(root, "ui/styling/🤖️generated"), { recursive: true });
      write(root, "ui/📦️packages/📋️project.json", JSON.stringify({ name: "@t/tokens", targets: { generate: { outputs: [] } } }));
    }
  } finally { rmSync(root, { recursive: true, force: true }); }
});
