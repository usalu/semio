/** 🎚️ Owner-local course forms preserve their argument boundaries and current parser semantics.
 * @see ../../../🧫️fixtures/🎛️dashboard/🎚️parameters/🔣️.json */
import Ajv from "ajv";
import Ajv2020 from "ajv/dist/2020";
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { parseArgs } from "node:util";
import { expect, it } from "vitest";
import { endToEndPlan } from "../../../🎭️e2e/🟦️.ts";
import { BundleScript, ScriptRouter, runScriptMain, createScriptProcessEnvelope, withScriptProcessEnvelope } from "../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { selectBuildTasksV1 } from "../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🏗️build/🧭️selection/🟦️.ts";
import fixture from "../../../🧫️fixtures/🎛️dashboard/🎚️parameters/🔣️.json" with { type: "json" };
import schema from "../../../🧫️fixtures/🎛️dashboard/🎚️parameters/🧬️schema/🔣️.json" with { type: "json" };

type Effect = { args?: string[]; env?: Record<string, string> };
type Parameter = Effect & { id: string; kind: "choice" | "flag" | "text"; title?: Record<string, string>; description?: Record<string, string>; default?: string | boolean; valueFlag?: string; values?: (Effect & { id: string })[] };
const site = resolve(dirname(fileURLToPath(import.meta.url)), "../../..");
const workspace = resolve(site, "../../..");
const project = JSON.parse(readFileSync(resolve(site, "📦️packages/🟦️typescript/📋️project.json"), "utf8"));
const registry = JSON.parse(readFileSync(resolve(workspace, "🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧬️schema/🎮️registry/🔣️.json"), "utf8"));
const parameters = (target: string): Parameter[] => project.targets[target].metadata?.semio?.dashboard?.parameters ?? [];

it("admits language-neutral course form vectors and the current registry declaration", () => {
  const validate = new Ajv({ strict: true }).compile(schema);
  expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
  const owner = new Ajv2020({ strict: false }).compile({ ...registry, $ref: "#/$defs/ProjectManifest" });
  expect(owner(project), JSON.stringify(owner.errors)).toBe(true);
  for (const [target, ids] of Object.entries(fixture.targets)) {
    expect(parameters(target).map(parameter => parameter.id)).toEqual(ids);
    expect(project.targets[target].options.forwardAllArgs).toBe(true);
    for (const parameter of parameters(target)) {
      expect(Object.keys(parameter.title ?? {}).sort(), `${target}:${parameter.id}`).toEqual(["de", "en"]);
      if (parameter.description) expect(Object.keys(parameter.description).sort()).toEqual(["de", "en"]);
    }
  }
});

for (const row of fixture.cases) it(`projects course form ${row.id} into the parser's exact argv`, () => {
  const args: string[] = [], env: Record<string, string> = {};
  for (const parameter of parameters(row.target)) {
    const value = (row.values as Record<string, string | boolean>)[parameter.id] ?? parameter.default;
    if (value === undefined || value === false) continue;
    if (parameter.kind === "text") {
      expect(parameter.valueFlag).toBeDefined();
      args.push(parameter.valueFlag!, String(value));
      continue;
    }
    const effect = parameter.kind === "choice" ? parameter.values?.find(choice => choice.id === value) : parameter;
    expect(effect, `${row.id}:${parameter.id}`).toBeDefined();
    args.push(...(effect?.args ?? []));
    Object.assign(env, effect?.env);
  }
  args.push(...row.extra);
  expect(args).toEqual(row.args);
  expect(env).toEqual(row.env);
  if (row.target === "test-e2e") {
    const plan = endToEndPlan(args);
    expect({ ...plan, proctor: plan.proctor ?? null }).toEqual(row.parsed);
    const boundary = args.indexOf("--");
    expect(boundary).toBeGreaterThanOrEqual(0);
    const prefix = parseArgs({ args: args.slice(0, boundary), options: { topology: { type: "string", default: "both" }, serial: { type: "boolean", default: false }, keep: { type: "boolean", default: false }, proctor: { type: "string" } } }).values;
    expect({ topologies: prefix.topology === "both" ? ["dev", "rehearsal"] : [prefix.topology], serial: prefix.serial, keep: prefix.keep, proctor: prefix.proctor ?? null, playwright: args.slice(boundary + 1) }).toEqual(row.parsed);
  } else {
    const options: Record<string, { type: "string" | "boolean"; default?: boolean }> = row.target === "docker-stack-bundle" ? { tag: { type: "string" }, out: { type: "string" } } : { tag: { type: "string" }, "http-port": { type: "string" }, "https-port": { type: "string" }, keep: { type: "boolean", default: false } };
    const parsed = parseArgs({ args, options }).values;
    expect({ ...parsed }).toEqual(row.parsed);
    if (row.target === "docker-stack-bundle") {
      const cwd = resolve(workspace, project.targets[row.target].options.cwd);
      expect(cwd).toBe(resolve(site, "📦️packages/🟦️typescript"));
      if (row.id === "bundle-defaults") expect(resolve(cwd, String(parsed.out))).toBe(resolve(cwd, "dist/proctor"));
    }
  }
});

for (const row of fixture.invalid) it(`rejects unseparated or incomplete e2e controls ${JSON.stringify(row.args)}`, () => {
  expect(() => endToEndPlan(row.args)).toThrow(row.error);
});

for (const row of fixture.cases.filter(row => row.target === "test-e2e")) it(`parses the explicit boundary vector ${row.id}`, () => {
  const plan = endToEndPlan(row.args);
  expect({ ...plan, proctor: plan.proctor ?? null }).toEqual(row.parsed);
});

it("retains the forwarding boundary through the repository selector and Script router", async () => {
  await withScriptProcessEnvelope(createScriptProcessEnvelope({ version: 1, owner: "course:argument-boundary-test", maximumElapsedMilliseconds: 5000 }, {}, Date.now()), async invocation => {
    const target = project.targets["test-e2e"];
    const graph = { version: 1 as const, projects: { [project.name]: { name: project.name, root: target.options.cwd, targets: { "test-e2e": { execution: { kind: "command" as const, command: target.options.command, cwd: target.options.cwd, forwardAllArgs: true } } } } }, dependencies: [] };
    for (const row of fixture.cases.filter(row => row.target === "test-e2e")) {
      const selection = await selectBuildTasksV1(workspace, graph, ["run", `${project.name}:test-e2e`, "--", ...row.args], invocation);
      expect(selection.args).toEqual(row.args);
      let received: string[] | undefined;
      class CaptureCourseArguments extends BundleScript {
        run(segments: string[]): void {
          received = segments;
          const plan = endToEndPlan(segments);
          expect({ ...plan, proctor: plan.proctor ?? null }).toEqual(row.parsed);
        }
      }
      const router = new ScriptRouter(resolve(workspace, target.options.cwd), workspace).register("test-e2e", CaptureCourseArguments);
      await runScriptMain(router, { invocation, argv: ["test-e2e", ...selection.args] });
      expect(received).toEqual(row.args);
    }
  });
});
