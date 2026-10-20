/** 🧾️ The manifests of the continuous targets declare the parameters the neutral fixture names, and each chosen parameter is plain environment that moves the declared readiness to the port the stack listens on. Ajv validates the fixture.
 * @see ../../../🧫️fixtures/🟢️continuous/🔣️.json */
import Ajv from "ajv";
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { expect, it } from "vitest";
import contract from "../../../🧫️fixtures/🟢️continuous/🔣️.json" with { type: "json" };
import contractSchema from "../../../🧫️fixtures/🟢️continuous/🧬️schema/🔣️.json" with { type: "json" };
import { resolveReadinessV1 } from "../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🟢️readiness/🟦️.ts";

type Parameter = { id: string; kind: "flag" | "choice"; env?: Record<string, string>; values?: { id: string; env?: Record<string, string> }[] };
type Target = { metadata?: { semio?: { dashboard?: { ready?: { port?: number; portEnv?: string }; parameters?: Parameter[] } } } };
const here = dirname(fileURLToPath(import.meta.url));
const read = (path: string): { name: string; targets: Record<string, Target> } => JSON.parse(readFileSync(resolve(here, path), "utf8"));
const projects = [read("../../../📦️packages/🟦️typescript/📋️project.json"), read("../../../../../🛂️proctor/📦️packages/🦀️rust/📋️project.json")];
const declared = (id: string): Target => {
  const [name, target] = id.split(":") as [string, string];
  return projects.find(project => project.name === name)!.targets[target]!;
};
const derive = (target: Target, choices: Record<string, string>): Record<string, string> => {
  const environment: Record<string, string> = {};
  for (const parameter of target.metadata?.semio?.dashboard?.parameters ?? []) {
    const chosen = choices[parameter.id];
    if (chosen === undefined) continue;
    Object.assign(environment, parameter.kind === "flag" ? parameter.env : parameter.values!.find(value => value.id === chosen)!.env);
  }
  return environment;
};

it("declares exactly the parameters the fixture names, on every continuous target", () => {
  expect(new Ajv({ strict: false }).compile(contractSchema)(contract)).toBe(true);
  for (const [id, names] of Object.entries(contract.parameters)) {
    expect((declared(id).metadata?.semio?.dashboard?.parameters ?? []).map(parameter => parameter.id).sort(), id).toEqual([...names].sort());
    expect(declared(id).metadata?.semio?.dashboard?.ready, id).toEqual(contract.targets[id as keyof typeof contract.targets].ready);
  }
});

it("turns every parameter choice into the environment and the ready port the fixture states", () => {
  for (const row of contract.cases) {
    for (const [id, port] of Object.entries(row.ports)) {
      const environment = derive(declared(id), row.choices);
      for (const [key, value] of Object.entries(environment)) expect(row.environment[key as keyof typeof row.environment], `${row.id} ${id} ${key}`).toBe(value);
      if (id === "@teaching/architecture-course:dev") expect(environment, `${row.id} ${id}`).toEqual(row.environment);
      const ready = declared(id).metadata!.semio!.dashboard!.ready!;
      expect(resolveReadinessV1(ready, environment, id).port, `${row.id} ${id}`).toBe(port);
    }
  }
});
