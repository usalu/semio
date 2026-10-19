import type { ScriptInvocation } from "@semio-tech/process/routing";
import { buildBudgetMs } from "@semio-tech/process/budget";
import { runCmd, devToolingEnv } from "@semio-tech/process/execution";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { strict as assert } from "node:assert";
import Ajv from "ajv";

/** 🏃️ Runs the `InferenceTransportCheckScript` command under the caller's roots and invocation. */
export function runInferenceTransportCheckScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>): void {
  const root = resolve(context.repoRoot, "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/💡️inference/🔌️client");
  const fixture = JSON.parse(readFileSync(resolve(root, "🧪️tests/🧫️fixtures/🔣️.json"), "utf8"));
  const validators = new Map<string, ReturnType<Ajv["compile"]>>();
  const ajv = new Ajv({ strict: false });
  for (const vector of fixture.schemaVectors) {
    let validate = validators.get(vector.schema);
    if (!validate) {
      validate = ajv.compile(JSON.parse(readFileSync(resolve(root, "🧬️schema", vector.schema), "utf8")));
      validators.set(vector.schema, validate);
    }
    assert.equal(validate(vector.value), vector.valid, vector.name);
  }
  for (const law of ["inference_client::tests::production_manifest_installs_owner_transport_and_schema_vectors", "inference_client::tests::typed_owner_transport_preserves_proposal_binding_and_closed_geometry"]) {
    runCmd("cargo", ["test", "--manifest-path", resolve(context.repoRoot, "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/Cargo.toml"), "-p", "semio-s-artifact-gis-gismap", "--lib", "--", law, "--exact", "--nocapture"], { cwd: context.repoRoot, env: devToolingEnv(), budgetMs: buildBudgetMs() });
  }
  console.log("GIS document-http: Ajv schema parity, owner manifest injection, typed submit/events/approval and geometry binding passed");
}
