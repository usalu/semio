// #region Header
/**
 * 📊️ Third-party oracle (Ajv) for the hub's observability body (`🌎️hub/📊️observability/🧬️schema`, Rust laws
 * `the_observability_body_is_the_declared_schema` and `route_metrics_count_answers_by_class_and_take_percentiles_of_the_last_64_samples`).
 * Ajv — not the hub — decides that the fixture's complete body is a valid `HubObservabilityV1` (its residency
 * resolved through the trusted-catalog module's own `TrustedCatalogGuestResidencyStateV1`), that every invalid
 * body is refused, and this file recomputes the fixture's route rows from its observations independently.
 *
 * 📖️ It also holds the hub's README to what the hub serves (acceptance row 4.9): every health route the README documents
 * is registered by the router and documents exactly its body's schema fields (liveness, readiness, observability incl.
 * its event and route rows), the README's claim that there is no scrape endpoint holds for the router, the trace levels
 * it names are the trace module's, and its environment tables name exactly the variables the hub reads. The served
 * bodies are held to those same schemas by the Rust laws `the_served_liveness_body_is_the_declared_liveness_schema`,
 * `every_served_readiness_body_is_the_declared_readiness_schema` and `the_observability_body_is_the_declared_schema`.
 */
// #endregion Header

import Ajv from "ajv";
import { readdirSync, readFileSync } from "node:fs";
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

/** 📖️ The fields one README `{…}` body documents: each name, whether it is always present (`name?` marks a field present
 * only sometimes), and for `name: [{…}]` the fields of that list's rows. */
type DocumentedFields = { readonly fields: Map<string, boolean>; readonly rows: Map<string, DocumentedFields> };

/** 📖️ Parses one README body notation `{a, b?, c: "const", d: [{e, f}]}`. */
function documentedFields(notation: string): DocumentedFields {
  const inner = notation.trim().replace(/^\{/u, "").replace(/\}$/u, "");
  const entries: string[] = [];
  let [depth, quoted, start] = [0, false, 0];
  for (let at = 0; at < inner.length; at += 1) {
    const char = inner[at]!;
    if (char === '"') quoted = !quoted;
    else if (!quoted && (char === "[" || char === "{")) depth += 1;
    else if (!quoted && (char === "]" || char === "}")) depth -= 1;
    else if (!quoted && depth === 0 && char === ",") {
      entries.push(inner.slice(start, at));
      start = at + 1;
    }
  }
  entries.push(inner.slice(start));
  const documented: DocumentedFields = { fields: new Map(), rows: new Map() };
  for (const entry of entries.map((text) => text.trim()).filter(Boolean)) {
    const colon = entry.indexOf(":");
    const name = (colon < 0 ? entry : entry.slice(0, colon)).trim();
    documented.fields.set(name.replace(/\?$/u, ""), !name.endsWith("?"));
    const list = colon < 0 ? null : /^\[(\{.*\})\]$/su.exec(entry.slice(colon + 1).trim());
    if (list) documented.rows.set(name.replace(/\?$/u, ""), documentedFields(list[1]!));
  }
  return documented;
}

type SchemaDefinition = { properties: Record<string, { items?: { $ref?: string } }>; required?: string[] };

/** 📖️ Asserts `documented` names exactly `definition`'s properties, with `?` exactly on the optional ones. */
function expectDocumentedAs(documented: DocumentedFields | undefined, definition: SchemaDefinition, label: string) {
  expect(documented, `${label} is documented`).toBeDefined();
  expect([...documented!.fields.keys()].sort(), label).toEqual(Object.keys(definition.properties).sort());
  for (const [name, always] of documented!.fields) expect(always, `${label}.${name} is ${always ? "always" : "sometimes"} present`).toBe((definition.required ?? []).includes(name));
}

const readme = readFileSync(join(hubRoot, "README.md"), "utf8");

/** 📖️ One `## ` section of the README, heading included. */
function readmeSection(heading: string): string {
  const start = readme.indexOf(`\n## ${heading}\n`);
  expect(start, `README section ${heading}`).toBeGreaterThanOrEqual(0);
  const end = readme.indexOf("\n## ", start + 1);
  return readme.slice(start, end < 0 ? undefined : end);
}

/** 🛣️ The `.route("…")` paths the hub's router registers (the boot server's included). */
function registeredRoutes(): Set<string> {
  const source = readFileSync(join(hubRoot, "🏗️bootstrap", "🦀️.rs"), "utf8");
  return new Set([...source.matchAll(/\.route\(\s*"([^"]+)"/gu)].map((match) => match[1]!));
}

/** 🌐️ Every `"OS_HUB_…"` a non-test hub source reads, plus the process variables of the framework modules the hub hosts,
 * named by their own constants: the trace sink and level, the database driver runtime's thread count. */
function environmentVariablesRead(): Set<string> {
  const names = new Set<string>();
  const walk = (directory: string) => {
    for (const entry of readdirSync(directory, { withFileTypes: true })) {
      if (entry.isDirectory() && !entry.name.startsWith(".") && !["node_modules", "dist", "target", "🗑️generated", "🧪️tests"].includes(entry.name)) walk(join(directory, entry.name));
      else if (entry.isFile() && entry.name === "🦀️.rs") for (const match of readFileSync(join(directory, entry.name), "utf8").matchAll(/"(OS_HUB_[A-Z0-9_]+)"/gu)) names.add(match[1]!);
    }
  };
  walk(hubRoot);
  const constant = (file: string[], name: string) => {
    const match = new RegExp(`const ${name}: &str = "([A-Z0-9_]+)";`, "u").exec(readFileSync(join(hubRoot, "..", ...file), "utf8"));
    expect(match, `${name} in ${file.join("/")}`).not.toBeNull();
    names.add(match![1]!);
  };
  const trace = ["🧰️framework", "🔨️modules", "⏱️trace", "📝️record", "🦀️.rs"];
  constant(trace, "TRACE_LEVEL_ENV");
  constant(trace, "TRACE_SINK_ENV");
  constant(["🧰️framework", "🛍️products", "💻️os", "🔨️modules", "🛢️db", "🗄️storage", "🧵️driver-runtime", "🦀️.rs"], "DRIVER_THREADS_ENV");
  return names;
}

describe("hub README parity", () => {
  const localBootstrap = read("🚀️local-bootstrap", "🧬️schema", "🔣️.json");
  const health = readmeSection("Health endpoints");
  const documented = new Map(
    [...health.matchAll(/^\| `(GET|POST) (\S+)` \| (.*) \|$/gmu)].map((row) => [`${row[1]} ${row[2]}`, /`(\{[^`]*\})`/u.exec(row[3]!)?.[1]] as const),
  );

  it("documents exactly the fields of every health body its schema declares", () => {
    expect([...documented.keys()]).toEqual(["GET /healthz", "GET /readyz", "GET /admin/api/observability"]);
    expectDocumentedAs(documentedFields(documented.get("GET /healthz")!), localBootstrap.$defs.LocalBootstrapLivenessV1, "/healthz");
    expectDocumentedAs(documentedFields(documented.get("GET /readyz")!), localBootstrap.$defs.LocalBootstrapReadinessV1, "/readyz");
    const body = documentedFields(documented.get("GET /admin/api/observability")!);
    const definitions = observability.$defs as Record<string, SchemaDefinition>;
    expectDocumentedAs(body, definitions.HubObservabilityV1!, "/admin/api/observability");
    for (const [name, rows] of body.rows) {
      const reference = definitions.HubObservabilityV1!.properties[name]?.items?.$ref;
      expect(reference, `${name} rows reference a definition`).toMatch(/^#\/\$defs\//u);
      expectDocumentedAs(rows, definitions[reference!.slice("#/$defs/".length)]!, `/admin/api/observability ${name}[]`);
    }
    expect(health, "the startup object's documented fields").toContain(`{${Object.keys(localBootstrap.$defs.startupProgress.properties).map((name) => (localBootstrap.$defs.startupProgress.required.includes(name) ? name : `${name}?`)).join(", ")}}`);
    expect(health, "the closed-gate list's field name").toContain("`blockedBy` is a");
  });

  it("documents a route the router registers for every endpoint, and no scrape endpoint", () => {
    const routes = registeredRoutes();
    for (const route of documented.keys()) expect(routes.has(route.split(" ")[1]!), route).toBe(true);
    expect([...routes].filter((route) => /metric|prometheus|scrape/iu.test(route)), "the README says there is no scrape endpoint").toEqual([]);
    expect(health).toContain("no scrape endpoint");
  });

  it("names the trace levels the trace module accepts", () => {
    const source = readFileSync(join(hubRoot, "..", "🧰️framework", "🔨️modules", "⏱️trace", "📝️record", "🦀️.rs"), "utf8");
    const spellings = source.slice(source.indexOf("impl TraceLevel {"), source.indexOf("pub fn parse", source.indexOf("impl TraceLevel {")));
    const levels = [...spellings.matchAll(/Self::[A-Z][a-z]+ => "([a-z]+)",/gu)].map((match) => match[1]!);
    expect(levels).toEqual(["off", "error", "warn", "info", "debug"]);
    expect(health).toContain(`\`SEMIO_TRACE_LEVEL\` (\`${levels.join("|")}\``);
  });

  it("documents exactly the environment variables the hub reads", () => {
    const tabled = new Set([...readmeSection("Environment variables").matchAll(/^\| `((?:OS_HUB|SEMIO)_[A-Z0-9_]+)` \|/gmu)].map((match) => match[1]!));
    expect([...tabled].sort()).toEqual([...environmentVariablesRead()].sort());
  });
});
