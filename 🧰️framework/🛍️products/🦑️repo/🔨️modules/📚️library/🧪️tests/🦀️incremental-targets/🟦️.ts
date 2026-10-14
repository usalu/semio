import { expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { cleanCollectRustIncrementalRemovals, cleanPathBytes } from "../../🧼️workspace-cleanup/🔍️candidate-discovery/🟦️.ts";
import { cleanProjectRemovals, cleanProtectedPrefixes, cleanSelectHugeRustIncremental, type CleanRustIncrementalDirectory } from "../../🧼️workspace-cleanup/🛡️protection/🟦️.ts";
import { runWorkspaceClean } from "../../🧼️workspace-cleanup/🗑️removal/🟦️.ts";

const fixture = JSON.parse(readFileSync(new URL("../🧫️fixtures/🦀️incremental-targets/🔣️.json", import.meta.url), "utf8")) as {
  maxBytes: number;
  directories: CleanRustIncrementalDirectory[];
  expected: string[];
};

const PYTHON = `import json,sys
rows=json.loads(sys.argv[1]); limit=json.loads(sys.argv[2]); out=[]
for directory in rows:
    base=directory["path"].replace("\\\\","/").rstrip("/")
    crates=directory["crates"]
    total=sum(crate["bytes"] for crate in crates)
    if total<=limit: continue
    idle=[crate for crate in crates if not crate["working"]]
    if not idle: continue
    if len(idle)==len(crates):
        out.append(base)
    else:
        out.extend(sorted(base+"/"+crate["name"] for crate in idle))
print(json.dumps(out))
`;

const WALK = `import os,stat,json,sys
total=0
for directory,dirs,files in os.walk(sys.argv[1],followlinks=False):
    dirs.sort(); files.sort()
    for name in files:
        path=os.path.join(directory,name)
        info=os.lstat(path)
        if stat.S_ISREG(info.st_mode): total+=info.st_size
print(json.dumps(total))
`;

test("selects huge rust incremental directories and keeps working sessions, matching Python", () => {
  const selected = fixture.directories.flatMap((directory) => [...cleanSelectHugeRustIncremental(directory, fixture.maxBytes)]).sort();
  expect(selected).toEqual([...fixture.expected].sort());
  const python = Bun.which("python3") ?? Bun.which("python");
  expect(python).toBeTruthy();
  const oracle = spawnSync(python!, ["-c", PYTHON, JSON.stringify(fixture.directories), JSON.stringify(fixture.maxBytes)], { encoding: "utf8" });
  expect(oracle.status, oracle.stderr).toBe(0);
  expect(JSON.parse(oracle.stdout)).toEqual([...fixture.expected].sort());
});

test("clean removes huge cargo incremental caches inside the protected cache and leaves everything else", () => {
  const root = mkdtempSync(join(tmpdir(), "semio-rust-incremental-"));
  try {
    const big = 200_000;
    const small = 10;
    const limit = 100_000;
    const session = (rel: string, bytes: number, working = false): string => {
      const path = join(root, rel, working ? "s-0000000001-aaaa-working" : "s-0000000001-aaaa-bbbb", "data");
      mkdirSync(dirname(path), { recursive: true });
      writeFileSync(path, "x".repeat(bytes));
      return dirname(dirname(path));
    };
    const cache = join(".🧬semio", "🦑️repo", "⚡️cache", "cargo");
    const bigCrate = session(join(cache, "build", "debug", "incremental", "big-dead"), big);
    const keep = join(root, cache, "build", "debug", "build", "pkg", "hash", "keep.txt");
    mkdirSync(dirname(keep), { recursive: true });
    writeFileSync(keep, "keep");
    session(join(cache, "build", "release", "incremental", "small-aaaa"), small);
    session(join(cache, "canonical-architecture", "prototype", "incremental", "live-ffff"), big, true);
    const stale = session(join(cache, "canonical-architecture", "prototype", "incremental", "stale-gggg"), big);
    session(join(".🧬semio", "🦑️repo", "⚡️cache", "vite", "incremental", "fake-hhhh"), big);
    session(join("pkg", "incremental", "fake-iiii"), big);
    const notes = join(root, cache, "build", "wasm-dev", "incremental", "notes", "readme.txt");
    mkdirSync(dirname(notes), { recursive: true });
    writeFileSync(notes, "x".repeat(big));
    const python = Bun.which("python3") ?? Bun.which("python");
    expect(python).toBeTruthy();
    const walked = spawnSync(python!, ["-c", WALK, bigCrate], { encoding: "utf8" });
    expect(walked.status, walked.stderr).toBe(0);
    expect(cleanPathBytes(bigCrate)).toBe(JSON.parse(walked.stdout));
    const removals = cleanCollectRustIncrementalRemovals(root, limit).map((row) => row.path.split("\\").join("/")).sort();
    expect(removals).toEqual([
      join(cache, "build", "debug", "incremental").split("\\").join("/"),
      join(cache, "canonical-architecture", "prototype", "incremental", "stale-gggg").split("\\").join("/"),
    ].sort());
    const projected = cleanProjectRemovals(root, [
      ...cleanCollectRustIncrementalRemovals(root, limit),
      { kind: "cargo-incremental", path: join(".🧬semio", "🦑️repo", "⚡️cache", "vite", "incremental"), bytes: big },
    ], cleanProtectedPrefixes(root));
    expect(projected.map((row) => row.path.split("\\").join("/")).sort()).toEqual(removals);
    const dry = runWorkspaceClean(root, true, limit);
    expect(dry.removals.map((row) => row.path.split("\\").join("/")).sort()).toEqual(removals);
    expect(existsSync(join(bigCrate, "s-0000000001-aaaa-bbbb", "data"))).toBe(true);
    const applied = runWorkspaceClean(root, false, limit);
    expect(applied.removals.map((row) => row.path.split("\\").join("/")).sort()).toEqual(removals);
    expect(existsSync(bigCrate)).toBe(false);
    expect(existsSync(stale)).toBe(false);
    expect(existsSync(keep)).toBe(true);
    expect(existsSync(join(root, cache, "build", "release", "incremental", "small-aaaa"))).toBe(true);
    expect(existsSync(join(root, cache, "canonical-architecture", "prototype", "incremental", "live-ffff", "s-0000000001-aaaa-working", "data"))).toBe(true);
    expect(existsSync(join(root, ".🧬semio", "🦑️repo", "⚡️cache", "vite", "incremental", "fake-hhhh"))).toBe(true);
    expect(existsSync(join(root, "pkg", "incremental", "fake-iiii"))).toBe(true);
    expect(existsSync(notes)).toBe(true);
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});
