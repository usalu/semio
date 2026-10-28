import { describe, expect, test } from "../../../🧪️test/🟦️.ts";
import Ajv from "ajv";
import { spawnSync } from "node:child_process";
import { chmodSync, mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { validateJsonSchemaSubset } from "../../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with { type: "json" };
import corpus from "./🔣️.json" with { type: "json" };
import { defaultNativeToolchainPortsV1, nativeToolchainHostV1, nativeToolchainRequirementsV1, nativeToolchainToolIdsV1, probeNativeToolV1, probeNativeToolchainV1, type NativeToolchainPortsV1 } from "../🟦️.ts";

type ScenarioV1 = Readonly<{ id: string; platform: string; arch: string; env: Record<string, string>; home: string; files: string[]; directories: string[]; commands: { program: string; arguments: string[]; status: number | null; stdout: string }[]; expected: { id: string; found: boolean; location: string | null; how: string }[] }>;
const scenarios = corpus.scenarios as unknown as ScenarioV1[];
const python = ["python3", "python"].find(name => spawnSync(name, ["--version"], { stdio: "ignore" }).status === 0);
const oracleFile = join(import.meta.dirname, "🐍️.py"), schemaFile = join(import.meta.dirname, "../🧬️schema/🔣️.json"), corpusFile = join(import.meta.dirname, "🔣️.json");
const portsOf = (scenario: ScenarioV1): NativeToolchainPortsV1 => ({
  isExecutable: path => scenario.files.includes(path),
  isDirectory: path => scenario.directories.includes(path),
  run: (program, arguments_) => { const row = scenario.commands.find(command => command.program === program && JSON.stringify(command.arguments) === JSON.stringify(arguments_)); return row === undefined ? undefined : { status: row.status, stdout: row.stdout }; },
});
const hostOf = (scenario: ScenarioV1) => ({ platform: scenario.platform, arch: scenario.arch, env: scenario.env, home: scenario.home });
const ask = (...arguments_: string[]): string => { const run = spawnSync(python!, ["-I", oracleFile, ...arguments_], { encoding: "utf8" }); expect(run.status, run.stderr).toBe(0); return run.stdout; };

describe("native toolchain probes", () => {
  test("the corpus is closed by its schema, covers every platform and every probe kind, and the table declares a probe for each requirement", () => {
    expect(validateJsonSchemaSubset(schema.definitions.Corpus, corpus, schema)).toEqual([]);
    const ajv = new Ajv({ strict: false });
    expect(ajv.validate({ definitions: schema.definitions, $ref: "#/definitions/Corpus" }, corpus), JSON.stringify(ajv.errors)).toBe(true);
    expect(ajv.validate({ definitions: schema.definitions, $ref: "#/definitions/Corpus" }, { scenarios: [{ ...scenarios[0], platform: "freebsd" }] })).toBe(false);
    expect(new Set(scenarios.map(row => row.platform))).toEqual(new Set(["darwin", "linux", "win32"]));
    const ids = new Set(scenarios.flatMap(row => row.expected.map(probe => probe.id)));
    expect(ids).toEqual(new Set(nativeToolchainToolIdsV1()));
    for (const platform of Object.keys(schema.definitions.Protocol.const.requirements)) for (const id of nativeToolchainRequirementsV1(platform)) expect(nativeToolchainToolIdsV1(), `${platform} ${id}`).toContain(id);
    expect(new Set(scenarios.flatMap(row => row.expected.map(probe => probe.how)))).toEqual(new Set(["environment", "path", "cargo-home", "developer-directory", "installer", "none"]));
  });

  test("every scenario gives the authored verdicts, on every host", () => {
    for (const scenario of scenarios) expect(probeNativeToolchainV1(hostOf(scenario), portsOf(scenario)), scenario.id).toEqual(scenario.expected as never);
  });

  test("an independent Python implementation of the table agrees on every scenario, and a corrupted table does not", () => {
    expect(python, "a Python 3 interpreter is required for the independent implementation").toBeDefined();
    const model = JSON.parse(ask(schemaFile, corpusFile)) as Record<string, unknown>;
    for (const scenario of scenarios) expect(probeNativeToolchainV1(hostOf(scenario), portsOf(scenario)), scenario.id).toEqual(model[scenario.id] as never);
    const directory = mkdtempSync(join(tmpdir(), "semio-toolchain-"));
    try {
      const broken = structuredClone(schema) as any;
      broken.definitions.Protocol.const.tools.rust.programs = ["rustup"];
      broken.definitions.Protocol.const.tools["msvc-linker"].toolsSegments = ["VC", "Tools"];
      const file = join(directory, "schema.json");
      writeFileSync(file, JSON.stringify(broken));
      const wrong = JSON.parse(ask(file, corpusFile)) as Record<string, unknown>;
      const differing = scenarios.filter(scenario => JSON.stringify(wrong[scenario.id]) !== JSON.stringify(model[scenario.id]));
      expect(differing.length).toBeGreaterThan(2);
    } finally { rmSync(directory, { recursive: true, force: true }); }
  });

  test("a probe asks at most one program and only with the declared arguments", () => {
    for (const scenario of scenarios) {
      const calls: string[] = [], ports = { ...portsOf(scenario), run: (program: string, arguments_: readonly string[], timeout: number) => { calls.push(JSON.stringify([program, arguments_, timeout])); return portsOf(scenario).run(program, arguments_, timeout); } };
      for (const id of nativeToolchainRequirementsV1(scenario.platform)) probeNativeToolV1(id, hostOf(scenario), ports);
      expect(calls.length, scenario.id).toBeLessThanOrEqual(2);
      for (const call of calls) expect(call.includes("installationPath") || call.includes("xcode-select"), call).toBe(true);
    }
    expect(probeNativeToolV1("unknown-tool", { platform: "linux", arch: "x64", env: {}, home: "/h" }, portsOf(scenarios[0]!))).toBeUndefined();
    expect(nativeToolchainRequirementsV1("freebsd")).toEqual([]);
  });

  test("the real file system finds a planted compiler and linker like Python's own search does, and this machine agrees with Python", () => {
    const directory = mkdtempSync(join(tmpdir(), "semio-toolchain-")), bin = join(directory, "bin"), cargo = join(directory, "cargo", "bin");
    try {
      mkdirSync(bin); mkdirSync(cargo, { recursive: true });
      for (const file of [join(bin, "cc"), join(cargo, "rustc")]) { writeFileSync(file, "#!/bin/sh\nexit 0\n"); chmodSync(file, 0o755); }
      writeFileSync(join(bin, "rustup"), "not executable"); chmodSync(join(bin, "rustup"), 0o644);
      if (process.platform === "win32") return;
      const host = { platform: "linux", arch: "x64", env: { PATH: bin, CARGO_HOME: join(directory, "cargo") }, home: directory };
      expect(probeNativeToolV1("cc", host, defaultNativeToolchainPortsV1())).toEqual({ id: "cc", found: true, location: join(bin, "cc"), how: "path" });
      expect(probeNativeToolV1("rust", host, defaultNativeToolchainPortsV1())).toEqual({ id: "rust", found: true, location: join(cargo, "rustc"), how: "cargo-home" });
      expect(probeNativeToolV1("rust", { ...host, env: { PATH: bin } }, defaultNativeToolchainPortsV1())?.found, "a file without the executable bit is not a compiler").toBe(false);
    } finally { rmSync(directory, { recursive: true, force: true }); }
    const real = JSON.parse(ask(schemaFile, corpusFile, "real")) as Record<string, boolean>, host = nativeToolchainHostV1();
    for (const probe of probeNativeToolchainV1(host, defaultNativeToolchainPortsV1())) expect({ id: probe.id, found: probe.found }, JSON.stringify(probe)).toEqual({ id: probe.id, found: real[probe.id] });
  });
});
