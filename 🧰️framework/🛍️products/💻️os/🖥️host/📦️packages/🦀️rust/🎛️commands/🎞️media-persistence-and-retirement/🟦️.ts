/** 🎞️ The media projection, persistence contract and document retirement checks of the OS host router.
 *
 * @see ../../📜️script.ts — the package script that registers the commands of this module
 */
import { scriptCompilerAdmissionControlV1 as exactCargoCallerControlV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { configuredExactCargoLawPolicyV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️laws/🟦️.ts";
import { join } from "node:path";
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import Ajv, { type ValidateFunction } from "ajv";
import { runRepositoryExactCargoLaws, configuredRepositoryCargoPreparationPolicyV1 } from "../../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";

/** 🧬️ A compiled owned-schema export, typed as a boolean runtime check so `assert` never narrows its validated subject to `unknown`. */
export type SchemaCheck = ((data: unknown) => boolean) & Pick<ValidateFunction, "errors">;

export class MediaProjectionScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.some(segment => segment !== "--oracle-only")) throw new Error("media-projection-check accepts only --oracle-only");
    const owner = join(this.root, "../..");
    const fixture = JSON.parse(readFileSync(join(owner, "🧪️tests/🧫️fixtures/🕸️media-projection/🔣️.json"), "utf8"));
    const projection = JSON.parse(readFileSync(join(owner, "🧬️schema/🕸️media-projection/🔣️.json"), "utf8")) as { $id: string };
    const validate: SchemaCheck = new Ajv({ strict: true, allErrors: true }).addSchema(projection).getSchema(`${projection.$id}#/$defs/WorkflowMediaContractProjectionV1`) as ValidateFunction;
    assert.equal(fixture.schema, "semio.workflow.media-contract-presentation/v1");
    assert.equal(fixture.cases.length, 4);
    const ids = new Set<string>();
    let denied = 0;
    for (const row of fixture.cases) {
      assert(!ids.has(row.id));
      ids.add(row.id);
      assert(validate(row.input), JSON.stringify(validate.errors));
      assert(validate(row.expected), JSON.stringify(validate.errors));
      const { kindId, mediaType, wire, conversion } = row.input;
      const result = { kindId, mediaType: { class: mediaType.class, form: mediaType.form }, wire: wire.kind === "document" ? { kind: "document", schema: wire.schema } : { kind: "binary", formatKind: wire.formatKind }, conversion: conversion === null ? null : [conversion[0], conversion[1]] };
      assert.deepEqual(result, row.expected);
      assert.equal(conversion !== null, row.isConversion);
      for (const hostile of [{ ...result, unknown: true }, { ...result, wire: { ...wire, unknown: true } }, { ...result, conversion: ["vector"] }, { ...result, mediaType: { class: "other", form: "vector" } }]) {
        assert(!validate(hostile));
        denied++;
      }
    }
    const source = readFileSync(join(this.root, "../../🔀️workflow/🦀️.rs"), "utf8");
    assert(!source.includes("edge.contract.to_value()"), "presentation must not invent a persistence codec");
    assert(source.includes('"contract": workflow_media_contract_payload(&edge.contract)'), "the live window payload uses the explicit projection");
    const projectionLaw = readFileSync(join(this.root, "../../🧪️tests/🔬️workflow-standalone/🦀️.rs"), "utf8");
    assert(projectionLaw.includes("fn workflow_media_contract_projection_matches_neutral_document_binary_and_conversion_cases"), "an exact native projection law is registered");
    console.log(`media contract presentation oracle: ${fixture.cases.length} document/binary/conversion vectors, ${denied} strict hostile denials; no native claim`);
    if (segments.includes("--oracle-only")) return;
    const receipts = await runRepositoryExactCargoLaws({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0), preparation: configuredRepositoryCargoPreparationPolicyV1(), invocation: this.invocation, policy: { ...configuredExactCargoLawPolicyV1() }, cwd: this.repoRoot, groups: [{ package: "semio-framework-os-host-workflow", target: { kind: "lib" }, laws: ["workflow_media_contract_projection_matches_neutral_document_binary_and_conversion_cases"] }, { package: "semio-framework-os-host-registry", target: { kind: "lib" }, laws: ["owned_artifact_kind_formats_survive_host_registry_projection"] }] });
    console.log(`media contract presentation native assertions=${receipts.reduce((total, receipt) => total + receipt.assertions, 0)}; executables=${receipts.map((receipt) => receipt.sha256).join(",")}`);
  }
}

export class PersistenceContractScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.some(segment => segment !== "--oracle-only")) throw new Error("persistence-contract-check accepts only --oracle-only");
    const base = join(this.root, "../../💾️persistence");
    const fixture = JSON.parse(readFileSync(join(base, "🧪️tests/🧫️fixtures/🔣️.json"), "utf8"));
    const schema = JSON.parse(readFileSync(join(base, "🧬️schema/🔣️.json"), "utf8"));
    const ajv = new Ajv({ strict: true, allErrors: true });
    ajv.addSchema(schema);
    const requestValid: SchemaCheck = ajv.getSchema(`${schema.$id}#/$defs/Request`) as ValidateFunction;
    const eventValid: SchemaCheck = ajv.getSchema(`${schema.$id}#/$defs/Event`) as ValidateFunction;
    assert.equal(fixture.schema, "semio.shell.persistence-neutral/v1");
    assert.equal(fixture.capacity, 64);
    assert.equal(fixture.requests.length, 4);
    const requests = new Map<string, any>();
    for (const row of fixture.requests) {
      assert(!requests.has(row.id));
      assert(requestValid(row.value), JSON.stringify(requestValid.errors));
      requests.set(row.id, row.value);
    }
    for (const row of fixture.requestNegatives) {
      const hostile = structuredClone(requests.get("create"));
      const keys = row.path.split(".");
      const last = keys.pop()!;
      let parent = hostile;
      for (const key of keys) parent = parent[key];
      parent[last] = row.value;
      assert(!requestValid(hostile), row.id);
    }
    const phases = ["queued", "selecting", "opening", "writing", "committing"];
    let accepted = 0;
    let rejected = 0;
    let published = 0;
    for (const trace of fixture.traces) {
      const request = requests.get(trace.request);
      assert(request, trace.id);
      let terminal = false;
      let phase = -1;
      let completed = 0;
      let total: number | undefined;
      let publications = 0;
      for (const step of trace.steps) {
        const event = step.value;
        const expected = request.correlation;
        const actual = event.correlation;
        let allow: boolean = Boolean(eventValid(event)) && !terminal && actual.requestId === expected.requestId && ["windowId", "spaceId", "sessionId", "generation"].every(key => actual.scope[key] === expected.scope[key]);
        if (allow && event.kind === "progress") {
          const next = phases.indexOf(event.phase);
          allow = next >= phase && event.completedBytes >= completed && event.completedBytes <= event.totalBytes && (total === undefined || total === event.totalBytes) && (next !== 4 || event.completedBytes === event.totalBytes);
          if (allow) { phase = next; completed = event.completedBytes; total = event.totalBytes; }
        } else if (allow) {
          const outcome = event.outcome;
          if (outcome.kind === "cancelled") allow = phase < 4;
          if (outcome.kind === "committed") allow = phase === 4 && total === outcome.byteLength && completed === total && outcome.contentDigest !== "0".repeat(64) && request.operation.kind !== "deleteEntry" && (request.operation.kind !== "bindSpace" || outcome.spaceId === request.correlation.scope.spaceId) && outcome.catalogGeneration === request.correlation.scope.generation + 1;
          if (outcome.kind === "deleted") allow = phase === 4 && request.operation.kind === "deleteEntry" && outcome.entryId === request.operation.entryId && outcome.catalogGeneration === request.correlation.scope.generation + 1;
          if (allow) { terminal = true; if (["committed", "deleted"].includes(outcome.kind)) publications++; }
        }
        assert.equal(allow, step.accept, `${trace.id}: ${JSON.stringify(event)}`);
        if (allow) accepted++; else rejected++;
      }
      assert.equal(publications, trace.published, trace.id);
      published += publications;
    }
    let eventDenials = 0;
    const completedEvent = fixture.traces[0].steps.at(-1).value;
    for (const extra of ["path", "token", "message"]) {
      assert(!eventValid({ ...completedEvent, outcome: { ...completedEvent.outcome, [extra]: "private" } }));
      eventDenials++;
    }
    for (const contentDigest of ["0".repeat(64), "A".repeat(64), "a".repeat(63)]) {
      assert(!eventValid({ ...completedEvent, outcome: { ...completedEvent.outcome, contentDigest } }));
      eventDenials++;
    }
    for (const key of ["requestId", "generation"]) {
      const hostile = structuredClone(completedEvent);
      (key === "requestId" ? hostile.correlation : hostile.correlation.scope)[key] = 9_007_199_254_740_992;
      assert(!eventValid(hostile));
      eventDenials++;
    }
    const source = readFileSync(join(base, "🧪️tests/🔬️unit/🦀️.rs"), "utf8");
    assert(source.includes("persistence_contract_matches_neutral_scope_progress_and_terminal_traces"));
    assert(!source.includes("resolve_kernel_future") && !source.includes("resolve_ready") && !source.includes("ReplayShellCommand"));
    console.log(`persistence contract oracle: requests=${requests.size}, hostileRequests=${fixture.requestNegatives.length}, hostileEvents=${eventDenials}, traces=${fixture.traces.length}, accepted=${accepted}, rejected=${rejected}, durablePublications=${published}; no IO/runtime activation claim`);
    if (segments.includes("--oracle-only")) return;
    const receipts = await runRepositoryExactCargoLaws({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0), preparation: configuredRepositoryCargoPreparationPolicyV1(), invocation: this.invocation, policy: { ...configuredExactCargoLawPolicyV1() }, cwd: this.repoRoot, groups: [{ package: "semio-framework-os", target: { kind: "lib" }, laws: ["persistence_contract_matches_neutral_scope_progress_and_terminal_traces", "persistence_contract_rejects_closed_fields_and_cross_scope_receipts"] }] });
    assert.equal(receipts[0]!.assertions, 2);
    console.log(`persistence contract native assertions=${receipts[0]!.assertions}; executable=${receipts[0]!.sha256}`);
  }
}

export class DocumentRetirementScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.some(segment => segment !== "--oracle-only")) throw new Error("document-retirement-check accepts only --oracle-only");
    const base = join(this.repoRoot, "🧰️framework/🔨️modules/🌱️value/♻️retirement");
    const fixture = JSON.parse(readFileSync(join(base, "🧪️tests/🧫️fixtures/🔣️.json"), "utf8"));
    
    let grants = 0;
    const measure = (value: unknown, text: (value: string) => number): number => value === null ? 0 : typeof value === "string" ? text(value) : typeof value === "number" ? 8 : typeof value === "boolean" ? 1 : Array.isArray(value) ? value.reduce((sum, item) => sum + measure(item, text), 0) : Object.entries(value as Record<string, unknown>).reduce((sum, [key, item]) => sum + text(key) + measure(item, text), 0);
    for (const row of fixture.cases) {
      const bytes = row.kind === "bytes" ? Buffer.from(row.value).byteLength : row.kind === "words" ? Buffer.alloc(row.value.length * Uint32Array.BYTES_PER_ELEMENT).byteLength : measure(row.value, value => Buffer.byteLength(value, "utf8"));
      assert.equal(bytes, row.bytes, row.id);
      assert.equal(row.kind === "bytes" ? Uint8Array.from(row.value).byteLength : row.kind === "words" ? Uint32Array.from(row.value).byteLength : measure(row.value, value => new TextEncoder().encode(value).byteLength), bytes, row.id);
      for (const budget of fixture.budgets) {
        let remaining = bytes;
        let released = 0;
        while (remaining > 0) {
          const turn = Math.min(remaining, budget.bytes);
          assert(turn <= budget.bytes);
          remaining -= turn;
          released += turn;
          grants++;
        }
        assert.equal(released, row.bytes, row.id);
      }
    }
    const source = readFileSync(join(base, "🧪️tests/🔬️unit/🦀️.rs"), "utf8");
    assert(source.includes("owned_retirement_matches_neutral_exact_byte_grants"));
    assert(source.includes("owned_retirement_rejects_false_terminal_and_preserves_shared_roots"));
    const stdio = readFileSync(join(this.repoRoot, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🦀️.rs"), "utf8");
    assert(!stdio.includes("trait RetireOwned:") && !stdio.includes("struct Bytes(Vec<u8>)"), "Stdio must consume the shared owner primitive");
    console.log(`owned retirement oracle: cases=${fixture.cases.length}, budgets=${fixture.budgets.length}, exactByteGrants=${grants}, declaredHostile=${fixture.hostile.length}; native pending`);
    if (segments.includes("--oracle-only")) return;
    const receipts = await runRepositoryExactCargoLaws({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0), preparation: configuredRepositoryCargoPreparationPolicyV1(), invocation: this.invocation, policy: { ...configuredExactCargoLawPolicyV1() }, cwd: this.repoRoot, groups: [{ package: "semio-framework-os-kernel", target: { kind: "lib" }, laws: ["owned_retirement_matches_neutral_exact_byte_grants", "owned_retirement_rejects_false_terminal_and_preserves_shared_roots"] }] });
    assert.equal(receipts[0]!.assertions, 2);
  }
}
