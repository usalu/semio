#!/usr/bin/env bun
/** 📜️ `@semio-tech/framework-schema` task router. */
import { resolve } from "node:path";
import { readFileSync } from "node:fs";
import { strict as assert } from "node:assert";
import Ajv from "ajv";
import { resolveTestLevel, runCargoTestBudgeted, runCmd, devToolingEnv, buildBudgetMs } from "../../../../🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
import { CheckScript, GenerateScript, PreviewGeneratedScript } from "../../🏷️entity-kinds/🏃️execution/🟦️.ts";

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runCargoTestBudgeted(["semio-framework-schema"], this.repoRoot, rest);
    runCmd(process.execPath, ["test", resolve(this.root, "../../🧪️tests/🩹️fragment-validation-oracle/🟦️.ts")], { cwd: this.repoRoot });
  }
}

/** 🧩️ Executes portable schema-subset laws through Bun, Node and AJV. */
class SubsetContractScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw Error("test-subset-contract accepts no arguments");
    const { proveJsonSchemaSubsetContractV1 } = await import("../../✅️validator/🧪️tests/🟦️.ts");
    console.log("schema-subset-contract: " + await proveJsonSchemaSubsetContractV1() + " vectors passed");
  }
}

/** 🌐️ Checks owner removal with neutral schema vectors and independent Ajv validation. */
class DocumentHttpCheckScript extends BundleScript {
  run(): void {
    const fixture = JSON.parse(readFileSync(resolve(this.root, "../../🧫️fixtures/🌐️document-http/🔣️.json"), "utf8"));
    const ajv = new Ajv({ strict: false });
    const declarationSchema = JSON.parse(readFileSync(resolve(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🧬️schema/🔣️.json"), "utf8"));
    const declared = ajv.compile(declarationSchema);
    assert(declared(fixture.neutral));
    assert(declared(fixture.secondary));
    const replyBounds=ajv.compile(fixture.replyNodeBounds.oracleSchema);
    for(const vector of fixture.replyNodeBounds.vectors) assert.equal(replyBounds(Array(vector.items).fill(null)),vector.valid);
    const validate = ajv.compile(JSON.parse(fixture.neutral.operations[0].inputSchema));
    for (const vector of fixture.vectors) assert.equal(validate(vector.value), vector.valid, vector.name);
    for (const law of ["document_http::tests::owner_removal_preserves_neutral_document_transport", "document_http::tests::schema_vectors_match_owned_validator", "document_http::tests::decoded_replies_obey_the_same_node_bounds_as_owner_inputs"]) {
      runCmd("cargo", ["test", "-p", "semio-framework-schema", "--lib", "--", law, "--exact", "--nocapture"], { cwd: this.repoRoot, env: devToolingEnv(), budgetMs: buildBudgetMs() });
    }
    runCmd("cargo", ["test", "-p", "semio-framework-os-kernel", "--lib", "--", "os_directory::client::tests::document_http_transport_preserves_scope_bounds_and_owner_decode", "--exact"], { cwd: this.repoRoot, env: devToolingEnv(), budgetMs: buildBudgetMs() });
    console.log("document-http: Ajv vectors, removal/reinstall, scope, bounds and owner decoding passed");
  }
}

const router = new ScriptRouter(import.meta.dir)
  .register("generate", GenerateScript)
  .register("preview-generated", PreviewGeneratedScript)
  .register("check", CheckScript)
  .register("test-subset-contract", SubsetContractScript)
  .register("document-http-check", DocumentHttpCheckScript)
  .register("test", TestScript);

if (import.meta.main) await runScriptMain(router, { defaultCommand: "generate" });
