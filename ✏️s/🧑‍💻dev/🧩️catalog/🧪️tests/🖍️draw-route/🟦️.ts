import { expect, test } from "bun:test";
import { ok } from "node:assert/strict";
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { parse as parseToml } from "smol-toml";
import Ajv from "ajv";
import { findWorkspaceRoot } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { parsePlaygroundBlock } from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🎮️playground/🔎️discovery/🟦️.ts";
import { loadDevContribution, resolveDevContributionFile } from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧩️contribution/📥️loading/🟦️.ts";
import fixture from "./🧫️fixtures/🔣️.json" with { type: "json" };

test("Draw declares a Specific catalog browser route for the drawing editor", () => {
  const workspace = findWorkspaceRoot();
  const read = (file: string) => readFileSync(join(workspace, file), "utf8");
  const authored = read(fixture.manifest);
  const manifest = parseToml(authored) as { package: { metadata: { semio: { playground: unknown[] } } } };
  const block = authored.split("[[package.metadata.semio.playground]]")[1].split("\n[")[0];
  const route = parsePlaygroundBlock(block, fixture.pluginId, dirname(fixture.manifest));
  const expected = { variant: fixture.variant, app: fixture.app, ports: fixture.ports, devContribution: fixture.devContribution };
  expect(route).toMatchObject(expected);
  expect(manifest.package.metadata.semio.playground).toContainEqual(expect.objectContaining(expected));

  ok(route);
  const contribution = loadDevContribution(workspace, route);
  expect(contribution).toEqual({ ownerRoot: fixture.ownerRoot, viteConfig: fixture.viteConfig, browserEntry: fixture.browserEntry });
  const os = "🧰️framework/🛍️products/💻️os/🔨️modules";
  const dev = `${os}/🧑‍💻dev`;
  const oracle = new Ajv({ strict: true });
  oracle.addKeyword({ keyword: "x-semio-formats", schemaType: "array", valid: true });
  oracle.addSchema(JSON.parse(read(`${os}/📇️directory/🎫️local-session/🗄️broker/🧬️schema/🔣️.json`)));
  oracle.addSchema(JSON.parse(read(`${dev}/🚀️local-hub/🧬️schema/🔣️.json`)));
  const validates = oracle.compile(JSON.parse(read(`${dev}/🧩️contribution/🧬️schema/🔣️.json`)));
  expect(validates(JSON.parse(read(fixture.devContribution)))).toBe(true);

  for (const file of [fixture.viteConfig, fixture.browserEntry]) {
    expect(resolveDevContributionFile(workspace, file, fixture.ownerRoot)).toBe(join(workspace, file));
  }
});
