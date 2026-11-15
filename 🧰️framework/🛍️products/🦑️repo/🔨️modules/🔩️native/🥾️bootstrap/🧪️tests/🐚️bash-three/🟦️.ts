/** 🐚️ Shell scripts stay safe under bash 3.2 (the macOS system bash): a `$name` directly followed by a non-ASCII character, such as the ellipsis of a log line, is read as part of the name in a UTF-8 locale and is an unbound variable under `set -u`, so every variable before such a character is braced. The rule is checked on every tracked shell script and against the real system bash. */
import {describe, expect, test} from "bun:test";
import {spawnSync} from "node:child_process";
import {existsSync, readFileSync} from "node:fs";
import {join, resolve} from "node:path";

const root = resolve(import.meta.dir, "../../../../../../.."), systemBash = "/bin/bash";
const tracked = spawnSync("git", ["-c", "core.quotepath=false", "ls-files", "-z", "*.sh"], {cwd: root, encoding: "utf8"}).stdout.split("\0").filter(path => path !== "" && !path.startsWith(".🧬semio/"));
const unbraced = /\$[A-Za-z_][A-Za-z0-9_]*[^\x00-\x7f]/u;
const bashThree = existsSync(systemBash) && /version 3\./u.test(spawnSync(systemBash, ["--version"], {encoding: "utf8"}).stdout);
const run = (script: string) => spawnSync(systemBash, ["-c", script], {encoding: "utf8", env: {PATH: "/usr/bin:/bin", LANG: "en_US.UTF-8"}});

describe("variables before non-ASCII characters", () => {
  test("are braced in every tracked shell script", () => {
    expect(tracked.length).toBeGreaterThan(0);
    for (const path of tracked) {
      const hits = readFileSync(join(root, path), "utf8").split("\n").flatMap((line, index) => unbraced.test(line) ? [`${path}:${index + 1}`] : []);
      expect(hits, path).toEqual([]);
    }
  });

  test("the rule detects exactly the spelling that fails", () => {
    expect(unbraced.test('log "Installing $root…"')).toBe(true);
    expect(unbraced.test('log "Installing ${root}…"')).toBe(false);
    expect(unbraced.test('log "Installing $root and …"')).toBe(false);
  });

  test.skipIf(!bashThree)("the system bash 3.2 fails on the unbraced spelling and accepts the braced one under set -u", () => {
    const bad = run('set -u; DOTNET_ROOT=/x; echo "Installing into $DOTNET_ROOT…"'), good = run('set -u; DOTNET_ROOT=/x; echo "Installing into ${DOTNET_ROOT}…"');
    expect({status: bad.status === 0, unbound: bad.stderr.includes("unbound variable")}).toEqual({status: false, unbound: true});
    expect({status: good.status, out: good.stdout.trim()}).toEqual({status: 0, out: "Installing into /x…"});
  });
});
