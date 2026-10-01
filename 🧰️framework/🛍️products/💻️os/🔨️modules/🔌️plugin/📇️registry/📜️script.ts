#!/usr/bin/env bun
import { BundleScript, ScriptRouter, runBundleScriptMain, runVitest } from "../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

/** 📦️ Verifies owner-supplied deployment inventory without compiled component prerequisites. */
class DeploymentContractTestScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    await runVitest(this.root, ["./📦️deployment/🧪️tests/📇️inventory/🟦️.ts", ...args], "./🧪️tests/🎚️config/🟦️.ts");
  }
}

/** 🏷️Verifies owner-authored launch names without concrete owner prerequisites. */
class LaunchNameContractTestScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    await runVitest(this.root, ["./🚀️launch/🏷️name-prefix/🧪️tests/🟦️.ts", ...args], "./🧪️tests/🎚️config/🟦️.ts");
  }
}

/** 🗂️Verifies the owner-declared playground asset contract. */
class AssetContractTestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw Error("test-asset-contract has no arguments");
    const { proveTileProxyAssetCorpusV1, proveTileProxyTransportV1, proveTileAssetMetadataV1 } = await import("./🎮️playground/🗂️assets/🧪️tests/🟦️.ts");
    console.log(`playground-asset-contract: cases=${proveTileProxyAssetCorpusV1()} passed`);
    const artifactRoot = process.env.SEMIO_TEST_ARTIFACT_DIR;
    if (!artifactRoot) throw Error("test-asset-contract requires a ticket artifact directory");
    await proveTileAssetMetadataV1(artifactRoot);
    await proveTileProxyTransportV1(artifactRoot);
  }
}

/** ⭐️Verifies explicit default policy and removal of all concrete owners. */
class PlaygroundDefaultContractTestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw Error("test-playground-default-contract accepts no arguments");
    const { provePlaygroundDefaultContractV1 } = await import("./🎮️playground/⭐️default/🧪️tests/🟦️.ts");
    console.log(`playground-default-contract: ${await provePlaygroundDefaultContractV1()} vectors passed`);
  }
}

/** 🧾️Verifies distinct source, compiled and deployed component admission. */
class ComponentOwnerContractTestScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length) throw Error("test-component-owners has no arguments");
    const { spawnSync } = await import("node:child_process");
    const result = spawnSync(process.execPath, ["test", "./🔎️discovery/🧪️tests/🟦️.ts"], { cwd: this.root, stdio: "inherit", env: process.env });
    if (result.error) throw result.error;
    if (result.status !== 0) throw Error("Component owner contract failed");
  }
}

const router = new ScriptRouter(import.meta.dir)
  .registerLazy("generate", async () => (await import("./📽️projection/🟦️.ts")).GenerateScript)
  .registerLazy("session", async () => (await import("./🎮️playground/🧭️session/🟦️.ts")).SessionScript)
  .registerLazy("preview-generated", async () => (await import("./📽️projection/🟦️.ts")).PreviewGeneratedScript)
  .registerLazy("check-generated", async () => (await import("./📽️projection/🟦️.ts")).CheckGeneratedScript)
  .registerLazy("rust-taxonomy-mounts-check", async () => (await import("./🗿️taxonomy-validation/🟦️.ts")).RustTaxonomyMountsCheckScript)
  .registerLazy("plugin-root-ownership-check", async () => (await import("./🗿️taxonomy-validation/🟦️.ts")).PluginRootOwnershipCheckScript)
  .registerLazy("native-catalog-selection-check", async () => (await import("./✅️catalog-verification/🟦️.ts")).NativeCatalogSelectionCheckScript)
  .registerLazy("catalog-complete", async () => (await import("./✅️catalog-verification/🟦️.ts")).CatalogCompleteScript)
  .registerLazy("check", async () => (await import("./📽️projection/🟦️.ts")).CheckScript)
  .registerLazy("rebuild-all", async () => (await import("./🔁️rebuild/🟦️.ts")).RebuildAllScript)
  .registerLazy("guest-framework-check", async () => (await import("./🔁️rebuild/🟦️.ts")).GuestFrameworkCheckScript)
  .registerLazy("verify-staged", async () => (await import("./🔁️rebuild/🟦️.ts")).VerifyStagedScript)
  .registerLazy("test", async () => (await import("./✅️catalog-verification/🟦️.ts")).RegistryTestScript)
  .registerLazy("test-catalog-contract", async () => (await import("./✅️catalog-verification/🟦️.ts")).CatalogContractTestScript)
  .register("test-asset-contract", AssetContractTestScript)
  .register("test-deployment-contract", DeploymentContractTestScript)
  .register("test-component-owners", ComponentOwnerContractTestScript)
  .register("test-launch-name-contract", LaunchNameContractTestScript)
  .register("test-playground-default-contract", PlaygroundDefaultContractTestScript)
  .registerLazy("new", async () => (await import("./🌳️surface-scaffold/🟦️.ts")).NewScript)
  .registerLazy("surface-schema", async () => (await import("./🧬️surface-schema/🟦️.ts")).SurfaceSchemaScript);
if (import.meta.main) await runBundleScriptMain(router, import.meta.url, { defaultCommand: "generate" });
