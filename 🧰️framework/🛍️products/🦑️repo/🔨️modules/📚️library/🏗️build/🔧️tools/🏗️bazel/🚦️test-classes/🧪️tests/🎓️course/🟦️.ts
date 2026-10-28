/** 🎓️ Laws of the course test shards: the table splits `@teaching/architecture-course:test` into three Bazel shards, the neutral protocol turns Bazel's shard variables into `--shard=i/3` for the Vitest command, and Vitest's own assignment (recorded from the real tool, reproduced by an independent Python model) covers the 17 files of the course config exactly once with sizes that differ by at most one. */
import { expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { existsSync, readFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, resolve } from "node:path";
import { prepareTestCommandV1 } from "../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🚦️protocol/🟦️.ts";
import { bazelTestAttributesV1, bazelTestEnvironmentV1, bazelTestPlanV1 } from "../../🟦️.ts";
import fixture from "./🧫️fixtures/🔣️.json" with { type: "json" };

const COURSE = "@teaching/architecture-course";
const oracle = (files: readonly string[], count: number): string[][] => {
  const run = spawnSync("python3", ["-I", join(import.meta.dir, "🐍️oracle", "🐍️.py")], { input: JSON.stringify({ files, count }), encoding: "utf8" });
  expect(run.status, run.stderr).toBe(0);
  return JSON.parse(run.stdout).shards;
};
const checkoutRoot = (): string => { let here = import.meta.dir; while (!existsSync(join(here, "🧰️framework")) && dirname(here) !== here) here = dirname(here); return here; };

test("the authored table shards the course test three ways and nothing around it", () => {
  const plan = bazelTestPlanV1({ project: COURSE, target: "test", files: 0 });
  expect(plan.shards).toBe(3);
  expect(plan.rule).toBe("course-test-shards");
  expect(bazelTestAttributesV1(plan)).toContainEqual(["shard_count", "3"]);
  for (const other of [{ project: COURSE, target: "test-e2e" }, { project: COURSE, target: "typecheck" }, { project: "@teaching/proctor", target: "test" }, { project: "@semio-tech/repo-library", target: "test" }]) expect(bazelTestPlanV1({ ...other, files: 0 }).shards, JSON.stringify(other)).toBe(1);
});

test("Bazel's shard variables reach the Vitest command as --shard=i/3 and every other tool argument is kept", () => {
  const argv = ["/w/node_modules/vitest/vitest.mjs", "run", "--config", "/w/c.ts", "--configLoader", "native", "--testTimeout", "300000", "--maxWorkers=4"];
  for (const index of [0, 1, 2]) {
    const environment = bazelTestEnvironmentV1({ TEST_SHARD_INDEX: String(index), TEST_TOTAL_SHARDS: "3", TEST_SHARD_STATUS_FILE: "/s/status" });
    expect(environment.SEMIO_TEST_SHARD).toBe(`${index + 1}/3`);
    const prepared = prepareTestCommandV1("node", argv, { SEMIO_TEST_SHARD: environment.SEMIO_TEST_SHARD }, tmpdir());
    expect(prepared.args).toEqual([...argv, `--shard=${index + 1}/3`]);
    expect(prepareTestCommandV1("node", [...argv, "--shard=9/9"], { SEMIO_TEST_SHARD: environment.SEMIO_TEST_SHARD }, tmpdir()).args).toEqual([...argv, "--shard=9/9"]);
  }
});

test("the independent model reproduces what the real Vitest assigned to each shard of the course config", () => {
  const model = oracle(fixture.files, fixture.count);
  expect(model).toEqual(fixture.shards as string[][]);
  expect(model.flat().sort()).toEqual([...fixture.files].sort());
  expect(new Set(model.flat()).size).toBe(fixture.files.length);
});

test("the course config's own test files split into three non-empty shards that cover each file once and differ in size by at most one", () => {
  const config = join(checkoutRoot(), "🎓️teaching/🏛️architecture/📚️course/🧪️tests/🎚️config/🟦️.ts");
  if (!existsSync(config)) return;
  const text = readFileSync(config, "utf8"), include = /include: \[([^\]]*)\]/u.exec(text);
  expect(include).not.toBeNull();
  const files = [...include![1]!.matchAll(/"([^"]+)"/gu)].map(match => match[1]!);
  expect(files.length).toBeGreaterThanOrEqual(3);
  const shards = oracle(files, 3);
  expect(shards.flat().sort()).toEqual([...files].sort());
  expect(Math.max(...shards.map(row => row.length)) - Math.min(...shards.map(row => row.length))).toBeLessThanOrEqual(1);
  expect(Math.min(...shards.map(row => row.length))).toBeGreaterThan(0);
  expect(files.every(file => existsSync(resolve(dirname(config), "../..", file))), "every include names an existing file").toBe(true);
});
