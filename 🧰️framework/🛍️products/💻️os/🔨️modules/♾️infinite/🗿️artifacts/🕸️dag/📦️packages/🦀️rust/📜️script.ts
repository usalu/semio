import { runArtifactRustPackageMain } from "../../../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/🟦️.ts";
import { runHostKindAdmissionChecks } from "../../🧪️tests/📥️host-kind-admission/🟦️.ts";
await runArtifactRustPackageMain(import.meta.dir, "semio-framework-artifact-infinite-dag", { snapshotSqliteTests: ["../../🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts"], twins: [{ name: "host-kind-admission", run: runHostKindAdmissionChecks }] });
