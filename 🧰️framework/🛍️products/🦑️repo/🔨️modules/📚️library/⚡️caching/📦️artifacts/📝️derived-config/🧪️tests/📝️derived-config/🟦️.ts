import assert from "node:assert/strict";
import { cpSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { createRequire } from "node:module";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { DEVCONTAINER, NEXTEST, NEXTEST_SCOPES, checkDerivedConfig, dependabotProblems, deriveContainerEditor, deriveNextest, describeEditorDrift, expectedContainerEditor, nextestProblems, nextestScopes, readCanonicalEditor, readContainerEditor, replaceJsoncValue, writeDerivedConfig, type EditorOverlay, type EditorSurface } from "../../🟦️.ts";

type EditorCase = { name: string; canonical: EditorSurface; overlay: EditorOverlay; expected: EditorSurface };

/** 📝️ Checks every derivation against its language-agnostic fixture with an independent library for each: RFC 6902 patches
 * (`fast-json-patch`) and `lodash` for the editor block, `jsonc-parser` edits for the in-place JSONC rewrite, `@iarna/toml` for
 * the nextest scopes and `yaml` for dependabot; then proves the checked-in files are derived and that drift is reported. */
export async function testDerivedConfig(workspace: string): Promise<void> {
  const require = createRequire(import.meta.url), fixture = JSON.parse(readFileSync(join(import.meta.dir, "../../🧫️fixtures/📝️derived-config/🔣️.json"), "utf8"));
  const jsonpatch = require("fast-json-patch"), lodash = require("lodash"), jsonc = require("jsonc-parser"), toml = require("@iarna/toml"), yaml = require("yaml");
  const pointer = (key: string): string => `/${key.replaceAll("~", "~0").replaceAll("/", "~1")}`;
  for (const row of fixture.editor as EditorCase[]) {
    const derived = deriveContainerEditor(row.canonical, row.overlay);
    assert.deepEqual(derived, row.expected, row.name);
    const inherited = lodash.cloneDeepWith(row.canonical.settings, (value: unknown) => (typeof value === "string" ? value.replaceAll("${workspaceFolder}", "${containerWorkspaceFolder}") : undefined));
    const operations = [...row.overlay.settings.omit.filter((key) => key in inherited).map((key) => ({ op: "remove", path: pointer(key) })), ...Object.entries(row.overlay.settings.set).map(([key, value]) => ({ op: "add", path: pointer(key), value }))];
    assert.deepEqual(jsonpatch.applyPatch(inherited, operations).newDocument, derived.settings, `${row.name}: RFC 6902 oracle`);
    assert.deepEqual(derived.extensions, lodash.uniq([...lodash.difference(row.canonical.extensions, row.overlay.extensions.omit), ...row.overlay.extensions.add]), `${row.name}: lodash oracle`);
  }
  for (const row of fixture.jsonc) {
    const ours = replaceJsoncValue(row.text, row.path, row.value);
    const oracle = jsonc.applyEdits(row.text, jsonc.modify(row.text, row.path, row.value, { formattingOptions: { insertSpaces: true, tabSize: 2 } }));
    assert.deepEqual(Bun.JSONC.parse(ours), jsonc.parse(oracle), row.name);
    for (const kept of row.preserved) assert.ok(ours.includes(kept), `${row.name}: keeps ${kept}`);
  }
  for (const row of fixture.nextest) {
    const derived = deriveNextest(row.root, row.scope), parsed = toml.parse(derived), root = toml.parse(row.root), scope = toml.parse(row.scope);
    assert.deepEqual(Bun.TOML.parse(derived), parsed, row.name);
    assert.deepEqual(parsed.profile.quick["slow-timeout"], root.profile.quick["slow-timeout"], `${row.name}: root profile wins`);
    assert.deepEqual(parsed.profile.default, root.profile.default, row.name);
    assert.deepEqual(parsed.profile.quick.overrides ?? [], scope.profile.quick.overrides ?? [], `${row.name}: own overrides survive`);
    assert.equal((parsed.profile.quick.overrides ?? []).length, row.overrides, row.name);
    assert.equal(deriveNextest(row.root, derived), derived, `${row.name}: idempotent`);
  }
  const temporary = mkdtempSync(join(tmpdir(), "semio-derived-config-"));
  try {
    for (const path of fixture.dependabot.files) {
      mkdirSync(dirname(join(temporary, path)), { recursive: true });
      writeFileSync(join(temporary, path), path.endsWith("go.mod") ? "module x\n" : "{}\n");
    }
    writeFileSync(join(temporary, "go.work"), fixture.dependabot.goWork);
    for (const [name, source] of [["valid", fixture.dependabot.valid], ["invalid", fixture.dependabot.invalid]] as const) {
      writeFileSync(join(temporary, ".github/dependabot.yml"), source);
      assert.deepEqual(Bun.YAML.parse(source), yaml.parse(source), `${name}: YAML oracle`);
      assert.deepEqual(dependabotProblems(temporary), name === "valid" ? [] : fixture.dependabot.invalidProblems, name);
    }
  } finally {
    rmSync(temporary, { recursive: true, force: true });
  }
  const dependabot = readFileSync(join(workspace, ".github/dependabot.yml"), "utf8");
  assert.deepEqual(Bun.YAML.parse(dependabot), yaml.parse(dependabot), "dependabot.yml: YAML oracle");
  const canonical = readCanonicalEditor(workspace), overlay = JSON.parse(readFileSync(join(workspace, ".devcontainer/editor-overlay.json"), "utf8")) as EditorOverlay;
  for (const key of overlay.settings.omit) assert.ok(key in canonical.settings, `The overlay omits ${key}, which .vscode/settings.json no longer holds`);
  for (const id of overlay.extensions.omit) assert.ok(canonical.extensions.includes(id), `The overlay omits ${id}, which .vscode/extensions.json no longer recommends`);
  assert.deepEqual(checkDerivedConfig(workspace), [], "Checked-in derived configuration must equal its derivation; run: bun nx run workspace:generate-config");
  const expected = expectedContainerEditor(workspace), actual = readContainerEditor(workspace);
  assert.deepEqual(describeEditorDrift(expected, actual), []);
  assert.equal(describeEditorDrift(expected, { ...actual, settings: { ...actual.settings, "editor.formatOnSave": "tampered" } }).length, 1);
  assert.equal(describeEditorDrift(expected, { ...actual, extensions: actual.extensions.slice(1) }).length, 1);
  const copy = mkdtempSync(join(tmpdir(), "semio-derived-write-"));
  try {
    const files = [".vscode/settings.json", ".vscode/extensions.json", ".devcontainer/editor-overlay.json", DEVCONTAINER, NEXTEST, NEXTEST_SCOPES, ...nextestScopes(workspace).map((scope) => `${scope}/${NEXTEST}`)];
    for (const path of files) {
      mkdirSync(dirname(join(copy, path)), { recursive: true });
      cpSync(join(workspace, path), join(copy, path));
    }
    const hub = `${nextestScopes(workspace)[1]}/${NEXTEST}`, container = readFileSync(join(copy, DEVCONTAINER), "utf8");
    writeFileSync(join(copy, DEVCONTAINER), replaceJsoncValue(container, ["customizations", "vscode", "settings"], { "tampered.longer": "x".repeat(4000) }));
    writeFileSync(join(copy, hub), "[profile.quick]\nslow-timeout = { period = \"1s\", terminate-after = 1 }\n\n[[profile.quick.overrides]]\nfilter = 'package(only-this)'\nslow-timeout = { period = \"9s\", terminate-after = 1 }\n");
    assert.equal(describeEditorDrift(expectedContainerEditor(copy), readContainerEditor(copy)).length, 1);
    assert.equal(nextestProblems(copy).length, 1);
    assert.deepEqual(await writeDerivedConfig(copy), [DEVCONTAINER, hub]);
    assert.deepEqual(describeEditorDrift(expectedContainerEditor(copy), readContainerEditor(copy)), []);
    assert.deepEqual(nextestProblems(copy), []);
    assert.equal(Bun.TOML.parse(readFileSync(join(copy, hub), "utf8")).profile.quick.overrides.length, 1);
    assert.deepEqual(await writeDerivedConfig(copy), [], "the writer is idempotent");
    assert.equal(readFileSync(join(copy, DEVCONTAINER), "utf8"), readFileSync(join(workspace, DEVCONTAINER), "utf8").replaceAll("\r\n", "\n"), "the written devcontainer equals the checked-in one");
  } finally {
    rmSync(copy, { recursive: true, force: true });
  }
  console.log("✅️ Devcontainer editor block, nextest scopes and dependabot manifests derive from their canonical sources; fast-json-patch, lodash, jsonc-parser, @iarna/toml and yaml oracles PASS");
}
