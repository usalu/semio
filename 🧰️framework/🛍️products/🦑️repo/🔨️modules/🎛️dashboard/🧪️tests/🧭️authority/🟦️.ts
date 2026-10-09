import { expect, test } from "bun:test";
import Ajv2020 from "ajv/dist/2020";
import { readdirSync, readFileSync } from "node:fs";
import { join, relative, resolve } from "node:path";

const root = resolve(import.meta.dir, "../../../../../../.."), dashboard = resolve(import.meta.dir, "../..");
const read = (path: string) => JSON.parse(readFileSync(path, "utf8"));
const fixture = read(join(dashboard, "🧫️fixtures/🧭️authority/🔣️.json"));
const validate = new Ajv2020({ strict: false, allErrors: true }).compile(read(join(dashboard, "🧬️schema/🧭️authority/🔣️.json")));
const accepts = (scripts: Record<string, unknown>) => Object.keys(scripts).length === Object.keys(fixture.scripts).length && Object.entries(fixture.scripts).every(([name, command]) => scripts[name] === command);

test("the bootstrap decision matches the independent schema validator for every shared vector", () => {
  for (const row of fixture.cases) {
    const scripts = { ...fixture.scripts, ...row.add };
    for (const name of row.remove) delete scripts[name];
    expect(accepts(scripts), row.name).toBe(row.accepted);
    expect(validate({ scripts }), row.name).toBe(row.accepted);
  }
});

test("the root manifest keeps only the canonical dashboard bootstrap", () => {
  const manifest = read(join(root, "package.json"));
  expect(validate(manifest), JSON.stringify(validate.errors?.slice(0, 8))).toBe(true);
  expect(accepts(manifest.scripts)).toBe(true);
});

test("every active source directory is free of editor and agent launch configuration", () => {
  const editors = new Set<string>(fixture.editorDirectories), excluded = new Set<string>(fixture.excludedDirectories);
  const failures: string[] = [], pending = [{ directory: root, editor: false }];
  let visited = 0;
  const key = (name: string) => name.replaceAll("\uFE0F", "").replace(/^[^\p{L}\p{N}.]+/u, "");
  while (pending.length) {
    const { directory, editor } = pending.pop()!;
    if (++visited % 1000 === 0) console.log(`[DEBUG] authority gate inspected ${visited} source directories`);
    for (const entry of readdirSync(directory, { withFileTypes: true })) {
      const path = join(directory, entry.name);
      if (entry.isSymbolicLink()) continue;
      if (entry.isDirectory() && !excluded.has(key(entry.name)) && (!entry.name.startsWith(".") || editors.has(entry.name))) pending.push({ directory: path, editor: editor || editors.has(entry.name) });
      if (editor && entry.isFile() && /(?:^|[.️])launch(?:[.][^/]*)?[.]jsonc?$/u.test(entry.name)) failures.push(relative(root, path).replaceAll("\\", "/"));
    }
  }
  expect(failures.sort(), "active launch configurations compete with the dashboard registry").toEqual([]);
}, 300_000);

test("root runnable commands remain owned by Nx targets and dashboard declarations", () => {
  const manifest = read(join(root, "📋️project.json"));
  expect(Object.keys(manifest.targets).length).toBeGreaterThan(50);
  expect(manifest.targets.setup).toBeDefined();
  expect(manifest.metadata.semio.dashboard.parameters.length).toBeGreaterThan(0);
  const registry = readFileSync(join(dashboard, "🎮️registry/🦀️.rs"), "utf8");
  expect(registry).not.toContain("workspace_scripts");
  console.log(`[DEBUG] authority gate checked ${Object.keys(manifest.targets).length} root Nx targets`);
});

test("authored scripts use one declarative Nx entry", () => {
  const manifest = read(join(root, "📋️project.json"));
  const tool = manifest.metadata.semio.dashboard.tools.find((tool: { id: string }) => tool.id === "run-script");
  expect(tool).toBeDefined();
  expect(tool.command).toEqual(["bun", "nx", "exec", "--projects={project}", "--", "bun", "{workspace}/{script}"]);
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
