import { expect, test } from "bun:test";
import Ajv from "ajv/dist/2020";
import glob from "fast-glob";
import { readFileSync, writeFileSync, mkdirSync, mkdtempSync, rmSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";
import { spawnSync } from "node:child_process";
import { fileURLToPath } from "node:url";
import { rustTokens, rustTokenPairs } from "../../../../../../../🔨️modules/📚️compiler/📖️syntax/🦀️rust/📦️packages/🟦️typescript/🟦️.ts";
import schema from "../../../🧬️schema/🧭️journeys/🔣️.json" with { type: "json" };
import fixture from "../../🧫️fixtures/🧭️journeys/🔣️.json" with { type: "json" };

const dashboard = resolve(import.meta.dir, "../../.."), source = readFileSync(new URL("../🏗️workspace/🦀️.rs", import.meta.url), "utf8");
const imports = source.match(/^use std::[^;]+;/gm)!.join("\n");
function actualFunction(name: string): string {
  const tokens = rustTokens(source), pairs = rustTokenPairs(tokens), at = tokens.findIndex((token, index) => token.text === "fn" && tokens[index + 1]?.text === name);
  if (at < 0) throw Error(`Actual journey ${name} required`);
  const open = tokens.findIndex((token, index) => index > at && token.text === "{"), close = pairs.get(open);
  if (close === undefined) throw Error(`Complete journey ${name} required`);
  return source.slice(tokens[at]!.start, tokens[close]!.end);
}
function withProbe(name: string, body: (executable: string) => void): void {
  const parent = process.env.SEMIO_TEST_ARTIFACT_DIR; if (!parent) throw Error("Caller-owned journey source artifacts required");
  mkdirSync(parent, { recursive: true }); const workspace = mkdtempSync(join(parent, "js-"));
  try {
    const program = join(workspace, "probe.rs"), executable = join(workspace, process.platform === "win32" ? "probe.exe" : "probe");
    if (executable.length > 256) throw Error("Portable journey source fixture path capacity exceeded");
    let included = "";
    if (name === "fixture_script") {
      const declaration = actualFunction("journey_fixture"), tokens = rustTokens(declaration), pairs = rustTokenPairs(tokens), at = tokens.findIndex(token => token.text === "include_str"), close = pairs.get(at + 2);
      if (at < 0 || close === undefined) throw Error("Actual compile-time journey fixture include required");
      included = `println!("{}",${declaration.slice(tokens[at]!.start,tokens[close]!.end)});`;
    }
    writeFileSync(program, `${imports}\n${actualFunction(name)}\nfn main(){println!("{}",${name}().display());${included}}`);
    const compiled = spawnSync("rustc", ["--edition=2021", "--crate-name=journey_source_probe", program, "-o", executable], { env: { ...process.env, CARGO_MANIFEST_DIR: dirname(fileURLToPath(new URL("../📦️packages/🦀️rust/Cargo.toml", import.meta.url))) }, encoding: "utf8", timeout: 30_000, maxBuffer: 1_048_576 });
    expect(compiled.status, compiled.stderr).toBe(0); body(executable);
  } finally { const bounded = relative(resolve(parent), resolve(workspace)); if (!bounded || bounded.startsWith("..") || bounded.includes(":")) throw Error("Journey source cleanup escaped caller-owned directory"); rmSync(workspace, { recursive: true, force: true }); }
}

test("actual Rust journey fixture source resolves the shared authored script", () => {
  const oracle = new Ajv({ strict: false }).compile(schema); expect(oracle(fixture), JSON.stringify(oracle.errors)).toBe(true);
  expect(glob.sync(fixture.nativeSources.script, { cwd: dashboard, onlyFiles: true, followSymbolicLinks: false })).toEqual([fixture.nativeSources.script]);
  withProbe("fixture_script", executable => {
    const run = spawnSync(executable, [], { encoding: "utf8", timeout: 10_000 }); expect(run.status, run.stderr).toBe(0);
    const boundary = run.stdout.indexOf("\n"), actual = run.stdout.slice(0,boundary).trim(), expected = join(dashboard, fixture.nativeSources.script), portable = (value: string) => value.replace(/^\\\\\?\\/, "").replaceAll("\\", "/");
    expect(portable(actual)).toBe(portable(expected)); expect(readFileSync(actual)).toEqual(readFileSync(expected));
    expect(JSON.parse(run.stdout.slice(boundary+1))).toEqual(fixture);
  });
});

test("actual Rust journey binary selection requires the owner-supplied executable", () => {
  const oracle = new Ajv({ strict: true }).compile(schema.$defs.ownedExecutable);
  withProbe("binary", executable => {
    for (const row of fixture.nativeSources.binaryCases) {
      expect(oracle(row.value)).toBe(row.accepted);
      const run = spawnSync(executable, [], { env: { ...process.env, [fixture.nativeSources.binaryEnvironment]: row.value ?? undefined }, encoding: "utf8", timeout: 10_000 });
      expect(run.status === 0, run.stderr).toBe(row.accepted);
      if (row.accepted) expect(run.stdout.trim()).toBe(row.value!); else expect(run.stderr).toContain(fixture.nativeSources.binaryEnvironment);
    }
  });
});
