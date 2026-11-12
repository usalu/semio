import { ScriptRouter } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runArtifactRustPackageMain } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts";
import { OwnedVerifyScript } from "../../🧪️tests/🔬️owned-verify/🟦️.ts";
await runArtifactRustPackageMain(import.meta.dirname, "semio-framework-artifact-infinite-dag", { commands: new ScriptRouter(import.meta.dirname).register("verify", OwnedVerifyScript), snapshotSqliteTests: ["../../🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts"] });
