/** 🚦️ The exit status of a refused repo command passes through the native bootstrap unchanged: a graph read the fleet did not serve ends `repo run workspace:setup` with 75 (try again later), and the bootstrap, and with it the worker runner, must report 75, not a generic 1. */
import {describe, expect, test} from "bun:test";
import {spawnSync} from "node:child_process";
import {readFileSync} from "node:fs";
import {resolve} from "node:path";

const script = readFileSync(resolve(import.meta.dir, "../../🐚️.sh"), "utf8");
const body = script.slice(script.indexOf("repo_bootstrap() {"), script.indexOf("#endregion 🔖️RepoBootstrap"));
const run = (failing: string, status: number) => spawnSync("/bin/bash", ["-c", `set -euo pipefail
SKIP_REPO_BOOTSTRAP=0; REPO_ROOT=/tmp; LOCAL_BIN=/tmp
log() { :; }
repo() { if [ "$*" = "${failing}" ]; then return ${status}; fi; return 0; }
${body}
repo_bootstrap
echo reached-end`], {encoding: "utf8", env: {PATH: "/usr/bin:/bin"}});

describe("native bootstrap status", () => {
  test("the status of the failing repo command is the status of the bootstrap", () => {
    expect(body.length).toBeGreaterThan(100);
    for (const [command, status] of [["run workspace:setup", 75], ["dependencies sync --all", 75], ["run repo-mcp:install", 3]] as const) {
      const result = run(command, status);
      expect({command, status: result.status, end: result.stdout.includes("reached-end")}).toEqual({command, status, end: false});
    }
  });
  test("no failure reaches the end of the bootstrap", () => {
    const result = run("none", 0);
    expect({status: result.status, end: result.stdout.includes("reached-end")}).toEqual({status: 0, end: true});
  });
});
