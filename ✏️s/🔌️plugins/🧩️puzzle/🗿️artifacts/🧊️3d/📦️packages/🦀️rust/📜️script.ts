#!/usr/bin/env bun
import { GenerateScript as GraphGenerateScript, PreviewGeneratedScript as GraphPreviewScript, OwnerGraphWireCheckScript } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🕸️graph/🛂️manifest/🏃️execution/🟦️.ts";
/** 📦️ puzzle-3d Rust artifact package router. */
import { runArtifactRustPackageMain } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
import {BundleScript, ScriptRouter} from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runOwnedVerifyScript } from "../../🧪️tests/🔬️rust-package-commands/🟦️.ts";

/** 🛂️ Verify the exact canonical provider through every published owning Source consumer. */
class OwnedVerifyScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    await runOwnedVerifyScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, args);
  }
}

/**
 * 🧵️ Every app-driving test in this crate polls one `VcsArtifactApp` dispatch/render future, whose
 * unoptimized state machine plus the retained tool-job poll chain under it outgrows libtest's default
 * 2 MiB per-test thread stack and aborts the binary with `fatal runtime error: stack overflow` before
 * any suite summary is reachable. Raised here — the one permanent script this package owns — so `nx
 * test`, `bun ./📜️script.ts test` and the dashboard commands that call them all inherit it on every
 * platform without a per-developer environment step. A value already present in the environment wins.
 */
process.env.RUST_MIN_STACK ??= "134217728";

const router = new ScriptRouter(import.meta.dir).register("graph-generate", GraphGenerateScript).register("preview-generated", GraphPreviewScript).register("graph-wire-check", OwnerGraphWireCheckScript).register("verify", OwnedVerifyScript);
await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-puzzle-3d", {commands: router,snapshotSqliteTests:["../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts"] });
