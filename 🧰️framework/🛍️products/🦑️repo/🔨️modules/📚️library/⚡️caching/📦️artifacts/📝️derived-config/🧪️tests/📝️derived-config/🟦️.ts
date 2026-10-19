import assert from "node:assert/strict";
import { test } from "bun:test";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { createRequire } from "node:module";
import { dirname, join } from "node:path";
import { NEXTEST, NEXTEST_SCOPES, dependabotProblems, deriveNextest, nextestProblems, writeDerivedConfig } from "../../🟦️.ts";

/** 📝️ Validates portable nextest and dependency ownership vectors through independent TOML/YAML parsers, with editor-free generation. */
export async function testDerivedConfig(workspace: string): Promise<void> {
  const require = createRequire(import.meta.url);
  const fixture = JSON.parse(readFileSync(join(import.meta.dir, "../🧫️fixtures/📝️derived-config/🔣️.json"), "utf8"));
  const toml = require("@iarna/toml"), yaml = require("yaml");
  for (const row of fixture.nextest) {
    const derived = deriveNextest(row.root, row.scope), parsed = toml.parse(derived), root = toml.parse(row.root), scope = toml.parse(row.scope);
    assert.deepEqual(Bun.TOML.parse(derived), parsed, row.name);
    assert.deepEqual(parsed.profile.quick["slow-timeout"], root.profile.quick["slow-timeout"], row.name);
    assert.deepEqual(parsed.profile.default, root.profile.default, row.name);
    assert.deepEqual(parsed.profile.quick.overrides ?? [], scope.profile.quick.overrides ?? [], row.name);
    assert.equal((parsed.profile.quick.overrides ?? []).length, row.overrides, row.name);
    assert.equal(deriveNextest(row.root, derived), derived, row.name);
  }
  const output = process.env.SEMIO_TEST_OUTPUT_DIR ?? join(workspace, ".🧬semio/🦑️repo/⚡️cache/tests/derived-config");
  mkdirSync(output, { recursive: true });
  const temporary = mkdtempSync(join(output, "derived-config-"));
  const write = (path: string, text: string): void => { mkdirSync(dirname(join(temporary, path)), { recursive: true }); writeFileSync(join(temporary, path), text); };
  try {
    for (const path of fixture.dependabot.files) write(path, path.endsWith("go.mod") ? "module x\n" : "{}\n");
    write("go.work", fixture.dependabot.goWork);
    for (const [name, source] of [["valid", fixture.dependabot.valid], ["invalid", fixture.dependabot.invalid]] as const) {
      write(".github/dependabot.yml", source);
      assert.deepEqual(Bun.YAML.parse(source), yaml.parse(source), name);
      assert.deepEqual(dependabotProblems(temporary), name === "valid" ? [] : fixture.dependabot.invalidProblems, name);
    }
    const row = fixture.nextest.find((entry: { overrides: number }) => entry.overrides > 0);
    write(NEXTEST, row.root);
    write(NEXTEST_SCOPES, JSON.stringify({ workspaces: [{ directory: "." }, { directory: fixture.generation.scope }] }));
    write(fixture.generation.expectedPaths[0], row.scope);
    for (const path of fixture.generation.untouchedPaths) write(path, "{\"name\":\"dashboard-only\"}\n");
    const before = fixture.generation.untouchedPaths.map((path: string) => readFileSync(join(temporary, path), "utf8"));
    assert.deepEqual(await writeDerivedConfig(temporary), fixture.generation.expectedPaths);
    assert.deepEqual(nextestProblems(temporary), []);
    assert.deepEqual(Bun.TOML.parse(readFileSync(join(temporary, fixture.generation.expectedPaths[0]), "utf8")), toml.parse(deriveNextest(row.root, row.scope)));
    assert.deepEqual(await writeDerivedConfig(temporary), []);
    assert.deepEqual(fixture.generation.untouchedPaths.map((path: string) => readFileSync(join(temporary, path), "utf8")), before);
    for (const path of fixture.generation.absentPaths) assert.equal(existsSync(join(temporary, path)), false, path);
  } finally { rmSync(temporary, { recursive: true, force: true }); }
}

test("derived configuration remains editor-free and agrees with third-party parsers", async () => { await testDerivedConfig(process.cwd()); });
