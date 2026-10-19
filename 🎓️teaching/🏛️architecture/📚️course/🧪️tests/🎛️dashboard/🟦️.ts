/** 🎛️ Course commands retain their canonical dashboard execution, readiness and compound contracts.
 * @see ../../🧫️fixtures/🎛️dashboard/🔣️.json — language-neutral vectors independently checked by Ajv. */
import Ajv from "ajv";
import { existsSync, readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { expect, it } from "vitest";
import contract from "../../🧫️fixtures/🎛️dashboard/🔣️.json" with { type: "json" };

type Parameter = { id: string; default?: string; values?: { id: string; env?: Record<string, string> }[] };
type Ready = { port: number; portEnv: string };
type Target = { executor: string; options: { command: string; cwd: string }; metadata?: { semio?: { dashboard?: { ready?: Ready; parameters?: Parameter[] } } } };
const site = resolve(dirname(fileURLToPath(import.meta.url)), "../..");
const workspace = resolve(site, "../../..");
const bundle = resolve(site, "📦️packages/🟦️typescript");
const project = JSON.parse(readFileSync(resolve(bundle, "📋️project.json"), "utf8")) as { name: string; targets: Record<string, Target>; metadata: { semio: { dashboard: { compounds: unknown[] } } } };
const manifest = JSON.parse(readFileSync(resolve(bundle, "package.json"), "utf8")) as { semio: { app: { hostKind: string; port: { dev: number; env: string } } } };
const targets = Object.entries(project.targets).sort(([a], [b]) => a.localeCompare(b, "en"));

it("matches the language-neutral dashboard contract through Ajv and direct comparison", () => {
  const modes = targets.flatMap(([id, target]) => (target.metadata?.semio?.dashboard?.parameters ?? []).flatMap(parameter => {
    const mode = parameter.values?.find(value => value.id === parameter.default)?.env?.SEMIO_BUILD_MODE;
    return mode ? [[id, mode]] : [];
  }));
  const actual = {
    project: project.name,
    host: manifest.semio.app.hostKind,
    port: manifest.semio.app.port,
    executor: project.targets.build!.executor,
    cwd: project.targets.build!.options.cwd,
    targets: targets.map(([id]) => id),
    compilerModes: Object.fromEntries(modes),
    ready: Object.fromEntries(targets.flatMap(([id, target]) => target.metadata?.semio?.dashboard?.ready ? [[id, target.metadata.semio.dashboard.ready]] : [])),
    compounds: project.metadata.semio.dashboard.compounds,
  };
  const validate = new Ajv({ strict: true }).compile(contract);
  expect(validate(actual), JSON.stringify(validate.errors)).toBe(true);
  expect(actual).toEqual(contract.const);
});

it("routes every canonical target through its owner script and manifest readiness", () => {
  for (const [id, target] of targets) {
    expect({ executor: target.executor, cwd: target.options.cwd, command: target.options.command }).toEqual({ executor: contract.const.executor, cwd: contract.const.cwd, command: `bun ./📜️script.ts ${id}` });
    const ready = target.metadata?.semio?.dashboard?.ready;
    if (ready) expect(ready).toEqual({ port: manifest.semio.app.port.dev, portEnv: manifest.semio.app.port.env });
  }
});

it("has no separate IDE command catalog", () => {
  expect([".vscode/launch.json", ".vscode/tasks.json"].filter(path => existsSync(resolve(workspace, path)))).toEqual([]);
});

it("documents the canonical dashboard owner without retired editor instructions", () => {
  const readme = readFileSync(resolve(site, "README.md"), "utf8");
  const schema = contract.definitions.Documentation;
  const validate = new Ajv({ strict: true }).compile(schema);
  expect(validate(readme), JSON.stringify(validate.errors)).toBe(true);
  for (const rule of schema.allOf) {
    if (rule.not) expect(readme).not.toMatch(new RegExp(rule.not.pattern, "u"));
    else expect(readme).toMatch(new RegExp(rule.pattern!, "u"));
  }
});
