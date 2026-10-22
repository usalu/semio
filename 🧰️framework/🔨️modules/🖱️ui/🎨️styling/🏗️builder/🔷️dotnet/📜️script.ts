import { acquireCompilerCapacityLeaseV1, scriptCompilerAdmissionControlV1, compilerAdmissionBudgetV1, type CompilerAdmissionControl } from "../../../../🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { buildModePolicy, readBuildMode } from "../../../../🏃️process/📦️packages/🟦️typescript/🏗️build/🎚️mode/🟦️.ts";
import { toolchainIdentityV1 } from "../../../../🏃️process/📦️packages/🟦️typescript/🏗️build/🧰️toolchains/🟦️.ts";
import { mkdirSync, mkdtempSync, rmSync } from "node:fs";
import { join, resolve } from "node:path";
import { BundleScript, type ScriptInvocation } from "../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { readProcessOwnerContextV1, processCacheDirectoryV1 } from "../../../../🏃️process/📦️packages/🟦️typescript/📋️context/🟦️.ts";
import { buildBudgetMs } from "../../../../🏃️process/📦️packages/🟦️typescript/⏱️budget/🟦️.ts";
import { stageArtifacts } from "../../../../🏃️process/📦️packages/🟦️typescript/📦️artifacts/📤️publication/🟦️.ts";
import { collectArtifactFiles } from "../../../../🏃️process/📦️packages/🟦️typescript/📦️artifacts/🗂️files/🟦️.ts";
import { runOwnedCommand } from "../../../../🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { acquireResourceLease } from "../../../../🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";

const dotnetState = (root: string): string => process.env.SEMIO_STYLING_DOTNET_ARTIFACTS_ROOT ?? processCacheDirectoryV1(readProcessOwnerContextV1(process.env,root),"dotnet","ui-styling",`${process.platform}-${process.arch}-${toolchainIdentityV1("dotnet").digest.slice(0,12)}`,readBuildMode(process.env.SEMIO_BUILD_MODE));

/** 🔐️ Serializes access to the shared .NET compiler store across build processes. */
async function withDotnetState(root: string, invocation: ScriptInvocation, action: (state: string, signal: AbortSignal, parallelism: number, control:CompilerAdmissionControl) => Promise<void>): Promise<void> {
  const state = dotnetState(root), control = scriptCompilerAdmissionControlV1(invocation, "styling:dotnet", buildBudgetMs()), signal = control.signal;
  let capacity: Awaited<ReturnType<typeof acquireCompilerCapacityLeaseV1>> | undefined;
  try {
    capacity = await acquireCompilerCapacityLeaseV1({request:{kind:"whole-host"},directory:readProcessOwnerContextV1(process.env,root).leaseDirectory,control});
    const lease = await acquireResourceLease({ directory: readProcessOwnerContextV1(process.env,root).leaseDirectory, resource: `styling-dotnet:${resolve(state)}`, mode: "exclusive", signal, control, onWait: ({ elapsedMs }) => console.log(`Waiting for styling .NET compiler state: ${elapsedMs}ms`) });
    try { signal.throwIfAborted(); await action(state, control.signal, capacity.parallelism,control); } finally { lease.release(); }
  } finally { capacity?.release(); }
}

/** 📥️ Restores the dependency-free styling project into isolated native state. */
export class StylingDotnetDepsScript extends BundleScript {
  async run(): Promise<void> {
    await withDotnetState(this.root, this.invocation, (state,signal,parallelism,control) => runOwnedCommand("dotnet", ["restore", "🔷️.csproj", "--artifacts-path", state], this.root, "styling-dotnet-restore", compilerAdmissionBudgetV1(control),{signal}));
  }
}

/** 🔷️ Publishes the .NET palette assembly separately from incremental compiler state. */
export class StylingDotnetBuildScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length) throw new Error("The .NET styling build uses SEMIO_BUILD_MODE");
    const mode = readBuildMode(process.env.SEMIO_BUILD_MODE);
    await withDotnetState(this.root, this.invocation, async (state, signal, parallelism,control) => {
      const temporaryRoot = join(state, "staging");
      mkdirSync(temporaryRoot, { recursive: true });
      const temporary = mkdtempSync(join(temporaryRoot, "build-"));
      try {
        await runOwnedCommand("dotnet", ["build", "🔷️.csproj", "--no-restore", "--configuration", buildModePolicy(mode).dotnetConfiguration, `-maxcpucount:${parallelism}`, "--artifacts-path", state, "--output", temporary, `-p:PathMap=${state}=/_/native%2C${this.repoRoot}=/_/`, "-p:ContinuousIntegrationBuild=true"], this.root, "styling-dotnet-build", compilerAdmissionBudgetV1(control),{signal});
        const files = await collectArtifactFiles(temporary, signal);
        if (!files.has("Semio.Framework.Ui.Styling.dll")) throw new Error("The styling assembly was not produced");
        await stageArtifacts(join(this.root, "dist/build", mode), `@semio-tech/ui-styling-dotnet:build:${mode}`, files, { signal, leaseDirectory:readProcessOwnerContextV1(process.env,this.root).leaseDirectory });
        console.log(`Published styling .NET assembly: ${files.size} files`);
      } finally { rmSync(temporary, { recursive: true, force: true }); }
    });
  }
}
