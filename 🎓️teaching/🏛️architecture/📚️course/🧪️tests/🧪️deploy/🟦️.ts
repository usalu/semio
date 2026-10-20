/** 🧪️ The split deployment agrees with its one authored source (`🚀️deploy/🔣️.json`, held to its JSON Schema by the
 * third-party `ajv`): nothing in the repository drifts from it and every seeded drift is found; the Dockerfile bakes the
 * production gate into a shell-less image of one pinned runtime base; `compose.yaml` (parsed by the third-party `yaml` library) runs
 * the proctor unexposed, hardened, limited and log-rotated behind a pinned Caddy; the Caddyfile serves the proctor host with
 * a body cap, timeouts and response headers and loads the operator's certificates from the directory the stack mounts
 * read-only; the workflow (parsed by `yaml`) releases from the repository's default branch only with pinned actions and the least permissions;
 * `publish` accepts exactly a CDN artifact whose document is sealed for the baked proctor origin, with hashed assets, no
 * development leftover and within its size budget; and the bundle for the proctor host pins its tag, lists exactly the
 * defaults of `compose.yaml` and tells every step with the hosts of the one source. The readiness gate type-checks the
 * package's sources as its second step — the steps, the package's targets and the README agree.
 * @see ../../🚀️deploy/🔣️.json — the hosts, image and port
 * @see ../../🚀️deploy/🧬️schema/🔣️.json — their contract
 * @see ../../🚀️deploy/🟦️.ts — the operator verbs under test */
import { createHash } from "node:crypto";
import { mkdtempSync, realpathSync, mkdirSync, readFileSync, rmSync, statSync, utimesSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { afterEach, describe, expect, it } from "vitest";
import { writeAssetArtifacts } from "../../../../../🧰️framework/🔨️modules/🖼️assets/🏗️builder/📦️publication/🟦️.ts";
import publication from "../../../../../🧰️framework/🔨️modules/🖼️assets/🏗️builder/📦️publication/🧪️tests/🧫️fixtures/🔣️.json" with { type: "json" };
import { scratch } from "./🧰️deploy-fixtures/🟦️.ts";
import { registerCancellationAndStagingTests } from "./🛑️cancellation-and-staging/🟦️.ts";
import { registerDockerContextAndImageTests } from "./🐳️docker-context-and-image/🟦️.ts";
import { registerComposeCaddyAndWorkflowTests } from "./🧩️compose-caddy-and-workflow/🟦️.ts";
import { registerSealedPublicationAndBundleTests } from "./🌐️sealed-publication-and-bundle/🟦️.ts";

for (const row of publication.cases) it(`publishes ${row.id} assets without rewriting unchanged files`, () => {
  const parent = process.env.SEMIO_TICKET_DIR ? join(process.env.SEMIO_TICKET_DIR, "🗑️generated", "nx", "asset-publication") : join(realpathSync(tmpdir()), "asset-publication");
  mkdirSync(parent, { recursive: true });
  const root = mkdtempSync(join(parent, "run-"));
  scratch.push(root);
  const file = join(root, "asset.txt");
  const before = new Date("2000-01-01T00:00:00Z");
  if (row.before !== null) { writeFileSync(file, row.before); utimesSync(file, before, before); }
  writeAssetArtifacts([{ path: file, content: row.output }]);
  expect(createHash("sha256").update(readFileSync(file)).digest("hex")).toBe(createHash("sha256").update(Buffer.from(row.output, "utf8")).digest("hex"));
  expect(statSync(file).mtimeMs !== before.getTime()).toBe(row.rewrite);
});

afterEach(() => {
  for (const root of scratch.splice(0)) rmSync(root, { recursive: true, force: true });
});

describe("architecture course deployment", () => {
  registerCancellationAndStagingTests();
  registerDockerContextAndImageTests();
  registerComposeCaddyAndWorkflowTests();
  registerSealedPublicationAndBundleTests();
});
