import { BundleScript } from "../../../📦️packages/🟦️typescript/🟦️.ts";


export class DoctorScript extends BundleScript {
  run(): void {
    for (const command of ["bun", "node", "cargo", "rustc", "go", "uv", "dotnet", "cmake"]) {
      try {
        const result = (globalThis as any).Bun.spawnSync([command, command === "go" ? "version" : "--version"], { stdout: "pipe", stderr: "pipe" });
        console.log(`[nx-doctor] ${command}: ${result.exitCode === 0 ? result.stdout.toString().split("\n")[0] : "unavailable"}`);
      } catch { console.log(`[nx-doctor] ${command}: unavailable`); }
    }
  }
}

