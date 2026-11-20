import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { TOML } from "bun";
import Ajv from "ajv";
import { localSessionBrokerSchema } from "@semio-tech/framework-os/modules/directory/testing/fixtures";
import { loadDevContribution } from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧩️contribution/📥️loading/🟦️.ts";
import contributionSchema from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧩️contribution/🧬️schema/🔣️.json";
import localHubSchema from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚀️local-hub/🧬️schema/🔣️.json";
import expected from "../🧫️fixtures/🧩️dev-contribution.json";

type Playground = {
  variant: string;
  app: string;
  ports: { react: number; wgpu: number };
  devContribution?: string;
};

/** 🧩️ Verifies that the Note editor lane selects its owned browser contribution. */
export function testNoteDevContributionV1(workspace: string): void {
  const manifestPath = "🌎️hub/🧩️compositions/🗒️note/🧩️component/📦️packages/🦀️rust/Cargo.toml";
  const manifest = TOML.parse(readFileSync(join(workspace, manifestPath), "utf8")) as {
    package: { metadata: { semio: { playground: Playground[] } } };
  };
  const lane = manifest.package.metadata.semio.playground.find(row => row.variant === expected.variant);
  assert.ok(lane, "Note editor playground must be declared");
  assert.equal(lane.app, expected.app);
  assert.deepEqual(lane.ports, expected.ports);
  assert.equal(lane.devContribution, `${expected.ownerRoot}/🧬️schema/🔣️.json`);

  const oracle = new Ajv({ strict: true }).addKeyword("x-semio-formats").addSchema(localSessionBrokerSchema).addSchema(localHubSchema).compile(contributionSchema);
  const declaration = JSON.parse(readFileSync(join(workspace, lane.devContribution), "utf8"));
  assert.ok(oracle(declaration), JSON.stringify(oracle.errors));
  const contribution = loadDevContribution(workspace, lane);
  assert.deepEqual(contribution, declaration);
  assert.equal(contribution?.ownerRoot, expected.ownerRoot);
  assert.equal(contribution?.viteConfig, `${expected.ownerRoot}/🏗️builder/🌐️vite/🟦️.ts`);
  assert.equal(contribution?.browserEntry, `${expected.ownerRoot}/🚀️entry/🟦️.ts`);
}
