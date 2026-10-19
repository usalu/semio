#!/usr/bin/env bun
import { receiveScriptProcessInvocation } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { resolveTestLevel } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import { readFileSync } from "node:fs";
import { runCargo } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { runAdmittedCargoPackageTestsV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts";
import { mutationSchemaDocumentIndexProjection } from "../../🗂️schema-index/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { GenerateScript, PreviewGeneratedScript, CheckGeneratedScript, GenerateSchemaIndexScript, PreviewSchemaIndexScript, ColdReferenceTestScript, SCHEMA_INDEX_PATH } from "../../🧪️tests/🔬️rust-package-commands/🟦️.ts";


class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runAdmittedCargoPackageTestsV1(["semio-framework-os-kernel-dsl-derive"], this.repoRoot, this.invocation, rest);
  }
}

class ExportTestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runCargo(["test", "--manifest-path", "Cargo.toml", "--lib", "facade_exports_match_registered_macros", ...segments], this.root);
  }
}

class ExportSourceTestScript extends BundleScript {
  async run(): Promise<void> { await import("../../🧪️tests/📤️macro-exports/🟦️.ts"); }
}

class SourceAuthorityTestScript extends BundleScript {
  async run(): Promise<void> { await import("../../🧪️tests/🛂️mutation-source-authority/🟦️.ts"); }
}

/** ✅️ Fails when the committed schema document index is not the index of the current schemas. */
class CheckSchemaIndexScript extends BundleScript {
  async run(): Promise<void> {
    if (readFileSync(SCHEMA_INDEX_PATH, "utf8") !== mutationSchemaDocumentIndexProjection(this.repoRoot)) throw new Error("schema document index is stale; run bun repo run @semio-tech/dsl-derive-rs:generate-schema-index");
    console.log("schema-document-index: fresh");
  }
}

class SchemaIndexSourceTestScript extends BundleScript {
  async run(): Promise<void> { await import("../../🧪️tests/🗂️schema-document-index/🟦️.ts"); }
}


const router = new ScriptRouter(import.meta.dir).register("generate", GenerateScript).register("preview-generated", PreviewGeneratedScript).register("check-generated", CheckGeneratedScript).register("generate-schema-index", GenerateSchemaIndexScript).register("schema-index", PreviewSchemaIndexScript).register("check-schema-index", CheckSchemaIndexScript).register("test-schema-index-source", SchemaIndexSourceTestScript).register("test", TestScript).register("test-exports", ExportTestScript).register("test-exports-source", ExportSourceTestScript).register("test-source-authority-source", SourceAuthorityTestScript).register("test-cold-reference",ColdReferenceTestScript);
if (import.meta.main) await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, defaultCommand: "test" }));