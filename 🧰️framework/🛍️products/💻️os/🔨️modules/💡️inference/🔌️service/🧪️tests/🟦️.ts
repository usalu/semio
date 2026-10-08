import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import Ajv from "ajv";
import fixture from "../🧫️fixtures/🔣️.json" with { type: "json" };
import { DOCUMENT_SERVICE_TOPIC_V1, InstalledServiceRegistryV1, boundedServicePayloadV1, admitDocumentServiceDeclarationV1, parseInstalledServiceOperationV1, type InstalledServiceDriverV1 } from "../🟦️.ts";
import {compileDocumentSchemaV1} from "../../../📇️directory/🔌️client/🌐️document-http/🧬️schema/🟦️.ts";
import {documentServiceRequestV1} from "../🚪️io/🟦️.ts";

/** ⚖️ Language-neutral service laws with independent Ajv admission and exact JSON observations. */
export function verifyInstalledServiceLawsV1(): Readonly<Record<string, number>> {
  const schema = JSON.parse(readFileSync(new URL("../🧬️schema/🔣️.json", import.meta.url), "utf8"));
  const oracle = new Ajv({ strict: true }).compile(schema);
  for (const vector of fixture.operations) {
    assert.equal(oracle(vector.value), vector.valid, vector.name);
    let accepted = false;
    try { parseInstalledServiceOperationV1({ kind: "service-operation", ...vector.value }); accepted = true; } catch {}
    assert.equal(accepted, vector.valid, vector.name);
  }
  const counts = { neutralCalls: 0, secondaryCalls: 0, neutralRetired: 0, secondaryRetired: 0, absentDispatches: 0 };
  const registry = new InstalledServiceRegistryV1();
  const make = (owner: "neutral" | "secondary"): InstalledServiceDriverV1 => ({ owner, serviceId: `${owner}.service`, dispatch() { counts[`${owner}Calls`]++; }, retire() { counts[`${owner}Retired`]++; }, sessionRetired() {}, documentClosed() {}, documentRebootstrapped() {}, documentMounted() {} });
  const removers = new Map<string, () => void>();
  for (const event of fixture.lifecycle) {
    const [action, owner] = event.split("-") as [string, "neutral" | "secondary"];
    if (action === "install") removers.set(owner, registry.install(make(owner)));
    else if (action === "remove") { removers.get(owner)!(); removers.get(owner)!(); }
    else if (!registry.dispatch({ owner, serviceId: `${owner}.service`, action: "run", operationEpoch: 1, payload: fixture.request })) counts.absentDispatches++;
  }
  assert.deepEqual(counts, fixture.expected);
  const source = (owner: string, direction: string) => ({ $schema: "http://json-schema.org/draft-07/schema#", $id: `${owner}:${direction}`, type: "object", additionalProperties: false, required: ["value"], properties: { value: { type: "integer", minimum: 0, maximum: 16 } } });
  const declaration = admitDocumentServiceDeclarationV1("neutral", { schema: DOCUMENT_SERVICE_TOPIC_V1, owner: "neutral", serviceId: "neutral.service", operations: [{ action: "run", method: "POST", route: ["contributed", "neutral", "run"], sendBody: true, cursorField: null, requestMaxBytes: 1024, responseMaxBytes: 1024, inputSchema: source("neutral", "request"), outputSchema: source("neutral", "reply") }] });
  assert.equal(documentServiceRequestV1(declaration, fixture.scope, "run", fixture.request).path, fixture.path);
  assert.throws(() => documentServiceRequestV1(declaration, fixture.scope, "foreign", fixture.request));
  assert.throws(() => documentServiceRequestV1(declaration, fixture.scope, "run", { value: 17 }));
  assert.throws(() => admitDocumentServiceDeclarationV1("secondary", declaration));
  assert.throws(() => boundedServicePayloadV1({ value: NaN }));
  assert.throws(() => boundedServicePayloadV1({ value: "x".repeat(16 * 1024) }));
  assert.throws(() => boundedServicePayloadV1(new Date()));
  const validate = compileDocumentSchemaV1(declaration.operations[0]!.inputSchema), ajv = new Ajv({ strict: true }).compile(declaration.operations[0]!.inputSchema);
  for (const value of [fixture.request, { value: -1 }, { value: 17 }, { value: 1.5 }, { value: 7, authority: "forged" }, {}]) assert.equal(validate(value), ajv(value));
  assert.throws(() => compileDocumentSchemaV1({type:'string',format:'unknown'}));
  assert.throws(() => compileDocumentSchemaV1({$ref:'#/missing'}));
  return { envelopeVectors: fixture.operations.length, lifecycleEvents: fixture.lifecycle.length, jsonOracleVectors: 6, transportHostiles: 8 };
}
