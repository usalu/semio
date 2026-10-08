#!/usr/bin/env bun
import {completeCargoPreparationObservationV1} from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🛠️preparation/🧾️custody/🟦️.ts";
/** 📦️ remodel remodeling Rust artifact package router. */
import { runArtifactRustPackageMain } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/🟦️.ts";
import { runCmd, runCargo } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { join, resolve } from "node:path";

import { prepareCargoCapabilityLinksV1 } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🧩️capabilities/🟦️.ts";
import {runOwnedCommand} from "./../../../../../../../🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts";
/** 🔌️ Prepares source-owned optional Cargo capabilities. */
class CompositionScript extends BundleScript {
  run(args: string[]): void {
    if (args.length !== 1 || args[0] !== "prepare") throw new Error("composition prepare");
    console.log(`remodeling capabilities: targets=${prepareCargoCapabilityLinksV1(this.repoRoot, resolve(this.root, "../.."), "📦️packages/🦀️rust/Cargo.toml", "🧩️composition/🔗️capabilities/🔣️.json")}`);
    completeCargoPreparationObservationV1();
  }
}

/** 🧪️ Executes the contracts owned by this component. */
class OwnedVerifyScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length === 1 && segments[0] === "physical-codecs") {
      await runOwnedCommand(process.execPath, ["test", resolve(this.root, "./../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/📸️snapshot/🔣️json/🧪️tests/🔣️transport/🟦️.ts")], this.repoRoot, "owned-physical-codecs", 120_000);
      return;
    }

    if (segments.length === 1 && segments[0] === "snapshot-sqlite-source") {
      const schema=resolve(this.root,"../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema");
      runCmd("bun",[resolve(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--skipLibCheck",resolve(schema,"🟦️.ts"),resolve(schema,"📸️snapshot/🟦️.ts"),resolve(schema,"🧬️mutations/🟦️.ts"),resolve(schema,"🔺️diff/🟦️.ts"),resolve(schema,"../🚪️io/🪶️sqlite/📸️snapshot/🟦️.ts"),resolve(schema,"../🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts"),resolve(schema,"../🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🧬️schema/🟦️.ts"),resolve(schema,"../🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🟦️.ts")],{cwd:this.repoRoot});
      return;
    }
    if (segments.length === 1 && segments[0] === "snapshot-sqlite-native-syntax") {
      const owner=resolve(this.root,"../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot");
      await runOwnedCommand("rustfmt",["--emit","stdout","--edition","2024",resolve(owner,"🦀️.rs"),resolve(owner,"📏️cells/🦀️.rs"),resolve(owner,"🧪️tests/🧬️owned/🦀️.rs")],this.repoRoot,"owned-sqlite-native-syntax",120_000);
      console.log("[DEBUG] Remodeling three owning Native SQLite sources parsed without changing files");
      return;
    }
    if (segments[0] === "video-container-providers") {
      if (segments.length !== 1) throw Error("video-container-providers accepts no additional arguments");
      const { proveVideoContainerProviderOracleV1 } = await import("../../🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎥️video/🧪️tests/🔌️container-providers/🟦️.ts");
      console.log("video-container-providers: " + proveVideoContainerProviderOracleV1() + " portable oracle vectors admitted");
      const base = ["test", "--manifest-path", join(this.root, "Cargo.toml"), "--lib"];
      await runCargo([...base, "engine::video::tests::", "--", "--nocapture"], this.root);
      for (const feature of [null, "video-mp4", "video-avi"]) {
        await runCargo([...base, "--no-default-features", ...(feature ? ["--features", feature] : []), "video_container_provider_", "--", "--nocapture"], this.root);
      }
      return;
    }
    if (segments[0] === "remodel-window-ownership") {
      const schemaRoot = join(this.repoRoot, "✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes");
      const testRoot = join(schemaRoot, "🧊️model/🪟️windows/🧊️model/🎚️config/🧪️tests/🔬️window");
      const { testRemodelWindowOwnershipOracle } = await import(`${testRoot}/🟦️.ts`);
      testRemodelWindowOwnershipOracle();
      runCmd("bun", [join(this.repoRoot, "node_modules/typescript/bin/tsc"), "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--resolveJsonModule", "--allowImportingTsExtensions", "--esModuleInterop", "--skipLibCheck", join(schemaRoot, "🧊️model/🪟️windows/🧊️model/🎚️config/🧬️schema/🟦️.ts"), join(schemaRoot, "📷️capture/🪟️windows/🖼️frames/🎚️config/🧬️schema/🟦️.ts"), join(schemaRoot, "🔍️analyze/🪟️windows/📊️report/🎚️config/🧬️schema/🟦️.ts"), `${testRoot}/🟦️.ts`], { cwd: this.repoRoot });
      if (segments[1] === "native") {
        await runCargo(["test", "--manifest-path", join(this.root, "Cargo.toml"), "--lib", "remodel_window_ownership_", "--", "--nocapture"], this.root);
      }
      return;
    }
    throw new Error('Unknown owned verification '+segments.join(' '));
  }
}

await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-remodel-remodeling", { commands: { verify: OwnedVerifyScript, composition: CompositionScript }, snapshotSqliteTests: ["../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts","../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🧬️schema/🟦️.ts"] });


