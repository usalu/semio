#!/usr/bin/env bun
/** 🏃️ Shared playbook artifact package router. */
import { runArtifactRustPackageMain } from "../../../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/🟦️.ts";
if (process.argv[2] === "generation-root-policy") {
  const { proceduralGenerationRootSelfTests } = await import("../../🧬️generation/🧪️tests/🔬️procedural-generation-root/🟦️.ts");
  console.log(`[DEBUG] generation root policy checks=${proceduralGenerationRootSelfTests()}`);
} else {
  await runArtifactRustPackageMain(import.meta.dir, "semio-framework-artifact-playbook-playbook");
}
