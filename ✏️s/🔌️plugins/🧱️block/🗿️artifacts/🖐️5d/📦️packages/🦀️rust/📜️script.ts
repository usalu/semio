#!/usr/bin/env bun
/** 📦️ Block5d owning package, canonical Source consumers and snapshot proof. */
import {runArtifactRustPackageMain} from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/🟦️.ts";
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

await runArtifactRustPackageMain(import.meta.dir,"semio-s-artifact-block-5d",{commands:{verify:OwnedVerifyScript},snapshotSqliteTests:["../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🟦️.ts"]});
