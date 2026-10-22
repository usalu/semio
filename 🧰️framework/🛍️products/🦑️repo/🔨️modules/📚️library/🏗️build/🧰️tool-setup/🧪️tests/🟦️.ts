/** 🧰️ The pinned tool table agrees with its language-agnostic fixture, an independent Python implementation of pin resolution, two JSON Schema validators, the TOML and JSON parsers of the platform and the real pin files; verification and setup are exercised against replaceable machines, so every status, every language and every failure is testable on any host. */
import {describe, expect, setDefaultTimeout, test} from "bun:test";
import {spawnSync} from "node:child_process";
import {existsSync, readFileSync, readdirSync} from "node:fs";
import {dirname, join, sep} from "node:path";
import Ajv from "ajv";
import {testPythonV1} from "../../../../../../../🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";
import {validateJsonSchemaSubset} from "../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {pythonInterpreterDirectoryV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🧰️toolchains/🟦️.ts";
import {TOOL_SETUP_V1, locateToolV1, renderToolReportV1, resolvePinV1, resolveSiblingPackageV1, runtimeWarningV1, setupToolsV1, toolPinAwaitsDependenciesV1, toolPinValueV1, toolRowV1, toolRowsV1, toolSetupMessageV1, toolVersionMatchesV1, verifyToolV1, verifyToolsV1, type ToolContextV1, type ToolRowV1} from "../🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type: "json"};
import fixture from "./🧫️fixtures/🔣️.json" with {type: "json"};
import fixtureSchema from "./🧬️schema/🔣️.json" with {type: "json"};

setDefaultTimeout(60000);
const here = import.meta.dir, root = join(here, "../../../../../../../.."), python = testPythonV1(), signal = AbortSignal.timeout(30000);
const oracle = (files: Record<string, string>, packages: Record<string, string>, matches: readonly {found: string; pin: string}[] = []) => {
  if (python === null) throw new Error("A Python 3 interpreter is required for the independent implementation");
  const run = spawnSync(python, ["-I", join(here, "🐍️oracle/🐍️.py")], {input: JSON.stringify({table: TOOL_SETUP_V1, files, packages, matches}), encoding: "utf8", maxBuffer: 1 << 26});
  if (run.status !== 0) throw new Error(run.stderr);
  return JSON.parse(run.stdout) as {pins: Record<string, string | null>; matches: boolean[]};
};
const machine = (overrides: Partial<ToolContextV1> & {files?: Readonly<Record<string, string>>; present?: readonly string[]; outputs?: Readonly<Record<string, string>>} = {}): ToolContextV1 => {
  const files: Readonly<Record<string, string>> = overrides.files ?? fixture.files, present = new Set(overrides.present ?? []), outputs = overrides.outputs ?? {};
  return Object.freeze({
    root: "/repo", toolsRoot: "/repo/.cache/b/t/m", platform: "linux" as NodeJS.Platform, environment: {SEMIO_LANGUAGE: "en", PATH: "/usr/bin"}, home: "/home/dev",
    read: (path: string) => { const key = path.startsWith("/repo/") ? path.slice(6) : path; if (!(key in files)) throw Object.assign(new Error(path), {code: "ENOENT"}); return files[key]!; },
    exists: (path: string) => present.has(path),
    list: (directory: string) => [...present].filter(path => dirname(path.replaceAll("\\", "/").replace(/\/[^/]+$/u, "")) === directory.replaceAll("\\", "/") || path.startsWith(directory + "/")).map(path => path.slice(directory.length + 1).split("/")[0]!),
    resolvePackage: (name: string) => "/repo/" + fixture.packages[name as keyof typeof fixture.packages] + "/package.json",
    capture: async (command: string) => { if (!(command in outputs)) throw new Error("no such program: " + command); return outputs[command]!; },
    provisioned: (locate, pin) => locate.family === "native" ? `/repo/.cache/b/t/m/native/${locate.tool}/${pin}/${locate.executable}` : undefined,
    hook: async () => ({found: null, location: null}),
    ...overrides,
  });
};

describe("the table", () => {
  test("is valid under its strict schema and the repository subset validator, and refuses every mutation", () => {
    const validate = new Ajv({strict: true}).compile(schema);
    expect(validate(TOOL_SETUP_V1), JSON.stringify(validate.errors)).toBe(true);
    expect(validateJsonSchemaSubset(schema, TOOL_SETUP_V1, schema)).toEqual([]);
    const mutations: Record<string, (copy: any) => void> = {
      "unknown pin format": copy => { copy.tools[0].pin.format = "yaml"; },
      "unknown locator": copy => { copy.tools[0].locate[0].kind = "registry"; },
      "unknown provisioner": copy => { copy.tools[0].setup.kind = "brew"; },
      "missing probe": copy => { delete copy.tools[0].probe; },
      "missing german text": copy => { delete copy.messages.texts.ok.de; },
      "unknown language": copy => { copy.messages.languages.push("fr"); },
      "uppercase id": copy => { copy.tools[0].id = "Bun"; },
      "absolute pin file": copy => { copy.tools.find((row: any) => row.pin.format === "json").pin.file = "/etc/passwd"; },
      "parent pin file": copy => { copy.tools.find((row: any) => row.pin.format === "json").pin.file = "../pins.json"; },
      "empty value": copy => { copy.tools.find((row: any) => row.pin.format === "value").pin.value = ""; },
    };
    for (const [name, mutate] of Object.entries(mutations)) {
      const copy = structuredClone(TOOL_SETUP_V1);
      mutate(copy);
      expect({name, ajv: validate(copy), subset: validateJsonSchemaSubset(schema, copy, schema).length === 0}, name).toEqual({name, ajv: false, subset: false});
    }
  });

  test("lists every tool once, every group member exists and every row names a real provisioner and pin file", () => {
    const ids = TOOL_SETUP_V1.tools.map(row => row.id);
    expect(new Set(ids).size).toBe(ids.length);
    expect([...TOOL_SETUP_V1.groups.all].sort()).toEqual([...ids].sort());
    for (const row of TOOL_SETUP_V1.tools) {
      if (row.pin.format !== "value" && !("package" in row.pin && row.pin.package !== undefined)) expect({id: row.id, file: existsSync(join(root, row.pin.file))}).toEqual({id: row.id, file: true});
      if (row.setup.kind === "script") expect({id: row.id, script: existsSync(join(root, row.setup.script))}).toEqual({id: row.id, script: true});
    }
  });

  test("keeps every German text next to an English one with the same placeholders", () => {
    for (const [key, text] of Object.entries(TOOL_SETUP_V1.messages.texts)) {
      const holes = (value: string) => [...value.matchAll(/\{([a-z]+)\}/gu)].map(match => match[1]).sort();
      expect({key, holes: holes(text.de)}).toEqual({key, holes: holes(text.en)});
    }
  });

  test("renders the language it resolves and every language when it resolves none", () => {
    expect(toolSetupMessageV1("ok", {}, {SEMIO_LANGUAGE: "de"})).toBe("ok");
    expect(toolSetupMessageV1("missing", {}, {SEMIO_LANGUAGE: "de"})).toBe("fehlt");
    expect(toolSetupMessageV1("missing", {}, {SEMIO_LANGUAGE: "en"})).toBe("missing");
    expect(toolSetupMessageV1("missing", {}, {LANG: "C", LC_ALL: "C", LANGUAGE: ""})).toBe("missing / fehlt");
    expect(toolSetupMessageV1("clean", {count: 3}, {SEMIO_LANGUAGE: "de"})).toContain("Alle 3");
  });
});

describe("pin resolution", () => {
  test("the fixture is valid under its strict schema and equal to the independent implementation", () => {
    const validate = new Ajv({strict: true}).compile(fixtureSchema);
    expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
    const result = oracle(fixture.files, fixture.packages, fixture.matches);
    expect(result.pins).toEqual(fixture.expected);
    expect(result.matches).toEqual(fixture.matches.map(row => row.expected));
  });

  test("the engine reads the fixture pins from the files that own them", () => {
    const context = machine();
    expect(Object.fromEntries(TOOL_SETUP_V1.tools.map(row => [row.id, resolvePinV1(row, context)]))).toEqual(fixture.expected);
  });

  test("the engine decides the version matches the independent implementation does", () => {
    expect(fixture.matches.map(row => toolVersionMatchesV1(row.found, row.pin))).toEqual(fixture.matches.map(row => row.expected));
  });

  test("a pin file that cannot be read or has no pin names the tool and the file in the resolved language", () => {
    const missing = machine({files: {}});
    expect(() => resolvePinV1(toolRowV1("go"), missing)).toThrow(/go.*toolchains|Pin of go|pin of go/iu);
    const broken = machine({files: {...fixture.files, "rust-toolchain.toml": "[toolchain]\n"}});
    expect(() => resolvePinV1(toolRowV1("rust"), broken)).toThrow(/rust-toolchain\.toml/u);
    const german = machine({files: {}, environment: {SEMIO_LANGUAGE: "de"}});
    expect(() => resolvePinV1(toolRowV1("go"), german)).toThrow(/nicht lesbar/u);
  });

  test("the real pin files give the independent implementation's pins, and the platform's own parsers agree with the regular expressions", () => {
    const files: Record<string, string> = {}, packages: Record<string, string> = {};
    for (const row of TOOL_SETUP_V1.tools) {
      if (row.pin.format === "value") continue;
      if ("package" in row.pin && row.pin.package !== undefined) {
        const directory = dirname(resolveSiblingPackageV1(root, row.pin.package, row.pin.from!));
        packages[row.pin.package] = directory.slice(root.length + 1).split(sep).join("/");
        files[packages[row.pin.package]! + "/" + row.pin.file] = readFileSync(join(directory, row.pin.file), "utf8");
      } else files[row.pin.file] = readFileSync(join(root, row.pin.file), "utf8");
    }
    const context: ToolContextV1 = machine({files, root, resolvePackage: (name, from) => resolveSiblingPackageV1(root, name, from), read: (path: string) => readFileSync(path, "utf8")}), real = Object.fromEntries(TOOL_SETUP_V1.tools.map(row => [row.id, resolvePinV1(row, {...context, root})]));
    expect<unknown>(real).toEqual(oracle(files, packages).pins);
    expect(real.rust).toBe((Bun.TOML.parse(files["rust-toolchain.toml"]!) as {toolchain: {channel: string}}).toolchain.channel);
    const lock = Bun.TOML.parse(files["Cargo.lock"]!) as {package: {name: string; version: string}[]};
    expect(real["wasm-bindgen"]).toBe(lock.package.filter(row => row.name === "wasm-bindgen").map(row => row.version).join());
    expect(real.bun).toBe((JSON.parse(files["package.json"]!) as {packageManager: string}).packageManager.replace("bun@", ""));
    expect(toolPinValueV1("wasm-pack")).toBe(real["wasm-pack"]!);
    expect(() => toolPinValueV1("go")).toThrow();
  });
});

const absent = (id: string, pin: string, extra: Partial<ToolContextV1> = {}) => verifyToolV1(toolRowV1(id), machine({files: fixture.files, ...extra}), signal).then(report => ({...report, pin}));

describe("verification", () => {
  test("an installed copy that reports the pin is ok, a missing one is missing and a different one differs", async () => {
    const uv = "/repo/.cache/b/t/m/native/uv/8.8.8/uv";
    expect(await verifyToolV1(toolRowV1("uv"), machine({present: [uv], outputs: {[uv]: "uv 8.8.8 (abc 2030-01-01 x86_64)\n"}}), signal)).toMatchObject({status: "ok", found: "8.8.8", pin: "8.8.8", location: uv});
    expect(await verifyToolV1(toolRowV1("uv"), machine({present: [uv], outputs: {[uv]: "uv 0.11.15 (3cffe97c2 2026-05-18 aarch64-apple-darwin)\n"}}), signal)).toMatchObject({status: "differs", found: "0.11.15", pin: "8.8.8"});
    expect(await verifyToolV1(toolRowV1("uv"), machine(), signal)).toMatchObject({status: "missing", found: null, location: null});
    expect(await verifyToolV1(toolRowV1("uv"), machine({present: [uv], outputs: {[uv]: "garbage"}}), signal)).toMatchObject({status: "failed"});
    expect(await verifyToolV1(toolRowV1("uv"), machine({present: [uv], outputs: {}}), signal)).toMatchObject({status: "failed", message: expect.stringContaining("no such program")});
    expect((await verifyToolV1(toolRowV1("uv"), machine({files: {}}), signal)).status).toBe("failed");
  });

  test("a tool on the machine is found in the declared directories before the PATH, with the Windows suffix", async () => {
    const row = toolRowV1("node");
    expect(locateToolV1(row, "11.1.1", machine({present: ["/home/dev/.local/node/bin/node", "/usr/bin/node"]}))).toBe("/home/dev/.local/node/bin/node");
    expect(locateToolV1(row, "11.1.1", machine({present: ["/usr/bin/node"]}))).toBe("/usr/bin/node");
    expect(locateToolV1(row, "11.1.1", machine({platform: "win32", environment: {PATH: "/bin"}, present: ["/bin/node.exe"]}))).toBe(join("/bin", "node.exe"));
    expect(locateToolV1(row, "11.1.1", machine())).toBeUndefined();
    const dotnet = toolRowV1("dotnet");
    expect(locateToolV1(dotnet, "5.5.5", machine({environment: {PATH: "/usr/bin", DOTNET_ROOT: "/opt/dotnet"}, present: ["/opt/dotnet/dotnet"]}))).toBe("/opt/dotnet/dotnet");
  });

  test("a managed Python interpreter is found by its series in the interpreter directory of the tools partition", async () => {
    const directory = pythonInterpreterDirectoryV1("/repo/.cache/b/t/m").replaceAll("\\", "/"), python313 = `${directory}/cpython-7.7.2-linux-x86_64-gnu/bin/python3`, other = `${directory}/cpython-7.70.1-linux-x86_64-gnu/bin/python3`;
    const context = machine({present: [other, python313], list: () => ["cpython-7.70.1-linux-x86_64-gnu", "cpython-7.7.2-linux-x86_64-gnu", "cpython-6.6.1-linux-x86_64-gnu"], outputs: {[python313]: "Python 7.7.2\n"}});
    expect(await verifyToolV1(toolRowV1("python-main"), context, signal)).toMatchObject({status: "ok", found: "7.7.2", pin: "7.7", location: python313});
    expect((await verifyToolV1(toolRowV1("python-oracle"), context, signal)).status).toBe("missing");
  });

  test("the dotnet SDK is ok when the pin is listed among several and differs when it is not", async () => {
    const dotnet = "/home/dev/.dotnet/dotnet", listing = "9.0.314 [/x/sdk]\n5.5.5 [/y/sdk]\n10.0.300 [/z/sdk]\n";
    expect(await verifyToolV1(toolRowV1("dotnet"), machine({present: [dotnet], outputs: {[dotnet]: listing}}), signal)).toMatchObject({status: "ok", found: "5.5.5"});
    expect(await verifyToolV1(toolRowV1("dotnet"), machine({present: [dotnet], outputs: {[dotnet]: "9.0.314 [/x/sdk]\n"}}), signal)).toMatchObject({status: "differs", found: null});
  });

  test("a hook row reports what its hook found and the report renders as one verdict in each language", async () => {
    const hooked = machine({hook: async (_row: ToolRowV1, pin: string) => ({found: pin, location: "/cache/chromium-" + pin})});
    expect(await verifyToolV1(toolRowV1("chromium"), hooked, signal)).toMatchObject({status: "ok", found: "4242", location: "/cache/chromium-4242"});
    const reports = await verifyToolsV1(toolRowsV1(["chromium", "uv"]), machine(), signal);
    expect(reports.map(report => report.status)).toEqual(["missing", "missing"]);
    const english = renderToolReportV1(reports, {SEMIO_LANGUAGE: "en"}), german = renderToolReportV1(reports, {SEMIO_LANGUAGE: "de"});
    expect(english.at(-1)).toContain("2 of 2 pinned tools differ");
    expect(german.at(-1)).toContain("2 von 2 gepinnten Werkzeugen");
    expect(german[1]).toContain("fehlt");
    expect(renderToolReportV1([{id: "go", pin: "1", found: "1", status: "ok", location: "/x"}], {SEMIO_LANGUAGE: "en"}).at(-1)).toBe("All 1 pinned tools match their pins.");
  });

  test("warns once about a running Bun that is not the pinned one, in the resolved language", () => {
    const reports = [{id: "bun", pin: "1.4.2", found: "1.4.2", status: "ok" as const, location: "/x"}];
    expect(runtimeWarningV1(reports, "1.4.2", {SEMIO_LANGUAGE: "en"})).toBeUndefined();
    expect(runtimeWarningV1(reports, undefined, {SEMIO_LANGUAGE: "en"})).toBeUndefined();
    expect(runtimeWarningV1([], "1.3.14", {SEMIO_LANGUAGE: "en"})).toBeUndefined();
    expect(runtimeWarningV1(reports, "1.3.14", {SEMIO_LANGUAGE: "en"})).toContain("running Bun 1.3.14 is not the pinned Bun 1.4.2");
    expect(runtimeWarningV1(reports, "1.3.14", {SEMIO_LANGUAGE: "de"})).toContain("laufende Bun 1.3.14");
  });

  test("stops between tools when the signal is aborted", async () => {
    const controller = new AbortController();
    controller.abort(new Error("cancelled"));
    await expect(verifyToolsV1(toolRowsV1("all"), machine(), controller.signal)).rejects.toThrow(/cancelled/u);
  });
});

describe("setup", () => {
  const uv = "/repo/.cache/b/t/m/native/uv/8.8.8/uv";
  test("skips what matches, reports the bootstrap tools, provisions the rest once and verifies it again", async () => {
    const present = new Set<string>(), lines: string[] = [], provisioned: string[] = [];
    const context = machine({exists: (path: string) => present.has(path), capture: async (command: string) => { if (command === uv) return "uv 8.8.8\n"; throw new Error("no such program: " + command); }});
    const reports = await setupToolsV1(toolRowsV1(["uv", "node"]), context, async row => { provisioned.push(row.id); present.add(uv); }, signal, line => lines.push(line));
    expect(provisioned).toEqual(["uv"]);
    expect(reports.map(report => [report.id, report.status])).toEqual([["uv", "ok"], ["node", "missing"]]);
    expect(lines).toEqual(["Provisioning uv 8.8.8.", "uv 8.8.8 is ready.", "node 11.1.1 is installed by the bootstrap script, not by this command; run the bootstrap again."]);
    lines.length = 0;
    await setupToolsV1(toolRowsV1(["uv"]), context, async () => { throw new Error("must not run"); }, signal, line => lines.push(line));
    expect(lines).toEqual(["uv 8.8.8 already matches its pin."]);
  });

  test("fails when a provisioned tool still differs from its pin, in the resolved language", async () => {
    const context = machine({exists: () => true, capture: async () => "uv 0.1.0\n", environment: {SEMIO_LANGUAGE: "de"}});
    await expect(setupToolsV1(toolRowsV1(["uv"]), context, async () => undefined, signal, () => undefined)).rejects.toThrow(/wurde eingerichtet, meldet aber 0\.1\.0 statt des Pins 8\.8\.8/u);
  });

  test("fails before provisioning anything when a pin cannot be read, and stops when cancelled", async () => {
    await expect(setupToolsV1(toolRowsV1(["uv"]), machine({files: {}}), async () => { throw new Error("must not run"); }, signal, () => undefined)).rejects.toThrow(/uv/u);
    const controller = new AbortController();
    controller.abort(new Error("cancelled"));
    await expect(setupToolsV1(toolRowsV1(["uv"]), machine(), async () => undefined, controller.signal, () => undefined)).rejects.toThrow(/cancelled/u);
  });

  test("postpones a tool whose pin lives in a package that is not installed yet, in each language, and refuses it once the package is installed but unreadable", async () => {
    const chromium = toolRowsV1(["chromium"]), lines: string[] = [];
    const reports = await setupToolsV1(chromium, machine({files: {}, exists: () => false}), async () => { throw new Error("must not run"); }, signal, line => lines.push(line));
    expect(reports.map(report => [report.id, report.status, report.pin])).toEqual([["chromium", "deferred", "-"]]);
    expect(lines).toEqual(["chromium reads its pin from the installed playwright, which is not installed yet; install the dependencies, then run `repo tools setup chromium` again."]);
    expect(toolSetupMessageV1("postponed", {tool: "chromium", package: "playwright"}, {SEMIO_LANGUAGE: "de"})).toContain("liest seinen Pin aus dem installierten playwright");
    expect(toolPinAwaitsDependenciesV1(chromium[0]!, {root: "/repo", exists: path => path === "/repo/node_modules/playwright"})).toBe(false);
    expect(toolPinAwaitsDependenciesV1(toolRowV1("uv"), {root: "/repo", exists: () => false})).toBe(false);
    await expect(setupToolsV1(chromium, machine({files: {}, exists: path => path === "/repo/node_modules/playwright"}), async () => { throw new Error("must not run"); }, signal, () => undefined)).rejects.toThrow(/chromium/u);
    expect(renderToolReportV1(reports, {SEMIO_LANGUAGE: "de"})[1]).toContain("zurückgestellt");
  });

  test("selects a group, refuses an unknown tool and keeps the order of the table", () => {
    expect(toolRowsV1("all").map(row => row.id)).toEqual(TOOL_SETUP_V1.groups.all);
    expect(() => toolRowsV1(["nope"], {SEMIO_LANGUAGE: "de"})).toThrow(/Unbekanntes gepinntes Werkzeug: nope/u);
    expect(readdirSync(here).length).toBeGreaterThan(0);
  });
});
