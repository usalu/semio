import { expect, mock, test } from "bun:test";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import fixture from "../🧫️fixtures/🔣️.json";

const calls: unknown[][] = [];
mock.module("@semio-tech/framework/process/cargo-test", () => ({
  runArtifactRustPackagesTests: async (...args: unknown[]) => { calls.push(args); },
}));
const { runOwnedVerifyScript } = await import("../🟦️.ts");

test("import acceptance reaches the owning unit test crate and forwards invocation", async () => {
  const invocation = { requestId: "puzzle5d-import-routing" } as never;
  await runOwnedVerifyScript({ root: "owner", repoRoot: "repository", invocation }, fixture.command);
  expect(calls).toEqual([[fixture.packages, "repository", fixture.arguments, invocation]]);
});

test("the default Rust test package set includes the owning editor unit test crate", () => {
  const script = readFileSync(resolve(import.meta.dir, "../../../📦️packages/🦀️rust/📜️script.ts"), "utf8");
  const packages = script.match(/packages:\s*(\[[^\]]*\])/u)?.[1];
  expect(packages).toBeDefined();
  expect(JSON.parse(packages!)).toContain(fixture.packages[0]);
});
