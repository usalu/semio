import "../♻️reuse/🟦️.ts";
import { expect, test } from "bun:test";
import Ajv2020 from "ajv/dist/2020";
import { readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { spawnSync } from "node:child_process";
import { parseConfigFileTextToJson } from "typescript";

const root = resolve(import.meta.dir, "../../../../../../.."), dashboard = resolve(import.meta.dir, "../..");
const read = (path: string) => JSON.parse(readFileSync(path, "utf8"));
const fixture = read(join(dashboard, "🧪️tests/🧫️fixtures/🧭️authority/🔣️.json"));
const authority = read(join(dashboard, "🧬️schema/🧭️authority/🔣️.json")), oracle = new Ajv2020({ strict: false, allErrors: true });
const validate = oracle.compile(authority), neutralAlias = new RegExp(authority.properties.scripts.additionalProperties.pattern);
const accepts = (scripts: Record<string, unknown>) => Object.entries(fixture.scripts).every(([name, command]) => scripts[name] === command) && Object.entries(scripts).every(([name, command]) => Object.hasOwn(fixture.scripts, name) || typeof command === "string" && neutralAlias.test(command));

test("the bootstrap decision matches the independent schema validator for every shared vector", () => {
  const aliases: string[] = [];
  for (const row of fixture.cases) {
    const scripts = { ...fixture.scripts, ...row.add };
    for (const name of row.remove) delete scripts[name];
    expect(accepts(scripts), row.name).toBe(row.accepted);
    expect(validate({ scripts }), row.name).toBe(row.accepted);
    if (row.accepted) for (const [name, command] of Object.entries(scripts)) if (!Object.hasOwn(fixture.scripts, name)) {
      aliases.push(command as string);
    }
  }
  const parsed = spawnSync(Bun.which("python3") ?? Bun.which("python") ?? "python3", ["-c", "import json,shlex,sys; print(json.dumps([shlex.split(value) for value in json.load(sys.stdin)]))"], { input: JSON.stringify(aliases), encoding: "utf8", timeout: 5000, maxBuffer: 65536 });
  expect(parsed.status, parsed.stderr).toBe(0);
  expect(parsed.signal).toBeNull();
  expect(JSON.parse(parsed.stdout)).toEqual(aliases.map(command => command.split(" ")));
});

test("the root manifest keeps canonical bootstraps and neutral task aliases", () => {
  const manifest = read(join(root, "package.json"));
  expect(validate(manifest), JSON.stringify(validate.errors?.slice(0, 8))).toBe(true);
  expect(accepts(manifest.scripts)).toBe(true);
});

test("registered editor launch projections preserve neutral build dispatch", () => {
  const validateLaunch = oracle.getSchema(`${authority.$id}#/$defs/LaunchCommand`)!;
  for (const row of fixture.launchCases) expect(validateLaunch(row.command), row.command).toBe(row.accepted);
  let commands = 0;
  for (const file of fixture.launchFiles) {
    const parsed = parseConfigFileTextToJson(file, readFileSync(join(root, file), "utf8"));
    expect(parsed.error).toBeUndefined();
    expect(parsed.config.version).toBe("0.2.0");
    expect(parsed.config.configurations.length).toBeGreaterThan(0);
    for (const row of parsed.config.configurations) if (typeof row.command === "string") {
      expect(validateLaunch(row.command), `${file}: ${row.name}`).toBe(true);
      commands++;
    }
  }
  expect(commands).toBeGreaterThan(0);
});

test("root runnable commands remain owned by build targets and dashboard declarations", () => {
  const manifest = read(join(root, "📋️project.json"));
  expect(Object.keys(manifest.targets).length).toBeGreaterThan(50);
  expect(manifest.targets.setup).toBeDefined();
  expect(manifest.metadata.semio.dashboard.parameters.length).toBeGreaterThan(0);
  const registry = readFileSync(join(dashboard, "🎮️registry/🦀️.rs"), "utf8");
  expect(registry).not.toContain("workspace_scripts");
});

test("authored scripts use one declarative build entry", () => {
  const manifest = read(join(root, "📋️project.json"));
  const tool = manifest.metadata.semio.dashboard.tools.find((tool: { id: string }) => tool.id === "run-script");
  expect(tool).toBeDefined();
  expect(tool.command).toEqual(["bun", "repo", "exec", "--projects={project}", "--", "bun", "{workspace}/{script}"]);
  expect(tool.cwd).toBe("{directory}");
  expect(tool.parameters.find((parameter: { id: string }) => parameter.id === "script")).toEqual({ id: "script", kind: "text", required: true });
  expect(tool.parameters.find((parameter: { id: string }) => parameter.id === "project").default).toBe("workspace");
  const schema = read(join(dashboard, "🧬️schema/🎮️registry/🔣️.json"));
  const oracle = new Ajv2020({ strict: false, allErrors: true }); oracle.addSchema(schema);
  const declaration = oracle.getSchema(`${schema.$id}#/$defs/ProjectManifest`)!;
  expect(declaration(manifest), oracle.errorsText(declaration.errors)).toBe(true);
});

test("workspace startup never waits for Nx analytics consent", () => {
  const config = read(join(root, "nx.json"));
  expect(config.analytics).toBe(false);
  const schema = read(join(root, "node_modules/nx/schemas/nx-schema.json"));
  const validateAnalytics = new Ajv2020({ strict: false }).compile(schema.properties.analytics);
  expect(validateAnalytics(config.analytics)).toBe(true);
});
