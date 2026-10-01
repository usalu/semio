#!/usr/bin/env bun
/** 🧬️ Shared stdio artifact contract package router. */
import { runArtifactRustPackageMain } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/🟦️.ts";
import { runContributionChecks } from "../../🧪️tests/📇️contributions/🟦️.ts";
import { runDefinitionHierarchyChecks } from "../../🧪️tests/🪜️definition-hierarchy/🟦️.ts";
import { runCompositionContributionChecks } from "../../🧩️composition/🧪️tests/🟦️.ts";
await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-stdio-contract", { twins: [{ name: "definition-hierarchy", run: runDefinitionHierarchyChecks }, { name: "contribution-removal", run: runContributionChecks }, { name: "composition-selection", run: runCompositionContributionChecks }] });
