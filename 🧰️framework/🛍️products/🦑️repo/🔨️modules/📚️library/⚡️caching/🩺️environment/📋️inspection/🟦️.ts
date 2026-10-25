import { BundleScript } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runtimeCommandNamesV1 } from "../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🏷️identity/🟦️.ts";
import { runtimeSpawnSyncV1 } from "../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🚀️process/🟦️.ts";

export class DoctorScript extends BundleScript {
  run(): void {
    for (const command of [...runtimeCommandNamesV1(), "cargo", "rustc", "go", "uv", "dotnet", "cmake"]) {
      try {
        const result = runtimeSpawnSyncV1([command, command === "go" ? "version" : "--version"], { stdout: "pipe", stderr: "pipe" });
        console.log(`[nx-doctor] ${command}: ${result.exitCode === 0 ? result.stdout.toString().split("\n")[0] : "unavailable"}`);
      } catch {
        console.log(`[nx-doctor] ${command}: unavailable`);
      }
    }
  }
}
