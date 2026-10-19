#!/usr/bin/env bun
/** 🏃️ Workflow workflow artifact package router. */
import { runArtifactRustPackageMain } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts";
import {BundleScript, ScriptRouter} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runWorkflowSqliteTestScript, runWorkflowSqliteVerifyScript } from "../../🧪️tests/🔬️rust-package-commands/🟦️.ts";
/** 🪶️ Runs the authored Workflow ownership laws through the existing task budgets. */
class WorkflowSqliteTestScript extends BundleScript {
  async run(segments:string[]): Promise<void> {
    await runWorkflowSqliteTestScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, segments);
  }
}
/** 🔎️ Verifies the Workflow source facade with strict public types. */
class WorkflowSqliteVerifyScript extends BundleScript {
  async run(segments:string[]): Promise<void> {
    await runWorkflowSqliteVerifyScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, segments);
  }
}
const router = new ScriptRouter(import.meta.dir).register("test-snapshot-sqlite", WorkflowSqliteTestScript).register("verify", WorkflowSqliteVerifyScript);
await runArtifactRustPackageMain(import.meta.dir,"semio-framework-artifact-workflow-workflow",{packages:["semio-framework-artifact-workflow-workflow","semio-framework-artifact-workflow-workflow-graph","semio-framework-artifact-workflow-workflow-snapshot","semio-framework-artifact-workflow-workflow-schema","semio-framework-artifact-workflow-workflow-schema-laws"],commands: router});
