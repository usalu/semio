#!/usr/bin/env bun
/** 📦️ dxf Rust artifact package router. */
import { runArtifactRustPackageMain } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-stdio-dxf", { packages: ["semio-s-artifact-stdio-dxf", "semio-s-artifact-stdio-dxf-diff", "semio-s-artifact-stdio-dxf-diff-algebra", "semio-s-artifact-stdio-dxf-inferences", "semio-s-artifact-stdio-dxf-mutations", "semio-s-artifact-stdio-dxf-schema", "semio-s-artifact-stdio-dxf-snapshot", "semio-s-artifact-stdio-dxf-tests"], snapshotSqliteTests: ["../../🏅️standards/🔖️r12/🪆️subsets/📰️header/🧬️schema/♻️retirement/🧪️tests/🟦️.ts", "../../🏅️standards/🔖️r12/🪆️subsets/📰️header/🚪️io/📝️text/📸️snapshot/🧪️tests/🏭️producer/🟦️.ts"] });
