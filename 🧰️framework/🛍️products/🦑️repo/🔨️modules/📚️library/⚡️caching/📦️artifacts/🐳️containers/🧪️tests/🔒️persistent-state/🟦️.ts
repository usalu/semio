import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { createRequire } from "node:module";
import { join } from "node:path";
import { spawnSync } from "node:child_process";

/** 🔒️ Keeps container restarts from deleting persisted source-control and database state. */
export function testContainerPersistentState(workspace: string, native = false): void {
  const fixture = JSON.parse(readFileSync(join(import.meta.dir, "../../🧫️fixtures/🔒️persistent-state/🔣️.json"), "utf8")), source = readFileSync(join(workspace, fixture.path), "utf8");
  const matches = fixture.forbidden.filter((command: string) => source.includes(command));
  assert.deepEqual(matches, createRequire(import.meta.url)("lodash").filter(fixture.forbidden, (command: string) => source.includes(command)));
  assert.deepEqual(matches, fixture.expected, "Container restarts must preserve existing source-control and database state");
  if (native) {
    const syntax = spawnSync("bash", ["--noprofile", "--norc", "-n", join(workspace, fixture.path)], { encoding: "utf8", timeout: 10000 });
    assert.equal(syntax.status, 0, syntax.stderr);
  }
  console.log(`✅️ Container persistent-state contract matches lodash${native ? " and native Bash syntax validation" : ""} PASS`);
}
