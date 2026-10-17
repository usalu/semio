/** 🚪️ The member open protocol check and the public member open handoff source and check of the OS host router.
 *
 * @see ../../📜️script.ts — the package script that registers the commands of this module
 */
import { scriptCompilerAdmissionControlV1 as exactCargoCallerControlV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { configuredExactCargoLawPolicyV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️laws/🟦️.ts";
import { join } from "node:path";
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import Ajv from "ajv";
import { runRepositoryExactCargoLaws, configuredRepositoryCargoPreparationPolicyV1 } from "../../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import type { SchemaCheck } from "../🎞️media-persistence-and-retirement/🟦️.ts";

export class MemberOpenProtocolScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.some(segment => segment !== "--oracle-only")) throw new Error("member-open-protocol-check accepts only --oracle-only");
    const base = join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open");
    const fixture = JSON.parse(readFileSync(join(base, "🧪️tests/🧫️fixtures/🔣️.json"), "utf8"));
    const ajv = new Ajv({ strict: true, allErrors: true });
    
    let rejectedBytes = 0;
    for (const row of fixture.cases) {
      const text = [row.artifactId, row.openedActor].every(value => value.length > 0 && Buffer.byteLength(value) <= fixture.limits.identityBytes && !/\p{Cc}/u.test(value));
      const reason = !row.sealed ? "unsealed" : row.bytes === 0 ? "empty" : row.nowUs >= row.expiresAtUs ? "expired" : !text ? "identity" : row.ownerChildId !== null && (row.ownerChildId.length === 0 || Buffer.byteLength(row.ownerChildId) > fixture.limits.identityBytes || /\p{Cc}/u.test(row.ownerChildId)) ? "owner" : null;
      const structural: SchemaCheck = ajv.compile({ type: "object", required: ["sealed", "bytes", "expiresAtUs", "artifactId", "openedActor", "ownerChildId"], properties: {
        sealed: { const: true }, bytes: { type: "integer", minimum: 1 }, expiresAtUs: { type: "integer", exclusiveMinimum: row.nowUs },
        artifactId: { type: "string", minLength: 1, pattern: "^[^\\p{Cc}]+$" }, openedActor: { type: "string", minLength: 1, pattern: "^[^\\p{Cc}]+$" }, ownerChildId: { anyOf: [{ type: "null" }, { type: "string", minLength: 1, pattern: "^[^\\p{Cc}]+$" }] },
      } });
      assert.equal(structural(row) && new TextEncoder().encode(row.artifactId).length <= fixture.limits.identityBytes && new TextEncoder().encode(row.openedActor).length <= fixture.limits.identityBytes && (row.ownerChildId === null || new TextEncoder().encode(row.ownerChildId).length <= fixture.limits.identityBytes), reason === null, row.id);
      assert.equal(reason, row.reason, row.id);
      assert.equal(reason === null, row.admitted, row.id);
      if (reason !== null) rejectedBytes += row.bytes;
    }
    for (const row of fixture.framing) {
      let offset = 0;
      let length = 0n;
      let reason: string | null = null;
      for (;;) {
        const byte = row.bytes[offset++];
        if (byte === undefined || offset > 10 || (offset === 10 && byte > 1)) { reason = "malformed"; break; }
        length |= BigInt(byte & 127) << BigInt((offset - 1) * 7);
        if (byte < 128) {
          if ((offset > 1 && byte === 0) || length === 0n || BigInt(offset) + length >= BigInt(row.bytes.length)) reason = "malformed";
          break;
        }
      }
      assert.equal(reason, row.reason, row.id);
      if (reason === null) {
        assert.deepEqual([offset, Number(length)], row.snapshot, row.id);
        assert.deepEqual([offset + Number(length), row.bytes.length - offset - Number(length)], row.history, row.id);
        const encoded: number[] = [];
        for (let value = Number(length);;) { const next = value % 128; value = Math.floor(value / 128); encoded.push(next + (value ? 128 : 0)); if (!value) break; }
        assert.deepEqual(Buffer.from(encoded), Buffer.from(row.bytes.slice(0, offset)), row.id);
      }
    }
    const stages = ["input", "snapshot", "forward", "inverse", "envelope", "initialization"];
    for (const row of fixture.retention) {
      const index = stages.indexOf(row.stage);
      assert.deepEqual([row.snapshots, row.mutations], [index === 0 ? 0 : index === 5 ? 2 : 1, index === 2 ? 1 : index === 3 ? 2 : 0], row.id);
      let retained = row.snapshots + row.mutations;
      let retired = 0;
      while (retained !== 0) { retained--; retired++; }
      assert.equal(retired, row.snapshots + row.mutations);
    }
    const source = readFileSync(join(base, "🦀️.rs"), "utf8");
    assert(source.includes("pub trait MemberOpenOperation") && source.includes("StepContext<'_>"));
    assert(source.includes('#[path = "🧪️tests/🔬️unit/🦀️.rs"]') && source.includes("mod tests;"));
    const sourceTests = readFileSync(join(base, "🧪️tests/🔬️unit/🦀️.rs"), "utf8");
    assert(sourceTests.includes("member_open_request_rejection_retains_exact_pages_and_identity"));
    assert(sourceTests.includes("member_open_input_framing_is_canonical_scoped_and_budgeted"));
    const store = readFileSync(join(base, "../../🧪️tests/🌆️bin-02/🦀️.rs"), "utf8");
    assert(store.includes('#[path = "🔬️unit/🦀️.rs"]') && store.includes("mod store_unit_tests;"));
    const storeTests = readFileSync(join(base, "../../🧪️tests/🌆️bin-02/🔬️unit/🦀️.rs"), "utf8");
    assert(storeTests.includes("member_open_partial_parse_and_initialization_owners_retire_exactly"));
    console.log(`[TRACE] member open request oracle: ${fixture.cases.length} admission cases, ${fixture.framing.length} framing cases, ${fixture.retention.length} declared retained-stage cases; rejected input bytes retained=${rejectedBytes}; typed parser/factory activation not claimed`);
    if (segments.includes("--oracle-only")) return;
    const receipts = await runRepositoryExactCargoLaws({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0), preparation: configuredRepositoryCargoPreparationPolicyV1(), invocation: this.invocation, policy: { ...configuredExactCargoLawPolicyV1() }, cwd: this.repoRoot, groups: [{ package: "semio-framework-os-store-member-open-tests", target: { kind: "lib" }, laws: ["member_open_request_rejection_retains_exact_pages_and_identity", "member_open_input_framing_is_canonical_scoped_and_budgeted"] }, { package: "semio-framework-os-store-tests-02", target: { kind: "lib" }, laws: ["member_open_partial_parse_and_initialization_owners_retire_exactly"] }] });
    assert.equal(receipts[0]!.assertions, 3);
  }
}

export class PublicMemberOpenHandoffScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("public-member-open-handoff-source accepts no arguments");
    this.proveSource();
  }

  protected proveSource(): void {
    const owner = join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/🚪️member-open");
    const fixture = JSON.parse(readFileSync(join(owner, "🧪️tests/🧫️fixtures/🔣️.json"), "utf8"));
    const selectedFixture = JSON.parse(readFileSync(join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/📜️history/🏭️factory/🧪️tests/🧫️fixtures/🔣️.json"), "utf8"));
    assert.equal(selectedFixture.declarations.length, fixture.requiredFactoryOperations);
    assert.deepEqual(selectedFixture.declarations.find((row: string[]) => row.slice(0, 3).every((value, index) => value === fixture.input.dialect[index])), fixture.selected);



    const bytes = (values: string[]) => values.reduce((total, value) => total + Buffer.byteLength(value), 0);
    assert.equal(bytes([fixture.input.slot, fixture.input.childId, ...fixture.input.dialect]), fixture.requestIdentityBytes);
    assert.equal(bytes([fixture.input.childId, ...fixture.input.dialect, fixture.parent.id, ...fixture.parent.dialect, fixture.input.slot, fixture.input.childId]), fixture.derivedIdentityBytes);
    const declaredSubsets = selectedFixture.declarations.map((declaration: string[]) => declaration[2]);
    assert.deepEqual(fixture.operations.map((row: { subset: string }) => row.subset), declaredSubsets, "operation enum order is the declaration order");
    assert.equal(new Set(declaredSubsets).size, fixture.requiredFactoryOperations);
    for (const row of fixture.operations) {
      const flow = row.subset === "flow";
      assert.deepEqual(
        { support: flow ? "flow" : "unsupported", terminal: flow ? "ready" : "decode", handoffs: flow ? 1 : 0, retiredInputBytes: flow ? 0 : fixture.inputBytes, publications: 0 },
        { support: row.support, terminal: row.terminal, handoffs: row.handoffs, retiredInputBytes: row.retiredInputBytes, publications: row.publications },
        row.subset,
      );
    }
    assert.equal(fixture.operations.filter((row: { support: string }) => row.support === "flow").length, 1, "only the bounded Flow decoder is currently admitted");
    for (const row of fixture.operationLifecycle) {
      const success = row.event === "none" || row.event === "repeat";
      const terminal = row.event === "cancel" ? "cancelled" : row.event === "operation" || row.event === "generation" ? "stale" : row.event === "expired" ? "expired" : row.event === "close" ? "closed" : "ready";
      assert.deepEqual(
        { terminal, handoffs: success ? 1 : 0, retiredInputBytes: success ? 0 : fixture.inputBytes, publications: 0 },
        { terminal: row.terminal, handoffs: row.handoffs, retiredInputBytes: row.retiredInputBytes, publications: row.publications },
        row.id,
      );
    }
    const ids = new Set<string>();
    for (const row of fixture.cases) {
      assert(!ids.has(row.id)); ids.add(row.id);
      const request = structuredClone(fixture.input); const parent = structuredClone(fixture.parent);
      const capacity = { operations: true, abortRetirements: true, childMembers: true, rootRetirements: true };
      let cancelled = false; let factoryReject = false; let repeat = false; let expiresAt = fixture.authority.expiresAt;
      switch (row.mutation) {
        case "none": break; case "wrong-slot": request.slot = "foreign"; break; case "wrong-child": request.childId = "evil-member"; break;
        case "wrong-kind": request.dialect[0] = "s.other.semio"; break; case "wrong-standard": request.dialect[1] = "v2"; break; case "wrong-subset": request.dialect[2] = "tree"; break;
        case "parent-dialect": parent.dialect[0] = "s.other.flow"; break; case "projection": parent.projection.childId = "evil-member"; break;
        case "unsealed": request.sealed = false; break; case "empty": request.inputBytes = 0; break; case "expired": expiresAt = 1; break;
        case "operation-capacity": capacity.operations = false; break; case "abort-capacity": capacity.abortRetirements = false; break; case "member-capacity": capacity.childMembers = false; break; case "root-capacity": capacity.rootRetirements = false; break;
        case "cancel-before": cancelled = true; break; case "cancel-after": case "parent-generation": break; case "factory-reject": factoryReject = true; break; case "repeat-take": repeat = true; break;
        default: throw new Error(`unbound public handoff mutation ${row.mutation}`);
      }
      const exactProjection = request.slot === parent.projection.slot && request.childId === parent.projection.childId && request.dialect.every((value: string, index: number) => value === parent.projection.dialect[index]);
      let error: string | null = cancelled ? "cancelled" : !request.sealed ? "unsealed" : request.inputBytes === 0 ? "empty" : expiresAt <= fixture.authority.now ? "expired"
        : parent.dialect.join("/") !== fixture.parent.dialect.join("/") || !exactProjection ? "identity" : Object.values(capacity).some(available => !available) ? "capacity" : null;
      let retainedBy = "caller"; let identityBytes = fixture.requestIdentityBytes; let reservations = [0, 0, 0, 0]; let handoffs = 0; let retiredInputBytes = error === null ? 0 : request.inputBytes;
      if (error === null) {
        retainedBy = "app"; identityBytes = fixture.derivedIdentityBytes; reservations = [1, 1, 1, 1];
        if (row.mutation === "cancel-after") error = "cancelled";
        if (row.mutation === "parent-generation") error = "stale";
        if (factoryReject) error = "identity";
        if (error !== null) { reservations = [0, 0, 0, 0]; retiredInputBytes = request.inputBytes; }
        else { retainedBy = "factory"; handoffs = 1; if (repeat) assert.equal(handoffs, 1, "a repeated take cannot mint a second operation owner"); }
      }
      assert.deepEqual({ error, retainedBy, identityBytes, reservations, handoffs, publications: 0, creates: 0, retiredInputBytes },
        { error: row.error, retainedBy: row.retainedBy, identityBytes: row.identityBytes, reservations: row.reservations, handoffs: row.handoffs, publications: row.publications, creates: row.creates, retiredInputBytes: row.retiredInputBytes }, row.id);
    }
    assert.equal(ids.size, 20);
    console.log("public member-open handoff oracle:20 projection/input/capacity cases + exact18 operation arms +8 Flow lifecycle traces; one bounded Flow decoder,17 explicit retained denials, schema+owner derived, publication0");
    const store = readFileSync(join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/👥️space-member/📥️lifted/🏪️store/🦀️.rs"), "utf8");
    const factory = store.slice(store.indexOf("pub trait MemberFactory"), store.indexOf("pub enum NoMembers"));
    assert(/type Open\s*:\s*MemberOpenOperation<Member\s*=\s*Self>/.test(factory) && factory.includes("fn begin_open(request: MemberOpenRequest)"),
      "public handoff requires the actual closed MemberFactory operation; borrowed M::open is not retained authority");
  }
}

export class PublicMemberOpenHandoffCheckScript extends PublicMemberOpenHandoffScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("public-member-open-handoff-check accepts no arguments");
    this.proveSource();
    const laws = ["semio_member_factory_request_owned_open_admits_only_retained_flow"];
    const receipts = await runRepositoryExactCargoLaws({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0), preparation: configuredRepositoryCargoPreparationPolicyV1(), invocation: this.invocation, policy: { ...configuredExactCargoLawPolicyV1() }, cwd: this.repoRoot, groups: [{ package: "semio-s-plugin-stdio", target: { kind: "test", name: "flow_retained_decode" }, laws }] });
    assert.equal(receipts.length, 1);
    assert.equal(receipts[0]!.assertions, 1);
  }
}
