import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";

/** 🚪️ Holds that artifact Rust `check` runs through the issued compiler launcher, never a raw `cargo`, which the compiler ingress refuses. */
test("artifact rust check is issued through the compiler launcher", () => {
  const source = readFileSync(new URL("../../📦️artifacts/🦀️rust/🟦️.ts", import.meta.url), "utf8");
  const check = source.slice(source.indexOf("class CheckScript"), source.indexOf("class TestScript"));
  expect(check).toContain("runRepositoryCargoCommandV1");
  expect(check).toContain("prepareCargoWorkspaceInvocation");
  expect(check).not.toContain("runRepositoryCommand");
});
