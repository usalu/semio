// #region Header
/**
 * 🚧️ Third-party oracle (Ajv) for the hub's hostile-input law (`🧫️fixtures/🚧️hostile-input-v1`, Rust law
 * `every_route_answers_hostile_input_with_a_typed_signed_refusal`). Ajv — not the hub — decides that every
 * wrong-schema body the law sends is invalid against the route's own declared request schema, that every
 * refusal body the status table yields is a valid `HubRefusalV1`, and that a body naming a code outside the
 * declared vocabulary is not.
 */
// #endregion Header

import Ajv from "ajv";
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";

const repoRoot = join(dirname(fileURLToPath(import.meta.url)), "../../..");
const hubRoot = join(repoRoot, "🌎️hub");
const read = (...parts: string[]) => JSON.parse(readFileSync(join(...parts), "utf8"));
const fixture = read(hubRoot, "🧫️fixtures", "🚧️hostile-input-v1", "🔣️.json");
const refusal = read(hubRoot, "🚧️refusal", "🧬️schema", "🔣️.json");

/** 🧪️ Compiles one `$defs` entry of a schema module with the module as its own root, so `#/$defs/…` refs resolve. */
function compileDef(module: Record<string, unknown>, def: string) {
  const ajv = new Ajv({ allErrors: true, strict: false });
  ajv.addSchema({ ...module, $id: "urn:semio:module" });
  const validate = ajv.getSchema(`urn:semio:module#/$defs/${def}`);
  if (!validate) throw new Error(`no $defs/${def}`);
  return validate;
}

describe("hub hostile-input oracle", () => {
  it("every declared route body schema refuses the wrong-schema and malformed vectors", () => {
    const bodyRoutes = fixture.routes.filter((route: any) => route.body?.kind === "json");
    expect(bodyRoutes.length).toBeGreaterThanOrEqual(19);
    for (const route of bodyRoutes) {
      const validate = compileDef(read(repoRoot, route.body.schema.module), route.body.schema.def);
      expect(validate(fixture.bodies.wrongSchema), `${route.method} ${route.path} accepts the wrong-schema body`).toBe(false);
      expect(() => JSON.parse(fixture.bodies.malformedJson), "the malformed vector is not JSON").toThrow();
    }
  });

  it("every refusal the status table yields is a valid HubRefusalV1, and only those", () => {
    const validate = compileDef(refusal, "HubRefusalV1");
    const table = refusal.$defs.HubRefusalStatusCodesV1.const as Record<string, string>;
    for (const [status, code] of Object.entries(table)) {
      expect(validate({ schema: "semio.hub.refusal/v1", status: Number(status), code }), `${status} ${code}`).toBe(true);
    }
    expect(validate({ schema: "semio.hub.refusal/v1", status: 418, code: "refused" })).toBe(true);
    expect(validate({ schema: "semio.hub.refusal/v1", status: 403, code: "insecure-transport" })).toBe(true);
    expect(validate({ schema: "semio.hub.refusal/v1", status: 404, code: "teapot" })).toBe(false);
    expect(validate({ schema: "semio.hub.refusal/v1", status: 200, code: "not-found" })).toBe(false);
    expect(validate({ schema: "semio.hub.refusal/v1", status: 404, code: "not-found", detail: "leak" })).toBe(false);
  });

  it("the fixture's oversized vector exceeds every declared body limit", () => {
    expect(fixture.oversizedBytes).toBeGreaterThan(2 * 1024 * 1024);
    expect(fixture.hostilePathSegments.some((segment: string) => segment.length > 1024)).toBe(true);
  });
});
