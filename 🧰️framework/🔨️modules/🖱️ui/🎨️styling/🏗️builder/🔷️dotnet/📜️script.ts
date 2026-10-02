import { mkdirSync, mkdtempSync, rmSync } from "node:fs";
import { join, resolve } from "node:path";
import { BundleScript } from "../../../../🏃️process/🧭️routing/🟦️.ts";
import { readProcessOwnerContextV1, processCacheDirectoryV1 } from "../../../../🏃️process/📋️context/🟦️.ts";
import { buildBudgetMs } from "../../../../🏃️process/⏱️budget/🟦️.ts";
import { stageArtifacts } from "../../../../🏃️process/📦️artifacts/📤️publication/🟦️.ts";
import { collectArtifactFiles } from "../../../../🏃️process/📦️artifacts/🗂️files/🟦️.ts";
import { runOwnedCommand } from "../../../../🏃️process/🎛️owned-execution/🟦️.ts";
import { acquireResourceLease } from "../../../../🏃️process/🔒️leases/🟦️.ts";

const dotnetState = (root: string): string => process.env.SEMIO_STYLING_DOTNET_ARTIFACTS_ROOT ?? processCacheDirectoryV1(readProcessOwnerContextV1(process.env,root),"dotnet","ui-styling");

/** 🔐️ Serializes access to the shared .NET compiler store across Nx processes. */
async function withDotnetState(root: string, action: (state: string, signal: AbortSignal) => Promise<void>): Promise<void> {
  const state = dotnetState(root), signal = new AbortController();
  const cancel = () => signal.abort();
  process.once("SIGINT", cancel);
  process.once("SIGTERM", cancel);
  try {
    const lease = await acquireResourceLease({ directory: readProcessOwnerContextV1(process.env,root).leaseDirectory, resource: `styling-dotnet:${resolve(state)}`, mode: "exclusive", signal: signal.signal, onWait: ({ elapsedMs }) => console.log(`Waiting for styling .NET compiler state: ${elapsedMs}ms`) });
    try { signal.signal.throwIfAborted(); await action(state, signal.signal); } finally { lease.release(); }
  } finally { process.off("SIGINT", cancel); process.off("SIGTERM", cancel); }
}

/** 📥️ Restores the dependency-free styling project into isolated native state. */
export class StylingDotnetDepsScript extends BundleScript {
  async run(): Promise<void> {
    await withDotnetState(this.root, (state,signal) => runOwnedCommand("dotnet", ["restore", "🔷️.csproj", "--artifacts-path", state], this.root, "styling-dotnet-restore", buildBudgetMs(),{signal}));
  }
}

/** 🔷️ Publishes the .NET palette assembly separately from incremental compiler state. */
export class StylingDotnetBuildScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length) throw new Error("The .NET styling package has one Release build contract");
    await withDotnetState(this.root, async (state, signal) => {
      const temporaryRoot = join(state, "staging");
      mkdirSync(temporaryRoot, { recursive: true });
      const temporary = mkdtempSync(join(temporaryRoot, "build-"));
      try {
        await runOwnedCommand("dotnet", ["build", "🔷️.csproj", "--no-restore", "--configuration", "Release", "--artifacts-path", state, "--output", temporary, `-p:PathMap=${state}=/_/native%2C${this.repoRoot}=/_/`, "-p:ContinuousIntegrationBuild=true"], this.root, "styling-dotnet-build", buildBudgetMs(),{signal});
        const files = await collectArtifactFiles(temporary, signal);
        if (!files.has("Semio.Framework.Ui.Styling.dll")) throw new Error("The styling assembly was not produced");
        await stageArtifacts(join(this.root, "dist/build"), "@semio-tech/ui-styling-dotnet:build", files, { signal, leaseDirectory:readProcessOwnerContextV1(process.env,this.root).leaseDirectory });
        console.log(`Published styling .NET assembly: ${files.size} files`);
      } finally { rmSync(temporary, { recursive: true, force: true }); }
    });
  }
}
