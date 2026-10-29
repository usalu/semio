import assert from "node:assert/strict";
import { test } from "bun:test";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { createRequire } from "node:module";
import { dirname, join, posix } from "node:path";
import { Database } from "bun:sqlite";
import { NEXTEST, NEXTEST_SCOPES, dependabotProblems, deriveNextest, nextestProblems, writeNextestConfig } from "../../🟦️.ts";

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
    assert.deepEqual(await writeNextestConfig(temporary), fixture.generation.expectedPaths);
    assert.deepEqual(nextestProblems(temporary), []);
    assert.deepEqual(Bun.TOML.parse(readFileSync(join(temporary, fixture.generation.expectedPaths[0]), "utf8")), toml.parse(deriveNextest(row.root, row.scope)));
    assert.deepEqual(await writeNextestConfig(temporary), []);
    assert.deepEqual(fixture.generation.untouchedPaths.map((path: string) => readFileSync(join(temporary, path), "utf8")), before);
    for (const path of fixture.generation.absentPaths) assert.equal(existsSync(join(temporary, path)), false, path);
  } finally { rmSync(temporary, { recursive: true, force: true }); }
}

test("derived configuration remains editor-free and agrees with third-party parsers", async () => { await testDerivedConfig(process.cwd()); });


test("dependency-update action manifests agree with closed vectors and independent SQLite selection", () => {
  const fixture = JSON.parse(readFileSync(join(import.meta.dir, "../🧫️fixtures/📝️derived-config/🔣️.json"), "utf8")), yaml = createRequire(import.meta.url)("yaml"), database = new Database(":memory:");
  const output = process.env.SEMIO_TEST_OUTPUT_DIR ?? join(process.cwd(), ".cache/test-sandbox");
  mkdirSync(output, { recursive: true });
  database.exec("CREATE TABLE entries(folder TEXT, name TEXT, kind TEXT)");
  const insert = database.prepare("INSERT INTO entries VALUES(?,?,?)"), oracle = database.prepare("SELECT count(*)>0 AS valid FROM entries WHERE kind='file' AND ((?='/' AND ((folder='.' AND name IN('action.yml','action.yaml')) OR (folder='.github/workflows' AND (name GLOB '*.yml' OR name GLOB '*.yaml')))) OR (?<>'/' AND folder=? AND (name GLOB '*.yml' OR name GLOB '*.yaml')))");
  try {
    for (const row of fixture.actions) {
      const root = mkdtempSync(join(output, "actions-")), write = (file:string,text:string) => {mkdirSync(dirname(join(root,file)),{recursive:true});writeFileSync(join(root,file),text);};
      try {
        database.exec("DELETE FROM entries");
        mkdirSync(join(root,row.directory.slice(1)),{recursive:true});
        for(const file of row.files){write(file,"name: Fixture\n");insert.run(posix.dirname(file),posix.basename(file),"file");}
        for(const file of row.directories??[]){mkdirSync(join(root,file),{recursive:true});insert.run(posix.dirname(file),posix.basename(file),"directory");}
        write("go.work","go 1.26\nuse ()\n");
        write(".github/dependabot.yml",yaml.stringify({version:2,updates:[{"package-ecosystem":"github-actions",directory:row.directory,schedule:{interval:"weekly"}}]}));
        const independent = Boolean((oracle.get(row.directory,row.directory,row.directory.slice(1)) as {valid:number}).valid);
        assert.equal(independent,row.valid,row.name+" oracle");
        assert.deepEqual(dependabotProblems(root),independent?[]:[".github/dependabot.yml: github-actions directory "+row.directory+" holds no manifest"],row.name);
      } finally {rmSync(root,{recursive:true,force:true});}
    }
  } finally {database.close();}
});
