#!/usr/bin/env bun
import { GenerateScript as GraphGenerateScript, PreviewGeneratedScript as GraphPreviewScript, OwnerGraphWireCheckScript } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🕸️graph/🛂️manifest/🏃️execution/🟦️.ts";
/** 📦️ puzzle-3d Rust artifact package router. */
import { runArtifactRustPackageMain, runArtifactRustPackagesTests } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts";

import { runCmd } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts";
import { BundleScript } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { readdirSync } from "node:fs";
import { resolve } from "node:path";

/** 🛂️ Verify the exact canonical provider through every published owning Source consumer. */
class OwnedVerifyScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if(args[0]==="original-fill-fixture"&&(args.length===1||args.length===2&&args[1]==="native")){runCmd("bun",["test",resolve(this.repoRoot,"✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🪣️fill/🎟️caller/🧪️tests/🟦️.ts")],{cwd:this.repoRoot});if(args[1]==="native")await runArtifactRustPackagesTests(["semio-s-artifact-puzzle-3d-editor-session"],this.repoRoot,["--offline","--test","precompute-fill","--","--nocapture"],this.invocation);return;}
    if (args.length === 1 && args[0] === "original-work-construction") {
      runCmd("bun", ["test", resolve(this.repoRoot, "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️retained/🎬️action/🧪️tests/🟦️.ts"), resolve(this.repoRoot, "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🧪️tests/🟦️.ts")], { cwd: this.repoRoot });
      return;
    }
    if (args.length !== 1 || args[0] !== "snapshot-sqlite-source") throw new Error("verify snapshot-sqlite-source");
    const roots: string[] = [];
    const visit = (dir: string): void => {
      for (const item of readdirSync(dir, { withFileTypes: true })) {
        const path = resolve(dir, item.name);
        if (item.isDirectory() && path !== resolve(this.root,"../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests")) visit(path);
        else if (item.name.endsWith(".ts")) roots.push(path);
      }
    };
    visit(resolve(this.root, "../../🏅️standards/🔖️1/🪆️subsets/✳️any"));
    runCmd("bun", [resolve(this.repoRoot, "node_modules/typescript/bin/tsc"), "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--allowImportingTsExtensions", "--resolveJsonModule", "--esModuleInterop", "--skipLibCheck", ...roots], { cwd: this.repoRoot });
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

await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-puzzle-3d", {commands:{"graph-generate":GraphGenerateScript,"preview-generated":GraphPreviewScript,"graph-wire-check":OwnerGraphWireCheckScript,verify:OwnedVerifyScript},snapshotSqliteTests:["../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts"] });
