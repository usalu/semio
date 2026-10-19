
import { runArtifactRustPackageMain } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
import { familyPackages } from "../../🧪️tests/🔬️rust-package-commands/🟦️.ts";
await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-stdio-step", {packages:familyPackages,snapshotSqliteTests:["../../🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts"],snapshotSqliteTestBudgetMs:120000});
