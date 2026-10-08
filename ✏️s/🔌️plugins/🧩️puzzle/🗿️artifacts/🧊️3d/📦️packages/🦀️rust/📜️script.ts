#!/usr/bin/env bun
import { GenerateScript as GraphGenerateScript, PreviewGeneratedScript as GraphPreviewScript, OwnerGraphWireCheckScript } from "../../../../../../../🧰️framework/🔨️modules/🕸️graph/🛂️manifest/🏃️execution/🟦️.ts";
/** 📦️ puzzle-3d Rust artifact package router. */
import { runArtifactRustPackageMain } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/🟦️.ts";

import { runCmd } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { readdirSync } from "node:fs";
import { resolve } from "node:path";

/** 🛂️ Verify the exact canonical provider through every published owning Source consumer. */
class OwnedVerifyScript extends BundleScript {
  run(args: string[]): void {
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

/**
 * 🎛️ The whole `editor` tree is `#[cfg(feature = "component-app-assembly")]` (crate root `🦀️.rs:14-21`),
 * so a default-feature `cargo test` compiles 286 of this crate's 577 tests and runs not one of the
 * `editor::puzzle3d` app tests — which is how a harness that could not construct its own app survived
 * a whole wave (📓️2026-09-09-wave-X-test-suite.md §6). Declared here so `nx test`, `bun ./📜️script.ts
 * test` and the dashboard commands that call them all measure the same suite.
 */
await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-puzzle-3d", { testFeatures: ["component-app-assembly"],commands:{"graph-generate":GraphGenerateScript,"preview-generated":GraphPreviewScript,"graph-wire-check":OwnerGraphWireCheckScript,verify:OwnedVerifyScript},snapshotSqliteTestFeatures:["component-app-assembly"],snapshotSqliteTests:["../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts"] });
