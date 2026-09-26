// #region Header
/**
 * 📊️ Third-party oracle (Ajv) for the hub's observability body (`🌎️hub/📊️observability/🧬️schema`, Rust laws
 * `the_observability_body_is_the_declared_schema` and `route_metrics_count_answers_by_class_and_take_percentiles_of_the_last_64_samples`).
 * Ajv — not the hub — decides that the fixture's complete body is a valid `HubObservabilityV1` (its residency
 * resolved through the trusted-catalog module's own `TrustedCatalogGuestResidencyStateV1`), that every invalid
 * body is refused, and this file recomputes the fixture's route rows from its observations independently.
 */
// #endregion Header

import Ajv from "ajv";
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import { HUB_OBSERVABILITY_SCHEMA, type HubObservabilityV1 } from "../../📊️observability/🟦️.ts";

const hubRoot = join(dirname(fileURLToPath(import.meta.url)), "../..");
const read = (...parts: string[]) => JSON.parse(readFileSync(join(hubRoot, ...parts), "utf8"));
const fixture = read("🧫️fixtures", "📊️observability-v1", "🔣️.json");
const observability = read("📊️observability", "🧬️schema", "🔣️.json");
const trustedCatalog = read("🗿️artifact-authority", "🔏️trusted-catalog", "🧬️schema", "🔣️.json");
const directory = read("..", "🧰️framework", "🛍️products", "💻️os", "🔨️modules", "📇️directory", "🧬️schema", "🔣️.json");

/** 🧪️ Compiles `HubObservabilityV1` with the trusted-catalog module and the directory module it references registered under their own `$id`s. */
function compileBody() {
  const ajv = new Ajv({ allErrors: true, strict: false });
  ajv.addSchema(directory);
  ajv.addSchema(trustedCatalog);
  const validate = ajv.compile(observability);
  return validate;
}

type Observation = { method: string; route: string; status: number; durationUs: number };

/** 🛣️ The route table recomputed from the observations: classes by status, percentiles over the last 64 clamped samples. */
function routeRows(observations: Observation[]) {
  const rows = new Map<string, { method: string; route: string; requests: number; successes: number; clientRefusals: number; rateLimited: number; unavailable: number; serverFailures: number; latency: number[]; maxUs: number }>();
  for (const { method, route, status, durationUs } of observations) {
    const key = JSON.stringify([route, method]);
    const row = rows.get(key) ?? { method, route, requests: 0, successes: 0, clientRefusals: 0, rateLimited: 0, unavailable: 0, serverFailures: 0, latency: [], maxUs: 0 };
    row.requests += 1;
    if (status < 400) row.successes += 1;
    else if (status === 429) row.rateLimited += 1;
    else if (status < 500) row.clientRefusals += 1;
    else if (status === 503) row.unavailable += 1;
    else row.serverFailures += 1;
    const sample = Math.min(durationUs, 4294967295);
    row.latency = [...row.latency, sample].slice(-64);
    row.maxUs = Math.max(row.maxUs, sample);
    rows.set(key, row);
  }
  const percentile = (samples: number[], p: number) => {
    const sorted = [...samples].sort((left, right) => left - right);
    return sorted[Math.min(sorted.length - 1, Math.floor(sorted.length * p))];
  };
  return [...rows.entries()]
    .sort(([left], [right]) => (left < right ? -1 : left > right ? 1 : 0))
    .map(([, { latency, ...row }]) => ({ method: row.method, route: row.route, requests: row.requests, successes: row.successes, clientRefusals: row.clientRefusals, rateLimited: row.rateLimited, unavailable: row.unavailable, serverFailures: row.serverFailures, samples: latency.length, p50Us: percentile(latency, 0.5), p95Us: percentile(latency, 0.95), p99Us: percentile(latency, 0.99), maxUs: row.maxUs }));
}

describe("hub observability oracle", () => {
  it("the fixture's complete body is a valid HubObservabilityV1", () => {
    const validate = compileBody();
    expect(validate(fixture.body), JSON.stringify(validate.errors)).toBe(true);
    const body: HubObservabilityV1 = fixture.body;
    expect(body.schema).toBe(HUB_OBSERVABILITY_SCHEMA);
    expect(Object.keys(body).sort()).toEqual([...observability.$defs.HubObservabilityV1.required].sort());
  });

  it("every invalid body is refused", () => {
    const validate = compileBody();
    for (const { name, body } of fixture.invalidBodies) expect(validate(body), name).toBe(false);
  });

  it("the route rows follow from the observations", () => {
    expect(routeRows(fixture.observations)).toEqual(fixture.routes);
    expect(fixture.body.routes).toEqual(fixture.routes);
  });
});
