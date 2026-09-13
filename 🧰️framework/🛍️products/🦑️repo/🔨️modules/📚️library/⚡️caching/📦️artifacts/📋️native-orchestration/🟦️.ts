import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { createRequire } from "node:module";
import { dirname, resolve } from "node:path";
import { BundleScript } from "../../../🏃️process/🧭️routing/🟦️.ts";
import { buildBudgetMs, runCmdStatus } from "../../../🏃️process/🟦️.ts";
import { runOwnedCommand } from "../../../🏃️process/🎛️owned-execution/🟦️.ts";
import { buildCargoArtifacts } from "../🏗️native-build/🟦️.ts";
import { validateNativeCargoArguments } from "../🎛️native-input/🟦️.ts";

/** 🦀️ Routes native Cargo and component operations to their owned behaviors. */
export class NativeScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    const [tool, operation] = args;
    const index = args.indexOf("--manifest");
    const manifest = index >= 0 ? args[index + 1] : undefined;
    if (tool === "cargo" && operation === "metadata") {
      if (index !== 2 || args.length !== 4 || !manifest) throw new Error("native cargo metadata --manifest <Cargo.toml>");
      const path = resolve(this.repoRoot, manifest);
      await runOwnedCommand("cargo", ["metadata", "--locked", "--format-version=1", "--manifest-path", path], dirname(path), "cargo:locked-metadata", buildBudgetMs(), { stdout: "ignore" });
      console.log("[cargo:locked-metadata] dependency lock validated");
      return;
    }
    if (tool === "component") {
      if ((operation !== "dev" && operation !== "release") || index !== 2 || args.length !== 4 || !manifest) throw new Error("native component dev|release --manifest <Cargo.toml>");
      const cargo = createRequire(import.meta.url)("@iarna/toml").parse(readFileSync(resolve(this.repoRoot, manifest), "utf8"));
      if (!cargo.package?.metadata?.component?.package || !["plugin", "extension"].includes(cargo.package?.metadata?.semio?.role)) throw new Error(`Not a plugin component manifest: ${manifest}`);
      await buildCargoArtifacts(
        manifest,
        [
          "-p",
          cargo.package.name,
          "--lib",
          "--crate-type",
          "cdylib",
          "--target",
          "wasm32-wasip2",
          "--profile",
          `wasm-${operation}`,
          "--",
          "-C",
          "link-arg=-zstack-size=8388608",
          ...(process.env.SEMIO_PLUGIN_SYMBOLS === "1" ? ["-C", "strip=none"] : []),
        ],
        this.repoRoot,
        {
          command: "rustc",
          output: `dist/component-${operation}`,
          validate: (files) => {
            assert.equal(files.size, 1, "Component output must contain only the linked WASM component");
            const artifact = [...files][0];
            assert.ok(artifact);
            const [name, path] = artifact;
            assert.equal(name, `${cargo.package.name.replaceAll("-", "_")}.wasm`);
            assert.deepEqual([...readFileSync(path).subarray(0, 8)], [0, 97, 115, 109, 13, 0, 1, 0], "Invalid WASI component header");
          },
        },
      );
      return;
    }
    if (tool !== "cargo" || (operation !== "build" && operation !== "check" && operation !== "test") || !manifest) throw new Error("native cargo build|check|test --manifest <Cargo.toml>");
    const extra = args.slice(index + 2);
    validateNativeCargoArguments(operation, extra);
    if (operation === "build") {
      await buildCargoArtifacts(manifest, extra, this.repoRoot);
      return;
    }
    const status = runCmdStatus("cargo", [operation, "--locked", "--manifest-path", resolve(this.repoRoot, manifest), ...extra], { cwd: this.repoRoot });
    if (status) throw new Error(`cargo ${operation} failed (${status})`);
  }
}
