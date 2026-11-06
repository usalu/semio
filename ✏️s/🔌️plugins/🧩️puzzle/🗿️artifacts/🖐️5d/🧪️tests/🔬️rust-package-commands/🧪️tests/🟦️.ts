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

test("the owning project exposes the import acceptance route", () => {
  const project = JSON.parse(readFileSync(resolve(import.meta.dir, "../../../📦️packages/🦀️rust/📋️project.json"), "utf8"));
  expect(project.targets["verify-import-acceptance"].options.command).toBe("repo ./📜️script.ts verify import-acceptance");
});

test("edit acceptance reaches the owning unit test crate", async () => {
  calls.length = 0;
  const invocation = { requestId: "puzzle5d-edit-routing" } as never;
  await runOwnedVerifyScript({ root: "owner", repoRoot: "repository", invocation }, ["edit-acceptance"]);
  expect(calls).toEqual([[fixture.packages, "repository", ["--offline", "--lib", "created_part_board_and_world_edits_reopen_with_history", "--", "--nocapture"], invocation]]);
  const project = JSON.parse(readFileSync(resolve(import.meta.dir, "../../../📦️packages/🦀️rust/📋️project.json"), "utf8"));
  expect(project.targets["verify-edit-acceptance"].options.command).toBe("repo ./📜️script.ts verify edit-acceptance");
});

test("data acceptance routes run the named 5d unit laws", async () => {
  const project = JSON.parse(readFileSync(resolve(import.meta.dir, "../../../📦️packages/🦀️rust/📋️project.json"), "utf8"));
  for (const [command, filter] of [["optional-fields", "optional_part_and_grip_fields_round_trip_against_independent_json"], ["inline-capacity", "inline_import_matches_independent_json_capacity_boundary"], ["catalog-fields", "catalog_extensions_round_trip_against_independent_json"]]) {
    calls.length = 0;
    const invocation = { requestId: command } as never;
    await runOwnedVerifyScript({ root: "owner", repoRoot: "repository", invocation }, [command]);
    expect(calls).toEqual([[fixture.packages, "repository", ["--offline", "--lib", filter, "--", "--nocapture"], invocation]]);
    expect(project.targets[`verify-${command}`].options.command).toBe(`repo ./📜️script.ts verify ${command}`);
  }
});

test("the 5d React owner exposes its browser lifecycle scenario", () => {
  const project = JSON.parse(readFileSync(resolve(import.meta.dir, "../../../../../🎯️targets/⚛️5d-react/📦️packages/🟦️typescript/📋️project.json"), "utf8"));
  const target = project.targets["verify-lifecycle"];
  expect(target.cache).toBe(false);
  expect(target.metadata.semio.cache.class).toBe("live");
  expect(target.options.cwd).toBe("🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript");
  expect(target.options.command).toBe("repo ./📜️script.ts verify time-travel --scenario ✏️s/🔌️plugins/🧩️puzzle/🧪️tests/🖐️5d-lifecycle/🟦️.ts --renderer react --locales en,de");
  expect(target.options.forwardAllArgs).toBe(true);
});
