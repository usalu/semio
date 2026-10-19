import type { ArtifactPackageProviderV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts";
import { runArtifactRustPackageMain, runArtifactRustPackagesTests } from "../../../⚡️caching/📦️artifacts/🦀️rust/🟦️.ts";
import { runArtifactTypeScriptPackageMain } from "../../../⚡️caching/📦️artifacts/🟦️typescript/📜️script.ts";

/** 🔌️ Implements the framework artifact package port with the repository's Nx-native Rust and TypeScript package contracts. */
export const provider: ArtifactPackageProviderV1 = {
  runRustPackageMain: runArtifactRustPackageMain,
  runRustPackagesTests: runArtifactRustPackagesTests,
  runTypeScriptPackageMain: runArtifactTypeScriptPackageMain,
};
