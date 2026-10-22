#!/usr/bin/env repo
import { GenerateScript as GraphGenerateScript, PreviewGeneratedScript as GraphPreviewScript, OwnerGraphWireCheckScript } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🕸️graph/🛂️manifest/🏃️execution/🟦️.ts";
/** 📦️ puzzle-5d Rust artifact package router. */
import { runArtifactRustPackageMain, runArtifactRustTests } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
import {BundleScript, ScriptRouter} from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { runOwnedVerifyScript } from "../../🧪️tests/🔬️rust-package-commands/🟦️.ts";

/** 🛂️ Verify the exact canonical provider through every published owning Source consumer. */
class OwnedVerifyScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    await runOwnedVerifyScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, args);
  }
}
class HostAdmissionTest extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length) throw new Error("Expected test host-admission");
    await runArtifactRustTests("semio-s-artifact-puzzle-5d", this.repoRoot, ["--features", "component-app-assembly", "--lib", "puzzle5d_host_admission_", "--", "--nocapture"], this.invocation);
  }
}
const testRouter = new ScriptRouter(import.meta.dir).register("host-admission", HostAdmissionTest);
const router = new ScriptRouter(import.meta.dir).register("graph-generate", GraphGenerateScript).register("preview-generated", GraphPreviewScript).register("graph-wire-check", OwnerGraphWireCheckScript).register("verify", OwnedVerifyScript);
await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-puzzle-5d", { packages: ["semio-s-artifact-puzzle-5d", "semio-s-artifact-puzzle-5d-catalog", "semio-s-artifact-puzzle-5d-diff", "semio-s-artifact-puzzle-5d-diff-2", "semio-s-artifact-puzzle-5d-dispatch", "semio-s-artifact-puzzle-5d-dispatch-panels", "semio-s-artifact-puzzle-5d-document-records", "semio-s-artifact-puzzle-5d-host-admission-tests", "semio-s-artifact-puzzle-5d-interaction", "semio-s-artifact-puzzle-5d-kind-catalogs", "semio-s-artifact-puzzle-5d-mut-any", "semio-s-artifact-puzzle-5d-mut-any-part2d", "semio-s-artifact-puzzle-5d-mut-any-2", "semio-s-artifact-puzzle-5d-mut-any-3", "semio-s-artifact-puzzle-5d-mut-any-4", "semio-s-artifact-puzzle-5d-mut-any-5", "semio-s-artifact-puzzle-5d-mut-any-6", "semio-s-artifact-puzzle-5d-mut-any-7", "semio-s-artifact-puzzle-5d-mut-any-8", "semio-s-artifact-puzzle-5d-puzzle5d", "semio-s-artifact-puzzle-5d-retained", "semio-s-artifact-puzzle-5d-schema", "semio-s-artifact-puzzle-5d-snapshot", "semio-s-artifact-puzzle-5d-store-preparation", "semio-s-artifact-puzzle-5d-store-preparation-commands", "semio-s-artifact-puzzle-5d-testkit", "semio-s-artifact-puzzle-5d-tests", "semio-s-artifact-puzzle-5d-v1"],testCommands: testRouter,commands: router,snapshotSqliteTests:["../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts"]});
