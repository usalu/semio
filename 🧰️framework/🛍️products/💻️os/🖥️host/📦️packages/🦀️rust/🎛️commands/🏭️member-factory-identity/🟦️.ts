/** 🏭️ The member factory identity check of the OS host router.
 *
 * @see ../../📜️script.ts — the package script that registers the commands of this module
 */
import { scriptCompilerAdmissionControlV1 as exactCargoCallerControlV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { unsignedLebOracleV1 } from "../../../../../../../🔨️modules/🎒️pack/📦️packages/🟦️typescript/📐️format/🧪️testing/🔮️oracles/🔢️leb/🟦️.ts";
import { configuredExactCargoLawPolicyV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️laws/🟦️.ts";
import { join } from "node:path";
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { runAdmittedExactCargoLawsV1 } from "@semio-tech/process/cargo-test";
import { BundleScript } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { readModuleTreeSource } from "@semio-tech/framework-compiler-ts/syntax/module-tree";

export class MemberFactoryIdentityScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.some(segment => segment !== "--oracle-only")) throw new Error("member-factory-identity-check accepts only --oracle-only");
    const owner = join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/📜️history/🏭️factory");
    const fixture = JSON.parse(readFileSync(join(owner, "🧪️tests/🧫️fixtures/🔣️.json"), "utf8"));
    
    const semioSource = readFileSync(join(this.repoRoot, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🦀️.rs"), "utf8");
    const semioTable = semioSource.slice(semioSource.indexOf("pub enum SemioMembers, SemioMembersOpen {"), semioSource.indexOf("/// 🏭️ Mints a typed Semio child"));
    const declarations = [...semioTable.matchAll(/\w+\("([^"]+)", "([^"]+)", "([^"]+)", "([^"]+)"\) =>/g)].map(match => match.slice(1));
    assert.equal(declarations.length, 18); assert.deepEqual(declarations, fixture.declarations, "neutral rows must be exactly the owning closed factory declaration, never parallel authority");
    const inputFixture = JSON.parse(readFileSync(join(owner, "../🧪️tests/🧫️fixtures/🔣️.json"), "utf8"));
    assert.equal(Buffer.from(inputFixture.historyHex, "hex").length + 2, fixture.inputBytes);
    const validText = (value: string) => value.length > 0 && Buffer.byteLength(value) <= fixture.limits.fieldBytes && !/\p{Cc}/u.test(value);
    const validKind = /^s\.[a-z0-9]+(?:-[a-z0-9]+)*\.[a-z0-9]+(?:-[a-z0-9]+)*$/;
    const tableFor = (mutation: string): string[][] => {
      const table = structuredClone(declarations);
      if (mutation === "reverse") table.reverse();
      if (mutation === "missing") table.splice(6, 1);
      if (mutation === "empty") table.length = 0;
      if (mutation === "capacity") while (table.length < 65) table.push([...table[0]!]);
      if (mutation === "duplicate") table.push([...table[6]!]);
      if (mutation === "duplicate-foreign") table.push([...table[0]!.slice(0, 3), "different.schema"]);
      if (mutation === "late-kind") table[17]![0] = "";
      if (mutation === "late-schema") table[17]![3] = "";
      if (mutation === "late-control") table[17]![3] = "stdio.\u0085semio";
      if (mutation === "late-length") table[17]![3] = "x".repeat(257);
      if (mutation === "late-noncanonical") table[17]![0] = "stdio.semio";
      if (mutation === "late-standard") table[17]![1] = "";
      if (mutation === "late-subset") table[17]![2] = "";
      if (mutation === "late-standard-length") table[17]![1] = "x".repeat(257);
      if (mutation === "late-subset-control") table[17]![2] = "vi\u0085deo";
      return table;
    };
    const model = (table: string[][], dialect: string[]) => {
      const request = [...fixture.requestIdentity]; request.splice(1, 3, ...dialect);
      const state = { candidate: null as string[] | null, ready: false, error: null as string | null, holder: "operation", handoffs: 0, units: 0 };
      function* scan() {
        if (!table.length) { state.error = "identity"; return; }
        if (table.length > fixture.limits.declarations) { state.error = "capacity"; return; }
        for (let index = 0; index < table.length; index++) {
          const row = table[index]!;
          if (!row.every(validText) || !validKind.test(row[0]!)) { state.error = "identity"; return; }
          yield "validated-row";
          for (let earlier = 0; earlier < index; earlier++) {
            if (row.slice(0, 3).every((value, field) => value === table[earlier]![field])) { state.error = "identity"; return; }
            yield "unique-pair";
          }
          if (row.slice(0, 3).every((value, field) => value === request[field + 1])) { state.candidate = row; yield "selected-unpublished"; }
        }
        if (!state.candidate) { state.error = "identity"; return; }
        state.ready = true; yield "ready";
      }
      const iterator = scan();
      const next = () => { if (state.error || state.holder !== "operation") return { done: true, value: undefined }; state.units++; return iterator.next(); };
      const take = () => {
        if (state.holder !== "operation") return "stale"; if (state.error) return state.error; if (!state.ready) return "pending";
        state.holder = "witness"; state.handoffs++; return null;
      };
      const close = (grant: number) => {
        state.holder = "retiring"; let remaining = fixture.inputBytes + request.reduce((sum, value) => sum + Buffer.byteLength(value), 0); let retired = 0;
        while (remaining) { const bytes = Math.min(grant, remaining); remaining -= bytes; retired += bytes; assert(bytes <= grant); assert.equal(take(), "stale"); }
        state.candidate = null; state.holder = "terminal"; return retired;
      };
      return { state, next, take, close };
    };
    for (const declaration of declarations) {
      const run = model(declarations, declaration.slice(0, 3)); while (!run.next().done) {};
      assert.equal(run.take(), null); assert.deepEqual(run.state.candidate, declaration); assert.equal(run.take(), "stale"); run.close(1);
    }
    assert.deepEqual(Object.keys(fixture.caseRetirement).sort(), fixture.cases.map((row: { id: string }) => row.id).sort());
    for (const row of fixture.cases) for (const grant of fixture.grants) {
      const run = model(tableFor(row.mutation), row.dialect); let done = false;
      while (!done) for (let fuel = grant; fuel && !done; fuel--) done = Boolean(run.next().done);
      assert.equal(run.state.error, row.error, row.id);
      assert.deepEqual(run.state.error ? null : run.state.candidate, row.selected, row.id);
      if (row.error === null) { assert.equal(run.take(), null); assert.equal(run.take(), "stale"); } else assert.equal(run.take(), row.error);
      assert.equal(run.close(grant), fixture.caseRetirement[row.id], row.id);
    }
    for (const row of fixture.lifecycle) for (const grant of fixture.grants) {
      const run = model(declarations, fixture.requestIdentity.slice(1, 4));
      if (row.at !== "begin") { let found = false; while (!found) { const step = run.next(); assert(!step.done); found = step.value === (row.at === "witness" ? "ready" : row.at); } }
      if (row.at === "selected-unpublished") assert.equal(run.take(), "pending");
      if (row.at === "witness") assert.equal(run.take(), null);
      run.state.error = row.event === "cancel" ? "cancelled" : ["generation", "operation"].includes(row.event) ? "stale" : ["expired", "clock-absent"].includes(row.event) ? "expired" : null;
      if (row.event === "none") assert.equal(run.take(), null);
      const before = run.state.units; assert(run.next().done); assert.equal(run.state.units, before);
      assert.equal(run.state.error, row.error); assert.equal(run.state.handoffs, row.handoffs); assert.equal(run.close(grant), row.retiredBytes);
    }
    const leb = unsignedLebOracleV1();
    const size = (value: number) => leb.encodeUnsigned32(value).length;
    const request = fixture.requestIdentity;
    const selected = declarations.find(row => row.slice(0, 3).every((value, index) => value === request[index + 1]));
    if(!selected)throw Error("Selected declaration fixture missing");
    const [kind,standard,subset,schema]=selected;
    if(kind===undefined||standard===undefined||subset===undefined||schema===undefined)throw Error("Complete selected declaration fixture required");
    const expected = { document: request[0], schema, parent: `${request[4]}!${request[5]}@${request[6]}/${request[7]}`, slot: request[8], child: request[9], kind,standard,subset };
    const seenSemantic = new Set<string>();
    for (const row of fixture.semantic) {
      assert(!seenSemantic.has(row.field)); seenSemantic.add(row.field);
      const persisted: Record<string, string> = { ...expected }; if (row.field !== "none") persisted[row.field] = row.value;
      const dictionary: string[] = [];
      const id = (text: string) => { let index = dictionary.indexOf(text); if (index < 0) { index = dictionary.length; dictionary.push(text); } return 1 + size(index); };
      const deltaBytes = (base: number) => 1 + size(base) + size(dictionary.length - base) + dictionary.slice(base).reduce((bytes, entry) => bytes + size(Buffer.byteLength(entry)) + Buffer.byteLength(entry), 0);
      const documentBytes = 1 + id(persisted.document!) + id(persisted.schema!); const firstDelta = deltaBytes(0); const base = dictionary.length;
      const compositionBytes = 2 + ["parent", "slot", "child", "kind", "standard", "subset"].reduce((bytes, field) => bytes + id(persisted[field]!), 0);
      const secondDelta = deltaBytes(base); const frameBytes = (payload: number) => size(payload + 2) + 2 + payload + 8;
      const inputBytes = 2 + 32 + [firstDelta, documentBytes, secondDelta, compositionBytes, 64].reduce((bytes, payload) => bytes + frameBytes(payload), 0);
      const error = Object.entries(expected).some(([key, value]) => value !== persisted[key]) ? "identity" : null;
      assert.equal(error, row.error, row.id); assert.equal(Number(error === null), row.handoffs, row.id);
      assert.equal(inputBytes, row.inputBytes, row.id); assert.equal(inputBytes + request.reduce((bytes: number, field: string) => bytes + Buffer.byteLength(field), 0) + 1024, row.retiredBytes, row.id);
    }
    assert.equal(seenSemantic.size, 9);
    for (const row of fixture.semanticLifecycle) for (const grant of fixture.grants) {
      const state = { holder: row.at === "ready" ? "dictionary" : "selected", input: fixture.semantic[0].inputBytes, index: row.at === "ready" ? 1024 : 0, error: null as string | null, handoffs: 0 };
      state.error = row.event === "capacity" ? "capacity" : row.event === "cancel" ? "cancelled" : "stale";
      const take = () => state.error === null ? ++state.handoffs : 0;
      assert.equal(state.error, row.error); assert.equal(take(), 0); assert.equal(take(), 0); assert.equal(state.handoffs, 0);
      let remaining = state.input + state.index + request.reduce((bytes: number, field: string) => bytes + Buffer.byteLength(field), 0); let retired = 0;
      state.holder = "retiring"; while (remaining) { const bytes = Math.min(grant, remaining); remaining -= bytes; retired += bytes; } state.holder = "terminal";
      assert.equal(retired, row.retiredBytes, row.id);
    }
    console.log(`[TRACE] selected MemberFactory identity oracle: exact18 source-owned declarations,21 selection rows +7 selection/5 semantic lifecycle traces x3 grants,9 persisted schema/dialect/owner cases with literal input/close bytes; caller schema denied; no native/public opening claim`);
    const source = readFileSync(join(owner, "🦀️.rs"), "utf8");
    assert(source.includes("M::OPEN_DECLARATIONS") && source.includes("VerifiedMemberHistoryInput"));
    assert(source.includes("MemberHistoryDictionaryOwner::begin(input, self.declaration.schema"));
    const tests = readFileSync(join(owner, "🧪️tests/🏭️factory/🦀️.rs"), "utf8");
    const laws = ["member_factory_selection_uses_only_complete_closed_declarations", "member_factory_selection_retains_input_through_denial_and_handoff"];
    for (const law of laws) assert(tests.includes(`fn ${law}`));
    assert(tests.includes('fixture["semantic"]') && tests.includes('fixture["semanticLifecycle"]') && tests.includes("begin_dictionary"), "native selected schema law must traverse the actual dictionary owner and retain every denied input");
    const store = readModuleTreeSource(join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs"));
    const storeTests = readFileSync(join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/🧪️tests/🍰️companion/🔬️unit/🦀️.rs"), "utf8");
    const generatedSource = storeTests.slice(storeTests.indexOf("pub enum RetainedTestMembers, RetainedTestMembersOpen {"), storeTests.indexOf("fn member_publication_fixture()"));
    assert.deepEqual([...generatedSource.matchAll(/\w+\("([^"]+)", "([^"]+)", "([^"]+)", "([^"]+)"\) =>/g)].map(match => match.slice(1)), fixture.generatedDeclarations);
    assert(tests.includes("super::store_unit_tests::RetainedTestMembers") && tests.includes("Generated::OPEN_DECLARATIONS"), "native laws must exercise the actual space_members macro expansion, not just hand-declared fixture factories");
    assert(store.includes("const OPEN_DECLARATIONS:"), "selected identity requires the coordinated MemberFactory declaration API; staged source is not mounted authority");
    const macroFactory = store.slice(store.indexOf("impl $crate::os_store::MemberFactory for $enum_name"), store.indexOf("//#endregion SpaceMember"));
    for (const field of ["kind: $kind", "standard: $standard", "subset: $subset", "schema: $schema"]) assert(macroFactory.includes(field), `factory declaration must reuse its creation/open literal ${field}`);
    const noMembers = store.slice(store.indexOf("impl MemberFactory for NoMembers"), store.indexOf("/// 🧬️ Generates a per-plugin"));
    assert(/const OPEN_DECLARATIONS:\s*&'static\s*\[MemberOpenDeclaration\]\s*=\s*&\[\];/.test(noMembers), "NoMembers has exactly the empty declaration table");
    if (segments.includes("--oracle-only")) return;
    assert(readFileSync(join(owner, "../🦀️.rs"), "utf8").includes("mod factory;"), "selected factory native laws require the coordinated mount");
    const receipts = await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0), invocation: this.invocation, policy: { ...configuredExactCargoLawPolicyV1() }, cwd: this.repoRoot, groups: [{ package: "semio-framework-os-kernel", target: { kind: "lib" }, laws }] });
    assert.equal(receipts[0]!.assertions, 2);
  }
}
