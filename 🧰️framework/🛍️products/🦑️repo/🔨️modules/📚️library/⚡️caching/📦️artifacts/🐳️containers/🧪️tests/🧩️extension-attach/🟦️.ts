import assert from "node:assert/strict";
import { mkdtempSync, readFileSync, writeFileSync, utimesSync } from "node:fs";
import { createRequire } from "node:module";
import { join } from "node:path";
import { spawnSync } from "node:child_process";

/** 🧩️ Executes the attach packaging block against native Bash with isolated package/editor doubles. */
export function testExtensionAttach(workspace: string, generated: string, native = process.platform !== "win32"): void {
  const require = createRequire(import.meta.url), fixture = JSON.parse(readFileSync(join(import.meta.dir, "🔣️.json"), "utf8"));
  const source = readFileSync(join(workspace, fixture.path), "utf8"), start = source.indexOf(fixture.start), end = source.indexOf(fixture.end);
  assert.ok(start >= 0 && end > start);
  const block = source.slice(start, end), project = JSON.parse(readFileSync(join(workspace, fixture.project), "utf8"));
  assert.deepEqual(project.targets["build-vsix"].dependsOn, ["build"]);
  assert.deepEqual(project.targets["build-vsix"].outputs, ["{projectRoot}/🧩️repo.vsix"]);
  const expected = [`nx run ${fixture.target}`];
  assert.deepEqual(expected, require("lodash").map([fixture.target], (target: string) => `nx run ${target}`));
  if (native) for (const row of fixture.cases) {
    const root = mkdtempSync(join(generated, `extension-attach-${row.id}-`)), archive = join(root, "🧩️repo.vsix"), entry = join(root, "🟦️extension.ts");
    writeFileSync(join(root, "package.json"), JSON.stringify({ publisher: "fixture", name: "extension" }));
    utimesSync(join(root, "package.json"), row.sourceTime, row.sourceTime);
    writeFileSync(entry, "changed source"); utimesSync(entry, row.sourceTime, row.sourceTime);
    if (row.archive) { writeFileSync(archive, "previous archive"); utimesSync(archive, row.archiveTime, row.archiveTime); }
    const program = `set -e
IDE_CLIS=(fake_ide)
EXTENSION_ID=""
EXTENSION_PUBLISHER=""
EXTENSION_NAME=""
WSL_ERROR="unavailable fixture editor"
flock() { return 0; }
bun() {
  if [ "$1" = "nx" ]; then
    printf '%s\\n' "$*" >> "$FIXTURE_CALLS"
    if [ "$FIXTURE_BUILD_STATUS" != "0" ]; then return "$FIXTURE_BUILD_STATUS"; fi
    printf '%s' 'restored archive' > "$VSIX_PATH"
  elif [[ "$2" == *publisher* ]]; then printf '%s\\n' fixture
  else printf '%s\\n' extension; fi
}
fake_ide() {
  if [ "$1" = "--install-extension" ]; then printf '%s\\n' install >> "$FIXTURE_INSTALLS"
  elif [ "$1" = "--list-extensions" ]; then printf '%s\\n' fixture.extension; fi
}
${block}`;
    const calls = join(root, "calls.log"), installs = join(root, "installs.log");
    writeFileSync(calls, ""); writeFileSync(installs, "");
    const result = spawnSync("bash", ["--noprofile", "--norc", "-c", program], { cwd: root, encoding: "utf8", timeout: 10000, env: { ...process.env, VSCODE_PACKAGE_ROOT: root, VSIX_PATH: archive, INSTALL_LOCK_FILE: join(root, "install.lock"), FIXTURE_CALLS: calls, FIXTURE_INSTALLS: installs, FIXTURE_BUILD_STATUS: String(row.buildStatus) } });
    writeFileSync(join(root, "output.log"), result.stdout + result.stderr);
    assert.equal(result.status, 0, result.stderr);
    assert.deepEqual(readFileSync(calls, "utf8").trim().split("\n").filter(Boolean), expected, row.id);
    assert.equal(readFileSync(installs, "utf8").trim().split("\n").filter(Boolean).length, row.installs, row.id);
  }
  assert.equal((block.match(/bun nx run /g) ?? []).length, 1);
  assert.ok(!block.includes(" -ot ") && !block.includes("needs_rebuild"));
  console.log(`[DEBUG] Extension attach delegates packaging once to Nx and skips stale installation after failure${native ? "; native Bash covers current/stale/missing archives" : ""} PASS`);
}
