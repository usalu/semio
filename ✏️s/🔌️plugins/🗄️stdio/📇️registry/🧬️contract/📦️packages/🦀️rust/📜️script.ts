#!/usr/bin/env bun
/** 🧬️ Shared stdio artifact contract package router. */
import { runArtifactRustPackageMain } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/📜️script.ts";
import { runContributionChecks } from "../../🧪️tests/📇️contributions/🟦️.ts";
await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-stdio-contract", { twins: [{ name: "contribution-removal", run: runContributionChecks }] });
